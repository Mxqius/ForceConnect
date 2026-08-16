import { useQuery } from "@tanstack/react-query";
import { isTauri } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  Info,
  Network,
  PanelBottom,
  PanelLeft,
  Settings,
  Shield,
  ShieldAlert,
  Zap,
  type LucideIcon,
} from "lucide-react";
import { useEffect } from "react";
import { NavLink, Outlet } from "react-router";

import { IconButton, Tooltip } from "@/design/primitives";
import { TitleBar } from "@/app/TitleBar";
import { useHydrateLayoutPrefs, useLayoutPrefs } from "@/app/useLayoutPrefs";
import { useConnection } from "@/features/connection/useConnection";
import { cn } from "@/lib/cn";
import { ipc } from "@/lib/ipc";
import { queryKeys } from "@/lib/query-keys";

interface NavItem {
  to: string;
  icon: LucideIcon;
  label: string;
  end?: boolean;
}

const NAV: NavItem[] = [
  { to: "/", icon: Zap, label: "Connection", end: true },
  { to: "/servers", icon: Shield, label: "Servers" },
  { to: "/interfaces", icon: Network, label: "Interfaces" },
  { to: "/settings", icon: Settings, label: "Settings" },
  { to: "/about", icon: Info, label: "About" },
];

export function Shell() {
  useHydrateLayoutPrefs();
  useTrayDisconnect();
  const navLayout = useLayoutPrefs((state) => state.navLayout);
  const setNavLayout = useLayoutPrefs((state) => state.setNavLayout);

  const docked = navLayout === "dock";

  return (
    <div className="bg-surface-base text-text-primary flex h-full flex-col">
      <TitleBar />
      <ElevationNotice />

      <div
        className={cn("flex min-h-0 flex-1", docked ? "flex-col" : "flex-row")}
      >
        <nav
          aria-label="Main"
          className={cn(
            "bg-surface-subtle border-border-subtle flex shrink-0 gap-1.5",
            docked
              ? "order-2 mx-auto mb-6 max-w-[28rem] flex-row items-center justify-center rounded-full border-t px-3 py-2.5"
              : "flex-col items-center border-r px-2.5 py-4",
          )}
        >
          {NAV.map(({ to, icon: Icon, label, end }) => (
            <Tooltip key={to} content={label} side={docked ? "top" : "right"}>
              <NavLink
                to={to}
                end={end}
                aria-label={label}
                className={({ isActive }) =>
                  cn(
                    "relative flex items-center justify-center rounded-xl transition-all duration-200",
                    "focus-visible:outline-accent outline-none focus-visible:outline-2 focus-visible:outline-offset-2",
                    docked ? "h-11 gap-2 px-4" : "size-12",
                    isActive
                      ? "bg-accent text-text-on-accent shadow-[0_4px_16px_-6px_var(--accent-glow)]"
                      : "text-text-muted hover:bg-surface-hover hover:text-text-primary",
                  )
                }
              >
                {({ isActive }) => (
                  <>
                    <Icon className="size-5 shrink-0" />
                    {/* In the dock the label appears alongside the active icon —
                        there is horizontal room there that the rail does not
                        have, and it removes the guesswork from a row of glyphs. */}
                    {docked && isActive ? (
                      <span className="text-xs font-medium">{label}</span>
                    ) : null}
                  </>
                )}
              </NavLink>
            </Tooltip>
          ))}

          <div className={cn(docked ? "ml-2" : "mt-auto")}>
            <Tooltip
              content={
                docked ? "Move navigation to the side" : "Dock to bottom"
              }
              side={docked ? "top" : "right"}
            >
              <IconButton
                label={
                  docked ? "Move navigation to the side" : "Dock to bottom"
                }
                size="sm"
                onClick={() => setNavLayout(docked ? "rail" : "dock")}
                className="text-text-faint hover:text-text-secondary"
              >
                {docked ? <PanelLeft /> : <PanelBottom />}
              </IconButton>
            </Tooltip>
          </div>
        </nav>

        <main
          className={cn(
            "fc-scroll min-w-0 flex-1 overflow-y-auto",
            docked && "order-1",
          )}
        >
          <Outlet />
        </main>
      </div>
    </div>
  );
}

/**
 * Lets "Disconnect" on the tray menu actually disconnect.
 *
 * The tray has no adapter handle of its own — only `useConnection`'s store
 * knows which adapter is live — so the menu click arrives here as an event
 * rather than being handled in Rust. Lives in `Shell` rather than the
 * connection screen so it still works while that screen is unmounted, which
 * is the whole point of reaching it from the tray.
 */
function useTrayDisconnect() {
  const { disconnect } = useConnection();

  useEffect(() => {
    if (!isTauri()) return;

    const unlisten = listen("tray://disconnect", () => disconnect());
    return () => void unlisten.then((stop) => stop());
  }, [disconnect]);
}

/**
 * Says so when the app is running without administrator rights.
 *
 * This should be impossible in a shipped build — the executable embeds a
 * `requireAdministrator` manifest, so Windows either grants the rights at launch
 * or the app does not start. It happens in two real cases: a developer build on
 * a toolchain that could not embed the manifest, and a binary someone has
 * stripped it from.
 *
 * Worth a permanent strip rather than a toast, because in that state *every*
 * action on every screen will refuse, and a user who has not been told why will
 * reasonably conclude the app is broken.
 */
function ElevationNotice() {
  const { data } = useQuery({
    queryKey: queryKeys.appInfo,
    queryFn: ipc.appInfo,
  });

  // `undefined` while loading: assuming the worst would flash a scary banner on
  // every launch, and assuming the best is right in the overwhelming majority.
  if (data?.elevated !== false) return null;

  return (
    <div
      role="status"
      className="border-danger/30 bg-danger-surface text-danger flex shrink-0 items-center gap-3 border-b px-5 py-3 text-xs"
    >
      <ShieldAlert className="size-4 shrink-0" />
      <p>
        ForceConnect is running without administrator rights, so it cannot
        change DNS settings. Close it and start it again, then approve the
        Windows prompt.
      </p>
    </div>
  );
}
