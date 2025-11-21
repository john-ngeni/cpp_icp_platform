# ic-canister-core

Generic runtime configuration patterns for Internet Computer canisters.

## Overview

Build environment-agnostic canisters that can be deployed with different runtime configurations, similar to Docker containers with environment variables.

**Pattern:** Build once, deploy many with different init args.

## Features

- **Runtime Configuration** - Pass config via init args, store in stable vars
- **Stable Storage** - Helpers for persisting state across upgrades
- **HTTP Utilities** - Request/response types and simple routing
- **Authorization** - Controller checks and admin list management

## Quick Start

### 1. Add Dependency

```toml
[dependencies]
ic-canister-core = { path = "../sdk/ic-canister-core" }
```

### 2. Define Your Config

```rust
use ic_canister_core::config::{Config, EnvState};
use candid::{CandidType, Deserialize};

#[derive(Clone, CandidType, Deserialize)]
pub struct MyConfig {
    pub api_url: String,
    pub log_level: String,
    pub gateway_url: Option<String>,
}

impl Config for MyConfig {}
```

### 3. Initialize in Canister

```rust
use std::cell::RefCell;

thread_local! {
    static STATE: RefCell<EnvState<MyConfig>> = RefCell::new(EnvState::new());
}

#[ic_cdk::init]
fn init(config: MyConfig) {
    STATE.with(|s| s.borrow_mut().init(config));
}

#[ic_cdk::query]
fn get_config() -> MyConfig {
    STATE.with(|s| s.borrow().get().clone())
}
```

### 4. Deploy with Different Configs

```bash
# Preprod
dfx deploy my_canister --network preprod --argument '(record {
  api_url = "https://preprod-api.example.com";
  log_level = "debug";
  gateway_url = opt "https://gateway-preprod.example.com";
})'

# Production
dfx deploy my_canister --network production --argument '(record {
  api_url = "https://api.example.com";
  log_level = "info";
  gateway_url = opt "https://gateway.example.com";
})'
```

## Modules

### `config`
Runtime configuration management with `EnvState<T>` container.

### `stable`
Helpers for saving/restoring state to stable memory across upgrades.

### `http`
HTTP request/response types and simple router for serving HTTP content.

### `auth`
Authorization helpers for checking controllers and admin permissions.

## Example: Backend Canister with HTTP

```rust
use ic_canister_core::{
    config::{Config, EnvState},
    http::{HttpRequest, HttpResponse, Router},
    auth::require_controller,
};
use candid::{CandidType, Deserialize};
use std::cell::RefCell;

#[derive(Clone, CandidType, Deserialize)]
pub struct BackendConfig {
    pub api_url: String,
    pub log_level: String,
}

impl Config for BackendConfig {}

thread_local! {
    static STATE: RefCell<EnvState<BackendConfig>> =
        RefCell::new(EnvState::new());
}

#[ic_cdk::init]
fn init(config: BackendConfig) {
    STATE.with(|s| s.borrow_mut().init(config));
}

#[ic_cdk::query]
fn http_request(req: HttpRequest) -> HttpResponse {
    let router = Router::new(&req);

    if router.path("/health") {
        return HttpResponse::ok(b"OK");
    }

    if router.path("/config") {
        let config = STATE.with(|s| s.borrow().get().clone());
        let json = format!(r#"{{"api_url":"{}"}}"#, config.api_url);
        return HttpResponse::json(json.as_bytes());
    }

    HttpResponse::not_found()
}

#[ic_cdk::update]
fn update_config(new_config: BackendConfig) {
    require_controller(); // Only controllers can update
    STATE.with(|s| s.borrow_mut().update(new_config));
}
```

## Authors

**Fourth Transition Initiative** (IP owner)

## Operator

**Cool Planet Foundation** (platform operator, legal responsibility)

## License

MIT
