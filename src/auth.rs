use base64::{alphabet, engine::general_purpose, Engine as _};
use open;
use crate::server;
use url::Url;
use std::{fs::File, io::{Read, Write}, path::Path};

use sha2::{Digest, Sha256};

use serde::{Deserialize, Serialize};

static SECRET_FILE_NAME: &str = "secret.json";
static REDIRECT_URI_ADDR: &str = "localhost:5173";
static REDIRECT_URI: &str = "http://localhost:5173/callback";

#[derive(Serialize, Deserialize)]
struct AuthData {
    code: String,
    scope: String,
    code_verifier: String
}

fn generate_random_string(len: usize) -> String {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    random_string::generate(len, charset)
}

const CUSTOM_ENGINE: general_purpose::GeneralPurpose = general_purpose::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

fn oauth2(client_id: &str, scope: &str, code_verifier: &str) -> Result<String, Box<dyn std::error::Error>>{
    let sha_output = Sha256::digest(code_verifier.as_bytes());
    let sha_hashed_bytes = sha_output.as_slice();
    let code_challenge = CUSTOM_ENGINE.encode(sha_hashed_bytes);

    let mut spotify_auth_url = Url::parse("https://accounts.spotify.com/authorize").unwrap();
    {
        let mut query = spotify_auth_url.query_pairs_mut();

        query.append_pair("response_type", "code");
        query.append_pair("client_id", client_id);
        query.append_pair("scope", scope);
        query.append_pair("code_challenge_method", "S256");
        query.append_pair("code_challenge", code_challenge.as_str());
        query.append_pair("redirect_uri", REDIRECT_URI);

    }

    println!("spotify auth url: {}", spotify_auth_url);


    open::that(spotify_auth_url.to_string())?; 
    
    let queries = server::get_queries_from_redirect(REDIRECT_URI_ADDR)?;

    let mut code: Option<String> = None;
    for (k, v) in queries {
        match k.as_str() {
            "code" => code = Some(v.into()),
            _ => {}
        }
    }

    code.ok_or("code not found".into())
}

fn store(code: &str, scope: &str, code_verifier: &str) -> Result<AuthData, Box<dyn std::error::Error>>{
    let data = AuthData {
        code: code.to_owned(),
        scope: scope.to_owned(),
        code_verifier: code_verifier.to_owned()
    };

    let json = serde_json::to_string(&data)?;

    File::create(SECRET_FILE_NAME)?.write_all(json.as_bytes())?;

    Ok(data)
}

fn load(scope: &str) -> Result<AuthData, Box<dyn std::error::Error>> {
    if !Path::new(SECRET_FILE_NAME).exists() {
        return Err("file does not exist".into());
    }


    let mut contents = String::new();
    File::open(SECRET_FILE_NAME)?.read_to_string(&mut contents)?;

    let data: AuthData = serde_json::from_str(contents.as_str())?;

    if data.scope != scope {
        return Err("scope needs to be the same or else you need to reverify".into());
    }

    Ok(data)
}


fn authenticate(client_id: &str, scope: &str) -> Result<AuthData, Box<dyn std::error::Error>> {
    match load(scope) {
        Ok(succ) => Ok(succ),
        Err(err) => {
            eprintln!("was not able to load authentication: {}", err);
            let code_verifier = generate_random_string(64);
            let succ = oauth2(client_id, scope, &code_verifier)?;
            let data = store(&succ, scope, &code_verifier)?;
            Ok(data)
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    scope: String,
    expires_in: u64,
    refresh_token: String
}

pub fn get_token(client_id: &str, client_secret: &str, scope: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = authenticate(client_id, scope)?;

    let client = reqwest::blocking::Client::new();
    let token_response = client.post("https://accounts.spotify.com/api/token")
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", data.code.as_str()),
            ("code_verifier", data.code_verifier.as_str()),
            ("redirect_uri", REDIRECT_URI),
        ])
        .basic_auth(client_id, Some(client_secret))
        .send()?;

    if token_response.status().is_client_error() {
        eprintln!("client error. status: {}", token_response.status());
        let body = token_response.text()?;
        eprintln!("body: {}", body);
        return Err("client error".into());
    }

    let token = token_response.error_for_status()?.text()?;

    println!("token response: {}", token);

    let token: TokenResponse = serde_json::from_str(token.as_str())?;

    Ok(token.access_token)
}
