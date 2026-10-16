import { ExternalLink, Lock, Star, Trash2 } from "lucide-react";
import { openUrl } from "@tauri-apps/plugin-opener";
import type { KeyboardEvent, MouseEvent } from "react";

import { IconButton, Tooltip } from "@/design/primitives";
import { ProviderMark } from "@/features/servers/ProviderMark";
import { cn } from "@/lib/cn";
import { classifyLatency, type LatencyTone } from "@/lib/latency";
import type { Protocol, Server } from "@/lib/ipc";
import { humanTag, sortProtocols } from "@/features/servers/filter";

interface ServerCardProps {
  server: Server;
  /** Which protocol filter is active, so its address is the one shown. */
  focusProtocol: Protocol | null;
  /** The server the connection screen has picked. */
  selected?: boolean;
  /** The server DNS is going to right now. */
  inUse?: boolean;
  favourite?: boolean;
  latencyMs?: number | null;
  latencyPending?: boolean;
  onSelect?: (server: Server) => void;
  onToggleFavourite?: (id: string) => void;
  onRemove?: (id: string) => void;
}

export function ServerCard({
  server,
  focusProtocol,
  selected,
  inUse,
  favourite,
  latencyMs,
  latencyPending,
  onSelect,
  onToggleFavourite,
  onRemove,
}: ServerCardProps) {
  const shown =
    server.endpoints.find((e) => e.protocol === focusProtocol) ??
    server.endpoints.find((e) => !e.requiresForwarder) ??
    server.endpoints[0];

  // The card is the control, but it contains buttons of its own, and a
  // button cannot contain a button — so it is a focusable element with button
  // semantics, and the inner controls stop their clicks from reaching it.
  function activate() {
    onSelect?.(server);
  }

  function onKeyDown(event: KeyboardEvent<HTMLElement>) {
    if (event.target !== event.currentTarget) return;
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      activate();
    }
  }

  const stop = (handler: () => void) => (event: MouseEvent) => {
    event.stopPropagation();
    handler();
  };

  return (
    <article
      role={onSelect ? "button" : undefined}
      tabIndex={onSelect ? 0 : undefined}
      aria-pressed={onSelect ? selected : undefined}
      onClick={onSelect ? activate : undefined}
      onKeyDown={onSelect ? onKeyDown : undefined}
      className={cn(
        "group bg-surface shadow-tile relative flex min-h-[172px] flex-col rounded-[22px] border-[1.5px] px-5 pt-4 pb-4 transition-[border-color,transform] duration-150",
        "focus-visible:outline-accent outline-none focus-visible:outline-2 focus-visible:outline-offset-2",
        onSelect && "cursor-pointer hover:-translate-y-0.5",
        inUse
          ? "border-success"
          : selected
            ? "border-accent"
            : "border-border-subtle hover:border-border-strong",
      )}
    >
      <div className="flex items-start justify-between">
        <ProviderMark
          provider={server.provider}
          name={server.name}
          className="size-11 rounded-[14px] text-base"
        />

        {inUse ? (
          <span className="bg-success-surface text-success absolute top-4 left-[76px] flex items-center gap-1.5 rounded-full px-2.5 py-0.5 text-[11px] font-bold">
            <span className="bg-success size-1.5 rounded-full" />
            In use
          </span>
        ) : null}

        <LatencyGauge ms={latencyMs} pending={latencyPending} />
      </div>

      <div className="mt-3 flex min-w-0 items-center gap-1">
        <h3 className="truncate text-[15px] font-semibold">{server.name}</h3>
        {onToggleFavourite ? (
          <Tooltip content={favourite ? "Remove favourite" : "Favourite"}>
            <IconButton
              label={
                favourite
                  ? `Unfavourite ${server.name}`
                  : `Favourite ${server.name}`
              }
              size="sm"
              onClick={stop(() => onToggleFavourite(server.id))}
              className={cn(
                "size-6 shrink-0 rounded-md [&_svg]:size-3.5",
                favourite
                  ? "text-warning"
                  : "text-text-faint opacity-0 group-hover:opacity-100 focus-visible:opacity-100",
              )}
            >
              <Star className={cn(favourite && "fill-current")} />
            </IconButton>
          </Tooltip>
        ) : null}
      </div>

      <p className="text-text-muted truncate text-[13px]">
        {[server.variant, ...server.tags.slice(0, 2).map(humanTag)]
          .filter(Boolean)
          .join(" · ")}
      </p>
      {shown ? (
        <p
          data-selectable
          className="text-text-muted mt-0.5 truncate font-mono text-[13px]"
          title={shown.addrs.join(", ")}
        >
          {shown.addrs[0]}
        </p>
      ) : null}

      <div className="mt-auto flex items-end gap-1 pt-3">
        <div className="flex min-w-0 flex-1 flex-wrap gap-1">
          {sortProtocols(server.protocols).map((protocol) => {
            const endpoint = server.endpoints.find(
              (e) => e.protocol === protocol,
            );
            if (!endpoint) return null;
            return (
              <span
                key={protocol}
                className={cn(
                  "flex items-center gap-1 rounded-md px-2 py-0.5 text-[11px] font-bold",
                  endpoint.encrypted
                    ? "bg-accent-cyan-surface text-protocol-encrypted"
                    : "bg-surface-inset text-text-muted",
                  focusProtocol && protocol !== focusProtocol && "opacity-45",
                )}
              >
                {endpoint.encrypted ? <Lock className="size-2.5" /> : null}
                {endpoint.label}
              </span>
            );
          })}
        </div>

        <div className="-mr-1 -mb-1 flex shrink-0 opacity-0 transition-opacity group-focus-within:opacity-100 group-hover:opacity-100">
          {server.homepage ? (
            <Tooltip content="Open provider site">
              <IconButton
                label={`Open ${server.name} website`}
                size="sm"
                onClick={stop(() => void openUrl(server.homepage!))}
                className="text-text-muted size-7 rounded-lg [&_svg]:size-3.5"
              >
                <ExternalLink />
              </IconButton>
            </Tooltip>
          ) : null}

          {onRemove && server.source === "custom" ? (
            <Tooltip content="Remove this server">
              <IconButton
                label={`Remove ${server.name}`}
                size="sm"
                onClick={stop(() => onRemove(server.id))}
                className="text-text-muted hover:text-danger size-7 rounded-lg [&_svg]:size-3.5"
              >
                <Trash2 />
              </IconButton>
            </Tooltip>
          ) : null}
        </div>
      </div>
    </article>
  );
}

const GAUGE_STROKE: Record<LatencyTone, string> = {
  success: "stroke-success",
  warning: "stroke-warning",
  danger: "stroke-danger",
  neutral: "stroke-text-faint",
};

const GAUGE_TEXT: Record<LatencyTone, string> = {
  success: "text-success",
  warning: "text-warning",
  danger: "text-danger",
  neutral: "text-text-muted",
};

/**
 * Latency as a ring: fuller is faster.
 *
 * Reads at a glance across a grid of cards in a way a column of numbers does
 * not — the eye finds the full green rings before it reads any figure.
 */
function LatencyGauge({
  ms,
  pending,
}: {
  ms: number | null | undefined;
  pending?: boolean;
}) {
  const radius = 19;
  const length = 2 * Math.PI * radius;
  const tone = classifyLatency(ms);
  const measured = typeof ms === "number";
  const fill = measured ? Math.max(0.08, 1 - Math.min(ms, 300) / 300) : 0;

  const label = pending
    ? "Measuring latency"
    : measured
      ? `${Math.round(ms)} milliseconds`
      : ms === null
        ? "No answer"
        : "Not measured yet";

  return (
    <div className="relative size-12 shrink-0" role="img" aria-label={label}>
      <svg viewBox="0 0 44 44" className="size-full -rotate-90">
        <circle
          cx="22"
          cy="22"
          r={radius}
          fill="none"
          strokeWidth="4"
          className="stroke-surface-active"
          strokeDasharray={measured || pending ? undefined : "3 4"}
        />
        {pending ? (
          <circle
            cx="22"
            cy="22"
            r={radius}
            fill="none"
            strokeWidth="4"
            strokeLinecap="round"
            strokeDasharray={`${length * 0.25} ${length}`}
            className="stroke-accent-bright origin-center animate-spin"
          />
        ) : measured ? (
          <circle
            cx="22"
            cy="22"
            r={radius}
            fill="none"
            strokeWidth="4"
            strokeLinecap="round"
            strokeDasharray={`${length * fill} ${length}`}
            className={cn(
              "transition-[stroke-dasharray] duration-500",
              GAUGE_STROKE[tone],
            )}
          />
        ) : null}
      </svg>
      {!pending ? (
        <span
          className={cn(
            "absolute inset-0 flex flex-col items-center justify-center font-mono text-[11.5px] leading-none font-bold",
            GAUGE_TEXT[tone],
          )}
        >
          {measured ? Math.max(1, Math.round(ms)) : ms === null ? "×" : "—"}
          {measured ? (
            <small className="text-text-muted text-[8.5px] font-semibold">
              ms
            </small>
          ) : null}
        </span>
      ) : null}
    </div>
  );
}
