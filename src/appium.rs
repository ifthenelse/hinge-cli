//! Appium / WebDriverAgent client over the W3C WebDriver HTTP protocol. Used only by `hinge doctor` and
//! `hinge inspect` (exploration of the iOS app); `tier`, `limits` and `auth` never touch it.
use crate::api::client;
use anyhow::{Result, anyhow, bail};
use reqwest::{Client, Url};
use serde_json::{Value, json};

pub struct Config {
    pub server: Url,
    pub udid: Option<String>,
    pub bundle_id: String,
    pub team_id: Option<String>,
}

pub fn config_from_env(env: &dyn Fn(&str) -> Option<String>) -> Result<Config> {
    Ok(Config {
        server: Url::parse(
            &env("HINGE_APPIUM_URL").unwrap_or_else(|| "http://127.0.0.1:4723".into()),
        )?,
        udid: env("HINGE_UDID"),
        // shortcut: bundle id not verified against the App Store build; override with HINGE_BUNDLE_ID.
        bundle_id: env("HINGE_BUNDLE_ID").unwrap_or_else(|| "co.hinge.mobile.ios".into()),
        team_id: env("HINGE_TEAM_ID"),
    })
}

fn endpoint(server: &Url, path: &str) -> Result<Url> {
    let mut base = server.clone();
    if !base.path().ends_with('/') {
        base.set_path(&format!("{}/", base.path()));
    }
    Ok(base.join(path)?)
}

/// `GET /status`: is an Appium server reachable? Returns the HTTP status.
pub async fn status(c: &Config) -> Result<u16> {
    let res = client()?
        .get(endpoint(&c.server, "status")?)
        .send()
        .await
        .map_err(|e| {
            anyhow!(
                "Appium not reachable at {}: {:#}",
                c.server,
                anyhow::Error::from(e)
            )
        })?;
    Ok(res.status().as_u16())
}

pub struct Device {
    http: Client,
    server: Url,
    id: String,
}

async fn send(req: reqwest::RequestBuilder) -> Result<Value> {
    let res = req.send().await?;
    let ok = res.status().is_success();
    let status = res.status().as_u16();
    let body: Value = res.json().await.unwrap_or(Value::Null);
    if !ok {
        bail!(
            "Appium: {}",
            body["value"]["message"]
                .as_str()
                .map_or_else(|| format!("HTTP {status}"), String::from)
        );
    }
    Ok(body["value"].clone())
}

pub async fn connect(c: &Config) -> Result<Device> {
    let mut caps = json!({
        "platformName": "iOS",
        "appium:automationName": "XCUITest",
        "appium:bundleId": c.bundle_id,
        "appium:noReset": true,
    });
    if let Some(u) = &c.udid {
        caps["appium:udid"] = json!(u);
    }
    if let Some(t) = &c.team_id {
        caps["appium:xcodeOrgId"] = json!(t);
        caps["appium:xcodeSigningId"] = json!("iPhone Developer");
    }
    let http = client()?;
    let v = send(
        http.post(endpoint(&c.server, "session")?)
            .json(&json!({ "capabilities": { "alwaysMatch": caps } })),
    )
    .await?;
    let id = v["sessionId"]
        .as_str()
        .ok_or_else(|| anyhow!("Appium returned no session id"))?
        .to_string();
    Ok(Device {
        http,
        server: c.server.clone(),
        id,
    })
}

fn element_id(el: &Value) -> Result<String> {
    el.as_object()
        .and_then(|o| o.values().next())
        .and_then(Value::as_str)
        .map(String::from)
        .ok_or_else(|| anyhow!("Appium returned a malformed element"))
}

impl Device {
    fn url(&self, rest: &str) -> Result<Url> {
        endpoint(&self.server, &format!("session/{}/{rest}", self.id))
    }

    /// Every visible text label on the current screen (label of StaticText and Button elements).
    pub async fn visible_labels(&self) -> Result<Vec<String>> {
        let predicate =
            r#"type IN {"XCUIElementTypeStaticText","XCUIElementTypeButton"} AND visible == 1"#;
        let els = send(
            self.http
                .post(self.url("elements")?)
                .json(&json!({ "using": "-ios predicate string", "value": predicate })),
        )
        .await?;
        let mut out = vec![];
        for el in els.as_array().into_iter().flatten() {
            let label = send(
                self.http
                    .get(self.url(&format!("element/{}/attribute/label", element_id(el)?))?),
            )
            .await?;
            if let Some(l) = label.as_str().filter(|l| !l.is_empty()) {
                out.push(l.to_string());
            }
        }
        Ok(out)
    }

    #[allow(dead_code)] // exploration helper kept from the TypeScript version; no command uses it yet
    pub async fn tap(&self, label: &str) -> Result<()> {
        let el = send(
            self.http
                .post(self.url("element")?)
                .json(&json!({ "using": "accessibility id", "value": label })),
        )
        .await?;
        send(
            self.http
                .post(self.url(&format!("element/{}/click", element_id(&el)?))?)
                .json(&json!({})),
        )
        .await?;
        Ok(())
    }

    pub async fn close(self) -> Result<()> {
        send(
            self.http
                .delete(endpoint(&self.server, &format!("session/{}", self.id))?),
        )
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_defaults_and_overrides() {
        let c = config_from_env(&|_| None).unwrap();
        assert_eq!(
            (c.server.as_str(), c.bundle_id.as_str(), c.udid),
            ("http://127.0.0.1:4723/", "co.hinge.mobile.ios", None)
        );
        let c = config_from_env(&|k| (k == "HINGE_UDID").then(|| "U".to_string())).unwrap();
        assert_eq!(c.udid.as_deref(), Some("U"));
        assert!(
            config_from_env(&|k| (k == "HINGE_APPIUM_URL").then(|| "nope".to_string())).is_err()
        );
    }

    #[test]
    fn endpoints_keep_a_base_path() {
        let u = Url::parse("http://h:4723/wd/hub").unwrap();
        assert_eq!(
            endpoint(&u, "session").unwrap().as_str(),
            "http://h:4723/wd/hub/session"
        );
    }

    #[test]
    fn element_ids_come_from_the_w3c_element_key() {
        assert_eq!(
            element_id(&json!({"element-6066-11e4-a52e-4f735466cecf": "abc"})).unwrap(),
            "abc"
        );
        assert!(element_id(&json!(1)).is_err());
    }
}
