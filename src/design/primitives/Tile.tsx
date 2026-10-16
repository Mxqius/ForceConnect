import type { HTMLAttributes, ReactNode } from "react";

import { cn } from "@/lib/cn";

/**
 * The bento tile: the one container every screen is built from.
 *
 * No border in the dark theme does the outlining — the step from the window to
 * the tile is large enough on its own — so the hairline here is only there to
 * keep edges crisp where two tiles' corners meet. In light, the shadow lifts it.
 */
export function Tile({ className, ...props }: HTMLAttributes<HTMLElement>) {
  return (
    <section
      className={cn(
        "bg-surface border-border-subtle rounded-tile shadow-tile relative flex flex-col overflow-hidden border px-6 py-5",
        className,
      )}
      {...props}
    />
  );
}

/** The small icon-and-caption line at the top of a tile. */
export function TileLabel({
  icon,
  action,
  children,
  className,
}: {
  icon?: ReactNode;
  action?: ReactNode;
  children: ReactNode;
  className?: string;
}) {
  return (
    <div
      className={cn(
        "text-text-muted flex min-h-7 items-center gap-2 text-[13px] font-medium [&>svg]:size-4 [&>svg]:shrink-0",
        className,
      )}
    >
      {icon}
      <span className="min-w-0 truncate">{children}</span>
      {action ? <span className="ml-auto flex shrink-0">{action}</span> : null}
    </div>
  );
}
