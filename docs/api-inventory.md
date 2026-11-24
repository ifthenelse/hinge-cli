# Hinge API inventory (observed, unofficial)

Captured once from the stock iOS app on the author's own account. No tokens, ids or personal data are recorded here.
The API is private and undocumented: it can change without notice and its use may breach Hinge's terms of service.

| Area | Host | Calls observed |
| --- | --- | --- |
| REST (JSON) | `prod-api.hingeaws.net` | `GET /store/v2/account`, `/likelimit`, `/like/v2`, `/connection/v2`, `/profilestate/profile`, `/boost`, `/user/config`, `/config/v3/preferences`, `/notification/v1/settings`; `POST /user/v3/public`, `/content/v2/public`, `/prompts`, `/store/v6/product`, `/offers/v2/eligible` |
| Chat (gRPC over HTTP/2) | `chat.hingeprod.net` | `chat.v1.ChatService/ListConversations`, `ListMessages`, `SendMessage` (fires by itself when a chat is opened; not a user message) |
| Telemetry (ignore) | `client-telemetry.hingeprod.net`, `/metric/*`, `events.split.io` | analytics only |

Certificate pinning: none observed on the iOS app (version at capture time).

## Read-only calls the CLI uses
- Tier: `GET /store/v2/account` → `subscription.{tier:int, name, start, end, autoRenew}`. Tier integer mapping still to verify on a paid account.
- Limits: `GET /likelimit` → `likesLeft, superlikesLeft, freeSuperlikesLeft, snoozesLeft, skipUndosLeft`.

## Request headers seen
`authorization`, `user_id`, `x-device-id`, `x-install-id`, `x-session-id`, `x-app-version`, `x-build-number`, `x-os-version`, `x-device-platform`, `x-device-model`, `x-hinge-request-id`.

## Login flow (phone number + SMS), observed sequence
Endpoints and ordering only — no request/response bodies, tokens or codes were read or recorded.

1. `POST /auth/sms/v2/initiate` — requests a code be sent to the phone number
2. `POST /auth/sms/v2` — submits the code
3. `POST /flow` — onboarding/account flow-state check
4. `POST /attestation/v1/challenge` then `POST /attestation/v1/ios/attest` — device/app attestation, almost certainly Apple App Attest/DeviceCheck, called right after the SMS exchange
5. `POST /clienttelemetryauth/token` — a separate token for the telemetry endpoints, not the main session
6. Normal screen calls resume (`/standouts/v3`, `/whatworks/v1/guides`, …)

**Open risk for `hinge auth login`:** step 4 means the server likely expects a hardware-backed Apple attestation
tied to the requesting app's bundle and an Apple-issued key, not just a phone number and code. Replicating this
from a CLI (any OS, not just an iPhone) may not be possible without also forging or replaying an attestation,
which is a much larger reverse-engineering task than the rest of this inventory and was not attempted here.
Until that is resolved, `hinge auth import` (capturing a session from the real app, see `docs/auth.md`) is the
supported way to authenticate.
