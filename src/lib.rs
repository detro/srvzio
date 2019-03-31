pub mod service;
pub mod status;
pub mod manager;

pub use service::Service;
pub use status::*;
pub use manager::ServiceManager;

#[allow(unused_must_use)]
#[cfg(test)]
mod tests {
  use super::*;
  use std::{time::{Duration, Instant}, thread};
  use crossbeam_channel::{Sender, bounded};

  struct ExampleService {
    id: String,
    pub delay: Duration,
    sender: Sender<String>,
    flag: ServiceStatusFlag
  }

  impl ExampleService {
    fn new(id: String, delay: Duration, sender: Sender<String>) -> ExampleService {
      ExampleService {
        id,
        delay,
        sender,
        flag: ServiceStatusFlag::default()
      }
    }
  }

  impl Service for ExampleService {

    fn start(&mut self) {
      self.flag.starting();

      let id = self.id.clone();
      let delay = self.delay.clone();
      let sender = self.sender.clone();
      let flag = self.flag.clone();
      thread::spawn(move || {
        thread::sleep(delay);
        sender.send(format!("Service {} STARTED", id).to_string());

        flag.started();
      });
    }

    fn await_started(&mut self) {
      while !self.flag.is_started() {}
    }


    fn stop(&mut self) {
      self.flag.stopping();

      let id = self.id.clone();
      let delay = self.delay.clone();
      let sender = self.sender.clone();
      let flag = self.flag.clone();
      thread::spawn(move || {
        thread::sleep(delay);
        sender.send(format!("Service {} STOPPED", id).to_string());

        flag.stopped();
      });
    }

    fn await_stopped(&mut self) {
      while !self.flag.is_stopped() {}
    }

  }

  #[test]
  fn should_start_and_stop_services() {
    let (sender, receiver) = bounded(2);
    let mut sm = ServiceManager::new();

    sm.register(Box::new(ExampleService::new("SA".to_string(), Duration::from_millis(123), sender.clone())));
    sm.register(Box::new(ExampleService::new("SB".to_string(), Duration::from_millis(234), sender.clone())));

    let now = Instant::now();
    sm.start();
    sm.await_started();
    assert!(now.elapsed().as_millis() >= 234);
    assert_eq!("Service SA STARTED", receiver.recv().unwrap());
    assert_eq!("Service SB STARTED", receiver.recv().unwrap());

    let now = Instant::now();
    sm.stop();
    sm.await_stopped();
    assert!(now.elapsed().as_millis() >= 234);
    assert_eq!("Service SA STOPPED", receiver.recv().unwrap());
    assert_eq!("Service SB STOPPED", receiver.recv().unwrap());
  }
}
