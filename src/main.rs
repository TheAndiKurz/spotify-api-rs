mod server;
mod auth;

use std::fs::File;

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    client_id: String,
    client_secret: String
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let config: Config = serde_json::from_reader(File::open("config.json")?)?;

    let scope = "user-library-read playlist-read-private user-library-modify playlist-modify-private playlist-modify-public";
    let client_id = config.client_id.as_str();

    let (code, state) = auth::authenticate(client_id, scope).unwrap();


    Ok(())
}
