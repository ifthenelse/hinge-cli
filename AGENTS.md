# hinge-cli: instructions for AI coding agents

## Scope
- This file is the canonical contract for coding agents (Claude Code, Codex, Copilot, Aider).
  Tool adapters (`CLAUDE.md`, `.github/copilot-instructions.md`) only point here.
- Purpose and non-goals: `intent.md`. Architecture: `docs/architecture.md`. Decisions: `docs/adr/`.

## Stack
- Rust (edition 2024, Cargo). Single binary `hinge`; modules in `src/`, unit tests inline (`#[cfg(test)]`).

## Commands
- Verify everything: `scripts/verify` (`cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`)
- Build: `cargo build --release`

## Verification
- Run `scripts/verify` before handing work back; it must pass. CI runs the same command and is the authoritative gate.
- Add or update tests for behaviour you change. Live or state-changing tests must be opt-in.

## Constraints
- Commit messages follow Conventional Commits (see `.gitmessage.txt`). Keep changes small and focused.
- Do not reformat unrelated code. Do not edit generated files or `Cargo.lock` by hand; the version lives only in `Cargo.toml`.
- Never commit secrets or `.env` files. `.env.example` holds names and safe placeholders only.

## Approval boundaries
Ask the user before: adding or upgrading a dependency, changing CI/CD or infrastructure, changing a data
schema or public contract, deleting files, touching secrets or credentials, pushing or opening a PR.

## Hand-off
Summarise what changed and why, which checks you ran with their result, and any open risk or follow-up.
