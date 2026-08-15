import { Outlet } from "react-router";

/**
 * Window chrome and the left navigation rail live here. Both arrive with the
 * design system; for now this is just the frame the routes render into.
 */
export function Shell() {
  return (
    <div className="bg-surface-base text-text-primary flex h-full flex-col">
      <main className="flex-1 overflow-y-auto">
        <Outlet />
      </main>
    </div>
  );
}
