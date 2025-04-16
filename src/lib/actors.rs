// src/lib/actors.rs

// dependencies
use tokio::spawn;
use tokio::sync::mpsc::{self, Sender};
use tokio::task::JoinHandle;

// struct type to represent an actor that counts incoming pings
pub struct PingCounterActor;

impl PingCounterActor {
    pub fn start() -> (Sender<()>, JoinHandle<()>) {
        let (tx, mut rx) = mpsc::channel(32);
        let handle = spawn(async move {
            let mut count = 0;
            while (rx.recv().await).is_some() {
                count += 1;
                println!("Received Ping #{}", count);
            }
        });

        (tx, handle)
    }
}
