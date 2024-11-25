pub mod abi;
pub mod pb;

pub mod config;

use crate::pb::notification_server::Notification;
use crate::pb::{SendRequest, SendResponse};
pub use abi::*;
pub use config::AppConfig;
use futures::Stream;
use serde::__private::de::Content;
use std::pin::Pin;
use tonic::{async_trait, Request, Response, Status, Streaming};

#[allow(unused)]
pub struct NotificationService {
    config: AppConfig,
}
#[allow(dead_code)]
type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<SendResponse, Status>> + Send>>;
#[async_trait]
impl Notification for NotificationService {
    type SendStream = ResponseStream;

    async fn send(
        &self,
        request: Request<Streaming<SendRequest>>,
    ) -> Result<Response<Self::SendStream>, Status> {
        unimplemented!();
    }
}
