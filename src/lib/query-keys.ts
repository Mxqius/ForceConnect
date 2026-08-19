/**
 * Every React Query key in one place.
 *
 * Backend state changes outside the app — a user edits DNS in Windows Settings,
 * a VPN comes up — so invalidation is driven by Tauri events. Keeping the keys
 * centralized means an event handler can invalidate precisely without guessing
 * at the string a hook used.
 */
export const queryKeys = {
  appInfo: ["app-info"] as const,
  updaterStatus: ["updater-status"] as const,
  adapters: ["adapters"] as const,
  adapter: (luid: string) => ["adapters", luid] as const,
  servers: ["servers"] as const,
  customServers: ["servers", "custom"] as const,
  favourites: ["favourites"] as const,
  protocols: ["protocols"] as const,
  // Keyed by protocol as well as server: the same resolver answers plain UDP
  // and DoH3 at genuinely different speeds, so one cached figure per server
  // would show the wrong number every time the protocol changed.
  latency: (serverId: string, protocol: string) =>
    ["latency", serverId, protocol] as const,
  connection: ["connection"] as const,
} as const;
