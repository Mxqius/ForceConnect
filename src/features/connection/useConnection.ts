import { useMutation, useQueryClient } from "@tanstack/react-query";
import { create } from "zustand";

import { useActiveAdapter } from "@/features/interfaces/useAdapters";
import { ipc, type Protocol, type Server } from "@/lib/ipc";
import { queryKeys } from "@/lib/query-keys";

interface ActiveConnection {
  serverId: string;
  serverName: string;
  protocol: Protocol;
  protocolLabel: string;
  adapterLuid: string;
  adapterName: string;
}

interface ConnectionState {
  active: ActiveConnection | null;
  setActive: (connection: ActiveConnection | null) => void;
}

/**
 * The live connection, held in UI state rather than derived from the adapter.
 *
 * Reading it back from the adapter would only tell us *that* DNS is static,
 * not which catalogue entry produced it — and the UI needs the name, protocol,
 * and provider to render anything useful.
 */
const useConnectionStore = create<ConnectionState>((set) => ({
  active: null,
  setActive: (active) => set({ active }),
}));

export function useConnection() {
  const client = useQueryClient();
  const adapter = useActiveAdapter();
  const active = useConnectionStore((state) => state.active);
  const setActive = useConnectionStore((state) => state.setActive);

  const connect = useMutation({
    mutationFn: async ({
      server,
      protocol,
    }: {
      server: Server;
      protocol: Protocol;
    }) => {
      if (!adapter) {
        throw new Error("No active network adapter to configure.");
      }

      const endpoint = server.endpoints.find((e) => e.protocol === protocol);
      if (!endpoint) {
        throw new Error(`${server.displayName} does not offer ${protocol}.`);
      }

      if (endpoint.requiresForwarder) {
        // The local resolver is not started yet, so an encrypted pick cannot
        // be honoured. Saying so plainly beats setting plain DNS behind the
        // user's back and letting them believe traffic is encrypted.
        throw new Error(
          `${endpoint.label} needs the local resolver, which is not running yet. Pick a plain DNS endpoint for now.`,
        );
      }

      await ipc.setDns(adapter.luid, endpoint.addrs);

      return {
        serverId: server.id,
        serverName: server.displayName,
        protocol,
        protocolLabel: endpoint.label,
        adapterLuid: adapter.luid,
        adapterName: adapter.name,
      } satisfies ActiveConnection;
    },
    onSuccess: (connection) => {
      setActive(connection);
      void client.invalidateQueries({ queryKey: queryKeys.adapters });
      void ipc.setTrayStatus({
        connected: true,
        serverName: connection.serverName,
        protocolLabel: connection.protocolLabel,
      });
    },
  });

  const disconnect = useMutation({
    mutationFn: async () => {
      const luid = active?.adapterLuid ?? adapter?.luid;
      if (!luid) throw new Error("Nothing to disconnect.");
      await ipc.clearDns(luid);
    },
    onSuccess: () => {
      setActive(null);
      void client.invalidateQueries({ queryKey: queryKeys.adapters });
      void ipc.setTrayStatus({
        connected: false,
        serverName: null,
        protocolLabel: null,
      });
    },
  });

  return {
    active,
    adapter,
    connect: connect.mutate,
    disconnect: disconnect.mutate,
    isConnecting: connect.isPending,
    isDisconnecting: disconnect.isPending,
    activeServerId: active?.serverId ?? null,
  };
}
