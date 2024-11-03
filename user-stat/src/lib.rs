mod config;
pub mod pb;
mod abi;

use std::ops::Deref;
#[allow(unused_imports)]
use crate::pb::UserBuilder;
use crate::pb::{
    user_stats_server::{UserStats, UserStatsServer},
    QueryRequest, RawQueryRequest, User,
};
pub use config::*;
use futures::Stream;
use sqlx::PgPool;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{Request, Response, Status};

#[allow(dead_code)]
type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>;
#[derive(Debug,Clone)]
pub struct UserStatsService {
    inner: Arc<UserStatsServiceInner>,
}
#[allow(unused)]
#[derive(Debug,Clone)]
pub struct UserStatsServiceInner {
    config: AppConfig,
    pool: PgPool,
}
#[tonic::async_trait]
impl UserStats for UserStatsService {
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

impl From<UserStatsService> for UserStatsServer<UserStatsService> {
    fn from(value: UserStatsService) -> Self {
        UserStatsServer::new(value)
    }
}
impl UserStatsService{
   pub async fn new() ->Self{
       let config = AppConfig::load().unwrap();
       let db = PgPool::connect(&config.server.db_url).await.expect("Failed to connect to db");
       UserStatsService{
           inner: Arc::new(UserStatsServiceInner{
               config,
               pool:db,
           })
       }
   }
   pub fn into_server(self) -> UserStatsServer<UserStatsService> {
        UserStatsServer::new(self)
    }
}
impl Deref for UserStatsService {
    type Target = UserStatsServiceInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}