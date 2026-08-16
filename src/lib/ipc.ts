import { isTauri } from "@tauri-apps/api/core";

import {
  commands,
  type CommandError,
  type NewServer,
  type Protocol,
  type Server,
  type TrayStatus,
} from "@/lib/bindings";
import * as fixtures from "@/lib/fixtures";

export type {
  Adapter,
  AdapterKind,
  AdapterStatus,
  AppInfo,
  CommandError,
  DnsSource,
  Endpoint,
  LatencyResult,
  NewServer,
  Protocol,
  Server,
  ServerSource,
  TrayStatus,
} from "@/lib/bindings";

/**
 * A failure returned by a Rust command.
 *
 * The backend models failure as a discriminated union rather than a string, so
 * callers can branch on `detail.kind` — for example to offer a "retry as
 * administrator" action only for elevation failures.
 */
export class IpcError extends Error {
  readonly detail: CommandError;

  constructor(detail: CommandError) {
    super(detail.message);
    this.name = "IpcError";
    this.detail = detail;
  }
}

type CommandResult<T> =
  { status: "ok"; data: T } | { status: "error"; error: CommandError };

/**
 * Convert a generated command's `Result` into a promise that rejects.
 *
 * React Query expects thrown errors, not result objects, so every call site
 * goes through here instead of unwrapping by hand.
 */
export async function unwrap<T>(call: Promise<CommandResult<T>>): Promise<T> {
  const result = await call;
  if (result.status === "error") {
    throw new IpcError(result.error);
  }
  return result.data;
}

/**
 * Outside Tauri there is no IPC, so every command would reject.
 *
 * In development that is worth working around: serving the UI in a normal
 * browser gives fast refresh and real devtools without rebuilding Rust, which
 * makes iterating on layout and empty/loading states much quicker. The branch
 * is compiled out of production builds, and the fixtures are typed as the real
 * command return types so a contract change breaks them at build time rather
 * than letting them drift.
 */
const useFixtures = import.meta.env.DEV && !isTauri();

async function fixture<T>(value: T): Promise<T> {
  // A little latency so loading states are actually visible while developing.
  await new Promise((resolve) => setTimeout(resolve, 150));
  return value;
}

/** Typed, unwrapped wrappers over the generated command surface. */
export const ipc = {
  appInfo: () =>
    useFixtures ? fixture(fixtures.appInfo) : unwrap(commands.appInfo()),
  setMinimizeToTray: (enabled: boolean) =>
    useFixtures ? fixture(null) : unwrap(commands.setMinimizeToTray(enabled)),
  setTrayStatus: (status: TrayStatus) =>
    useFixtures ? fixture(null) : unwrap(commands.setTrayStatus(status)),
  listAdapters: () =>
    useFixtures ? fixture(fixtures.adapters) : unwrap(commands.listAdapters()),
  listServers: () =>
    useFixtures ? fixture(fixtures.servers) : unwrap(commands.listServers()),
  listProtocols: () =>
    useFixtures
      ? fixture(fixtures.protocols)
      : unwrap(commands.listProtocols()),

  listCustomServers: () =>
    useFixtures
      ? fixture([] as Server[])
      : unwrap(commands.listCustomServers()),
  addCustomServer: (server: NewServer) =>
    useFixtures
      ? Promise.reject(new Error("custom servers need the desktop app"))
      : unwrap(commands.addCustomServer(server)),
  removeCustomServer: (id: string) =>
    useFixtures ? fixture(null) : unwrap(commands.removeCustomServer(id)),

  setDns: (luid: string, servers: string[]) =>
    useFixtures
      ? Promise.reject(new Error("changing DNS needs the desktop app"))
      : unwrap(commands.setDns(luid, servers)),
  clearDns: (luid: string) =>
    useFixtures
      ? Promise.reject(new Error("changing DNS needs the desktop app"))
      : unwrap(commands.clearDns(luid)),
  flushDnsCache: () =>
    useFixtures
      ? Promise.reject(new Error("flushing the cache needs the desktop app"))
      : unwrap(commands.flushDnsCache()),

  probeServers: (protocol: Protocol) =>
    useFixtures ? fixture([]) : unwrap(commands.probeServers(protocol)),
  probeServer: (serverId: string, protocol: Protocol) =>
    useFixtures
      ? fixture(fixtures.latency(serverId, protocol))
      : unwrap(commands.probeServer(serverId, protocol)),
};
