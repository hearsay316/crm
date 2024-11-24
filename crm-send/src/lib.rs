pub mod pb;
pub mod abi;

pub mod config;

use std::pin::Pin;
use futures::Stream;
use serde::__private::de::Content;
use tonic::{async_trait, Request, Response, Status, Streaming};
pub use config::AppConfig;
pub use abi::*;
use crate::pb::notification_server::Notification;
use crate::pb::{SendRequest, SendResponse};

#[allow(unused)]
pub struct NotificationService {
    config: AppConfig,
}
#[allow(dead_code)]
type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<SendResponse, Status>> + Send>>;
#[async_trait]
impl Notification for NotificationService{
    type SendStream = ResponseStream;

    async fn send(&self, request: Request<Streaming<SendRequest>>) -> Result<Response<Self::SendStream>, Status> {
        unimplemented!();
    }
}