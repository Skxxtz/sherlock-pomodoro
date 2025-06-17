use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;

/// Encapsulates a cancelable timer
pub struct PomodoroTimer {
    cancel_sender: mpsc::Sender<()>,
    handle: thread::JoinHandle<()>,
    pub is_active: Arc<AtomicBool>,
}

impl PomodoroTimer {
    pub fn new(duration: Duration, on_complete: impl FnOnce() + Send + 'static) -> Self {
        let (tx, rx) = mpsc::channel();
        let is_active = Arc::new(AtomicBool::new(true));

        let handle = thread::spawn({
            let is_active = Arc::clone(&is_active);
            move || {
                // Wait for either cancel or timeout
                match rx.recv_timeout(duration) {
                    Ok(()) => {} // cancellation
                    Err(mpsc::RecvTimeoutError::Timeout) => {
                        is_active.store(false, Ordering::SeqCst);
                        on_complete();
                    }
                    Err(mpsc::RecvTimeoutError::Disconnected) => {}
                }
            }
        });

        PomodoroTimer {
            cancel_sender: tx,
            handle,
            is_active,
        }
    }

    pub fn cancel(self) {
        let _ = self.cancel_sender.send(()); // ignore if already closed
        self.handle.join().unwrap();
    }
}
