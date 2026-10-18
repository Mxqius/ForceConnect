import { useQueryClient, type QueryClient } from "@tanstack/react-query";
import { useCallback } from "react";
import { create } from "zustand";

import { toast } from "@/design/primitives";
import { useDnsTarget } from "@/features/interfaces/useAdapters";
import { ipc, type Adapter, type Protocol, type Server } from "@/lib/ipc";
import { queryKeys } from "@/lib/query-keys";

interface ActiveConnection {
  serverId: string;
  serverName: string;
  protocol: Protocol;
  protocolLabel: string;
  encrypted: boolean;
  /** What was written to the adapter, for the interface tile to show. */
  addrs: string[];
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
 * - `apply`: write the servers to the adapter. The only step that changes
 *   system state, and the point after which a connect cannot be cancelled.
 * - `flush`: drop cached answers, so the very next lookup goes to the new
 *   server instead of being served from what the old one said.
 */
export type ConnectStep = "probe" | "apply" | "flush";
export const CONNECT_STEPS: readonly ConnectStep[] = [
  "probe",
  "apply",
  "flush",
];

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
  /** Index into `CONNECT_STEPS` of the step in progress; its length once done. */
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

const useConnectionStore = create<ConnectionState>((set) => ({
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

async function runConnect(
  client: QueryClient,
  adapter: Adapter | undefined,
  server: Server,
  protocol: Protocol,
): Promise<ConnectResult> {
  const { getState, setState } = useConnectionStore;
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
  if (endpoint.requiresForwarder) {
    // Setting plain DNS behind the user's back and letting them believe
    // traffic is encrypted would be worse than refusing.
    setState({ attempt: null });
    return fail(
      null,
      `${endpoint.label} needs the local resolver, which is not available yet. Pick plain DNS for now.`,
    );
  }

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

  setState({ step: 1 });
  try {
    await paced(() => ipc.setDns(adapter.luid, endpoint.addrs), STEP_MIN_MS);
  } catch (error) {
    return fail(
      "apply",
      `Windows did not accept the change: ${messageOf(error)}`,
    );
  }

  setState({ step: 2 });
  // Best-effort: the new servers are already in place, and a cache that could
  // not be flushed only means a few stale answers until they expire. Failing
  // the whole connect over it would misreport what state the machine is in.
  await paced(() => ipc.flushDnsCache().catch(() => undefined), STEP_MIN_MS);

  setState({ step: 3 });
  await delay(SETTLE_MS);

  const connection: ActiveConnection = {
    serverId: server.id,
    serverName: server.displayName,
    protocol,
    protocolLabel: endpoint.label,
    encrypted: endpoint.encrypted,
    addrs: endpoint.addrs,
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

  setState({ active: null, phase: "idle", connectedAt: null, step: 0 });
  void client.invalidateQueries({ queryKey: queryKeys.adapters });
  void ipc.setTrayStatus({
    connected: false,
    serverName: null,
    protocolLabel: null,
  });
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
