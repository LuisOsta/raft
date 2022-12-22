pub const DEFAULT_PORT: &str = "5050";
pub const DEFAULT_KEEPALIVE: std::time::Duration = std::time::Duration::from_secs(30);

#[derive(clap::Parser, Debug, Clone)]
pub struct DataServiceConfig {
    /// Listen address
    #[clap(short, long, default_value = DEFAULT_PORT)]
    pub port: String,

    #[clap(long)]
    pub peers: Vec<String>,
}
