// tests/api/helpers.rs

// dependencies
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use shuttle_hyper_template_lib::routes::router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

// start a test server and return the address it is listening on
pub async fn start_test_server() -> SocketAddr {
    let address = "127.0.0.1";
    let port: u16 = 0;
    let socket = format!("{}:{}", address, port);
    let listener = TcpListener::bind(socket).await.unwrap();
    let addr = listener
        .local_addr()
        .expect("Unable to obtain the address the test server is using");

    tokio::spawn(async move {
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let io = TokioIo::new(stream);
            tokio::spawn(async move {
                if let Err(e) = http1::Builder::new()
                    .serve_connection(io, service_fn(router))
                    .await
                {
                    eprintln!("Error serving connection: {}", e);
                }
            });
        }
    });

    addr
}
