use chrono::{DateTime, FixedOffset};

use crate::db::Course;

#[derive(Clone, Debug)]
pub struct Announcement {
    pub title: Option<String>,
    pub url: Option<String>,
    pub content: Option<String>,
    pub author: Option<String>,
    pub date: DateTime<FixedOffset>,
    pub course: Course,
}

#[derive(Clone, Debug)]
pub struct CourseAnnouncements {
    pub announcements: Vec<Announcement>,
}
