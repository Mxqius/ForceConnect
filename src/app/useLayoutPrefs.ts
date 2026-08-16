import { useEffect } from "react";
import { create } from "zustand";

import {
  DEFAULT_PREFS,
  readPref,
  writePref,
  type NavLayout,
} from "@/lib/prefs";

interface LayoutState {
  navLayout: NavLayout;
  theme: "dark" | "light";
  hydrated: boolean;
  setNavLayout: (layout: NavLayout) => void;
  setTheme: (theme: "dark" | "light") => void;
  hydrate: () => Promise<void>;
}

/**
 * UI-only state, kept out of React Query.
 *
 * These are preferences rather than server state: there is nothing to
 * invalidate or refetch, and treating them as queries would mean a loading
 * state for something that should be applied before first paint.
 */
export const useLayoutPrefs = create<LayoutState>((set) => ({
  navLayout: DEFAULT_PREFS.navLayout,
  theme: DEFAULT_PREFS.theme,
  hydrated: false,

  setNavLayout: (navLayout) => {
    set({ navLayout });
    void writePref("navLayout", navLayout);
  },

  setTheme: (theme) => {
    set({ theme });
    document.documentElement.dataset.theme = theme;
    void writePref("theme", theme);
  },

  hydrate: async () => {
    const [navLayout, theme] = await Promise.all([
      readPref("navLayout"),
      readPref("theme"),
    ]);

    document.documentElement.dataset.theme = theme;
    set({ navLayout, theme, hydrated: true });
  },
}));

/** Load persisted layout preferences once, at startup. */
export function useHydrateLayoutPrefs() {
  const hydrate = useLayoutPrefs((state) => state.hydrate);

  useEffect(() => {
    void hydrate();
  }, [hydrate]);
}
