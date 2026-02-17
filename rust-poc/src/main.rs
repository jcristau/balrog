mod config;
mod error;
mod health;
mod db;
mod rule_matching;
mod blobs;
mod update;

use axum::{
    routing::get,
    Router,
};
use sqlx::mysql::MySqlPoolOptions;
use std::time::Duration;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "balrog_rust_poc=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = config::Config::from_env()?;

    tracing::info!("Connecting to database...");

    // Create database pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.db_uri)
        .await?;

    tracing::info!("Database connected successfully");

    // Build router
    let app = Router::new()
        .route("/__lbheartbeat__", get(health::lbheartbeat))
        .route("/__heartbeat__", get(health::heartbeat))
        .route("/update/6/{product}/{version}/{build_id}/{build_target}/{locale}/{channel}/{os_version}/{system_capabilities}/{distribution}/{dist_version}/update.xml",
               get(update::handle_update))
        .layer(TraceLayer::new_for_http())
        .with_state(pool.clone());

    // Store config in environment for handlers to access
    std::env::set_var("CACHE_CONTROL", &config.cache_control);

    // Start server
    let addr = format!("0.0.0.0:{}", config.port);
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    tracing::info!("Shutdown signal received");
}
