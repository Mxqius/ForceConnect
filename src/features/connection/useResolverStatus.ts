import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { toast } from "@/design/primitives";
import { useConnection } from "@/features/connection/useConnection";
import { ipc } from "@/lib/ipc";
import { queryKeys } from "@/lib/query-keys";

/**
 * Upstream lookups failed in a row before the server counts as not answering.
 *
 * Above the noise of a single dropped packet or a slow handshake, which the
 * streak resets on at the next answer; low enough that a blocked server is
 * flagged within seconds of the first page that fails to load.
 */
export const FAILING_STREAK = 5;

/**
 * The local resolver's counters, polled while it is in use.
 *
 * Polled rather than pushed: an event per query would flood the webview at
 * any real query rate, and a figure that moves every couple of seconds is all
 * the screen needs.
 */
export function useResolverStatus(enabled: boolean) {
  const query = useQuery({
    queryKey: queryKeys.resolver,
    queryFn: ipc.resolverStatus,
    enabled,
    refetchInterval: enabled ? 2_000 : false,
    refetchOnWindowFocus: false,
    retry: false,
  });

  return enabled ? query.data : undefined;
}

/**
 * Whether the server behind the resolver is answering.
 *
 * Only meaningful for an encrypted connection: plain DNS goes from the OS
 * straight to the server, and there is nothing in between to count failures.
 */
export function useResolverHealth() {
  const { active, phase } = useConnection();
  const watching =
    (phase === "connected" || phase === "switching") &&
    (active?.viaResolver ?? false);
  const status = useResolverStatus(watching);
  const streak = status?.failingStreak ?? 0;

  return {
    watching,
    failing: watching && streak >= FAILING_STREAK,
    streak,
    /** The chosen server is quiet and the backup is answering in its place. */
    usingBackup: watching && (status?.usingBackup ?? false),
  };
}

/**
 * Say so — once — when the server stops answering, and again when it is back.
 *
 * Mounted in `Shell`, so it fires from any screen. A toast rather than only the
 * connection tile: the user is likely somewhere else entirely, wondering why
 * pages stopped loading, and the tile is the last place they would look.
 */
export function useResolverHealthAlerts() {
  const { active } = useConnection();
  const { watching, failing, streak } = useResolverHealth();
  const seen = useRef({ upstream: "", failing: false });

  useEffect(() => {
    // Recovery is only news for the same server: a disconnect or a switch
    // also ends a streak, and "answering again" would describe neither.
    const upstream =
      watching && active ? `${active.serverId}/${active.protocol}` : "";
    const previous = seen.current;
    seen.current = { upstream, failing };
    if (!upstream || !active) return;
    if (upstream !== previous.upstream) {
      if (!failing) return;
    } else if (failing === previous.failing) {
      return;
    }

    if (failing) {
      toast.warning(`${active.serverName} is not answering`, {
        description: `The last ${streak} lookups over ${active.protocolLabel} got no reply, so pages may not load. Try another protocol or server, or disconnect.`,
      });
    } else {
      toast.success(`${active.serverName} is answering again`);
    }
  }, [watching, failing, streak, active]);
}
