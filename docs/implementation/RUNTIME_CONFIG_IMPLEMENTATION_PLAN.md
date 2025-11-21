# Runtime Config Implementation Plan

**Purpose:** Plan for implementing reusable runtime config libraries and establishing deployment patterns for CPP platform
**Status:** Planning phase
**Date:** 2025-11-17

---

## Overview

Transform runtime-config.md architecture into reusable Rust crates, establish deployment patterns, and solve the bootstrap coordination problem between on-chain canisters and off-chain gateway.

---

## IP Ownership and Operational Responsibility

**Critical Distinction for Bootstrap:**

**Authors (IP Owner):**
- **Fourth Transition Initiative (FTI)**
- Owns intellectual property for SDK libraries
- Authors of ic-canister-core and ic-assets-env crates
- Copyright holder for reusable infrastructure code

**Operator (Legal Responsibility):**
- **Cool Planet Foundation (CPF)**
- Legal responsibility for platform operations
- Manages regulatory compliance (KYC/AML, tax reporting, ANBI status)
- Operates canisters and infrastructure

**Bootstrap Questions to Clarify:**
1. Who controls canister upgrades? (CPF or FTI-governed DAO?)
2. Who holds ENS domain names? (cpf.nft owned by whom?)
3. What is the licensing model for derivative works?
4. How are governance decisions made about IP?
5. Can CPF use FTI's SDK libraries under what terms?
6. What happens if FTI/CPF relationship changes?

**Proposed Model:**
- FTI owns SDK libraries (MIT licensed, open source)
- CPF operates platform using these libraries
- Governance canister controls upgrades (board voting)
- ENS domains held by CPF foundation (operational control)
- IP licensing formalized in foundation documents

This distinction must be documented in bootstrap process and governance policy.

---

## Phase 1: SDK Library Structure

### 1.1 Create ic-canister-core Crate

**Location:** `sdk/ic-canister-core/`

**Purpose:** Generic runtime config patterns for ANY Rust canister

**Modules:**
```rust
sdk/ic-canister-core/
├── Cargo.toml
├── src/
│   ├── lib.rs              // Public API
│   ├── config.rs           // EnvState<T: Config> trait
│   ├── stable.rs           // Stable storage helpers
│   ├── http.rs             // HttpRequest/HttpResponse types
│   └── auth.rs             // Authorization helpers
└── examples/
    └── basic_backend.rs    // Example backend canister
```

**Key Types:**
```rust
// config.rs
pub trait Config: CandidType + for<'de> Deserialize<'de> + Clone + 'static {}

pub struct EnvState<T: Config> {
    inner: Option<T>,
}

impl<T: Config> EnvState<T> {
    pub fn init(&mut self, cfg: T);
    pub fn get(&self) -> &T;
    pub fn update(&mut self, cfg: T);
}
```

**Dependencies:**
- `candid`
- `ic-cdk`
- `serde`

### 1.2 Create ic-assets-env Crate

**Location:** `sdk/ic-assets-env/`

**Purpose:** Custom asset canister with `/env.js` runtime config injection

**Modules:**
```rust
sdk/ic-assets-env/
├── Cargo.toml
├── src/
│   ├── lib.rs              // Public API
│   ├── env.rs              // FrontendConfig using core::EnvState
│   ├── store.rs            // Wrapper around ic-certified-assets
│   └── http.rs             // /env.js + static file serving
└── examples/
    └── basic_frontend.rs   // Example frontend canister
```

**Key Types:**
```rust
// env.rs
#[derive(Clone, CandidType, Deserialize)]
pub struct FrontendConfig {
    pub api_url: String,
    pub log_level: String,
    pub backend_canister_id: Option<String>,
    pub gateway_url: Option<String>,
    pub gateway_principal: Option<Principal>,
}
```

**Dependencies:**
- `ic-canister-core` (internal dependency)
- `ic-certified-assets`
- `candid`
- `ic-cdk`

---

## Phase 2: Bootstrap Coordination Problem

### The Chicken-and-Egg Challenge

**Problem:** Canisters and gateway need to know about each other, but what gets deployed first?

```
Canister needs:
├── gateway_url (to call for webhooks)
└── gateway_principal (to trust calls from gateway)

Gateway needs:
├── canister_id (to call canister methods)
└── ic_identity_key (to authenticate as principal)
```

### 2.1 Bootstrap Sequence (Recommended)

**Step 1: Generate Gateway Identity (Offline)**
```bash
# One-time setup per environment
dfx identity new gateway-preprod
dfx identity new gateway-production

# Get principals
GATEWAY_PREPROD_PRINCIPAL=$(dfx identity get-principal --identity gateway-preprod)
GATEWAY_PROD_PRINCIPAL=$(dfx identity get-principal --identity gateway-production)

# Export private keys to secrets manager
dfx identity export gateway-preprod > /tmp/gateway-preprod.pem
# Store in Cloudflare Workers secrets / AWS Secrets Manager
rm /tmp/gateway-preprod.pem
```

**Step 2: Deploy Canisters with Placeholder Gateway URL**
```bash
# Deploy with known principal but TBD URL
dfx deploy cpf_members --network preprod --argument '(record {
  api_url = "https://preprod-backend.ic0.app";
  gateway_url = "https://gateway-preprod.example.com";  // Placeholder
  gateway_principal = principal "'$GATEWAY_PREPROD_PRINCIPAL'";
  log_level = "debug";
})'

# Get canister ID
CANISTER_ID=$(dfx canister id cpf_members --network preprod)
```

**Step 3: Deploy Gateway with Canister ID**
```bash
# Deploy Cloudflare Worker with env vars
wrangler deploy --env preprod

# Set secrets
wrangler secret put GATEWAY_IDENTITY_KEY --env preprod  # From Step 1
wrangler secret put STRIPE_SECRET_KEY --env preprod
wrangler secret put RESEND_API_KEY --env preprod

# Set public env vars
wrangler env:set BACKEND_CANISTER_ID "$CANISTER_ID" --env preprod
wrangler env:set IC_NETWORK "https://ic0.app" --env preprod

# Get actual gateway URL
GATEWAY_URL=$(wrangler deployments list --env preprod | grep "gateway-preprod")
```

**Step 4: Update Canister with Actual Gateway URL**
```bash
# Call updateConfig method on canister
dfx canister call cpf_members updateConfig --network preprod '(record {
  api_url = "https://preprod-backend.ic0.app";
  gateway_url = "'$GATEWAY_URL'";
  gateway_principal = principal "'$GATEWAY_PREPROD_PRINCIPAL'";
  log_level = "debug";
})'
```

### 2.2 Alternative: Two-Phase Init

**Canister Implementation:**
```rust
#[derive(Clone, CandidType, Deserialize)]
pub struct Config {
    pub api_url: String,
    pub gateway_principal: Principal,
    pub gateway_url: Option<String>,  // Optional during bootstrap
    pub log_level: String,
}

#[ic_cdk::update]
fn updateGatewayUrl(url: String) {
    let caller = ic_cdk::caller();
    let cfg = get_config();

    // Only allow update during bootstrap (gateway_url is None)
    // OR from controller
    assert!(
        cfg.gateway_url.is_none() || is_controller(caller),
        "Unauthorized: gateway URL already set"
    );

    update_config_gateway_url(url);
}
```

This allows:
1. Deploy canister with gateway principal (URL = None)
2. Deploy gateway with canister ID
3. Gateway calls canister's `updateGatewayUrl()` on first boot
4. Future updates require controller permission

---

## Phase 3: Infrastructure Config Repo

### 3.1 Repository Structure

**Option A: Separate cpp_infrastructure Repo**
```
cpp_infrastructure/
├── .github/
│   └── workflows/
│       ├── deploy-preprod.yml
│       └── deploy-production.yml
├── environments/
│   ├── preprod/
│   │   ├── governance.dhall
│   │   ├── cpf_members.dhall
│   │   ├── cpf_org.dhall
│   │   ├── fti_newsletter_archive.dhall
│   │   └── gateway.env
│   └── production/
│       ├── governance.dhall
│       ├── cpf_members.dhall
│       ├── cpf_org.dhall
│       ├── fti_newsletter_archive.dhall
│       └── gateway.env
├── scripts/
│   ├── bootstrap-environment.sh
│   ├── deploy-canisters.sh
│   ├── deploy-gateway.sh
│   └── update-gateway-urls.sh
└── README.md
```

**Option B: Private Directory in cpp_icp_platform**
```
cpp_icp_platform/
├── infrastructure/         # .gitignore this OR keep in private branch
│   ├── preprod/
│   └── production/
└── (rest of repo)
```

**Recommendation:** Option A (separate repo) for:
- Clear separation of code vs config
- Different access controls (infra repo can be private)
- Independent versioning of infrastructure changes

### 3.2 Config File Format

**governance.dhall:**
```dhall
let Principal = Text

in  { board_member_principals =
      [ "aaaaa-aa"
      , "bbbbb-bb"
      , "ccccc-cc"
      ]
    , voting_thresholds =
      { three_of_three_hours = 24
      , two_of_three_hours = 168
      , one_of_three_hours = 336
      }
    , audit_log_retention_days = 365
    }
```

**cpf_members.dhall:**
```dhall
{ api_url = "https://backend.cpf.nft"
, gateway_url = "https://gateway-preprod.cpf.workers.dev"
, gateway_principal = "xxxxx-xxxxx"
, stripe_publishable_key = "pk_test_..."
, log_level = "debug"
}
```

**gateway.env (NOT committed - secrets in Workers secrets):**
```bash
# Public config (committed)
BACKEND_CANISTER_ID=xyz-cai
IC_NETWORK=https://ic0.app

# Secrets (Cloudflare Workers secrets - NOT in git)
# wrangler secret put GATEWAY_IDENTITY_KEY
# wrangler secret put STRIPE_SECRET_KEY
# wrangler secret put STRIPE_WEBHOOK_SECRET
# Future Web2 APIs:
# wrangler secret put OTHER_API_KEY_IF_NEEDED
```

---

## Phase 4: Gateway Deployment Integration

### 4.1 Gateway Repository Structure

**Primary Focus:** Stripe integration (payments, webhooks)
**Extensible Design:** Can add other Web2 APIs as needed

**Location:** Could be in `cpp_icp_platform/infrastructure/gateway/` or separate repo

```
gateway/
├── src/
│   ├── index.ts                 # Worker entry point + router
│   ├── stripe/
│   │   ├── payments.ts          # Payment processing
│   │   ├── webhooks.ts          # Stripe webhook handling
│   │   └── types.ts             # Stripe-specific types
│   ├── ic-client.ts             # IC canister calls (authenticated)
│   ├── idempotency.ts           # Request deduplication (KV-based)
│   └── types.ts                 # Shared types
├── wrangler.toml                # Cloudflare Workers config
├── package.json
└── README.md
```

**Note on Future Web2 APIs:**
If email provider needed later:
```
├── src/
│   ├── email/                   # Add as needed
│   │   ├── provider.ts          # Email provider integration
│   │   └── templates.ts         # Email templates
```

Other potential Web2 integrations:
- KYC provider APIs (if external KYC needed)
- SMS provider (if 2FA needed)
- Analytics/monitoring services
```

**wrangler.toml:**
```toml
name = "cpp-gateway"
compatibility_date = "2024-01-01"

[env.preprod]
vars = { IC_NETWORK = "https://ic0.app" }

[env.production]
vars = { IC_NETWORK = "https://ic0.app" }
```

### 4.2 Gateway IC Client

**src/ic-client.ts:**
```typescript
import { Actor, HttpAgent } from '@dfinity/agent';
import { Ed25519KeyIdentity } from '@dfinity/identity-ed25519';
import { idlFactory as backendIdl } from './declarations/cpf_members';

export class ICClient {
  private agent: HttpAgent;
  private backend: Actor;

  async initialize() {
    // Load identity from Workers secret
    const identityKey = await env.GATEWAY_IDENTITY_KEY;
    const identity = Ed25519KeyIdentity.fromSecretKey(
      Buffer.from(identityKey, 'base64')
    );

    // Create agent with identity
    this.agent = new HttpAgent({
      identity,
      host: env.IC_NETWORK,
    });

    // Create actor for backend canister
    this.backend = Actor.createActor(backendIdl, {
      agent: this.agent,
      canisterId: env.BACKEND_CANISTER_ID,
    });
  }

  async recordPayment(intentId: string, amount: bigint, currency: string) {
    // This call will be authenticated as gateway_principal
    return this.backend.record_payment(intentId, amount, currency);
  }

  async reportBootstrap() {
    // On first boot, tell canister our URL
    const gatewayUrl = `https://gateway-preprod.cpf.workers.dev`;
    return this.backend.updateGatewayUrl(gatewayUrl);
  }
}
```

### 4.3 Bootstrap Coordination Script

**scripts/bootstrap-environment.sh:**
```bash
#!/bin/bash
set -e

ENV=$1  # preprod or production

echo "Bootstrapping $ENV environment..."

# 1. Generate gateway identity (if not exists)
if ! dfx identity list | grep -q "gateway-$ENV"; then
  dfx identity new "gateway-$ENV"
fi

GATEWAY_PRINCIPAL=$(dfx identity get-principal --identity "gateway-$ENV")
echo "Gateway principal: $GATEWAY_PRINCIPAL"

# 2. Load canister configs
GOVERNANCE_CONFIG=$(dhall-to-json < "environments/$ENV/governance.dhall")
MEMBERS_CONFIG=$(dhall-to-json < "environments/$ENV/cpf_members.dhall")

# 3. Deploy canisters with placeholder gateway URL
echo "Deploying canisters..."
dfx deploy governance --network "$ENV" --argument "$GOVERNANCE_CONFIG"

# Inject gateway principal into members config
MEMBERS_WITH_GATEWAY=$(echo "$MEMBERS_CONFIG" | jq \
  --arg principal "$GATEWAY_PRINCIPAL" \
  '.gateway_principal = $principal')

dfx deploy cpf_members --network "$ENV" --argument "$MEMBERS_WITH_GATEWAY"

# 4. Get canister IDs
MEMBERS_CANISTER_ID=$(dfx canister id cpf_members --network "$ENV")
echo "Members canister: $MEMBERS_CANISTER_ID"

# 5. Deploy gateway with canister ID and secrets
echo "Deploying gateway to Cloudflare Workers..."
cd gateway

# Set public environment variables in wrangler.toml via script
cat > wrangler.toml.tmp <<EOF
[env.$ENV]
vars = {
  BACKEND_CANISTER_ID = "$MEMBERS_CANISTER_ID",
  IC_NETWORK = "https://ic0.app"
}
EOF

# Deploy to Cloudflare Workers
wrangler deploy --env "$ENV"

# Set secrets (encrypted at rest in Cloudflare)
echo "Setting gateway secrets..."

# Gateway IC identity (export from dfx identity)
GATEWAY_KEY_PEM=$(dfx identity export "gateway-$ENV" | base64)
echo "$GATEWAY_KEY_PEM" | wrangler secret put GATEWAY_IDENTITY_KEY --env "$ENV"

# Stripe secrets (from secure environment variables)
echo "$STRIPE_SECRET_KEY" | wrangler secret put STRIPE_SECRET_KEY --env "$ENV"
echo "$STRIPE_WEBHOOK_SECRET" | wrangler secret put STRIPE_WEBHOOK_SECRET --env "$ENV"

# Resend API key
echo "$RESEND_API_KEY" | wrangler secret put RESEND_API_KEY --env "$ENV"

# 6. Get actual gateway URL
GATEWAY_URL=$(wrangler deployments list --env "$ENV" | grep "gateway-$ENV" | head -1 | awk '{print $2}')
echo "Gateway deployed at: $GATEWAY_URL"

# 7. Update canister with actual gateway URL
echo "Updating canister with gateway URL..."
dfx canister call cpf_members updateGatewayUrl \
  --network "$ENV" \
  "(\"$GATEWAY_URL\")"

echo "✅ Bootstrap complete for $ENV environment"
```

---

## Phase 5: Deployment Workflow

### 5.1 Developer Workflow (Application Repos)

**In cpf_members/, cpf_org/, etc.:**

**DEPLOYMENT.md:**
```markdown
# Deployment Guide

## Build

Single build for all environments:
```bash
npm run build  # → dist/
cargo build --release --target wasm32-unknown-unknown
```

## Local Testing

```bash
dfx start --clean
dfx deploy --argument '(record {
  api_url = "http://localhost:4943";
  gateway_principal = principal "xxxxx-local";
  log_level = "debug";
})'
```

## Preprod/Production

Deployment is handled by cpp_infrastructure repo.

Config is in cpp_infrastructure/environments/{env}/*.dhall

See cpp_infrastructure/README.md for deployment process.
```

### 5.2 Infrastructure Workflow

**In cpp_infrastructure repo:**

**GitHub Actions (deploy-preprod.yml):**
```yaml
name: Deploy to Preprod

on:
  push:
    branches: [main]
    paths:
      - 'environments/preprod/**'

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install dfx
        run: sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"

      - name: Install dhall
        run: |
          wget https://github.com/dhall-lang/dhall-haskell/releases/download/1.42.0/dhall-1.42.0-x86_64-linux.tar.bz2
          tar -xf dhall-1.42.0-x86_64-linux.tar.bz2
          sudo mv bin/dhall /usr/local/bin/

      - name: Deploy canisters
        run: ./scripts/deploy-canisters.sh preprod
        env:
          DFX_IDENTITY: ${{ secrets.DFX_IDENTITY_PREPROD }}

      - name: Deploy gateway
        run: ./scripts/deploy-gateway.sh preprod
        env:
          CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
```

---

## Phase 6: Documentation Structure

### 6.1 In cpp_icp_platform

**docs/architecture/runtime-config.md** (updated)
- Reference sdk/ implementation
- Generic pattern explanation
- Reusable library architecture

**docs/architecture/web2-secrets.md** (updated)
- Gateway deployment patterns
- Bootstrap coordination
- Secrets management

**docs/implementation/deployment-guide.md** (NEW)
- How to consume sdk/ crates in your canister
- Svelte frontend integration with window.__ENV__
- Bootstrap sequence explanation
- Link to infrastructure repo

**sdk/README.md** (NEW)
- How to use ic-canister-core
- How to use ic-assets-env
- Example usage
- Testing guidelines

### 6.2 In Application Repos

**cpf_members/DEPLOYMENT.md**
**cpf_org/DEPLOYMENT.md**
**fti_newsletter_archive/DEPLOYMENT.md**

Each contains:
- Local development setup
- Build commands (single build)
- Link to cpp_infrastructure for actual deployment
- Config schema for this canister

### 6.3 In cpp_infrastructure Repo

**README.md**
- Bootstrap sequence
- How to deploy to preprod
- How to deploy to production
- Emergency rollback procedures
- Secrets management guide

---

## Repository Mapping

This plan will be executed across multiple repositories:

### cpp_icp_platform/ (This Repo - Library & Docs)
**Purpose:** Reusable libraries and architecture documentation

**Deliverables:**
- `sdk/ic-canister-core/` - Generic Rust runtime config library
- `sdk/ic-assets-env/` - Asset canister wrapper with /env.js
- `docs/architecture/` - Architecture patterns (runtime-config.md, web2-secrets.md)
- `docs/implementation/` - Deployment guides, this plan

**Does NOT contain:**
- Application code
- Environment-specific configs
- Actual canisters

---

### cpf_members/ (Application Repo)
**Purpose:** Member portal, donations, NFT management

**Deliverables:**
- Backend canister (Motoko) using `ic-canister-core` patterns
- Frontend (Svelte) consuming `window.__ENV__`
- **gateway/** (NEW) - Cloudflare Workers for Stripe integration
  - **Replaces:** Existing `payment_bridge` canister (Rust)
  - **Why:** Cloudflare Workers better suited for Web2 API integration
  - **Contains:** Stripe payments, webhooks, idempotency, IC client

**Dependencies:**
- `ic-canister-core` from cpp_icp_platform (via git submodule or package registry)

---

### cpf_org/ (Public Website)
**Purpose:** Public marketing site, newsletter signup

**Deliverables:**
- Frontend canister using `ic-assets-env` with /env.js
- SvelteKit static build

**Dependencies:**
- `ic-assets-env` from cpp_icp_platform

---

### fti_newsletter_archive/ (Newsletter Archive)
**Purpose:** Newsletter portal and admin

**Deliverables:**
- Frontend canister using `ic-assets-env`
- React Admin (legacy - keep as-is)
- Svelte public portal (if migrating)

**Dependencies:**
- `ic-assets-env` from cpp_icp_platform

---

### cpp_infrastructure/ (Deployment Configs - NEW)
**Purpose:** Environment-specific deployment configurations

**Options:**
- **Option A:** Separate private repository (recommended)
- **Option B:** Private directory in cpp_icp_platform (gitignored or separate branch)

**Deliverables:**
- `environments/preprod/*.dhall` - Preprod configs
- `environments/production/*.dhall` - Production configs
- `scripts/bootstrap-environment.sh` - Bootstrap orchestration
- GitHub Actions workflows for deployment

**Contains:**
- Canister init args per environment
- Gateway URLs per environment
- Board member principals
- Canister IDs (canister_ids.json per environment)

**Does NOT contain:**
- Secrets (those live in Cloudflare Workers secrets)
- Application code

---

## Implementation Phases

### Phase 1: Library Foundation (Week 1)
**Repository:** `cpp_icp_platform/`

- [ ] Create sdk/ic-canister-core/ skeleton
- [ ] Implement config.rs with EnvState<T>
- [ ] Implement stable.rs helpers
- [ ] Create example backend canister
- [ ] Write tests
- [ ] Publish crate (local or registry)

### Phase 2: Asset Canister (Week 2)
**Repository:** `cpp_icp_platform/`

- [ ] Create sdk/ic-assets-env/ skeleton
- [ ] Implement /env.js injection
- [ ] Wrap ic-certified-assets
- [ ] Create example frontend canister
- [ ] Write tests
- [ ] Publish crate (local or registry)

### Phase 3: Gateway Integration (Week 3)
**Repository:** `cpf_members/gateway/`

**Primary Focus:** Stripe integration, replaces payment_bridge canister

- [ ] Create gateway/ directory structure (Cloudflare Workers)
- [ ] Implement IC client with identity (authenticated calls to canister)
- [ ] Implement idempotency layer (Cloudflare KV for request deduplication)
- [ ] Implement Stripe payments integration
- [ ] Implement Stripe webhook handling
- [ ] Test gateway ↔ canister communication
- [ ] Remove old payment_bridge canister
- [ ] *Optional:* Email provider integration (defer decision)
- [ ] *Optional:* Other Web2 APIs as needed

### Phase 4: Bootstrap Tooling (Week 4)
**Repository:** `cpp_infrastructure/`

- [ ] Create infrastructure/ directory structure (or separate repo)
- [ ] Write bootstrap-environment.sh script
- [ ] Create example Dhall config files (governance, cpf_members, cpf_org)
- [ ] Document bootstrap sequence
- [ ] Test preprod deployment

### Phase 5: Migration (Week 5)
**Repositories:** `cpf_members/`, `cpf_org/`, `fti_newsletter_archive/`

- [ ] Update cpf_members backend to use ic-canister-core
- [ ] Update cpf_members frontend to use window.__ENV__
- [ ] Deploy gateway to Cloudflare Workers (preprod)
- [ ] Update cpf_org to use ic-assets-env
- [ ] Update fti_newsletter_archive to use ic-assets-env
- [ ] Migrate from VITE_ICP_PRACTICES pattern
- [ ] Test in preprod

### Phase 6: Production Deployment (Week 6)
**Repositories:** `cpp_infrastructure/`, all application repos

- [ ] Production bootstrap (cpp_infrastructure)
- [ ] Production gateway deployment (cpf_members/gateway to Cloudflare)
- [ ] Production canister updates (all repos)
- [ ] Monitoring setup (Cloudflare Analytics, canister metrics)
- [ ] Documentation finalization (cpp_icp_platform/docs)

---

## Open Questions

### Q1: Infrastructure Repo Location
- **Option A:** Separate cpp_infrastructure repo (recommended)
- **Option B:** Private directory in cpp_icp_platform
- **Decision needed:** Board input on repo structure

### Q2: Config File Format
- **Option A:** Dhall (type-safe, validated)
- **Option B:** JSON/YAML (simpler, less validation)
- **Decision needed:** Team preference

### Q3: Gateway Self-Registration
Should gateway call `canister.updateGatewayUrl()` on first boot, or require manual update?
- **Pro:** Fully automated bootstrap
- **Con:** Security concern - anyone with gateway identity can update URL
- **Mitigation:** Only allow update if gateway_url is None (first-time only)

### Q4: Gravatar Verification Approach
How should Budding Authors prove Gravatar ownership?
- **Option A:** Email verification (send code, user enters)
  - Requires email provider in gateway
  - Standard UX pattern
- **Option B:** Profile challenge-response (no email)
  - User adds challenge code to Gravatar profile temporarily
  - Gateway verifies via Gravatar API
  - No email provider needed
  - Proves ownership (only owner can edit profile)
- **Decision:** Defer - probably avoid email, but need to test Gravatar API

### Q5: Email Provider
Do we need email capabilities?
- **Use cases:** Donation receipts, transactional notifications
- **Alternatives:**
  - PDF download for receipts (no email)
  - In-app notifications (no email)
  - MailerLite integration for important digests
- **Decision:** Defer - focus on Stripe first, add email only if needed

### Q6: Secrets Management ✅ DECIDED
**Decision:** Cloudflare Workers secrets (encrypted at rest)

**Rationale:**
- Simple deployment with wrangler CLI
- Secrets encrypted at rest, accessed via env bindings
- No additional infrastructure needed
- Generous free tier (10,000 requests/day free, then $0.50/million)
- Global edge network for low latency
- Built-in KV storage if needed for state later

**Implementation:**
```bash
# Set secrets via wrangler CLI
wrangler secret put GATEWAY_IDENTITY_KEY --env preprod
wrangler secret put STRIPE_SECRET_KEY --env preprod
wrangler secret put RESEND_API_KEY --env preprod
```

**Access in Worker:**
```typescript
export default {
  async fetch(request: Request, env: Env) {
    // Secrets available via env bindings
    const stripeKey = env.STRIPE_SECRET_KEY;
    const gatewayKey = env.GATEWAY_IDENTITY_KEY;
    // ...
  }
}
```

---

## Success Criteria

### Phase 1-2: Libraries
- [ ] ic-canister-core crate builds and passes tests
- [ ] ic-assets-env crate builds and passes tests
- [ ] Example canisters deploy successfully
- [ ] Documentation explains usage clearly

### Phase 3: Gateway
- [ ] Gateway deploys to Cloudflare Workers
- [ ] Gateway can call canister methods
- [ ] Canister validates gateway principal
- [ ] Webhooks from Stripe/Resend work

### Phase 4: Bootstrap
- [ ] Bootstrap script deploys preprod environment
- [ ] All canisters operational
- [ ] Gateway knows canister IDs
- [ ] Canisters know gateway URL

### Phase 5: Migration
- [ ] All CPP canisters use new pattern
- [ ] Single build deploys to multiple environments
- [ ] No secrets in git
- [ ] VITE_ICP_PRACTICES pattern fully replaced

### Phase 6: Production
- [ ] Production environment bootstrapped
- [ ] All services operational
- [ ] Monitoring and alerting in place
- [ ] Team trained on new deployment process

---

**Next Steps:**
1. Review this plan with team
2. Make decisions on open questions
3. Begin Phase 1 implementation
4. Update architecture docs to reference implementation

**Last Updated:** 2025-11-17
**Version:** 1.0.0
**Status:** Planning - awaiting review
