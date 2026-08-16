import { Search, ServerOff, Star, X, Zap } from "lucide-react";
import { useMemo, useState } from "react";

import {
  Button,
  EmptyState,
  IconButton,
  Input,
  Skeleton,
  Tab,
  TabList,
  Tabs,
  toast,
} from "@/design/primitives";
import { AddServerDialog } from "@/features/servers/AddServerDialog";
import { ServerCard } from "@/features/servers/ServerCard";
import {
  countByProtocol,
  filterServers,
  protocolLabel,
  sortProtocols,
} from "@/features/servers/filter";
import {
  useFavourites,
  useProtocols,
  useRemoveCustomServer,
  useServers,
} from "@/features/servers/useServers";
import { useConnection } from "@/features/connection/useConnection";
import type { Protocol, Server } from "@/lib/ipc";

const ALL = "all";

export function ServersScreen() {
  const { data: servers, isPending, error } = useServers();
  const { data: protocols } = useProtocols();
  const { favourites, isFavourite, toggle } = useFavourites();
  const remove = useRemoveCustomServer();
  const { connect, isConnecting, activeServerId } = useConnection();

  const [tab, setTab] = useState<string>(ALL);
  const [query, setQuery] = useState("");

  const orderedProtocols = useMemo(
    () => sortProtocols(protocols ?? []),
    [protocols],
  );

  const counts = useMemo(
    () => countByProtocol(servers ?? [], orderedProtocols),
    [servers, orderedProtocols],
  );

  const activeProtocol: Protocol | null =
    tab === ALL ? null : (tab as Protocol);

  const visible = useMemo(
    () => filterServers(servers ?? [], { protocol: activeProtocol, query }),
    [servers, activeProtocol, query],
  );

  const starred = visible.filter((s) => favourites.includes(s.id));
  const rest = visible.filter((s) => !favourites.includes(s.id));

  function handleConnect(server: Server) {
    // Under the All tab pick the server's own first endpoint; under a protocol
    // tab honour the tab, so clicking a card does what the filter implies.
    const protocol = activeProtocol ?? server.protocols[0];
    if (!protocol) return;

    connect(
      { server, protocol },
      {
        onError: (err: unknown) =>
          toast.error("Could not connect", {
            description: err instanceof Error ? err.message : String(err),
          }),
      },
    );
  }

  const cardProps = (server: Server) => ({
    server,
    focusProtocol: activeProtocol,
    favourite: isFavourite(server.id),
    selected: server.id === activeServerId,
    onToggleFavourite: toggle,
    onRemove: (id: string) => {
      remove.mutate(id, {
        onSuccess: () => toast.success("Server removed"),
        onError: (err: unknown) =>
          toast.error("Could not remove that server", {
            description: err instanceof Error ? err.message : String(err),
          }),
      });
    },
    onSelect: isConnecting ? undefined : handleConnect,
  });

  return (
    <div className="mx-auto flex h-full max-w-3xl flex-col gap-4 p-6">
      <header className="flex items-start justify-between gap-4">
        <div>
          <h1 className="text-xl font-semibold tracking-tight">Servers</h1>
          <p className="text-text-muted mt-1 text-sm">
            {servers
              ? `${servers.length} providers. Click a card to connect.`
              : "Loading the catalogue…"}
          </p>
        </div>
        <AddServerDialog />
      </header>

      <div className="relative">
        <Search className="text-text-faint pointer-events-none absolute top-1/2 left-3 size-3.5 -translate-y-1/2" />
        <Input
          value={query}
          onChange={(event) => setQuery(event.target.value)}
          placeholder="Search by name, address, or tag…"
          aria-label="Search servers"
          className="h-10 rounded-xl pr-9 pl-9"
        />
        {query ? (
          <IconButton
            label="Clear search"
            size="sm"
            onClick={() => setQuery("")}
            className="absolute top-1/2 right-1 -translate-y-1/2"
          >
            <X />
          </IconButton>
        ) : null}
      </div>

      <Tabs value={tab} onValueChange={(value) => setTab(String(value))}>
        <TabList>
          <Tab value={ALL} badge={servers?.length ?? 0}>
            All
          </Tab>
          {orderedProtocols.map((protocol) => (
            <Tab
              key={protocol}
              value={protocol}
              badge={counts.get(protocol) ?? 0}
            >
              {protocolLabel(protocol)}
            </Tab>
          ))}
        </TabList>
      </Tabs>

      {error ? (
        <p className="border-danger/30 bg-danger-surface text-danger rounded-xl border px-3 py-2 text-sm">
          {error instanceof Error ? error.message : String(error)}
        </p>
      ) : null}

      <div className="fc-scroll -mr-2 flex min-h-0 flex-1 flex-col gap-2 overflow-y-auto pr-2">
        {isPending
          ? [0, 1, 2, 3, 4].map((i) => (
              <Skeleton key={i} className="h-24 w-full rounded-2xl" />
            ))
          : null}

        {!isPending && visible.length === 0 ? (
          <EmptyState
            icon={<ServerOff />}
            title="No servers match"
            description={
              query
                ? `Nothing matches “${query}” in this protocol.`
                : "No provider in the catalogue offers this protocol."
            }
            action={
              query ? (
                <Button size="sm" onClick={() => setQuery("")}>
                  Clear search
                </Button>
              ) : undefined
            }
          />
        ) : null}

        {starred.length > 0 ? (
          <>
            <SectionLabel icon={<Star className="size-3 fill-current" />}>
              Favourites
            </SectionLabel>
            {starred.map((server) => (
              <ServerCard key={server.id} {...cardProps(server)} />
            ))}
            {rest.length > 0 ? (
              <SectionLabel icon={<Zap className="size-3" />}>
                All servers
              </SectionLabel>
            ) : null}
          </>
        ) : null}

        {rest.map((server) => (
          <ServerCard key={server.id} {...cardProps(server)} />
        ))}
      </div>
    </div>
  );
}

function SectionLabel({
  icon,
  children,
}: {
  icon: React.ReactNode;
  children: React.ReactNode;
}) {
  return (
    <h2 className="text-text-faint text-2xs mt-2 flex items-center gap-1.5 px-1 font-semibold tracking-widest uppercase first:mt-0">
      {icon}
      {children}
    </h2>
  );
}
