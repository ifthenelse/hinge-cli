# Architecture

One canonical location for the current architecture. Link to authoritative diagrams rather than copying them.

## System context
hinge-cli is a single Node.js/TypeScript binary (`hinge`) that a Hinge member runs locally. It makes
direct HTTPS calls to Hinge's own backend (`prod-api.hingeaws.net`) using session headers the member
captured from their own phone — it does not drive the Hinge app's UI and does not go through Appium
at runtime. Appium/WebDriverAgent is a one-time exploration tool (`hinge inspect`), not part of the
request path. There is no server component and no multi-user backend: each install talks only to
Hinge's API and to the member's own secret store.

```
member's terminal ── hinge <command> ──▶ src/cli.ts
                                            │
                              ┌─────────────┼─────────────┐
                              ▼             ▼              ▼
                        session-store.ts  api.ts      session.ts (Appium, `inspect` only)
                              │             │
                              ▼             ▼
                      secrets.ts      prod-api.hingeaws.net (Hinge's real API)
                     (op:// bw:// keychain:// cmd:// env://)
```

## Components and boundaries
- **`src/cli.ts`** — commander-based entrypoint; owns argument parsing, the `--json` flag, and the
  top-level error-to-exit-code mapping. No business logic lives here beyond wiring.
- **`src/api.ts`** — the only module that talks to Hinge. `call()` issues authenticated GETs and
  normalizes 401/403/non-2xx into plain errors that never echo the token. `describeTier()` maps the
  real `/store/v2/account` response shape into `TierInfo`.
- **`src/session-store.ts`** — owns where a session lives (file or password-manager reference),
  validates untrusted JSON on import (`parseSession`, header allowlist, `authorization` required),
  and the cross-platform config-path logic.
- **`src/secrets.ts`** — resolves `scheme://` references (`env://`, `op://`, `bw://`, `keychain://`,
  `cmd://`) to a secret value. The only module allowed to shell out for credentials.
- **`src/session.ts`** — Appium/WebDriverAgent connection used solely by `hinge inspect`, the
  exploration command used while reverse-engineering the API. Not used by `tier`/`limits`/`auth`.

## Data flows and integration contracts
1. **Capture (manual, one-time, outside the CLI):** member runs mitmproxy against their own phone
   with `tools/hinge_token.py` to extract session headers, per `docs/auth.md`.
2. **Import:** `hinge auth import <file>` → `session-store.parseSession` (validates/allowlists
   headers) → `saveSession` (0600 file, or writes nothing if `HINGE_SESSION_REF` is set) → source
   file deleted by default.
3. **Authenticated call:** `hinge tier`/`limits` → `loadSession()` (file or `resolveSecret` via
   `HINGE_SESSION_REF`) → `api.call(path, session)` → Hinge REST API → response mapped and printed.

Endpoint inventory, headers observed, and the (unimplemented) login/attestation sequence:
[`docs/api-inventory.md`](api-inventory.md).

## Trust boundaries
The member's captured session header is the only secret in the system; everything downstream of
`loadSession()` treats it as sensitive and no code path logs or echoes it (see `secret` command,
which reports length only). See the [threat model](security/threat-model.md).
