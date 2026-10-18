import { useQuery } from "@tanstack/react-query";
import { useEffect } from "react";
import { create } from "zustand";

import { useLayoutPrefs } from "@/app/useLayoutPrefs";
import { ipc, type Adapter } from "@/lib/ipc";
import { readPref, writePref } from "@/lib/prefs";
import { queryKeys } from "@/lib/query-keys";

export function useAdapters() {
  const compact = useLayoutPrefs((state) => state.compact);

  return useQuery({
    queryKey: queryKeys.adapters,
    queryFn: ipc.listAdapters,
    // Adapters change when a cable is plugged in or a VPN connects, and there
    // is no push notification wired up yet. Once NotifyIpInterfaceChange lands
    // this drops to event-driven invalidation.
    //
    // Each poll is a full system adapter query plus a DNS read per adapter,
    // so the compact window — which only shows one adapter's name — polls
    // three times less often.
    refetchInterval: compact ? 30_000 : 10_000,
  });
}

interface TargetState {
  /**
   * The adapter the user pinned, or `null` for "whichever one is active".
   *
   * Stored as a LUID rather than an `Adapter`, because the adapter list is
   * refetched every ten seconds and holding an object here would pin a
   * snapshot whose addresses and DNS servers go stale immediately.
   */
  luid: string | null;
  hydrated: boolean;
  setLuid: (luid: string | null) => void;
  hydrate: () => Promise<void>;
}

const useTargetStore = create<TargetState>((set) => ({
  luid: null,
  hydrated: false,

  setLuid: (luid) => {
    set({ luid });
    void writePref("interfaceLuid", luid);
  },

  hydrate: async () => {
    const luid = await readPref("interfaceLuid");
    set({ luid, hydrated: true });
  },
}));

/** Load the pinned interface once, at startup. */
export function useHydrateTargetAdapter() {
  const hydrate = useTargetStore((state) => state.hydrate);

  useEffect(() => {
    void hydrate();
  }, [hydrate]);
}

export interface DnsTarget {
  /** The adapter DNS will actually be written to, or `undefined` if none. */
  adapter: Adapter | undefined;
  /** The user's stored choice, whether or not it currently resolves. */
  pinnedLuid: string | null;
  /** True when the choice is being honoured rather than falling back. */
  isPinned: boolean;
  /**
   * Set when a pinned adapter is gone or no longer usable — the DNS write
   * would silently land somewhere else, so the UI has to say so.
   */
  unavailableReason: "missing" | "ineligible" | null;
  select: (luid: string | null) => void;
}

/**
 * Which adapter DNS gets applied to.
 *
 * The active adapter is the right default — it is where traffic leaves — but it
 * is only a default. A machine with a VPN tunnel, a Wi-Fi card, and a Hyper-V
 * switch has several plausible answers, and the routing table's pick is not
 * always the one the user means. So the choice is theirs, and "automatic" is
 * one of the options rather than the only behaviour.
 *
 * The fallback is deliberate and reported rather than silent: if the pinned
 * adapter has gone (a tunnel dropped, a dock was unplugged) we still have
 * somewhere sensible to write, but writing there without saying so would mean
 * the user's DNS quietly moved to a different interface than the one they
 * chose.
 */
export function useDnsTarget(): DnsTarget {
  const { data } = useAdapters();
  const pinnedLuid = useTargetStore((state) => state.luid);
  const select = useTargetStore((state) => state.setLuid);

  const active = data?.find((a) => a.isActive);
  const pinned = pinnedLuid
    ? data?.find((a) => a.luid === pinnedLuid)
    : undefined;

  if (!pinnedLuid) {
    return {
      adapter: active,
      pinnedLuid: null,
      isPinned: false,
      unavailableReason: null,
      select,
    };
  }

  // While the first adapter fetch is in flight there is nothing to check
  // against, so report neither a match nor a failure yet.
  if (!data) {
    return {
      adapter: undefined,
      pinnedLuid,
      isPinned: true,
      unavailableReason: null,
      select,
    };
  }

  if (!pinned) {
    return {
      adapter: active,
      pinnedLuid,
      isPinned: false,
      unavailableReason: "missing",
      select,
    };
  }

  if (!pinned.eligible) {
    return {
      adapter: active,
      pinnedLuid,
      isPinned: false,
      unavailableReason: "ineligible",
      select,
    };
  }

  return {
    adapter: pinned,
    pinnedLuid,
    isPinned: true,
    unavailableReason: null,
    select,
  };
}
