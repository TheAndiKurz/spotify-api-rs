mod server;
mod auth;
mod playlist;

use std::fs::File;

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    client_id: String,
    client_secret: String
}

fn read_config() -> Result<Config, std::io::Error> {
    let file = File::open("config.json")?;
    let config: Config = serde_json::from_reader(file)?;

    Ok(config)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = read_config()?;

    let scope = "user-library-read playlist-read-private user-library-modify playlist-modify-private playlist-modify-public";

    let token = auth::get_token(&config.client_id, &config.client_secret, scope)?;

    let playlists = playlist::my_playlists::get_my_playlists(&token.access_token)?;

    for playlist in playlists.items {
        println!("{:?}", playlist.id);
    }

    Ok(())
}
