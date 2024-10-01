use std::env;
use hyper_util::rt::TokioIo;
use tokio::net::UnixStream;
use tower::service_fn;
use tesseract_grpc::cri::runtime_service_client::RuntimeServiceClient;
use tesseract_grpc::cri::StatusRequest;
use tesseract_grpc::tonic::transport::{Endpoint, Uri};
use crate::config::Config;

pub mod config;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let config_path = env::var("TESSERACTD_CONFIG").unwrap_or("/etc/tesseractd/config.toml".to_string());
    let config = Config::init(&config_path)?;
    let channel = Endpoint::try_from("lttp://[::]:50051")?.connect_with_connector(service_fn(move |_: Uri| {
        let uri = config.cri.endpoint.clone();
        async move {
            UnixStream::connect(uri).await.map(TokioIo::new)
        }
    })).await?;
    let mut cri_client = RuntimeServiceClient::new(channel);
    dbg!(cri_client.status(StatusRequest { verbose: true }).await?);
    Ok(())
}
