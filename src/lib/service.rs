// src/lib/service.rs

// dependencies
use crate::routes::router;
use crate::utilities::shutdown_signal;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use hyper_util::server::graceful::GracefulShutdown;
use shuttle_runtime::Service;
use std::net::SocketAddr;
use std::pin::pin;
use tokio::net::TcpListener;
use tokio::sync::mpsc::Sender;

// Customize this struct with things from `shuttle_main` needed in `bind`,
// such as secrets or database connections
pub struct HyperService {
    pub ping_tx: Sender<()>,
}

#[shuttle_runtime::async_trait]
impl Service for HyperService {
    async fn bind(self, addr: SocketAddr) -> Result<(), shuttle_runtime::Error> {
        // set up a listener, using the Shuttle provided address
        let listener = TcpListener::bind(addr).await?;

        // create a new http instance
        let http = http1::Builder::new();

        // create a new instance of GracefulShutdown
        let graceful = GracefulShutdown::new();

        // pin the shutdown_signal function in memory
        let mut signal = pin!(shutdown_signal());

        // the main loop, listen for incoming connections and serve the router to respond to
        // incoming requests
        loop {
            tokio::select! {
                Ok((stream, _)) = listener.accept() => {
                    let io = TokioIo::new(stream);
                    let ping_tx_clone = self.ping_tx.clone();
                    let conn = http.serve_connection(io, service_fn(move |req| {
                        let tx = ping_tx_clone.clone();
                        async move { router(req, tx).await }
                    }));
                    let fut = graceful.watch(conn);
                    tokio::spawn(async move {
                        if let Err(e) = fut.await {
                            eprintln!("Error serving conection: {:?}", e);
                        }
                    });
                },

                _ = &mut signal => {
                    eprintln!("Graceful shutdown signal received...");
                    break Ok(());
                }
            }
        }
    }
}
