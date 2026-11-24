import { mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { homedir } from "node:os";
import { dirname, join } from "node:path";
import { resolveSecret } from "./secrets.ts";

export interface Session { headers: Record<string, string> }

const KEEP = new Set(["authorization", "user_id", "user-agent", "accept-language", "x-app-identifier", "x-app-version", "x-build-number", "x-device-id", "x-device-model", "x-device-model-code", "x-device-platform", "x-device-region", "x-install-id", "x-os-version", "x-session-id"]);

export function sessionPath(env = process.env, platform = process.platform): string {
  if (env.HINGE_SESSION_FILE) return env.HINGE_SESSION_FILE;
  const base = platform === "win32" ? (env.APPDATA ?? join(homedir(), "AppData", "Roaming")) : (env.XDG_CONFIG_HOME ?? join(homedir(), ".config"));
  return join(base, "hinge-cli", "session.json");
}

/** Validate untrusted JSON and keep only allow-listed headers. Never echoes values in errors. */
export function parseSession(text: string): Session {
  let raw: unknown;
  try { raw = JSON.parse(text); } catch { throw new Error("Session is not valid JSON"); }
  const h = (raw as { headers?: Record<string, unknown> } | null)?.headers;
  if (!h || typeof h !== "object") throw new Error('Session has no "headers" object');
  const headers: Record<string, string> = {};
  for (const [k, v] of Object.entries(h)) if (KEEP.has(k.toLowerCase()) && typeof v === "string") headers[k.toLowerCase()] = v;
  if (!headers.authorization) throw new Error("Session has no authorization header");
  return { headers };
}

export async function loadSession(env = process.env): Promise<Session> {
  const ref = env.HINGE_SESSION_REF;
  try {
    return parseSession(ref ? await resolveSecret(ref) : await readFile(sessionPath(env), "utf8"));
  } catch (e) {
    if ((e as NodeJS.ErrnoException).code === "ENOENT") throw new Error("Not signed in: run `hinge auth import <file>` (see docs/auth.md)");
    throw e;
  }
}

// shortcut: mode 0600/0700 is a no-op on Windows (relies on the per-user %APPDATA% ACL); move to the OS credential store if that matters.
export async function saveSession(s: Session, path = sessionPath()): Promise<void> {
  await mkdir(dirname(path), { recursive: true, mode: 0o700 });
  await writeFile(path, JSON.stringify(s), { mode: 0o600 });
}

export const clearSession = (path = sessionPath()) => rm(path, { force: true });
