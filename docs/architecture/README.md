# CPP Platform Architecture Documentation

**Date:** 2025-11-14
**Purpose:** Central hub for Cool Planet Platform (CPP) architecture documentation
**Context:** Comprehensive system architecture for NFT crowdfunding campaign and community platform
**Scope:** Technical architecture, integration patterns, security, and implementation strategies

---

## 🚀 Quick Start: Where to Begin

**NEW READERS START HERE:**

1. **[canister-architecture-diagram.md](./canister-architecture-diagram.md)** - **CRITICAL FOUNDATION**
   - Control plane vs data plane architecture
   - Bootstrap sequence and deployment topology
   - Production migration strategy (Wix → ICP)
   - This is the primary architectural overview

2. **[architectural_decisions.md](./architectural_decisions.md)** - Decision log and rationale
   - Domain routing strategy, governance structure, voting thresholds
   - Cross-references to implementation details

3. **[user-journey-funnel.md](./user-journey-funnel.md)** - User engagement progression
   - Progressive authentication gates
   - Data plane user journey (visitors → members → NFT holders)

4. **This README** - Extended feature documentation for later phases

---

## Documentation Organization

### **Phase 1: Foundational Infrastructure** (NEW - Nov 2024)

**Focus:** Control plane, governance, ENS/DNS, bootstrap, production deployment

These documents describe the **infrastructure layer** required before any user-facing features:

- **[canister-architecture-diagram.md](./canister-architecture-diagram.md)** - Complete system architecture
- **[architectural_decisions.md](./architectural_decisions.md)** - Decision log with rationale
- **[governance-policy.md](./governance-policy.md)** - Board governance and approval tiers
- **[admin-architecture.md](./admin-architecture.md)** - Control plane security patterns
- **[ens-dns-setup.md](./ens-dns-setup.md)** - ENS/DNS bootstrap sequence
- **[crypto_funding.md](./crypto_funding.md)** - Thermostat algorithm for crypto treasury
- **[multi-sig-governance-comparison.md](./multi-sig-governance-comparison.md)** - Governance options
- **[derivation-origins-integration.md](./derivation-origins-integration.md)** - Identity architecture
- **[origins.md](./origins.md)** - Detailed derivation origin analysis

**Implementation Priority:** These must be deployed FIRST (Phases 0-4 in bootstrap sequence).

### **Phase 2+: Community & Social Features** (OLDER - Jul 2024)

**Focus:** User-facing features, KYC, donations, NFTs, discussion forums, calendar, video conferencing

These documents describe **advanced features** that will be implemented AFTER foundational infrastructure:

- Community management, discussion forums
- KYC/identity verification, donations, NFT portfolio
- Calendar, Jitsi video integration
- Wallet cache, sponsorship ledger
- Educational content (Camino platform)

**Implementation Priority:** These are deployed AFTER control plane is operational and production migration begins.

**Note:** The remainder of this README focuses on Phase 2+ features. See documents listed above for foundational infrastructure.

---

## Platform Selection & Architecture Overview

The Cool Planet Platform (CPP) is a comprehensive system combining **NFT crowdfunding**, **community management**, and **educational content** on the Internet Computer (IC) with Polygon blockchain integration.

**Architecture Model:** Two-plane separation
- **Control Plane** (authors.cpf.nft): Infrastructure, governance, upgrades - THIS REPO
- **Data Plane** (cpf.nft): User-facing operations, content, donations - OTHER REPOS

See [canister-architecture-diagram.md](./canister-architecture-diagram.md) for complete architecture overview.

### **Why Internet Computer (IC)?**

Our platform selection analysis evaluated multiple blockchain architectures and determined that **IC provides the optimal foundation** for CPP's requirements. See [Platform Comparison Analysis](platform-comparison.md) for detailed evaluation of:

- **Ethereum Primary**: High vendor dependencies (15+ vendors), complex authentication
- **DFINITY Primary**: Limited standards compliance, proprietary solutions  
- **Hybrid Ecosystem (IC + Polygon)**: **RECOMMENDED** - Best balance of user experience, standards compliance, and operational efficiency

**Key IC Advantages:**
- **73% vendor reduction** compared to Ethereum-based solutions
- **Internet Identity** eliminates seed phrase complexity for mainstream users
- **Swiss jurisdiction** for authentication and dApp sovereignty
- **Proven scalability** for 2.9M NFT campaign requirements
- **Cost-effective operations** with native IC cycles
- **Progressive Web App** eliminates App Store dependencies

### **Core Architecture Principles**

- **Two-Plane Architecture**: Control plane (governance) + Data plane (user operations)
- **Hybrid Canister Strategy**: Motoko for core operations, Rust for payment bridge
- **Frontend**: Svelte (primary), React (newsletter Admin only)
- **Security Through Transparency**: Comprehensive open source with externalized secrets
- **Provider Integration**: External KYC providers with webhook-based processing
- **Multi-Chain Support**: IC for dApp backend, Polygon for NFTs and DIDs
- **Progressive Web App**: Modern, responsive user experience
- **IC-Native Architecture**: Internet Identity, asset canisters, chain-key cryptography

---

## Phase 2+ Feature Documentation

The sections below describe **advanced community and social features** planned for later implementation phases. These features depend on the foundational infrastructure documented in Phase 1.

**Context:** These documents were created in July 2024 and describe the vision for full platform features. They will be implemented AFTER:
- Control plane is deployed and operational
- Production migration to ICP is complete
- Basic data plane (newsletters, members portal) is stable

**⚠️ Technology Update:** Many documents below reference outdated technology stacks:
- **TypeScript/Azle**: No longer being used
- **V0.dev/React**: Superseded by **Svelte** (except newsletter Admin which remains React)
- **builder.io**: Status uncertain for Svelte integration
- See [frontend-technology-evaluation.md](./frontend-technology-evaluation.md) for historical context (note: this doc is also outdated and describes React + V0.dev approach)

---

## Architecture Documents (Phase 2+ Features)

**Note:** The documents below describe the **feature vision** but may reference outdated technology stacks. Use them for understanding **what** features are planned, not necessarily **how** they'll be implemented technically.

### **User Features & Flows**

- **[Identity Verification](identity-verification.md)** - KYC flow, consent management, external provider integration
- **[Portfolio Dashboard](portfolio-dashboard.md)** - Wallet cache, NFT portfolio, sponsorship management
- **[Core User Management](kyc.md)** - Motoko backend for KYC, risk assessment, compliance
- **[User Identity States](user-identity-states.md)** - User lifecycle, state transitions, event-driven architecture
- **[Frontend Architecture](frontend-architecture.md)** - **Crypto-native donation flow** where users donate directly using cryptocurrency (not Stripe). Describes CPF-owned KYC process and wallet integration patterns for future crypto-native path.

### **Community & Social Features**

- **[Community Strategy](community-strategy.md)** - Discussion platforms (OpenChat, Matrix/DeltaChat), community scaling
- **[Calendar Architecture](calendar.md)** - Meeting management, event coordination, IC ecosystem integration
- **[Jitsi Integration Architecture](jitsi-integration.md)** - Video conferencing with Internet Identity

### **Advanced Integration**

- **[Bundle Holder Integration](bundle-holder-integration.md)** - ENS integration for distributed content
- **[Wallet Association](wallet-association.md)** - External wallet association with Internet Identity

### **Security & Open Source**

- **[Security Architecture](security.md)** - Data encryption, GDPR compliance, KYC compliance
- **[Open Source Strategy](open_source.md)** - Transparency principles, secrets externalization

---

## Phase 2+ Implementation Vision

**⚠️ Note:** The sections below represent the **original July 2024 vision** for Phase 2+ features. Technology stack has evolved significantly. See [canister-architecture-diagram.md](./canister-architecture-diagram.md) for current architecture.

## Technology Stack (Current Direction)

### **Frontend**
- **Svelte**: Primary frontend framework for most components
- **React**: Newsletter Admin only (legacy/existing codebase)
- **builder.io**: Status uncertain for Svelte integration
- **PWA**: Progressive Web App for offline capabilities

### **Backend - Internet Computer (IC)**
- **Motoko**: Core business logic, data storage, blockchain operations
- **Rust**: Payment bridge canister (Stripe webhook verification)
- **Canister Storage**: Secure, encrypted data storage
- **Chain-key Security**: Cross-chain operations with Ethereum/Polygon

### **Blockchain Integration**
- **Polygon**: NFT storage, DID registry, smart contracts
- **Ethereum**: Fallback for Polygon operations
- **IPFS/IPLD**: Decentralized content storage
- **Lighthouse**: Long-term IPFS storage
- **Bridge Canister**: Polygon event synchronization for wallet cache

### **External Services**
- **KYC Providers**: Jumio, Onfido, Coinbase
- **LinkedIn OAuth**: Social verification
- **Gravatar**: Optional identity linking, profile images, and KYC-light verification
- **Jitsi Meet**: Video conferencing
- **Modal**: NFT image generation

## Security Considerations

### **Data Protection**
- **Selective Encryption**: High-risk data encrypted in IC storage
- **Externalized Secrets**: All secrets in environment variables
- **GDPR Compliance**: Right to be Forgotten via key rotation
- **KYC Compliance**: Secure regulatory access mechanisms

### **Authentication & Authorization**
- **Internet Identity**: Primary authentication for mainstream users
- **WalletConnect**: Authentication for crypto-natives
- **Multi-level Identity**: Pseudonymous DID and Gravatar options
- **Gravatar Integration**: Optional profile image linking, identity revelation, and KYC-light verification
- **Access Control**: Bundle-specific permissions and Dunbar's number scaling

### **External Integrations**
- **Webhook Security**: Signature verification for all provider webhooks
- **OAuth Security**: Secure LinkedIn OAuth integration
- **API Security**: Secure external API integrations
- **Error Handling**: Secure error handling without information leakage

## Performance Requirements

### **Response Times**
- **Web Interface**: <2 seconds for page load
- **KYC Processing**: <10 minutes for provider verification
- **Inter-canister Calls**: <2 seconds for cross-canister communication
- **Blockchain Operations**: <5 seconds for NFT/DID operations
- **Wallet Cache Queries**: <1 second for user holdings and sponsorship data

### **Scalability**
- **User Capacity**: Support for 2.9M NFT holders
- **Concurrent Users**: 10,000+ concurrent community users
- **Storage Efficiency**: Optimized IC storage usage
- **Memory Management**: Efficient canister memory usage
- **Wallet Cache**: Efficient sparse representation for large NFT trees

## Compliance & Regulatory

### **KYC/AML Compliance**
- **Risk-based Approach**: Multi-tier verification based on donation amount
- **Jurisdiction Compliance**: Location-specific verification requirements
- **Regulatory Reporting**: Automated compliance reporting
- **Audit Trail**: Complete audit trail for regulatory access

### **GDPR Compliance**
- **Data Minimization**: Collect only necessary data
- **Right to be Forgotten**: Encryption key rotation for data deletion
- **Data Portability**: Export user data on request
- **Consent Management**: Clear consent collection and management

## Community & Open Source

### **Open Source Strategy**
- **Transparency = Trust**: Comprehensive open source approach
- **Security Through Transparency**: Community security review
- **Secrets Externalization**: All secrets externalized to environment variables
- **Community Contribution**: Guidelines for community contributions

### **Documentation Standards**
- **Comprehensive API Documentation**: Complete API reference
- **Architecture Documentation**: Detailed system architecture
- **Security Documentation**: Security policies and procedures
- **User Guides**: Complete user documentation

---

## Getting Started

### **Phase 1: Infrastructure Team (START HERE)**

**Focus:** Deploy control plane, governance, and production migration

1. **[canister-architecture-diagram.md](./canister-architecture-diagram.md)** - Complete system architecture overview
2. **[architectural_decisions.md](./architectural_decisions.md)** - Understand key decisions and rationale
3. **[ens-dns-setup.md](./ens-dns-setup.md)** - Bootstrap sequence for ENS/DNS
4. **[governance-policy.md](./governance-policy.md)** - Board governance and voting structure
5. **[admin-architecture.md](./admin-architecture.md)** - Control plane security patterns
6. **[user-journey-funnel.md](./user-journey-funnel.md)** - Understand data plane user journey

### **Phase 2+: Feature Development Team**

**Focus:** Build user-facing features after infrastructure is operational

**Current Tech Stack:** Svelte (primary), React (newsletter Admin only), Motoko (backend), Rust (payment bridge)

1. Review **[user-journey-funnel.md](./user-journey-funnel.md)** for user engagement progression and gates
2. Review **[canister-architecture-diagram.md](./canister-architecture-diagram.md)** for current system architecture
3. Review [KYC Integration Architecture](kyc.md) for donations and compliance features
4. Review [Portfolio Dashboard](portfolio-dashboard.md) for NFT portfolio and sponsorship features
5. Review [Community Strategy](community-strategy.md) for discussion forums and social features
6. Review [Security Architecture](security.md) for security implementation
7. **Note:** [frontend-technology-evaluation.md](./frontend-technology-evaluation.md) describes React + V0.dev approach (outdated)

### **For System Administrators**

**Phase 1 (Immediate):**
1. **[canister-architecture-diagram.md § Bootstrap Sequence](./canister-architecture-diagram.md)** - Deployment steps
2. **[canister-architecture-diagram.md § Production Migration](./canister-architecture-diagram.md)** - Wix → ICP migration
3. **[ens-dns-setup.md](./ens-dns-setup.md)** - ENS/DNS configuration

**Phase 2+ (Later):**
1. Review [Security Architecture](security.md) for ongoing security
2. Review [Open Source Strategy](open_source.md) for open source considerations
3. Review [Jitsi Integration Architecture](jitsi-integration.md) for video conferencing setup

### **For Board Members & Governance**

1. **[governance-policy.md](./governance-policy.md)** - Board voting structure and approval tiers
2. **[architectural_decisions.md](./architectural_decisions.md)** - Key decisions requiring board input
3. **[crypto_funding.md](./crypto_funding.md)** - Crypto treasury management strategy
4. **[canister-architecture-diagram.md § Production Migration](./canister-architecture-diagram.md)** - Production cutover decision points

## Contributing

See [Open Source Strategy](open_source.md) for detailed contribution guidelines and security requirements.

## Support

For technical questions about the architecture:
- **Documentation Issues**: Create issues in the repository
- **Security Concerns**: Contact security@cpp.community
- **Architecture Questions**: Contact architecture@cpp.community

---

## Document Change Log

**Version 2.0.0 (2025-11-14):**
- Reorganized to distinguish Phase 1 (infrastructure) vs Phase 2+ (social features)
- Added Quick Start section pointing to foundational infrastructure docs
- Added control plane vs data plane architecture context
- Cross-referenced new November 2024 architecture documents
- Updated Getting Started sections for different audiences
- Clarified implementation priority (infrastructure first, features later)
- **Updated technology stack:** Svelte (primary), React (newsletter Admin only)
- Marked frontend-technology-evaluation.md as outdated (React + V0.dev approach)
- Removed obsolete TypeScript/Azle references
- Updated to reflect Rust payment bridge canister

**Version 1.0.0 (2025-07-26):**
- Initial comprehensive architecture documentation
- Focus on community features, KYC, NFT portfolio, social features
- V0.dev workflow, wallet cache, sponsorship ledger

---

**Last Updated:** 2025-11-14
**Version:** 2.0.0 (Infrastructure-first reorganization)
**Status:** Active Development - Phase 1 (Infrastructure) in progress 