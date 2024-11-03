use tonic::transport::Server;
use tracing::info;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::Layer as _;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use user_stat::UserStatsService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
    info!("Hello, world!");
    let addr = "[::1]:50051".parse()?;
    let svc = UserStatsService::new().await.into_server();
    println!("UserServiceServer  listening on {}", addr);
    Server::builder().add_service(svc).serve(addr).await?;
    Ok(())
}
