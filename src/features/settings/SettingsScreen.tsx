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
  { value: "dark", label: "Dark", hint: "The design target" },
  { value: "light", label: "Light", hint: "Supported, secondary" },
];

export function SettingsScreen() {
  const navLayout = useLayoutPrefs((state) => state.navLayout);
  const setNavLayout = useLayoutPrefs((state) => state.setNavLayout);
  const theme = useLayoutPrefs((state) => state.theme);
  const setTheme = useLayoutPrefs((state) => state.setTheme);

  const { data: info } = useQuery({
    queryKey: queryKeys.appInfo,
    queryFn: ipc.appInfo,
  });

  const { data: helper } = useQuery({
    queryKey: queryKeys.helper,
    queryFn: ipc.helperStatus,
    refetchInterval: 5000,
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
    <div className="mx-auto flex max-w-2xl flex-col gap-8 p-6">
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
            className="w-44"
          />
        </Row>

        <Row label="Theme" hint="Dark is the design target.">
          <Select
            label="Theme"
            value={theme}
            onValueChange={(value) => setTheme(value ?? "dark")}
            options={THEME_OPTIONS}
            className="w-44"
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
      </Section>

      <Section title="Privileges">
        <Row
          label="Helper process"
          hint="Changing DNS needs administrator rights. The interface never runs elevated; a small helper does, and only while the app is open."
        >
          {helper?.connected ? (
            <Badge tone="success">running · pid {helper.pid ?? "?"}</Badge>
          ) : (
            <Badge tone="neutral">not started</Badge>
          )}
        </Row>

        <Row
          label="This process"
          hint="Expected to be unelevated. Anything else means the app was started as administrator."
        >
          <Badge tone={info?.elevated ? "warning" : "neutral"}>
            {info?.elevated ? "elevated" : "standard"}
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
      <h2 className="text-text-secondary mb-2 text-xs font-semibold tracking-widest uppercase">
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
    <div className="flex items-center justify-between gap-6 px-4 py-3.5">
      <div className="min-w-0">
        <p className="text-sm font-medium">{label}</p>
        {hint ? <p className="text-text-muted mt-0.5 text-xs">{hint}</p> : null}
      </div>
      <div className="shrink-0">{children}</div>
    </div>
  );
}
