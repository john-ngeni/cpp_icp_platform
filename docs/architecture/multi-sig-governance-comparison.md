# Multi-Sig Governance: Implementation Options for IC Canister Control

**Date:** 2025-11-13
**Purpose:** Compare multi-sig governance approaches for IC canister controllers
**Context:** Evaluating Gnosis Safe vs IC-native solutions for CPP platform governance
**Cross-Reference:**
- See [governance-policy.md](./governance-policy.md) for **WHO approves WHAT** (governance policy)
- See [canister-bootstrap-pattern.md](./canister-bootstrap-pattern.md) for bootstrap patterns

## Executive Summary

**The Question:** How should CPP implement multi-sig control over critical canisters?

**Options Evaluated:**
1. **Gnosis Safe (Cross-Chain)** - Ethereum-based multi-sig via Chain Fusion
2. **NNS-Based Multi-Sig (Hybrid Rust/Motoko)** - NNS execution engine + simple voting
3. **Custom Multi-Sig (Pure Motoko)** - Build from scratch
4. **SNS (Service Nervous System)** - Full NNS-style governance for dapps
5. **Hybrid Approach** - Different governance for different stages

**Recommendation:** Start with **NNS-based multi-sig (Rust + Motoko)**, graduate to **SNS** when mature.

### **Recommended Architecture: Hybrid Rust/Motoko**

```
┌─────────────────────────────────────────────────┐
│  Voting Canister (Motoko - NEW, ~300 lines)    │
│  • Simple M-of-N voting (1 principal = 1 vote) │
│  • Two-level: Board (3-of-3) + Tech (1-of-2+)  │
│  • Returns: hasReachedThreshold() -> Bool      │
└────────────────┬────────────────────────────────┘
                 │ voting results
                 ↓
┌─────────────────────────────────────────────────┐
│  Governance Canister (Rust - ADAPTED FROM NNS)  │
│  • Proposal submission & execution (battle-tested)│
│  • Proposal types from NNS (UpgradeCanister, etc)│
│  • Automatic execution when approved            │
└────────────────┬────────────────────────────────┘
                 │ executes proposals
                 ↓
          Production Canisters
```

**Why This Approach:**
- ✅ **Rust (NNS):** Battle-tested proposal execution (governs billions in ICP)
- ✅ **Motoko:** Simple voting logic, easy to audit and customize
- ✅ **Clean separation:** Execution vs voting = clear interfaces
- ✅ **Reasonable effort:** 4-7 days vs weeks for pure custom
- ✅ **SNS upgrade path:** Same Rust base as full SNS

**Governance Structure:** See [governance-policy.md](./governance-policy.md) for complete governance policy separating:
- **Board governance** (3 board members) - Strategic and financial decisions
- **Technical governance** (1-2 initially → 3-5) - Day-to-day operations

---

## Option 1: Gnosis Safe as Controller (Cross-Chain)

### **How It Would Work**

```
Gnosis Safe (Ethereum)
    ↓ (3-of-5 signers approve action)
Chain Fusion Bridge
    ↓ (Ethereum signature verification on IC)
Threshold ECDSA Wallet (IC)
    ↓ (Derived IC principal)
Canister Controller
    ↓
Upgrade/manage canisters
```

**Implementation:**
1. Deploy Gnosis Safe on Ethereum/Polygon
2. Gnosis Safe controls an Ethereum wallet
3. IC canister derives principal from Ethereum wallet using Chain Fusion
4. That derived principal is set as canister controller
5. For upgrades: Gnosis Safe signers approve → transaction sent to IC via Chain Fusion

### **Advantages**
- ✅ Proven security model (Gnosis Safe is battle-tested)
- ✅ Familiar UI for Ethereum ecosystem users
- ✅ Can use hardware wallets (Ledger, Trezor)
- ✅ Established multi-sig best practices

### **Disadvantages**
- ❌ **Complex cross-chain dependency**: Requires Chain Fusion bridge reliability
- ❌ **External dependency**: IC governance depends on Ethereum network
- ❌ **High friction**: Every upgrade requires Ethereum transaction + gas fees
- ❌ **Latency**: Ethereum confirmation times + IC processing
- ❌ **Not IC-native**: Doesn't leverage IC's native capabilities
- ❌ **Bridge risk**: Chain Fusion bridge is additional attack surface
- ❌ **Cost**: Ethereum gas fees for every governance action

### **Friction Analysis**

**For a typical canister upgrade:**
```
1. Prepare upgrade proposal
2. Submit to Gnosis Safe (Ethereum transaction, $5-50 gas)
3. Wait for signers to approve (hours to days)
4. Execute on Gnosis Safe (Ethereum transaction, $10-100 gas)
5. Chain Fusion bridge processes (minutes to hours)
6. IC canister upgrade executes
Total time: Days + $15-150 in gas fees
```

**Evolution friction:** 🔴 **VERY HIGH** - Every upgrade requires cross-chain coordination

---

## Option 2: NNS-Based Multi-Sig (Hybrid Rust/Motoko Architecture)

### **Use Battle-Tested NNS Code, Simplify Voting**

**Key Insight:** The NNS (Network Nervous System) that governs ICP is open source and written in **Rust**. We can use its battle-tested proposal execution engine with simplified voting logic.

### **Hybrid Architecture: Rust Core + Motoko Voting**

```
┌──────────────────────────────────────────────────────┐
│  Voting Canister (Motoko - NEW)                      │
│  - Simple M-of-N voting (no tokens/neurons)          │
│  - Two-level governance (Board 3-of-3, Tech M-of-N)  │
│  - Vote tracking: 1 principal = 1 vote               │
│  - Returns: hasReachedThreshold(proposalId) -> Bool  │
└─────────────────┬────────────────────────────────────┘
                  │ (voting results)
                  ↓
┌──────────────────────────────────────────────────────┐
│  Governance Canister (Rust - ADAPTED FROM NNS)       │
│  - Proposal submission system                        │
│  - Proposal types (UpgradeCanister, TransferCycles)  │
│  - Execution engine (automatic when threshold met)   │
│  - Audit trail (on-chain logging)                    │
│  - Wait periods (optional delay)                     │
└─────────────────┬────────────────────────────────────┘
                  │ (executes proposals)
                  ↓
           Production Canisters
```

### **What to Keep from NNS (Rust)**

**From:** `rs/nns/governance/src/governance.rs`

- ✅ **Proposal submission system** - How proposals are created, validated
- ✅ **Proposal types** - UpgradeCanister, ManageCycles, AddController, etc.
- ✅ **Execution engine** - Automatic execution when approved
- ✅ **Audit trail** - All actions logged on-chain with timestamps
- ✅ **Wait periods** - Optional delay before execution (e.g., 48 hour review)
- ✅ **Proposal lifecycle** - Pending → Approved/Rejected → Executed/Failed

**Remove from NNS:**
- ❌ `neuron.rs` - Neuron management (staking, dissolve delays)
- ❌ `voting_rewards.rs` - Reward distribution to voters
- ❌ Voting power calculations (weighted by stake + age)
- ❌ Follow relationships (auto-voting based on followees)
- ❌ Token economics (ICP integration)

### **What to Implement in Motoko (NEW)**

**Voting canister** replaces NNS neuron-based voting:

```motoko
// voting_canister.mo
actor VotingCanister {
  // Two voting groups
  type VotingGroup = {
    #Board;      // 3 board members
    #Technical;  // 1-2 initially, grows to 3-5
  };

  // Simple M-of-N voting (no neurons, no tokens)
  type Vote = {
    proposalId: Nat64;
    voter: Principal;
    vote: { #Approve; #Reject };
    votedAt: Nat64;
  };

  stable var boardMembers: [Principal] = [];
  stable var technicalSigners: [Principal] = [];
  stable var votes: [Vote] = [];

  // Check if proposal has reached threshold
  public query func hasReachedThreshold(
    proposalId: Nat64,
    votingGroup: VotingGroup
  ): async Bool {
    let approvals = countApprovals(proposalId, votingGroup);
    let (required, _) = getThreshold(votingGroup);
    approvals >= required
  };

  // Cast vote (1 principal = 1 vote)
  public shared(msg) func castVote(
    proposalId: Nat64,
    vote: { #Approve; #Reject }
  ): async Result<(), Text> {
    // Check authorization
    let ?group = getVotingGroup(msg.caller) else {
      return #err("Not authorized to vote");
    };

    // Record vote
    let voteRecord: Vote = {
      proposalId = proposalId;
      voter = msg.caller;
      vote = vote;
      votedAt = Time.now();
    };
    votes := Array.append(votes, [voteRecord]);

    // Notify governance canister if threshold reached
    if (hasReachedThreshold(proposalId, group)) {
      await governanceCanister.notifyThresholdReached(proposalId);
    };

    #ok(())
  };

  // Get threshold based on group and team phase
  func getThreshold(group: VotingGroup): (Nat, Nat) {
    switch (group) {
      case (#Board) { (3, 3) };  // Always 3-of-3 for board
      case (#Technical) {
        // Evolves based on team size
        let signerCount = technicalSigners.size();
        if (signerCount <= 2) { (1, 2) }       // Phase 1: 1-of-2
        else if (signerCount == 3) { (2, 3) }  // Phase 2: 2-of-3
        else { (3, 5) }                        // Phase 3: 3-of-5
      };
    };
  };
};
```

### **Interface Between Rust and Motoko**

**Rust Governance Canister** calls **Motoko Voting Canister**:

```rust
// In governance canister (Rust)
async fn check_if_approved(proposal_id: u64) -> bool {
    // Call Motoko voting canister
    let voting_canister = Principal::from_text("voting-canister-id");
    let result: (bool,) = call(
        voting_canister,
        "hasReachedThreshold",
        (proposal_id,)
    ).await.unwrap();

    result.0  // true if threshold reached
}

async fn execute_proposal_if_approved(proposal_id: u64) {
    if check_if_approved(proposal_id).await {
        // Execute using NNS execution engine
        match proposals.get(&proposal_id) {
            Some(ProposalData::UpgradeCanister(data)) => {
                upgrade_canister(data.canister_id, data.wasm_module).await;
            }
            Some(ProposalData::TransferCycles(data)) => {
                transfer_cycles(data.to, data.amount).await;
            }
            // ... other proposal types from NNS
        }
    }
}
```

**What We Get:**
```
NNS Governance (Rust)
    ↓ (keep proposal submission & execution)
Remove Neuron Voting Logic
    ↓ (replace with simple M-of-N)
Motoko Voting Canister
    ↓ (1 principal = 1 vote, two-level governance)
CPP Multi-Sig Governance
```

### **Concrete Implementation Architecture**

**Two Canisters Working Together:**

1. **Governance Canister (Rust)** - Adapted from NNS
   - Handles proposal submission and execution
   - Manages proposal lifecycle
   - Calls voting canister to check approval

2. **Voting Canister (Motoko)** - Custom implementation
   - Handles M-of-N voting logic
   - Manages two voting groups (Board, Technical)
   - Returns approval status to governance canister

**Workflow:**

```
User submits proposal
    ↓
Governance Canister (Rust)
    ├─ Validate proposal
    ├─ Determine voting group (Board or Technical)
    ├─ Store proposal
    └─ Wait for votes
         ↓
Voting Canister (Motoko)
    ├─ Signers cast votes (1 principal = 1 vote)
    ├─ Track approvals/rejections
    ├─ Check if threshold reached
    └─ Notify governance canister
         ↓
Governance Canister (Rust)
    ├─ Query voting canister: hasReachedThreshold()?
    ├─ If yes: Execute proposal (upgrade canister, transfer cycles, etc.)
    └─ Update audit trail
```

**Key Files from NNS to Adapt:**

```
dfinity/ic/rs/nns/governance/
├── governance.rs          ← Keep: Proposal execution engine
├── proposals.rs           ← Keep: Proposal types and validation
├── proposal_submission.rs ← Keep: How proposals are submitted
├── neuron.rs              ← Remove: Replace with voting_canister.mo
├── voting_rewards.rs      ← Remove: No rewards needed
└── types.rs               ← Adapt: Remove neuron types, keep proposal types
```

**New Files to Create:**

```
cpp_icp_platform/canisters/
├── governance/            ← Rust (adapted from NNS)
│   ├── Cargo.toml
│   ├── governance.rs      ← Stripped-down NNS governance
│   └── lib.rs
└── voting/                ← Motoko (new, simple)
    └── voting.mo          ← M-of-N voting logic
```

### **Advantages**
- ✅ **Battle-tested execution**: NNS proposal execution has governed billions in value
- ✅ **Open source**: Rust governance code is available, audited, proven
- ✅ **Clean separation**: Rust (execution) + Motoko (voting) = clear interfaces
- ✅ **Simpler voting**: Motoko voting is ~200-300 lines vs complex neuron logic
- ✅ **Flexible**: Easy to adapt for two-level governance (board + technical)
- ✅ **IC-native**: No external dependencies, all on-chain
- ✅ **Familiar**: Community knows NNS proposal model
- ✅ **SNS upgrade path**: Can upgrade to full SNS when ready (same Rust base)
- ✅ **Rust benefits**: Type safety, performance, tooling for governance logic
- ✅ **Motoko benefits**: Simple syntax, easy to audit voting logic

### **Disadvantages**
- ⚠️ **Two canisters**: Governance + Voting (adds cross-canister call latency)
- ⚠️ **Rust adaptation**: Need to strip out neuron code from NNS (moderate effort)
- ⚠️ **Two languages**: Team needs both Rust and Motoko expertise
- ⚠️ **Custom voting**: Motoko voting canister needs to be written and audited

### **Friction Analysis**

**For a typical canister upgrade (technical):**
```
1. Technical signer submits proposal to governance canister (Rust)
2. Governance canister validates, stores proposal
3. Technical signers vote via voting canister (Motoko)
4. Voting canister checks: 1-of-2 threshold reached?
5. Governance canister queries voting canister: approved?
6. Governance canister executes upgrade automatically
Total time: Minutes to hours
Cost: Minimal cycles (2 cross-canister calls)
```

**For NFT contract deployment (board):**
```
1. Technical team prepares, submits to governance canister
2. Governance determines: Board voting group required
3. Board members vote via voting canister (Motoko)
4. Voting canister checks: 3-of-3 threshold reached?
5. Governance canister executes deployment automatically
Total time: Days (board review time, not technical latency)
Cost: Minimal cycles
```

**Evolution friction:** 🟢 **LOW** - Simple voting, fast execution, cross-canister call adds ~1s

### **Implementation Steps**

**Step 1: Adapt NNS Rust Governance**
```bash
# Clone NNS governance code
git clone https://github.com/dfinity/ic.git
cd ic/rs/nns/governance

# Create CPP governance canister
cp -r . ~/git/cpp_icp_platform/canisters/governance/

# Strip out (delete):
- neuron.rs (neuron management)
- voting_rewards.rs (reward distribution)
- Voting power calculations in governance.rs

# Adapt (modify):
- governance.rs: Replace check_votes() with call to voting canister
- proposals.rs: Keep proposal types, add CPP-specific types
- types.rs: Remove Neuron types, keep Proposal types

# Add (new):
- voting_client.rs: Interface to Motoko voting canister
```

**Step 2: Create Motoko Voting Canister**
```bash
# Create new voting canister
mkdir -p ~/git/cpp_icp_platform/canisters/voting
cd ~/git/cpp_icp_platform/canisters/voting

# voting.mo (new file, ~300 lines)
# - VotingGroup type (Board, Technical)
# - Vote tracking (1 principal = 1 vote)
# - Threshold calculation (evolving with team size)
# - Public API: castVote(), hasReachedThreshold()
```

**Step 3: Deploy and Link**
```bash
# Deploy voting canister first
dfx deploy voting_canister --network ic

# Deploy governance canister with voting canister ID
dfx deploy governance_canister --network ic \
  --argument '(record { voting_canister = principal "aaaaa-aaaaa-aaaaa" })'

# Set governance canister as controller of production canisters
dfx canister update-settings backend_canister \
  --add-controller $(dfx canister id governance_canister)
```

### **NNS Code Reference**

**Where to find it:**
- GitHub: https://github.com/dfinity/ic/tree/master/rs/nns/governance
- NNS Governance canister: `rrkah-fqaaa-aaaaa-aaaaq-cai`
- Documentation: https://internetcomputer.org/docs/current/developer-docs/integrations/sns/

**Key Rust files to adapt:**
- `governance/src/governance.rs` - Keep: Proposal execution engine
- `governance/src/proposals.rs` - Keep: Proposal types
- `governance/src/neuron.rs` - **Remove**: Replace with voting_canister.mo
- `governance/src/types.rs` - Adapt: Remove Neuron types

**Estimated adaptation effort:**
- Rust governance adaptation: 2-3 days (strip out neuron code)
- Motoko voting canister: 1-2 days (write from scratch, ~300 lines)
- Testing and integration: 1-2 days
- **Total: 4-7 days for experienced IC developer**

---

## Option 3: Basic Multi-Sig Canister (Simplified Custom)

### **Architecture Overview**

**The multi-sig canister IS the controller** for all production canisters:

```
Multi-Sig Canister (Controller)
    ├─ Controls: backend_canister (cpf_members)
    ├─ Controls: backend_api (fti_newsletter_archive)
    ├─ Controls: payment_bridge
    ├─ Controls: admin_dashboard
    ├─ Controls: pwa_canister
    └─ Controls: [any other production canisters]

Signers (5 Internet Identity principals)
    ├─ Signer 1: Core team member
    ├─ Signer 2: Core team member
    ├─ Signer 3: Core team member
    ├─ Signer 4: Advisor/investor
    └─ Signer 5: Advisor/investor

Threshold: 3-of-5 approvals required
```

**What this means:**
- Multi-sig canister is set as `controller` via `dfx canister update-settings`
- All canister management actions go through multi-sig approval
- Individual dfx identities are removed as controllers
- Signers use Internet Identity to approve proposals

### **Governance Actions: What Requires Multi-Sig?**

Not all actions should require multi-sig approval. We need different governance levels:

#### **Tier 1: Critical Actions (3-of-5 Multi-Sig Required)**

| Action | Description | Rationale |
|--------|-------------|-----------|
| **Upgrade canister** | Deploy new WASM code | Code changes could introduce vulnerabilities |
| **Add/remove controller** | Change who controls canister | Fundamental security change |
| **Change multi-sig threshold** | Modify M-of-N requirement | Changes governance rules |
| **Add/remove signer** | Change who can approve | Changes who has control |
| **Delete canister** | Permanently destroy canister | Irreversible action |
| **Transfer cycles (large)** | Transfer >10T cycles | Significant asset movement |
| **Change stable memory** | Modify stable variables directly | Can corrupt canister state |
| **Enable/disable canister** | Stop/start canister | Service availability impact |

#### **Tier 2: Admin Actions (2-of-5 Multi-Sig or Admin Role)**

| Action | Description | Governance |
|--------|-------------|------------|
| **Top up cycles (routine)** | Add <10T cycles | 2-of-5 or automated |
| **Update application config** | Change app parameters | 2-of-5 or admin principal |
| **Emergency pause** | Temporary service stop | 2-of-5 (fast response) |
| **Blacklist address** | Block malicious user | Admin principal |
| **Adjust rate limits** | Change API limits | Admin principal |

#### **Tier 3: Operational Actions (Automated or Single Admin)**

| Action | Description | Governance |
|--------|-------------|------------|
| **Monitor cycles** | Check cycle balance | Automated monitoring |
| **Monitor canister status** | Health checks | Automated monitoring |
| **Alert on low cycles** | Notify admins | Automated alert |
| **Metrics collection** | Gather usage stats | Automated |
| **Log rotation** | Manage audit logs | Automated |
| **Backup state** | Export canister state | Automated or single admin |

#### **Tier 4: Emergency Actions (1-of-5 or Auto)**

| Action | Description | Governance |
|--------|-------------|------------|
| **Emergency stop** | Immediate pause | Any 1 signer |
| **Auto cycle top-up** | Prevent canister freeze | Automated (if balance < threshold) |
| **Circuit breaker** | Stop on anomaly detection | Automated |

### **How It Would Work**

```motoko
// Multi-sig controller canister
actor MultiSigController {
  type Proposal = {
    id: Nat;
    proposer: Principal;
    target: Principal;           // Canister to upgrade
    wasmModule: Blob;            // New WASM code
    approvals: [Principal];      // Who approved
    status: ProposalStatus;      // Pending, Approved, Rejected, Executed
    createdAt: Nat64;
  };

  stable var proposals: [Proposal] = [];
  stable var signers: [Principal] = [/* 5 signer principals */];
  stable var threshold: Nat = 3;  // 3-of-5

  // Submit upgrade proposal
  public shared(msg) func proposeUpgrade(
    target: Principal,
    wasmModule: Blob
  ): async Result<Nat, Text> {
    if (not isSigner(msg.caller)) {
      return #err("Not authorized");
    };

    let proposalId = proposals.size();
    let proposal = {
      id = proposalId;
      proposer = msg.caller;
      target = target;
      wasmModule = wasmModule;
      approvals = [msg.caller];  // Proposer auto-approves
      status = #Pending;
      createdAt = Time.now();
    };

    proposals := Array.append(proposals, [proposal]);
    #ok(proposalId)
  };

  // Approve proposal
  public shared(msg) func approve(proposalId: Nat): async Result<(), Text> {
    if (not isSigner(msg.caller)) {
      return #err("Not authorized");
    };

    let ?proposal = proposals[proposalId] else {
      return #err("Proposal not found");
    };

    if (proposal.status != #Pending) {
      return #err("Proposal not pending");
    };

    // Add approval
    let newApprovals = Array.append(proposal.approvals, [msg.caller]);
    proposals[proposalId] := { proposal with approvals = newApprovals };

    // Check if threshold reached
    if (newApprovals.size() >= threshold) {
      // Execute upgrade
      await executeUpgrade(proposal);
      proposals[proposalId] := {
        proposals[proposalId] with status = #Executed
      };
    };

    #ok(())
  };

  // Execute upgrade (called automatically when threshold reached)
  func executeUpgrade(proposal: Proposal): async () {
    await Management.install_code({
      mode = #upgrade;
      canister_id = proposal.target;
      wasm_module = proposal.wasmModule;
      arg = [];
    });
  };
};
```

### **Implementation Steps**

1. **Deploy multi-sig canister** with initial signers (5 principals)
2. **Set multi-sig as controller** of managed canisters
3. **Workflow for upgrades:**
   - Developer prepares WASM
   - Signer #1 submits proposal via `proposeUpgrade()`
   - Signers #2, #3 call `approve(proposalId)`
   - Upgrade executes automatically when 3 approvals reached

### **Advantages**
- ✅ **IC-native**: No external dependencies
- ✅ **Low friction**: Approvals happen on IC (fast, cheap)
- ✅ **Flexible**: Easy to adjust threshold or signers
- ✅ **Low cost**: Only IC cycles, no gas fees
- ✅ **Fast**: Minutes, not hours/days
- ✅ **Transparent**: All proposals on-chain and auditable
- ✅ **Programmable**: Can add custom logic (time locks, veto periods)

### **Disadvantages**
- ⚠️ **Trust in signers**: Signers control IC principals (could be compromised)
- ⚠️ **Custom implementation**: Need to build and audit
- ⚠️ **Less proven**: Not as battle-tested as Gnosis Safe
- ⚠️ **Key management**: Signers need to secure their IC principals

### **Friction Analysis**

**For a typical canister upgrade:**
```
1. Prepare upgrade proposal
2. Signer #1 submits proposal (IC update call, ~0.1s, minimal cycles)
3. Signers #2, #3 approve (IC update calls, ~0.1s each)
4. Upgrade executes automatically
Total time: Minutes + negligible cost
```

**Evolution friction:** 🟢 **LOW** - Fast iteration, minimal overhead

### **Existing IC Multi-Sig Solutions**

**1. Internet Identity as Signers**
- Each signer uses their Internet Identity
- Hardware security keys (YubiKey, etc.) via WebAuthn
- Multi-device support
- Recovery via seed phrase or social recovery

**2. ICDevs.org Multi-Sig Canister**
- Open-source implementation
- Basic M-of-N functionality
- Used by several IC projects

**3. NFID Multi-Sig**
- Built on Internet Identity
- Team account management
- Multi-sig approvals

---

## Option 3: SNS (Service Nervous System)

### **How It Would Work**

SNS is the **NNS-style governance** model for individual dapps:

```
1. Create SNS for CPP Platform
    ↓
2. Issue governance tokens (e.g., CPP tokens)
    ↓
3. Users stake tokens → neurons
    ↓
4. Neurons have voting power (based on stake + age)
    ↓
5. Proposals submitted (upgrade canister, change parameters)
    ↓
6. Neurons vote on proposals
    ↓
7. If approved → automatic execution
```

### **SNS Components**

**Governance Canister:**
- Manages proposals and voting
- Tracks neurons (staked tokens)
- Executes approved proposals

**Ledger Canister:**
- Tracks CPP token balances
- Handles staking/unstaking
- SNS-1 token standard

**Root Canister:**
- Controls all managed canisters
- Executes upgrades approved by governance

**Swap Canister (Optional):**
- Decentralization sale
- Initial token distribution

### **Advantages**
- ✅ **True decentralization**: Community governance
- ✅ **Proven model**: NNS has governed ICP since genesis
- ✅ **Automatic execution**: No manual intervention
- ✅ **Token economics**: Align incentives
- ✅ **IC-native**: Fully integrated with IC
- ✅ **Audited**: DFINITY-provided, well-tested

### **Disadvantages**
- ❌ **Very high friction**: Proposals require voting period (days)
- ❌ **Complex setup**: Requires token launch, distribution
- ❌ **Slow iteration**: Not suitable for early-stage development
- ❌ **Governance overhead**: Need active community
- ❌ **Token mechanics**: Complex economics, legal considerations
- ❌ **Irreversible**: Once SNS, hard to go back

### **Friction Analysis**

**For a typical canister upgrade:**
```
1. Prepare upgrade proposal
2. Submit proposal to SNS governance
3. Wait for voting period (48-72 hours minimum)
4. Neurons vote (requires quorum)
5. If approved, automatic execution
6. If rejected, start over
Total time: 2-7 days
```

**Evolution friction:** 🔴 **VERY HIGH** - Not suitable for rapid iteration

### **When to Use SNS**

SNS is appropriate when:
- ✅ Project is **mature and stable** (fewer upgrades needed)
- ✅ Have **active community** willing to participate in governance
- ✅ Want **true decentralization** (no centralized control)
- ✅ **Token economics** make sense for the project
- ✅ Ready for **slower iteration** (governance takes time)

**NOT appropriate for:**
- ❌ Early-stage development (too much friction)
- ❌ Projects requiring frequent updates
- ❌ Small teams without community

---

## Option 4: Hybrid Approach (RECOMMENDED)

### **Staged Governance Evolution**

```
┌─────────────────────────────────────────────────────────────┐
│                  CPP Governance Evolution                    │
└─────────────────────────────────────────────────────────────┘

Stage 1: Single Controller (Current - Dev/Preprod)
    ↓ Controller: Individual dfx identity
    ↓ Purpose: Rapid iteration, testing
    ↓ Duration: During active development
    ↓ Friction: NONE (instant upgrades)
    ↓
Stage 2: Basic Multi-Sig (Production Launch)
    ↓ Controller: 3-of-5 multi-sig canister
    ↓ Purpose: Security without sacrificing speed
    ↓ Duration: 1-2 years (platform stabilization)
    ↓ Friction: LOW (minutes for upgrades)
    ↓
Stage 3: SNS Governance (Mature/Decentralized)
    ↓ Controller: SNS root canister
    ↓ Purpose: Full decentralization
    ↓ Duration: Long-term
    ↓ Friction: HIGH (days for upgrades)
```

### **Implementation Plan**

#### **Stage 1: Single Controller (Current)**

```bash
# Development
dfx deploy backend_canister --network local

# Preprod
dfx deploy backend_canister --network ic --identity preprod-admin

# Controller: Individual dfx identity
# Upgrade: Instant, no approval needed
```

**Advantages:**
- Fast iteration
- Quick bug fixes
- Test everything rapidly

**Risks:**
- Single point of failure
- No checks and balances

**Duration:** Until production launch

---

#### **Stage 2: Basic Multi-Sig (Production)**

```bash
# 1. Deploy multi-sig canister
dfx deploy multi_sig_controller --network ic --argument '(
  record {
    signers = vec {
      principal "aaaaa-aaaaa-aaaaa";  # Admin 1
      principal "bbbbb-bbbbb-bbbbb";  # Admin 2
      principal "ccccc-ccccc-ccccc";  # Admin 3
      principal "ddddd-ddddd-ddddd";  # Admin 4
      principal "eeeee-eeeee-eeeee";  # Admin 5
    };
    threshold = 3;
  }
)'

# 2. Set multi-sig as controller of all production canisters
dfx canister update-settings backend_canister \
  --add-controller $(dfx canister id multi_sig_controller) \
  --network ic

# 3. Remove individual controller
dfx canister update-settings backend_canister \
  --remove-controller $(dfx identity get-principal) \
  --network ic
```

**Upgrade Workflow:**
```bash
# Signer 1: Submit proposal
dfx canister call multi_sig_controller proposeUpgrade '(
  principal "backend-canister-id",
  blob "$(cat backend_canister.wasm)"
)'
# Returns: (ok 42)  -- Proposal ID

# Signers 2 & 3: Approve
dfx canister call multi_sig_controller approve '(42)'

# Upgrade executes automatically after 3rd approval
```

**Advantages:**
- Security (3-of-5 required)
- Low friction (minutes)
- Flexible (can adjust signers/threshold)
- Suitable for regular updates

**Who are the signers?**
- 2-3 core team members
- 1-2 trusted advisors/investors
- Geographic diversity (different jurisdictions)

**Duration:** 1-2 years of production operation

---

#### **Stage 3: SNS Governance (Mature)**

```bash
# 1. Prepare SNS configuration
dfx sns init sns_config.yaml

# 2. Deploy SNS
dfx sns deploy --network ic

# 3. Transfer canister control to SNS
# (Multi-sig approves this final transition)
dfx canister update-settings backend_canister \
  --add-controller $(dfx canister id sns_root) \
  --network ic

# 4. Launch token sale (if desired)
dfx sns swap open

# 5. Decentralization complete
# All future upgrades require SNS proposals + voting
```

**Upgrade Workflow:**
```bash
# 1. Submit upgrade proposal
dfx canister call sns_governance submit_proposal '(
  record {
    title = "Upgrade backend canister to v2.0";
    summary = "Bug fixes and new features";
    action = variant {
      UpgradeCanister = record {
        canister_id = principal "backend-id";
        wasm_module = blob "...";
      }
    };
  }
)'

# 2. Wait for voting period (48-72 hours)

# 3. Neurons vote

# 4. If approved, automatic execution

# 5. Cannot be reversed (need new proposal)
```

**When to transition:**
- Platform is stable (infrequent upgrades)
- Active community of users
- Token economics designed
- Ready for slower iteration
- Want true decentralization

---

## ENS Control via IC Governance (Chain-Key ECDSA)

### **Overview**

**Cross-cutting concern:** Regardless of which multi-sig governance approach you choose, the CPP platform needs to control the `cpf.nft` ENS NFT on Ethereum. The ENS NFT is an **ERC-721 token** that controls:
- DNS records for cpf.nft and subdomains
- Subdomain creation (e.g., authors.cpf.nft)
- ENS text records
- Ownership transfer

### **The Challenge**

ENS is on **Ethereum**, but IC governance is on the **Internet Computer**. How do board members using IC-based identities (authors.cpf.nft Internet Identity principals) control an Ethereum NFT?

### **Solution: ICP Chain-Key Control of ENS NFT**

```
Board Members (authors.cpf.nft identities)
    ↓ submit/approve governance proposals (IC)
Governance Canister (IC)
    ↓ uses threshold ECDSA to derive Ethereum address
ICP-Derived Ethereum Address
    ↓ owns ENS NFT on Ethereum
    ↓ signs transactions via chain-key ECDSA
Ethereum Network
    ↓ executes ENS contract updates
cpf.nft ENS NFT
```

### **How It Works**

**Step 1: Derive Ethereum Address from IC Canister**

```rust
// In governance canister (Rust)
async fn derive_ethereum_address() -> String {
    let key_id = EcdsaKeyId {
        curve: EcdsaCurve::Secp256k1,
        name: "key_1".to_string(),
    };

    let arg = EcdsaPublicKeyArgument {
        canister_id: None,
        derivation_path: vec![b"ens-control".to_vec()],
        key_id: key_id.clone(),
    };

    let (response,) = ecdsa_public_key(arg).await.unwrap();
    let ethereum_address = derive_eth_address_from_pubkey(&response.public_key);

    format!("0x{}", hex::encode(ethereum_address))
}
```

**Step 2: Board Submits ENS Update Proposal**

```rust
// Board member (via authors.cpf.nft) submits proposal
public async fn propose_ens_update(
    caller: Principal,  // Board member principal
    domain: String,     // e.g., "authors.cpf.nft"
    record_type: String,  // "A", "CNAME", "TXT", etc.
    value: String       // e.g., "ic0.app"
) -> Result<ProposalId, String> {
    // Verify caller is board member (authors.cpf.nft identity)
    if !is_board_member(caller) {
        return Err("Unauthorized: Board members only".to_string());
    }

    // Create proposal
    let proposal = Proposal {
        proposer: caller,
        action: ProposalAction::UpdateENSRecord {
            domain,
            record_type,
            value,
        },
        voting_group: VotingGroup::Board,  // Requires 3-of-3
        status: ProposalStatus::Pending,
    };

    // Submit to voting canister
    let proposal_id = voting_canister.create_proposal(proposal).await?;
    Ok(proposal_id)
}
```

**Step 3: Board Members Vote (3-of-3)**

```typescript
// Board members authenticate with authors.cpf.nft
const authClient = await AuthClient.create();
await authClient.login({
  identityProvider: "https://identity.ic0.app",
  derivationOrigin: "https://authors.cpf.nft",  // Board only
});

// Vote on proposal
await governanceCanister.vote({
  proposalId: 42,
  vote: "approve"
});
```

**Step 4: Execute ENS Update via Chain-Key Signature**

```rust
// In governance canister (Rust)
async fn execute_ens_update(proposal: UpdateENSRecord) -> Result<(), String> {
    // 1. Encode ENS contract call
    let ens_contract_address = "0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e";
    let call_data = encode_ens_update_call(
        &proposal.domain,
        &proposal.record_type,
        &proposal.value
    );

    // 2. Build Ethereum transaction
    let tx = EthereumTransaction {
        to: ens_contract_address,
        value: 0,  // No ETH transfer
        gas: 100_000,
        gas_price: get_current_gas_price().await?,
        nonce: get_nonce().await?,
        data: call_data,
    };

    // 3. Sign transaction using chain-key ECDSA
    let signature = sign_ethereum_transaction(tx).await?;

    // 4. Submit to Ethereum via HTTPS outcalls
    submit_to_ethereum(signature).await?;

    Ok(())
}

async fn sign_ethereum_transaction(tx: EthereumTransaction) -> Result<Vec<u8>, String> {
    let key_id = EcdsaKeyId {
        curve: EcdsaCurve::Secp256k1,
        name: "key_1".to_string(),
    };

    let message_hash = keccak256(&tx.rlp_encode());

    let arg = SignWithEcdsaArgument {
        message_hash: message_hash.to_vec(),
        derivation_path: vec![b"ens-control".to_vec()],
        key_id,
    };

    let (response,) = sign_with_ecdsa(arg).await
        .map_err(|e| format!("ECDSA signing failed: {:?}", e))?;

    Ok(response.signature)
}
```

### **Advantages**

- ✅ **Fully on-chain governance**: ENS updates governed by same IC multi-sig as canister upgrades
- ✅ **No personal wallet dependency**: Board doesn't need personal Ethereum wallets
- ✅ **Same voting mechanism**: Board uses authors.cpf.nft to vote on ENS changes
- ✅ **Audit trail**: All ENS changes logged in IC governance proposals
- ✅ **Threshold security**: ENS controlled by 3-of-3 board approval (via threshold ECDSA)
- ✅ **No cross-chain custody**: Ethereum address derived deterministically from IC canister

### **Disadvantages**

- ⚠️ **Implementation complexity**: Need to build ENS management code on IC
- ⚠️ **Ethereum gas fees**: Each ENS update costs gas (paid from ICP-derived ETH address)
- ⚠️ **HTTPS outcalls**: Requires IC HTTPS outcalls to submit to Ethereum
- ⚠️ **Gas management**: Need to fund ICP-derived Ethereum address with ETH for gas
- ⚠️ **Limited ENS operations initially**: Start with basic DNS updates, add features over time

### **Gas Management Requirements**

**Critical Operational Detail:** The ICP-derived Ethereum address must be funded with ETH to pay for gas.

**Gas Cost Estimates:**

| Operation | Estimated Gas | Cost @ 30 gwei | Cost @ 100 gwei |
|-----------|---------------|----------------|-----------------|
| Update ENS DNS record | ~50,000 | $0.05 | $0.15 |
| Create subdomain | ~100,000 | $0.10 | $0.30 |
| Transfer ENS NFT | ~50,000 | $0.05 | $0.15 |
| Update text record | ~40,000 | $0.04 | $0.12 |

**Funding Strategy:**
- **Initial funding:** 0.5 ETH (~$1000 USD, sufficient for 500-1000 operations)
- **Monitoring:** Canister queries ETH balance via RPC, alerts when < 0.1 ETH
- **Refill threshold:** Refill when balance < 0.05 ETH
- **Refill responsibility:** Board members or designated operations staff
- **Refill amount:** 0.5 ETH per refill

**Who Can Fund?**
- Anyone can send ETH to the ICP-derived Ethereum address (public address)
- Typically board members' responsibility
- Can be tracked via governance proposals for transparency

**Monthly Cost Estimate:**
- Assuming 10 ENS updates per month @ 50 gwei average
- ~10 × $0.075 = **$0.75/month** (~$9/year)
- One-time 0.5 ETH funding lasts multiple years at this rate

**See [ens-dns-setup.md § Gas Management](./ens-dns-setup.md) for complete funding procedures.**

### **Ownership Evolution Path**

**Recommended approach balancing simplicity and decentralization:**

```
Phase 0: Personal Wallet (Bootstrap)
    ↓ Duration: Initial setup (days)
    ↓ Owner: Founder's personal Ethereum wallet
    ↓ Action: Purchase cpf.nft ENS NFT
    ↓ Rationale: Fastest way to acquire ENS domain
    ↓
Phase 1: ICP Chain-Key Control (Production)
    ↓ Duration: Production years 1-2
    ↓ Owner: ICP-derived Ethereum address (via governance canister)
    ↓ Action: Transfer ENS NFT to ICP-derived address
    ↓ Control: Board (3-of-3) via IC governance proposals
    ↓ Rationale: Decentralized control without building complex ENS code initially
    ↓
Phase 2: SNS Governance (Mature)
    ↓ Duration: Long-term
    ↓ Owner: SNS-controlled ICP-derived Ethereum address
    ↓ Control: Community token voting
    ↓ Rationale: Full decentralization
```

### **Implementation Recommendation**

**For CPP Platform:**

1. **Bootstrap (Phase 0):**
   - Purchase cpf.nft using personal Ethereum wallet
   - Configure DNS immediately after purchase (see [ens-dns-setup.md](./ens-dns-setup.md))
   - Create authors.cpf.nft subdomain
   - Manual DNS updates during development

2. **Production Launch (Phase 1):**
   - Deploy governance canister with chain-key ECDSA
   - Derive Ethereum address from governance canister
   - Transfer cpf.nft ENS NFT to ICP-derived address
   - Fund ICP-derived address with ~0.5 ETH for gas
   - Board controls ENS via governance proposals (3-of-3)

3. **Mature Platform (Phase 2):**
   - Migrate to SNS governance
   - ENS control transitions to community token voting
   - Same chain-key mechanism, different approval process

### **When to Build ENS Management Code**

**Not Immediately:**
- Start with personal wallet ownership (Phase 0)
- Manual DNS configuration during development
- Focus on core platform features

**Before Production Launch:**
- Build basic ENS update functionality (DNS records only)
- Test on testnet with Goerli ENS
- Implement gas management and monitoring

**After Platform Stabilizes:**
- Add advanced ENS features (subdomains, text records)
- Build UI for board to submit ENS proposals
- Add ENS monitoring and alerts

### **Integration with Governance Options**

This ENS control mechanism works with **any** of the governance options:

| Governance Option | ENS Control Method |
|-------------------|-------------------|
| **Gnosis Safe** | ❌ Not recommended - adds cross-chain complexity on top of existing cross-chain approach |
| **NNS-Based (Rust + Motoko)** | ✅ **Best fit** - Add ENS proposals to existing proposal types |
| **Custom Multi-Sig** | ✅ Works - Add ENS action type to custom multi-sig |
| **SNS** | ✅ Natural fit - ENS proposals alongside canister upgrade proposals |
| **Hybrid** | ✅ **Recommended** - Start Phase 0 (personal), migrate to Phase 1 (chain-key) at production |

### **See Also**

- [ens-dns-setup.md](./ens-dns-setup.md) - Complete ENS/DNS bootstrap guide with chain-key control details
- [governance-policy.md](./governance-policy.md) - ENS actions requiring board approval (Tier 0)
- [canister-architecture-diagram.md](./canister-architecture-diagram.md) - Board ENS control workflow and checkpoints

---

## Comparison Matrix

| Aspect | Gnosis Safe | NNS-Based (Rust+Motoko) | Custom Multi-Sig (Motoko) | SNS | Hybrid |
|--------|-------------|-------------------------|---------------------------|-----|--------|
| **Setup Complexity** | 🔴 High | 🟡 Medium | 🟢 Low | 🔴 Very High | 🟡 Medium |
| **Implementation Effort** | 🔴 High | 🟡 4-7 days | 🟢 2-3 days | 🔴 Weeks | 🟡 Progressive |
| **Upgrade Friction** | 🔴 Very High | 🟢 Low | 🟢 Low | 🔴 Very High | 🟢 Adaptive |
| **Cost per Action** | 🔴 $15-150 | 🟢 Negligible | 🟢 Negligible | 🟢 Negligible | 🟢 Negligible |
| **External Dependencies** | 🔴 Ethereum + Bridge | 🟢 None | 🟢 None | 🟢 None | 🟢 None |
| **Decentralization** | 🟡 Medium | 🟡 Medium | 🟡 Medium | 🟢 Full | 🟢 Evolves |
| **Security** | 🟢 Battle-tested | 🟢 NNS execution + custom voting | 🟡 Custom impl | 🟢 NNS-proven | 🟢 Progressive |
| **Code Quality** | 🟢 Audited | 🟢 Rust (NNS) + Motoko (new) | ⚠️ Custom | 🟢 DFINITY | 🟢 Progressive |
| **Two-Level Governance** | ❌ No | ✅ Yes (Board + Tech) | ⚠️ Must build | ❌ No | ✅ Yes |
| **Languages** | Solidity + Bridge | Rust + Motoko | Motoko only | Rust | Progressive |
| **Speed** | 🔴 Days | 🟢 Minutes | 🟢 Minutes | 🔴 Days | 🟢 Adaptive |
| **Monitoring** | ❌ No | ✅ Can add | ✅ Can add | ⚠️ External | ✅ Built-in |
| **Suitable for Dev** | ❌ No | ✅ Yes | ✅ Yes | ❌ No | ✅ Yes |
| **Suitable for Prod** | ⚠️ Maybe | ✅ Yes | ✅ Yes | ⚠️ Mature only | ✅ Yes |
| **Community Governance** | ❌ No | ❌ No | ❌ No | ✅ Yes | ✅ Eventually |
| **Path to SNS** | ❌ No | ✅ Easy (same Rust base) | ⚠️ Rebuild | ✅ Native | ✅ Smooth |

---

## Recommendation for CPP Platform

### **Use Hybrid Approach with NNS-Based Multi-Sig (Rust + Motoko)**

**Rationale:**
1. **Current Stage:** CPP is in active development
2. **Team Size:** 1-2 technical people initially
3. **Need:** Security without killing velocity
4. **Requirement:** Two-level governance (board + technical)
5. **Future:** Smooth path to SNS (full decentralization)

**Why NNS-Based Rust + Motoko (Not Pure Custom):**
- ✅ **Battle-tested execution:** NNS Rust code governs billions in ICP
- ✅ **Simple voting:** Motoko voting is easy to write/audit (~300 lines)
- ✅ **Two-level governance:** Native support in voting canister
- ✅ **Clean architecture:** Rust (execution) + Motoko (voting) = clear separation
- ✅ **Flexible:** Easy to customize voting logic without touching execution engine
- ✅ **SNS upgrade path:** Can upgrade to full SNS (same Rust base)
- ✅ **Community familiar:** Everyone knows NNS proposal model
- ✅ **Reasonable effort:** 4-7 days vs weeks for full custom or SNS

**Why NOT Pure Motoko Custom:**
- ⚠️ Would need to reimplement all proposal execution logic
- ⚠️ Less battle-tested than NNS execution engine
- ⚠️ Harder to upgrade to SNS later (different codebase)

**Implementation Timeline:**

**Q1 2025: Stage 1 (Single Controller)**
- Preprod environment with single admin controller
- Rapid iteration and testing
- Beta testing with early users

**Q2 2025: Stage 2 (NNS-Based Multi-Sig - Rust + Motoko)**
- Production launch
- Deploy **governance canister** (Rust, adapted from NNS)
- Deploy **voting canister** (Motoko, custom M-of-N logic)
- **Board governance:** 3 board members (3-of-3 for strategic)
- **Technical governance:** 1-2 technical signers initially (evolves to 3-5)
- See [governance-policy.md](./governance-policy.md) for complete approval matrix

**2026-2027: Stage 2 Continued**
- Platform stabilization
- Build community
- Design token economics
- Add community advisors to technical signers

**2027+: Stage 3 (SNS Transition)**
- Upgrade governance canister to full SNS
- Token launch and distribution
- Community voting on all proposals
- Board becomes proposal submitters (like everyone else)
- Full decentralization achieved

---

## Implementation: Basic Multi-Sig Canister

### **Full Implementation Example**

```motoko
// multi_sig_controller.mo
import Array "mo:base/Array";
import Time "mo:base/Time";
import Principal "mo:base/Principal";
import Result "mo:base/Result";
import HashMap "mo:base/HashMap";
import Iter "mo:base/Iter";
import Management "canister:aaaaa-aa";

actor MultiSigController {
  type ProposalId = Nat;

  type ProposalStatus = {
    #Pending;
    #Executed;
    #Rejected;
    #Expired;
  };

  type Proposal = {
    id: ProposalId;
    proposer: Principal;
    description: Text;
    target: Principal;        // Canister to upgrade
    wasmModule: Blob;         // New WASM code
    approvals: [Principal];   // Who approved
    rejections: [Principal];  // Who rejected
    status: ProposalStatus;
    createdAt: Int;
    expiresAt: Int;          // Proposal expires after 7 days
  };

  // Configuration
  stable var signers: [Principal] = [];
  stable var threshold: Nat = 3;
  stable let proposalExpiry: Int = 7 * 24 * 60 * 60 * 1_000_000_000; // 7 days in nanoseconds

  // State
  stable var nextProposalId: ProposalId = 0;
  stable var proposalsArray: [(ProposalId, Proposal)] = [];
  let proposals = HashMap.fromIter<ProposalId, Proposal>(
    proposalsArray.vals(),
    10,
    Nat.equal,
    Hash.hash
  );

  // System upgrade hooks
  system func preupgrade() {
    proposalsArray := Iter.toArray(proposals.entries());
  };

  system func postupgrade() {
    proposalsArray := [];
  };

  // Initialize signers (controller-only, called once during deployment)
  public shared(msg) func initialize(
    initialSigners: [Principal],
    initialThreshold: Nat
  ): async Result.Result<(), Text> {
    if (not isController(msg.caller)) {
      return #err("Unauthorized: Only controller can initialize");
    };

    if (signers.size() > 0) {
      return #err("Already initialized");
    };

    if (initialThreshold > initialSigners.size()) {
      return #err("Threshold cannot exceed number of signers");
    };

    signers := initialSigners;
    threshold := initialThreshold;
    #ok(())
  };

  // Check if caller is a signer
  func isSigner(principal: Principal): Bool {
    Array.find(signers, func(p: Principal): Bool { p == principal }) != null
  };

  // Check if caller is canister controller (for admin functions)
  func isController(principal: Principal): Bool {
    // In production, check against actual controller
    // For now, simplified
    true
  };

  // Submit upgrade proposal
  public shared(msg) func proposeUpgrade(
    description: Text,
    target: Principal,
    wasmModule: Blob
  ): async Result.Result<ProposalId, Text> {
    if (not isSigner(msg.caller)) {
      return #err("Unauthorized: Only signers can propose");
    };

    let proposalId = nextProposalId;
    nextProposalId += 1;

    let now = Time.now();
    let proposal: Proposal = {
      id = proposalId;
      proposer = msg.caller;
      description = description;
      target = target;
      wasmModule = wasmModule;
      approvals = [msg.caller];  // Proposer auto-approves
      rejections = [];
      status = #Pending;
      createdAt = now;
      expiresAt = now + proposalExpiry;
    };

    proposals.put(proposalId, proposal);
    #ok(proposalId)
  };

  // Approve proposal
  public shared(msg) func approve(proposalId: ProposalId): async Result.Result<(), Text> {
    if (not isSigner(msg.caller)) {
      return #err("Unauthorized: Only signers can approve");
    };

    let ?proposal = proposals.get(proposalId) else {
      return #err("Proposal not found");
    };

    if (proposal.status != #Pending) {
      return #err("Proposal not pending");
    };

    // Check expiry
    if (Time.now() > proposal.expiresAt) {
      proposals.put(proposalId, { proposal with status = #Expired });
      return #err("Proposal expired");
    };

    // Check if already approved
    if (Array.find(proposal.approvals, func(p: Principal): Bool { p == msg.caller }) != null) {
      return #err("Already approved");
    };

    // Add approval
    let newApprovals = Array.append(proposal.approvals, [msg.caller]);
    let updatedProposal = { proposal with approvals = newApprovals };
    proposals.put(proposalId, updatedProposal);

    // Check if threshold reached
    if (newApprovals.size() >= threshold) {
      // Execute upgrade
      try {
        await executeUpgrade(updatedProposal);
        proposals.put(proposalId, { updatedProposal with status = #Executed });
      } catch (error) {
        return #err("Upgrade failed: " # Error.message(error));
      };
    };

    #ok(())
  };

  // Reject proposal
  public shared(msg) func reject(proposalId: ProposalId): async Result.Result<(), Text> {
    if (not isSigner(msg.caller)) {
      return #err("Unauthorized: Only signers can reject");
    };

    let ?proposal = proposals.get(proposalId) else {
      return #err("Proposal not found");
    };

    if (proposal.status != #Pending) {
      return #err("Proposal not pending");
    };

    // Add rejection
    let newRejections = Array.append(proposal.rejections, [msg.caller]);
    let updatedProposal = { proposal with rejections = newRejections };

    // If rejections >= (signers - threshold + 1), proposal cannot pass
    let vetoThreshold = signers.size() - threshold + 1;
    if (newRejections.size() >= vetoThreshold) {
      proposals.put(proposalId, { updatedProposal with status = #Rejected });
    } else {
      proposals.put(proposalId, updatedProposal);
    };

    #ok(())
  };

  // Execute upgrade (internal)
  func executeUpgrade(proposal: Proposal): async () {
    await Management.install_code({
      mode = #upgrade;
      canister_id = proposal.target;
      wasm_module = proposal.wasmModule;
      arg = [];
    });
  };

  // Query proposal
  public query func getProposal(proposalId: ProposalId): async Result.Result<Proposal, Text> {
    let ?proposal = proposals.get(proposalId) else {
      return #err("Proposal not found");
    };
    #ok(proposal)
  };

  // List all proposals
  public query func listProposals(): async [Proposal] {
    Iter.toArray(proposals.vals())
  };

  // Get configuration
  public query func getConfig(): async { signers: [Principal]; threshold: Nat } {
    { signers = signers; threshold = threshold }
  };
};
```

### **Monitoring & Operations Management**

**Inspired by Juno's monitoring model**, the multi-sig canister should also handle:

#### **Canister Registry**

```motoko
// Track all controlled canisters
type CanisterInfo = {
  canisterId: Principal;
  name: Text;                    // Human-readable name
  repo: Text;                    // Source repository
  cycleBalance: Nat;             // Current cycles
  cycleThreshold: Nat;           // Alert threshold
  status: CanisterStatus;        // Running, Stopped, Stopping
  lastUpgrade: ?Nat64;           // Timestamp of last upgrade
  controllers: [Principal];      // Should only be multi-sig
  memorySize: Nat;               // Current memory usage
  wasmHash: ?Blob;               // Current WASM module hash
};

stable var managedCanisters: HashMap.HashMap<Principal, CanisterInfo> = HashMap.HashMap(10, Principal.equal, Principal.hash);

// Add canister to registry
public shared(msg) func registerCanister(
  canisterId: Principal,
  name: Text,
  repo: Text,
  cycleThreshold: Nat
): async Result<(), Text> {
  if (not isSigner(msg.caller)) {
    return #err("Unauthorized");
  };

  let info: CanisterInfo = {
    canisterId = canisterId;
    name = name;
    repo = repo;
    cycleBalance = 0;  // Will be updated by monitoring
    cycleThreshold = cycleThreshold;
    status = #Running;
    lastUpgrade = null;
    controllers = [];
    memorySize = 0;
    wasmHash = null;
  };

  managedCanisters.put(canisterId, info);
  #ok(())
};
```

#### **Automated Monitoring (Heartbeat)**

```motoko
// Runs every ~1 minute
system func heartbeat(): async () {
  // Check all managed canisters
  for ((canisterId, info) in managedCanisters.entries()) {
    try {
      // Query canister status
      let status = await Management.canister_status({ canister_id = canisterId });

      // Update registry
      let updatedInfo = {
        info with
        cycleBalance = status.cycles;
        status = status.status;
        memorySize = status.memory_size;
        controllers = status.settings.controllers;
      };

      managedCanisters.put(canisterId, updatedInfo);

      // Check for alerts
      if (status.cycles < info.cycleThreshold) {
        // Log alert
        await logAlert({
          severity = #Critical;
          canisterId = canisterId;
          message = "Low cycles: " # Nat.toText(status.cycles);
          timestamp = Time.now();
        });

        // Auto top-up if enabled
        if (autoTopUpEnabled) {
          await topUpCanister(canisterId, autoTopUpAmount);
        };
      };

      // Check if controller is still multi-sig
      if (not Array.find(status.settings.controllers, func(c: Principal): Bool { c == Principal.fromActor(MultiSigController) })) {
        await logAlert({
          severity = #Critical;
          canisterId = canisterId;
          message = "Controller changed! Multi-sig is no longer controller";
          timestamp = Time.now();
        });
      };

    } catch (error) {
      // Log error
      await logAlert({
        severity = #Error;
        canisterId = canisterId;
        message = "Failed to query status: " # Error.message(error);
        timestamp = Time.now();
      });
    };
  };
};
```

#### **Alert System**

```motoko
type AlertSeverity = {
  #Critical;
  #Warning;
  #Info;
  #Error;
};

type Alert = {
  id: Nat;
  severity: AlertSeverity;
  canisterId: ?Principal;
  message: Text;
  timestamp: Int;
  acknowledged: Bool;
  acknowledgedBy: ?Principal;
};

stable var alerts: [Alert] = [];
stable var nextAlertId: Nat = 0;

// Log alert
func logAlert(alert: {
  severity: AlertSeverity;
  canisterId: ?Principal;
  message: Text;
  timestamp: Int;
}): async () {
  let newAlert: Alert = {
    id = nextAlertId;
    severity = alert.severity;
    canisterId = alert.canisterId;
    message = alert.message;
    timestamp = alert.timestamp;
    acknowledged = false;
    acknowledgedBy = null;
  };

  nextAlertId += 1;
  alerts := Array.append(alerts, [newAlert]);

  // TODO: Send notifications (email, Slack, etc.)
};

// Query alerts
public query func getAlerts(acknowledged: ?Bool): async [Alert] {
  switch (acknowledged) {
    case null { alerts };  // All alerts
    case (?ack) {
      Array.filter(alerts, func(a: Alert): Bool { a.acknowledged == ack })
    };
  }
};

// Acknowledge alert
public shared(msg) func acknowledgeAlert(alertId: Nat): async Result<(), Text> {
  if (not isSigner(msg.caller)) {
    return #err("Unauthorized");
  };

  let ?alert = Array.find(alerts, func(a: Alert): Bool { a.id == alertId }) else {
    return #err("Alert not found");
  };

  let updatedAlert = {
    alert with
    acknowledged = true;
    acknowledgedBy = ?msg.caller;
  };

  // Update in array (simplified - in production use HashMap)
  alerts := Array.map(alerts, func(a: Alert): Alert {
    if (a.id == alertId) { updatedAlert } else { a }
  });

  #ok(())
};
```

#### **Automated Cycle Management**

```motoko
stable var autoTopUpEnabled: Bool = true;
stable var autoTopUpAmount: Nat = 5_000_000_000_000;  // 5T cycles
stable var autoTopUpThreshold: Nat = 1_000_000_000_000;  // 1T cycles

// Configure auto top-up (requires 3-of-5 approval)
public shared(msg) func configureAutoTopUp(
  enabled: Bool,
  amount: Nat,
  threshold: Nat
): async Result<(), Text> {
  // This should go through proposal system for critical changes
  autoTopUpEnabled := enabled;
  autoTopUpAmount := amount;
  autoTopUpThreshold := threshold;
  #ok(())
};

// Top up canister
func topUpCanister(canisterId: Principal, amount: Nat): async () {
  await Management.deposit_cycles({
    canister_id = canisterId;
    cycles = amount;
  });

  await logAlert({
    severity = #Info;
    canisterId = ?canisterId;
    message = "Auto top-up: " # Nat.toText(amount) # " cycles";
    timestamp = Time.now();
  });
};
```

#### **Dashboard Queries**

```motoko
// Get overview of all managed canisters
public query func getCanistersOverview(): async [{
  canisterId: Principal;
  name: Text;
  repo: Text;
  cycleBalance: Nat;
  cycleHealth: Text;  // "Healthy", "Warning", "Critical"
  status: CanisterStatus;
  memoryUsage: Nat;
  lastUpgrade: ?Nat64;
}] {
  Array.map(Iter.toArray(managedCanisters.vals()), func(info: CanisterInfo): {
    canisterId: Principal;
    name: Text;
    repo: Text;
    cycleBalance: Nat;
    cycleHealth: Text;
    status: CanisterStatus;
    memoryUsage: Nat;
    lastUpgrade: ?Nat64;
  } {
    let cycleHealth = if (info.cycleBalance >= info.cycleThreshold * 3) {
      "Healthy"
    } else if (info.cycleBalance >= info.cycleThreshold) {
      "Warning"
    } else {
      "Critical"
    };

    {
      canisterId = info.canisterId;
      name = info.name;
      repo = info.repo;
      cycleBalance = info.cycleBalance;
      cycleHealth = cycleHealth;
      status = info.status;
      memoryUsage = info.memorySize;
      lastUpgrade = info.lastUpgrade;
    }
  })
};

// Get specific canister details
public query func getCanisterDetails(canisterId: Principal): async Result<CanisterInfo, Text> {
  let ?info = managedCanisters.get(canisterId) else {
    return #err("Canister not found in registry");
  };
  #ok(info)
};
```

---

### **Usage Example**

```bash
# 1. Deploy multi-sig controller
dfx deploy multi_sig_controller

# 2. Initialize with signers
dfx canister call multi_sig_controller initialize '(
  vec {
    principal "aaaaa-aaaaa-aaaaa-aaaaa-cai";
    principal "bbbbb-bbbbb-bbbbb-bbbbb-cai";
    principal "ccccc-ccccc-ccccc-ccccc-cai";
    principal "ddddd-ddddd-ddddd-ddddd-cai";
    principal "eeeee-eeeee-eeeee-eeeee-cai";
  },
  3
)'

# 3. Register canisters to monitor
dfx canister call multi_sig_controller registerCanister '(
  principal "backend-canister-id",
  "Backend Canister",
  "cpf_members",
  1_000_000_000_000  // 1T cycle threshold
)'

# 4. Set multi-sig as controller
dfx canister update-settings backend_canister \
  --add-controller $(dfx canister id multi_sig_controller)

# 5. Configure auto top-up
dfx canister call multi_sig_controller configureAutoTopUp '(
  true,
  5_000_000_000_000,  // 5T cycles per top-up
  1_000_000_000_000   // Alert when below 1T
)'

# 6. Monitor status
dfx canister call multi_sig_controller getCanistersOverview

# 7. Check alerts
dfx canister call multi_sig_controller getAlerts '(opt false)'  // Unacknowledged alerts

# 8. Propose upgrade (as signer #1)
dfx canister call multi_sig_controller proposeUpgrade '(
  "Upgrade to v2.0 - Bug fixes",
  principal "backend-canister-id",
  blob "$(cat backend.wasm)"
)'
# Returns: (ok 0)  -- Proposal ID

# 9. Approve (as signer #2)
dfx canister call multi_sig_controller approve '(0)'

# 10. Approve (as signer #3)
dfx canister call multi_sig_controller approve '(0)'
# Upgrade executes automatically after 3rd approval

# 11. Verify upgrade in registry
dfx canister call multi_sig_controller getCanisterDetails '(principal "backend-canister-id")'
```

---

## Complete List of Multi-Sig Actions

### **For CPP Platform Implementation**

When deploying the multi-sig controller for CPP, these are ALL the actions it will manage:

#### **Tier 1: Critical Actions (3-of-5 Required)**

1. **Upgrade canister** - Deploy new WASM to any controlled canister
2. **Add/remove controller** - Change canister controller settings
3. **Change multi-sig threshold** - Modify M-of-N requirement (e.g., 3-of-5 → 4-of-6)
4. **Add/remove signer** - Change who can approve proposals
5. **Delete canister** - Permanently destroy a canister
6. **Transfer large cycles** - Transfer >10T cycles between canisters
7. **Change stable memory** - Direct modification of stable variables (emergency only)
8. **Enable/disable canister** - Stop or start a canister
9. **Change canister settings** - Modify memory allocation, compute allocation, freezing threshold
10. **Register new canister** - Add a new canister to the managed registry

#### **Tier 2: Admin Actions (2-of-5 or Delegated Admin)**

1. **Top up cycles (routine)** - Add <10T cycles to canisters
2. **Update application config** - Change application-level parameters (delegated to app admins)
3. **Emergency pause** - Temporarily stop service (2-of-5 for faster response)
4. **Blacklist address** - Block malicious user (delegated to app admins)
5. **Adjust rate limits** - Change API rate limits (delegated to app admins)
6. **Configure monitoring** - Change alert thresholds, auto top-up settings
7. **Acknowledge alerts** - Mark alerts as reviewed (any signer)

#### **Tier 3: Operational Actions (Automated)**

1. **Monitor cycles** - Automatic heartbeat checks every minute
2. **Monitor canister status** - Health checks, memory usage, controller verification
3. **Alert on low cycles** - Automatic alerts when balance < threshold
4. **Auto cycle top-up** - Automatic top-up when balance < threshold (if enabled)
5. **Metrics collection** - Gather usage statistics
6. **Log rotation** - Manage audit logs
7. **Update canister registry** - Update cached canister status from monitoring

#### **Tier 4: Emergency Actions (1-of-5 or Automated)**

1. **Emergency stop** - Any single signer can immediately pause
2. **Circuit breaker** - Automated stop on anomaly detection
3. **Critical alert** - Immediate notification to all signers

---

### **What the Multi-Sig Controller Manages**

**For CPP Platform:**

```
Multi-Sig Controller Canister
│
├─ Controlled Canisters (ALL production canisters)
│   ├─ cpf_members/backend_canister
│   ├─ cpf_members/payment_bridge
│   ├─ cpf_members/admin_dashboard
│   ├─ cpf_members/pwa_canister
│   ├─ fti_newsletter_archive/backend_api
│   ├─ fti_newsletter_archive/frontend
│   ├─ cpf_org/frontend_assets
│   └─ cpp_icp_platform/[future canisters]
│
├─ Monitoring (Automated via heartbeat)
│   ├─ Cycle balance tracking
│   ├─ Canister status checks
│   ├─ Memory usage monitoring
│   ├─ Controller verification
│   └─ Alert generation
│
├─ Operations (Automated or Admin)
│   ├─ Auto cycle top-ups
│   ├─ Alert acknowledgment
│   ├─ Metrics collection
│   └─ Audit log management
│
└─ Governance (Multi-Sig Approval Required)
    ├─ Canister upgrades (3-of-5)
    ├─ Controller changes (3-of-5)
    ├─ Signer management (3-of-5)
    ├─ Threshold changes (3-of-5)
    └─ Emergency actions (1-of-5)
```

---

## Summary & Decision Matrix

### **Choose Based on Stage:**

| Stage | Solution | Friction | When |
|-------|----------|----------|------|
| **Development** | Single Controller | None | Active development, frequent changes |
| **Production (Early)** | Basic Multi-Sig (3-of-5) | Low | Launch to stabilization (1-2 years) |
| **Production (Mature)** | SNS Governance | High | Stable platform, active community |

### **Don't Use Gnosis Safe Because:**
1. ❌ Cross-chain dependency introduces complexity and risk
2. ❌ High friction (days + gas fees) kills iteration speed
3. ❌ External dependencies (Ethereum, Chain Fusion bridge)
4. ❌ IC-native solutions are simpler and better suited

### **CPP Platform Recommendation:**

**Now → Production Launch:**
- Single controller for preprod/testing

**Production Launch → Year 2:**
- Deploy basic multi-sig canister (3-of-5)
- Low friction, high security
- Suitable for regular updates

**Year 2+:**
- Evaluate SNS transition when:
  - Platform is stable
  - Community is active
  - Token economics designed
  - Slower iteration is acceptable

---

---

## Inspiration: Juno's Monitoring Model

**Juno** (a Web3 platform on IC) implements excellent monitoring for their canisters:

**What Juno Does:**
- ✅ Dashboard showing all canister statuses
- ✅ Cycle balance monitoring with alerts
- ✅ Automated cycle top-ups
- ✅ Memory usage tracking
- ✅ Controller verification
- ✅ Health checks

**What We're Adding (CPP Enhancement):**
- ✅ **Multi-sig governance** for all controlled canisters
- ✅ **Tiered approval** (3-of-5, 2-of-5, 1-of-5, automated)
- ✅ **Proposal system** for upgrades and critical actions
- ✅ **Alert acknowledgment** by signers
- ✅ **Audit trail** of all governance actions
- ✅ **Emergency actions** (circuit breaker, emergency stop)

**Key Difference:**
- Juno: Monitoring + operations management
- CPP: Monitoring + operations + **multi-sig governance**

The multi-sig canister serves **three roles**:
1. **Controller**: Sets itself as controller for all production canisters
2. **Monitor**: Automated heartbeat checks canister health
3. **Governor**: Multi-sig approval for critical actions

---

**Last Updated:** 2025-11-13
**Version:** 3.0.0 (Added hybrid Rust/Motoko architecture for NNS-based approach)
**Status:** Ready for Implementation
**Decision:** Hybrid NNS-based approach (Rust governance + Motoko voting) with staged evolution

**Key Architecture Decision:**
- **Governance Canister (Rust):** Adapted from NNS, handles proposal execution
- **Voting Canister (Motoko):** Custom M-of-N voting, two-level governance
- **Interface:** Clean separation via cross-canister calls
- **Estimated effort:** 4-7 days for experienced IC developer

**Cross-References:**
- [governance-policy.md](./governance-policy.md) - Who approves what (policy)
- [admin-architecture.md](./admin-architecture.md) - Admin bootstrap
- [canister-bootstrap-pattern.md](./canister-bootstrap-pattern.md) - Security patterns

**Implementation Priority:**
1. ✅ **Now**: Document architecture (this file)
2. 🚧 **Next**: Adapt NNS Rust governance (strip out neurons)
3. 🚧 **Next**: Implement Motoko voting canister (~300 lines)
4. ⏳ **Future**: Add Juno-style monitoring and automated operations
5. ⏳ **Long-term**: Transition to full SNS when mature
