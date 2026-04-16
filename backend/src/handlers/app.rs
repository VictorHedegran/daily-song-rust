use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};
use serde::Deserialize;
use tower_sessions::Session;

use crate::models::{AppError, SearchForItem, User};

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub query: String,
}

pub async fn search(
    State(state): State<crate::state::AppState>,
    Query(params): Query<SearchParams>,
    session: Session,
) -> Result<impl IntoResponse, AppError> {
    let user_id: String = session
        .get("user_id")
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?
        .ok_or(AppError::Unauthorized("Not logged in".to_string()))?;

    let user = sqlx::query_as!(
        User,
        "SELECT id, email, name, spotify_id, spotify_access_token, spotify_refresh_token, spotify_token_expires_at, avatar_url, daily_song_playlist_id, created_at
         FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(&state.pg_pool)
    .await
    .map_err(|e| AppError::SpotifyError(format!("Database error: {e}")))?;

    let access_token = user
        .get_valid_access_token(
            &state.pg_pool,
            &state.spotify_client_id,
            &state.spotify_client_secret,
        )
        .await?;

    let response = reqwest::Client::new()
        .get(format!(
            "https://api.spotify.com/v1/search?q={}&type=track&limit=10&include_external=audio",
            params.query
        ))
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .map_err(|e| AppError::SpotifyError(format!("Spotify request failed: {e}")))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read error response".to_string());
        return Err(AppError::SpotifyError(format!(
            "Spotify API error: Status {status}, Response: {error_text}"
        )));
    }

    let body = response
        .text()
        .await
        .map_err(|e| AppError::SpotifyError(format!("Failed to read Spotify response: {e}")))?;

    let search_result: SearchForItem = serde_json::from_str(&body).map_err(|e| {
        AppError::SpotifyError(format!(
            "Failed to parse Spotify response: {e}\nBody: {body}"
        ))
    })?;

    let items = search_result.tracks.map(|t| t.items).unwrap_or_default();

    Ok(Json(items))
}
