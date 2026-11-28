//! Guided session capture: wraps the manual mitmproxy steps in `docs/auth.md` into one command, so a
//! member without a technical background can run `hinge auth capture` instead of mitmproxy by hand.
use crate::session_store::{Session, parse_session, real_env, save_session, session_path};
use anyhow::{Context, Result, bail};
use std::io::Write;
use std::net::IpAddr;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

const SCRIPT: &str = include_str!("../tools/hinge_token.py");
const PROXY_PORT: u16 = 8080;
const TIMEOUT: Duration = Duration::from_secs(300);

fn local_ip() -> Option<IpAddr> {
    // shortcut: UDP "connect" picks the outbound interface without sending a packet; good enough to
    // print a LAN address for the phone to type in, falls back to "your computer's IP" if it fails.
    let sock = std::net::UdpSocket::bind("0.0.0.0:0").ok()?;
    sock.connect("8.8.8.8:80").ok()?;
    sock.local_addr().ok().map(|a| a.ip())
}

struct Proxy(Child);
impl Drop for Proxy {
    fn drop(&mut self) {
        self.0.kill().ok();
        self.0.wait().ok();
    }
}

pub fn capture(print: &dyn Fn(&str)) -> Result<(Session, PathBuf)> {
    if Command::new("mitmdump")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_err()
    {
        bail!(
            "mitmproxy is not installed. Install it from https://mitmproxy.org, then run `hinge auth capture` again."
        );
    }

    let script = std::env::temp_dir().join(format!("hinge_token_{}.py", std::process::id()));
    std::fs::write(&script, SCRIPT)?;
    let out = std::env::temp_dir().join(format!("hinge_session_{}.json", std::process::id()));
    std::fs::remove_file(&out).ok();

    let child = Command::new("mitmdump")
        .args([
            "-s",
            script.to_str().unwrap(),
            "-p",
            &PROXY_PORT.to_string(),
            "-q",
        ])
        .env("HINGE_OUT", &out)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .context("could not start mitmdump")?;
    let _proxy = Proxy(child); // killed on every return path, including early bail!s below

    let ip = local_ip().map_or_else(
        || "<your computer's IP address>".to_string(),
        |ip| ip.to_string(),
    );
    print(&format!(
        "Capturing your Hinge session. On your phone, with the SAME Wi-Fi network as this computer and any VPN off:\n\
         \n\
         1. Set the Wi-Fi proxy to {ip} : {PROXY_PORT}\n\
         2. Open http://mitm.it in a browser on the phone, install the certificate, then enable full trust for it\n\
            (iPhone: Settings > General > About > Certificate Trust Settings)\n\
         3. Force-quit the Hinge app and reopen it\n\
         \n\
         Waiting up to 5 minutes for your session... (Ctrl+C to cancel)"
    ));

    let start = Instant::now();
    while !out.exists() {
        if start.elapsed() > TIMEOUT {
            bail!(
                "No session was captured within 5 minutes. Check the phone's proxy and certificate trust, then try again."
            );
        }
        std::thread::sleep(Duration::from_millis(500));
    }
    std::thread::sleep(Duration::from_millis(300)); // the file is written, then closed; avoid a half-written read
    let text = std::fs::read_to_string(&out)?;
    std::fs::remove_file(&out).ok();
    std::fs::remove_file(&script).ok();

    print(
        "Got it. Turn the phone's Wi-Fi proxy back off and remove the mitmproxy certificate when you're done.",
    );
    Ok((
        parse_session(&text)?,
        session_path(&real_env, std::env::consts::OS),
    ))
}

pub fn run() -> Result<PathBuf> {
    let stdout_print = |s: &str| {
        println!("{s}");
        std::io::stdout().flush().ok();
    };
    let (session, path) = capture(&stdout_print)?;
    save_session(&session, &path)?;
    Ok(path)
}
