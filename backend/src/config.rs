use time::Duration;
use tokio::task::JoinHandle;
use tower_sessions::{
    Expiry, SessionManagerLayer,
    cookie::SameSite,
    session_store::{Error as SessionStoreError, ExpiredDeletion},
};
use tower_sessions_sqlx_store::PostgresStore;

use crate::{models::ConfigError, state::AppState};

pub struct Config {
    pub state: AppState,
    pub session_layer: SessionManagerLayer<PostgresStore>,
    pub deletion_task: JoinHandle<Result<(), SessionStoreError>>,
}

pub async fn load_from_env() -> Result<Config, ConfigError> {
    let spotify_client_id = std::env::var("SPOTIFY_CLIENT_ID")
        .map_err(|_| ConfigError::MissingEnvVar("SPOTIFY_CLIENT_ID".to_string()))?;

    let spotify_redirect_uri = std::env::var("SPOTIFY_REDIRECT_URI")
        .map_err(|_| ConfigError::MissingEnvVar("SPOTIFY_REDIRECT_URI".to_string()))?;

    let spotify_client_secret = std::env::var("SPOTIFY_CLIENT_SECRET")
        .map_err(|_| ConfigError::MissingEnvVar("SPOTIFY_CLIENT_SECRET".to_string()))?;

    let spotify_scopes = std::env::var("SPOTIFY_SCOPES").unwrap_or_else(|_| {
        "user-read-private user-read-email playlist-modify-public playlist-modify-private playlist-read-private playlist-read-collaborative".to_string()
    });

    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| ConfigError::MissingEnvVar("DATABASE_URL".to_string()))?;

    let playlist_id = std::env::var("DAILY_SONG_PLAYLIST_ID")
        .map_err(|_| ConfigError::MissingEnvVar("DAILY_SONG_PLAYLIST_ID".to_string()))?;

    let frontend_url = std::env::var("FRONTEND_URL")
        .map_err(|_| ConfigError::MissingEnvVar("FRONTEND_URL".to_string()))?;

    let secure_cookies = std::env::var("SECURE_COOKIES")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);

    let pg_pool = sqlx::PgPool::connect(&database_url).await?;

    sqlx::migrate!("./migrations").run(&pg_pool).await?;

    let session_store = PostgresStore::new(pg_pool.clone());
    session_store.migrate().await?;

    let deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
    );

    let same_site = if secure_cookies { SameSite::None } else { SameSite::Lax };
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(secure_cookies)
        .with_same_site(same_site)
        .with_expiry(Expiry::OnInactivity(Duration::days(30)));

    Ok(Config {
        state: AppState {
            spotify_client_id,
            spotify_redirect_uri,
            spotify_client_secret,
            spotify_scopes,
            pg_pool,
            playlist_id,
            frontend_url,
        },
        session_layer,
        deletion_task,
    })
}
