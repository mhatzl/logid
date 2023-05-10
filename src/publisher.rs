use std::{sync::{Arc, RwLock}, collections::HashMap, thread};

use crossbeam_channel::{Sender, Receiver, Select};
use once_cell::sync::Lazy;

use crate::{log_id::LogId, id_entry::LogIdEntry};


pub(crate) static PUBLISHER: Lazy<LogIdEventPublisher> = Lazy::new(LogIdEventPublisher::new);


pub(crate) struct LogIdEventPublisher {
  pub(crate) subscriptions: Arc<RwLock<HashMap<SubscriptionKey, Vec<Sender<EventMsg>>>>>,

  pub(crate) sender: Sender<EventMsg>,
}

/// Buffersize of bounded channels. One finalized event is passed over one message.
///
/// See [crossbeam_channel::bounded](https://docs.rs/crossbeam-channel/latest/crossbeam_channel/fn.bounded.html) for more information.
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


#[derive(Debug, Default, Clone)]
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

pub fn subscribe(log_id: LogId, crate_name: &'static str) -> Option<Receiver<EventMsg>> {
  let key = SubscriptionKey::new(crate_name, log_id);
  let (sender, receiver) = crossbeam_channel::bounded(CHANNEL_BOUND);

  PUBLISHER.subscriptions.write().ok()?.entry(key).and_modify(|c| c.push(sender.clone())).or_insert(vec![sender]);

  Some(receiver)
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


pub fn subscribe_to_logs<T>(log_ids: T, crate_name: &'static str) -> Option<Vec<Receiver<EventMsg>>>
where T: Iterator<Item = LogId> {
    let rcvs: Vec<Receiver<EventMsg>> = log_ids.filter_map(|log_id| {
        subscribe(log_id, crate_name)
    }).collect();

    if rcvs.is_empty() {
      return None;
    }
    Some(rcvs)
}


#[macro_export]
macro_rules! subscribe_to_logs {
    ($logids:ident) => {
        $crate::publisher::subscribe_to_logs(
            $crate::logids!($logids),
            env!("CARGO_PKG_NAME"),
        )
    };
    ($logids:expr) => {
        $crate::publisher::subscribe_to_logs(
            $crate::logids!($logids),
            env!("CARGO_PKG_NAME"),
        )
    };
}


pub fn subscribe_to_crates<T, L>(crate_logs: T) -> Option<Vec<Receiver<EventMsg>>> 
where 
  L: Iterator<Item = LogId>,
  T: Iterator<Item = (&'static str, L)> {
    let mut rcvs: Vec<Receiver<EventMsg>> = Vec::new();
    for (crate_name, log_ids) in crate_logs {
        rcvs.extend(subscribe_to_logs(log_ids, crate_name).unwrap_or_default());
    }

    if rcvs.is_empty() {
      return None;
    }
    Some(rcvs)
}

pub enum ReceiveKind {
  Select,
  SelectTimeout(std::time::Duration),
  SelectDeadline(std::time::Instant),
  Ready,
  ReadyTimeout(std::time::Duration),
  ReadyDeadline(std::time::Instant),
}

pub fn receive_any(receiver: &[Receiver<EventMsg>], kind: ReceiveKind) -> Option<EventMsg> {
    let mut select = Select::new();
    for rcv in receiver {
      select.recv(rcv);
    }
    
    match kind {
        ReceiveKind::Select |
        ReceiveKind::SelectTimeout(_) |
        ReceiveKind::SelectDeadline(_) => {
          let op = match kind {
            ReceiveKind::Select => select.select(),
            ReceiveKind::SelectTimeout(duration) => select.select_timeout(duration).ok()?,
            ReceiveKind::SelectDeadline(instant) => select.select_deadline(instant).ok()?,
            _ => {return None;}
          };
      
          match receiver.get(op.index()) {
              Some(rcv) => op.recv(rcv).ok(),
              None => None,
          }
        },
        ReceiveKind::Ready |
        ReceiveKind::ReadyTimeout(_) |
        ReceiveKind::ReadyDeadline(_) => {
          let op = match kind {
            ReceiveKind::Ready => select.ready(),
            ReceiveKind::ReadyTimeout(duration) => select.ready_timeout(duration).ok()?,
            ReceiveKind::ReadyDeadline(instant) => select.ready_deadline(instant).ok()?,
            _ => {return None;}
          };

          match receiver.get(op) {
            Some(rcv) => rcv.try_recv().ok(),
            None => None,
          }
        },
    }
}


pub(crate) fn on_event(event_msg: EventMsg) {
  let key = SubscriptionKey::new(event_msg.crate_name, event_msg.entry.id);

  if let Ok(locked_subscriptions) = PUBLISHER.subscriptions.read() {
    if let Some(senders) = locked_subscriptions.get(&key) {
      senders.iter().for_each(|sender| {
        let _ = sender.try_send(event_msg.clone());
      });
    }
  }
}
