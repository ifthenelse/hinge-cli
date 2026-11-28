use crate::secrets::{default_runner, resolve_secret};
use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub headers: BTreeMap<String, String>,
}

/// `user-agent` is deliberately absent: requests always send the generic Android one from `api::USER_AGENT`.
const KEEP: [&str; 14] = [
    "authorization",
    "user_id",
    "accept-language",
    "x-app-identifier",
    "x-app-version",
    "x-build-number",
    "x-device-id",
    "x-device-model",
    "x-device-model-code",
    "x-device-platform",
    "x-device-region",
    "x-install-id",
    "x-os-version",
    "x-session-id",
];

pub type Env<'a> = &'a dyn Fn(&str) -> Option<String>;
pub fn real_env(k: &str) -> Option<String> {
    std::env::var(k).ok()
}

pub fn session_path(env: Env, os: &str) -> PathBuf {
    if let Some(f) = env("HINGE_SESSION_FILE").filter(|f| !f.is_empty()) {
        return f.into();
    }
    let home = || {
        PathBuf::from(
            env("HOME")
                .or_else(|| env("USERPROFILE"))
                .unwrap_or_default(),
        )
    };
    let base = if os == "windows" {
        env("APPDATA").map_or_else(|| home().join("AppData").join("Roaming"), PathBuf::from)
    } else {
        env("XDG_CONFIG_HOME").map_or_else(|| home().join(".config"), PathBuf::from)
    };
    base.join("hinge-cli").join("session.json")
}

/// Validate untrusted JSON and keep only allow-listed headers. Never echoes values in errors.
pub fn parse_session(text: &str) -> Result<Session> {
    let Ok(raw) = serde_json::from_str::<Value>(text) else {
        bail!("Session is not valid JSON")
    };
    let Some(h) = raw["headers"].as_object() else {
        bail!("Session has no \"headers\" object")
    };
    let headers: BTreeMap<String, String> = h
        .iter()
        .filter_map(|(k, v)| Some((k.to_lowercase(), v.as_str()?.to_string())))
        .filter(|(k, _)| KEEP.contains(&k.as_str()))
        .collect();
    if !headers.contains_key("authorization") {
        bail!("Session has no authorization header");
    }
    Ok(Session { headers })
}

pub fn load_session() -> Result<Session> {
    if let Some(r) = real_env("HINGE_SESSION_REF").filter(|r| !r.is_empty()) {
        return parse_session(&resolve_secret(
            &r,
            &default_runner,
            std::env::consts::OS,
            &real_env,
        )?);
    }
    match fs::read_to_string(session_path(&real_env, std::env::consts::OS)) {
        Ok(text) => parse_session(&text),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            bail!("Not signed in: run `hinge auth import <file>` (see docs/auth.md)")
        }
        Err(e) => Err(e.into()),
    }
}

// shortcut: mode 0600/0700 is a no-op on Windows (relies on the per-user %APPDATA% ACL); move to the OS credential store if that matters.
pub fn save_session(s: &Session, path: &Path) -> Result<()> {
    if let Some(dir) = path.parent() {
        let mut b = fs::DirBuilder::new();
        b.recursive(true);
        #[cfg(unix)]
        std::os::unix::fs::DirBuilderExt::mode(&mut b, 0o700);
        b.create(dir)?;
    }
    let mut o = fs::OpenOptions::new();
    o.write(true).create(true).truncate(true);
    #[cfg(unix)]
    std::os::unix::fs::OpenOptionsExt::mode(&mut o, 0o600);
    std::io::Write::write_all(&mut o.open(path)?, serde_json::to_string(s)?.as_bytes())?;
    Ok(())
}

pub fn clear_session(path: &Path) -> Result<()> {
    match fs::remove_file(path) {
        Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(e.into()),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        move |k| {
            pairs
                .iter()
                .find(|(n, _)| *n == k)
                .map(|(_, v)| v.to_string())
        }
    }

    #[test]
    fn parsing_keeps_only_allow_listed_headers_and_requires_authorization() {
        let s = parse_session(r#"{"headers":{"Authorization":"Bearer x","cookie":"no","user-agent":"iOS","x-device-id":"d"}}"#).unwrap();
        assert_eq!(
            s.headers.keys().collect::<Vec<_>>(),
            ["authorization", "x-device-id"]
        );
        assert!(
            parse_session("{")
                .unwrap_err()
                .to_string()
                .contains("not valid JSON")
        );
        assert!(
            parse_session(r#"{"headers":{"x-device-id":"d"}}"#)
                .unwrap_err()
                .to_string()
                .contains("no authorization")
        );
        assert!(
            parse_session("{}")
                .unwrap_err()
                .to_string()
                .contains("no \"headers\"")
        );
    }

    #[test]
    fn path_per_os() {
        assert!(
            session_path(&env(&[("APPDATA", "C:\\A")]), "windows")
                .to_string_lossy()
                .contains("hinge-cli")
        );
        assert_eq!(
            session_path(&env(&[("HINGE_SESSION_FILE", "/f")]), "linux"),
            PathBuf::from("/f")
        );
        assert_eq!(
            session_path(&env(&[("XDG_CONFIG_HOME", "/x")]), "linux"),
            PathBuf::from("/x/hinge-cli/session.json")
        );
        assert_eq!(
            session_path(&env(&[("HOME", "/h")]), "macos"),
            PathBuf::from("/h/.config/hinge-cli/session.json")
        );
    }

    #[test]
    fn save_then_clear_round_trip_is_private() {
        let dir = std::env::temp_dir().join(format!("hinge-cli-test-{}", std::process::id()));
        let path = dir.join("nested").join("session.json");
        let s = parse_session(r#"{"headers":{"authorization":"t"}}"#).unwrap();
        save_session(&s, &path).unwrap();
        assert_eq!(
            parse_session(&fs::read_to_string(&path).unwrap())
                .unwrap()
                .headers["authorization"],
            "t"
        );
        #[cfg(unix)]
        assert_eq!(
            std::os::unix::fs::PermissionsExt::mode(&fs::metadata(&path).unwrap().permissions())
                & 0o777,
            0o600
        );
        clear_session(&path).unwrap();
        clear_session(&path).unwrap();
        fs::remove_dir_all(dir).ok();
    }
}
