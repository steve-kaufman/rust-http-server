mod fields;
mod http_version;
mod method;
mod request;
mod response;
mod status_code;

use std::{
    io::Write,
    net::{SocketAddr, TcpListener, TcpStream},
};

use self::{request::Request, response::Response};

pub fn start_server(port: u16) {
    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], port)))
        .expect(&format!("Unable to listen on localhost:{port}"));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let request = Request::from_stream(&mut stream).unwrap();

    println!(
        "Raw body (length {}): {:?}",
        request.body.len(),
        request.body
    );

    let body_text = String::from_utf8(
        request
            .body
            .into_iter()
            .take_while(|b| *b != 0)
            .collect::<Vec<_>>(),
    )
    .expect("Body is not valid UTF-8");

    let mut response = Response::new();
    response
        .body
        .write_all(&format!("You sent me: \"{}\"\n", body_text).as_bytes())
        .unwrap();

    response.write_to(&mut stream).unwrap();
}
