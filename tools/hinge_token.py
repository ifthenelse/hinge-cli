"""mitmproxy add-on: saves the request headers hinge-cli needs (including your auth token) to a file.

    HINGE_OUT=hinge-session.json mitmdump -s tools/hinge_token.py

Then: hinge auth import hinge-session.json   (the file is deleted after import)
Only run this against your own account and phone.
"""
import json
import os

KEEP = {
    "authorization", "user_id", "user-agent", "accept-language", "x-app-identifier", "x-app-version",
    "x-build-number", "x-device-id", "x-device-model", "x-device-model-code", "x-device-platform",
    "x-device-region", "x-install-id", "x-os-version", "x-session-id",
}
OUT = os.environ.get("HINGE_OUT", "hinge-session.json")


def request(flow):
    if flow.request.host != "prod-api.hingeaws.net" or "authorization" not in flow.request.headers:
        return
    headers = {k.lower(): v for k, v in flow.request.headers.items() if k.lower() in KEEP}
    fd = os.open(OUT, os.O_WRONLY | os.O_CREAT | os.O_TRUNC, 0o600)
    with os.fdopen(fd, "w") as f:
        json.dump({"headers": headers}, f)
