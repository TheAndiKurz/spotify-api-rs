use open;
use crate::server;
use url::Url;
use rand::{distributions::Alphanumeric, Rng};
use std::{error::Error, fs::File, io::{Read, Write}, net::TcpListener, path::Path};

use serde::{Deserialize, Serialize};

static secret_file_name: &str = "secret.json";

#[derive(Serialize, Deserialize)]
struct AuthData {
    code: String,
    state: String,
    scope: String
}

fn oauth2(client_id: &str, scope: &str) -> Result<(String, String), Box<dyn std::error::Error>>{
    let listener = TcpListener::bind("localhost:5173").unwrap();

    let addr = listener.local_addr().unwrap();

    std::mem::drop(listener);

    let mut spotify_auth_url = Url::parse("https://accounts.spotify.com/authorize").unwrap();
    {
        let mut query = spotify_auth_url.query_pairs_mut();

        query.append_pair("response_type", "code");
        query.append_pair("client_id", client_id);
        query.append_pair("scope", scope);
        let redirect_uri = format!("http://{}/callback", addr.to_string());
        query.append_pair("redirect_uri", redirect_uri.as_str());
        query.append_pair("state", 
                          rand::thread_rng()
                                .sample_iter(&Alphanumeric)
                                .take(16)
                                .map(char::from)
                                .collect::<String>()
                                .as_str());
    }

    println!("spotify auth url: {}", spotify_auth_url);
    println!("redirect_uri: {}", addr.to_string());


    open::that(spotify_auth_url.to_string())?; 

    println!("listening on {}", addr);
    
    let queries = server::get_queries_from_redirect(addr.to_string().as_str())?;

    let mut code: Option<String> = None;
    let mut state: Option<String> = None;
    for (k, v) in queries {
        match k.as_str() {
            "code" => code = Some(v.into()),
            "state" => state = Some(v.into()),
            _ => {}
        }
    }

    match (code, state) {
        (Some(code), Some(state)) => Ok((code, state)),
        _ => Err("could not find both state and code in the queries of the redirection".into())
    }
}

fn store((code, state): &(String, String), scope: &str) -> Result<(), Box<dyn std::error::Error>>{
    let data = AuthData {
        code: code.to_owned(),
        state: state.to_owned(),
        scope: scope.to_owned()
    };

    let json = serde_json::to_string(&data)?;

    File::create(secret_file_name)?.write_all(json.as_bytes())?;

    Ok(())
}

fn load(scope: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    if !Path::new(secret_file_name).exists() {
        return Err("file does not exist".into());
    }


    let mut contents = String::new();
    File::open(secret_file_name)?.read_to_string(&mut contents)?;

    let data: AuthData = serde_json::from_str(contents.as_str())?;

    if data.scope != scope {
        return Err("scope needs to be the same or else you need to reverify".into());
    }

    Ok((data.code, data.state))
}


pub fn authenticate(client_id: &str, scope: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    match load(scope) {
        Ok(succ) => Ok(succ),
        Err(err) => {
            eprintln!("was not able to load authentication: {}", err);
            let succ = oauth2(client_id, scope)?;
            store(&succ, scope)?;
            Ok(succ)
        }
    }
}
