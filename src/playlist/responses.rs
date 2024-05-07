
// pub https://developer.spotify.com/documentation/web-api/reference/get-a-list-of-current-users-playlists
// pub https://developer.spotify.com/documentation/web-api/reference/get-list-users-playlists

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Owner {
    pub href: String,
    pub id: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub uri: String,
    pub display_name: String
}

#[derive(Debug, Deserialize)]
pub struct Track {
    pub href: String,
    pub total: i32
}

#[derive(Debug, Deserialize)]
pub struct PlaylistItemResponse {
    pub collaborative: bool,
    pub description: String,
    pub href: String,
    pub id: String,
    pub name: String,
    pub owner: Owner,
    pub public: bool,
    pub snapshot_id: String,
    pub tracks: Track,
    #[serde(rename = "type")]
    pub _type: String,
    pub uri: String
}

#[derive(Debug, Deserialize)]
pub struct PlaylistsResponse {
    pub href: String,
    pub limit: i32,
    pub next: Option<String>,
    pub offset: i32,
    pub previous: Option<String>,
    pub total: i32,
    pub items: Vec<PlaylistItemResponse>
}


