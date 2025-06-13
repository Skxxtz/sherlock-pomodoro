use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::timer::PomodoroTimer;

pub struct API {
    pub listener: Option<UnixListener>,
    remaining: Option<Duration>,
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
            remaining: None,
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
                if let Some(remaining) = self.remaining {
                    let rem = if let Some(start) = self.start {
                        let since_start = SystemTime::now()
                            .duration_since(start)
                            .unwrap_or(Duration::new(0, 0));
                        remaining.saturating_sub(since_start)
                    } else {
                        remaining
                    };
                    let mut buf = Vec::with_capacity(32);
                    write!(buf, "{:?}", rem).ok()?;
                    let _ = client.write(&buf);
                }
            }
            "show" => {
                let info = self.show_self();
                let _ = client.write(info.as_bytes());
            }
            _ => {}
        };
        Some(())
    }
    fn start(&mut self) {
        if self.timer.is_some() {
            return;
        }

        let remaining = self
            .remaining
            .take()
            .unwrap_or(Duration::from_secs(25 * 60));
        let pomodoro = PomodoroTimer::new(remaining, || {
            Self::on_complete();
        });

        self.remaining = Some(remaining);
        self.timer = Some(pomodoro);
        self.start = Some(SystemTime::now());
    }
    fn stop(&mut self) {
        if let Some(pomodoro) = self.timer.take() {
            if let Some(start) = self.start.take() {
                let target = Duration::from_secs(25 * 60);
                let completed = SystemTime::now()
                    .duration_since(start)
                    .unwrap_or(Duration::from_secs(0));
                let diff = target.saturating_sub(completed);
                self.remaining = Some(diff);
                pomodoro.cancel();
            }
        }
    }
    fn reset(&mut self) {
        if let Some(pomodoro) = self.timer.take() {
            pomodoro.cancel();
        }
        self.start = None;
        self.remaining = None;
    }
    fn on_complete() {
        println!("test");
    }
    fn show_self(&self) -> String {
        let start_json = if let Some(s) = self.start {
            s.duration_since(UNIX_EPOCH).unwrap().as_secs().to_string()
        } else {
            "null".to_string()
        };

        let remaining_json = if let Some(d) = self.remaining {
            if let Some(start) = self.start {
                let elapsed = SystemTime::now().duration_since(start).unwrap_or_default();
                d.saturating_sub(elapsed).as_secs().to_string()
            } else {
                d.as_secs().to_string()
            }
        } else {
            "null".to_string()
        };

        format!(
            r#"{{"start":{},"remaining":{}}}"#,
            start_json, remaining_json
        )
    }
}
