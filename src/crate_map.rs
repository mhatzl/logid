// use std::{
//     collections::{HashMap, HashSet},
//     sync::{
//         atomic::Ordering,
//         mpsc::{sync_channel, SyncSender},
//         Arc, RwLock,
//     },
//     thread,
// };

// use once_cell::sync::Lazy;

// use crate::{id_entry::LogIdEntry, id_map::LogIdMap};

// /// Map to capture [`LogId`]s per crate in separate [`LogIdMap`]s.
// pub(crate) struct CratesMap {
//     /// Map of [`LogIdMap`]s, where one [`LogIdMap`] collects all set [`LogId`](crate::log_id::LogId)s of one crate.
//     /// The crate name is used as key.
//     pub(crate) map: Arc<RwLock<HashMap<String, LogIdMap>>>,

//     pub(crate) sender: SyncSender<EventMsg>,
// }

// /// Main map to capture all logs.
// pub(crate) static CRATES_MAP: Lazy<CratesMap> = Lazy::new(CratesMap::new);

// /// Buffersize of the sync channel. One finalized event is passed over one message.
// ///
// /// See [sync_channel](https://doc.rust-lang.org/std/sync/mpsc/fn.sync_channel.html) for more information.
// const CHANNEL_BOUND: usize = 1000;
// // TODO: Use when `unwrap_or` becomes available for const fn.
// // match option_env!("LOGID_EVENT_BUFFER") {
// //     Some(v) => usize::from_str_radix(v, 10).unwrap_or(1000),
// //     None => 1000,
// // };

// impl CratesMap {
//     /// Create a new [`CratesMap`].
//     pub(crate) fn new() -> Self {
//         let (send, recv) = sync_channel(CHANNEL_BOUND);

//         thread::spawn(move || loop {
//             if let Ok(event_msg) = recv.recv() {
//                 finalize_log_event(event_msg);
//             }
//         });

//         CratesMap {
//             map: Arc::new(RwLock::new(HashMap::new())),
//             sender: send,
//         }
//     }
// }

// fn finalize_log_event(mut event_msg: EventMsg) {
//     let id = event_msg.entry.id;
//     let mut create_new_map = false;

//     if let Ok(locked_crate_map) = CRATES_MAP.map.read() {
//         match locked_crate_map.get(event_msg.crate_name) {
//             Some(crate_map) => {
//                 add_entry_to_map(std::mem::take(&mut event_msg), crate_map);
//             }
//             None => {
//                 create_new_map = true;
//             }
//         }
//     }

//     // Note: Duplication of get() is needed since another thread might have aquired a write lock between the above read lock and this write lock.
//     if create_new_map {
//         if let Ok(mut locked_crate_map) = CRATES_MAP.map.write() {
//             match locked_crate_map.get(event_msg.crate_name) {
//                 Some(crate_map) => {
//                     add_entry_to_map(event_msg, crate_map);
//                 }
//                 None => {
//                     let map = LogIdMap::new_with(
//                         vec![(id, RwLock::new(HashSet::from([event_msg.entry])))].into_iter(),
//                     );
//                     map.last_finalized_id.store(id, Ordering::Relaxed);
//                     locked_crate_map.insert(event_msg.crate_name.to_owned(), map);
//                 }
//             }
//         }
//     }
// }

// fn add_entry_to_map(mut event_msg: EventMsg, crate_map: &LogIdMap) {
//     let id = event_msg.entry.id;
//     let mut create_new_key = false;

//     if let Ok(locked_id_map) = crate_map.map.read() {
//         match locked_id_map.get(&id) {
//             Some(id_entry) => {
//                 if let Ok(mut locked_id_entry) = id_entry.write() {
//                     locked_id_entry.insert(std::mem::take(&mut event_msg.entry));
//                 }
//             }
//             None => {
//                 create_new_key = true;
//             }
//         }
//     }

//     // Note: Duplication of get() is needed since another thread might have aquired a write lock between the above read lock and this write lock.
//     if create_new_key {
//         if let Ok(mut locked_id_map) = crate_map.map.write() {
//             match locked_id_map.get(&id) {
//                 Some(id_entry) => {
//                     if let Ok(mut locked_id_entry) = id_entry.write() {
//                         locked_id_entry.insert(event_msg.entry);
//                     }
//                 }
//                 None => {
//                     locked_id_map.insert(id, RwLock::new(HashSet::from([event_msg.entry])));
//                 }
//             }
//         }
//     }

//     crate_map.last_finalized_id.store(id, Ordering::Relaxed);
// }
