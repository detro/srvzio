//! Representation of the internal status of a `Service`

use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

/// The possible statuses of a `Service`
#[derive(Debug, PartialEq)]
pub enum ServiceStatus {
  Starting,
  Started,
  Stopping,
  Stopped,
}

impl From<usize> for ServiceStatus {
  fn from(raw_state: usize) -> Self {
    match raw_state {
      0x01 => ServiceStatus::Starting,
      0x02 => ServiceStatus::Started,
      0x04 => ServiceStatus::Stopping,
      0x08 => ServiceStatus::Stopped,
      _ => panic!("There is no `ServiceState` that corresponds to {}", raw_state),
    }
  }
}

impl From<ServiceStatus> for usize {
  fn from(state: ServiceStatus) -> Self {
    match state {
      ServiceStatus::Starting  => 0x01,
      ServiceStatus::Started   => 0x02,
      ServiceStatus::Stopping  => 0x04,
      ServiceStatus::Stopped   => 0x08,
    }
  }
}

/// A flag that wraps the internal status of a `Service`, in a thread safe _envelope_.
///
/// This should usually be stored as a field of a `Service` implementation
#[derive(Debug, Clone)]
pub struct ServiceStatusFlag {
  flag: Arc<AtomicUsize>,
}

impl ServiceStatusFlag {

  /// Constructor
  ///
  /// # Parameters
  ///
  /// * `status`: a `ServiceStatus` for how this flag is at its creation
  pub fn new(status: ServiceStatus) -> Self {
    ServiceStatusFlag {
      flag: Arc::new(AtomicUsize::new(status.into())),
    }
  }

  /// Get the `ServiceStatus` wrapped by this flag
  pub fn get_status(&self) -> ServiceStatus {
    self.flag.load(Ordering::SeqCst).into()
  }

  /// Set the `ServiceStatus` wrapped by this flag
  pub fn set_status(&self, status: ServiceStatus) {
    self.flag.store(status.into(), Ordering::SeqCst)
  }

  /// Set starting
  ///
  /// Usually used by a `Service` at the beginning of the `start()` method: the `Service` hasn't
  /// started yet, but it's going through it's _launch sequence_.
  pub fn starting(&self) {
    self.set_status(ServiceStatus::Starting)
  }

  /// Set started
  ///
  /// Usually used by a `Service` at the end of the `start()` logic.
  /// If `start()` spawns a thread, this should be called at the end of that thread, not sooner.
  pub fn started(&self) {
    self.set_status(ServiceStatus::Started)
  }

  /// Set stopping
  ///
  /// Usually used by a `Service` at the beginning of the `stop()` method: the `Service` hasn't
  /// stopeed yet, but it's going through it's _shutdown sequence_.
  pub fn stopping(&self) {
    self.set_status(ServiceStatus::Stopping)
  }

  /// Set stopped
  ///
  /// Usually used by a `Service` at the end of the `stop()` logic.
  /// If `stop()` has to wait for a thread to end (i.e. `.join()`), this should be called after
  /// that, not sooner.
  pub fn stopped(&self) {
    self.set_status(ServiceStatus::Stopped)
  }

  /// Is it starting?
  pub fn is_starting(&self) -> bool {
    self.get_status() == ServiceStatus::Starting
  }

  /// Is it started?
  pub fn is_started(&self) -> bool {
    self.get_status() == ServiceStatus::Started
  }

  /// Is it stopping?
  pub fn is_stopping(&self) -> bool {
    self.get_status() == ServiceStatus::Stopping
  }

  /// Is it stopped?
  pub fn is_stopped(&self) -> bool {
    self.get_status() == ServiceStatus::Stopped
  }

  /// Await started
  ///
  /// This method **blocks** the current thread until the `ServiceStatus::Started` is set on
  /// this instance by _another_ thread.
  pub fn await_started(&self) {
    while !self.is_started() {};
  }

  /// Await stopped
  ///
  /// This method **blocks** the current thread until the `ServiceStatus::Stopped` is set on
  /// this instance by _another_ thread.
  pub fn await_stopped(&self) {
    while !self.is_stopped() {};
  }

}

impl Default for ServiceStatusFlag {

  /// Creates a `ServiceStatusFlag` that is stopped: a predictable default.
  fn default() -> Self {
    ServiceStatusFlag::new(ServiceStatus::Stopped)
  }

}
