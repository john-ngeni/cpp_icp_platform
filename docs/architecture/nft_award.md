# NFT Award Workflow & Accounting Data Architecture

**Date:** 2025-11-13
**Purpose:** Document NFT minting workflow, wallet architecture, and accounting data reconciliation
**Context:** How donations flow from Stripe → IC → Ethereum wallet → Polygon NFT
**Cross-Reference:** See [admin-architecture.md](./admin-architecture.md) for admin bootstrap

## Executive Summary

The CPP platform awards NFTs to donors based on donation amounts. The workflow spans **three systems**:

1. **Stripe**: Payment processing (fiat → Payment Intent)
2. **IC Canister**: Business logic, user profiles, wallet management
3. **Polygon**: NFT minting and ownership

**Key Architectural Decision:** Users can choose between:
- **External Wallet**: User-provided Ethereum wallet (MetaMask, Ledger, etc.)
- **Chain-Key Wallet**: IC-derived wallet using threshold ECDSA (no user wallet required)

---

## Three-System Architecture

### **1. Stripe (External SaaS - Payment Processing)**

**Location:** Stripe Dashboard (stripe.com)

**Data Stored:**
- Payment Intents (authorized, captured, failed)
- Customer records (linked to IC principals via metadata)
- Refunds and disputes
- Settlement reports (bank transfers)
- Webhook event logs

**Access Control:**
- Stripe account owners (multi-factor auth required)
- Restricted API keys for payment_bridge canister (HMAC-SHA256 webhook verification)

**Governance:**
- Stripe account owned by Cool Planet Foundation
- Account credentials stored in secure password manager (1Password/Bitwarden)
- Multi-sig required for account ownership changes

**Links to IC:**
- Payment Intent ID stored in metadata
- Webhook sends Payment Intent ID to IC canister

---

### **2. IC Canisters (On-Chain - Business Logic & Coordination)**

**Primary Canister:** `cpf_members/backend_canister` (Motoko)

**Data Stored:**
- Donation records (amount, timestamp, IC principal, Stripe Payment Intent ID)
- User profiles (IC Principal, Ethereum wallet, wallet type)
- KYB/EDD review status (pending, approved, rejected)
- Admin approval/rejection notes
- NFT minting status (token ID, minting timestamp)
- Cross-reference to Stripe Payment Intent IDs

**Data Types:**

```motoko
// cpf_members/src/backend_canister/types/Donation.mo
type Donation = {
  id: Text;                          // Internal donation ID
  principal: Principal;              // Donor's IC principal (Internet Identity)
  amount: Nat;                       // Amount in cents
  stripePaymentIntentId: Text;       // Links to Stripe
  ethereumWallet: ?Text;             // Ethereum wallet for NFT minting
  walletSource: WalletSource;        // External or ChainKey
  nftTokenId: ?Nat;                  // NFT token ID after minting
  status: DonationStatus;            // Pending, Approved, Rejected, Completed
  eddRequired: Bool;                 // True if >$15K
  reviewedBy: ?Principal;            // Admin who reviewed
  reviewNotes: ?Text;                // Admin notes
  createdAt: Nat64;                  // Timestamp
  nftMinted: Bool;                   // NFT minting status
};

type WalletSource = {
  #External;                         // User-provided wallet
  #ChainKey;                         // IC-derived wallet via threshold ECDSA
};

type DonationStatus = {
  #Pending;                          // Awaiting processing
  #PendingEDD;                       // >$15K, awaiting admin review
  #Approved;                         // Admin approved (if EDD required)
  #Rejected;                         // Admin rejected (if EDD required)
  #Completed;                        // Payment captured, NFT minted
  #Failed;                           // Payment failed
};

// User profile stores Ethereum wallet linkage
type UserProfile = {
  principal: Principal;              // Internet Identity
  ethereumWallet: ?Text;             // Ethereum address (external or chain-key)
  walletType: WalletType;            // How wallet was obtained
  gravatarEmail: ?Text;              // For avatar
  // ... other profile fields
};

type WalletType = {
  #External;                         // User provided their own wallet
  #ChainKeyDerived;                  // System derived wallet from II
  #NotSet;                           // User hasn't set up wallet yet
};
```

**Access Control:**
- User-facing endpoints: Authenticated users can view their own donations
- Admin endpoints: Only principals in admin list can approve/reject (see admin-architecture.md)

**Backup/Recovery:**
- Canister stable memory persists across upgrades
- Regular canister state exports (future: backup to IC storage canister)

---

### **3. Einstein Solidity Contract (On-Chain Polygon - NFT Records)**

**Location:** Polygon mainnet smart contract

**Data Stored:**
- NFT ownership records (Ethereum wallet address → NFT token ID)
- Donation amount encoded in NFT metadata
- Minting timestamp

**Cross-Reference:**
- IC canister stores `nftTokenId` in donation record
- User's Internet Identity links to their Ethereum wallet
- NFT minted to Ethereum wallet (NOT to IC principal)

**Governance:**
- Contract owner: Multi-sig wallet (3-of-5 Gnosis Safe)
- Minting authority: payment_bridge canister (via Chain Fusion)

---

## Wallet Architecture: Dual-Path Approach

### **Option A: User-Provided Wallet (External)**

**User Flow:**
```
1. User authenticates with Internet Identity
    ↓
2. User navigates to profile settings
    ↓
3. User enters Ethereum wallet address (e.g., from MetaMask)
    ↓
4. System validates address format (0x...)
    ↓
5. Store in profile: ethereumWallet = "0x...", walletType = #External
    ↓
6. When donation occurs, NFT mints to this external wallet
```

**Advantages:**
- ✅ User retains full control of private key
- ✅ User can use existing wallet with other dApps
- ✅ No IC dependency for wallet access
- ✅ User can recover wallet independently

**Disadvantages:**
- ⚠️ User must already have Ethereum wallet
- ⚠️ User must manage private key security
- ⚠️ User can lose access if private key lost

**Implementation:**
```motoko
// cpf_members/src/backend_canister/profile.mo
public shared(msg) func setExternalWallet(
  walletAddress: Text
): async Result<(), Text> {
  // Validate Ethereum address format
  if (not isValidEthereumAddress(walletAddress)) {
    return #err("Invalid Ethereum address format");
  };

  // Get user profile
  let principal = msg.caller;
  let ?profile = userProfiles.get(Principal.toText(principal)) else {
    return #err("Profile not found");
  };

  // Update profile
  let updatedProfile = {
    profile with
    ethereumWallet = ?walletAddress;
    walletType = #External;
  };

  userProfiles.put(Principal.toText(principal), updatedProfile);
  #ok(())
};
```

---

### **Option B: Chain-Key Derived Wallet (IC-Managed)**

**User Flow:**
```
1. User authenticates with Internet Identity
    ↓
2. User navigates to profile settings
    ↓
3. User clicks "Create IC Wallet"
    ↓
4. System derives Ethereum address from II Principal
    ↓ (Using IC threshold ECDSA)
5. Store in profile: ethereumWallet = derived address, walletType = #ChainKeyDerived
    ↓
6. When donation occurs, NFT mints to this chain-key wallet
```

**Advantages:**
- ✅ No existing Ethereum wallet required
- ✅ Deterministic: Same II → Same ETH address
- ✅ Recoverable via II anchor recovery
- ✅ User-friendly (no private key management)

**Disadvantages:**
- ⚠️ IC subnet controls key shares (trust in IC network)
- ⚠️ Requires IC threshold ECDSA subnet
- ⚠️ User must access via IC (can't import to MetaMask directly)

**Implementation:**
```motoko
// cpf_members/src/backend_canister/wallet.mo
import Management "canister:aaaaa-aa";

public shared(msg) func createChainKeyWallet(): async Result<Text, Text> {
  let principal = msg.caller;

  // Check if wallet already exists
  let ?profile = userProfiles.get(Principal.toText(principal)) else {
    return #err("Profile not found");
  };

  if (profile.walletType != #NotSet) {
    return #err("Wallet already set");
  };

  // Derive Ethereum address from II Principal using threshold ECDSA
  let derivationPath = [Principal.toBlob(principal)];

  try {
    let publicKeyResult = await Management.ecdsa_public_key({
      canister_id = null;
      derivation_path = derivationPath;
      key_id = { curve = #secp256k1; name = "key_1" };
    });

    // Derive Ethereum address from public key
    let ethereumAddress = deriveEthereumAddress(publicKeyResult.public_key);

    // Update profile
    let updatedProfile = {
      profile with
      ethereumWallet = ?ethereumAddress;
      walletType = #ChainKeyDerived;
    };

    userProfiles.put(Principal.toText(principal), updatedProfile);
    #ok(ethereumAddress)
  } catch (error) {
    #err("Failed to derive wallet: " # Error.message(error))
  };
};

// Helper function to derive Ethereum address from secp256k1 public key
func deriveEthereumAddress(publicKey: Blob): Text {
  // Public key is 65 bytes: 0x04 + 32 bytes X + 32 bytes Y
  // Remove first byte (0x04), take Keccak-256 hash, use last 20 bytes
  let pubKeyWithoutPrefix = Blob.toArray(publicKey)[1..]; // Remove 0x04 prefix
  let hash = Keccak256.hash(pubKeyWithoutPrefix);
  let addressBytes = Array.subArray(hash, hash.size() - 20, 20);

  "0x" # Hex.encode(addressBytes)
};
```

**Chain-Key Cryptography Details:**
- Uses IC's **threshold ECDSA** (secp256k1 curve, same as Ethereum)
- Multiple IC subnet nodes hold **key shares**
- Signing requires threshold of nodes (Byzantine fault tolerant)
- **Deterministic derivation**: Same derivation path → Same public key
- No private key is ever stored or exposed

**Derivation Path:**
```
II Principal (e.g., xxxxx-xxxxx-xxxxx)
    ↓
Convert to Blob
    ↓
Use as derivation path
    ↓
IC threshold ECDSA generates public key
    ↓
Derive Ethereum address (Keccak-256 hash)
    ↓
Store in profile: 0x1234...abcd
```

---

## NFT Award Workflow

### **End-to-End Flow:**

```
User makes donation
    ↓
1. Stripe Payment Intent created
    ├─ Amount, customer info
    └─ Metadata: IC Principal
    ↓
2. Webhook → payment_bridge canister (Rust)
    ├─ HMAC-SHA256 signature verification
    └─ Extract Payment Intent ID
    ↓
3. payment_bridge → backend_canister (cross-canister call)
    ├─ Create donation record
    ├─ Link to Stripe Payment Intent ID
    ├─ Link to user's IC Principal
    └─ If >$15K: Status = Pending EDD
    ↓
4. Admin Review (if >$15K)
    ├─ Admin views pending donations in admin_dashboard
    ├─ Reviews KYB documents (Stripe Identity verification)
    ├─ Decision:
    │   ├─ Approve → Capture payment in Stripe
    │   └─ Reject → Cancel Payment Intent in Stripe
    └─ Update donation status
    ↓
5. Ethereum Wallet Resolution
    ├─ Check user profile for ethereumWallet
    ├─ If walletType = #External:
    │   └─ Use user-provided wallet address
    ├─ If walletType = #ChainKeyDerived:
    │   └─ Use IC-derived wallet address
    └─ If walletType = #NotSet:
        ├─ Prompt user to set up wallet
        └─ Hold NFT minting until wallet configured
    ↓
6. NFT Minting (via Chain Fusion)
    ├─ payment_bridge calls Einstein contract on Polygon
    ├─ Mint NFT to Ethereum wallet (external or chain-key)
    ├─ Store NFT token ID in donation record
    └─ Update donation status = Completed
```

---

## Accounting Data Flow & Reconciliation

### **Linking Keys:**

| From | To | Link | Type |
|------|-----|------|------|
| **Stripe** | **IC** | Payment Intent ID | Text |
| **IC** | **User** | Internet Identity Principal | Principal |
| **User** | **Ethereum Wallet** | Wallet address (external or chain-key) | Text (0x...) |
| **IC** | **NFT** | NFT token ID | Nat |
| **NFT** | **Ethereum Wallet** | Ownership on Polygon | On-chain |

### **Data Flow Paths:**

**Path A: External Wallet**
```
Stripe Payment (Payment Intent ID: pi_xxxxx)
    ↓
IC Donation Record
    ├─ stripePaymentIntentId = "pi_xxxxx"
    ├─ principal = xxxxx-xxxxx-xxxxx (II)
    ├─ ethereumWallet = "0x1234...abcd"
    └─ walletSource = #External
    ↓
User Profile
    ├─ principal = xxxxx-xxxxx-xxxxx (II)
    ├─ ethereumWallet = "0x1234...abcd"
    └─ walletType = #External
    ↓
NFT on Polygon
    ├─ Minted to: 0x1234...abcd
    └─ Token ID: 42
    ↓
IC Donation Record Updated
    └─ nftTokenId = ?42
```

**Path B: Chain-Key Wallet**
```
Stripe Payment (Payment Intent ID: pi_yyyyy)
    ↓
IC Donation Record
    ├─ stripePaymentIntentId = "pi_yyyyy"
    ├─ principal = bbbbb-bbbbb-bbbbb (II)
    ├─ ethereumWallet = "0x5678...efgh"
    └─ walletSource = #ChainKey
    ↓
User Profile
    ├─ principal = bbbbb-bbbbb-bbbbb (II)
    ├─ ethereumWallet = "0x5678...efgh" (derived via threshold ECDSA)
    └─ walletType = #ChainKeyDerived
    ↓
IC Derives Signing Key
    ├─ Derivation path: [Principal.toBlob(bbbbb-bbbbb-bbbbb)]
    └─ Threshold ECDSA signature for minting transaction
    ↓
NFT on Polygon
    ├─ Minted to: 0x5678...efgh
    └─ Token ID: 43
    ↓
IC Donation Record Updated
    └─ nftTokenId = ?43
```

---

## Reconciliation Requirements

### **Monthly Reconciliation Tasks:**

**1. Financial Reconciliation (Stripe ↔ IC)**
```sql
-- Pseudo-query: Check Stripe vs IC
SELECT
  stripe.payment_intent_id,
  stripe.amount,
  stripe.status,
  ic.donation_id,
  ic.amount,
  ic.status
FROM stripe_payments stripe
LEFT JOIN ic_donations ic ON stripe.payment_intent_id = ic.stripe_payment_intent_id
WHERE stripe.status = 'succeeded' AND ic.donation_id IS NULL
-- Identifies Stripe payments not recorded in IC
```

**2. Identity Reconciliation (IC ↔ User Profile ↔ Wallet)**
```motoko
// Verify all donations have valid wallet addresses
for ((donationId, donation) in donations.entries()) {
  switch (donation.ethereumWallet) {
    case null {
      // Log: Donation missing wallet address
      logWarning(donationId, "No wallet address set");
    };
    case (?wallet) {
      // Verify wallet in user profile
      let ?profile = userProfiles.get(Principal.toText(donation.principal)) else {
        logError(donationId, "User profile not found");
        continue;
      };

      if (profile.ethereumWallet != ?wallet) {
        logError(donationId, "Wallet mismatch: donation vs profile");
      };

      // For chain-key wallets, verify derivation
      if (donation.walletSource == #ChainKey) {
        let derivedWallet = await deriveWalletFromPrincipal(donation.principal);
        if (derivedWallet != wallet) {
          logError(donationId, "Chain-key wallet derivation mismatch");
        };
      };
    };
  };
};
```

**3. Asset Reconciliation (IC ↔ Polygon NFT)**
```typescript
// Verify NFTs minted to correct wallets
async function reconcileNFTs() {
  const donations = await backend.listCompletedDonations();

  for (const donation of donations) {
    if (!donation.nftMinted || !donation.nftTokenId) {
      console.warn(`Donation ${donation.id}: NFT not minted`);
      continue;
    }

    // Query Polygon: Who owns this NFT?
    const nftOwner = await einsteinContract.ownerOf(donation.nftTokenId);

    if (nftOwner.toLowerCase() !== donation.ethereumWallet.toLowerCase()) {
      console.error(`Donation ${donation.id}: NFT owner mismatch`);
      console.error(`  Expected: ${donation.ethereumWallet}`);
      console.error(`  Actual: ${nftOwner}`);
    }

    // Verify NFT metadata matches donation amount
    const metadata = await einsteinContract.tokenURI(donation.nftTokenId);
    // ... parse metadata, check donation amount
  }
}
```

**4. Wallet Source Verification**
```motoko
// Audit wallet sources for compliance
func auditWalletSources(): async AuditReport {
  var externalCount = 0;
  var chainKeyCount = 0;
  var notSetCount = 0;

  for ((principalText, profile) in userProfiles.entries()) {
    switch (profile.walletType) {
      case (#External) { externalCount += 1 };
      case (#ChainKeyDerived) { chainKeyCount += 1 };
      case (#NotSet) { notSetCount += 1 };
    };
  };

  {
    totalUsers = userProfiles.size();
    externalWallets = externalCount;
    chainKeyWallets = chainKeyCount;
    walletsNotSet = notSetCount;
    timestamp = Time.now();
  }
};
```

---

## Security Considerations

### **External Wallets:**
- ✅ User controls private key (full sovereignty)
- ⚠️ User responsible for key security
- ⚠️ CPP cannot recover if user loses key
- ✅ No IC dependency for wallet access

### **Chain-Key Wallets:**
- ✅ No private key storage/exposure
- ✅ Recoverable via II anchor recovery
- ⚠️ Trust in IC subnet threshold ECDSA
- ⚠️ Requires IC network availability

### **Payment Security:**
- Stripe webhook signatures verified (HMAC-SHA256)
- Payment Intent ID is primary financial key
- Admin approval required for >$15K donations
- Audit trail: All actions logged with timestamp + principal

### **NFT Minting Security:**
- Minting authority: payment_bridge canister only
- Contract owner: Multi-sig (3-of-5 Gnosis Safe)
- Metadata immutable after minting
- Ownership verified on-chain (Polygon)

---

## Governance & Access Control

### **System-Level Governance:**

| System | Owner | Admin Access | Purpose |
|--------|-------|--------------|---------|
| **Stripe Account** | Cool Planet Foundation | Stripe Dashboard + API keys | Payment processing |
| **IC Canister** | Canister controller (multi-sig) | Admin principals (II) | Business logic, KYB/EDD |
| **Einstein Contract** | Multi-sig (3-of-5 Gnosis Safe) | Contract owner | NFT minting, ownership |

### **Admin Operations:**

**Donation Review (>$15K):**
- Admin authenticates with Internet Identity
- View pending EDD donations via admin_dashboard
- Review KYB documents (Stripe Identity)
- Approve or reject donation
- See [admin-architecture.md](./admin-architecture.md) for admin bootstrap

**Wallet Management:**
- Users manage their own wallet settings
- Admins cannot change user wallet addresses
- Admins can view wallet type for audit purposes

**NFT Reconciliation:**
- Admins run monthly reconciliation reports
- Flag discrepancies for manual review
- Verify wallet sources (External vs ChainKey)

---

## Implementation Roadmap

### **Phase 1: Basic Donation Flow (Complete)**
- ✅ Stripe integration (payment_bridge canister)
- ✅ Donation record creation
- ✅ Payment Intent ID linking

### **Phase 2: Wallet Integration (In Progress)**
- 🚧 User profile wallet fields (ethereumWallet, walletType)
- 🚧 External wallet: User-provided address input
- ⏳ Chain-key wallet: Threshold ECDSA derivation
- ⏳ Wallet validation and verification

### **Phase 3: NFT Minting (Future)**
- ⏳ Einstein contract integration via Chain Fusion
- ⏳ NFT minting to external wallets
- ⏳ NFT minting to chain-key wallets
- ⏳ Token ID storage in donation records

### **Phase 4: Reconciliation & Monitoring (Future)**
- ⏳ Automated financial reconciliation (Stripe ↔ IC)
- ⏳ Automated asset reconciliation (IC ↔ Polygon)
- ⏳ Wallet source audit reports
- ⏳ Alerting for discrepancies

---

## Summary

**Three-System Architecture:**
- Stripe (payments) → IC (coordination) → Polygon (NFTs)

**Dual-Wallet Approach:**
- External: User-owned (MetaMask, Ledger)
- Chain-Key: IC-derived (threshold ECDSA)

**Linking Strategy:**
- Payment Intent ID: Stripe ↔ IC
- II Principal: IC ↔ User
- Ethereum Wallet: User ↔ NFT (external or chain-key)
- NFT Token ID: IC ↔ Polygon

**Reconciliation:**
- Monthly financial: Stripe settlements vs IC donations
- Monthly asset: IC donations vs Polygon NFTs
- Wallet source audit: External vs ChainKeyDerived

---

**Last Updated:** 2025-11-13
**Version:** 1.0.0
**Status:** Active Development
**Review Cycle:** Update as NFT workflow is implemented

**Cross-References:**
- [admin-architecture.md](./admin-architecture.md) - Admin bootstrap and governance
- [canister-bootstrap-pattern.md](./canister-bootstrap-pattern.md) - Canister security patterns
- [system-architecture-overview.md](./system-architecture-overview.md) - Overall CPP architecture
