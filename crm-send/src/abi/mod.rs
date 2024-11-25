use crate::pb::notification_server::NotificationServer;
use crate::{AppConfig, NotificationService};

impl NotificationService {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }
    pub fn into_server(self) -> NotificationServer<Self> {
        NotificationServer::new(self)
    }
}
