# CPP User Journey Funnel: Progressive Authentication & Access Gates

**Date:** 2025-11-13
**Purpose:** Define the progressive user journey through CPP platform with authentication gates
**Context:** Maps user progression from anonymous visitor to full community member
**Cross-Reference:** See [origins.md](./origins.md) for derivation origin strategy

## Executive Summary

The CPP platform uses a **progressive disclosure and authentication funnel** where users gradually gain access to deeper platform features as they pass through authentication and verification gates. Each gate requires specific credentials or actions, building trust and engagement progressively.

**Key Principle:** *Minimum friction for discovery, progressive authentication for deeper engagement*

**Important Architecture Context:**
- **This funnel describes the DATA PLANE** (cpf.nft) - user-facing operations
- **Separate CONTROL PLANE** exists (authors.cpf.nft) - governance and infrastructure management
- Board members and operations staff have TWO identities:
  - `authors.cpf.nft` → Control plane (governance, upgrades, financial decisions)
  - `cpf.nft` → Data plane (content creation, moderation - same funnel as users)
- See [canister-architecture-diagram.md](./canister-architecture-diagram.md) for control/data plane architecture

## Funnel Overview

```
Anonymous Visitor
    ↓ [Analytics Consent]
Newsletter Reader
    ↓ [Email Address]
Blog Commenter
    ↓ [CPF ID (II based) + Gravatar Profile]
Donor
    ↓ [KYC for >$1000, Enhanced KYC for >$15000]
NFT Holder
    ↓ [Wallet Optional]
Community Member
    ↓ [Camino Module Completion]
Full Platform Access
```

## Detailed Gate Progression

### **Gate 1: Websites → Newsletter(s)**

**Repository:** `cpf_org` (coolplanet-foundation.org)
**Requirement:** Analytics consent
**Purpose:** Public discovery and information gathering

**User Flow:**
1. User visits `coolplanet-foundation.org` (served by cpf_org asset canister)
2. Analytics consent banner appears (GDPR compliance)
3. User can browse public content without account
4. Call-to-action: "Subscribe to Newsletter" button

**Technical Implementation:**
```svelte
<!-- cpf_org/src/routes/+page.svelte -->
<script>
  import { analyticsConsent } from '$lib/stores/consent';
  import NewsletterSignup from '$lib/components/NewsletterSignup.svelte';
</script>

<!-- Analytics consent banner -->
{#if !$analyticsConsent}
  <ConsentBanner />
{/if}

<!-- Public content accessible without authentication -->
<Hero />
<AboutSection />
<NewsletterSignup />
```

**Data Collected:**
- Analytics: Page views, time on site, referrers (with consent)
- No personal information required
- No authentication needed

**Network Privacy Integration:**
- Anonymous visitors may be in `T_XXXX` (Thought Leaders) or `C_XXXX` (Collaboration Partners) classifications in network_privacy repo
- Public LinkedIn/social media monitoring for potential collaboration (legitimate interest, no consent needed)

---

### **Gate 2: Newsletter(s) → Newsletter/Blog Comments (CPF)**

**Repository:** `fti_newsletter_archive` (newsletters.coolplanet-foundation.org)
**Requirement:** Email address
**Purpose:** Newsletter delivery and basic engagement

**User Flow:**
1. User submits email on `coolplanet-foundation.org`
2. Redirected to `newsletters.coolplanet-foundation.org` for subscription confirmation
3. Email added to MailerLite (via network_privacy integration)
4. User receives newsletters but cannot comment yet

**Technical Implementation:**
```typescript
// fti_newsletter_archive/src/components/NewsletterSubscription.tsx
export async function subscribeToNewsletter(email: string) {
  // Add to MailerLite via network_privacy integration
  await fetch('/api/subscribe', {
    method: 'POST',
    body: JSON.stringify({ email }),
  });

  // No CPF ID created yet - just email subscription
}
```

**Data Collected:**
- Email address (required)
- Newsletter preferences (which topics)
- Consent timestamp (GDPR compliance)

**Network Privacy Integration:**
- User transitions from anonymous to `R_XXXX` (Research Contributors) or remains unclassified
- Email stored in MailerLite with appropriate segment (FTI_Newsletter_Subscribers)
- Privacy: Explicit consent for email communications, no KYC required

---

### **Gate 3: Newsletter/Blog Comments (CPF) → CPF Donations**

**Repository:** `fti_newsletter_archive` (newsletters.coolplanet-foundation.org)
**Requirements:**
- **CPF ID (II based)** - Internet Identity principal required to read
- **Gravatar Profile** - Required to write comments

**Purpose:** Enable authenticated commenting and community discussion

**User Flow:**
1. User clicks "Sign In to Comment" on newsletter
2. Redirected to Internet Identity for authentication
3. Returns with principal (CPF ID created automatically)
4. User prompted to set up Gravatar profile to post comments
5. Can now read all content (authenticated) and write comments (Gravatar configured)

**Technical Implementation:**
```typescript
// fti_newsletter_archive/src/components/Comments.tsx
import { AuthClient } from '@dfinity/auth-client';

export function CommentSection() {
  const { isAuthenticated, principal } = useAuth();
  const { hasGravatar, gravatarEmail } = useGravatar();

  if (!isAuthenticated) {
    return <SignInPrompt message="Sign in with Internet Identity to read comments" />;
  }

  if (!hasGravatar) {
    return <GravatarSetup message="Set up Gravatar to post comments" />;
  }

  return <CommentThread canRead={isAuthenticated} canWrite={hasGravatar} />;
}
```

**Internet Identity Integration:**
```typescript
// Derivation origin: cpf.nft (canonical)
const authClient = await AuthClient.create();
await authClient.login({
  identityProvider: "https://identity.ic0.app",
  derivationOrigin: "https://cpf.nft", // Ensures consistent principal across all CPP services
  onSuccess: async () => {
    const principal = authClient.getIdentity().getPrincipal();
    // Create CPF ID record in backend
    await backend.createOrUpdateUserProfile(principal);
  }
});
```

**Data Collected:**
- Internet Identity principal (CPF ID)
- Gravatar email (for profile picture and identity)
- Comment content and timestamps
- User permissions (RBAC: Viewer, Author, Editor, Admin, Superadmin)

**Network Privacy Integration:**
- User may upgrade to `C_XXXX` (Collaboration Partners) if actively commenting
- Professional consent for networking (if appropriate)
- No KYC required at this stage

**Side Path: Camino Beta Invitation**
- **Trigger:** Active commenters with quality contributions
- **Action:** Invite subset of commenters to Camino Beta (educational content platform)
- **Requirement:** Still using CPF ID + Gravatar, no additional authentication
- **Purpose:** Test educational content and engagement before full launch

---

### **Gate 4: CPF Donations → NFT**

**Repository:** `cpf_members` (members.coolplanet-foundation.org)
**Requirements:**
- **KYC required for donations > $1,000**
- **Enhanced KYC/KYB for donations > $15,000**

**Purpose:** Financial compliance and donation processing

**User Flow:**
1. User clicks "Donate" on `coolplanet-foundation.org`
2. Redirected to `members.coolplanet-foundation.org`
3. Authenticated with same CPF ID (derivation origin: cpf.nft)
4. Enters donation amount:
   - **< $1,000:** Basic info (name, address) via Stripe
   - **> $1,000:** Einstein DID KYC verification required
   - **> $15,000:** Enhanced KYC/KYB (business verification if applicable)
5. Payment processed via Stripe webhook to payment_bridge canister
6. Donation recorded in Motoko backend
7. NFT minting triggered on Polygon (if applicable)

**Technical Implementation:**
```typescript
// cpf_members/src/backend_canister/main.mo
actor MembersBackend {
  // Process donation and determine KYC requirements
  public shared(msg) func processDonation(amount: Nat): async Result<DonationId, Text> {
    let principal = msg.caller;

    // KYC requirement check
    if (amount > 1000) {
      // Verify KYC via Einstein DID integration
      let kycVerified = await verifyKYC(principal);
      if (not kycVerified) {
        return #err("KYC verification required for donations > $1,000");
      };
    };

    if (amount > 15000) {
      // Enhanced KYC/KYB required
      let enhancedKYC = await verifyEnhancedKYC(principal);
      if (not enhancedKYC) {
        return #err("Enhanced KYC/KYB required for donations > $15,000");
      };
    };

    // Process donation
    let donationId = await recordDonation(principal, amount);

    // Trigger NFT minting if applicable
    await mintNFT(principal, donationId, amount);

    #ok(donationId)
  };
}
```

**Stripe Webhook Processing:**
```rust
// cpf_members/src/payment_bridge/lib.rs
#[update]
fn handle_stripe_webhook(payload: String, signature: String) -> Result<(), String> {
    // HMAC-SHA256 signature verification
    verify_stripe_signature(&payload, &signature)?;

    // Parse webhook event
    let event: StripeEvent = serde_json::from_str(&payload)?;

    match event.event_type {
        "payment_intent.succeeded" => {
            // Route to backend canister for business logic
            ic_cdk::call(backend_canister_id(), "process_payment_success", (event,)).await?;
        },
        _ => return Err("Unhandled event type".to_string()),
    }

    Ok(())
}
```

**Data Collected:**
- **< $1,000:** Name, email, address (Stripe)
- **> $1,000:** Full KYC (Einstein DID): Identity verification, document upload
- **> $15,000:** Enhanced KYC/KYB: Business verification, beneficial ownership
- Payment details (encrypted in Cloudflare storage per network_privacy)
- Donation amount and NFT allocation

**Network Privacy Integration:**
- **< $1,000:** May remain `C_XXXX` (Collaboration Partners) or upgrade to `I_XXXX` (Investment Network)
- **> $1,000:** Must upgrade to `I_XXXX` (Investment Network) - KYC required
- **> $15,000:** Upgrade to `P_XXXX` (Cool Planet People) - Enhanced KYC required
- Cloudflare storage for KYC compliance data (7-10 year retention)
- MailerLite segment updated to reflect donor status

**Side Path: Private Discussion Forum Invitation**
- **Trigger:** Donation of any amount + active community participation
- **Action:** Invite subset of donors to private discussion forum
- **Requirement:** Same CPF ID + Gravatar, no additional authentication
- **Purpose:** Deeper engagement with committed community members

---

### **Gate 5: NFT → Gated Discussion Forum(s)**

**Repository:** `cpf_members` (members.coolplanet-foundation.org) + `cpp_icp_platform`
**Requirement:** Wallet optional (can use derived IC-Polygon address)
**Purpose:** NFT ownership verification and portfolio management

**User Flow:**
1. User receives NFT after donation (minted on Polygon)
2. NFT visible in `members.coolplanet-foundation.org/portfolio`
3. Wallet options:
   - **Option A:** Use IC-derived Polygon address (automatic, no wallet needed)
   - **Option B:** Connect external wallet (WalletConnect) for advanced features
4. NFT ownership tracked in Wallet Cache canister
5. Can view portfolio, sponsorship patterns, and NFT metadata

**Technical Implementation:**
```typescript
// cpf_members/src/backend_canister/wallet.mo
actor WalletBackend {
  // Derive Polygon address from IC principal
  public shared(msg) func getDerivedPolygonAddress(): async Text {
    let principal = msg.caller;
    let polygonAddress = derivePolygonAddress(principal); // Chain-key cryptography
    polygonAddress
  };

  // Get user's NFT holdings
  public shared(msg) func getNFTHoldings(): async [NFT] {
    let principal = msg.caller;
    let polygonAddress = await getDerivedPolygonAddress();

    // Query Wallet Cache for holdings
    let holdings = await WalletCache.getHoldings(polygonAddress);
    holdings
  };
}
```

**Wallet Cache Integration:**
```motoko
// cpp_icp_platform/canisters/wallet_cache/main.mo
actor WalletCache {
  // Chain Fusion bridge updates wallet cache when NFTs are minted
  public func updateHoldings(address: Text, tokenId: Nat): async () {
    // Listen to Polygon NFTMinted events
    // Update holdings in real-time
  };
}
```

**Data Collected:**
- NFT token IDs and metadata
- Polygon address (derived or external)
- Sponsorship patterns (if bundle holder)
- Portfolio analytics

**Network Privacy Integration:**
- User remains `I_XXXX` or `P_XXXX` depending on donation amount
- NFT ownership recorded in Cloudflare storage (audit trail)
- MailerLite segment: CPF_NFT_Holders

**Side Path: Full Camino Launch**
- **Trigger:** NFT ownership
- **Action:** Access to full Camino educational content platform
- **Benefit:** Can sponsor special NFTs to mentees (bundle holder feature)
- **Requirement:** NFT ownership verified, no additional authentication

---

### **Gate 6: Gated Discussion Forum(s) → Full Platform Access**

**Repository:** `cpp_icp_platform` (community canisters)
**Requirement:** Camino module completion
**Purpose:** Full community engagement and governance participation

**User Flow:**
1. User completes Camino educational modules
2. Achievement NFTs unlocked (on-chain credentials)
3. Gains access to gated discussion forums (subset by bundle or achievement)
4. Can participate in governance (voting, proposals)
5. Full member benefits (conference access, advanced features, etc.)

**Technical Implementation:**
```motoko
// cpp_icp_platform/canisters/community/main.mo
actor Community {
  // Check Camino module completion
  public shared(msg) func checkCaminoAccess(forumId: Text): async Bool {
    let principal = msg.caller;

    // Check required modules for this forum
    let requiredModules = getForumRequirements(forumId);
    let completedModules = await CaminoBackend.getUserCompletions(principal);

    // Verify all required modules completed
    hasCompletedAll(completedModules, requiredModules)
  };

  // Grant forum access
  public shared(msg) func accessForum(forumId: Text): async Result<ForumAccess, Text> {
    let hasAccess = await checkCaminoAccess(forumId);

    if (not hasAccess) {
      return #err("Complete required Camino modules to access this forum");
    };

    #ok(grantForumAccess(msg.caller, forumId))
  };
}
```

**Achievement System:**
```motoko
// Track on-chain achievements
public func awardAchievement(principal: Principal, achievement: Achievement): async () {
  // Mint achievement NFT on Polygon
  // Update user's achievement record
  // Unlock new forum/feature access
};
```

**Data Collected:**
- Camino module progress and completions
- Achievement NFT ownership
- Forum participation and contributions
- Governance voting history

**Network Privacy Integration:**
- User fully transitioned to `P_XXXX` (Cool Planet People)
- Full profile with comprehensive engagement history
- MailerLite segment: CPF_Full_Members
- Enhanced features and VIP access

## Cross-Repository Authentication Flow

### **Shared Derivation Origin: cpf.nft**

All repositories use the **same derivation origin** (`cpf.nft`) to ensure users have the **same Internet Identity principal** across all CPP services:

```typescript
// Common auth configuration across all repos

// cpf_org (public site)
await authClient.login({
  identityProvider: "https://identity.ic0.app",
  derivationOrigin: "https://cpf.nft"
});

// fti_newsletter_archive (newsletters)
await authClient.login({
  identityProvider: "https://identity.ic0.app",
  derivationOrigin: "https://cpf.nft" // Same principal as cpf_org
});

// cpf_members (member portal)
await authClient.login({
  identityProvider: "https://identity.ic0.app",
  derivationOrigin: "https://cpf.nft" // Same principal everywhere
});
```

**Why This Matters:**
- User authenticates **once** with Internet Identity
- Same principal used across **all CPP services**
- Seamless transitions between repositories
- No separate logins for different domains
- Shared user profile and permissions

### **Domain Routing Strategy**

Based on [architectural_decisions.md § OQ-001](./architectural_decisions.md) and [origins.md](./origins.md), CPP uses **subdomain-based routing with alternative origins**:

**Primary Domains:**
```
# Data Plane (users, authors in content role)
cpf.nft                                    → cpf_org (public site) [ENS domain]
newsletters.coolplanet-foundation.org      → fti_newsletter_archive
members.coolplanet-foundation.org          → cpf_members

# Control Plane (board, operations staff)
authors.cpf.nft                            → governance_canister [ENS subdomain]
```

**Alternative Origins Configuration:**

All data plane domains serve the same canisters and share Internet Identity principals via alternative origins:

```json
// Served from cpf.nft/.well-known/ii-alternative-origins
{
  "alternativeOrigins": [
    "https://cpf.nft",
    "https://newsletters.coolplanet-foundation.org",
    "https://members.coolplanet-foundation.org",
    "https://coolplanet-foundation.org"
  ]
}
```

**Key Benefits:**
- Same Internet Identity principal across all data plane domains
- Subdomain isolation for security (different canisters)
- ENS domain (cpf.nft) as canonical derivation origin
- Traditional .org domains for user familiarity

**See Also:**
- [canister-architecture-diagram.md § Production Migration](./canister-architecture-diagram.md) for Wix → ICP migration strategy
- [ens-dns-setup.md](./ens-dns-setup.md) for ENS/DNS configuration details

## Network Privacy Classification Progression

As users progress through the funnel, their **network_privacy classification** evolves:

| Funnel Stage | network_privacy Class | KYC Required | MailerLite Segment | Data Sensitivity |
|--------------|----------------------|--------------|-------------------|------------------|
| Website visitor | `T_XXXX` (Thought Leader) or unclassified | No | None | Minimal (public info) |
| Newsletter subscriber | `R_XXXX` (Research) or `C_XXXX` (Collab) | No | FTI_Newsletter_Subscribers | Low (email only) |
| Blog commenter | `C_XXXX` (Collaboration Partners) | No | FTI_Active_Community | Low-Medium (CPF ID + Gravatar) |
| Donor < $1K | `C_XXXX` or `I_XXXX` (Investment) | No | FTI_Donors_Basic | Medium (payment info) |
| Donor > $1K | `I_XXXX` (Investment Network) | **Yes** (Einstein DID) | FTI_Donors_KYC | High (KYC data) |
| Donor > $15K | `P_XXXX` (Cool Planet People) | **Yes** (Enhanced KYC/KYB) | FTI_Major_Donors | Very High (full KYC/KYB) |
| Full member | `P_XXXX` (Cool Planet People) | Yes (already completed) | FTI_Full_Members | Very High (comprehensive) |

## Privacy & Compliance Considerations

### **GDPR Compliance by Stage**

**Analytics Consent (Gate 1):**
- Legal Basis: Consent
- Retention: Until consent withdrawal
- Right to Deletion: Immediate (no personal data)

**Email Subscription (Gate 2):**
- Legal Basis: Explicit consent for marketing
- Retention: Until unsubscribe + audit period
- Right to Deletion: MailerLite deletion + network_privacy cleanup

**CPF ID & Comments (Gate 3):**
- Legal Basis: Contractual (service usage)
- Retention: Account lifecycle + required audit period
- Right to Deletion: Key rotation (makes data unrecoverable per [security.md](./security.md))

**Donations & KYC (Gate 4):**
- Legal Basis: Regulatory requirement (AML/KYC laws)
- Retention: 7-10 years (financial regulation compliance)
- Right to Deletion: Cannot delete (legal requirement), but encrypt and restrict access

**NFT & Community (Gates 5-6):**
- Legal Basis: Contractual (NFT ownership) + Explicit consent (community features)
- Retention: Blockchain immutable, community data deletable
- Right to Deletion: Community data via key rotation, blockchain data immutable (disclosed to user)

### **Cross-Border Data Transfers**

Per network_privacy architecture:
- **Cloudflare Storage:** Jurisdiction-aware data residency (EU, US, UK, CA, SG)
- **MailerLite:** Lithuania-based (EU GDPR compliant)
- **IC Canisters:** Swiss jurisdiction for data sovereignty
- **Polygon Blockchain:** Public blockchain (appropriate disclosures required)

## Implementation Checklist

**Note:** This checklist focuses on DATA PLANE features. For CONTROL PLANE deployment (governance, voting, ENS management), see [canister-architecture-diagram.md § Bootstrap Sequence](./canister-architecture-diagram.md).

**Production Migration Strategy:** See [canister-architecture-diagram.md § Production Migration](./canister-architecture-diagram.md) for phased rollout plan including:
- Phase A: Subdomain Launch (newsletters.*, members.* on ICP)
- Phase B: Stripe Production Account Migration
- Phase C: Smart Contract Integration (CPF DID + 4T NFT)
- Phase D: Main Domain Cutover (Wix → ICP) with three milestone options

### **Phase 1: Foundation** (Weeks 1-8)
- [ ] Deploy cpf_org to `cpf.nft` (ENS domain)
- [ ] Configure Internet Identity with derivation origin `cpf.nft`
- [ ] Set up alternative origins for coolplanet-foundation.org
- [ ] Integrate fti_newsletter_archive for CPF ID + Gravatar
- [ ] Set up MailerLite integration via network_privacy
- [ ] Implement analytics consent (GDPR)
- [ ] Launch test.cpf.nft for beta testing

### **Phase 2: Donations & KYC** (Weeks 9-16)
- [ ] Deploy cpf_members to `members.coolplanet-foundation.org`
- [ ] Integrate Stripe webhooks via payment_bridge (Rust canister)
- [ ] Implement Einstein DID KYC for >$1K donations
- [ ] Configure Enhanced KYC/KYB for >$15K
- [ ] Set up Cloudflare storage for KYC compliance data

### **Phase 3: NFTs & Portfolio** (Weeks 17-24)
- [ ] Integrate Polygon NFT minting via Chain Fusion bridge
- [ ] Implement Wallet Cache for NFT holdings
- [ ] Build portfolio dashboard in cpf_members
- [ ] Enable IC-derived Polygon addresses (chain-key)
- [ ] Support external wallet connections (WalletConnect)

### **Phase 4: Community & Governance** (Weeks 25-32)
- [ ] Deploy Camino educational content platform
- [ ] Implement achievement NFT system
- [ ] Build gated discussion forums
- [ ] Integrate governance voting
- [ ] Launch full member benefits

## Related Documentation

**Architecture Overview:**
- [canister-architecture-diagram.md](./canister-architecture-diagram.md) - **CRITICAL** - Control/data plane architecture, production migration strategy
- [architectural_decisions.md](./architectural_decisions.md) - Decision log including domain routing strategy (OQ-001)
- [system-architecture-overview.md](./system-architecture-overview.md) - Overall system architecture

**Identity & Domains:**
- [origins.md](./origins.md) - Detailed derivation origin analysis and domain strategy
- [ens-dns-setup.md](./ens-dns-setup.md) - ENS/DNS configuration and bootstrap sequence

**Governance & Operations:**
- [governance-policy.md](./governance-policy.md) - Board governance structure and approval tiers
- [admin-architecture.md](./admin-architecture.md) - Control plane security and bootstrap patterns
- [crypto_funding.md](./crypto_funding.md) - Thermostat algorithm for crypto treasury management

**Security & Privacy:**
- [security.md](./security.md) - Security architecture and data protection
- [network_privacy/README.md](/Users/john/git/network_privacy/README.md) - Privacy-first contact management

---

**Last Updated:** 2025-11-14
**Version:** 1.1.0 (Updated for control/data plane alignment)
**Status:** Active Development
**Review Cycle:** Update with each new gate implementation
