#!/usr/bin/env node
import { Command } from "commander";
import { resolveSecret } from "./secrets.ts";
import { configFromEnv, connect, tap, visibleLabels } from "./session.ts";
import { call, describeTier } from "./api.ts";
import { clearSession, loadSession, parseSession, saveSession, sessionPath } from "./session-store.ts";
import { readFile, rm } from "node:fs/promises";

const program = new Command("hinge").description("CLI for your own Hinge account, via its reverse-engineered HTTP API").option("--json", "machine-readable output");
const out = (data: unknown, text: string) => console.log(program.opts().json ? JSON.stringify(data) : text);

async function withDevice<T>(fn: (d: Awaited<ReturnType<typeof connect>>) => Promise<T>): Promise<T> {
  const d = await connect(configFromEnv());
  try { return await fn(d); } finally { await d.deleteSession(); }
}

program.command("doctor").description("check the Appium server and the connected device").action(async () => {
  const c = configFromEnv();
  const res = await fetch(new URL("status", c.server)).catch((e) => { throw new Error(`Appium not reachable at ${c.server}: ${e.cause?.code ?? e.message}`); });
  out({ appium: res.ok, udid: c.udid ?? null }, `Appium ${res.ok ? "OK" : "HTTP " + res.status} at ${c.server}; device: ${c.udid ?? "(first available)"}`);
});

program.command("tier").description("show your subscription tier").action(async () => {
  const t = describeTier(await call("/store/v2/account", await loadSession()));
  out(t, t.tier === null ? "free" : `${t.name} (tier ${t.tier})${t.end ? `, ends ${t.end}` : ""}`);
});

program.command("limits").description("likes, superlikes, snoozes and undos left").action(async () => {
  const l = await call("/likelimit", await loadSession());
  out(l, Object.entries(l).map(([k, v]) => `${k}: ${v}`).join(", "));
});

const auth = program.command("auth").description("manage the saved Hinge session");
auth.command("import").description("import a session captured with tools/hinge_token.py").argument("<file>").option("--keep", "keep the source file").action(async (file, o) => {
  await saveSession(parseSession(await readFile(file, "utf8")));
  if (!o.keep) await rm(file);
  out({ ok: true }, `session saved to ${sessionPath()}${o.keep ? "" : "; source file deleted"}`);
});
auth.command("status").description("check that the saved session works").action(async () => {
  await call("/store/v2/account", await loadSession());
  out({ ok: true }, "session OK");
});
auth.command("logout").description("delete the saved session").action(async () => { await clearSession(); out({ ok: true }, "session deleted"); });

program.command("inspect").description("print the visible labels on the current screen (use it to learn selectors)").action(async () => {
  const labels = await withDevice(visibleLabels);
  out({ labels }, labels.join("\n"));
});

program.command("secret").description("resolve a secret reference and check it works (value is hidden)").argument("<ref>", "env://, op://, bw://, keychain://, cmd://").action(async (ref) => {
  const v = await resolveSecret(ref);
  out({ ok: true, length: v.length }, `resolved (${v.length} chars)`);
});

program.parseAsync().catch((e) => { console.error(`error: ${e.message}`); process.exit(1); });
