import type { ReactNode } from "react";

import { cn } from "@/lib/cn";

/**
 * The title block every full-page screen opens with.
 *
 * One component so the type scale and the space below it are the same on
 * every screen — they had drifted to four slightly different versions, which
 * is exactly the kind of difference that reads as "something is off" without
 * anyone being able to say what.
 */
export function PageHeader({
  title,
  description,
  actions,
  className,
}: {
  title: ReactNode;
  description?: ReactNode;
  actions?: ReactNode;
  className?: string;
}) {
  return (
    <header className={cn("mb-6 flex items-end gap-4", className)}>
      <div className="min-w-0 flex-1">
        <h1 className="text-2xl font-semibold tracking-tight">{title}</h1>
        {description ? (
          <p className="text-text-secondary mt-1 text-sm">{description}</p>
        ) : null}
      </div>
      {actions ? (
        <div className="flex shrink-0 items-center gap-2.5">{actions}</div>
      ) : null}
    </header>
  );
}
