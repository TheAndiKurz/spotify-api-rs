use reqwest::blocking::Client;
use super::responses::PlaylistsResponse;

pub fn get_my_playlists(token: &str) -> Result<PlaylistsResponse, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get("https://api.spotify.com/v1/me/playlists")
        .bearer_auth(token)
        .send()?;

    let response_text = response.text()?;

    let playlists: PlaylistsResponse = serde_json::from_str(&response_text)?;
    Ok(playlists)
}
