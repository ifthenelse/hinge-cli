# hinge-cli

An unofficial, read-only command-line client for your own Hinge account — the only one that tells
you upfront this almost certainly violates Hinge's Terms of Service, and commits to never
automating a swipe.

[![CI](https://github.com/ifthenelse/hinge-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/ifthenelse/hinge-cli/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/github/license/ifthenelse/hinge-cli)](LICENSE)

It talks directly to Hinge's real HTTP API (reverse-engineered from captured app traffic, not UI
automation at runtime) using a session you capture from your own phone, and reports things the app
shows but doesn't let you script: your subscription tier, like/rose limits. Business intent and
non-goals (no bots, no bulk actions, no multi-account) live in [intent.md](intent.md).

| | |
| --- | --- |
| Owner | Andrea Collet |
| Support | GitHub issues on this repository (single-maintainer project — see CONTRIBUTING.md) |

## Features

- **Read your account** — subscription tier and remaining like/rose limits, from the terminal.
- **Bring your own session** — no reimplementation of Hinge's phone+SMS/App Attest login; you
  capture a session from your own logged-in app and import it.
- **No server, no third party** — a single local binary calls Hinge's API directly; nothing is
  proxied through infrastructure this project runs.
- **Manual only, by design** — every action is one explicit command for your own account; no
  auto-swipe, no bulk messaging, no multi-account support, and none planned (see
  [intent.md](intent.md#non-goals)).

## Quickstart

Requires Node.js >=22.18.

```sh
git clone https://github.com/ifthenelse/hinge-cli.git
cd hinge-cli
npm install
npm run build
```

Then capture a session from your own device ([docs/auth.md](docs/auth.md)), import it, and check
your account:

```sh
hinge auth import <captured-session-file>
hinge auth status
hinge tier
hinge limits
```

## Usage

- Verify everything (typecheck + tests): `scripts/verify`
- Build: `npm run build`
- Import a captured session: `hinge auth import <file>`
- Check the stored session: `hinge auth status`
- Read your account: `hinge tier`, `hinge limits`

## Configuration

| Variable | Default | Description |
| --- | --- | --- |
| `HINGE_SESSION_FILE` | none | Path to a session file to use instead of the password-manager-backed store |
| `HINGE_SESSION_REF` | none | Reference to a password-manager entry holding the session |
| `HINGE_APPIUM_URL` | `http://127.0.0.1:4723` | Appium server used only by `hinge inspect` (exploration), not by `tier`/`limits`/`auth` |
| `HINGE_UDID` | none | Device UDID for session capture |
| `HINGE_BUNDLE_ID` | `co.hinge.mobile.ios` | Hinge app bundle id for session capture |
| `HINGE_TEAM_ID` | none | Apple team id for session capture |

Full list and defaults: [.env.example](.env.example). No `.env` is required to build or test — only
to import/use a real session.

## Architecture

A single local binary (`hinge`) calls `prod-api.hingeaws.net` directly with a member-supplied
session; there is no server component. Full summary: [docs/architecture.md](docs/architecture.md);
decisions: [docs/adr/](docs/adr/).

## Deployment and recovery

- Deployment, promotion path and rollback: [docs/deployment.md](docs/deployment.md)
- Operational procedures: [docs/runbooks/](docs/runbooks/)

## Contributing and security

See [CONTRIBUTING.md](CONTRIBUTING.md) and [SECURITY.md](SECURITY.md). Agents: [AGENTS.md](AGENTS.md).

## License

[MIT](LICENSE)
