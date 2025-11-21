//! Test Frontend Canister
//!
//! Minimal canister demonstrating ic-assets-env usage.
//! Serves /env.js with runtime configuration for frontend consumption.

use ic_assets_env::{FrontendConfig, HttpRequest, HttpResponse};

/// Initialize canister with frontend configuration.
#[ic_cdk::init]
fn init(config: FrontendConfig) {
    ic_assets_env::init(config);
}

/// Post-upgrade handler.
#[ic_cdk::post_upgrade]
fn post_upgrade() {
    ic_assets_env::post_upgrade();
}

/// Serve HTTP requests.
///
/// Routes:
/// - /env.js - Runtime config as JavaScript
/// - /* - Static assets (placeholder in test canister)
#[ic_cdk::query]
fn http_request(req: HttpRequest) -> HttpResponse {
    ic_assets_env::http_request(req)
}

/// Get current frontend configuration (for testing).
#[ic_cdk::query]
fn get_config() -> FrontendConfig {
    ic_assets_env::env::get()
}

/// Update configuration (controllers only).
#[ic_cdk::update]
fn update_config(config: FrontendConfig) {
    ic_assets_env::env::update(config);
}

// Export Candid interface
ic_cdk::export_candid!();
