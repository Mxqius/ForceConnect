import { useQuery } from "@tanstack/react-query";
import {
  disable as disableAutostart,
  enable as enableAutostart,
  isEnabled as autostartEnabled,
} from "@tauri-apps/plugin-autostart";
import { isTauri } from "@tauri-apps/api/core";
import {
  CheckCircle2,
  Download,
  History,
  LifeBuoy,
  Loader2,
  Minimize2,
  Moon,
  Paintbrush,
  Power,
  RefreshCw,
  ShieldAlert,
  ShieldCheck,
  Sun,
  TriangleAlert,
} from "lucide-react";
import { useEffect, useState, type ReactNode } from "react";

import {
  Button,
  PageHeader,
  pagePadding,
  Progress,
  Segmented,
  Select,
  Switch,
  Tile,
  TileLabel,
  toast,
  type SegmentedOption,
  type SelectOption,
} from "@/design/primitives";
import { useLayoutPrefs } from "@/app/useLayoutPrefs";
import { useReleasePolicy } from "@/features/connection/useReleasePolicy";
import { useDnsTarget } from "@/features/interfaces/useAdapters";
import { useServers } from "@/features/servers/useServers";
import { useUpdater, useUpdaterStatus } from "@/features/settings/useUpdater";
import { cn } from "@/lib/cn";
import { ipc, type FamilyDns, type Server } from "@/lib/ipc";
import type { DefaultDns, MotionPref, NavLayout } from "@/lib/prefs";
import { queryKeys } from "@/lib/query-keys";

const NAV_OPTIONS: SegmentedOption<NavLayout>[] = [
  { value: "dock", label: "Dock" },
  { value: "rail", label: "Sidebar" },
];

const THEME_OPTIONS: SegmentedOption<"dark" | "light">[] = [
  { value: "dark", label: "Dark", icon: <Moon /> },
  { value: "light", label: "Light", icon: <Sun /> },
];

const MOTION_OPTIONS: SegmentedOption<MotionPref>[] = [
  { value: "full", label: "Full" },
  { value: "reduced", label: "Reduced" },
];

export function SettingsScreen() {
  const navLayout = useLayoutPrefs((state) => state.navLayout);
  const setNavLayout = useLayoutPrefs((state) => state.setNavLayout);
  const theme = useLayoutPrefs((state) => state.theme);
  const setTheme = useLayoutPrefs((state) => state.setTheme);
  const minimizeToTray = useLayoutPrefs((state) => state.minimizeToTray);
  const setMinimizeToTray = useLayoutPrefs((state) => state.setMinimizeToTray);
  const motion = useLayoutPrefs((state) => state.motion);
  const setMotion = useLayoutPrefs((state) => state.setMotion);
  const setCompact = useLayoutPrefs((state) => state.setCompact);

  const { data: info } = useQuery({
    queryKey: queryKeys.appInfo,
    queryFn: ipc.appInfo,
  });

  const [autostart, setAutostart] = useState(false);

  useEffect(() => {
    if (!isTauri()) return;
    void autostartEnabled()
      .then(setAutostart)
      .catch(() => undefined);
  }, []);

  async function toggleAutostart(next: boolean) {
    try {
      await (next ? enableAutostart() : disableAutostart());
      setAutostart(next);
    } catch (error) {
      toast.error("Could not change the startup setting", {
        description: error instanceof Error ? error.message : String(error),
      });
    }
  }

  // `undefined` while loading reads as granted: a shipped build cannot start
  // without the rights, and flashing a warning on every launch would be wrong
  // far more often than right.
  const notElevated = info?.elevated === false;

  return (
    <div className={cn("flex flex-col", pagePadding)}>
      <PageHeader
        title="Settings"
        description={
          <>
            ForceConnect {info?.version ?? "—"}
            {info?.os ? ` · ${info.os}` : ""}
          </>
        }
      />

      <div className="grid grid-cols-2 gap-4">
        <Section title="Appearance" icon={<Paintbrush />} wide>
          <Row label="Theme" hint="Both keep colour for what carries meaning">
            <Segmented
              label="Theme"
              value={theme}
              onValueChange={setTheme}
              options={THEME_OPTIONS}
            />
          </Row>
          <Row label="Navigation" hint="A floating dock, or a sidebar">
            <Segmented
              label="Navigation position"
              value={navLayout}
              onValueChange={setNavLayout}
              options={NAV_OPTIONS}
            />
          </Row>
          <Row
            label="Motion"
            hint="Reduced swaps the connect bloom and ripples for simple fades"
          >
            <Segmented
              label="Motion"
              value={motion}
              onValueChange={setMotion}
              options={MOTION_OPTIONS}
            />
          </Row>
          <Row
            label="Compact window"
            hint="A small panel with just the connection, latency and server. Lighter on memory and CPU."
          >
            <Button
              size="sm"
              className="rounded-full"
              startIcon={<Minimize2 />}
              onClick={() => setCompact(true)}
            >
              Switch to compact
            </Button>
          </Row>
        </Section>

        <Section title="Startup" icon={<Power />}>
          <Row label="Start at login" hint="It does not connect by itself">
            <Switch
              checked={autostart}
              onCheckedChange={(checked) => void toggleAutostart(checked)}
            />
          </Row>
          <Row
            label="Close to tray"
            hint="Keeps your DNS running; quit from the tray icon"
          >
            <Switch
              checked={minimizeToTray}
              onCheckedChange={(checked) => setMinimizeToTray(checked)}
            />
          </Row>
        </Section>

        <Section
          title="Privileges"
          icon={notElevated ? <ShieldAlert /> : <ShieldCheck />}
        >
          <Row
            label="Administrator"
            hint="Needed to change adapter DNS. Asked for at each launch."
          >
            <Pill tone={notElevated ? "danger" : "success"}>
              {notElevated ? "Not granted" : "Granted"}
            </Pill>
          </Row>
          <Row label="Process" hint="Look for this id in your task manager">
            <Pill tone="neutral" mono>
              pid {info?.pid ?? "—"}
            </Pill>
          </Row>
        </Section>

        <Section title="When you disconnect" icon={<LifeBuoy />} wide>
          <ReleaseRows />
        </Section>

        <Section title="Updates" icon={<Download />} wide>
          <UpdateRows />
        </Section>
      </div>
    </div>
  );
}

/**
 * Where the adapter goes when ForceConnect lets go of it.
 *
 * Shown as two switches because that is how people think about it — "put my
 * old DNS back", "fall back to my own DNS" — but they are one three-way
 * choice underneath, so turning one on turns the other off. With both off the
 * adapter goes back to DHCP.
 *
 * The same choice applies on disconnect, when the app closes, and after a
 * crash; the backend writes it into the recovery journal for that last case.
 */
function ReleaseRows() {
  const mode = useReleasePolicy((state) => state.mode);
  const setMode = useReleasePolicy((state) => state.setMode);
  const defaultDns = useReleasePolicy((state) => state.defaultDns);
  const setDefaultDns = useReleasePolicy((state) => state.setDefaultDns);
  const { data: servers } = useServers();
  const { adapter } = useDnsTarget();

  const { data: baseline } = useQuery({
    queryKey: queryKeys.dnsBaseline,
    queryFn: ipc.dnsBaseline,
    // Captured once at launch and never changes for the life of the process.
    staleTime: Infinity,
  });
  const before =
    baseline?.find((entry) => entry.luid === adapter?.luid) ?? baseline?.[0];

  const candidates = (servers ?? [])
    .map((server) => toDefaultDns(server))
    .filter((dns): dns is DefaultDns => dns !== null);
  const options: SelectOption<string>[] = candidates.map((dns) => ({
    value: dns.serverId,
    label: dns.name,
    hint: dns.addrs.join(", "),
  }));

  function toggleFallback(on: boolean) {
    if (on && !defaultDns && candidates[0]) setDefaultDns(candidates[0]);
    setMode(on ? "fallback" : "automatic");
  }

  return (
    <>
      <div>
        <Row
          label="Restore previous DNS"
          hint="Put back the DNS your system had before ForceConnect started"
        >
          <Switch
            checked={mode === "restore"}
            onCheckedChange={(on) => setMode(on ? "restore" : "automatic")}
          />
        </Row>
        {mode === "restore" ? (
          <div className="bg-surface-inset text-text-secondary mb-3 flex items-center gap-2.5 rounded-xl px-3.5 py-2.5 text-[13px]">
            <History className="text-text-muted size-4 shrink-0" />
            {before ? (
              <span className="min-w-0 truncate">
                <span className="text-text-muted">
                  Saved at launch · {before.name} ·{" "}
                </span>
                <span className="font-mono text-xs">
                  {describeFamily("IPv4", before.v4)}
                  {before.v6 ? ` · ${describeFamily("IPv6", before.v6)}` : ""}
                </span>
              </span>
            ) : (
              <span className="text-text-muted">
                No adapter DNS was captured at launch, so disconnecting falls
                back to automatic DNS.
              </span>
            )}
          </div>
        ) : null}
      </div>

      <div>
        <Row
          label="Use a default DNS"
          hint="Switch to a server you choose on disconnect and when ForceConnect closes"
        >
          <Switch
            checked={mode === "fallback"}
            disabled={candidates.length === 0}
            onCheckedChange={toggleFallback}
          />
        </Row>
        {mode === "fallback" ? (
          <div className="mb-3 flex items-center gap-4">
            <span className="text-text-muted w-28 shrink-0 text-[13px]">
              Default server
            </span>
            <Select
              label="Default DNS server"
              value={defaultDns?.serverId ?? null}
              options={options}
              placeholder="Pick a server"
              onValueChange={(id) => {
                const next = candidates.find((dns) => dns.serverId === id);
                if (next) setDefaultDns(next);
              }}
            />
          </div>
        ) : null}
      </div>

      {mode === "automatic" ? (
        <p className="text-text-muted py-3 text-[13px] leading-snug">
          With both off, disconnecting hands the adapter back to automatic DNS
          from your network (DHCP).
        </p>
      ) : null}
    </>
  );
}

/**
 * A server as a default DNS: its plain addresses, or nothing.
 *
 * Only plain DNS can be a default. It is used once ForceConnect has closed,
 * and Windows can only point an adapter at bare IP addresses — the encrypted
 * protocols need the app's own resolver running.
 */
function toDefaultDns(server: Server): DefaultDns | null {
  const plain =
    server.endpoints.find((e) => e.protocol === "udp") ??
    server.endpoints.find((e) => e.protocol === "tcp");
  if (!plain || plain.requiresForwarder || plain.addrs.length === 0)
    return null;
  return {
    serverId: server.id,
    name: server.displayName,
    addrs: plain.addrs,
  };
}

function describeFamily(label: string, dns: FamilyDns | null): string {
  if (!dns) return `${label} unknown`;
  if (dns.source === "dhcp" || dns.servers.length === 0)
    return `${label} automatic`;
  return `${label} ${dns.servers.join(", ")}`;
}

/**
 * The update controls.
 *
 * Two separate decisions, kept separate: whether ForceConnect *looks* for a new
 * version on its own, and whether it installs one. It never does the second
 * without being asked — the app runs elevated and the update replaces the
 * running binary, so the install is a deliberate click every time.
 */
function UpdateRows() {
  const { stage, autoUpdate, setAutoUpdate, check, install, isBusy } =
    useUpdater();
  const { data: status } = useUpdaterStatus();

  // An unsigned build reaches the endpoint fine and then refuses the download
  // at verification. Better to say so here than to offer a button that is
  // guaranteed to fail at the last step.
  const canInstall = status?.configured ?? true;

  return (
    <>
      <Row
        label="Check on launch"
        hint="Looks for a newer release at startup. It never installs anything on its own."
      >
        <Switch
          checked={autoUpdate}
          onCheckedChange={(checked) => setAutoUpdate(checked)}
        />
      </Row>

      <div className="flex flex-col gap-3 py-3">
        <div className="flex items-center justify-between gap-6">
          <div className="min-w-0">
            <p className="text-sm font-semibold">Status</p>
            <UpdateStatusLine
              stage={stage}
              canInstall={canInstall}
              endpoint={status?.endpoints[0]}
            />
          </div>

          <div className="flex shrink-0 items-center gap-2">
            {stage.kind === "available" && canInstall ? (
              <Button
                size="sm"
                variant="primary"
                className="rounded-full"
                startIcon={<Download />}
                onClick={() => void install()}
              >
                Install {stage.version}
              </Button>
            ) : null}

            <Button
              size="sm"
              className="rounded-full"
              startIcon={
                <RefreshCw
                  className={cn(stage.kind === "checking" && "animate-spin")}
                />
              }
              disabled={isBusy}
              onClick={() => void check()}
            >
              {stage.kind === "checking" ? "Checking…" : "Check now"}
            </Button>
          </div>
        </div>

        {stage.kind === "downloading" ? (
          <Progress
            label={`Downloading ${stage.version}`}
            value={stage.percent}
            max={100}
          />
        ) : null}

        {stage.kind === "available" && stage.notes ? (
          <p className="bg-surface-inset text-text-muted fc-scroll max-h-32 overflow-y-auto rounded-xl px-3 py-2 text-xs leading-relaxed whitespace-pre-line">
            {stage.notes}
          </p>
        ) : null}
      </div>
    </>
  );
}

function UpdateStatusLine({
  stage,
  canInstall,
  endpoint,
}: {
  stage: ReturnType<typeof useUpdater>["stage"];
  canInstall: boolean;
  endpoint: string | undefined;
}) {
  const line = "text-text-muted mt-0.5 flex items-center gap-1.5 text-[13px]";

  if (!canInstall && stage.kind !== "error") {
    return (
      <p className={cn(line, "text-warning")}>
        <TriangleAlert className="size-3.5 shrink-0" />
        This build has no update signing key, so it cannot install updates.
        {endpoint ? " Releases are published on GitHub." : ""}
      </p>
    );
  }

  switch (stage.kind) {
    case "idle":
      return <p className={line}>Not checked yet this session.</p>;

    case "checking":
      return (
        <p className={line}>
          <Loader2 className="size-3.5 shrink-0 animate-spin" />
          Looking for a newer release…
        </p>
      );

    case "current":
      return (
        <p className={cn(line, "text-success")}>
          <CheckCircle2 className="size-3.5 shrink-0" />
          You are on the latest version.
        </p>
      );

    case "available":
      return (
        <p className={cn(line, "text-accent-bright")}>
          Version {stage.version} is available
          {stage.date ? ` (${stage.date.slice(0, 10)})` : ""}.
        </p>
      );

    case "downloading":
      return (
        <p className={line}>
          <Loader2 className="size-3.5 shrink-0 animate-spin" />
          Downloading {stage.version}
          {stage.percent === null ? "…" : ` — ${stage.percent}%`}
        </p>
      );

    case "installing":
      return (
        <p className={line}>
          <Loader2 className="size-3.5 shrink-0 animate-spin" />
          Installing {stage.version}. ForceConnect will close and reopen on the
          new version.
        </p>
      );

    case "error":
      return (
        <p className={cn(line, "text-danger")}>
          <TriangleAlert className="size-3.5 shrink-0" />
          {stage.message}
        </p>
      );
  }
}

function Section({
  title,
  icon,
  wide,
  children,
}: {
  title: string;
  icon: ReactNode;
  wide?: boolean;
  children: ReactNode;
}) {
  return (
    <Tile className={cn("pt-5 pb-2", wide && "col-span-2")}>
      <TileLabel icon={icon} className="mb-1 font-semibold">
        {title}
      </TileLabel>
      <div className="[&>*+*]:border-border-subtle flex flex-col [&>*+*]:border-t">
        {children}
      </div>
    </Tile>
  );
}

function Row({
  label,
  hint,
  children,
}: {
  label: string;
  hint?: string;
  children: ReactNode;
}) {
  return (
    <div className="flex min-h-14 items-center justify-between gap-6 py-3">
      <div className="min-w-0">
        <p className="text-sm font-semibold">{label}</p>
        {hint ? (
          <p className="text-text-muted mt-0.5 text-[13px] leading-snug">
            {hint}
          </p>
        ) : null}
      </div>
      <div className="shrink-0">{children}</div>
    </div>
  );
}

function Pill({
  tone,
  mono,
  children,
}: {
  tone: "success" | "danger" | "neutral";
  mono?: boolean;
  children: ReactNode;
}) {
  return (
    <span
      className={cn(
        "rounded-full px-2.5 py-0.5 text-[11px] font-bold",
        mono && "font-mono",
        tone === "success" && "bg-success-surface text-success",
        tone === "danger" && "bg-danger-surface text-danger",
        tone === "neutral" && "bg-surface-inset text-text-secondary",
      )}
    >
      {children}
    </span>
  );
}
