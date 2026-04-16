use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use base64::Engine;
use serde::{Deserialize, Serialize};
use time;
use ts_rs::TS;

use super::errors::AppError;
use super::spotify::SpotifyAuthResponse;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub spotify_id: String,
    pub spotify_access_token: Option<String>,
    pub spotify_refresh_token: Option<String>,
    pub spotify_token_expires_at: Option<time::PrimitiveDateTime>,
    pub avatar_url: Option<String>,
    pub daily_song_playlist_id: Option<String>,
    pub created_at: time::PrimitiveDateTime,
}

impl IntoResponse for User {
    fn into_response(self) -> Response {
        let body = serde_json::to_string(&self).unwrap_or_else(|_| "{}".to_string());
        (StatusCode::OK, body).into_response()
    }
}

impl User {
    pub async fn get_valid_access_token(
        &self,
        pg_pool: &sqlx::PgPool,
        client_id: &str,
        client_secret: &str,
    ) -> Result<String, AppError> {
        let Some(expires_at) = self.spotify_token_expires_at else {
            return Err(AppError::Unauthorized(
                "No valid access token available".into(),
            ));
        };

        let Some(refresh_token) = &self.spotify_refresh_token else {
            return Err(AppError::Unauthorized(
                "No valid access token available".into(),
            ));
        };

        if time::OffsetDateTime::now_utc() < expires_at.assume_utc() {
            return self
                .spotify_access_token
                .clone()
                .ok_or_else(|| AppError::Unauthorized("No valid access token available".into()));
        }

        let credentials = base64::engine::general_purpose::STANDARD
            .encode(format!("{client_id}:{client_secret}"));

        let response = reqwest::Client::new()
            .post("https://accounts.spotify.com/api/token")
            .header("Authorization", format!("Basic {credentials}"))
            .form(&[
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh_token.as_str()),
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

        let new_expiry =
            time::OffsetDateTime::now_utc() + time::Duration::seconds(auth.expires_in as i64);
        let new_expires_at = Some(time::PrimitiveDateTime::new(
            new_expiry.date(),
            new_expiry.time(),
        ));

        let new_refresh_token = auth
            .refresh_token
            .as_deref()
            .unwrap_or(refresh_token.as_str());

        sqlx::query!(
            "UPDATE users SET spotify_access_token = $1, spotify_refresh_token = $2, spotify_token_expires_at = $3 WHERE spotify_id = $4",
            auth.access_token,
            new_refresh_token,
            new_expires_at,
            self.spotify_id,
        )
        .execute(pg_pool)
        .await
        .map_err(|e| AppError::SpotifyError(format!("Database error: {e}")))?;

        Ok(auth.access_token)
    }
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub daily_song_playlist_id: Option<String>,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        UserResponse {
            id: u.id,
            name: u.name,
            email: u.email,
            avatar_url: u.avatar_url,
            daily_song_playlist_id: u.daily_song_playlist_id,
        }
    }
}

#[derive(Debug)]
pub struct NewUser {
    pub email: String,
    pub name: String,
    pub spotify_id: String,
    pub spotify_access_token: Option<String>,
    pub spotify_refresh_token: Option<String>,
    pub spotify_token_expires_at: Option<time::PrimitiveDateTime>,
    pub avatar_url: Option<String>,
    pub daily_song_playlist_id: Option<String>,
}
