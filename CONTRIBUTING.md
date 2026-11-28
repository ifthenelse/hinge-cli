# Contributing

Applies to humans and coding agents alike.

## Workflow
1. Branch from `main` (or use a git worktree per task; never share one working copy between two agents).
2. Make a focused change; keep unrelated refactors out.
3. Run `scripts/verify` locally. CI runs the same command and is the authoritative gate.
4. Open a pull request: shared and production code enters only through a PR with required checks.

## Commit convention
Conventional Commits, see `.gitmessage.txt`: `type(scope)!: description`.
Enable the template once: `git config commit.template .gitmessage.txt`.

## Pull request requirements
- Description of the change and its reason; link to the related decision or ticket.
- `scripts/verify` green; tests added or updated for changed behaviour.
- Docs, ADR or changelog updated when behaviour, contracts or architecture change.
- Dependency changes reviewed explicitly (new packages, version bumps, lockfile diff).
- **Every PR bumps `Cargo.toml`'s version per [SemVer 2.0.0](https://semver.org/):** patch for
  fixes/docs/internal changes, minor for backward-compatible additions, major for breaking changes.
  Edit the `version` line in `Cargo.toml` (the single version source) and run `cargo build` so
  `Cargo.lock` follows; add a matching entry to `CHANGELOG.md`. Git tags are still created only at
  actual release time (`docs/deployment.md`'s Release steps), not on every PR.

## README demo GIF
`.github/assets/demo.gif` is recorded, not hand-made. After a change to the CLI's output or
command list, re-record it with [VHS](https://github.com/charmbracelet/vhs): `cargo build --release && vhs
.github/assets/demo.tape` (run from the repo root; the tape builds a throwaway `hinge` shim and a
fake `$HOME`, so it never touches a real session or calls the live Hinge API).

## Review roles
- Reviewer: @ifthenelse approves all changes; CODEOWNERS requires their review on every path, and on
  `.github/` and `docs/security/` specifically.
- Single-maintainer repository: self-review follows the PR requirements above (verify green, tests
  updated, docs/ADR updated) plus a 24h cooling-off before merging anything touching `src/secrets.rs`,
  `src/session_store.rs`, `src/api.rs`, or `.github/workflows/`.

## Releases
Two on-demand GitHub Actions workflows, both triggered manually (Actions tab or `gh workflow run
<file>`) — nothing runs on every push:
- **`release.yml`** — builds `hinge` for Linux, macOS (x86_64 + arm64) and Windows, tags
  `v<Cargo.toml version>`, and publishes a GitHub Release with the binaries and that version's
  `CHANGELOG.md` section as the notes. Bump `Cargo.toml`'s version first; it refuses to re-release an
  existing tag.
- **`pages.yml`** — publishes `docs/index.html` (the GitHub Pages site) as-is.

## Escalation
Security concerns: see [SECURITY.md](SECURITY.md). Blocked reviews or anything else: open a GitHub
issue — there is no other channel for this single-maintainer project.
