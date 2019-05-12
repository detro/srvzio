//! Where you have services, you need managers

use crate::service::Service;
use crate::utils;

use log::*;

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
///
/// Thanks to the use of Composite Pattern, it can contain further instances of `ServiceManager`,
/// allowing for the creation of complex
/// [DAG diagrams](https://en.wikipedia.org/wiki/Directed_acyclic_graph) of services,
/// with expressive relationship between them.
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
  /// The order of registration is important: `Service`s are started in order, and stopped
  /// in reverse order.
  ///
  /// # Parameters
  ///
  /// * `service_box`: a `Box` containing an instance of implementation of the `Service` trait
  pub fn register(&mut self, service_box: Box<Service>) {
    debug!("Registering: {}", service_box.as_ref().name());
    self.services.push(service_box);
  }

  /// Wait for the Process to receive a termination signal, then stop this `ServiceManager`.
  ///
  /// It's strongly advised to use this method only onces, for the _root_ `ServiceManager`,
  /// at the end of the `main()` thread.
  pub fn await_termination_signal_then_stop(&mut self) {
    // Block until the process is terminated by a signal...
    utils::await_for_process_termination_signal();

    // ... then gracefully shut every service down
    self.stop_and_await();
  }

  /// Apply the same closure to all contained `Service`s, in order
  fn apply_ordered<F>(&mut self, closure: F) where F: Fn(&mut Box<Service>) -> () {
    self.services
      .iter_mut()
      .for_each(closure);
  }

  /// Apply the same closure to all contained `Service`s, in reverse order
  fn apply_reversed<F>(&mut self, closure: F) where F: FnMut(&mut Box<Service>) -> () {
    self.services
      .iter_mut()
      .rev()
      .for_each(closure);
  }

}

const SERVICE_MANAGER_SERVICE_NAME: &'static str = "srvzio::ServiceManager";

impl Service for ServiceManager {

  fn name(&self) -> &'static str {
    SERVICE_MANAGER_SERVICE_NAME
  }

  /// Start all registered `Service`s, in order of registration
  fn start(&mut self) {
    self.apply_ordered(|s: &mut Box<Service>| {
      debug!("Starting: {}", s.name());
      s.start()
    });
  }

  /// Wait for all registered `Service`s to be started, in order of registration
  fn await_started(&mut self) {
    self.apply_ordered(|s: &mut Box<Service>| {
      debug!("Awaiting started: {}", s.name());
      s.await_started()
    });
  }

  /// Start and then wait for all registered `Service`, in order of registration
  ///
  /// This is different then calling `start()` and then `await_started()`, because this method
  /// will wait for a `Service` to be started, before moving to the next one.
  ///
  /// This can be used to implement a _gracefull start_.
  fn start_and_await(&mut self) {
    self.apply_ordered(|s: &mut Box<Service>| s.start_and_await());
  }


  /// Stop all registered `Service`s, in reverse order of registration
  fn stop(&mut self) {
    self.apply_reversed(|s: &mut Box<Service>| {
      debug!("Stopping: {}", s.name());
      s.stop()
    });
  }

  /// Wait for all registered `Service`s to be stopped, in reverse order of registration
  fn await_stopped(&mut self) {
    self.apply_reversed(|s: &mut Box<Service>| {
      debug!("Awaiting stopped: {}", s.name());
      s.await_stopped()
    });
  }

  /// Stop and then wait for all registered `Service`, in reverse order of registration
  ///
  /// This is different then calling `stop()` and then `await_stopped()`, because this method
  /// will wait for a `Service` to be stopped, before moving to the next one.
  ///
  /// This can be used to implement a _gracefull stop_.
  fn stop_and_await(&mut self) {
    self.apply_reversed(|s: &mut Box<Service>| s.stop_and_await());
  }

}