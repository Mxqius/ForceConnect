import { ExternalLink, Lock } from "lucide-react";
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
  onSelect?: (server: Server) => void;
}

export function ServerCard({
  server,
  focusProtocol,
  selected,
  onSelect,
}: ServerCardProps) {
  // Under a protocol tab, show that protocol's addresses; otherwise fall back
  // to the first endpoint, which is plain DNS for most providers.
  const shown =
    server.endpoints.find((e) => e.protocol === focusProtocol) ??
    server.endpoints[0];

  return (
    <article
      className={cn(
        "border-border-subtle bg-surface group rounded-lg border transition-colors",
        onSelect && "hover:border-border-strong cursor-pointer",
        selected && "border-accent bg-accent-surface/30",
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
              <LatencyBadge />
              {server.homepage ? (
                <Tooltip content="Open provider site">
                  <IconButton
                    label={`Open ${server.name} website`}
                    size="sm"
                    className="opacity-0 transition-opacity group-hover:opacity-100 focus-visible:opacity-100"
                    onClick={(event) => {
                      // The card itself is clickable; opening a browser must
                      // not also select the server.
                      event.stopPropagation();
                      void openUrl(server.homepage!);
                    }}
                  >
                    <ExternalLink />
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
