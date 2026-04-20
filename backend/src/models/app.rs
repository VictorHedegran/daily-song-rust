use crate::models::spotify::SpotifyPlaylistItem;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use zod_gen_derive::ZodSchema;

#[derive(Debug, Deserialize, Serialize)]
pub struct Submission {
    pub id: String,
    pub user_id: String,
    pub track_id: String,
    pub date: String,
    pub mood: Option<String>,
    pub notes: Option<String>,
    pub notified_at: Option<PrimitiveDateTime>,
}

#[derive(Debug, Serialize, ZodSchema)]
pub struct GetHistoryResponse {
    pub notes: Option<String>,
    pub mood: Option<String>,
    pub date: String,
    pub spotify_details: Option<SpotifyPlaylistItem>,
}
