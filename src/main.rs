// src/main.rs

// dependencies
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use tokio::net::TcpListener;
use tokio::signal;
use tokio::sync::broadcast;

// Customize this struct with things from `shuttle_main` needed in `bind`,
// such as secrets or database connections
struct HyperService {}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for HyperService {
    async fn bind(self, addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        let listener = TcpListener::bind(addr).await?;

        let (shutdown_tx, mut shutdown_rx) = broadcast::channel(1);

        tokio::spawn(async move {
            signal::ctrl_c()
                .await
                .expect("Failed to install Ctrl-C handler");
            let _ = shutdown_tx.send(());
        });

        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    println!("Shutdown signal received. Stopping server.");
                    break;
                }
                Ok((stream,  _)) = listener.accept() => {
                    let io = TokioIo::new(stream);
                    tokio::task::spawn(async move {
                        if let Err(err) = http1::Builder::new()
                            .serve_connection(io, service_fn(hello))
                            .await
                        {
                            eprintln!("Error serving connection: {:?}", err);
                        }
                    });
                }
            }
        }
        Ok(())
    }
}

// handler function, returns the message Hello, World!
async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

// main function
#[shuttle_runtime::main]
async fn shuttle_main() -> Result<HyperService, shuttle_runtime::Error> {
    Ok(HyperService {})
}
