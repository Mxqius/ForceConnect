import { RefreshCw, Search, ServerOff, Star, X } from "lucide-react";
import { useMemo, useState, type ReactNode } from "react";
import { useNavigate } from "react-router";

import {
  Button,
  EmptyState,
  IconButton,
  PageHeader,
  pagePadding,
  Skeleton,
  toast,
} from "@/design/primitives";
import { AddServerDialog } from "@/features/servers/AddServerDialog";
import { ServerCard } from "@/features/servers/ServerCard";
import {
  countByProtocol,
  filterServers,
  preferredProtocol,
  protocolLabel,
  sortProtocols,
} from "@/features/servers/filter";
import { useLatencies, useProbeAll } from "@/features/servers/useLatencies";
import {
  useFavourites,
  useProtocols,
  useRemoveCustomServer,
  useServers,
} from "@/features/servers/useServers";
import {
  useConnection,
  useServerSelection,
} from "@/features/connection/useConnection";
import { useServerPicker } from "@/features/connection/useServerPicker";
import { cn } from "@/lib/cn";
import type { Protocol, Server } from "@/lib/ipc";

type Filter = "all" | "favourites" | Protocol;

export function ServersScreen() {
  const { data: servers, isPending, error } = useServers();
  const { data: protocols } = useProtocols();
  const { favourites, isFavourite, toggle } = useFavourites();
  const remove = useRemoveCustomServer();
  const { phase, activeServerId } = useConnection();
  const { selectedId } = useServerSelection();
  const { pick } = useServerPicker();
  const navigate = useNavigate();

  const [filter, setFilter] = useState<Filter>("all");
  const [query, setQuery] = useState("");

  const orderedProtocols = useMemo(
    () => sortProtocols(protocols ?? []),
    [protocols],
  );

  const counts = useMemo(
    () => countByProtocol(servers ?? [], orderedProtocols),
    [servers, orderedProtocols],
  );

  const focusProtocol: Protocol | null =
    filter === "all" || filter === "favourites" ? null : filter;

  // Favourites first, then catalogue order. Not sorted by latency: cards that
  // jumped around as each probe landed would move the one under the cursor.
  const visible = useMemo(() => {
    const matched = filterServers(servers ?? [], {
      protocol: focusProtocol,
      query,
    }).filter((s) => filter !== "favourites" || favourites.includes(s.id));
    const rank = (s: Server) => (favourites.includes(s.id) ? 0 : 1);
    return matched.sort((a, b) => rank(a) - rank(b));
  }, [servers, focusProtocol, query, filter, favourites]);

  const targets = useMemo(
    () =>
      visible.map((server) => ({
        server,
        protocol: focusProtocol ?? preferredProtocol(server),
      })),
    [visible, focusProtocol],
  );

  // Reads what is already measured; measuring is on request, below.
  const latencies = useLatencies(targets, { probe: false });
  const { probeAll, isProbing } = useProbeAll();

  const live = phase === "connected" || phase === "switching";

  function handleSelect(server: Server) {
    // Under a protocol filter honour it, so clicking a card does what the
    // filter implies; otherwise the server's own preferred protocol.
    if (pick(server, focusProtocol ?? undefined)) void navigate("/");
  }

  return (
    <div className={cn("flex min-h-full flex-col", pagePadding)}>
      <PageHeader
        title="Servers"
        description={
          servers
            ? `${servers.length} servers. Pick one to use it on the Connection screen.`
            : "Loading the catalogue…"
        }
        actions={
          <>
            <Button
              size="sm"
              className="rounded-full"
              startIcon={
                <RefreshCw className={cn(isProbing && "animate-spin")} />
              }
              disabled={isProbing || targets.length === 0}
              onClick={() => probeAll(targets)}
            >
              {isProbing ? "Testing…" : "Test all"}
            </Button>
            <AddServerDialog />
          </>
        }
      />

      <div className="mb-5 flex items-center gap-3">
        <label className="bg-surface border-border-subtle text-text-muted focus-within:border-accent flex h-10 w-64 shrink-0 items-center gap-2.5 rounded-full border px-4 transition-colors">
          <Search className="size-4 shrink-0" />
          <input
            value={query}
            onChange={(event) => setQuery(event.target.value)}
            placeholder="Name, address or tag"
            aria-label="Search servers"
            className="text-text-primary placeholder:text-text-muted min-w-0 flex-1 bg-transparent text-sm outline-none"
          />
          {query ? (
            <IconButton
              label="Clear search"
              size="sm"
              onClick={() => setQuery("")}
              className="-mr-2 size-7 rounded-full [&_svg]:size-3.5"
            >
              <X />
            </IconButton>
          ) : null}
        </label>

        <div
          role="radiogroup"
          aria-label="Filter servers"
          className="flex min-w-0 flex-wrap gap-2"
        >
          <FilterChip
            active={filter === "all"}
            onClick={() => setFilter("all")}
            count={servers?.length}
          >
            All
          </FilterChip>
          <FilterChip
            active={filter === "favourites"}
            onClick={() => setFilter("favourites")}
            count={favourites.length}
          >
            <Star className="size-3 fill-current" />
            Favourites
          </FilterChip>
          {orderedProtocols.map((protocol) => (
            <FilterChip
              key={protocol}
              active={filter === protocol}
              onClick={() => setFilter(protocol)}
              count={counts.get(protocol)}
            >
              {protocolLabel(protocol)}
            </FilterChip>
          ))}
        </div>
      </div>

      {error ? (
        <p className="border-danger/30 bg-danger-surface text-danger mb-5 rounded-2xl border px-4 py-3 text-sm">
          {error instanceof Error ? error.message : String(error)}
        </p>
      ) : null}

      {!isPending && visible.length === 0 ? (
        <EmptyState
          icon={<ServerOff />}
          title="No servers match"
          description={
            query
              ? `Nothing matches “${query}” here.`
              : filter === "favourites"
                ? "Star a server and it will show up here, and first on the Connection screen."
                : "No provider in the catalogue offers this protocol."
          }
          action={
            query ? (
              <Button onClick={() => setQuery("")}>Clear search</Button>
            ) : undefined
          }
        />
      ) : null}

      <div className="grid grid-cols-3 gap-4">
        {isPending
          ? [0, 1, 2, 3, 4, 5].map((i) => (
              <Skeleton key={i} className="h-[172px] rounded-[22px]" />
            ))
          : null}

        {visible.map((server) => {
          const reading = latencies.get(server.id);
          return (
            <ServerCard
              key={server.id}
              server={server}
              focusProtocol={focusProtocol}
              selected={server.id === selectedId}
              inUse={live && server.id === activeServerId}
              favourite={isFavourite(server.id)}
              latencyMs={reading?.ms}
              latencyPending={reading?.probing}
              onSelect={handleSelect}
              onToggleFavourite={toggle}
              onRemove={(id) =>
                remove.mutate(id, {
                  onSuccess: () => toast.success("Server removed"),
                  onError: (err: unknown) =>
                    toast.error("Could not remove that server", {
                      description:
                        err instanceof Error ? err.message : String(err),
                    }),
                })
              }
            />
          );
        })}
      </div>
    </div>
  );
}

function FilterChip({
  active,
  count,
  onClick,
  children,
}: {
  active: boolean;
  count?: number;
  onClick: () => void;
  children: ReactNode;
}) {
  return (
    <button
      type="button"
      role="radio"
      aria-checked={active}
      onClick={onClick}
      className={cn(
        "flex h-9 items-center gap-1.5 rounded-full border px-3.5 text-[13px] font-medium transition-colors",
        "focus-visible:outline-accent outline-none focus-visible:outline-2",
        active
          ? "border-accent bg-accent-surface text-accent-bright"
          : "border-border-subtle bg-surface text-text-secondary hover:border-border-strong",
      )}
    >
      {children}
      {count !== undefined ? (
        <span
          className={cn(
            "font-mono text-[11px]",
            active ? "text-accent-bright/70" : "text-text-faint",
          )}
        >
          {count}
        </span>
      ) : null}
    </button>
  );
}
