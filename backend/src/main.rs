use axum::{
    Router,
    http::{HeaderValue, Method},
    routing::{get, post},
};
use tokio::{signal, task::AbortHandle};
use tower_http::cors::CorsLayer;

mod config;
mod handlers;
mod models;
mod state;

use config::load_from_env;
use handlers::*;

mod zod_generator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    zod_generator::generate_types();

    dotenvy::dotenv().ok();

    let config = load_from_env().await?;

    let cors = CorsLayer::new()
        .allow_origin(
            config
                .frontend_url
                .parse::<HeaderValue>()
                .expect("invalid FRONTEND_URL"),
        )
        .allow_methods([Method::GET, Method::POST])
        .allow_credentials(true);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/say_hi", get(say_hi))
        .route("/echo", post(echo))
        .route("/auth/spotify", get(auth))
        .route("/auth/spotify/callback", get(callback))
        .route("/me", get(get_me))
        .route("/logout", post(logout))
        .route("/search", get(search))
        .route("/add_track", post(add_track))
        .route("/history", get(get_history))
        .with_state(config.state)
        .layer(config.session_layer)
        .layer(cors);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(config.deletion_task.abort_handle()))
        .await?;

    config.deletion_task.await??;

    Ok(())
}

async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort() },
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}
