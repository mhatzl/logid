// //! This module provides helper functions for integration tests.
// //!
// //! Note: Every top level file is treated as its own crate, so this module must be imported explicitly if needed.
// //!
// //! Note: Allow *dead_code* needed, because rust-analyzer marks these functions as dead_code eventhough they are used in tests.

// use std::collections::{HashMap, HashSet};

// use logid::{drain_map, get_last_finalized_id, id_entry::LogIdEntry, log_id::LogId};

// /// Helper function for draining the central log map.
// /// A delay is explicitly set before draining to ensure that sent events were captured.
// ///
// /// Note: Panics if central map is empty.
// #[allow(dead_code)]
// pub fn delayed_map_drain() -> HashMap<LogId, HashSet<LogIdEntry>> {
//     std::thread::sleep(std::time::Duration::from_millis(10));
//     drain_map!().unwrap()
// }

// /// Helper function to get the last finalized id of the central log map.
// /// A delay is explicitly set before accessing the map to ensure that sent events were captured.
// #[allow(dead_code)]
// pub fn delayed_get_last_finalized_id() -> Option<LogId> {
//     std::thread::sleep(std::time::Duration::from_millis(10));
//     get_last_finalized_id!()
// }
