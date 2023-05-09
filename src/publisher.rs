use std::{sync::{Arc, RwLock}, collections::HashMap, thread};

use crossbeam_channel::{Sender, Receiver};
use once_cell::sync::Lazy;

use crate::{log_id::LogId, id_entry::LogIdEntry};


pub(crate) static PUBLISHER: Lazy<LogIdEventPublisher> = Lazy::new(LogIdEventPublisher::new);


pub(crate) struct LogIdEventPublisher {
  pub(crate) subscriptions: Arc<RwLock<HashMap<SubscriptionKey, SubscriptionChannel>>>,

  pub(crate) sender: Sender<EventMsg>,
}

/// Buffersize of the sync channel. One finalized event is passed over one message.
///
/// See [sync_channel](https://doc.rust-lang.org/std/sync/mpsc/fn.sync_channel.html) for more information.
const CHANNEL_BOUND: usize = 1000;
// TODO: Use when `unwrap_or` becomes available for const fn.
// match option_env!("LOGID_EVENT_BUFFER") {
//     Some(v) => usize::from_str_radix(v, 10).unwrap_or(1000),
//     None => 1000,
// };

impl LogIdEventPublisher {
  fn new() -> Self {
    let (send, recv) = crossbeam_channel::bounded(CHANNEL_BOUND);

    thread::spawn(move || loop {
        if let Ok(event_msg) = recv.recv() {
          on_event(event_msg);
        }
    });

    LogIdEventPublisher {
        subscriptions: Arc::new(RwLock::new(HashMap::new())),
        sender: send,
    }
  }
}


#[derive(Debug, Default)]
pub struct EventMsg {
    pub crate_name: &'static str,
    pub entry: LogIdEntry,
}


#[derive(Default, Hash, Debug, PartialEq, Eq)]
pub(crate) struct SubscriptionKey {
  crate_name: &'static str,
  log_id: LogId,
}

impl SubscriptionKey {
  pub fn new(crate_name: &'static str, log_id: LogId) -> Self {
    SubscriptionKey{ crate_name, log_id }
  }
}

pub(crate) struct SubscriptionChannel {
  sender: Sender<EventMsg>,
  receiver: Receiver<EventMsg>,
}

impl From<(Sender<EventMsg>, Receiver<EventMsg>)> for SubscriptionChannel {
    fn from(value: (Sender<EventMsg>, Receiver<EventMsg>)) -> Self {
      SubscriptionChannel {
        sender: value.0,
        receiver: value.1,
    }
    }
}


pub fn subscribe(log_id: LogId, crate_name: &'static str, ) -> Option<Receiver<EventMsg>> {
  let key = SubscriptionKey::new(crate_name, log_id);

  if let Some(event_channel) = PUBLISHER.subscriptions.read().ok()?.get(&key) {
    return Some(event_channel.receiver.clone());
  }

  let sub_channel = SubscriptionChannel::from(crossbeam_channel::bounded(CHANNEL_BOUND));
  Some(PUBLISHER.subscriptions.write().ok()?.entry(key).or_insert(sub_channel).receiver.clone())
}

#[macro_export]
macro_rules! subscribe {
    ($logid:ident) => {
        $crate::publisher::subscribe(
            $crate::logid!($logid),
            env!("CARGO_PKG_NAME"),
        )
    };
    ($logid:expr) => {
        $crate::publisher::subscribe(
            $crate::logid!($logid),
            env!("CARGO_PKG_NAME"),
        )
    };
}

pub(crate) fn on_event(event_msg: EventMsg) {
  let key = SubscriptionKey::new(event_msg.crate_name, event_msg.entry.id);

  if let Ok(locked_subscriptions) = PUBLISHER.subscriptions.read() {
    if let Some(sub_channel) = locked_subscriptions.get(&key) {
      let _ = sub_channel.sender.try_send(event_msg);
    }
  }
}
