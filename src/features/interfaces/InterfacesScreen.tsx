import {
  Cable,
  CircuitBoard,
  Repeat,
  ShieldCheck,
  Wifi,
  type LucideIcon,
} from "lucide-react";

import {
  Badge,
  CopyableValue,
  EmptyState,
  Skeleton,
  StatusDot,
  Tooltip,
} from "@/design/primitives";
import { useAdapters } from "@/features/interfaces/useAdapters";
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

  const eligible = data?.filter((a) => a.eligible) ?? [];
  const rest = data?.filter((a) => !a.eligible) ?? [];

  return (
    <div className="mx-auto flex max-w-3xl flex-col gap-7 p-8">
      <header>
        <h1 className="text-xl font-semibold tracking-tight">Interfaces</h1>
        <p className="text-text-muted mt-1 text-sm">
          DNS is applied per adapter. The active one is where traffic actually
          leaves — which is not always the one you would expect.
        </p>
      </header>

      {error ? (
        <p className="border-danger/30 bg-danger-surface text-danger rounded-xl border px-4 py-3 text-sm">
          {error instanceof Error ? error.message : String(error)}
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
          {eligible.map((adapter) => (
            <AdapterCard key={adapter.luid} adapter={adapter} />
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

function AdapterCard({
  adapter,
  muted,
}: {
  adapter: Adapter;
  muted?: boolean;
}) {
  const Icon = KIND_ICONS[adapter.kind];
  const isUp = adapter.status === "up";

  return (
    <article
      className={cn(
        "border-border-subtle bg-surface rounded-xl border p-4 transition-colors",
        adapter.isActive && "border-accent-muted bg-accent-surface/30",
        muted && "opacity-60",
      )}
    >
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
      </div>
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
