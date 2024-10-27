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

#[derive(Debug, Clone, Dummy, Serialize, Deserialize, Eq, PartialEq)]
enum Gender {
    Female,
    Male,
    Unknown,
}
#[derive(Debug, Dummy, Serialize, Deserialize, PartialEq, Eq)]
struct UserStat {
    #[dummy(faker = "UniqueEmail")]
    email: String,
    #[dummy(faker = "Name()")]
    name: String,
    gender: Gender,
    #[dummy(faker = "DateTimeBetween(start(365*3), end())")]
    created_at: DateTime<Utc>,
    #[dummy(faker = "DateTimeBetween(start(30), end())")]
    last_visited_at: DateTime<Utc>,
    #[dummy(faker = "DateTimeBetween(start(90), end())")]
    last_watched_at: DateTime<Utc>,
    #[dummy(faker = "IntList(50,10000,10000)")]
    recent_watched: Vec<i32>,
    #[dummy(faker = "IntList(50,20000,10000)")]
    viewed_but_not_started: Vec<i32>,
    #[dummy(faker = "IntList(50,30000,10000)")]
    started_but_not_finished: Vec<i32>,
    #[dummy(faker = "IntList(50,40000,10000)")]
    finished: Vec<i32>,
    #[dummy(faker = "DateTimeBetween(start(45), end())")]
    last_email_notification: DateTime<Utc>,
    #[dummy(faker = "DateTimeBetween(start(15), end())")]
    last_in_app_notification: DateTime<Utc>,
    #[dummy(faker = "DateTimeBetween(start(90), end())")]
    last_sms_notification: DateTime<Utc>,
}

//DateTime<Local>

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // DateTimeBefore()
    let user: UserStat = Faker.fake();
    println!("{:?}", user);
    Ok(())
}
async fn bulk_insert(users: Vec<UserStat>, pool: &PgPool) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    for user in users {
        let query =    sqlx::query(
            r#"
            INSERT INTO user_stats(email, name, created_at, last_visited_at, last_watched_at, recent_watched, viewed_but_not_started, started_but_not_finished, finished, last_email_notification, last_in_app_notification, last_sms_notification)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)

            "#
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

fn start(days: u64) -> DateTime<Utc> {
    Utc::now().checked_sub_days(Days::new(days)).unwrap()
}
fn end() -> DateTime<Utc> {
    Utc::now()
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
