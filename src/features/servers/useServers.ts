import { useQuery } from "@tanstack/react-query";

import { ipc } from "@/lib/ipc";
import { queryKeys } from "@/lib/query-keys";

export function useServers() {
  return useQuery({
    queryKey: queryKeys.servers,
    queryFn: ipc.listServers,
    // The built-in catalog is embedded in the binary and cannot change while
    // the app is running. Custom servers will invalidate this explicitly.
    staleTime: Infinity,
  });
}

export function useProtocols() {
  return useQuery({
    queryKey: queryKeys.protocols,
    queryFn: ipc.listProtocols,
    staleTime: Infinity,
  });
}
