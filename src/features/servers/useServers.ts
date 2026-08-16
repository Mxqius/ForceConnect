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
    },
  });
}

export function useRemoveCustomServer() {
  const client = useQueryClient();

  return useMutation({
    mutationFn: (id: string) => ipc.removeCustomServer(id),
    onSuccess: async (_result, id) => {
      // A removed server must not linger in favourites, or it would count
      // toward the favourites section forever without ever rendering.
      const current = await readPref("favourites");
      if (current.includes(id)) {
        await writePref(
          "favourites",
          current.filter((item) => item !== id),
        );
        client.setQueryData(
          queryKeys.favourites,
          current.filter((item) => item !== id),
        );
      }

      void client.invalidateQueries({ queryKey: queryKeys.customServers });
    },
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
