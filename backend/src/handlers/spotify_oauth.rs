use axum::{
    Json,
    extract::{Query, State},
    response::{IntoResponse, Redirect, Response},
};
use base64::Engine;
use std::collections::HashMap;
use time::Duration;
use tower_sessions::Session;

use crate::{
    models::{AppError, NewUser, SpotifyAuthResponse, SpotifyMeResponse, User, UserResponse},
    state::AppState,
};

pub async fn auth(State(state): State<AppState>) -> Response {
    let client_id = state.spotify_client_id.clone();
    let redirect_uri = state.spotify_redirect_uri.clone();
    let scopes = state.spotify_scopes.clone();

    let auth_url = format!(
        "https://accounts.spotify.com/authorize?client_id={}&response_type=code&redirect_uri={}&scope={}",
        client_id, redirect_uri, scopes
    );

    Redirect::to(&auth_url).into_response()
}

pub async fn callback(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
    session: Session,
) -> Result<impl IntoResponse, AppError> {
    let code = params
        .get("code")
        .ok_or(AppError::BadRequest(
            "Authentication failed. No code received.".to_string(),
        ))?
        .clone();

    let credentials = base64::engine::general_purpose::STANDARD.encode(format!(
        "{}:{}",
        state.spotify_client_id, state.spotify_client_secret
    ));

    let response = reqwest::Client::new()
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", format!("Basic {credentials}"))
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code.as_str()),
            ("redirect_uri", state.spotify_redirect_uri.as_str()),
        ])
        .send()
        .await
        .map_err(|e| AppError::SpotifyError(format!("Spotify request failed: {e}")))?;

    let body = response
        .text()
        .await
        .map_err(|e| AppError::SpotifyError(format!("Failed to read Spotify response: {e}")))?;

    let auth: SpotifyAuthResponse = serde_json::from_str(&body).map_err(|e| {
        AppError::SpotifyError(format!(
            "Failed to parse Spotify response: {e}\nBody: {body}"
        ))
    })?;

    let profile_response = reqwest::Client::new()
        .get("https://api.spotify.com/v1/me")
        .header("Authorization", format!("Bearer {}", auth.access_token))
        .send()
        .await
        .map_err(|e| AppError::SpotifyError(format!("Spotify profile request failed: {e}")))?;

    let profile_body = profile_response.text().await.map_err(|e| {
        AppError::SpotifyError(format!("Failed to read Spotify profile response: {e}"))
    })?;

    let profile: SpotifyMeResponse = serde_json::from_str(&profile_body).map_err(|e| {
        AppError::SpotifyError(format!(
            "Failed to parse Spotify profile response: {e}\nBody: {profile_body}"
        ))
    })?;

    let now = time::OffsetDateTime::now_utc();
    let expires_at = now
        .checked_add(Duration::seconds(auth.expires_in as i64))
        .ok_or(AppError::SpotifyError("Token expiry overflow".to_string()))?;

    let user_struct = NewUser {
        email: profile.email.clone(),
        name: profile.display_name.clone(),
        spotify_id: profile.id.clone(),
        spotify_access_token: Some(auth.access_token),
        spotify_refresh_token: auth.refresh_token,
        spotify_token_expires_at: Some(time::PrimitiveDateTime::new(
            expires_at.date(),
            expires_at.time(),
        )),
        avatar_url: profile.images.first().map(|img| img.url.clone()),
        daily_song_playlist_id: None,
    };

    sqlx::query!(
        "INSERT INTO users (id, email, name, spotify_id, spotify_access_token, spotify_refresh_token, spotify_token_expires_at, avatar_url, daily_song_playlist_id)
         VALUES (gen_random_uuid(), $1, $2, $3, $4, $5, $6, $7, $8)
         ON CONFLICT (spotify_id) DO UPDATE SET
            email = EXCLUDED.email,
            name = EXCLUDED.name,
            spotify_access_token = EXCLUDED.spotify_access_token,
            spotify_refresh_token = EXCLUDED.spotify_refresh_token,
            spotify_token_expires_at = EXCLUDED.spotify_token_expires_at,
            avatar_url = EXCLUDED.avatar_url,
            daily_song_playlist_id = EXCLUDED.daily_song_playlist_id",
        user_struct.email,
        user_struct.name,
        user_struct.spotify_id,
        user_struct.spotify_access_token,
        user_struct.spotify_refresh_token,
        user_struct.spotify_token_expires_at,
        user_struct.avatar_url,
        user_struct.daily_song_playlist_id,
    )
    .execute(&state.pg_pool)
    .await
    .map_err(|e| AppError::SpotifyError(format!("Database error: {e}")))?;

    let user_id = sqlx::query_scalar!(
        "SELECT id
         FROM users WHERE spotify_id = $1",
        profile.id
    )
    .fetch_one(&state.pg_pool)
    .await
    .map_err(|e| AppError::SpotifyError(format!("Database error: {e}")))?;

    session
        .insert("user_id", user_id)
        .await
        .map_err(|e| AppError::BadRequest(format!("Session error: {e}")))?;

    Ok(Redirect::to("http://127.0.0.1:5173"))
}

pub async fn get_me(
    State(state): State<AppState>,
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
    Ok(Json(UserResponse::from(user)))
}

pub async fn logout(session: Session) -> Result<impl IntoResponse, AppError> {
    session
        .flush() // deletes all keys from the session
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok(Redirect::to("http://127.0.0.1:5173"))
}
