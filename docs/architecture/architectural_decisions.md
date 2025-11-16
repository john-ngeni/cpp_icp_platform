# Architectural Decision Log

**Date:** 2025-11-14
**Purpose:** Track key architectural decisions and open questions for CPP platform
**Status:** Living document - decisions and questions evolve as platform develops

## Document Structure

This document tracks:
1. **Decided**: Architectural decisions that have been made with rationale
2. **Open Questions**: Decisions still under consideration
3. **Deferred**: Questions postponed for later stages

---

## Decided Architectural Decisions

### AD-001: Derivation Origins - Control Plane vs Data Plane Separation

**Status:** ✅ **DECIDED** (2025-11-13, updated 2025-11-14)

**Context:**
CPP platform has two distinct operational planes:
- **Control Plane:** Infrastructure, governance, canister management, financial decisions (THIS REPO's focus)
- **Data Plane:** User-facing operations (newsletters, content, donations, NFTs)

Internet Identity derivation origins enable separating these planes while allowing authors to participate in both.

**Problem:**
- Single domain (cpf.nft): Simpler, but control plane and data plane share identity space
- Separate domains: Better separation, authors need dual identities to participate in both planes

**Options Considered:**

| Option | Description | Pros | Cons |
|--------|-------------|------|------|
| **1. Single domain (cpf.nft)** | All identities from cpf.nft, RBAC in canisters | Simple, single DNS setup | Control plane identities exposed to data plane, phishing risk |
| **2. Fully separate** | Control, operations, data planes on separate domains | Maximum isolation | Complex, many subdomains |
| **3. Control/Data Split (CHOSEN)** | governance.cpf.nft (control), cpf.nft (data) | Clean plane separation, authors dual identity | Authors need two II principals |
| **4. Path-based routing** | All on cpf.nft, use /control, /data paths | Single domain | Doesn't work with derivation origins |

**Decision:**
**Control plane and data plane separation with author dual identities:**

**Control Plane (governance.cpf.nft) - THIS REPO's PRIMARY FOCUS:**
- **Purpose:** Infrastructure management, governance, strategic operations
- **Who:** Board members, operations staff
- **Operations:** Canister upgrades, ENS management, financial decisions, governance voting, cycle management
- **Scope:** cpp_icp_platform repository (this repo)

**Data Plane (cpf.nft) - FACILITATED BY CONTROL PLANE:**
- **Purpose:** User-facing operations, content, donations, NFTs
- **Who:** All users (visitors, members) + authors (in content creation role)
- **Operations:** Newsletter creation, content moderation, donations, NFT minting, member management
- **Scope:** cpf_org, fti_newsletter_archive, cpf_members repositories

**Author Dual Identity Pattern:**
Authors have **two Internet Identity principals** to participate in both planes:
1. **governance.cpf.nft principal:** For control plane (governance, infrastructure)
2. **cpf.nft principal:** For data plane (creating newsletters, moderating content)

Example: Board member Alice logs into governance.cpf.nft to vote on canister upgrades, then logs into cpf.nft to create newsletter articles.

**Evolution path:**
- **Phase 1 (Bootstrap):** All use cpf.nft (authorization-based)
- **Phase 2 (Production):** Authors use governance.cpf.nft, public/members use cpf.nft
- **Phase 3 (Future):** May add additional subdomains if needed

**Rationale:**
1. **Plane separation:** Control plane (governance, infrastructure) isolated from data plane (operations)
2. **Security:** Governance identities never exposed to user-facing data plane services
3. **Semantic clarity:** "Authors" describes platform creators who operate both planes
4. **Audit clarity:** Control plane actions clearly separated in governance logs
5. **Repo focus:** This repo (cpp_icp_platform) focuses on control plane infrastructure
6. **Flexibility:** Authors can participate in both planes with appropriate identities

**Consequences:**
- ✅ Clear control/data plane separation
- ✅ Governance identities isolated from data plane services
- ✅ This repo's scope clearly defined (control plane)
- ✅ Authors can operate in both planes
- ⚠️ Authors need two Internet Identity principals (one per plane)
- ⚠️ Need to manage two DNS configurations (cpf.nft + governance.cpf.nft)
- ⚠️ Cross-plane operations need inter-canister calls

**References:**
- [canister-architecture-diagram.md § Derivation Origins](./canister-architecture-diagram.md)
- [ens-dns-setup.md § Phase 1](./ens-dns-setup.md)
- [governance-policy.md § Governance Identity Architecture](./governance-policy.md)
- [admin-architecture.md § Three-Level Identity Hierarchy](./admin-architecture.md)

---

### AD-002: Governance Center - ICP-Native vs Ethereum-Centric

**Status:** ✅ **DECIDED** (2025-11-13)

**Context:**
CPP platform runs on IC, but cpf.nft ENS NFT lives on Ethereum. Where should governance decisions be made?

**Problem:**
- Ethereum-centric (Gnosis Safe): Familiar, battle-tested, but adds cross-chain complexity
- ICP-centric (IC multi-sig): IC-native, but need to control Ethereum assets

**Options Considered:**

| Option | Governance Location | ENS Control | Pros | Cons |
|--------|-------------------|-------------|------|------|
| **1. Gnosis Safe (Ethereum)** | Ethereum multi-sig → Chain Fusion → IC | Direct Ethereum ownership | Battle-tested, familiar | High friction, gas fees, cross-chain dependency |
| **2. IC-Native Multi-Sig (CHOSEN)** | IC governance → Chain-key ECDSA → Ethereum | ICP-derived ETH address | IC-native, low friction, same voting for all actions | Need to build ENS management code |
| **3. Hybrid** | IC for canisters, Gnosis for ENS | Split governance | Separate concerns | Complex, two governance systems |

**Decision:**
**ICP-centric governance with chain-key control of ENS:**
- All governance (canister upgrades, ENS updates, etc.) via IC multi-sig
- Board votes using governance.cpf.nft Internet Identity principals
- Governance canister uses chain-key ECDSA to derive Ethereum address
- ICP-derived Ethereum address owns cpf.nft ENS NFT
- Same 3-of-3 board approval for both IC and Ethereum actions

**Rationale:**
1. **Unified governance:** Single voting mechanism for all platform actions
2. **Low friction:** No Ethereum gas for voting, only for ENS execution
3. **IC-native:** Leverages IC capabilities (threshold ECDSA)
4. **Audit trail:** All decisions logged on-chain in IC governance
5. **No cross-chain bridge risk:** Direct chain-key signatures to Ethereum

**Consequences:**
- ✅ Single governance system to build and maintain
- ✅ Board uses same authentication for all actions
- ✅ No Gnosis Safe subscription or cross-chain bridge dependency
- ⚠️ Need to build ENS management code on IC (moderate effort)
- ⚠️ Need to fund ICP-derived Ethereum address with ETH for gas
- ⚠️ ENS updates have gas cost (paid from ICP-derived address)

**References:**
- [multi-sig-governance-comparison.md § Option 1 (Gnosis Safe - rejected)](./multi-sig-governance-comparison.md)
- [multi-sig-governance-comparison.md § Option 2 (NNS-Based - chosen)](./multi-sig-governance-comparison.md)
- [multi-sig-governance-comparison.md § ENS Control via IC Governance](./multi-sig-governance-comparison.md)
- [ens-dns-setup.md § Phase 2 (Transfer to ICP Governance)](./ens-dns-setup.md)

---

### AD-003: Bootstrap vs Production Governance - Staged Evolution

**Status:** ✅ **DECIDED** (2025-11-13)

**Context:**
During development, we need fast iteration. In production, we need security. When should we introduce multi-sig governance?

**Problem:**
- Multi-sig from day one: Secure, but kills development velocity
- Single controller forever: Fast, but insecure for production
- When to transition?

**Options Considered:**

| Stage | Controller | Upgrade Time | Security | Suitable For |
|-------|-----------|--------------|----------|--------------|
| **Stage 1: Single Controller** | Individual dfx identity | Instant | ⚠️ Single point of failure | Development, preprod testing |
| **Stage 2: Multi-Sig (CHOSEN for production)** | 3-of-5 multi-sig canister | Minutes | ✅ Distributed control | Production years 1-2 |
| **Stage 3: SNS** | Community governance | Days | ✅✅ Full decentralization | Mature platform |

**Decision:**
**Staged governance evolution:**

```
NOW (Development)
    ↓ Single dfx identity controller
    ↓ Purpose: Rapid iteration, testing, preprod
    ↓ Duration: Until production launch
    ↓
PRODUCTION LAUNCH (Q2 2025)
    ↓ Multi-sig canister (NNS-based Rust + Motoko voting)
    ↓ Board: 3-of-3 for strategic decisions
    ↓ Technical: 1-of-2 initially, evolves to 3-of-5
    ↓ Duration: 1-2 years of production operation
    ↓
MATURE PLATFORM (2027+)
    ↓ SNS governance (community token voting)
    ↓ Full decentralization
    ↓ Duration: Long-term
```

**Rationale:**
1. **Development velocity:** Don't slow down development with multi-sig
2. **Production security:** Multi-sig before public launch
3. **Governance maturity:** SNS when platform is stable and community is active
4. **Progressive decentralization:** Gradual transition reduces risk

**Consequences:**
- ✅ Fast development during build phase
- ✅ Security in place before production launch
- ✅ Clear upgrade path to full decentralization
- ⚠️ Need to plan governance canister deployment before launch
- ⚠️ Need to transfer control from dfx identity to multi-sig (one-time operation)

**References:**
- [multi-sig-governance-comparison.md § Option 4 (Hybrid Approach - recommended)](./multi-sig-governance-comparison.md)
- [multi-sig-governance-comparison.md § Implementation Plan](./multi-sig-governance-comparison.md)
- [canister-bootstrap-pattern.md](./canister-bootstrap-pattern.md)

---

### AD-004: ENS Ownership Evolution - Personal → ICP Governance → SNS

**Status:** ✅ **DECIDED** (2025-11-13)

**Context:**
cpf.nft ENS NFT needs to be owned by someone on Ethereum. Who owns it at different stages?

**Problem:**
- ICP governance from day one: Requires building ENS management code before launch
- Personal wallet forever: Centralized, not decentralized
- When to transfer ownership?

**Options Considered:**

| Option | Owner | Control | Pros | Cons |
|--------|-------|---------|------|------|
| **1. Personal wallet indefinitely** | Founder's Ethereum wallet | Manual DNS updates | Simple, no code needed | Centralized, single point of failure |
| **2. ICP governance immediately** | ICP-derived address | Governance proposals | Decentralized from start | Need to build ENS code before launch |
| **3. Gnosis Safe** | Ethereum multi-sig | 3-of-5 Ethereum signers | Familiar Ethereum tooling | Cross-chain governance complexity |
| **4. Evolution path (CHOSEN)** | Personal → ICP governance → SNS | Evolves with platform | Balance simplicity and decentralization | Multiple transitions |

**Decision:**
**Ownership evolution path:**

```
Phase 0: Personal Wallet (Bootstrap)
    ↓ Duration: Initial setup (days to weeks)
    ↓ Owner: Founder's personal Ethereum wallet
    ↓ Action: Purchase cpf.nft, configure initial DNS
    ↓ Rationale: Fastest way to acquire and configure ENS
    ↓
Phase 1: ICP Chain-Key Control (Production)
    ↓ Duration: Production years 1-2
    ↓ Owner: ICP-derived Ethereum address
    ↓ Control: Governance canister via chain-key ECDSA
    ↓ Action: Transfer ENS NFT, board controls via 3-of-3 proposals
    ↓ Rationale: Decentralized control, unified governance
    ↓
Phase 2: SNS Governance (Mature)
    ↓ Duration: Long-term
    ↓ Owner: SNS-controlled ICP-derived address
    ↓ Control: Community token voting
    ↓ Rationale: Full decentralization
```

**Rationale:**
1. **Bootstrap speed:** Don't delay launch building ENS management code
2. **Production security:** Transfer to ICP governance before significant user base
3. **Progressive decentralization:** Matches overall governance evolution
4. **Avoid complex Ethereum tooling:** No Gnosis Safe, no cross-chain bridge for governance

**Consequences:**
- ✅ Can acquire cpf.nft immediately without custom code
- ✅ Unified governance once ICP multi-sig is ready
- ✅ Clear path to full decentralization
- ⚠️ Need to build ENS management code before Phase 1 transition
- ⚠️ Two ownership transfers (personal → ICP, ICP → SNS)
- ⚠️ Founder holds centralized control during Phase 0 (mitigated by short duration)

**References:**
- [ens-dns-setup.md § ENS Ownership Evolution](./ens-dns-setup.md)
- [multi-sig-governance-comparison.md § ENS Control via IC Governance](./multi-sig-governance-comparison.md)
- [governance-policy.md § ENS NFT Ownership and Control](./governance-policy.md)

---

### AD-005: Bootstrap Order - Parent Domain First

**Status:** ✅ **DECIDED** (2025-11-13)

**Context:**
We need both cpf.nft (parent) and governance.cpf.nft (subdomain). Which to set up first?

**Problem:**
- governance.cpf.nft is for board/operations (more privileged)
- cpf.nft is for public/members (less privileged)
- Does governance subdomain need to exist before parent is configured?

**Options Considered:**

| Option | Order | Rationale | Issue |
|--------|-------|-----------|-------|
| **1. governance.cpf.nft first** | Subdomain before parent | Bootstrap governance before users | ❌ ENS requires parent domain to exist before creating subdomain |
| **2. cpf.nft first (CHOSEN)** | Parent then subdomain | Standard ENS workflow | ✅ Works with ENS architecture |
| **3. Simultaneous** | Configure both at once | Fastest | ❌ Not possible - ENS requires parent first |

**Decision:**
**Standard ENS order: cpf.nft first, then governance.cpf.nft subdomain**

Bootstrap sequence:
1. Acquire cpf.nft ENS NFT
2. Configure cpf.nft DNS → governance_canister (initially)
3. Create governance.cpf.nft subdomain using ENS Manager
4. Configure governance.cpf.nft DNS → governance_canister
5. Board members authenticate with governance.cpf.nft
6. Update cpf.nft DNS → user-facing canisters

**Rationale:**
1. **ENS requirement:** Cannot create subdomain without parent domain existing
2. **Standard workflow:** Follows normal ENS subdomain creation pattern
3. **Dark mode deployment:** Can configure cpf.nft to point to governance initially, then update to production canisters when ready

**Consequences:**
- ✅ Follows standard ENS patterns
- ✅ Parent domain exists throughout platform lifetime
- ✅ Can deploy infrastructure in "dark mode" (DNS not public yet)
- ⚠️ Need to update cpf.nft DNS twice (governance → production)

**References:**
- [ens-dns-setup.md § Phase 1 (Acquire cpf.nft)](./ens-dns-setup.md)
- [ens-dns-setup.md § Phase 2 (Create governance.cpf.nft subdomain)](./ens-dns-setup.md)
- [canister-architecture-diagram.md § Checkpoint 1 (Acquire cpf.nft)](./canister-architecture-diagram.md)

---

### AD-006: Identity Hierarchy - Three Levels (Control Plane Focus)

**Status:** ✅ **DECIDED** (2025-11-13, updated 2025-11-14)

**Context:**
Control plane (this repo) needs clear identity separation for infrastructure deployment, governance, and potentially data plane access.

**Problem:**
- Two levels (controller + users): Simple, but mixes infrastructure and governance concerns
- Three levels (controller + control plane + data plane): Cleaner separation with plane boundaries
- Four+ levels: Too granular for initial platform

**Options Considered:**

| Option | Levels | Use Cases | Pros | Cons |
|--------|--------|-----------|------|------|
| **1. Two-level** | Controller + All users | Controller deploys, everyone else uses services | Simple | Mixes infrastructure, control plane, and data plane |
| **2. Three-level (CHOSEN)** | Controller + Control Plane + Data Plane | Clear plane separation | Clean governance, control/data isolation | Authors need dual identity for both planes |
| **3. Four-level** | Controller + Board + Operations + Data Plane | Maximum granularity | Very fine-grained | Overkill for initial platform |

**Decision:**
**Three-level identity hierarchy aligned with plane separation:**

```
Level 1: Controller (dfx identity) - INFRASTRUCTURE
    ↓ Identity: dfx identity (non-II)
    ↓ Purpose: Infrastructure deployment, canister installation
    ↓ Plane: Neither (bootstrap only)
    ↓ Bootstrap: dfx deploy with controller identity
    ↓
Level 2: Authors (governance.cpf.nft) - CONTROL PLANE
    ↓ Identity: Internet Identity principals
    ↓ Derivation Origin: governance.cpf.nft
    ↓ Purpose: Governance, infrastructure management, strategic decisions
    ↓ Who: Board members, operations staff
    ↓ Operations: Canister upgrades, ENS management, financial decisions, cycle management
    ↓ Repo: cpp_icp_platform (THIS REPO)
    ↓ Bootstrap: initializeBoardMembers() after governance.cpf.nft DNS configured
    ↓
Level 3: Public/Members (cpf.nft) - DATA PLANE
    ↓ Identity: Internet Identity principals
    ↓ Derivation Origin: cpf.nft
    ↓ Purpose: User-facing operations, content, donations
    ↓ Who: All users (visitors, members) + authors (in content creation role)
    ↓ Operations: Newsletter creation, content moderation, donations, NFT minting
    ↓ Repos: cpf_org, fti_newsletter_archive, cpf_members
    ↓ Bootstrap: Available after cpf.nft DNS configured

Note: Authors exist in BOTH Level 2 and Level 3 with different II principals
```

**Rationale:**
1. **Plane separation:** Control plane (Level 2) isolated from data plane (Level 3)
2. **Bootstrap facilitation:** Controller sets up infrastructure, hands off to board
3. **Security isolation:** Control plane identities never exposed to data plane services
4. **Audit clarity:** Control plane actions separated in governance logs
5. **Repo focus:** This repo (cpp_icp_platform) manages control plane (Level 2)

**Consequences:**
- ✅ Clear separation: infrastructure, control plane, data plane
- ✅ Control plane identities isolated from data plane services
- ✅ This repo's scope clearly defined (control plane - Level 2)
- ✅ Authors can participate in both planes with different principals
- ⚠️ Authors need two Internet Identity principals (one per plane)
- ⚠️ Three different identity types to manage during bootstrap
- ⚠️ Need to plan handoff from controller to board

**References:**
- [admin-architecture.md § Three-Level Identity Hierarchy](./admin-architecture.md)
- [governance-policy.md § Governance Identity Architecture](./governance-policy.md)
- [canister-architecture-diagram.md § Derivation Origins](./canister-architecture-diagram.md)

---

### AD-007: Multi-Sig Implementation - NNS-Based Rust + Motoko

**Status:** ✅ **DECIDED** (2025-11-13)

**Context:**
For Stage 2 (production multi-sig), what implementation approach?

**Problem:**
- Pure Motoko: Need to build everything from scratch
- Pure Rust (SNS): Too heavy for initial production
- NNS-based: Reuse battle-tested code

**Options Considered:**

| Option | Languages | Effort | Maturity | SNS Path |
|--------|-----------|--------|----------|----------|
| **1. Pure Motoko custom** | Motoko only | 2-3 days | New code | Hard to upgrade |
| **2. NNS-Based (CHOSEN)** | Rust (execution) + Motoko (voting) | 4-7 days | Battle-tested NNS execution | Easy upgrade path |
| **3. Full SNS immediately** | Rust | Weeks | Very mature | Already there |
| **4. Gnosis Safe** | Solidity + Bridge | High | Very mature | No path to SNS |

**Decision:**
**Hybrid architecture: NNS-based Rust governance + Motoko voting**

```
Voting Canister (Motoko - NEW)
    ↓ Simple M-of-N voting (1 principal = 1 vote)
    ↓ Two-level: Board (3-of-3) + Technical (evolving)
    ↓ Returns: hasReachedThreshold() -> Bool
    ↓
Governance Canister (Rust - ADAPTED FROM NNS)
    ↓ Proposal submission & execution (battle-tested)
    ↓ Proposal types from NNS (UpgradeCanister, etc.)
    ↓ Automatic execution when approved
    ↓ ENS updates via chain-key ECDSA
    ↓
Production Canisters
```

**Rationale:**
1. **Battle-tested execution:** NNS proposal execution governs billions in ICP
2. **Simple voting:** Motoko voting is ~300 lines vs complex neuron logic
3. **Clean separation:** Execution (Rust) vs voting (Motoko) = clear interfaces
4. **SNS upgrade path:** Same Rust base as full SNS
5. **Two-level governance:** Easy to implement in Motoko voting canister

**Consequences:**
- ✅ Reuse battle-tested NNS execution engine
- ✅ Simple, auditable voting logic in Motoko
- ✅ Clear upgrade path to full SNS
- ⚠️ Two canisters (cross-canister call latency ~1s)
- ⚠️ Need both Rust and Motoko expertise
- ⚠️ 4-7 days implementation effort

**References:**
- [multi-sig-governance-comparison.md § Option 2 (NNS-Based - recommended)](./multi-sig-governance-comparison.md)
- [multi-sig-governance-comparison.md § Hybrid Architecture](./multi-sig-governance-comparison.md)

---

### AD-008: Board Voting Thresholds - Time-Based Escalation

**Status:** ✅ **DECIDED** (2025-11-14)

**Context:**
Board governance needs to balance security (prevent rogue actions) with availability (don't block on one absent member). Traditional multi-sig is binary: either unanimous or simple majority.

**Problem:**
- **3-of-3 only:** Maximum security, but any unavailability blocks decisions
- **2-of-3 only:** More flexible, but 2-person collusion can execute immediately
- **Need flexibility:** Fast unanimous decisions, but still allow action when someone unavailable

**Decision:**
**Time-based threshold escalation with 3 board members:**

| Threshold | Time Delay | Use Case | Security |
|-----------|-----------|----------|----------|
| **3-of-3 (unanimous)** | ✅ **Immediate** execution | Normal operations, everyone agrees | Maximum - all members approve |
| **2-of-3 (supermajority)** | ⏱️ **3-day delay** then execute | One member unavailable/unreachable | High - 3 days for dissent |
| **1-of-3 (single member)** | ⏱️ **1-week delay** then execute | Emergency, two members unavailable | Medium - 7 days for objection |

**How It Works:**

```
Proposal Created → First Vote Cast
    ↓
    ├─ If 3-of-3 within delay period → Execute immediately
    ├─ If 2-of-3 after 3 days → Execute (timer expires)
    └─ If 1-of-3 after 7 days → Execute (long timer expires)

At any time:
- Additional vote → May trigger immediate execution
- Rejection vote → Proposal fails (veto)
```

**Example Scenarios:**

1. **Normal (Everyone Available):** 3-of-3 → Immediate execution
2. **One Member on Vacation:** 2-of-3 → Wait 3 days → Execute
3. **Emergency (Two Unreachable):** 1-of-3 → Wait 7 days → Execute (or veto if others return)

**Rationale:**
1. **Fast when unanimous:** No delay for normal operations
2. **Resilient to unavailability:** Can proceed after delay if supermajority
3. **Emergency escape hatch:** Single member can act after 1 week
4. **Veto protection:** Time delay allows absent members to object
5. **Prevents collusion:** 2 members can't execute immediately (3-day buffer)
6. **Security gradient:** Shorter delay = more consensus required

**Consequences:**
- ✅ Fast unanimous decisions (no delay)
- ✅ Resilient to 1 member unavailable (3-day delay acceptable)
- ✅ Emergency action possible (1 member after 1 week)
- ✅ Time delay prevents hasty/malicious actions
- ⚠️ More complex implementation (need timer logic)
- ⚠️ Need monitoring for timer expirations
- ⚠️ Need notification system (alert members when timer started)

**Implementation:**

```rust
// In voting canister
pub fn execute_if_threshold_met(proposal: &mut Proposal) -> Result<(), String> {
    let vote_count = proposal.votes.len();
    let time_since_first_vote = ic_cdk::api::time() - proposal.first_vote_at.unwrap();
    let days = time_since_first_vote / (24 * 60 * 60 * 1_000_000_000);

    if vote_count >= 3 {
        execute_immediately(proposal)  // 3-of-3: immediate
    } else if vote_count >= 2 && days >= 3 {
        execute_with_delay(proposal, "2-of-3 after 3 days")
    } else if vote_count >= 1 && days >= 7 {
        execute_with_delay(proposal, "1-of-3 after 7 days")
    } else {
        Ok(())  // Not ready to execute
    }
}
```

**References:**
- [governance-policy.md § Tier 0 (Strategic & Financial)](./governance-policy.md)
- [multi-sig-governance-comparison.md § Voting Canister](./multi-sig-governance-comparison.md)
- Timelock patterns: Compound Governance, Gnosis Safe with timelock

---

## Open Questions (Under Consideration)

### OQ-001: Domain Routing Strategy - Path-Based vs Subdomain-Based

**Status:** 🤔 **OPEN** (as of 2025-11-14)

**Context:**
CPP platform has multiple services (cpf_org, fti_newsletter_archive, cpf_members) that need to share user identity. Two architectural approaches exist:

**Approach 1: Path-Based Routing (Single Domain)**
```
cpf.nft/                  → cpf_org (public site)
cpf.nft/newsletters/      → fti_newsletter_archive
cpf.nft/members/          → cpf_members
cpf.nft/community/        → cpp_icp_platform
```

**Approach 2: Subdomain-Based Routing (Alternative Origins)**
```
coolplanet-foundation.org           → cpf_org
newsletters.coolplanet-foundation.org → fti_newsletter_archive
members.coolplanet-foundation.org     → cpf_members
```

**Problem:**
Each approach has different tradeoffs for deployment, security, and user experience.

**Detailed Comparison:**

| Aspect | Path-Based (Single Domain) | Subdomain-Based (Alternative Origins) |
|--------|----------------------------|---------------------------------------|
| **Same-origin** | ✅ Same origin everywhere | ❌ Different origins (CORS required) |
| **Cookies/localStorage** | ✅ Shared automatically | ❌ Not shared across subdomains |
| **Derivation origin** | ✅ Not needed (same origin) | ⚠️ Must configure alternative origins |
| **Deployment** | ❌ Complex routing setup | ✅ Deploy canisters independently |
| **DNS management** | ✅ Single domain to manage | ❌ Multiple DNS configurations |
| **Independent updates** | ❌ Harder (shared routing) | ✅ Update services independently |
| **URL clarity** | ⚠️ Longer URLs (/newsletters/post/123) | ✅ Shorter URLs (newsletter.org/post/123) |
| **Security surface** | ✅ Single domain to secure | ❌ Multiple subdomains to secure |
| **IC routing** | ❌ Router canister or gateway config | ✅ Standard IC boundary node routing |

**Questions:**
- How important is same-origin for CPP? (shared state, no CORS)
- Can IC boundary nodes support path-based routing to different canisters?
- Does marketing prefer branded subdomains (newsletters.cpf.org) vs paths (cpf.org/newsletters)?
- How often will services be deployed independently?
- What's the operational overhead of managing alternative origins?

**Current State:**
- **Currently using Approach 2 (subdomain-based)** with alternative origins
- fti_newsletter_archive: newsletters.coolplanet-foundation.org
- cpf_members: members.coolplanet-foundation.org
- All services configured with `cpf.nft` as derivation origin
- Alternative origins file served from cpf.nft

**Implications:**

**If Path-Based (Approach 1):**
- ✅ Simplest authentication (no derivation origin needed)
- ✅ Shared cookies/localStorage
- ✅ No CORS configuration
- ❌ Need router canister or custom HTTP gateway
- ❌ All services must coordinate routing
- ❌ Harder to deploy services independently

**If Subdomain-Based (Approach 2 - current):**
- ✅ Independent canister deployment
- ✅ Standard IC boundary node routing
- ✅ Clear service separation
- ❌ Must configure alternative origins correctly
- ❌ CORS required for cross-service calls
- ❌ More DNS management overhead

**References:**
- [derivation-origins-integration.md § Domain Routing Strategies](./derivation-origins-integration.md)
- [derivation-origins-integration.md § Approach 1 (Path-Based)](./derivation-origins-integration.md)
- [derivation-origins-integration.md § Approach 2 (Alternative Origins)](./derivation-origins-integration.md)

**Next Steps:**
- Investigate IC boundary node path-based routing capabilities
- Assess router canister complexity and performance
- User testing: Do users care about URLs?
- Team assessment: How often do services need independent deployment?
- Security review: Attack surface comparison

---

### OQ-002: ENS Management Code - Build Now or Later?

**Status:** 🤔 **OPEN** (as of 2025-11-14)

**Context:**
We've decided on ICP governance controlling ENS via chain-key ECDSA (AD-004). But WHEN to build the ENS management code?

**Problem:**
- Building ENS management code is moderate effort (estimated 1-2 weeks)
- Not needed during Phase 0 (personal wallet ownership)
- Needed before Phase 1 transition (ICP governance)
- Could delay production launch if built too late

**Options:**

| Option | Timing | Pros | Cons |
|--------|--------|------|------|
| **1. Build now** | During development | Ready for production, can test early | May not need it for months, priorities? |
| **2. Build before Phase 1** | Just before transferring ENS to ICP | Only build when needed | Risk of delaying Phase 1 transition |
| **3. MVP now, iterate later** | Basic DNS updates now, advanced features later | Balance urgency and completeness | May need to refactor |

**Questions:**
- How long will Phase 0 (personal wallet) last? Weeks? Months?
- What's minimum viable ENS management? (DNS records only? Or subdomains too?)
- Can we launch production (Phase 1) with personal wallet ownership? (Security risk?)

**Dependencies:**
- Governance canister implementation (AD-007)
- Chain-key ECDSA integration
- Ethereum gas management (need to fund ICP-derived address)
- Testing on Goerli/Sepolia testnet

**Implications:**
- **Build now:** Delays other features, but ready for Phase 1
- **Build later:** Focus on core platform, but Phase 1 transition may be delayed
- **MVP approach:** Balanced, but may need refactoring

**References:**
- [multi-sig-governance-comparison.md § When to Build ENS Management Code](./multi-sig-governance-comparison.md)
- [ens-dns-setup.md § Phase 2 (Transfer to ICP Governance)](./ens-dns-setup.md)

**Next Steps:**
- Define minimum viable ENS management features
- Estimate development effort (is 1-2 weeks realistic?)
- Set deadline for Phase 0 → Phase 1 transition
- Prioritize against other platform features

---

### OQ-003: Technical Admin Threshold Evolution - When to Change?

**Status:** 🤔 **OPEN** (as of 2025-11-14)

**Context:**
Technical governance threshold should evolve as team grows:
- Phase 1: 1-of-2 (small team)
- Phase 2: 2-of-3 (growing team)
- Phase 3: 3-of-5 (mature team)

**Problem:**
When should threshold changes happen? What triggers the evolution?

**Options:**

| Trigger | Description | Pros | Cons |
|---------|-------------|------|------|
| **1. Manual threshold updates** | Board votes to change threshold | Explicit control | Need to remember to do it |
| **2. Automatic based on team size** | When technical signers reach N, threshold updates | No manual intervention | May change at wrong time |
| **3. Time-based** | After 6 months → 2-of-3, after 1 year → 3-of-5 | Predictable | Ignores actual team growth |
| **4. Milestone-based** | After X users, Y revenue, Z canisters | Tied to platform maturity | Hard to define milestones |

**Questions:**
- How fast will technical team grow? (1-2 → 3-5 in how many months?)
- Should threshold change be automatic or require board approval?
- What if team shrinks? (Someone leaves - lower threshold?)
- Should there be a maximum threshold? (Always allow 1-of-N for emergency?)

**Current State:**
- Voting canister designed to support evolving thresholds
- No specific trigger mechanism defined

**Implications:**
- **Manual:** Simple, but requires governance overhead
- **Automatic:** Convenient, but may surprise team
- **Hybrid:** Manual approval of threshold changes, but automatic suggestions

**References:**
- [multi-sig-governance-comparison.md § Voting Canister (getThreshold function)](./multi-sig-governance-comparison.md)
- [governance-policy.md § Technical Signers (Phase Evolution)](./governance-policy.md)

**Next Steps:**
- Define hiring plan (when will team reach 3? 5?)
- Decide on manual vs automatic threshold evolution
- Implement threshold evolution logic in voting canister
- Document threshold change governance process

---

### OQ-005: Emergency Stop Mechanism - Who Can Trigger?

**Status:** 🤔 **OPEN** (as of 2025-11-14)

**Context:**
Platform may need emergency stop capability (circuit breaker) for security incidents. Who should be able to trigger it?

**Problem:**
- Too restrictive (3-of-3): Takes too long in emergency
- Too permissive (any user): Can be abused for DoS
- Balance: Fast response vs preventing abuse

**Options:**

| Option | Who Can Stop | Approval | Time to Stop | Abuse Risk |
|--------|--------------|----------|--------------|------------|
| **1. Any board member (1-of-3)** | Any single board member | None | Seconds | Low (only 3 people) |
| **2. Any technical signer (1-of-N)** | Any technical admin | None | Seconds | Medium (more people) |
| **3. Board supermajority (2-of-3)** | 2 board members | 2-of-3 vote | Minutes | Very low |
| **4. Automated circuit breaker** | Canister logic | Programmatic | Milliseconds | False positives |
| **5. Tiered (1-of-3 board OR automated)** | Board or canister | Either | Seconds | Low |

**Questions:**
- What constitutes an "emergency"? (Security breach? Bug? DoS attack?)
- Should emergency stop require justification? (On-chain message?)
- How to restart after emergency stop? (Same threshold or higher?)
- Should there be tiered emergency responses? (Soft pause vs hard stop?)

**Related Considerations:**
- Emergency stop should log reason on-chain
- Should notify all board members immediately
- Need monitoring to detect false positives (automated circuit breaker)
- Recovery process needs to be documented

**Implications:**
- **1-of-N:** Fast response, but potential for accidental stops
- **M-of-N:** Safer, but slower in true emergency
- **Automated:** Fastest, but may have false positives
- **Tiered:** Flexible, but more complex

**References:**
- [multi-sig-governance-comparison.md § Tier 4 (Emergency Actions)](./multi-sig-governance-comparison.md)
- [governance-policy.md § Emergency Procedures](./governance-policy.md)

**Next Steps:**
- Define emergency scenarios (security, operational, financial)
- Design circuit breaker logic (what triggers automated stop?)
- Document emergency response playbook
- Implement emergency stop with reason logging
- Define restart procedure and approval threshold

---

### OQ-006: Cycle Management - Auto Top-Up or Manual?

**Status:** 🤔 **OPEN** (as of 2025-11-14)

**Context:**
IC canisters need cycles to run. Running out of cycles freezes the canister. Should cycle top-ups be automated or require approval?

**Problem:**
- Manual top-ups: More control, but risk of forgetting and freezing
- Automated top-ups: Convenient, but potential for cycle drain attacks
- Balance: Availability vs cost control

**Options:**

| Option | Trigger | Approval | Risk | Operational Overhead |
|--------|---------|----------|------|---------------------|
| **1. Fully automated** | Balance < threshold | None | Cycle drain attacks | None (set and forget) |
| **2. Automated with caps** | Balance < threshold, max per day | None (within caps) | Limited drain | Low (adjust caps) |
| **3. Alert only** | Balance < threshold | Technical admin | Canister freeze if delayed | High (need to monitor) |
| **4. Hybrid (auto + alerts)** | Auto top-up + alert board | None (auto), board reviews | Medium | Low |

**Questions:**
- What's appropriate cycle threshold? (1T? 5T? 10T cycles?)
- What's appropriate top-up amount? (5T? 10T? 20T cycles?)
- Should different canisters have different thresholds? (Critical vs non-critical?)
- What's maximum auto top-up per day/week to prevent drain?

**Attack Scenarios:**
- Malicious user triggers high computation to drain cycles
- Infinite loop bug causes rapid cycle burn
- Need daily/weekly caps to limit damage

**Current State:**
- Multi-sig implementation includes auto top-up capability
- Thresholds and caps not yet defined

**Implications:**
- **Fully automated:** Convenient but riskiest
- **Capped automation:** Good balance of availability and safety
- **Manual only:** Safest but highest operational burden
- **Hybrid:** Best of both, but most complex

**References:**
- [multi-sig-governance-comparison.md § Automated Cycle Management](./multi-sig-governance-comparison.md)
- [multi-sig-governance-comparison.md § Monitoring & Operations Management](./multi-sig-governance-comparison.md)

**Next Steps:**
- Analyze cycle burn rates for each canister type
- Define cycle threshold (when to alert/top-up)
- Define daily/weekly top-up caps
- Implement monitoring dashboard for cycle balances
- Set up alerts for abnormal cycle burn

---

### OQ-007: Blockchain Gas Management - Admin vs User Operations

**Status:** 🤔 **OPEN** (as of 2025-11-14)

**Context:**
CPP platform has blockchain gas costs on two different chains and for two different purposes:

1. **Admin domain management (Unstoppable Domains on Polygon)** - Minimal, one-time costs
2. **User NFT operations (NFT minting/transfers)** - Ongoing operational costs

**Important Clarifications:**
- **cpf.nft is Unstoppable Domains** (already reserved), NOT ENS
- **DNS control must be on Ethereum mainnet** - ICP chain-key ECDSA only supports Ethereum, not Polygon (yet)
- **NFT blockchain is separate decision** - Ethereum vs Polygon for NFT smart contracts (pending)
- **Admin gas costs are minimal** - One-time subdomain setup, rarely changed
- **Donations come through Stripe (fiat)** - NOT crypto donations, so gas requires fiat → crypto conversion
- **NFT gas requires ETH acquisition** - CPF must buy ETH/MATIC to fund NFT minting operations

**Problem:**
How to manage DNS control on Ethereum (for ICP chain-key compatibility) and **convert fiat donations into ETH/MATIC for NFT gas?**

**Treasury Management Challenge:**
- Donations arrive as **fiat (Stripe)** → Bank account
- NFT minting requires **ETH/MATIC (crypto)** → Blockchain gas
- Solution: **Thermostat model** - Algorithm advises Treasurer when to buy, how much (see [crypto_funding.md](./crypto_funding.md))

**Gas Cost Context:**

**Admin Operations (Domain DNS Control on Ethereum):**
- **Why Ethereum?** ICP threshold ECDSA only supports Ethereum mainnet secp256k1 curve
- **Subdomain creation:** ~$5-$20 per subdomain (one-time, Ethereum mainnet gas)
- **DNS updates:** ~$2-$10 per update (rare)
- **Annual cost:** < $50/year (after initial setup, assuming minimal changes)
- **Initial funding:** 0.1 ETH (~$200) sufficient for years of domain management

**User NFT Operations (Ethereum or Polygon - Decision Pending):**
- **NFT mint cost:** $0.50-$5.00 per NFT (depending on chain: Polygon cheap, Ethereum expensive)
- **Funding model:** **Fiat → Crypto Conversion** - Buy ETH/MATIC from donation revenue
- **Example:** $100 donation (fiat) → $2-5 allocated for gas → Convert to ETH when needed
- **Treasury decision:** When to buy ETH? How much? Monthly? Quarterly? When balance low?
- **Annual cost:** $0 net cost (gas covered by donations), but requires treasury management
- **Overhead:** ~2-5% of donation revenue allocated to gas reserve

**Admin Operations Funding Options:**

| Option | Funding Source | Monitoring | Cost | Pros | Cons |
|--------|---------------|------------|------|------|------|
| **1. Board member ETH** | Board sends ETH to ICP-derived address | Manual (low priority) | < $50/year | Simple, minimal cost | Requires board Ethereum wallet |
| **2. Personal wallet (Phase 0)** | Founder pays gas from personal wallet | None needed | < $50/year | Simplest bootstrap | Centralized during Phase 0 |

**User NFT Operations Funding Options:**

| Option | Funding Source | Who Pays | ETH Acquisition | Pros | Cons |
|--------|---------------|----------|-----------------|------|------|
| **1. Donation-funded treasury (RECOMMENDED)** | Fiat donations → Buy ETH periodically | Donor (indirectly via 2-5% overhead) | CPF buys ETH when reserve low | Sustainable, covers cost from revenue | Need treasury management process |
| **2. User pays gas separately** | User's own wallet | User | Not needed | No platform cost, no treasury management | Friction, requires user crypto wallet |
| **3. Hybrid** | Treasury OR user wallet | Donor OR User | CPF buys for sponsored mints only | Flexibility | Complex treasury + both mechanisms |
| **4. IC-native NFTs** | IC cycles | CPF (via cycles) | Not needed (no blockchain gas) | No ETH acquisition needed | Not ERC-721 standard |

**Questions:**

**Admin Operations (Low Priority):**
- ~~Should ETH funding be tracked via governance proposals?~~ Not needed, costs are minimal
- ~~Who is responsible for monitoring?~~ Board member checks quarterly
- **DNS on Ethereum mainnet:** Required for ICP chain-key compatibility (done)

**User NFT Operations (Medium Priority - Thermostat Model Operational):**
- **Which blockchain for NFT contracts?** Ethereum (expensive gas, ICP-compatible) vs Polygon (cheap gas, not ICP-compatible yet) - **Decision pending**
- **Thermostat algorithm details:** See [crypto_funding.md](./crypto_funding.md) for:
  - When to buy (weekly check, balance threshold)
  - How much to buy (target reserve - current balance)
  - Who authorizes (Treasurer operational authority up to €2k/month)
  - Projection adjustments (monthly actuals vs projected)
- **Exchange setup:** Which CEX? (Coinbase, Kraken) - Treasurer decides
- **Initial parameters:** Bootstrap thresholds (€500 min, €1000 target) - see crypto_funding.md

**Current Thinking:**

**Admin Operations:**
- **Phase 0 (Current):** Personal wallet pays Ethereum gas for Unstoppable Domains DNS management
- **Phase 1 (Production):** Board member funds ICP-derived Ethereum address with 0.1 ETH (~$200, lasts years)
- **Phase 2+:** No change needed, costs remain minimal

**User NFT Operations (Thermostat Model - Operational):**
- **Operational model:** Treasurer manages gas reserves using thermostat algorithm (see [crypto_funding.md](./crypto_funding.md))
  - **Weekly check:** Algorithm monitors balance, projects spend, advises Treasurer
  - **Purchase trigger:** When balance < min threshold OR runway < 3 months
  - **Purchase amount:** Enough to reach target reserve (typically €1,000)
  - **Treasurer authority:** Up to €2,000/month without board approval (Tier 3 operational)
  - **Board oversight:** Monthly review of crypto treasury report (informational)
- **Example flow:**
  - Week 1: Balance €1,234, runway 3.2 months → NO_ACTION
  - Week 2: Balance €987, runway 2.5 months → BUY_RECOMMENDED €1,013
  - Week 3: Treasurer wires €1,000 → Buys ETH → Sends to ICP address
  - Week 4: Balance €1,987, runway 5.1 months → NO_ACTION
  - Month end: Treasurer reports to board, adjusts projections based on actuals
- **Blockchain decision:** Pending (Ethereum for ICP compatibility vs Polygon for lower costs)

**Implementation Considerations:**

**Admin Operations Implementation (Minimal Cost):**
```bash
# Check Polygon balance for domain management
dfx canister call governance_canister getPolygonBalance --network ic

# When low (rare), board member sends MATIC
# Using MetaMask on Polygon network → ICP-derived Polygon address
# Amount: 0.5 MATIC (~$0.50, lasts years)
```

**User NFT Operations Implementation (Thermostat Model):**

See [crypto_funding.md](./crypto_funding.md) for full implementation details. Summary:

**Weekly Monitoring (Automated):**
```rust
// In governance canister - runs every Sunday
pub async fn crypto_treasury_weekly_check() {
    let recommendation = check_and_advise_crypto_purchase().await;

    if recommendation.action == "BUY_URGENT" || recommendation.action == "BUY_RECOMMENDED" {
        notify_treasurer(recommendation);  // Email/Slack notification
    }
}
```

**Treasurer Receives Notification:**
```
Subject: Crypto Reserve Weekly Report

Current ETH reserve: €987 (2.5 months runway)
Recommendation: BUY_RECOMMENDED
Amount: €1,013 to reach €2,000 target

Action: Purchase €1,000 ETH via Coinbase
```

**Treasurer Executes Purchase:**
```
1. Wire €1,000 from CPF bank to Coinbase
2. Wait for wire to clear (1-3 days)
3. Buy ETH at market price
4. Withdraw to ICP-derived address
5. Canister detects balance increase → Normal operations
```

**Implications:**

**Admin Operations (Low Impact):**
- ✅ Costs are trivial (< $10/year)
- ✅ One-time setup, rarely changed
- ✅ No need for complex funding mechanisms
- ⚠️ Board member needs Polygon wallet (MetaMask on Polygon network)

**User NFT Operations (Medium Impact - Thermostat Model Operational):**

**Fiat → Crypto Conversion (Thermostat-Advised Treasury):**
- ✅ Sustainable: Gas covered by donations (2-5% overhead)
- ✅ Smooth UX: Users don't need crypto wallets
- ✅ Inclusive: Crypto newbies can participate
- ✅ **Operational simplicity:** Algorithm advises Treasurer (like a thermostat), not board-level decisions
- ✅ **Predictable:** Weekly checks, clear triggers (balance < threshold OR runway < 3 months)
- ✅ **Self-adjusting:** Projections update monthly based on actuals (if spending more → buy more)
- ⚠️ **Exchange relationship needed:** CPF needs account at CEX (Coinbase, Kraken, etc.)
- ⚠️ **Wire transfer delays:** 1-3 days from bank → exchange → ETH purchase → withdrawal
- ⚠️ **Treasurer overhead:** Weekly check review (~5 min), monthly purchases (~30 min), monthly report (~1 hour)

**If Users Pay Gas (Alternative):**
- ✅ No operational cost or treasury management
- ✅ No ETH acquisition decisions needed
- ❌ User friction (need crypto wallet + ETH/MATIC)
- ❌ Excludes non-crypto users
- ❌ Poor UX for fiat-only donors

**If IC-Native NFTs (Alternative):**
- ✅ Lowest cost (IC cycles ~$1/month vs $100s/month gas)
- ✅ No ETH acquisition or treasury management
- ✅ Fully on-chain, ICP-native
- ❌ Not ERC-721 standard (different ecosystem)
- ❌ Lower liquidity/marketability
- ❌ Can't use Ethereum/Polygon NFT infrastructure

**References:**
- [crypto_funding.md](./crypto_funding.md) - **Thermostat algorithm for crypto treasury management**
- [ens-dns-setup.md § Unstoppable Domains Setup](./ens-dns-setup.md) - Domain management (needs update for Unstoppable Domains)
- [multi-sig-governance-comparison.md § Gas Management Requirements](./multi-sig-governance-comparison.md) - Cost estimates (needs update)
- [governance-policy.md § Tier 3 (Operational)](./governance-policy.md) - Treasurer authority levels

**Next Steps:**

**Admin Operations (Low Priority):**
- Update ens-dns-setup.md to reflect Unstoppable Domains instead of ENS
- Document Polygon address derivation (instead of Ethereum)
- Board member sets up Polygon wallet with 0.5 MATIC

**User NFT Operations (Medium Priority - Thermostat Implementation):**

**See [crypto_funding.md](./crypto_funding.md) for implementation checklist.** Key next steps:

1. **Blockchain Decision (Strategic - Board):**
   - Ethereum (expensive gas, ICP chain-key compatible) vs Polygon (cheap gas, not ICP-compatible yet)
   - Decide: ICP chain-key control vs manual wallet management

2. **Exchange Setup (Operational - Treasurer):**
   - Set up CPF account at CEX (Coinbase recommended)
   - Complete KYC/AML for organization
   - Link bank account, test small purchase flow

3. **Algorithm Implementation (Technical):**
   - Build thermostat algorithm in governance canister (see crypto_funding.md)
   - Weekly timer for balance checks
   - Email/Slack notifications to Treasurer
   - Dashboard endpoint for Treasurer

4. **Bootstrap (Operational - Treasurer):**
   - Initial €500-1000 ETH purchase (personal funding temporarily)
   - Set initial parameters (min €500, target €1000)
   - Start weekly monitoring

---

## Deferred Decisions (For Later Stages)

### DD-001: Community Governance Model - NFT-Based vs Token-Based

**Status:** ⏸️ **DEFERRED** (will revisit 2026-2027)

**Context:**
Long-term, CPP needs community governance. Traditional SNS uses **fungible tokens** for voting. However, CPP may use **NFT-based governance** instead, aligned with the platform's NFT focus.

**Why Deferred:**
- Not needed until platform is mature (2-3 years out)
- Governance model requires understanding user base and community dynamics
- Legal/regulatory landscape may change
- Need to see how NFT holders naturally engage with platform

**Likely Direction:**
- ❌ **Probably NOT** standard SNS with fungible tokens
- ✅ **More likely:** Non-fungible governance (NFT-based voting)

**Potential NFT Governance Models:**

| Model | Voting Power | Pros | Cons |
|-------|--------------|------|------|
| **1. One NFT = One Vote** | Equal voting (democratic) | Simple, fair | Whales can buy multiple NFTs |
| **2. NFT rarity-weighted** | Rare NFTs have more votes | Rewards early supporters | Complex, may feel unfair |
| **3. Soulbound + staking** | Soulbound NFTs + stake duration | Aligned long-term incentives | Complex, illiquid |
| **4. Hybrid NFT + contribution** | NFT ownership + participation score | Rewards active community | Hard to measure participation |

**Future Questions:**
- Should governance rights be tied to CPF NFT ownership?
- One NFT = one vote, or weighted by rarity/tier?
- Soulbound NFTs (non-transferable) for governance?
- How to prevent governance attacks (buying many NFTs to control voting)?
- Integration with existing board governance (3-of-3)?
- Quadratic voting for NFT holders?

**Revisit When:**
- Platform has 10k+ NFT holders
- Community is engaged in governance discussions
- Clear use cases for community voting emerge
- Legal clarity on NFT-based governance

**References:**
- [multi-sig-governance-comparison.md § Option 3 (SNS)](./multi-sig-governance-comparison.md)
- [governance-policy.md](./governance-policy.md)
- Examples: Nouns DAO (one NFT = one vote), MolochDAO (shares-based)

---

### DD-002: Cross-Canister State/Session Management

**Status:** ⏸️ **DEFERRED** (will revisit if specific use cases emerge)

**Context:**
**Cross-canister authentication is SOLVED** via Internet Identity + derivation origins (AD-001). Users authenticate once with II, same principal across all CPP canisters.

However, there's a separate question of **cross-canister state/session management** beyond authentication. For example: donation flow → automatic NFT issuance.

**What Already Works:**
- ✅ User authenticates once with Internet Identity
- ✅ Same principal across all canisters (cpf.nft derivation origin)
- ✅ Canisters can query each other for user data via inter-canister calls

**What Might Be Needed (Deferred):**
- ⚠️ **Stateful workflows across canisters** (e.g., donation in members canister → trigger NFT mint in NFT canister)
- ⚠️ **Session context sharing** beyond principal (e.g., "user is in middle of onboarding flow")
- ⚠️ **Transactional coordination** (ensure donation + NFT issuance are atomic)

**Why Deferred:**
- Most cross-canister interactions can use simple inter-canister calls
- No identified use cases requiring complex session state beyond authentication
- Can build specific flows (donation → NFT) without general session framework

**Potential Future Patterns:**

| Pattern | Use Case | Complexity |
|---------|----------|------------|
| **1. Simple inter-canister call** | Donation canister calls NFT canister to mint | Low - already works |
| **2. Callback pattern** | Async workflow: donate → wait → NFT minted → callback | Medium |
| **3. Saga pattern** | Multi-step transaction with rollback (donate → KYC → NFT) | High |
| **4. Shared session canister** | Central session state for multi-step flows | Very high |

**Current Approach:**
- Use simple inter-canister calls for most workflows
- Each canister maintains its own state for user
- No shared session state beyond what II provides (principal)

**Revisit When:**
- Specific use case emerges requiring complex multi-canister workflows
- Simple inter-canister calls become insufficient
- Need transactional guarantees across canisters

**Example Flow (Already Possible Without Session Management):**
```motoko
// In members canister
public shared(msg) func processDonation(amount: Nat): async Result<NFTId, Text> {
  let principal = msg.caller;  // Same principal as in NFT canister

  // Record donation
  let donationId = await recordDonation(principal, amount);

  // Trigger NFT mint in NFT canister (inter-canister call)
  let nftId = await NFTCanister.mintDonationNFT(principal, amount);

  #ok(nftId)
}
```

**References:**
- [derivation-origins-integration.md](./derivation-origins-integration.md) - Cross-canister authentication (solved)
- IC Inter-Canister Calls: https://internetcomputer.org/docs/current/developer-docs/backend/candid/

---

### DD-003: Geographic Distribution - Subnet Selection

**Status:** ⏸️ **DEFERRED** (will revisit when scaling globally)

**Context:**
IC has multiple subnets in different geographic regions. Should CPP canisters be distributed geographically for performance?

**Why Deferred:**
- Not a concern for initial launch
- IC routing is already optimized
- Subnet selection is more relevant at scale

**Future Questions:**
- Deploy canisters on multiple subnets?
- Geographic routing for performance?
- Data residency requirements?

**Revisit When:**
- Platform has global user base
- Performance metrics show geographic latency issues
- Regulatory requirements mandate data residency

---

## Decision-Making Process

### How Decisions Are Made

**For Strategic Decisions (AD-001 through AD-007):**
1. Research and document options
2. Discuss with stakeholders (board, technical team)
3. Document decision with rationale
4. Update this document
5. Update relevant technical documentation

**For Open Questions (OQ-001 through OQ-006):**
1. Monitor as platform develops
2. Gather data (user feedback, metrics, security analysis)
3. Set deadline for decision (before blocking critical path)
4. Make decision using strategic decision process above

**For Deferred Decisions (DD-001 through DD-003):**
1. Revisit when trigger conditions met
2. Promote to "Open Questions" for active consideration
3. Follow normal decision-making process

### Decision Authority

| Decision Type | Authority | Approval Required |
|---------------|-----------|------------------|
| **Strategic architecture** | Board | 3-of-3 board members |
| **Technical implementation** | Technical team | 1-of-2 technical signers (evolves to 3-of-5) |
| **Operational configuration** | Technical admin | Single admin (within limits) |
| **Emergency changes** | Any board member or automated | 1-of-3 board (for manual triggers) |

---

## Revision History

| Date | Version | Changes | Author |
|------|---------|---------|--------|
| 2025-11-14 | 1.0.0 | Initial architectural decision log created | Architecture team |

---

## Related Documents

**Governance:**
- [governance-policy.md](./governance-policy.md) - Who approves what (policy)
- [multi-sig-governance-comparison.md](./multi-sig-governance-comparison.md) - Multi-sig options analysis

**Identity & Authentication:**
- [admin-architecture.md](./admin-architecture.md) - Three-level identity hierarchy
- [canister-architecture-diagram.md](./canister-architecture-diagram.md) - Derivation origins and identity

**ENS & DNS:**
- [ens-dns-setup.md](./ens-dns-setup.md) - ENS/DNS bootstrap guide
- [canister-bootstrap-pattern.md](./canister-bootstrap-pattern.md) - Security patterns

**Treasury & Finance:**
- [crypto_funding.md](./crypto_funding.md) - Thermostat algorithm for crypto treasury (ETH/MATIC/ICP)

**Implementation:**
- [origins.md](./origins.md) - Derivation origins technical details (if exists)
- [frontend-technology-evaluation.md](./frontend-technology-evaluation.md) - Frontend decisions

---

**Last Updated:** 2025-11-14
**Status:** Living Document
**Next Review:** Before production launch (Q2 2025)
