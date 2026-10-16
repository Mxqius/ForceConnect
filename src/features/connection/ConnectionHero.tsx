import {
  Check,
  Eraser,
  MoreHorizontal,
  Power,
  RefreshCw,
  RotateCcw,
  X,
} from "lucide-react";
import { useEffect, useState, type ReactNode } from "react";

import { Popover, Tile, Tooltip, toast } from "@/design/primitives";
import {
  CONNECT_STEPS,
  useConnection,
  type ConnectPhase,
  type ConnectStep,
} from "@/features/connection/useConnection";
import { useDnsTools } from "@/features/connection/useDnsTools";
import { ProviderMark } from "@/features/servers/ProviderMark";
import { cn } from "@/lib/cn";
import type { Endpoint, Server } from "@/lib/ipc";

import "@/features/connection/connection.css";

interface ConnectionHeroProps {
  selected: Server | null;
  endpoint: Endpoint | undefined;
  /** Why the power button cannot connect right now, if it cannot. */
  blocker: string | null;
}

/** The phases where the tile wears the brand gradient. */
const LIT: ReadonlySet<ConnectPhase> = new Set(["connected", "switching"]);

/**
 * The connection tile.
 *
 * Everything about the connection's *state* is here and nowhere else on the
 * screen: colour, title, the step list, and the one control that changes it.
 * The other tiles describe the choice; this one describes what the machine is
 * actually doing.
 */
export function ConnectionHero({
  selected,
  endpoint,
  blocker,
}: ConnectionHeroProps) {
  const {
    phase,
    step,
    attempt,
    failure,
    active,
    connectedAt,
    canCancel,
    adapter,
    connect,
    disconnect,
    cancel,
    dismissFailure,
  } = useConnection();

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

  // Ripples mark the moment a connect lands, not every later visit to a
  // screen that happens to be connected: only a connection made while this
  // tile was on screen gets them.
  const [mountedAt] = useState(() => Date.now());
  const celebrate =
    phase === "connected" && connectedAt !== null && connectedAt > mountedAt
      ? connectedAt
      : null;

  const network = adapter?.dnsV4[0];

  const chip: { tone: string; text: string; pulse?: boolean } = {
    idle: {
      tone: "bg-text-muted",
      text: network ? `Network DNS · ${network}` : "Network DNS",
    },
    connecting: { tone: "bg-accent-bright", text: "Connecting", pulse: true },
    switching: { tone: "bg-white", text: "Switching", pulse: true },
    disconnecting: {
      tone: "bg-text-muted",
      text: "Restoring network DNS",
      pulse: true,
    },
    connected: active?.encrypted
      ? { tone: "bg-[#7dffb6]", text: `Encrypted · ${active.protocolLabel}` }
      : { tone: "bg-[#ffd56b]", text: "Plain DNS · not encrypted" },
    failed: {
      tone: "bg-danger",
      text: failure?.step === "apply" ? "Not connected" : "Nothing was changed",
    },
  }[phase];

  const title = {
    idle: "Not connected",
    connecting: "Connecting…",
    switching: "Switching…",
    connected: "Connected",
    disconnecting: "Disconnecting…",
    failed: "Couldn’t connect",
  }[phase];

  const subtitle = {
    idle: blocker ?? "Lookups go to the DNS your network hands out.",
    connecting: null,
    switching: `Moving to ${attempt?.serverName ?? "the new server"}. Your current DNS stays in place until the new one is set.`,
    connected: active
      ? active.encrypted
        ? `Every lookup is encrypted and answered by ${active.serverName}.`
        : `Lookups now go straight to ${active.serverName}.`
      : null,
    disconnecting: "Handing the adapter back to your network’s DNS.",
    failed: null,
  }[phase];

  return (
    <Tile
      data-lit={lit}
      data-phase={phase}
      aria-label="Connection"
      className={cn(
        "fc-hero col-span-2 row-span-3 p-7 transition-[background-color,border-color,color] duration-500",
        phase === "connecting" && "bg-hero-busy border-hero-busy-border",
        phase === "failed" && "bg-hero-failed border-hero-failed-border",
        lit && "border-transparent text-white",
      )}
    >
      <div className="fc-hero-glow" />
      {working ? <div className="fc-shimmer" /> : null}
      {phase === "connecting" ? (
        <div
          className="fc-progress"
          style={{
            width: `${Math.min(100, ((step + 0.5) / CONNECT_STEPS.length) * 100)}%`,
          }}
        />
      ) : null}

      <div className="relative flex h-full flex-col">
        <div className="flex items-center gap-2">
          <span
            className={cn(
              "inline-flex min-w-0 items-center gap-2 rounded-full py-1.5 pr-3.5 pl-3 text-[13px] font-semibold transition-colors duration-500",
              lit
                ? "bg-white/18 text-white"
                : "bg-surface-inset text-text-secondary",
            )}
          >
            <span
              className={cn(
                "size-2 shrink-0 rounded-full",
                chip.tone,
                chip.pulse && "animate-pulse",
              )}
            />
            <span className="truncate">{chip.text}</span>
          </span>

          <ToolsMenu lit={lit} reapplyServers={active?.addrs ?? []} />
        </div>

        <div className="flex flex-1 flex-col justify-end pb-6">
          <h1
            key={title}
            className="fc-rise text-[42px] leading-[1.02] font-bold tracking-[-0.035em]"
          >
            {title}
          </h1>

          {subtitle ? (
            <p className="mt-2.5 max-w-[340px] text-[15px] leading-snug opacity-80">
              {subtitle}
            </p>
          ) : null}

          {(phase === "connecting" || phase === "failed") && attempt ? (
            <StepList
              step={step}
              failedStep={phase === "failed" ? (failure?.step ?? null) : null}
              labels={{
                probe: [
                  `Checking ${attempt.serverName} answers`,
                  attempt.protocolLabel,
                ],
                apply: [`Setting DNS on ${attempt.adapterName}`, ""],
                flush: ["Clearing the DNS cache", ""],
              }}
            />
          ) : null}

          {phase === "failed" && failure ? (
            <>
              <p className="text-text-secondary mt-3 max-w-[360px] text-sm leading-snug">
                {failure.message}
              </p>
              <div className="mt-4 flex gap-2.5">
                <button
                  type="button"
                  onClick={handleConnect}
                  disabled={Boolean(blocker)}
                  className="bg-control-strong text-control-strong-ink flex h-10 items-center gap-2 rounded-full px-5 text-sm font-semibold disabled:opacity-45"
                >
                  <RefreshCw className="size-4" strokeWidth={2.4} />
                  Try again
                </button>
                <button
                  type="button"
                  onClick={dismissFailure}
                  className="border-border-strong text-text-secondary hover:bg-surface-hover hover:text-text-primary flex h-10 items-center rounded-full border px-5 text-sm font-semibold"
                >
                  Dismiss
                </button>
              </div>
            </>
          ) : null}
        </div>

        <div className="flex items-end justify-between gap-3">
          <div className="min-w-0">
            {phase === "connected" ||
            phase === "switching" ||
            phase === "disconnecting" ? (
              <>
                <small className="block text-[13px] opacity-70">
                  Connected for
                </small>
                <Uptime since={connectedAt} />
              </>
            ) : (
              <>
                <small className="block text-[13px] opacity-70">
                  {phase === "connecting" ? "Connecting to" : "Ready with"}
                </small>
                {selected ? (
                  <div className="mt-1.5 flex min-w-0 items-center gap-2.5">
                    <ProviderMark
                      provider={selected.provider}
                      name={selected.name}
                      className="size-7 rounded-lg text-xs"
                    />
                    <span className="truncate text-[17px] font-semibold">
                      {selected.name}
                    </span>
                    {endpoint ? (
                      <span className="text-sm opacity-65">
                        {endpoint.label}
                      </span>
                    ) : null}
                  </div>
                ) : (
                  <span className="mt-1.5 block text-[17px] font-semibold">
                    Pick a server
                  </span>
                )}
              </>
            )}
          </div>

          <PowerButton
            phase={phase}
            action={action}
            spinning={working}
            celebrate={celebrate}
            cancelLocked={phase === "connecting" && !canCancel}
          />
        </div>
      </div>
    </Tile>
  );
}

type StepStatus = "pending" | "running" | "done" | "failed";

function StepList({
  step,
  failedStep,
  labels,
}: {
  step: number;
  failedStep: ConnectStep | null;
  labels: Record<ConnectStep, [string, string]>;
}) {
  const failedIndex = failedStep ? CONNECT_STEPS.indexOf(failedStep) : -1;

  return (
    <ol className="mt-5 flex flex-col gap-1" aria-label="Connection steps">
      {CONNECT_STEPS.map((id, index) => {
        const status: StepStatus =
          index === failedIndex
            ? "failed"
            : failedIndex >= 0
              ? index < failedIndex
                ? "done"
                : "pending"
              : index < step
                ? "done"
                : index === step
                  ? "running"
                  : "pending";
        const [label, detail] = labels[id];

        return (
          <li
            key={id}
            aria-current={status === "running" ? "step" : undefined}
            className={cn(
              "flex h-9 items-center gap-3 text-sm transition-opacity duration-300",
              status === "pending" && "opacity-40",
            )}
          >
            <StepIcon key={status} status={status} />
            <span className="min-w-0 truncate">{label}</span>
            <span
              className={cn(
                "ml-auto shrink-0 font-mono text-xs",
                status === "failed" ? "text-danger" : "text-text-muted",
              )}
            >
              {status === "failed" ? "failed" : detail}
            </span>
          </li>
        );
      })}
    </ol>
  );
}

function StepIcon({ status }: { status: StepStatus }) {
  const base = "flex size-6 shrink-0 items-center justify-center rounded-full";

  switch (status) {
    case "pending":
      return (
        <span
          className={cn(
            base,
            "border-border-strong border-[1.5px] border-dashed",
          )}
        />
      );
    case "running":
      return (
        <span
          className={cn(
            base,
            "border-hero-busy-border border-t-accent-bright animate-spin border-2",
          )}
        />
      );
    case "done":
      return (
        <span className={cn(base, "fc-check bg-success-surface text-success")}>
          <Check className="size-3.5" strokeWidth={3} />
        </span>
      );
    case "failed":
      return (
        <span className={cn(base, "fc-check bg-danger-surface text-danger")}>
          <X className="size-3.5" strokeWidth={3} />
        </span>
      );
  }
}

function PowerButton({
  phase,
  action,
  spinning,
  celebrate,
  cancelLocked,
}: {
  phase: ConnectPhase;
  action: { label: string; run: () => void } | null;
  spinning: boolean;
  /** The connection time to celebrate, which also keys the replay. */
  celebrate: number | null;
  cancelLocked: boolean;
}) {
  const lit = LIT.has(phase);
  const icon =
    phase === "connecting" ? (
      <X className="size-8" strokeWidth={2.2} />
    ) : (
      <Power className="size-9" strokeWidth={2.3} />
    );

  const button = (
    <button
      type="button"
      onClick={action?.run}
      disabled={!action}
      aria-label={
        action?.label ?? (phase === "connecting" ? "Cancel" : "Connect")
      }
      aria-pressed={phase === "connected"}
      className={cn(
        "absolute inset-[11px] flex items-center justify-center rounded-full transition-[transform,background-color,color,box-shadow] duration-300",
        "focus-visible:outline-accent outline-none focus-visible:outline-2 focus-visible:outline-offset-4",
        "enabled:hover:scale-105 enabled:active:scale-95 disabled:cursor-not-allowed",
        phase === "connected"
          ? "text-accent bg-white shadow-[0_12px_30px_-12px_rgb(0_0_0/0.6)]"
          : phase === "connecting"
            ? "bg-surface-active text-text-secondary"
            : lit
              ? "bg-white/20 text-white"
              : "bg-control-strong text-control-strong-ink shadow-[0_12px_30px_-12px_rgb(0_0_0/0.6)] disabled:opacity-45",
      )}
    >
      <span
        key={celebrate ?? "rest"}
        className={cn("flex", celebrate && "fc-pop")}
      >
        {icon}
      </span>
    </button>
  );

  return (
    <div className="relative size-28 shrink-0">
      <svg viewBox="0 0 96 96" className="absolute inset-0" aria-hidden>
        <circle
          cx="48"
          cy="48"
          r="46"
          fill="none"
          strokeWidth="2.5"
          className={cn(
            "transition-[stroke] duration-500",
            lit ? "stroke-white/30" : "stroke-border-default",
          )}
        />
        {spinning ? (
          <circle
            cx="48"
            cy="48"
            r="46"
            fill="none"
            strokeWidth="2.5"
            strokeLinecap="round"
            strokeDasharray="60 300"
            className={cn(
              "fc-arc",
              lit ? "stroke-white" : "stroke-accent-bright",
            )}
          />
        ) : null}
      </svg>

      {celebrate ? (
        <span key={celebrate} aria-hidden>
          <span className="fc-ripple" />
          <span className="fc-ripple" />
        </span>
      ) : null}

      {cancelLocked ? (
        <Tooltip content="DNS is being written now, so this can no longer be cancelled">
          {button}
        </Tooltip>
      ) : (
        button
      )}
    </div>
  );
}

function Uptime({ since }: { since: number | null }) {
  const [now, setNow] = useState(() => Date.now());

  useEffect(() => {
    const timer = setInterval(() => setNow(Date.now()), 1000);
    return () => clearInterval(timer);
  }, []);

  const total = since ? Math.max(0, Math.floor((now - since) / 1000)) : 0;
  const text = [
    Math.floor(total / 3600),
    Math.floor(total / 60) % 60,
    total % 60,
  ]
    .map((n) => String(n).padStart(2, "0"))
    .join(":");

  return (
    <b className="mt-1 block font-mono text-[26px] font-semibold tracking-tight tabular-nums">
      {text}
    </b>
  );
}

/**
 * The maintenance actions.
 *
 * Behind a menu because they are needed rarely — after a VPN overwrites DNS,
 * or when a site still resolves to an old address — and three buttons on the
 * main tile would make them look like part of connecting.
 */
function ToolsMenu({
  lit,
  reapplyServers,
}: {
  lit: boolean;
  reapplyServers: string[];
}) {
  const { flush, reset, reapply, isBusy } = useDnsTools();

  const report = (success: string, failure: string) => ({
    onSuccess: () => toast.success(success),
    onError: (err: unknown) =>
      toast.error(failure, {
        description: err instanceof Error ? err.message : String(err),
      }),
  });

  return (
    <Popover
      side="bottom"
      align="end"
      className="w-72 rounded-2xl p-1.5"
      trigger={
        <button
          type="button"
          aria-label="DNS tools"
          className={cn(
            "ml-auto flex size-8 shrink-0 items-center justify-center rounded-[10px] opacity-75 transition-colors hover:opacity-100",
            "focus-visible:outline-accent outline-none focus-visible:outline-2",
            lit ? "hover:bg-white/18" : "hover:bg-surface-hover",
          )}
        >
          <MoreHorizontal className="size-5" strokeWidth={2.4} />
        </button>
      }
    >
      <MenuItem
        icon={<Eraser />}
        title="Flush DNS cache"
        hint="Forget cached answers now"
        disabled={isBusy}
        onClick={() =>
          flush.mutate(
            undefined,
            report("DNS cache flushed", "Could not flush the DNS cache"),
          )
        }
      />
      <MenuItem
        icon={<RefreshCw />}
        title="Re-apply DNS"
        hint="Use after a VPN replaces it"
        disabled={isBusy || reapplyServers.length === 0}
        onClick={() =>
          reapply.mutate(
            reapplyServers,
            report("DNS re-applied", "Could not re-apply DNS"),
          )
        }
      />
      <MenuItem
        icon={<RotateCcw />}
        title="Reset to automatic"
        hint="Give the adapter back to DHCP"
        disabled={isBusy}
        onClick={() =>
          reset.mutate(
            undefined,
            report("Adapter returned to automatic DNS", "Could not reset DNS"),
          )
        }
      />
    </Popover>
  );
}

function MenuItem({
  icon,
  title,
  hint,
  disabled,
  onClick,
}: {
  icon: ReactNode;
  title: string;
  hint: string;
  disabled?: boolean;
  onClick: () => void;
}) {
  return (
    <Popover.Close
      disabled={disabled}
      onClick={onClick}
      className={cn(
        "text-text-primary flex w-full items-start gap-3 rounded-[10px] px-2.5 py-2 text-left",
        "hover:bg-surface-hover focus-visible:bg-surface-hover outline-none disabled:pointer-events-none disabled:opacity-45",
        "[&>svg]:text-text-muted [&>svg]:mt-0.5 [&>svg]:size-4 [&>svg]:shrink-0",
      )}
    >
      {icon}
      <span>
        <b className="block text-sm font-semibold">{title}</b>
        <small className="text-text-muted block text-xs">{hint}</small>
      </span>
    </Popover.Close>
  );
}
