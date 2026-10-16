import { Select } from "@base-ui/react/select";
import { Check, ChevronsUpDown, Lock, Star } from "lucide-react";
import { useMemo, useState } from "react";

import { focusRing, overlayPositioner, overlaySurface } from "@/design/styles";
import { ProviderMark } from "@/features/servers/ProviderMark";
import {
  humanTag,
  preferredProtocol,
  sortProtocols,
} from "@/features/servers/filter";
import {
  useLatencies,
  type LatencyReading,
} from "@/features/servers/useLatencies";
import { cn } from "@/lib/cn";
import {
  classifyLatency,
  formatLatency,
  type LatencyTone,
} from "@/lib/latency";
import type { Server } from "@/lib/ipc";

const TONE_TEXT: Record<LatencyTone, string> = {
  success: "text-success",
  warning: "text-warning",
  danger: "text-danger",
  neutral: "text-text-muted",
};

interface ServerSelectProps {
  servers: Server[];
  favourites: string[];
  value: Server | null;
  /** The server DNS is going to right now, marked in the list. */
  liveId: string | null;
  /** The trigger's second line; defaults to the preferred endpoint's address. */
  detail?: string;
  disabled?: boolean;
  onChange: (server: Server) => void;
}

/**
 * The server picker: a select, but one whose rows carry what you choose by.
 *
 * Each option shows its protocols, its first tags, and a measured response
 * time, so comparing servers does not mean visiting another screen. Opening
 * the list is what triggers the measurements — a probe is real DNS traffic,
 * so nothing is sent for a list nobody looked at.
 */
export function ServerSelect({
  servers,
  favourites,
  value,
  liveId,
  detail,
  disabled,
  onChange,
}: ServerSelectProps) {
  const [open, setOpen] = useState(false);

  // Favourites first, then catalogue order. Never sorted by speed: rows that
  // reshuffled as each probe landed would move the one under the pointer.
  const groups = useMemo(() => {
    const starred = servers.filter((s) => favourites.includes(s.id));
    const rest = servers.filter((s) => !favourites.includes(s.id));
    return [
      { label: "Favourites", items: starred },
      { label: starred.length > 0 ? "All servers" : null, items: rest },
    ].filter((group) => group.items.length > 0);
  }, [servers, favourites]);

  const latencies = useLatencies(
    servers.map((server) => ({ server, protocol: preferredProtocol(server) })),
    { probe: open },
  );

  return (
    <Select.Root
      value={value?.id ?? null}
      onValueChange={(id) => {
        const next = servers.find((s) => s.id === id);
        if (next) onChange(next);
      }}
      open={open}
      onOpenChange={setOpen}
      disabled={disabled}
      items={servers.map((s) => ({ value: s.id, label: s.displayName }))}
    >
      <Select.Trigger
        aria-label="DNS server"
        className={cn(
          "group border-border-default bg-surface-inset flex h-[68px] w-full items-center gap-3.5 rounded-2xl border px-3.5 text-left transition-colors",
          "hover:border-border-strong data-[popup-open]:border-accent",
          "disabled:cursor-not-allowed disabled:opacity-55",
          focusRing,
        )}
      >
        {value ? (
          <>
            <ProviderMark
              provider={value.provider}
              name={value.name}
              className="size-10 rounded-xl text-[15px]"
            />
            <span className="min-w-0 flex-1">
              <span className="flex items-center gap-1.5">
                <b className="truncate text-[15px] font-semibold">
                  {value.name}
                </b>
                {favourites.includes(value.id) ? (
                  <Star className="text-warning size-3.5 shrink-0 fill-current" />
                ) : null}
              </span>
              <span className="text-text-muted mt-0.5 block truncate text-[13px]">
                {detail ?? describe(value)}
              </span>
            </span>
          </>
        ) : (
          <span className="text-text-muted flex-1 text-sm">
            Choose a DNS server…
          </span>
        )}
        <ChevronsUpDown className="text-text-muted group-hover:text-text-secondary size-4 shrink-0" />
      </Select.Trigger>

      <Select.Portal>
        <Select.Positioner
          className={overlayPositioner}
          sideOffset={6}
          alignItemWithTrigger={false}
        >
          <Select.Popup
            className={cn(
              overlaySurface,
              "fc-scroll max-h-[360px] w-[var(--anchor-width)] overflow-y-auto rounded-2xl p-1.5",
            )}
          >
            <Select.List>
              {groups.map((group) => (
                <Select.Group key={group.label ?? "all"} className="pb-1">
                  {group.label ? (
                    <Select.GroupLabel className="text-text-muted flex items-center gap-1.5 px-3 pt-2 pb-1.5 text-[11px] font-bold tracking-[0.12em] uppercase">
                      {group.label === "Favourites" ? (
                        <Star className="size-3 fill-current" />
                      ) : null}
                      {group.label}
                    </Select.GroupLabel>
                  ) : null}
                  {group.items.map((server) => (
                    <Option
                      key={server.id}
                      server={server}
                      live={server.id === liveId}
                      reading={latencies.get(server.id)}
                    />
                  ))}
                </Select.Group>
              ))}
            </Select.List>
          </Select.Popup>
        </Select.Positioner>
      </Select.Portal>
    </Select.Root>
  );
}

function Option({
  server,
  live,
  reading,
}: {
  server: Server;
  live: boolean;
  reading: LatencyReading | undefined;
}) {
  const tone = classifyLatency(reading?.ms);
  const usable = server.endpoints.some((e) => !e.requiresForwarder);

  return (
    <Select.Item
      value={server.id}
      className={cn(
        "group flex cursor-pointer items-center gap-3 rounded-xl px-2.5 py-2 outline-none",
        "data-[highlighted]:bg-surface-hover data-[selected]:bg-accent-surface",
      )}
    >
      <ProviderMark
        provider={server.provider}
        name={server.name}
        className="size-9 rounded-[11px] text-sm"
      />

      <span className="min-w-0 flex-1">
        <span className="flex items-center gap-2">
          <Select.ItemText className="truncate text-sm font-semibold">
            {server.name}
          </Select.ItemText>
          {server.variant ? (
            <span className="text-text-muted truncate text-[13px]">
              {server.variant}
            </span>
          ) : null}
          {live ? (
            <span className="bg-success-surface text-success shrink-0 rounded-full px-1.5 py-px text-[10.5px] font-bold">
              In use
            </span>
          ) : null}
        </span>

        <span className="mt-1 flex items-center gap-1">
          {sortProtocols(server.protocols).map((protocol) => {
            const endpoint = server.endpoints.find(
              (e) => e.protocol === protocol,
            );
            if (!endpoint) return null;
            return (
              <span
                key={protocol}
                className={cn(
                  "flex items-center gap-0.5 rounded-md px-1.5 py-px text-[10.5px] font-bold",
                  endpoint.encrypted
                    ? "bg-accent-cyan-surface text-protocol-encrypted"
                    : "bg-surface-active text-text-secondary",
                  endpoint.requiresForwarder && "opacity-60",
                )}
              >
                {endpoint.encrypted ? <Lock className="size-2.5" /> : null}
                {endpoint.label}
              </span>
            );
          })}
          <span className="text-text-muted ml-1 truncate text-xs">
            {usable
              ? server.tags.slice(0, 2).map(humanTag).join(" · ")
              : "Encrypted only · not supported yet"}
          </span>
        </span>
      </span>

      <span
        className={cn(
          "w-16 shrink-0 text-right font-mono text-[13px] font-semibold tabular-nums",
          TONE_TEXT[tone],
        )}
      >
        {reading?.probing && reading.ms === undefined
          ? "···"
          : formatLatency(reading?.ms)}
      </span>

      <span className="flex w-4 shrink-0 justify-center">
        <Select.ItemIndicator className="text-accent-bright flex">
          <Check className="size-4" strokeWidth={3} />
        </Select.ItemIndicator>
      </span>
    </Select.Item>
  );
}

/** The trigger's second line: the address you would actually be using. */
function describe(server: Server): string {
  const protocol = preferredProtocol(server);
  const endpoint = server.endpoints.find((e) => e.protocol === protocol);
  const parts = [
    server.variant,
    endpoint && !endpoint.requiresForwarder ? endpoint.addrs[0] : null,
  ].filter(Boolean);
  return parts.length > 0
    ? parts.join(" · ")
    : "Encrypted only · not supported yet";
}
