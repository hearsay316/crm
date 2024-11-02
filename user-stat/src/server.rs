use futures::Stream;
use std::pin::Pin;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use user_stat::pb::{
    user_stats_server::{UserStats, UserStatsServer as UserStatsServers},
    User,
};
#[allow(unused_imports)]
use user_stat::pb::{QueryRequest, RawQueryRequest, UserBuilder};
#[allow(dead_code)]
type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>;
#[derive(Debug, Default)]
pub struct UserStatsServer {}
#[tonic::async_trait]
impl UserStats for UserStatsServer {
    type QueryStream = ResponseStream;
    #[allow(unused_variables)]
    async fn query(
        &self,
        request: Request<QueryRequest>,
    ) -> Result<Response<Self::QueryStream>, Status> {
        todo!()
    }

    type RawQueryStream = ResponseStream;
    #[allow(unused_variables)]
    async fn raw_query(
        &self,
        request: Request<RawQueryRequest>,
    ) -> Result<Response<Self::RawQueryStream>, Status> {
        todo!()
    }
}
// include!(concat!(env!("OUT_DIR"), "/crm.rs"));
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "[::1]:50051".parse()?;
    let svc = UserStatsServer::default();
    println!("UserServiceServer  listening on {}", addr);
    Server::builder()
        .add_service(UserStatsServers::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
