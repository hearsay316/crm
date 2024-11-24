use crate::{AppConfig, NotificationService};
use crate::pb::notification_server::NotificationServer;

impl NotificationService{
    pub fn new (config: AppConfig) -> Self{
        Self{
            config,
        }
    }
    pub fn into_server(self) -> NotificationServer<Self>{
        NotificationServer::new(self)
    }
}