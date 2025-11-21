# Port Allocation for CPP Repos

**Purpose:** Prevent port conflicts when running multiple repos locally
**Maintained by:** cpp_icp_platform (coordination repo)
**Source of Truth:** config/ports.dhall (Dhall config with imports)

---

## Dhall Config System

Port allocation is defined in **config/ports.dhall** (Dhall format).

Repos import this file to get their assigned ports:
```dhall
let ports = ../../cpp_icp_platform/config/ports.dhall
in ports.cpf_org  -- { dfx = 8002, vite = 3001 }
```

See **tools/dhall-config.js** for config utility.

---

## Vite Dev Server Ports (npm run dev)

| Repository | Port | URL | Purpose |
|------------|------|-----|---------|
| **fti_newsletter_archive** | 3000 | http://localhost:3000 | Newsletter portal frontend |
| **cpf_org** | 3001 | http://localhost:3001 | Public website frontend |
| **cpf_members** | 3002 | http://localhost:3002 | Members portal frontend |

### Configuration

Set in `vite.config.ts`:
```typescript
export default defineConfig({
  server: {
    port: 3001, // Assigned port
  },
});
```

---

## dfx Replica Ports (dfx start --background)

| Repository | Port | Canister URL Pattern | Purpose |
|------------|------|----------------------|---------|
| **fti_newsletter_archive** | 8000 | `http://{canister-id}.localhost:8000` | Newsletter canisters |
| **cpf_members** | 8001 | `http://{canister-id}.localhost:8001` | Members portal canisters |
| **cpf_org** | 8002 | `http://{canister-id}.localhost:8002` | Public site canister |
| **cpp_icp_platform** | 8003 | `http://{canister-id}.localhost:8003` | SDK test canisters |

### Configuration

Set in `dfx.json`:
```json
{
  "networks": {
    "local": {
      "bind": "127.0.0.1:8002",
      "type": "ephemeral"
    }
  }
}
```

---

## Local Development Workflow

### Running Multiple Repos

```bash
# Terminal 1: newsletters
cd ~/git/fti_newsletter_archive
dfx start --background  # Port 8000
npm run dev             # Port 3000

# Terminal 2: cpf_org
cd ~/git/cpf_org
dfx start --background  # Port 4946
npm run dev             # Port 3001

# Terminal 3: cpf_members
cd ~/git/cpf_members
dfx start --background  # Port 8001
npm run dev             # Port 3002
```

### Cross-Canister Local URLs

**cpf_org runtime config (local):**
```bash
dfx deploy frontend --argument '(record {
  log_level = "debug";
  members_url = "http://localhost:3002";                           # cpf_members Vite
  newsletter_url = "http://localhost:3000";                        # newsletters Vite
  ga_measurement_id = null;
  usercentrics_settings_id = null;
})'
```

---

## Port Conflict Resolution

### Check What's Using a Port

```bash
lsof -i :8000
lsof -i :3001
```

### Kill Specific Port

```bash
lsof -ti:8000 | xargs kill -9
```

### Kill All dfx Instances

```bash
pkill -f "dfx start"
dfx stop  # In each repo directory
```

---

## Adding New Repos

**Next available ports:**
- Vite: 3003, 3004, 3005...
- dfx: 8003, 8004, 8005...

**Process:**
1. Update this document with new allocation
2. Set in new repo's vite.config.ts and dfx.json
3. Document in new repo's .claude/CLAUDE.md or README.md

---

**Last Updated:** 2025-11-21
**Version:** 1.0.0
