#[allow(non_snake_case, clippy::all, unused_imports, dead_code)]
#[rustfmt::skip]
#[path = "gen/data_api.v1.rs"]
mod data_api;
pub mod config;

use std::str::FromStr;
use tracing::info;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub struct DataServer {}

#[tonic::async_trait]
impl data_api::data_service_server::DataService for DataServer {
    #[tracing::instrument(skip_all)]
    async fn insert(
        &self,
        _req: tonic::Request<data_api::InsertRequest>,
    ) -> Result<tonic::Response<data_api::InsertResponse>, tonic::Status> {
        unimplemented!()
    }

    #[tracing::instrument(skip_all)]
    async fn get(
        &self,
        _req: tonic::Request<data_api::GetRequest>,
    ) -> Result<tonic::Response<data_api::GetResponse>, tonic::Status> {
        unimplemented!()
    }
}

impl DataServer {
    pub async fn start(config: config::DataServiceConfig) -> Result<(), Error> {
        let filter_layer = tracing_subscriber::filter::Targets::from_str(
            std::env::var("RUST_LOG").as_deref().unwrap_or("info"),
        )
        .unwrap();
        let format_layer = tracing_subscriber::fmt::layer();
        tracing_subscriber::registry()
            .with(filter_layer)
            .with(format_layer)
            .init();

        let address: std::net::SocketAddr = format!("[::]:{}", config.port).parse()?;

        info!("Starting data server at {:?}", address);
        tonic::transport::Server::builder()
            .trace_fn(|_| tracing::error_span!("data_server"))
            .http2_keepalive_interval(Some(config::DEFAULT_KEEPALIVE))
            .tcp_keepalive(Some(config::DEFAULT_KEEPALIVE))
            .tcp_nodelay(true)
            .add_service(data_api::data_service_server::DataServiceServer::new(
                DataServer {},
            ))
            .serve(address)
            .await?;

        Ok(())
    }
}
