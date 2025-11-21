# CPP Development Tools

Shared utilities for CPP project repositories.

**Authors:** Fourth Transition Initiative
**Operator:** Cool Planet Foundation

---

## dhall-config.js

Generates JSON config from Dhall files with imports.

### Usage in Application Repos

**Copy to your repo:**
```bash
cp ../cpp_icp_platform/tools/dhall-config.js scripts/
```

**Add to package.json:**
```json
{
  "scripts": {
    "config:generate": "node scripts/dhall-config.js generate",
    "deploy:local": "node scripts/dhall-config.js deploy local",
    "deploy:preprod": "node scripts/dhall-config.js deploy preprod",
    "deploy:production": "node scripts/dhall-config.js deploy production"
  }
}
```

**Or run directly from cpp_icp_platform** (if sibling repos):
```bash
node ../cpp_icp_platform/tools/dhall-config.js generate
```

### Commands

**generate [env]** - Generate config JSON from Dhall
```bash
node tools/dhall-config.js generate           # → config/local.json
node tools/dhall-config.js generate preprod   # → config/preprod.json
```

**deploy [env]** - Deploy canister with config
```bash
node tools/dhall-config.js deploy local
node tools/dhall-config.js deploy preprod
```

**print [env]** - Print config as JSON
```bash
node tools/dhall-config.js print local
```

**args [env]** - Print Candid init args
```bash
node tools/dhall-config.js args preprod
```

### Config Structure

**Required in your config/local.dhall:**
```dhall
{
  runtime_config = {
    -- Fields depend on your canister's Config type
    log_level = "debug",
    members_url = "http://...",
    -- ... other fields
  }
}
```

### Example: cpf_org

**config/local.dhall:**
```dhall
let ports = ../../cpp_icp_platform/config/ports.dhall

in {
  ports = ports.cpf_org,
  runtime_config = {
    log_level = "debug",
    members_url = "http://canister.localhost:${Natural/show ports.cpf_members.dfx}",
    -- ...
  }
}
```

**Usage:**
```bash
npm run config:generate  # Generate config/local.json
npm run deploy:local     # Deploy with local config
```

---

## Requirements

Each repo using this tool must have:
- `config/{env}.dhall` files
- `dhall-to-json-cli` as dev dependency
- `runtime_config` field in Dhall output

---

**See Also:**
- docs/implementation/RUNTIME_CONFIG_IMPLEMENTATION_PLAN.md
- config/ports.dhall (port allocation)
