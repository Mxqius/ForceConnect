import { createHashRouter, type RouteObject } from "react-router";

import { Shell } from "@/app/Shell";
import { HomeScreen } from "@/features/connection/HomeScreen";

// The kitchen sink is a development tool, not a feature. Gating it on the
// build mode keeps it out of the production bundle entirely — Vite drops the
// branch, and with it the import.
const devRoutes: RouteObject[] = import.meta.env.DEV
  ? [
      {
        path: "dev/kitchen-sink",
        lazy: async () => {
          const { KitchenSink } =
            await import("@/design/kitchen-sink/KitchenSink");
          return { Component: KitchenSink };
        },
      },
    ]
  : [];

// Hash routing: the production build is served from Tauri's custom protocol,
// where path-based history needs a server-side rewrite we do not have.
export const router = createHashRouter([
  {
    path: "/",
    Component: Shell,
    children: [{ index: true, Component: HomeScreen }, ...devRoutes],
  },
]);
