//! HTTP request handling for asset canister with /env.js injection.
//!
//! Routes requests to either /env.js (dynamic) or asset store (static).

use ic_canister_core::http::{HttpRequest, HttpResponse, Router};

/// Handle HTTP request: serve /env.js or delegate to asset store.
///
/// ## Routes
///
/// - `/env.js` - Dynamically generated runtime config
/// - `/*` - Static assets from ic-certified-assets
pub fn handle_request(req: HttpRequest) -> HttpResponse {
    let router = Router::new(&req);

    // Special route: /env.js (dynamically generated from config)
    if router.path("/env.js") {
        let env_js = crate::env::generate_env_js();
        return HttpResponse::javascript(env_js.as_bytes())
            .with_header("cache-control", "no-cache, no-store, must-revalidate");
    }

    // All other routes: serve from asset store
    crate::store::serve_http(req)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_js_route() {
        let req = HttpRequest {
            method: "GET".to_string(),
            url: "/env.js".to_string(),
            headers: vec![],
            body: vec![],
        };

        let router = Router::new(&req);
        assert!(router.path("/env.js"));
    }

    #[test]
    fn test_static_asset_route() {
        let req = HttpRequest {
            method: "GET".to_string(),
            url: "/index.html".to_string(),
            headers: vec![],
            body: vec![],
        };

        let router = Router::new(&req);
        assert!(!router.path("/env.js"));
    }
}
