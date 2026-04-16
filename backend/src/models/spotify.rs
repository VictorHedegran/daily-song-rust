use serde::{Deserialize, Serialize};
use zod_gen_derive::ZodSchema;

#[derive(Debug, Deserialize, Serialize)]
pub struct SpotifyAuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub expires_in: u32,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ZodSchema)]
pub struct Image {
    pub url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpotifyMeResponse {
    pub display_name: String,
    pub email: String, // DEPRECATED, From Spotify API docs: Important! This email address is unverified; there is no proof that it actually belongs to the user. This field is only available when the current user has granted access to the user-read-email scope.
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub uri: String,
}

#[derive(Debug, Deserialize, Serialize, ZodSchema)]
pub struct Artist {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, ZodSchema)]
pub struct Album {
    pub images: Vec<Image>,
}

#[derive(Debug, Deserialize, Serialize, ZodSchema)]
pub struct Track {
    pub id: String,
    pub album: Album,
    pub artists: Vec<Artist>,
    pub name: String,
    pub uri: String,
}

#[derive(Debug, Deserialize, Serialize, ZodSchema)]
pub struct TracksPage {
    pub items: Vec<Track>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
    pub href: String,
    pub next: Option<String>,
    pub previous: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchForItem {
    pub tracks: Option<TracksPage>,
}
