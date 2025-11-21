//! # IC Assets Env
//!
//! Custom IC asset canister with runtime `/env.js` injection.
//!
//! ## Overview
//!
//! Extends `ic-canister-core` for frontend canisters by wrapping
//! `ic-certified-assets` and adding dynamic `/env.js` endpoint for
//! runtime configuration injection.
//!
//! ## Key Features
//!
//! - **Runtime Config Injection**: Serve `/env.js` with `window.__ENV__`
//! - **Certified Assets**: Wrap `ic-certified-assets` for static file serving
//! - **Single Build**: Same frontend dist deployed with different configs
//! - **Type-Safe Config**: Frontend config using `EnvState<FrontendConfig>`
//!
//! ## Usage
//!
//! ```rust,no_run
//! use ic_assets_env::{FrontendConfig, http_request, init, post_upgrade};
//!
//! #[ic_cdk::init]
//! fn canister_init(config: FrontendConfig) {
//!     init(config);
//! }
//!
//! #[ic_cdk::post_upgrade]
//! fn canister_post_upgrade() {
//!     post_upgrade();
//! }
//!
//! #[ic_cdk::query]
//! fn http_request(req: ic_assets_env::HttpRequest) -> ic_assets_env::HttpResponse {
//!     ic_assets_env::http_request(req)
//! }
//! ```
//!
//! ## Frontend Integration (Svelte)
//!
//! ```html
//! <!-- index.html -->
//! <script src="/env.js"></script>
//! <script type="module" src="/src/main.ts"></script>
//! ```
//!
//! ```typescript
//! // src/lib/env.ts
//! export function getRuntimeEnv() {
//!   return window.__ENV__ ?? {};
//! }
//! ```
//!
//! ## Authors
//!
//! Fourth Transition Initiative (IP owner)
//!
//! ## Operator
//!
//! Cool Planet Foundation (platform operator, legal responsibility)

pub mod env;
pub mod store;
pub mod http;

// Re-export commonly used types
pub use env::FrontendConfig;
pub use ic_canister_core::http::{HttpRequest, HttpResponse};

/// Initialize the asset canister with frontend configuration.
///
/// Should be called from `#[ic_cdk::init]`.
pub fn init(config: FrontendConfig) {
    env::init(config);
    store::init();
}

/// Post-upgrade handler.
///
/// Should be called from `#[ic_cdk::post_upgrade]`.
pub fn post_upgrade() {
    env::post_upgrade();
    store::post_upgrade();
}

/// HTTP request handler.
///
/// Serves `/env.js` for runtime config, delegates to asset store for static files.
///
/// Should be called from `#[ic_cdk::query]`.
pub fn http_request(req: HttpRequest) -> HttpResponse {
    http::handle_request(req)
}
