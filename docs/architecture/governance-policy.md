# CPP Platform Governance Policy

**Date:** 2025-11-13
**Purpose:** Define governance structure and approval requirements for CPP platform
**Context:** Who can approve what actions - separation of board governance vs technical operations
**Cross-Reference:** See [multi-sig-governance-comparison.md](./multi-sig-governance-comparison.md) for implementation details

## Executive Summary

CPP platform governance has **two distinct levels**:

1. **Board Governance** - Strategic and financial decisions (3 board members)
2. **Technical Operations** - Day-to-day technical management (technical signers)

**Key Principle:** Board controls **strategic direction and capital**, technical team controls **operational execution**.

---

## Governance Structure

### **Board Members (3 persons)**

**Role:** Strategic oversight, financial control, major decisions

**Who:**
- Board Member 1: [Name] (Chairperson)
- Board Member 2: [Name]
- Board Member 3: [Name]

**Authority:**
- Control foundation assets (ICP, cycles, fiat)
- Approve major contracts (NFT deployment, external integrations)
- Hire/fire technical signers
- Approve strategic direction changes

---

### **Technical Signers (1-3 persons initially, growing to 3-5)**

**Role:** Day-to-day technical operations, canister management

**Initial Team (Bootstrap Phase):**
- Technical Lead: [Name] (primary)
- Developer/Backup: [Name] (optional, for redundancy)

**Mature Team (Production):**
- Technical Lead: [Name]
- Developer 1: [Name]
- Developer 2: [Name]
- DevOps: [Name] (as team grows)
- Security Advisor: [Name] (as team grows)

**Authority:**
- Upgrade canisters (code updates)
- Manage cycles (routine top-ups)
- Configure monitoring and alerts
- Emergency responses (pause/unpause)

**Evolution:**
- **Phase 0 (Bootstrap):** Single controller deploys governance infrastructure
- **Phase 1 (1-2 signers):** Minimal multi-sig (1-of-2 or 2-of-2)
- **Phase 2 (2-3 signers):** Standard multi-sig (2-of-3)
- **Phase 3 (3-5 signers):** Full multi-sig (3-of-5)

---

## Governance Identity Architecture

> **For complete ENS/DNS setup and bootstrap procedures, see [ens-dns-setup.md](./ens-dns-setup.md)**

### Board Member Authentication: governance.cpf.nft

**Board members authenticate using a SEPARATE derivation origin from users:**

| Identity Type | Derivation Origin | Purpose | Principals |
|---------------|------------------|---------|------------|
| **Board Member (Governance)** | governance.cpf.nft | Vote on governance proposals, control ENS | governance.cpf.nft-derived II principals |
| **Technical Signer** | cpf.nft | Canister upgrades, operations | cpf.nft-derived II principals |
| **Regular User** | cpf.nft | Use services (newsletters, member portal) | cpf.nft-derived II principals |

**Key Insight:** Board members get **different Internet Identity principals** when they authenticate with governance.cpf.nft (for governance) vs cpf.nft (as a user).

**Why Separate governance.cpf.nft Origin?**

1. **Bootstrap Facilitation:**
   - Board identities established FIRST using governance.cpf.nft
   - These identities deploy and control user-facing cpf.nft infrastructure
   - Clean separation of governance from users

2. **Phishing Protection:**
   - Board members only access governance.cpf.nft for governance operations
   - Users never see or access governance.cpf.nft
   - Separate domain makes phishing attacks more obvious

3. **Audit Clarity:**
   - Governance actions (ENS updates, multi-sig proposals) clearly separated
   - Board member principals distinct from user/admin principals
   - On-chain audit trail shows governance vs operational actions

4. **Security Isolation:**
   - Board governance identities never exposed to user-facing services
   - Even if user-facing service compromised, governance identities safe
   - Board members can have user accounts (with different principals)

---

### ENS NFT Ownership & Control

**cpf.nft is an ERC-721 NFT on Ethereum**

The cpf.nft ENS domain is not just a domain name - it's an **NFT** on Ethereum mainnet. Ownership of this NFT determines who can:
- Update DNS records for cpf.nft
- Create/modify subdomains (like governance.cpf.nft)
- Transfer ownership to another wallet/address

**Ownership Evolution:**

```
Phase 0: Bootstrap (Personal Wallet)
    ↓ Personal Ethereum wallet owns cpf.nft
    ↓ Fast iteration during initial deployment
    ↓
Phase 1: Governance Control (ICP Chain-Key)
    ↓ ENS NFT transferred to ICP-derived Ethereum address
    ↓ Board members (governance.cpf.nft) control via IC governance
    ↓
Phase 2: Full Decentralization (SNS)
    ↓ ENS controlled by SNS governance
    ↓ Community votes on all ENS updates
```

**Current Control Mechanism (Phase 1):**

```
Board Members (governance.cpf.nft identities)
    ↓ submit/approve governance proposals
Governance Canister (Rust + Motoko voting)
    ↓ derives Ethereum address via chain-key ECDSA
ICP-Derived Ethereum Address
    ↓ owns ENS NFT on Ethereum
cpf.nft ENS NFT
    ↓ controls DNS, subdomains, ownership
```

**How Board Members Control ENS:**

1. **Board member authenticates** with governance.cpf.nft derivation origin
2. **Submits governance proposal:** "Update cpf.nft DNS to point to new canister"
3. **Other board members approve:** 3-of-3 required (Tier 0 action)
4. **Governance canister executes:**
   - Derives Ethereum signature using chain-key ECDSA (threshold signature)
   - Signs Ethereum transaction to ENS Registry contract
   - Submits transaction to Ethereum network
5. **ENS records updated** on Ethereum blockchain
6. **DNS propagates** to IC boundary nodes

**ENS Actions Requiring Board Approval (Tier 0):**
- Update cpf.nft DNS records (point to different canister)
- Create new subdomains (e.g., community.cpf.nft)
- Modify existing subdomains (e.g., governance.cpf.nft)
- Transfer cpf.nft ENS NFT ownership
- Update ENS text records (metadata, contact info, etc.)

**See [ens-dns-setup.md](./ens-dns-setup.md) for:**
- How to transfer ENS NFT to ICP governance control
- How governance canister derives Ethereum address
- How to execute ENS updates via IC governance
- Verification procedures

---

### Three-Level Identity Hierarchy

CPP platform uses three distinct identity levels:

**Level 1: Controller (dfx identity)**
- **Who:** Technical lead during bootstrap → Governance canister
- **Purpose:** Deploy/upgrade canisters, manage cycles, system control
- **Principal Type:** dfx identity (NOT Internet Identity)
- **Authority:** Infrastructure-level (canister lifecycle)

**Level 2: Board Governance (governance.cpf.nft identities)**
- **Who:** 3 board members
- **Purpose:** Strategic governance (ENS, major decisions, financial)
- **Principal Type:** Internet Identity (governance.cpf.nft derivation)
- **Authority:**
  - Control cpf.nft ENS NFT
  - Approve Tier 0 actions (3-of-3 required)
  - Hire/fire technical signers
  - Approve strategic direction

**Level 3: Technical Operations (cpf.nft identities)**
- **Who:** 1-2 technical signers initially → 3-5 as team grows
- **Purpose:** Day-to-day technical operations, canister management
- **Principal Type:** Internet Identity (cpf.nft derivation)
- **Authority:**
  - Approve Tier 1 actions (1-of-2 → 3-of-5)
  - Routine cycle management
  - Emergency responses
  - Application administration

**Example: Board Member Has Both Identities**

A board member might have:
- **Board identity (governance.cpf.nft):** Principal `aaaaa-aaaaa-aaaaa` (for governance)
- **User identity (cpf.nft):** Principal `bbbbb-bbbbb-bbbbb` (for using services)

These are **completely separate identities**:
- When voting on governance: Uses governance.cpf.nft → Principal aaaaa...
- When using newsletter service: Uses cpf.nft → Principal bbbbb...
- No connection between the two (different derivation origins)

---

### Bootstrap Sequence for Identity Hierarchy

**Phase 0: ENS Acquisition (Before canister deployment)**
```bash
# Personal Ethereum wallet acquires cpf.nft ENS NFT
# Create governance.cpf.nft subdomain immediately
# See ens-dns-setup.md § Phase 0
```

**Phase 1: Deploy IC Infrastructure**
```bash
# Controller (dfx identity) deploys all canisters
dfx deploy governance_canister --network ic
dfx deploy voting_canister --network ic
# ... other canisters
```

**Phase 2: Bootstrap Board Identities (governance.cpf.nft)**
```bash
# Configure governance.cpf.nft DNS → governance_canister
# Board members visit https://governance.cpf.nft
# Authenticate with Internet Identity
# Board member principals initialized in voting canister

BOARD_MEMBER_1="xxxxx-xxxxx-xxxxx"  # From governance.cpf.nft auth
BOARD_MEMBER_2="yyyyy-yyyyy-yyyyy"
BOARD_MEMBER_3="zzzzz-zzzzz-zzzzz"

dfx canister call voting_canister initializeBoardMembers \
  "(vec {
    principal \"$BOARD_MEMBER_1\";
    principal \"$BOARD_MEMBER_2\";
    principal \"$BOARD_MEMBER_3\"
  })"
```

**Phase 3: Transfer ENS to Governance**
```bash
# Governance canister derives Ethereum address
# Transfer cpf.nft ENS NFT to that address
# Board now controls ENS via IC governance
# See ens-dns-setup.md § Phase 3
```

**Phase 4: Deploy User-Facing Infrastructure (cpf.nft)**
```bash
# Configure cpf.nft DNS → cpf_nft_canister
# Deploy .well-known files
# Users can now authenticate with cpf.nft
# Technical signers also use cpf.nft (different from board!)
```

**Result:**
- ✅ Board members (governance.cpf.nft) control ENS + IC governance
- ✅ Technical signers (cpf.nft) control day-to-day operations
- ✅ Users (cpf.nft) access services
- ✅ Clear separation at every level

---

## Approval Requirements by Action

### **Tier 0: Board-Level Actions (3-of-3 Board Members Required)**

| Action | Description | Why Board Approval |
|--------|-------------|-------------------|
| **Deploy NFT contract** | Deploy Einstein Solidity contract on Polygon | Legal and financial liability |
| **Fund ICP/cycles (large)** | Transfer >100 ICP or purchase cycles | Foundation capital deployment |
| **New canister deployment** | Deploy new canister to production | New service = new liability |
| **Change governance structure** | Modify board composition or approval thresholds | Fundamental governance change |
| **Hire/remove technical signers** | Add or remove technical signer | Trust and access control |
| **Major contract agreements** | Sign agreements with external parties | Legal and financial commitment |
| **Stripe account changes** | Change Stripe account owners | Financial control |
| **Multi-sig wallet changes** | Add/remove board members from Gnosis Safe | Board composition change |
| **Delete production canister** | Permanently destroy canister | Irreversible, potential data loss |

**Process:**
- All 3 board members must approve via Internet Identity
- Proposal submitted to board governance canister
- 7-day discussion period (optional)
- Board members sign approval
- Executes when 3-of-3 approvals received

---

### **Tier 1: Critical Technical Actions (Threshold Evolves with Team Size)**

| Action | Description | Why Technical Approval |
|--------|-------------|----------------------|
| **Upgrade production canister** | Deploy new WASM code | Code changes require review |
| **Change canister controller** | Modify controller settings | Security critical |
| **Change multi-sig threshold** | Modify technical signer threshold | Changes governance rules |
| **Add/remove technical signer** | Change who can approve technical actions | Access control |
| **Enable/disable canister** | Stop or start production canister | Service availability |
| **Transfer cycles (medium)** | Transfer 10-100T cycles | Moderate asset movement |
| **Change stable memory** | Direct modification of stable variables | Can corrupt state (emergency only) |
| **Update security settings** | Change rate limits, blacklists, CORS | Security posture |

**Threshold Evolution:**
- **Phase 1 (1-2 signers):** 1-of-2 (requires backup review)
- **Phase 2 (2-3 signers):** 2-of-3 (simple majority)
- **Phase 3 (3-5 signers):** 3-of-5 (majority with good availability)

**Process:**
- Any technical signer submits proposal
- Required approvals based on current team size
- No waiting period (can execute immediately when threshold reached)
- Board notification (informational only)

---

### **Tier 2: Routine Technical Actions (Threshold Evolves with Team Size)**

| Action | Description | Why Lower Threshold |
|--------|-------------|-------------------|
| **Top up cycles (routine)** | Add <10T cycles to canister | Operational necessity, low risk |
| **Update monitoring config** | Change alert thresholds, auto top-up settings | Operational tuning |
| **Emergency pause** | Temporarily stop service | Fast response needed |
| **Acknowledge alerts** | Mark alerts as reviewed | Routine operations |
| **Deploy to preprod** | Deploy to preprod environment | Testing, not production |
| **Blacklist address** | Block malicious user | Security response |
| **Adjust rate limits** | Change API rate limits | Operational tuning |

**Threshold Evolution:**
- **Phase 1 (1-2 signers):** 1-of-2 (any signer can act)
- **Phase 2 (2-3 signers):** 1-of-3 or 2-of-3 (operational flexibility)
- **Phase 3 (3-5 signers):** 2-of-5 (good availability, low friction)

**Process:**
- Any technical signer submits proposal
- Required approvals based on current team size
- Executes when threshold reached
- Board notification (informational only)

---

### **Tier 3: Automated Actions (No Approval Required)**

| Action | Description | Why Automated |
|--------|-------------|---------------|
| **Monitor cycle balance** | Heartbeat checks every minute | Operational necessity |
| **Monitor canister status** | Health checks, memory usage | Operational necessity |
| **Alert on low cycles** | Generate alert when balance < threshold | Preventive monitoring |
| **Auto cycle top-up** | Automatic top-up when balance < threshold (if enabled) | Prevent service disruption |
| **Metrics collection** | Gather usage statistics | Operational intelligence |
| **Log rotation** | Manage audit logs | Housekeeping |
| **Update canister registry** | Sync cached canister info | Data freshness |

**Process:**
- Runs automatically via heartbeat
- No approval required
- Alerts generated for review
- Board/technical team can disable automation if needed (requires Tier 1 approval)

---

### **Tier 4: Emergency Actions (1-of-N Technical Signers + Board Notification)**

| Action | Description | Why Emergency |
|--------|-------------|---------------|
| **Emergency stop** | Immediately pause all canisters | Attack in progress |
| **Circuit breaker** | Automated stop on anomaly detection | Prevent damage |
| **Emergency cycle transfer** | Move cycles to prevent freeze | Service preservation |

**Threshold:**
- **Any Phase:** 1-of-N (any single technical signer can act immediately)
- Emergency situations require immediate response regardless of team size

**Process:**
- Any 1 technical signer can execute immediately
- Board notified immediately (email, SMS)
- Board can override/reverse within 24 hours
- If not reversed, requires Tier 1 approval to resume

---

## Delegation Model

### **Board Delegates to Technical Team:**

**What Board Delegates:**
- Day-to-day canister upgrades
- Cycle management
- Monitoring and alerting
- Bug fixes and patches
- Performance tuning
- Security responses (blacklisting, rate limiting)

**What Board Retains:**
- Capital deployment (ICP, fiat)
- New service deployment (new canisters)
- Contract deployment (NFTs, external integrations)
- Governance structure changes
- Major strategic decisions

**Oversight Mechanism:**
- Technical team reports to board monthly
- Board reviews all Tier 1 actions (informational)
- Board can revoke delegation at any time (requires 3-of-3 board vote)
- Board receives all critical alerts

---

## Escalation Paths

### **Technical → Board Escalation:**

**When technical team must escalate to board:**
1. New canister deployment needed
2. Funding request >100 ICP
3. Legal/compliance concern
4. Major security incident
5. Service-level agreement (SLA) breach
6. Hiring/removing technical signers
7. Uncertainty about approval authority

**Process:**
1. Technical lead submits escalation request
2. Board reviews within 48 hours
3. Board votes (3-of-3 required)
4. Board decision final

---

### **Board → Technical Delegation:**

**When board delegates to technical team:**
1. Board approves new canister deployment → Technical team deploys
2. Board approves funding → Technical team manages cycles
3. Board sets security policy → Technical team implements

**Process:**
1. Board votes approval (3-of-3)
2. Board issues delegation directive
3. Technical team executes
4. Technical team reports completion

---

## Bootstrap Chicken-and-Egg Problem

### **The Challenge:**

You cannot use multi-sig governance to deploy the multi-sig governance canister itself. This creates a bootstrap sequence challenge:

```
Problem: Need multi-sig canister to control production canisters
But: Need single controller to deploy multi-sig canister
Therefore: Must bootstrap with single controller, then transition
```

### **Realistic Initial Team:**

- **Technical Lead:** 1 person (primary infrastructure deployer)
- **Backup Developer:** 1 person (optional, for redundancy and review)
- **Total:** 1-2 technical people initially

### **Bootstrap Strategy:**

**Phase 0: Infrastructure Deployment (Single Controller)**
```bash
# Technical Lead uses dfx identity as controller
dfx deploy board_governance_canister --network ic
dfx deploy technical_multisig_canister --network ic
dfx deploy backend_canister --network ic
# ... deploy all infrastructure canisters
```

**Phase 1: Transfer Control to Governance (1-2 Signers)**
```bash
# Set multi-sig canister as controller of production canisters
dfx canister update-settings backend_canister \
  --add-controller <technical-multisig-principal> \
  --network ic

# Add Technical Lead's II principal to multi-sig canister
dfx canister call technical_multisig_canister addSigner \
  '(principal "aaaaa-aaaaa-aaaaa-aaaaa")' \
  --network ic

# Add Backup Developer's II principal (if available)
dfx canister call technical_multisig_canister addSigner \
  '(principal "bbbbb-bbbbb-bbbbb-bbbbb")' \
  --network ic

# NOW: Multi-sig canister controls production
# Technical Lead can optionally remove their dfx controller access
```

**Phase 2: Grow Team (2-3 Signers)**
- Add Developer 2 to multi-sig
- Increase threshold from 1-of-2 to 2-of-3

**Phase 3: Mature Governance (3-5 Signers)**
- Add DevOps, Security Advisor
- Increase threshold to 3-of-5

### **Key Principle:**

**Bootstrap requires temporary centralization** (single dfx controller) to deploy governance infrastructure, then **immediate decentralization** by transferring control to multi-sig canister.

---

## Initial Bootstrap: Special Rules

### **Pre-Production (Development/Preprod):**

**Controller:** Single developer identity
- Fast iteration, no approvals needed
- Testing governance processes
- Preparing for production launch

### **Production Launch (Transition):**

**Step 1: Establish Board Governance**
- Board members set up Internet Identity principals
- Deploy board governance canister
- Board becomes controller of:
  - NFT contract (Gnosis Safe on Polygon)
  - Cycle wallet (ICP funds)
  - Stripe account

**Step 2: Establish Technical Governance**
- Technical signers set up Internet Identity principals
- Deploy multi-sig technical canister
- Technical canister becomes controller of:
  - All production canisters
  - Monitoring and operations

**Step 3: Link Governance Levels**
- Technical canister has board governance canister as controller
- Board can override technical decisions (3-of-3 vote)
- Technical team operates independently within delegated authority

---

## Governance Evolution

### **Stage 1: Foundation Board + Technical Team (Years 1-2)**

```
Board Governance Canister (3-of-3 board members)
    ↓ (controls)
Technical Multi-Sig Canister (1-2 signers initially → 3-5 as team grows)
    ↓ (controls)
All Production Canisters
```

**Initial Configuration (1-2 technical signers):**
- Technical Lead + optional Backup
- Threshold: 1-of-2 or 2-of-2 (depending on redundancy needs)
- Board retains ultimate control via board governance canister
- Fast iteration for infrastructure deployment

**Mature Configuration (3-5 technical signers):**
- Technical Lead, Developers, DevOps, Security Advisor
- Threshold: 3-of-5 for critical actions, 2-of-5 for routine
- Clear separation: Board = strategic, Technical = operational
- Technical team has operational autonomy within delegated authority
- Board oversight for major decisions

---

### **Stage 2: Community Participation (Years 2-3)**

**Add community advisors to technical signers:**
- Expand technical signers from 3-5 to 5-7 (add 2 community members)
- Threshold adjusted: 3-of-7 or 4-of-7 (community can't unilaterally control)
- Board retains 3-of-3 control over strategic decisions

**Add community input to board decisions:**
- Board posts proposals publicly before voting
- Community can comment (non-binding)
- Board considers community input
- Board makes final decision

**Technical Team Evolution:**
- **Years 0-1:** 1-2 signers (bootstrap, infrastructure deployment)
- **Years 1-2:** 3-5 signers (mature internal team)
- **Years 2-3:** 5-7 signers (adding community advisors)
- **Year 3+:** SNS governance (full decentralization)

---

### **Stage 3: SNS Governance (Year 3+)**

**When platform is mature and stable:**

```
SNS Governance (Community)
    ↓ (controls via token voting)
All Production Canisters
```

**Transition:**
- Board votes to transfer control to SNS (3-of-3 required)
- SNS token distribution to community
- Board becomes proposal submitters (like everyone else)
- Community votes on all governance actions
- Full decentralization achieved

**Board role changes:**
- No longer has veto power
- Can submit proposals
- Retains control of foundation assets (ICP treasury)
- Foundation still owns Stripe account, legal entities

---

## Governance Principles

### **1. Separation of Powers**
- Board: Strategy, capital, legal
- Technical: Operations, code, infrastructure
- Clear boundaries, no overlap in authority

### **2. Least Privilege**
- Only grant minimum necessary authority
- Technical signers can't deploy new services (board only)
- Board can't push code changes (technical only)

### **3. Transparency**
- All proposals logged on-chain
- All votes recorded with principal and timestamp
- Audit trail immutable
- Monthly reports to board

### **4. Reversibility**
- Most actions can be reversed (upgrade → rollback)
- Emergency actions can be overridden by board
- Board can remove technical signers
- Technical team can escalate to board

### **5. Progressive Decentralization**
- Start centralized (board control)
- Gradually decentralize (community participation)
- Eventually fully decentralized (SNS)
- No forced timeline, based on maturity

---

## Practical Example: Canister Upgrade Flow

### **Scenario:** Deploy new version of backend_canister

**Step 1: Developer prepares upgrade**
- Write code, test locally
- Deploy to preprod (threshold based on team size)
- Test in preprod environment
- Prepare WASM for production

**Step 2: Submit proposal**
- Technical Lead submits upgrade proposal to multi-sig canister
- Includes: Description, WASM hash, testing results
- Proposal ID: 42

**Step 3: Technical review**
- Technical signers review code changes, test results, security implications
- Number of required approvals depends on team phase:
  - Phase 1 (1-2 signers): 1-of-2 approval
  - Phase 2 (2-3 signers): 2-of-3 approval
  - Phase 3 (3-5 signers): 3-of-5 approval
- When threshold reached → Upgrade executes automatically

**Step 4: Board notification**
- Board receives email: "Proposal #42 executed: backend_canister upgraded to v2.1"
- Board can review (informational only, no approval needed)
- If board has concerns, can escalate for discussion

**Time to Deploy:** 1-2 hours (technical review time)

**Board Involvement:** None (delegated authority)

---

### **Scenario:** Deploy NFT contract to Polygon

**Step 1: Technical team prepares**
- Write Solidity contract
- Audit contract (external firm)
- Test on testnet
- Prepare deployment plan

**Step 2: Technical team submits to board**
- Escalation request: "Deploy Einstein NFT contract"
- Includes: Contract code, audit report, gas estimates, risks
- Board receives request

**Step 3: Board review**
- Board Member 1 reviews legal implications
- Board Member 2 reviews financial implications (gas costs)
- Board Member 3 reviews strategic fit
- Board discusses via governance canister

**Step 4: Board votes**
- Board Member 1: Approve
- Board Member 2: Approve
- Board Member 3: Approve
- 3-of-3 achieved → Approval granted

**Step 5: Technical team executes**
- Board approval triggers delegation to technical team
- Technical team deploys contract using Gnosis Safe
- Technical team reports deployment complete
- Board verifies contract address

**Time to Deploy:** 3-7 days (board review + discussion)

**Board Involvement:** Full (3-of-3 approval required)

---

## Summary Table: Who Approves What

| Action Type | Approval Required | Typical Response Time | Board Involvement |
|-------------|------------------|---------------------|------------------|
| **Strategic (New services, contracts, funding)** | 3-of-3 Board | 3-7 days | Full control |
| **Critical Technical (Upgrades, security)** | Evolves: 1-of-2 → 2-of-3 → 3-of-5 | 1-24 hours | Notification only |
| **Routine Technical (Cycles, config)** | Evolves: 1-of-2 → 1-of-3 → 2-of-5 | Minutes to hours | Notification only |
| **Automated (Monitoring, alerts)** | None | Real-time | Alert review |
| **Emergency (Stop, circuit breaker)** | 1-of-N (any signer) | Immediate | Notification + 24hr override |

**Note:** Technical thresholds evolve as team grows from 1-2 signers (bootstrap) to 3-5 signers (mature).

---

## Governance Canister Principals

### **Board Members (Internet Identity)**
- Board Member 1: `xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-cai`
- Board Member 2: `yyyyy-yyyyy-yyyyy-yyyyy-yyyyy-cai`
- Board Member 3: `zzzzz-zzzzz-zzzzz-zzzzz-zzzzz-cai`

### **Technical Signers (Internet Identity)**

**Initial Team (Bootstrap):**
- Technical Lead: `aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-cai`
- Developer/Backup: `bbbbb-bbbbb-bbbbb-bbbbb-bbbbb-cai` (optional)

**As Team Grows:**
- Developer 2: `ccccc-ccccc-ccccc-ccccc-ccccc-cai`
- DevOps: `ddddd-ddddd-ddddd-ddddd-ddddd-cai`
- Security Advisor: `eeeee-eeeee-eeeee-eeeee-eeeee-cai`

*(Actual principals to be filled in during bootstrap)*

---

**Last Updated:** 2025-11-13
**Version:** 1.1.0
**Status:** Policy Draft
**Changes:** Updated for realistic initial team size (1-2 technical signers), added bootstrap chicken-and-egg solution
**Review Cycle:** Annual review by board, or as needed for governance changes

**Approved By:** (Pending initial board vote)
- [ ] Board Member 1: _________________ Date: _______
- [ ] Board Member 2: _________________ Date: _______
- [ ] Board Member 3: _________________ Date: _______

**Next Review Date:** 2026-11-13

**Cross-References:**
- [multi-sig-governance-comparison.md](./multi-sig-governance-comparison.md) - Implementation options
- [admin-architecture.md](./admin-architecture.md) - Admin bootstrap
- [canister-bootstrap-pattern.md](./canister-bootstrap-pattern.md) - Security patterns
