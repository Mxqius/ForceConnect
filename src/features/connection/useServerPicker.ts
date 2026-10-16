import { toast } from "@/design/primitives";
import {
  useConnection,
  useServerSelection,
} from "@/features/connection/useConnection";
import { preferredProtocol } from "@/features/servers/filter";
import type { Protocol, Server } from "@/lib/ipc";

/**
 * Choosing a server, wherever it is chosen from.
 *
 * While disconnected a pick only changes the selection; the power button is
 * still the one place a connection starts. While connected a pick *is* the
 * switch — making the user disconnect, choose, and reconnect would open a
 * window in which their DNS falls back to the network's, which is the thing
 * they are using the app to avoid.
 */
export function useServerPicker() {
  const { phase, busy, active, connect } = useConnection();
  const { selectServer, selectProtocol } = useServerSelection();

  /** Returns whether the pick was taken. */
  function pick(server: Server, protocol?: Protocol): boolean {
    if (busy) return false;

    const chosen = protocol ?? preferredProtocol(server);

    if (phase === "connected" && chosen) {
      if (active?.serverId === server.id && active.protocol === chosen)
        return true;
      const endpoint = server.endpoints.find((e) => e.protocol === chosen);
      if (!endpoint || endpoint.requiresForwarder) {
        // Selecting it would leave the picker pointing at one server while
        // the adapter uses another, with nothing on screen saying which wins.
        toast.info(`${server.displayName} cannot be used yet`, {
          description: `${endpoint?.label ?? "That protocol"} needs the local resolver, which is not available yet. Your connection is unchanged.`,
        });
        return false;
      }

      selectServer(server.id);
      selectProtocol(chosen);
      void connect(server, chosen);
      return true;
    }

    selectServer(server.id);
    if (protocol) selectProtocol(protocol);
    return true;
  }

  return { pick };
}
