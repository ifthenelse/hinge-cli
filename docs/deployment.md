# Deployment

hinge-cli has no server and no hosted environment — "deployment" means publishing a versioned Rust crate/binary
that members build or install locally. There is no staging/production split for a local CLI.

## Environments and promotion path
| Environment | Purpose | Promotion from | Approval |
| --- | --- | --- | --- |
| local (dev) | member's own machine, running against the real Hinge API with their own session | n/a | n/a |
| source / crates.io | distributable, versioned release | `main` branch, tagged | maintainer (self-review; single-maintainer repo, see CONTRIBUTING.md) |

## Release
1. Bump `version` in `Cargo.toml` on `main` (the single version source) and run `cargo build` to refresh `Cargo.lock`.
2. `cargo build --release` to produce `target/release/hinge`.
3. `scripts/verify` must be green.
4. `git tag vX.Y.Z && git push --follow-tags`, then `cargo publish` if releasing to crates.io (not yet automated — no publish job exists in
   `.github/workflows/`; this is a manual step today, tracked as an open item below).

## Smoke tests
After install (`cargo install --path .`, or `cargo install hinge-cli` once published):
- `hinge doctor` — checks Appium reachability (only relevant if `inspect` will be used).
- `hinge auth status` — confirms a session, once imported, is accepted by Hinge's API.
- `hinge tier` — confirms a real authenticated call round-trips correctly.

## Rollback
There is nothing running to roll back (no server). If a published version is broken:
- `cargo yank --version <version>` to stop new installs resolving to it.
- Publish a patched version; members on the broken version re-run `cargo install hinge-cli --force`.

## Open item
No CI job publishes a release yet, and CI/tests currently run on `ubuntu-latest` only even though the
tool targets Windows/macOS/Linux — a Windows/macOS matrix job is not yet in `.github/workflows/ci.yml`.
