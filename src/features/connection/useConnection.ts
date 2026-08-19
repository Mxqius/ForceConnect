import { useMutation, useQueryClient } from "@tanstack/react-query";
import { create } from "zustand";

import { useDnsTarget } from "@/features/interfaces/useAdapters";
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

  /**
   * What the connection screen has picked but not yet connected.
   *
   * This lives here rather than in `HomeScreen` because the screen unmounts:
   * the router swaps the whole route element when the user visits Interfaces
   * or Settings, so component state was being thrown away and the screen came
   * back with an empty server picker and every field showing an em dash. A
   * half-filled form is not something a tab switch should destroy.
   *
   * `protocol` is `null` for "whatever this server offers first", so a server
   * change does not have to guess a protocol the new server may not support.
   */
  selectedId: string | null;
  protocol: Protocol | null;
  selectServer: (id: string | null) => void;
  selectProtocol: (protocol: Protocol | null) => void;
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

  selectedId: null,
  protocol: null,
  // Changing server clears the protocol: the previous one may not exist on the
  // new server, and a stale protocol would render an endpoint that is not there.
  selectServer: (selectedId) => set({ selectedId, protocol: null }),
  selectProtocol: (protocol) => set({ protocol }),
}));

/** The connection screen's pending selection, kept across route changes. */
export function useServerSelection() {
  return {
    selectedId: useConnectionStore((state) => state.selectedId),
    protocol: useConnectionStore((state) => state.protocol),
    selectServer: useConnectionStore((state) => state.selectServer),
    selectProtocol: useConnectionStore((state) => state.selectProtocol),
  };
}

export function useConnection() {
  const client = useQueryClient();
  const target = useDnsTarget();
  const adapter = target.adapter;
  const active = useConnectionStore((state) => state.active);
  const setActive = useConnectionStore((state) => state.setActive);
  const selectServer = useConnectionStore((state) => state.selectServer);

  const connect = useMutation({
    mutationFn: async ({
      server,
      protocol,
    }: {
      server: Server;
      protocol: Protocol;
    }) => {
      if (!adapter) {
        throw new Error("No network adapter to configure.");
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
      // Keep the picker showing what is connected. They are normally already
      // the same; they diverge when a connection is restored or made from
      // somewhere other than the picker.
      selectServer(connection.serverId);
      useConnectionStore.setState({ protocol: connection.protocol });
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
    target,
    connect: connect.mutate,
    disconnect: disconnect.mutate,
    isConnecting: connect.isPending,
    isDisconnecting: disconnect.isPending,
    activeServerId: active?.serverId ?? null,
  };
}
