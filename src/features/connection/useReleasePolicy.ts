import { useEffect } from "react";
import { create } from "zustand";

import { toast } from "@/design/primitives";
import { IpcError, ipc, type ReleasePolicy } from "@/lib/ipc";
import {
  DEFAULT_PREFS,
  readPref,
  writePref,
  type DefaultDns,
  type ReleaseMode,
} from "@/lib/prefs";

interface ReleaseState {
  mode: ReleaseMode;
  defaultDns: DefaultDns | null;
  hydrated: boolean;
  setMode: (mode: ReleaseMode) => void;
  setDefaultDns: (dns: DefaultDns) => void;
  hydrate: () => Promise<void>;
}

/**
 * The policy as the backend takes it.
 *
 * `fallback` with no server chosen yet is sent as `restore`: the switch can be
 * on for a moment before a server is picked, and in that moment the safest
 * destination is the one the user already had.
 */
function toPolicy(mode: ReleaseMode, dns: DefaultDns | null): ReleasePolicy {
  if (mode === "fallback") {
    return dns && dns.addrs.length > 0
      ? { mode: "fallback", servers: dns.addrs }
      : { mode: "restore" };
  }
  return { mode };
}

/**
 * Tell the backend, which is what actually acts on it at disconnect, at exit,
 * and in the crash journal.
 *
 * A refusal is worth saying out loud: the backend rejects a default DNS it
 * could not use (a loopback address, say), and a setting that looks on but is
 * not would only be discovered the moment the user needed it.
 */
function push(mode: ReleaseMode, dns: DefaultDns | null) {
  void ipc.setReleasePolicy(toPolicy(mode, dns)).catch((error: unknown) => {
    // Without administrator rights nothing can be released anyway, and the
    // app already says so in a banner; a second message about the default
    // DNS would only misplace the blame.
    if (error instanceof IpcError && error.detail.kind === "elevation") return;
    toast.error("Could not use that default DNS", {
      description: error instanceof Error ? error.message : String(error),
    });
  });
}

/**
 * Where the adapter goes when ForceConnect lets go of it.
 *
 * Persisted in the preferences store; the backend's copy starts at `restore`
 * on every launch and is brought in line on hydrate, before the user can
 * reach a disconnect.
 */
export const useReleasePolicy = create<ReleaseState>((set, get) => ({
  mode: DEFAULT_PREFS.releaseMode,
  defaultDns: DEFAULT_PREFS.defaultDns,
  hydrated: false,

  setMode: (mode) => {
    set({ mode });
    void writePref("releaseMode", mode);
    push(mode, get().defaultDns);
  },

  setDefaultDns: (defaultDns) => {
    set({ defaultDns });
    void writePref("defaultDns", defaultDns);
    push(get().mode, defaultDns);
  },

  hydrate: async () => {
    const [mode, defaultDns] = await Promise.all([
      readPref("releaseMode"),
      readPref("defaultDns"),
    ]);
    set({ mode, defaultDns, hydrated: true });
    push(mode, defaultDns);
  },
}));

/** Load the release policy once, at startup, and hand it to the backend. */
export function useHydrateReleasePolicy() {
  const hydrate = useReleasePolicy((state) => state.hydrate);

  useEffect(() => {
    void hydrate();
  }, [hydrate]);
}

/**
 * Whether the adapter is currently sitting on the user's default DNS.
 *
 * Read from what the adapter reports rather than remembered, so it is right
 * after a relaunch too — the default stays on the adapter after the app has
 * closed, which is the point of it.
 */
export function onDefaultDns(
  dnsV4: string[] | undefined,
  mode: ReleaseMode,
  dns: DefaultDns | null,
): boolean {
  return (
    mode === "fallback" &&
    dns !== null &&
    (dnsV4 ?? []).some((addr) => dns.addrs.includes(addr))
  );
}
