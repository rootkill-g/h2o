mod tokio_executor;
mod tokio_io;

use std::net::SocketAddr;

use http_body_util::Full;
use hyper::{
    Request, Response,
    body::{Bytes, Incoming},
    server::conn::http2,
    service::service_fn,
};
use tokio::net::TcpListener;
use tokio_executor::TokioExecutor;
use tokio_io::TokioIo;

async fn index(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error> {
    println!("Incoming: {:?}", req);
    let body = Full::from(Bytes::from("Hello HTTP/2"));
    Ok(Response::new(body))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            let service = service_fn(index);
            let io = TokioIo::new(stream);
            // Create an HTTP/2 connection
            let conn = http2::Builder::new(TokioExecutor).serve_connection(io, service);

            if let Err(err) = conn.await {
                eprintln!("Error service connection: {:?}", err);
            } else {
                println!("Connection served successfully");
            }
        });
    }
}
