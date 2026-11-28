# Authentication

hinge-cli has no official login. It reuses the session your own Hinge app already has, captured once from your own phone.
Hinge's terms may forbid this; automation can get an account flagged. Use your own account only.

## Capture (once, per device)
1. Install [mitmproxy](https://mitmproxy.org) and run `HINGE_OUT=hinge-session.json mitmdump -s tools/hinge_token.py`.
2. On the phone: set the Wi-Fi proxy to this computer (port 8080), open `http://mitm.it`, install the profile and enable full trust under Settings → General → About → Certificate Trust Settings. Turn any VPN off.
3. Force-quit and reopen Hinge. When `hinge-session.json` appears, stop mitmproxy.
4. `hinge auth import hinge-session.json` (saves the session and deletes the file).
5. Undo the phone changes: proxy Off, remove the mitmproxy profile.

## Storage
- Default: `~/.config/hinge-cli/session.json` (`%APPDATA%\hinge-cli\session.json` on Windows), mode 0600 where the OS supports it.
- Password manager: set `HINGE_SESSION_REF` to an `op://`, `bw://`, `keychain://` or `cmd://` reference holding the same JSON. Nothing is written to disk.
- `hinge auth logout` deletes the saved session. Tokens expire; repeat the capture when `hinge auth status` fails.

## Not done yet
Phone-number + SMS login inside the CLI (needs the login traffic captured).

## Note: user agent
`hinge auth import` keeps only an allow-list of headers and no longer stores `user-agent`: every request
the CLI sends identifies as a generic Android phone instead (`src/api.rs`). Sessions imported by an
older version still work; their stored `user-agent` is ignored.
