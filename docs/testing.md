# Testing

## Strategy
Unit tests cover the modules that don't need a live Hinge session or a real phone: secret resolution,
session parsing/storage, and the API client's response mapping (success, 401, 500, free-tier fallback)
against a one-shot local HTTP server (which also asserts the Android `user-agent` header). Nothing in CI calls Hinge's real API or Appium — those require
live, member-specific state (a captured session, a connected iPhone) that isn't safe or reproducible
in CI.

## Suites
| Suite | Location | Speed | Run with |
| --- | --- | --- | --- |
| unit | `#[cfg(test)]` modules in `src/*.rs` | fast (<1s) | `cargo test` / `scripts/verify` |
| integration (live Hinge API) | none yet | — | manual: `hinge tier` against a real imported session |
| e2e (Appium/`hinge inspect`) | none | slow, opt-in | manual, requires a connected iPhone + WebDriverAgent |

## Fixtures and environments
Unit tests use literal fixture objects (fake sessions, fake `Account` JSON shapes) inline in
the `tests` modules of `src/*.rs` — no fixtures touch a real captured session file. Tests that would touch a live
or state-changing system (a real Hinge session, a real phone) are intentionally not automated; they
are manual smoke checks (see `docs/deployment.md`).

## Definition of a passing release
`scripts/verify` green in CI (`cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`), plus: a manual `hinge auth status` /
`hinge tier` check against one real account before publishing, since no automated test exercises
the real API contract.
