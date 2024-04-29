mod server;

use std::net::TcpListener;

fn main() {
    
    let listener = TcpListener::bind("localhost:0").unwrap();

    let addr = listener.local_addr().unwrap();

    std::mem::drop(listener);

    println!("listening on {}", addr);
    
    if let Ok((code, state)) = server::serve_request(addr.to_string().as_str()) {
        println!("get code {} and state {}", code, state);
    }
}
