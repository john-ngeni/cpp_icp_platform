//! Frontend configuration management with trait-based opt-in system.
//!
//! Provides optional traits that canisters can implement based on their needs.

use candid::Principal;
use ic_canister_core::config::Config;

/// Optional trait: Logging configuration.
///
/// Implement if your canister needs configurable log level.
pub trait LoggableConfig: Config {
    fn log_level(&self) -> &str;
}

/// Optional trait: Analytics integration.
///
/// Implement if your canister uses Google Analytics or similar.
pub trait AnalyticsConfig: Config {
    fn ga_measurement_id(&self) -> Option<&str>;
}

/// Optional trait: Consent management.
///
/// Implement if your canister uses UserCentrics or similar consent platform.
pub trait ConsentConfig: Config {
    fn usercentrics_settings_id(&self) -> Option<&str>;
}

/// Optional trait: Gateway integration.
///
/// Implement if your canister integrates with Web2 gateway (Stripe, etc.).
pub trait GatewayConfig: Config {
    fn gateway_url(&self) -> &str;
    fn gateway_principal(&self) -> Principal;
}

/// Optional trait: Backend canister integration.
///
/// Implement if your frontend calls IC backend canisters.
pub trait BackendConfig: Config {
    fn backend_canister_id(&self) -> &str;
    fn api_url(&self) -> &str;
}

/// Optional trait: Member portal link.
///
/// Implement if your canister (cpf_org) links to members portal (cpf_members).
pub trait MemberPortalConfig: Config {
    fn members_url(&self) -> &str;
}

/// Required trait: Environment JavaScript generation.
///
/// All frontend canisters MUST implement this to generate /env.js content.
///
/// ## Example
///
/// ```rust
/// use ic_assets_env::EnvJsConfig;
///
/// impl EnvJsConfig for MyConfig {
///     fn to_env_js(&self) -> String {
///         format!(r#"window.__ENV__ = {{
///   MY_FIELD: "{}"
/// }};"#, self.my_field)
///     }
/// }
/// ```
pub trait EnvJsConfig: Config {
    /// Generate JavaScript code for /env.js endpoint.
    ///
    /// Should create window.__ENV__ object with config values.
    fn to_env_js(&self) -> String;
}

/// Helper function to convert Option<String> to JavaScript value.
pub fn opt_string_to_js(opt: &Option<String>) -> String {
    opt.as_ref()
        .map(|s| format!("\"{}\"", s))
        .unwrap_or_else(|| "null".to_string())
}

/// Helper function to convert Option<&str> to JavaScript value.
pub fn opt_str_to_js(opt: Option<&str>) -> String {
    opt.map(|s| format!("\"{}\"", s))
        .unwrap_or_else(|| "null".to_string())
}
