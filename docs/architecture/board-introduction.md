# Cool Planet Platform: Introduction for Board Members

**Date:** 2025-11-14
**Audience:** Board members (who hold legal responsibility for platform operations)
**Purpose:** Understand what we're building, why, and your role in governance
**Time to read:** 10-15 minutes

---

## Welcome to Your Platform

Congratulations on getting your Internet Identity! This is your first step into the Cool Planet Platform you'll be governing and for which you hold legal responsibility.

**Who This Document Is For:**
- **Board members** - You hold legal responsibility and voting authority for platform decisions

**Governance Context:**
- **Senior authors (technical/legal advisors)** - Two senior authors with web3 expertise serve as technical/legal advisors
  - They propose technical solutions and interpret legal ramifications
  - They do NOT have legal responsibility
  - The head of senior authors is also a board member
- **You (board members)** decide if you're comfortable with the legal implications of their proposals

This document explains what we're building in plain language, what decisions you'll need to make, and what you can safely delegate to the technical team.

**No technical background required.** If anything is unclear, that's our failure to explain it well, not yours.

---

## Quick Decision-to-Compliance Map

| Decision | Main Compliance Trigger | Legal Risk Level | Professional Support Needed | ANBI Impact |
|----------|------------------------|------------------|----------------------------|-------------|
| **0. ANBI Application** | Dutch tax law | Low | Tax advisor for application | **Critical - 3-6 month process** |
| **1. Launch Subdomains** | GDPR, Content Moderation | Low | Privacy attorney (review) | None |
| **2. Enable Donations** | **KYC/AML + ANBI status** | **Medium-High** | **Financial compliance attorney** | **Major - affects donor incentive** |
| **3. NFT Automation** | Securities law uncertainty | **High** | **Securities attorney + Auditor** | Minor - ANBI on top of NFT |
| **4. Main Site Cutover** | Depends on milestone chosen | Varies | Based on which features are live | Depends on which features |

**Bottom Line:** Decision 0 (ANBI application) should happen NOW (3-6 month approval timeline). Decision 2 (donations) requires the most immediate board attention due to KYC/AML obligations AND timing with ANBI approval. Decision 3 (NFTs) has the most regulatory uncertainty.

---

## What Are We Building?

### The Big Picture

We're building a **decentralized platform** for Cool Planet Foundation that does three main things:

1. **Public Website** - Share your mission, stories, and newsletters with the world
2. **Donation System** - Accept donations and automatically award NFTs as thank-you gifts
3. **Member Community** - Create a space for engaged supporters to connect and learn

**Key Difference from Traditional Websites:**
Instead of relying on a single company (like Wix or Squarespace), we're building on the **Internet Computer** - a blockchain that runs websites in a decentralized way. This means:
- No single company can shut you down or change the rules
- Lower long-term costs (no monthly hosting fees that increase over time)
- You (the board) control the infrastructure through governance votes
- Data is stored in Switzerland for better privacy protection

---

## Why This Matters: Two Separate Identities

You now have **one Internet Identity** that will work in **two different ways**:

### Your "Board Member" Hat (governance.cpf.nft - Control Plane)
When you log in to the control plane, you're wearing your governance hat:
- Vote on platform upgrades and approve major financial decisions
- Manage domain names and technical infrastructure
- Review proposals from senior authors (your technical/legal advisors)
- Decide if you're comfortable with legal implications
- **This is like being on a building's board of directors** - you decide on major renovations, but don't fix the plumbing yourself

### Your "Community Contributor" Hat (cpf.nft - Data Plane)
When you create content or participate in the community, you're wearing your contributor hat:
- Write newsletters and blog posts
- Review private feedback from users (available in Phase B)
- Moderate public comments from content contributors (available when Budding Authors features launch)
- Participate in community discussions
- **This is like being a contributor to the foundation's mission** - separate from governance

**Note:** "Budding Authors" is a specific platform role for NFT holders who add Gravatar profiles to contribute public comments. As a senior author/board member, you can create content without being a Budding Author.

**Why Separate?** Just like you wouldn't use your board credentials to write a blog post, these two roles (governance vs. community participation) are kept separate for security and clarity.

---

## The Journey from Wix to Internet Computer

### Where We Are Now
- **Main website** is on Wix (coolplanet-foundation.org)
- Wix charges monthly fees and controls the platform
- Limited functionality - can't do donations or NFTs easily

### Where We're Going
We'll migrate in **small, safe steps** with multiple decision points:

```
Phase A: Launch Newsletters (IMMINENT)
├─ Newsletters portal on Internet Computer
├─ **ROUTING DECISION:** Subdomain (independent launch) vs. Path-based (requires Phase F first)
├─ Email collection and distribution
└─ If subdomain: Main Wix site stays unchanged (safe fallback)

Phase B: Launch App/Camino (SOON)
├─ Educational content platform (Camino)
├─ Requires Internet Identity for access
├─ Learning progress tracking
├─ **Same routing decision as Phase A**
└─ Wix still running (safe fallback)

Phase C: Add Donation Processing
├─ Connect to Stripe for credit card donations
├─ Test with small amounts first
└─ Wix still running (safe fallback)

Phase D: Add NFT Automation
├─ Connect to Ethereum/Polygon for NFT awards
├─ Test with small batches
└─ Wix still running (safe fallback)

Phase E: Add Gated Features to Cool Planet App
├─ Webinars and on-demand replays
├─ Advanced Camino modules (Climbing the Mountain)
├─ Gated discussion forums (requires Camino completion)
└─ Wix still running (safe fallback)

Phase F: Replace Main Website (BOARD DECISION POINT)
├─ **TIMING DEPENDS ON ROUTING DECISION:**
│  - If subdomain: Happens later (after Phases A-E proven)
│  - If path-based: Happens BEFORE Phase A (newsletters requires it)
├─ Option 1: Replace when new site has same features as Wix
├─ Option 2: Replace when donations are working
├─ Option 3: Replace when NFTs are fully automated
└─ Board votes on which milestone triggers the switch
```

**Important Rollback Implications:**
- **Subdomain approach:** Can roll back at any time during Phases A-E. Wix stays active as a safety net.
- **Path-based approach:** Main domain switch happens first, so rollback is more complex (reverting DNS changes vs. disabling subdomains).

---

## Your Role: Governance & Legal Responsibility

### What Requires Governance Decisions

These are **strategic decisions** that require governance approval (formal board votes):

#### Financial Decisions
- Spending more than €2,000 (e.g., buying cryptocurrency for gas fees)
- Major infrastructure purchases
- **Example:** "Should we buy €5,000 of ETH to cover NFT minting costs for the next year?"

#### Platform Upgrades
- Upgrading the donation system with new features
- Adding new functionality to the Cool Planet App
- **Example:** "Should we add webinar capabilities to the Cool Planet App?"

#### Domain & Infrastructure
- Changing where coolplanet-foundation.org points
- Adding or removing major features
- **Example:** "Should we switch from Wix to the Internet Computer version of the main site?"

#### Cutover Milestone Decision
- **When** do we replace the Wix main site with the Internet Computer version?
  - After basic website parity? (Content-Only Migration)
  - After donations are working? (With Donation Capability - requires ANBI resolution)
  - After NFT automation is complete? (With Full NFT Automation)
- This is a **board strategic decision**, not a technical one

### How Board Voting Works

We use **time-based voting thresholds** for safety:

| Board Votes | What Happens | Use Case |
|-------------|--------------|----------|
| **3 of 3 (Everyone agrees)** | ✅ Executes immediately | Normal operations when everyone is available |
| **2 of 3 (Supermajority)** | ⏱️ Waits 3 days, then executes | One board member unreachable, but two agree |
| **1 of 3 (Single member)** | ⏱️ Waits 7 days, then executes | Emergency when two members unreachable |

**Why the delays?** If board members disagree with a decision, they have time to vote "no" and block it. The delay is your safety mechanism.

**Example Scenario:**
- Alice (board member) proposes upgrading the donation system
- Alice and Bob vote yes → Waits 3 days for Carol to weigh in
- If Carol doesn't vote in 3 days → Upgrade proceeds automatically
- If Carol votes no → Upgrade is blocked, needs discussion

### Typical Proposal Workflow

Most proposals will originate from the **senior authors** (your technical/legal advisors) and flow to the board for decision:

1. **Senior Authors Propose** - Senior authors (with web3 expertise) identify technical solution and prepare proposal
2. **Senior Authors Analyze Legal Implications** - They interpret the legal ramifications:
   - Regulatory compliance implications (GDPR, KYC/AML, securities law, etc.)
   - Financial risk assessment
   - Legal obligations triggered by the change
   - Professional support needed (attorneys, auditors, compliance consultants)
3. **Board Reviews & Decides** - You (board members with legal responsibility) review the proposal and decide:
   - Are we comfortable with these legal implications?
   - Do we want to proceed?
   - Which strategic option should we choose?
4. **Board Vote** - Formal vote using time-based thresholds
5. **Technical Implementation** - Senior authors execute the approved strategy

**Example:**
- Senior authors propose: "Enable Stripe donations with this technical architecture"
- Senior authors explain: "This triggers KYC/AML obligations and requires ANBI decision. Here are three strategies (A, B, C) with different financial implications"
- Board reviews: "Are we comfortable with Strategy B (modest reserves)?"
- Board votes: "Yes, proceed with Strategy B in Q1 2026"
- Senior authors implement approved strategy

**Your Role (Board Members):**
- You hold **legal responsibility** for all platform decisions
- Senior authors advise on "what are the technical options" and "what are the legal ramifications"
- You decide "are we comfortable with this" and "should we proceed"
- Senior authors execute what you approve

---

## What You CAN Delegate (Technical Operations)

These are **routine technical tasks** that don't need board votes:

### Technical Team Can Do Without Board Approval:
- **Fix bugs** in the website or donation system
- **Deploy routine updates** to keep systems running
- **Monitor and maintain** servers and infrastructure
- **Respond to technical issues** (site down, broken links, etc.)
- **Add blog posts or newsletter content** (within editorial guidelines)
- **Review private user feedback** and moderate public comments (when available)
- **Small purchases** under €2,000 threshold

### Treasury Management (Systematic Financial Planning)

CPF uses **systematic financial planning** for two key areas:

**1. Crypto Treasury ("Thermostat" Algorithm):**
- Automatically recommends when to buy ETH/MATIC for gas fees
- Based on current balance, projected spending, and runway
- **Under €2,000:** Treasurer handles (operational)
- **Over €2,000:** Board votes required
- **Think of it like your home thermostat:** Automatically suggests when to "refuel"

**2. Gift Tax Reserve Management (Until ANBI Confirmed):**
- Systematically reserve 40% of donations received before ANBI decision
- Segregated account that cannot be spent until ANBI confirmed
- Monthly monitoring of reserves vs. liability
- Board approval required to spend beyond available (non-reserved) funds
- **Similar to crypto reserves:** Prudent financial planning for known risk

**Why This Approach:** Rather than avoiding donations until ANBI confirmed (which could delay revenue 12-18 months), CPF can accept donations BUT must manage financial reserves responsibly. This is treasury management, not technical implementation.

---

## Key Decisions Coming Up

**⚠️ FOUNDATIONAL ARCHITECTURAL DECISION:** The subdomain vs. path-based routing decision affects the ENTIRE migration sequence:
- **Subdomain approach:** Allows incremental phased rollout (A → B → C → D → E → F) with Wix as safety net
- **Path-based approach:** Requires main domain switch (Phase F) to happen FIRST, before newsletters/app can launch

This decision must be made early as it determines whether Phases A/B can proceed independently or require Phase F completion.

---

### Immediate (Next 1-2 Months)

**Decision 0:** Submit ANBI application to Belastingdienst?

**What it means:**
- Apply for Dutch public benefit organization status (tax deductibility for donors)
- Begin 3-6 month approval process

**Legal & Strategic Implications:**
- **Why Now:** Must be submitted BEFORE Decision 2 (donations) to have approval ready
- **Application Requirements:**
  - Foundation statutes and bylaws
  - Financial projections for 3 years
  - Activity plan demonstrating public benefit
  - Board member information
  - Bank account details
- **Approval Timeline:** 3-6 months (typical)
- **If Granted:** Donors can claim tax deduction on donations
- **If Denied:** Must reapply or operate without ANBI (significantly affects fundraising)

**Board Decision Point:** Do we have documentation ready and should we submit now?

**Recommended:** YES - Submit immediately to have ANBI status before launching donations

---

**Decision:** Do we proceed with deploying the governance system?
- **What it means:** Set up the voting system so board members can vote on proposals
- **What you need:** Your Internet Identity (✅ you have this!)
- **Board involvement:** Initial setup and testing
- **Technical details:** See [governance-policy.md](./governance-policy.md) if curious

### Immediate (Next 1-2 Months)

**Decision 1a:** Launch Newsletters portal (Phase A)?

**What it means:**
- Newsletters portal goes live on Internet Computer (imminent)
- **CRITICAL ROUTING DECISION:** Subdomain vs. path-based determines phasing:
  - **Option A (Subdomain):** newsletters.coolplanet-foundation.org - can launch independently, Wix stays active
  - **Option B (Path-based):** coolplanet-foundation.org/newsletters - **REQUIRES switching main domain to IC first** (brings forward Phase F decision)
- Email collection and newsletter distribution

**Phasing Impact:**
- **If subdomain approach:** Phase A can proceed independently, main site stays on Wix
- **If path-based approach:** Must switch main domain to IC BEFORE newsletters can launch (Phase F moves before Phase A)

**Legal & Strategic Implications:**
- **GDPR Compliance Required:** Users submitting emails triggers data protection obligations
  - Must have privacy policy in place
  - Must handle data subject rights (access, deletion, portability)
  - User data stored in Swiss jurisdiction (Internet Computer) - favorable for GDPR
- **Brand Risk:**
  - Subdomain: Low - this is an addition, not replacement
  - Path-based: Higher - main domain switch happens earlier
- **Rollback:**
  - Subdomain: Can disable anytime, Wix unaffected
  - Path-based: More complex - main site already switched to IC

**Board Decision Point:**
1. Subdomain vs. path-based routing strategy?
2. Are we ready for data protection obligations for newsletter subscribers?
3. If path-based: Are we ready to switch main domain to IC now (before newsletters)?

---

### Near-Term (2-4 Months)

**Decision 1b:** Launch Cool Planet App with Camino (Phase B)?

**What it means:**
- **Cool Planet App** goes live (soon after newsletters)
- **One app for everyone** - not a separate "members portal"
- **Progressive feature unlocks** based on what technical capabilities users have:
  - Newsletter subscription → enables basic Camino content
  - Internet Identity login → enables learning progress tracking
  - Donations → unlock advanced features (added in Phase E)
  - NFT ownership → unlock premium content (added in Phase E)
- **Camino (educational content platform)** included in first release
- **Requires Internet Identity** for user access
- Learning progress tracking
- **Feedback functionality** - Users can provide private feedback to senior authors on content
- **Note:** Public comments come later (requires NFT + Gravatar profile for content contributors)
- **Routing decision:** Same as newsletters (subdomain or path-based)
  - **Subdomain:** members.coolplanet-foundation.org or coolplanet-app.coolplanet-foundation.org (can launch independently)
  - **Path-based:** coolplanet-foundation.org/app or /members (requires main domain on IC)

**Phasing Impact:**
- **If subdomain approach:** Phase B can proceed independently, main site stays on Wix
- **If path-based approach:** Requires main domain switch (Phase F) to be completed first

**Legal & Strategic Implications:**
- **Educational Content Standards:** Camino platform provides learning materials
  - Content quality and accuracy standards
  - Attribution and intellectual property considerations
  - Educational material liability (accuracy, safety)
- **Comments/Feedback in App:** Content moderation for app-based feedback
  - Different from blog comments (those come later in phased rollout)
  - App feedback tied to Camino modules
- **Additional GDPR:** Learning progress data storage
- **Brand Risk:** Low - this is an addition, not replacement
- **Rollback:** Can disable anytime, Wix unaffected

**Board Decision Point:** Are we ready for educational content standards, learning progress tracking, and app-based feedback moderation?

**Understanding Progressive Unlocks:**
- **Technical capabilities** (left side of user journey): Newsletter signup → II login → Donations → NFT ownership
- **Content experiences** (right side of user journey): Camino → Comments/Feedback → Webinars → Climbing the Mountain
- Each technical capability unlocks new content experiences in the same app
- No separate apps or portals - just one Cool Planet App with progressive access

---

**Decision 2:** When to enable Stripe production donations?

**What it means:**
- **2025:** Stripe deployed in test mode for technical validation (happens regardless)
- **2026 Decision:** When to switch from test mode to production (real donations)
- Complete Stripe business verification (KYC for CPF as organization)

**Legal & Strategic Implications:**
- **KYC/AML Obligations Triggered:** You become responsible for anti-money laundering compliance
  - **< $1,000 donations:** Stripe handles most compliance (name, address)
  - **> $1,000 donations:** CPF must perform enhanced KYC (identity verification via external provider)
  - **> $15,000 donations:** Enhanced KYC/KYB required (beneficial ownership for business donors)
- **Financial Reporting:** Must track donations for tax reporting
- **Refund Policy:** Need clear policy for donation refunds (if any)
- **PCI Compliance:** Stripe handles credit card data, but CPF must follow Stripe's terms
- **Tax Implications:** Donors may request tax receipts (ANBI status or equivalent)
- **Regulatory Scrutiny:** Financial transactions attract regulatory attention

**ANBI Status Considerations (CRITICAL):**

The timing of your **ANBI (Algemeen Nut Beogende Instelling)** application and potential granting significantly affects this decision:

**Technical Testing Phase (2025 - Happens Regardless):**

Stripe will be deployed in **test mode throughout 2025** for technical validation:
- Full donation flow testing with test credit cards
- KYC procedures tested with board members and beta testers
- NFT award flow verified (if implementing)
- No real money processed, no gift tax liability
- **This is a technical requirement, not a strategic choice**

**Strategic Decision: When to Launch Production Donations in 2026?**

After test mode completes, board must decide when to enable production donations:

**Option A: Launch Production Early 2026 (Before ANBI Decision)**
- **Pros:**
  - Start generating revenue earlier (Q1/Q2 2026)
  - Build donor base ahead of tax benefit availability
  - Platform proven through 2025 test mode
- **Cons:**
  - Donors cannot claim tax deduction yet (less attractive)
  - Must communicate clearly "ANBI application pending"
  - **Must maintain 40% gift tax reserves until ANBI decided**
  - Risk: What if ANBI is denied? (donor expectations + tax liability)
- **Requires:** CPF has financial capacity to reserve 40% of donations
- **Best for:** CPF needs revenue flow and can manage reserves

**Option B: Wait for ANBI Decision (Mid/Late 2026)**
- **Pros:**
  - Can immediately offer tax deduction (stronger donor incentive)
  - No "pending" status communication needed
  - No gift tax reserve management needed
  - Clean launch with full benefits
- **Cons:**
  - Delayed revenue generation (6-12 months)
  - Platform sits idle in production-ready state during wait
  - Risk: Application rejected, further delays
- **Requires:** CPF has sufficient operating funds to wait
- **Best for:** If donor tax deductibility is critical to fundraising strategy

**ANBI Compliance Obligations (Once Granted):**
- Annual financial reporting to Dutch tax authority (Belastingdienst)
- Public disclosure of financial information
- Spending requirements (90% rule for charitable purposes)
- Restrictions on commercial activities
- Governance requirements (board independence, conflict of interest policies)

**Timeline Considerations:**

**Realistic Timeline Assumption:**
- **Q4 2024 / Q1 2025:** ANBI application preparation and submission
- **2025:** ANBI under review by Belastingdienst
- **2026:** ANBI decision (approval or denial)

| Stage | Expected Timing | Decision Impact |
|-------|-----------------|-----------------|
| **ANBI Application Preparation** | Q4 2024 / Q1 2025 | Should happen NOW (before Decision 2) |
| **ANBI Application Submission** | Q1 2025 | Can happen during Phase A/B (newsletters and App/Camino launch) |
| **ANBI Review by Belastingdienst** | Throughout 2025 | Platform can launch in test mode during review |
| **ANBI Decision** | 2026 | Triggers production donations OR gift tax handling |
| **If Approved** | 2026 | Go-live for production donations with tax deduction |
| **If Denied** | 2026 | **CPF must pay gift tax on 2026 donations** OR reapply |

**Board Decision Points:**

1. **Submit ANBI application Q1 2025?** (Recommended: Yes, start process immediately)
2. **2025 Test Mode:** Stripe test mode throughout 2025 (technical requirement - not a board decision)
3. **2026 Production Launch Timing:** Early 2026 before ANBI decision (Option A or B) OR wait for ANBI decision (Option C)?
4. **Gift tax reserve strategy:** If launching before ANBI decision, which reserve approach (Strategy A, B, or C)?
5. **What if ANBI is denied in 2026?** (Contingency: Reapply? Operate without ANBI? Pay gift tax from reserves?)
6. **Donor communication strategy?** (How to explain "ANBI pending" status if launching early)

**Why This Matters:** ANBI status is the primary tax incentive for Dutch donors. Without it, donation volume may be significantly lower. However, waiting for approval delays platform launch by 3-6 months. This is a strategic trade-off between revenue timing and donor incentive strength.

**CRITICAL: Gift Tax Reserve Requirement if ANBI Not Yet Confirmed:**

If CPF accepts donations in 2026 while ANBI application is pending, and ANBI is subsequently **denied**, CPF becomes liable for **gift tax** (schenkbelasting) on those donations:

- **Gift tax rate:** Up to 40% on donations (depends on donor relationship and amount)
- **Liability:** CPF must pay the tax, not the donors
- **Financial impact:** Could be substantial if significant donations received
- **Example:** €100K in 2026 donations → up to €40K gift tax owed by CPF

**Treasury Management Strategy (REQUIRED):**

If CPF chooses to accept donations in 2026 before ANBI confirmation, **financial reserves MUST be managed** to cover potential gift tax liability:

**Reserve Calculation:**
```
For each donation received before ANBI confirmation:
- Assume worst case: 40% gift tax liability
- Reserve 40% of donation amount in segregated account
- DO NOT spend reserved funds until ANBI confirmed
- If ANBI approved → Release reserves for program spending
- If ANBI denied → Use reserves to pay gift tax
```

**Example Reserve Management:**

| Month | Donations Received | 40% Reserved | Available for Programs | Total Reserves |
|-------|-------------------|--------------|----------------------|----------------|
| Jan 2026 | €10,000 | €4,000 | €6,000 | €4,000 |
| Feb 2026 | €15,000 | €6,000 | €9,000 | €10,000 |
| Mar 2026 | €20,000 | €8,000 | €12,000 | €18,000 |
| **ANBI Decision** | | | | |
| If APPROVED | - | €18,000 released | +€18,000 available | €0 (released) |
| If DENIED | - | Pay €18,000 tax | No change | €0 (paid as tax) |

**Board Oversight Required:**
- Monthly monitoring of donation reserves vs. gift tax liability
- Board approval before spending beyond available (non-reserved) funds
- Financial reports must separate "available funds" from "tax reserve"
- This is similar to crypto treasury management (thermostat model) - prudent financial planning

**Production Launch Timing & Reserve Strategies:**

**Timeline (Applies to All Options):**
- **Throughout 2025:** Stripe test mode (technical testing - happens regardless)
- **Q1 2025:** ANBI application submitted (parallel process)
- **2026:** Board decides when to enable production donations

**Strategy A: Early Production Launch with Full Reserves**
- Launch production Q1 2026 (before ANBI decision)
- Reserve 40% of ALL donations received before ANBI confirmation
- Only spend 60% until ANBI status resolved
- **Financial requirement:** CPF must have capacity to operate on 60% of donations
- **Best for:** CPF needs revenue flow and has strong financial position

**Strategy B: Modest Production Launch with Scaled Reserves**
- Launch production Q1 2026 with modest goals (€25K-50K initially)
- Reserve 40% of early 2026 donations
- Scale up donation campaigns after ANBI decision
- **Financial requirement:** CPF can manage reserves on smaller donation volume
- **Best for:** Balances revenue needs with prudent financial management (RECOMMENDED)

**Strategy C: Delayed Production Launch**
- Continue test mode through Q1-Q2 2026
- Wait for ANBI decision (mid/late 2026) before production
- No gift tax risk, no reserve management needed
- **Financial requirement:** CPF has sufficient operating funds to wait 6-12 months
- **Best for:** CPF prioritizes tax deductibility for donors over early revenue

**Board Decision Required:**
Which strategy aligns with CPF's financial position and risk tolerance?

**Key Question:** Can CPF operate effectively while reserving 40% of donations, or does CPF need full donation amounts to cover operating costs?

---

### Medium-Term (4-8 Months)

**Decision 3:** Enable NFT awards on donations?

**What it means:**
- Automatically mint NFTs on Polygon blockchain as donation receipts
- Connect CPF-owned smart contract with 4T NFT smart contract

**Legal & Strategic Implications:**
- **Securities Law Risk:** NFTs could be considered securities in some jurisdictions
  - **Mitigation:** Donations are voluntary contributions, not investments
  - **Mitigation:** NFTs are receipts/thank-you gifts, not profit-sharing instruments
  - **Risk:** Regulatory uncertainty around NFTs (evolving landscape)
- **Tax Implications:**
  - Donors receiving NFTs may trigger gift tax reporting thresholds
  - NFT value must be documented for tax purposes
  - CPF may need to report NFT issuance to tax authorities
- **Smart Contract Risk:** Code bugs could result in lost funds or incorrect NFT awards
  - **Mitigation:** Audit smart contracts before deployment
  - **Mitigation:** Start with small batches and manual approval
- **Blockchain Permanence:** NFT records are permanent and public
  - **Note:** Donation amounts are NOT visible on blockchain (Stripe processes payments off-chain)
  - Only NFT issuance and recipient address are on-chain
  - Cannot "delete" NFT records (GDPR complication for recipient address)
- **Gas Fee Obligations:** CPF pays Ethereum/Polygon fees for minting
  - Budget required for ongoing gas costs
  - Price volatility of ETH/MATIC affects costs

**Board Decision Point:** Are we comfortable with regulatory uncertainty around NFTs and the permanence of blockchain records?

**Why This Matters:** NFTs are cutting-edge technology with unclear regulatory treatment. Starting with manual approval gives you control before full automation.

---

**Decision 4:** Which milestone triggers replacing Wix main site?

**Note:** If path-based routing is chosen, this decision must happen BEFORE Decisions 1a/1b (newsletters and app launch require main domain on IC).

**Option 1: Content-Only Migration**
- **When:** Internet Computer version has same content/features as Wix
- **What's live:** Public website, newsletters (Phase A if subdomain OR integrated if path-based), Cool Planet App with Camino (Phase B)
- **What's NOT yet:** Donations, NFTs, gated features (webinars/forums), blog comments
- **Legal Implications:**
  - GDPR obligations (already triggered in Decisions 1a/1b)
  - Educational content standards (already triggered in Decision 1b)
  - App-based feedback moderation (already triggered in Decision 1b)
  - No financial regulations yet
  - Blog comments moderation (only if comments launched)
- **Strategic Risk:** Low - like-for-like replacement
- **Brand Risk:** Minimal if content is equivalent
- **SEO Risk:** Managed with proper DNS/redirect strategy
- **ANBI Impact:** None - no donations yet
- **Best if:** Want fastest migration with lowest risk

**Option 2: With Donation Capability**
- **When:** Donations working smoothly on Internet Computer + ANBI status resolved
- **What's live:** All of Option 1 + Stripe integration (Phase C) + KYC processes
- **What's NOT yet:** Automated NFT awards, gated features (webinars/forums)
- **Legal Implications:**
  - **KYC/AML obligations** (Decision 2) must be functioning well
  - **Financial reporting** systems must be tested
  - **ANBI compliance** (if granted) or gift tax handling (if denied)
  - Higher regulatory scrutiny due to financial transactions
- **Strategic Risk:** Medium - adding financial functionality
- **Brand Risk:** Higher - financial transactions are sensitive
- **Operational Risk:** Need proven donation processing before main site cutover
- **ANBI Impact:** Major - donation functionality only makes sense with ANBI clarity
- **Best if:** Want main fundraising capability integrated from day one

**Option 3: With Full NFT Automation & Gated Features**
- **When:** NFT awards automated and proven reliable + ANBI status resolved + gated features launched
- **What's live:** All of Option 2 + smart contracts (Phase D) + automated NFT minting + gated app features (Phase E: webinars, advanced Camino, forums)
- **What's NOT yet:** Blog comments, full social platform (Phase 2)
- **Legal Implications:**
  - **All previous obligations** plus NFT regulatory uncertainty
  - **Securities law exposure** (mitigated but present)
  - **Smart contract audit** required before automation
  - **Tax reporting complexity** for NFT issuance + ANBI interaction
  - **Forum moderation** for gated discussion forums
- **Strategic Risk:** Higher - cutting-edge technology with regulatory unknowns
- **Brand Risk:** Can be controversial, or innovative depending on positioning
- **Technical Risk:** Smart contract bugs could affect donations
- **Reputational Opportunity:** Innovative foundation using cutting-edge technology
- **ANBI Impact:** Complex - NFT gifts may complicate ANBI compliance reporting
- **Best if:** Want to showcase innovation and full platform capabilities

**This is YOUR strategic decision** - the technical team will tell you when each milestone is ready, but you decide:
1. How much regulatory/legal complexity you're comfortable with
2. How much operational testing you need before cutover
3. What level of risk aligns with CPF's mission and values
4. **How ANBI decision timeline affects your choice** (Options 2-3 only make sense with ANBI clarity)

---

## How Decisions Connect to User Journey

Each decision point above corresponds to a "gate" in the user journey funnel. Each gate has different legal/compliance implications:

### Gate 1: Website → Newsletter Signup
**Decision Required:** Decision 1a (Launch Newsletters - Phase A)
**Triggers:**
- GDPR compliance (email collection)
- Privacy policy requirement
- Data subject rights handling

**Board Oversight:**
- Review privacy policy before launch
- Approve data retention policies
- Understand right-to-deletion obligations

**Timing:** Imminent

---

### Gate 2: Newsletter → App/Camino (Educational Content)
**Decision Required:** Decision 1b (Launch App/Camino - Phase B)
**Triggers:**
- Internet Identity requirement for access
- Educational content standards and quality
- Intellectual property and attribution
- Educational material liability (accuracy, safety)
- Additional GDPR obligations for learning progress data

**Board Oversight:**
- Approve educational content standards
- Review content accuracy and safety processes
- Approve data retention for learning progress
- Understand educational liability exposure

**Timing:** Soon after newsletters

---

### Gate 3: App/Camino → Donations
**Decision Required:** Decision 2 (Stripe production)
**Triggers:**
- KYC/AML obligations (critical compliance requirement)
- Financial reporting obligations
- ANBI status decision (tax deductibility)
- Gift tax reserve management (if pre-ANBI)
- PCI compliance (handled by Stripe)

**Board Oversight:**
- **Review and approve KYC/AML procedures** (this is a big one!)
- **Decide on ANBI timing and gift tax reserve strategy** (A, B, or C)
- Approve donation tiers and KYC thresholds
- Review refund policy
- Ensure proper financial tracking/reporting
- Monitor gift tax reserves monthly (if launching before ANBI)

**Why This Matters Most:** This gate has the highest regulatory burden AND financial implications. You're moving from educational content to handling money.

---

### Gate 4: Donations → NFT Awards
**Decision Required:** Decision 3 (NFT automation)
**Triggers:**
- Securities law considerations (regulatory uncertainty)
- Tax implications for NFT recipients
- Smart contract audit requirements
- Blockchain permanence (GDPR complications)
- Ongoing gas fee obligations

**Board Oversight:**
- **Obtain legal opinion on NFT securities status** (recommended)
- Approve NFT terms of service
- Review smart contract audit results
- Approve budget for gas fees
- Understand tax reporting requirements

**Why This Matters:** NFTs are innovative but legally uncertain. Get legal advice before automating.

---

### Gate 5: NFT Awards → Gated Features in Cool Planet App
**Decision Required:** Phase E (Add gated features to Cool Planet App)
**Triggers:**
- Webinars and on-demand replays (gated access)
- Advanced Camino modules (Climbing the Mountain)
- Gated discussion forums (requires Camino completion)
- Access control based on achievements/donations/NFT ownership
- Enhanced community moderation for private forums

**Board Oversight:** (Future)
- Approve gated access criteria (Camino completion, NFT ownership, etc.)
- Review advanced features and benefits
- Approve forum moderation policies
- Decide on gating strategy (achievement-based vs. donation-based)

**Note:** Gated features are added to the existing Cool Planet App - not a separate "members portal". The same app serves everyone, with progressive feature unlocks.

---

### Gate 6: (Later Phase) → Comments
**Decision Required:** Future phase after initial launch (not part of Decision 1)
**Triggers:**
- Content moderation obligations
- Community guidelines requirement
- Potential liability for user content

**Phased Rollout Approach:**
Comments can be rolled out gradually to manage moderation burden:
1. **Phase 1:** Board members and close advisors only (private testing)
2. **Phase 2:** Invited beta testers (vetted community members)
3. **Phase 3:** Newsletter subscribers (wider but engaged audience)
4. **Phase 4:** Public comments (full access)

**Board Oversight:** (Future)
- Approve community guidelines
- Review moderation approach (manual vs. automated)
- Decide on phased rollout timing
- Understand liability exposure for harmful content
- Monitor moderation burden at each phase before expanding

**Note:** Comments come AFTER newsletters and App/Camino launch, not as part of initial releases.

---

### Gate 7: (Phase 2 - Much Later) → Full Social Platform
**Decision Required:** Future phase (Phase 2 - not yet)
**Triggers:**
- Advanced community features (discussion forums, social networking)
- Community governance and moderation at scale
- Advanced privacy for member interactions
- Think: MightyNetworks-style community or more decentralized alternative

**Board Oversight:** (Future)
- Approve community governance model
- Review social platform moderation policies
- Decide on centralized vs. decentralized approach

---

## Legal/Compliance Checklist by Decision

### Before Decision 1a (Newsletters - Phase A):
- [ ] Privacy policy drafted and reviewed
- [ ] GDPR compliance plan in place
- [ ] Data retention policy defined (newsletter subscribers)
- [ ] Email collection and distribution process established
- [ ] Data protection officer identified (if required)

**Note:** This is the imminent first release.

### Before Decision 1b (App/Camino - Phase B):
- [ ] Privacy policy updated for learning progress data
- [ ] Educational content standards established
- [ ] Content accuracy and safety review process
- [ ] Intellectual property and attribution policies
- [ ] Data retention policy for learning progress
- [ ] Internet Identity integration tested

**Note:** This launches soon after newsletters. Comments are NOT part of Decisions 1a/1b - they come later with separate moderation requirements.

### Before Decision 2 (Donations via Stripe):
- [ ] **ANBI application submitted to Belastingdienst** (3-6 month process)
- [ ] Board decision on gift tax reserve strategy (A, B, or C)
- [ ] **Gift tax reserve account established** (40% of pre-ANBI donations)
- [ ] **Financial tracking system** to segregate reserved vs. available funds
- [ ] Board-approved spending limits (only spend non-reserved funds until ANBI confirmed)
- [ ] **KYC/AML procedures documented and approved**
- [ ] KYC provider selected and vetted (for >$1K donations)
- [ ] Financial reporting system established (required for ANBI)
- [ ] Donation refund policy approved
- [ ] Tax receipt process defined (ANBI receipts if granted)
- [ ] Stripe terms of service reviewed and accepted
- [ ] **Legal review of KYC/AML compliance** (strongly recommended)
- [ ] Donor communication strategy for "ANBI pending" period
- [ ] Monthly gift tax reserve monitoring process established

### Before Decision 3 (NFT Automation):
- [ ] **Legal opinion obtained on NFT securities status**
- [ ] Smart contract security audit completed
- [ ] NFT terms of service drafted and reviewed
- [ ] Tax implications for donors understood and documented
- [ ] Gas fee budget approved
- [ ] GDPR implications of blockchain permanence addressed
- [ ] Manual approval process tested before automation

### Before Phase E (Gated Features in Cool Planet App):
- [ ] Access control system implemented (Camino completion, NFT ownership, donations)
- [ ] Webinar platform selected and integrated
- [ ] On-demand replay infrastructure ready
- [ ] Advanced Camino modules (Climbing the Mountain) content prepared
- [ ] Gated forum moderation policies established
- [ ] Privacy policy updated for forum data
- [ ] Community guidelines for gated forums approved

**Note:** This is for future gated features (webinars, advanced modules, forums) - not a separate "members portal".

### Before Decision 4 (Main Site Cutover):
Depends on which milestone option chosen:
- **Option 1 (Content-Only):** All Decision 1a/1b checklists must be complete
- **Option 2 (With Donations):** All Decision 1a/1b + Decision 2 checklists must be complete + ANBI status resolved
- **Option 3 (With NFTs & Gated Features):** All Decision 1a/1b + 2 + 3 + Phase E checklists must be complete + ANBI status resolved

---

## Recommended Professional Support

Based on the legal implications above, consider engaging:

**For Decision 0 (ANBI Application):**
- **Dutch tax advisor** - Assist with ANBI application preparation and submission (~€2K-5K)
- **Foundation attorney** - Review statutes/bylaws for ANBI compliance (~€3K-8K)
- **Financial advisor** - Prepare 3-year financial projections

**For Decision 2 (Donations):**
- **Financial compliance attorney** - Review KYC/AML procedures
- **Tax advisor** - Ensure proper donation tracking and ANBI-compliant receipts
- **Stripe compliance specialist** - Ensure proper integration
- **ANBI compliance specialist** - Ongoing reporting to Belastingdienst

**For Decision 3 (NFTs):**
- **Securities attorney** - Opinion on NFT regulatory status
- **Smart contract auditor** - Security review before deployment
- **Tax advisor** - NFT gift tax implications + ANBI compliance interaction

**Ongoing:**
- **Data protection officer** - GDPR compliance monitoring
- **Content moderation service** - If comment volume is high
- **ANBI annual reporting** - Financial reporting to Belastingdienst (if granted)

**Cost Consideration:** Legal/compliance costs are part of platform launch. Budget for professional reviews before each major decision point.

**ANBI-Specific Costs:**
- Application preparation: €2K-5K
- Annual ANBI reporting: €1K-3K/year (ongoing)
- Contingency for reapplication if denied: €2K-5K

---

## Common Questions

### "Do I need to understand blockchain or cryptocurrency?"

**No.** You need to understand:
- What decisions you're voting on (in plain language)
- What the risks and benefits are
- What the rollback plan is

The technical team handles the "how" - you handle the "what" and "when".

### "What if I make the wrong vote?"

The voting system has built-in safety:
- **3-day delays** give time for discussion before changes happen
- **Rollback plans** for every major change
- **Unanimous votes** can override previous decisions
- **Nothing is irreversible** (except transferring ENS ownership, which requires unanimous vote)

### "How much time will this take?"

**Expected time commitment:**
- **During setup (next 2 months):** ~2-4 hours/month total (includes learning curve)
- **Ongoing (after launch):** ~30-60 minutes/month for routine votes
- **Major decisions:** Extra time for discussion, but infrequent (quarterly)

Most routine operations don't need votes - technical team handles them.

### "How does ANBI status affect our spending and operations?"

**ANBI has specific requirements that affect how CPF operates:**

**90% Spending Rule:**
- At least 90% of income must be spent on charitable objectives
- Only 10% can go to fundraising, administration, or reserves (cumulative over 5 years)
- **Platform impact:** Technical infrastructure costs count as charitable spending if they directly support mission

**Public Disclosure:**
- Financial information must be publicly available (via Belastingdienst website)
- Annual reports must be published
- **Transparency benefit:** Aligns with our open-source approach

**Activity Requirements:**
- Must demonstrate actual charitable activities (not just fundraising)
- Educational content (Camino platform) helps demonstrate public benefit
- Community building counts as charitable activity

**Governance Requirements:**
- Board independence (no more than 1/3 can have close relationships)
- Conflict of interest policies required
- **Already planned:** Time-based voting thresholds provide governance checks

**Why This Matters for Platform Design:**
The platform architecture (educational content, community forums, transparent governance) actually **strengthens** the ANBI application by demonstrating genuine public benefit activities beyond just collecting donations.

---

### "Can I experiment without breaking anything?"

**Yes!** We'll set up a **test environment** where you can:
- Practice voting on proposals
- Try out the donation flow with test money
- See how NFT awards work
- Learn the system risk-free

**Nothing in the test environment affects production.**

### "What happens if two board members disagree?"

**Good!** Disagreement means discussion is needed:
1. Proposal enters discussion period
2. Board members share perspectives
3. Technical team provides more info if needed
4. Vote happens when consensus is reached
5. If no consensus, proposal is withdrawn and revised

The time delays are specifically designed to **encourage discussion**, not force decisions.

---

## Next Steps

### For You (Board Members)

1. **✅ You have Internet Identity** - First step complete!

2. **Test your access** (when ready):
   - Log in to governance.cpf.nft (board interface)
   - See the voting interface
   - Try a test vote on a mock proposal
   - *Technical team will walk you through this*

3. **Provide feedback** on comfort level:
   - What decisions do you want to vote on?
   - What are you comfortable delegating?
   - What rollback safeguards do you need?
   - How much technical detail do you want?

4. **Decide on cutover milestone** (no rush):
   - Review the three options
   - Discuss risk tolerance as a board
   - Can revisit this decision as you gain confidence

### For Technical Team

1. **Set up governance system** based on board feedback
2. **Create test environment** for board practice
3. **Deploy Phase A** (newsletters portal - imminent) when board is comfortable
4. **Deploy Phase B** (App/Camino - soon after Phase A)
5. **Report progress** at each milestone with plain-language summaries

---

## More Information (Optional Reading)

If you want to dive deeper into any topic, here are the detailed technical documents:

**Governance & Decision-Making:**
- [governance-policy.md](./governance-policy.md) - Detailed voting rules and approval tiers
- [architectural_decisions.md](./architectural_decisions.md) - Why we made key technical choices

**Platform Architecture:**
- [canister-architecture-diagram.md](./canister-architecture-diagram.md) - Complete technical overview
- [user-journey-funnel.md](./user-journey-funnel.md) - How users experience the platform

**Production Migration:**
- [canister-architecture-diagram.md § Production Migration](./canister-architecture-diagram.md) - Detailed rollout plan with three milestone options

**Treasury Management:**
- [crypto_funding.md](./crypto_funding.md) - How the cryptocurrency "thermostat" works

**Remember:** You don't need to read these to do your job as board members. They're here if you're curious or want more detail on specific topics.

---

## Your Feedback Matters

We need your input on:

1. **Legal/Compliance Comfort Level**
   - Are you comfortable with the KYC/AML obligations for donations?
   - Do you want legal review before each major decision?
   - What professional support do you want engaged before proceeding?
   - Are you comfortable with NFT regulatory uncertainty, or wait for clarity?

2. **Decision Pace & Sequencing**
   - Should we tackle decisions sequentially or in parallel?
   - Do you want to pause between decisions for assessment?
   - Is this moving too fast or too slow?
   - What would make you more confident at each stage?

3. **Decision Delegation**
   - What decisions do you want to vote on?
   - What are you comfortable delegating to technical team?
   - Where do you want veto power vs. trust & verify?
   - **Critical:** Are you comfortable delegating routine compliance operations (under €2K, bug fixes) to technical team?

4. **Communication Preferences**
   - How much legal/compliance detail do you want?
   - Do you want plain-language summaries or full legal reviews?
   - How often do you want updates?
   - What format works best (written, video calls, in-person)?

5. **Risk Tolerance**
   - Which cutover milestone feels right given legal implications?
   - What rollback safeguards do you need?
   - What legal/regulatory exposure keeps you up at night?
   - Are you willing to be "first movers" with NFT automation, or prefer to wait and see?

6. **Resource Allocation**
   - What budget for legal/compliance support?
   - Should we engage attorneys before each decision or as-needed?
   - Who should be responsible for compliance monitoring (board member, staff, external)?

7. **Financial Capacity for Gift Tax Reserves**
   - Can CPF maintain 40% of donations in reserve until ANBI confirmed?
   - What's CPF's current cash position and operating runway?
   - Does Option B (Phased Launch with modest 2026 goals) or Option C (Delayed Launch) make more sense given CPF's finances?
   - Who will monitor and report on gift tax reserves monthly?

**Let's schedule a discussion** to walk through this together and get your feedback. We strongly recommend including legal counsel in the discussion for Decision 2 (donations/KYC) and Decision 3 (NFTs).

---

**Questions?** Ask anytime. If this document is confusing, that's on us to clarify, not on you to figure out.

---

## Summary: Your Strategic Role

As board members, you provide **strategic oversight**, not technical implementation:

### You Decide:
✅ **WHEN** to launch each feature (based on legal/compliance readiness)
✅ **WHETHER** we're comfortable with regulatory obligations
✅ **HOW MUCH** professional support to engage
✅ **WHICH** cutover milestone aligns with CPF's risk tolerance
✅ **HOW** to manage gift tax reserves until ANBI confirmed (financial planning)
✅ **WHETHER** CPF can maintain 40% reserves or needs to delay donation launch

### Technical Team Executes:
🔧 HOW to implement features
🔧 Technical operations and maintenance
🔧 Bug fixes and routine updates
🔧 Compliance monitoring (within approved frameworks)

### The Critical Questions for Your Next Discussion:

1. **Decision 0 (ANBI):** Submit application immediately?
   - Do we have all required documentation ready?
   - Who will coordinate with Belastingdienst?
   - What's our contingency if denied?

2. **Decision 2 (Donations):** When to launch production donations in 2026?
   - 2025 test mode happens regardless (technical testing)
   - Launch production early 2026 (before ANBI) OR wait for ANBI decision?
   - If launching early, which gift tax reserve strategy (A: Full, B: Modest, C: Delayed)?
   - Can CPF operate on 60% of donations while reserving 40%?
   - Do we need legal review of KYC procedures before proceeding?
   - What budget for compliance support?

3. **Decision 3 (NFTs):** Are we comfortable with regulatory uncertainty?
   - Should we wait for clearer regulations?
   - If proceeding, obtain legal opinion on securities status?
   - How do NFTs interact with ANBI compliance?

4. **Decision 4 (Cutover):** Which milestone triggers main site replacement?
   - Option 1 (Content-Only): No financial features yet
   - Option 2 (With Donations): Donations working and tested + ANBI status resolved
   - Option 3 (With NFTs): Full NFT automation proven + ANBI status resolved

5. **Resource Allocation:** What professional support budget?
   - ANBI application support: ~€2K-5K (tax advisor)
   - Legal reviews: ~€5K-15K per major decision
   - Smart contract audit: ~€20K-50K
   - Ongoing compliance monitoring: TBD

### Action Items for Board:

- [ ] **URGENT:** Assess ANBI application readiness (documentation, financials)
- [ ] **URGENT:** Decide whether to submit ANBI application now (recommended: yes)
- [ ] Assign ANBI application coordinator (board member or staff)
- [ ] Review this document before board meeting
- [ ] **CRITICAL:** Decide on gift tax reserve strategy (A: Conservative, B: Phased, C: Delayed)
- [ ] **CRITICAL:** Establish financial tracking to segregate gift tax reserves from available funds
- [ ] Assess CPF's ability to maintain 40% reserves until ANBI confirmed (cash flow analysis)
- [ ] Discuss comfort level with KYC/AML obligations (Decision 2)
- [ ] Decide on legal support budget and engagement approach
- [ ] Choose cutover milestone (Option 1, 2, or 3)
- [ ] Clarify delegation boundaries (what requires board vote vs. technical team autonomy)
- [ ] Schedule follow-up discussion with legal counsel for Decision 2 and 3
- [ ] Appoint treasurer/financial officer to monitor gift tax reserves monthly

**Welcome to governing the Cool Planet Platform!**

Your role is to ensure CPF moves forward responsibly, balancing innovation with legal compliance and mission alignment.
