# Domain Setup Guide for CPP Platform

**Date:** 2025-11-14 (Updated)
**Purpose:** Domain acquisition, ownership evolution, and governance control for cpf.nft
**Scope:** Unstoppable Domains on Polygon/Ethereum, not generic IC DNS configuration
**Context:** cpf.nft domain ownership evolution and admin/user identity separation

**⚠️ IMPORTANT CORRECTION:**
- **cpf.nft is Unstoppable Domains** (already reserved), NOT ENS
- Unstoppable Domains uses Polygon blockchain (cheaper gas than Ethereum)
- This document was originally written for ENS and needs updating for Unstoppable Domains specifics
- Core concepts remain the same: domain ownership, chain-key control, governance

**Cross-Reference:**
- **[ic_dns.md](./ic_dns.md)** - Generic IC DNS setup (applies to all domains)
- [canister-architecture-diagram.md](./canister-architecture-diagram.md) - Identity architecture
- [governance-policy.md](./governance-policy.md) - Board control of domains
- [derivation-origins-integration.md](./derivation-origins-integration.md) - Alternative origins
- [architectural_decisions.md § OQ-007](./architectural_decisions.md) - Gas management strategy

## Executive Summary

CPP platform uses **cpf.nft (Unstoppable Domains)** as the canonical derivation origin for Internet Identity across all services.

**This document covers domain-specific aspects:**

1. **Domain Acquisition:** cpf.nft already reserved with Unstoppable Domains
2. **Subdomain Creation:** Create authors.cpf.nft for control plane (governance, infrastructure)
3. **Ownership Evolution:** Personal wallet → ICP governance control → SNS
4. **Chain-Key Control:** How ICP governance controls domain NFT via threshold ECDSA (Polygon)
5. **Bootstrap Sequence:** Order of operations for clean identity hierarchy
6. **Control Plane Focus:** This repo manages control plane infrastructure (authors.cpf.nft)

**Gas Cost Reality:**
- **Admin operations (this document):** < $10/year - Minimal, one-time subdomain setup
- **User NFT operations:** $1000s-$10000s/year - Minting/transferring NFTs to users (CPF sponsors if user doesn't use own wallet)
- See [architectural_decisions.md § OQ-007](./architectural_decisions.md) for NFT gas management strategy

**For generic IC DNS configuration (applies to both ENS and traditional DNS):**
- See **[ic_dns.md](./ic_dns.md)** for:
  - 3 required DNS records (CNAME to icp1.io, TXT _canister-id, CNAME _acme-challenge)
  - .well-known/ic-domains file requirements
  - Domain registration via IC API
  - SSL certificate provisioning
  - Troubleshooting DNS issues

**Important:** ENS and traditional DNS domains **coexist** in CPP platform:
- **ENS authors.cpf.nft (Control Plane):** Derivation origin for governance, infrastructure management
- **ENS cpf.nft (Data Plane):** Derivation origin for user-facing operations
- **Traditional DNS (coolplanet-foundation.org, newsletters.*):** User-facing domains, alternative origins
- Both use the same IC DNS configuration (see [ic_dns.md](./ic_dns.md))
- **Authors have TWO principals:** One for control plane (authors.cpf.nft), one for data plane (cpf.nft)

---

## ENS NFT Ownership Architecture

### The cpf.nft ENS Domain is an NFT

**Key Facts:**
- cpf.nft is an **ERC-721 NFT** on Ethereum mainnet
- Lives in an Ethereum wallet (MetaMask, Ledger, hardware wallet, or contract)
- Can be transferred like any NFT
- Ownership determines who can update DNS records and create subdomains

### Ownership Evolution Strategy

```
Phase 0: Bootstrap (Personal Wallet)
    ↓
Phase 1: Governance (ICP Chain-Key Derived Address)
    ↓
Phase 2: Full Decentralization (SNS/DAO control)
```

---

## Phase 0: ENS Acquisition & Bootstrap

### Step 1: Acquire cpf.nft ENS NFT

**Option A: Buy existing NFT (if available)**
```bash
# Check if cpf.nft is available for purchase
# Visit: https://app.ens.domains/cpf.nft

# If available, purchase using Ethereum wallet
# Typical cost: Varies based on ENS marketplace
```

**Option B: Register new .nft domain**
```bash
# .nft is a special TLD on ENS
# Visit: https://ens.domains
# Connect Ethereum wallet
# Search for "cpf.nft"
# Register if available
```

**Initial Owner:** Personal Ethereum wallet (founder/technical lead)
- **Why:** Fast iteration during bootstrap
- **Security:** Use hardware wallet (Ledger, Trezor)
- **Backup:** Securely store seed phrase

---

### Step 2: Create authors.cpf.nft Subdomain

**Immediately after acquiring cpf.nft:**

```bash
# Visit ENS Manager: https://app.ens.domains/cpf.nft
# Connect wallet (owner of cpf.nft)

# Navigate to "Subdomains" tab
# Create new subdomain: "admin"
#   Full name: authors.cpf.nft
#   Owner: [Same wallet initially, transfer later]

# Save changes (Ethereum transaction required)
```

**Purpose of authors.cpf.nft (Control Plane):**
- **Control plane derivation origin** for board members and operations staff
- Governance, infrastructure management, strategic decisions
- Separate Internet Identity principals from data plane (cpf.nft) identities
- Phishing protection (authors only access authors.cpf.nft for control plane operations)
- Clean bootstrap of governance hierarchy
- **This repo (cpp_icp_platform) focuses on control plane infrastructure**

---

### Step 3: Configure Initial DNS Records

**For cpf.nft (user-facing):**

```bash
# In ENS Manager for cpf.nft:

# Add Content Hash (points to IC canister)
# Content: IC canister ID of cpf_nft_canister
# Format: ipfs://[canister-id].raw.ic0.app
# Or better: Direct canister ID if ENS supports IC

# Add A record (if needed for traditional DNS)
# Type: A
# Name: @
# Value: [IC boundary node IP, or use CNAME]

# Add TXT record for IC binding
# Type: TXT
# Name: _canister-id
# Value: [cpf_nft_canister_id]
```

**For authors.cpf.nft (governance):**

```bash
# In ENS Manager for authors.cpf.nft:

# Add Content Hash
# Content: IC canister ID of governance_canister
# Format: ipfs://[governance-canister-id].raw.ic0.app

# Add TXT record
# Type: TXT
# Name: _canister-id
# Value: [governance_canister_id]
```

---

## Phase 1: Transfer ENS to ICP Governance

### Why Transfer to ICP Control?

**Advantages:**
- ✅ Board members control ENS via IC governance (same mechanism as canisters)
- ✅ No need for external Ethereum multi-sig (Gnosis Safe)
- ✅ Fully on-chain governance
- ✅ Board uses authors.cpf.nft for ALL control plane actions (ENS + IC + governance)

**Challenge:**
- Requires chain-key ECDSA integration
- Governance canister needs to derive Ethereum address
- ENS management code on ICP side

### Architecture: ICP Chain-Key Control of ENS

```
CONTROL PLANE (authors.cpf.nft)
    ↓
Board Members (authors.cpf.nft identities)
    ↓ submit proposal
Governance Canister (Rust + Motoko voting)
    ↓ votes reach threshold (time-based: 3-of-3 immediate, 2-of-3 after 3 days, 1-of-3 after 7 days)
Execute Proposal
    ↓ derive Ethereum signature
Chain-Key ECDSA (threshold signature)
    ↓ sign Ethereum transaction
ENS Contract on Ethereum
    ↓ update DNS records, transfer ownership, etc.
```

---

### Implementation: Derive Ethereum Address from Governance Canister

**Step 1: Governance Canister Derives Ethereum Address**

```rust
// In governance canister (Rust)
use ic_cdk::api::management_canister::ecdsa::{
    ecdsa_public_key, EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgument,
};

async fn derive_ethereum_address() -> String {
    // Derive public key using IC's threshold ECDSA
    let key_id = EcdsaKeyId {
        curve: EcdsaCurve::Secp256k1,
        name: "key_1".to_string(), // Production key
    };

    let arg = EcdsaPublicKeyArgument {
        canister_id: None, // Use caller's canister ID
        derivation_path: vec![b"ens-control".to_vec()],
        key_id: key_id.clone(),
    };

    let (response,) = ecdsa_public_key(arg).await.unwrap();
    let public_key_bytes = response.public_key;

    // Derive Ethereum address from public key
    // Ethereum address = last 20 bytes of Keccak256(public_key)
    let ethereum_address = derive_eth_address_from_pubkey(&public_key_bytes);

    format!("0x{}", hex::encode(ethereum_address))
}

fn derive_eth_address_from_pubkey(pubkey: &[u8]) -> [u8; 20] {
    use tiny_keccak::{Hasher, Keccak};

    // Remove 0x04 prefix if present (uncompressed key format)
    let pubkey_bytes = if pubkey[0] == 0x04 {
        &pubkey[1..]
    } else {
        pubkey
    };

    // Keccak256 hash
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(pubkey_bytes);
    hasher.finalize(&mut output);

    // Take last 20 bytes
    let mut address = [0u8; 20];
    address.copy_from_slice(&output[12..]);
    address
}
```

**Step 2: Fund Ethereum Address with ETH for Gas**

**⚠️ CRITICAL:** The ICP-derived Ethereum address needs ETH to pay gas fees for ENS transactions.

```bash
# Get ICP-derived Ethereum address
dfx canister call governance_canister getEthereumAddress --network ic
# Returns: "0xABCD...1234"

# Fund this address with ETH for gas fees
# Recommended initial funding: 0.5 ETH (~$1000 USD, sufficient for 500-1000 ENS updates)

# Using MetaMask or any Ethereum wallet:
# 1. Send 0.5 ETH to 0xABCD...1234
# 2. Verify transaction on Etherscan
# 3. Confirm balance

# Check balance programmatically (via governance canister)
dfx canister call governance_canister getEthereumBalance --network ic
# Returns: ETH balance in Wei
```

**Gas Cost Estimates:**

| Operation | Estimated Gas | Cost @ 30 gwei | Cost @ 100 gwei |
|-----------|---------------|----------------|-----------------|
| Update ENS DNS record | ~50,000 | $0.05 | $0.15 |
| Create subdomain | ~100,000 | $0.10 | $0.30 |
| Transfer ENS NFT | ~50,000 | $0.05 | $0.15 |
| Update text record | ~40,000 | $0.04 | $0.12 |

**Funding Strategy:**
- **Initial funding:** 0.5 ETH (sufficient for ~500-1000 operations)
- **Monitoring:** Alert when balance < 0.1 ETH
- **Refill threshold:** Refill when balance < 0.05 ETH
- **Refill amount:** 0.5 ETH per refill

**Who Can Fund the Address?**
- Anyone can send ETH to the ICP-derived address (it's just an Ethereum address)
- Board members typically responsible for ensuring adequate funding
- Should be part of regular governance operational procedures

**Monitoring ETH Balance:**

```rust
// In governance canister (Rust)
async fn check_ethereum_balance() -> Result<u128, String> {
    let eth_address = derive_ethereum_address().await;

    // Query Ethereum RPC via HTTPS outcalls
    let balance_wei = query_eth_balance_via_rpc(eth_address).await?;

    // Alert if balance is low
    if balance_wei < 50_000_000_000_000_000 { // 0.05 ETH
        log_alert("Low ETH balance for ENS operations");
    }

    Ok(balance_wei)
}

// Expose as governance query
#[query]
async fn getEthereumBalance() -> u128 {
    check_ethereum_balance().await.unwrap_or(0)
}
```

**Refilling Process:**

```bash
# Board members monitor ETH balance
dfx canister call governance_canister getEthereumBalance --network ic

# When low, any board member can send ETH
# Using MetaMask:
# To: [ICP-derived address]
# Amount: 0.5 ETH

# Or create governance proposal to track refills
dfx canister call governance_canister submitProposal '(
  record {
    proposal_type = variant {
      RecordETHDeposit = record {
        amount_wei = 500000000000000000;  // 0.5 ETH
        tx_hash = "0x...";
        depositor = principal "xxxxx-xxxxx";
      }
    };
    description = "Refilled ENS gas wallet with 0.5 ETH";
  }
)'
```

**Step 3: Transfer cpf.nft ENS NFT to Derived Address**

```bash
# Get derived Ethereum address from governance canister
dfx canister call governance_canister getEthereumAddress --network ic

# Returns: "0xABCD...1234" (governance canister's derived Ethereum address)

# In MetaMask (or personal wallet owning cpf.nft):
# 1. Visit: https://app.ens.domains/cpf.nft
# 2. Go to "Transfer" tab
# 3. Enter derived address: 0xABCD...1234
# 4. Confirm transaction (gas fee required)

# Verify transfer
# Visit: https://etherscan.io/token/0x57f1887a8BF19b14fC0dF6Fd9B2acc9Af147eA85?a=[derived-address]
# Should show cpf.nft owned by governance canister's address
```

---

### ENS Management via IC Governance

**Proposal Type: Update ENS Records**

```rust
// In governance canister (Rust)
#[derive(CandidType, Deserialize)]
pub enum ProposalType {
    // ... existing types
    UpdateENSRecord {
        domain: String,           // "cpf.nft" or "authors.cpf.nft"
        record_type: ENSRecordType,
        value: String,
    },
}

#[derive(CandidType, Deserialize)]
pub enum ENSRecordType {
    ContentHash,  // Point to IC canister
    ARecord,      // Traditional DNS
    TxtRecord,    // _canister-id binding
    Subdomain,    // Create new subdomain
}

async fn execute_ens_update(proposal: UpdateENSRecord) -> Result<(), String> {
    // 1. Encode ENS contract call
    let ens_contract_address = "0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e"; // ENS Registry
    let call_data = encode_ens_update_call(
        &proposal.domain,
        &proposal.record_type,
        &proposal.value
    );

    // 2. Sign transaction using chain-key ECDSA
    let signature = sign_ethereum_transaction(call_data).await?;

    // 3. Submit to Ethereum via HTTPS outcalls
    submit_to_ethereum(signature).await?;

    Ok(())
}
```

**Board Member Workflow:**

```bash
# Board member authenticates with authors.cpf.nft
# Visits governance dashboard

# Submit proposal: Update cpf.nft DNS to point to new canister
dfx canister call governance_canister submitProposal '(
  record {
    proposal_type = variant {
      UpdateENSRecord = record {
        domain = "cpf.nft";
        record_type = variant { ContentHash };
        value = "icns://rrkah-fqaaa-aaaaa-aaaaq-cai"; // New canister ID
      }
    };
    description = "Point cpf.nft to updated canister";
  }
)' --network ic

# Other board members approve (3-of-3 required)
# Governance canister signs Ethereum transaction
# ENS record updated automatically
```

---

## Alternative: Gnosis Safe Control (Simpler Initially)

If you want to **avoid building ENS management code on ICP** initially:

### Step 1: Create Gnosis Safe on Ethereum

```bash
# Visit: https://app.safe.global
# Connect Ethereum wallet
# Create new Safe

# Configuration:
# - Network: Ethereum Mainnet
# - Owners: 3 board member Ethereum addresses
# - Threshold: 3-of-3 required

# Save Safe address: 0xSAFE...ADDRESS
```

### Step 2: Transfer cpf.nft ENS NFT to Gnosis Safe

```bash
# In personal wallet:
# 1. Visit: https://app.ens.domains/cpf.nft
# 2. Transfer to Gnosis Safe address: 0xSAFE...ADDRESS
# 3. Confirm transaction
```

### Step 3: Update ENS via Gnosis Safe

```bash
# To update ENS records:
# 1. Visit Gnosis Safe app
# 2. Go to "Transaction Builder" or "Apps"
# 3. Add ENS Manager app
# 4. Select cpf.nft
# 5. Update records
# 6. Submit transaction
# 7. Wait for 3-of-3 board member approvals
# 8. Execute transaction (gas fee required)
```

**Trade-offs:**
- ✅ Simpler: No IC code needed for ENS management
- ✅ Familiar: Board members use Ethereum wallets (MetaMask, Ledger)
- ❌ External dependency: Relies on Ethereum network + Gnosis Safe
- ❌ Higher friction: Every ENS update requires Ethereum gas fees
- ❌ Separate governance: ENS controlled via Ethereum, IC via ICP

---

## DNS Configuration for IC Canisters

**For detailed IC DNS configuration, see [ic_dns.md](./ic_dns.md).**

This section covers ENS-specific DNS configuration via ENS Manager.

### Configure cpf.nft → cpf_nft_canister

**DNS Records (via ENS Manager):**

```bash
# Visit: https://app.ens.domains/cpf.nft
# Go to "Records" tab

# Add CNAME record (route traffic to IC)
# Type: CNAME
# Name: @
# Value: cpf.nft.icp1.io

# Add TXT record (bind to canister)
# Type: TXT
# Name: _canister-id
# Value: [cpf_nft_canister_id from dfx canister id cpf_nft --network ic]

# Add CNAME for ACME challenge (TLS certificates)
# Type: CNAME
# Name: _acme-challenge
# Value: _acme-challenge.cpf.nft.icp2.io

# Save changes (Ethereum transaction required)
```

**For complete details on:**
- `.well-known/ic-domains` file format
- Domain registration via IC API
- SSL certificate provisioning
- Verification procedures
- Troubleshooting

**See [ic_dns.md](./ic_dns.md) - Generic IC DNS Setup Guide**

---

### Configure authors.cpf.nft → governance_canister

**DNS Records (via ENS Manager):**

```bash
# Visit: https://app.ens.domains/authors.cpf.nft
# (Subdomain management under cpf.nft)

# CNAME record
# Name: @
# Value: authors.cpf.nft.icp1.io

# TXT record
# Name: _canister-id
# Value: [governance_canister_id]

# ACME CNAME
# Name: _acme-challenge
# Value: _acme-challenge.authors.cpf.nft.icp2.io

# Save changes (Ethereum transaction)
```

**Complete setup details in [ic_dns.md](./ic_dns.md)**

---

## Alternative Origins Configuration

### ENS and Traditional DNS Coexistence

**CPP platform runs ENS and traditional DNS in parallel:**

```
CONTROL PLANE (ENS):
└─ authors.cpf.nft                    # Control plane derivation origin (governance, infrastructure)

DATA PLANE (ENS):
└─ cpf.nft                            # Data plane derivation origin (user operations)

DATA PLANE (Alternative Origins - Traditional DNS):
├─ coolplanet-foundation.org        # Main organization site
├─ newsletters.coolplanet-foundation.org
├─ members.coolplanet-foundation.org
└─ [other organization domains]
```

**Key Points:**
- **authors.cpf.nft (Control Plane):** Governance, infrastructure, strategic decisions - THIS REPO
- **cpf.nft (Data Plane):** User-facing operations, content, donations, NFTs
- **Traditional DNS domains:** Data plane alternative origins (familiar .org domains)
- **All domains** use the same IC DNS configuration (see [ic_dns.md](./ic_dns.md))
- **Data plane users maintain same principal** across all alternative origins
- **Authors have TWO principals:** Control plane + data plane

### What are Alternative Origins?

From Internet Identity documentation:
- Users can authenticate with **one canonical derivation origin** (cpf.nft)
- Get the **same principal** when accessing **alternative origins** (subdomains, custom domains)
- Alternative origins must be **explicitly authorized** by canonical origin

### Serve ii-alternative-origins from cpf.nft

**File location:** `cpf_nft_canister/.well-known/ii-alternative-origins`

**Example content (traditional DNS domains as alternative origins):**
```
https://coolplanet-foundation.org
https://newsletters.coolplanet-foundation.org
https://members.coolplanet-foundation.org
```

**Note:** These traditional DNS domains must also be configured with IC DNS records (see [ic_dns.md](./ic_dns.md))

**In Rust canister:**

```rust
const ALTERNATIVE_ORIGINS: &str = r#"https://coolplanet-foundation.org
https://newsletters.coolplanet-foundation.org
https://members.coolplanet-foundation.org"#;

#[query]
fn http_request(req: HttpRequest) -> HttpResponse {
    match req.url.as_str() {
        "/.well-known/ic-domains" => HttpResponse {
            status_code: 200,
            headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
            body: b"cpf.nft".to_vec(),
        },
        "/.well-known/ii-alternative-origins" => HttpResponse {
            status_code: 200,
            headers: vec![
                ("Content-Type".to_string(), "application/json".to_string()),
                ("Access-Control-Allow-Origin".to_string(), "*".to_string()),
            ],
            body: format!(r#"{{"alternativeOrigins":{:?}}}"#,
                         ALTERNATIVE_ORIGINS.lines().collect::<Vec<_>>()).into_bytes(),
        },
        _ => // ... rest of routing
    }
}
```

**Verify:**

```bash
curl https://cpf.nft/.well-known/ii-alternative-origins

# Should return:
# {
#   "alternativeOrigins": [
#     "https://coolplanet-foundation.org",
#     "https://newsletters.coolplanet-foundation.org",
#     "https://members.coolplanet-foundation.org"
#   ]
# }
```

---

## Bootstrap Sequence: Complete Order of Operations

### Prerequisites

- [ ] Ethereum wallet set up (hardware wallet recommended)
- [ ] ENS funds (ETH for gas fees)
- [ ] dfx installed and configured
- [ ] IC cycles wallet funded

---

### Phase 0: ENS Acquisition (Day 0)

```bash
# Step 1: Acquire cpf.nft ENS NFT
# - Purchase or register via https://app.ens.domains
# - Owner: Personal Ethereum wallet (founder/technical lead)
# - Cost: Varies (check ENS marketplace)

# Step 2: Create authors.cpf.nft subdomain
# - In ENS Manager for cpf.nft
# - Navigate to Subdomains → Create "admin"
# - Owner: Same personal wallet initially

# Step 3: Initial DNS configuration (optional)
# - Can skip if deploying canisters first
# - Or point to placeholder
```

**State after Phase 0:**
- ✅ cpf.nft owned by personal wallet
- ✅ authors.cpf.nft subdomain created
- ❌ No IC canisters deployed yet
- ❌ No derivation origins configured yet

---

### Phase 1: Deploy IC Infrastructure (Week 0-1)

```bash
# Step 4: Deploy governance canisters
cd /Users/john/git/cpp_icp_platform

# Deploy voting canister first
dfx deploy voting_canister --network ic

# Deploy governance canister with voting canister ID
VOTING_ID=$(dfx canister id voting_canister --network ic)
dfx deploy governance_canister --network ic \
  --argument "(record { voting_canister = principal \"$VOTING_ID\" })"

# Get governance canister ID
GOVERNANCE_ID=$(dfx canister id governance_canister --network ic)
echo "Governance Canister: $GOVERNANCE_ID"

# Step 5: Deploy cpf_nft canister (serves cpf.nft)
dfx deploy cpf_nft --network ic
CPF_NFT_ID=$(dfx canister id cpf_nft --network ic)
echo "CPF NFT Canister: $CPF_NFT_ID"

# Step 6: Deploy other infrastructure canisters
# (See canister-architecture-diagram.md for full sequence)
```

**State after Phase 1:**
- ✅ All IC canisters deployed
- ✅ Canister IDs known
- ❌ DNS not configured yet (canisters not accessible via cpf.nft)
- ❌ Governance not bootstrapped yet

---

### Phase 2: Bootstrap Governance Identities (Week 1)

```bash
# Step 7: Configure authors.cpf.nft DNS → governance_canister
# In ENS Manager for authors.cpf.nft:
# - CNAME: @ → admin.cpf.icp1.io
# - TXT: _canister-id → $GOVERNANCE_ID

# Step 8: Register authors.cpf.nft with IC
curl -X POST https://icp0.io/registrations \
  -H "Content-Type: application/json" \
  -d '{
    "name": "authors.cpf.nft",
    "canister_id": "'$GOVERNANCE_ID'"
  }'

# Step 9: Board members authenticate with authors.cpf.nft
# - Board members visit https://authors.cpf.nft
# - Authenticate with Internet Identity
# - Copy their principals

# Step 10: Initialize board members in voting canister
BOARD_MEMBER_1="xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-cai"  # From step 9
BOARD_MEMBER_2="yyyyy-yyyyy-yyyyy-yyyyy-yyyyy-cai"
BOARD_MEMBER_3="zzzzz-zzzzz-zzzzz-zzzzz-zzzzz-cai"

dfx canister call voting_canister initializeBoardMembers \
  "(vec {
    principal \"$BOARD_MEMBER_1\";
    principal \"$BOARD_MEMBER_2\";
    principal \"$BOARD_MEMBER_3\"
  })" \
  --network ic
```

**State after Phase 2:**
- ✅ authors.cpf.nft accessible and points to governance canister
- ✅ Board members have Internet Identity principals (from authors.cpf.nft)
- ✅ Board members initialized in voting canister
- ❌ cpf.nft (user-facing) not configured yet

---

### Phase 3: Transfer ENS to Governance (Week 1-2)

**Option A: Transfer to ICP-derived address (if chain-key code ready)**

```bash
# Step 11: Get ICP-derived Ethereum address
dfx canister call governance_canister getEthereumAddress --network ic
# Returns: "0xABCD...1234"

# Step 12: Transfer cpf.nft ENS NFT to that address
# In personal Ethereum wallet (MetaMask):
# - Visit https://app.ens.domains/cpf.nft
# - Transfer → 0xABCD...1234
# - Confirm transaction

# Step 13: Verify transfer
# Check on Etherscan that governance canister now owns cpf.nft
```

**Option B: Transfer to Gnosis Safe (simpler initially)**

```bash
# Step 11: Create Gnosis Safe (3-of-5 board members)
# - Visit https://app.safe.global
# - Create Safe with board member Ethereum addresses
# - Get Safe address: 0xSAFE...ADDRESS

# Step 12: Transfer cpf.nft to Gnosis Safe
# Same as Option A, but transfer to Safe address

# Step 13: Board members can now update ENS via Gnosis Safe app
```

**State after Phase 3:**
- ✅ cpf.nft ENS NFT controlled by governance (ICP or Gnosis Safe)
- ✅ Board members can update ENS records via governance
- ❌ cpf.nft still not accessible (DNS not configured)

---

### Phase 4: Configure User-Facing DNS (Week 2)

```bash
# Step 14: Configure cpf.nft DNS → cpf_nft_canister
# In ENS Manager (via governance):
# - CNAME: @ → cpf.icp1.io
# - TXT: _canister-id → $CPF_NFT_ID

# If using Gnosis Safe:
# - Board members vote in Safe app
# - Execute transaction to update ENS records

# If using ICP governance:
# - Board member submits proposal
# - 3-of-3 board approves
# - Governance canister signs Ethereum transaction

# Step 15: Deploy .well-known files in cpf_nft canister
# - .well-known/ic-domains → "cpf.nft"
# - .well-known/ii-alternative-origins → list of subdomains

# Step 16: Register cpf.nft with IC
curl -X POST https://icp0.io/registrations \
  -H "Content-Type: application/json" \
  -d '{
    "name": "cpf.nft",
    "canister_id": "'$CPF_NFT_ID'"
  }'

# Step 17: Wait for DNS propagation (24-48 hours)
# Monitor:
dig cpf.nft

# Step 18: Verify cpf.nft is accessible
curl https://cpf.nft
# Should return content from cpf_nft canister
```

**State after Phase 4:**
- ✅ cpf.nft accessible and serves content
- ✅ authors.cpf.nft accessible for governance
- ✅ Alternative origins configured
- ✅ Board controls ENS via governance
- 🎉 **BOOTSTRAP COMPLETE**

---

### Phase 5: Configure Alternative Origins (Week 2-3)

```bash
# Step 19: Configure DNS for alternative origin subdomains
# For each subdomain (coolplanet-foundation.org, newsletters.*, members.*):

# In your DNS provider (not ENS, unless you own these domains on ENS too):
# - CNAME subdomain.yourdomain.org → subdomain.yourdomain.icp1.io
# - TXT _canister-id.subdomain → [respective canister ID]

# Step 20: Register each alternative origin
curl -X POST https://icp0.io/registrations \
  -H "Content-Type: application/json" \
  -d '{
    "name": "coolplanet-foundation.org",
    "canister_id": "[cpf_org_frontend_canister_id]"
  }'

# Repeat for other subdomains

# Step 21: Update .well-known/ii-alternative-origins in cpf_nft canister
# Add all alternative origins to the list
# Redeploy cpf_nft canister

# Step 22: Verify cross-origin authentication works
# - Visit coolplanet-foundation.org
# - Authenticate with Internet Identity (using derivationOrigin: cpf.nft)
# - Get principal
# - Visit members.coolplanet-foundation.org
# - Authenticate again
# - Verify SAME principal
```

---

## Verification Procedures

### Verify ENS Ownership

```bash
# Check who owns cpf.nft
# Visit: https://app.ens.domains/cpf.nft
# Should show:
# - Governance canister's Ethereum address, OR
# - Gnosis Safe address

# Check on Etherscan
# Visit: https://etherscan.io/token/0x57f1887a8BF19b14fC0dF6Fd9B2acc9Af147eA85?a=[address]
# Should show cpf.nft in holdings
```

### Verify DNS Configuration

```bash
# Check DNS records
dig cpf.nft
dig authors.cpf.nft

# Should show CNAME pointing to IC boundary nodes

# Check TXT records
dig _canister-id.cpf.nft TXT
# Should return canister ID

# Check .well-known files
curl https://cpf.nft/.well-known/ic-domains
# Should return: "cpf.nft"

curl https://cpf.nft/.well-known/ii-alternative-origins
# Should return JSON with alternative origins
```

### Verify Admin Authentication

```bash
# Test authors.cpf.nft authentication
# 1. Visit https://authors.cpf.nft
# 2. Authenticate with Internet Identity
# 3. Copy your principal

# 4. Verify this principal is in board members list
dfx canister call voting_canister getBoardMembers --network ic
# Should include your principal
```

### Verify User Authentication & Cross-Origin

```bash
# Test cpf.nft derivation origin
# 1. Visit https://coolplanet-foundation.org
# 2. Open browser console
# 3. Check auth code:

const authClient = await AuthClient.create();
await authClient.login({
  identityProvider: "https://identity.ic0.app",
  derivationOrigin: "https://cpf.nft",
  onSuccess: () => {
    const principal = authClient.getIdentity().getPrincipal().toText();
    console.log("Principal:", principal);
  }
});

# 4. Note the principal
# 5. Visit https://members.coolplanet-foundation.org
# 6. Authenticate again (same code)
# 7. Verify SAME principal (cross-origin works!)
```

---

## Troubleshooting

### ENS NFT Transfer Failed

**Symptom:** Transaction reverted when transferring cpf.nft

**Possible Causes:**
1. Not the current owner
2. Insufficient gas
3. Transfer to invalid address

**Fix:**
```bash
# Verify current owner
# Visit: https://app.ens.domains/cpf.nft

# Check Ethereum address is valid
# Visit: https://etherscan.io/address/[target-address]

# Try again with higher gas limit
```

### DNS Not Resolving

**Symptom:** `curl https://cpf.nft` fails or times out

**Possible Causes:**
1. DNS not propagated yet (can take 24-48 hours)
2. Wrong CNAME target
3. Missing TXT record

**Fix:**
```bash
# Check DNS propagation
dig cpf.nft

# Verify CNAME points to IC boundary nodes
# Should show: cpf.icp1.io or similar

# Verify TXT record exists
dig _canister-id.cpf.nft TXT

# If missing, add in ENS Manager
```

### Canister Not Serving .well-known Files

**Symptom:** `curl https://cpf.nft/.well-known/ic-domains` returns 404

**Possible Causes:**
1. Canister doesn't handle this route
2. Wrong HTTP request handler

**Fix:**
```rust
// In canister code, ensure http_request handles .well-known

#[query]
fn http_request(req: HttpRequest) -> HttpResponse {
    match req.url.as_str() {
        "/.well-known/ic-domains" => HttpResponse {
            status_code: 200,
            headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
            body: b"cpf.nft".to_vec(),
        },
        // ... other routes
    }
}

// Redeploy canister
dfx deploy cpf_nft --network ic
```

### Alternative Origins Not Working

**Symptom:** Get different principals on different subdomains

**Possible Causes:**
1. .well-known/ii-alternative-origins not served correctly
2. CORS headers missing
3. Alternative origin not in the list

**Fix:**
```bash
# Verify file is served
curl https://cpf.nft/.well-known/ii-alternative-origins

# Should return JSON with CORS header:
# Access-Control-Allow-Origin: *

# Verify target subdomain is in the list
# If not, update and redeploy cpf_nft canister

# Check CORS headers
curl -I https://cpf.nft/.well-known/ii-alternative-origins
# Should include: Access-Control-Allow-Origin: *
```

### Board Member Can't Access Governance

**Symptom:** Board member gets "Unauthorized" when calling governance functions

**Possible Causes:**
1. Used wrong derivation origin (cpf.nft instead of authors.cpf.nft)
2. Principal not in board members list
3. Wrong canister ID

**Fix:**
```bash
# Verify board member is using authors.cpf.nft
# Check browser console for derivation origin in auth call

# Verify principal is in board list
dfx canister call voting_canister getBoardMembers --network ic

# If not in list, add via controller-only call
dfx canister call voting_canister addBoardMember \
  "(principal \"xxxxx-xxxxx-xxxxx\")" \
  --network ic
```

---

## Summary Checklist

### ENS Setup
- [ ] cpf.nft ENS NFT acquired
- [ ] authors.cpf.nft subdomain created
- [ ] ENS NFT transferred to governance (ICP or Gnosis Safe)
- [ ] Board members can update ENS records

### DNS Configuration
- [ ] cpf.nft CNAME → cpf.icp1.io
- [ ] cpf.nft TXT _canister-id → cpf_nft_canister
- [ ] authors.cpf.nft CNAME → admin.cpf.icp1.io
- [ ] authors.cpf.nft TXT _canister-id → governance_canister
- [ ] Both domains registered with IC boundary nodes
- [ ] DNS propagation complete (24-48 hours)

### Canister Configuration
- [ ] cpf_nft_canister serves .well-known/ic-domains
- [ ] cpf_nft_canister serves .well-known/ii-alternative-origins
- [ ] governance_canister serves .well-known/ic-domains
- [ ] All canisters deployed and accessible

### Identity Bootstrap
- [ ] Board members authenticated with authors.cpf.nft
- [ ] Board member principals initialized in voting canister
- [ ] Users can authenticate with cpf.nft
- [ ] Cross-origin authentication works (same principal across subdomains)

### Verification
- [ ] https://cpf.nft accessible
- [ ] https://authors.cpf.nft accessible
- [ ] Alternative origins return same principal
- [ ] Board members can submit/approve governance proposals
- [ ] ENS updates work via governance

---

**Last Updated:** 2025-11-14
**Version:** 2.0.0 (Refactored to focus on ENS-specific content)
**Status:** Complete Guide
**Next Steps:** Follow bootstrap sequence in order, verify each step

**Changes in v2.0.0:**
- Extracted generic IC DNS content to [ic_dns.md](./ic_dns.md)
- Now focuses on ENS-specific aspects (acquisition, ownership, governance)
- Clarified coexistence of ENS and traditional DNS domains
- Consistent with fti_newsletter_archive/docs/DNS_SETUP.md

**Cross-References:**
- **[ic_dns.md](./ic_dns.md)** - Generic IC DNS setup (all domains)
- [canister-architecture-diagram.md](./canister-architecture-diagram.md) - Identity architecture
- [governance-policy.md](./governance-policy.md) - Board control of ENS
- [derivation-origins-integration.md](./derivation-origins-integration.md) - Alternative origins technical details
- fti_newsletter_archive/docs/DNS_SETUP.md - Detailed newsletter subdomain setup
