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
}

// We need data structure 'HttpRequest' (type struct) which represents HTTP request

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
