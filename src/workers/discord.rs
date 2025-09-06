use structs::{Embed, EmbedAuthor, WebhookMessage};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::{
    announcement::{Announcement, CourseAnnouncements},
    config::AppConfig,
};

pub mod structs;

const MAX_TITLE_SIZE: usize = 256;
const MAX_DESCRIPTION_SIZE: usize = 4096;

pub async fn discord_worker(
    mut receiver: UnboundedReceiver<CourseAnnouncements>,
    sender: UnboundedSender<Announcement>,
    config: AppConfig,
) {
    log::info!("Started Discord notifier worker!");
    let client = reqwest::Client::new();
    loop {
        let Some(announcements) = receiver.recv().await.map(|v| v.announcements) else {
            log::warn!("Notification channel closed, stopping Discord notifier worker!");
            break;
        };
        let mut last_announcement: Option<Announcement> = None;
        for announcement in announcements {
            log::debug!("Sending announcement notification: {:#?}", announcement);
            let message = create_webhook_message(&announcement, &config);
            if let Err(e) = client
                .post(&config.webhook_url)
                .json(&message)
                .send()
                .await
                .map(|r| r.error_for_status())
            {
                log::error!(
                    "Failed to send discord webhook message: {}\nMessage: {:#?}",
                    e,
                    message
                );
                break;
            }
            last_announcement = match last_announcement {
                Some(x) => Some(if announcement.date > x.date {
                    announcement
                } else {
                    x
                }),
                None => Some(announcement),
            };
        }
        if let Some(announcement) = last_announcement {
            sender.send(announcement).unwrap();
        }
    }
}

fn create_webhook_message<'a>(
    announcement: &'a Announcement,
    config: &'a AppConfig,
) -> WebhookMessage<'a> {
    let author = announcement
        .author
        .as_ref()
        .map(|x| EmbedAuthor { name: x });
    let embed = Embed {
        author,
        title: announcement
            .title
            .as_ref()
            .map(|x| trim_str_to_size(x, MAX_TITLE_SIZE)),
        description: announcement
            .content
            .as_ref()
            .map(|x| trim_str_to_size(x, MAX_DESCRIPTION_SIZE)),
        url: announcement.url.as_ref().map(|x| x.as_str()),
        color: announcement.course.color,
        timestamp: announcement.date.to_rfc3339(),
    };
    WebhookMessage {
        content: format!(
            "<@&{}> Novo anúncio na cadeira {}",
            announcement.course.role_id, announcement.course.name
        ),
        username: &config.username,
        avatar_url: &config.avatar_url,
        embeds: vec![embed],
    }
}

fn trim_str_to_size(str: &str, max_size: usize) -> &str {
    let max_size = std::cmp::min(str.len(), max_size);
    &str[0..max_size]
}
