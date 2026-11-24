import { test } from "node:test";
import assert from "node:assert/strict";
import { resolveSecret } from "../src/secrets.ts";
import { call, describeTier } from "../src/api.ts";
import { parseSession, sessionPath } from "../src/session-store.ts";

const fake = (calls: string[][]) => async (cmd: string, args: string[]) => { calls.push([cmd, ...args]); return "s3cret\n"; };

test("env, op, bw and keychain references", async () => {
  const calls: string[][] = [];
  assert.equal(await resolveSecret("env://X", fake(calls), "linux", { X: "v" }), "v");
  assert.equal(await resolveSecret("op://v/i/f", fake(calls)), "s3cret");
  assert.equal(await resolveSecret("bw://hinge/totp", fake(calls)), "s3cret");
  await resolveSecret("keychain://svc/me", fake(calls), "darwin");
  await resolveSecret("keychain://svc/me", fake(calls), "linux");
  assert.deepEqual(calls.map((c) => c[0]), ["op", "bw", "security", "secret-tool"]);
});

test("bad references fail without leaking the reference path", async () => {
  await assert.rejects(resolveSecret("nope"), /Invalid secret reference/);
  await assert.rejects(resolveSecret("zz://a"), /Unknown secret scheme/);
  await assert.rejects(resolveSecret("env://MISSING_VAR_XYZ", undefined, "linux", {}), /not set/);
  await assert.rejects(resolveSecret("keychain://a/b", fake([]), "win32"), /not supported/);
});

test("session parsing keeps only allow-listed headers and requires authorization", () => {
  const s = parseSession(JSON.stringify({ headers: { Authorization: "Bearer x", cookie: "no", "x-device-id": "d" } }));
  assert.deepEqual(Object.keys(s.headers).sort(), ["authorization", "x-device-id"]);
  assert.throws(() => parseSession("{"), /not valid JSON/);
  assert.throws(() => parseSession('{"headers":{"x-device-id":"d"}}'), /no authorization/);
});

test("session path per OS", () => {
  assert.match(sessionPath({ APPDATA: "C:\\A" }, "win32"), /hinge-cli/);
  assert.equal(sessionPath({ HINGE_SESSION_FILE: "/f" }, "linux"), "/f");
  assert.match(sessionPath({ XDG_CONFIG_HOME: "/x" }, "linux"), /^\/x\/hinge-cli/);
});

test("tier and API errors", async () => {
  assert.deepEqual(describeTier({}), { tier: null, name: "free" });
  assert.equal(describeTier({ subscription: { tier: 2, name: "Hinge+" } }).name, "Hinge+");
  const s = { headers: { authorization: "t" } };
  const f = (status: number) => (async () => new Response("{}", { status })) as typeof fetch;
  await assert.rejects(call("/x", s, f(401)), /Session expired/);
  await assert.rejects(call("/x", s, f(500)), /HTTP 500/);
  assert.deepEqual(await call("/x", s, f(200)), {});
});
