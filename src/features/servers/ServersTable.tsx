import {
  createColumnHelper,
  createSortedRowModel,
  rowSelectionFeature,
  rowSortingFeature,
  sortFn_basic,
  sortFn_text,
  tableFeatures,
  useTable,
} from "@tanstack/react-table";
import { openUrl } from "@tauri-apps/plugin-opener";
import {
  ArrowDown,
  ArrowUp,
  ArrowUpDown,
  ExternalLink,
  Star,
  Trash2,
  X,
} from "lucide-react";
import {
  createContext,
  useContext,
  type KeyboardEvent,
  type MouseEvent,
  type ReactNode,
} from "react";

import {
  Button,
  Checkbox,
  IconButton,
  Skeleton,
  Tooltip,
} from "@/design/primitives";
import { ProviderMark } from "@/features/servers/ProviderMark";
import {
  humanTag,
  latencySortValue,
  sortProtocols,
} from "@/features/servers/filter";
import type { LatencyReading } from "@/features/servers/useLatencies";
import { cn } from "@/lib/cn";
import type { Protocol, Server } from "@/lib/ipc";
import {
  classifyLatency,
  formatLatency,
  type LatencyTone,
} from "@/lib/latency";

/** One line of the table: a server, and the protocol whose figures it shows. */
export interface ServerRow {
  server: Server;
  /** The protocol whose address and latency this row reports. */
  protocol: Protocol | null;
  latency: LatencyReading | undefined;
}

interface TableActions {
  /** The server the connection screen has picked. */
  pickedId: string | null;
  /** The server DNS is going to right now. */
  inUseId: string | null;
  favourites: readonly string[];
  onPick: (server: Server) => void;
  onToggleFavourite: (id: string) => void;
  onRemove: (servers: Server[]) => void;
}

/**
 * Cells render from context rather than from props the column definitions
 * close over. The definitions have to live at module scope — the table rebuilds
 * its models whenever they change identity — so anything that varies (which row
 * is picked, who is starred) reaches them this way.
 */
const ActionsContext = createContext<TableActions | null>(null);

function useActions(): TableActions {
  const actions = useContext(ActionsContext);
  if (!actions) throw new Error("ServersTable cells render inside the table");
  return actions;
}

/** Keeps a click on a control inside the row from also picking the row. */
const stop = (event: MouseEvent) => event.stopPropagation();

const features = tableFeatures({
  rowSortingFeature,
  rowSelectionFeature,
  sortedRowModel: createSortedRowModel(),
  sortFns: { text: sortFn_text, basic: sortFn_basic },
});

const helper = createColumnHelper<typeof features, ServerRow>();

/** Fixed widths for everything but the name, which takes the rest. */
const WIDTHS: Record<string, string> = {
  select: "w-10",
  favourite: "w-10",
  address: "w-[172px]",
  protocols: "w-[224px]",
  latency: "w-[120px]",
  actions: "w-[92px]",
};

const columns = helper.columns([
  helper.display({
    id: "select",
    header: ({ table }) => (
      <Checkbox
        aria-label="Select all servers"
        checked={table.getIsAllRowsSelected()}
        indeterminate={
          table.getIsSomeRowsSelected() && !table.getIsAllRowsSelected()
        }
        onCheckedChange={(checked) => table.toggleAllRowsSelected(checked)}
        onClick={stop}
      />
    ),
    cell: ({ row }) => (
      <Checkbox
        aria-label={`Select ${row.original.server.displayName}`}
        checked={row.getIsSelected()}
        onCheckedChange={(checked) => row.toggleSelected(checked)}
        onClick={stop}
      />
    ),
    enableSorting: false,
  }),

  helper.display({
    id: "favourite",
    header: () => <span className="sr-only">Favourite</span>,
    cell: ({ row }) => <FavouriteCell server={row.original.server} />,
    enableSorting: false,
  }),

  helper.accessor((row) => row.server.displayName, {
    id: "name",
    header: "Server",
    cell: ({ row }) => <NameCell server={row.original.server} />,
    sortFn: "text",
  }),

  helper.display({
    id: "address",
    header: "Address",
    cell: ({ row }) => <AddressCell row={row.original} />,
    enableSorting: false,
  }),

  helper.display({
    id: "protocols",
    header: "Protocols",
    cell: ({ row }) => <ProtocolsCell row={row.original} />,
    enableSorting: false,
  }),

  helper.accessor((row) => latencySortValue(row.latency?.ms), {
    id: "latency",
    header: "Latency",
    cell: ({ row }) => <LatencyCell reading={row.original.latency} />,
    sortFn: "basic",
    // Fastest first on the first click: for latency, ascending is the answer to
    // "which is best", where the numeric default would open on the slowest.
    sortDescFirst: false,
    sortUndefined: "last",
  }),

  helper.display({
    id: "actions",
    header: () => <span className="sr-only">Actions</span>,
    cell: ({ row }) => <ActionsCell server={row.original.server} />,
    enableSorting: false,
  }),
]);

export function ServersTable({
  rows,
  loading,
  ...actions
}: TableActions & {
  rows: ServerRow[];
  loading?: boolean;
}) {
  const table = useTable({
    features,
    columns,
    data: rows,
    getRowId: (row) => row.server.id,
  });

  const checked = table.getSelectedRowModel().rows;

  function removeChecked() {
    actions.onRemove(checked.map((row) => row.original.server));
    table.resetRowSelection(true);
  }

  return (
    <ActionsContext.Provider value={actions}>
      {checked.length > 0 ? (
        <div
          role="status"
          className="bg-accent-surface border-accent-muted mb-3 flex items-center gap-3 rounded-2xl border py-2 pr-2 pl-4"
        >
          <p className="text-accent-bright flex-1 text-sm font-medium">
            {checked.length} selected
          </p>
          <Button
            size="sm"
            variant="ghost"
            startIcon={<X />}
            onClick={() => table.resetRowSelection(true)}
          >
            Clear
          </Button>
          <Button
            size="sm"
            variant="danger"
            startIcon={<Trash2 />}
            onClick={removeChecked}
          >
            Remove
          </Button>
        </div>
      ) : null}

      <div className="border-border-subtle bg-surface rounded-2xl border">
        <table className="w-full table-fixed border-separate border-spacing-0 text-left">
          <caption className="sr-only">
            DNS servers. Select a row to use that server.
          </caption>

          <thead>
            {table.getHeaderGroups().map((group) => (
              <tr key={group.id}>
                {group.headers.map((header) => {
                  const column = header.column;
                  const sorted = column.getIsSorted();
                  return (
                    <th
                      key={header.id}
                      scope="col"
                      aria-sort={
                        sorted === "asc"
                          ? "ascending"
                          : sorted === "desc"
                            ? "descending"
                            : column.getCanSort()
                              ? "none"
                              : undefined
                      }
                      className={cn(
                        "bg-surface-subtle border-border-subtle text-text-muted sticky top-0 z-10 h-11 border-b px-3 text-[11px] font-semibold tracking-wider uppercase",
                        "first:rounded-tl-2xl last:rounded-tr-2xl",
                        WIDTHS[column.id],
                      )}
                    >
                      {header.isPlaceholder ? null : column.getCanSort() ? (
                        <button
                          type="button"
                          onClick={column.getToggleSortingHandler()}
                          className={cn(
                            "-mx-2 flex h-8 items-center gap-1.5 rounded-md px-2 tracking-wider uppercase transition-colors",
                            "hover:text-text-primary focus-visible:outline-accent outline-none focus-visible:outline-2",
                            sorted && "text-text-primary",
                          )}
                        >
                          <table.FlexRender header={header} />
                          {sorted === "asc" ? (
                            <ArrowUp className="size-3.5" />
                          ) : sorted === "desc" ? (
                            <ArrowDown className="size-3.5" />
                          ) : (
                            <ArrowUpDown className="size-3.5 opacity-50" />
                          )}
                        </button>
                      ) : (
                        <table.FlexRender header={header} />
                      )}
                    </th>
                  );
                })}
              </tr>
            ))}
          </thead>

          <tbody>
            {loading
              ? [0, 1, 2, 3, 4, 5, 6].map((i) => <SkeletonRow key={i} />)
              : table.getRowModel().rows.map((row) => (
                  <ServerTableRow
                    key={row.id}
                    server={row.original.server}
                    checked={row.getIsSelected()}
                  >
                    {row.getAllCells().map((cell) => (
                      <td
                        key={cell.id}
                        data-column={cell.column.id}
                        className="border-border-subtle h-[68px] border-b px-3 group-last:border-b-0 first:relative"
                      >
                        <table.FlexRender cell={cell} />
                      </td>
                    ))}
                  </ServerTableRow>
                ))}
          </tbody>
        </table>
      </div>
    </ActionsContext.Provider>
  );
}

function ServerTableRow({
  server,
  checked,
  children,
}: {
  server: Server;
  checked: boolean;
  children: ReactNode;
}) {
  const { pickedId, inUseId, onPick } = useActions();
  const inUse = server.id === inUseId;
  const picked = server.id === pickedId;

  function onKeyDown(event: KeyboardEvent<HTMLElement>) {
    // Only when the row itself has focus: Space on a checkbox or button inside
    // it belongs to that control.
    if (event.target !== event.currentTarget) return;
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      onPick(server);
    }
  }

  return (
    <tr
      tabIndex={0}
      aria-current={picked ? "true" : undefined}
      aria-selected={checked}
      data-picked={picked || undefined}
      data-in-use={inUse || undefined}
      onClick={() => onPick(server)}
      onKeyDown={onKeyDown}
      className={cn(
        "group cursor-pointer transition-colors outline-none",
        "hover:bg-surface-hover/70 focus-visible:bg-surface-hover",
        "focus-visible:outline-accent focus-visible:outline-2 focus-visible:-outline-offset-2",
        checked && "bg-accent-surface/50 hover:bg-accent-surface/70",
        // The marker in the first cell, so "picked" reads without relying on a
        // tint that a checked row also uses.
        "[&>td:first-child]:before:absolute [&>td:first-child]:before:inset-y-3 [&>td:first-child]:before:left-0 [&>td:first-child]:before:w-[3px] [&>td:first-child]:before:rounded-full [&>td:first-child]:before:content-['']",
        inUse
          ? "[&>td:first-child]:before:bg-success"
          : picked && "[&>td:first-child]:before:bg-accent",
      )}
    >
      {children}
    </tr>
  );
}

function SkeletonRow() {
  return (
    <tr>
      <td colSpan={columns.length} className="h-[68px] px-4">
        <Skeleton className="h-9 w-full rounded-xl" />
      </td>
    </tr>
  );
}

function FavouriteCell({ server }: { server: Server }) {
  const { favourites, onToggleFavourite } = useActions();
  const favourite = favourites.includes(server.id);

  return (
    <Tooltip content={favourite ? "Remove favourite" : "Favourite"}>
      <IconButton
        label={
          favourite
            ? `Unfavourite ${server.displayName}`
            : `Favourite ${server.displayName}`
        }
        size="sm"
        aria-pressed={favourite}
        onClick={(event) => {
          stop(event);
          onToggleFavourite(server.id);
        }}
        className={cn(
          "size-8 rounded-lg [&_svg]:size-4",
          favourite
            ? "text-warning"
            : "text-text-faint opacity-0 group-focus-within:opacity-100 group-hover:opacity-100 focus-visible:opacity-100",
        )}
      >
        <Star className={cn(favourite && "fill-current")} />
      </IconButton>
    </Tooltip>
  );
}

function NameCell({ server }: { server: Server }) {
  const { pickedId, inUseId } = useActions();
  const inUse = server.id === inUseId;
  const picked = server.id === pickedId;

  const detail = server.tags.slice(0, 3).map(humanTag).join(" · ");

  return (
    <div className="flex min-w-0 items-center gap-3">
      <ProviderMark
        provider={server.provider}
        name={server.name}
        className="size-10 rounded-xl text-sm"
      />

      <div className="min-w-0">
        <div className="flex min-w-0 items-center gap-2">
          <span className="truncate text-sm font-semibold">
            {server.variant ? (
              <>
                {server.name}
                <span className="text-text-muted font-medium">
                  {" "}
                  · {server.variant}
                </span>
              </>
            ) : (
              server.name
            )}
          </span>

          {inUse ? (
            <span className="bg-success-surface text-success flex shrink-0 items-center gap-1.5 rounded-full px-2 py-0.5 text-[11px] font-bold">
              <span className="bg-success size-1.5 rounded-full" />
              In use
            </span>
          ) : picked ? (
            <span className="bg-accent-surface text-accent-bright shrink-0 rounded-full px-2 py-0.5 text-[11px] font-bold">
              Selected
            </span>
          ) : null}

          {server.source === "custom" ? (
            <span className="bg-surface-active text-text-secondary shrink-0 rounded-full px-2 py-0.5 text-[11px] font-semibold">
              Custom
            </span>
          ) : null}
        </div>

        {detail ? (
          <p className="text-text-muted truncate text-xs">{detail}</p>
        ) : null}
      </div>
    </div>
  );
}

function shownEndpoint(row: ServerRow) {
  const { server, protocol } = row;
  return (
    server.endpoints.find((e) => e.protocol === protocol) ?? server.endpoints[0]
  );
}

/** A DoH address is a URL; the host is the part worth reading in a table. */
function displayAddr(addr: string): string {
  try {
    return new URL(addr).host;
  } catch {
    return addr;
  }
}

function AddressCell({ row }: { row: ServerRow }) {
  const endpoint = shownEndpoint(row);
  if (!endpoint) return <span className="text-text-faint">—</span>;

  const [first, ...rest] = endpoint.addrs;
  return (
    <div
      className="flex min-w-0 items-baseline gap-1.5"
      title={endpoint.addrs.join("\n")}
    >
      <span
        data-selectable
        className="text-text-secondary truncate font-mono text-[13px]"
      >
        {displayAddr(first!)}
      </span>
      {rest.length > 0 ? (
        <span className="text-text-faint shrink-0 font-mono text-[11px]">
          +{rest.length}
        </span>
      ) : null}
    </div>
  );
}

function ProtocolsCell({ row }: { row: ServerRow }) {
  const { server, protocol: focus } = row;

  return (
    <div className="flex flex-wrap gap-1">
      {sortProtocols(server.protocols).map((protocol) => {
        const endpoint = server.endpoints.find((e) => e.protocol === protocol);
        if (!endpoint) return null;
        return (
          <span
            key={protocol}
            className={cn(
              "rounded-md px-1.5 py-0.5 text-[11px] font-bold",
              endpoint.encrypted
                ? "bg-accent-cyan-surface text-protocol-encrypted"
                : "bg-surface-inset text-text-muted",
              // Under a protocol filter the row reports that one; the rest are
              // still listed, but quieter.
              focus && protocol !== focus && "opacity-45",
            )}
          >
            {endpoint.label}
          </span>
        );
      })}
    </div>
  );
}

const BAR_FILL: Record<LatencyTone, string> = {
  success: "bg-success",
  warning: "bg-warning",
  danger: "bg-danger",
  neutral: "bg-text-faint",
};

const BAR_TEXT: Record<LatencyTone, string> = {
  success: "text-success",
  warning: "text-warning",
  danger: "text-danger",
  neutral: "text-text-muted",
};

/** Latency at which the bar is empty; anything faster fills part of it. */
const BAR_SCALE_MS = 300;

function LatencyCell({ reading }: { reading: LatencyReading | undefined }) {
  if (reading?.probing) {
    return (
      <span
        role="status"
        className="text-text-muted flex animate-pulse items-center gap-2 text-xs"
      >
        <span className="border-accent-bright size-3 animate-spin rounded-full border-2 border-t-transparent" />
        Testing
      </span>
    );
  }

  const ms = reading?.ms;

  if (ms === null) {
    return <span className="text-danger text-xs font-medium">No answer</span>;
  }
  if (ms === undefined) {
    return (
      <span className="text-text-faint font-mono text-sm" title="Not tested">
        —
      </span>
    );
  }

  const tone = classifyLatency(ms);
  const fill = Math.max(0.06, 1 - Math.min(ms, BAR_SCALE_MS) / BAR_SCALE_MS);

  return (
    <div className="flex items-center gap-2.5">
      <span
        aria-hidden
        className="bg-surface-active h-1.5 w-10 shrink-0 overflow-hidden rounded-full"
      >
        <span
          className={cn(
            "block h-full rounded-full transition-[width] duration-500",
            BAR_FILL[tone],
          )}
          style={{ width: `${fill * 100}%` }}
        />
      </span>
      <span
        className={cn(
          "font-mono text-[13px] font-semibold tabular-nums",
          BAR_TEXT[tone],
        )}
      >
        {formatLatency(ms)}
      </span>
    </div>
  );
}

function ActionsCell({ server }: { server: Server }) {
  const { onRemove } = useActions();

  return (
    <div className="-mr-1 flex justify-end opacity-0 transition-opacity group-focus-within:opacity-100 group-hover:opacity-100">
      {server.homepage ? (
        <Tooltip content="Open provider site">
          <IconButton
            label={`Open ${server.name} website`}
            size="sm"
            onClick={(event) => {
              stop(event);
              void openUrl(server.homepage!);
            }}
            className="text-text-muted size-8 rounded-lg [&_svg]:size-4"
          >
            <ExternalLink />
          </IconButton>
        </Tooltip>
      ) : null}

      <Tooltip content="Remove from the list">
        <IconButton
          label={`Remove ${server.displayName}`}
          size="sm"
          onClick={(event) => {
            stop(event);
            onRemove([server]);
          }}
          className="text-text-muted hover:text-danger size-8 rounded-lg [&_svg]:size-4"
        >
          <Trash2 />
        </IconButton>
      </Tooltip>
    </div>
  );
}
