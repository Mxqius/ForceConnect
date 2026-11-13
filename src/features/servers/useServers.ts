import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useMemo } from "react";

import { ipc, type NewServer, type Server } from "@/lib/ipc";
import { readPref, togglePref, writePref } from "@/lib/prefs";
import { queryKeys } from "@/lib/query-keys";

/** Built-in and custom servers as one list, custom last. */
export function useServers() {
  const builtin = useQuery({
    queryKey: queryKeys.servers,
    queryFn: ipc.listServers,
    // Embedded in the binary; it cannot change while the app runs.
    staleTime: Infinity,
  });

  const custom = useQuery({
    queryKey: queryKeys.customServers,
    queryFn: ipc.listCustomServers,
    staleTime: Infinity,
  });

  const data = useMemo(() => {
    if (!builtin.data) return undefined;
    return [...builtin.data, ...(custom.data ?? [])];
  }, [builtin.data, custom.data]);

  return {
    data,
    isPending: builtin.isPending,
    error: builtin.error ?? custom.error,
  };
}

export function useProtocols() {
  return useQuery({
    queryKey: queryKeys.protocols,
    queryFn: ipc.listProtocols,
    staleTime: Infinity,
  });
}

export function useFavourites() {
  const client = useQueryClient();

  const query = useQuery({
    queryKey: queryKeys.favourites,
    queryFn: () => readPref("favourites"),
    staleTime: Infinity,
  });

  const toggle = useMutation({
    mutationFn: (id: string) => togglePref("favourites", id),
    // Written straight into the cache rather than invalidated: the star should
    // fill the instant it is clicked, and a round trip to disk would make it
    // feel laggy.
    onSuccess: (next) => client.setQueryData(queryKeys.favourites, next),
  });

  return {
    favourites: query.data ?? [],
    isFavourite: (id: string) => (query.data ?? []).includes(id),
    toggle: (id: string) => toggle.mutate(id),
  };
}

export function useAddCustomServer() {
  const client = useQueryClient();

  return useMutation({
    mutationFn: (server: NewServer) => ipc.addCustomServer(server),
    onSuccess: () => {
      void client.invalidateQueries({ queryKey: queryKeys.customServers });
      // A first server on a protocol nobody else offers adds its filter.
      void client.invalidateQueries({ queryKey: queryKeys.protocols });
    },
  });
}

/** Everything that changes which servers exist, and so what the filters offer. */
function useRefreshServerList() {
  const client = useQueryClient();
  return () =>
    Promise.all([
      client.invalidateQueries({ queryKey: queryKeys.servers }),
      client.invalidateQueries({ queryKey: queryKeys.protocols }),
    ]);
}

/**
 * Remove servers from the list, whichever kind they are.
 *
 * Defaults are hidden and can be brought back; the user's own are deleted.
 * Callers that offer an undo should only do so for the first kind — the
 * returned `defaults` is the set to hand to `useReinstateServers`.
 */
export function useRemoveServers() {
  const client = useQueryClient();
  const refresh = useRefreshServerList();

  return useMutation({
    mutationFn: async (servers: readonly Server[]) => {
      const defaults = servers.filter((s) => s.source === "builtin");
      const custom = servers.filter((s) => s.source === "custom");

      if (defaults.length > 0) {
        await ipc.removeBuiltinServers(defaults.map((s) => s.id));
      }
      for (const server of custom) {
        await ipc.removeCustomServer(server.id);
      }
      return { defaults, custom };
    },
    onSettled: () => refresh(),
    onSuccess: async ({ custom }) => {
      // A deleted custom server must not linger in favourites, or it would
      // count toward the favourites section forever without ever rendering.
      // Hidden defaults keep their star, so a restored one comes back starred.
      const gone = new Set(custom.map((s) => s.id));
      if (gone.size === 0) return;

      const current = await readPref("favourites");
      const next = current.filter((id) => !gone.has(id));
      if (next.length !== current.length) {
        await writePref("favourites", next);
        client.setQueryData(queryKeys.favourites, next);
      }
    },
  });
}

/** Undo for removing default servers. */
export function useReinstateServers() {
  const refresh = useRefreshServerList();

  return useMutation({
    mutationFn: (ids: readonly string[]) =>
      ipc.reinstateBuiltinServers([...ids]),
    onSettled: () => refresh(),
  });
}

/** Get the default servers back, from GitHub where it can be reached. */
export function useRestoreDefaults() {
  const refresh = useRefreshServerList();

  return useMutation({
    mutationFn: () => ipc.restoreDefaultServers(),
    onSuccess: () => refresh(),
  });
}

/** Favourites first, then the original order. */
export function sortByFavourite(
  servers: readonly Server[],
  favourites: readonly string[],
): Server[] {
  const rank = (server: Server) => (favourites.includes(server.id) ? 0 : 1);
  return [...servers].sort((a, b) => rank(a) - rank(b));
}
