# Changelog

User-visible and operationally material changes. Generate entries from the agreed release process
rather than editing several version sources by hand. Format: [Keep a Changelog](https://keepachangelog.com), [SemVer](https://semver.org).

## [Unreleased]

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
