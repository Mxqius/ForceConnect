import {
  Info,
  Network,
  Settings,
  Shield,
  Zap,
  type LucideIcon,
} from "lucide-react";
import { NavLink, Outlet } from "react-router";

import { Tooltip } from "@/design/primitives";
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
  return (
    <div className="bg-surface-base text-text-primary flex h-full">
      <nav
        aria-label="Main"
        className="border-border-subtle bg-surface-subtle flex w-14 shrink-0 flex-col items-center gap-1 border-r py-3"
      >
        {NAV.map(({ to, icon: Icon, label, end }) => (
          <Tooltip key={to} content={label} side="right">
            <NavLink
              to={to}
              end={end}
              aria-label={label}
              className={({ isActive }) =>
                cn(
                  "relative flex size-10 items-center justify-center rounded-md transition-colors",
                  "focus-visible:outline-accent outline-none focus-visible:outline-2 focus-visible:outline-offset-2",
                  isActive
                    ? "bg-accent-surface text-accent"
                    : "text-text-muted hover:bg-surface-hover hover:text-text-primary",
                )
              }
            >
              {({ isActive }) => (
                <>
                  <Icon className="size-[18px]" />
                  {/* A rail this narrow needs more than colour to signal the
                      active route, especially for anyone who cannot easily
                      distinguish the accent from the muted foreground. */}
                  {isActive ? (
                    <span className="bg-accent absolute -left-3 h-5 w-[3px] rounded-r-full" />
                  ) : null}
                </>
              )}
            </NavLink>
          </Tooltip>
        ))}
      </nav>

      <main className="fc-scroll min-w-0 flex-1 overflow-y-auto">
        <Outlet />
      </main>
    </div>
  );
}
