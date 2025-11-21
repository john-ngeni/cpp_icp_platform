# ic-assets-env

Custom IC asset canister with runtime `/env.js` injection for frontend configuration.

## Overview

Extends `ic-canister-core` for frontend canisters. Wraps `ic-certified-assets` and adds dynamic `/env.js` endpoint for injecting runtime configuration into Svelte/React applications.

**Pattern:** Build frontend once, deploy with different runtime configs.

## Features

- **Runtime `/env.js` Endpoint** - Dynamically generated JavaScript with `window.__ENV__`
- **Certified Asset Serving** - Wraps `ic-certified-assets` for static files
- **Type-Safe Config** - Uses `FrontendConfig` with `EnvState<T>`
- **Single Build Artifact** - Same frontend dist with different configs per environment

## Quick Start

### 1. Add Dependency

```toml
[dependencies]
ic-assets-env = { path = "../sdk/ic-assets-env" }
```

### 2. Canister Implementation

```rust
use ic_assets_env::{FrontendConfig, HttpRequest, HttpResponse};

#[ic_cdk::init]
fn init(config: FrontendConfig) {
    ic_assets_env::init(config);
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    ic_assets_env::post_upgrade();
}

#[ic_cdk::query]
fn http_request(req: HttpRequest) -> HttpResponse {
    ic_assets_env::http_request(req)
}

ic_cdk::export_candid!();
```

### 3. Frontend Integration (Svelte)

**index.html:**
```html
<!doctype html>
<html>
  <head><title>CPP Platform</title></head>
  <body>
    <div id="app"></div>

    <!-- Load runtime config BEFORE app bundle -->
    <script src="/env.js"></script>
    <script type="module" src="/src/main.ts"></script>
  </body>
</html>
```

**src/lib/env.ts:**
```typescript
declare global {
  interface Window {
    __ENV__?: {
      API_URL?: string;
      LOG_LEVEL?: string;
      BACKEND_CANISTER_ID?: string;
      GATEWAY_URL?: string;
    };
  }
}

export function getRuntimeEnv() {
  return window.__ENV__ ?? {
    API_URL: '',
    LOG_LEVEL: 'info'
  };
}
```

**src/main.ts:**
```typescript
import App from './App.svelte';
import { getRuntimeEnv } from './lib/env';

const env = getRuntimeEnv();

const app = new App({
  target: document.getElementById('app')!,
  props: { runtimeEnv: env }
});
```

### 4. Deploy with Different Configs

```bash
# Preprod
dfx deploy frontend --network preprod --argument '(record {
  api_url = "https://preprod-api.cpf.nft";
  log_level = "debug";
  backend_canister_id = opt "preprod-backend-id";
  gateway_url = opt "https://gateway-preprod.cpf.workers.dev";
  gateway_principal = opt principal "xxxxx-preprod";
})'

# Production
dfx deploy frontend --network production --argument '(record {
  api_url = "https://api.cpf.nft";
  log_level = "info";
  backend_canister_id = opt "production-backend-id";
  gateway_url = opt "https://gateway.cpf.workers.dev";
  gateway_principal = opt principal "xxxxx-production";
})'
```

## What Gets Served

**GET /env.js:**
```javascript
// Dynamically generated from canister config
window.__ENV__ = {
  API_URL: "https://api.cpf.nft",
  LOG_LEVEL: "info",
  BACKEND_CANISTER_ID: "production-backend-id",
  GATEWAY_URL: "https://gateway.cpf.workers.dev",
  GATEWAY_PRINCIPAL: "xxxxx-production"
};
```

**GET /index.html:**
Served from ic-certified-assets (static files uploaded during deployment)

## Key Differences from Vite .env Pattern

| Vite .env Pattern | ic-assets-env Pattern |
|-------------------|----------------------|
| Build-time (`import.meta.env`) | Runtime (`window.__ENV__`) |
| Multiple builds (.env.development, .env.production) | Single build, different init args |
| Baked into bundle | Loaded before bundle |
| Rebuild to change config | Redeploy with new init args |

## Authors

**Fourth Transition Initiative** (IP owner)

## Operator

**Cool Planet Foundation** (platform operator, legal responsibility)

## License

MIT
