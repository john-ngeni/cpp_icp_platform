# Web2 Secrets Management for ICP Canisters

**Purpose:** Define patterns for managing Web2 API secrets (Stripe, Resend, etc.) when building on Internet Computer
**Audience:** Technical team, Security architects
**Cross-Reference:** [runtime-config.md](./runtime-config.md), [canister-architecture-diagram.md](./canister-architecture-diagram.md)

---

## Problem Statement

**Cool Planet Platform is a Web3 platform** that needs to create interfaces to regulated Web2 services where legally required or practically necessary.

**CPP Core (Web3 - Always Operational):**
- Internet Identity authentication
- On-chain governance (board voting on IC)
- NFT awards (Polygon blockchain)
- ENS domains (Ethereum naming service)
- Crypto donations (ckBTC, ETH)
- Content delivery (decentralized canisters on IC)
- User journey state (on-chain)

**Interfaces to Regulation (Web2 - Optional Enhancement):**
- **Fiat payments (Stripe)** - Credit card processing, KYC/AML compliance for donations
- **Email delivery (Resend)** - Transactional emails, receipts (email is fundamentally Web2)
- **Traditional banking** - CPF bank account for fiat settlement from Stripe

**Question:** How do we handle Web2 API secrets when interfacing with regulated services, given that IC canisters are replicated state machines (not secret vaults)?

**Key Architectural Principle:** If the Web2 gateway fails, CPP core continues operating. Gateway provides convenience (fiat payments, email notifications) but is not critical infrastructure.

---

## Why IC ≠ Kubernetes for Secrets

### Kubernetes Secret Model

In Kubernetes:
- **Secret** is encrypted at rest
- Only mounted into specific pods as env vars or files
- Never visible to arbitrary other tenants
- Not trivially retrievable by users of your app
- **Mental model:** "Secrets are injected into trusted containers"

### IC Canister Reality

On the Internet Computer:
- **Canister code and state are replicated across many nodes**
- Platform gives you **integrity & consistency**, not strong "no operator can ever see this" secrecy
- There is **no generic user-facing enclave** mechanism like "just inject this secret at runtime and it stays invisible forever"
- Anything stored in canister state (including config) must be treated as **potentially inspectable by node operators or anyone with low-level access**

**Mental Model Shift:**

> **"Canisters are public state machines with cryptographic guarantees, not private VMs."**

Secrets must be handled assuming infrastructure operators *could* access raw memory/state.

---

## Types of Secrets

Different secrets require different handling:

### 1. Per-User Secrets (passwords, keys, tokens)

**Pattern:** Client-heavy / end-to-end encryption
- Keep the *real* secret client-side when possible (wallet keys, OAuth tokens)
- Frontend talks directly to third party using user's own credential
- If something must pass through canister, treat it as **ephemeral** (short-lived tokens, not long-lived master keys)

**Think:** End-to-end encryption / client-held keys, not "backend holds everyone's secrets"

---

### 2. App-Level Secrets (API keys for Stripe, Resend, etc.)

**This is the critical category for CPP Platform.**

In k8s you'd solve with:
- `Secret` objects
- Mounted files like `/var/run/secrets/...`
- Env vars like `DB_PASSWORD`, `API_KEY`

On IC, **three architectural options:**

---

### 3. Crypto Keys (signing keys, decryption keys)

**Pattern:** Use IC's built-in cryptographic features
- **Threshold ECDSA / chain-key signing**
  - Ask IC management canister to sign messages with keys never exposed to your canister
  - Canister obtains signatures but never sees private key
  - Good for: Bitcoin/EVM transactions, verifying ownership, signing session cookies

- **Verifiable encrypted key derivation (vETKD)**
  - IC derives secrets for user/device without exposing master secrets to any one node

**Key Difference from K8s:**
- In k8s: Pod gets the raw secret
- On IC: Canister often *never gets the raw secret*, only the *effect* (signatures, derived key material)

---

## Three Architectural Options for Web2 Secrets

### Option A: IC-Native Bridge Canister

**Architecture:**
- Store **restricted API keys** inside an IC canister (as config)
- Use **HTTPS outcalls** from canister to call Stripe/Resend directly
- All on IC infrastructure

**Pros:**
- ✅ No separate Web2 server
- ✅ Infra is **replicated and governed on-chain**
- ✅ Access policy enforced by canister logic + governance
- ✅ Upgrades and config changes gated behind DAO/NNS governance
- ✅ **Most decentralized infrastructure**

**Cons:**
- ❌ Node operators *could* potentially introspect canister memory and extract secrets
- ❌ No k8s-style secret vault or TEEs guaranteed by protocol
- ❌ Secrecy **weaker** than well-managed HSM/KMS in classic backend

**Mitigations:**
1. Use **restricted Stripe keys** (limited scopes, low per-key limits)
2. Use separate keys per canister / per tenant
3. Add on-chain governance so keys can be rotated quickly
4. Log & monitor overall usage via Stripe/Resend dashboards
5. On anomaly → rotate key and update canister config

**When to Use:**
- Want **decentralized infra & governance**
- Can live with "operators could see secrets"
- Prioritize on-chain governance over maximum secrecy

---

### Option B: Federated / DAO-Operated Bridge Network

**Architecture:**
- **DAO** owns Stripe/Resend keys (on-chain governance)
- Multiple independent operators run identical "bridge nodes" off-chain
- Keys live in HSMs/MPC setups controlled collectively by operators
- IC canister talks to bridge network using authenticated protocol
- DAO can rotate keys, slash operators, or change operator set

**Pros:**
- ✅ **Most decentralized trust** - no single operator controls secrets
- ✅ Governance entirely on-chain
- ✅ Hardware security modules protect keys
- ✅ Operators can be slashed for misbehavior

**Cons:**
- ❌ **Extreme complexity** - entire off-chain network + governance layer
- ❌ Large operational surface
- ❌ Only makes sense if building **infrastructure for many apps**, not single project

**When to Use:**
- Building shared infrastructure for Web2 bridges
- Need decentralized trust in bridge itself
- Have resources to operate multi-node network with MPC/HSM setup

---

### Option C: Centralized Off-Chain Bridge (RECOMMENDED FOR CPP)

**Architecture:**
- Single backend service (k8s/VM/serverless) holds Stripe/Resend keys
- IC canister calls bridge; frontend can call IC or bridge directly
- Bridge uses mature secret management (KMS, HSM, Vault)

**Pros:**
- ✅ **Operational model well understood**
- ✅ Mature secret management tools available
- ✅ **Best secrecy guarantees** for API keys
- ✅ Simplest to implement and maintain
- ✅ Pragmatic balance: user data + logic decentralized on IC, Web2 integrations centralized

**Cons:**
- ❌ **Least decentralized** - single point of trust for Web2 integrations
- ❌ Bridge operator must be trusted

**When to Use:**
- Want best possible secrecy guarantees for API keys
- Main goal is "user data + logic decentralized on IC, but Web2 integrations pragmatic"
- Team familiar with traditional backend ops

**This is the recommended approach for CPP Platform** - see below for detailed architecture.

---

## Recommended CPP Architecture: Centralized Bridge with Gateway Principal

### High-Level Flow

```
Frontend (Vite + Custom Asset Canister)
    ↓
Off-Chain Gateway (Holds Stripe/Resend Secrets)
    ↓
    ├─→ Stripe API (payments, KYC)
    ├─→ Resend API (emails)
    └─→ IC Canisters (record events with trusted principal)
```

### Why This Topology

**Frontend → Gateway (not Frontend → IC → Gateway):**
- **Idempotency** handled entirely in gateway + Stripe (using Stripe's idempotency keys)
- **Webhooks** go directly to gateway (verify signatures, dedupe) → then notify IC
- IC canister only receives clean, deduplicated "facts" like "Payment X succeeded"
- Simpler error handling

---

## Gateway with IC Principal: Trusted Calls

### Concept

**Goal:** Gateway needs to call IC canisters to record payment/email events, and canister should trust those calls.

**Solution:** Gateway has its own **IC principal** (like a service account)

### How It Works

1. **Generate Gateway Identity:**
```bash
dfx identity new gateway
dfx identity get-principal --identity gateway
# Output: w7x7r-cok77-xa (example principal)
```

2. **Store Principal in Canister Config:**
```candid
type Config = record {
  gateway_principal : principal;
  stripe_publishable_key : text;
  // other non-secret config...
};
```

3. **Canister Validates Caller:**
```rust
use candid::Principal;

#[derive(Clone)]
struct Config {
    gateway_principal: Principal,
}

#[ic_cdk::update]
fn record_payment(intent_id: String, amount: u64, currency: String) {
    let cfg = get_config();
    let caller = ic_cdk::caller();
    assert_eq!(
        caller, cfg.gateway_principal,
        "unauthorized: only gateway can record payments"
    );
    // ... mutate state to record payment ...
}
```

4. **Gateway Calls Canister:**
```typescript
import { HttpAgent, Actor } from "@dfinity/agent";
import { Ed25519KeyIdentity } from "@dfinity/identity-ed25519";

const PRIVATE_KEY = process.env.GATEWAY_IDENTITY_KEY!; // from secrets

const identity = Ed25519KeyIdentity.fromSecretKey(
  Buffer.from(PRIVATE_KEY, "base64")
);

const agent = new HttpAgent({ identity, host: "https://ic0.app" });
const backend = Actor.createActor(backendIdl, {
  agent,
  canisterId: process.env.BACKEND_CANISTER_ID!,
});

async function handleStripeWebhook(event: Stripe.Event) {
  // ... verify webhook signature, dedupe, etc ...

  await backend.record_payment(
    event.data.object.id,
    event.data.object.amount,
    event.data.object.currency
  );
}
```

**Security:**
- Users can't spoof gateway calls (they can't sign as that principal)
- Only the gateway (holding the private key) can authenticate as that principal
- Canister trusts `msg.caller == GATEWAY_PRINCIPAL`

---

## CPP Platform: Concrete Examples

### Stripe Integration

**Stripe Provides:**
- **Publishable key** (`pk_live_...`) - Safe to be public, goes in frontend
- **Secret key** (`sk_live_...`) - Full power, NEVER in frontend or canister
- **Restricted keys** (`rk_...`) - Limited scope/permissions
- **Webhook signing secret** - Verify Stripe webhooks

**Architecture:**

1. **Frontend (cpf_members asset canister)**
   - Runtime config includes: `STRIPE_PUBLISHABLE_KEY` (via `/env.js`)
   - Uses Stripe.js to collect card details, confirm Payment Intents
   - Calls gateway for server-side operations

2. **Off-Chain Gateway**
   - Holds `STRIPE_SECRET_KEY` as env var / k8s Secret / KMS
   - Exposes minimal API:
     - `POST /payments/create-intent`
     - `POST /payments/refund`
     - `POST /billing/create-portal-session`
     - `POST /webhooks/stripe` (receives Stripe webhooks)
   - Talks to Stripe with secret key
   - Calls IC canister (as gateway principal) to record events

3. **IC Canister (cpf_members backend)**
   - Does NOT hold `sk_live_...`
   - Runtime config includes:
     - `payments_bridge_url` (URL of gateway)
     - `gateway_principal` (for validating calls from gateway)
     - `stripe_publishable_key` (safe to store)
   - Methods:
     - `record_payment(intent_id, amount, currency)` - Only callable by gateway principal
     - `record_refund(...)` - Only callable by gateway principal
     - User-facing methods use Internet Identity / delegation checks

4. **Webhook Flow:**
```
Stripe → Gateway → (verify signature, dedupe) → IC Canister (as gateway principal)
```

---

### Resend Integration

**Resend Provides:**
- **Single API key** (`re_...`) - Equivalent to backend secret

**Architecture:**

1. **Frontend**
   - Does NOT know Resend key
   - Calls IC canister: "send verification email to user@example.com"

2. **IC Canister**
   - Receives user request
   - Calls gateway: `POST /emails/send-verification`
   - Gateway principal is stored in config

3. **Off-Chain Gateway**
   - Holds `RESEND_API_KEY` as secret
   - Exposes API:
     - `POST /emails/send-verification`
     - `POST /emails/send-receipt`
     - `POST /emails/send-newsletter-confirmation`
   - Talks to Resend with API key
   - Calls IC canister to record email events

---

## Gateway Platform: Cloudflare Workers

**CPP uses Cloudflare Workers** for the serverless gateway. This decision balances simplicity, security, and cost-effectiveness.

### Why Cloudflare Workers

**Technical Benefits:**
- ✅ **Built-in secrets management** - Encrypted at rest, accessed via env bindings
- ✅ **Global edge network** - Low latency worldwide (300+ cities)
- ✅ **Simple deployment** - `wrangler deploy` from CLI
- ✅ **TypeScript/JavaScript** - Matches frontend skillset
- ✅ **No infrastructure management** - Serverless, no containers to manage
- ✅ **Instant cold starts** - V8 isolates, not containers

**Cost-Effective:**
- Free tier: 100,000 requests/day
- Paid: $5/month for 10M requests
- No minimum spend, pay-as-you-go
- Significantly cheaper than AWS Lambda + API Gateway

**Security:**
- Secrets encrypted at rest in Cloudflare's infrastructure
- HTTPS-only (automatic TLS certificates)
- DDoS protection included
- Audit logging available

### Deployment Example

```bash
# Install wrangler CLI
npm install -g wrangler

# Login to Cloudflare
wrangler login

# Deploy to preprod environment
wrangler deploy --env preprod

# Set secrets (encrypted at rest)
echo "$STRIPE_SECRET_KEY" | wrangler secret put STRIPE_SECRET_KEY --env preprod
echo "$RESEND_API_KEY" | wrangler secret put RESEND_API_KEY --env preprod
echo "$GATEWAY_IC_IDENTITY" | wrangler secret put GATEWAY_IDENTITY_KEY --env preprod
```

### Alternative Platforms (Not Recommended for CPP)

If Cloudflare Workers doesn't fit your requirements, alternatives include:
- **AWS Lambda** - More complex, higher cost, but more flexible
- **Vercel Functions** - Similar to Workers, but less control over secrets
- **Fly.io** - Good for Rust services, but overkill for simple gateway
- **Kubernetes** - Only if already running k8s infrastructure (significant overhead)

---

## Gateway API Design

### Keep It Boring and Narrow

**Stripe Gateway Endpoints:**
```
POST /payments/create-intent
POST /payments/refund
POST /billing/create-portal-session
POST /webhooks/stripe
```

**Resend Gateway Endpoints:**
```
POST /emails/send-verification
POST /emails/send-receipt
POST /emails/send-newsletter-confirmation
```

**Key Principles:**
1. **Minimal surface area** - Only expose what IC canister needs
2. **No direct passthrough** - Don't expose raw Stripe/Resend APIs
3. **Sanitize responses** - Return only necessary data to canister
4. **Idempotency** - Use Stripe's idempotency keys for payments
5. **Webhook verification** - Verify signatures before calling IC

---

## Security Model

### Frontend → Gateway

**Auth Options:**
1. **User-level tokens** - Session cookies / JWT from auth system
2. **Origin restrictions** - Only allow calls from your frontend domain
3. **Rate limiting** - Prevent abuse

### Gateway → IC Canister

**Auth Pattern:**
1. **Gateway Principal** (as described above)
   - Gateway authenticates using Ed25519 identity
   - Canister validates `msg.caller == gateway_principal`
   - Most secure option

2. **Shared Secret** (alternative for simple cases)
   - Generate `BRIDGE_API_KEY` (NOT the Stripe key)
   - Store in gateway env vars AND canister config
   - Gateway sends `X-Bridge-Key` header or HMAC
   - If leaked, worst case: someone can call bridge API (not get Stripe keys)
   - Can rotate quickly

---

## What Lives Where

### ✅ Safe for IC Canister Runtime Config

From [runtime-config.md](./runtime-config.md):
- API URLs / canister IDs
- `payments_bridge_url`
- `email_bridge_url`
- `gateway_principal` (public by nature)
- `stripe_publishable_key` (meant to be public)
- Feature flags, thresholds, non-sensitive IDs

### ❌ Must Live in Off-Chain Gateway

- `STRIPE_SECRET_KEY`
- `STRIPE_WEBHOOK_SECRET`
- `RESEND_API_KEY`
- `GATEWAY_IDENTITY_KEY` (Ed25519 private key for IC principal)

### 🔐 Gateway Secret Storage

**Cloudflare Workers:**
```bash
wrangler secret put STRIPE_SECRET_KEY
wrangler secret put RESEND_API_KEY
wrangler secret put GATEWAY_IDENTITY_KEY
```

**Kubernetes:**
```yaml
apiVersion: v1
kind: Secret
metadata:
  name: gateway-secrets
type: Opaque
data:
  stripe-secret-key: <base64>
  resend-api-key: <base64>
  gateway-identity-key: <base64>
```

**AWS Lambda:**
- Use AWS Secrets Manager
- Reference secrets in Lambda env vars

---

## CPP Platform Phased Rollout

### Phase C: Donations (3-6 Months)

**Gateway Needed For:**
- Stripe payments (create PaymentIntent, handle webhooks)
- Email receipts via Resend

**Infrastructure:**
1. Deploy gateway to Cloudflare Workers
2. Store secrets: `STRIPE_SECRET_KEY`, `RESEND_API_KEY`, `GATEWAY_IDENTITY_KEY`
3. Update cpf_members canister config:
   ```candid
   {
     payments_bridge_url = "https://gateway.cpf.nft";
     email_bridge_url = "https://gateway.cpf.nft";
     gateway_principal = principal "w7x7r-cok77-xa";
     stripe_publishable_key = "pk_live_...";
   }
   ```
4. Frontend loads `STRIPE_PUBLISHABLE_KEY` from `/env.js`
5. Test in Stripe sandbox first, then production

### Phase D: NFT Awards (6-9 Months)

**Gateway Extended For:**
- Polygon NFT minting (if using external minting service)
- OR use IC's Chain Fusion bridge (no gateway needed)

**Decision Point:** Does Polygon minting happen:
- **Option 1:** In IC canister via Chain Fusion → No gateway needed
- **Option 2:** Via external service → Add to gateway

### Phase E: Gated Features (9-12 Months)

**Gateway Extended For:**
- Webinar platform integration (if using Zoom/etc APIs)
- Advanced email templates (transactional emails for Camino completion, etc.)

---

## Comparison: Runtime Config vs Secrets

| Item                         | Where It Lives           | How It's Deployed                        | Secrecy Level |
| ---------------------------- | ------------------------ | ---------------------------------------- | ------------- |
| Stripe Publishable Key       | IC canister config       | Init args / `/env.js`                    | Public        |
| Stripe Secret Key            | Off-chain gateway        | KMS / k8s Secret / Worker secret         | Secret        |
| Payments Bridge URL          | IC canister config       | Init args                                | Public        |
| Gateway Principal            | IC canister config       | Init args                                | Public        |
| Gateway Identity Private Key | Off-chain gateway        | KMS / k8s Secret / Worker secret         | Secret        |
| Resend API Key               | Off-chain gateway        | KMS / k8s Secret / Worker secret         | Secret        |
| KYC Thresholds               | IC canister config       | Init args                                | Public        |
| Log Level                    | IC canister config       | Init args                                | Public        |

---

## Threat Model

### What We're Protecting Against

1. **User compromise** - Users can't impersonate gateway, can't call `record_payment` directly
2. **Key theft** - Stripe/Resend keys not in canister state (not inspectable by node operators)
3. **Unauthorized operations** - Only gateway principal can record payment/email events
4. **Replay attacks** - Idempotency keys (Stripe) + webhook signature verification

### What We Accept

1. **Gateway is trusted** - We accept centralization at the Web2 boundary
2. **Gateway operator must be honest** - They hold the secrets
3. **Gateway availability** - If gateway is down, payments/emails are down (but IC app logic continues)

### Mitigation Strategies

1. **Restricted API keys** - Use Stripe restricted keys with minimal permissions
2. **Rate limiting** - Gateway enforces rate limits per user/canister
3. **Monitoring** - Alert on unusual patterns in Stripe/Resend dashboards
4. **Key rotation** - Rotate gateway secrets regularly
5. **Audit logging** - Gateway logs all IC canister calls
6. **Fallback** - Consider multiple gateway instances for availability

---

## Interface to Regulated Web2 World

### Why CPP Needs This Architecture

**Cool Planet Platform is a Web3 platform** that creates interfaces to existing regulatory frameworks where legally required.

**CPP Platform (Web3 - Always Operational):**
- User identities (Internet Identity on IC)
- Governance (board voting on IC)
- Content delivery (Camino, newsletters on decentralized canisters)
- NFT awards (Polygon blockchain)
- ENS domains (cpf.nft on Ethereum)
- Crypto donations (ckBTC, ETH directly on-chain)
- User journey state (stored in canisters)

**Regulated Interfaces (Web2 - Optional):**
- **Fiat payment processing** - Stripe for credit card donations (PCI-DSS, KYC/AML)
- **Email delivery** - Resend for transactional emails (SMTP is Web2)
- **Traditional banking** - CPF bank account for fiat settlement from Stripe
- **Tax reporting** - ANBI compliance (Netherlands), donation receipts

**The Gateway: Interface to Regulation**

```
┌─────────────────────────────────────────────────────────┐
│   Cool Planet Platform (Web3 - Decentralized)           │
│                                                          │
│  ✅ Internet Identity (IC)                               │
│  ✅ Governance (board voting on IC)                      │
│  ✅ Content (decentralized canisters)                    │
│  ✅ NFT awards (Polygon)                                 │
│  ✅ ENS domains (cpf.nft on Ethereum)                    │
│  ✅ Crypto donations (ckBTC, ETH)                        │
│  ✅ User journey (on-chain state)                        │
│                                                          │
│  🔄 ALWAYS OPERATIONAL (even if gateway fails)           │
│                                                          │
└───────────────────┬─────────────────────────────────────┘
                    │
                    │ Optional Gateway (Cloudflare Workers)
                    │ Gateway with IC Principal
                    │ (Interface to regulation)
                    │
┌───────────────────▼─────────────────────────────────────┐
│   Regulated Web2 Services (Centralized)                 │
│                                                          │
│  💳 Stripe (fiat payments, KYC/AML)                      │
│  📧 Resend (email delivery, SMTP)                        │
│  🏦 CPF Bank Account (fiat settlement)                   │
│  📊 Tax Authorities (ANBI reporting)                     │
│                                                          │
│  ⚠️  IF UNAVAILABLE: Fiat payments stop, emails stop    │
│  ✅ CPP CORE CONTINUES: Users can still authenticate,    │
│      access content, donate crypto, earn NFTs            │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

**Key Architectural Principles:**

1. **CPP is Web3** - The platform core is fully decentralized
2. **Gateway is complementary** - Provides fiat payment and email convenience
3. **Resilient by design** - Gateway failure doesn't break platform
4. **Graceful degradation** - If Stripe unavailable → accept crypto donations instead
5. **Interface to regulation** - Gateway allows legal compliance while keeping platform decentralized

**The gateway is NOT a compromise** - it's an **architected interface** to existing regulatory frameworks (fiat payment processing, email delivery, tax reporting) that cannot be decentralized due to legal requirements.

**What This Enables:**
1. **Legal compliance** - KYC/AML handled by Stripe (regulated entity)
2. **Tax optimization** - ANBI status requires proper donation receipts
3. **User experience** - Familiar payment methods (credit cards), reliable email delivery
4. **Fraud prevention** - Stripe Radar for payment protection
5. **Auditability** - Complete audit trail for regulators

---

## Recommended Implementation for CPP

### Phase C: Initial Gateway (Donations + Email)

**Tech Stack:**
- **Hosting:** Cloudflare Workers
- **Language:** TypeScript
- **Secrets:** Cloudflare Workers secrets (encrypted at rest)
- **Monitoring:** Cloudflare Analytics + Sentry

**Endpoints:**
```
POST /payments/create-intent
POST /payments/refund
POST /webhooks/stripe
POST /emails/send-verification
POST /emails/send-receipt
```

**Config in IC Canister:**
```rust
struct Config {
    payments_bridge_url: String,  // "https://gateway.cpf.nft"
    email_bridge_url: String,      // "https://gateway.cpf.nft"
    gateway_principal: Principal,  // Principal derived from Ed25519 key
    stripe_publishable_key: String, // "pk_live_..."
}
```

### Future: Consider Federated Gateway (Phase E+)

**If CPP becomes infrastructure for other organizations:**
- Multiple gateway operators (DAO-governed)
- MPC/HSM for secret management
- Slash operators for misbehavior
- On-chain governance for key rotation

**Not needed for initial launch** - centralized gateway is appropriate for single-organization platform.

---

## Summary

### Key Takeaways

1. **IC canisters ≠ secret vaults** - Treat canister state as potentially inspectable
2. **Runtime config ≠ secrets** - Config is for non-sensitive data only (see [runtime-config.md](./runtime-config.md))
3. **Web2 secrets need Web2 infrastructure** - Stripe/Resend keys belong in off-chain gateway
4. **Gateway with IC principal** - Enables trusted calls from gateway to canisters
5. **Pragmatic decentralization** - Decentralize what matters (user data, governance), centralize interfaces to regulated world
6. **Compliance requires centralization** - Financial regulation, email deliverability, tax reporting all require Web2 interfaces

### For CPP Platform

- **Option C (Centralized Gateway) is recommended**
- Use Cloudflare Workers for simplicity
- Gateway holds Stripe/Resend secrets securely
- IC canisters hold only runtime config (public data)
- Gateway has IC principal for trusted event recording
- This architecture enables legal compliance while keeping user data and governance decentralized

---

**Last Updated:** 2025-11-17
**Version:** 1.0.0
**Status:** Architecture pattern for Web2 secrets management
**Related:** [runtime-config.md](./runtime-config.md), [canister-architecture-diagram.md](./canister-architecture-diagram.md)
