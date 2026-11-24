# Intent: hinge-cli

Durable statement of intent. It is not a requirements backlog.

## Business problem
Hinge has no official API client and no data-export tooling for a member's own account: checking
subscription tier, remaining likes, or automating anything beyond what the mobile app's UI exposes
requires manually tapping through the app. There is no way to script, audit, or integrate a member's
own Hinge usage from a terminal or CI job.

## Intended outcomes
- A member can read, edit and check their own profile and account (subscription tier, like/rose
  limits) from the command line, on Windows, macOS or Linux, without opening the mobile app.
- Session credentials are supplied by the member (captured from their own device) and stored through
  a password manager or a local file with restrictive permissions — never hardcoded, never shared
  across users.
- The reverse-engineered API surface is documented (`docs/api-inventory.md`) so the client's behaviour
  is auditable rather than opaque.
- **Future (not yet built):** browse Discover/Standout candidates from the terminal — rendering their
  photos inline (Kitty/iTerm2/Sixel graphics protocol where supported, ASCII art otherwise) — and let
  the member manually like or pass. This is a manual, one-decision-at-a-time interface change (a
  different client for the same human decision the app already asks for), not automation of the
  decision itself.

## Non-goals
- No AI- or script-driven decision-making on who to like/pass/message — every like/pass/message
  action is a single explicit command run by the member for one candidate; there is no "auto-swipe"
  or "auto-match" mode, and none is planned.
- No automated/bulk messaging, mass-liking, or any action that mimics bot-like use of the platform.
- No multi-account or on-behalf-of-other-users operation — this is a single-member, bring-your-own-session tool.
- No reimplementation of Hinge's phone+SMS login flow in this version — it requires passing Apple
  App Attest, which a plain CLI cannot satisfy (see `docs/api-inventory.md`, "Open risk for `hinge auth login`").
- No live testing (security or otherwise) against Hinge's production infrastructure from this repo's CI.

## Success measures
| Measure | Target | Source |
| --- | --- | --- |
| `hinge tier` / `hinge limits` return correct, current account data | Matches what the app shows | Manual comparison against the Hinge app |
| Session import works without a captured credential ever being committed or logged | 0 leaked secrets | `gitleaks` in CI, manual review |
| CLI runs unmodified on Windows, macOS, Linux | `scripts/verify` green on all three | CI matrix (currently ubuntu-only — see open item in `docs/testing.md`) |

## Accountable owners
- Andrea Collet — sole maintainer; owns design, security review, and release decisions.
