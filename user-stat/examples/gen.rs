// create table user_stats
// (
// email                    varchar(64) NOT NULL PRIMARY KEY,
// name                     varchar(64) NOT NULL,
// created_at               timestamptz DEFAULT CURRENT_TIMESTAMP,
// last_visited_at          timestamptz NOT NULL,
// last_watched_at          timestamptz NOT NULL,
// recent_watched           int[],
// viewed_but_not_started   int[],
// started_but_not_finished int[],
// finished                 int[],
// last_email_notification  timestamptz NOT NULL,
// last_in_app_notification timestamptz NOT NULL,
// last_sms_notification    timestamptz NOT NULL
// );

use chrono::{DateTime, Days, Utc};
use fake::faker::chrono::zh_cn::DateTimeBetween;
use fake::{
    faker::{internet::en::SafeEmail, name::zh_cn::Name},
    Dummy, Fake, Faker,
};
use nanoid::nanoid;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, PgPool};
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use tokio::time::Instant;

#[derive(Clone, Dummy, Serialize, Deserialize, Eq, PartialEq)]
enum Gender {
    Female,
    Male,
    Unknown,
}
impl Debug for Gender {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                Gender::Female => "female",
                Gender::Male => "male",
                Gender::Unknown => "unknown",
            },
        )
    }
}

#[derive(Debug, Dummy, Serialize, Deserialize, PartialEq, Eq)]
struct UserStat {
    #[dummy(faker = "UniqueEmail")]
    email: String,
    #[dummy(faker = "Name()")]
    name: String,
    gender: Gender,
    #[dummy(faker = "DateTimeBetween(before(365*3), before(90))")]
    created_at: DateTime<Utc>,
    #[dummy(faker = "DateTimeBetween(before(30), now())")]
    last_visited_at: DateTime<Utc>,
    #[dummy(faker = "DateTimeBetween(before(90), now())")]
    last_watched_at: DateTime<Utc>,
    #[dummy(faker = "IntList(50,10000,10000)")]
    recent_watched: Vec<i32>,
    #[dummy(faker = "IntList(50,20000,10000)")]
    viewed_but_not_started: Vec<i32>,
    #[dummy(faker = "IntList(50,30000,10000)")]
    started_but_not_finished: Vec<i32>,
    #[dummy(faker = "IntList(50,40000,10000)")]
    finished: Vec<i32>,
    #[dummy(faker = "DateTimeBetween(before(45), now())")]
    last_email_notification: DateTime<Utc>,
    #[dummy(faker = "DateTimeBetween(before(15), now())")]
    last_in_app_notification: DateTime<Utc>,
    #[dummy(faker = "DateTimeBetween(before(90), now())")]
    last_sms_notification: DateTime<Utc>,
}

//DateTime<Local>

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPool::connect("postgres://postgres:123321@localhost:5432/stats")
        .await
        .expect("这份是错误");
    // DateTimeBefore()
    let user: UserStat = Faker.fake();
    for i in 1..=500 {
        let users: HashSet<_> = (0..10000).map(|_| Faker.fake::<UserStat>()).collect();
        let start = Instant::now();
        raw_insert(users, &pool).await?;
        println!("正在插入数据,  当前进度 {}", i);
        println!("耗时: {:?}", start.elapsed());
    }
    println!("{:?}", user);
    Ok(())
}

async fn raw_insert(users: HashSet<UserStat>, pool: &PgPool) -> anyhow::Result<()> {
    let mut sql = String::with_capacity(10 * 1000 * 1000);
    sql.push_str(
        "INSERT INTO user_stats(email, name, gender, created_at, last_visited_at,
            last_watched_at, recent_watched, viewed_but_not_started,
            started_but_not_finished, finished, last_email_notification,
            last_in_app_notification, last_sms_notification) VALUES",
    );
    for user in users {
        sql.push_str(&format!(
            "('{}', '{}','{:?}', '{}', '{}', '{}', {}::int[], {}::int[], {}::int[], {}::int[], '{}', '{}', '{}'),",
            user.email,
            user.name,
            user.gender,
            user.created_at,
            user.last_visited_at,
            user.last_watched_at,
            list_to_string(user.recent_watched),
            list_to_string(user.viewed_but_not_started),
            list_to_string(user.started_but_not_finished),
            list_to_string(user.finished),
            user.last_email_notification,
            user.last_in_app_notification,
            user.last_sms_notification,
        ));
    }
    let v = &sql[..sql.len() - 1];
    sqlx::query(v).execute(pool).await?;
    Ok(())
}
fn list_to_string(list: Vec<i32>) -> String {
    format!("ARRAY{:?}", list)
}
#[allow(dead_code)]
async fn bulk_insert(users: HashSet<UserStat>, pool: &PgPool) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    for user in users {
        let query = sqlx::query(
            r#"
            INSERT INTO user_stats
            (email, name, created_at, last_visited_at,
            last_watched_at, recent_watched, viewed_but_not_started,
            started_but_not_finished, finished, last_email_notification,
            last_in_app_notification, last_sms_notification)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
        )
        .bind(user.email)
        .bind(user.name)
        .bind(user.created_at)
        .bind(user.last_visited_at)
        .bind(user.last_watched_at)
        .bind(user.recent_watched)
        .bind(user.viewed_but_not_started)
        .bind(user.started_but_not_finished)
        .bind(user.finished)
        .bind(user.last_email_notification)
        .bind(user.last_in_app_notification)
        .bind(user.last_sms_notification);
        tx.execute(query).await?;
    }
    tx.commit().await?;
    Ok(())
}

fn before(days: u64) -> DateTime<Utc> {
    Utc::now().checked_sub_days(Days::new(days)).unwrap()
}
fn now() -> DateTime<Utc> {
    Utc::now()
}

impl Hash for UserStat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.email.hash(state)
    }
}

struct IntList(pub i32, pub i32, pub i32); // does not handle locale, see locales module for more

impl Dummy<IntList> for Vec<i32> {
    fn dummy_with_rng<R: Rng + ?Sized>(v: &IntList, rng: &mut R) -> Vec<i32> {
        let (max, start, len) = (v.0, v.1, v.2);
        let size = rng.gen_range(0..max);
        (0..size)
            .map(|_| rng.gen_range(start..start + len))
            .collect()
    }
}
pub const ALPHABET: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

struct UniqueEmail;
impl Dummy<UniqueEmail> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_v: &UniqueEmail, rng: &mut R) -> String {
        let email: String = SafeEmail().fake_with_rng(rng);
        let id = format!(".{}", nanoid!(8, &ALPHABET));
        let at = email.find("@").unwrap();
        let email = format!("{}{}{}", &email[..at], id, &email[at..]);
        email
    }
}
