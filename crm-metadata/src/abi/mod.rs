use crate::pb::{Content, MaterializeRequest, Publisher};
use crate::{MetaDataService, ResponseStream, ServiceResult};
use chrono::{DateTime, Days, Utc};
use fake::faker::chrono::zh_cn::DateTimeBetween;
use fake::faker::lorem::zh_cn::Sentence;
use fake::faker::name::zh_cn::Name;
use fake::{Fake, Faker};
use futures::{SinkExt, Stream};
use prost_types::Timestamp;
use rand::Rng;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::codegen::tokio_stream::StreamExt;
use tonic::Response;
const CHANNEL_SIZE: usize = 1024;
impl MetaDataService {
    pub async fn materialize(
        &self,
        mut stream: impl Stream<Item = Result<MaterializeRequest, tonic::Status>>
            + Send
            + 'static
            + Unpin,
    ) -> ServiceResult<ResponseStream> {
        let (mut tx, rx) = mpsc::channel(CHANNEL_SIZE);
        tokio::spawn(async move {
            while let Some(Ok(req)) = stream.next().await {
                let id = req.id;
                let content = Content::materialize(id);
                tx.send(Ok(content)).await.unwrap();
            }
        });
        let stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(stream)))
    }
}
impl Content {
    pub fn materialize(id: u32) -> Self {
        let mut rng = rand::thread_rng();
        Content {
            id,
            name: Name().fake(),
            description: Sentence(3..7).fake(),
            publishers: (1..rng.gen_range(2..10))
                .map(|_| Publisher::new())
                .collect(),
            url: "https://placehold.co/400*400".to_string(),
            image: "https://placehold.co/1600*900".to_string(),
            r#type: Faker.fake(),
            last_visited_at: created_at(),
            views: rng.gen_range(123432..10000000),
            likes: rng.gen_range(1234..100000),
            dislikes: rng.gen_range(123..100000),
        }
    }
}
impl Publisher {
    pub fn new() -> Self {
        Publisher {
            id: (10000..2000000).fake(),
            name: Name().fake(),
            avatar: "https://placehold.co/400*400".to_string(),
        }
    }
}
fn before(days: u64) -> DateTime<Utc> {
    Utc::now().checked_sub_days(Days::new(days)).unwrap()
}
fn now() -> DateTime<Utc> {
    Utc::now()
}

fn created_at() -> Option<Timestamp> {
    let data: DateTime<Utc> = DateTimeBetween(before(365), before(0)).fake();
    Some(Timestamp {
        seconds: data.timestamp(),
        nanos: data.timestamp_subsec_nanos() as i32,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    use futures::StreamExt;
    #[tokio::test]
    async fn test_materialize_should_work() -> anyhow::Result<()> {
        let config = AppConfig::load()?;
        let server = MetaDataService::new(config);
        let request = tokio_stream::iter(vec![
            Ok(MaterializeRequest { id: 1 }),
            Ok(MaterializeRequest { id: 2 }),
            Ok(MaterializeRequest { id: 3 }),
        ]);
        let response = server.materialize(request).await?;
        let ret = response.into_inner().collect::<Vec<_>>().await;
        println!("生成数据是: {:?}", ret);
        assert_eq!(ret.len(), 3);
        Ok(())
    }
}
