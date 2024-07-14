pub struct HttpRequest {
    pub method: String,
    pub version: String,
    pub route: String,
    pub body: String,
}

impl HttpRequest {
    pub fn from(req: &str) -> Self {
        let parts = req
            .split(|c| ['\r', '\n', ' '].contains(&c))
            .filter(|txt| !txt.is_empty())
            .collect::<Vec<&str>>();

        let method = parts[0].to_string();
        let route = parts[1].to_string();
        let version = parts[2].to_string();
        let body = parts[3..].concat();

        HttpRequest {
            method,
            route,
            version,
            body,
        }
    }
}

pub fn response_by_route(buf: &[u8]) -> String {
    use std::fs;
    use std::path::Path;

    let req = String::from_utf8_lossy(buf).to_string();
    let req = req.as_str();
    let req = HttpRequest::from(req);
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
        res
    } else {
        default_response.to_string()
    }
}
