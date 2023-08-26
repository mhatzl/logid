use evident::event::finalized::FinalizedEvent;

use crate::log_id::LogId;

pub struct LoggedData<T> {
    data: T,
    event: Option<FinalizedEvent<LogId>>,
}

pub type LoggedResult<T, E> = Result<LoggedData<T>, LoggedData<E>>;

impl<T> std::ops::Deref for LoggedData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> LoggedData<T> {
    pub fn event(&self) -> &Option<FinalizedEvent<LogId>> {
        &self.event
    }

    pub fn data(self) -> T {
        self.data
    }
}

impl<T> From<T> for LoggedData<T> {
    fn from(value: T) -> Self {
        LoggedData {
            data: value,
            event: None,
        }
    }
}

impl<T> From<(T, FinalizedEvent<LogId>)> for LoggedData<T> {
    fn from(value: (T, FinalizedEvent<LogId>)) -> Self {
        LoggedData {
            data: value.0,
            event: Some(value.1),
        }
    }
}

// Conflicts with core...
// impl<T, E1, E2: From<E1>> From<self::Result<T, E1>> for self::Result<T, E2> {
//     fn from(value: self::Result<T, E1>) -> Self {
//         todo!()
//     }
// }

// impl<T, E> std::ops::Deref for LoggedResult<T, E> {
//     type Target = std::result::Result<T, E>;

//     fn deref(&self) -> &Self::Target {
//         &self.std_res
//     }
// }

// impl<T, E> From<(E, FinalizedEvent<LogId>)> for self::Result<T, E> {
//     fn from(value: (E, FinalizedEvent<LogId>)) -> Self {
//         self::Result {
//             std_res: Err(value.0),
//             event: Some(value.1),
//         }
//     }
// }

// impl<T, E, F: From<E>> From<std::result::Result<T, E>> for self::Result<T, F> {
//     fn from(value: std::result::Result<T, E>) -> Self {
//         self::Result {
//             std_res: value.map_err(|e| e.into()),
//             event: None,
//         }
//     }
// }

// impl<T, E> From<self::Result<T, E>> for std::result::Result<T, E> {
//     fn from(value: self::Result<T, E>) -> Self {
//         value.std_res
//     }
// }

// impl<'a, T, E> From<&'a self::Result<T, E>> for &'a std::result::Result<T, E> {
//     fn from(value: &'a self::Result<T, E>) -> Self {
//         &value.std_res
//     }
// }
