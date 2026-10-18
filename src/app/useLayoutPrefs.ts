import { useEffect } from "react";
import { create } from "zustand";

import { ipc } from "@/lib/ipc";
import {
  DEFAULT_PREFS,
  readPref,
  writePref,
  type MotionPref,
  type NavLayout,
} from "@/lib/prefs";

interface LayoutState {
  navLayout: NavLayout;
  theme: "dark" | "light";
  minimizeToTray: boolean;
  motion: MotionPref;
  compact: boolean;
  compactOnTop: boolean;
  hydrated: boolean;
  setNavLayout: (layout: NavLayout) => void;
  setTheme: (theme: "dark" | "light") => void;
  setMinimizeToTray: (enabled: boolean) => void;
  setMotion: (motion: MotionPref) => void;
  setCompact: (compact: boolean) => void;
  setCompactOnTop: (onTop: boolean) => void;
  hydrate: () => Promise<void>;
}

/**
 * UI-only state, kept out of React Query.
 *
 * These are preferences rather than server state: there is nothing to
 * invalidate or refetch, and treating them as queries would mean a loading
 * state for something that should be applied before first paint.
 */
export const useLayoutPrefs = create<LayoutState>((set, get) => ({
  navLayout: DEFAULT_PREFS.navLayout,
  theme: DEFAULT_PREFS.theme,
  minimizeToTray: DEFAULT_PREFS.minimizeToTray,
  motion: DEFAULT_PREFS.motion,
  compact: DEFAULT_PREFS.compact,
  compactOnTop: DEFAULT_PREFS.compactOnTop,
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

  setMotion: (motion) => {
    set({ motion });
    document.documentElement.dataset.motion = motion;
    void writePref("motion", motion);
  },

  // The window is resized before the layout swaps, so the full layout is
  // never drawn squeezed into the small window or the reverse. A resize that
  // fails leaves the layout where it was rather than mismatched with the
  // window it is in.
  setCompact: (compact) => {
    void ipc
      .setCompactMode(compact, get().compactOnTop)
      .then(() => {
        set({ compact });
        void writePref("compact", compact);
      })
      .catch(() => undefined);
  },

  setCompactOnTop: (compactOnTop) => {
    set({ compactOnTop });
    void writePref("compactOnTop", compactOnTop);
    void ipc.setCompactMode(get().compact, compactOnTop).catch(() => undefined);
  },

  hydrate: async () => {
    const [navLayout, theme, minimizeToTray, motion, compact, compactOnTop] =
      await Promise.all([
        readPref("navLayout"),
        readPref("theme"),
        readPref("minimizeToTray"),
        readPref("motion"),
        readPref("compact"),
        readPref("compactOnTop"),
      ]);

    document.documentElement.dataset.theme = theme;
    document.documentElement.dataset.motion = motion;
    // No resize here: the backend already applied the saved window mode
    // before the window was first shown (`window::restore_saved_mode`).
    set({
      navLayout,
      theme,
      minimizeToTray,
      motion,
      compact,
      compactOnTop,
      hydrated: true,
    });
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
