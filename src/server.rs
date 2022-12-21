#[allow(non_snake_case, clippy::all, unused_imports, dead_code)]
#[rustfmt::skip]
#[path = "gen/data_api.v1.rs"]
mod data_api;
pub mod config;
mod peer_manager;

use std::str::FromStr;
use tracing::info;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub struct DataServer<T>
where
    T: super::consensus_module::PeerClient + 'static,
{
    cm: super::consensus_module::ConsensusModule<T>,
    storage: tokio::sync::RwLock<std::collections::HashMap<String, String>>,
}

#[tonic::async_trait]
impl<T> data_api::data_service_server::DataService for DataServer<T>
where
    T: super::consensus_module::PeerClient + 'static,
{
    #[tracing::instrument(skip_all)]
    async fn insert(
        &self,
        _req: tonic::Request<data_api::InsertRequest>,
    ) -> Result<tonic::Response<data_api::InsertResponse>, tonic::Status> {
        // add the entry
        // propagate it to the rest of the replica sets
        // wait for the propagation to complete
        // insert it into the collection
        unimplemented!()
    }

    #[tracing::instrument(skip_all)]
    async fn get(
        &self,
        req: tonic::Request<data_api::GetRequest>,
    ) -> Result<tonic::Response<data_api::GetResponse>, tonic::Status> {
        let key = req.into_inner().key;
        let storage = self.storage.read().await;

        let value = storage.get(&key).ok_or(tonic::Status::invalid_argument(
            "No value associated with that key.",
        ))?;

        Ok(tonic::Response::new(data_api::GetResponse {
            value: value.to_string(),
        }))
    }

    #[tracing::instrument(skip_all)]
    async fn request_vote(
        &self,
        _req: tonic::Request<data_api::RequestVoteRequest>,
    ) -> Result<tonic::Response<data_api::RequestVoteResponse>, tonic::Status> {
        // add the entry
        // propagate it to the rest of the replica sets
        // wait for the propagation to complete
        // insert it into the collection
        unimplemented!()
    }

    #[tracing::instrument(skip_all)]
    async fn append_entries(
        &self,
        _req: tonic::Request<data_api::AppendEntriesRequest>,
    ) -> Result<tonic::Response<data_api::AppendEntriesResponse>, tonic::Status> {
        // add the entry
        // propagate it to the rest of the replica sets
        // wait for the propagation to complete
        // insert it into the collection
        unimplemented!()
    }
}

impl DataServer<peer_manager::PeerManager> {
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

        let peer_manager = peer_manager::PeerManager::new(config.peers);

        let consensus_manager = super::consensus_module::ConsensusModule::new(peer_manager);

        info!("Starting data server at {:?}", address);
        tonic::transport::Server::builder()
            .trace_fn(|_| tracing::error_span!("data_server"))
            .http2_keepalive_interval(Some(config::DEFAULT_KEEPALIVE))
            .tcp_keepalive(Some(config::DEFAULT_KEEPALIVE))
            .tcp_nodelay(true)
            .add_service(data_api::data_service_server::DataServiceServer::new(
                DataServer {
                    cm: consensus_manager,
                    storage: tokio::sync::RwLock::new(std::collections::HashMap::new()),
                },
            ))
            .serve(address)
            .await?;

        Ok(())
    }
}
