# Frontend Technology Evaluation for CPP Platform

## Executive Summary

This document evaluates frontend technology options for the Cool Planet Platform (CPP) Progressive Web App (PWA), incorporating critical lessons learned from SSL certificate investigations and Internet Computer (IC) deployment best practices.

**Key Finding**: Based on SSL certificate investigation results, **React with IC Asset Canister** is the recommended approach for optimal SSL certificate support and deployment simplicity.

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

## Architectural Choice: Frontend Deployment Strategy

The framework choice is fundamentally about **how to deploy the frontend**:

### **Option A: Next.js + Asset Canister**
```
Frontend: Next.js → Static Export → Asset Canister ("type": "assets")
Backend: Separate Motoko/Azle canisters
SSL: ✅ Automatic via asset canister
```

**Advantages:**
- ✅ **SSL Certificate Support**: Automatic provisioning
- ✅ **Superior Static Export**: Next.js excels at static generation
- ✅ **Built-in Optimizations**: Image, font, performance optimizations
- ✅ **Claude Code Integration**: Excellent AI-assisted development support

**Disadvantages:**
- ⚠️ **Complexity**: More complex build and deployment process
- ⚠️ **Bundle Size**: Larger than React + Vite
- ❌ **Limited Builder.io Integration**: Builder.io works best with React + Vite

### **Option B: React + Vite + V0.dev + Azle Canister (RECOMMENDED)**
```
Frontend: React + Vite + V0.dev → Azle Canister ("type": "azle")
Static Assets: Asset Canister ("type": "assets") for SSL, images, CSS
Backend: Separate Motoko canister for core business logic
Workflow: BDD → Claude → V0.dev → React Components → IC Deployment
SSL: ✅ Automatic via asset canister
```

**Advantages:**
- ✅ **SSL Certificate Support**: Automatic provisioning via asset canister
- ✅ **BDD-First Development**: Direct mapping from BDD scenarios to components
- ✅ **Claude Integration**: AI generates Builder.io components from BDD scenarios
- ✅ **Visual Development**: Immediate visual feedback for non-designers
- ✅ **Code Generation**: Automatic React component creation from Builder.io
- ✅ **IC Community Dominance**: Most IC projects use React + Vite
- ✅ **Hot Reload**: Vite's instant development feedback
- ✅ **Smaller Bundle**: No Next.js framework overhead
- ✅ **Dynamic Content**: Server-side rendering for personalized experiences

**Disadvantages:**
- ⚠️ **Learning Curve**: Team needs to learn Builder.io workflow
- ⚠️ **Tool Dependency**: Requires Builder.io subscription and setup
- ⚠️ **Azle Limitations**: File processing and library compatibility issues

### **Why Option B is Recommended**

**Option B (React + Vite + V0.dev + Azle Canister) is recommended** because it combines the SSL certificate benefits of asset canisters with the BDD-first development workflow that V0.dev enables, while leveraging Azle for dynamic content. This approach provides:

1. **BDD-First Development**: Start with behavior specifications you understand
2. **AI-Powered Development**: V0.dev provides superior BDD scenario understanding
3. **Claude Integration**: AI assistance for component generation and refinement
4. **IC Community Alignment**: React + Vite is the dominant IC pattern
5. **SSL Certificate Support**: Asset canisters provide automatic SSL for all domains
6. **Dynamic Content**: Azle canister handles server-side rendering and personalized experiences

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

Given we must use Option B (Azle canister with React + Vite + V0.dev + asset canister for SSL), the framework choice is driven by:

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
- **Frontend Asset Canister** (`"type": "assets"`): Serves the PWA with SSL certificate support
- **Core User Management Canister** (`"type": "motoko"`): Handles KYC, wallet cache, risk assessment, audit trail, ZK proof generation
- **External Integration Canister** (`"type": "azle"`): Manages provider APIs, webhooks, external service communication
- **Notification System Canister** (`"type": "azle"`): Handles real-time notifications and event broadcasting

## Workflow Comparison: Static vs Dynamic Frontend

### **The Two Real Options**

#### **Option A: Static Workflow (Next.js)**
- **Frontend**: Next.js → Static Export → Asset Canister
- **Backend**: Separate Motoko canister for all API/business logic
- **External Integration**: Separate Azle canister (with known limitations)
- **Workflow**: Build-time static generation, client-side routing
- **SSL**: Automatic via asset canister

#### **Option B: Dynamic Workflow (React + Azle)**
- **Frontend**: React → Azle Canister (dynamic rendering)
- **Static Assets**: Asset Canister (for SSL, images, CSS, etc.)
- **Backend**: Separate Motoko canister for core business logic
- **External Integration**: Same Azle canister handles both frontend and external APIs
- **Workflow**: Server-side rendering, dynamic content generation
- **SSL**: ✅ Automatic via asset canister (same as Option A)

### **Option A: Static Workflow (Next.js + Asset Canister)**

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
    },
    "external-integration": {
      "type": "azle",
      "main": "src/external_integration/main.ts"
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
- ❌ **Azle Integration**: May have compatibility issues with Azle limitations

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
- ⚠️ **Team Experience**: May need Vue training for team
- ⚠️ **IC Integration**: Less proven IC agent integration
- ⚠️ **Community**: Smaller than React community

### **Option B: Dynamic Workflow (React + Azle Canister)**

#### **Architecture**
```json
// dfx.json
{
  "canisters": {
    "static-assets": {
      "type": "assets",
      "source": ["public/"]
    },
    "frontend": {
      "type": "azle",
      "main": "src/frontend/main.ts"
    },
    "core-user-management": {
      "type": "motoko",
      "main": "src/core_user_management/main.mo"
    }
  }
}
```

#### **React + Vite Configuration**
```javascript
// vite.config.js
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  build: {
    outDir: 'dist',
    emptyOutDir: true,
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ['react', 'react-dom'],
          ic: ['@dfinity/agent', '@dfinity/auth-client']
        }
      }
    }
  },
  base: './'
})
```

#### **Advantages**
- ✅ **Dynamic Content**: Server-side rendering for personalized content
- ✅ **Real-time Updates**: Live wallet updates, KYC status changes
- ✅ **Simpler Architecture**: Single Azle canister handles frontend + external APIs
- ✅ **IC Community Adoption**: More examples and patterns available
- ✅ **Proven Azle Integration**: Well-established React + Azle patterns
- ✅ **Smaller Bundle**: No Next.js framework overhead
- ✅ **Faster Development**: Vite's instant hot reload

#### **Disadvantages**
- ❌ **Performance**: Slower than static serving (server-side rendering)
- ❌ **Azle Limitations**: File processing and library compatibility issues
- ❌ **SEO**: Limited SEO support compared to static generation
- ❌ **Bundle Size**: Larger than static export due to server-side rendering

## Detailed Comparison Matrix

## IC Community Adoption Analysis

### **Current IC Ecosystem Trends**

Based on analysis of IC projects and community patterns:

#### **React + Vite Dominance**
- **Most IC Projects**: Use React + Vite for frontend
- **Examples**: DSCVR, OpenChat, many DeFi projects
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

## Head-to-Head Comparison: Static vs Dynamic Workflow

| Feature                     | Option A: Static (Next.js)  | Option B: Dynamic (React + Azle) |
| --------------------------- | --------------------------- | -------------------------------- |
| **SSL Certificate Support** | ✅ Automatic                 | ✅ Automatic (via asset canister) |
| **Performance**             | ✅ Excellent                 | ❌ Slower (server-side rendering) |
| **Bundle Size**             | ❌ Larger (Next.js overhead) | ✅ Smaller (React only)           |
| **Dynamic Content**         | ❌ Limited (client-side)     | ✅ Excellent (server-side)        |
| **Real-time Updates**       | ❌ Manual implementation     | ✅ Built-in server-side updates   |
| **IC Community Adoption**   | ❌ Limited examples          | ✅ Dominant pattern               |
| **Azle Compatibility**      | ✅ N/A (No Azle)             | ✅ Proven integration             |
| **Build Complexity**        | ❌ Complex (Next.js)         | ✅ Simple (Vite)                  |
| **Maintenance**             | ❌ Complex                   | ✅ Standard (React + Azle)        |
| **SEO Support**             | ✅ Excellent                 | ❌ Limited                        |
| **PWA Support**             | ✅ Excellent                 | ✅ Good                           |
| **Learning Curve**          | ❌ Steep (Next.js)           | ✅ Gentle (React)                 |

## Implementation Recommendations

### **Phase 1: Technology Selection (Week 1)**

**Recommended Choice: Option B - Dynamic Workflow (React + Azle)**

**Rationale:**
1. **IC Community Dominance**: Most IC projects use React + Azle, extensive community support
2. **Proven Integration**: Well-established patterns for React + Azle integration
3. **Dynamic Content**: Excellent for real-time updates and personalized content
4. **Smaller Bundle**: No Next.js framework overhead
5. **Faster Development**: Vite's instant hot reload
6. **Team Experience**: Most developers familiar with React
7. **Standard Pattern**: React + Azle is the standard IC frontend pattern

**Alternative: Option A - Static Workflow (Next.js)**
- **Use if**: Team values performance and SEO over community support
- **Trade-offs**: Limited IC community examples, larger bundle, more complex build
- **Risk**: Unknown IC integration patterns



### **Phase 2: Architecture Setup (Weeks 2-3)**

```bash
# Project structure (Next.js)
cpp-frontend/
├── app/                     # Next.js 13+ app directory
│   ├── layout.tsx           # Root layout
│   ├── page.tsx             # Home page
│   ├── donate/              # Donation flow pages
│   ├── portfolio/           # Portfolio dashboard pages
│   ├── kyc/                 # KYC flow pages
│   └── globals.css          # Global styles
├── components/              # Reusable components
│   ├── ui/                  # UI components
│   ├── forms/               # Form components
│   └── layout/              # Layout components
├── lib/                     # Utility libraries
│   ├── ic-client.ts         # IC integration
│   ├── utils.ts             # Utility functions
│   └── types.ts             # TypeScript types
├── hooks/                   # Custom React hooks
├── public/
│   ├── .well-known/
│   │   └── ic-domains       # coolplanet-foundation.org
│   ├── manifest.json        # PWA manifest
│   └── icons/               # PWA icons
├── out/                     # Static export (for IC asset canister)
├── next.config.js           # Next.js configuration
├── package.json
└── tsconfig.json
```

### **Phase 3: IC Integration (Weeks 4-5)**

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
- **Fallback**: Dynamic workflow if needed (not recommended due to complexity)

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

**Recommended Architecture: Unified V0.dev + React + Vite Approach**

### **Unified Technology Stack: V0.dev + React + Vite + Asset Canisters**

**All frontend development uses the same technology stack:**
- **Frontend Framework**: React + Vite (for both static and dynamic content)
- **AI-Powered Development**: V0.dev (for both static and dynamic content)
- **Deployment**: Asset Canisters (for SSL certificate support)
- **Backend**: Separate Motoko/Azle canisters for business logic

### **Phase 1: Dynamic Member Platform (`members.coolplanet-foundation.org`)**
- **Technology**: React + Vite + V0.dev → Asset Canister
- **Workflow**: BDD → Claude → V0.dev → React Components → IC Deployment
- **Content**: Gated member content, real-time updates, personalized experiences
- **Functionality**: II authentication, wallet management, KYC, portfolio
- **Rationale**: BDD-first workflow with AI-powered development and IC community dominance

### **Phase 2: Static Public Site (`coolplanet-foundation.org`)**
- **Technology**: React + Vite + V0.dev → Asset Canister
- **Workflow**: V0.dev AI Editor → React + Vite Build → IC Deployment
- **Content**: Public content, SEO-optimized, legal/terms pages
- **Functionality**: Login/donate buttons that redirect to member platform
- **Rationale**: Same technology stack, optimized for static content

### **ENS Integration Strategy**
- **ENS Mirror**: `cpf.nft` mirrors `coolplanet-foundation.org` DNS (ICANN independence)
- **NFT Models**: `cpp1.members.coolplanet-foundation.org`, `cpp2.members.coolplanet-foundation.org`
- **Bundle Holders**: Publish to their ENS subdomains following CPP standards

### **Why This Unified Approach:**

1. **Single Technology Stack**: Same tools and workflows for all frontend development
2. **BDD-First Development**: Start with behavior specifications you understand
3. **AI-Powered Development**: V0.dev provides superior BDD scenario understanding
4. **Claude Integration**: AI assistance for component generation and refinement
5. **IC Community Dominance**: React + Vite is the standard IC pattern
6. **SSL Certificate Support**: Asset canisters provide automatic SSL for all domains
7. **Consistent Experience**: Unified look and feel across all platforms

### **Technology Choices:**
- **Frontend Framework**: React + Vite (for both static and dynamic content)
- **AI-Powered Development**: V0.dev (for both static and dynamic content)
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
# 1. V0.dev AI Development
# Create components using V0.dev AI editor
# Optimize for SEO and performance

# 2. React + Vite Build
# Copy React components from V0.dev
# Build static site with React + Vite
# Configure for IC asset canister deployment

# 3. IC Deployment
# Deploy to IC asset canister
# Configure custom domain and SSL
```

### **Domain Strategy:**
- **Public**: `coolplanet-foundation.org` (V0.dev + React + Vite static build)
- **Member Platform**: `members.coolplanet-foundation.org` (V0.dev + React + Vite dynamic build)
- **ENS Mirror**: `cpf.nft` mirrors `coolplanet-foundation.org` DNS (ICANN independence)
- **NFT Models**: `cpp1.members.coolplanet-foundation.org`, `cpp2.members.coolplanet-foundation.org`

**Next Steps:**
1. **Phase 1**: Set up unified V0.dev + React + Vite workflow
2. **Phase 1**: Write comprehensive BDD scenarios for member platform
3. **Phase 1**: Implement II authentication, wallet, KYC, portfolio features
4. **Phase 2**: Create static public site using same V0.dev + React + Vite stack
5. **Phase 2**: Design integrated look and feel across both domains
6. **Future**: Design CPP standards for bundle holder ENS integration

This unified architecture provides consistent technology stack across all frontend development while leveraging V0.dev's AI-powered development capabilities and React + Vite's IC community dominance. 