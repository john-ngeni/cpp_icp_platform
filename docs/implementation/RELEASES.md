## Release Phases

Progress indicators:
- (C) Concept defined
- (D) Design in progress
- (I) Implementation in progress
- (T) Testing in progress
- (✓) Complete

## NFT/DIDs

| Component                    | Alpha                             | Beta                                            | MVP                                   | MVP++                                       |
| ---------------------------- | --------------------------------- | ----------------------------------------------- | ------------------------------------- | ------------------------------------------- |
| **Stakeholders**             | Authors                           | + Regulators + Potential Backers                | + Discontented                        | + Partner Organizations                     |
| **Legals**                   | cpf.nft CPP facing Ts&Cs (I)      | Internal Compliance doc (D)                     | Doing KYC, complying with audits (D)  | + PALS                                      |
| **Narrative**                | Internal perspective (C)          | + Draft Onboarding (C)                          | Branding & SEO                        | + Onboarding analytics (C)                  |
| **NFT Scale**                | 169 NFTs (T)                      | 54,289 NFTs (T)                                 | 2.9M (I) NFTs                         | Multiple models                             |
| **Image Generation**         | Local single-process (I)          | Modal functions - free tier (D)                 | Modeal functions - paid tier (D)      | Ditto                                       |
| **NFT Image**                | Original with serialcode only (I) | Zoomed Einstein and derived background          | Final Design, Chunk deduplication     | Ditto                                       |
| **Other Images**             |                                   | Bundle and DeepZoom                             | Ditto                                 | Ditto                                       |
| **IPLD usage**               | Single CAR upload (I)             | Distributed CAR upload with checkpoints (C)     | Ditto                                 | Ditto                                       |
| **Smart Contract**           | Testnet (C)                       | Polygon (C)                                     | Polygon (C)                           | Multi-chain                                 |
| **IPFS Visualizer**          | Basic "odometer" selector (I)     | + DeepZoom tree navigation(D)                   | Holdings -> DeepZoom (C)              | Interactive exploration (D)                 |
| **Cryptographic guarantees** |                                   | All NFTs derive from all other NFTs (D)         | Relate to original SVG dag (C)        |                                             |
| **Cool Planet ID**           | DID creation (I)                  |                                                 | Roles e.g. Author/Member/...          | CPF issued VCs (C)                          |
| **Cool Planet Org ID**       | DID creation (I)                  | ENS subdomain creation (I)                      | Multisig and bad actor revocation (C) | Cool Planet B-Corp (C)                      |
| **Outstanding Issues**       |                                   | Single item groups, non-contiguous ordering (D) |                                       |                                             |
| **Donation handling**        | Simulated (C)                     | Simulated Stripe on IC (C)                      | Fiat + Bitcoin via IC (C)             | + ETH + Tax receipts (with ANBI status) (D) |
| **Network effects**          |                                   |                                                 |                                       | Gitcoin for matching funds (C)              |
| **Tracker**                  | Basic event logging (T)           | Bundle ownership (C)                            | Full sponsor map (C)                  | Cross-model tracking (C)                    |
| **Test Focus**               | Award process mechanics (T)       | Scale testing & security testing (I)            | Security audits & stress testing (C)  | Cross-platform & multi-model validation (D) |

## CPP App

| Component               | Alpha                                     | Beta                                     | MVP                                      | MVP++                                    |
| ----------------------- | ----------------------------------------- | ---------------------------------------- | ---------------------------------------- | ---------------------------------------- |
| **PWA**                 | Canister served,  Basic branding (I)      | + II signup/signin (I)                   | + Cross-platform consistency (D)         | + Offline support + WalletConnect Signin |
| **Donation capability** | Stripe sandbox (I)                        | Stripe Tax Status configured             | CPF Bank Account, Radar rules configured | Paypal, Crypto donations                 |
| **NFT Awards**          | Direct testnet call (C)                   | + II managed Ethereum address (D)        | + Stripe->Polygon, ckBTC->Polygon (C)    | + ETH->Polygon                           |
| **KYC/KYB**             | Juristiction, large donation handling (T) | KYB in Donation workflow (T)             | + Medium Risk KYC (D)                    | + High risk KYC                          |
| **Audit**               |                                           | Encryption of KYC data, audit trails (C) | + Auditor interface (I)                  |                                          |
| **Accounting**          | Admin interface, Total Donations Received |
| **Lichen NFT**          |                                           | View NFT metadata (C)                    | + Deepzoom view at Login (C)             | + Transfer to another Wallet             |
| **Manage NFT Bundle**   |                                           | View Bundle (C)                          | Sponsor (sub)bundle / NFT (D)            |                                          |
| **Manage Org**          |                                           |                                          |                                          | Create ENS domain (with external wallet) |
| **CPP Content**         | Basic branding (C)                        | Welcome + Camino Orientation (C)         | + Camino Module 1+ (I)                   | + additional Camino modules              |
| **Messaging**           |                                           |                                          | Public (C)                               | + Private (C)                            |
| **Group Chat**          |                                           |                                          | Bundle groups (C)                        | + Topic Forums (C)                       |
| **Events**              |                                           |                                          | Basic Calendar (C)                       | + Jitsu Web Conference (C)               |
| **Merch**               |                                           |                                          |                                          | Partial supertile printables (C)         |

### Alpha Release Focus
- Small-scale test with 169 Einstein tree NFTs
- Simplified NFT image generation (single-thread)
- Testnet smart contract deployment
- Basic visualizer with odometer-style serial code selection
- Manual NFT claiming process
- Core event logging
- Manual onboarding process

### Beta Release Focus
- **IC Stack Introduction**: Deploy core Motoko canisters for KYC, wallet cache, and user management
- **V0.dev + React + Vite Workflow**: Implement BDD-first development with AI-powered component generation
- **Cursor/Claude Integration**: Establish AI-assisted development workflow for rapid prototyping
- **Internet Identity Integration**: User authentication and wallet association with II
- **Wallet Cache System**: IC-native tracking of NFT holdings and sponsorship capacity
- **ENS Integration**: Bundle holder subdomain creation and content publishing standards
- **Security Architecture**: GDPR compliance, encryption, and audit trail implementation