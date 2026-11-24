import WebDriver from "webdriver";

export interface Config {
  server: URL;
  udid?: string;
  bundleId: string;
  teamId?: string;
}

export function configFromEnv(env = process.env): Config {
  return {
    server: new URL(env.HINGE_APPIUM_URL ?? "http://127.0.0.1:4723"),
    udid: env.HINGE_UDID,
    // shortcut: bundle id not verified against the App Store build; override with HINGE_BUNDLE_ID.
    bundleId: env.HINGE_BUNDLE_ID ?? "co.hinge.mobile.ios",
    teamId: env.HINGE_TEAM_ID,
  };
}

export type Client = Awaited<ReturnType<typeof WebDriver.newSession>>;

export async function connect(c: Config): Promise<Client> {
  return WebDriver.newSession({
    protocol: c.server.protocol.replace(":", ""),
    hostname: c.server.hostname,
    port: Number(c.server.port || (c.server.protocol === "https:" ? 443 : 80)),
    path: c.server.pathname,
    capabilities: {
      platformName: "iOS",
      "appium:automationName": "XCUITest",
      "appium:bundleId": c.bundleId,
      "appium:noReset": true,
      ...(c.udid && { "appium:udid": c.udid }),
      ...(c.teamId && { "appium:xcodeOrgId": c.teamId, "appium:xcodeSigningId": "iPhone Developer" }),
    },
  });
}

/** Every visible text label on the current screen (name/label/value of StaticText, Button, Cell). */
export async function visibleLabels(d: Client): Promise<string[]> {
  const els = await d.findElements("-ios predicate string", 'type IN {"XCUIElementTypeStaticText","XCUIElementTypeButton"} AND visible == 1');
  const out: string[] = [];
  for (const el of els) {
    const id = Object.values(el)[0] as string;
    const label = (await d.getElementAttribute(id, "label")) as string | null;
    if (label) out.push(label);
  }
  return out;
}

export async function tap(d: Client, label: string): Promise<void> {
  const el = await d.findElement("accessibility id", label);
  await d.elementClick(Object.values(el)[0] as string);
}
