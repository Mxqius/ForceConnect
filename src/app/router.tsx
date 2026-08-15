import { createHashRouter } from "react-router";

import { Shell } from "@/app/Shell";
import { HomeScreen } from "@/features/connection/HomeScreen";

// Hash routing: the production build is served from Tauri's custom protocol,
// where path-based history needs a server-side rewrite we do not have.
export const router = createHashRouter([
  {
    path: "/",
    Component: Shell,
    children: [{ index: true, Component: HomeScreen }],
  },
]);
