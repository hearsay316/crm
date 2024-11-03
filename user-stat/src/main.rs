use tonic::transport::Server;
use user_stat::pb::user_stats_server::UserStatsServer;
use user_stat::UserStatsService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let addr = "[::1]:50051".parse()?;
    let svc =UserStatsService::new().await.into_server();
    println!("UserServiceServer  listening on {}", addr);
    Server::builder()
        .add_service(svc)
        .serve(addr)
        .await?;
    Ok(())
}
