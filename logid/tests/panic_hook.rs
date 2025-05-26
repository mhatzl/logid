#[cfg(feature = "payloads")]
mod panic_hook_tests {
    use logid::log;
    use logid_core::{
        log_id::LogId,
        logging::{event_entry::AddonKind, LOGGER},
    };
    use logid_derive::DbgLogId;
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, Serialize, Deserialize)]
    struct PanicInfo {
        payload: Option<String>,
        location: Option<String>,
    }

    impl std::fmt::Display for PanicInfo {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let p = self.payload.clone().unwrap_or("None".to_string());
            let l = self.location.clone().unwrap_or("None".to_string());

            write!(f, "Payload='{}'; Location='{}'", p, l)
        }
    }

    #[derive(Debug, Clone, DbgLogId)]
    enum CriticalInfo {
        Panic,
    }

    fn set_panic_hook() {
        std::panic::set_hook(Box::new(|panic_info| {
            let payload = panic_info.payload().downcast_ref::<&str>();
            let location = panic_info.location().map(|l| l.to_string());

            let info = PanicInfo {
                payload: payload.map(|p| p.to_string()),
                location,
            };

            log!(CriticalInfo::Panic, "Custom panic hook called.", add: AddonKind::Payload(serde_json::to_value(info).unwrap()));
        }));
    }

    #[test]
    fn custom_logid_panic_hook() {
        let recv = LOGGER.subscribe(CriticalInfo::Panic.into());

        set_panic_hook();

        std::thread::spawn(|| {
            std::thread::sleep(std::time::Duration::from_millis(100));

            panic!("Intentional panic for testing.");
        });

        let event = recv
            .unwrap()
            .get_receiver()
            .recv_timeout(std::time::Duration::from_millis(2000))
            .unwrap();

        dbg!(event.get_entry());

        assert_eq!(
            *event.get_event_id(),
            std::convert::Into::<LogId>::into(CriticalInfo::Panic),
            "Received event has wrong LogId."
        );

        let logged_payload: PanicInfo =
            serde_json::from_value(event.get_entry().get_payloads().first().unwrap().clone())
                .unwrap();

        assert_eq!(
            &logged_payload.payload.unwrap(),
            "Intentional panic for testing.",
            "Received event did not include panic payload."
        );
    }
}
