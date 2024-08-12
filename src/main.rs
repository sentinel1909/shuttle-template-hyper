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

// Customize this struct with things from `shuttle_main` needed in `bind`,
// such as secrets or database connections
struct HyperService {}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for HyperService {
    async fn bind(self, addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        let listener = TcpListener::bind(addr).await?;

        let http = http1::Builder::new();

        let graceful = hyper_util::server::graceful::GracefulShutdown::new();

        let mut signal = std::pin::pin!(shutdown_signal());

        loop {
            tokio::select! {
                Ok((stream, _addr)) = listener.accept() => {
                    let io = TokioIo::new(stream);
                    let conn = http.serve_connection(io, service_fn(health_check));
                    let fut = graceful.watch(conn);
                    tokio::spawn(async move {
                        if let Err(e) = fut.await {
                            eprintln!("Error serving conection: {:?}", e);
                        }
                    });
                },

                _ = &mut signal => {
                    eprintln!("Graceful shtudown signal received...");
                    break Ok(());
                }
            }
        }
            
    }
}

// function which provides a graceful shutdown signal
async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Failed to install CTRL-C signal handler");
}

// handler function, returns a 200 OK response and empty body
async fn health_check(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::new())))
}

// main function
#[shuttle_runtime::main]
async fn shuttle_main() -> Result<HyperService, shuttle_runtime::Error> {
    Ok(HyperService {})
}
