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
    /// 处理内容物化请求的异步方法
    ///
    /// # 参数
    /// * `stream` - 一个包含 MaterializeRequest 的输入流
    ///
    /// # 返回值
    /// 返回一个 Result，包含一个 ResponseStream
    ///
    /// # 功能
    /// 此方法接收一个包含 MaterializeRequest 的流，为每个请求生成对应的 Content，
    /// 并将生成的 Content 通过一个新的流返回。
    pub async fn materialize(
        &self,
        mut stream: impl Stream<Item = Result<MaterializeRequest, tonic::Status>>
            + Send
            + 'static
            + Unpin,
    ) -> ServiceResult<ResponseStream> {
        // 创建一个 mpsc 通道，用于发送生成的 Content
        let (mut tx, rx) = mpsc::channel(CHANNEL_SIZE);

        // 在新的异步任务中处理输入流
        tokio::spawn(async move {
            // 循环处理输入流中的每个请求
            while let Some(Ok(req)) = stream.next().await {
                // 从请求中获取 id
                let id = req.id;
                // 使用 id 生成新的 Content
                let content = Content::materialize(id);
                // 将生成的 Content 发送到通道
                tx.send(Ok(content)).await.unwrap();
            }
        });

        // 将接收端包装成 ReceiverStream
        let stream = ReceiverStream::new(rx);
        // 将 stream 包装在 Response 中并返回
        Ok(Response::new(Box::pin(stream)))
    }
}

impl Content {
    /// 根据给定的 ID 生成一个新的 Content 实例
    ///
    /// # 参数
    /// * `id` - 要生成的 Content 的唯一标识符
    ///
    /// # 功能
    /// 此方法使用随机生成的数据创建一个新的 Content 实例，包括名称、描述、发布者等信息。
    pub fn materialize(id: u32) -> Self {
        // 创建一个随机数生成器
        let mut rng = rand::thread_rng();

        // 创建并返回一个新的 Content 实例
        Content {
            id,  // 使用传入的 id
            name: Name().fake(),  // 生成随机名称
            description: Sentence(3..7).fake(),  // 生成随机描述，3-7个句子
            publishers: (1..rng.gen_range(2..10))  // 生成1-9个随机发布者
                .map(|_| Publisher::new())
                .collect(),
            url: "https://placehold.co/400*400".to_string(),  // 设置固定的URL
            image: "https://placehold.co/1600*900".to_string(),  // 设置固定的图片URL
            r#type: Faker.fake(),  // 生成随机类型
            last_visited_at: created_at(),  // 生成随机的最后访问时间
            views: rng.gen_range(123432..10000000),  // 生成随机的浏览量
            likes: rng.gen_range(1234..100000),  // 生成随机的点赞数
            dislikes: rng.gen_range(123..100000),  // 生成随机的踩数
        }
    }
}

impl Publisher {
    /// 创建一个新的 Publisher 实例
    ///
    /// # 返回值
    /// 返回一个新的 Publisher 实例
    ///
    /// # 功能
    /// 此方法生成一个新的 Publisher 实例，包括随机生成的 ID、名称和头像 URL。
    pub fn new() -> Self {
        Publisher {
            id: (10000..2000000).fake(),  // 生成一个随机的 ID，范围在 10000 到 2000000 之间
            name: Name().fake(),  // 生成一个随机的名称
            avatar: "https://placehold.co/400*400".to_string(),  // 设置一个固定的头像 URL
        }
    }
}

/// 计算指定天数之前的日期时间
///
/// # 参数
/// * `days` - 要回溯的天数
///
/// # 返回值
/// 返回一个 DateTime<Utc> 实例，表示当前时间减去指定天数后的日期时间
fn before(days: u64) -> DateTime<Utc> {
    // 从当前时间减去指定的天数
    Utc::now().checked_sub_days(Days::new(days)).unwrap()
}

/// 获取当前的 UTC 日期时间
///
/// # 返回值
/// 返回一个 DateTime<Utc> 实例，表示当前的 UTC 日期时间
fn now() -> DateTime<Utc> {
    Utc::now()  // 返回当前的 UTC 时间
}

/// 生成一个随机的创建时间
///
/// # 返回值
/// 返回一个 Option<Timestamp>，包含一个随机生成的过去一年内的时间戳
fn created_at() -> Option<Timestamp> {
    // 生成一个在过去一年内的随机日期时间
    let data: DateTime<Utc> = DateTimeBetween(before(365), before(0)).fake();
    Some(Timestamp {
        seconds: data.timestamp(),  // 获取时间戳的秒数
        nanos: data.timestamp_subsec_nanos() as i32,  // 获取时间戳的纳秒部分
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
