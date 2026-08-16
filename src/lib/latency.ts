export type LatencyTone = "success" | "warning" | "danger" | "neutral";

/**
 * Thresholds carried over from ForceConnect 1.x, which used 120ms and 180ms.
 * They held up in practice, and keeping them means users upgrading do not have
 * to relearn what a green badge means.
 */
export const LATENCY_GOOD_MS = 120;
export const LATENCY_FAIR_MS = 180;

/**
 * Classify a round-trip time.
 *
 * `null` means the probe never got an answer, which is a different thing from
 * a slow answer and is coloured as such — an unreachable server and a sluggish
 * one need different responses from the user.
 */
export function classifyLatency(ms: number | null | undefined): LatencyTone {
  if (ms === null || ms === undefined) return "neutral";
  if (!Number.isFinite(ms) || ms < 0) return "neutral";
  if (ms <= LATENCY_GOOD_MS) return "success";
  if (ms <= LATENCY_FAIR_MS) return "warning";
  return "danger";
}

/** Render a round-trip time for display. Sub-millisecond values round up to 1. */
export function formatLatency(ms: number | null | undefined): string {
  if (ms === null || ms === undefined) return "—";
  if (!Number.isFinite(ms) || ms < 0) return "—";
  if (ms < 1) return "<1 ms";
  return `${Math.round(ms)} ms`;
}
