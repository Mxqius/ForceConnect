import { useMutation } from "@tanstack/react-query";
import {
  CircleCheck,
  CircleX,
  Eraser,
  Info,
  ListX,
  Radar,
  Search,
} from "lucide-react";
import { useMemo, useState, type ReactNode } from "react";

import {
  Button,
  EmptyState,
  PageHeader,
  pagePadding,
  Tile,
  TileLabel,
} from "@/design/primitives";
import { useQueryLog } from "@/features/activity/useQueryLog";
import { useConnection } from "@/features/connection/useConnection";
import { useResolverStatus } from "@/features/connection/useResolverStatus";
import { cn } from "@/lib/cn";
import {
  ipc,
  type LeakReport,
  type Protocol,
  type QueryLogEntry,
  type QueryOutcome,
} from "@/lib/ipc";

/**
 * What the machine is asking, and whether it is going where it should.
 *
 * The log only exists for encrypted connections — plain DNS goes from the OS
 * straight to the server, past nothing of ours — but the leak test works for
 * both, because its second half asks the internet rather than the resolver.
 */
export function ActivityScreen() {
  const { active, phase, connectedAt } = useConnection();
  const connected = phase === "connected" || phase === "switching";
  const viaResolver = connected && (active?.viaResolver ?? false);

  const status = useResolverStatus(viaResolver);
  const { entries, clear } = useQueryLog(viaResolver, connectedAt);

  return (
    <div className={cn("flex min-h-full flex-col gap-4", pagePadding)}>
      <PageHeader
        title="Activity"
        description={
          viaResolver
            ? "Lookups through the local resolver. Kept in memory only, and gone when you disconnect."
            : connected
              ? "Plain DNS goes straight to the server, so there is no log to show."
              : "Connect to see lookups as they happen."
        }
        actions={
          viaResolver ? (
            <Button
              size="sm"
              className="rounded-full"
              startIcon={<Eraser />}
              disabled={entries.length === 0}
              onClick={clear}
            >
              Clear
            </Button>
          ) : null
        }
        className="mb-2"
      />

      {viaResolver && status ? (
        <dl className="border-border-subtle bg-border-subtle grid grid-cols-4 gap-px overflow-hidden rounded-2xl border">
          <Counter label="Lookups" value={status.queries} />
          <Counter
            label="From cache"
            value={status.cached}
            note={percent(status.cached, status.queries)}
          />
          <Counter
            label="By backup"
            value={status.answeredByBackup}
            note={status.backup ? undefined : "none set"}
          />
          <Counter
            label="Failed"
            value={status.failed}
            tone={status.failed > 0 ? "danger" : undefined}
          />
        </dl>
      ) : null}

      <LeakTest
        serverId={connected ? (active?.serverId ?? null) : null}
        serverName={active?.serverName ?? ""}
        protocol={active?.protocol ?? null}
        viaResolver={viaResolver}
      />

      <QueryLog entries={entries} enabled={viaResolver} />
    </div>
  );
}

const count = new Intl.NumberFormat();

function percent(part: number, whole: number): string | undefined {
  return whole > 0 ? `${Math.round((part / whole) * 100)}%` : undefined;
}

function Counter({
  label,
  value,
  note,
  tone,
}: {
  label: string;
  value: number;
  note?: string;
  tone?: "danger";
}) {
  return (
    <div className="bg-surface flex flex-col gap-1 px-5 py-3.5">
      <dt className="text-text-faint text-2xs font-semibold tracking-widest uppercase">
        {label}
      </dt>
      <dd className="flex items-baseline gap-2">
        <span
          className={cn(
            "font-mono text-lg font-semibold tabular-nums",
            tone === "danger" && "text-danger",
          )}
        >
          {count.format(value)}
        </span>
        {note ? <span className="text-text-muted text-xs">{note}</span> : null}
      </dd>
    </div>
  );
}

/**
 * Whether the network that answers looks like the server that was chosen.
 *
 * A heuristic, and presented as one: it compares words from the server's
 * name with the registrant of the answering network. "Cloudflare" matching
 * "Cloudflare, Inc." is a good sign; no match is not proof of a leak — a
 * provider can route through someone else's network — so it is said neutrally.
 */
function looksLike(owner: string, serverName: string): boolean {
  const haystack = owner.toLowerCase();
  return serverName
    .toLowerCase()
    .split(/[^a-z0-9]+/)
    .filter((word) => word.length >= 3 && word !== "dns")
    .some((word) => haystack.includes(word));
}

function LeakTest({
  serverId,
  serverName,
  protocol,
  viaResolver,
}: {
  serverId: string | null;
  serverName: string;
  protocol: Protocol | null;
  viaResolver: boolean;
}) {
  const test = useMutation({
    mutationFn: () =>
      serverId && protocol
        ? ipc.leakTest(serverId, protocol)
        : Promise.reject(new Error("not connected")),
  });
  const report = test.data;

  return (
    <Tile className="gap-3">
      <TileLabel
        icon={<Radar />}
        action={
          <Button
            size="sm"
            variant={report ? "secondary" : "primary"}
            className="rounded-full"
            disabled={!serverId || !protocol || test.isPending}
            onClick={() => test.mutate()}
          >
            {test.isPending ? "Testing…" : report ? "Run again" : "Run test"}
          </Button>
        }
      >
        Leak test
      </TileLabel>

      {!serverId ? (
        <p className="text-text-muted text-sm">
          Connect first: the test checks where your lookups go now.
        </p>
      ) : test.isError ? (
        <p className="text-danger text-sm">
          The test could not run: {String(test.error)}
        </p>
      ) : report ? (
        <LeakResult
          report={report}
          serverName={serverName}
          viaResolver={viaResolver}
        />
      ) : (
        <p className="text-text-muted text-sm leading-relaxed">
          Looks up a name nobody has asked for before, the way any app would,
          and checks it reached ForceConnect; then asks which network your
          lookups really leave from.
        </p>
      )}
    </Tile>
  );
}

function LeakResult({
  report,
  serverName,
  viaResolver,
}: {
  report: LeakReport;
  serverName: string;
  viaResolver: boolean;
}) {
  const leaking = report.throughResolver === false;
  const owner = report.egressOwner;
  const matches = owner ? looksLike(owner, serverName) : false;

  return (
    <div className="flex flex-col gap-2.5">
      <Verdict
        tone={leaking ? "danger" : viaResolver ? "success" : "neutral"}
        title={
          leaking
            ? "Lookups are going around ForceConnect"
            : viaResolver
              ? "No leak found"
              : "Plain DNS — the path cannot be checked"
        }
      >
        {leaking
          ? "A lookup made through the system never reached the local resolver, so something else on this machine is answering — another adapter, a VPN, or a browser with its own DNS."
          : viaResolver
            ? "A lookup made through the system arrived at the local resolver and went out encrypted."
            : "Lookups go straight from the system to the server, so only where they come out can be shown."}
      </Verdict>

      <div className="bg-surface-inset flex flex-wrap items-center gap-x-3 gap-y-1 rounded-xl px-4 py-2.5 text-[13px]">
        <span className="text-text-muted">Answered from</span>
        {report.egress.length > 0 ? (
          <>
            <span className="font-mono text-xs">
              {report.egress.join(", ")}
            </span>
            {owner ? (
              <span className="text-text-secondary">· {owner}</span>
            ) : null}
            {owner ? (
              <span
                className={cn(
                  "ml-auto rounded-full px-2.5 py-0.5 text-xs font-semibold",
                  matches
                    ? "bg-success-surface text-success"
                    : "bg-surface-active text-text-secondary",
                )}
              >
                {matches
                  ? `Looks like ${serverName}`
                  : `Not obviously ${serverName}`}
              </span>
            ) : null}
          </>
        ) : (
          <span className="text-text-secondary">
            could not be found out — the test name did not resolve
          </span>
        )}
      </div>
    </div>
  );
}

function Verdict({
  tone,
  title,
  children,
}: {
  tone: "success" | "danger" | "neutral";
  title: string;
  children: ReactNode;
}) {
  const Icon =
    tone === "success" ? CircleCheck : tone === "danger" ? CircleX : Info;
  return (
    <div className="flex gap-3">
      <Icon
        className={cn(
          "mt-0.5 size-5 shrink-0",
          tone === "success" && "text-success",
          tone === "danger" && "text-danger",
          tone === "neutral" && "text-text-muted",
        )}
      />
      <div className="min-w-0">
        <p className="text-sm font-semibold">{title}</p>
        <p className="text-text-muted mt-0.5 text-[13px] leading-snug">
          {children}
        </p>
      </div>
    </div>
  );
}

const OUTCOME: Record<QueryOutcome, [string, string]> = {
  answered: ["Answered", "bg-surface-active text-text-secondary"],
  cached: ["Cache", "bg-accent-surface text-accent-bright"],
  backup: ["Backup", "bg-warning-surface text-warning"],
  failed: ["Failed", "bg-danger-surface text-danger"],
  blocked: ["Blocked", "bg-surface-inset text-text-muted"],
};

// A 24-hour clock whatever the locale: a log is scanned down a fixed-width
// column, and "AM"/"PM" would wrap it.
const time = new Intl.DateTimeFormat(undefined, {
  hour: "2-digit",
  minute: "2-digit",
  second: "2-digit",
  hourCycle: "h23",
});

function QueryLog({
  entries,
  enabled,
}: {
  entries: QueryLogEntry[];
  enabled: boolean;
}) {
  const [query, setQuery] = useState("");

  const shown = useMemo(() => {
    const needle = query.trim().toLowerCase();
    const matched = needle
      ? entries.filter(
          (entry) =>
            entry.name.includes(needle) ||
            entry.recordType.toLowerCase() === needle ||
            entry.outcome === needle,
        )
      : entries;
    // Newest first: the log is watched for what is happening now.
    return [...matched].reverse();
  }, [entries, query]);

  return (
    <Tile className="min-h-64 flex-1 gap-3 pb-2">
      <TileLabel
        icon={<Search />}
        action={
          enabled ? (
            <input
              value={query}
              onChange={(event) => setQuery(event.target.value)}
              placeholder="Filter by name, type or result"
              aria-label="Filter lookups"
              className="bg-surface-inset border-border-subtle focus:border-accent text-text-primary placeholder:text-text-muted h-8 w-60 rounded-full border px-3.5 text-[13px] outline-none"
            />
          ) : null
        }
      >
        Lookups
      </TileLabel>

      {!enabled ? (
        <EmptyState
          icon={<ListX />}
          title="Nothing to show"
          description="Lookups appear here while you are connected over DoH, DoT, DoQ or DoH3."
        />
      ) : shown.length === 0 ? (
        <EmptyState
          icon={<ListX />}
          title={query ? "No lookups match" : "No lookups yet"}
          description={
            query
              ? "Try part of a domain name, a record type like AAAA, or a result like failed."
              : "Open a page and they will appear here."
          }
        />
      ) : (
        <ol className="fc-scroll -mx-2 max-h-[420px] overflow-y-auto">
          {shown.map((entry) => (
            <LogRow key={entry.id} entry={entry} />
          ))}
        </ol>
      )}
    </Tile>
  );
}

function LogRow({ entry }: { entry: QueryLogEntry }) {
  const [label, tone] = OUTCOME[entry.outcome];
  const negative = entry.responseCode && entry.responseCode !== "NoError";

  return (
    <li className="hover:bg-surface-hover flex h-9 items-center gap-3 rounded-lg px-2 text-[13px]">
      <span className="text-text-faint w-[4.5rem] shrink-0 font-mono text-xs tabular-nums">
        {entry.at === null ? "—" : time.format(entry.at)}
      </span>
      <span
        className="min-w-0 flex-1 truncate font-mono text-xs"
        title={entry.name}
      >
        {entry.name}
      </span>
      <span className="text-text-muted w-12 shrink-0 font-mono text-[11px]">
        {entry.recordType}
      </span>
      <span className="text-text-muted w-16 shrink-0 text-right text-xs">
        {negative ? entry.responseCode : ""}
      </span>
      <span
        className={cn(
          "w-[4.5rem] shrink-0 rounded-full py-0.5 text-center text-[11px] font-semibold",
          tone,
        )}
      >
        {label}
      </span>
      <span className="text-text-muted w-16 shrink-0 text-right font-mono text-xs tabular-nums">
        {entry.outcome === "cached" || entry.outcome === "blocked"
          ? "—"
          : `${Math.round(entry.elapsedMs ?? 0)} ms`}
      </span>
    </li>
  );
}
