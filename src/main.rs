use std::sync::Arc;

use axum::{
    routing::{delete, get},
    Router,
};
use clap::{command, Parser};
use config::AppConfig;
use db::Database;
use handlers::AppState;
use tokio::{net::TcpListener, runtime::Runtime};
use tower_http::services::ServeDir;

mod announcement;
mod config;
mod db;
mod handlers;
mod workers;

pub async fn run_app(config: &AppConfig) {
    let db = Database::connect(&config.database_url)
        .await
        .expect("Failed to connect to database!");
    let db = Arc::new(db);
    log::info!("Connected to database at '{}'", config.database_url);

    let (notify_sender, notify_receiver) = tokio::sync::mpsc::unbounded_channel();
    let (storage_sender, storage_receiver) = tokio::sync::mpsc::unbounded_channel();

    tokio::spawn(workers::rss_reader_worker(
        db.clone(),
        notify_sender,
        config.clone(),
    ));
    tokio::spawn(workers::discord_worker(
        notify_receiver,
        storage_sender,
        config.clone(),
    ));
    tokio::spawn(workers::storage_worker(storage_receiver, db.clone()));

    let state = AppState::new(db.clone());
    let api_router = Router::new()
        .route(
            "/courses",
            get(handlers::get_courses).post(handlers::create_course),
        )
        .route("/courses/{id}", delete(handlers::delete_course))
        .with_state(state);
    let cors_router = Router::new().route("/", get(handlers::cors));
    let app_router = Router::new()
        .nest("/api", api_router)
        .nest("/cors", cors_router)
        .fallback_service(ServeDir::new(config.web_dir.as_str()));
    let listener = TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("Failed to bind TcpListener");
    axum::serve(listener, app_router)
        .await
        .expect("Failed to server web application!");
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "AppConfig.toml")]
    config: String,
}

fn main() {
    pretty_env_logger::init_timed();
    let args = Args::parse();
    let config = AppConfig::load_from_file(args.config).expect("Failed to load config!");
    log::info!("Loaded configuration: {:#?}", config);
    let rt = Runtime::new().expect("Failed to create async runtime!");
    rt.block_on(run_app(&config));
}
