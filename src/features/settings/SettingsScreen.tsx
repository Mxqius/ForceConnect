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
  Loader2,
  RefreshCw,
  TriangleAlert,
} from "lucide-react";
import { useEffect, useState, type ReactNode } from "react";

import {
  Badge,
  Button,
  Progress,
  Select,
  StatusDot,
  Switch,
  toast,
  type SelectOption,
} from "@/design/primitives";
import { useLayoutPrefs } from "@/app/useLayoutPrefs";
import { useUpdater, useUpdaterStatus } from "@/features/settings/useUpdater";
import { cn } from "@/lib/cn";
import { ipc } from "@/lib/ipc";
import { queryKeys } from "@/lib/query-keys";

const NAV_OPTIONS: SelectOption<"rail" | "dock">[] = [
  { value: "rail", label: "Side rail", hint: "Navigation on the left" },
  { value: "dock", label: "Bottom dock", hint: "Navigation along the bottom" },
];

const THEME_OPTIONS: SelectOption<"dark" | "light">[] = [
  { value: "dark", label: "Dark" },
  { value: "light", label: "Light" },
];

export function SettingsScreen() {
  const navLayout = useLayoutPrefs((state) => state.navLayout);
  const setNavLayout = useLayoutPrefs((state) => state.setNavLayout);
  const theme = useLayoutPrefs((state) => state.theme);
  const setTheme = useLayoutPrefs((state) => state.setTheme);
  const minimizeToTray = useLayoutPrefs((state) => state.minimizeToTray);
  const setMinimizeToTray = useLayoutPrefs((state) => state.setMinimizeToTray);

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

  return (
    <div className="mx-auto flex max-w-2xl flex-col gap-9 p-8">
      <header>
        <h1 className="text-xl font-semibold tracking-tight">Settings</h1>
      </header>

      <Section title="Appearance">
        <Row
          label="Navigation"
          hint="Where the main navigation sits. Also switchable from the icon at the end of the bar."
        >
          <Select
            label="Navigation position"
            value={navLayout}
            onValueChange={(value) => setNavLayout(value ?? "rail")}
            options={NAV_OPTIONS}
            className="w-48"
          />
        </Row>

        <Row
          label="Theme"
          hint="Both are neutral greys, so the only colour on screen carries meaning."
        >
          <Select
            label="Theme"
            value={theme}
            onValueChange={(value) => setTheme(value ?? "dark")}
            options={THEME_OPTIONS}
            className="w-48"
          />
        </Row>
      </Section>

      <Section title="Startup">
        <Row
          label="Start with Windows"
          hint="Launches ForceConnect when you sign in. It does not connect by itself."
        >
          <Switch
            checked={autostart}
            onCheckedChange={(checked) => void toggleAutostart(checked)}
          />
        </Row>

        <Row
          label="Minimize to tray"
          hint="Closing the window sends ForceConnect to the system tray instead of quitting. Your connection keeps running — use the tray icon's Quit to actually exit."
        >
          <Switch
            checked={minimizeToTray}
            onCheckedChange={(checked) => setMinimizeToTray(checked)}
          />
        </Row>
      </Section>

      <Section title="Updates">
        <UpdateRows />
      </Section>

      <Section title="Privileges">
        <Row
          label="Administrator rights"
          hint="Changing an adapter's DNS and flushing the resolver cache both need them. ForceConnect asks Windows once, when it starts, and holds them for that session only."
        >
          <Badge tone={info?.elevated ? "success" : "danger"}>
            <StatusDot tone={info?.elevated ? "success" : "danger"} />
            {info?.elevated ? "granted" : "not granted"}
          </Badge>
        </Row>

        <Row
          label="Process"
          hint="One process, not two. This is the id to look for in Task Manager if you want to confirm what is running elevated."
        >
          <Badge tone="neutral" mono>
            pid {info?.pid ?? "—"}
          </Badge>
        </Row>
      </Section>

      <p className="text-text-faint text-xs">
        ForceConnect {info?.version ?? "—"} · {info?.os ?? ""}
      </p>
    </div>
  );
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
        label="Check for updates on launch"
        hint="Looks for a newer release when ForceConnect starts. It never downloads or installs anything on its own."
      >
        <Switch
          checked={autoUpdate}
          onCheckedChange={(checked) => setAutoUpdate(checked)}
        />
      </Row>

      <div className="flex flex-col gap-3 px-5 py-4">
        <div className="flex items-center justify-between gap-8">
          <div className="min-w-0">
            <p className="text-sm font-medium">Status</p>
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
                startIcon={<Download />}
                onClick={() => void install()}
              >
                Install {stage.version}
              </Button>
            ) : null}

            <Button
              size="sm"
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
          <p className="border-border-subtle bg-surface-inset text-text-muted fc-scroll max-h-32 overflow-y-auto rounded-lg border px-3 py-2 text-xs leading-relaxed whitespace-pre-line">
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
  const line = "text-text-muted mt-1 flex items-center gap-1.5 text-xs";

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

function Section({ title, children }: { title: string; children: ReactNode }) {
  return (
    <section className="flex flex-col gap-1">
      <h2 className="text-text-secondary mb-3 text-xs font-semibold tracking-widest uppercase">
        {title}
      </h2>
      <div className="border-border-subtle bg-surface divide-border-subtle divide-y rounded-2xl border">
        {children}
      </div>
    </section>
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
    <div className="flex items-center justify-between gap-8 px-5 py-4">
      <div className="min-w-0">
        <p className="text-sm font-medium">{label}</p>
        {hint ? <p className="text-text-muted mt-1 text-xs">{hint}</p> : null}
      </div>
      <div className="shrink-0">{children}</div>
    </div>
  );
}
