# Canister Runtime Configuration

**Purpose:** Define patterns for environment-agnostic canister builds with runtime configuration injection
**Audience:** Technical team, DevOps engineers
**Cross-Reference:** [web2-secrets.md](./web2-secrets.md), [canister-architecture-diagram.md](./canister-architecture-diagram.md)

---

## Problem Statement

**Goal:** Build canisters once and promote the same WASM across environments (dev, staging, production) with different runtime configurations - similar to Docker containers with environment variables.

**Challenge:**
- Backend canisters (Motoko/Rust) support init args ✅
- Asset canisters do NOT support init args ❌
- Need consistent approach for both backend and frontend canisters

---

## Backend Canisters: Init Args Pattern

### Standard Approach

Every canister has an **init function** called on `install_code` that accepts arbitrary init arguments:

```bash
dfx deploy my_canister --argument '(record {
  api_url = "https://dev-api.example.com";
  log_level = "debug";
})'
```

### Motoko Example

```motoko
actor class MyCanister(config : {
  apiUrl : Text;
  logLevel : Nat;
}) = this {

  stable var stateConfig = config;  // persisted across upgrades

  public query func getConfig() : async Text {
    stateConfig.apiUrl;
  };

  system func preupgrade() {
    // stateConfig already stable, nothing special needed
  };

  system func postupgrade() {
    // noop
  };
};
```

### Key Benefits

- **One WASM** build artifact
- Different `config` values per environment at install time
- Config stored in stable vars, survives code upgrades
- Closest equivalent to "injecting config at container start"

### Runtime Config Updates

Add a guarded setter for post-deployment config changes:

```motoko
public shared(msg) func updateConfig(newConfig : {
  apiUrl : Text;
  logLevel : Nat;
}) : async () {
  assert(msg.caller == env.adminPrincipal);
  stateConfig := newConfig;
};
```

---

## Frontend Asset Canisters: Custom Wrapper Pattern

### Why Standard Asset Canister Doesn't Work

The default `asset` canister on IC:
- Is a **static file server** only
- No environment awareness
- No configurable logic
- No ability to accept init args
- No dynamic asset generation
- It only knows: "store certified files and return them"

**This is intentional** - it's a certified CDN, not an app server.

### Solution: Custom Asset Canister with Runtime Env

Build a **custom Rust canister** that:
1. Wraps `ic-certified-assets` for certified static file serving
2. Accepts init args for environment config
3. Serves special endpoint `/env.js` with runtime config
4. Maintains full compatibility with standard asset API

---

## Architecture: Layered Crate Design

### Layer 1: Generic Canister Runtime (`ic-canister-core`)

**Purpose:** Reusable by *any* Rust canister for common patterns

**Modules:**
- `config.rs` - Generic config handling with init/upgrade/update
- `stable.rs` - Stable storage helpers
- `http.rs` - HTTP request/response types + simple router
- `auth.rs` - Authorization helpers (controller check, admin list)

**Example Usage:**

```rust
use ic_canister_core::{stable, config::EnvState};
use candid::{CandidType, Deserialize};

#[derive(Clone, CandidType, Deserialize)]
pub struct BackendConfig {
    pub api_base_url: String,
    pub log_level: String,
}

thread_local! {
    static CFG: std::cell::RefCell<EnvState<BackendConfig>> =
        std::cell::RefCell::new(EnvState::new());
}

#[ic_cdk::init]
fn init(config: BackendConfig) {
    CFG.with(|c| c.borrow_mut().init(config.clone()));
    stable::stable_save(&config);
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    let cfg: BackendConfig = stable::stable_restore();
    CFG.with(|c| c.borrow_mut().init(cfg));
}
```

---

### Layer 2: Asset Runtime (`ic-assets-env`)

**Purpose:** Extends `ic-canister-core` for frontend canisters

**Dependencies:**
- `ic-canister-core` (generic runtime)
- `ic-certified-assets` (standard asset serving)

**Modules:**
- `env.rs` - Frontend-specific config using core's `EnvState`
- `store.rs` - Wrapper around `ic-certified-assets`
- `http.rs` - Combines `/env.js` + static asset serving

**Config Structure:**

```rust
#[derive(Clone, CandidType, Deserialize)]
pub struct FrontendConfig {
    pub api_url: String,
    pub log_level: String,
    pub config_canister_id: Option<String>,
}
```

**HTTP Handler with `/env.js` Injection:**

```rust
use ic_canister_core::http::{HttpRequest, HttpResponse};

pub fn http_request(req: HttpRequest) -> HttpResponse {
    let path = req.url.split('?').next().unwrap_or("/");

    if path == "/env.js" {
        let cfg = env::get();
        let body = format!(
            "window.__ENV__ = {{
              API_URL: \"{}\",
              LOG_LEVEL: \"{}\",
              CONFIG_CANISTER_ID: {}
            }};",
            cfg.api_url,
            cfg.log_level,
            cfg.config_canister_id
                .as_ref()
                .map(|id| format!("\"{}\"", id))
                .unwrap_or("null".to_string()),
        );
        return HttpResponse {
            status_code: 200,
            headers: vec![("content-type".into(), "application/javascript".into())],
            body: body.into_bytes(),
        };
    }

    // Otherwise, delegate to asset store for static files
    store::serve_http(req)
}
```

---

### Layer 3: Application Canisters (Thin Glue)

**Backend Canister:**
```rust
use ic_canister_core::*;
// Use core crate only
```

**Frontend Canister:**
```rust
use ic_assets_env::{FrontendConfig, init as assets_init};

#[ic_cdk::init]
fn init(config: FrontendConfig) {
    assets_init(config);
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    ic_assets_env::post_upgrade();
}

#[ic_cdk::query]
fn http_request(req: ic_assets_env::HttpRequest) -> ic_assets_env::HttpResponse {
    ic_assets_env::http_request(req)
}
```

**Result:** Application canisters are tiny - most logic lives in reusable crates.

---

## Vite Frontend Integration (Svelte)

**Note:** CPP uses **Svelte** as the primary frontend framework. React is only used for fti_newsletter_archive Admin (legacy codebase).

### Project Structure

```
project/
  frontend/                  # Svelte + Vite app
    index.html
    src/
      main.ts               # Svelte app entry point
      App.svelte            # Root component
      lib/
        env.ts              # Helpers to read window.__ENV__
  canisters/
    env_assets/            # Rust custom asset canister
      src/
      Cargo.toml
  dfx.json
```

### index.html: Load `/env.js` Before Bundle

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>CPP Platform</title>
  </head>
  <body>
    <div id="app"></div>

    <!-- Runtime config from canister (served dynamically) -->
    <script src="/env.js"></script>

    <!-- Vite + Svelte entry point -->
    <script type="module" src="/src/main.ts"></script>
  </body>
</html>
```

### lib/env.ts: Type-Safe Runtime Config

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

export type RuntimeEnv = {
  apiUrl: string;
  logLevel: string;
  backendCanisterId?: string;
  gatewayUrl?: string;
};

export function getRuntimeEnv(): RuntimeEnv {
  const e = window.__ENV__ ?? {};
  return {
    apiUrl: e.API_URL ?? "",
    logLevel: e.LOG_LEVEL ?? "info",
    backendCanisterId: e.BACKEND_CANISTER_ID,
    gatewayUrl: e.GATEWAY_URL,
  };
}
```

### main.ts: Consume Runtime Config (Svelte)

```typescript
import App from './App.svelte';
import { getRuntimeEnv } from './lib/env';

const runtimeEnv = getRuntimeEnv();

const app = new App({
  target: document.getElementById('app')!,
  props: {
    runtimeEnv
  }
});

export default app;
```

### App.svelte: Use Runtime Config

```svelte
<script lang="ts">
  import type { RuntimeEnv } from './lib/env';

  export let runtimeEnv: RuntimeEnv;

  console.log('API URL:', runtimeEnv.apiUrl);
  console.log('Log Level:', runtimeEnv.logLevel);
</script>

<main>
  <h1>Cool Planet Platform</h1>
  <p>Connected to: {runtimeEnv.apiUrl}</p>
</main>
```

**Key Point:** Do NOT use `import.meta.env` for IC deployment - that's build-time. Use `window.__ENV__` for runtime config.

---

## CI/CD Workflow: Build Once, Deploy Many

### 1. Build Phase (Once)

```bash
# Frontend
cd frontend
npm run build      # outputs frontend/dist

# Backend
cd canisters/env_assets
cargo build --release --target wasm32-unknown-unknown
```

### 2. Deploy Per Environment (Dev/Stage/Prod)

**Dev Deployment:**
```bash
dfx deploy env_assets \
  --argument '(record {
    api_url = "https://dev-backend.ic0.app";
    log_level = "debug";
    config_canister_id = opt "aaaaa-aa";
  })'

ic-assets --canister env_assets sync ./frontend/dist
```

**Production Deployment:**
```bash
dfx deploy env_assets \
  --argument '(record {
    api_url = "https://backend.cpf.nft";
    log_level = "info";
    config_canister_id = opt "bbbbb-bb";
  })'

ic-assets --canister env_assets sync ./frontend/dist
```

**Same WASM, same frontend dist, only init config varies** → Docker-like behavior

---

## Local Development

### Option 1: Proxy to Local Canister

In `vite.config.ts`:

```typescript
export default defineConfig({
  server: {
    proxy: {
      "/env.js": "http://127.0.0.1:4943",
      "/api": "http://127.0.0.1:4943",
    },
  },
});
```

### Option 2: Use .env.local for Dev Only

- Purely for local development
- Doesn't affect IC deploys
- Vite uses `import.meta.env` locally, `window.__ENV__` on IC

---

## Mapping to CPP Platform Architecture

### Control Plane (governance.cpf.nft)

**Canister:** Governance canister (Motoko)

**Runtime Config:**
```motoko
{
  boardMemberPrincipals : [Principal];
  votingThresholds : { three_of_three_hours : Nat; two_of_three_hours : Nat };
  auditLogRetentionDays : Nat;
}
```

**Environments:**
- **Testnet:** Testing governance logic with fake board member principals
- **Mainnet:** Real board members, production thresholds

---

### Data Plane Canisters

**cpf_org (Public Website):**
- Custom asset canister with `/env.js`
- Config: SEO metadata, analytics consent banner settings, newsletter signup URL

**fti_newsletter_archive (Newsletters):**
- Custom asset canister
- Config: MailerLite API endpoint (public), newsletter list IDs, GDPR privacy policy URL

**cpf_members (Cool Planet App):**
- Rust backend canister
- Config: Stripe publishable key, payment bridge URL, KYC thresholds, canister IDs for cross-canister calls

---

## Comparison to Docker/Kubernetes

| Concern                     | Docker                           | IC Canister Equivalent                                |
| --------------------------- | -------------------------------- | ----------------------------------------------------- |
| Image                       | Docker image                     | Compiled WASM module                                  |
| Container start             | `docker run ...`                 | `install_code` + init args                            |
| Env vars                    | `-e FOO=bar` / `ENV`             | Init arguments / config canister                      |
| Persistent config           | Volume / mounted file            | Stable variables inside canister                      |
| Changing config             | New container / re-run           | `updateConfig` method call                            |
| Multiple environments       | Same image, different env vars   | Same WASM, different init args per env                |
| Frontend runtime config     | Build-time `ENV` in Dockerfile   | Runtime `/env.js` served by custom asset canister     |

---

## Benefits of This Approach

1. ✅ **Single build artifact** - One WASM for all environments
2. ✅ **Environment parity** - Dev/staging/prod run identical code
3. ✅ **Easy testing** - Test production WASM in testnet first
4. ✅ **Fast deployments** - No rebuild, just redeploy with new config
5. ✅ **Audit trail** - Config changes visible in canister upgrade history
6. ✅ **Type safety** - Config struct enforced by Candid types
7. ✅ **Upgrade safety** - Config persisted in stable vars across upgrades

---

## What Should Live in Runtime Config

### ✅ Safe for Runtime Config

- API URLs / canister IDs
- Log levels / debug flags
- Feature flags (enable donations, enable NFT awards)
- Public keys / publishable keys (Stripe publishable key)
- Derivation origins for Internet Identity
- KYC/AML thresholds
- GDPR compliance settings
- SEO metadata

### ❌ DO NOT Put in Runtime Config

- **Secret keys** (Stripe secret key, Resend API key) - See [web2-secrets.md](./web2-secrets.md)
- **Private keys** (wallet private keys, signing keys)
- **Database passwords** (N/A on IC, but principle applies)
- **OAuth client secrets**

**Why?** IC canisters are replicated state machines - node operators can potentially inspect memory. Runtime config should contain only **non-sensitive** data.

For secrets, see [web2-secrets.md](./web2-secrets.md) for proper handling patterns.

---

## Recommended Crate Structure

```
ic-canister-core/              # Generic runtime (any Rust canister)
  src/
    lib.rs
    config.rs                  # Generic EnvState<T: Config>
    stable.rs                  # Stable storage helpers
    http.rs                    # HTTP types + router
    auth.rs                    # Authorization helpers
  Cargo.toml

ic-assets-env/                 # Asset-specific runtime (frontend canisters)
  src/
    lib.rs                     # Facade for app canisters
    env.rs                     # FrontendConfig using core::EnvState
    store.rs                   # Wrapper around ic-certified-assets
    http.rs                    # Combines /env.js + static file serving
  Cargo.toml

examples/
  basic_app/                   # Example usage
    canister/
      src/main.rs              # Tiny glue using ic-assets-env
      Cargo.toml
    frontend/                  # Vite app consuming window.__ENV__
```

---

## Next Steps for CPP Platform

1. **Implement `ic-canister-core` crate** (generic patterns)
2. **Implement `ic-assets-env` crate** (asset-specific wrapper)
3. **Migrate cpf_org to custom asset canister** (public website)
4. **Migrate fti_newsletter_archive to custom asset canister** (newsletters)
5. **Update cpf_members backend** to use `ic-canister-core`
6. **Define config schemas per environment** (dev, staging, production)
7. **CI/CD pipeline** to build once, deploy with env-specific config

---

**Last Updated:** 2025-11-17
**Version:** 1.0.0
**Status:** Architecture pattern for runtime config
**Related:** [web2-secrets.md](./web2-secrets.md), [canister-architecture-diagram.md](./canister-architecture-diagram.md)
