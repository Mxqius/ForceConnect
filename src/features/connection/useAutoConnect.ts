import { useQuery } from "@tanstack/react-query";
import { useEffect } from "react";

import {
  useConnection,
  useServerSelection,
} from "@/features/connection/useConnection";
import { useConnectionPrefs } from "@/features/connection/useConnectionPrefs";
import { useServers } from "@/features/servers/useServers";
import { ipc } from "@/lib/ipc";
import { queryKeys } from "@/lib/query-keys";

/** Once per launch: a disconnect afterwards is the user's, and must stick. */
let attempted = false;

/**
 * Reconnect to the last server when the app starts, if the user asked for it.
 *
 * Waits for everything a connect needs rather than trying early and failing:
 * the preferences, the server list, administrator rights, and an adapter — at
 * login the network is often not up yet, and the attempt simply happens the
 * moment it is. Mounted in `Shell`, so it runs whether the app opens to the
 * connection screen, another one, or straight to the tray.
 */
export function useAutoConnect() {
  const { autoConnect, lastConnection, hydrated } = useConnectionPrefs();
  const { data: servers } = useServers();
  const { data: info } = useQuery({
    queryKey: queryKeys.appInfo,
    queryFn: ipc.appInfo,
  });
  const { phase, adapter, connect } = useConnection();
  const { selectServer, selectProtocol } = useServerSelection();

  useEffect(() => {
    if (attempted || !hydrated) return;
    if (!autoConnect || !lastConnection) {
      attempted = true;
      return;
    }
    if (!servers || !info || !adapter || phase !== "idle") return;

    attempted = true;
    // Without rights every write refuses, and the app already says why in a
    // banner; a failed connect on top would only repeat it less clearly.
    if (!info.elevated) return;

    const server = servers.find((s) => s.id === lastConnection.serverId);
    if (!server?.protocols.includes(lastConnection.protocol)) return;

    selectServer(server.id);
    selectProtocol(lastConnection.protocol);
    void connect(server, lastConnection.protocol);
  }, [
    hydrated,
    autoConnect,
    lastConnection,
    servers,
    info,
    adapter,
    phase,
    connect,
    selectServer,
    selectProtocol,
  ]);
}
