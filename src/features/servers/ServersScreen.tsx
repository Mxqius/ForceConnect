import { Search, ServerOff, X } from "lucide-react";
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
} from "@/design/primitives";
import { ServerCard } from "@/features/servers/ServerCard";
import {
  countByProtocol,
  filterServers,
  protocolLabel,
  sortProtocols,
} from "@/features/servers/filter";
import { useProtocols, useServers } from "@/features/servers/useServers";
import type { Protocol } from "@/lib/ipc";

const ALL = "all";

export function ServersScreen() {
  const { data: servers, isPending, error } = useServers();
  const { data: protocols } = useProtocols();

  const [tab, setTab] = useState<string>(ALL);
  const [query, setQuery] = useState("");
  const [selectedId, setSelectedId] = useState<string | null>(null);

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

  return (
    <div className="mx-auto flex h-full max-w-3xl flex-col gap-4 p-6">
      <header>
        <h1 className="text-xl font-semibold tracking-tight">Servers</h1>
        <p className="text-text-muted mt-1 text-sm">
          {servers
            ? `${servers.length} providers. Encrypted protocols route through a local resolver.`
            : "Loading the catalog…"}
        </p>
      </header>

      <div className="relative">
        <Search className="text-text-faint pointer-events-none absolute top-1/2 left-2.5 size-3.5 -translate-y-1/2" />
        <Input
          value={query}
          onChange={(event) => setQuery(event.target.value)}
          placeholder="Search by name, address, or tag…"
          aria-label="Search servers"
          className="pr-8 pl-8"
        />
        {query ? (
          <IconButton
            label="Clear search"
            size="sm"
            onClick={() => setQuery("")}
            className="absolute top-1/2 right-0.5 -translate-y-1/2"
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
        <p className="border-danger/30 bg-danger-surface text-danger rounded-md border px-3 py-2 text-sm">
          {error instanceof Error ? error.message : String(error)}
        </p>
      ) : null}

      <div className="fc-scroll -mr-2 flex min-h-0 flex-1 flex-col gap-2 overflow-y-auto pr-2">
        {isPending
          ? [0, 1, 2, 3, 4].map((i) => (
              <Skeleton key={i} className="h-[86px] w-full" />
            ))
          : null}

        {!isPending && visible.length === 0 ? (
          <EmptyState
            icon={<ServerOff />}
            title="No servers match"
            description={
              query
                ? `Nothing matches “${query}” in this protocol.`
                : "No provider in the catalog offers this protocol."
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

        {visible.map((server) => (
          <ServerCard
            key={server.id}
            server={server}
            focusProtocol={activeProtocol}
            selected={server.id === selectedId}
            onSelect={() => setSelectedId(server.id)}
          />
        ))}
      </div>
    </div>
  );
}
