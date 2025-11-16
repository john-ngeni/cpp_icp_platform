# CPP Admin Architecture: Security, Bootstrap & Cross-Repo Coordination

**Date:** 2025-11-13
**Purpose:** Document admin security, bootstrap sequences, and cross-repository admin coordination for cpp_icp_platform
**Context:** How admin systems are securely initialized and governed across all CPP repos
**Cross-Reference:** See [canister-architecture-diagram.md](./canister-architecture-diagram.md) for overall canister structure

> **NOTE:** This document focuses on **cross-repository admin architecture, security, and bootstrap**. Payment-specific admin details have been moved to the cpf_members repository.

## Executive Summary

The CPP platform requires **secure admin capabilities** across multiple repositories with different patterns:

- **fti_newsletter_archive**: Admin integrated into same backend_api canister (unified microservice)
- **cpf_members**: Admin module in backend_canister + separate admin_dashboard frontend
- **cpp_icp_platform**: Platform governance and multi-sig operations (future)

**Key Focus:** This document addresses:
1. **Admin bootstrap sequences** (how to securely initialize admin systems)
2. **Cross-repo admin security coordination** (consistent RBAC and authentication)
3. **Accounting data location and governance** (where financial records are stored)
4. **Multi-sig governance** for critical infrastructure

## Admin Capabilities by Repository

### **1. Content & Author Management Admin (Newsletter)**

**Repository:** `fti_newsletter_archive`
**Pattern:** Unified microservice (admin integrated in backend_api)
**Details:** See fti_newsletter_archive repo documentation

**Key Features:**
- RBAC hierarchy: Superadmin → Admin → Editor → Author → Viewer
- Permission-gated admin endpoints
- Admin analytics and user management
- Newsletter publishing workflow

**Admin Dashboard:** `https://newsletters.coolplanet-foundation.org/admin`

---

### **2. Payment Administration (Members)**

> ⚠️ **MOVE TO cpf_members REPO:** Detailed payment admin docs belong in cpf_members repo

**Repository:** `cpf_members`
**Pattern:** Modular (Admin.mo module + separate admin_dashboard)
**Details:** See cpf_members repo documentation

**Key Features:**
- KYB/EDD review for donations >$15K
- Hybrid Stripe Dashboard + Custom Admin approach
- Donation approval/rejection workflow
- NFT minting integration

**Admin Dashboard:** `https://members.coolplanet-foundation.org/admin`

---

## Accounting Data & NFT Award Workflow

> **For detailed NFT award workflow, wallet architecture, and accounting reconciliation, see [nft_award.md](./nft_award.md)**

### **High-Level Overview**

The CPP platform maintains accounting data across **three systems**:

1. **Stripe**: Payment processing (Payment Intents, settlements)
2. **IC Canister**: Business logic, user profiles, KYB/EDD review, wallet management
3. **Polygon NFT**: NFT ownership (Einstein contract)

**Key Linkages:**
- Stripe → IC: Payment Intent ID
- IC → User: Internet Identity principal
- User → Ethereum Wallet: External (user-provided) or Chain-Key (IC-derived)
- IC → NFT: NFT token ID

**Dual-Wallet Architecture:**
- **External Wallet**: User provides their own Ethereum wallet (MetaMask, Ledger, etc.)
- **Chain-Key Wallet**: IC derives Ethereum wallet from II Principal using threshold ECDSA

### **Admin Governance of Accounting Data**

| System | Admin Access | Bootstrap Required | Governance |
|--------|--------------|-------------------|------------|
| **Stripe Account** | Stripe Dashboard + API keys | Stripe account setup, API key generation | Multi-factor auth, credentials in password manager |
| **IC Canister** | Admin principals (Internet Identity) | Initialize admin via controller-only call | See [Canister Bootstrap](#canister-bootstrap--admin-initialization) section |
| **Einstein Contract** | Multi-sig (3-of-5 Gnosis Safe) | Deploy contract, set controller | 3-of-5 multi-sig for minting authority |

**Admin Responsibilities:**
- **Payment Admin**: Review >$15K donations (KYB/EDD), approve/reject
- **Reconciliation**: Monthly audit of Stripe → IC → Polygon data flow
- **Wallet Audit**: Verify wallet sources (External vs ChainKeyDerived)
- **Security**: Monitor for discrepancies, unauthorized changes

**See [nft_award.md](./nft_award.md) for:**
- Detailed data types and schemas
- NFT minting workflow
- Wallet derivation (chain-key ECDSA)
- Reconciliation procedures
- Security considerations for both wallet types

---

### **3. Platform Admin (Governance & Operations)**

**Primary Repository:** `cpp_icp_platform` (future)
**Implementation:** TBD
**Purpose:** Platform-wide operations, governance, technical admin

#### **Planned Admin Functions:**

```motoko
// cpp_icp_platform/canisters/admin/main.mo (future)
actor PlatformAdmin {
  // Canister management
  public shared(msg) func upgradeCanister(
    canisterId: Principal,
    wasmModule: Blob
  ): async Result<(), Text> {
    // Only multi-sig controller can upgrade
    if (not isMultiSigController(msg.caller)) {
      return #err("Unauthorized: Multi-sig required");
    };

    // Upgrade canister with new WASM
    await IC.install_code({
      mode = #upgrade;
      canister_id = canisterId;
      wasm_module = wasmModule;
    });

    #ok(())
  };

  // Cycle management
  public shared(msg) func topUpCycles(
    canisterId: Principal,
    amount: Nat
  ): async Result<(), Text> {
    // Admin can top up cycles
  };

  // ENS domain management
  public shared(msg) func updateAlternativeOrigins(
    newOrigins: [Text]
  ): async Result<(), Text> {
    // Only multi-sig can update alternative origins
  };

  // Governance
  public shared(msg) func executeProposal(
    proposalId: Nat
  ): async Result<(), Text> {
    // Execute approved governance proposal
  };
}
```

---

## Admin Architecture Patterns

CPP uses **two distinct admin patterns** across repositories:

### **Pattern 1: Unified Microservice (Newsletter)**

**Repository:** fti_newsletter_archive
**Structure:** Admin integrated in same backend_api canister
**Best For:** Content management, simple RBAC

**Key Characteristics:**
- Admin and user endpoints in same actor
- Permission-gated admin functions
- Single deployment unit
- Shared data structures

**Trade-offs:**
- ✅ Simple deployment
- ✅ No cross-canister calls
- ⚠️ Tightly coupled

---

### **Pattern 2: Modular (Members)**

**Repository:** cpf_members
**Structure:** Admin.mo module + separate admin_dashboard frontend
**Best For:** Complex workflows, separate admin UI

**Key Characteristics:**
- Admin functions in separate module (Admin.mo)
- Dedicated admin_dashboard Svelte app
- Modular code organization

**Trade-offs:**
- ✅ Clear separation of concerns
- ✅ Independent admin UI deployment
- ⚠️ More complex setup

---

### **Pattern 3: Separate Admin Canister (Platform - Future)**

**Repository:** cpp_icp_platform
**Structure:** Dedicated admin canister with multi-sig controller
**Best For:** Platform governance, high-security operations

**Key Characteristics:**
- Complete security isolation
- Multi-sig required for operations
- Cross-canister calls to managed canisters
- Platform-wide governance

**Trade-offs:**
- ✅ Security isolation
- ✅ Multi-sig governance
- ⚠️ Cross-canister latency
- ⚠️ Higher complexity

**Use Cases:**
- Canister upgrades across all repos
- ENS domain management (cpf.nft alternative origins)
- Cross-repository permission coordination
- Platform-wide settings

---

## Admin Authentication & Authorization Patterns

### **Authentication Flow:**

```
Admin authenticates with Internet Identity
    ↓
Backend verifies principal against admin store
    ↓
Returns admin permissions for this principal
```

### **Authorization Patterns:**

**Pattern 1: Role-Based (Newsletter)**
- Store role in user profile (Superadmin, Admin, Editor, Author, Viewer)
- Check role on every admin endpoint call
- Granular permissions per role

**Pattern 2: Admin List (Members)**
- Maintain list of admin principals
- Binary check: isAdmin() returns true/false
- Simpler, less granular

**Pattern 3: Controller-Only (Bootstrap)**
- Check if caller is canister controller
- For system-level operations only
- See [canister-bootstrap-pattern.md](./canister-bootstrap-pattern.md)

**Pattern 4: Multi-Sig (Platform - Future)**
- Require 3-of-5 multi-sig approval
- For critical infrastructure changes
- Uses Gnosis Safe or SNS/DAO

---

## Canister Bootstrap & Admin Initialization

> **For detailed bootstrap security patterns, see [canister-bootstrap-pattern.md](./canister-bootstrap-pattern.md)**

### **Three-Level Identity Hierarchy**

> **For complete identity architecture and ENS setup, see [ens-dns-setup.md](./ens-dns-setup.md)**

CPP platform uses **three distinct identity levels** that are often confused. Each level has different purposes, principals, and capabilities:

#### **Level 1: Controller (dfx identity)**

**Purpose:** Infrastructure deployment and system-level control

**Who:**
- Technical lead during bootstrap
- Governance canister (after transition)

**Principal Type:** dfx identity (NOT Internet Identity)

**Capabilities:**
- Deploy canisters
- Upgrade canisters
- Manage cycles
- Add/remove controllers
- Call controller-only initialization functions

**Example:**
```bash
# Controller deploys canister
dfx deploy backend_api --network ic

# Controller gets dfx principal
dfx identity get-principal
# Returns: xxxxx-xxxxx-xxxxx (dfx identity)
```

**Bootstrap:**
```bash
# 1. Controller deploys infrastructure
dfx deploy governance_canister --network ic

# 2. Set governance canister as controller (transition control)
dfx canister update-settings backend_api \
  --add-controller $(dfx canister id governance_canister)

# 3. Optionally remove personal dfx controller
dfx canister update-settings backend_api \
  --remove-controller $(dfx identity get-principal)
```

---

#### **Level 2: Board Governance (governance.cpf.nft identities)**

**Purpose:** Strategic governance of ENS and IC infrastructure

**Who:**
- 3 board members
- **Authenticate using governance.cpf.nft derivation origin**

**Principal Type:** Internet Identity (governance.cpf.nft derivation)

**Capabilities:**
- Control cpf.nft ENS NFT (update DNS, create subdomains)
- Approve governance proposals (3-of-3 required)
- Deploy new canisters (via governance proposal)
- Upgrade production canisters (via governance proposal)
- Hire/fire technical signers
- Approve major strategic decisions (NFT deployment, funding >100 ICP)

**Example:**
```typescript
// Board member authenticates with governance.cpf.nft
const authClient = await AuthClient.create();
await authClient.login({
  identityProvider: "https://identity.ic0.app",
  derivationOrigin: "https://governance.cpf.nft",  // ← Board only!
  onSuccess: () => {
    const principal = authClient.getIdentity().getPrincipal().toText();
    // This principal can submit/approve governance proposals
  }
});
```

**Bootstrap:**
```bash
# 1. Configure governance.cpf.nft DNS → governance_canister
# See ens-dns-setup.md § Phase 2

# 2. Board members authenticate with governance.cpf.nft
# Get their II principals (from governance.cpf.nft derivation)
BOARD_MEMBER_1="xxxxx-xxxxx-xxxxx"  # From governance.cpf.nft auth

# 3. Initialize board members in voting canister
dfx canister call voting_canister initializeBoardMembers \
  "(vec { principal \"$BOARD_MEMBER_1\"; ... })" \
  --network ic
```

**Key Insight:** Board members get **different Internet Identity principals** when using governance.cpf.nft vs cpf.nft!

---

#### **Level 3: Technical/Application Admin (cpf.nft identities)**

**Purpose:** Application-level administration and day-to-day operations

**Who:**
- Technical team (1-2 initially → 3-5 as team grows)
- Content managers, editors
- **Authenticate using cpf.nft derivation origin** (same as users!)

**Principal Type:** Internet Identity (cpf.nft derivation)

**Capabilities:**
- Upgrade production canisters (via technical multi-sig)
- Manage cycles (routine top-ups)
- Configure monitoring and alerts
- Emergency responses (pause/unpause)
- Application admin:
  - Manage user roles (RBAC)
  - Moderate content
  - Review donations >$15K (KYB/EDD)
  - Blacklist malicious users

**Example:**
```typescript
// Technical admin authenticates with cpf.nft (same as users)
const authClient = await AuthClient.create();
await authClient.login({
  identityProvider: "https://identity.ic0.app",
  derivationOrigin: "https://cpf.nft",  // ← Users AND admins
  onSuccess: () => {
    const principal = authClient.getIdentity().getPrincipal().toText();
    // This principal checked against admin list in canister
  }
});

// In canister:
public shared(msg) func approveHighValueDonation(donationId: Nat): async Result<(), Text> {
  if (not isAdmin(msg.caller)) {
    return #err("Unauthorized: Admin access required");
  };
  // Admin-specific logic
};
```

**Bootstrap:**
```bash
# 1. Technical admin authenticates with cpf.nft (after DNS configured)
# Gets Internet Identity principal (from cpf.nft derivation)
TECH_ADMIN_1="aaaaa-aaaaa-aaaaa"  # From cpf.nft auth

# 2. Initialize as superadmin in application canisters
dfx canister call backend_api initializeSuperadmin \
  "(principal \"$TECH_ADMIN_1\")" \
  --network ic
```

---

### **Critical Distinctions**

| Level | Principal Type | Derivation Origin | Purpose | Bootstrap Method |
|-------|---------------|------------------|---------|-----------------|
| **Controller** | dfx identity | N/A | Infrastructure | `dfx deploy` |
| **Board Governance** | Internet Identity | **governance.cpf.nft** | Strategic governance | `initializeBoardMembers()` |
| **Technical Admin** | Internet Identity | **cpf.nft** | Application admin | `initializeSuperadmin()` |

**Why This Matters:**

1. **Bootstrap Facilitation:**
   - Controller deploys everything (dfx identity)
   - Board identities established with governance.cpf.nft
   - Board controls transition to governance
   - Technical admins use cpf.nft (separate from board)

2. **Security Isolation:**
   - Board governance identities (governance.cpf.nft) never exposed to user-facing services
   - Even if cpf.nft services compromised, governance safe
   - Board members can have user accounts with different principals

3. **Operational Clarity:**
   - Strategic decisions (board via governance.cpf.nft)
   - Technical operations (admins via cpf.nft)
   - Clear separation in audit trails

**See [governance-policy.md](./governance-policy.md) for complete governance structure and approval requirements**

---

### **Cross-Repository Admin Coordination**

**Pattern:** Use same Internet Identity principals as superadmins across all repos

| Admin | Principal (II) | Newsletter Role | Members Role | Platform Role |
|-------|---------------|----------------|--------------|---------------|
| **Admin 1** | `aaaaa-aaaaa-aaaaa` | Superadmin | Admin | Multi-sig signer |
| **Admin 2** | `bbbbb-bbbbb-bbbbb` | Admin | Admin | Multi-sig signer |
| **Editor 1** | `ccccc-ccccc-ccccc` | Editor | - | - |

**Why Coordinate?**
- Consistent admin access across all CPP services
- Simplified user management
- Single sign-on via Internet Identity
- Unified audit trail

**How to Bootstrap:**
```bash
# Store principals in config (NOT committed to git)
SUPERADMIN_1="aaaaa-aaaaa-aaaaa"

# Initialize across repos (controller-only calls)
cd ~/git/fti_newsletter_archive
dfx canister call backend_api initializeSuperadmin "(principal \"$SUPERADMIN_1\")"

cd ~/git/cpf_members
dfx canister call backend_canister initializeAdmin "(principal \"$SUPERADMIN_1\")"

cd ~/git/cpp_icp_platform
dfx canister call platform_admin initializeSuperadmin "(principal \"$SUPERADMIN_1\")"
```

---

### **Multi-Sig Controller for Production**

**Problem:** Single controller = single point of failure

**Solution:** Staged governance evolution

> **For detailed multi-sig comparison and implementation, see [multi-sig-governance-comparison.md](./multi-sig-governance-comparison.md)**

**Recommended Approach:**
1. **Stage 1 (Dev/Preprod):** Single controller for rapid iteration
2. **Stage 2 (Production):** IC-native 3-of-5 multi-sig canister
3. **Stage 3 (Mature):** SNS governance for full decentralization

**Why NOT Gnosis Safe:**
- ❌ Cross-chain complexity (Ethereum + Chain Fusion bridge)
- ❌ High friction (days + gas fees for every upgrade)
- ❌ External dependencies

**Why IC-Native Multi-Sig:**
- ✅ Low friction (minutes for approvals)
- ✅ No external dependencies
- ✅ Low cost (only IC cycles)
- ✅ Suitable for active development

**Applies to:**
- Platform admin canister (cpp_icp_platform)
- ENS domain controller (cpf.nft)
- Critical infrastructure canisters

**See:** [multi-sig-governance-comparison.md](./multi-sig-governance-comparison.md) for full comparison and implementation

---


## Security Considerations

### **Admin Security Principles:**

1. **Separation of Privileges:**
   - Canister Controller (dfx) ≠ Application Superadmin (II)
   - Never use dfx identity as superadmin
   - See [canister-bootstrap-pattern.md](./canister-bootstrap-pattern.md)

2. **Authentication:**
   - All admin functions require Internet Identity authentication
   - Controller-only functions check canister controller

3. **Authorization:**
   - RBAC for different admin levels (where applicable)
   - Principle of least privilege
   - Consistent admin principals across repos

4. **Audit Logging:**
   - Log all admin actions (timestamp, actor, action, target)
   - Store logs in stable storage
   - Regular audit review

5. **Multi-Sig for Critical Operations:**
   - ENS domain changes (cpf.nft)
   - Canister upgrades in production
   - Platform governance decisions
   - Use 3-of-5 Gnosis Safe

6. **Cross-Repository Coordination:**
   - Same admin principals across all repos
   - Consistent security policies
   - Coordinated security updates

---

## Implementation Roadmap

### **Phase 1: Repository-Level Admin (Complete/In Progress)**

**Newsletter (fti_newsletter_archive):**
- ✅ RBAC system deployed
- ✅ Bootstrap pattern documented
- ✅ Unified microservice pattern

**Members (cpf_members):**
- 🚧 Admin module (Admin.mo)
- 🚧 Admin dashboard (Svelte app)
- 🚧 Bootstrap sequence
- See cpf_members repo for details

### **Phase 2: Cross-Repository Coordination (Current)**
- ✅ Bootstrap security pattern generalized (canister-bootstrap-pattern.md)
- ✅ Accounting data governance documented
- 🚧 Admin principal coordination table
- ⏳ Shared admin config file (not committed to git)
- ⏳ Cross-repo bootstrap script

### **Phase 3: Platform Admin & Multi-Sig Governance (Future)**
- ⏳ Deploy platform admin canister (cpp_icp_platform)
- ⏳ Configure 3-of-5 Gnosis Safe as controller
- ⏳ ENS domain management (cpf.nft alternative origins)
- ⏳ Cross-canister upgrade governance
- ⏳ Platform-wide operations canister

### **Phase 4: Accounting Reconciliation (Future)**
- ⏳ Automated reconciliation: Stripe ↔ IC ↔ Polygon
- ⏳ Monthly compliance reports
- ⏳ Discrepancy alerting
- ⏳ Backup and disaster recovery

---

## Summary: Admin Architecture Patterns & Bootstrap

| Repository | Pattern | Admin UI | Bootstrap Method | Cross-Repo Coordination |
|------------|---------|----------|-----------------|------------------------|
| **fti_newsletter_archive** | Unified (admin in same canister) | Part of frontend canister | initializeRBAC() on deploy | Shared admin principals |
| **cpf_members** | Modular (Admin.mo module + separate dashboard) | Separate Svelte admin_dashboard | initialize() + addAdmin() | Shared admin principals |
| **cpp_icp_platform** | Separate admin canister (future) | TBD | Multi-sig controller | Multi-sig governance |

---

## Key Architectural Decisions for cpp_icp_platform

**What belongs in this repo:**
1. ✅ **Bootstrap security pattern** - Generalized pattern in [canister-bootstrap-pattern.md](./canister-bootstrap-pattern.md)
2. ✅ **Cross-repo admin coordination** - Ensuring consistent admin principals
3. ✅ **Accounting data governance** - High-level (detailed in [nft_award.md](./nft_award.md))
4. ✅ **Multi-sig governance** - 3-of-5 Gnosis Safe for critical infrastructure
5. ✅ **Admin architecture patterns** - High-level patterns across repositories
6. ✅ **Platform admin canister** - Future governance (cpp_icp_platform specific)
7. ✅ **NFT award workflow** - Extracted to [nft_award.md](./nft_award.md)

**What belongs in other repos:**

**cpf_members repo:**
- Payment admin implementation (KYB/EDD workflow)
- Stripe vs Custom admin decision details
- Admin dashboard implementation
- Payment admin user flows
- Donation review endpoints

**fti_newsletter_archive repo:**
- Newsletter RBAC implementation
- Content management admin endpoints
- Newsletter publishing workflow
- Author/Editor admin flows

---

## Critical Bootstrap Checklist

**Before production deployment:**

**Controller & Infrastructure:**
- [ ] **Controller Security:** dfx identity backed up, stored securely
- [ ] **Multi-Sig Setup:** 3-of-5 Gnosis Safe configured for critical canisters
- [ ] **ENS Domain:** Verify cpf.nft controlled by multi-sig
- [ ] **Bootstrap Scripts:** Test in preprod, document actual commands

**Admin Principals:**
- [ ] **Admin Principals:** Document all Internet Identity principals (secure storage)
- [ ] **Cross-Repo Coordination:** Bootstrap same admin principals across all repos
- [ ] **Newsletter Admin:** Initialize superadmin in fti_newsletter_archive
- [ ] **Members Admin:** Initialize admin in cpf_members
- [ ] **Platform Admin:** Deploy and initialize platform admin canister (future)

**Accounting & Integration:**
- [ ] **Stripe Account:** Set up, API keys generated, credentials secured
- [ ] **Payment Integration:** Test Stripe webhooks, Payment Intent ID linking
- [ ] **Accounting Reconciliation:** Monthly reconciliation process documented
- [ ] **NFT Workflow:** See [nft_award.md](./nft_award.md) for NFT/wallet checklist

**Verification & Security:**
- [ ] **Admin Access Verification:** Confirm admin access works in each repo
- [ ] **Audit Logging:** Enable logging for all admin actions
- [ ] **Recovery Plan:** Document recovery procedures (controller, admin principals)
- [ ] **Security Review:** Cross-repo security audit before production

**Critical Distinctions:**
- **Canister Controller (dfx) ≠ Application Superadmin (II)** - Bootstrap separately
- **Infrastructure (controller) ≠ Application (admin)** - Different privilege levels
- See [canister-bootstrap-pattern.md](./canister-bootstrap-pattern.md) for details

---

**Last Updated:** 2025-11-13
**Version:** 4.0.0 (Extracted NFT workflow to nft_award.md, refocused on admin bootstrap)
**Status:** Active Development
**Review Cycle:** Update as admin systems are bootstrapped

**Next Steps:**
1. Create payment admin docs in cpf_members repo (admin dashboard, KYB/EDD review)
2. Test bootstrap sequences in preprod (controller + superadmin separation)
3. Create shared admin config file (not committed to git)
4. Implement cross-repo bootstrap script
5. Document actual admin principals (currently using placeholders)
6. See [nft_award.md](./nft_award.md) for NFT workflow implementation steps
