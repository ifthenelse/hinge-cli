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

## Review roles
- Reviewer: @ifthenelse approves all changes; CODEOWNERS requires their review on every path, and on
  `.github/` and `docs/security/` specifically.
- Single-maintainer repository: self-review follows the PR requirements above (verify green, tests
  updated, docs/ADR updated) plus a 24h cooling-off before merging anything touching `src/secrets.ts`,
  `src/session-store.ts`, `src/api.ts`, or `.github/workflows/`.

## Escalation
Security concerns: see [SECURITY.md](SECURITY.md). Blocked reviews or anything else: open a GitHub
issue — there is no other channel for this single-maintainer project.
