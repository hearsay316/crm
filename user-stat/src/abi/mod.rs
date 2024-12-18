use crate::pb::{QueryRequest, RawQueryRequest, User};
use crate::{ResponseStream, ServiceResult, UserStatsService};
use chrono::{DateTime, TimeZone, Utc};
use itertools::Itertools;
use prost_types::Timestamp;
use tonic::Response;

impl UserStatsService {
    pub async fn query(&self, query: QueryRequest) -> ServiceResult<ResponseStream> {
        let mut sql = "SELECT email ,name FROM user_stats WHERE ".to_string();
        let time_conditions = query
            .timestamps
            .into_iter()
            .map(|(k, v)| timestamp_query(&k, v.lower, v.upper))
            .join(" AND ");

        sql.push_str(&time_conditions);
        let ids_conditions = query
            .ids
            .into_iter()
            .map(|(k, v)| ids_query(&k, v.ids))
            .join(" AND ");
        sql.push_str(" AND ");
        sql.push_str(&ids_conditions);
        println!("Generated SQL: {}", sql);
        self.raw_query(RawQueryRequest { query: sql }).await
    }
    pub async fn raw_query(&self, req: RawQueryRequest) -> ServiceResult<ResponseStream> {
        let Ok(ret) = sqlx::query_as::<_, User>(&req.query)
            .fetch_all(&self.pool)
            .await
        else {
            return Err(tonic::Status::internal(format!(
                "Failed to fetch data :{:?}",
                req.query
            )));
        };
        Ok(Response::new(Box::pin(futures::stream::iter(
            ret.into_iter().map(Ok),
        ))))
    }
}
fn timestamp_query(name: &str, lower: Option<Timestamp>, upper: Option<Timestamp>) -> String {
    if lower.is_none() && upper.is_none() {
        return "TRUE".to_string();
    }
    if lower.is_none() {
        let upper = upper.unwrap();
        let upper = ts_to_utc(upper);

        return format!("{} <= '{}'", name, upper.to_rfc3339());
    }
    if upper.is_none() {
        let lower = lower.unwrap();
        let lower = ts_to_utc(lower);

        return format!("{} >= '{}'", name, lower.to_rfc3339());
    }

    format!(
        "{} BETWEEN '{}' AND '{}'",
        name,
        ts_to_utc(upper.unwrap()).to_rfc3339(),
        ts_to_utc(lower.unwrap()).to_rfc3339()
    )
}
fn ids_query(name: &str, ids: Vec<u32>) -> String {
    if ids.is_empty() {
        return "TRUE".to_string();
    }
    format!("array{:?} <@  {}", ids, name)
}
fn ts_to_utc(ts: Timestamp) -> DateTime<Utc> {
    Utc.timestamp_opt(ts.seconds, ts.nanos as u32).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pb::{IdQuery, QueryRequestBuilder, TimeQuery};
    use crate::AppConfig;
    use anyhow::Result;
    use futures::StreamExt;
    use tracing::info;

    #[tokio::test]
    async fn raw_query_should_work() -> Result<()> {
        let config = AppConfig::load()?;
        let svc = UserStatsService::new(config).await;
        let mut stream = svc
            .raw_query(RawQueryRequest {
                query: "SELECT email ,name FROM user_stats where created_at > '2024-01-01' limit 5"
                    .to_string(),
            })
            .await?
            .into_inner();
        // futures::stream::iter(stream).then(|item| async move{
        //     println!("{:?}",item);
        //     Ok(item)
        // }).try_collect().await?;
        let mut vec = vec![];
        while let Some(res) = stream.next().await {
            println!("{:?}", res);
            vec.push(res?);
        }
        assert_eq!(vec.len(), 5);
        Ok(())
    }
    #[tokio::test]
    async fn query_should_work() -> Result<()> {
        let config = AppConfig::load()?;
        let svc = UserStatsService::new(config).await;
        let query = QueryRequestBuilder::default()
            .timestamp(("created_at".to_string(), tq(Some(120), None)))
            .timestamp(("last_visited_at".to_string(), tq(Some(45), None)))
            .id(("viewed_but_not_started".to_string(), id(&[29789])))
            .build()?;
        info!("{:?}", query);
        let mut stream = svc.query(query).await?.into_inner();
        while let Some(res) = stream.next().await {
            println!("{:?}", res);
        }
        Ok(())
    }
    fn id(id: &[u32]) -> IdQuery {
        IdQuery { ids: id.to_vec() }
    }
    fn tq(lower: Option<i64>, upper: Option<i64>) -> TimeQuery {
        TimeQuery {
            lower: lower.map(to_ts),
            upper: upper.map(to_ts),
        }
    }
    fn to_ts(days: i64) -> Timestamp {
        let dt = Utc::now()
            .checked_sub_signed(chrono::Duration::days(days))
            .unwrap();
        Timestamp {
            seconds: dt.timestamp(),
            nanos: dt.timestamp_subsec_nanos() as i32,
        }
    }
}
