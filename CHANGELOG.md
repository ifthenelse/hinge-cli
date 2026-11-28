# Changelog

User-visible and operationally material changes. Generate entries from the agreed release process
rather than editing several version sources by hand. Format: [Keep a Changelog](https://keepachangelog.com), [SemVer](https://semver.org).

## [Unreleased]

## [0.2.3] - 2026-11-28
### Added
- `.github/assets/social-preview.svg` (+ rendered `.png`, 1280×640): link-preview card for Show HN/Reddit/social shares. Not wired up automatically — GitHub's repository social-preview image has no API, so it needs uploading once at Settings → General → Social preview.

## [0.2.2] - 2026-11-28
### Added
- `.github/workflows/release.yml`: on-demand (manual-dispatch only) GitHub Release, building `hinge`
  for Linux, macOS (x86_64 + arm64) and Windows and tagging `v<version>`.
- `.github/workflows/pages.yml`: on-demand (manual-dispatch only) GitHub Pages deploy of `docs/`.
- GitHub Pages enabled (Actions build) at https://ifthenelse.github.io/hinge-cli/; linked from the
  README and set as the repository homepage. Repository topics added.
### Changed
- Branch protection on `main` now requires all four CI checks (`verify`, `codeql`, `secret-scan`,
  `dependency-audit`), not just `verify`.

## [0.2.1] - 2026-11-28
### Added
- `hinge auth capture`: runs the session-capture steps for you (starts the local proxy, prints what
  to do on the phone, waits, imports, cleans up) instead of requiring mitmproxy to be run by hand.
  `hinge auth import <file>` is unchanged and still works for anyone who prefers the manual steps or
  needs to script the capture themselves.

## [0.2.0] - 2026-11-28
### Changed
- **Rewritten in Rust** (edition 2024; clap, tokio, reqwest with rustls, serde). All commands and
  output formats are preserved: `doctor`, `tier`, `limits`, `auth import|status|logout`, `inspect`,
  `secret`, the global `--json` flag, `HINGE_*` environment variables and the `op://`, `bw://`,
  `keychain://`, `cmd://`, `env://` secret references. Appium is now spoken over the W3C WebDriver HTTP
  protocol directly. Build with `cargo build --release` (binary `target/release/hinge`); Node.js is no
  longer needed.
- Every HTTP request now sends the user agent of a generic Android phone. The session's own
  `user-agent` is no longer stored (`hinge_token.py` and `auth import` drop it).
- CI, the Security workflow (`cargo audit`, CodeQL for Rust), Dependabot (cargo), `CONTRIBUTING.md`, the
  README, `AGENTS.md` and `docs/` updated for the Rust toolchain; the demo GIF was re-recorded.
- The version source is now `Cargo.toml` (was `package.json`).
### Removed
- The TypeScript sources, `package.json`, `package-lock.json`, `tsconfig*.json` and `.node-version`.

## [0.1.6] - 2026-11-28
### Added
- `docs/index.html`: single-file landing page for GitHub Pages with an animated ASCII background
  (hearts, roses, swipe cards, chat bubbles, hinges) drawn on a canvas behind the content. Honours
  light/dark and `prefers-reduced-motion` (static frame). `docs/.nojekyll` serves it as-is.

## [0.1.5] - 2026-11-28
### Changed
- Logo: the `>_` prompt chevron is now a pink heart rotated 90° counter-clockwise (tip pointing right),
  keeping the underscore; symbol and light/dark banner SVGs updated.
### Added
- README "Trademarks and affiliation" section: Hinge is a trademark of Hinge, Inc.; the project is
  unaffiliated and claims no rights in their marks.

## [0.1.4] - 2026-11-27
### Changed
- `.gitignore` now ignores `.claude/`, `.codex/` and `launches/` (local agent tooling and launch-pipeline
  artifacts), replacing the narrower `.claude/` entries.

## [0.1.3] - 2026-11-25
### Added
- Project logo (`.github/assets/hinge-cli-symbol-{black,white}.svg`) and a light/dark README banner
  (`.github/assets/banner-{light,dark}.svg`), shown via GitHub's `prefers-color-scheme` `<picture>`
  pattern.
- Recorded CLI demo GIF (`.github/assets/demo.gif`, source `.github/assets/demo.tape`) embedded in
  `README.md`, showing `hinge --help`, a clean "not signed in" disclosure from `hinge tier`, and
  `hinge auth import`. Re-recording instructions in `CONTRIBUTING.md`.

## [0.1.2] - 2026-11-25
### Fixed
- `package.json`'s `description` and the CLI's own `--help` text still said the tool was "driven
  through Appium and WebDriverAgent" — stale since the reverse-engineered HTTP API client was built;
  Appium is only used by the one-time `hinge inspect` exploration command. Corrected both.
- README's `HINGE_APPIUM_URL` default (`localhost:4723`) didn't match the actual code/`.env.example`
  default (`127.0.0.1:4723`). Corrected.

## [0.1.1] - 2026-11-24
### Changed
- Filled functional/technical documentation scaffolding: `intent.md`, `README.md`, `CONTRIBUTING.md`,
  `SECURITY.md`, `docs/architecture.md`, `docs/deployment.md`, `docs/testing.md`,
  `docs/security/threat-model.md`, `.env.example`.
- Overhauled `README.md` for launch: badges, verified Quickstart, Features and Configuration
  sections, disclosed-risk/manual-only positioning.
### Added
- `.agents/product-marketing.md` — product marketing context (positioning, messaging, anti-persona).
