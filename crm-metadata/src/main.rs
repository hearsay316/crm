use crm_metadata::config::AppConfig;
use crm_metadata::MetaDataService;
use tonic::transport::Server;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer as _;
// 坐标
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
    info!("Hello, world!");
    let config = AppConfig::load()?;
    let addr = config.server.port;
    let addr = format!("[::1]:{}", addr).parse()?;
    let svc = MetaDataService::new(config).into_server();
    info!("UserServiceServer  listening on {}", addr);
    Server::builder().add_service(svc).serve(addr).await?;
    Ok(())
}
