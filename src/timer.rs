use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Encapsulates a cancelable timer
pub struct PomodoroTimer {
    cancel_sender: mpsc::Sender<()>,
    handle: thread::JoinHandle<()>,
}

impl PomodoroTimer {
    pub fn new(duration: Duration, on_complete: impl FnOnce() + Send + 'static) -> Self {
        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            // Wait for either cancel or timeout
            match rx.recv_timeout(duration) {
                Ok(()) => {} // cancellation
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    on_complete();
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {}
            }
        });

        PomodoroTimer {
            cancel_sender: tx,
            handle,
        }
    }

    pub fn cancel(self) {
        let _ = self.cancel_sender.send(()); // ignore if already closed
        self.handle.join().unwrap();
    }
}
