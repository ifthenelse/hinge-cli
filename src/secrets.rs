use anyhow::{Result, bail};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

pub type Runner<'a> = &'a dyn Fn(&str, &[&str]) -> Result<String>;

/// Runs a program (no shell) and returns its stdout; killed after 30 s.
pub fn default_runner(cmd: &str, args: &[&str]) -> Result<String> {
    let mut child = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| anyhow::anyhow!("could not run `{cmd}`: {e}"))?;
    let start = Instant::now();
    // shortcut: polls instead of reading concurrently, fine for the few bytes a secret is; use a reader thread if outputs grow.
    while child.try_wait()?.is_none() {
        if start.elapsed() > Duration::from_secs(30) {
            child.kill().ok();
            bail!("`{cmd}` timed out");
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    let out = child.wait_with_output()?;
    if !out.status.success() {
        bail!("`{cmd}` failed ({})", out.status);
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// Resolve a secret reference without ever storing it. Supported schemes:
///   env://NAME                  environment variable
///   op://vault/item/field       1Password CLI (`op read`)
///   bw://item-name[/field]      Bitwarden CLI (`bw get`); field = password|username|totp
///   keychain://service/account  macOS Keychain, Linux libsecret (secret-tool)
/// KeePassXC, Dashlane, pass etc.: use `cmd://<program> <args>`, run without a shell.
pub fn resolve_secret(
    reference: &str,
    runner: Runner,
    os: &str,
    env: &dyn Fn(&str) -> Option<String>,
) -> Result<String> {
    let parsed = reference.split_once("://").filter(|(s, p)| {
        !s.is_empty() && s.bytes().all(|b| b.is_ascii_lowercase()) && !p.is_empty()
    });
    let Some((scheme, path)) = parsed else {
        bail!(
            "Invalid secret reference \"{}://…\": expected scheme://path",
            reference.split("://").next().unwrap_or("")
        );
    };
    let out = match scheme {
        "env" => {
            return env(path)
                .ok_or_else(|| anyhow::anyhow!("Environment variable {path} is not set"));
        }
        "op" => runner("op", &["read", reference])?,
        "bw" => {
            let mut parts = path.split('/');
            let item = parts.next().unwrap_or("");
            let field = parts.next().unwrap_or("password");
            if !["password", "username", "totp"].contains(&field) {
                bail!("Unsupported bw field \"{field}\"");
            }
            runner("bw", &["get", field, item])?
        }
        "keychain" => {
            let mut parts = path.split('/');
            let (Some(service), Some(account)) = (
                parts.next().filter(|s| !s.is_empty()),
                parts.next().filter(|s| !s.is_empty()),
            ) else {
                bail!("keychain:// needs service/account");
            };
            match os {
                "macos" => runner(
                    "security",
                    &["find-generic-password", "-s", service, "-a", account, "-w"],
                )?,
                "linux" => runner(
                    "secret-tool",
                    &["lookup", "service", service, "account", account],
                )?,
                _ => bail!("keychain:// is not supported on this OS; use op://, bw:// or cmd://"),
            }
        }
        "cmd" => {
            let mut words = path.split(' ').filter(|w| !w.is_empty());
            let Some(prog) = words.next() else {
                bail!("cmd:// needs a program")
            };
            runner(prog, &words.collect::<Vec<_>>())?
        }
        _ => bail!("Unknown secret scheme \"{scheme}\""),
    };
    Ok(out
        .strip_suffix('\n')
        .map(|s| s.strip_suffix('\r').unwrap_or(s))
        .unwrap_or(&out)
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    fn no_env(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn env_op_bw_and_keychain_references() {
        let calls = RefCell::new(Vec::<String>::new());
        let fake = |cmd: &str, _: &[&str]| {
            calls.borrow_mut().push(cmd.to_string());
            Ok("s3cret\n".to_string())
        };
        let env = |k: &str| (k == "X").then(|| "v".to_string());
        assert_eq!(
            resolve_secret("env://X", &fake, "linux", &env).unwrap(),
            "v"
        );
        assert_eq!(
            resolve_secret("op://v/i/f", &fake, "linux", &no_env).unwrap(),
            "s3cret"
        );
        assert_eq!(
            resolve_secret("bw://hinge/totp", &fake, "linux", &no_env).unwrap(),
            "s3cret"
        );
        resolve_secret("keychain://svc/me", &fake, "macos", &no_env).unwrap();
        resolve_secret("keychain://svc/me", &fake, "linux", &no_env).unwrap();
        assert_eq!(*calls.borrow(), ["op", "bw", "security", "secret-tool"]);
    }

    #[test]
    fn bad_references_fail_without_leaking_the_reference_path() {
        let fake = |_: &str, _: &[&str]| Ok(String::new());
        let err = |r: &str, os: &str| {
            resolve_secret(r, &fake, os, &no_env)
                .unwrap_err()
                .to_string()
        };
        assert!(err("nope", "linux").contains("Invalid secret reference"));
        assert!(err("zz://a", "linux").contains("Unknown secret scheme"));
        assert!(err("env://MISSING_VAR_XYZ", "linux").contains("not set"));
        assert!(err("keychain://a/b", "windows").contains("not supported"));
        assert!(!err("zz://secretpath", "linux").contains("secretpath"));
    }
}
