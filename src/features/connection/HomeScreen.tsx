import {
  Activity,
  ArrowRight,
  ChevronRight,
  Globe,
  Lock,
  TriangleAlert,
  Wifi,
  Zap,
} from "lucide-react";
import { useId, useMemo, type ReactNode } from "react";
import { Link } from "react-router";

import {
  CopyableValue,
  Segmented,
  Skeleton,
  Tile,
  TileLabel,
  Tooltip,
  type SegmentedOption,
} from "@/design/primitives";
import { ConnectionHero } from "@/features/connection/ConnectionHero";
import { ServerSelect } from "@/features/connection/ServerSelect";
import { useConnectTarget } from "@/features/connection/useConnectTarget";
import { useConnection } from "@/features/connection/useConnection";
import { useLatencyHistory } from "@/features/connection/useLatencyHistory";
import { useServerPicker } from "@/features/connection/useServerPicker";
import {
  humanTag,
  preferredProtocol,
  sortProtocols,
} from "@/features/servers/filter";
import { useLatencies } from "@/features/servers/useLatencies";
import { cn } from "@/lib/cn";
import {
  classifyLatency,
  formatLatency,
  type LatencyTone,
} from "@/lib/latency";
import type { Endpoint, Protocol, Server } from "@/lib/ipc";

export function HomeScreen() {
  const {
    servers,
    isPending,
    favourites,
    selected,
    chosenProtocol,
    endpoint,
    blocker,
  } = useConnectTarget();
  const { adapter, target } = useConnection();

  return (
    <div className="grid h-full min-h-[560px] grid-cols-4 grid-rows-3 gap-4 px-8 pt-8 pb-(--dock-clearance)">
      <ConnectionHero
        selected={selected}
        endpoint={endpoint}
        blocker={blocker}
      />

      {isPending || !servers ? (
        <Skeleton className="rounded-tile col-span-2 row-span-2 h-full" />
      ) : (
        <ServerTile
          servers={servers}
          favourites={favourites}
          selected={selected}
          chosenProtocol={chosenProtocol}
          endpoint={endpoint}
        />
      )}

      <LatencyTile server={selected} protocol={chosenProtocol} />
      <InterfaceTile
        adapterName={adapter?.name ?? null}
        description={adapter?.description ?? null}
        networkDns={adapter?.dnsV4[0] ?? null}
        fallback={target.unavailableReason !== null}
        pinned={target.isPinned}
      />
    </div>
  );
}

function ServerTile({
  servers,
  favourites,
  selected,
  chosenProtocol,
  endpoint,
}: {
  servers: Server[];
  favourites: string[];
  selected: Server | null;
  chosenProtocol: Protocol | null;
  endpoint: Endpoint | undefined;
}) {
  const { phase, busy, activeServerId } = useConnection();
  const { pick } = useServerPicker();
  const live = phase === "connected" || phase === "switching";

  const protocolOptions: SegmentedOption<Protocol>[] = selected
    ? sortProtocols(selected.protocols).map((value) => {
        const option = selected.endpoints.find((e) => e.protocol === value);
        const waiting = option?.requiresForwarder ?? false;
        return {
          value,
          label: option?.label ?? value,
          icon: option?.encrypted ? <Lock /> : undefined,
          disabled: waiting,
          hint: waiting
            ? "Needs the local resolver, which arrives in a later version"
            : undefined,
        };
      })
    : [];

  return (
    <Tile className="col-span-2 row-span-2">
      <TileLabel
        icon={<Globe />}
        action={
          <Link
            to="/servers"
            className="text-text-muted hover:bg-surface-hover hover:text-text-primary -mr-2 flex items-center rounded-lg py-1 pr-1.5 pl-2.5 text-[13px]"
          >
            Browse all {servers.length}
            <ChevronRight className="size-4" />
          </Link>
        }
      >
        DNS server
      </TileLabel>

      <div className="mt-3">
        <ServerSelect
          servers={servers}
          favourites={favourites}
          value={selected}
          liveId={live ? activeServerId : null}
          detail={
            endpoint && !endpoint.requiresForwarder
              ? [selected?.variant, endpoint.addrs.join(", ")]
                  .filter(Boolean)
                  .join(" · ")
              : undefined
          }
          disabled={busy}
          onChange={(server) => pick(server)}
        />
      </div>

      {protocolOptions.length > 0 ? (
        <DetailRow label="Protocol" className="mt-4">
          <Segmented
            label="Protocol"
            value={chosenProtocol}
            options={protocolOptions}
            disabled={busy}
            onValueChange={(value) => selected && pick(selected, value)}
          />
        </DetailRow>
      ) : null}

      {endpoint && !endpoint.requiresForwarder ? (
        <DetailRow label="Address" className="mt-3">
          <span className="flex flex-wrap justify-end gap-x-3 gap-y-1">
            {endpoint.addrs.map((addr) => (
              <CopyableValue key={addr} value={addr} />
            ))}
          </span>
        </DetailRow>
      ) : null}

      {selected && selected.tags.length > 0 ? (
        <DetailRow label="Known for" className="mt-3">
          <span className="flex flex-wrap justify-end gap-1.5">
            {selected.tags.slice(0, 3).map((tag) => (
              <span
                key={tag}
                className="bg-surface-inset text-text-secondary rounded-full px-2.5 py-0.5 text-xs font-medium"
              >
                {humanTag(tag)}
              </span>
            ))}
          </span>
        </DetailRow>
      ) : null}

      <FastestHint
        servers={servers}
        selected={selected}
        disabled={busy}
        onUse={(server) => pick(server)}
      />
    </Tile>
  );
}

function DetailRow({
  label,
  className,
  children,
}: {
  label: string;
  className?: string;
  children: ReactNode;
}) {
  return (
    <div className={cn("flex min-h-8 items-center gap-4", className)}>
      <span className="text-text-muted w-20 shrink-0 text-[13px]">{label}</span>
      <div className="flex min-w-0 flex-1 justify-end">{children}</div>
    </div>
  );
}

/**
 * One line on speed, so the list itself can stay in a stable order.
 *
 * Reads whatever has been measured — the selected server always is, and
 * opening the list measures the rest. Only speaks up when the difference is
 * worth a click: a server 2ms faster is noise, and nagging about it would
 * teach the user to ignore the line.
 */
function FastestHint({
  servers,
  selected,
  disabled,
  onUse,
}: {
  servers: Server[];
  selected: Server | null;
  disabled: boolean;
  onUse: (server: Server) => void;
}) {
  const latencies = useLatencies(
    servers.map((server) => ({ server, protocol: preferredProtocol(server) })),
    { probe: false },
  );

  const measured = servers
    .filter((server) => server.endpoints.some((e) => !e.requiresForwarder))
    .map((server) => ({ server, ms: latencies.get(server.id)?.ms }))
    .filter(
      (entry): entry is { server: Server; ms: number } =>
        typeof entry.ms === "number",
    )
    .sort((a, b) => a.ms - b.ms);

  const fastest = measured[0];
  const current = measured.find((entry) => entry.server.id === selected?.id);

  let body: ReactNode;
  if (measured.length < 2) {
    body = <span>Open the list to compare response times</span>;
  } else if (
    current &&
    fastest &&
    fastest.server.id !== current.server.id &&
    current.ms - fastest.ms >= 10 &&
    fastest.ms <= current.ms * 0.8
  ) {
    body = (
      <>
        <span className="min-w-0 truncate">
          <b className="text-text-secondary font-semibold">
            {fastest.server.name}
          </b>{" "}
          answers faster here ({formatLatency(fastest.ms)})
        </span>
        <button
          type="button"
          disabled={disabled}
          onClick={() => onUse(fastest.server)}
          className="text-accent-bright hover:bg-accent-surface ml-auto shrink-0 rounded-lg px-2.5 py-1 font-semibold disabled:opacity-45"
        >
          Use it
        </button>
      </>
    );
  } else if (current && fastest?.server.id === current.server.id) {
    body = (
      <span className="truncate">
        <b className="text-text-secondary font-semibold">
          {current.server.name}
        </b>{" "}
        is the fastest you have measured
      </span>
    );
  } else {
    body = <span>Nothing measured is meaningfully faster</span>;
  }

  return (
    <div className="border-border-subtle text-text-muted mt-auto flex min-h-10 items-center gap-2 border-t pt-3 text-[13px]">
      <Zap className="size-4 shrink-0" />
      {body}
    </div>
  );
}

const QUALITY: Record<LatencyTone, [string, string]> = {
  success: ["Fast", "bg-success-surface text-success"],
  warning: ["Fair", "bg-warning-surface text-warning"],
  danger: ["Slow", "bg-danger-surface text-danger"],
  neutral: ["", ""],
};

const TONE_TEXT: Record<LatencyTone, string> = {
  success: "text-success",
  warning: "text-warning",
  danger: "text-danger",
  neutral: "text-text-muted",
};

/** The line colour per tone. Cyan rather than green for "fast", as designed. */
const TONE_LINE: Record<LatencyTone, string> = {
  success: "var(--accent-cyan)",
  warning: "var(--warning)",
  danger: "var(--danger)",
  neutral: "var(--text-faint)",
};

/**
 * Latency for the selected server, drawn live.
 *
 * Every point on the line is a real probe: the tile samples every few seconds
 * while it is on screen and keeps the last couple of minutes. Until there are
 * two points there is no line, rather than a made-up one.
 */
function LatencyTile({
  server,
  protocol,
}: {
  server: Server | null;
  protocol: Protocol | null;
}) {
  const { ms, isProbing, samples } = useLatencyHistory(
    server?.id ?? null,
    protocol,
  );
  const tone = classifyLatency(ms);
  const noAnswer = ms === null && !isProbing;
  const [quality, qualityClass] = noAnswer
    ? ["No answer", "bg-danger-surface text-danger"]
    : QUALITY[tone];

  return (
    <Tile>
      {/* Drawn first and pinned to the bottom edge, as in the design: the
          figures sit over the top of the line rather than above it, which is
          what lets a tile this short carry both. */}
      <Sparkline values={samples} color={TONE_LINE[tone]} />

      <TileLabel
        className="relative"
        icon={<Activity />}
        action={
          quality ? (
            <span
              className={cn(
                "rounded-full px-2.5 py-0.5 text-xs font-semibold",
                qualityClass,
              )}
            >
              {quality}
            </span>
          ) : null
        }
      >
        Latency
      </TileLabel>

      <div
        className={cn(
          "relative mt-2 font-mono text-[32px] leading-none font-bold tracking-tight tabular-nums",
          TONE_TEXT[tone],
        )}
      >
        {typeof ms === "number"
          ? Math.max(1, Math.round(ms))
          : isProbing
            ? "···"
            : "—"}
        {typeof ms === "number" ? (
          <small className="text-text-muted ml-1 font-sans text-sm font-medium tracking-normal">
            ms
          </small>
        ) : null}
      </div>
      <p className="text-text-muted relative mt-1.5 truncate text-[13px]">
        {server ? `ping to ${server.name}` : "No server selected"}
      </p>
    </Tile>
  );
}

/**
 * The live line along the bottom edge of the latency tile.
 *
 * The vertical range is fitted to the samples with some headroom, so jitter
 * of a few milliseconds is visible; the figure above it gives the scale.
 */
function Sparkline({ values, color }: { values: number[]; color: string }) {
  const id = useId();
  const width = 200;
  const height = 60;
  const pad = 8;

  const path = useMemo(() => {
    if (values.length < 2) return null;
    const min = Math.min(...values);
    const max = Math.max(...values);
    const span = Math.max(max - min, 4);
    const low = Math.max(0, min - span * 0.6);
    const high = max + span * 0.6;

    const line = values
      .map((v, i) => {
        const x = (i / (values.length - 1)) * width;
        const y = pad + (1 - (v - low) / (high - low)) * (height - pad * 2);
        return `${i ? "L" : "M"}${x.toFixed(1)} ${y.toFixed(1)}`;
      })
      .join(" ");
    return { line, area: `${line} L${width} ${height} L0 ${height} Z` };
  }, [values]);

  return (
    <div className="absolute inset-x-0 bottom-0 h-16">
      {path ? (
        <svg
          viewBox={`0 0 ${width} ${height}`}
          preserveAspectRatio="none"
          className="absolute inset-0 size-full"
          aria-hidden
        >
          <defs>
            <linearGradient id={id} x1="0" y1="0" x2="0" y2="1">
              {/* Through `style`: presentation attributes do not resolve
                  CSS variables, and the colours here are theme tokens. */}
              <stop
                offset="0"
                style={{ stopColor: color, stopOpacity: 0.32 }}
              />
              <stop offset="1" style={{ stopColor: color, stopOpacity: 0 }} />
            </linearGradient>
          </defs>
          <path d={path.area} fill={`url(#${id})`} />
          <path
            d={path.line}
            fill="none"
            style={{ stroke: color }}
            strokeWidth="2"
            strokeLinejoin="round"
            vectorEffect="non-scaling-stroke"
          />
        </svg>
      ) : (
        <span className="text-text-faint absolute inset-x-6 bottom-5 text-xs">
          Measuring…
        </span>
      )}
    </div>
  );
}

function InterfaceTile({
  adapterName,
  description,
  networkDns,
  fallback,
  pinned,
}: {
  adapterName: string | null;
  description: string | null;
  networkDns: string | null;
  fallback: boolean;
  pinned: boolean;
}) {
  const { active, phase } = useConnection();
  const routed = phase === "connected" || phase === "switching";

  return (
    <Tile>
      <TileLabel
        icon={<Wifi />}
        action={
          <Link
            to="/interfaces"
            aria-label="Choose an interface"
            className="text-text-muted hover:bg-surface-hover hover:text-text-primary -mr-2 flex size-8 items-center justify-center rounded-lg"
          >
            <ChevronRight className="size-4" />
          </Link>
        }
      >
        Interface
      </TileLabel>

      <div className="mt-auto flex min-w-0 items-center gap-1.5">
        <span className="truncate text-xl font-bold tracking-tight">
          {adapterName ?? "None"}
        </span>
        {fallback ? (
          <Tooltip content="The interface you picked is unavailable, so DNS is going here instead.">
            <TriangleAlert className="text-warning size-4 shrink-0" />
          </Tooltip>
        ) : null}
      </div>
      <p className="text-text-muted mt-0.5 truncate text-[13px]">
        {pinned ? "Chosen" : "Automatic"}
        {description ? ` · ${description}` : ""}
      </p>

      <div
        className={cn(
          "mt-3 flex items-center gap-2 truncate rounded-xl px-3 py-2 font-mono text-xs",
          routed
            ? active?.encrypted
              ? "bg-accent-cyan-surface text-protocol-encrypted"
              : "bg-accent-surface text-accent-bright"
            : "bg-surface-inset text-text-secondary",
        )}
      >
        {routed && active?.encrypted ? (
          <Lock className="size-3.5 shrink-0" strokeWidth={2.4} />
        ) : (
          <ArrowRight className="size-3.5 shrink-0" strokeWidth={2.4} />
        )}
        <span className="truncate">
          DNS {routed ? active?.addrs[0] : (networkDns ?? "automatic")}
        </span>
      </div>
    </Tile>
  );
}
