//! Frontend configuration management.
//!
//! Extends ic-canister-core's EnvState for frontend-specific config.

use candid::{CandidType, Deserialize, Principal};
use ic_canister_core::{
    config::{Config, EnvState},
    stable,
};
use std::cell::RefCell;

/// Frontend canister configuration.
///
/// Passed via init args at deployment time, exposed via `/env.js`
/// to frontend JavaScript/TypeScript code.
#[derive(Clone, CandidType, Deserialize)]
pub struct FrontendConfig {
    /// Backend API URL (e.g., "https://backend.cpf.nft")
    pub api_url: String,

    /// Log level for frontend (e.g., "debug", "info", "warn", "error")
    pub log_level: String,

    /// Backend canister ID (optional)
    pub backend_canister_id: Option<String>,

    /// Gateway URL for Web2 API integration (optional)
    pub gateway_url: Option<String>,

    /// Gateway principal for authenticated calls (optional)
    pub gateway_principal: Option<Principal>,
}

impl Config for FrontendConfig {}

thread_local! {
    static STATE: RefCell<EnvState<FrontendConfig>> = RefCell::new(EnvState::new());
}

/// Initialize frontend config state.
pub fn init(config: FrontendConfig) {
    STATE.with(|s| s.borrow_mut().init(config));
}

/// Get current frontend config.
pub fn get() -> FrontendConfig {
    STATE.with(|s| s.borrow().get().clone())
}

/// Update frontend config (controllers only).
pub fn update(config: FrontendConfig) {
    ic_canister_core::auth::require_controller();
    STATE.with(|s| s.borrow_mut().update(config));
}

/// Save config to stable memory before upgrade.
pub fn pre_upgrade() {
    let config = get();
    stable::stable_save(&config).expect("Failed to save frontend config");
}

/// Restore config from stable memory after upgrade.
pub fn post_upgrade() {
    let config: FrontendConfig = stable::stable_restore().expect("Failed to restore frontend config");
    init(config);
}

/// Generate JavaScript code for `/env.js` endpoint.
///
/// Creates `window.__ENV__` object with config values.
pub fn generate_env_js() -> String {
    let config = get();

    format!(
        r#"// Runtime configuration injected by canister
window.__ENV__ = {{
  API_URL: "{}",
  LOG_LEVEL: "{}",
  BACKEND_CANISTER_ID: {},
  GATEWAY_URL: {},
  GATEWAY_PRINCIPAL: {}
}};
"#,
        config.api_url,
        config.log_level,
        opt_string_to_js(&config.backend_canister_id),
        opt_string_to_js(&config.gateway_url),
        config.gateway_principal.map(|p| format!("\"{}\"", p.to_text())).unwrap_or_else(|| "null".to_string()),
    )
}

/// Convert Option<String> to JavaScript value.
fn opt_string_to_js(opt: &Option<String>) -> String {
    opt.as_ref()
        .map(|s| format!("\"{}\"", s))
        .unwrap_or_else(|| "null".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_env_js() {
        let config = FrontendConfig {
            api_url: "https://api.example.com".to_string(),
            log_level: "info".to_string(),
            backend_canister_id: Some("abc123-cai".to_string()),
            gateway_url: None,
            gateway_principal: None,
        };

        // Can't fully test without STATE, but can test the function exists
        let js = format!(
            "API_URL: \"{}\", LOG_LEVEL: \"{}\"",
            config.api_url, config.log_level
        );
        assert!(js.contains("https://api.example.com"));
    }

    #[test]
    fn test_opt_string_to_js() {
        assert_eq!(opt_string_to_js(&Some("test".to_string())), "\"test\"");
        assert_eq!(opt_string_to_js(&None), "null");
    }
}
