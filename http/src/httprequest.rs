use std::collections::HashMap;

// We need data structure 'Method' (type enum) which specifies allowed variants for HTTP methods
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}
// Then we need trait implementation 'From<&str> for conversion incoming string slices
impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

// to test if 'from' method works, we have next unit-test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }
    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }
    // Unit test for HTTP request parsing logic
    #[test]
    fn test_read_http() {
        // Simulate incoming http request
        let s: String = String::from(
            "GET /greeting HTTP/1.1\r\nHost:localhost:3000\r\nUser-Agent:curl/7.64.1\r\nAccept:*/*\r\n\r\n",
        );
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), "localhost".into());
        headers_expected.insert("Accept".into(), "*/*".into());
        headers_expected.insert("User-Agent".into(), "curl/7.64.1".into());
        let req: HttpRequest = s.into();
        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }
}

// We need data structure 'HttpRequest' (type struct) which represents HTTP request
#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}
#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

// Parsing incoming HTTP request
impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        // Read each line in the incoming request
        for line in req.lines() {
            // If the line is a request line, call helper function 'process_req_line()' (see below)
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = resource;
                // If the line is a header line, call helper function 'process_heder_line'
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
                // If it is a blank line, do nothing
            } else if line.len() == 0 {
                // If none of these, this supposed to be message body
            } else {
                parsed_msg_body = line;
            }
        }
        // Parse the incoming HTTP request into HttpRequest struct
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

// Helper function to parse request line
fn process_req_line(s: &str) -> (Method, Resource, Version) {
    // Parse the request line into individual parts separated by whitespaces
    let mut words = s.split_whitespace();
    // Extract the HTTP method as it's first part fothe request line
    let method = words.next().unwrap();
    // Extract the resource (URL, URI) from second part of the request line
    let resource = words.next().unwrap();
    // Extract the version from third part of the request line
    let version = words.next().unwrap();
    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

// Helper function to parse header lines
fn process_header_line(s: &str) -> (String, String) {
    // Parse the header line into words split by separator ':'
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    // Extract the key part of the header
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }
    // Extract the value part
    if let Some(v) = header_items.next() {
        value = v.to_string();
    }

    (key, value)
}

// We need data structure 'Version' (type enum) - specifies allowed values for HTTP versions
#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

// Similarly to Method, we add implementation for From trait
// Then we need trait implementation 'From<&str> for conversion incoming string slices
impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}
