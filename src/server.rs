use tiny_http;
use url::Url;


fn handle_request(request: tiny_http::Request) -> Result<(String, String), String> {
    let abs_url = "http://x.x/".to_string() + request.url();
    let req_url = Url::parse(abs_url.as_str()).unwrap();

    let query = req_url.query_pairs();

    let mut code: Option<String> = None;
    let mut state: Option<String> = None;
    for (k, v) in query {
        match k.into_owned().as_str() {
            "code" => code = Some(v.into()),
            "state" => state = Some(v.into()),
            _ => {}
        }
    }

    return match (code, state) {
        (Some(c), Some(s)) => Ok((c, s)),
        _ => Err("coundn't retrieve code or state".into())
    };
}


pub fn serve_request(addr: &str) -> Result<(String, String), String> {
    let server = tiny_http::Server::http(addr).unwrap();

    let request = match server.recv() {
        Ok(rq) => rq,
        Err(_) => return Err("Coundn't recieve request".into())
    };
    
    handle_request(request)
}
