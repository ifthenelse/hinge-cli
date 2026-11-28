mod api;
mod appium;
mod secrets;
mod session_store;

use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::Serialize;
use serde_json::{Value, json};
use session_store::{
    clear_session, load_session, parse_session, real_env, save_session, session_path,
};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "hinge",
    version,
    about = "CLI for your own Hinge account, via its reverse-engineered HTTP API"
)]
struct Cli {
    /// machine-readable output
    #[arg(long, global = true)]
    json: bool,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// check the Appium server and the connected device
    Doctor,
    /// show your subscription tier
    Tier,
    /// likes, superlikes, snoozes and undos left
    Limits,
    /// manage the saved Hinge session
    Auth {
        #[command(subcommand)]
        cmd: AuthCmd,
    },
    /// print the visible labels on the current screen (use it to learn selectors)
    Inspect,
    /// resolve a secret reference and check it works (value is hidden)
    Secret {
        /// env://, op://, bw://, keychain://, cmd://
        #[arg(value_name = "ref")]
        reference: String,
    },
}

#[derive(Subcommand)]
enum AuthCmd {
    /// import a session captured with tools/hinge_token.py
    Import {
        file: PathBuf,
        /// keep the source file
        #[arg(long)]
        keep: bool,
    },
    /// check that the saved session works
    Status,
    /// delete the saved session
    Logout,
}

fn out<T: Serialize>(json: bool, data: &T, text: String) -> Result<()> {
    println!(
        "{}",
        if json {
            serde_json::to_string(data)?
        } else {
            text
        }
    );
    Ok(())
}

fn show(v: &Value) -> String {
    v.as_str().map_or_else(|| v.to_string(), String::from)
}

async fn run(cli: Cli) -> Result<()> {
    let j = cli.json;
    match cli.cmd {
        Cmd::Doctor => {
            let c = appium::config_from_env(&real_env)?;
            let code = appium::status(&c).await?;
            let ok = (200..300).contains(&code);
            let udid = c.udid.as_deref().unwrap_or("(first available)");
            out(
                j,
                &json!({ "appium": ok, "udid": c.udid }),
                format!(
                    "Appium {} at {}; device: {udid}",
                    if ok {
                        "OK".into()
                    } else {
                        format!("HTTP {code}")
                    },
                    c.server
                ),
            )
        }
        Cmd::Tier => {
            let account = api::call(
                &api::client()?,
                api::BASE,
                "/store/v2/account",
                &load_session()?,
            )
            .await?;
            let t = api::describe_tier(&account);
            let text = match t.tier {
                None => "free".to_string(),
                Some(n) => format!(
                    "{} (tier {n}){}",
                    t.name,
                    t.end
                        .as_ref()
                        .map(|e| format!(", ends {e}"))
                        .unwrap_or_default()
                ),
            };
            out(j, &t, text)
        }
        Cmd::Limits => {
            let l = api::call(&api::client()?, api::BASE, "/likelimit", &load_session()?).await?;
            let text = l
                .as_object()
                .map(|o| {
                    o.iter()
                        .map(|(k, v)| format!("{k}: {}", show(v)))
                        .collect::<Vec<_>>()
                        .join(", ")
                })
                .unwrap_or_default();
            out(j, &l, text)
        }
        Cmd::Auth {
            cmd: AuthCmd::Import { file, keep },
        } => {
            let path = session_path(&real_env, std::env::consts::OS);
            save_session(&parse_session(&std::fs::read_to_string(&file)?)?, &path)?;
            if !keep {
                std::fs::remove_file(&file)?;
            }
            out(
                j,
                &json!({ "ok": true }),
                format!(
                    "session saved to {}{}",
                    path.display(),
                    if keep { "" } else { "; source file deleted" }
                ),
            )
        }
        Cmd::Auth {
            cmd: AuthCmd::Status,
        } => {
            api::call(
                &api::client()?,
                api::BASE,
                "/store/v2/account",
                &load_session()?,
            )
            .await?;
            out(j, &json!({ "ok": true }), "session OK".into())
        }
        Cmd::Auth {
            cmd: AuthCmd::Logout,
        } => {
            clear_session(&session_path(&real_env, std::env::consts::OS))?;
            out(j, &json!({ "ok": true }), "session deleted".into())
        }
        Cmd::Inspect => {
            let d = appium::connect(&appium::config_from_env(&real_env)?).await?;
            let labels = d.visible_labels().await;
            d.close().await.ok(); // always release the Appium session, even when reading labels failed
            let labels = labels?;
            out(j, &json!({ "labels": labels }), labels.join("\n"))
        }
        Cmd::Secret { reference } => {
            let v = secrets::resolve_secret(
                &reference,
                &secrets::default_runner,
                std::env::consts::OS,
                &real_env,
            )?;
            out(
                j,
                &json!({ "ok": true, "length": v.chars().count() }),
                format!("resolved ({} chars)", v.chars().count()),
            )
        }
    }
}

#[tokio::main]
async fn main() {
    if let Err(e) = run(Cli::parse()).await {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
