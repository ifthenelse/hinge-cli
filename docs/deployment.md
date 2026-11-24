# Deployment

hinge-cli has no server and no hosted environment — "deployment" means publishing a versioned npm
package that members install locally. There is no staging/production split for a local CLI.

## Environments and promotion path
| Environment | Purpose | Promotion from | Approval |
| --- | --- | --- | --- |
| local (dev) | member's own machine, running against the real Hinge API with their own session | n/a | n/a |
| npm registry | distributable, versioned release | `main` branch, tagged | maintainer (self-review; single-maintainer repo, see CONTRIBUTING.md) |

## Release
1. `npm version <major|minor|patch>` on `main` (bumps `package.json`, the single version source).
2. `npm run build` (`tsc -p tsconfig.build.json`) to produce `dist/`.
3. `scripts/verify` must be green.
4. `git push --follow-tags`, then `npm publish` (not yet automated — no publish job exists in
   `.github/workflows/`; this is a manual step today, tracked as an open item below).

## Smoke tests
After install (`npm install -g hinge-cli` or `npm link`):
- `hinge doctor` — checks Appium reachability (only relevant if `inspect` will be used).
- `hinge auth status` — confirms a session, once imported, is accepted by Hinge's API.
- `hinge tier` — confirms a real authenticated call round-trips correctly.

## Rollback
There is nothing running to roll back (no server). If a published npm version is broken:
- `npm deprecate hinge-cli@<version> "<reason>"` to warn installers off it.
- Publish a patched version; members on the broken version re-run `npm install`.

## Open item
No CI job publishes to npm yet, and CI/tests currently run on `ubuntu-latest` only even though the
tool targets Windows/macOS/Linux — a Windows/macOS matrix job is not yet in `.github/workflows/ci.yml`.
