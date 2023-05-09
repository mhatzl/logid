use std::thread;

use logid::{
    drain_map,
    id_entry::LogIdEntrySet,
    log_id::{get_log_id, EventLevel},
    set_event,
};

mod helper;
use crate::helper::delayed_map_drain;

#[test]
fn set_different_events_in_two_threads() {
    drain_map!();

    let log_id_side = get_log_id(0, 0, EventLevel::Error, 1);
    let log_id_main = get_log_id(0, 0, EventLevel::Error, 2);

    let side_thread = thread::spawn(move || {
        let msg = "Set side thread log message";
        let event = set_event!(log_id_side, msg);
        event.finalize();
    });

    let msg = "Set main thread message";
    let event = set_event!(log_id_main, msg);
    event.clone().finalize();

    assert!(side_thread.join().is_ok(), "Side thread panicked.");

    let map = delayed_map_drain();
    assert_eq!(map.len(), 2, "More or less than two events captured!");

    assert!(
        map.contains_key(&log_id_side),
        "Side Log-id not inside captured map!"
    );
    assert!(
        map.contains_key(&log_id_main),
        "Main Log-id not inside captured map!"
    );

    let entries = map.get(&log_id_main).unwrap();
    let entry = entries.get_entry(&event).unwrap();
    assert_eq!(
        *entry.get_msg(),
        msg,
        "Set and stored main thread messages are not equal"
    );
}

#[test]
fn set_same_logid_in_two_threads() {
    drain_map!();

    let log_id = get_log_id(0, 0, EventLevel::Error, 1);

    let side_thread = thread::spawn(move || {
        let msg = "Set side thread log message";
        let event = set_event!(log_id, msg);
        event.finalize();
    });

    let msg = "Set main thread message";
    let event = set_event!(log_id, msg);
    event.clone().finalize();

    assert!(side_thread.join().is_ok(), "Side thread panicked.");

    let map = delayed_map_drain();
    assert_eq!(map.len(), 1, "More or less than one event captured!");

    assert!(map.contains_key(&log_id), "Log-id not inside captured map!");

    let entries = map.get(&log_id).unwrap();
    assert_eq!(
        entries.len(),
        2,
        "More or less than two entries for the same log-id captured!"
    );

    let entry = entries.get_entry(&event).unwrap();
    assert_eq!(
        *entry.get_msg(),
        msg,
        "Set and stored main thread messages are not equal"
    );
}

#[test]
fn set_events_in_many_threads() {
    drain_map!();

    const THREAD_CNT: u8 = 63; // Note: 63 is the maximum for the local nr of a LogId
    let base_log_id = get_log_id(0, 0, EventLevel::Error, 1);
    let msg = "Set log message";

    set_event!(base_log_id, msg).finalize();

    rayon::scope(|s| {
        // start at 2 to jump over base_log_id
        for i in 2..=THREAD_CNT {
            s.spawn(move |_| {
                // Note: `finalize()` would not be needed, since events are finalized on drop, but it makes this test easier to read
                set_event!(base_log_id, msg).finalize();
                set_event!(get_log_id(0, 0, EventLevel::Error, i), msg).finalize();
            });
        }
    });

    let map = delayed_map_drain();
    assert_eq!(
        map.len(),
        THREAD_CNT as usize,
        "Not all log-id events captured!"
    );

    for i in 1..=THREAD_CNT {
        let log_id = get_log_id(0, 0, EventLevel::Error, i);
        assert!(
            map.contains_key(&log_id),
            "Log-id {} not inside captured map!",
            log_id
        );
    }

    let entries = map.get(&base_log_id).unwrap();
    assert_eq!(
        entries.len(),
        THREAD_CNT as usize,
        "Not all base log-id event entries captured!"
    );
}
