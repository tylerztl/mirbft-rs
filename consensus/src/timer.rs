use crossbeam::crossbeam_channel::Sender;
use std::time::Duration;
use std::thread;
use logger::prelude::*;

pub struct TimeoutInfo {}

pub struct BatchTimer();

impl BatchTimer {
    pub fn start(s: Sender<TimeoutInfo>, interval: u64) {
        info!("Batch timer started.");

        thread::spawn(move || {
            loop {
                let timeout = Duration::from_millis(interval);
                thread::sleep(timeout);
                s.send(TimeoutInfo {}).unwrap();
            }
        });
    }
}

