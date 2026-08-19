import { useQuery } from "@tanstack/react-query";
import { isTauri } from "@tauri-apps/api/core";
import { check, type Update } from "@tauri-apps/plugin-updater";
import { useEffect } from "react";
import { create } from "zustand";

import { ipc } from "@/lib/ipc";
import { readPref, writePref } from "@/lib/prefs";
import { queryKeys } from "@/lib/query-keys";

/**
 * Where the update is up to.
 *
 * Modelled as a union rather than a handful of booleans because the states are
 * genuinely exclusive and the screen has to say which one it is in — "did it
 * finish or not?" is the question this exists to answer, and a bag of
 * `isChecking`/`isDownloading`/`hasError` flags is exactly how a UI ends up
 * unable to answer it.
 */
export type UpdateStage =
  | { kind: "idle" }
  | { kind: "checking" }
  /** Checked, and this is already the newest release. */
  | { kind: "current"; checkedAt: number }
  | {
      kind: "available";
      version: string;
      notes: string | null;
      date: string | null;
    }
  | {
      kind: "downloading";
      version: string;
      /** `null` until the server reports a content length. */
      percent: number | null;
    }
  /** Downloaded and verified; the installer is running. */
  | { kind: "installing"; version: string }
  | { kind: "error"; message: string };

function messageOf(error: unknown): string {
  const message = error instanceof Error ? error.message : String(error);

  // The plugin reports a missing or mismatched key as a signature failure,
  // which is accurate and completely opaque to anyone who has not read its
  // source. Say what it means for the user instead.
  if (/signature|pubkey|public key/i.test(message)) {
    return "The download could not be verified against this build's signing key, so nothing was installed.";
  }
  return message;
}

interface UpdaterState {
  stage: UpdateStage;
  autoUpdate: boolean;
  hydrated: boolean;
  /**
   * The release the last check resolved.
   *
   * Held rather than re-fetched at install time: `Update` carries a handle to
   * one specific release, and checking again in order to install could hand
   * back a different one than the notes the user just read.
   */
  pending: Update | null;
  checkedOnLaunch: boolean;

  setAutoUpdate: (enabled: boolean) => void;
  hydrate: () => Promise<void>;
  check: () => Promise<void>;
  install: () => Promise<void>;
}

/**
 * A store rather than component state, because the launch check and the
 * Settings screen have to be looking at the same thing. `Shell` starts the
 * check when the app opens; Settings may not be visited until minutes later,
 * and when it is, it should show the result of that check rather than starting
 * its own.
 */
export const useUpdaterStore = create<UpdaterState>((set, get) => ({
  stage: { kind: "idle" },
  autoUpdate: true,
  hydrated: false,
  pending: null,
  checkedOnLaunch: false,

  setAutoUpdate: (autoUpdate) => {
    set({ autoUpdate });
    void writePref("autoUpdate", autoUpdate);
  },

  hydrate: async () => {
    const autoUpdate = await readPref("autoUpdate");
    set({ autoUpdate, hydrated: true });
  },

  check: async () => {
    if (!isTauri()) {
      set({
        stage: {
          kind: "error",
          message: "Updates are only available in the desktop app.",
        },
      });
      return;
    }

    set({ stage: { kind: "checking" } });

    try {
      const update = await check();

      if (!update) {
        set({
          pending: null,
          stage: { kind: "current", checkedAt: Date.now() },
        });
        return;
      }

      set({
        pending: update,
        stage: {
          kind: "available",
          version: update.version,
          notes: update.body?.trim() || null,
          date: update.date ?? null,
        },
      });
    } catch (error) {
      set({
        pending: null,
        stage: { kind: "error", message: messageOf(error) },
      });
    }
  },

  install: async () => {
    const update = get().pending;
    if (!update) return;

    set({
      stage: { kind: "downloading", version: update.version, percent: null },
    });

    let total = 0;
    let received = 0;

    try {
      await update.downloadAndInstall((event) => {
        switch (event.event) {
          case "Started":
            total = event.data.contentLength ?? 0;
            set({
              stage: {
                kind: "downloading",
                version: update.version,
                percent: total > 0 ? 0 : null,
              },
            });
            break;

          case "Progress":
            received += event.data.chunkLength;
            set({
              stage: {
                kind: "downloading",
                version: update.version,
                // No content length means no honest percentage, and a made-up
                // one is worse than none.
                percent:
                  total > 0
                    ? Math.min(100, Math.round((received / total) * 100))
                    : null,
              },
            });
            break;

          case "Finished":
            set({ stage: { kind: "installing", version: update.version } });
            break;
        }
      });

      // On Windows this line is not reached: the plugin hands off to the NSIS
      // installer and exits the process, and the installer starts the new
      // version once it finishes. The state is set anyway so the screen is
      // never left on a spinner if that behaviour ever changes.
      set({ stage: { kind: "installing", version: update.version } });
    } catch (error) {
      set({ stage: { kind: "error", message: messageOf(error) } });
    }
  },
}));

/**
 * Whether this build can verify and install an update at all.
 *
 * Asked separately from the check itself: an unsigned build can reach the
 * endpoint and see that a newer version exists, then refuse it at the last
 * step. The screen needs to know that before it offers the button.
 */
export function useUpdaterStatus() {
  return useQuery({
    queryKey: queryKeys.updaterStatus,
    queryFn: ipc.updaterStatus,
    staleTime: Infinity,
  });
}

/**
 * Load the preference and, if it is on, check once at startup.
 *
 * Checking only. It never downloads on its own: this app runs elevated and the
 * update replaces the running binary, which is not something to do without
 * being asked.
 */
export function useLaunchUpdateCheck() {
  const hydrated = useUpdaterStore((state) => state.hydrated);
  const autoUpdate = useUpdaterStore((state) => state.autoUpdate);
  const hydrate = useUpdaterStore((state) => state.hydrate);

  useEffect(() => {
    void hydrate();
  }, [hydrate]);

  useEffect(() => {
    if (!hydrated || !autoUpdate || !isTauri()) return;

    // Guarded in the store rather than with a ref: React 19's StrictMode runs
    // effects twice in development, and two checks racing would leave two
    // `Update` handles fighting over one `pending` slot.
    const { checkedOnLaunch, check: run } = useUpdaterStore.getState();
    if (checkedOnLaunch) return;

    useUpdaterStore.setState({ checkedOnLaunch: true });
    void run();
  }, [hydrated, autoUpdate]);
}

export function useUpdater() {
  const stage = useUpdaterStore((state) => state.stage);

  return {
    stage,
    autoUpdate: useUpdaterStore((state) => state.autoUpdate),
    setAutoUpdate: useUpdaterStore((state) => state.setAutoUpdate),
    check: useUpdaterStore((state) => state.check),
    install: useUpdaterStore((state) => state.install),
    /** True while nothing else should be started. */
    isBusy:
      stage.kind === "checking" ||
      stage.kind === "downloading" ||
      stage.kind === "installing",
  };
}
