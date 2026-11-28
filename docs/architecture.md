# Architecture

One canonical location for the current architecture. Link to authoritative diagrams rather than copying them.

## System context
hinge-cli is a single Rust binary (`hinge`, built with Cargo) that a Hinge member runs locally. It makes
direct HTTPS calls to Hinge's own backend (`prod-api.hingeaws.net`) using session headers the member
captured from their own phone — it does not drive the Hinge app's UI and does not go through Appium
at runtime. Appium/WebDriverAgent is a one-time exploration tool (`hinge inspect`), not part of the
request path. There is no server component and no multi-user backend: each install talks only to
Hinge's API and to the member's own secret store.

```
member's terminal ── hinge <command> ──▶ src/main.rs
                                            │
                              ┌─────────────┼─────────────┐
                              ▼             ▼              ▼
                       session_store.rs   api.rs       appium.rs (Appium, `doctor`/`inspect` only)
                              │             │
                              ▼             ▼
                        secrets.rs      prod-api.hingeaws.net (Hinge's real API)
                     (op:// bw:// keychain:// cmd:// env://)
```

## Components and boundaries
- **`src/main.rs`** — clap-based entrypoint (tokio runtime); owns argument parsing, the `--json` flag, and the
  top-level error-to-exit-code mapping. No business logic lives here beyond wiring.
- **`src/api.rs`** — the only module that talks to Hinge. `call()` issues authenticated GETs and
  normalizes 401/403/non-2xx into plain errors that never echo the token. `describe_tier()` maps the
  real `/store/v2/account` response shape into `TierInfo`.
- **`src/session_store.rs`** — owns where a session lives (file or password-manager reference),
  validates untrusted JSON on import (`parse_session`, header allowlist, `authorization` required),
  and the cross-platform config-path logic.
- **`src/secrets.rs`** — resolves `scheme://` references (`env://`, `op://`, `bw://`, `keychain://`,
  `cmd://`) to a secret value. The only module allowed to shell out for credentials.
- **`src/appium.rs`** — W3C WebDriver client for Appium/WebDriverAgent, used solely by `hinge doctor`
  and `hinge inspect`, the exploration commands used while reverse-engineering the API. Not used by
  `tier`/`limits`/`auth`.

## Data flows and integration contracts
1. **Capture (manual, one-time, outside the CLI):** member runs mitmproxy against their own phone
   with `tools/hinge_token.py` to extract session headers, per `docs/auth.md`.
2. **Import:** `hinge auth import <file>` → `session_store::parse_session` (validates/allowlists
   headers) → `save_session` (0600 file, or writes nothing if `HINGE_SESSION_REF` is set) → source
   file deleted by default.
3. **Authenticated call:** `hinge tier`/`limits` → `load_session()` (file or `resolve_secret` via
   `HINGE_SESSION_REF`) → `api::call(path, session)` → Hinge REST API → response mapped and printed.

Endpoint inventory, headers observed, and the (unimplemented) login/attestation sequence:
[`docs/api-inventory.md`](api-inventory.md).

## Request identity
Every HTTP request (Hinge's API and the local Appium server) goes out with the user agent of a generic
Android phone, `api::USER_AGENT` (Chrome on a Pixel-class device). It replaces the `user-agent` of the
captured session, which is no longer stored. The other captured `x-device-*`/`x-os-version` headers are
sent unchanged, so they still describe the phone the session came from.

## Trust boundaries
The member's captured session header is the only secret in the system; everything downstream of
`load_session()` treats it as sensitive and no code path logs or echoes it (see `secret` command,
which reports length only). See the [threat model](security/threat-model.md).
