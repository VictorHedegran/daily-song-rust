#[derive(Clone)]
pub struct AppState {
    pub spotify_client_id: String,
    pub spotify_client_secret: String,
    pub spotify_redirect_uri: String,
    pub spotify_scopes: String,
    pub pg_pool: sqlx::PgPool,
}
