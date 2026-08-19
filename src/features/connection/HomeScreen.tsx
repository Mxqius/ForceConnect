import {
  Activity,
  Eraser,
  Loader2,
  Lock,
  Power,
  RefreshCw,
  RotateCcw,
  ShieldCheck,
  TriangleAlert,
  Unlock,
  Wrench,
} from "lucide-react";
import { useMemo, type ReactNode } from "react";

import {
  Badge,
  Button,
  Collapsible,
  CollapsiblePanel,
  CollapsibleTrigger,
  CopyableValue,
  IconButton,
  Select,
  Skeleton,
  StatusDot,
  Tooltip,
  toast,
  type SelectOption,
} from "@/design/primitives";
import {
  useConnection,
  useServerSelection,
} from "@/features/connection/useConnection";
import { useDnsTools } from "@/features/connection/useDnsTools";
import { usePing } from "@/features/connection/usePing";
import { ProviderMark } from "@/features/servers/ProviderMark";
import { useFavourites, useServers } from "@/features/servers/useServers";
import { cn } from "@/lib/cn";
import { classifyLatency, formatLatency } from "@/lib/latency";
import type { Protocol, Server } from "@/lib/ipc";

const TONE_TEXT = {
  success: "text-success",
  warning: "text-warning",
  danger: "text-danger",
  neutral: "text-text-faint",
} as const;

/**
 * Mutation callbacks that announce the outcome.
 *
 * Every action on this screen can fail for reasons the user needs the text of —
 * an adapter that vanished between listing and writing, a VPN that took the
 * interface down — so none of them is allowed to fail silently.
 */
function report(success: string, failure: string) {
  return {
    onSuccess: () => toast.success(success),
    onError: (err: unknown) =>
      toast.error(failure, {
        description: err instanceof Error ? err.message : String(err),
      }),
  };
}

export function HomeScreen() {
  const { data: servers, isPending } = useServers();
  const { favourites } = useFavourites();
  const {
    active,
    adapter,
    target,
    connect,
    disconnect,
    isConnecting,
    isDisconnecting,
  } = useConnection();

  // Held in a store rather than in this component: the router unmounts the
  // screen on every tab change, and component state took the whole selection
  // with it — the picker came back empty and every field showed an em dash.
  const { selectedId, protocol, selectServer, selectProtocol } =
    useServerSelection();

  // Favourites float to the top of the picker for the same reason they do in
  // the browser: they are the entries this user actually returns to.
  const options: SelectOption<string>[] = useMemo(() => {
    if (!servers) return [];

    const rank = (server: Server) => (favourites.includes(server.id) ? 0 : 1);
    return [...servers]
      .sort((a, b) => rank(a) - rank(b))
      .map((server) => ({
        value: server.id,
        label: server.displayName,
        hint: server.endpoints.map((e) => e.label).join(" · "),
      }));
  }, [servers, favourites]);

  const selected = servers?.find((s) => s.id === selectedId) ?? null;

  const protocolOptions: SelectOption<string>[] = useMemo(
    () =>
      selected?.endpoints.map((endpoint) => ({
        value: endpoint.protocol,
        label: endpoint.label,
        hint: endpoint.encrypted ? "Encrypted" : "Unencrypted",
      })) ?? [],
    [selected],
  );

  const chosenProtocol = protocol ?? selected?.protocols[0] ?? null;
  const endpoint = selected?.endpoints.find(
    (e) => e.protocol === chosenProtocol,
  );

  const connected = active !== null;
  const busy = isConnecting || isDisconnecting;

  // Probed for whatever is selected, not only for what is connected — seeing
  // the latency *before* committing to a server is the point of having it.
  const ping = usePing(selected?.id ?? null, chosenProtocol);

  function handleConnect() {
    if (!selected || !chosenProtocol) return;

    connect(
      { server: selected, protocol: chosenProtocol },
      report(`${selected.displayName} connected`, "Could not connect"),
    );
  }

  function handleDisconnect() {
    disconnect(undefined, report("Disconnected", "Could not disconnect"));
  }

  const caption = isConnecting
    ? "Connecting…"
    : isDisconnecting
      ? "Disconnecting…"
      : connected
        ? "Click to disconnect"
        : selected
          ? "Click to connect"
          : "Choose a server";

  return (
    <div className="mx-auto flex h-full w-full max-w-4xl flex-col justify-center gap-8 p-8">
      <div className="grid items-stretch gap-10 md:grid-cols-[minmax(0,0.85fr)_minmax(0,1.15fr)]">
        {/* Left: the one control that matters, at the size that says so. */}
        <section className="flex flex-col items-center justify-center gap-6">
          <PowerButton
            connected={connected}
            busy={busy}
            disabled={!selected && !connected}
            onClick={connected ? handleDisconnect : handleConnect}
          />

          <div className="flex flex-col items-center gap-2.5">
            <p className="text-text-secondary text-xs font-semibold tracking-[0.18em] uppercase">
              {caption}
            </p>

            <LatencyReadout
              ms={ping.ms}
              probing={ping.isProbing}
              disabled={!selected}
              onRefresh={ping.refresh}
            />
          </div>
        </section>

        {/* Right: everything that describes or adjusts that control. */}
        <section className="flex min-w-0 flex-col gap-4">
          {isPending ? (
            <Skeleton className="h-11 w-full rounded-xl" />
          ) : (
            <Select
              label="DNS server"
              value={selectedId}
              onValueChange={selectServer}
              options={options}
              placeholder="Choose a DNS server…"
              disabled={connected || busy}
            />
          )}

          <div className="flex items-stretch gap-2.5">
            <div className="border-border-subtle bg-surface flex min-w-0 flex-1 items-center gap-3.5 rounded-xl border px-4 py-3">
              {selected ? (
                <>
                  <ProviderMark
                    provider={selected.provider}
                    name={selected.displayName}
                    className="size-10 rounded-lg"
                  />
                  <div className="flex min-w-0 flex-col">
                    <span className="truncate text-sm font-semibold">
                      {selected.displayName}
                    </span>
                    {endpoint ? (
                      <span className="text-text-muted truncate text-xs">
                        {endpoint.encrypted ? "Encrypted" : "Plain"} ·{" "}
                        {endpoint.label}
                      </span>
                    ) : null}
                  </div>
                </>
              ) : (
                <span className="text-text-faint text-sm">
                  No server selected
                </span>
              )}
            </div>

            <Tooltip content="Refresh ping">
              <IconButton
                label="Refresh ping"
                variant="secondary"
                size="lg"
                disabled={!selected || ping.isProbing}
                onClick={ping.refresh}
                // Square is the wrong shape next to a two-line card: matching
                // the row's height is what makes them read as one control bar.
                className="h-auto w-14 shrink-0"
              >
                <RefreshCw className={cn(ping.isProbing && "animate-spin")} />
              </IconButton>
            </Tooltip>

            {protocolOptions.length > 1 ? (
              <Select
                label="Protocol"
                value={chosenProtocol}
                onValueChange={(value) =>
                  selectProtocol(value as Protocol | null)
                }
                options={protocolOptions}
                disabled={connected || busy}
                className="h-auto w-28 shrink-0"
              />
            ) : null}
          </div>

          <div className="border-border-subtle bg-surface grid grid-cols-2 gap-x-6 gap-y-4 rounded-xl border px-5 py-4">
            <Stat label="Address">
              {endpoint ? (
                <div className="flex flex-wrap items-center gap-x-2 gap-y-1">
                  {endpoint.addrs.map((addr) => (
                    <CopyableValue key={addr} value={addr} />
                  ))}
                </div>
              ) : (
                <span className="text-text-faint text-xs">—</span>
              )}
            </Stat>

            <Stat label="Status">
              <Badge tone={connected ? "success" : "danger"}>
                <StatusDot
                  tone={connected ? "success" : "danger"}
                  pulse={connected}
                />
                {connected ? "Connected" : "Disconnected"}
              </Badge>
            </Stat>

            <Stat label="Protocol">
              {endpoint ? (
                <Badge tone={endpoint.encrypted ? "encrypted" : "neutral"} mono>
                  {endpoint.encrypted ? (
                    <Lock className="size-2.5" />
                  ) : (
                    <Unlock className="size-2.5" />
                  )}
                  {endpoint.label}
                </Badge>
              ) : (
                <span className="text-text-faint text-xs">—</span>
              )}
            </Stat>

            <Stat label="Adapter">
              <span className="flex min-w-0 items-center gap-1.5">
                <span className="truncate text-xs">
                  {adapter?.name ?? "Looking for an adapter…"}
                </span>
                {target.unavailableReason ? (
                  <Tooltip
                    content={
                      target.unavailableReason === "missing"
                        ? "The interface you picked on the Interfaces tab is gone, so DNS is going here instead."
                        : "The interface you picked on the Interfaces tab is not usable right now, so DNS is going here instead."
                    }
                  >
                    <TriangleAlert className="text-warning size-3.5 shrink-0" />
                  </Tooltip>
                ) : null}
              </span>
            </Stat>
          </div>

          <Collapsible className="border-border-subtle border-t pt-3">
            <CollapsibleTrigger startIcon={<Wrench className="size-3.5" />}>
              Maintenance
            </CollapsibleTrigger>
            <CollapsiblePanel>
              <div className="pt-3">
                <DnsToolbar currentServers={endpoint?.addrs ?? []} />
              </div>
            </CollapsiblePanel>
          </Collapsible>
        </section>
      </div>
    </div>
  );
}

function Stat({ label, children }: { label: string; children: ReactNode }) {
  return (
    <div className="flex min-w-0 flex-col gap-1.5">
      <span className="text-text-faint text-2xs font-semibold tracking-widest uppercase">
        {label}
      </span>
      <div className="text-text-secondary flex min-w-0 items-center text-sm">
        {children}
      </div>
    </div>
  );
}

interface PowerButtonProps {
  connected: boolean;
  busy: boolean;
  disabled: boolean;
  onClick: () => void;
}

/**
 * The connect control.
 *
 * A button rather than a decorative ring with a separate action underneath:
 * the ring is the largest thing on the screen and already reads as pressable,
 * so anything else would be a target people miss on purpose.
 */
function PowerButton({ connected, busy, disabled, onClick }: PowerButtonProps) {
  return (
    <button
      type="button"
      onClick={onClick}
      disabled={disabled || busy}
      aria-label={connected ? "Disconnect" : "Connect"}
      aria-pressed={connected}
      className={cn(
        "group relative flex size-48 items-center justify-center rounded-full transition-transform duration-200",
        "focus-visible:outline-accent outline-none focus-visible:outline-2 focus-visible:outline-offset-4",
        "disabled:cursor-not-allowed disabled:opacity-45",
        !disabled && !busy && "hover:scale-[1.02] active:scale-[0.98]",
      )}
    >
      {/* Outer track: the only always-present ring, so the control keeps its
          footprint whether or not anything is connected. */}
      <span
        className={cn(
          "absolute inset-0 rounded-full border transition-colors duration-500",
          connected ? "border-accent/35" : "border-border-subtle",
        )}
      />

      <span
        className={cn(
          "absolute inset-4 rounded-full border-2 transition-all duration-500",
          "group-hover:not-disabled:border-border-strong",
          connected
            ? "border-accent bg-accent-surface group-hover:not-disabled:border-accent shadow-[0_0_48px_-8px_var(--accent-glow)]"
            : "border-border-default bg-surface-subtle",
        )}
      />

      <span
        className={cn(
          "relative transition-colors duration-500",
          connected ? "text-accent-bright" : "text-text-faint",
        )}
      >
        {busy ? (
          <Loader2 className="size-16 animate-spin" strokeWidth={1.5} />
        ) : connected ? (
          <ShieldCheck className="size-16" strokeWidth={1.5} />
        ) : (
          <Power className="size-16" strokeWidth={1.5} />
        )}
      </span>
    </button>
  );
}

interface LatencyReadoutProps {
  /** `undefined` before the first probe, `null` when nothing answered. */
  ms: number | null | undefined;
  probing: boolean;
  disabled: boolean;
  onRefresh: () => void;
}

function LatencyReadout({
  ms,
  probing,
  disabled,
  onRefresh,
}: LatencyReadoutProps) {
  const tone = probing ? "neutral" : classifyLatency(ms);

  return (
    <button
      type="button"
      onClick={onRefresh}
      disabled={disabled || probing}
      title="Refresh ping"
      className={cn(
        "text-text-muted inline-flex items-center gap-2 rounded-lg px-3 py-1.5 text-xs transition-colors",
        "hover:not-disabled:bg-surface-hover hover:not-disabled:text-text-secondary",
        "focus-visible:outline-accent outline-none focus-visible:outline-2 focus-visible:outline-offset-2",
        "disabled:opacity-60",
      )}
    >
      <Activity className={cn("size-4", TONE_TEXT[tone])} />
      <span>Ping</span>
      <span className={cn("font-mono tabular-nums", TONE_TEXT[tone])}>
        {probing ? "···" : ms === null ? "no answer" : formatLatency(ms)}
      </span>
    </button>
  );
}

/**
 * Maintenance actions, kept on this screen because this is where a user
 * notices they are needed — a page still resolving to the old server is the
 * symptom, and the cache flush is the fix.
 */
function DnsToolbar({ currentServers }: { currentServers: string[] }) {
  const { flush, reset, reapply, isBusy } = useDnsTools();

  return (
    <div className="flex flex-wrap items-center gap-2.5">
      <Button
        size="sm"
        startIcon={<Eraser />}
        disabled={isBusy}
        onClick={() =>
          flush.mutate(
            undefined,
            report("DNS cache flushed", "Could not flush the DNS cache"),
          )
        }
      >
        {flush.isPending ? "Flushing…" : "Flush cache"}
      </Button>

      <Button
        size="sm"
        startIcon={<RotateCcw />}
        disabled={isBusy}
        onClick={() =>
          reset.mutate(
            undefined,
            report("Adapter returned to automatic DNS", "Could not reset DNS"),
          )
        }
      >
        {reset.isPending ? "Resetting…" : "Reset to automatic"}
      </Button>

      <Tooltip content="Write the selected servers back onto the adapter — useful after a VPN replaces them">
        <Button
          size="sm"
          startIcon={<RefreshCw />}
          disabled={isBusy || currentServers.length === 0}
          onClick={() =>
            reapply.mutate(
              currentServers,
              report("DNS re-applied", "Could not re-apply DNS"),
            )
          }
        >
          {reapply.isPending ? "Applying…" : "Re-apply"}
        </Button>
      </Tooltip>
    </div>
  );
}
