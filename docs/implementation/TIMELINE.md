# Project Timeline

Below is a diagram of Project milestones (called commits in the source code).  These are in (anticipated) chronological order and organised into lanes for the different FTI entities and projects.  Note that none of the streams are expected to end, but the depiction only extends to the last milestone we define in the graph.

## GitGraph Convention
Our project timeline uses the following conventions in the Mermaid GitGraph:
- Regular commits represent completed work items
- Commits marked with `type: HIGHLIGHT` represent planned future work
- Tags (e.g., "2024") represent significant milestone dates

```mermaid
gitGraph TB:
    commit id: "Completed milestone"
    commit id: "Future milestone" type: HIGHLIGHT
```

The timeline below shows both historical achievements and planned future work, with highlighted items indicating our upcoming focus areas.

TODO: Rename main branch to FTI (current graph limitation)

```mermaid
gitGraph TB:
    commit id: "Develop nGeni Technology class"
    commit id: "FTI formation"
    branch "4T"
    commit id: "4T Companyformation" tag: "2017"
    commit id: "Existing fourthtransition.com Webstie"
 
    checkout main
    commit id: "Form Cool Planet Foundation"
    branch "CPF"
    commit id: "Stichting formation" tag: "2018"
    commit id: "Existing coolplanet-foundation.org website"
 
    checkout main
    commit id: "Form SkyBank"
    branch "SkyBank"
    commit id: "Initial SkyBank formation" tag: "2021"
 
    checkout 4T
    commit id: "US Patent" tag: "2022"
    commit id: "EU Patent" tag: "2023"

    checkout main
    commit id: "Form Lichen Engineering"
    branch "LEL"
    commit id: "Initial LEL formation"
    commit id: "Cork Seminar" tag: "Feb 2024"
 
    checkout main
    commit id: "Conduct VC outreach"
    commit id: "Kickoff Lichen NFT Crowdfunder"
    branch "NFT"
    commit id: "Draft Lichen NFT Whitepaper"
    commit id: "Initial Digital Assets python code"
    commit id: "Complete Art basis"
    commit id: "Complete Einstein-Tile basis"
    commit id: "Acquire cpf.nft domain"
    commit id: "Initial Smart Contract design"
 
    checkout SkyBank
    commit id: "Publish skybank.international"
    checkout LEL
    commit id: "Complete LEL formation" tag: "Jan 2025"
 
    checkout CPF
    commit id: "Launch Cool Planet People"
    branch "CPP"
    commit id: "MightyNetworks subscription" tag: "Jan 2025"
    commit id: "Add core team"
 
    checkout CPF
    commit id: "Investigate ANBI status"
    commit id: "Publish new website"
    commit id: "SEO new CPF website"

    checkout CPP
    commit id: "Add Michael"
    commit id: "MN CPP branding" type: HIGHLIGHT
    commit id: "Camino Journey" type: HIGHLIGHT
 
    checkout NFT
    commit id: "Scalability of Digital Assets generation" type: HIGHLIGHT
    commit id: "Scalability of Smart Contract" type: HIGHLIGHT
    commit id: "Finalize Digital Assets artwork" type: HIGHLIGHT
    commit id: "Small scale test" type: HIGHLIGHT

    checkout 4T
    commit id: "Launch new 4T website"
    commit id: "SEO 4T content" type: HIGHLIGHT
    commit id: "Library Content" type: HIGHLIGHT

    checkout CPP
    merge 4T
    commit id: "4 Workshops Course" type: HIGHLIGHT

    checkout LEL
    commit id:"Launch lichen-engineering.com" type: HIGHLIGHT
    commit id: "SEO LEL content" type: HIGHLIGHT
 
    checkout CPF
    commit id: "Basic donation capability" type: HIGHLIGHT
    checkout CPP
    merge CPF
    commit id: "Manually add donors" type: HIGHLIGHT
    checkout CPF
    commit id: "Add CPP and Lichen symbolism page" type: HIGHLIGHT
    merge CPP
    commit id: "CPP on cpp subdomain" type: HIGHLIGHT

    checkout NFT
    commit id: "Navigate/award NFT/Bundles SDK" type: HIGHLIGHT
    commit id: "Create cpf.nft site" type: HIGHLIGHT
    commit id: "Full deployment 2.9m NFTS" type: HIGHLIGHT
    checkout CPF
    merge NFT
    commit id: "New Lichen Mural page" type: HIGHLIGHT

    checkout NFT
    commit id: "Deploy SSO provider (Okta)" type: HIGHLIGHT
    commit id: "Deploy Onboarding API to Cloudflare" type: HIGHLIGHT
    checkout CPP
    merge NFT
    commit id: "Upgrade to Business Plan" type: HIGHLIGHT
    commit id: "Enable SSO" type: HIGHLIGHT

    checkout NFT
    commit id: "NFT claim workflow with Venly wallet" type: HIGHLIGHT
    commit id: "Deploy Venly webhook on Cloudflare to Auto-kickoff Onboarding" type: HIGHLIGHT
    commit id: "Update cpf.nft website to allow donation workflow" type: HIGHLIGHT
    checkout CPF
    commit id: "Remove simple donation form" type: HIGHLIGHT

    checkout CPF
    merge NFT
    commit id: "Email/message unthanked donors" type: HIGHLIGHT

    checkout NFT
    commit id: "Automated NFT award on donation via Venly" type: HIGHLIGHT
    commit id: "Update cpf.nft website" type: HIGHLIGHT
    checkout CPP
    merge NFT
    commit id: "Automated CPP onboarding for donors" type: HIGHLIGHT
    checkout CPF
    merge NFT
    commit id: "Marketing campaign for discontented" type: HIGHLIGHT

    checkout SkyBank
    commit id: "Publish WP1" type: HIGHLIGHT
    commit id: "Marketing campaign for builders" type: HIGHLIGHT

    checkout CPP
    commit id: "Upgrade to Path to Pro Plan" type: HIGHLIGHT
    commit id: "Engage MN on Branded CPP App" type: HIGHLIGHT

    checkout CPF
    commit id: "Reach funding target" type: HIGHLIGHT
    commit id: "Fund demonstration program" type: HIGHLIGHT
    commit id: "Revisit ANBI status" type: HIGHLIGHT

    checkout LEL
    merge CPF
    commit id: "Commence Phase 1" type: HIGHLIGHT
    commit id: "Phase 1 complete" type: HIGHLIGHT
    commit id: "Commence Phase 2" type: HIGHLIGHT
    commit id: "Phase 2 complete" type: HIGHLIGHT
    commit id: "Cork Event" type: HIGHLIGHT
    commit id: "Events in other locations" type: HIGHLIGHT
    merge SkyBank
    commit id: "Complete demonstration program" type: HIGHLIGHT
```

## Current Focus Areas
We are currently focused on two major phases:

### NFT Implementation
Highlighted items in the NFT branch represent our immediate technical priorities:
- Scaling our Digital Assets generation system
- Smart Contract scalability improvements
- Digital Assets artwork finalization
- Testing deployment at small scale
- Development of navigation/award SDK
- Full-scale NFT deployment (2.9M NFTs)
- Integration with Venly wallet system
- Automated onboarding workflows

### Platform Integration
Highlighted items across CPF, CPP, and NFT branches show our integration priorities:
- SSO implementation with Okta
- Automated donor onboarding
- MightyNetworks platform integration
- Marketing campaign deployment

See [TESTING.md](./TESTING.md) for detailed testing requirements and implementation plans for these focus areas.

