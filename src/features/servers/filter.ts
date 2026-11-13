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

/** Display order: encrypted first, the most widely served leading. */
const PROTOCOL_ORDER: Protocol[] = ["doh", "dot", "doq", "doh3", "udp", "tcp"];

const PROTOCOL_DESCRIPTIONS: Record<Protocol, string> = {
  doh: "DNS over HTTPS",
  dot: "DNS over TLS",
  doq: "DNS over QUIC",
  doh3: "DNS over HTTP/3",
  udp: "Plain DNS over UDP",
  tcp: "Plain DNS over TCP",
};

/** The protocol picker's second line, e.g. "DNS over HTTPS". */
export function protocolDescription(protocol: Protocol): string {
  return PROTOCOL_DESCRIPTIONS[protocol];
}

export function sortProtocols(protocols: readonly Protocol[]): Protocol[] {
  return [...protocols].sort(
    (a, b) => PROTOCOL_ORDER.indexOf(a) - PROTOCOL_ORDER.indexOf(b),
  );
}

/**
 * The protocol to preselect for a server: the first in display order, so
 * encrypted whenever the server offers it. The local resolver carries the
 * encrypted ones, so every protocol can connect.
 */
export function preferredProtocol(server: Server): Protocol | null {
  return sortProtocols(server.protocols)[0] ?? null;
}

/** `no-logging` to "No logging"; two-letter tags are country codes, so "IR". */
export function humanTag(tag: string): string {
  if (tag.length <= 2) return tag.toUpperCase();
  const text = tag.replace(/-/g, " ");
  return text.charAt(0).toUpperCase() + text.slice(1);
}

/**
 * What the latency column sorts on.
 *
 * Three states share one column: a number is a measurement, `null` is a probe
 * that got no answer, and `undefined` is a server nobody has measured. An
 * unreachable server sorts after every slow one — it is the worst result, not
 * a missing one — while unmeasured rows return `undefined` so the table keeps
 * them at the bottom in either direction rather than mixing them into the
 * ranking.
 */
export function latencySortValue(
  ms: number | null | undefined,
): number | undefined {
  if (ms === undefined) return undefined;
  return ms === null ? Number.MAX_SAFE_INTEGER : ms;
}
