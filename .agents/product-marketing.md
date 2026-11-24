# Product Marketing Context

**Document version:** v1
**Last updated:** 2026-11-24

## Product Overview
**One-liner:** An unofficial, open-source CLI for your own Hinge account.
**What it does:** hinge-cli talks directly to Hinge's own API (reverse-engineered from real app traffic, not UI automation) using a session you capture from your own phone. It lets you check your subscription tier and like/rose limits, and read/edit your own profile — all from a terminal. Planned: browse Discover/Standout candidates with photos rendered in-terminal (Kitty/iTerm2/Sixel, ASCII fallback) and manually like/pass, one decision at a time.
**Product category:** unofficial API client / developer CLI tool (same shelf as unofficial Spotify/Twitter CLI clients), not a "dating app" or "dating automation" product.
**Product type:** free, open-source software (MIT license), self-hosted/local — no SaaS, no account with us, no server we operate.
**Business model:** none. $0 price, no monetization planned. Success is adoption/usefulness, not revenue.

## Target Audience
**Target companies:** n/a — individual consumer/developer tool, not sold to companies.
**Decision-makers:** the installer themselves; no buying committee.
**Primary use case:** a technically comfortable Hinge member wants to check or manage their own account without opening the app — e.g. scripting a tier check, or (future) swiping through candidates without the app's UI.
**Jobs to be done:**
- Check account state (tier, limits) quickly, scriptably, without the app.
- Manage/edit your own profile from a terminal-first workflow.
- (Future) Review and act on Discover/Standout candidates in an interface you control.
**Use cases:**
- A developer who prefers terminal workflows and is curious/annoyed that no official API exists.
- Someone auditing exactly what data Hinge's API exposes about their own account.

## Personas
B2C, single-actor product — skipped per skill guidance (no multi-stakeholder buying committee).

## Problems & Pain Points
**Core problem:** Hinge has no official API, no data export, and no way to interact with your own account except through its mobile app's UI.
**Why alternatives fall short:**
- The official app: no scripting, no terminal workflow, UI-only.
- UI-automation bots (Appium/Selenium-driven): slow, brittle, and indistinguishable from "automating the platform" in a way this project deliberately avoids.
**What it costs them:** time (manual UI checks), and no way to integrate account state into their own tools/scripts.
**Emotional tension:** mild — this is a convenience/curiosity-driven tool, not a painkiller. No urgency, no money at stake. Overclaiming urgency here would read as dishonest.

## Competitive Landscape
**Direct:** other unofficial dating-app CLI/API clients — see the market-map step (`gtm-planner:market-map`) for sourced detail; summary to be merged in once that research returns.
**Secondary:** generic UI-automation scripts (Appium/Selenium bots) for dating apps — fall short because they're slower, more fragile against app updates, and read by platforms as bot-like even when run by the account owner.
**Indirect:** "do nothing" — just use the official app — is the dominant alternative for the vast majority of Hinge members, and will stay that way; this product serves a narrow, self-selecting niche, not a mass market.

## Differentiation
**Key differentiators:**
- Built from real reverse-engineered API traffic (documented in `docs/api-inventory.md`), not UI automation at runtime — faster, more stable against minor app UI changes.
- Explicit, published scope limits: no automation of decisions, no multi-account, no bulk actions (`intent.md`) — a credibility signal in a space where "dating bot" tools are usually shady.
- Password-manager-first credential handling (1Password/Bitwarden/Keychain/etc.) rather than a plaintext token file by default.
**How we do it differently:** positions as an honest, narrow, inspectable dev tool rather than a growth-hacking or "beat the algorithm" product.
**Why that's better:** lower trust bar to clear for a technical audience already skeptical of anything touching a dating platform's private API.
**Why customers choose us:** curiosity + distrust of closed platforms + preference for terminal tools, not a promise of more matches or dates.

## Objections
| Objection | Response |
|-----------|----------|
| "Isn't this against Hinge's ToS?" | Yes, likely — stated plainly in SECURITY.md/threat model, not hidden. This is a bring-your-own-session tool for your own account; use is the member's own risk decision, same as any unofficial API client. |
| "Will this get my account banned?" | Unknown, not promised safe. Documented as an open risk, not glossed over. |
| "Is this a bot / will it mess with other people on the app?" | No — every like/pass/message action is a single explicit command for one candidate; there is no automation of decisions, by design and by `intent.md`'s non-goals. |

**Anti-persona:** anyone looking for automated swiping, bulk messaging, multi-account farming, or an "agentic dating" tool — explicitly out of scope and not what this project will become.

## Switching Dynamics
**Push:** frustration with no official API / no scriptable access to your own data.
**Pull:** curiosity, terminal-native workflow preference, trust earned by published scope limits and open API documentation.
**Habit:** the app already works fine for 99% of members — there's no real habit to break, which caps addressable audience size honestly.
**Anxiety:** ToS/account-ban risk (real, disclosed); trusting a third-party CLI with session credentials (mitigated by password-manager support, never-log-the-secret design).

## Customer Language
**How they describe the problem:** not yet collected — zero real users interviewed (see `/design:user-research` plan, pending).
**How they describe us:** not yet collected.
**Words to use:** "unofficial client," "your own account," "read/edit," "reverse-engineered API," "bring your own session."
**Words to avoid:** "automate," "bot," "agentic," "bypass," "hack Hinge" — anything implying action on other people's behalf or without their knowledge.
**Glossary:**
| Term | Meaning |
|------|---------|
| Session | Captured auth headers from the member's own logged-in app, used to call Hinge's API as them |
| Discover / Standout | Hinge's own names for its candidate-browsing surfaces |

## Brand Voice
| Attribute | Means | Doesn't mean | Example line |
|-----------|-------|--------------|--------------|
| Plain | State what it does and its real limits | Marketing superlatives ("revolutionary") | "Reads and edits your own Hinge profile from the terminal." |
| Disclosed | Say the ToS/legal risk out loud | Hide it in fine print | "This almost certainly violates Hinge's ToS — use at your own risk." |
| Scoped | Name what it won't do | Vague "coming soon, do anything" roadmap talk | "No automated swiping. No bulk messaging. No multi-account." |

**Tone by context:**
- Support (GitHub issues): plain, fast, no deflection.
- Launch (Product Hunt, Reddit): honest framing as a niche dev-tool experiment, not hype.
- Errors (CLI output): never echo secrets; state exactly what failed.
- Sales: n/a, nothing is sold.
- Social: same plain/disclosed/scoped voice — no "agentic dating" or growth-bait language.

**Terms we own:** "unofficial client for your own account."
**Never use (and why):** "agentic dating," "automate dating apps," "bot" — discarded this session as mischaracterizing a manual, single-decision tool and inviting exactly the ToS-violating use this project explicitly rules out.

## Proof Points
**Metrics:** none yet (pre-launch).
**Customers:** none yet.
**Testimonials:** none yet.
**Value themes:** n/a until real usage exists.

## Goals
**Business goal:** none revenue-related — goal is working software, honest positioning, and (per user) a Product Hunt launch for visibility among developers.
**Conversion action:** `git clone`/`npm install` + first successful `hinge tier` call.
**Current metrics:** 0 installs, 0 GitHub stars tracked at time of writing (repo just made public).

## Paid Ads
Account not yet created; budget not yet set. Captured here so the **ads** skill has the real constraints once it exists:
**Target CPA / ROAS:** n/a — no revenue to convert against. Success metric for any paid spend is installs/clicks, not ROAS.
**Hard floor:** no ad spend without an explicit budget number from the maintainer first — none has been given yet.
**Budget and pacing:** TBD — flagged as an open point.
**Conversions that count:** repo visit, star, clone/install — no purchase event exists.
**Known tracking weaknesses:** no analytics installed on the repo beyond GitHub's own traffic insights; no UTM/pixel setup yet.
**Campaigns:** none launched yet — planned: Reddit (r/commandline, r/opensource, r/degoogle-adjacent dev communities), Product Hunt.
**Agent may:** read, compare, flag, draft, recommend.
**Agent may not (without approval):** spend money, publish ads, launch campaigns, change budgets — none of this has been authorized yet.

## Learnings
| Date | Learning | Evidence | Applies to | Status |
|------|----------|----------|------------|--------|
| 2026-11-24 | "Agentic dating"/automation framing was rejected by the maintainer after one round of discussion — it mischaracterizes a manual tool and invites ToS-violating use | Direct maintainer decision, this session | positioning, all copy, PH listing, ad copy | active |

## Changelog
- v1 (2026-11-24) — Initial context, drafted from the repo (README, intent.md, docs/) plus this session's scope correction.
