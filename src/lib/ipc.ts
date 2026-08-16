import { isTauri } from "@tauri-apps/api/core";

import { commands, type CommandError } from "@/lib/bindings";
import * as fixtures from "@/lib/fixtures";

export type {
  Adapter,
  AdapterKind,
  AdapterStatus,
  AppInfo,
  CommandError,
  DnsSource,
  Endpoint,
  Protocol,
  Server,
  ServerSource,
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
  listAdapters: () =>
    useFixtures ? fixture(fixtures.adapters) : unwrap(commands.listAdapters()),
  listServers: () =>
    useFixtures ? fixture(fixtures.servers) : unwrap(commands.listServers()),
  listProtocols: () =>
    useFixtures
      ? fixture(fixtures.protocols)
      : unwrap(commands.listProtocols()),
};
