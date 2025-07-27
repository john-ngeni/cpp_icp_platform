# Portfolio Dashboard & Wallet Management for CPP Platform

## Overview

This document outlines the **portfolio dashboard and wallet management architecture** for the Cool Planet Platform (CPP), including wallet cache integration, sponsorship management, and real-time portfolio updates.

## Wallet Cache Integration

### **Wallet Cache Data Structure**

The wallet cache provides efficient access to user holdings and sponsorship data:

```typescript
// Wallet cache types
type DID = string;
type TokenId = number;

// What a user holds (directly)
type Holding = {
  token_id: TokenId;
  acquired_via: 'donation' | { sponsorship: DID };
  timestamp: number;
};

// What a user has sponsored
type Sponsorship = {
  sponsoree: DID;
  token_id: TokenId;
  timestamp: number;
};

// User wallet cache entry
type WalletCache = {
  holdings: Holding[];
  sponsored: Sponsorship[];
  sponsorship_limit: number;
  sponsorship_used: number;
};

// Wallet cache update event
type WalletCacheUpdate = {
  user_did: DID;
  cache: WalletCache;
  update_type: 'HOLDING_ADDED' | 'SPONSORSHIP_ADDED' | 'LIMIT_UPDATED';
  timestamp: number;
};
```

### **Frontend Wallet Cache Integration**

The frontend integrates with the wallet cache for real-time portfolio management:

```typescript
// Wallet cache integration
class WalletCacheManager {
  private eventManager: RealTimeUpdateManager;
  private coreUserManagement: CoreUserManagementClient;

  constructor(eventManager: RealTimeUpdateManager, coreUserManagement: CoreUserManagementClient) {
    this.eventManager = eventManager;
    this.coreUserManagement = coreUserManagement;
    this.setupWalletCacheHandlers();
  }

  private setupWalletCacheHandlers() {
    this.eventManager.subscribe('WALLET_CACHE_UPDATED', (update: WalletCacheUpdate) => {
      this.handleWalletCacheUpdate(update);
    });
  }

  private async handleWalletCacheUpdate(update: WalletCacheUpdate) {
    // Update portfolio display
    await this.updatePortfolioDisplay(update.cache);
    
    // Check sponsorship limits
    this.checkSponsorshipLimits(update.cache);
    
    // Update navigation indicators
    this.updateNavigationIndicators(update.cache);
    
    // Show notification if needed
    this.showWalletUpdateNotification(update);
  }

  public async loadWalletCache(userDID: string): Promise<WalletCache> {
    try {
      const cache = await this.coreUserManagement.getWalletCache(userDID);
      await this.updatePortfolioDisplay(cache);
      return cache;
    } catch (error) {
      console.error('Failed to load wallet cache:', error);
      throw error;
    }
  }

  private async updatePortfolioDisplay(cache: WalletCache) {
    // Update holdings display
    this.updateHoldingsDisplay(cache.holdings);
    
    // Update sponsorship display
    this.updateSponsorshipDisplay(cache.sponsored);
    
    // Update sponsorship limits
    this.updateSponsorshipLimits(cache.sponsorship_limit, cache.sponsorship_used);
    
    // Update portfolio summary
    this.updatePortfolioSummary(cache);
  }

  private updateHoldingsDisplay(holdings: Holding[]) {
    const holdingsContainer = document.getElementById('holdings-container');
    if (!holdingsContainer) return;

    holdingsContainer.innerHTML = holdings.map(holding => `
      <div class="holding-item" data-token-id="${holding.token_id}">
        <div class="holding-type">
          <span class="badge ${holding.acquired_via === 'donation' ? 'badge-donation' : 'badge-sponsorship'}">
            ${holding.acquired_via === 'donation' ? 'Donation' : 'Sponsored'}
          </span>
        </div>
        <div class="holding-details">
          <h4>NFT #${holding.token_id}</h4>
          <p>Acquired: ${new Date(holding.timestamp).toLocaleDateString()}</p>
          ${holding.acquired_via !== 'donation' ? 
            `<p>Sponsored by: ${this.formatDID(holding.acquired_via.sponsorship)}</p>` : 
            ''
          }
        </div>
        <div class="holding-actions">
          <button onclick="viewNFT(${holding.token_id})">View NFT</button>
          <button onclick="shareNFT(${holding.token_id})">Share</button>
        </div>
      </div>
    `).join('');
  }

  private updateSponsorshipDisplay(sponsored: Sponsorship[]) {
    const sponsoredContainer = document.getElementById('sponsored-container');
    if (!sponsoredContainer) return;

    sponsoredContainer.innerHTML = sponsored.map(sponsorship => `
      <div class="sponsorship-item" data-token-id="${sponsorship.token_id}">
        <div class="sponsorship-details">
          <h4>Sponsored NFT #${sponsorship.token_id}</h4>
          <p>Sponsored: ${new Date(sponsorship.timestamp).toLocaleDateString()}</p>
          <p>Recipient: ${this.formatDID(sponsorship.sponsoree)}</p>
        </div>
        <div class="sponsorship-actions">
          <button onclick="viewSponsoredNFT(${sponsorship.token_id})">View NFT</button>
          <button onclick="contactRecipient('${sponsorship.sponsoree}')">Contact</button>
        </div>
      </div>
    `).join('');
  }

  private updateSponsorshipLimits(limit: number, used: number) {
    const remaining = limit - used;
    const progressElement = document.getElementById('sponsorship-progress');
    const limitElement = document.getElementById('sponsorship-limit');
    
    if (progressElement) {
      const percentage = (used / limit) * 100;
      progressElement.style.width = `${percentage}%`;
      progressElement.className = `progress-bar ${percentage >= 90 ? 'progress-warning' : ''}`;
    }
    
    if (limitElement) {
      limitElement.textContent = `${used}/${limit} (${remaining} remaining)`;
    }

    // Disable sponsorship features if limit reached
    if (remaining <= 0) {
      this.disableSponsorshipFeatures();
    } else {
      this.enableSponsorshipFeatures();
    }
  }

  private updatePortfolioSummary(cache: WalletCache) {
    const totalHoldings = cache.holdings.length;
    const totalSponsored = cache.sponsored.length;
    const remainingSponsorships = cache.sponsorship_limit - cache.sponsorship_used;
    
    const summaryElement = document.getElementById('portfolio-summary');
    if (summaryElement) {
      summaryElement.innerHTML = `
        <div class="summary-stats">
          <div class="stat">
            <h3>${totalHoldings}</h3>
            <p>NFTs Held</p>
          </div>
          <div class="stat">
            <h3>${totalSponsored}</h3>
            <p>NFTs Sponsored</p>
          </div>
          <div class="stat">
            <h3>${remainingSponsorships}</h3>
            <p>Sponsorships Remaining</p>
          </div>
        </div>
      `;
    }
  }

  private checkSponsorshipLimits(cache: WalletCache) {
    const remaining = cache.sponsorship_limit - cache.sponsorship_used;
    
    if (remaining <= 0) {
      this.showWarningNotification('You have reached your sponsorship limit');
    } else if (remaining <= 2) {
      this.showInfoNotification(`You have ${remaining} sponsorships remaining`);
    }
  }

  private disableSponsorshipFeatures() {
    const sponsorButtons = document.querySelectorAll('.sponsor-button');
    sponsorButtons.forEach(button => {
      button.setAttribute('disabled', 'true');
      button.title = 'Sponsorship limit reached';
    });
  }

  private enableSponsorshipFeatures() {
    const sponsorButtons = document.querySelectorAll('.sponsor-button');
    sponsorButtons.forEach(button => {
      button.removeAttribute('disabled');
      button.title = 'Sponsor an NFT';
    });
  }

  private formatDID(did: string): string {
    return did.length > 20 ? `${did.substring(0, 10)}...${did.substring(did.length - 10)}` : did;
  }
}
```

## Sponsorship Management

### **Sponsorship UX Patterns**

The platform supports two sponsorship patterns:

```typescript
// Sponsorship pattern types
type SponsorshipPattern = 'DIRECT_SELECTION' | 'NUMBER_BASED';

// Sponsorship configuration
type SponsorshipConfig = {
  pattern: SponsorshipPattern;
  max_sponsorships: number;
  sequential_only: boolean; // For number-based pattern
  warning_threshold: number;
};

// Sponsorship manager
class SponsorshipManager {
  private config: SponsorshipConfig;
  private walletCache: WalletCacheManager;

  constructor(config: SponsorshipConfig, walletCache: WalletCacheManager) {
    this.config = config;
    this.walletCache = walletCache;
  }

  public async initiateSponsorship(
    userDID: string,
    pattern: SponsorshipPattern
  ): Promise<SponsorshipInitiation> {
    // Check sponsorship limits
    const cache = await this.walletCache.loadWalletCache(userDID);
    const remaining = cache.sponsorship_limit - cache.sponsorship_used;
    
    if (remaining <= 0) {
      throw new Error('Sponsorship limit reached');
    }

    // Validate pattern selection
    if (pattern === 'NUMBER_BASED' && this.hasAdhocSponsorships(cache)) {
      throw new Error('Number-based sponsorship not available due to existing ad-hoc sponsorships');
    }

    return {
      pattern: pattern,
      remaining_sponsorships: remaining,
      available_options: await this.getAvailableOptions(pattern, cache)
    };
  }

  public async executeDirectSponsorship(
    userDID: string,
    targetTokenId: number,
    sponsoreeDID: string
  ): Promise<SponsorshipResult> {
    // Validate sponsorship
    await this.validateSponsorship(userDID, targetTokenId, sponsoreeDID);
    
    // Execute sponsorship on Polygon
    const result = await this.executePolygonSponsorship({
      sponsor: userDID,
      sponsoree: sponsoreeDID,
      token_id: targetTokenId
    });
    
    // Update wallet cache
    await this.updateWalletCacheAfterSponsorship(userDID, result);
    
    return result;
  }

  public async executeNumberBasedSponsorship(
    userDID: string,
    numberOfNFTs: number,
    sponsoreeDID: string
  ): Promise<SponsorshipResult> {
    // Validate sequential sponsorship
    if (!this.isSequentialSponsorship(userDID, numberOfNFTs)) {
      throw new Error('Number-based sponsorship requires sequential allocation');
    }
    
    // Execute sequential sponsorship
    const result = await this.executeSequentialSponsorship({
      sponsor: userDID,
      sponsoree: sponsoreeDID,
      count: numberOfNFTs
    });
    
    // Update wallet cache
    await this.updateWalletCacheAfterSponsorship(userDID, result);
    
    return result;
  }

  private hasAdhocSponsorships(cache: WalletCache): boolean {
    // Check if user has made any non-sequential sponsorships
    const sponsoredTokens = cache.sponsored.map(s => s.token_id).sort((a, b) => a - b);
    
    for (let i = 1; i < sponsoredTokens.length; i++) {
      if (sponsoredTokens[i] !== sponsoredTokens[i-1] + 1) {
        return true; // Non-sequential sponsorship found
      }
    }
    
    return false;
  }

  private async getAvailableOptions(
    pattern: SponsorshipPattern,
    cache: WalletCache
  ): Promise<SponsorshipOption[]> {
    if (pattern === 'DIRECT_SELECTION') {
      return await this.getDirectSelectionOptions(cache);
    } else {
      return await this.getNumberBasedOptions(cache);
    }
  }

  private async getDirectSelectionOptions(cache: WalletCache): Promise<SponsorshipOption[]> {
    // Get available NFTs for direct selection
    const availableNFTs = await this.getAvailableNFTs();
    
    return availableNFTs.map(nft => ({
      type: 'DIRECT_SELECTION',
      token_id: nft.id,
      description: `NFT #${nft.id}`,
      available: true
    }));
  }

  private async getNumberBasedOptions(cache: WalletCache): Promise<SponsorshipOption[]> {
    // Get sequential NFT ranges for number-based sponsorship
    const sequentialRanges = await this.getSequentialRanges();
    
    return sequentialRanges.map(range => ({
      type: 'NUMBER_BASED',
      start_id: range.start,
      end_id: range.end,
      count: range.end - range.start + 1,
      description: `NFTs #${range.start}-#${range.end}`,
      available: true
    }));
  }
}
```

## Real-Time Portfolio Updates

### **Polygon Event Processing**

The portfolio dashboard receives real-time updates when Polygon events are processed:

```typescript
// Polygon event processing for portfolio
class PortfolioEventProcessor {
  private eventManager: RealTimeUpdateManager;
  private walletCache: WalletCacheManager;

  constructor(eventManager: RealTimeUpdateManager, walletCache: WalletCacheManager) {
    this.eventManager = eventManager;
    this.walletCache = walletCache;
    this.setupPortfolioEventHandlers();
  }

  private setupPortfolioEventHandlers() {
    this.eventManager.subscribe('POLYGON_EVENT_PROCESSED', (event: PolygonEvent) => {
      switch (event.event_type) {
        case 'award':
          this.handleAwardEvent(event);
          break;
        case 'digital_asset_price_updated':
          this.handlePriceUpdate(event);
          break;
        case 'donation_config_set':
          this.handleConfigUpdate(event);
          break;
      }
    });
  }

  private async handleAwardEvent(event: PolygonEvent) {
    // Update portfolio for the recipient
    await this.walletCache.loadWalletCache(event.did);
    
    // Show notification
    this.showAwardNotification(event);
    
    // Update portfolio summary
    this.updatePortfolioSummary();
  }

  private handlePriceUpdate(event: PolygonEvent) {
    // Update price displays in portfolio
    this.updatePriceDisplays(event.newPrice);
    
    // Recalculate portfolio value
    this.recalculatePortfolioValue();
  }

  private handleConfigUpdate(event: PolygonEvent) {
    // Update donation configuration displays
    this.updateDonationConfig(event);
    
    // Refresh sponsorship options if needed
    this.refreshSponsorshipOptions();
  }

  private showAwardNotification(event: PolygonEvent) {
    const notification = {
      type: 'success',
      title: 'New NFT Awarded',
      message: `NFT #${event.startIndex} has been awarded to ${this.formatDID(event.did)}`,
      duration: 5000
    };
    
    this.showNotification(notification);
  }

  private updatePriceDisplays(newPrice: number) {
    const priceElements = document.querySelectorAll('.nft-price');
    priceElements.forEach(element => {
      element.textContent = `$${newPrice.toFixed(2)}`;
    });
  }

  private async recalculatePortfolioValue() {
    const cache = await this.walletCache.loadWalletCache(this.getCurrentUserDID());
    const totalValue = cache.holdings.length * this.getCurrentNFTPrice();
    
    const valueElement = document.getElementById('portfolio-value');
    if (valueElement) {
      valueElement.textContent = `$${totalValue.toFixed(2)}`;
    }
  }
}
```

## Portfolio Analytics

### **Portfolio Performance Tracking**

The dashboard provides analytics on portfolio performance:

```typescript
// Portfolio analytics
class PortfolioAnalytics {
  public async generatePortfolioReport(userDID: string): Promise<PortfolioReport> {
    const cache = await this.getWalletCache(userDID);
    const transactions = await this.getTransactionHistory(userDID);
    
    return {
      total_holdings: cache.holdings.length,
      total_sponsored: cache.sponsored.length,
      portfolio_value: this.calculatePortfolioValue(cache),
      average_holding_time: this.calculateAverageHoldingTime(cache),
      sponsorship_efficiency: this.calculateSponsorshipEfficiency(cache),
      recent_activity: this.getRecentActivity(transactions),
      performance_metrics: await this.calculatePerformanceMetrics(cache, transactions)
    };
  }

  private calculatePortfolioValue(cache: WalletCache): number {
    const currentPrice = this.getCurrentNFTPrice();
    return cache.holdings.length * currentPrice;
  }

  private calculateAverageHoldingTime(cache: WalletCache): number {
    if (cache.holdings.length === 0) return 0;
    
    const totalTime = cache.holdings.reduce((sum, holding) => {
      return sum + (Date.now() - holding.timestamp);
    }, 0);
    
    return totalTime / cache.holdings.length / (1000 * 60 * 60 * 24); // Days
  }

  private calculateSponsorshipEfficiency(cache: WalletCache): number {
    if (cache.sponsorship_limit === 0) return 0;
    
    return (cache.sponsored.length / cache.sponsorship_limit) * 100;
  }

  private async calculatePerformanceMetrics(
    cache: WalletCache,
    transactions: Transaction[]
  ): Promise<PerformanceMetrics> {
    return {
      total_return: this.calculateTotalReturn(cache, transactions),
      volatility: this.calculateVolatility(transactions),
      sharpe_ratio: this.calculateSharpeRatio(transactions),
      max_drawdown: this.calculateMaxDrawdown(transactions)
    };
  }
}
```

## Implementation Timeline

### **Phase 1: Core Portfolio Display (Weeks 1-2)**
- [ ] Implement wallet cache integration
- [ ] Create basic portfolio display
- [ ] Add holdings and sponsorship views
- [ ] Implement sponsorship limit tracking

### **Phase 2: Real-Time Updates (Weeks 3-4)**
- [ ] Add Polygon event processing
- [ ] Implement real-time portfolio updates
- [ ] Create notification system
- [ ] Add loading states and error handling

### **Phase 3: Sponsorship Management (Weeks 5-6)**
- [ ] Implement direct selection sponsorship
- [ ] Add number-based sponsorship (with warnings)
- [ ] Create sponsorship validation
- [ ] Add sponsorship analytics

### **Phase 4: Advanced Analytics (Weeks 7-8)**
- [ ] Implement portfolio performance tracking
- [ ] Add transaction history
- [ ] Create performance metrics
- [ ] Add portfolio reporting

## Integration with Core User Management

### **Data Flow**
1. **Frontend** requests wallet cache from Core User Management Canister
2. **Core User Management Canister** provides wallet cache data
3. **Frontend** displays portfolio and sponsorship information
4. **Polygon events** trigger real-time updates via Core User Management Canister
5. **Frontend** updates portfolio display in real-time

### **Performance Considerations**
- Wallet cache is cached locally for performance
- Real-time updates use efficient event-driven architecture
- Portfolio analytics are calculated on-demand
- Large datasets are paginated for optimal performance 