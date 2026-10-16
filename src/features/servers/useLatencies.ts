import { useMutation, useQueries, useQueryClient } from "@tanstack/react-query";

import { ipc, type LatencyResult, type Protocol, type Server } from "@/lib/ipc";
import { queryKeys } from "@/lib/query-keys";

export interface LatencyTarget {
  server: Server;
  /** `null` when the server has nothing that can be probed. */
  protocol: Protocol | null;
}

export interface LatencyReading {
  /** `undefined` until probed, `null` when the probe got no answer. */
  ms: number | null | undefined;
  result: LatencyResult | undefined;
  probing: boolean;
}

const options = (server: Server, protocol: Protocol) => ({
  queryKey: queryKeys.latency(server.id, protocol),
  queryFn: () => ipc.probeServer(server.id, protocol),
  // Same policy as `usePing`, and the same cache entries: a probe is real DNS
  // traffic and costs a second, so a fresh-enough figure is reused rather
  // than re-measured behind the user's back.
  staleTime: 60_000,
  refetchOnWindowFocus: false,
  retry: false,
});

/**
 * Latency for several servers at once, keyed by server id.
 *
 * With `probe: false` this only reads what is already cached — the Servers
 * screen shows earlier measurements without sending a query per card on every
 * visit, and measures on request through `useProbeAll`.
 */
export function useLatencies(
  targets: readonly LatencyTarget[],
  { probe }: { probe: boolean },
): Map<string, LatencyReading> {
  const results = useQueries({
    queries: targets.map(({ server, protocol }) => ({
      ...options(server, protocol ?? "udp"),
      queryKey: queryKeys.latency(server.id, protocol ?? ""),
      enabled: probe && protocol !== null,
    })),
  });

  const readings = new Map<string, LatencyReading>();
  targets.forEach(({ server }, index) => {
    const query = results[index];
    readings.set(server.id, {
      ms: query?.data ? query.data.medianMs : undefined,
      result: query?.data,
      probing: Boolean(query?.isFetching),
    });
  });
  return readings;
}

/** Measure every target again, ignoring anything cached. */
export function useProbeAll() {
  const client = useQueryClient();

  const mutation = useMutation({
    mutationFn: (targets: readonly LatencyTarget[]) =>
      Promise.allSettled(
        targets.flatMap(({ server, protocol }) =>
          protocol
            ? [
                client.fetchQuery({
                  ...options(server, protocol),
                  staleTime: 0,
                }),
              ]
            : [],
        ),
      ),
  });

  return {
    probeAll: (targets: readonly LatencyTarget[]) => mutation.mutate(targets),
    isProbing: mutation.isPending,
  };
}
