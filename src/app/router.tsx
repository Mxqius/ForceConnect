import { createHashRouter, type RouteObject } from "react-router";

import { Shell } from "@/app/Shell";
import { HomeScreen } from "@/features/connection/HomeScreen";
import { InterfacesScreen } from "@/features/interfaces/InterfacesScreen";
import { Placeholder } from "@/features/shared/Placeholder";

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
    children: [
      { index: true, Component: HomeScreen },
      { path: "interfaces", Component: InterfacesScreen },
      {
        path: "servers",
        element: (
          <Placeholder
            title="Servers"
            description="The catalog browser lands next, with protocol filtering and live latency."
          />
        ),
      },
      {
        path: "settings",
        element: <Placeholder title="Settings" />,
      },
      {
        path: "about",
        element: <Placeholder title="About" />,
      },
      ...devRoutes,
    ],
  },
]);
