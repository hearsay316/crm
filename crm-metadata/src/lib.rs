pub mod config;
pub mod pb;
pub mod abi;

use std::pin::Pin;
use futures::Stream;
use tonic::{async_trait, Request, Response, Status, Streaming};
use crate::config::AppConfig;
use crate::pb::{Content, MaterializeRequest};
use crate::pb::metadata_server::{Metadata, MetadataServer};

#[allow(dead_code)]
type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<Content, Status>> + Send>>;
#[allow(unused)]
pub struct MetaDataService{
    config:AppConfig
}
#[async_trait]
impl Metadata for MetaDataService{
    type MaterializeStream = ResponseStream;

    async fn materialize(&self, request: Request<Streaming<MaterializeRequest>>) -> ServiceResult<Self::MaterializeStream> {
        let query = request.into_inner();
        self.materialize(query).await?;
        todo!()
    }
}
impl MetaDataService{
    pub fn new(config:AppConfig)->Self{
        MetaDataService{
            config
        }
    }
    pub fn  into_server(self)->MetadataServer<Self>{
        MetadataServer::new(self)
    }
}