import { describe, expect, it } from "vitest";

import {
  countByProtocol,
  filterServers,
  sortProtocols,
} from "@/features/servers/filter";
import type { Endpoint, Protocol, Server } from "@/lib/ipc";

function endpoint(protocol: Protocol, ...addrs: string[]): Endpoint {
  return {
    protocol,
    addrs,
    bootstrap: [],
    label: protocol.toUpperCase(),
    encrypted: protocol !== "udp" && protocol !== "tcp",
    requiresForwarder: protocol !== "udp" && protocol !== "tcp",
  };
}

function server(
  id: string,
  name: string,
  variant: string | null,
  tags: string[],
  endpoints: Endpoint[],
): Server {
  return {
    id,
    name,
    variant,
    displayName: variant ? `${name} — ${variant}` : name,
    provider: id,
    tags,
    homepage: null,
    endpoints,
    protocols: endpoints.map((e) => e.protocol),
    source: "builtin",
  };
}

const catalog: Server[] = [
  server(
    "cloudflare",
    "Cloudflare",
    null,
    ["fast", "no-logging"],
    [
      endpoint("udp", "1.1.1.1", "1.0.0.1"),
      endpoint("doh", "https://cloudflare-dns.com/dns-query"),
      endpoint("doh3", "https://cloudflare-dns.com/dns-query"),
    ],
  ),
  server(
    "quad9",
    "Quad9",
    "Secured",
    ["malware-blocking", "non-profit"],
    [endpoint("udp", "9.9.9.9"), endpoint("dot", "dns.quad9.net")],
  ),
  server(
    "mullvad",
    "Mullvad",
    "Base",
    ["privacy"],
    [endpoint("doh", "https://base.dns.mullvad.net/dns-query")],
  ),
];

describe("filterServers", () => {
  it("returns everything when unfiltered", () => {
    expect(filterServers(catalog, { protocol: null, query: "" })).toHaveLength(
      3,
    );
  });

  it("narrows to servers offering a protocol", () => {
    const doh = filterServers(catalog, { protocol: "doh", query: "" });
    expect(doh.map((s) => s.id)).toEqual(["cloudflare", "mullvad"]);
  });

  it("excludes a provider that does not offer plain DNS", () => {
    // Mullvad is DoH/DoT only. Showing it under the DNS tab and then failing
    // to connect would be worse than not listing it.
    const udp = filterServers(catalog, { protocol: "udp", query: "" });
    expect(udp.map((s) => s.id)).not.toContain("mullvad");
  });

  it("searches name, variant, tags, and addresses", () => {
    const byName = filterServers(catalog, { protocol: null, query: "quad" });
    expect(byName.map((s) => s.id)).toEqual(["quad9"]);

    const byVariant = filterServers(catalog, {
      protocol: null,
      query: "secured",
    });
    expect(byVariant.map((s) => s.id)).toEqual(["quad9"]);

    const byTag = filterServers(catalog, { protocol: null, query: "privacy" });
    expect(byTag.map((s) => s.id)).toEqual(["mullvad"]);

    // Someone who remembers the address but not the brand.
    const byAddress = filterServers(catalog, {
      protocol: null,
      query: "9.9.9",
    });
    expect(byAddress.map((s) => s.id)).toEqual(["quad9"]);

    const byUrl = filterServers(catalog, { protocol: null, query: "mullvad" });
    expect(byUrl.map((s) => s.id)).toEqual(["mullvad"]);
  });

  it("ignores case and surrounding whitespace", () => {
    expect(
      filterServers(catalog, { protocol: null, query: "  CLOUDFLARE " }),
    ).toHaveLength(1);
  });

  it("combines protocol and query", () => {
    expect(
      filterServers(catalog, { protocol: "dot", query: "cloudflare" }),
    ).toHaveLength(0);
  });
});

describe("countByProtocol", () => {
  it("counts against the full catalog, not the filtered view", () => {
    const counts = countByProtocol(catalog, ["udp", "doh", "dot", "doq"]);
    expect(counts.get("udp")).toBe(2);
    expect(counts.get("doh")).toBe(2);
    expect(counts.get("dot")).toBe(1);
    expect(counts.get("doq")).toBe(0);
  });
});

describe("sortProtocols", () => {
  it("orders plain first, then by protocol generation", () => {
    expect(sortProtocols(["doh3", "udp", "dot", "doh"])).toEqual([
      "udp",
      "dot",
      "doh",
      "doh3",
    ]);
  });
});
