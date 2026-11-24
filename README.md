# hinge-cli

An unofficial, read-only command-line client for your own Hinge account. It talks directly to
Hinge's real HTTP API (reverse-engineered, not UI automation) using a session you capture from your
own phone, and reports things the app shows but doesn't let you script: your subscription tier,
like/rose limits. For members who want to check or audit their own account from a terminal or a
script, on Windows, macOS or Linux. Business intent lives in [intent.md](intent.md).

| | |
| --- | --- |
| Owner | Andrea Collet |
| Support | GitHub issues on this repository (single-maintainer project — see CONTRIBUTING.md) |

## Architecture

A single local binary (`hinge`) calls `prod-api.hingeaws.net` directly with a member-supplied
session; there is no server component. Full summary: [docs/architecture.md](docs/architecture.md);
decisions: [docs/adr/](docs/adr/).

## Prerequisites

- Git and the toolchain described in AGENTS.md.

## Local setup

1. Clone the repository.
2. Copy `.env.example` to `.env` and fill in local values (never commit `.env`).

## Commands

- Verify everything: `scripts/verify`
- Build: `npm run build` · Import a captured session: `hinge auth import <file>` · Check it:
  `hinge auth status` · Read your account: `hinge tier`, `hinge limits`. Capturing a session in the
  first place is a manual, documented process: [docs/auth.md](docs/auth.md).

## Environment model

| Environment | Purpose | Notes |
| --- | --- | --- |
| local | development, and the only environment this tool runs in | configured through `.env` (see `.env.example`); no `.env` is required to build/test, only to import/use a real session |
| npm registry | distribution | versioned release, see docs/deployment.md — no staging/production split, this is a local CLI |

## Deployment and recovery

- Deployment, promotion path and rollback: [docs/deployment.md](docs/deployment.md)
- Operational procedures: [docs/runbooks/](docs/runbooks/)

## Contributing and security

See [CONTRIBUTING.md](CONTRIBUTING.md) and [SECURITY.md](SECURITY.md). Agents: [AGENTS.md](AGENTS.md).
