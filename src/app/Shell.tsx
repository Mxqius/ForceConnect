import { useQuery } from "@tanstack/react-query";
import { isTauri } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  Info,
  Network,
  Settings,
  Shield,
  ShieldAlert,
  Zap,
  type LucideIcon,
} from "lucide-react";
import { useEffect } from "react";
import { NavLink, Outlet } from "react-router";

import { Tooltip } from "@/design/primitives";
import { TitleBar } from "@/app/TitleBar";
import { useHydrateLayoutPrefs, useLayoutPrefs } from "@/app/useLayoutPrefs";
import {
  useConnection,
  useConnectionPhase,
  type ConnectPhase,
} from "@/features/connection/useConnection";
import { useHydrateTargetAdapter } from "@/features/interfaces/useAdapters";
import { useLaunchUpdateCheck } from "@/features/settings/useUpdater";
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

/**
 * The connection state, carried on the Connection item so it is visible from
 * every screen — the hero tile is the only other place it shows, and that tile
 * is gone the moment you open Servers.
 */
const PHASE_DOT: Partial<Record<ConnectPhase, string>> = {
  connected: "bg-success",
  connecting: "bg-accent-bright animate-pulse",
  switching: "bg-accent-bright animate-pulse",
  disconnecting: "bg-text-muted animate-pulse",
  failed: "bg-danger",
};

export function Shell() {
  useHydrateLayoutPrefs();
  useHydrateTargetAdapter();
  useLaunchUpdateCheck();
  useTrayDisconnect();
  const navLayout = useLayoutPrefs((state) => state.navLayout);
  const phase = useConnectionPhase();

  const docked = navLayout === "dock";

  return (
    <div className="bg-surface-base text-text-primary flex h-full flex-col">
      <TitleBar />
      <ElevationNotice />

      <div className="relative flex min-h-0 flex-1">
        {docked ? null : <Rail phase={phase} />}

        <main
          // Screens pad their own bottom edge by this much, so their last row
          // can scroll clear of the floating dock instead of hiding under it.
          style={
            {
              "--dock-clearance": docked ? "104px" : "32px",
            } as React.CSSProperties
          }
          className="fc-scroll min-w-0 flex-1 overflow-y-auto"
        >
          <Outlet />
        </main>

        {docked ? <Dock phase={phase} /> : null}
      </div>
    </div>
  );
}

function Dock({ phase }: { phase: ConnectPhase }) {
  return (
    <nav
      aria-label="Main"
      className="bg-surface-elevated/90 shadow-dock absolute bottom-5 left-1/2 z-30 flex -translate-x-1/2 gap-1 rounded-full p-1.5 backdrop-blur-xl"
    >
      {NAV.map((item) => (
        <NavLink
          key={item.to}
          to={item.to}
          end={item.end}
          aria-label={item.label}
          className={({ isActive }) =>
            cn(
              "flex h-11 min-w-11 items-center justify-center gap-2 rounded-full px-3.5 text-sm font-semibold transition-[background-color,color,padding] duration-200",
              "focus-visible:outline-accent outline-none focus-visible:outline-2 focus-visible:outline-offset-2",
              isActive
                ? "bg-control-strong text-control-strong-ink px-5"
                : "text-text-muted hover:bg-surface-hover hover:text-text-primary",
            )
          }
        >
          {({ isActive }) => (
            <>
              <NavIcon item={item} phase={phase} />
              {isActive ? <span>{item.label}</span> : null}
            </>
          )}
        </NavLink>
      ))}
    </nav>
  );
}

function Rail({ phase }: { phase: ConnectPhase }) {
  return (
    <nav
      aria-label="Main"
      className="bg-surface rounded-tile shadow-tile border-border-subtle my-6 ml-6 flex shrink-0 flex-col gap-1.5 border p-1.5"
    >
      {NAV.map((item) => (
        <Tooltip key={item.to} content={item.label} side="right">
          <NavLink
            to={item.to}
            end={item.end}
            aria-label={item.label}
            className={({ isActive }) =>
              cn(
                "flex size-12 items-center justify-center rounded-[18px] transition-colors duration-200",
                "focus-visible:outline-accent outline-none focus-visible:outline-2 focus-visible:outline-offset-2",
                isActive
                  ? "bg-control-strong text-control-strong-ink"
                  : "text-text-muted hover:bg-surface-hover hover:text-text-primary",
              )
            }
          >
            <NavIcon item={item} phase={phase} />
          </NavLink>
        </Tooltip>
      ))}
    </nav>
  );
}

function NavIcon({ item, phase }: { item: NavItem; phase: ConnectPhase }) {
  const Icon = item.icon;
  const dot = item.to === "/" ? PHASE_DOT[phase] : undefined;

  return (
    <span className="relative flex">
      <Icon className="size-5 shrink-0" />
      {dot ? (
        <span
          aria-hidden
          className={cn(
            "ring-surface-elevated absolute -top-1 -right-1 size-2 rounded-full ring-2",
            dot,
          )}
        />
      ) : null}
    </span>
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

    const unlisten = listen("tray://disconnect", () => void disconnect());
    return () => void unlisten.then((stop) => stop());
  }, [disconnect]);
}

/**
 * Says so when the app is running without administrator rights.
 *
 * On Windows this should be impossible in a shipped build — the executable
 * embeds a `requireAdministrator` manifest, so Windows either grants the rights
 * at launch or the app does not start. It happens there in two real cases: a
 * developer build on a toolchain that could not embed the manifest, and a
 * binary someone has stripped it from. On Linux and macOS it is the ordinary
 * result of cancelling the administrator prompt at launch.
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
        administrator prompt.
      </p>
    </div>
  );
}
