use std::sync::Arc;

use tokio::sync::mpsc::UnboundedReceiver;

use crate::{announcement::Announcement, db::Database};

pub async fn storage_worker(mut receiver: UnboundedReceiver<Announcement>, db: Arc<Database>) {
    log::info!("Started storage worker!");
    loop {
        let Some(announcement) = receiver.recv().await else {
            log::warn!("Storage channel closed, stopping storage worker!");
            break;
        };
        if let Err(e) = db
            .update_course_last_message(&announcement.course, announcement.date.timestamp_millis())
            .await
        {
            log::error!("Failed to update course in database with last message: {e}");
        }
    }
}
