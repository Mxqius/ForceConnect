import { load, type Store } from "@tauri-apps/plugin-store";
import { isTauri } from "@tauri-apps/api/core";

/**
 * Persisted user preferences.
 *
 * Backed by `tauri-plugin-store` (a JSON file under %APPDATA%), with an
 * in-memory fallback so the UI still works when running outside Tauri for
 * development. Nothing here is secret, and nothing here is worth failing a
 * render over — every read has a default.
 */

export type NavLayout = "rail" | "dock";

export interface Prefs {
  favourites: string[];
  navLayout: NavLayout;
  theme: "dark" | "light";
  minimizeToTray: boolean;
  /**
   * The adapter DNS is applied to, or `null` for "whichever one is active".
   *
   * A LUID rather than a name or index: names are user-editable and indices
   * are reassigned when adapters come and go, so either would silently point
   * at a different interface after a reboot.
   */
  interfaceLuid: string | null;
}

export const DEFAULT_PREFS: Prefs = {
  favourites: [],
  navLayout: "rail",
  theme: "dark",
  minimizeToTray: false,
  interfaceLuid: null,
};

const FILE = "preferences.json";

let handle: Promise<Store> | null = null;
const memory = new Map<string, unknown>();

function store(): Promise<Store> | null {
  if (!isTauri()) return null;
  handle ??= load(FILE, { autoSave: true });
  return handle;
}

export async function readPref<K extends keyof Prefs>(
  key: K,
): Promise<Prefs[K]> {
  const backing = store();

  if (!backing) {
    return memory.has(key) ? (memory.get(key) as Prefs[K]) : DEFAULT_PREFS[key];
  }

  try {
    const value = await (await backing).get<Prefs[K]>(key);
    // `undefined` means "never written"; `null` is a real stored value for
    // `interfaceLuid`, so `??` alone would overwrite an explicit "automatic"
    // with the default on every read.
    return value === undefined ? DEFAULT_PREFS[key] : value;
  } catch {
    // A corrupt or unreadable store should degrade to defaults rather than
    // leave the app unusable.
    return DEFAULT_PREFS[key];
  }
}

export async function writePref<K extends keyof Prefs>(
  key: K,
  value: Prefs[K],
): Promise<void> {
  const backing = store();

  if (!backing) {
    memory.set(key, value);
    return;
  }

  try {
    await (await backing).set(key, value);
  } catch {
    // Losing a preference is not worth interrupting the user for.
  }
}

/** Toggle membership of a set stored under `key`. */
export async function togglePref(
  key: "favourites",
  id: string,
): Promise<string[]> {
  const current = await readPref(key);
  const next = current.includes(id)
    ? current.filter((item) => item !== id)
    : [...current, id];

  await writePref(key, next);
  return next;
}
