import { QueryClient } from "@tanstack/react-query";
import { beforeEach, describe, expect, it, vi } from "vitest";

import type { Adapter } from "@/lib/ipc";

const calls: string[] = [];
const ipc = vi.hoisted(() => ({
  setDns: vi.fn(),
  releaseDns: vi.fn(),
  flushDnsCache: vi.fn(),
}));

vi.mock("@/lib/ipc", () => ({ ipc }));
vi.mock("@/design/primitives", () => ({
  toast: { info: vi.fn(), error: vi.fn(), warning: vi.fn(), success: vi.fn() },
}));
// Pulls in the preferences store, which needs Tauri; `runFollow` is handed
// its adapter directly and never reads it.
vi.mock("@/features/interfaces/useAdapters", () => ({ useDnsTarget: vi.fn() }));

const { runFollow, useConnectionStore } =
  await import("@/features/connection/useConnection");

function adapter(luid: string, name: string): Adapter {
  return { luid, name } as Adapter;
}

const wifi = adapter("1", "Wi-Fi");
const ethernet = adapter("2", "Ethernet");

function connectedOn(target: Adapter) {
  useConnectionStore.setState({
    phase: "connected",
    active: {
      serverId: "cloudflare",
      serverName: "Cloudflare",
      protocol: "doh",
      protocolLabel: "DoH",
      encrypted: true,
      addrs: ["127.0.0.1", "::1"],
      viaResolver: true,
      upstream: ["https://cloudflare-dns.com/dns-query"],
      adapterLuid: target.luid,
      adapterName: target.name,
    },
  });
}

beforeEach(() => {
  vi.clearAllMocks();
  calls.length = 0;
  const record = (entry: string) => {
    calls.push(entry);
    return Promise.resolve();
  };
  ipc.setDns.mockImplementation((luid: string) => record(`set ${luid}`));
  ipc.releaseDns.mockImplementation((luid: string) =>
    record(`release ${luid}`),
  );
  ipc.flushDnsCache.mockImplementation(() => record("flush"));
});

describe("runFollow", () => {
  it("sets the new adapter before releasing the old one", async () => {
    connectedOn(wifi);
    await runFollow(new QueryClient(), ethernet);

    // New first: there must be no moment where neither adapter is set.
    expect(calls).toEqual(["set 2", "release 1", "flush"]);
    expect(ipc.setDns).toHaveBeenCalledWith("2", ["127.0.0.1", "::1"]);

    const { active } = useConnectionStore.getState();
    expect(active?.adapterLuid).toBe("2");
    expect(active?.adapterName).toBe("Ethernet");
  });

  it("does nothing when the connection is already on the adapter", async () => {
    connectedOn(ethernet);
    await runFollow(new QueryClient(), ethernet);
    expect(calls).toEqual([]);
  });

  it("does nothing while not connected", async () => {
    connectedOn(wifi);
    useConnectionStore.setState({ phase: "switching" });
    await runFollow(new QueryClient(), ethernet);
    expect(calls).toEqual([]);
  });

  it("keeps the old adapter and does not retry an adapter that refused", async () => {
    const vpn = adapter("3", "VPN");
    connectedOn(wifi);
    ipc.setDns.mockRejectedValueOnce(new Error("access denied"));

    await runFollow(new QueryClient(), vpn);
    expect(ipc.releaseDns).not.toHaveBeenCalledWith("1");
    expect(useConnectionStore.getState().active?.adapterLuid).toBe("1");

    // The next poll sees the same adapter; it must not toast again.
    ipc.setDns.mockClear();
    await runFollow(new QueryClient(), vpn);
    expect(ipc.setDns).not.toHaveBeenCalled();
  });
});
