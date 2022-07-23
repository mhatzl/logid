//! Offers functionality to set an event on a [`LogId`] and capture its content in a [`LogIdMap`].

use crate::{
  id_entry::{LogIdEntry},
  id_map::{LOG_ID_MAP, LogIdMap}, log_id::{EventLevel, LogId},
};

/// Trait to use [`LogId`] for tracing.
pub trait LogIdTracing {
  /// Set an event for a [`LogId`] using the global [`LogIdMap`] reference [`LOG_ID_MAP`].
  ///
  /// # Arguments
  ///
  /// * `msg` - main message that is set for this log-id (should be a user-centered event description)
  /// * `filename` - name of the source file where the event is set (Note: use `file!()`)
  /// * `line_nr` - line number where the event is set (Note: use `line!()`)
  fn set_event<'a>(self, msg: &str, filename: &str, line_nr: u32) -> MappedLogId<'a>;

  /// Set an event for a [`LogId`] using a given [`LogIdMap`].
  ///
  /// # Arguments
  ///
  /// * `log_map` - the map the log-id and all its addons are captured in
  /// * `msg` - main message that is set for this log-id (should be a user-centered event description)
  /// * `filename` - name of the source file where the event is set (Note: use `file!()`)
  /// * `line_nr` - line number where the event is set (Note: use `line!()`)
  fn set_event_with<'a>(self, log_map: &'a LogIdMap, msg: &str, filename: &str, line_nr: u32) -> MappedLogId<'a>;

  /// Set an event for a [`LogId`] **without** adding it to a [`LogIdMap`].
  ///
  /// # Arguments
  ///
  /// * `msg` - main message that is set for this log-id (should be a user-centered event description)
  /// * `filename` - name of the source file where the event is set (Note: use `file!()`)
  /// * `line_nr` - line number where the event is set (Note: use `line!()`)
  fn set_silent_event<'a>(self, msg: &str, filename: &str, line_nr: u32) -> MappedLogId<'a>;
}

/// Traces a [`LogIdEntry`] creation.
fn trace_entry_creation(id: LogId, msg: &str, filename: &str, line_nr: u32) -> LogIdEntry {
  let id_entry = LogIdEntry::new(id, msg, filename, line_nr);

  // Note: It is not possible to set `target` via parameter, because it requires `const`
  // Same goes for `level` for the `event` macro => match and code duplication needed
  match id_entry.level {
      EventLevel::Error => tracing::error!("{}: {}", id, msg),
      EventLevel::Warn => tracing::warn!("{}: {}", id, msg),
      EventLevel::Info => tracing::info!("{}: {}", id, msg),
      EventLevel::Debug => tracing::debug!("{}: {}", id, msg),
  }

  tracing::trace!(
      "{}(origin): {}", id, String::from(&id_entry.origin)
  );

  id_entry
}

impl LogIdTracing for LogId {
  fn set_event<'a>(self, msg: &str, filename: &str, line_nr: u32) -> MappedLogId<'a> {
      self.set_event_with(&*LOG_ID_MAP, msg, filename, line_nr)
  }

  fn set_event_with<'a>(self, log_map: &'a LogIdMap, msg: &str, filename: &str, line_nr: u32) -> MappedLogId<'a> {
      let entry = trace_entry_creation(self, msg, filename, line_nr);

      let update_map = log_map.map.write();
      if let Ok(mut map) = update_map {
          match map.get_mut(&self) {
              Some(entries) => entries.push(entry),
              None => {
                  map.insert(self, [entry].into());
              }
          };
      }

      MappedLogId { id: self, map: Some(log_map) }
  }

  fn set_silent_event<'a>(self, msg: &str, filename: &str, line_nr: u32) -> MappedLogId<'a> {
      let id_entry = trace_entry_creation(self, msg, filename, line_nr);

      MappedLogId{ id: id_entry.id, map: None }
  }
}

/// Struct linking a [`LogId`] to the map the entry for the ID was added to.
pub struct MappedLogId<'a> {
  id: LogId,
  map: Option<&'a LogIdMap>,
}

impl<'a> MappedLogId<'a> {
  // Returns the [`LogId`] of the [`MappedLogId`].
  pub fn id(&self) -> LogId {
    self.id
  }

  /// Add a message describing the cause for this log-id
  pub fn add_cause(self, msg: &str) -> Self {
      tracing::info!("{}(cause): {}", self.id, msg);

      if let Some(log_map) = self.map {
          let update_map = log_map.map.write();
          if let Ok(mut map) = update_map {
              match map.get_mut(&self.id) {
                  Some(entries) => {
                      if let Some(last) = entries.last_mut() {
                          last.add_cause(msg.to_string());
                      };
                  }
                  None => {
                      tracing::warn!(
                          "Got cause=\"{}\" for log-id={}, but no base for log-id was set!",
                          msg,
                          self.id
                      )
                  }
              };
          }
      }

      self
  }

  /// Add an info message for this log-id
  pub fn add_info(self, msg: &str) -> Self {
      tracing::info!("{}(addon): {}", self.id, msg);
      add_addon_to_map(&self, msg, &tracing::Level::INFO);
      self
  }

  /// Add a debug message for this log-id
  pub fn add_debug(self, msg: &str) -> Self {
      tracing::debug!("{}(addon): {}", self.id, msg);
      add_addon_to_map(&self, msg, &tracing::Level::DEBUG);
      self
  }

  /// Add a trace message for this log-id
  pub fn add_trace(self, msg: &str) -> Self {
      tracing::trace!("{}(addon): {}", self.id, msg);
      add_addon_to_map(&self, msg, &tracing::Level::TRACE);
      self
  }
}

fn add_addon_to_map(mapped_id: &MappedLogId, msg: &str, level: &tracing::Level) {
  if let Some(log_map) = mapped_id.map {
      let update_map = log_map.map.write();
      if let Ok(mut map) = update_map {
          match map.get_mut(&mapped_id.id) {
              Some(entries) => {
                  if let Some(last) = entries.last_mut() {
                      last.add_addon(level, msg.to_string());
                  };
              }
              None => {
                  tracing::warn!(
                      "Got addon=\"{}\" for log-id={}, but no base for log-id was set!",
                      msg,
                      mapped_id.id
                  )
              }
          };
      }
  }
}
