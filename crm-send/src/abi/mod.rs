use crate::pb::notification_server::NotificationServer;
use crate::pb::{EmailMessage, SendRequest, SendResponse};
use crate::{AppConfig, NotificationService, ResponseStream};
use chrono::Utc;
use futures::StreamExt;
use prost_types::Timestamp;
use tokio::sync::mpsc;
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;
use tonic::codegen::tokio_stream::Stream;
use tonic::Status;

const CHANNEL_SIZE: usize = 1024;

pub trait Sender {
    async fn send(self) -> Result<SendResponse, Status>;
}
impl NotificationService {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }
    pub fn into_server(self) -> NotificationServer<Self> {
        NotificationServer::new(self)
    }
    pub fn send(
        &self,
        mut stream: impl Stream<Item = Result<SendRequest, Status>> + Send + 'static + Unpin,
    ) -> ResponseStream {
        let (mut tx, rx) = mpsc::channel(CHANNEL_SIZE);
        tokio::spawn(async move {
            while let Some(Ok(req)) = stream.next().await {
                let id = match req.msg {
                    Some(msg) => msg.id,
                    None => 0,
                };
                let content = Content::materialize(id);
                tx.send(Ok(content)).await.unwrap();
            }
        });
        let stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(stream)))
    }
}
impl Sender for EmailMessage {
    async fn send(self) -> Result<SendResponse, Status> {
        Ok(SendResponse {
            message_id: self.message_id,
            timestamp: Some(to_ts()),
        })
    }
}
fn to_ts() -> Timestamp {
    let now = Utc::now();
    Timestamp {
        seconds: now.timestamp(),
        nanos: now.timestamp_subsec_nanos() as i32,
    }
}
