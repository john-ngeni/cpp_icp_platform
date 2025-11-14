# Crypto Treasury Funding Algorithm (Thermostat Model)

**Date:** 2025-11-14
**Purpose:** Define simple, predictable algorithm for converting fiat donations into crypto for gas reserves
**Owner:** Treasurer (operational decision, not board-level)

---

## Overview

CPP platform receives **fiat donations via Stripe** but needs **ETH/MATIC for NFT gas** and **ICP for cycles**. This document defines a "thermostat" algorithm to automatically advise when to buy crypto and how much.

**Key Principle:** Like a thermostat maintains temperature, this algorithm maintains crypto reserves at optimal levels based on spend rate and projections.

---

## Thermostat Model

### Inputs

| Input | Source | Update Frequency |
|-------|--------|------------------|
| **Current reserve balance** | On-chain balance check (ICP-derived ETH/MATIC address) | Real-time |
| **Actual spend rate** | Gas expenses logged by NFT canister | Daily aggregate |
| **Donation inflow** | Stripe revenue reports | Monthly |
| **Gas price trends** | Ethereum/Polygon gas oracle | Daily average |
| **EUR/ETH price** | Exchange API (Coinbase, CoinGecko) | Hourly |

### Parameters (Configurable)

```yaml
# Reserve thresholds (in EUR equivalent)
min_reserve: 500          # Alert threshold - buy crypto when below this
target_reserve: 2000      # Target to maintain
max_reserve: 5000         # Cap - don't buy if above this

# Spend projections
projected_monthly_nfts: 100        # Expected NFT mints per month
gas_per_nft_eur: 2.00             # Average gas cost per NFT (EUR)
projected_monthly_spend: 200       # 100 NFTs × €2 = €200/month

# Purchase parameters
purchase_amount_eur: 1000          # Standard purchase size
purchase_cooldown_days: 30         # Minimum days between purchases
allocation_percent: 3.0            # % of donations allocated to gas reserve

# ICP-specific parameters
icp_min_reserve: 10000            # Minimum ICP tokens for cycles
icp_target_reserve: 50000         # Target ICP balance
projected_monthly_cycles_cost: 100 # EUR equivalent in cycles
```

### Algorithm (Weekly Check)

```python
def check_and_advise_crypto_purchase():
    """
    Run weekly: Check reserves and advise Treasurer if purchase needed
    """
    # Step 1: Get current state
    current_balance_eur = get_eth_balance_in_eur()
    actual_spend_last_30d = get_gas_spend_last_30_days_eur()
    donation_revenue_last_30d = get_stripe_revenue_eur()

    # Step 2: Update projections based on actuals
    if actual_spend_last_30d > 0:
        # Adjust projection based on actual spend
        projected_monthly_spend = actual_spend_last_30d
        projected_monthly_nfts = get_nft_count_last_30_days()
        gas_per_nft_eur = actual_spend_last_30d / projected_monthly_nfts

    # Step 3: Calculate runway (months of reserves remaining)
    if projected_monthly_spend > 0:
        runway_months = current_balance_eur / projected_monthly_spend
    else:
        runway_months = float('inf')  # No spend yet

    # Step 4: Decision logic (thermostat)
    if current_balance_eur < min_reserve:
        # CRITICAL: Reserve depleted
        return {
            "action": "BUY_URGENT",
            "amount_eur": target_reserve - current_balance_eur,
            "reason": f"Reserve critically low: €{current_balance_eur} < €{min_reserve}",
            "runway_months": runway_months
        }

    elif current_balance_eur < target_reserve and runway_months < 3:
        # WARNING: Low runway, buy to replenish
        return {
            "action": "BUY_RECOMMENDED",
            "amount_eur": target_reserve - current_balance_eur,
            "reason": f"Runway low ({runway_months:.1f} months), replenish to target",
            "runway_months": runway_months
        }

    elif current_balance_eur > max_reserve:
        # EXCESS: Too much crypto, reduce next allocation
        return {
            "action": "REDUCE_ALLOCATION",
            "amount_eur": 0,
            "reason": f"Reserve high: €{current_balance_eur} > €{max_reserve}, reduce allocation %",
            "runway_months": runway_months
        }

    else:
        # OK: Reserve healthy
        return {
            "action": "NO_ACTION",
            "amount_eur": 0,
            "reason": f"Reserve healthy: €{current_balance_eur} ({runway_months:.1f} months runway)",
            "runway_months": runway_months
        }
```

### Adjustment Logic (Monthly)

```python
def adjust_projections_monthly():
    """
    Run monthly: Adjust projections based on actuals
    """
    # Get actuals from last month
    actual_nfts = get_nft_count_last_month()
    actual_spend_eur = get_gas_spend_last_month_eur()
    actual_donations_eur = get_stripe_revenue_last_month_eur()

    # Calculate variance
    nft_variance = (actual_nfts - projected_monthly_nfts) / projected_monthly_nfts
    spend_variance = (actual_spend_eur - projected_monthly_spend) / projected_monthly_spend

    # Adjust projections (exponential smoothing)
    alpha = 0.3  # Smoothing factor (30% weight on new data)
    projected_monthly_nfts = (alpha * actual_nfts) + ((1 - alpha) * projected_monthly_nfts)
    projected_monthly_spend = (alpha * actual_spend_eur) + ((1 - alpha) * projected_monthly_spend)
    gas_per_nft_eur = projected_monthly_spend / projected_monthly_nfts

    # Adjust allocation percentage if needed
    if spend_variance > 0.5:  # Spending 50% more than projected
        allocation_percent = min(5.0, allocation_percent + 0.5)  # Increase allocation
    elif spend_variance < -0.3:  # Spending 30% less than projected
        allocation_percent = max(1.0, allocation_percent - 0.5)  # Decrease allocation

    return {
        "projected_monthly_nfts": projected_monthly_nfts,
        "projected_monthly_spend": projected_monthly_spend,
        "gas_per_nft_eur": gas_per_nft_eur,
        "allocation_percent": allocation_percent
    }
```

---

## Treasurer Workflow

### Weekly Check (Automated)

1. **Canister runs algorithm** (Sunday 00:00 UTC)
2. **Sends email/notification to Treasurer** with recommendation:
   ```
   Subject: Crypto Reserve Weekly Report

   Current ETH reserve: €1,234 (3.2 months runway)
   Recommendation: BUY_RECOMMENDED
   Amount: €766 to reach €2,000 target

   Last 30 days:
   - NFTs minted: 87 (projected: 100)
   - Gas spent: €174 (projected: €200)
   - Avg gas/NFT: €2.00

   Action: Purchase €1,000 ETH via Coinbase
   ```

3. **Treasurer reviews** recommendation

### Purchase Execution (When Recommended)

```
Step 1: Login to Coinbase (or approved CEX)
Step 2: Wire transfer EUR from CPF bank account
        Amount: As recommended (typically €1,000)
Step 3: Wait for wire to clear (1-3 business days)
Step 4: Buy ETH at market price
Step 5: Withdraw ETH to ICP-derived address
        Address: [governance canister provides via getEthAddress()]
Step 6: Confirm delivery (canister detects balance increase)
```

### Monthly Review (Treasurer + Board)

1. **Treasurer prepares monthly crypto treasury report:**
   - Opening balance
   - Purchases made (EUR spent, ETH acquired, price)
   - Gas consumed (NFTs minted, total spend)
   - Closing balance
   - Projection accuracy (actual vs projected)
   - Recommended parameter adjustments

2. **Board reviews at monthly meeting** (informational, not approval required)

---

## ICP Funding (Cycles)

**Separate process** for ICP tokens (used for canister cycles):

### ICP Reserve Parameters

```yaml
icp_min_reserve: 10000          # Minimum ICP tokens
icp_target_reserve: 50000       # Target ICP balance
icp_max_reserve: 100000         # Cap
projected_monthly_burn: 1000    # ICP burned per month (cycles)
icp_purchase_amount: 20000      # Standard purchase size (ICP tokens)
```

### ICP Purchase Flow

1. **Treasurer buys ICP** directly on exchange (Coinbase, Kraken)
2. **Sends ICP to governance canister's ICP address**
3. **Governance canister auto-tops up** canisters using ICP → cycles conversion
4. **Same thermostat model** applies (check weekly, buy when < min_reserve)

---

## Dashboard (For Treasurer)

**Governance canister provides dashboard endpoint:**

```
GET /crypto_treasury_status

Response:
{
  "eth_reserve": {
    "balance_eth": 1.2,
    "balance_eur": 1234.56,
    "runway_months": 3.2,
    "status": "OK",
    "recommendation": "NO_ACTION"
  },
  "icp_reserve": {
    "balance_icp": 45000,
    "balance_eur": 890.00,
    "runway_months": 45.0,
    "status": "OK",
    "recommendation": "NO_ACTION"
  },
  "last_30_days": {
    "nfts_minted": 87,
    "gas_spent_eur": 174.00,
    "gas_per_nft_eur": 2.00,
    "variance_from_projection": -13.0  // % (negative = under-spending)
  },
  "projections": {
    "monthly_nfts": 95,
    "monthly_spend_eur": 190.00,
    "allocation_percent": 3.0
  },
  "next_check": "2025-11-17T00:00:00Z"
}
```

---

## Parameter Tuning

### Initial Bootstrap (Phase 0)

```yaml
# Conservative start
min_reserve: 500
target_reserve: 1000
projected_monthly_nfts: 50
gas_per_nft_eur: 3.00  # Assume higher until actuals known
```

### After 3 Months (Phase 1)

```yaml
# Adjust based on actuals
projected_monthly_nfts: <actual average>
gas_per_nft_eur: <actual average>
target_reserve: 2000  # Increase as volume grows
```

### Mature Platform (Phase 2+)

```yaml
# Higher volume, tighter margins
projected_monthly_nfts: 500+
target_reserve: 5000
allocation_percent: 2.5  # Lower % as efficiency improves
```

---

## Governance Integration

### Tier 3 (Operational) - Treasurer Authority

- ✅ Execute crypto purchases **up to €2,000/month** (within algorithm recommendations)
- ✅ Adjust allocation percentage **between 2-5%**
- ✅ Update projections monthly based on actuals

### Tier 0 (Strategic) - Board Approval Required

- ❌ Purchases **> €2,000** (extraordinary circumstances)
- ❌ Change min/target/max reserve thresholds
- ❌ Change allocation percentage **outside 2-5% range**

**Rationale:** Thermostat algorithm makes this operational (like paying electricity bill), not strategic.

---

## Risk Management

### What if reserve depletes to zero?

1. **NFT minting pauses** (canister returns error: "Gas reserve depleted")
2. **Treasurer receives URGENT alert**
3. **Emergency purchase** authorized (Tier 0 - board approval for urgent funds)
4. **Resume operations** when ETH arrives

### What if EUR/ETH price crashes 50%?

1. **Algorithm detects lower balance (EUR-equivalent)**
2. **Recommends purchase** to replenish to target
3. **Treasurer buys more ETH** (same EUR amount = more ETH tokens)
4. **Reserve back to target**

### What if gas prices spike 10x?

1. **Algorithm detects higher spend rate**
2. **Projects faster depletion** (lower runway_months)
3. **Recommends earlier purchase**
4. **May increase allocation_percent** if sustained

---

## Implementation Checklist

- [ ] Build `check_and_advise_crypto_purchase()` in governance canister
- [ ] Build `adjust_projections_monthly()` function
- [ ] Create Treasurer dashboard endpoint
- [ ] Set up weekly cron job (timer) in canister
- [ ] Configure email/Slack notifications for Treasurer
- [ ] Set up Coinbase/Kraken account for CPF
- [ ] Link CPF bank account to exchange
- [ ] Document ICP-derived ETH address for withdrawals
- [ ] Test purchase flow (small amount first)
- [ ] Define initial parameters (bootstrap values)
- [ ] Create monthly reporting template
- [ ] Add to governance policy (Tier 3 authority for Treasurer)

---

## Related Documents

- [architectural_decisions.md § OQ-007](./architectural_decisions.md) - Gas management architecture decision
- [governance-policy.md](./governance-policy.md) - Governance tiers and authority levels
- [multi-sig-governance-comparison.md](./multi-sig-governance-comparison.md) - Governance implementation

---

**Last Updated:** 2025-11-14
**Status:** Draft - Algorithm defined, awaiting implementation
**Owner:** Treasurer (operational), Board oversight (monthly review)
