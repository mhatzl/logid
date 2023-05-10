use std::{
    collections::HashMap,
    sync::{Arc, RwLock}, thread,
    future::Future,
};

use futures::{stream::FuturesUnordered, StreamExt};
use once_cell::sync::Lazy;
use tokio::sync::{watch, broadcast};
use tokio_stream::wrappers::WatchStream;

use crate::{log_id::LogId, event::msg::EventMsg};

pub(crate) static PUBLISHER: Lazy<LogIdEventPublisher> = Lazy::new(LogIdEventPublisher::new);

pub(crate) struct LogIdEventPublisher {
    pub(crate) subscriptions: Arc<RwLock<HashMap<SubscriptionKey, watch::Sender<EventMsg>>>>,
    pub(crate) capturer: broadcast::Sender<EventMsg>,
}

/// Buffersize of the broadcast channel for capturing log events.
/// One finalized event is passed over one message in the channel.
///
/// See tokio's [broadcast:channel](https://docs.rs/tokio/latest/tokio/sync/broadcast/fn.channel.html) for more information.
const CHANNEL_BOUND: usize = 1000;
// TODO: Use when `unwrap_or` becomes available for const fn.
// match option_env!("LOGID_EVENT_BUFFER") {
//     Some(v) => usize::from_str_radix(v, 10).unwrap_or(1000),
//     None => 1000,
// };

impl LogIdEventPublisher {
    fn new() -> Self {
        let (send, mut recv) = broadcast::channel(CHANNEL_BOUND);

        thread::spawn(move || loop {
            match futures::executor::block_on(recv.recv()) {
                Ok(event_msg) => {
                    on_event(event_msg);
                },
                Err(_) => {
                    // Sender got dropped => Publisher got dropped
                    return;
                }
            }
        });

        LogIdEventPublisher {
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
            capturer: send,
        }
    }
}

#[derive(Default, Hash, Debug, PartialEq, Eq)]
pub(crate) struct SubscriptionKey {
    crate_name: &'static str,
    log_id: LogId,
}

impl SubscriptionKey {
    pub fn new(crate_name: &'static str, log_id: LogId) -> Self {
        SubscriptionKey { crate_name, log_id }
    }
}

pub enum ReceiveKind {
    Block,
    Timeout(std::time::Duration),
    Deadline(std::time::Instant),
}

pub trait SyncReceiver {
    fn recv(&mut self, kind: ReceiveKind) -> Option<EventMsg>;
}

impl SyncReceiver for watch::Receiver<EventMsg> {
    fn recv(&mut self, kind: ReceiveKind) -> Option<EventMsg> {
        let mut recv = self.clone();
        let changed = async move {
            recv.changed().await.map_or_else(|_| false, |_| true)
        };

        let future = async move {
            match kind {
                ReceiveKind::Block => {
                    changed.await
                },
                _ => {
                    changed.await
                },
                // ReceiveKind::Timeout(timeout) => {
                //     (tokio::time::timeout(timeout, changed).await).unwrap_or(false)
                // },
                // ReceiveKind::Deadline(deadline) => {
                //     (tokio::time::timeout_at(deadline.into(), changed).await).unwrap_or(false)
                // },
            }
        };

        if futures::executor::block_on(future) {
            Some(self.borrow().clone())
        } else {
            None
        }
    }
}

pub fn subscribe(log_id: LogId, crate_name: &'static str) -> Option<watch::Receiver<EventMsg>> {
    let key = SubscriptionKey::new(crate_name, log_id);

    match PUBLISHER
        .subscriptions
        .write().ok() {
        Some(mut locked_subs) => {
            let entry = locked_subs.entry(key);
            let sender = entry.or_insert_with(|| {
                let (sender, mut recv) = watch::channel(EventMsg::default());
                let _ = recv.borrow_and_update(); // Note: Consume initial event
                sender
            });
    
            Some(sender.subscribe())
        },
        None => {
            None
        }
    }
}

#[macro_export]
macro_rules! subscribe {
    ($logid:ident) => {
        $crate::publisher::subscribe($crate::logid!($logid), env!("CARGO_PKG_NAME"))
    };
    ($logid:expr) => {
        $crate::publisher::subscribe($crate::logid!($logid), env!("CARGO_PKG_NAME"))
    };
}

pub fn subscribe_to_logs<T>(log_ids: T, crate_name: &'static str) -> Option<Vec<watch::Receiver<EventMsg>>>
where
    T: Iterator<Item = LogId>,
{
    let rcvs: Vec<watch::Receiver<EventMsg>> = log_ids
        .filter_map(|log_id| subscribe(log_id, crate_name))
        .collect();

    if rcvs.is_empty() {
        return None;
    }
    Some(rcvs)
}

#[macro_export]
macro_rules! subscribe_to_logs {
    ($logids:ident) => {
        $crate::publisher::subscribe_to_logs($crate::logids!($logids), env!("CARGO_PKG_NAME"))
    };
    ($logids:expr) => {
        $crate::publisher::subscribe_to_logs($crate::logids!($logids), env!("CARGO_PKG_NAME"))
    };
}

pub fn subscribe_to_crates<T, L>(crate_logs: T) -> Option<Vec<watch::Receiver<EventMsg>>>
where
    L: Iterator<Item = LogId>,
    T: Iterator<Item = (&'static str, L)>,
{
    let mut rcvs: Vec<watch::Receiver<EventMsg>> = Vec::new();
    for (crate_name, log_ids) in crate_logs {
        rcvs.extend(subscribe_to_logs(log_ids, crate_name).unwrap_or_default());
    }

    if rcvs.is_empty() {
        return None;
    }
    Some(rcvs)
}


// pub async fn get_first_received(receivers: &[watch::Receiver<EventMsg>], kind: ReceiveKind) -> Option<EventMsg> {
//     let mut futures = FuturesUnordered::new();
//     for recv in receivers {
//         futures.push(async move {
//             WatchStream::new(recv.to_owned()).next().await
//         });
//     }

//     let res = match kind {
//         ReceiveKind::Timeout(timeout) => {
//             tokio::time::timeout(timeout, futures.next()).await.unwrap_or(None)
//         },
//         ReceiveKind::Deadline(deadline) => {
//             tokio::time::timeout_at(deadline.into(), futures.next()).await.unwrap_or(None)
//         },
//         ReceiveKind::Block => {
//             futures.next().await
//         },
//     };
//     res.unwrap_or(None) 
// }

pub(crate) fn on_event(event_msg: EventMsg) {
    let key = SubscriptionKey::new(event_msg.crate_name, event_msg.entry.id);

    if let Ok(locked_subscriptions) = PUBLISHER.subscriptions.read() {
        if let Some(sender) = locked_subscriptions.get(&key) {
            let _ = sender.send(event_msg);
        }
    }
}
