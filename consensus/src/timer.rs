use crossbeam::crossbeam_channel::{unbounded, Receiver, Sender, RecvError};
use std::time::Duration;
use std::thread;

pub struct TimeoutInfo {}

pub struct BatchTimer {
    interval: u64,
}

impl BatchTimer {
    pub fn new(interval: u64) -> Self {
        BatchTimer {
            interval,
        }
    }

    pub fn start(interval: u64) {
        let (time_sender, time_receiver) = unbounded();
        let timer = Self::new(interval);

        thread::spawn(move || {
            loop {
                let timeout = Duration::from_millis(interval);
                thread::sleep(timeout);
                time_sender.send(TimeoutInfo {}).unwrap();
            }
        });

        thread::spawn(move || {
            loop {
                let mut timeout_msg = Err(RecvError);
                select! {
                    recv(time_receiver) -> msg => timeout_msg = msg,
                }

                if let Ok(_msg) = timeout_msg {
                    timer.propose();
                }
            }
        });
    }

    pub fn propose(&self) {}
}

