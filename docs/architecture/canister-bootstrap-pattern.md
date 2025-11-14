# Canister Bootstrap Security Pattern

**Date:** 2025-11-13
**Purpose:** Generalized bootstrap pattern for IC canisters across CPP repositories
**Context:** Separation between canister controller (dfx) and application superadmin (II)
**Cross-Reference:** See [admin-architecture.md](./admin-architecture.md) for admin capabilities

## Executive Summary

IC canisters have **two distinct privilege levels** that must be bootstrapped separately:

1. **Canister Controller** (dfx identity): System-level control (upgrade, reinstall, manage cycles)
2. **Application Superadmin** (Internet Identity): Application-level admin privileges (manage users, content, settings)

**Critical Distinction:**
- Controller = Infrastructure operator (DevOps, system administrator)
- Superadmin = Application administrator (Content manager, business admin)

These are **separate principals** and require **separate bootstrap sequences**.

---

## The Two-Level Privilege Model

### Level 1: Canister Controller (dfx Identity)

**What is it?**
- A dfx principal that has system-level control over the canister
- Set during canister creation or via `dfx canister update-settings`
- NOT an Internet Identity (cannot authenticate via II)

**Capabilities:**
- Deploy/upgrade/reinstall canister
- Manage canister cycles (top up, monitor)
- Delete canister
- Add/remove other controllers
- Call controller-only canister functions

**Bootstrap Method:**
```bash
# Controller is automatically set during deployment
dfx deploy my_canister --network ic

# Controller = dfx identity that ran the deploy command
```

**Security:**
- Store dfx identity in hardware wallet or secure key management system
- Consider multi-sig controller (Gnosis Safe, SNS/DAO) for production
- Never commit dfx identity private keys to git

---

### Level 2: Application Superadmin (Internet Identity)

**What is it?**
- An Internet Identity principal that has admin privileges within the application
- Set via canister initialization function or admin-only endpoint
- Used by humans to log in and manage the application

**Capabilities:**
- Manage user roles and permissions (RBAC)
- Access admin dashboards
- Modify application settings
- Create/delete content
- View analytics

**Bootstrap Method:**
```motoko
// Example: Newsletter backend canister
actor BackendAPI {
  stable var initialized: Bool = false;
  stable var domainACLs: ACLStore = ACLStore.new();

  // Controller-only function to set initial superadmin
  public shared(msg) func initializeSuperadmin(
    superadminPrincipal: Principal
  ): async Result<(), Text> {
    // Only canister controller can call this
    if (not isController(msg.caller)) {
      return #err("Unauthorized: Controller access required");
    };

    if (initialized) {
      return #err("Already initialized");
    };

    // Set the superadmin principal (II)
    ACLStore.addSuperadmin(domainACLs, superadminPrincipal);
    initialized := true;
    #ok(())
  };
}
```

**Security:**
- Superadmin principal is an Internet Identity, NOT a dfx identity
- User authenticates via https://identity.ic0.app
- Canister verifies superadmin role via stored ACL/role data

---

## Bootstrap Sequence: Single Canister

### Step 1: Deploy Canister (Controller Level)

```bash
# Deploy canister - deployer becomes controller
dfx deploy backend_api --network ic

# Canister controller = dfx identity principal
# At this point, NO superadmin is set yet
```

**State after Step 1:**
- ✅ Canister deployed with controller
- ❌ No application superadmin set
- ⚠️ Admin functions will fail (no superadmin exists)

---

### Step 2: Set Application Superadmin

**Option A: Controller-Only Init Function (Recommended)**

```bash
# Get your Internet Identity principal
# (Log in at identity.ic0.app and copy principal)
II_PRINCIPAL="xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx"

# Call controller-only init function
dfx canister call backend_api initializeSuperadmin \
  "(principal \"$II_PRINCIPAL\")" \
  --network ic
```

**Option B: Hardcoded in Canister Code (Not Recommended)**

```motoko
actor BackendAPI {
  // DANGEROUS: Hardcoded superadmin
  let INITIAL_SUPERADMIN = Principal.fromText("xxxxx-xxxxx-xxxxx");

  system func preupgrade() {
    // Set superadmin on first upgrade
    if (not initialized) {
      ACLStore.addSuperadmin(domainACLs, INITIAL_SUPERADMIN);
      initialized := true;
    };
  };
}
```

**Why Option A is Better:**
- No secrets in code
- Flexible (can change superadmin without redeploying)
- Clear audit trail (init call is logged)

---

### Step 3: Verify Bootstrap

```bash
# Test that superadmin can access admin functions
# (Must authenticate with Internet Identity)

# From frontend or dfx (using II principal):
dfx canister call backend_api getAdminAnalytics \
  --network ic \
  --identity <your-ii-identity>
```

---

## Bootstrap Sequence: Multi-Canister (Same Repo)

**Scenario:** Frontend + Backend canisters that need to communicate

### Challenge: Cross-Canister References

**Example from fti_newsletter_archive:**
- Frontend (Rust) needs Backend's Principal (for authorization)
- Backend (Motoko) needs Frontend's Canister ID (for inter-canister calls)

**Bootstrap Order:**

```bash
# Step 1: Deploy backend first (gets stable canister ID)
dfx deploy backend_api --network ic
BACKEND_ID=$(dfx canister id backend_api --network ic)

# Step 2: Deploy frontend with backend Principal in init
dfx deploy frontend --network ic \
  --argument "(principal \"$BACKEND_ID\")"
FRONTEND_ID=$(dfx canister id frontend --network ic)

# Step 3: Link backend → frontend (controller-only call)
dfx canister call backend_api setFrontendCanisterId \
  "(\"$FRONTEND_ID\")" \
  --network ic

# Step 4: Initialize application superadmin (II principal)
dfx canister call backend_api initializeSuperadmin \
  "(principal \"$II_PRINCIPAL\")" \
  --network ic
```

**Key Points:**
- Steps 1-3: Controller-level bootstrap (canister linkage)
- Step 4: Application-level bootstrap (superadmin setup)
- Canister IDs are stable (don't change on upgrade)
- Frontend receives backend ID via `init()` (immutable)
- Backend receives frontend ID via controller-only call (mutable)

**Security Window:**
- Between Step 1 and Step 3: Backend doesn't know frontend yet
- Mitigation: `setFrontendCanisterId()` is controller-only
- Risk: LOW (attacker would need controller access)

---

## Bootstrap Sequence: Multi-Canister (Cross-Repo)

**Scenario:** Canisters in different repositories that need to communicate

**Example:** cpf_members → fti_newsletter_archive
- Members backend needs to call Newsletter backend for user profiles
- Newsletter backend needs to know Members backend for donation status

### Challenge: Circular Dependencies

**Problem:**
```
Members needs Newsletter's canister ID
Newsletter needs Members' canister ID
Which deploys first?
```

**Solution: Post-Deploy Linking**

```bash
# Repo 1: fti_newsletter_archive
cd ~/git/fti_newsletter_archive
dfx deploy backend_api --network ic
NEWSLETTER_BACKEND=$(dfx canister id backend_api --network ic)

# Repo 2: cpf_members
cd ~/git/cpf_members
dfx deploy backend_canister --network ic
MEMBERS_BACKEND=$(dfx canister id backend_canister --network ic)

# Link Newsletter → Members
cd ~/git/fti_newsletter_archive
dfx canister call backend_api setMembersCanisterId \
  "(\"$MEMBERS_BACKEND\")" \
  --network ic

# Link Members → Newsletter
cd ~/git/cpf_members
dfx canister call backend_canister setNewsletterCanisterId \
  "(\"$NEWSLETTER_BACKEND\")" \
  --network ic
```

**Controller-Only Linking Functions:**

```motoko
// fti_newsletter_archive/src/backend_api/main.mo
actor BackendAPI {
  stable var membersCanisterId: ?Text = null;

  // Controller-only: Link to members canister
  public shared(msg) func setMembersCanisterId(
    canisterId: Text
  ): async Result<(), Text> {
    if (not isController(msg.caller)) {
      return #err("Unauthorized: Controller only");
    };
    membersCanisterId := ?canisterId;
    #ok(())
  };
}
```

```motoko
// cpf_members/src/backend_canister/main.mo
actor MembersBackend {
  stable var newsletterCanisterId: ?Text = null;

  // Controller-only: Link to newsletter canister
  public shared(msg) func setNewsletterCanisterId(
    canisterId: Text
  ): async Result<(), Text> {
    if (not isController(msg.caller)) {
      return #err("Unauthorized: Controller only");
    };
    newsletterCanisterId := ?canisterId;
    #ok(())
  };
}
```

**Security:**
- All linking functions are controller-only
- Canister IDs are stored in stable vars (persist across upgrades)
- Can be updated if canister is reinstalled (rare)

---

## Admin Principal Coordination

**Pattern:** Same admin principals across all repos for consistency

### Bootstrap Superadmins Across Repos

```bash
# Define admin principals (Internet Identity)
SUPERADMIN_1="aaaaa-aaaaa-aaaaa-aaaaa-aaaaa"
SUPERADMIN_2="bbbbb-bbbbb-bbbbb-bbbbb-bbbbb"

# Initialize Newsletter superadmin
cd ~/git/fti_newsletter_archive
dfx canister call backend_api initializeSuperadmin \
  "(principal \"$SUPERADMIN_1\")" \
  --network ic

# Initialize Members admin
cd ~/git/cpf_members
dfx canister call backend_canister initializeAdmin \
  "(principal \"$SUPERADMIN_1\")" \
  --network ic

# Initialize Platform admin (future)
cd ~/git/cpp_icp_platform
dfx canister call platform_admin initializeSuperadmin \
  "(principal \"$SUPERADMIN_1\")" \
  --network ic
```

**Recommended: Store in Config File**

```json
// ~/git/.cpp-platform-config.json (NOT committed to git)
{
  "superadmins": [
    {
      "name": "Admin 1",
      "principal": "aaaaa-aaaaa-aaaaa-aaaaa-aaaaa",
      "email": "admin1@coolplanet.io",
      "role": "Superadmin"
    },
    {
      "name": "Admin 2",
      "principal": "bbbbb-bbbbb-bbbbb-bbbbb-bbbbb",
      "email": "admin2@coolplanet.io",
      "role": "Superadmin"
    }
  ]
}
```

---

## Multi-Sig Controller Pattern (Production)

### Problem: Single Controller = Single Point of Failure

**Risk:**
- If dfx identity is lost/compromised, entire canister is at risk
- No recovery mechanism
- No checks and balances on upgrades

### Solution: Multi-Sig Controller

**Option 1: Gnosis Safe (Ethereum-based)**
```bash
# Create 3-of-5 Gnosis Safe on Polygon
# Get Safe's IC principal via Chain Fusion bridge

# Add Gnosis Safe as controller
dfx canister update-settings backend_api \
  --add-controller <gnosis-safe-principal> \
  --network ic

# Remove individual controller (after testing)
dfx canister update-settings backend_api \
  --remove-controller <your-dfx-principal> \
  --network ic
```

**Option 2: SNS/DAO (IC-native)**
```bash
# Initialize SNS for the canister
dfx sns init <config-file>

# Canister controller becomes SNS governance canister
# All upgrades require DAO proposal + voting
```

**Recommended for CPP Platform:**
- **Preprod:** Individual dfx controller (faster iteration)
- **Production:** 3-of-5 Gnosis Safe (secure, decentralized)
- **Future:** Migrate to SNS/DAO (fully decentralized governance)

---

## Upgrade vs Reinstall

### Upgrade (Preserves Stable Storage)

```bash
# Upgrade preserves stable vars (canister links, superadmins)
dfx canister install backend_api --mode upgrade --network ic

# NO need to re-initialize:
# - Superadmin principals persist
# - Linked canister IDs persist
# - All stable storage intact
```

**Use upgrade for:**
- Code updates
- Bug fixes
- New features
- Production deployments

---

### Reinstall (Clears Stable Storage)

```bash
# Reinstall WIPES all stable storage
dfx canister install backend_api --mode reinstall --network ic

# MUST re-bootstrap:
# - Re-link canister IDs
# - Re-initialize superadmins
# - All data lost
```

**Use reinstall for:**
- Breaking changes to stable storage format
- Complete system reset
- Testing/development
- Recovery from corruption

**After Reinstall, Re-Bootstrap:**
```bash
# 1. Re-link canisters (if cross-canister)
dfx canister call backend_api setFrontendCanisterId "(\"$FRONTEND_ID\")"

# 2. Re-initialize superadmin
dfx canister call backend_api initializeSuperadmin "(principal \"$II_PRINCIPAL\")"

# 3. Verify bootstrap
dfx canister call backend_api getAdminAnalytics
```

---

## Security Best Practices

### Controller Security

1. **Protect dfx Identity:**
   - Use hardware wallet (Ledger, Trezor)
   - Never commit to git
   - Store backup securely (encrypted, offline)

2. **Multi-Sig for Production:**
   - 3-of-5 Gnosis Safe minimum
   - Signers across different jurisdictions
   - Test multi-sig approval workflow before production

3. **Least Privilege:**
   - Separate identities for dev/preprod/prod
   - Remove individual controllers after multi-sig setup
   - Regular audit of controller list

### Superadmin Security

1. **Separate from Controller:**
   - NEVER use dfx identity as superadmin
   - Use Internet Identity for superadmins
   - Enable 2FA on Internet Identity

2. **Immutable Root:**
   - First superadmin cannot be removed
   - Only add/remove additional admins
   - Document initial superadmin in secure location

3. **Role-Based Access:**
   - Superadmin → Admin → Editor → Author → Viewer
   - Principle of least privilege
   - Regular audit of admin list

### Audit Trail

```motoko
// Log all controller-only calls
stable var bootstrapLog: [BootstrapEvent] = [];

type BootstrapEvent = {
  timestamp: Nat64;
  caller: Principal;
  action: Text;  // "setFrontendCanisterId", "initializeSuperadmin"
  target: ?Principal;
};

public shared(msg) func setFrontendCanisterId(id: Text): async Result<(), Text> {
  if (not isController(msg.caller)) {
    return #err("Unauthorized");
  };

  // Log the action
  bootstrapLog := Array.append(bootstrapLog, [{
    timestamp = Time.now();
    caller = msg.caller;
    action = "setFrontendCanisterId";
    target = ?Principal.fromText(id);
  }]);

  frontendCanisterId := ?id;
  #ok(())
};
```

---

## Bootstrap Checklist

### Before Production Deployment

- [ ] Controller identity backed up securely
- [ ] Multi-sig controller configured (3-of-5 minimum)
- [ ] Superadmin principals documented
- [ ] Cross-canister links established
- [ ] Bootstrap sequence tested in preprod
- [ ] Verification script confirms all linkages
- [ ] Audit log reviewed for unexpected calls
- [ ] Recovery plan documented

### After Production Deployment

- [ ] Verify canister IDs match expected values
- [ ] Test superadmin login via Internet Identity
- [ ] Confirm cross-canister communication works
- [ ] Test admin functions (analytics, user management)
- [ ] Monitor canister cycles
- [ ] Document actual principals in secure location
- [ ] Remove temporary controllers

---

## Common Pitfalls

### Pitfall 1: Confusing Controller with Superadmin

**Wrong:**
```bash
# Using dfx identity as superadmin
dfx identity get-principal  # Returns dfx principal
# This is a CONTROLLER, not a superadmin
```

**Right:**
```bash
# Get Internet Identity principal for superadmin
# Log in at identity.ic0.app, copy principal
II_PRINCIPAL="xxxxx-xxxxx-xxxxx"
```

---

### Pitfall 2: Forgetting to Re-Link After Reinstall

**Symptom:** Cross-canister calls fail after reinstall

**Cause:** Stable storage wiped, canister IDs lost

**Fix:**
```bash
# Re-establish all canister links
dfx canister call backend_api setFrontendCanisterId "(\"$FRONTEND_ID\")"
```

---

### Pitfall 3: Losing Controller Access

**Symptom:** Cannot upgrade canister

**Cause:** Controller identity lost or deleted

**Prevention:**
- Backup controller identity
- Use multi-sig controller
- Document recovery procedure

**Recovery:**
- If multi-sig: Use other signers
- If single controller: Canister is unrecoverable

---

## Summary

| Concept | Controller (dfx) | Superadmin (II) |
|---------|-----------------|-----------------|
| **Type** | dfx identity | Internet Identity |
| **Purpose** | System control | Application admin |
| **Capabilities** | Upgrade, cycles, delete | Manage users, content, settings |
| **Bootstrap** | Automatic on deploy | Manual init function |
| **Security** | Hardware wallet, multi-sig | Internet Identity + 2FA |
| **Used By** | DevOps, infrastructure | Business admins, content managers |

**Key Takeaway:** Controller ≠ Superadmin. Bootstrap both separately.

---

**Last Updated:** 2025-11-13
**Version:** 1.0.0
**Status:** Active Documentation
**References:**
- [fti_newsletter_archive/docs/CANISTER_BOOTSTRAP_SECURITY.md](/Users/john/git/fti_newsletter_archive/docs/CANISTER_BOOTSTRAP_SECURITY.md)
- [admin-architecture.md](./admin-architecture.md)
