use rust_webserver::web::response_by_route;
use std::{
    fs,
    io::{Error, Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:8246")?;

    for req in listener.incoming() {
        handle_connection(req?)?;
    }

    Ok(())
}

fn handle_connection(mut req: TcpStream) -> Result<(), Error> {
    let mut buf = [0u8; 1024];
    req.read(&mut buf)?;

    let res = response_by_route(&buf);
    let res = res.as_bytes();
    req.write(res)?;
    req.flush()?;

    Ok(())
}
