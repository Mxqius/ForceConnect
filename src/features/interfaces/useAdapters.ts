import { useQuery } from "@tanstack/react-query";

import { ipc, type Adapter } from "@/lib/ipc";
import { queryKeys } from "@/lib/query-keys";

export function useAdapters() {
  return useQuery({
    queryKey: queryKeys.adapters,
    queryFn: ipc.listAdapters,
    // Adapters change when a cable is plugged in or a VPN connects, and there
    // is no push notification wired up yet. Once NotifyIpInterfaceChange lands
    // this drops to event-driven invalidation.
    refetchInterval: 10_000,
  });
}

/** The adapter the routing table says traffic is actually leaving by. */
export function useActiveAdapter(): Adapter | undefined {
  const { data } = useAdapters();
  return data?.find((a) => a.isActive);
}
