import { useEffect } from "react";
import { create } from "zustand";

import {
  DEFAULT_PREFS,
  readPref,
  writePref,
  type UpstreamChoice,
} from "@/lib/prefs";

interface ConnectionPrefsState {
  autoConnect: boolean;
  lastConnection: UpstreamChoice | null;
  backup: UpstreamChoice | null;
  hydrated: boolean;
  setAutoConnect: (enabled: boolean) => void;
  rememberConnection: (choice: UpstreamChoice) => void;
  setBackup: (choice: UpstreamChoice | null) => void;
  hydrate: () => Promise<void>;
}

/**
 * What the connection remembers between launches: the last server, whether
 * to reconnect to it at startup, and the backup the resolver falls back on.
 */
export const useConnectionPrefs = create<ConnectionPrefsState>((set) => ({
  autoConnect: DEFAULT_PREFS.autoConnect,
  lastConnection: DEFAULT_PREFS.lastConnection,
  backup: DEFAULT_PREFS.backupServer,
  hydrated: false,

  setAutoConnect: (autoConnect) => {
    set({ autoConnect });
    void writePref("autoConnect", autoConnect);
  },

  rememberConnection: (lastConnection) => {
    set({ lastConnection });
    void writePref("lastConnection", lastConnection);
  },

  setBackup: (backup) => {
    set({ backup });
    void writePref("backupServer", backup);
  },

  hydrate: async () => {
    const [autoConnect, lastConnection, backup] = await Promise.all([
      readPref("autoConnect"),
      readPref("lastConnection"),
      readPref("backupServer"),
    ]);
    set({ autoConnect, lastConnection, backup, hydrated: true });
  },
}));

/** Load the connection preferences once, at startup. */
export function useHydrateConnectionPrefs() {
  const hydrate = useConnectionPrefs((state) => state.hydrate);

  useEffect(() => {
    void hydrate();
  }, [hydrate]);
}
