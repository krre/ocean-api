use super::router;
use crate::config;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use log::info;
use tokio::net::TcpListener;

pub struct ApiServer;

impl ApiServer {
    pub fn new() -> Self {
        Self
    }

    pub async fn listen(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let addr = config::CONFIG.server_addr.clone();

        let listener = TcpListener::bind(&addr).await?;
        info!("listening on {addr}");

        loop {
            let (stream, _) = listener.accept().await?;

            tokio::task::spawn(async move {
                let io = TokioIo::new(stream);

                if let Err(err) = http1::Builder::new()
                    .serve_connection(io, service_fn(router::route))
                    .await
                {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    }
}

impl Default for ApiServer {
    fn default() -> Self {
        Self::new()
    }
}
