import {
  CloudDownload,
  RefreshCw,
  Search,
  ServerOff,
  Star,
  X,
} from "lucide-react";
import { useMemo, useState, type ReactNode } from "react";
import { useNavigate } from "react-router";

import {
  AlertDialog,
  Button,
  EmptyState,
  IconButton,
  PageHeader,
  pagePadding,
  toast,
  Tooltip,
} from "@/design/primitives";
import { AddServerDialog } from "@/features/servers/AddServerDialog";
import { ServersTable, type ServerRow } from "@/features/servers/ServersTable";
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
  useReinstateServers,
  useRemoveServers,
  useRestoreDefaults,
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
  const { favourites, toggle } = useFavourites();
  const remove = useRemoveServers();
  const reinstate = useReinstateServers();
  const restore = useRestoreDefaults();
  const { phase, activeServerId } = useConnection();
  const { selectedId } = useServerSelection();
  const { pick } = useServerPicker();
  const navigate = useNavigate();

  const [filter, setFilter] = useState<Filter>("all");
  const [query, setQuery] = useState("");
  // Removing servers the user added is permanent, so it asks first.
  const [pendingRemoval, setPendingRemoval] = useState<Server[] | null>(null);

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

  // Favourites first, then catalogue order. Not sorted by latency unless the
  // user asks: rows that jumped around as each probe landed would move the one
  // under the cursor.
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

  const rows = useMemo<ServerRow[]>(
    () =>
      targets.map(({ server, protocol }) => ({
        server,
        protocol,
        latency: latencies.get(server.id),
      })),
    [targets, latencies],
  );

  const live = phase === "connected" || phase === "switching";

  // Counted from the servers that exist rather than from the stored ids: a
  // default server that was removed keeps its star, but must not inflate this.
  const favouriteCount = useMemo(
    () => (servers ?? []).filter((s) => favourites.includes(s.id)).length,
    [servers, favourites],
  );

  function handleSelect(server: Server) {
    // Under a protocol filter honour it, so clicking a row does what the
    // filter implies; otherwise the server's own preferred protocol.
    if (pick(server, focusProtocol ?? undefined)) void navigate("/");
  }

  function requestRemoval(list: Server[]) {
    if (list.length === 0) return;
    if (list.some((s) => s.source === "custom")) setPendingRemoval(list);
    else removeNow(list);
  }

  function removeNow(list: Server[]) {
    remove.mutate(list, {
      onSuccess: ({ defaults, custom }) => {
        const what =
          list.length === 1 ? list[0]!.displayName : `${list.length} servers`;

        // Only default servers can come back, so only a removal made purely of
        // them offers an undo.
        if (custom.length === 0) {
          toast.success(`Removed ${what}`, {
            description: "Restore defaults brings default servers back.",
            action: {
              label: "Undo",
              onClick: () => reinstate.mutate(defaults.map((s) => s.id)),
            },
          });
        } else {
          toast.success(`Removed ${what}`);
        }
      },
      onError: (err: unknown) =>
        toast.error("Could not remove that", {
          description: err instanceof Error ? err.message : String(err),
        }),
    });
  }

  function restoreDefaults() {
    restore.mutate(undefined, {
      onSuccess: ({ origin, restored, total }) => {
        if (origin === "local") {
          toast.warning("Could not reach GitHub", {
            description:
              restored > 0
                ? `Brought back ${restored} from the copy on this computer.`
                : "The copy on this computer is already complete.",
          });
        } else if (restored > 0) {
          toast.success(
            `Restored ${restored} ${restored === 1 ? "server" : "servers"}`,
            { description: `Up to date with GitHub: ${total} defaults.` },
          );
        } else {
          toast.success("Already up to date", {
            description: `All ${total} default servers are here.`,
          });
        }
      },
      onError: (err: unknown) =>
        toast.error("Could not restore the defaults", {
          description: err instanceof Error ? err.message : String(err),
        }),
    });
  }

  const restoreButton = (
    <Tooltip content="Bring back removed servers and refresh the default list from GitHub">
      <Button
        size="sm"
        className="rounded-full"
        startIcon={
          <CloudDownload className={cn(restore.isPending && "animate-pulse")} />
        }
        disabled={restore.isPending}
        onClick={restoreDefaults}
      >
        {restore.isPending ? "Restoring…" : "Restore defaults"}
      </Button>
    </Tooltip>
  );

  const nothingLeft = !isPending && servers?.length === 0;

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
            {restoreButton}
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
            count={favouriteCount}
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
          title={nothingLeft ? "No servers left" : "No servers match"}
          description={
            nothingLeft
              ? "You have removed every server. Restore the defaults to get them back."
              : query
                ? `Nothing matches “${query}” here.`
                : filter === "favourites"
                  ? "Star a server and it will show up here, and first on the Connection screen."
                  : "No server offers this protocol."
          }
          action={
            nothingLeft ? (
              restoreButton
            ) : query ? (
              <Button onClick={() => setQuery("")}>Clear search</Button>
            ) : undefined
          }
        />
      ) : (
        <ServersTable
          rows={rows}
          loading={isPending}
          pickedId={selectedId}
          inUseId={live ? activeServerId : null}
          favourites={favourites}
          onPick={handleSelect}
          onToggleFavourite={toggle}
          onRemove={requestRemoval}
        />
      )}

      <AlertDialog
        open={pendingRemoval !== null}
        onOpenChange={(open) => {
          if (!open) setPendingRemoval(null);
        }}
        title={
          pendingRemoval?.length === 1
            ? `Remove ${pendingRemoval[0]!.displayName}?`
            : `Remove ${pendingRemoval?.length ?? 0} servers?`
        }
        description={removalWarning(pendingRemoval ?? [])}
        footer={
          <>
            <Button onClick={() => setPendingRemoval(null)}>Cancel</Button>
            <Button
              variant="danger"
              onClick={() => {
                if (pendingRemoval) removeNow(pendingRemoval);
                setPendingRemoval(null);
              }}
            >
              Remove
            </Button>
          </>
        }
      />
    </div>
  );
}

function removalWarning(servers: Server[]): string {
  const own = servers.filter((s) => s.source === "custom").length;
  const defaults = servers.length - own;

  if (defaults === 0) {
    return own === 1
      ? "You added this server. Removing it deletes it for good."
      : "You added these servers. Removing them deletes them for good.";
  }
  return `${own} of these ${own === 1 ? "is a server" : "are servers"} you added and will be deleted for good. The other ${defaults} ${defaults === 1 ? "is a default, which" : "are defaults, which"} Restore defaults can bring back.`;
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
