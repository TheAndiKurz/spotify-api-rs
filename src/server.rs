use tiny_http;
use url::Url;


fn handle_request(request: tiny_http::Request) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let abs_url = "http://x.x/".to_string() + request.url();
    let req_url = Url::parse(abs_url.as_str())?;


    let succ = req_url.query_pairs().map(|(k, v)| {
        (k.into_owned(), v.into_owned())
    }).collect();
    
    Ok(succ)
}


pub fn get_queries_from_redirect(addr: &str) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let server = tiny_http::Server::http(addr).unwrap();

    let request = match server.recv() {
        Ok(rq) => rq,
        Err(_) => return Err("Coundn't recieve request".into())
    };
    
    handle_request(request)
}
