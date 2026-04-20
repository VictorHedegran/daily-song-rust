use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_sessions::Session;

use crate::models::{
    AppError, GetHistoryResponse, SearchForItem, SpotifyPlaylistItem, SpotifyPlaylistResponse,
    Submission, TracksPage, User,
};

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

    let res = search_result.tracks.unwrap_or(TracksPage {
        items: vec![],
        total: 0,
        limit: 0,
        offset: 0,
        href: "".to_string(),
        next: None,
        previous: None,
    });

    Ok(Json(res))
}

#[derive(Debug, Deserialize)]
pub struct AddTrackBody {
    pub uri: String,
    pub notes: Option<String>,
    pub mood: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
struct SpotifyAddTrackRequest {
    uris: Vec<String>,
    position: u32,
}

pub async fn add_track(
    State(state): State<crate::state::AppState>,
    session: Session,
    Json(body): Json<AddTrackBody>,
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

    let playlist_id = state.playlist_id.clone();

    let request = SpotifyAddTrackRequest {
        uris: vec![body.uri.clone()],
        position: 0,
    };

    reqwest::Client::new()
        .post(format!(
            "https://api.spotify.com/v1/playlists/{}/items",
            playlist_id
        ))
        .header("Authorization", format!("Bearer {}", access_token))
        .json(&request)
        .send()
        .await
        .map_err(|e| AppError::SpotifyError(format!("Spotify request failed: {e}")))?;

    let track_id = body
        .uri
        .split(':')
        .last()
        .ok_or(AppError::BadRequest("Invalid track URI".to_string()))?;

    let current_date = time::OffsetDateTime::now_utc().date().to_string();

    sqlx::query!(
        "DELETE FROM submissions WHERE user_id = $1 AND date = $2",
        user.id,
        current_date
    )
    .execute(&state.pg_pool)
    .await
    .map_err(|e| AppError::SpotifyError(format!("Database error: {e}")))?;

    sqlx::query!(
        "INSERT INTO submissions (id, user_id, track_id, notes, mood, date) VALUES (gen_random_uuid(), $1, $2, $3, $4, $5)",
        user.id,
        track_id,
        body.notes,
        body.mood,
        current_date
    )
    .execute(&state.pg_pool)
    .await
    .map_err(|e| AppError::SpotifyError(format!("Database error: {e}")))?;

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetHistoryParams {
    pub page: Option<i64>,
}

// idea: fetch spotify playlist submissions as source of truth for history, look up submissions for track_id and added_by user_id, then if no submission is found, prompt the user to add one retroactively if it was their track

pub async fn get_history(
    State(state): State<crate::state::AppState>,
    Query(params): Query<GetHistoryParams>,
    session: Session,
) -> Result<Json<Vec<GetHistoryResponse>>, AppError> {
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

    let playlist_id = state.playlist_id.clone();

    let page: i64 = params.page.unwrap_or(1);
    let limit: i64 = 10;
    let offset: i64 = (page - 1) * limit;

    let history_data = sqlx::query!(
        "SELECT *
         FROM submissions
         ORDER BY date DESC
         LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(&state.pg_pool)
    .await
    .map_err(|e| AppError::SpotifyError(format!("Database error: {e}")))?;

    let mut history: HashMap<String, Vec<GetHistoryResponse>> = history_data
        .into_iter()
        .map(|record| {
            let track_id = record.track_id;
            let submission = Submission {
                id: record.id,
                track_id: track_id.clone(),
                notes: record.notes,
                mood: record.mood,
                date: record.date,
                user_id: record.user_id,
                notified_at: record.notified_at,
            };
            (
                track_id,
                vec![GetHistoryResponse {
                    notes: submission.notes,
                    mood: submission.mood,
                    date: submission.date,
                    spotify_details: None,
                }],
            )
        })
        .collect();

    let mut tracks: SpotifyPlaylistResponse = SpotifyPlaylistResponse {
        next: None,
        items: Vec::new(),
    };

    let mut next = format!(
        "https://api.spotify.com/v1/playlists/{}/items?fields=next,items(item(id,name,album(images),artists(name),external_urls))&limit={}&offset={}",
        playlist_id, limit, offset
    );

    loop {
        let tracks_data = reqwest::Client::new()
            .get(next.clone())
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| AppError::SpotifyError(format!("Spotify request failed: {e}")))?;

        let tracks_body = tracks_data.text().await.map_err(|e| {
            AppError::SpotifyError(format!("Failed to read Spotify playlist response: {e}"))
        })?;

        let tracks_data: SpotifyPlaylistResponse =
            serde_json::from_str(&tracks_body).map_err(|e| {
                AppError::SpotifyError(format!(
                    "Failed to parse Spotify playlist response: {e}\nBody: {tracks_body}"
                ))
            })?;

        tracks.items.extend(tracks_data.items);

        if let Some(next_url) = tracks_data.next {
            next = next_url;
        } else {
            break;
        }
    }

    for wrapper in tracks.items {
        let track = wrapper.track;
        if let Some(submissions) = history.get_mut(&track.id) {
            for submission in submissions.iter_mut() {
                submission.spotify_details = Some(SpotifyPlaylistItem {
                    id: track.id.clone(),
                    name: track.name.clone(),
                    album: track.album.clone(),
                    artists: track.artists.clone(),
                    external_urls: track.external_urls.clone(),
                });
            }
        }
    }

    Ok(Json(history.into_iter().flat_map(|(_, v)| v).collect()))
}
