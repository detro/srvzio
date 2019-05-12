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

  fn name(&self) -> &'static str {
    "ExampleService"
  }

  fn start(&mut self) {
    self.flag.starting();

    let id = self.id.clone();
    let delay = self.delay.clone();
    let sender = self.sender.clone();
    let flag = self.flag.clone();
    thread::spawn(move || {
      thread::sleep(delay);
      sender.send(format!("Service {} STARTED", id).to_string()).unwrap();

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
      sender.send(format!("Service {} STOPPED", id).to_string()).unwrap();

      flag.stopped();
    });
  }

  fn await_stopped(&mut self) {
    while !self.flag.is_stopped() {}
  }

}

#[test]
fn should_start_then_stop() {
  let (sender, receiver) = bounded(2);
  let mut sm = ServiceManager::new();
  let sa_delay = 1000;
  let sb_delay = 2000;

  sm.register(Box::new(ExampleService::new("SA".to_string(), Duration::from_millis(sa_delay), sender.clone())));
  sm.register(Box::new(ExampleService::new("SB".to_string(), Duration::from_millis(sb_delay), sender.clone())));

  let now = Instant::now();
  sm.start();
  sm.await_started();
  let start_time = now.elapsed().as_millis();
  assert!(start_time >= sb_delay as u128);
  assert!(start_time < (sb_delay + sa_delay) as u128);
  assert_eq!("Service SA STARTED", receiver.recv().unwrap());
  assert_eq!("Service SB STARTED", receiver.recv().unwrap());

  let now = Instant::now();
  sm.stop();
  sm.await_stopped();
  let stop_time = now.elapsed().as_millis();
  assert!(stop_time >= sb_delay as u128);
  assert!(start_time < (sb_delay + sa_delay) as u128);
  assert_eq!("Service SA STOPPED", receiver.recv().unwrap());
  assert_eq!("Service SB STOPPED", receiver.recv().unwrap());
}

#[test]
fn should_start_and_await_then_stop_and_await() {
  let (sender, receiver) = bounded(2);
  let mut sm = ServiceManager::new();
  let sa_delay = 1000;
  let sb_delay = 2000;

  sm.register(Box::new(ExampleService::new("SA".to_string(), Duration::from_millis(sa_delay), sender.clone())));
  sm.register(Box::new(ExampleService::new("SB".to_string(), Duration::from_millis(sb_delay), sender.clone())));

  let now = Instant::now();
  sm.start_and_await();
  assert!(now.elapsed().as_millis() >= (sa_delay + sb_delay) as u128);
  assert_eq!("Service SA STARTED", receiver.recv().unwrap());
  assert_eq!("Service SB STARTED", receiver.recv().unwrap());

  let now = Instant::now();
  sm.stop_and_await();
  assert!(now.elapsed().as_millis() >= (sa_delay + sb_delay) as u128);
  assert_eq!("Service SB STOPPED", receiver.recv().unwrap());
  assert_eq!("Service SA STOPPED", receiver.recv().unwrap());
}
