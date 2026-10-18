import {
  useConnection,
  useServerSelection,
} from "@/features/connection/useConnection";
import { preferredProtocol } from "@/features/servers/filter";
import { useFavourites, useServers } from "@/features/servers/useServers";

/**
 * What the power button would connect to right now, and why it cannot.
 *
 * Shared by the full connection screen and the compact window, so the two can
 * never disagree about which server is "selected" or whether connecting is
 * possible.
 */
export function useConnectTarget() {
  const { data: servers, isPending } = useServers();
  const { favourites } = useFavourites();
  const { selectedId, protocol } = useServerSelection();
  const { adapter } = useConnection();

  // With nothing chosen yet, start on the user's first favourite, or the top
  // of the catalogue — an empty picker on first launch is a question the app
  // can answer sensibly itself.
  const selected =
    servers?.find((s) => s.id === selectedId) ??
    servers?.find((s) => favourites.includes(s.id)) ??
    servers?.[0] ??
    null;

  const chosenProtocol =
    protocol && selected?.protocols.includes(protocol)
      ? protocol
      : selected
        ? preferredProtocol(selected)
        : null;
  const endpoint = selected?.endpoints.find(
    (e) => e.protocol === chosenProtocol,
  );

  const blocker = !selected
    ? "Pick a server to connect to."
    : !adapter
      ? "No network adapter is connected, so there is nowhere to set DNS."
      : endpoint?.requiresForwarder
        ? `${endpoint.label} needs the local resolver, which is not available yet. Pick plain DNS to connect.`
        : null;

  return {
    servers,
    isPending,
    favourites,
    selected,
    chosenProtocol,
    endpoint,
    blocker,
  };
}
