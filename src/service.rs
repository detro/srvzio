//! The key module of this library: a `Service`

/// A `Service` is a _black box_ that does work: it can be started and it can be stopped.
///
/// This trait abstracts away the actions that can be done from the outside to a `Service`.
/// It's up to the specific implementor to make sense of what starting/stopping means.
///
/// Note that every method in this trait is by default implemented as a no-op: this leaves to the
/// actual implementor to decide what is fitting to implement, and what is not.
pub trait Service {

  /// Starts the service
  fn start(&mut self) {
    // By default, nothing to do
  }

  /// Awaits that the service is done starting.
  ///
  /// This is usually used _after_ a call to `start()`.
  fn await_started(&mut self) {
    // By default, nothing to do
  }

  /// Stops the service
  fn stop(&mut self) {
    // By default, nothing to do
  }

  /// Awaits that the service is done stopping.
  ///
  /// This is usually used _after_ a call to `stop()`.
  fn await_stopped(&mut self) {
    // By default, nothing to do
  }

}