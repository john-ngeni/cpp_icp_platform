# Origins: Internet Identity Derivation Origins & CORS Deep-Dive

**Date:** Original conversation, updated 2025-11-13
**Purpose:** Technical deep-dive on CORS and Internet Identity derivation origins
**Context:** Research and analysis informing CPP architecture decisions
**Source:** ChatGPT conversation exploring IC origin model and browser security

## Cross-Reference: CPP Architecture Documentation

This document contains the **research and technical analysis** that informed CPP's architectural decisions around derivation origins and domain routing. For implementation details across actual CPP repositories, see:

- **[derivation-origins-integration.md](./derivation-origins-integration.md)** - How derivation origins work across all CPP repositories (cpf_org, fti_newsletter_archive, cpf_members)
- **[user-journey-funnel.md](./user-journey-funnel.md)** - Progressive authentication gates and cross-repo identity flows
- **[system-architecture-overview.md](./system-architecture-overview.md)** - Overall CPP ecosystem architecture

**Key Architectural Decisions Based on This Research:**
1. ✅ Use **single domain with path-based routing** for same-origin benefits (recommended)
2. ✅ Use **cpf.nft as canonical derivation origin** for ICANN independence
3. ✅ Configure **alternative origins** for current subdomain architecture
4. ✅ All CPP services share same Internet Identity principal via derivation origin

**Implementation Status:**
- fti_newsletter_archive: ✅ Derivation origin configured
- cpf_members: 🚧 In development, derivation origin planned
- cpf_org: 🚧 In development, derivation origin planned

---

## Original Research Conversation

###### ChatGPT said:

Thought for 24s

Here’s the practical checklist to make CORS work between two (sub)domains—for example, front-end at `https://app.example.com` calling an API at `https://api.example.com`.

# 1) Basics you must return from the API

**On every CORS response from `api.example.com`:**

* **Access-Control-Allow-Origin:**

	* If you don’t use cookies or auth headers, you may send `*`.

	* If you **do** use cookies/authorization (credentials), you **must** echo the exact origin, e.g. `https://app.example.com` (no `*`).

* **Access-Control-Allow-Credentials:** `true` (only if you need cookies/Authorization headers).

* **Vary:** `Origin` (so caches don’t mix responses for different origins).

**On preflight (OPTIONS) responses:**

* **Access-Control-Allow-Methods:** list the methods you’ll accept (e.g. `GET, POST, PUT, DELETE`).

* **Access-Control-Allow-Headers:** list custom headers the browser will send (e.g. `Content-Type, Authorization, X-Requested-With`).

* **Access-Control-Max-Age:** optional; seconds to cache the preflight (e.g. `600`).

# 2) What the browser sends (and how to code it)

From `app.example.com`, when calling cross-origin:

```
jsCopy code// No credentials (simpler)
fetch('https://api.example.com/data', {
  method: 'GET'
});
```

```
jsCopy code// With credentials (cookies / Authorization header)
fetch('https://api.example.com/data', {
  method: 'GET',
  credentials: 'include',          // or XMLHttpRequest.withCredentials = true
  headers: { 'Authorization': 'Bearer <token>' } // if applicable
});
```

If you send non-simple headers (e.g. `Authorization`) or non-simple content types (anything besides
`text/plain`, `multipart/form-data`, `application/x-www-form-urlencoded`), the browser will **preflight** with `OPTIONS`.

# 3) Example server setups

### Node/Express

```
jsCopy codeimport express from 'express';
import cors from 'cors';

const app = express();

const allowedOrigin = 'https://app.example.com';

app.use(cors({
  origin: allowedOrigin,        // cannot be '*'' if credentials: true
  credentials: true,
  methods: ['GET','POST','PUT','DELETE','OPTIONS'],
  allowedHeaders: ['Content-Type','Authorization'],
  maxAge: 600
}));

app.get('/data', (req, res) => {
  res.json({ ok: true });
});

app.listen(3000);
```

### Nginx (proxying to your API)

```
nginxCopy codelocation / {
  if ($request_method = OPTIONS) {
    add_header Access-Control-Allow-Origin "https://app.example.com" always;
    add_header Access-Control-Allow-Methods "GET,POST,PUT,DELETE,OPTIONS" always;
    add_header Access-Control-Allow-Headers "Content-Type,Authorization" always;
    add_header Access-Control-Allow-Credentials "true" always;
    add_header Access-Control-Max-Age 600 always;
    return 204;
  }

  add_header Access-Control-Allow-Origin "https://app.example.com" always;
  add_header Access-Control-Allow-Credentials "true" always;

  proxy_pass http://your_api_upstream;
}
```

# 4) Cookies & SameSite (when using subdomains)

* Subdomains are **different origins** (CORS still applies) but they can be **same-site** for cookies if the eTLD+1 matches (`example.com`) and scheme is the same (schemeful same-site).

* To share cookies with subdomains, set `Domain=.example.com`.

* For API calls triggered by `fetch`, cookies are only sent if `credentials: 'include'`.

* If calls are **cross-site** (different eTLD+1), set cookies with `SameSite=None; Secure`. For same-site subdomains, `Lax` is often fine.

# 5) Common pitfalls (and quick fixes)

* **Using `*`with credentials** → not allowed. Use the exact origin string.

* **Forgetting `Vary: Origin`** → shared caches may leak or block CORS.

* **Redirects** (3xx) → ensure the **final** response (and ideally the redirect responses) include CORS headers.

* **Missing headers on preflight** → browser blocks before your route logic runs.

* **Access-Control-Expose-Headers**: if the client must read custom response headers, expose them:

```
httpCopy codeAccess-Control-Expose-Headers: X-Total-Count, X-RateLimit-Remaining
```

* **Port numbers and schemes matter**: `https://app.example.com:8443` is a different origin from `https://app.example.com`.

# 6) Minimal working matrix

| Need | Client setting | Server headers (minimum) |
|---|---|---|
| Simple GET, no creds | default | `Access-Control-Allow-Origin: *` | 
| GET/POST with JSON body, no creds | default (preflight occurs) | + `Access-Control-Allow-Methods`, `Access-Control-Allow-Headers` | 
| Any request with cookies/Authorization | `credentials: 'include'` | `Access-Control-Allow-Origin: https://app.example.com`, `Access-Control-Allow-Credentials: true` (+ preflight headers if non-simple) | 

# 7) Alternatives when CORS is hard

* **Reverse proxy** the API under the same origin/path (e.g., serve `/api` from the front-end origin).

* **postMessage** for controlled data passing between windows/iframes.

* **Server-to-server** calls when applicable.

If you want, tell me your stack (server, proxy, auth style), and I’ll give you copy-paste-ready config for your exact setup.

*]:pointer-events-auto scroll-mt-(--header-height)" tabindex="-1" dir="auto" data-turn-id="d8032f6e-aa6a-43b6-8f0b-36238ad1a932" data-testid="conversation-turn-3" data-scroll-anchor="false" data-turn="user">

##### You said:

I need a higher level first. When one domain A wants to include something from another domain B, B must explicitly allow for A - and browsers will verify that. Is that anyway accurate...

*]:pointer-events-auto [content-visibility:auto] supports-[content-visibility:auto]:[contain-intrinsic-size:auto_100lvh] scroll-mt-[calc(var(--header-height)+min(200px,max(70px,20svh)))]" tabindex="-1" dir="auto" data-turn-id="d59fbcc1-554c-4431-89fd-cb61f4ac2ba8" data-testid="conversation-turn-4" data-scroll-anchor="false" data-turn="assistant">

###### ChatGPT said:

You’re close! Here’s the high-level picture:

* **Browsers enforce the Same-Origin Policy (SOP):** By default, scripts running on **A** can’t read responses from **B**. That’s the baseline.

* **CORS is B’s opt-in to relax SOP for specific origins:** Server **B** sends HTTP headers (e.g., `Access-Control-Allow-Origin: https://A`) saying “it’s OK for A to read this response.” The browser checks those headers and **only then** lets code on **A** access the response. So yes—B must explicitly allow A, and browsers verify it.

Key nuances:

* **Requests vs. reading responses:** Any site can *send* a request to **B**; CORS governs whether the **response is readable by page A’s JavaScript**. It’s not an access control mechanism for who can hit your server; it’s a browser-enforced rule about who can *see* the data.

* **Some tags don’t need CORS to load, only to inspect:**

	* `<img src="https://B/...">`, `<script src="https://B/...">`, `<link rel="stylesheet">` will usually load without CORS.

	* But JS on **A** can’t read those resources’ contents (e.g., pixel data from an image, stylesheet text) unless **B** allows it via CORS (and for images, the “tainted canvas” rules apply).

* **“Simple” vs “non-simple” requests:** If **A** sends certain headers or methods (e.g., `Authorization`, `PUT`), the browser does a **preflight OPTIONS** check first. **B** must approve that too.

* **Credentials tighten the rule:** If **A** wants cookies or auth headers, **B** must:

	* echo the exact origin (`Access-Control-Allow-Origin: https://A`, not `*`), and

	* send `Access-Control-Allow-Credentials: true`.

* **CORS isn’t authentication/authorization:** It doesn’t prove the caller is A or prevent others from calling **B**; it just tells the browser whether to expose the response to scripts from **A**.

So your summary—“B must explicitly allow A, and browsers verify that”—is accurate, with the caveat that:

1. it’s about **reading** the response in A’s JS, and

2. not all types of inclusion require CORS to load, only to be programmatically inspected.

⠀
*]:pointer-events-auto scroll-mt-(--header-height)" tabindex="-1" dir="auto" data-turn-id="6c92e79a-defa-47c9-b8b5-b5b5ff7d5706" data-testid="conversation-turn-5" data-scroll-anchor="false" data-turn="user">

##### You said:

So loading and running JS/CSS from another domain is fine? But loading e.g. json/xml from another domain backend and parsing it is not....

*]:pointer-events-auto [content-visibility:auto] supports-[content-visibility:auto]:[contain-intrinsic-size:auto_100lvh] scroll-mt-[calc(var(--header-height)+min(200px,max(70px,20svh)))]" tabindex="-1" dir="auto" data-turn-id="bf6a5519-70c9-49c9-81ca-48341aea8632" data-testid="conversation-turn-6" data-scroll-anchor="false" data-turn="assistant">

###### ChatGPT said:

Exactly — that’s a perfect way to phrase it ✅

Here’s how that breaks down in browser security terms:

---

### 🧩 1. “Including” code or static assets

**Loading and executing JS/CSS/images from another domain is allowed by default.**

Examples:

```
htmlCopy code<script src="https://cdn.example.com/lib.js"></script>
<link rel="stylesheet" href="https://static.example.com/theme.css">
<img src="https://img.example.com/photo.jpg">
```

These work without CORS, because:

* They’re **not read programmatically** — the browser just fetches and uses them (executes JS, applies CSS, renders image).

* The Same-Origin Policy (SOP) still applies:

	* You can’t read the *contents* of those resources (e.g., you can’t fetch the JS text with AJAX unless CORS allows it).

	* You can’t inspect their internals (like pixel data from an image) without explicit permission.

---

### 🔒 2. “Fetching” data to read (AJAX / `fetch` / XHR)

**This *is*blocked by SOP unless the server allows it via CORS.**

Examples:

```
jsCopy codefetch('https://api.otherdomain.com/data.json')
  .then(r => r.json()) // ❌ blocked unless CORS headers present
```

Here the browser says:

> “You’re trying to read data from another origin. I’ll only let you see the response if that origin explicitly allows yours (via CORS headers).”

This protects users from a malicious site silently reading their private data from another domain where they might be logged in.

---

### ⚙️ 3. Why the distinction exists

It’s a balance between **usefulness** and **safety**:

* You need to load scripts, images, fonts from CDNs — that’s the *web ecosystem*.

* But if any page could freely read responses from anywhere, that would expose user sessions and data.

So:

* **Execution of remote code/resources** → allowed, sandboxed by the origin model.

* **Reading remote data** → forbidden unless the remote origin opts in (CORS).

---

### Summary table

| Type of resource | Example | Default allowed? | Need CORS to *read* content? | 
|---|---|---|---|
| `<script src>` | External JS | ✅ Load & run | 🚫 Can’t fetch source text | 
| `<link rel="stylesheet">` | External CSS | ✅ Apply | 🚫 Can’t read text | 
| `<img src>` | External image | ✅ Render | 🚫 Can’t access pixel data | 
| `fetch()` / `XMLHttpRequest` | JSON/XML/API | 🚫 Blocked | ✅ Needs CORS | 

---

Would you like a quick diagram of how a browser enforces those layers (load → execute → read)? It makes the SOP vs. CORS relationship very clear visually.

*]:pointer-events-auto scroll-mt-(--header-height)" tabindex="-1" dir="auto" data-turn-id="00986acb-d132-436e-a57e-159f287f04cc" data-testid="conversation-turn-7" data-scroll-anchor="false" data-turn="user">

##### You said:

On Internet Computer, does derivation origin follow a similar principle. Does it get controlled on the side of the origin providing the domain or the domains that are alternates

*]:pointer-events-auto [content-visibility:auto] supports-[content-visibility:auto]:[contain-intrinsic-size:auto_100lvh] scroll-mt-[calc(var(--header-height)+min(200px,max(70px,20svh)))]" tabindex="-1" dir="auto" data-turn-id="8e3c80d7-da09-4f70-a8e4-2a673545ea20" data-testid="conversation-turn-8" data-scroll-anchor="false" data-turn="assistant">

###### ChatGPT said:

Excellent question — and you’re thinking about this at the right conceptual level.

Let’s unpack how **“origin derivation”** and **CORS-like behavior** work in the **Internet Computer (IC)** context, since it’s a bit different from the traditional web but shares the same security philosophy.

---

## 🧩 1. The core similarity

Just like the web, the **Internet Computer** also uses the **origin model** for security:
Every HTTP response ultimately comes from a **distinct origin**, and browsers use that to enforce the **Same-Origin Policy** (SOP).

But in the IC, the *origin* is **derived** from a canister’s identity — meaning the combination of:

* the **canister ID**, and

* the **serving domain** (e.g., `ic0.app`, `icp0.io`, or a custom boundary node domain).

This derivation produces a unique origin for each canister, much like a subdomain in the traditional web.

---

## 🌐 2. How origin derivation works

If you access:

```
cppCopy codehttps://<canister-id>.ic0.app
```

The browser treats that as origin:

```
cppCopy codehttps://<canister-id>.ic0.app
```

— meaning other canisters (e.g., `https://<other-canister-id>.ic0.app`) are **different origins**, even though they share the `ic0.app` suffix.

So, requests between them are **cross-origin** from the browser’s point of view, and **CORS rules apply** just like they would on the traditional web.

---

## 🔒 3. Where control lives

This is the crucial bit that answers your question directly:

> Does derivation origin control come from the origin providing the domain or from the alternate domains?

It’s **controlled by the origin that serves the response** — i.e., the *canister (or backend) providing the data*.
Just like in standard web CORS, the **responding origin decides** who can read its data.

So:

* If **Canister B** serves data, and

* **Canister A** (on a different derived origin) tries to `fetch()` that data,

* Then **Canister B’s response headers** determine whether the browser lets A read it.

That means the control lies on the **server (data provider)** side, not on the “alternate domain” or caller side.

---

## 🧠 4. Why it’s designed that way

The Internet Computer doesn’t redefine browser security — it *extends* it.
Your browser still enforces SOP and CORS exactly as usual. The difference is that “origins” are derived cryptographically from canister IDs, not from arbitrary DNS records.

This means:

* A canister’s HTTP responses are isolated by default (safe by design).

* To make inter-canister frontend calls work (e.g., `fetch` JSON from another canister’s frontend), that target canister must emit **CORS headers** like:

```
perlCopy codeAccess-Control-Allow-Origin: https://<caller-canister-id>.ic0.app
```

* The boundary nodes and gateways (like `icp0.io`, `ic0.app`) simply forward these headers — they **don’t decide policy** themselves.

---

## 📦 5. Example

Say:

* Frontend canister = `aaaaa-aaaaa-aaa...q` → served at `https://aaaaa-aaaaa-aaa.ic0.app`

* API canister = `bbbbb-bbbbb-bbb...q` → served at `https://bbbbb-bbbbb-bbb.ic0.app`

Then the browser sees:

```
javaCopy codeOrigin A = https://aaaaa-aaaaa-aaa.ic0.app
Origin B = https://bbbbb-bbbbb-bbb.ic0.app
```

→ Same-origin policy applies.
If frontend A calls backend B via `fetch('https://bbbbb...')`,
B must reply with:

```
arduinoCopy codeAccess-Control-Allow-Origin: https://aaaaa-aaaaa-aaa.ic0.app
```

Otherwise, the browser blocks A from reading the response — exactly like classic web CORS.

---

## ⚙️ 6. Custom domains and delegation

If canisters are served under **custom domains** via boundary nodes (e.g. `https://app.mydao.org`), the principle is still identical:

* Each domain (or subdomain) defines an origin.

* The responding canister controls whether other domains are allowed to read its responses via CORS headers.

So:

> “Does derivation origin control come from the provider or the alternates?”
> ✅ **It’s always controlled by the provider** (the domain / canister that’s serving the response).

---

Would you like a short diagram showing the relationship between canister IDs, derived origins, and browser enforcement? It’s a neat visual that helps connect the IC origin derivation model with traditional web origins.

*]:pointer-events-auto scroll-mt-(--header-height)" tabindex="-1" dir="auto" data-turn-id="0ca27135-e68e-443d-abf4-2eb7ba968ff6" data-testid="conversation-turn-9" data-scroll-anchor="false" data-turn="user">

##### You said:

yes

*]:pointer-events-auto [content-visibility:auto] supports-[content-visibility:auto]:[contain-intrinsic-size:auto_100lvh] scroll-mt-[calc(var(--header-height)+min(200px,max(70px,20svh)))]" tabindex="-1" dir="auto" data-turn-id="a4c62e77-9723-4d0b-96fd-a580630948ba" data-testid="conversation-turn-10" data-scroll-anchor="false" data-turn="assistant">

###### ChatGPT said:

Perfect — here’s a clear, **conceptual diagram** of how **origin derivation** and **CORS control** work on the Internet Computer (IC).

---

### 🏗️ Browser origin model on the IC

```
lessCopy code              ┌──────────────────────────────────────────────┐
              │ Browser (enforces Same-Origin Policy & CORS) │
              └──────────────────────────────────────────────┘
                             ▲                 ▲
                             │                 │
                             │                 │
          Frontend Canister A│                 │Backend Canister B
       (served at derived origin)              (served at derived origin)
────────────────────────────────────────────────────────────────────────────
   https://aaaaa-aaaaa-aaa.ic0.app             https://bbbbb-bbbbb-bbb.ic0.app
────────────────────────────────────────────────────────────────────────────
               │                                      │
               │ fetch('https://bbbbb-...')           │
               │─────────────────────────────────────▶│
               │                                      │
               │                                      │ Responds with:
               │                                      │   Access-Control-Allow-Origin:
               │                                      │   https://aaaaa-aaaaa-aaa.ic0.app
               │                                      │
               │◀─────────────────────────────────────│
               │                                      │
 Browser checks:                                      │
 - Did B’s response include Access-Control-Allow-Origin?  
 - Does it match A’s origin exactly?  
   ✅ → allow JS in A to read data  
   ❌ → block access (response is opaque)
```

---

### 🌐 Derived origins and their meaning

| Resource | URL | Browser Origin | Notes |
|---|---|---|---|
| Frontend canister | `https://aaaaa-aaaaa-aaa.ic0.app` | `https://aaaaa-aaaaa-aaa.ic0.app` | Runs your JS frontend | 
| Backend canister | `https://bbbbb-bbbbb-bbb.ic0.app` | `https://bbbbb-bbbbb-bbb.ic0.app` | Provides JSON/data | 
| Both under `ic0.app` | ✅ Shared parent domain, but **different origins** |  |  | 

So even though they both use `ic0.app`, the browser treats them as fully separate origins, because the **subdomain (canister ID)** differs — just like `api.example.com` vs `app.example.com`.

---

### 🧠 Key takeaway

* **The “derived origin” = scheme + host + port** (browser’s rule).

* On the IC, that host is unique per canister, so each canister gets its own isolated origin.

* **The responding canister controls sharing** via CORS headers, just like a traditional web API controls whether another domain can read its data.

* The boundary nodes (like `ic0.app` or `icp0.io`) **do not override or decide CORS policy** — they just forward what the canister returns.

---

If you’d like, I can also show how this same flow looks when you use **a custom domain** (like `app.mydao.org` + `api.mydao.org` through the IC boundary nodes), so you can see how CORS and origin derivation behave there too. Would you like that version?

*]:pointer-events-auto scroll-mt-(--header-height)" tabindex="-1" dir="auto" data-turn-id="9fb56e44-1018-4743-88f1-ddc469b10fc6" data-testid="conversation-turn-11" data-scroll-anchor="false" data-turn="user">

##### You said:

yes, interested in how 2 canisters serving different (sub)domains can all have the same derivation origin for internet identity principals and read each others data

*]:pointer-events-auto [content-visibility:auto] supports-[content-visibility:auto]:[contain-intrinsic-size:auto_100lvh] scroll-mt-[calc(var(--header-height)+min(200px,max(70px,20svh)))]" tabindex="-1" dir="auto" data-turn-id="4a4c2f4d-9afe-4e86-b393-f4d55b684c06" data-testid="conversation-turn-12" data-scroll-anchor="false" data-turn="assistant">

###### ChatGPT said:

Excellent — you’re asking about one of the **most subtle and important** design details in Internet Computer (IC) web app architecture:

👉 how multiple canisters (frontend + backend + utilities) can share **the same derivation origin** so that

* Internet Identity (II) sees them as one logical “site,” and

* they can freely share authenticated data in the browser without CORS barriers.

Let’s unpack that carefully.

---

## 🧩 1. The problem: each canister has its own derived origin

By default:

```
arduinoCopy codehttps://aaaaa-aaaaa-aaa.ic0.app  → Origin A
https://bbbbb-bbbbb-bbb.ic0.app  → Origin B
```

The browser treats these as *completely separate origins*.
So:

* CORS blocks fetches unless explicitly allowed.

* Internet Identity (II) derives **different delegation keys** for each origin.

* Even if they belong to the same dapp, they can’t share the same login session.

So “A” and “B” don’t automatically trust or see each other, even if they’re part of the same project.

---

## 🧭 2. How Internet Identity defines “origin”

When you authenticate via **Internet Identity**, it looks at the **browser origin** (scheme + host + port).
It issues an identity delegation **for that specific origin**.

That’s the root of the isolation:
II says, “I’ll only sign requests from this origin.”
So if your app has multiple subdomains (or separate canisters on `ic0.app`), II treats them as different apps.

That’s secure by default — but inconvenient for multi-canister dapps.

---

## 🌐 3. The solution: *a shared derivation origin*

To make multiple canisters appear as *one origin* to browsers and II, the IC provides a way to serve them **under the same derived domain** — e.g., all through a shared custom domain or via a single route on a boundary node.

### Option A: use a custom domain (recommended)

Instead of serving each canister directly under its own `<canister-id>.ic0.app`,
you serve them via a custom domain, such as:

```
arduinoCopy codehttps://app.mydao.org/        → frontend canister
https://app.mydao.org/api/... → backend canister
https://app.mydao.org/assets/ → asset canister
```

All of these share the same **browser origin**:
`https://app.mydao.org`

Then:

* Internet Identity sees them as the same site → one delegation covers them all.

* Browser SOP sees them as same-origin → no CORS needed.

* `fetch` between them just works transparently.

Under the hood, the boundary node routes requests to the appropriate canister using the IC’s routing tables or query parameters.

That’s the cleanest, production-level way to give multiple canisters a single derivation origin.

---

### Option B: the `*.raw.icp0.io` derivation origin (for development or dynamic use)

There’s also a **“derivation origin” concept** in the newer boundary domains like `icp0.io`, where requests can be routed such that multiple canisters effectively share an origin derived from a common root.

Example:

```
cppCopy codehttps://<canister-id>.raw.icp0.io
```

serves data directly from the canister without origin translation, but if you use **a derivation origin header** or **link multiple canisters under a shared derived domain**, they can be grouped under the same logical origin for II and browser access.

However — this approach is lower-level and depends on how the dapp’s boundary node or canister front-end routing is configured.
Most real dapps achieve this using **custom domains**, not by manipulating derivation origins manually.

---

## 🔐 4. How II and CORS behave once origins are unified

Once multiple canisters share a single origin (through a custom domain or explicit derivation origin mapping):

| Layer | Behavior |
|---|---|
| **Browser SOP** | Treats all requests as same-origin → no CORS headers needed | 
| **Internet Identity** | Issues one delegation per origin (e.g. `https://app.mydao.org`) → shared principal | 
| **Session cookies / localStorage** | Shared across all routes under the same origin | 
| **Canister calls** | All authenticated via the same delegated identity keypair | 

This enables patterns like:

* One canister hosting the frontend UI

* Another handling authenticated API calls

* Both sharing the same II session seamlessly

---

## 📦 5. Visual summary

```
bashCopy code                ┌─────────────────────────────┐
                │ Internet Identity (II)      │
                │ Derives delegation for:     │
                │     https://app.mydao.org   │
                └────────────┬────────────────┘
                             │
                  Same origin: https://app.mydao.org
   ┌──────────────────────────────────────────────────────────┐
   │ Browser: Same-Origin Policy                              │
   │                                                          │
   │   /index.html  → Frontend canister (UI)                  │
   │   /api/...     → Backend canister (logic/data)           │
   │   /assets/...  → Asset canister                          │
   │                                                          │
   │ All share cookies, localStorage, and CORS context        │
   └──────────────────────────────────────────────────────────┘
```

---

## 🧠 6. Key takeaway

| Goal | How to achieve |
|---|---|
| Multiple canisters share a login and identity | Serve them under the **same origin** (e.g. custom domain or shared derivation origin) | 
| CORS-free API calls between canisters | Must be same-origin; otherwise add CORS headers manually |
| Same II principal across modules | Derivation origin must match exactly (scheme + host + port) |

---

Would you like me to show a **real configuration example** — e.g., how to map two canisters to one domain (`app.mydao.org`) using the IC’s DNS + boundary node routing, so they appear same-origin to Internet Identity and browsers?

*]:pointer-events-auto scroll-mt-(--header-height)" tabindex="-1" dir="auto" data-turn-id="83bc4e22-75e9-4966-8f0b-d50e989a6f83" data-testid="conversation-turn-13" data-scroll-anchor="false" data-turn="user">

##### You said:

Yes, compare all under a single domain and each under different subdomains

*]:pointer-events-auto [content-visibility:auto] supports-[content-visibility:auto]:[contain-intrinsic-size:auto_100lvh] scroll-mt-[calc(var(--header-height)+min(200px,max(70px,20svh)))]" tabindex="-1" dir="auto" data-turn-id="72cbc415-4fd2-4fba-8855-3d6ef9692eed" data-testid="conversation-turn-14" data-scroll-anchor="false" data-turn="assistant">

###### ChatGPT said:

Perfect — this comparison really crystallizes how the **Internet Computer’s origin model**, **Internet Identity (II)**, and **CORS** interplay.

Let’s break it down visually and conceptually.

---

## ⚙️ Scenario Setup

Let’s imagine your dapp has:

* **Frontend canister:** serves UI assets and JS.

* **Backend canister:** provides JSON data.

* **Internet Identity (II):** used for authentication.

We’ll compare two deployment patterns:

1. **Single domain** — everything served under one origin.

2. **Different subdomains** — each canister gets its own subdomain (different derived origins).

⠀
---

## 🏗️ 1️⃣ All under a single domain

### Example

```
arduinoCopy codehttps://app.mydao.org/           → frontend canister
https://app.mydao.org/api/...    → backend canister
https://app.mydao.org/ii/...     → Internet Identity (delegation UI)
```

### Browser sees:

All requests come from **one origin**:

```
iniCopy codeOrigin = https://app.mydao.org
```

### Behavior matrix

| Feature | Behavior |
|---|---|
| **Browser Same-Origin Policy** | ✅ All same-origin. JS can freely call `/api/...` without CORS. | 
| **CORS headers needed?** | ❌ None required (same-origin requests). | 
| **Internet Identity** | ✅ Issues **one delegation** for `https://app.mydao.org`. All canisters share the same user principal. | 
| **Cookies / localStorage** | ✅ Shared. Auth sessions persist across UI/API routes. | 
| **Cross-canister fetch()** | ✅ Works transparently, no preflights. | 
| **Security boundary** | 🧱 Controlled at backend canister level (your logic, not browser). | 

### Visualization

```
bashCopy codeBrowser Origin: https://app.mydao.org
    ├── /index.html     (frontend canister)
    ├── /api/...        (backend canister)
    ├── /ii/...         (Internet Identity)
```

➡️ Looks and behaves like one unified web app.
No CORS headaches. Shared login. Seamless UX.

---

## 🌐 2️⃣ Each under different subdomains

### Example

```
arduinoCopy codehttps://frontend.mydao.org/  → frontend canister
https://api.mydao.org/       → backend canister
https://identity.ic0.app/    → Internet Identity
```

### Browser sees:

Different origins:

```
javaCopy codeOrigin A = https://frontend.mydao.org
Origin B = https://api.mydao.org
Origin C = https://identity.ic0.app
```

### Behavior matrix

| Feature | Behavior |
|---|---|
| **Browser Same-Origin Policy** | 🚫 Each subdomain = different origin. | 
| **CORS headers needed?** | ✅ Yes. `api.mydao.org` must send `Access-Control-Allow-Origin: https://frontend.mydao.org`. | 
| **Internet Identity** | 🚫 Each origin gets a **separate delegation**. `frontend.mydao.org` and `api.mydao.org` are treated as different apps. | 
| **Cookies / localStorage** | 🚫 Not shared. Login at one subdomain doesn’t apply to another. | 
| **Cross-canister fetch()** | ⚠️ Blocked unless backend enables CORS. | 
| **Security boundary** | ✅ Stronger isolation (each subdomain sandboxed). But more friction. | 

### Visualization

```
nginxCopy codeBrowser Origins
   ├── https://frontend.mydao.org    (frontend)
   ├── https://api.mydao.org         (backend)
   ├── https://identity.ic0.app      (II)
```

Each box is isolated unless CORS + proper delegation linking is configured.

---

## 🔄 Side-by-side Summary

| Aspect | Single Domain (`app.mydao.org`) | Multiple Subdomains (`frontend.`, `api.`) | 
|---|---|---|
| **Browser origin** | Same for all routes | Different per subdomain | 
| **CORS required** | ❌ No | ✅ Yes | 
| **II principal** | One shared principal | Separate per origin | 
| **Cookies / localStorage** | Shared | Isolated | 
| **Ease of frontend↔backend calls** | Seamless | Requires `Access-Control-Allow-Origin` | 
| **Security isolation** | Less granular (app-level) | More granular (subdomain-level) | 
| **Best for** | Unified dapp (UI + API + II) | Multi-tenant or multi-app architecture | 

---

## 💡 3️⃣ Practical IC configuration

### Single-domain setup (recommended for unified dapps)

You register your domain with the IC boundary node routing service:

```
bashCopy codeapp.mydao.org  → frontend canister ID = aaaaa-aaaaa-aaa
                routes /api/* → backend canister ID = bbbbb-bbbbb-bbb
```

Boundary nodes map incoming HTTP paths to the right canister.
To Internet Identity and browsers, all responses come from `https://app.mydao.org`.

→ Shared derivation origin = one II session, no CORS.

---

### Subdomain setup (use only when needed)

If you deliberately keep them separate:

```
cssCopy codefrontend.mydao.org  → canister A
api.mydao.org       → canister B
```

Then `api.mydao.org` must send:

```
yamlCopy codeAccess-Control-Allow-Origin: https://frontend.mydao.org
Access-Control-Allow-Credentials: true
```

And you’ll have **different II delegations** for each subdomain unless you configure II to explicitly allow a shared derivation origin (which still relies on being under one “parent” domain boundary).

---

## 🧠 Bottom Line

| Goal | Recommended Setup |
|---|---|
| One login, one session, seamless UI/API | **Single domain** (shared origin) | 
| Isolated microservices or tenants | **Different subdomains**, with CORS + per-origin II delegation | 
| Traditional dapp pattern on IC | **Custom domain mapped via boundary node**, multiple canisters, single origin | 

---

Would you like me to show how to **configure this mapping** concretely — i.e., the DNS + IC boundary node setup that routes `/api` to one canister and `/` to another (while preserving the shared origin)?

*]:pointer-events-auto scroll-mt-(--header-height)" tabindex="-1" dir="auto" data-turn-id="d9dc86b9-bd6b-4adf-97cb-fa9a4a00d249" data-testid="conversation-turn-15" data-scroll-anchor="false" data-turn="user">

##### You said:

I'm looking to compare using the alternate origins configuration to accommodate different subdomains versus how the different canisters could be configured under a partitioned path namespace

*]:pointer-events-auto [content-visibility:auto] supports-[content-visibility:auto]:[contain-intrinsic-size:auto_100lvh] scroll-mt-[calc(var(--header-height)+min(200px,max(70px,20svh)))]" tabindex="-1" dir="auto" data-turn-id="4d153a0d-269e-4259-840c-4b8907931135" data-testid="conversation-turn-16" data-scroll-anchor="false" data-turn="assistant">

###### ChatGPT said:

Thought for 15s

Great question. You’re basically weighing **two ways to make one dapp span multiple canisters**:

1. **Alternate origins (multiple subdomains / sites that should share one II identity)**

2. **Partitioned path namespace (one domain; different paths route to different canisters)**

⠀
Here’s the crisp comparison.

# What they are

**A) Alternate origins (multi-subdomain)**

* You pick **one “derivation origin”** (e.g. `https://app.mydao.org` or `https://abcde.icp0.io`).

* You serve the app from **other origins** too (e.g. `https://api.mydao.org`, `https://cdn.mydao.org`, or a canister subdomain).

* Internet Identity will issue the **same delegation** to those alternates **if** the derivation origin explicitly authorizes them via a **`.well-known/ii-alternative-origins`** file, and your client sets `derivationOrigin` at login. [internetcomputer.org+2internetcomputer.org+2](https://internetcomputer.org/docs/references/ii-spec?utm_source=chatgpt.com)

**B) Partitioned path namespace (single origin)**

* You serve everything under **one domain** (e.g. `https://app.mydao.org`), and route paths to different canisters:

	* `/` → frontend canister

	* `/api/*` → backend canister

	* `/assets/*` → asset canister

* Boundary nodes / HTTP gateway route by path, but to the browser it’s **one origin**. [internetcomputer.org+1](https://internetcomputer.org/docs/building-apps/frontends/custom-domains/using-custom-domains?utm_source=chatgpt.com)

# How they compare

| Dimension | Alternate origins (subdomains) | Partitioned paths (single domain) |
|---|---|---|
| **Browser origin** | Different per subdomain → **cross-origin** | One origin → **same-origin** | 
| **II principal** | **Shared** across origins *if* alternates are authorized + client sets `derivationOrigin` | **Shared** (always; single origin) | 
| **CORS needs** | **Yes** for cross-origin `fetch/XHR` unless you proxy | **No** (same-origin calls) | 
| **Cookies / storage** | Not shared by default (cross-origin); can use same-site cookies only under same eTLD+1; still cross-origin for JS | Shared (same origin) | 
| **Infra setup** | Maintain `.well-known/ii-alternative-origins` on the derivation origin + configure each alt; client must pass `derivationOrigin` | Configure custom domain and path routing on the IC gateway/boundary node | 
| **Operational isolation** | Stronger blast-radius (subdomain sandboxing) | Tighter coupling; one origin for all | 
| **Caching/CDN knobs** | Per-host caching policies | Simpler, one host’s policy | 
| **SEO / link shape** | Multiple hosts | Single host, clean URLs | 
| **Developer ergonomics** | More moving parts (CORS, storage partitioning, II config) | Simplest (no CORS, one storage space) | 

# When to pick which

* **Pick Alternate Origins** if you *must* host components on different hosts (e.g., legacy subdomains, separate CDNs, or you need subdomain isolation) **but still want one II identity**. You’ll:

	1. Set **`derivationOrigin`** in your auth client to the canonical origin.

	2. Serve **`.well-known/ii-alternative-origins`** from that derivation origin listing every alternate origin.

	3. Add **CORS** on the APIs you call cross-origin (or proxy them). [internetcomputer.org+2internetcomputer.org+2](https://internetcomputer.org/docs/references/ii-spec?utm_source=chatgpt.com)

* **Pick Partitioned Paths** if you control a custom domain and want the **simplest app**: single II session, no CORS, shared cookies/storage, straightforward `fetch('/api/...')`. Configure the HTTP gateway/boundary node to route paths to canisters. [internetcomputer.org+1](https://internetcomputer.org/docs/building-apps/frontends/custom-domains/using-custom-domains?utm_source=chatgpt.com)

# Minimal config snippets (conceptual)

**Alternate origins (client-side II login):**

```
tsCopy codeimport { AuthClient } from "@dfinity/auth-client";

const auth = await AuthClient.create();
await auth.login({
  identityProvider: "https://identity.ic0.app",
  derivationOrigin: "https://app.mydao.org" // canonical
});
```

Serve on `https://app.mydao.org/.well-known/ii-alternative-origins`:

```
arduinoCopy codehttps://frontend.mydao.org
https://api.mydao.org
https://abcde.icp0.io
```

(Each line is an allowed alternate.) [ICP JS SDK Docs+1](https://js.icp.build/auth/v4.2/api/client/interfaces/authclientloginoptions?utm_source=chatgpt.com)

**Partitioned paths (one origin):**

* Register `app.mydao.org` as a custom domain.

* Route via the gateway: `/` → frontend canister, `/api/*` → backend canister.
* Now all browser traffic is same-origin. [internetcomputer.org+1](https://internetcomputer.org/docs/building-apps/frontends/custom-domains/using-custom-domains?utm_source=chatgpt.com)

# Common pitfalls (and how to avoid them)

* **Forgetting the well-known file** → II will mint different principals per origin. Ensure the derivation origin publishes the alternates exactly (scheme + host + port). [internetcomputer.org](https://internetcomputer.org/docs/references/ii-spec?utm_source=chatgpt.com)

* **Setting `derivationOrigin`but not hosting from it** → The client must *both* set `derivationOrigin` and the derivation origin must authorize the current page’s origin. [ICP JS SDK Docs](https://js.icp.build/auth/v4.2/api/client/interfaces/authclientloginoptions?utm_source=chatgpt.com)

* **CORS with alternates** → API canisters must send `Access-Control-Allow-Origin: https://frontend.example` (and credentials rules as usual). MDN’s CORS guide is a good checklist. [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/CORS?utm_source=chatgpt.com)

---

If you tell me your current hostnames and canister layout, I can sketch the exact files/headers and a step-by-step for either route (or a hybrid: single domain now, alternates later).

![](origins/favicons.jpg)

![](origins/favicons_2.jpg)

![](origins/favicons_3.jpg)

Sources

*]:pointer-events-auto scroll-mt-(--header-height)" tabindex="-1" dir="auto" data-turn-id="e9dd21ff-87ff-45a2-b876-ccde1a66d656" data-testid="conversation-turn-17" data-scroll-anchor="false" data-turn="user">

##### You said:

How is routing via the gateway configured

*]:pointer-events-auto [content-visibility:auto] supports-[content-visibility:auto]:[contain-intrinsic-size:auto_100lvh] scroll-mt-[calc(var(--header-height)+min(200px,max(70px,20svh)))]" tabindex="-1" dir="auto" data-turn-id="cb0cabc7-3d41-4557-906d-126dead326c4" data-testid="conversation-turn-18" data-scroll-anchor="false" data-turn="assistant">

###### ChatGPT said:

Thought for 38s

Short version: the **HTTP gateways (a.k.a. boundary nodes)** map **one custom domain → one canister ID**. You configure that mapping on the gateway; you don’t upload a routing table that splits `/api` vs `/` across multiple canisters. If you want many canisters under one domain (partitioned paths), you front them with a **router/edge canister** and fan out via **inter-canister calls**. Here’s how both pieces work.

# How to configure gateway routing (custom domain ↔ canister)

1. **DNS records**

⠀
* `CNAME` your domain to `YOURDOMAIN.icp1.io` (traffic to the gateways).

* `TXT` at `_canister-id.YOURDOMAIN` with the **target canister ID** (this is the binding).

* `CNAME` `_acme-challenge.YOURDOMAIN` → `_acme-challenge.YOURDOMAIN.icp2.io` (for TLS). [internetcomputer.org](https://internetcomputer.org/docs/building-apps/frontends/custom-domains/using-custom-domains)

1. **Prove ownership from the canister**

⠀
* In the **serving canister**, publish `.well-known/ic-domains` listing the custom domain(s), and ensure the file is deployed (via `.ic-assets.json*`). [internetcomputer.org](https://internetcomputer.org/docs/building-apps/frontends/custom-domains/using-custom-domains)

1. **Register with the gateways**

⠀
* Optional: validate:
* `GET https://icp0.io/custom-domains/v1/YOURDOMAIN/validate`

* Register:
* `POST https://icp0.io/custom-domains/v1/YOURDOMAIN`
* (You’ll see the `canister_id` the domain is bound to.) Updates later are via `PATCH`, removal via `DELETE`.) [internetcomputer.org](https://internetcomputer.org/docs/building-apps/frontends/custom-domains/using-custom-domains)

> Implication: A **single domain maps to a single canister ID** at a time (note the singular `_canister-id` TXT and the registration payload). To point the domain at another canister, you change the TXT and `PATCH` the registration. [internetcomputer.org+1](https://internetcomputer.org/docs/building-apps/frontends/custom-domains/using-custom-domains)

# Putting multiple canisters under one domain (partitioned paths)

Because gateways don’t path-split to multiple canisters, you typically:

* **Use a router/front canister** (the one bound to the domain) that:

	* serves assets for `/`, `/assets/*`

	* exposes HTTP endpoints like `/api/*`

	* **calls other canisters** internally (IC inter-canister calls) to fulfill `/api/*` and returns the JSON to the browser.
	* Result: everything is **same-origin** to the browser (`https://app.example`), so no CORS; Internet Identity issues **one delegation** for that origin. (If you later also serve from other hosts, use **Alternative Frontend Origins**.) [internetcomputer.org](https://internetcomputer.org/docs/building-apps/authentication/alternative-origins)

# If you must use multiple subdomains (alternate origins)

Keep separate hosts (e.g., `frontend.example`, `api.example`) but **share the same II principal** by:

* Logging in with `derivationOrigin` set to your canonical origin; and

* Publishing `/.well-known/ii-alternative-origins` on that canonical origin listing the alternates.
* (Up to 10 alternates, no paths/trailing slash.) You’ll still need **CORS** between hosts. [internetcomputer.org](https://internetcomputer.org/docs/building-apps/authentication/alternative-origins)

If you tell me your canister IDs and desired URL layout, I can sketch the minimal router canister surface (endpoints + inter-canister calls) so `/api/*` and `/assets/*` live cleanly under one domain.

![](origins/favicons_4.jpg)

Sources

*]:pointer-events-auto scroll-mt-(--header-height)" tabindex="-1" dir="auto" data-turn-id="c1d8b161-a131-4874-99f1-be4ef3fe9b21" data-testid="conversation-turn-19" data-scroll-anchor="false" data-turn="user">

##### You said:

Does having a router/front canister not introduce latency with inter-canister calls for everything - or does that need to be mitigated with careful caching/TTL?

*]:pointer-events-auto [content-visibility:auto] supports-[content-visibility:auto]:[contain-intrinsic-size:auto_100lvh] scroll-mt-[calc(var(--header-height)+min(200px,max(70px,20svh)))]" tabindex="-1" dir="auto" data-turn-id="86489e33-6ae5-43e5-9063-76f9b4fdc183" data-testid="conversation-turn-20" data-scroll-anchor="false" data-turn="assistant">

###### ChatGPT said:

Thought for 42s

Short answer: **yes, a router/front canister adds a hop**, so naïvely putting it in the hot path for *every* call will add latency. But you can keep it fast with a few design choices:

## What actually adds latency

* **Each inter-canister hop costs time.** Update hops go through consensus (hundreds of ms to seconds), while query hops are single-node and much faster. So chaining updates through a router is the slowest pattern. [internetcomputer.org](https://internetcomputer.org/docs/tutorials/developer-liftoff/level-2/2.2-advanced-canister-calls?utm_source=chatgpt.com)

* **Inter-canister calls are an `await`** (even for queries) — so unnecessary fan-out/fan-in increases tail latency. [internetcomputer.org](https://internetcomputer.org/docs/building-apps/best-practices/troubleshooting?utm_source=chatgpt.com)

## How to keep it snappy

### 1) Keep the router on the read path only, and only with queries

* Make your router expose **read endpoints** that aggregate data using **query** methods only. Queries don’t hit consensus, so they’re fast. [internetcomputer.org](https://internetcomputer.org/docs/tutorials/developer-liftoff/level-2/2.2-advanced-canister-calls?utm_source=chatgpt.com)

* Use **composite queries** so a query in the router can call other canisters’ **query** methods directly (no update round), which is the low-latency way to compose multi-canister reads. [Medium+2internetcomputer.org+2](https://medium.com/dfinity/composite-queries-horizontal-scaling-for-multi-canister-dapps-e766e62bdea9?utm_source=chatgpt.com)

* For **writes (updates)**, let the **client call the target canister directly** instead of piping the update through the router. That removes a whole consensus round in your critical path. (Router can still serve discovery/IDL and capability checks.) [internetcomputer.org](https://internetcomputer.org/docs/tutorials/developer-liftoff/level-2/2.2-advanced-canister-calls?utm_source=chatgpt.com)

### 2) Parallelize and minimize hops

* If the router must call N canisters, **issue calls in parallel** and only await once; avoid serial chains.

* Prefer **one router → many leaf canisters** over router → intermediate → leaf chaining (each extra hop adds variance). These are general IC best-practices for latency. [internetcomputer.org](https://internetcomputer.org/docs/building-apps/best-practices/troubleshooting?utm_source=chatgpt.com)

### 3) Cache at the right layers (reads)

* **Gateway/edge caching**: Serve **certified HTTP** responses so boundary nodes can cache safely; honor normal HTTP caching (`Cache-Control`, `ETag`) and the IC’s certification headers. This offloads repeat reads at the edge. [internetcomputer.org+2internetcomputer.org+2](https://internetcomputer.org/docs/references/http-gateway-protocol-spec?utm_source=chatgpt.com)

* **Browser caching**: Set appropriate `Cache-Control` (and `stale-while-revalidate` if compatible) for static and semi-static data. (Asset canisters support standard headers.) [internetcomputer.org](https://internetcomputer.org/docs/references/asset-canister?utm_source=chatgpt.com)

* **In-canister caching**: Memoize hot, read-only data in **heap** for queries; touch **stable memory** on updates and refresh the heap view lazily. DFINITY’s best-practices explicitly call out heap vs. stable memory latency differences. [internetcomputer.org](https://internetcomputer.org/docs/building-apps/best-practices/troubleshooting?utm_source=chatgpt.com)

### 4) Co-location & topology

* **Place tightly-coupled canisters on the same subnet** where possible to reduce cross-subnet latency for inter-canister calls. (DFINITY suggests subnet placement/migration when troubleshooting latency.) [internetcomputer.org](https://internetcomputer.org/docs/building-apps/best-practices/troubleshooting?utm_source=chatgpt.com)

### 5) When to skip the router entirely

* If you don’t need **same-origin** semantics, you can let the browser call multiple canisters directly (no router hop). You’ll then handle **CORS** and (optionally) configure **Alternative Frontend Origins** for a shared II principal — trading latency for cross-origin complexity. (We covered this earlier.)

## Rules of thumb

* **Reads:** Router is fine **if** it aggregates via **composite queries** + caching.

* **Writes:** Avoid routing updates through the router; go **client → target canister**.

* **Cache aggressively** (edge + browser + heap) and **parallelize** fan-out.

* If p95 still bites, revisit **hop count**, **subnet placement**, and **data denormalization**.

If you want, tell me which endpoints are “hot” (read vs write) and how many canisters you fan out to; I’ll sketch a concrete plan (which calls should be composite queries, where to cache, and which to bypass the router).

![](origins/favicons_5.jpg)

![](origins/favicons_6.jpg)

Sources

*]:pointer-events-auto scroll-mt-(--header-height)" tabindex="-1" dir="auto" data-turn-id="c99ba48a-33aa-435e-9ffa-85017aedb2af" data-testid="conversation-turn-21" data-scroll-anchor="false" data-turn="user">

##### You said:

what is p95

*]:pointer-events-auto [content-visibility:auto] supports-[content-visibility:auto]:[contain-intrinsic-size:auto_100lvh] scroll-mt-[calc(var(--header-height)+min(200px,max(70px,20svh)))]" tabindex="-1" dir="auto" data-turn-id="3cddeed9-b98f-42a4-ae73-b2dce080fbad" data-testid="conversation-turn-22" data-scroll-anchor="true" data-turn="assistant">

###### ChatGPT said:

Great question — this is a **performance metrics term** that comes up a lot in distributed systems and web ops.

### 🧩 Definition

**p95 (95th percentile latency)** means:

> “95% of requests complete *at or below* this time.”

So if your API’s **p95 latency** is 400 ms, that means 95% of all requests finish within 400 ms — but 5% of requests take longer than that.

---

### 📊 Why it matters

Average (mean) latency can look fine while users still see bad slowdowns, because a small tail of slow requests dominates perceived performance.

| Metric | What it tells you | Typical goal |
|---|---|---|
| **p50** (median) | What a “typical” request feels like | “Normal” speed | 
| **p95** | The start of the “tail” | Quality of service for almost all users | 
| **p99 / p99.9** | Extreme tail latency | Rare, but key for real-time systems | 

On the Internet Computer (or any distributed app), things like cross-subnet calls, consensus rounds, or occasional network retries will show up in the tail metrics (p95, p99), not in the average.

---

### 💡 Example

Suppose you measure 1000 HTTP requests:

* 950 finish in ≤ 250 ms

* 50 finish slower (up to 2 s)

Then:

* **Average latency** ≈ 300 ms (looks fine)

* **p95 latency** = 250 ms (95% within 250 ms)

* **p99 latency** = 1.8 s (1% very slow)

You’d report:

> p50 = 120 ms, p95 = 250 ms, p99 = 1.8 s.

---

### ⚙️ In practice

When people say things like:

> “We need to keep p95 under 400 ms,”

they mean the system should feel fast *for almost all users* — not just on average.

---

Would you like me to show how to **measure and visualize p95 latency** for IC canister HTTP responses (e.g., via browser metrics or logs)?

ChatGPT can make mistakes. Check important info. See [Cookie Preferences]().

[chatgpt.com](https://chatgpt.com/c/69127512-ec38-8326-a987-996f24642777)