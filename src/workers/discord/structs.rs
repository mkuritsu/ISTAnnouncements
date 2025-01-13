use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedAuthor<'a> {
    pub name: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Embed<'a> {
    pub author: Option<EmbedAuthor<'a>>,
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
    pub url: Option<&'a str>,
    pub color: i32,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebhookMessage<'a> {
    pub content: String,
    pub username: &'a str,
    pub avatar_url: &'a str,
    pub embeds: Vec<Embed<'a>>,
}
