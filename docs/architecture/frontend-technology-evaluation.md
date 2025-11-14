# Frontend Technology Evaluation for CPP Platform

---

## ⚠️ OUTDATED DOCUMENT WARNING

**Date of Obsolescence:** 2025-11-14

**This document describes a React + V0.dev approach that is NO LONGER the current direction.**

**Current Technology Direction (as of Nov 2024):**
- **Svelte**: Primary frontend framework for most components
- **React**: Newsletter Admin only (legacy/existing codebase)
- **builder.io**: Status uncertain for Svelte integration
- **Motoko**: Backend canisters
- **Rust**: Payment bridge canister

**Why Keep This Document?**
- Historical context for technology decision evolution
- Understanding of IC deployment patterns (still relevant)
- Asset canister concepts (still applicable to Svelte)

**For Current Architecture:** See [canister-architecture-diagram.md](./canister-architecture-diagram.md)

---

## Executive Summary (HISTORICAL - React + V0.dev Approach)

This document evaluates frontend technology options for the Cool Planet Platform (CPP) Progressive Web App (PWA), incorporating critical lessons learned from SSL certificate investigations and Internet Computer (IC) deployment best practices.

**Key Finding (OUTDATED)**: Based on SSL certificate investigation results, **React with IC Asset Canister** is the recommended approach for optimal SSL certificate support and deployment simplicity.

**Domain**: This PWA will be served on the main `coolplanet-foundation.org` domain, not a subdomain.

## What is Vite?

**Vite** is a modern build tool and development server that provides:
- **Fast Development**: Hot module replacement (HMR) for instant updates
- **Optimized Builds**: Production builds with code splitting and tree shaking
- **Framework Agnostic**: Works with React, Vue, Svelte, Vanilla JS, and more
- **PWA Support**: Excellent plugins for Progressive Web App features
- **TypeScript Support**: Native TypeScript support without configuration
- **Static Export**: Perfect for IC asset canister deployment

**Vite is included in all options** because it's the recommended build tool for modern frontend development on IC, regardless of the chosen framework.

## Framework Choice: Static Site PWA with BDD AI Workflow

The framework choice is about **which static site generator works best with BDD AI workflow on IC**:

### **Next.js**
```
Frontend: Next.js → Static Export → Asset Canister
Backend: Separate Motoko canister for core business logic
External Integration: Separate Azle canister for provider APIs and webhooks
```

**Advantages:**
- ✅ **Superior Static Export**: Next.js excels at static generation
- ✅ **Built-in Optimizations**: Image, font, performance optimizations
- ✅ **SEO**: Built-in SEO optimizations
- ✅ **Claude Code Integration**: Excellent AI-assisted development support
- ✅ **Performance**: Excellent performance with static generation
- ✅ **PWA Support**: Excellent PWA capabilities

**Disadvantages:**
- ❌ **Complexity**: More complex build and deployment process
- ❌ **Bundle Size**: Larger than React + Vite
- ❌ **IC Community**: Less established patterns for IC integration
- ❌ **Learning Curve**: Requires understanding Next.js conventions
- ❌ **V0.dev Integration**: Limited BDD AI workflow support

### **React + Vite (RECOMMENDED)**
```
Frontend: React + Vite → Asset Canister
Backend: Separate Motoko canister for core business logic
External Integration: Separate Azle canister for provider APIs and webhooks
```

**Advantages:**
- ✅ **Proven Pattern**: Asset Canister + Motoko Backend (DSCVR pattern)
- ✅ **IC Community Dominance**: Most IC projects use React + Vite
- ✅ **Performance**: Excellent performance with static serving
- ✅ **Smaller Bundle**: No Next.js framework overhead
- ✅ **Faster Development**: Vite's instant hot reload
- ✅ **PWA Support**: Excellent PWA capabilities with Vite
- ✅ **BDD AI Workflow**: Excellent support for V0.dev + Claude integration

**Disadvantages:**
- ❌ **Static Content**: Limited server-side rendering (client-side routing)
- ❌ **API Complexity**: Separate backend canisters for different functionality
- ❌ **SEO**: Limited SEO support compared to server-side rendering
- ❌ **Real-time Updates**: Manual implementation required for live updates

### **Why React + Vite is Recommended**

**React + Vite is recommended** because it provides the best combination of IC compatibility and BDD AI workflow support. This approach provides:

1. **Proven Architecture**: Asset Canister + Motoko Backend pattern used by successful IC apps (DSCVR)
2. **IC Community Alignment**: React + Vite is the dominant IC pattern
3. **Performance**: Excellent performance with static serving
4. **V0.dev Integration**: BDD-first development with AI-powered component generation
5. **Simpler Development**: Faster iteration with Vite's hot reload
6. **Smaller Bundle**: No Next.js framework overhead

### **V0.dev Integration Advantage**

**V0.dev provides significant advantages for React + Vite development:**

#### **BDD-First Development Workflow**
- **Direct BDD Understanding**: V0.dev understands BDD scenarios directly
- **Claude Integration**: AI generates React components from BDD scenarios
- **Production-Ready Code**: Generates complete, styled React components
- **Code Generation**: Automatic React component creation with TypeScript

#### **AI-Powered Development Benefits**
- **Superior BDD Integration**: Direct mapping from BDD scenarios to components
- **Natural Language Input**: Generate components from plain English descriptions
- **Built-in Best Practices**: Automatic accessibility, performance, and styling
- **Component Relationships**: Understands how components work together

#### **IC Integration Excellence**
- **React + Vite Compatibility**: V0.dev generates pure React components
- **Asset Canister Deployment**: Perfect for IC asset canister deployment
- **Community Support**: Most IC projects use React + Vite
- **Proven Patterns**: Well-established IC integration patterns

### **Alternative: Figma + Next.js Workflow**

**Note**: Figma could have been used with Next.js for visual design, providing superior design tools and industry-standard design system capabilities. However, **V0.dev is preferred for this project due to BDD compatibility**:

#### **Why V0.dev Over Figma for This Project**
- **BDD Integration**: V0.dev supports direct BDD scenario understanding, while Figma requires manual design-to-code translation
- **Claude Integration**: AI can generate React components directly from BDD scenarios, enabling BDD-first development
- **AI-Powered Development**: V0.dev provides superior BDD scenario understanding, while Figma requires design expertise
- **Code Generation**: Automatic React component creation vs manual code generation from Figma designs
- **Workflow Alignment**: V0.dev aligns with BDD-first development workflow, while Figma follows traditional design-first approach

### **Framework Selection Criteria**

Given we must use React + Vite for BDD AI workflow, the framework choice is driven by:

#### **Development Workflow**
- **Claude Code Integration**: How well the framework works with AI-assisted development
- **Hot Reload**: Development speed and iteration cycles
- **TypeScript Support**: Type safety and developer experience
- **Debugging**: Tooling and debugging capabilities

#### **PWA Support**
- **Service Worker Integration**: Offline functionality and caching
- **App Manifest**: Installation and native app-like experience
- **Performance**: Bundle size and runtime performance
- **Mobile Experience**: Responsive design and touch interactions

#### **IC Integration Requirements**
- **Static Export**: Must work with IC asset canister deployment
- **Client-Side Routing**: Handle routing without server-side rendering
- **State Management**: Complex state for wallet, KYC, and portfolio data
- **Real-Time Updates**: Handle live updates from IC canisters

**Note**: The canister architecture is defined in [frontend-architecture.md](./frontend-architecture.md) and includes:
- **Frontend Asset Canister** (`"type": "assets"`): Serves the PWA (SSL certificate support)
- **Core User Management Canister** (`"type": "motoko"`): Handles KYC, wallet cache, risk assessment, audit trail, ZK proof generation, provider APIs, webhooks, and notifications



### **Next.js Static Workflow**

#### **Architecture**
```json
// dfx.json
{
  "canisters": {
    "frontend": {
      "type": "assets",
      "source": ["out/"]
    },
    "core-user-management": {
      "type": "motoko",
      "main": "src/core_user_management/main.mo"
    }
  }
}
```

#### **Next.js Configuration**
```javascript
// next.config.js
/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'export',
  trailingSlash: true,
  images: {
    unoptimized: true
  },
  basePath: '',
  assetPrefix: './'
}

module.exports = nextConfig
```

#### **Advantages**
- ✅ **Superior Static Export**: Excellent static generation capabilities
- ✅ **Built-in Optimizations**: Automatic code splitting, image optimization, font optimization
- ✅ **SEO**: Built-in SEO optimizations
- ✅ **Claude Code Integration**: Excellent AI-assisted development support
- ✅ **Performance**: Excellent performance with static generation
- ✅ **PWA Support**: Excellent PWA capabilities
- ✅ **TypeScript**: Excellent TypeScript support
- ✅ **React Ecosystem**: Full React ecosystem and community

#### **Disadvantages**
- ❌ **Larger Bundle**: Next.js framework overhead
- ❌ **Complexity**: More complex than vanilla React
- ❌ **Learning Curve**: Requires understanding Next.js conventions
- ❌ **Static Export Limitations**: Some Next.js features not available in static export
- ❌ **IC Community**: Less established patterns for IC integration

#### **SSL Certificate Support**
```bash
# Automatic SSL certificate provisioning
dist/
├── .well-known/
│   └── ic-domains              # coolplanet-foundation.org
├── index.html
├── assets/
│   ├── index-abc123.js
│   └── index-def456.css
└── manifest.json
```

### **Option 2: Vanilla JavaScript + Vite**

#### **Architecture**
```json
// dfx.json
{
  "canisters": {
    "frontend": {
      "type": "assets",
      "source": ["dist/"]
    },
    "core-user-management": {
      "type": "motoko",
      "main": "src/core_user_management/main.mo"
    }
  }
}
```

#### **Vite Configuration**
```javascript
// vite.config.js
import { defineConfig } from 'vite'
import { VitePWA } from 'vite-plugin-pwa'

export default defineConfig({
  plugins: [
    VitePWA({
      registerType: 'autoUpdate',
      workbox: {
        globPatterns: ['**/*.{js,css,html,ico,png,svg}']
      },
      manifest: {
        name: 'Cool Planet Platform',
        short_name: 'CPP',
        theme_color: '#4ecdc4',
        background_color: '#ffffff',
        display: 'standalone'
      }
    })
  ],
  build: {
    outDir: 'dist',
    emptyOutDir: true
  },
  base: './'
})
```

#### **Advantages**
- ✅ **SSL Certificate Support**: Automatic via IC asset canister
- ✅ **PWA Support**: Excellent Vite PWA plugin
- ✅ **Performance**: Minimal bundle size
- ✅ **Simplicity**: No framework overhead
- ✅ **Learning Curve**: Familiar to most developers
- ✅ **IC Integration**: Direct IC agent usage

#### **Disadvantages**
- ❌ **Development Speed**: Manual DOM manipulation
- ❌ **State Management**: Complex for large applications
- ❌ **Component Reusability**: Limited compared to React
- ❌ **Ecosystem**: Fewer UI libraries and tools

### **Option 3: Svelte + Vite**

#### **Architecture**
```json
// dfx.json
{
  "canisters": {
    "frontend": {
      "type": "assets",
      "source": ["dist/"]
    },
    "core-user-management": {
      "type": "motoko",
      "main": "src/core_user_management/main.mo"
    }
  }
}
```

#### **Advantages**
- ✅ **SSL Certificate Support**: Automatic via IC asset canister
- ✅ **PWA Support**: Excellent Vite PWA plugin
- ✅ **Performance**: Very small bundle size
- ✅ **Reactivity**: Built-in reactive state management
- ✅ **Learning Curve**: Simpler than React
- ✅ **TypeScript**: Full TypeScript support

#### **Disadvantages**
- ⚠️ **Ecosystem**: Smaller than React ecosystem
- ⚠️ **Team Experience**: May need Svelte training
- ⚠️ **IC Integration**: Less proven IC agent integration

### **Option 4: Vue.js**

#### **Advantages**
- ✅ **Learning Curve**: Gentle learning curve, easy to pick up
- ✅ **Ecosystem**: Rich ecosystem for UI components and tools
- ✅ **Performance**: Good performance characteristics
- ✅ **TypeScript**: Full TypeScript support
- ✅ **Documentation**: Excellent documentation and guides
- ✅ **Progressive**: Can be used incrementally

#### **Disadvantages**
- ⚠️ **Bundle Size**: Larger than Svelte, smaller than React
- ✅ **Team Experience**: May need Vue training for team
- ✅ **IC Integration**: Less proven IC agent integration
- ✅ **Community**: Smaller than React community



## Detailed Comparison Matrix

## IC Community Adoption Analysis

### **Current IC Ecosystem Trends**

Based on analysis of IC projects and community patterns:

#### **React + Vite Dominance**
- **Most IC Projects**: Use React + Vite for frontend
- **Examples**: DSCVR, many DeFi projects
- **Community Support**: Extensive examples, tutorials, and patterns
- **Azle Integration**: Well-established patterns for React + Azle integration

#### **Next.js Limited Adoption**
- **Few IC Projects**: Limited examples of Next.js on IC
- **Static Export**: Most IC projects prefer simpler static builds
- **Community Knowledge**: Less established patterns and examples
- **Complexity**: IC developers prefer simpler, more direct approaches

### **Azle Limitations Impact**

From [REACT_VITE_IC_RECOMMENDATIONS.md](../fti_newsletter_archive/docs/REACT_VITE_IC_RECOMMENDATIONS.md):

#### **Azle Backend Limitations**
- ❌ **File Processing**: Limited document/image processing capabilities
- ❌ **Node.js Libraries**: Many popular npm packages don't work
- ❌ **Memory Constraints**: Large file processing issues
- ❌ **WebAssembly Compatibility**: Some libraries fail in Azle environment

#### **Impact on Frontend Choice**
- **React + Vite**: Better compatibility with Azle limitations
- **Next.js**: May have more complex integration issues with Azle
- **Recommendation**: Simpler frontend architecture works better with Azle constraints

## Framework Comparison: All Options

| Feature                   | Next.js                       | React + Vite                  | Vanilla JS + Vite             | Svelte + Vite                    | Vue.js + Vite                 |
| ------------------------- | ----------------------------- | ----------------------------- | ----------------------------- | -------------------------------- | ----------------------------- |
| **Bundle Size**           | ❌ Larger (framework overhead) | ✅ Smaller (React only)        | ✅ Smallest (no framework)     | ✅ Very small (Svelte)            | ⚠️ Medium (Vue overhead)       |
| **IC Community Adoption** | ❌ Limited examples            | ✅ Dominant pattern (DSCVR)    | ⚠️ Some examples               | ⚠️ Limited (OpenChat uses Svelte) | ❌ Very limited                |
| **Build Complexity**      | ❌ Complex (Next.js)           | ✅ Simple (Vite)               | ✅ Simple (Vite)               | ✅ Simple (Vite)                  | ✅ Simple (Vite)               |
| **Maintenance**           | ❌ Complex                     | ✅ Standard (React + Vite)     | ⚠️ Manual (no framework)       | ⚠️ Svelte-specific                | ⚠️ Vue-specific                |
| **SEO Support**           | ✅ Excellent                   | ✅ Excellent (static sites)    | ✅ Excellent (static sites)    | ✅ Excellent (static sites)       | ✅ Excellent (static sites)    |
| **PWA Support**           | ✅ Excellent                   | ✅ Excellent (Vite PWA plugin) | ✅ Excellent (Vite PWA plugin) | ✅ Excellent (Vite PWA plugin)    | ✅ Excellent (Vite PWA plugin) |
| **Learning Curve**        | ❌ Steep (Next.js)             | ✅ Gentle (React)              | ✅ Familiar (vanilla JS)       | ✅ Gentle (Svelte)                | ✅ Gentle (Vue)                |
| **BDD AI Workflow**       | ❌ Limited (V0.dev)            | ✅ Excellent (V0.dev)          | ❌ No V0.dev support           | ❌ No V0.dev support              | ❌ No V0.dev support           |
| **Wix Conversion**        | ✅ Builder.io support          | ✅ Builder.io support          | ❌ No Builder.io support       | ❌ No Builder.io support          | ❌ No Builder.io support       |

## Implementation Recommendations

### **Phase 1: Technology Selection (Week 1)**

**Recommended Choice: React + Vite + V0.dev**

**Rationale:**
1. **V0.dev Integration**: Excellent BDD AI workflow support
2. **IC Community Dominance**: Most IC projects use React + Vite, extensive community support
3. **Proven Pattern**: Asset Canister + Motoko Backend (DSCVR pattern)
4. **Smaller Bundle**: No Next.js framework overhead
5. **Faster Development**: Vite's instant hot reload
6. **Team Experience**: Most developers familiar with React
7. **Static Generation**: V0.dev generates React components that build to static content

**Alternative: Next.js**
- **Use if**: Team values SEO and built-in optimizations over V0.dev integration
- **Trade-offs**: Limited V0.dev BDD AI workflow support, larger bundle, more complex build
- **Risk**: Less established IC integration patterns



### **Phase 2: Architecture Setup (Weeks 2-3)**

```bash
# Project structure (React + Vite)
cpp-frontend/
├── src/                     # Source code
│   ├── main.tsx             # Entry point
│   ├── App.tsx              # Root component
│   ├── pages/               # Page components
│   │   ├── Home.tsx         # Home page
│   │   ├── Donate.tsx       # Donation flow
│   │   ├── Portfolio.tsx    # Portfolio dashboard
│   │   └── KYC.tsx          # KYC flow
│   ├── components/          # Reusable components
│   │   ├── ui/              # UI components (from V0.dev)
│   │   ├── forms/           # Form components (from V0.dev)
│   │   └── layout/          # Layout components (from V0.dev)
│   ├── lib/                 # Utility libraries
│   │   ├── ic-client.ts     # IC integration
│   │   ├── utils.ts         # Utility functions
│   │   └── types.ts         # TypeScript types
│   ├── hooks/               # Custom React hooks
│   └── styles/              # Global styles
├── public/
│   ├── .well-known/
│   │   └── ic-domains       # coolplanet-foundation.org
│   ├── manifest.json        # PWA manifest
│   └── icons/               # PWA icons
├── dist/                    # Static build (for IC asset canister)
├── vite.config.ts           # Vite configuration
├── package.json
└── tsconfig.json
```

### **Phase 3: V0.dev + React + Vite Workflow (Weeks 4-5)**

#### **V0.dev Component Generation**
```typescript
// 1. BDD Scenario Input to V0.dev
// "As a user, I want to donate to the Cool Planet Foundation
//  so that I can support environmental initiatives"

// 2. V0.dev generates React component
// Generated component from V0.dev
export function DonationForm() {
  const [amount, setAmount] = useState('')
  const [email, setEmail] = useState('')
  
  return (
    <div className="max-w-md mx-auto p-6 bg-white rounded-lg shadow-md">
      <h2 className="text-2xl font-bold mb-4">Donate to Cool Planet</h2>
      <form className="space-y-4">
        <div>
          <label className="block text-sm font-medium mb-2">Amount</label>
          <input
            type="number"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            className="w-full px-3 py-2 border rounded-md"
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-2">Email</label>
          <input
            type="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            className="w-full px-3 py-2 border rounded-md"
          />
        </div>
        <button className="w-full bg-green-600 text-white py-2 rounded-md">
          Donate Now
        </button>
      </form>
    </div>
  )
}
```

#### **Vite Build Process**
```bash
# 1. V0.dev generates React components
# 2. Copy components to src/components/
# 3. Build static site with Vite
npm run build

# 4. Vite creates static files in dist/
dist/
├── index.html
├── assets/
│   ├── index-abc123.js
│   └── index-def456.css
└── manifest.json

# 5. Deploy to IC asset canister
dfx deploy frontend
```

### **Phase 4: IC Integration (Weeks 6-7)**

```typescript
// src/services/ic-client.ts
import { Actor, HttpAgent } from '@dfinity/agent'
import { AuthClient } from '@dfinity/auth-client'
import { CoreUserManagement } from '../../../declarations/core_user_management'

export class ICClient {
  private agent: HttpAgent
  private authClient: AuthClient
  private coreUserManagement: Actor

  async initialize() {
    this.authClient = await AuthClient.create()
    this.agent = new HttpAgent({
      host: process.env.NODE_ENV === 'production' 
        ? 'https://ic0.app' 
        : 'http://localhost:4943'
    })
    
    // Initialize core user management canister
    this.coreUserManagement = Actor.createActor(CoreUserManagement, {
      agent: this.agent,
      canisterId: process.env.CORE_USER_MANAGEMENT_CANISTER_ID
    })
  }

  async authenticate() {
    return new Promise((resolve, reject) => {
      this.authClient.login({
        identityProvider: process.env.NODE_ENV === 'production'
          ? 'https://identity.ic0.app'
          : 'http://localhost:4943?canisterId=rdmx6-jaaaa-aaaaa-aaadq-cai',
        onSuccess: () => resolve(true),
        onError: reject
      })
    })
  }
}
```

### **Phase 4: PWA Features (Weeks 6-7)**

```typescript
// src/hooks/usePWA.ts
import { useEffect, useState } from 'react'

export function usePWA() {
  const [isInstalled, setIsInstalled] = useState(false)
  const [deferredPrompt, setDeferredPrompt] = useState<any>(null)

  useEffect(() => {
    // Check if app is already installed
    if (window.matchMedia('(display-mode: standalone)').matches) {
      setIsInstalled(true)
    }

    // Listen for beforeinstallprompt event
    window.addEventListener('beforeinstallprompt', (e) => {
      e.preventDefault()
      setDeferredPrompt(e)
    })

    // Listen for app installed event
    window.addEventListener('appinstalled', () => {
      setIsInstalled(true)
      setDeferredPrompt(null)
    })
  }, [])

  const installApp = async () => {
    if (deferredPrompt) {
      deferredPrompt.prompt()
      const { outcome } = await deferredPrompt.userChoice
      if (outcome === 'accepted') {
        setDeferredPrompt(null)
      }
    }
  }

  return { isInstalled, canInstall: !!deferredPrompt, installApp }
}
```

## SSL Certificate Deployment Strategy

### **Custom Domain Setup**

```bash
# 1. Create .well-known/ic-domains file
echo "coolplanet-foundation.org" > public/.well-known/ic-domains

# 2. Build and deploy
npm run build
dfx deploy frontend

# 3. Configure DNS
# coolplanet-foundation.org TXT "icp-canister=FRONTEND_CANISTER_ID"
# _canister-id.coolplanet-foundation.org TXT "FRONTEND_CANISTER_ID"
```

### **SSL Certificate Verification**

```bash
# Test SSL certificate provisioning
curl -I "https://FRONTEND_CANISTER_ID.icp0.io/.well-known/ic-domains"
# Should return 200 OK with proper IC certification headers

# Test custom domain (after DNS propagation)
curl -I "https://coolplanet-foundation.org/.well-known/ic-domains"
# Should return 200 OK with SSL certificate
```

## Risk Mitigation

### **SSL Certificate Risks**
- **Mitigation**: Use frontend asset canister architecture
- **Fallback**: None needed - asset canister provides automatic SSL

### **Performance Risks**
- **Mitigation**: Code splitting and lazy loading
- **Monitoring**: Bundle size analysis and optimization

### **Team Experience Risks**
- **Mitigation**: React training and documentation
- **Fallback**: Vanilla JS if React adoption fails

### **IC Integration Risks**
- **Mitigation**: Use proven IC agent libraries
- **Testing**: Comprehensive IC integration testing

## Newsletter SSL Architecture Lessons Learned

### **What We Learned from the Newsletter Project**

The newsletter project encountered significant SSL certificate issues that influenced our framework recommendation:

#### **Original Approach: Monolithic Backend Canister**
- **Problem**: Attempted to serve frontend from backend canister (`"type": "azle"`)
- **SSL Issues**: Response verification errors, complex HTTP serving requirements
- **Custom Domain**: Could not properly serve `.well-known/ic-domains` for SSL certificates
- **Result**: SSL certificate provisioning failed, requiring complex HTTP serving workaround

#### **Key Insight: Hybrid Canister Architecture**
- **Discovery**: Only `"type": "assets"` canisters support automatic SSL certificate provisioning
- **Solution**: Separate frontend (asset canister) from backend logic (motoko/azle canisters)
- **Architecture**: Frontend asset canister for SSL + multiple backend canisters for functionality
- **Implication**: Need an assets canister always to deal with SSL and not handle in Azel hosted frameworks

## Conclusion

**Recommended Architecture: Hybrid Builder.io + V0.dev + React + Vite Approach**

### **Hybrid Technology Stack: Optimized for Content Type**

**Different tools for different content types:**
- **Dynamic Content**: V0.dev + React + Vite (member platform, real-time features)
- **Static Content**: Builder.io + React + Vite (public site, marketing pages)
- **Deployment**: Asset Canisters (for SSL certificate support)
- **Backend**: Separate Motoko/Azle canisters for business logic

### **Phase 1: Dynamic Member Platform (`members.coolplanet-foundation.org`)**
- **Technology**: React + Vite + V0.dev → Asset Canister
- **Workflow**: BDD → Claude → V0.dev → React Components → IC Deployment
- **Content**: Gated member content, real-time updates, personalized experiences
- **Functionality**: II authentication, wallet management, KYC, portfolio
- **Rationale**: BDD-first workflow with AI-powered development for complex user flows

### **Phase 2: Static Public Site (`coolplanet-foundation.org`)**
- **Technology**: React + Vite + Builder.io → Asset Canister
- **Workflow**: Builder.io Visual Editor → React + Vite Build → IC Deployment
- **Content**: Public content, SEO-optimized, legal/terms pages, marketing materials
- **Functionality**: Login/donate buttons that redirect to member platform
- **Rationale**: Visual design-first approach for static marketing and informational content

### **ENS Integration Strategy**
- **ENS Mirror**: `cpf.nft` mirrors `coolplanet-foundation.org` DNS (ICANN independence)
- **NFT Models**: `cpp1.members.coolplanet-foundation.org`, `cpp2.members.coolplanet-foundation.org`
- **Bundle Holders**: Publish to their ENS subdomains following CPP standards

### **Why This Hybrid Approach:**

1. **Optimized for Content Type**: V0.dev for dynamic BDD-driven features, Builder.io for static visual design
2. **BDD-First Development**: V0.dev excels at complex user flows and behavior specifications
3. **Visual Design Excellence**: Builder.io provides superior visual design capabilities for static content
4. **Claude Integration**: AI assistance for both component generation (V0.dev) and visual design (Builder.io)
5. **IC Community Dominance**: React + Vite is the standard IC pattern for both approaches
6. **SSL Certificate Support**: Asset canisters provide automatic SSL for all domains
7. **Cost Optimization**: Builder.io Personal ($19/month) for static content, V0.dev Pro ($20/month) for dynamic content

### **Technology Choices:**
- **Frontend Framework**: React + Vite (for both static and dynamic content)
- **Dynamic Development**: V0.dev (for member platform and real-time features)
- **Static Development**: Builder.io (for public site and marketing content)
- **Deployment**: Asset Canisters (for SSL certificate support)
- **Backend**: Motoko canisters for core business logic
- **External Integration**: Azle canisters for provider APIs and webhooks

### **Implementation Strategy:**

#### **Phase 1: Dynamic Member Platform (Weeks 1-8)**
```bash
# 1. BDD Scenario Development
# Write comprehensive BDD scenarios for all user flows

# 2. Claude + V0.dev Setup
# Set up V0.dev with Claude integration
# Generate initial components from BDD scenarios

# 3. AI-Powered Development
# Refine components in V0.dev AI editor
# Test user flows and interactions

# 4. React + Vite Implementation
# Copy React components from V0.dev
# Integrate with IC canisters
# Deploy to IC asset canister
```

#### **Phase 2: Static Public Site (Weeks 9-12)**
```bash
# 1. Builder.io Visual Development
# Create visual design in Builder.io
# Optimize for SEO and performance
# Design marketing pages and informational content

# 2. React + Vite Build
# Export React components from Builder.io
# Build static site with React + Vite
# Configure for IC asset canister deployment

# 3. IC Deployment
# Deploy to IC asset canister
# Configure custom domain and SSL
```

### **Domain Strategy:**
- **Public**: `coolplanet-foundation.org` (Builder.io + React + Vite static build)
- **Member Platform**: `members.coolplanet-foundation.org` (V0.dev + React + Vite dynamic build)
- **ENS Mirror**: `cpf.nft` mirrors `coolplanet-foundation.org` DNS (ICANN independence)
- **NFT Models**: `cpp1.members.coolplanet-foundation.org`, `cpp2.members.coolplanet-foundation.org`

### **Tool Selection Rationale:**

#### **V0.dev for Dynamic Content (Member Platform)**
- **BDD Integration**: Direct mapping from BDD scenarios to components
- **Complex User Flows**: Excellent for KYC, wallet management, portfolio features
- **Real-time Updates**: Superior for dynamic, personalized content
- **AI-Powered Development**: Natural language component generation
- **Production-Ready Code**: Complete React components with styling

#### **Builder.io for Static Content (Public Site)**
- **Visual Design Excellence**: Superior for marketing and informational content
- **SEO Optimization**: Built-in SEO tools and performance optimization
- **Marketing Focus**: Perfect for landing pages, about pages, legal content
- **Visual Workflow**: Drag-and-drop design for non-technical content creation
- **Brand Consistency**: Visual design system for consistent branding

### **Cost Analysis:**
- **V0.dev Pro**: $20/month for dynamic member platform development
- **Builder.io Personal**: $19/month for static public site development
- **Total Development Cost**: $39/month (6 months = $234)
- **Production Cost**: $0 (build-time only workflow)

**Next Steps:**
1. **Phase 1**: Set up V0.dev + React + Vite workflow for member platform
2. **Phase 1**: Write comprehensive BDD scenarios for dynamic features
3. **Phase 1**: Implement II authentication, wallet, KYC, portfolio features
4. **Phase 2**: Set up Builder.io + React + Vite workflow for public site
5. **Phase 2**: Create visual design and marketing content with Builder.io
6. **Future**: Design CPP standards for bundle holder ENS integration

This hybrid architecture optimizes tool selection for content type while maintaining React + Vite consistency and leveraging the strengths of both V0.dev (BDD-driven development) and Builder.io (visual design excellence). 