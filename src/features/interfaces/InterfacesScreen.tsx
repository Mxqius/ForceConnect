import { RadioGroup } from "@base-ui/react/radio-group";
import { Radio } from "@base-ui/react/radio";
import {
  Cable,
  Check,
  CircuitBoard,
  Repeat,
  ShieldCheck,
  TriangleAlert,
  Wand2,
  Wifi,
  type LucideIcon,
} from "lucide-react";
import type { ReactNode } from "react";

import {
  CopyableValue,
  EmptyState,
  PageHeader,
  pagePadding,
  Skeleton,
  Tooltip,
} from "@/design/primitives";
import { useAdapters, useDnsTarget } from "@/features/interfaces/useAdapters";
import { cn } from "@/lib/cn";
import type { Adapter, AdapterKind } from "@/lib/ipc";

const KIND_ICONS: Record<AdapterKind, LucideIcon> = {
  ethernet: Cable,
  wifi: Wifi,
  tunnel: ShieldCheck,
  loopback: Repeat,
  other: CircuitBoard,
};

const KIND_LABELS: Record<AdapterKind, string> = {
  ethernet: "Ethernet",
  wifi: "Wi-Fi",
  tunnel: "Tunnel",
  loopback: "Loopback",
  other: "Virtual",
};

/** The radio value for "follow the routing table". */
const AUTOMATIC = "automatic";

export function InterfacesScreen() {
  const { data, isPending, error } = useAdapters();
  const target = useDnsTarget();

  const eligible = data?.filter((a) => a.eligible) ?? [];
  const rest = data?.filter((a) => !a.eligible) ?? [];

  return (
    <div className={cn("flex flex-col", pagePadding)}>
      <PageHeader
        title="Interfaces"
        description="Where ForceConnect writes DNS when you connect."
      />

      {error ? (
        <Notice tone="danger">
          {error instanceof Error ? error.message : String(error)}
        </Notice>
      ) : null}

      {target.unavailableReason ? (
        <Notice tone="warning">
          {target.unavailableReason === "missing"
            ? "The interface you picked is no longer present."
            : "The interface you picked is not usable right now."}{" "}
          DNS is going to{" "}
          <strong className="font-semibold">
            {target.adapter?.name ?? "no adapter"}
          </strong>{" "}
          instead.
        </Notice>
      ) : null}

      {isPending ? (
        <div className="grid grid-cols-2 gap-4">
          <Skeleton className="col-span-2 h-[88px] rounded-[22px]" />
          <Skeleton className="h-44 rounded-[22px]" />
          <Skeleton className="h-44 rounded-[22px]" />
        </div>
      ) : null}

      {!isPending && eligible.length === 0 && !error ? (
        <EmptyState
          icon={<CircuitBoard />}
          title="No usable adapters"
          description="Nothing is connected to a network right now."
        />
      ) : null}

      {eligible.length > 0 ? (
        // One radio group across the automatic option and every adapter:
        // they are a single choice, and arrow keys should move through all of
        // them the way they do in Windows' own network settings.
        <RadioGroup
          aria-label="Interface for DNS"
          value={target.pinnedLuid ?? AUTOMATIC}
          onValueChange={(value) =>
            target.select(value === AUTOMATIC ? null : value)
          }
          className="grid grid-cols-2 gap-4"
        >
          <Choice value={AUTOMATIC} className="col-span-2">
            <div className="flex items-center gap-3.5">
              <KindIcon icon={Wand2} />
              <div className="min-w-0 flex-1">
                <div className="flex items-center gap-2">
                  <h2 className="text-[15px] font-semibold">Automatic</h2>
                  <Pill tone="success">Recommended</Pill>
                </div>
                <p className="text-text-muted mt-0.5 text-[13px]">
                  Follows whichever adapter Windows sends internet traffic
                  through.
                  {target.pinnedLuid === null && target.adapter ? (
                    <>
                      {" "}
                      Right now that is{" "}
                      <span className="text-text-secondary font-semibold">
                        {target.adapter.name}
                      </span>
                      .
                    </>
                  ) : null}
                </p>
              </div>
              <RadioMark />
            </div>
          </Choice>

          <SectionLabel>Or pick one</SectionLabel>

          {eligible.map((adapter) => (
            <Choice key={adapter.luid} value={adapter.luid}>
              <AdapterBody
                adapter={adapter}
                receivesDns={target.adapter?.luid === adapter.luid}
              />
            </Choice>
          ))}
        </RadioGroup>
      ) : null}

      {rest.length > 0 ? (
        <>
          <SectionLabel className="mt-8 mb-4">
            Not available · disconnected or no route to the internet
          </SectionLabel>
          <div className="grid grid-cols-2 gap-4">
            {rest.map((adapter) => (
              <div
                key={adapter.luid}
                className="bg-surface border-border-subtle rounded-[22px] border px-5 py-4 opacity-55"
              >
                <AdapterBody adapter={adapter} />
              </div>
            ))}
          </div>
        </>
      ) : null}
    </div>
  );
}

/**
 * A selectable tile.
 *
 * The whole tile is the radio. The copy buttons inside it are separate
 * controls that stop their own clicks, so copying an address never changes
 * the selection.
 */
function Choice({
  value,
  className,
  children,
}: {
  value: string;
  className?: string;
  children: ReactNode;
}) {
  return (
    <Radio.Root
      value={value}
      className={cn(
        "group bg-surface shadow-tile block cursor-pointer rounded-[22px] border-[1.5px] px-5 py-4 text-left transition-colors",
        "border-border-subtle hover:border-border-strong data-checked:border-accent",
        "focus-visible:outline-accent outline-none focus-visible:outline-2 focus-visible:outline-offset-2",
        className,
      )}
    >
      {children}
    </Radio.Root>
  );
}

function RadioMark() {
  return (
    <span
      aria-hidden
      className={cn(
        "border-border-strong flex size-[18px] shrink-0 items-center justify-center rounded-full border-[1.5px] text-white transition-colors",
        "group-data-checked:border-accent group-data-checked:bg-accent",
      )}
    >
      <Check
        className="hidden size-[11px] group-data-checked:block"
        strokeWidth={3.4}
      />
    </span>
  );
}

function KindIcon({ icon: Icon }: { icon: LucideIcon }) {
  return (
    <span
      className={cn(
        "bg-surface-inset text-text-secondary flex size-11 shrink-0 items-center justify-center rounded-[14px] transition-colors",
        "group-data-checked:bg-accent-surface group-data-checked:text-accent-bright",
      )}
    >
      <Icon className="size-5" />
    </span>
  );
}

function AdapterBody({
  adapter,
  receivesDns = false,
}: {
  adapter: Adapter;
  /** DNS currently goes here, by explicit choice or by falling back. */
  receivesDns?: boolean;
}) {
  const stop = (event: React.SyntheticEvent) => event.stopPropagation();

  return (
    <>
      <div className="flex items-center gap-3">
        <KindIcon icon={KIND_ICONS[adapter.kind]} />
        <div className="min-w-0 flex-1">
          <h3 className="truncate text-[15px] font-semibold">{adapter.name}</h3>
          <p className="text-text-muted truncate text-[13px]">
            {adapter.description}
          </p>
        </div>
        {adapter.eligible ? <RadioMark /> : null}
      </div>

      <div className="mt-3 flex flex-wrap gap-1.5">
        {adapter.isActive ? (
          <Tooltip content="The routing table sends internet traffic here">
            <span>
              <Pill tone="success">Internet</Pill>
            </span>
          </Tooltip>
        ) : null}
        <Pill>{KIND_LABELS[adapter.kind]}</Pill>
        {adapter.status === "up" ? null : <Pill>Down</Pill>}
        {adapter.dnsSource === "static" ? (
          <Tooltip content="DNS servers are set explicitly, not supplied by DHCP">
            <span>
              <Pill tone="accent">Static DNS</Pill>
            </span>
          </Tooltip>
        ) : null}
        {receivesDns ? <Pill tone="accent">Receives DNS</Pill> : null}
      </div>

      {/* The values are copy buttons; keep their clicks and keys off the tile. */}
      <dl
        onClick={stop}
        onKeyDown={stop}
        className="border-border-subtle mt-4 grid grid-cols-[auto_1fr] items-center gap-x-4 gap-y-1.5 border-t pt-3 text-[13px]"
      >
        <dt className="text-text-muted">Address</dt>
        <dd className="flex justify-end">
          {adapter.addresses[0] ? (
            <CopyableValue value={adapter.addresses[0]} />
          ) : (
            <span className="text-text-faint">—</span>
          )}
        </dd>
        <dt className="text-text-muted">DNS now</dt>
        <dd className="flex flex-wrap justify-end gap-x-2">
          {adapter.dnsV4.length > 0 ? (
            adapter.dnsV4.map((ip) => <CopyableValue key={ip} value={ip} />)
          ) : (
            <span className="text-text-faint">none</span>
          )}
        </dd>
        {adapter.linkSpeedMbps > 0 ? (
          <>
            <dt className="text-text-muted">Link</dt>
            <dd className="text-text-secondary text-right font-mono tabular-nums">
              {formatSpeed(adapter.linkSpeedMbps)} · metric {adapter.metricV4}
            </dd>
          </>
        ) : null}
      </dl>

      {adapter.kind === "tunnel" && adapter.eligible ? (
        <p className="text-warning mt-3 flex items-center gap-1.5 text-[13px]">
          <TriangleAlert className="size-3.5 shrink-0" />A VPN may put its own
          DNS back
        </p>
      ) : null}
    </>
  );
}

function Pill({
  tone = "neutral",
  children,
}: {
  tone?: "neutral" | "success" | "accent";
  children: ReactNode;
}) {
  return (
    <span
      className={cn(
        "rounded-full px-2.5 py-0.5 text-[11px] font-bold",
        tone === "success" && "bg-success-surface text-success",
        tone === "accent" && "bg-accent-surface text-accent-bright",
        tone === "neutral" && "bg-surface-inset text-text-muted",
      )}
    >
      {children}
    </span>
  );
}

function SectionLabel({
  className,
  children,
}: {
  className?: string;
  children: ReactNode;
}) {
  return (
    <h2
      className={cn(
        "text-text-secondary col-span-2 mt-4 text-xs font-semibold tracking-widest uppercase",
        className,
      )}
    >
      {children}
    </h2>
  );
}

function Notice({
  tone,
  children,
}: {
  tone: "danger" | "warning";
  children: ReactNode;
}) {
  return (
    <p
      className={cn(
        "mb-5 flex items-start gap-2.5 rounded-2xl border px-4 py-3 text-[13px]",
        tone === "danger"
          ? "border-danger/30 bg-danger-surface text-danger"
          : "border-warning/30 bg-warning-surface text-warning",
      )}
    >
      <TriangleAlert className="mt-px size-4 shrink-0" />
      <span>{children}</span>
    </p>
  );
}

function formatSpeed(mbps: number): string {
  if (mbps >= 1000) {
    const gbps = mbps / 1000;
    return `${Number.isInteger(gbps) ? gbps : gbps.toFixed(1)} Gbps`;
  }
  return `${mbps} Mbps`;
}
