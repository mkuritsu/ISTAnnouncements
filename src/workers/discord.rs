use structs::{Embed, EmbedAuthor, WebhookMessage};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::{
    announcement::{Announcement, CourseAnnouncements},
    config::AppConfig,
};

pub mod structs;

pub async fn discord_worker(
    mut receiver: UnboundedReceiver<CourseAnnouncements>,
    sender: UnboundedSender<Announcement>,
    config: AppConfig,
    webhook_url: String,
) {
    log::info!("Started Discord notifier worker!");
    let client = reqwest::Client::new();
    loop {
        let announcements = match receiver.recv().await {
            Some(v) => v.announcements,
            None => {
                log::warn!("Notification channel closed, stopping Discord notifier worker!");
                break;
            }
        };
        let mut last_announcement: Option<Announcement> = None;
        for announcement in announcements {
            log::debug!("Sending announcement notification: {:#?}", announcement);
            let message = create_webhook_message(&announcement, &config);
            match client.post(&webhook_url).json(&message).send().await {
                Ok(r) => {
                    if let Err(e) = r.error_for_status() {
                        log::error!("Failed to send discord webhook message: {e}");
                        log::error!("Message: {:#?}", message);
                        break;
                    }
                }
                Err(e) => {
                    log::error!("Failed to send discord webhook message: {e}");
                    break;
                }
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
        title: announcement.title.as_ref().map(|x| x.as_str()),
        description: announcement.content.as_ref().map(|x| x.as_str()),
        url: announcement.url.as_ref().map(|x| x.as_str()),
        color: announcement.course.color,
        timestamp: announcement.date.to_rfc3339(),
    };
    WebhookMessage {
        content: format!(
            "<@&{}> Novo anúncioa na cadeira {}",
            config.mention_role, announcement.course.name
        ),
        username: &config.username,
        avatar_url: &config.avatar_url,
        embeds: vec![embed],
    }
}
