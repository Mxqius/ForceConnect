import { useQuery } from "@tanstack/react-query";

import { ipc, type Protocol } from "@/lib/ipc";
import { queryKeys } from "@/lib/query-keys";

/**
 * Round-trip time for one server over one protocol.
 *
 * A probe is four real DNS exchanges over the server's real transport, not an
 * ICMP ping — so it costs a second or two and is worth caching. It is never
 * refetched on a timer or on window focus: a number that silently changed while
 * the user was reading it would be worse than a slightly old one, and the
 * refresh control is right next to it.
 */
export function usePing(serverId: string | null, protocol: Protocol | null) {
  const enabled = Boolean(serverId && protocol);

  const query = useQuery({
    queryKey: queryKeys.latency(serverId ?? "", protocol ?? ""),
    queryFn: () => ipc.probeServer(serverId as string, protocol as Protocol),
    enabled,
    staleTime: 60_000,
    refetchOnWindowFocus: false,
    // A failed probe is a result, not a transient error to retry behind the
    // user's back — three silent retries would just make it take nine seconds
    // to say "unreachable".
    retry: false,
  });

  return {
    /** `undefined` until probed, `null` when the probe got no answer. */
    ms: query.data ? query.data.medianMs : undefined,
    jitterMs: query.data?.jitterMs ?? null,
    handshakeMs: query.data?.handshakeMs ?? null,
    verdict: query.data?.verdict ?? null,
    error: query.error,
    /** True while a probe is in flight, including a manual refresh. */
    isProbing: enabled && query.isFetching,
    refresh: () => void query.refetch(),
  };
}
