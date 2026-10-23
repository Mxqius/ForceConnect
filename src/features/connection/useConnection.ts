import { useQueryClient, type QueryClient } from "@tanstack/react-query";
import { useCallback, useEffect, useRef } from "react";
import { create } from "zustand";

import { toast } from "@/design/primitives";
import { useConnectionPrefs } from "@/features/connection/useConnectionPrefs";
import { useDnsTarget } from "@/features/interfaces/useAdapters";
import { ipc, type Adapter, type Protocol, type Server } from "@/lib/ipc";
import type { UpstreamChoice } from "@/lib/prefs";
import { queryKeys } from "@/lib/query-keys";

interface ActiveConnection {
  serverId: string;
  serverName: string;
  protocol: Protocol;
  protocolLabel: string;
  encrypted: boolean;
  /** What was written to the adapter, for the interface tile to show. */
  addrs: string[];
  /**
   * Whether lookups go through the local resolver. When they do, `addrs` is
   * loopback and the server's own addresses are in `upstream`.
   */
  viaResolver: boolean;
  upstream: string[];
  adapterLuid: string;
  adapterName: string;
}

/**
 * Where the connection is, as the user should see it.
 *
 * `switching` is its own phase rather than `connecting` again: the user is
 * protected throughout a switch — the old servers stay on the adapter until
 * the new ones are written — and the UI must not suggest otherwise by going
 * grey in between.
 */
export type ConnectPhase =
  | "idle"
  | "connecting"
  | "connected"
  | "switching"
  | "disconnecting"
  | "failed";

/**
 * The work a connect actually does, in order. The UI lists these verbatim, so
 * they must stay the real steps rather than a story about them.
 *
 * - `probe`: ask the server a real query over the chosen transport. Done
 *   first so a dead server fails *before* anything on the machine changes.
 * - `resolver`: encrypted protocols only. Start the local resolver on
 *   loopback port 53 — or point the running one at the new server — so the
 *   adapter has a plain address to be given. Before `apply`, because an
 *   adapter pointed at a port nothing answers on cannot resolve anything.
 * - `apply`: write the servers to the adapter. The only step that changes
 *   system state, and the point after which a connect cannot be cancelled.
 * - `flush`: drop cached answers, so the very next lookup goes to the new
 *   server instead of being served from what the old one said.
 */
export type ConnectStep = "probe" | "resolver" | "apply" | "flush";

/** The steps a connect runs, which depend on whether it needs the resolver. */
export function connectSteps(viaResolver: boolean): readonly ConnectStep[] {
  return viaResolver
    ? ["probe", "resolver", "apply", "flush"]
    : ["probe", "apply", "flush"];
}

export interface ConnectFailure {
  /** `null` when the attempt was refused before any step began. */
  step: ConnectStep | null;
  message: string;
}

export interface ConnectAttempt {
  serverId: string;
  serverName: string;
  protocolLabel: string;
  encrypted: boolean;
  adapterName: string;
  steps: readonly ConnectStep[];
}

export type ConnectResult = "connected" | "cancelled" | "failed";

/**
 * How long each step stays on screen at minimum.
 *
 * A floor, not a delay: a step that takes longer is shown for as long as it
 * takes. Without it, a probe answered from cache and a write that returns in
 * 20ms would flash three rows past faster than they can be read, and the
 * connect would look like it skipped its checks.
 */
const STEP_MIN_MS = 650;
/** Lets the last checkmark land before the tile turns blue. */
const SETTLE_MS = 260;
const DISCONNECT_MIN_MS = 700;

interface ConnectionState {
  active: ActiveConnection | null;
  phase: ConnectPhase;
  /** Index into `attempt.steps` of the step in progress; its length once done. */
  step: number;
  attempt: ConnectAttempt | null;
  failure: ConnectFailure | null;
  /** When the current connection was made, for the uptime counter. */
  connectedAt: number | null;
  /** Bumped to orphan an in-flight attempt, which is how cancel works. */
  run: number;

  /**
   * What the connection screen has picked but not yet connected.
   *
   * This lives here rather than in a component because the screen unmounts:
   * the router swaps the whole route element when the user visits another
   * tab, and a half-made choice is not something a tab switch should destroy.
   *
   * `protocol` is `null` for "the server's preferred protocol", so a server
   * change does not have to guess a protocol the new server may not support.
   */
  selectedId: string | null;
  protocol: Protocol | null;
  selectServer: (id: string | null) => void;
  selectProtocol: (protocol: Protocol | null) => void;
}

// Exported for tests only; components go through the hooks below.
export const useConnectionStore = create<ConnectionState>((set) => ({
  active: null,
  phase: "idle",
  step: 0,
  attempt: null,
  failure: null,
  connectedAt: null,
  run: 0,

  selectedId: null,
  protocol: null,
  // Changing the choice after a failure clears the failure: the error was
  // about the old choice, and leaving it up would describe something the user
  // has already moved away from.
  selectServer: (selectedId) =>
    set((state) => ({
      selectedId,
      protocol: null,
      ...clearedFailure(state),
    })),
  selectProtocol: (protocol) =>
    set((state) => ({ protocol, ...clearedFailure(state) })),
}));

function clearedFailure(state: ConnectionState): Partial<ConnectionState> {
  return state.phase === "failed"
    ? { phase: "idle", failure: null, attempt: null }
    : {};
}

const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

/** Run `work`, but take at least `minMs` whichever way it ends. */
async function paced<T>(work: () => Promise<T>, minMs: number): Promise<T> {
  const floor = delay(minMs);
  try {
    const value = await work();
    await floor;
    return value;
  } catch (error) {
    await floor;
    throw error;
  }
}

const messageOf = (error: unknown) =>
  error instanceof Error ? error.message : String(error);

/** Backend messages end without a full stop; one is needed before another sentence. */
const sentence = (text: string) => (/[.!?]$/.test(text) ? text : `${text}.`);

const isBusy = (phase: ConnectPhase) =>
  phase === "connecting" || phase === "switching" || phase === "disconnecting";

/** The connection screen's pending selection, kept across route changes. */
export function useServerSelection() {
  return {
    selectedId: useConnectionStore((state) => state.selectedId),
    protocol: useConnectionStore((state) => state.protocol),
    selectServer: useConnectionStore((state) => state.selectServer),
    selectProtocol: useConnectionStore((state) => state.selectProtocol),
  };
}

/** Just the phase, for chrome that only needs to colour a dot. */
export function useConnectionPhase(): ConnectPhase {
  return useConnectionStore((state) => state.phase);
}

/**
 * The backup the resolver should fall back on, unless it is the server being
 * connected to: a server cannot stand in for itself.
 */
function backupFor(serverId: string): UpstreamChoice | null {
  const { backup } = useConnectionPrefs.getState();
  return backup && backup.serverId !== serverId ? backup : null;
}

async function runConnect(
  client: QueryClient,
  adapter: Adapter | undefined,
  server: Server,
  protocol: Protocol,
): Promise<ConnectResult> {
  const { getState, setState } = useConnectionStore;
  await following;
  const before = getState();
  if (isBusy(before.phase)) return "cancelled";

  const switching = before.active !== null;
  const endpoint = server.endpoints.find((e) => e.protocol === protocol);

  function fail(step: ConnectStep | null, message: string): ConnectResult {
    if (switching) {
      // The previous servers are still on the adapter, so the connection the
      // user had is the connection they still have. Saying "failed" in the
      // tile would be wrong; a toast says what did not happen.
      setState({
        phase: "connected",
        attempt: null,
        step: 0,
        // Point the picker back at what is actually on the adapter.
        selectedId: before.active?.serverId ?? null,
        protocol: before.active?.protocol ?? null,
      });
      toast.error(`Could not switch to ${server.displayName}`, {
        description: `${message} You are still connected to ${before.active?.serverName}.`,
      });
    } else {
      setState({ phase: "failed", failure: { step, message } });
    }
    return "failed";
  }

  // Refusals before any step runs. The UI disables these cases, so reaching
  // here means a state changed underneath it — say so rather than guess.
  if (!adapter) {
    setState({ attempt: null });
    return fail(null, "There is no network adapter to configure.");
  }
  if (!endpoint) {
    setState({ attempt: null });
    return fail(null, `${server.displayName} does not offer that protocol.`);
  }

  // Encrypted protocols cannot be handed to the OS, which takes plain IP
  // addresses only: the adapter is pointed at the local resolver instead, and
  // the resolver speaks the encrypted protocol to the server.
  const viaResolver = endpoint.requiresForwarder;
  const wasViaResolver = before.active?.viaResolver ?? false;
  const steps = connectSteps(viaResolver);
  const stepOf = (id: ConnectStep) => steps.indexOf(id);

  const run = before.run + 1;
  setState({
    run,
    phase: switching ? "switching" : "connecting",
    step: 0,
    failure: null,
    attempt: {
      serverId: server.id,
      serverName: server.displayName,
      protocolLabel: endpoint.label,
      encrypted: endpoint.encrypted,
      adapterName: adapter.name,
      steps,
    },
  });
  const orphaned = () => getState().run !== run;

  try {
    const probe = await paced(
      () =>
        client.fetchQuery({
          queryKey: queryKeys.latency(server.id, protocol),
          queryFn: () => ipc.probeServer(server.id, protocol),
          // A result the user is looking at right now is good enough;
          // probing again would only add a second to every connect.
          staleTime: 30_000,
        }),
      STEP_MIN_MS,
    );
    if (orphaned()) return "cancelled";
    if (probe.verdict === "unreachable" || probe.medianMs === null) {
      return fail(
        "probe",
        `${server.displayName} did not answer a ${endpoint.label} lookup, so nothing was changed.`,
      );
    }
  } catch (error) {
    if (orphaned()) return "cancelled";
    return fail(
      "probe",
      `Could not reach ${server.displayName}: ${messageOf(error)}. Nothing was changed.`,
    );
  }

  let addrs = endpoint.addrs;
  if (viaResolver) {
    setState({ step: stepOf("resolver") });
    try {
      const status = await paced(
        () => ipc.startResolver(server.id, protocol, backupFor(server.id)),
        STEP_MIN_MS,
      );
      addrs = status.listen;
    } catch (error) {
      // A switch that fails leaves the running resolver on its old server,
      // so there is nothing to undo here either way.
      return fail(
        "resolver",
        `The local resolver could not start: ${messageOf(error)}. Nothing was changed.`,
      );
    }
  }

  setState({ step: stepOf("apply") });
  try {
    await paced(() => ipc.setDns(adapter.luid, addrs), STEP_MIN_MS);
  } catch (error) {
    if (viaResolver) {
      // Leave the resolver as the adapter still expects it: back on the old
      // server if the adapter was already pointed at it, or stopped if not.
      const previous = before.active;
      await (
        wasViaResolver && previous
          ? ipc.startResolver(
              previous.serverId,
              previous.protocol,
              backupFor(previous.serverId),
            )
          : ipc.stopResolver()
      ).catch(() => undefined);
    }
    return fail(
      "apply",
      `The system did not accept the change: ${messageOf(error)}`,
    );
  }

  // A switch made after traffic moved to another adapter, before the move was
  // followed: the old adapter still carries the previous servers, and nothing
  // else will ever let go of it.
  if (before.active && before.active.adapterLuid !== adapter.luid) {
    void ipc.releaseDns(before.active.adapterLuid).catch(() => undefined);
  }

  // Moving from encrypted to plain: the adapter no longer points at loopback,
  // so nothing needs the resolver any more.
  if (!viaResolver && wasViaResolver) {
    void ipc.stopResolver().catch(() => undefined);
  }

  setState({ step: stepOf("flush") });
  // Best-effort: the new servers are already in place, and a cache that could
  // not be flushed only means a few stale answers until they expire. Failing
  // the whole connect over it would misreport what state the machine is in.
  await paced(() => ipc.flushDnsCache().catch(() => undefined), STEP_MIN_MS);

  setState({ step: steps.length });
  await delay(SETTLE_MS);

  const connection: ActiveConnection = {
    serverId: server.id,
    serverName: server.displayName,
    protocol,
    protocolLabel: endpoint.label,
    encrypted: endpoint.encrypted,
    addrs,
    viaResolver,
    upstream: endpoint.addrs,
    adapterLuid: adapter.luid,
    adapterName: adapter.name,
  };

  setState({
    active: connection,
    phase: "connected",
    attempt: null,
    // A switch keeps the uptime running: the user never stopped being
    // connected, and a counter that reset would say they had.
    connectedAt: switching ? (before.connectedAt ?? Date.now()) : Date.now(),
    // Keep the picker showing what is connected. They are normally the same
    // already; they diverge when the connect came from the Servers screen.
    selectedId: server.id,
    protocol,
  });

  useConnectionPrefs.getState().rememberConnection({
    serverId: server.id,
    protocol,
  });
  void client.invalidateQueries({ queryKey: queryKeys.adapters });
  void ipc.setTrayStatus({
    connected: true,
    serverName: connection.serverName,
    protocolLabel: connection.protocolLabel,
  });

  return "connected";
}

async function runDisconnect(
  client: QueryClient,
  adapter: Adapter | undefined,
): Promise<void> {
  const { getState, setState } = useConnectionStore;
  await following;
  const { active, phase } = getState();
  if (isBusy(phase)) return;

  const luid = active?.adapterLuid ?? adapter?.luid;
  if (!luid) {
    toast.error("Nothing to disconnect");
    return;
  }

  setState({ phase: "disconnecting" });
  try {
    // Released rather than cleared: where the adapter goes is the user's
    // setting — their DNS from before launch, DHCP, or a default of their own
    // — and the backend applies the same choice on exit and after a crash.
    await paced(() => ipc.releaseDns(luid), DISCONNECT_MIN_MS);
  } catch (error) {
    setState({ phase: active ? "connected" : "idle" });
    toast.error("Could not disconnect", { description: messageOf(error) });
    return;
  }

  // Only once the adapter has let go of loopback: stopping first would leave
  // it pointed at a port nothing answers on, however briefly. A no-op when
  // the connection was plain DNS.
  await ipc.stopResolver().catch(() => undefined);

  setState({ active: null, phase: "idle", connectedAt: null, step: 0 });
  void client.invalidateQueries({ queryKey: queryKeys.adapters });
  void ipc.setTrayStatus({
    connected: false,
    serverName: null,
    protocolLabel: null,
  });
}

/**
 * A move of the connection to another adapter, while one is in flight.
 *
 * Connect and disconnect wait on it rather than refusing: it takes a fraction
 * of a second, and a disconnect that silently did nothing because it landed
 * mid-move would be worse than one that waits.
 */
let following: Promise<void> | null = null;
/** The adapter a move last failed to reach, so a poll does not retry it. */
let unreachable: string | null = null;

/**
 * Keep the connection on the adapter traffic actually leaves by.
 *
 * DNS is set per adapter, and the one traffic uses changes underneath a
 * connection: Wi-Fi gives way to a cable, a VPN comes up and its tunnel takes
 * the default route. The adapter that was set keeps the chosen servers, but
 * lookups now leave by one that still has the network's — and the screen goes
 * on saying "Connected". So the connection follows: the new adapter is set
 * first, so there is no moment where neither is, and only then is the old one
 * let go, back to wherever the release policy says.
 */
export async function runFollow(
  client: QueryClient,
  adapter: Adapter | undefined,
): Promise<void> {
  const { getState, setState } = useConnectionStore;
  const { active, phase } = getState();
  if (phase !== "connected" || !active || !adapter) return;
  if (adapter.luid === active.adapterLuid || adapter.luid === unreachable)
    return;

  try {
    await ipc.setDns(adapter.luid, active.addrs);
  } catch (error) {
    unreachable = adapter.luid;
    toast.error(`Could not move DNS to ${adapter.name}`, {
      description: `${sentence(messageOf(error))} Lookups through ${adapter.name} use the network’s DNS until you reconnect.`,
    });
    return;
  }
  unreachable = null;

  // Best-effort: an adapter that was unplugged is not there to release, and
  // the exit handler still covers one that comes back.
  await ipc.releaseDns(active.adapterLuid).catch(() => undefined);
  await ipc.flushDnsCache().catch(() => undefined);

  setState({
    active: { ...active, adapterLuid: adapter.luid, adapterName: adapter.name },
  });
  void client.invalidateQueries({ queryKey: queryKeys.adapters });
  toast.info(`DNS moved to ${adapter.name}`, {
    description: `Traffic now leaves through ${adapter.name}, so ${active.serverName} is set there too.`,
  });
}

/**
 * Run `runFollow` whenever the adapter DNS belongs on stops being the one the
 * connection is on. Mounted once, in `Shell`, so it works from any screen —
 * and from the tray, with the window hidden.
 */
export function useFollowAdapter() {
  const client = useQueryClient();
  const { adapter } = useDnsTarget();
  const phase = useConnectionStore((state) => state.phase);
  const activeLuid = useConnectionStore((state) => state.active?.adapterLuid);

  useEffect(() => {
    if (phase !== "connected" || !adapter || adapter.luid === activeLuid)
      return;
    if (following) return;

    following = runFollow(client, adapter).finally(() => {
      following = null;
    });
  }, [client, adapter, phase, activeLuid]);
}

/**
 * Hand a backup chosen in Settings to the resolver that is already running.
 *
 * Without this the choice would only take effect at the next connect, while
 * Settings said it was in place. Reacts to changes only: a connect passes the
 * backup itself.
 */
export function useApplyBackup() {
  const backup = useConnectionPrefs((state) => state.backup);
  const hydrated = useConnectionPrefs((state) => state.hydrated);
  const seen = useRef<UpstreamChoice | null | undefined>(undefined);

  useEffect(() => {
    if (!hydrated) return;
    const previous = seen.current;
    seen.current = backup;
    // The first value after hydrating is the stored one, not a change.
    if (previous === undefined || previous === backup) return;

    const { active, phase } = useConnectionStore.getState();
    if (phase !== "connected" || !active?.viaResolver) return;

    ipc
      .startResolver(
        active.serverId,
        active.protocol,
        backupFor(active.serverId),
      )
      .catch((error: unknown) => {
        toast.error("Could not change the backup server", {
          description: messageOf(error),
        });
      });
  }, [backup, hydrated]);
}

export function useConnection() {
  const client = useQueryClient();
  const target = useDnsTarget();
  const adapter = target.adapter;

  const active = useConnectionStore((state) => state.active);
  const phase = useConnectionStore((state) => state.phase);
  const step = useConnectionStore((state) => state.step);
  const attempt = useConnectionStore((state) => state.attempt);
  const failure = useConnectionStore((state) => state.failure);
  const connectedAt = useConnectionStore((state) => state.connectedAt);

  const connect = useCallback(
    (server: Server, protocol: Protocol) =>
      runConnect(client, adapter, server, protocol),
    [client, adapter],
  );

  const disconnect = useCallback(
    () => runDisconnect(client, adapter),
    [client, adapter],
  );

  /**
   * Abandon a connect while it is still only probing.
   *
   * Nothing has been written at that point, so cancelling is genuinely free.
   * Once the adapter write starts there is no clean way back short of a
   * disconnect, so the control is withdrawn rather than left pretending.
   */
  const cancel = useCallback(() => {
    useConnectionStore.setState((state) =>
      state.phase === "connecting" && state.step === 0
        ? { run: state.run + 1, phase: "idle", attempt: null, step: 0 }
        : {},
    );
  }, []);

  const dismissFailure = useCallback(() => {
    useConnectionStore.setState((state) => clearedFailure(state));
  }, []);

  return {
    active,
    adapter,
    target,
    phase,
    step,
    attempt,
    failure,
    connectedAt,
    busy: isBusy(phase),
    /** Cancel is only on offer while nothing on the machine has changed. */
    canCancel: phase === "connecting" && step === 0,
    connect,
    disconnect,
    cancel,
    dismissFailure,
    activeServerId: active?.serverId ?? null,
  };
}
