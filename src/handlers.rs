use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::db::{Course, Database};

type HandlerResult<T> = Result<T, (StatusCode, String)>;

#[derive(Clone)]
pub struct AppState {
    db: Arc<Database>,
}

impl AppState {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
}

pub async fn get_courses(state: State<AppState>) -> HandlerResult<Json<Vec<Course>>> {
    match state.db.get_courses().await {
        Ok(v) => Ok(Json(v)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[derive(Serialize, Deserialize)]
pub struct AddCourseDTO {
    id: i64,
    name: String,
    rss_url: String,
    color: i32,
    ignore_previous: bool,
}

pub async fn create_course(
    State(state): State<AppState>,
    Json(course): Json<AddCourseDTO>,
) -> HandlerResult<Json<Course>> {
    let last_announcement = match course.ignore_previous {
        true => Some(Local::now().timestamp_millis()),
        false => None,
    };
    match state
        .db
        .create_course(
            course.id,
            &course.name,
            &course.rss_url,
            course.color,
            last_announcement,
        )
        .await
    {
        Ok(course) => Ok(Json(course)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn delete_course(
    Path(course_id): Path<i64>,
    State(state): State<AppState>,
) -> HandlerResult<()> {
    match state.db.delete_course(course_id).await {
        Ok(_) => Ok(()),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[derive(Deserialize, Serialize)]
pub struct CorsParams {
    url: String,
}

pub async fn cors(query: Query<CorsParams>) -> HandlerResult<String> {
    log::debug!("Cors for url: {}", query.url);
    match reqwest::get(&query.url).await {
        Ok(response) => match response.error_for_status() {
            Ok(response) => match response.text().await {
                Ok(resposne) => Ok(resposne),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
            },
            Err(e) => Err((e.status().unwrap_or_default(), e.to_string())),
        },
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
