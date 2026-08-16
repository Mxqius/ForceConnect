import { Hammer } from "lucide-react";

import { EmptyState } from "@/design/primitives";

/**
 * Stands in for a screen that has a route and a nav entry but no
 * implementation yet.
 *
 * Better than omitting the route: the navigation reflects the shape of the
 * finished app, and clicking an unfinished entry says so instead of doing
 * nothing.
 */
export function Placeholder({
  title,
  description = "Not built yet.",
}: {
  title: string;
  description?: string;
}) {
  return (
    <div className="flex h-full flex-col p-6">
      <h1 className="text-xl font-semibold tracking-tight">{title}</h1>
      <div className="flex flex-1 items-center justify-center">
        <EmptyState
          icon={<Hammer />}
          title="Under construction"
          description={description}
        />
      </div>
    </div>
  );
}
