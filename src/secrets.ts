import { execFile } from "node:child_process";
import { promisify } from "node:util";

const run = promisify(execFile);
export type Runner = (cmd: string, args: string[]) => Promise<string>;

const defaultRunner: Runner = async (cmd, args) => (await run(cmd, args, { timeout: 30_000 })).stdout;

/**
 * Resolve a secret reference without ever storing it. Supported schemes:
 *   env://NAME                  environment variable
 *   op://vault/item/field       1Password CLI (`op read`)
 *   bw://item-name[/field]      Bitwarden CLI (`bw get`); field = password|username|totp
 *   keychain://service/account  macOS Keychain, Linux libsecret (secret-tool)
 * KeePassXC, Dashlane, pass etc.: use `cmd://<program> <args>`, run without a shell.
 */
export async function resolveSecret(ref: string, runner: Runner = defaultRunner, platform = process.platform, env = process.env): Promise<string> {
  const m = /^([a-z]+):\/\/(.+)$/.exec(ref);
  if (!m) throw new Error(`Invalid secret reference "${ref.split("://")[0]}://…": expected scheme://path`);
  const [, scheme, path] = m;
  let out: string;
  switch (scheme) {
    case "env": {
      const v = env[path];
      if (v === undefined) throw new Error(`Environment variable ${path} is not set`);
      return v;
    }
    case "op":
      out = await runner("op", ["read", ref]);
      break;
    case "bw": {
      const [item, field = "password"] = path.split("/");
      if (!["password", "username", "totp"].includes(field)) throw new Error(`Unsupported bw field "${field}"`);
      out = await runner("bw", ["get", field, item]);
      break;
    }
    case "keychain": {
      const [service, account] = path.split("/");
      if (!service || !account) throw new Error("keychain:// needs service/account");
      if (platform === "darwin") out = await runner("security", ["find-generic-password", "-s", service, "-a", account, "-w"]);
      else if (platform === "linux") out = await runner("secret-tool", ["lookup", "service", service, "account", account]);
      else throw new Error("keychain:// is not supported on this OS; use op://, bw:// or cmd://");
      break;
    }
    case "cmd": {
      const [prog, ...args] = path.split(" ").filter(Boolean);
      out = await runner(prog, args);
      break;
    }
    default:
      throw new Error(`Unknown secret scheme "${scheme}"`);
  }
  return out.replace(/\r?\n$/, "");
}
