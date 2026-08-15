import { commands, type CommandError } from "@/lib/bindings";

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

/** Typed, unwrapped wrappers over the generated command surface. */
export const ipc = {
  appInfo: () => unwrap(commands.appInfo()),
};
