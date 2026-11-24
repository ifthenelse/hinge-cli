import { randomUUID } from "node:crypto";
import type { Session } from "./session-store.ts";

const BASE = "https://prod-api.hingeaws.net";

export async function call(path: string, s: Session, fetchFn: typeof fetch = fetch): Promise<any> {
  const res = await fetchFn(BASE + path, { headers: { ...s.headers, accept: "application/json", "x-hinge-request-id": randomUUID() } });
  if (res.status === 401 || res.status === 403) throw new Error("Session expired or rejected: capture a new one and run `hinge auth import`");
  if (!res.ok) throw new Error(`Hinge API ${path} returned HTTP ${res.status}`);
  return res.json();
}

export interface TierInfo { tier: number | null; name: string; end?: string; autoRenew?: boolean }

/** /store/v2/account → tier. Only the free/paid split is verified; the tier integer is passed through as-is. */
export function describeTier(account: any): TierInfo {
  const sub = account?.subscription;
  if (!sub || typeof sub.tier !== "number") return { tier: null, name: "free" };
  return { tier: sub.tier, name: String(sub.name ?? `tier ${sub.tier}`), end: sub.end, autoRenew: sub.autoRenew };
}
