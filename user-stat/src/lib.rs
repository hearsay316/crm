mod abi;
mod config;
pub mod pb;

#[allow(unused_imports)]
use crate::pb::UserBuilder;
use crate::pb::{
    user_stats_server::{UserStats, UserStatsServer},
    QueryRequest, RawQueryRequest, User,
};
pub use config::*;
use futures::Stream;
use sqlx::PgPool;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{Request, Response, Status};

#[allow(dead_code)]
type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>;
#[derive(Debug, Clone)]
pub struct UserStatsService {
    inner: Arc<UserStatsServiceInner>,
}
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct UserStatsServiceInner {
    config: AppConfig,
    pool: PgPool,
}
#[tonic::async_trait]
impl UserStats for UserStatsService {
    type QueryStream = ResponseStream;
    #[allow(unused_variables)]
    async fn query(&self, request: Request<QueryRequest>) -> ServiceResult<Self::QueryStream> {
        let query = request.into_inner();
        self.query(query).await
    }

    type RawQueryStream = ResponseStream;
    #[allow(unused_variables)]
    async fn raw_query(
        &self,
        request: Request<RawQueryRequest>,
    ) -> ServiceResult<Self::RawQueryStream> {
        let query = request.into_inner();
        self.raw_query(query).await
    }
}

impl From<UserStatsService> for UserStatsServer<UserStatsService> {
    fn from(value: UserStatsService) -> Self {
        UserStatsServer::new(value)
    }
}
impl UserStatsService {
    pub async fn new(config: AppConfig) -> Self {
        // let config = AppConfig::load().unwrap();
        let db = PgPool::connect(&config.server.db_url)
            .await
            .expect("Failed to connect to db");
        UserStatsService {
            inner: Arc::new(UserStatsServiceInner { config, pool: db }),
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
