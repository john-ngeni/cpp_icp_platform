//! # IC Canister Core
//!
//! Generic runtime configuration patterns for Internet Computer canisters.
//!
//! ## Overview
//!
//! This library provides reusable patterns for building environment-agnostic
//! canisters that can be deployed with different runtime configurations,
//! similar to Docker containers with environment variables.
//!
//! ## Key Features
//!
//! - **Runtime Configuration**: Pass config via init args, store in stable vars
//! - **Stable Storage**: Helpers for persisting state across upgrades
//! - **HTTP Utilities**: Request/response types and simple routing
//! - **Authorization**: Controller checks and admin list management
//!
//! ## Usage
//!
//! ```rust,no_run
//! use ic_canister_core::config::{Config, EnvState};
//! use candid::{CandidType, Deserialize};
//!
//! #[derive(Clone, CandidType, Deserialize)]
//! pub struct MyConfig {
//!     pub api_url: String,
//!     pub log_level: String,
//! }
//!
//! impl Config for MyConfig {}
//!
//! // In your canister:
//! thread_local! {
//!     static STATE: std::cell::RefCell<EnvState<MyConfig>> =
//!         std::cell::RefCell::new(EnvState::new());
//! }
//!
//! #[ic_cdk::init]
//! fn init(config: MyConfig) {
//!     STATE.with(|s| s.borrow_mut().init(config));
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

pub mod config;
pub mod stable;
pub mod http;
pub mod auth;

// Re-export commonly used types
pub use config::{Config, EnvState};
pub use http::{HttpRequest, HttpResponse};
pub use auth::{is_controller, is_admin};
