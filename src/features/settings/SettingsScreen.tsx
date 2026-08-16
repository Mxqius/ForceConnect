import { useQuery } from "@tanstack/react-query";
import {
  disable as disableAutostart,
  enable as enableAutostart,
  isEnabled as autostartEnabled,
} from "@tauri-apps/plugin-autostart";
import { isTauri } from "@tauri-apps/api/core";
import { useEffect, useState, type ReactNode } from "react";

import {
  Badge,
  Select,
  StatusDot,
  Switch,
  toast,
  type SelectOption,
} from "@/design/primitives";
import { useLayoutPrefs } from "@/app/useLayoutPrefs";
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
