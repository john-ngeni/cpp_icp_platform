//! Test Frontend Canister
//!
//! Demonstrates trait-based config system for frontend canisters.

use candid::{CandidType, Principal};
use ic_assets_env::{
    opt_str_to_js, BackendConfig, Config, EnvJsConfig, GatewayConfig, HttpRequest, HttpResponse,
    LoggableConfig,
};
use ic_canister_core::config::EnvState;
use serde::Deserialize;
use std::cell::RefCell;

/// Test frontend configuration.
///
/// Demonstrates implementing multiple optional traits.
#[derive(Clone, CandidType, Deserialize)]
pub struct TestFrontendConfig {
    pub log_level: String,
    pub backend_canister_id: String,
    pub gateway_url: String,
    pub gateway_principal: Principal,
}

impl Config for TestFrontendConfig {}

impl LoggableConfig for TestFrontendConfig {
    fn log_level(&self) -> &str {
        &self.log_level
    }
}

impl BackendConfig for TestFrontendConfig {
    fn backend_canister_id(&self) -> &str {
        &self.backend_canister_id
    }

    fn api_url(&self) -> &str {
        "https://api.example.com" // Placeholder
    }
}

impl GatewayConfig for TestFrontendConfig {
    fn gateway_url(&self) -> &str {
        &self.gateway_url
    }

    fn gateway_principal(&self) -> Principal {
        self.gateway_principal
    }
}

impl EnvJsConfig for TestFrontendConfig {
    fn to_env_js(&self) -> String {
        format!(
            r#"// Test frontend runtime config
window.__ENV__ = {{
  LOG_LEVEL: "{}",
  BACKEND_CANISTER_ID: "{}",
  GATEWAY_URL: "{}",
  GATEWAY_PRINCIPAL: "{}"
}};
"#,
            self.log_level,
            self.backend_canister_id,
            self.gateway_url,
            self.gateway_principal.to_text()
        )
    }
}

thread_local! {
    static STATE: RefCell<EnvState<TestFrontendConfig>> = RefCell::new(EnvState::new());
}

#[ic_cdk::init]
fn init(config: TestFrontendConfig) {
    STATE.with(|s| s.borrow_mut().init(config));
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let config = STATE.with(|s| s.borrow().get().clone());
    ic_canister_core::stable::stable_save(&config).expect("Failed to save config");
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    let config: TestFrontendConfig =
        ic_canister_core::stable::stable_restore().expect("Failed to restore config");
    STATE.with(|s| s.borrow_mut().init(config));
}

#[ic_cdk::query]
fn http_request(req: HttpRequest) -> HttpResponse {
    ic_assets_env::http::handle_request_with_state(req, &STATE)
}

#[ic_cdk::query]
fn get_config() -> TestFrontendConfig {
    STATE.with(|s| s.borrow().get().clone())
}

#[ic_cdk::update]
fn update_config(config: TestFrontendConfig) {
    ic_canister_core::auth::require_controller();
    STATE.with(|s| s.borrow_mut().update(config));
}

ic_cdk::export_candid!();
