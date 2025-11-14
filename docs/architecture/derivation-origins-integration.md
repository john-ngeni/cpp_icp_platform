# Derivation Origins Integration: Cross-Repository Identity Architecture

**Date:** 2025-11-13
**Purpose:** Define how derivation origins work across all CPP repositories
**Context:** Ensure consistent Internet Identity principals across distributed services
**Cross-Reference:** See [origins.md](./origins.md) for technical deep-dive, [user-journey-funnel.md](./user-journey-funnel.md) for user flows

## Executive Summary

The CPP platform spans **multiple repositories** (`cpf_org`, `fti_newsletter_archive`, `cpf_members`) but presents a **unified identity experience** to users. This is achieved through careful **derivation origin configuration** that ensures users have the **same Internet Identity principal** regardless of which repository they're interacting with.

**Critical Architectural Decision:** Use `cpf.nft` as the **canonical derivation origin** for all CPP services to ensure ICANN independence and unified identity.

## The Problem: Multiple Domains, One Identity

### **Repository Domain Mapping**

CPP services are deployed across multiple domains:

```
cpf_org                    → coolplanet-foundation.org
fti_newsletter_archive     → newsletters.coolplanet-foundation.org
cpf_members                → members.coolplanet-foundation.org
cpp_icp_platform           → (architecture/planning, may serve community features)
```

### **The Challenge**

**Without proper configuration:**
- Each subdomain = different origin
- Different origins = **different Internet Identity principals**
- User would have **separate identities** for each service
- Cannot share data, permissions, or profiles across services

**Example of the problem:**
```typescript
// ❌ WITHOUT derivation origin (broken experience)

// User logs in on newsletters.coolplanet-foundation.org
principal1 = "xxxxx-yyyyy-zzzzz-11111-cai"

// User logs in on members.coolplanet-foundation.org
principal2 = "aaaaa-bbbbb-ccccc-22222-cai"  // DIFFERENT principal!

// Now user has TWO identities - cannot link donations to comments, etc.
```

## The Solution: Shared Derivation Origin

### **Canonical Derivation Origin: cpf.nft**

All CPP services use **cpf.nft** (ENS domain) as the derivation origin:

```typescript
// ✅ WITH shared derivation origin (unified experience)

// All services use the same derivation origin
const DERIVATION_ORIGIN = "https://cpf.nft";

// User logs in on newsletters.coolplanet-foundation.org
await authClient.login({
  identityProvider: "https://identity.ic0.app",
  derivationOrigin: DERIVATION_ORIGIN  // ← Key configuration
});
principal = "xxxxx-yyyyy-zzzzz-11111-cai"

// User logs in on members.coolplanet-foundation.org
await authClient.login({
  identityProvider: "https://identity.ic0.app",
  derivationOrigin: DERIVATION_ORIGIN  // ← Same origin
});
principal = "xxxxx-yyyyy-zzzzz-11111-cai"  // ✅ SAME principal!
```

### **Why cpf.nft?**

1. **ICANN Independence:** ENS domain not subject to ICANN/government control
2. **Permanent Control:** Controlled by DAO multi-sig, cannot be seized
3. **Brand Alignment:** "Cool Planet Foundation" (.cpf)
4. **NFT Integration:** Natural fit for NFT-based platform
5. **Decentralized Governance:** Aligns with web3 principles

## Per-Repository Implementation

### **1. cpf_org (Public Marketing Site)**

**Domain:** `coolplanet-foundation.org`
**Framework:** SvelteKit
**Purpose:** Public entry point, newsletter signup, donation links

**Authentication Implementation:**
```typescript
// cpf_org/src/lib/auth.ts
import { AuthClient } from '@dfinity/auth-client';

const DERIVATION_ORIGIN = "https://cpf.nft";
const II_PROVIDER = "https://identity.ic0.app";

export async function login() {
  const authClient = await AuthClient.create();

  return new Promise((resolve, reject) => {
    authClient.login({
      identityProvider: II_PROVIDER,
      derivationOrigin: DERIVATION_ORIGIN, // ← Shared across ALL CPP services

      onSuccess: async () => {
        const principal = authClient.getIdentity().getPrincipal();
        resolve(principal);
      },

      onError: reject
    });
  });
}
```

**User Experience:**
- Anonymous users can browse without authentication
- "Sign In" button triggers Internet Identity
- After authentication, user's principal is the SAME as in other services
- Can link to donation page with preserved identity

---

### **2. fti_newsletter_archive (Newsletter System)**

**Domain:** `newsletters.coolplanet-foundation.org`
**Framework:** React + TypeScript
**Purpose:** Newsletter content, blog comments, CPF ID management

**Authentication Implementation:**
```typescript
// fti_newsletter_archive/src/services/auth.ts
import { AuthClient } from '@dfinity/auth-client';

const DERIVATION_ORIGIN = "https://cpf.nft";  // ← Same as cpf_org
const II_PROVIDER = "https://identity.ic0.app";

export async function authenticateUser() {
  const authClient = await AuthClient.create();

  await authClient.login({
    identityProvider: II_PROVIDER,
    derivationOrigin: DERIVATION_ORIGIN,  // ← Ensures same principal

    onSuccess: async () => {
      const identity = authClient.getIdentity();
      const principal = identity.getPrincipal();

      // Create or update user profile in backend
      await backend.createOrUpdateUserProfile({
        principal: principal.toString(),
        email: null,  // Gravatar email optional
        role: "viewer",
      });
    }
  });
}
```

**CPF ID (II-based) Creation:**
```motoko
// fti_newsletter_archive/src/backend_api/auth.mo
actor AuthBackend {
  stable var users: HashMap<Principal, UserProfile> = HashMap.init();

  public shared(msg) func createOrUpdateUserProfile(email: ?Text): async UserProfile {
    let principal = msg.caller;

    // Check if user exists
    switch (users.get(principal)) {
      case (?existingUser) {
        // Update existing user
        let updated = {
          existingUser with
          lastLogin = Time.now();
          email = Option.get(email, existingUser.email);
        };
        users.put(principal, updated);
        updated
      };
      case null {
        // Create new CPF ID
        let newUser = {
          principal = principal;
          email = email;
          gravatarConfigured = Option.isSome(email);
          role = #viewer;
          createdAt = Time.now();
          lastLogin = Time.now();
        };
        users.put(principal, newUser);
        newUser
      };
    };
  };
}
```

**User Experience:**
- User clicks "Sign In to Read Comments"
- Redirected to Internet Identity (same II instance as cpf_org)
- Returns with principal (CPF ID created automatically if first time)
- Principal matches their identity from cpf_org
- Can now read all authenticated content
- To write comments, must set up Gravatar (provides profile picture + public identity)

---

### **3. cpf_members (Member Portal & Donations)**

**Domain:** `members.coolplanet-foundation.org`
**Framework:** Svelte PWA + Rust payment bridge
**Purpose:** Donations, KYC, NFT portfolio, wallet management

**Authentication Implementation:**
```typescript
// cpf_members/src/cool_planet_pwa/lib/auth.ts
import { AuthClient } from '@dfinity/auth-client';

const DERIVATION_ORIGIN = "https://cpf.nft";  // ← Same as ALL other services
const II_PROVIDER = "https://identity.ic0.app";

export async function login() {
  const authClient = await AuthClient.create();

  await authClient.login({
    identityProvider: II_PROVIDER,
    derivationOrigin: DERIVATION_ORIGIN,  // ← Critical for identity consistency

    onSuccess: async () => {
      const principal = authClient.getIdentity().getPrincipal();

      // User principal is SAME as in cpf_org and fti_newsletter_archive
      // Can now link donations to their existing CPF ID

      await backend.initializeMemberProfile(principal);
    }
  });
}
```

**Cross-Canister Profile Integration:**
```motoko
// cpf_members/src/backend_canister/main.mo
import NewsletterBackend "canister:newsletter_backend";

actor MembersBackend {
  public shared(msg) func processDonation(amount: Nat): async Result<DonationId, Text> {
    let principal = msg.caller;

    // Get user's existing profile from newsletter system
    let userProfile = await NewsletterBackend.getUserProfile(principal);

    // Can access email, Gravatar, comment history, etc.
    // All tied to the SAME principal via shared derivation origin

    // Record donation linked to their existing identity
    await recordDonation(principal, amount, userProfile);

    #ok(donationId)
  };
}
```

**User Experience:**
- User lands on `members.coolplanet-foundation.org` from donation link
- Signs in with Internet Identity (same II instance)
- Their principal matches their identity from newsletter comments
- Donation is linked to their existing CPF ID
- Can see comment history, newsletter engagement, donation history - all unified

---

### **4. cpp_icp_platform (Community & Governance)**

**Domain:** TBD (may be `community.coolplanet-foundation.org` or integrated into main domains)
**Purpose:** Discussion forums, Camino educational content, governance voting

**Authentication Implementation:**
```motoko
// cpp_icp_platform/canisters/community/main.mo
actor Community {
  public shared(msg) func accessGatedForum(forumId: Text): async Result<ForumAccess, Text> {
    let principal = msg.caller;

    // Principal is SAME as in all other CPP services
    // Can check:
    // - Comments on newsletter (via fti_newsletter_archive)
    // - Donations made (via cpf_members)
    // - NFTs owned (via wallet cache)
    // - Camino completions (via local storage)

    let hasForumAccess = await checkForumRequirements(principal, forumId);

    if (not hasForumAccess) {
      return #err("Complete required Camino modules to access this forum");
    };

    #ok(grantForumAccess(principal, forumId))
  };
}
```

## Domain Routing Strategies

Based on [origins.md](./origins.md) analysis, there are **two approaches** to domain routing:

### **Approach 1: Path-Based Routing (Recommended)**

**All services under single domain with path-based routing:**

```
cpf.nft (Canonical Derivation Origin)
├── /                              → cpf_org (public site)
├── /newsletters/                  → fti_newsletter_archive
├── /members/                      → cpf_members
└── /community/                    → cpp_icp_platform

coolplanet-foundation.org (Mirror)
├── /                              → cpf_org (public site)
├── /newsletters/                  → fti_newsletter_archive
├── /members/                      → cpf_members
└── /community/                    → cpp_icp_platform
```

**IC Boundary Node Configuration:**
```bash
# Single domain, path-based routing to different canisters

# Public site canister
dfx canister update-settings cpf_org_canister \
  --add-controller <router-canister-principal>

# Newsletter canister
dfx canister update-settings newsletter_canister \
  --add-controller <router-canister-principal>

# Members canister
dfx canister update-settings members_canister \
  --add-controller <router-canister-principal>
```

**Advantages:**
- ✅ Same-origin (no CORS)
- ✅ Shared cookies and localStorage
- ✅ No need for derivation origin (same origin everywhere)
- ✅ Simplest authentication flow

**Challenges:**
- More complex boundary node routing configuration
- Requires router canister or HTTP gateway path mapping

---

### **Approach 2: Alternative Origins (Current Implementation)**

**Each service on separate subdomain with alternative origins configuration:**

```
coolplanet-foundation.org          → cpf_org
newsletters.coolplanet-foundation.org → fti_newsletter_archive
members.coolplanet-foundation.org     → cpf_members
```

**Alternative Origins File:**
```
# Served from https://cpf.nft/.well-known/ii-alternative-origins
https://coolplanet-foundation.org
https://newsletters.coolplanet-foundation.org
https://members.coolplanet-foundation.org
```

**IC Asset Canister Configuration:**
```bash
# cpf_nft canister (served at cpf.nft)
echo "cpf.nft" > public/.well-known/ic-domains

# Also serve alternative origins file
cat > public/.well-known/ii-alternative-origins <<EOF
https://coolplanet-foundation.org
https://newsletters.coolplanet-foundation.org
https://members.coolplanet-foundation.org
EOF
```

**Advantages:**
- ✅ Easier deployment (separate canisters, separate domains)
- ✅ Clear separation of concerns
- ✅ Can deploy/update services independently

**Challenges:**
- ❌ Requires CORS configuration
- ❌ Cookies/localStorage not shared
- ❌ Must configure alternative origins correctly

---

## Testing Cross-Repository Identity

### **End-to-End Identity Verification Test**

```typescript
// test/e2e/cross-repo-identity.spec.ts
import { test, expect } from '@playwright/test';

test('User has same principal across all CPP services', async ({ page, context }) => {
  let principal1: string;
  let principal2: string;
  let principal3: string;

  // Step 1: Authenticate on cpf_org
  await page.goto('https://coolplanet-foundation.org');
  await page.click('button:text("Sign In")');

  // Internet Identity flow
  await page.fill('input[type="number"]', '10000');
  await page.click('button:text("Confirm")');

  // Get principal from cpf_org
  principal1 = await page.evaluate(() => {
    return window.authClient.getIdentity().getPrincipal().toString();
  });

  // Step 2: Navigate to newsletter site (same browser context)
  await page.goto('https://newsletters.coolplanet-foundation.org');

  // Get principal from fti_newsletter_archive
  principal2 = await page.evaluate(() => {
    return window.authClient.getIdentity().getPrincipal().toString();
  });

  // Step 3: Navigate to members site
  await page.goto('https://members.coolplanet-foundation.org');

  // Get principal from cpf_members
  principal3 = await page.evaluate(() => {
    return window.authClient.getIdentity().getPrincipal().toString();
  });

  // Verify all principals are identical
  expect(principal1).toBe(principal2);
  expect(principal2).toBe(principal3);

  console.log(`✅ Unified principal across all services: ${principal1}`);
});
```

### **Manual Verification Steps**

```bash
# 1. Deploy all services with derivation origin configured
cd cpf_org && dfx deploy
cd ../fti_newsletter_archive && dfx deploy
cd ../cpf_members && dfx deploy

# 2. Verify .well-known files are served correctly
curl https://cpf.nft/.well-known/ic-domains
# Should return: cpf.nft

curl https://cpf.nft/.well-known/ii-alternative-origins
# Should return:
# https://coolplanet-foundation.org
# https://newsletters.coolplanet-foundation.org
# https://members.coolplanet-foundation.org

# 3. Test authentication flow
# Open browser to each domain, authenticate, verify same principal in console:
# > window.authClient.getIdentity().getPrincipal().toString()
```

## Troubleshooting Common Issues

### **Issue 1: Different principals on different domains**

**Symptom:** User has different principals when logging in on different CPP services

**Root Cause:** Derivation origin not configured or misconfigured

**Solution:**
```typescript
// ❌ WRONG - Missing derivation origin
await authClient.login({
  identityProvider: "https://identity.ic0.app"
  // No derivationOrigin specified - each domain gets different principal
});

// ✅ CORRECT - Shared derivation origin
await authClient.login({
  identityProvider: "https://identity.ic0.app",
  derivationOrigin: "https://cpf.nft"  // ← Must match across all services
});
```

---

### **Issue 2: Alternative origins not recognized**

**Symptom:** Alternative origins file exists but Internet Identity doesn't recognize alternate domains

**Root Cause:** File not served from canonical derivation origin or incorrect format

**Solution:**
```bash
# Ensure file is served from CANONICAL derivation origin (cpf.nft)
# NOT from the alternative origins themselves

# File location: cpf_nft canister (served at cpf.nft)
# NOT: cpf_org canister, newsletter canister, members canister

# Correct:
https://cpf.nft/.well-known/ii-alternative-origins  ✅

# Incorrect:
https://coolplanet-foundation.org/.well-known/ii-alternative-origins  ❌
```

---

### **Issue 3: CORS errors between services**

**Symptom:** Cross-origin requests fail with CORS errors

**Root Cause:** Different subdomains = different origins, CORS not configured

**Solution Option A:** Use path-based routing (same origin)
**Solution Option B:** Configure CORS headers in backend canisters

```motoko
// Backend canister CORS configuration
public query func http_request(request: HttpRequest): async HttpResponse {
  {
    status_code = 200;
    headers = [
      ("Access-Control-Allow-Origin", "https://newsletters.coolplanet-foundation.org"),
      ("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS"),
      ("Access-Control-Allow-Headers", "Content-Type, Authorization"),
      ("Access-Control-Allow-Credentials", "true")
    ];
    body = responseBody;
  }
};
```

## Security Considerations

### **Derivation Origin Governance**

**Threat Model:**
- Loss of cpf.nft ENS domain = loss of all user identities
- Unauthorized transfer = platform identity crisis
- DNS hijacking = phishing attacks

**Mitigation:**
- **Multi-Sig Control:** Gnosis Safe 3-of-5 for cpf.nft management
- **ENS Independence:** Not subject to ICANN/government seizure
- **Backup Strategy:** Alternative origins provide fallback domains
- **Monitoring:** DNS record monitoring for unauthorized changes

### **Alternative Origins Security**

**Threat:** Attacker adds malicious domain to alternative origins list

**Mitigation:**
- Alternative origins file served from canister controlled by multi-sig
- Any changes require canister upgrade (multi-sig approval)
- Monitor for unauthorized canister upgrades
- Regular audits of alternative origins list

**Implementation:**
```motoko
// Only allow canister controller to update alternative origins
actor AlternativeOriginsManager {
  stable var authorizedOrigins: [Text] = [
    "https://coolplanet-foundation.org",
    "https://newsletters.coolplanet-foundation.org",
    "https://members.coolplanet-foundation.org"
  ];

  // Only callable by canister controller
  public shared(msg) func updateAlternativeOrigins(newOrigins: [Text]): async () {
    assert(msg.caller == Principal.fromText("<canister-controller>"));
    authorizedOrigins := newOrigins;
  };
}
```

## Migration Path

### **Current State → Target State**

**Current State:**
- fti_newsletter_archive: Deployed with derivation origin configured
- cpf_members: In development, derivation origin planned
- cpf_org: In development, needs derivation origin configuration

**Target State (Phase 1):**
- All services use `cpf.nft` as derivation origin
- Alternative origins configured for all current subdomains
- CORS configured where needed

**Target State (Phase 2):**
- Path-based routing under single domain
- Eliminate need for alternative origins
- Same-origin benefits across all services

### **Migration Steps**

```bash
# Phase 1: Configure derivation origin for all services (Weeks 1-2)

# 1. Deploy cpf_nft canister (serves cpf.nft domain)
cd cpf_org  # (to be renamed cpf_nft)
# Configure derivation origin in auth.ts
dfx deploy --network ic

# 2. Update fti_newsletter_archive (already has derivation origin)
cd ../fti_newsletter_archive
# Verify derivation origin is "https://cpf.nft"
dfx deploy --network ic

# 3. Configure cpf_members with same derivation origin
cd ../cpf_members
# Add derivation origin to auth.ts
dfx deploy --network ic

# 4. Test cross-repo identity
npm run test:e2e -- cross-repo-identity.spec.ts

# Phase 2: Set up alternative origins (Weeks 3-4)

# 1. Create alternative origins file in cpf_nft canister
cat > public/.well-known/ii-alternative-origins <<EOF
https://coolplanet-foundation.org
https://newsletters.coolplanet-foundation.org
https://members.coolplanet-foundation.org
EOF

# 2. Deploy updated cpf_nft canister
dfx deploy cpf_nft --network ic

# 3. Verify alternative origins are served
curl https://cpf.nft/.well-known/ii-alternative-origins

# Phase 3: Path-based routing (Future)

# 1. Deploy router canister or configure HTTP gateway
# 2. Test path-based routing locally
# 3. Migrate to production
# 4. Remove alternative origins (no longer needed)
```

## Conclusion

**Shared derivation origin (`cpf.nft`) is critical for unified CPP identity:**

✅ **Same principal** across all services
✅ **Unified user experience** (single login)
✅ **ICANN independence** (ENS-based)
✅ **Decentralized governance** (multi-sig control)
✅ **Cross-canister integration** (shared identity enables data sharing)

**Implementation Requirements:**
1. All repositories must use `https://cpf.nft` as derivation origin
2. Alternative origins file must be served from cpf.nft domain
3. CORS configured for cross-subdomain calls (until path-based routing)
4. Multi-sig governance for cpf.nft ENS domain

**Related Documentation:**
- [origins.md](./origins.md) - Technical deep-dive on derivation origins
- [user-journey-funnel.md](./user-journey-funnel.md) - User flows across repositories
- [system-architecture-overview.md](./system-architecture-overview.md) - Overall system architecture

---

**Last Updated:** 2025-11-13
**Version:** 1.0.0
**Status:** Active Implementation
**Review Cycle:** Update with each repository deployment
