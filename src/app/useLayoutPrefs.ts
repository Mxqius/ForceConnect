import { useEffect } from "react";
import { create } from "zustand";

import { ipc } from "@/lib/ipc";
import {
  DEFAULT_PREFS,
  readPref,
  writePref,
  type NavLayout,
} from "@/lib/prefs";

interface LayoutState {
  navLayout: NavLayout;
  theme: "dark" | "light";
  minimizeToTray: boolean;
  hydrated: boolean;
  setNavLayout: (layout: NavLayout) => void;
  setTheme: (theme: "dark" | "light") => void;
  setMinimizeToTray: (enabled: boolean) => void;
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
  minimizeToTray: DEFAULT_PREFS.minimizeToTray,
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

  setMinimizeToTray: (minimizeToTray) => {
    set({ minimizeToTray });
    void writePref("minimizeToTray", minimizeToTray);
    void ipc.setMinimizeToTray(minimizeToTray);
  },

  hydrate: async () => {
    const [navLayout, theme, minimizeToTray] = await Promise.all([
      readPref("navLayout"),
      readPref("theme"),
      readPref("minimizeToTray"),
    ]);

    document.documentElement.dataset.theme = theme;
    set({ navLayout, theme, minimizeToTray, hydrated: true });
    // The backend's copy of the flag starts false on every launch — the main
    // window-close handler needs it before the user can ever reach the
    // toggle, so hydrate pushes the persisted value across immediately.
    void ipc.setMinimizeToTray(minimizeToTray);
  },
}));

/** Load persisted layout preferences once, at startup. */
export function useHydrateLayoutPrefs() {
  const hydrate = useLayoutPrefs((state) => state.hydrate);

  useEffect(() => {
    void hydrate();
  }, [hydrate]);
}
