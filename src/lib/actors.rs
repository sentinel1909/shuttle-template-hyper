// src/lib/actors.rs

// dependencies
use tokio::spawn;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::task::JoinHandle;

// enum type to represent the PingCounterActors capabilities
pub enum PingMessage {
    Ping,
}

// struct type to represent an actor that counts incoming pings
pub struct PingCounterActor {
    rx: Receiver<PingMessage>,
    count: usize,
}

impl PingCounterActor {
    pub fn start() -> (Sender<PingMessage>, JoinHandle<()>) {
       let (tx, rx) = mpsc::channel(32);

       let actor = PingCounterActor {
        rx, 
        count: 0,
       };

       let handle = spawn(async move {
        actor.run().await;
       });

       (tx, handle)
    }

    async fn run(mut self) {
        while let Some(msg) = self.rx.recv().await {
            match msg {
                PingMessage::Ping => {
                    self.count += 1;
                    tracing::info!("Ping #{}", self.count);
                }
            }
        }

        tracing::info!("PingCounterActor shutting donw.");
    }
}
