import { useCallback, useEffect, useRef, useState } from "react";

import { ipc, type QueryLogEntry } from "@/lib/ipc";

/** As many as the resolver keeps; older ones have already fallen off there. */
const LIMIT = 500;
const POLL_MS = 1_500;

interface Log {
  session: unknown;
  entries: QueryLogEntry[];
}

/**
 * The resolver's recent lookups, fetched incrementally while on screen.
 *
 * Asks only for what is newer than the last entry it has, so a busy log costs
 * a few rows per poll rather than the whole buffer. `session` starts it over:
 * a new resolver numbers its entries from one again, and asking it for "after
 * 900" would hide everything it has.
 */
export function useQueryLog(enabled: boolean, session: unknown) {
  const [log, setLog] = useState<Log>({ session, entries: [] });
  const cursor = useRef(0);

  useEffect(() => {
    if (!enabled) return;
    cursor.current = 0;
    let stopped = false;

    async function poll() {
      try {
        const fresh = await ipc.resolverLog(cursor.current);
        if (stopped || fresh.length === 0) return;
        cursor.current = fresh[fresh.length - 1]?.id ?? cursor.current;
        setLog((current) => ({
          session,
          entries: [
            ...(current.session === session ? current.entries : []),
            ...fresh,
          ].slice(-LIMIT),
        }));
      } catch {
        // A missed poll is filled in by the next one; nothing to tell anyone.
      }
    }

    void poll();
    const timer = window.setInterval(() => void poll(), POLL_MS);
    return () => {
      stopped = true;
      window.clearInterval(timer);
    };
  }, [enabled, session]);

  const clear = useCallback(() => {
    setLog({ session, entries: [] });
    void ipc.clearResolverLog().catch(() => undefined);
  }, [session]);

  // A log from an earlier session is not this one's, even before the first
  // poll of the new one has landed.
  const entries = enabled && log.session === session ? log.entries : [];
  return { entries, clear };
}
