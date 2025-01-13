use std::{sync::Arc, time::Duration};

use chrono::DateTime;
use rss::Channel;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    announcement::{Announcement, CourseAnnouncements},
    config::AppConfig,
    db::Database,
};

// TOOD: maybe would be nice to also account for edits
pub async fn rss_reader_worker(
    db: Arc<Database>,
    sender: UnboundedSender<CourseAnnouncements>,
    config: AppConfig,
) {
    log::info!("Started RSS poll worker!");
    let client = reqwest::Client::new();
    loop {
        log::info!("Polling for new announcements...");
        let courses = db.get_courses().await.unwrap();
        for course in courses {
            let content = match client.get(&course.rss_url).send().await {
                Ok(v) => match v.bytes().await {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("Failed to convert data to bytes: {e}");
                        continue;
                    }
                },
                Err(e) => {
                    log::error!("Failed to request announcement '{}': {e}", course.rss_url);
                    continue;
                }
            };
            let channel = match Channel::read_from(&content[..]) {
                Ok(v) => v,
                Err(e) => {
                    log::error!("Failed to read RSS channel: {e}");
                    continue;
                }
            };
            let mut announcements = channel
                .items
                .into_iter()
                .filter(|item| {
                    let datetime =
                        DateTime::parse_from_rfc2822(&item.pub_date.as_ref().unwrap()).unwrap();
                    datetime.timestamp_millis() > course.last_announcement.unwrap_or_default()
                })
                .map(|item| Announcement {
                    title: item.title,
                    url: item.link,
                    content: item.description.map(|x| html2md::parse_html(&x)),
                    author: item.author,
                    date: DateTime::parse_from_rfc2822(&item.pub_date.unwrap()).unwrap(),
                    course: course.clone(),
                })
                .collect::<Vec<Announcement>>();
            announcements.sort_by(|a, b| a.date.cmp(&b.date));
            if !announcements.is_empty() {
                sender.send(CourseAnnouncements { announcements }).unwrap();
            }
        }
        tokio::time::sleep(Duration::from_millis(config.poll_time)).await;
    }
}
