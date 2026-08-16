import type {
  Adapter,
  AppInfo,
  LatencyResult,
  Protocol,
  Server,
} from "@/lib/bindings";

/**
 * Sample data used only when the UI runs outside Tauri.
 *
 * Opening the Vite dev server in a plain browser gives you fast refresh and
 * devtools without rebuilding Rust, which makes iterating on layout and states
 * far quicker. Nothing here reaches a production build — see `ipc.ts`.
 *
 * These are typed as the real command return types on purpose: if the Rust
 * contract changes, these stop compiling instead of quietly drifting into a
 * fiction that no longer matches the app.
 *
 * The adapters below are a real (anonymised) machine, including the two cases
 * that broke earlier assumptions: a VPN tunnel with no gateway that outranks
 * everything, and a Hyper-V switch that looks routable but goes nowhere.
 */

export const appInfo: AppInfo = {
  version: "2.0.0",
  os: "windows x86_64",
  elevated: true,
  pid: 4242,
};

export const adapters: Adapter[] = [
  {
    luid: "1689399632855040",
    guid: "{EA70F586-E6E4-47FA-B6AA-D34FCDCDFFC4}",
    index: 89,
    name: "ExpressVPN WinTUN Adapter",
    description: "ExpressVPN Tunnel",
    kind: "tunnel",
    status: "up",
    mac: null,
    linkSpeedMbps: 100000,
    addresses: ["100.64.100.6", "fe80::5f4a:2a75:1c9:dfe7"],
    gateways: [],
    dnsV4: ["100.64.100.1"],
    dnsV6: [],
    metricV4: 3,
    metricV6: 3,
    eligible: true,
    isActive: true,
    dnsSource: "static",
  },
  {
    luid: "1689399632855041",
    guid: "{74DF9DA9-98B8-4D3C-A210-EA277ECB4637}",
    index: 15,
    name: "Wi-Fi",
    description: "RZ608 Wi-Fi 6E 80MHz",
    kind: "wifi",
    status: "up",
    mac: "10:6F:D9:DD:03:B9",
    linkSpeedMbps: 72,
    addresses: ["192.168.16.241", "fe80::b543:5d0b:f125:18d3"],
    gateways: ["192.168.16.70"],
    dnsV4: ["192.168.16.70"],
    dnsV6: [],
    metricV4: 55,
    metricV6: 55,
    eligible: true,
    isActive: false,
    dnsSource: "dhcp",
  },
  {
    luid: "1689399632855042",
    guid: "{478F0958-0B53-4022-99C7-C92FD733AFBB}",
    index: 27,
    name: "vEthernet (Default Switch)",
    description: "Hyper-V Virtual Ethernet Adapter",
    kind: "ethernet",
    status: "up",
    mac: "00:15:5D:33:1B:8D",
    linkSpeedMbps: 10000,
    addresses: ["172.23.64.1"],
    gateways: [],
    dnsV4: [],
    dnsV6: [],
    metricV4: 15,
    metricV6: 15,
    eligible: false,
    isActive: false,
    dnsSource: "dhcp",
  },
  {
    luid: "1689399632855043",
    guid: "{7B47C4C1-1B98-464E-B897-23CA86641960}",
    index: 17,
    name: "Ethernet 2",
    description: "TAP-Windows Adapter V9",
    kind: "tunnel",
    status: "down",
    mac: "00:FF:7B:47:C4:C1",
    linkSpeedMbps: 100,
    addresses: ["169.254.11.136"],
    gateways: [],
    dnsV4: [],
    dnsV6: [],
    metricV4: 35,
    metricV6: 35,
    eligible: false,
    isActive: false,
    dnsSource: "unknown",
  },
];

function endpoint(
  protocol: Protocol,
  label: string,
  addrs: string[],
  bootstrap: string[] = [],
) {
  const encrypted = protocol !== "udp" && protocol !== "tcp";
  return {
    protocol,
    addrs,
    bootstrap,
    label,
    encrypted,
    requiresForwarder: encrypted,
  };
}

export const servers: Server[] = [
  {
    id: "cloudflare",
    name: "Cloudflare",
    variant: null,
    displayName: "Cloudflare",
    provider: "cloudflare",
    tags: ["fast", "no-logging", "global"],
    homepage: "https://1.1.1.1",
    endpoints: [
      endpoint("udp", "DNS", ["1.1.1.1", "1.0.0.1"]),
      endpoint("dot", "DoT", ["one.one.one.one"], ["1.1.1.1"]),
      endpoint(
        "doh",
        "DoH",
        ["https://cloudflare-dns.com/dns-query"],
        ["1.1.1.1"],
      ),
      endpoint(
        "doh3",
        "DoH3",
        ["https://cloudflare-dns.com/dns-query"],
        ["1.1.1.1"],
      ),
    ],
    protocols: ["udp", "dot", "doh", "doh3"],
    source: "builtin",
  },
  {
    id: "quad9",
    name: "Quad9",
    variant: "Secured",
    displayName: "Quad9 — Secured",
    provider: "quad9",
    tags: ["malware-blocking", "no-logging", "dnssec", "non-profit"],
    homepage: "https://quad9.net",
    endpoints: [
      endpoint("udp", "DNS", ["9.9.9.9", "149.112.112.112"]),
      endpoint("dot", "DoT", ["dns.quad9.net"], ["9.9.9.9"]),
      endpoint("doh", "DoH", ["https://dns.quad9.net/dns-query"], ["9.9.9.9"]),
    ],
    protocols: ["udp", "dot", "doh"],
    source: "builtin",
  },
  {
    id: "adguard",
    name: "AdGuard DNS",
    variant: "Default",
    displayName: "AdGuard DNS — Default",
    provider: "adguard",
    tags: ["ad-blocking", "tracker-blocking"],
    homepage: "https://adguard-dns.io",
    endpoints: [
      endpoint("udp", "DNS", ["94.140.14.14", "94.140.15.15"]),
      endpoint("dot", "DoT", ["dns.adguard-dns.com"], ["94.140.14.14"]),
      endpoint(
        "doh",
        "DoH",
        ["https://dns.adguard-dns.com/dns-query"],
        ["94.140.14.14"],
      ),
      endpoint("doq", "DoQ", ["dns.adguard-dns.com"], ["94.140.14.14"]),
      endpoint(
        "doh3",
        "DoH3",
        ["https://dns.adguard-dns.com/dns-query"],
        ["94.140.14.14"],
      ),
    ],
    protocols: ["udp", "dot", "doh", "doq", "doh3"],
    source: "builtin",
  },
  {
    id: "mullvad",
    name: "Mullvad",
    variant: "Base",
    displayName: "Mullvad — Base",
    provider: "mullvad",
    tags: ["no-logging", "privacy", "ad-blocking"],
    homepage: "https://mullvad.net",
    endpoints: [
      endpoint("dot", "DoT", ["base.dns.mullvad.net"], ["194.242.2.2"]),
      endpoint(
        "doh",
        "DoH",
        ["https://base.dns.mullvad.net/dns-query"],
        ["194.242.2.2"],
      ),
    ],
    protocols: ["dot", "doh"],
    source: "builtin",
  },
  {
    id: "shecan",
    name: "Shecan",
    variant: null,
    displayName: "Shecan",
    provider: "shecan",
    tags: ["ir", "unblocking"],
    homepage: "https://shecan.ir",
    endpoints: [endpoint("udp", "DNS", ["178.22.122.100", "185.51.200.2"])],
    protocols: ["udp"],
    source: "builtin",
  },
];

export const protocols: Protocol[] = ["udp", "dot", "doh", "doq", "doh3"];

/**
 * A plausible probe result, derived from the server id so a given server
 * always reads the same in the browser — a latency that jumped every render
 * would make the refresh control impossible to evaluate by eye.
 */
export function latency(serverId: string, protocol: Protocol): LatencyResult {
  const hash = [...serverId].reduce((acc, c) => acc * 31 + c.charCodeAt(0), 7);
  const median = 12 + (hash % 190);

  return {
    serverId,
    protocol,
    verdict: "good",
    medianMs: median,
    handshakeMs: protocol === "udp" ? null : median * 1.8,
    jitterMs: (hash % 7) + 1,
    answered: 4,
    rounds: 4,
  };
}
