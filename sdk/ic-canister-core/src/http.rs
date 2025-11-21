//! HTTP request/response types and simple routing helpers.
//!
//! Provides types compatible with IC HTTP gateway for serving HTTP content
//! from canisters.

use candid::{CandidType, Deserialize};

/// HTTP request from IC HTTP gateway.
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
}

/// HTTP response to IC HTTP gateway.
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
}

impl HttpResponse {
    /// Create a 200 OK response with text/plain content.
    pub fn ok(body: impl Into<Vec<u8>>) -> Self {
        Self {
            status_code: 200,
            headers: vec![("content-type".to_string(), "text/plain".to_string())],
            body: body.into(),
        }
    }

    /// Create a 200 OK response with application/json content.
    pub fn json(body: impl Into<Vec<u8>>) -> Self {
        Self {
            status_code: 200,
            headers: vec![("content-type".to_string(), "application/json".to_string())],
            body: body.into(),
        }
    }

    /// Create a 200 OK response with application/javascript content.
    pub fn javascript(body: impl Into<Vec<u8>>) -> Self {
        Self {
            status_code: 200,
            headers: vec![(
                "content-type".to_string(),
                "application/javascript".to_string(),
            )],
            body: body.into(),
        }
    }

    /// Create a 404 Not Found response.
    pub fn not_found() -> Self {
        Self {
            status_code: 404,
            headers: vec![("content-type".to_string(), "text/plain".to_string())],
            body: b"Not Found".to_vec(),
        }
    }

    /// Create a 400 Bad Request response.
    pub fn bad_request(message: impl Into<Vec<u8>>) -> Self {
        Self {
            status_code: 400,
            headers: vec![("content-type".to_string(), "text/plain".to_string())],
            body: message.into(),
        }
    }

    /// Create a 500 Internal Server Error response.
    pub fn internal_error(message: impl Into<Vec<u8>>) -> Self {
        Self {
            status_code: 500,
            headers: vec![("content-type".to_string(), "text/plain".to_string())],
            body: message.into(),
        }
    }

    /// Add a header to the response.
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }
}

/// Simple router for matching HTTP paths.
///
/// ## Example
///
/// ```rust,no_run
/// use ic_canister_core::http::{HttpRequest, HttpResponse, Router};
///
/// fn handle_request(req: HttpRequest) -> HttpResponse {
///     let router = Router::new(&req);
///
///     if router.path("/health") {
///         return HttpResponse::ok(b"OK");
///     }
///
///     if router.path("/api/config") {
///         return HttpResponse::json(b"{\"status\":\"ok\"}");
///     }
///
///     HttpResponse::not_found()
/// }
/// ```
pub struct Router<'a> {
    request: &'a HttpRequest,
    matched_path: String,
}

impl<'a> Router<'a> {
    pub fn new(request: &'a HttpRequest) -> Self {
        // Extract path from URL (remove query string)
        let path = request
            .url
            .split('?')
            .next()
            .unwrap_or("/")
            .to_string();

        Self {
            request,
            matched_path: path,
        }
    }

    /// Check if the request path matches.
    pub fn path(&self, pattern: &str) -> bool {
        self.matched_path == pattern
    }

    /// Check if the request path starts with a prefix.
    pub fn prefix(&self, pattern: &str) -> bool {
        self.matched_path.starts_with(pattern)
    }

    /// Check if the request method matches.
    pub fn method(&self, method: &str) -> bool {
        self.request.method.eq_ignore_ascii_case(method)
    }

    /// Check if request matches both path and method.
    pub fn route(&self, method: &str, path: &str) -> bool {
        self.method(method) && self.path(path)
    }

    /// Get the request path.
    pub fn get_path(&self) -> &str {
        &self.matched_path
    }

    /// Get a header value by name (case-insensitive).
    pub fn get_header(&self, name: &str) -> Option<&str> {
        self.request
            .headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_builders() {
        let resp = HttpResponse::ok(b"test");
        assert_eq!(resp.status_code, 200);
        assert_eq!(resp.body, b"test");

        let resp = HttpResponse::not_found();
        assert_eq!(resp.status_code, 404);

        let resp = HttpResponse::json(b"{}");
        assert_eq!(resp.status_code, 200);
        assert!(resp
            .headers
            .iter()
            .any(|(k, v)| k == "content-type" && v == "application/json"));
    }

    #[test]
    fn test_router_path_matching() {
        let req = HttpRequest {
            method: "GET".to_string(),
            url: "/api/config".to_string(),
            headers: vec![],
            body: vec![],
        };

        let router = Router::new(&req);
        assert!(router.path("/api/config"));
        assert!(!router.path("/api/status"));
        assert!(router.prefix("/api"));
    }

    #[test]
    fn test_router_method_matching() {
        let req = HttpRequest {
            method: "POST".to_string(),
            url: "/api/submit".to_string(),
            headers: vec![],
            body: vec![],
        };

        let router = Router::new(&req);
        assert!(router.method("POST"));
        assert!(router.method("post")); // case insensitive
        assert!(!router.method("GET"));
    }

    #[test]
    fn test_router_route_matching() {
        let req = HttpRequest {
            method: "GET".to_string(),
            url: "/health?check=1".to_string(),
            headers: vec![],
            body: vec![],
        };

        let router = Router::new(&req);
        assert!(router.route("GET", "/health"));
        assert!(!router.route("POST", "/health"));
    }

    #[test]
    fn test_router_header() {
        let req = HttpRequest {
            method: "GET".to_string(),
            url: "/".to_string(),
            headers: vec![
                ("Content-Type".to_string(), "application/json".to_string()),
                ("Authorization".to_string(), "Bearer token".to_string()),
            ],
            body: vec![],
        };

        let router = Router::new(&req);
        assert_eq!(router.get_header("content-type"), Some("application/json"));
        assert_eq!(router.get_header("authorization"), Some("Bearer token"));
        assert_eq!(router.get_header("missing"), None);
    }
}
