use logid::log;

use logid_core::{evident::event::finalized::FinalizedEvent, log_id::LogId, result::LoggedData};
use logid_derive::ErrLogId;

#[derive(ErrLogId)]
enum InnerError {
    First,
    Second,
}

impl From<(InnerError, FinalizedEvent<LogId>)> for InnerError {
    fn from(value: (InnerError, FinalizedEvent<LogId>)) -> Self {
        value.0
    }
}

fn inner() -> logid::result::LoggedResult<(), InnerError> {
    let event = log!(InnerError::First, "Test");
    Err((InnerError::First, event).into())
}

#[derive(ErrLogId)]
enum OuterError {
    A,
    B,
}

impl From<InnerError> for OuterError {
    fn from(_value: InnerError) -> Self {
        OuterError::A
    }
}

impl From<(OuterError, FinalizedEvent<LogId>)> for OuterError {
    fn from(value: (OuterError, FinalizedEvent<LogId>)) -> Self {
        value.0
    }
}

// fn outer() -> logid::result::LoggedResult<(), OuterError> {
//     inner().map_err(|err| {
//         LoggedData::from((
//             std::convert::Into::<OuterError>::into(err.data()),
//             err.event().clone(),
//         ))
//     })
// }

#[test]
fn one_hop_chain() {}
