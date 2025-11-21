//! Test Backend Canister
//!
//! Minimal canister demonstrating ic-canister-core usage.
//! Tests runtime configuration, HTTP serving, and authorization.

use candid::{CandidType, Deserialize, Principal};
use ic_canister_core::{
    auth::{is_controller, require_controller},
    config::{Config, EnvState},
    http::{HttpRequest, HttpResponse, Router},
    stable,
};
use std::cell::RefCell;

/// Configuration for this test canister.
#[derive(Clone, CandidType, Deserialize)]
pub struct TestConfig {
    pub api_url: String,
    pub log_level: String,
    pub gateway_principal: Option<Principal>,
}

impl Config for TestConfig {}

thread_local! {
    static STATE: RefCell<EnvState<TestConfig>> = RefCell::new(EnvState::new());
}

/// Initialize canister with runtime configuration.
#[ic_cdk::init]
fn init(config: TestConfig) {
    STATE.with(|s| s.borrow_mut().init(config));
}

/// Save config to stable memory before upgrade.
#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let config = STATE.with(|s| s.borrow().get().clone());
    stable::stable_save(&config).expect("Failed to save config to stable memory");
}

/// Restore config from stable memory after upgrade.
#[ic_cdk::post_upgrade]
fn post_upgrade() {
    let config: TestConfig = stable::stable_restore().expect("Failed to restore config");
    STATE.with(|s| s.borrow_mut().init(config));
}

/// Get current configuration (query method).
#[ic_cdk::query]
fn get_config() -> TestConfig {
    STATE.with(|s| s.borrow().get().clone())
}

/// Update configuration (only controllers can call).
#[ic_cdk::update]
fn update_config(new_config: TestConfig) {
    require_controller();
    STATE.with(|s| s.borrow_mut().update(new_config));
}

/// Serve HTTP requests.
#[ic_cdk::query]
fn http_request(req: HttpRequest) -> HttpResponse {
    let router = Router::new(&req);

    // Health check endpoint
    if router.path("/health") {
        return HttpResponse::ok(b"OK");
    }

    // Config endpoint (returns current config as JSON)
    if router.path("/config") {
        let config = STATE.with(|s| s.borrow().get().clone());
        let json = serde_json::json!({
            "api_url": config.api_url,
            "log_level": config.log_level,
            "gateway_principal": config.gateway_principal.map(|p| p.to_text()),
        });
        return HttpResponse::json(json.to_string().as_bytes());
    }

    // Caller info endpoint
    if router.path("/whoami") {
        let caller = ic_cdk::caller();
        let is_ctrl = is_controller(caller);
        let json = serde_json::json!({
            "principal": caller.to_text(),
            "is_controller": is_ctrl,
            "is_anonymous": caller == Principal::anonymous(),
        });
        return HttpResponse::json(json.to_string().as_bytes());
    }

    HttpResponse::not_found()
}

// Export Candid interface
ic_cdk::export_candid!();
