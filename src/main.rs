use clap::Parser;

mod server;

#[tokio::main]
async fn main() {
    let config = server::config::DataServiceConfig::parse();

    server::DataServer::start(config).await.unwrap();
    println!("Hello, world!");
}
