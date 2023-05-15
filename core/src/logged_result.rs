// use crate::{
//     log_id::LogId,
//     logging::{event_entry::LogEventEntry, intermediary_event::IntermediaryLogEvent},
// };

// pub type LoggedResult<T, E> = Result<T, ResultErr<E>>;

// pub struct ResultErr<E: From<LogId>> {
//     pub error: E,
//     pub event_entry_id: Option<crate::evident::uuid::Uuid>,
// }

// impl<E: From<LogId>> From<CapturedEvent<LogId>> for ResultErr<E> {
//     fn from(value: CapturedEvent<LogId>) -> Self {
//         let entry_id = value.entry_id;
//         ResultErr {
//             error: value.into_event_id().into(),
//             event_entry_id: Some(entry_id),
//         }
//     }
// }

// impl<E: From<LogId>> From<IntermediaryLogEvent> for ResultErr<E> {
//     fn from(value: IntermediaryLogEvent) -> Self {
//         value.finalize().into()
//     }
// }

// impl<E: From<LogId> + Into<LogId>> ResultErr<E> {
//     pub fn log_into<E2: From<LogId> + Into<LogId>>(
//         self,
//         other_err: E2,
//         origin: crate::evident::event::origin::Origin,
//     ) -> ResultErr<E2> {
//         let causing_entry_id = self.event_entry_id;
//         let self_log_id: LogId = self.error.into();
//         let other_log_id: LogId = other_err.into();

//         let cause_msg = if let Some(cause) = causing_entry_id {
//             format!(" Details see entry: '{}'.", cause)
//         } else {
//             String::from("")
//         };
//         let msg = format!(
//             "'{}' caused by '{}'.{}",
//             &other_log_id, &self_log_id, &cause_msg
//         );

//         let event =
//             evident::event::EventFns::<LogId, LogEventEntry, IntermediaryLogEvent>::set_event(
//                 other_log_id,
//                 &msg,
//                 origin,
//             );

//         let event = if let Some(cause) = causing_entry_id {
//             event.add_cause(CapturedEvent {
//                 event_id: self_log_id,
//                 entry_id: cause,
//             })
//         } else {
//             event
//         };

//         event.into()
//     }

//     pub fn silent_into<E2: From<LogId> + Into<LogId>>(self, other_err: E2) -> ResultErr<E2> {
//         let causing_entry_id = self.event_entry_id;
//         ResultErr {
//             error: other_err,
//             event_entry_id: causing_entry_id,
//         }
//     }
// }

// #[macro_export]
// macro_rules! log_map_err {
//     ($logged_result:ident() -> $outer_enum:ident) => {
//         $logged_result()
//             .map_err(|e| e.log_into($outer_enum::default(), $crate::evident::this_origin!()))
//     };
//     ($logged_result:ident() -> $outer_enum:ident::$enum_variant:ident) => {
//         $logged_result()
//             .map_err(|e| e.log_into($outer_enum::$enum_variant, $crate::evident::this_origin!()))
//     };
// }

// #[macro_export]
// macro_rules! map_err {
//     ($logged_result:ident() -> $outer_enum:ident) => {
//         $logged_result().map_err(|e| e.silent_into($outer_enum::default()))
//     };
//     ($logged_result:ident() -> $outer_enum:ident::$enum_variant:ident) => {
//         $logged_result().map_err(|e| e.silent_into($outer_enum::$enum_variant))
//     };
// }
