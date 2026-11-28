use crate::session_store::Session;
use anyhow::{Result, bail};
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

pub const BASE: &str = "https://prod-api.hingeaws.net";

/// Every HTTP request this tool makes identifies as a generic Android phone (Chrome on a Pixel-class
/// device). It replaces any `user-agent` the captured session carried.
pub const USER_AGENT: &str = "Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Mobile Safari/537.36";

pub fn client() -> Result<reqwest::Client> {
    Ok(reqwest::Client::builder().user_agent(USER_AGENT).build()?)
}

pub async fn call(http: &reqwest::Client, base: &str, path: &str, s: &Session) -> Result<Value> {
    let mut req = http.get(format!("{base}{path}"));
    for (k, v) in &s.headers {
        req = req.header(k, v);
    }
    let res = req
        .header("accept", "application/json")
        .header("x-hinge-request-id", Uuid::new_v4().to_string())
        .send()
        .await?;
    let status = res.status();
    if status.as_u16() == 401 || status.as_u16() == 403 {
        bail!("Session expired or rejected: capture a new one and run `hinge auth import`");
    }
    if !status.is_success() {
        bail!("Hinge API {path} returned HTTP {}", status.as_u16());
    }
    Ok(res.json().await?)
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TierInfo {
    pub tier: Option<i64>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
}

/// `/store/v2/account` → tier. Only the free/paid split is verified; the tier integer is passed through as-is.
pub fn describe_tier(account: &Value) -> TierInfo {
    let sub = &account["subscription"];
    let Some(tier) = sub["tier"].as_i64() else {
        return TierInfo {
            tier: None,
            name: "free".into(),
            end: None,
            auto_renew: None,
        };
    };
    TierInfo {
        tier: Some(tier),
        name: sub["name"]
            .as_str()
            .map_or_else(|| format!("tier {tier}"), String::from),
        end: sub["end"].as_str().map(String::from),
        auto_renew: sub["autoRenew"].as_bool(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::BTreeMap;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    /// One-shot local HTTP server: answers with `status`, returns the raw request it received.
    async fn serve(status: u16) -> (String, tokio::task::JoinHandle<String>) {
        let l = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base = format!("http://{}", l.local_addr().unwrap());
        let h = tokio::spawn(async move {
            let (mut sock, _) = l.accept().await.unwrap();
            let mut buf = vec![0u8; 8192];
            let n = sock.read(&mut buf).await.unwrap();
            let reply = format!(
                "HTTP/1.1 {status} X\r\ncontent-type: application/json\r\ncontent-length: 2\r\nconnection: close\r\n\r\n{{}}"
            );
            sock.write_all(reply.as_bytes()).await.unwrap();
            String::from_utf8_lossy(&buf[..n]).to_lowercase()
        });
        (base, h)
    }

    fn session() -> Session {
        Session {
            headers: BTreeMap::from([
                ("authorization".into(), "t".into()),
                ("x-device-id".into(), "d".into()),
            ]),
        }
    }

    #[test]
    fn tier_is_free_unless_a_tier_number_is_present() {
        assert_eq!(
            describe_tier(&json!({})),
            TierInfo {
                tier: None,
                name: "free".into(),
                end: None,
                auto_renew: None
            }
        );
        assert_eq!(
            describe_tier(&json!({"subscription": {"tier": 2, "name": "Hinge+"}})).name,
            "Hinge+"
        );
        assert_eq!(
            describe_tier(&json!({"subscription": {"tier": 3}})).name,
            "tier 3"
        );
    }

    #[tokio::test]
    async fn api_errors_and_android_user_agent() {
        let http = client().unwrap();
        for (status, want) in [
            (401, "Session expired"),
            (403, "Session expired"),
            (500, "HTTP 500"),
        ] {
            let (base, _) = serve(status).await;
            let e = call(&http, &base, "/x", &session())
                .await
                .unwrap_err()
                .to_string();
            assert!(e.contains(want), "{e}");
        }
        let (base, req) = serve(200).await;
        assert_eq!(
            call(&http, &base, "/x", &session()).await.unwrap(),
            json!({})
        );
        let req = req.await.unwrap();
        assert!(
            req.contains("user-agent: mozilla/5.0 (linux; android"),
            "{req}"
        );
        assert!(
            !req.contains("iphone") && !req.contains("cfnetwork"),
            "{req}"
        );
        assert!(
            req.contains("authorization: t") && req.contains("x-hinge-request-id: "),
            "{req}"
        );
    }
}
