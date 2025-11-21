//! # IC Assets Env
//!
//! Custom IC asset canister with runtime `/env.js` injection.
//!
//! ## Trait-Based Opt-In System
//!
//! ic-assets-env provides **optional traits** that canisters implement based on needs:
//!
//! - `LoggableConfig` - Log level configuration
//! - `AnalyticsConfig` - Google Analytics integration
//! - `ConsentConfig` - UserCentrics/consent management
//! - `GatewayConfig` - Web2 gateway integration (Stripe)
//! - `BackendConfig` - IC backend canister calls
//! - `MemberPortalConfig` - Link to members portal
//!
//! **Required:** `EnvJsConfig` - Generate /env.js for your specific config
//!
//! ## Example: cpf_org (Simple Public Site)
//!
//! ```rust,no_run
//! use ic_assets_env::{EnvJsConfig, LoggableConfig, MemberPortalConfig, AnalyticsConfig};
//! use ic_canister_core::config::Config;
//! use candid::{CandidType, Deserialize};
//!
//! #[derive(Clone, CandidType, Deserialize)]
//! pub struct CpfOrgConfig {
//!     pub log_level: String,
//!     pub members_url: String,
//!     pub ga_measurement_id: String,
//! }
//!
//! impl Config for CpfOrgConfig {}
//! impl LoggableConfig for CpfOrgConfig {
//!     fn log_level(&self) -> &str { &self.log_level }
//! }
//! impl MemberPortalConfig for CpfOrgConfig {
//!     fn members_url(&self) -> &str { &self.members_url }
//! }
//! impl AnalyticsConfig for CpfOrgConfig {
//!     fn ga_measurement_id(&self) -> Option<&str> { Some(&self.ga_measurement_id) }
//! }
//! impl EnvJsConfig for CpfOrgConfig {
//!     fn to_env_js(&self) -> String {
//!         format!("window.__ENV__ = {{ MEMBERS_URL: \"{}\" }}", self.members_url)
//!     }
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
pub mod http;
pub mod store;

// Re-export traits
pub use env::{
    AnalyticsConfig, BackendConfig, ConsentConfig, EnvJsConfig, GatewayConfig, LoggableConfig,
    MemberPortalConfig, opt_str_to_js, opt_string_to_js,
};

// Re-export core types
pub use ic_canister_core::config::Config;
pub use ic_canister_core::http::{HttpRequest, HttpResponse};

// Note: No generic init/post_upgrade/http_request functions exported
// Each canister implements these directly with their specific config type
