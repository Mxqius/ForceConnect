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
  adapters: ["adapters"] as const,
  adapter: (luid: string) => ["adapters", luid] as const,
  servers: ["servers"] as const,
  protocols: ["protocols"] as const,
  latency: (serverId: string) => ["latency", serverId] as const,
  connection: ["connection"] as const,
} as const;
