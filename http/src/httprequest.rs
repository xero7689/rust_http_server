use crate::{Method, Version};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::Uninitialized;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = resource;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {
                // Do nothing with the content
            } else {
                parsed_msg_body = line;
            }
        }

        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

fn process_req_line(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap_or("GET");
    let resource = words.next().unwrap_or("/");
    let version = words.next().unwrap_or("HTTP/1.1");

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");

    if let Some(k) = header_items.next() {
        key = k.to_string();
    }

    if let Some(v) = header_items.next() {
        value = v.to_string();
    }

    if key == "Host" {
        if let Some(p) = header_items.next() {
            let port = p.to_string();
            value = format!("{}:{}", value, port);
        }
    }

    (key, value)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_http() {
        let s: String = String::from(
            "GET /greeting HTTP/1.1\r\nHost:localhost:3000\r\nUser-Agent:curl/7.64.1\r\nAccept:*/*\r\n\r\n",
        );

        let mut headers_expected: HashMap<String, String> = HashMap::new();
        headers_expected.insert("Host".into(), "localhost:3000".into());
        headers_expected.insert("User-Agent".into(), "curl/7.64.1".into());
        headers_expected.insert("Accept".into(), "*/*".into());

        let req: HttpRequest = s.into();
        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }
}
