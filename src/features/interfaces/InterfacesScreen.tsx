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

import {
  Badge,
  Button,
  CopyableValue,
  EmptyState,
  Skeleton,
  StatusDot,
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

export function InterfacesScreen() {
  const { data, isPending, error } = useAdapters();
  const target = useDnsTarget();

  const eligible = data?.filter((a) => a.eligible) ?? [];
  const rest = data?.filter((a) => !a.eligible) ?? [];

  return (
    <div className="mx-auto flex max-w-3xl flex-col gap-7 p-8">
      <header>
        <h1 className="text-xl font-semibold tracking-tight">Interfaces</h1>
        <p className="text-text-muted mt-1 text-sm">
          DNS is applied to one adapter. Pick the one you want it written to, or
          leave it automatic and it follows whichever adapter traffic is
          actually leaving by.
        </p>
      </header>

      {error ? (
        <p className="border-danger/30 bg-danger-surface text-danger rounded-xl border px-4 py-3 text-sm">
          {error instanceof Error ? error.message : String(error)}
        </p>
      ) : null}

      {target.unavailableReason ? (
        <p className="border-warning/30 bg-warning-surface text-warning flex items-start gap-2.5 rounded-xl border px-4 py-3 text-xs">
          <TriangleAlert className="mt-px size-4 shrink-0" />
          <span>
            {target.unavailableReason === "missing"
              ? "The interface you picked is no longer present."
              : "The interface you picked is not usable right now."}{" "}
            DNS is going to{" "}
            <strong className="font-semibold">
              {target.adapter?.name ?? "no adapter"}
            </strong>{" "}
            instead.
          </span>
        </p>
      ) : null}

      {isPending ? (
        <div className="flex flex-col gap-2.5">
          {[0, 1, 2].map((i) => (
            <Skeleton key={i} className="h-24 w-full rounded-xl" />
          ))}
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
        <section className="flex flex-col gap-2.5">
          <SectionLabel
            title="Available"
            hint="Ordered by interface metric — lower wins"
          />

          <AutomaticCard
            selected={target.pinnedLuid === null}
            adapter={target.pinnedLuid === null ? target.adapter : undefined}
            onSelect={() => target.select(null)}
          />

          {eligible.map((adapter) => (
            <AdapterCard
              key={adapter.luid}
              adapter={adapter}
              receivesDns={target.adapter?.luid === adapter.luid}
              pinned={target.pinnedLuid === adapter.luid}
              onSelect={() => target.select(adapter.luid)}
            />
          ))}
        </section>
      ) : null}

      {rest.length > 0 ? (
        <section className="flex flex-col gap-2.5">
          <SectionLabel
            title="Not available"
            hint="Disconnected, loopback, or with no route off-link"
          />
          {rest.map((adapter) => (
            <AdapterCard key={adapter.luid} adapter={adapter} muted />
          ))}
        </section>
      ) : null}
    </div>
  );
}

function SectionLabel({ title, hint }: { title: string; hint: string }) {
  return (
    <div className="flex items-baseline gap-2.5">
      <h2 className="text-text-secondary text-xs font-semibold tracking-widest uppercase">
        {title}
      </h2>
      <span className="text-text-faint text-xs">{hint}</span>
    </div>
  );
}

/** Shared frame so the automatic option reads as one of the choices. */
function cardFrame(selected: boolean, muted?: boolean) {
  return cn(
    "border-border-subtle bg-surface rounded-xl border p-4 text-left transition-colors",
    selected && "border-accent-muted bg-accent-surface/40",
    muted && "opacity-60",
  );
}

function SelectedMark({ selected }: { selected: boolean }) {
  return (
    <span
      className={cn(
        "flex size-5 shrink-0 items-center justify-center rounded-full border transition-colors",
        selected
          ? "border-accent bg-accent text-text-on-accent"
          : "border-border-strong",
      )}
      aria-hidden
    >
      {selected ? <Check className="size-3" strokeWidth={3.5} /> : null}
    </span>
  );
}

function AutomaticCard({
  selected,
  adapter,
  onSelect,
}: {
  selected: boolean;
  adapter: Adapter | undefined;
  onSelect: () => void;
}) {
  return (
    <article className={cardFrame(selected)}>
      <div className="flex items-start gap-4">
        <div className="bg-surface-inset text-text-muted mt-0.5 flex size-10 shrink-0 items-center justify-center rounded-lg">
          <Wand2 className="size-5" />
        </div>

        <div className="min-w-0 flex-1">
          <h3 className="text-sm font-semibold">Automatic</h3>
          <p className="text-text-muted mt-1 text-xs">
            {selected && adapter
              ? `Following the routing table — currently ${adapter.name}`
              : "Follow whichever adapter traffic is leaving by"}
          </p>
        </div>

        <SelectedMark selected={selected} />
      </div>

      <div className="mt-3 flex justify-end">
        <Button
          size="sm"
          variant={selected ? "ghost" : "secondary"}
          disabled={selected}
          onClick={onSelect}
        >
          {selected ? "Selected" : "Use automatic"}
        </Button>
      </div>
    </article>
  );
}

function AdapterCard({
  adapter,
  muted,
  receivesDns = false,
  pinned = false,
  onSelect,
}: {
  adapter: Adapter;
  muted?: boolean;
  /** DNS currently goes here, whether by explicit choice or by falling back. */
  receivesDns?: boolean;
  /** The user pinned this one specifically. */
  pinned?: boolean;
  onSelect?: () => void;
}) {
  const Icon = KIND_ICONS[adapter.kind];
  const isUp = adapter.status === "up";

  // Unusable adapters are shown for diagnosis, not for choosing: writing DNS to
  // one with no route off-link would apply cleanly and change nothing.
  const selectable = Boolean(onSelect);

  return (
    <article className={cardFrame(receivesDns, muted)}>
      <div className="flex items-start gap-4">
        <div className="bg-surface-inset text-text-muted mt-0.5 flex size-10 shrink-0 items-center justify-center rounded-lg">
          <Icon className="size-5" />
        </div>

        <div className="min-w-0 flex-1">
          <div className="flex flex-wrap items-center gap-2">
            <h3 className="truncate text-sm font-semibold">{adapter.name}</h3>

            {adapter.isActive ? (
              <Tooltip content="The routing table sends internet traffic here">
                <span className="text-success inline-flex items-center gap-1.5 text-xs font-medium">
                  <StatusDot tone="success" pulse />
                  Active
                </span>
              </Tooltip>
            ) : null}

            <Badge tone="neutral">{KIND_LABELS[adapter.kind]}</Badge>

            {isUp ? null : <Badge tone="neutral">Down</Badge>}

            {adapter.dnsSource === "static" ? (
              <Tooltip content="DNS servers are configured explicitly, not supplied by DHCP">
                <span>
                  <Badge tone="accent">Static DNS</Badge>
                </span>
              </Tooltip>
            ) : null}

            {receivesDns ? (
              <Tooltip content="DNS changes are written to this adapter">
                <span>
                  <Badge tone="accent">Receives DNS</Badge>
                </span>
              </Tooltip>
            ) : null}
          </div>

          <p className="text-text-muted mt-1 truncate text-xs">
            {adapter.description}
          </p>

          <dl className="text-text-muted mt-3 flex flex-wrap items-center gap-x-5 gap-y-1.5 text-xs">
            <Detail label="Metric">
              <span className="font-mono tabular-nums">{adapter.metricV4}</span>
            </Detail>

            {adapter.linkSpeedMbps > 0 ? (
              <Detail label="Link">
                <span className="font-mono tabular-nums">
                  {formatSpeed(adapter.linkSpeedMbps)}
                </span>
              </Detail>
            ) : null}

            {adapter.addresses.length > 0 ? (
              <Detail label="Address">
                <CopyableValue value={adapter.addresses[0]!} />
              </Detail>
            ) : null}

            {adapter.dnsV4.length > 0 ? (
              <Detail label="DNS">
                <span className="flex flex-wrap gap-2">
                  {adapter.dnsV4.map((ip) => (
                    <CopyableValue key={ip} value={ip} />
                  ))}
                </span>
              </Detail>
            ) : null}
          </dl>
        </div>

        {selectable ? <SelectedMark selected={receivesDns} /> : null}
      </div>

      {selectable ? (
        <div className="mt-3 flex justify-end">
          {/* The card itself is not the control: the addresses inside it are
              copy buttons, and a button cannot contain a button. */}
          <Button
            size="sm"
            variant={pinned ? "ghost" : "secondary"}
            disabled={pinned}
            onClick={onSelect}
          >
            {pinned ? "Selected" : "Use this interface"}
          </Button>
        </div>
      ) : null}
    </article>
  );
}

function Detail({
  label,
  children,
}: {
  label: string;
  children: React.ReactNode;
}) {
  return (
    <div className="flex items-center gap-1.5">
      <dt className="text-text-faint">{label}</dt>
      <dd className="text-text-secondary">{children}</dd>
    </div>
  );
}

function formatSpeed(mbps: number): string {
  if (mbps >= 1000) {
    const gbps = mbps / 1000;
    return `${Number.isInteger(gbps) ? gbps : gbps.toFixed(1)} Gbps`;
  }
  return `${mbps} Mbps`;
}
