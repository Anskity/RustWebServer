use rust_webserver::web::response_by_route;
use std::{
    env, fs,
    io::{Error, Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() -> Result<(), Error> {
    let args = env::args().collect::<Vec<String>>();

    let port = if let Some(arg) = args.get(1) {
        arg.clone()
    } else {
        "8123".to_string()
    };

    let mut ip = "127.0.0.1:".to_string();
    ip.push_str(&port);

    let listener = TcpListener::bind(ip)?;

    for req in listener.incoming() {
        let _ = handle_connection(req?);
    }

    Ok(())
}

fn handle_connection(mut req: TcpStream) -> Result<(), Error> {
    let mut buf = [0u8; 1024];
    req.read(&mut buf)?;

    let res = response_by_route(&buf)?;
    let res = res.as_bytes();
    req.write(res)?;
    req.flush()?;

    Ok(())
}
