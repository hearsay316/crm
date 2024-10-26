create table user_stats
(
    email                    varchar(64) NOT NULL PRIMARY KEY,
    name                     varchar(64) NOT NULL,
    created_at               timestamptz DEFAULT CURRENT_TIMESTAMP,
    last_visited_at          timestamptz,
    last_watched_at          timestamptz,
    recent_watched           int[],
    viewed_but_not_started   int[],
    started_but_not_finished int[],
    finished                 int[],
    last_email_notification  timestamptz,
    last_in_app_notification timestamptz,
    last_sms_notification    timestamptz
);

create index user_stats_create_at_idx ON user_stats (created_at);

create index user_stats_last_visited_at_idx ON user_stats (last_visited_at);

create index user_stats_last_watched_at_idx ON user_stats (last_watched_at);

create index user_stats_recent_watched_idx ON user_stats USING GIN(recent_watched);

create index user_stats_viewed_but_not_started_idx ON user_stats USING GIN(viewed_but_not_started);

create index user_stats_started_but_not_finished_idx ON user_stats USING GIN(started_but_not_finished);

create index user_stats_last_email_notification_idx ON user_stats (last_email_notification);

create index user_stats_last_in_app_notification_idx ON user_stats (last_in_app_notification);

create index user_stats_last_sms_notification_idx ON user_stats (last_sms_notification);


