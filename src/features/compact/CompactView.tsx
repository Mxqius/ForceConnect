import { isTauri } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import {
  Activity,
  Lock,
  Maximize2,
  Minus,
  Pin,
  PinOff,
  Wifi,
  X,
} from "lucide-react";
import { useEffect, useRef, type ReactNode } from "react";

import logoIcon from "@/assets/brand/logo-icon.png";
import { Skeleton, Tooltip } from "@/design/primitives";
import { useLayoutPrefs } from "@/app/useLayoutPrefs";
import { PowerButton, Uptime } from "@/features/connection/ConnectionHero";
import { ServerSelect } from "@/features/connection/ServerSelect";
import { useConnectTarget } from "@/features/connection/useConnectTarget";
import {
  useConnection,
  type ConnectPhase,
} from "@/features/connection/useConnection";
import { usePing } from "@/features/connection/usePing";
import {
  onDefaultDns,
  useReleasePolicy,
} from "@/features/connection/useReleasePolicy";
import { useServerPicker } from "@/features/connection/useServerPicker";
import { cn } from "@/lib/cn";
import { classifyLatency, type LatencyTone } from "@/lib/latency";

import "@/features/connection/connection.css";

/**
 * How often the compact window re-measures latency.
 *
 * Six times slower than the full screen's live graph on purpose: there is no
 * graph here to feed, only a number, and every sample is four real DNS
 * queries. Keeping this window cheap is half of what it is for.
 */
const PING_INTERVAL_MS = 30_000;

const LIT: ReadonlySet<ConnectPhase> = new Set(["connected", "switching"]);

const TONE_TEXT: Record<LatencyTone, string> = {
  success: "text-success",
  warning: "text-warning",
  danger: "text-danger",
  neutral: "text-text-muted",
};

/**
 * The compact window: the connection and nothing else.
 *
 * Status, the power button, the server, and two facts worth glancing at —
 * latency and where DNS is going. Everything else lives one click away in the
 * full window. What is left out is also what costs the most to keep running:
 * the live latency graph, the shimmer and bloom animations, and the screens
 * that probe whole server lists.
 */
export function CompactView() {
  const { servers, isPending, favourites, selected, endpoint, blocker } =
    useConnectTarget();
  const {
    phase,
    active,
    adapter,
    failure,
    connectedAt,
    canCancel,
    busy,
    activeServerId,
    connect,
    disconnect,
    cancel,
  } = useConnection();
  const { pick } = useServerPicker();
  const releaseMode = useReleasePolicy((state) => state.mode);
  const defaultDns = useReleasePolicy((state) => state.defaultDns);

  const lit = LIT.has(phase);
  const working =
    phase === "connecting" ||
    phase === "switching" ||
    phase === "disconnecting";

  function handleConnect() {
    if (!selected || !endpoint || blocker) return;
    void connect(selected, endpoint.protocol);
  }

  const action: { label: string; run: () => void } | null =
    phase === "idle" || phase === "failed"
      ? blocker
        ? null
        : { label: "Connect", run: handleConnect }
      : phase === "connected"
        ? { label: "Disconnect", run: () => void disconnect() }
        : canCancel
          ? { label: "Cancel", run: cancel }
          : null;

  const onDefault = onDefaultDns(adapter?.dnsV4, releaseMode, defaultDns);

  const chip = {
    idle: onDefault ? "Default DNS" : "Network DNS",
    connecting: "Connecting",
    switching: "Switching",
    disconnecting: "Disconnecting",
    connected: active?.encrypted
      ? `Encrypted · ${active.protocolLabel}`
      : "Plain DNS",
    failed: failure?.step === "apply" ? "Not connected" : "Nothing changed",
  }[phase];

  const title = {
    idle: "Not connected",
    connecting: "Connecting…",
    switching: "Switching…",
    connected: "Connected",
    disconnecting: "Disconnecting…",
    failed: "Couldn’t connect",
  }[phase];

  const dot = {
    idle: "bg-text-muted",
    connecting: "bg-accent-bright animate-pulse",
    switching: "bg-white animate-pulse",
    disconnecting: "bg-text-muted animate-pulse",
    connected: active?.encrypted ? "bg-[#7dffb6]" : "bg-[#ffd56b]",
    failed: "bg-danger",
  }[phase];

  // When connected, the figures are about the live server, not whatever the
  // picker happens to show.
  const pingServerId = active?.serverId ?? selected?.id ?? null;
  const pingProtocol = active?.protocol ?? endpoint?.protocol ?? null;
  const routedDns = lit ? active?.addrs[0] : (adapter?.dnsV4[0] ?? "automatic");

  return (
    <div className="bg-surface-base text-text-primary flex h-full flex-col">
      <CompactTitleBar />

      <div className="flex min-h-0 flex-1 flex-col gap-2.5 px-3 pb-3">
        <section
          data-lit={lit}
          data-phase={phase}
          aria-label="Connection"
          // The bloom grows from the power button, which sits right of centre
          // here rather than in the corner as on the full tile.
          style={
            { "--fc-origin": "calc(100% - 62px) 50%" } as React.CSSProperties
          }
          className={cn(
            "fc-hero bg-surface border-border-subtle rounded-tile shadow-tile relative flex flex-1 flex-col overflow-hidden border p-5 transition-[background-color,border-color,color] duration-500",
            phase === "connecting" && "bg-hero-busy border-hero-busy-border",
            phase === "failed" && "bg-hero-failed border-hero-failed-border",
            lit && "border-transparent text-white",
          )}
        >
          <div className="fc-hero-glow" />

          <span
            className={cn(
              "relative inline-flex max-w-full items-center gap-2 self-start rounded-full py-1 pr-3 pl-2.5 text-xs font-semibold transition-colors duration-500",
              lit
                ? "bg-white/18 text-white"
                : "bg-surface-inset text-text-secondary",
            )}
          >
            <span className={cn("size-1.5 shrink-0 rounded-full", dot)} />
            <span className="truncate">{chip}</span>
          </span>

          <div className="relative mt-auto flex items-end justify-between gap-3">
            <div className="min-w-0 pb-1">
              <h1
                key={title}
                className="fc-rise text-[28px] leading-[1.05] font-bold tracking-[-0.03em]"
              >
                {title}
              </h1>
              <StatusLine
                phase={phase}
                since={connectedAt}
                serverName={active?.serverName ?? selected?.name ?? null}
                protocolLabel={endpoint?.label ?? null}
                blocker={blocker}
                failure={failure?.message ?? null}
              />
            </div>

            <PowerButton
              compact
              phase={phase}
              action={action}
              spinning={working}
              // No ripples here: a small window is somewhere to glance, not
              // somewhere to celebrate, and the animation is the cost.
              celebrate={null}
              cancelLocked={phase === "connecting" && !canCancel}
            />
          </div>
        </section>

        {isPending || !servers ? (
          <Skeleton className="h-[68px] rounded-2xl" />
        ) : (
          <ServerSelect
            servers={servers}
            favourites={favourites}
            value={selected}
            liveId={lit ? activeServerId : null}
            detail={
              endpoint && !endpoint.requiresForwarder
                ? `${endpoint.label} · ${endpoint.addrs[0] ?? ""}`
                : undefined
            }
            disabled={busy}
            onChange={(server) => pick(server)}
          />
        )}

        <div className="grid grid-cols-2 gap-2.5">
          <LatencyFact serverId={pingServerId} protocol={pingProtocol} />
          <Fact
            icon={
              lit && active?.encrypted ? (
                <Lock strokeWidth={2.4} />
              ) : (
                <Wifi strokeWidth={2.4} />
              )
            }
            label={adapter?.name ?? "No adapter"}
          >
            <span
              className={cn(
                "truncate font-mono text-[13px] font-semibold",
                lit ? "text-accent-bright" : "text-text-secondary",
              )}
            >
              {routedDns}
            </span>
          </Fact>
        </div>
      </div>
    </div>
  );
}

function StatusLine({
  phase,
  since,
  serverName,
  protocolLabel,
  blocker,
  failure,
}: {
  phase: ConnectPhase;
  since: number | null;
  serverName: string | null;
  protocolLabel: string | null;
  blocker: string | null;
  failure: string | null;
}) {
  if (phase === "connected" || phase === "switching") {
    return (
      <div className="mt-1 flex min-w-0 items-baseline gap-2">
        <Uptime since={since} className="mt-0 text-base" />
        <span className="truncate text-[13px] opacity-75">{serverName}</span>
      </div>
    );
  }

  const text =
    phase === "failed"
      ? failure
      : phase === "idle"
        ? (blocker ??
          (serverName
            ? `Ready with ${serverName}${protocolLabel ? ` · ${protocolLabel}` : ""}`
            : null))
        : serverName;

  return text ? (
    <p
      className={cn(
        "mt-1 line-clamp-2 text-[13px] leading-snug",
        phase === "failed" ? "text-text-secondary" : "opacity-75",
      )}
    >
      {text}
    </p>
  ) : null;
}

function Fact({
  icon,
  label,
  children,
}: {
  icon: ReactNode;
  label: string;
  children: ReactNode;
}) {
  return (
    <div className="bg-surface border-border-subtle rounded-2xl border px-3.5 py-2.5">
      <p className="text-text-muted flex min-w-0 items-center gap-1.5 text-xs [&>svg]:size-3.5 [&>svg]:shrink-0">
        {icon}
        <span className="truncate">{label}</span>
      </p>
      <div className="mt-1 flex min-w-0 items-baseline gap-1.5">{children}</div>
    </div>
  );
}

/**
 * Latency as a single number, re-measured slowly and only while visible.
 */
function LatencyFact({
  serverId,
  protocol,
}: {
  serverId: string | null;
  protocol: Parameters<typeof usePing>[1];
}) {
  const ping = usePing(serverId, protocol);
  const tone = classifyLatency(ping.ms);

  // The latest `refresh` without restarting the timer on every render.
  const refresh = useRef(ping.refresh);
  useEffect(() => {
    refresh.current = ping.refresh;
  });

  useEffect(() => {
    if (!serverId || !protocol) return;
    const timer = setInterval(() => {
      if (document.visibilityState === "visible") refresh.current();
    }, PING_INTERVAL_MS);
    return () => clearInterval(timer);
  }, [serverId, protocol]);

  return (
    <Fact icon={<Activity strokeWidth={2.4} />} label="Latency">
      <span
        className={cn(
          "font-mono text-[17px] leading-none font-bold tabular-nums",
          TONE_TEXT[tone],
        )}
      >
        {typeof ping.ms === "number"
          ? Math.max(1, Math.round(ping.ms))
          : ping.isProbing
            ? "···"
            : "—"}
      </span>
      {typeof ping.ms === "number" ? (
        <span className="text-text-muted text-xs">ms</span>
      ) : ping.ms === null && !ping.isProbing ? (
        <span className="text-danger text-xs">no answer</span>
      ) : null}
    </Fact>
  );
}

function withWindow(
  action: (win: ReturnType<typeof getCurrentWindow>) => void,
) {
  if (!isTauri()) return;
  action(getCurrentWindow());
}

/** The compact window's own title bar: drag, pin, expand, minimize, close. */
function CompactTitleBar() {
  const setCompact = useLayoutPrefs((state) => state.setCompact);
  const onTop = useLayoutPrefs((state) => state.compactOnTop);
  const setOnTop = useLayoutPrefs((state) => state.setCompactOnTop);

  return (
    <header
      data-tauri-drag-region
      className="flex h-10 shrink-0 items-center select-none"
    >
      <div
        data-tauri-drag-region
        className="text-text-muted flex flex-1 items-center gap-2 self-stretch pl-4"
      >
        <img src={logoIcon} alt="" className="size-4 shrink-0" />
        <span className="text-xs font-medium">ForceConnect</span>
      </div>

      <div className="flex h-full shrink-0 items-stretch">
        <BarButton
          label={onTop ? "Stop keeping on top" : "Keep on top"}
          active={onTop}
          onClick={() => setOnTop(!onTop)}
        >
          {onTop ? <PinOff /> : <Pin />}
        </BarButton>
        <BarButton label="Full window" onClick={() => setCompact(false)}>
          <Maximize2 />
        </BarButton>
        <BarButton
          label="Minimize"
          onClick={() => withWindow((win) => void win.minimize())}
        >
          <Minus />
        </BarButton>
        <BarButton
          label="Close"
          danger
          onClick={() => withWindow((win) => void win.close())}
        >
          <X />
        </BarButton>
      </div>
    </header>
  );
}

function BarButton({
  label,
  danger,
  active,
  onClick,
  children,
}: {
  label: string;
  danger?: boolean;
  active?: boolean;
  onClick: () => void;
  children: ReactNode;
}) {
  return (
    <Tooltip content={label} side="bottom">
      <button
        type="button"
        aria-label={label}
        aria-pressed={active}
        onClick={onClick}
        className={cn(
          "inline-flex w-10 items-center justify-center transition-colors [&>svg]:size-3.5",
          active ? "text-accent-bright" : "text-text-muted",
          danger
            ? "hover:bg-danger hover:text-white"
            : "hover:bg-surface-hover hover:text-text-primary",
        )}
      >
        {children}
      </button>
    </Tooltip>
  );
}
