pub mod abi;
pub mod config;
pub mod pb;

use crate::config::AppConfig;
use crate::pb::metadata_server::{Metadata, MetadataServer};
use crate::pb::{Content, MaterializeRequest};
use futures::Stream;
use std::pin::Pin;
use tonic::{async_trait, Request, Response, Status, Streaming};

#[allow(dead_code)]
type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<Content, Status>> + Send>>;
#[allow(unused)]
pub struct MetaDataService {
    config: AppConfig,
}
#[async_trait]
impl Metadata for MetaDataService {
    type MaterializeStream = ResponseStream;

    async fn materialize(
        &self,
        request: Request<Streaming<MaterializeRequest>>,
    ) -> ServiceResult<Self::MaterializeStream> {
        let query = request.into_inner();
        self.materialize(query).await?;
        todo!()
    }
}
impl MetaDataService {
    pub fn new(config: AppConfig) -> Self {
        MetaDataService { config }
    }
    pub fn into_server(self) -> MetadataServer<Self> {
        MetadataServer::new(self)
    }
}
