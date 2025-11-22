# Asset Certification Implementation Plan

**Purpose:** Extract working certification from fti_newsletter_archive into ic-assets-env
**Status:** Planning
**Date:** 2025-11-21

---

## Problem

ic-certified-assets has private API - can't wrap it for custom /env.js.

## Solution

**Extract proven certification from fti_newsletter_archive:**
- Uses `ic-certification` crate (not ic-certified-assets)
- Dual-tree V1+V2 pattern (dfx 0.27+ compatible)
- Embeds Vite assets via build.rs
- Custom HTTP certification
- **Works on IC mainnet!**

---

## Modules to Extract

### From fti_newsletter_archive/src/frontend/src/

**1. cert_v2.rs** (Core certification)
- Dual-tree implementation (V1 + V2)
- Fork/labeled hash functions
- CEL expression generation
- Witness generation

**2. storage.rs** (Asset storage)
- CertifiedAssetHashes struct with dual-tree
- NestedTree for V2
- RbTree for V1

**3. build.rs** (Asset embedding)
- Parse Vite manifest
- Generate `include_bytes!()` code
- Create asset HashMap

**4. serving.rs** (HTTP serving)
- Route matching
- Certification header generation
- MIME type detection

---

## Generalization Steps

### 1. Create ic-assets-env Modules

```
sdk/ic-assets-env/src/
├── cert.rs          # Dual-tree certification (from cert_v2.rs)
├── storage.rs       # Asset storage (from storage.rs)
├── build.rs         # Vite manifest parser (from build.rs)
└── serving.rs       # HTTP serving (from serving.rs)
```

### 2. Dependencies

Add to ic-assets-env/Cargo.toml:
```toml
ic-certification = "2.6"
ic-representation-independent-hash = "2.6"
sha2 = "0.10"
hex = "0.4"
base64 = "0.21"
```

**Remove:** ic-certified-assets dependency

### 3. Build Script Pattern

**Copy build.rs template** for embedding assets:
- Parse dist/.vite/manifest.json
- Generate include_bytes!() for each asset
- Create get_asset_map() function

### 4. Certification Trait

```rust
pub trait AssetCertification {
    fn certify_asset(&mut self, path: &str, body: &[u8], headers: &[(String, String)]);
    fn serve_certified(&self, path: &str) -> HttpResponse;
}
```

### 5. Storage Implementation

```rust
pub struct CertifiedAssets {
    tree_v1: RbTree,        // Backwards compat
    tree_v2: NestedTree,    // V2 pattern
    assets: HashMap<String, &'static [u8]>,  // From build.rs
}
```

---

## Testing Approach

### 1. Test in cpp_icp_platform

Update test_frontend to use new certification:
```bash
cargo build -p test_frontend
dfx deploy test_frontend
curl http://canister-id.localhost:8003/  # Should work!
```

### 2. Migrate cpf_org

Replace stub store.rs with certified implementation:
```bash
cargo update -p ic-assets-env
cargo build
dfx deploy
# HTTP access should work
```

### 3. Validate on IC

Deploy to IC mainnet testnet and verify certification

---

## Implementation Phases

**Phase 1:** Extract certification modules (1-2 hours)
- Copy cert_v2.rs → cert.rs (generalize)
- Copy storage.rs (adapt types)
- Test compiles

**Phase 2:** Vite build integration (1 hour)
- Adapt build.rs for generic use
- Test with test_frontend

**Phase 3:** HTTP serving (1 hour)
- Adapt serving.rs
- Route /env.js separately
- Serve static from embedded assets

**Phase 4:** Testing (1 hour)
- Local deployment
- HTTP access validation
- Certificate verification

**Phase 5:** Migration (1 hour)
- Update cpf_org
- Update docs
- Commit

---

## Success Criteria

- ✅ test_frontend serves assets via HTTP (not just Candid)
- ✅ /env.js works via HTTP
- ✅ Certification headers present
- ✅ No "Response verification failed" errors
- ✅ Works with Vite/Svelte builds
- ✅ cpf_org migrated and working

---

## Next Session Action Items

1. Extract cert_v2.rs into ic-assets-env/src/cert.rs
2. Extract storage patterns
3. Create build.rs template
4. Test with test_frontend
5. Fix any compilation issues
6. Deploy and verify HTTP access works

---

**Related:**
- fti_newsletter_archive/src/frontend/src/cert_v2.rs (source)
- docs/architecture/runtime-config.md
- RUNTIME_CONFIG_IMPLEMENTATION_PLAN.md

**Last Updated:** 2025-11-21
**Blocked by:** Session length (500k+ tokens)
**Next:** Fresh session to implement extraction
