use super::entry::Entry;

#[derive(Debug, Default, Clone)]
pub struct EventMsg {
    pub crate_name: &'static str,
    pub entry: Entry,
}
