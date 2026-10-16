import { useEffect, useRef } from "react";
import { create } from "zustand";

import { usePing } from "@/features/connection/usePing";
import type { Protocol } from "@/lib/ipc";

/** How many samples the graph keeps: about two and a half minutes' worth. */
const MAX_SAMPLES = 30;
/** How often the graph takes a sample while it is on screen. */
const SAMPLE_INTERVAL_MS = 5_000;

interface HistoryState {
  samples: Record<string, number[]>;
  /** When the newest sample per key was measured, so one is never added twice. */
  latestAt: Record<string, number>;
  push: (key: string, ms: number, at: number) => void;
}

/**
 * Measured latencies per server and protocol, newest last.
 *
 * Kept in a store rather than in the tile so the line survives a visit to
 * another screen: coming back to a graph that had restarted from nothing
 * would make it look like the measurements had been invented on mount.
 */
const useHistory = create<HistoryState>((set) => ({
  samples: {},
  latestAt: {},
  push: (key, ms, at) =>
    set((state) =>
      at <= (state.latestAt[key] ?? 0)
        ? state
        : {
            samples: {
              ...state.samples,
              [key]: [...(state.samples[key] ?? []), ms].slice(-MAX_SAMPLES),
            },
            latestAt: { ...state.latestAt, [key]: at },
          },
    ),
}));

const EMPTY: number[] = [];

/**
 * Live latency for one server: the current probe, plus the samples taken so
 * far, for a graph.
 *
 * Samples only while the window is visible — a minimised window has nobody to
 * show a graph to, and every sample is four real DNS queries.
 */
export function useLatencyHistory(
  serverId: string | null,
  protocol: Protocol | null,
) {
  const ping = usePing(serverId, protocol);
  const key = serverId && protocol ? `${serverId}:${protocol}` : "";
  const samples = useHistory((state) => state.samples[key]) ?? EMPTY;
  const push = useHistory((state) => state.push);

  // Record each measurement once, keyed on when it was taken — a remount
  // sees the same cached result again and must not add it a second time.
  const { ms, updatedAt } = ping;
  useEffect(() => {
    if (key && typeof ms === "number" && updatedAt > 0)
      push(key, ms, updatedAt);
  }, [key, ms, updatedAt, push]);

  // The latest `refresh` without making the timer restart on every render.
  const refresh = useRef(ping.refresh);
  useEffect(() => {
    refresh.current = ping.refresh;
  });

  useEffect(() => {
    if (!key) return;
    const timer = setInterval(() => {
      if (document.visibilityState === "visible") refresh.current();
    }, SAMPLE_INTERVAL_MS);
    return () => clearInterval(timer);
  }, [key]);

  return { ...ping, samples };
}
