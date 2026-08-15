import { RouterProvider } from "react-router";

import { Providers } from "@/app/providers";
import { router } from "@/app/router";

export function App() {
  return (
    <Providers>
      <RouterProvider router={router} />
    </Providers>
  );
}
