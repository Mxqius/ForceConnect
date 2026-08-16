import { useMutation, useQueryClient } from "@tanstack/react-query";

import { useActiveAdapter } from "@/features/interfaces/useAdapters";
import { ipc } from "@/lib/ipc";
import { queryKeys } from "@/lib/query-keys";

/**
 * Maintenance actions that operate on the machine's DNS state rather than on a
 * chosen server.
 *
 * They live together because they share a property: each one changes system
 * state without changing which server is in use, so none of them should touch
 * the connection store — only the adapter view they invalidate below.
 */
export function useDnsTools() {
  const client = useQueryClient();
  const adapter = useActiveAdapter();

  function refreshAdapters() {
    void client.invalidateQueries({ queryKey: queryKeys.adapters });
  }

  /** Drop every cached record, so the next lookup goes to the current server. */
  const flush = useMutation({
    mutationFn: () => ipc.flushDnsCache(),
    onSuccess: refreshAdapters,
  });

  /** Hand the adapter back to DHCP-assigned DNS. */
  const reset = useMutation({
    mutationFn: () => {
      if (!adapter) throw new Error("No active network adapter to reset.");
      return ipc.clearDns(adapter.luid);
    },
    onSuccess: refreshAdapters,
  });

  /**
   * Write the adapter's current servers back onto it.
   *
   * The case this exists for: a VPN connects, replaces DNS on its own tunnel
   * adapter, and the user's choice is quietly gone. Re-applying is faster than
   * disconnecting and reconnecting, and it does not need the catalogue.
   */
  const reapply = useMutation({
    mutationFn: (servers: string[]) => {
      if (!adapter) throw new Error("No active network adapter to configure.");
      if (servers.length === 0) throw new Error("Nothing to re-apply.");
      return ipc.setDns(adapter.luid, servers);
    },
    onSuccess: refreshAdapters,
  });

  return {
    adapter,
    flush,
    reset,
    reapply,
    isBusy: flush.isPending || reset.isPending || reapply.isPending,
  };
}
