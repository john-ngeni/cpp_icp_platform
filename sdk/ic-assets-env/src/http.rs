//! HTTP request handling for asset canister with /env.js injection.
//!
//! Provides generic request handler that works with any config implementing EnvJsConfig.

use ic_canister_core::{
    config::{Config, EnvState},
    http::{HttpRequest, HttpResponse, Router},
};
use std::cell::RefCell;

use crate::env::EnvJsConfig;

/// Handle HTTP request with generic config type.
///
/// Routes:
/// - `/env.js` - Dynamically generated from config.to_env_js()
/// - `/*` - Static assets from store
///
/// ## Usage
///
/// ```rust,no_run
/// use ic_assets_env::http::handle_request_with_state;
/// use ic_canister_core::config::EnvState;
/// use std::cell::RefCell;
///
/// thread_local! {
///     static STATE: RefCell<EnvState<MyConfig>> = RefCell::new(EnvState::new());
/// }
///
/// #[ic_cdk::query]
/// fn http_request(req: HttpRequest) -> HttpResponse {
///     handle_request_with_state(req, &STATE)
/// }
/// ```
pub fn handle_request_with_state<T: EnvJsConfig>(
    req: HttpRequest,
    state: &'static std::thread::LocalKey<RefCell<EnvState<T>>>,
) -> HttpResponse {
    let router = Router::new(&req);

    // Special route: /env.js (dynamically generated from config)
    if router.path("/env.js") {
        let env_js = state.with(|s| {
            let config = s.borrow().get().clone();
            config.to_env_js()
        });

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
