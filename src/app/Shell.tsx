import {
  Info,
  Network,
  PanelBottom,
  PanelLeft,
  Settings,
  Shield,
  Zap,
  type LucideIcon,
} from "lucide-react";
import { NavLink, Outlet } from "react-router";

import { IconButton, Tooltip } from "@/design/primitives";
import { useHydrateLayoutPrefs, useLayoutPrefs } from "@/app/useLayoutPrefs";
import { cn } from "@/lib/cn";

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
  const navLayout = useLayoutPrefs((state) => state.navLayout);
  const setNavLayout = useLayoutPrefs((state) => state.setNavLayout);

  const docked = navLayout === "dock";

  return (
    <div
      className={cn(
        "bg-surface-base text-text-primary flex h-full",
        docked ? "flex-col" : "flex-row",
      )}
    >
      <nav
        aria-label="Main"
        className={cn(
          "bg-surface-subtle border-border-subtle flex shrink-0 gap-1",
          docked
            ? "order-2 flex-row items-center justify-center rounded-full border-t px-3 py-2 max-w-96 mx-auto mb-5"
            : "flex-col items-center border-r px-2 py-3",
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
                  docked ? "h-10 gap-2 px-3.5" : "size-11",
                  isActive
                    ? "bg-accent text-text-on-accent shadow-[0_4px_16px_-6px_var(--accent-glow)]"
                    : "text-text-muted hover:bg-surface-hover hover:text-text-primary",
                )
              }
            >
              {({ isActive }) => (
                <>
                  <Icon className="size-[18px] shrink-0" />
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
            content={docked ? "Move navigation to the side" : "Dock to bottom"}
            side={docked ? "top" : "right"}
          >
            <IconButton
              label={docked ? "Move navigation to the side" : "Dock to bottom"}
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
  );
}
