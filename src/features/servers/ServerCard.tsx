import { ExternalLink, Lock, Star, Trash2 } from "lucide-react";
import { openUrl } from "@tauri-apps/plugin-opener";

import {
  Badge,
  CopyableValue,
  IconButton,
  LatencyBadge,
  Tooltip,
} from "@/design/primitives";
import { ProviderMark } from "@/features/servers/ProviderMark";
import { cn } from "@/lib/cn";
import type { Protocol, Server } from "@/lib/ipc";

interface ServerCardProps {
  server: Server;
  /** Which protocol tab is active, so its addresses are the ones shown. */
  focusProtocol: Protocol | null;
  selected?: boolean;
  favourite?: boolean;
  /** Measured latency for the focused protocol, if a probe has run. */
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
  favourite,
  latencyMs,
  latencyPending,
  onSelect,
  onToggleFavourite,
  onRemove,
}: ServerCardProps) {
  // Under a protocol tab, show that protocol's addresses; otherwise fall back
  // to the first endpoint, which is plain DNS for most providers.
  const shown =
    server.endpoints.find((e) => e.protocol === focusProtocol) ??
    server.endpoints[0];

  return (
    <article
      className={cn(
        "border-border-subtle bg-surface group rounded-2xl border transition-all duration-200",
        onSelect &&
          "hover:border-border-strong hover:bg-surface-hover cursor-pointer",
        selected &&
          "border-accent bg-accent-surface/40 shadow-[0_4px_20px_-10px_var(--accent-glow)]",
      )}
      onClick={onSelect ? () => onSelect(server) : undefined}
    >
      <div className="flex items-start gap-3 p-3">
        <ProviderMark provider={server.provider} name={server.name} />

        <div className="min-w-0 flex-1">
          <div className="flex items-start justify-between gap-3">
            <div className="min-w-0">
              <h3 className="truncate text-sm font-medium">
                {server.name}
                {server.variant ? (
                  <span className="text-text-muted font-normal">
                    {" "}
                    · {server.variant}
                  </span>
                ) : null}
              </h3>

              {shown ? (
                <div className="mt-1 flex flex-wrap items-center gap-x-3 gap-y-1">
                  {shown.addrs.map((addr) => (
                    <CopyableValue key={addr} value={addr} />
                  ))}
                </div>
              ) : null}
            </div>

            <div className="flex shrink-0 items-center gap-1">
              <LatencyBadge ms={latencyMs} pending={latencyPending} />

              {onToggleFavourite ? (
                <Tooltip content={favourite ? "Remove favourite" : "Favourite"}>
                  <IconButton
                    label={
                      favourite
                        ? `Unfavourite ${server.name}`
                        : `Favourite ${server.name}`
                    }
                    size="sm"
                    onClick={(event) => {
                      // The card is clickable; starring must not also select.
                      event.stopPropagation();
                      onToggleFavourite(server.id);
                    }}
                    className={cn(
                      "transition-opacity",
                      favourite
                        ? "text-accent"
                        : "text-text-faint opacity-0 group-hover:opacity-100 focus-visible:opacity-100",
                    )}
                  >
                    <Star className={cn(favourite && "fill-current")} />
                  </IconButton>
                </Tooltip>
              ) : null}

              {server.homepage ? (
                <Tooltip content="Open provider site">
                  <IconButton
                    label={`Open ${server.name} website`}
                    size="sm"
                    className="opacity-0 transition-opacity group-hover:opacity-100 focus-visible:opacity-100"
                    onClick={(event) => {
                      event.stopPropagation();
                      void openUrl(server.homepage!);
                    }}
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
                    variant="ghost"
                    onClick={(event) => {
                      event.stopPropagation();
                      onRemove(server.id);
                    }}
                    className="text-text-faint hover:text-danger opacity-0 transition-opacity group-hover:opacity-100 focus-visible:opacity-100"
                  >
                    <Trash2 />
                  </IconButton>
                </Tooltip>
              ) : null}
            </div>
          </div>

          <div className="mt-2 flex flex-wrap items-center gap-1.5">
            {server.endpoints.map((endpoint) => (
              <Badge
                key={endpoint.protocol}
                tone={endpoint.encrypted ? "encrypted" : "neutral"}
                mono
                className={cn(
                  focusProtocol &&
                    endpoint.protocol !== focusProtocol &&
                    "opacity-45",
                )}
              >
                {endpoint.encrypted ? <Lock className="size-2.5" /> : null}
                {endpoint.label}
              </Badge>
            ))}

            {server.tags.length > 0 ? (
              <span className="text-text-faint ml-1 truncate text-xs">
                {server.tags.join(" · ")}
              </span>
            ) : null}
          </div>
        </div>
      </div>
    </article>
  );
}
