//! Where you have services, you need managers

use crate::service::Service;

/// Manages an internal collection of Services
///
/// This implements a simple [Composite Pattern](https://en.wikipedia.org/wiki/Composite_pattern)
/// that allows to manages a group of Service in a coordinated fashion.
///
/// Services are `start()`ed in the order they are registered, and `stop()`ped in the reverse
/// order. Also, a `ServiceManager` is also a `Service`, so further composition is possible.
///
/// The design implies that in your application, you would hand over all the instances of `Service`
/// to a `ServiceManager`, that will then orchestrate their starting and stopping.
pub struct ServiceManager {
  services: Vec<Box<Service>>
}

impl ServiceManager {

  /// Constructor
  pub fn new() -> Self {
    ServiceManager {
      services: Vec::new(),
    }
  }

  /// Register an instance of `Service`.
  ///
  /// # Parameters
  ///
  /// * `service_box`: a `Box` containing an instance of implementation of the `Service` trait
  pub fn register(&mut self, service_box: Box<Service>) {
    self.services.push(service_box);
  }

}

impl Service for ServiceManager {

  /// Start all the registered `Service`s, in the order of registration
  fn start(&mut self) {
    self.services
      .iter_mut()
      .for_each(|s| s.start());
  }

  /// Start all the registered `Service`s to be started, in the order of registration
  fn await_started(&mut self) {
    self.services
      .iter_mut()
      .for_each(|s| s.await_started());
  }

  /// Stop all the registered `Service`s, in the inverse order of registration
  fn stop(&mut self) {
    self.services
      .iter_mut()
      .rev()
      .for_each(|s| s.stop());
  }

  /// Start all the registered `Service`s to be stopped, in the reverse order of registration
  fn await_stopped(&mut self) {
    self.services
      .iter_mut()
      .rev()
      .for_each(|s| s.await_stopped());
  }

}