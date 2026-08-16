import type { Protocol, Server } from "@/lib/ipc";

export interface ServerFilter {
  /** `null` means every protocol. */
  protocol: Protocol | null;
  query: string;
}

/**
 * Narrow the catalog to what the browser should show.
 *
 * Search covers the name, variant, tags, and every endpoint address — someone
 * who remembers `9.9.9.9` but not "Quad9" should still find it.
 */
export function filterServers(
  servers: readonly Server[],
  { protocol, query }: ServerFilter,
): Server[] {
  const needle = query.trim().toLowerCase();

  return servers.filter((server) => {
    if (protocol && !server.protocols.includes(protocol)) return false;
    if (!needle) return true;

    if (server.displayName.toLowerCase().includes(needle)) return true;
    if (server.tags.some((tag) => tag.toLowerCase().includes(needle)))
      return true;

    return server.endpoints.some((endpoint) =>
      endpoint.addrs.some((addr) => addr.toLowerCase().includes(needle)),
    );
  });
}

/**
 * How many servers offer each protocol, for the tab badges.
 *
 * Counting from the unfiltered catalog on purpose: a tab whose count changed as
 * you typed would make it impossible to tell "no results here" from "this
 * protocol has fewer servers".
 */
export function countByProtocol(
  servers: readonly Server[],
  protocols: readonly Protocol[],
): Map<Protocol, number> {
  const counts = new Map<Protocol, number>();
  for (const protocol of protocols) {
    counts.set(
      protocol,
      servers.filter((s) => s.protocols.includes(protocol)).length,
    );
  }
  return counts;
}

const PROTOCOL_LABELS: Record<Protocol, string> = {
  udp: "DNS",
  tcp: "DNS/TCP",
  dot: "DoT",
  doh: "DoH",
  doq: "DoQ",
  doh3: "DoH3",
};

export function protocolLabel(protocol: Protocol): string {
  return PROTOCOL_LABELS[protocol];
}

/** Display order for protocol tabs: plain first, then increasingly modern. */
const PROTOCOL_ORDER: Protocol[] = ["udp", "tcp", "dot", "doh", "doq", "doh3"];

export function sortProtocols(protocols: readonly Protocol[]): Protocol[] {
  return [...protocols].sort(
    (a, b) => PROTOCOL_ORDER.indexOf(a) - PROTOCOL_ORDER.indexOf(b),
  );
}
