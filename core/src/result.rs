use evident::event::finalized::FinalizedEvent;

use crate::log_id::LogId;

pub struct Result<T, E> {
    std_res: std::result::Result<T, E>,
    event: Option<FinalizedEvent<LogId>>,
}

impl<T, E> self::Result<T, E> {
    pub fn event(&self) -> Option<&FinalizedEvent<LogId>> {
        self.event.as_ref()
    }
}

// Conflicts with core...
// impl<T, E1, E2: From<E1>> From<self::Result<T, E1>> for self::Result<T, E2> {
//     fn from(value: self::Result<T, E1>) -> Self {
//         todo!()
//     }
// }

impl<T, E> std::ops::Deref for self::Result<T, E> {
    type Target = std::result::Result<T, E>;

    fn deref(&self) -> &Self::Target {
        &self.std_res
    }
}

impl<T, E> From<(E, FinalizedEvent<LogId>)> for self::Result<T, E> {
    fn from(value: (E, FinalizedEvent<LogId>)) -> Self {
        self::Result {
            std_res: Err(value.0),
            event: Some(value.1),
        }
    }
}

impl<T, E, F: From<E>> From<std::result::Result<T, E>> for self::Result<T, F> {
    fn from(value: std::result::Result<T, E>) -> Self {
        self::Result {
            std_res: value.map_err(|e| e.into()),
            event: None,
        }
    }
}

impl<T, E> From<self::Result<T, E>> for std::result::Result<T, E> {
    fn from(value: self::Result<T, E>) -> Self {
        value.std_res
    }
}

impl<'a, T, E> From<&'a self::Result<T, E>> for &'a std::result::Result<T, E> {
    fn from(value: &'a self::Result<T, E>) -> Self {
        &value.std_res
    }
}
