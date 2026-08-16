import { cn } from "@/lib/cn";
import { classifyLatency, formatLatency } from "@/lib/latency";
import { Badge } from "@/design/primitives/Badge";

interface LatencyBadgeProps {
  /**
   * Round-trip time in milliseconds. The three states are distinct and read
   * differently:
   *
   * - `undefined` — not probed yet
   * - `null` — probed, no answer
   * - a number — measured
   */
  ms?: number | null;
  /** A probe is in flight. */
  pending?: boolean;
  className?: string;
}

export function LatencyBadge({ ms, pending, className }: LatencyBadgeProps) {
  if (pending) {
    return (
      <Badge tone="neutral" mono className={cn("animate-pulse", className)}>
        ···
      </Badge>
    );
  }

  if (ms === undefined) {
    return (
      <Badge tone="neutral" mono className={className}>
        —
      </Badge>
    );
  }

  if (ms === null) {
    return (
      <Badge tone="danger" className={className}>
        unreachable
      </Badge>
    );
  }

  return (
    <Badge tone={classifyLatency(ms)} mono className={className}>
      {formatLatency(ms)}
    </Badge>
  );
}
