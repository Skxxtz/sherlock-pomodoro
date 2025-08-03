use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::atomic::Ordering;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::timer::PomodoroTimer;

pub struct API {
    pub listener: Option<UnixListener>,
    remaining: Duration,
    start: Option<SystemTime>,
    timer: Option<PomodoroTimer>,
}
impl API {
    pub fn new(socket_path: &str) -> Option<Self> {
        let _ = std::fs::remove_file(socket_path);
        let listener = UnixListener::bind(socket_path).ok()?;
        Some(Self {
            listener: Some(listener),
            start: None,
            remaining: Duration::new(25 * 60, 0),
            timer: None,
        })
    }
    pub fn listen(&mut self) -> Option<()> {
        let listener = self.listener.take()?;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_client(stream);
                }
                Err(_) => {}
            }
        }
        Some(())
    }
    fn handle_client(&mut self, mut client: UnixStream) -> Option<()> {
        let mut buf = [0u8; 128];
        let bytes_received = client.read(&mut buf).ok()?;
        let message = std::str::from_utf8(&buf[..bytes_received]).unwrap_or("");

        match message.trim() {
            "start" => self.start(),
            "reset" => {
                self.reset();
            }
            "stop" => {
                self.stop();
            }
            "remaining" => {
                let rem = if let Some(start) = self.start {
                    let since_start = SystemTime::now()
                        .duration_since(start)
                        .unwrap_or(Duration::new(0, 0));
                    self.remaining.saturating_sub(since_start)
                } else {
                    self.remaining
                };
                let mut buf = Vec::with_capacity(32);
                write!(buf, "{:?}", rem).ok()?;
                let _ = client.write(&buf);
            }
            "show" => {
                let info = self.show_self();
                let _ = client.write_sized(info.as_bytes());
            }
            _ => {}
        };
        Some(())
    }
    fn start(&mut self) {
        if let Some(timer) = &self.timer {
            if timer.is_active.load(Ordering::SeqCst) {
                return;
            }
        }

        let pomodoro = PomodoroTimer::new(self.remaining, || {
            Self::on_complete();
        });

        self.timer = Some(pomodoro);
        self.start = Some(SystemTime::now());
    }
    fn stop(&mut self) {
        if let Some(pomodoro) = self.timer.take() {
            if let Some(start) = self.start.take() {
                let target = self.remaining;
                let completed = SystemTime::now()
                    .duration_since(start)
                    .unwrap_or(Duration::from_secs(0));

                let diff = target.saturating_sub(completed);

                if diff.as_secs() == 0 {
                    self.reset();
                } else {
                    self.remaining = diff;
                }
                pomodoro.cancel();
            }
        }
    }
    fn reset(&mut self) {
        if let Some(pomodoro) = self.timer.take() {
            pomodoro.cancel();
        }
        self.start = None;
        self.remaining = Duration::new(25 * 60, 0);
    }
    fn on_complete() {
        let _ = std::process::Command::new("notify-send")
            .arg("--icon=time")
            .arg("Pomodoro Timer")
            .arg("Timer completed!")
            .spawn();
    }
    fn show_self(&self) -> String {
        let end = if let Some(start) = self.start {
            start
                .checked_add(self.remaining)
                .unwrap()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string()
        } else {
            "null".to_string()
        };
        let rem = self.remaining.as_secs().to_string();

        format!(
            r#"{{"end":{}, "remaining": {}, "active": {}}}"#,
            end,
            rem,
            self.timer
                .as_ref()
                .map_or(false, |t| t.is_active.load(Ordering::SeqCst)),
        )
    }
}

trait SizedMessage {
    fn write_sized(&mut self, buf: &[u8]) -> Option<()>;
}
impl SizedMessage for UnixStream {
    fn write_sized(&mut self, buf: &[u8]) -> Option<()> {
        let buf_len = buf.len();
        if buf_len > u32::MAX as usize {
            // return error for size too big
        }
        let buf_len = buf_len as u32;
        self.write_all(&buf_len.to_be_bytes()).ok()?;
        self.write_all(buf).ok()?;

        Some(())
    }
}
