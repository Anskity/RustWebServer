use std::io::{Error, ErrorKind};

pub struct HttpRequest {
    pub method: String,
    pub version: String,
    pub route: String,
    pub body: String,
}

impl HttpRequest {
    pub fn from(req: &str) -> Result<Self, std::io::Error> {
        let parts = req
            .split(|c| ['\r', '\n', ' '].contains(&c))
            .filter(|txt| !txt.is_empty())
            .collect::<Vec<&str>>();

        let method = parts.get(0);
        let route = parts.get(1);
        let version = parts.get(2);
        let body = if parts.len() < 4 {
            None
        } else {
            Some(parts[3..].concat())
        };

        if method.is_none() {
            return Err(Error::new(ErrorKind::InvalidInput, "No method"));
        }
        let method = method.unwrap().to_string();
        if route.is_none() {
            return Err(Error::new(ErrorKind::InvalidInput, "No route"));
        }
        let route = route.unwrap().to_string();
        if version.is_none() {
            return Err(Error::new(ErrorKind::InvalidInput, "No version"));
        }
        let version = version.unwrap().to_string();
        if body.is_none() {
            return Err(Error::new(ErrorKind::InvalidInput, "No version"));
        }
        let body = body.unwrap();

        Ok(HttpRequest {
            method,
            route,
            version,
            body,
        })
    }
}

pub fn response_by_route(buf: &[u8]) -> Result<String, std::io::Error> {
    use std::fs;
    use std::path::Path;

    let req = String::from_utf8_lossy(buf).to_string();
    let req = req.as_str();
    let req = HttpRequest::from(req)?;
    let default_response = "HTTP/1.1 200 OK\r\n\r\nInvalid route";
    let mut route_path = "./html".to_string();
    route_path.push_str(&req.route);
    route_path.push_str(".html");
    let mut route_path = Path::new(&route_path);

    if req.route == "/" {
        route_path = Path::new("./index.html");
    }

    if route_path.exists() {
        let html = fs::read_to_string(route_path).unwrap();
        let mut res = "HTTP/1.1 200 OK\r\n\r\n".to_string();
        res.push_str(&html);
        Ok(res)
    } else {
        Ok(default_response.to_string())
    }
}
