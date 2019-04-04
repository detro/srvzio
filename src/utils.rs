//! Utilities to help you write services

use crossbeam_channel;
use ctrlc;

/// Block current thread, waiting for system termination signals (i.e. `SIGINT` / `SIGTERM`).
///
/// When writing any kind of service that has to remain in execution, it will be probably necessary
/// to find a way to _block_ the `main()` thread until the process it's terminated.
///
/// This function offers an easy mechanism: until the process receives either a POSIX `SIGINT`,
/// or a POSIX `SIGTERM`, or the equivalent on Windows, the thread calling this will block.
///
/// Just call this in your `main()` and add your "graceful termination logic" afterwards: it might
/// be a bit _naive_, but it's simple and easy to use.
pub fn wait_for_process_termination_signal() {
  let (term_sender, term_receiver) = crossbeam_channel::bounded(1);

  // Register termination signal handler that sends a single message across the channel
  ctrlc::set_handler(move || {
    term_sender.send(true).unwrap();
  }).expect("Unable to define handler for SIGTERM/SIGINT");

  // Block current thread until the single message is received across the channel
  assert_eq!(term_receiver.iter().take(1).count(), 1, "Unable to handle SIGTERM/SIGINT");
}
