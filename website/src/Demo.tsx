import { Activity, Check, Lock, Power } from "lucide-react";
import { useEffect, useRef, useState } from "react";

type Phase = "idle" | "connecting" | "connected";

const STEPS = ["Probe", "Apply", "Flush"] as const;
const STEP_MS = 650;

const SERVERS = [
  { name: "Cloudflare", addr: "1.1.1.1", ms: 9, color: "#f38020" },
  { name: "Quad9", addr: "9.9.9.9", ms: 14, color: "#d6336c" },
  { name: "AdGuard", addr: "94.140.14.14", ms: 21, color: "#68bc71" },
  { name: "Mullvad", addr: "194.242.2.2", ms: 33, color: "#ffd524" },
] as const;

const PROTOCOLS = ["DNS", "DoT", "DoH", "DoH3"] as const;

/**
 * A working miniature of the app's connection screen. Nothing touches the
 * network; it plays the same probe → apply → flush sequence the app runs, so
 * a visitor can feel the one-click flow instead of reading about it.
 */
export function Demo() {
  const [phase, setPhase] = useState<Phase>("idle");
  const [step, setStep] = useState(0);
  const [server, setServer] = useState(0);
  const [protocol, setProtocol] = useState<(typeof PROTOCOLS)[number]>("DoH");
  const timers = useRef<number[]>([]);

  const clear = () => {
    timers.current.forEach(clearTimeout);
    timers.current = [];
  };
  useEffect(() => clear, []);

  const run = () => {
    clear();
    setPhase("connecting");
    setStep(0);
    STEPS.forEach((_, i) => {
      timers.current.push(
        window.setTimeout(() => setStep(i + 1), STEP_MS * (i + 1)),
      );
    });
    timers.current.push(
      window.setTimeout(() => setPhase("connected"), STEP_MS * STEPS.length),
    );
  };

  const toggle = () => {
    if (phase === "idle") run();
    else {
      clear();
      setPhase("idle");
      setStep(0);
    }
  };

  // Switching server or protocol while connected reconnects, as the app does.
  const pick = (fn: () => void) => {
    fn();
    if (phase !== "idle") run();
  };

  const s = SERVERS[server]!;
  const lit = phase === "connected";
  const encrypted = protocol !== "DNS";

  return (
    <div className="grid w-full grid-cols-2 gap-3">
      {/* Connection tile */}
      <div
        data-lit={lit}
        className="demo border-line bg-tile relative col-span-2 h-[300px] overflow-hidden rounded-(--radius-tile) border sm:h-[320px]"
      >
        <div className="demo-glow" />
        {phase === "connecting" && <div className="demo-shimmer" />}

        <div className="relative flex h-full flex-col p-6">
          <span
            className={`inline-flex w-fit items-center gap-2 rounded-full px-3 py-1.5 text-xs font-medium transition-colors ${
              lit ? "bg-white/18 text-white" : "bg-elevated text-ink-2"
            }`}
          >
            <span
              className={`size-1.5 rounded-full ${
                lit
                  ? "bg-white"
                  : phase === "connecting"
                    ? "bg-accent-bright animate-pulse"
                    : "bg-ink-3"
              }`}
            />
            {lit ? `${s.name} · ${protocol}` : "Network DNS"}
          </span>

          <div className="mt-auto">
            <h3
              className={`font-display text-[34px] leading-none font-semibold tracking-tight transition-colors sm:text-[40px] ${
                lit ? "text-white" : "text-ink"
              }`}
            >
              {phase === "idle"
                ? "Not connected"
                : phase === "connecting"
                  ? "Connecting…"
                  : "Protected"}
            </h3>

            <div className="mt-3 h-6 text-sm">
              {phase === "idle" && (
                <p className="text-ink-3">Press the button to switch.</p>
              )}
              {phase === "connecting" && (
                <ol className="flex gap-4">
                  {STEPS.map((label, i) => (
                    <li
                      key={label}
                      className={`flex items-center gap-1.5 transition-colors ${
                        i < step
                          ? "text-ink"
                          : i === step
                            ? "text-accent-bright"
                            : "text-ink-3/60"
                      }`}
                    >
                      {i < step ? (
                        <Check className="size-3.5" strokeWidth={3} />
                      ) : (
                        <span className="size-1.5 rounded-full bg-current" />
                      )}
                      {label}
                    </li>
                  ))}
                </ol>
              )}
              {lit && (
                <p className="flex items-center gap-1.5 text-white/85">
                  {encrypted && <Lock className="size-3.5" />}
                  {encrypted ? "Queries encrypted" : "Plain DNS"} via{" "}
                  <span className="font-mono">{s.addr}</span>
                </p>
              )}
            </div>
          </div>
        </div>

        <button
          type="button"
          onClick={toggle}
          aria-label={phase === "idle" ? "Connect" : "Disconnect"}
          aria-pressed={phase !== "idle"}
          className="group absolute right-6 bottom-6 grid size-[88px] cursor-pointer place-items-center rounded-full"
        >
          {phase === "idle" && (
            <span className="power-ring absolute inset-0 rounded-full border-2 border-white/40" />
          )}
          <span
            className={`absolute inset-0 rounded-full border-[6px] transition-colors ${
              lit ? "border-white/25" : "border-elevated"
            }`}
          />
          <span
            className={`relative grid size-[68px] place-items-center rounded-full shadow-lg transition-all duration-300 group-hover:scale-105 group-active:scale-95 ${
              lit ? "text-accent bg-white" : "bg-ink text-ground"
            }`}
          >
            <Power className="size-7" strokeWidth={2.4} />
          </span>
        </button>
      </div>

      {/* Server picker */}
      <div className="border-line bg-tile col-span-2 rounded-(--radius-tile) border p-4 sm:col-span-1">
        <p className="text-ink-3 mb-3 text-xs font-medium">DNS server</p>
        <div className="grid grid-cols-2 gap-1.5">
          {SERVERS.map((srv, i) => (
            <button
              key={srv.name}
              type="button"
              onClick={() => pick(() => setServer(i))}
              className={`flex cursor-pointer items-center gap-2 rounded-xl px-2.5 py-2 text-left text-[13px] transition-colors ${
                i === server
                  ? "bg-elevated text-ink ring-line-strong ring-1"
                  : "text-ink-2 hover:bg-subtle"
              }`}
            >
              <span
                className="grid size-5 shrink-0 place-items-center rounded-md text-[10px] font-bold text-black"
                style={{ background: srv.color }}
              >
                {srv.name[0]}
              </span>
              <span className="truncate">{srv.name}</span>
            </button>
          ))}
        </div>
      </div>

      {/* Protocol + latency */}
      <div className="border-line bg-tile col-span-2 flex flex-col rounded-(--radius-tile) border p-4 sm:col-span-1">
        <div className="text-ink-3 flex items-center justify-between text-xs font-medium">
          <span className="flex items-center gap-1.5">
            <Activity className="size-3.5" /> Latency
          </span>
          <span className="bg-ok/15 text-ok rounded-full px-2 py-0.5 text-[11px]">
            {s.ms < 20 ? "Excellent" : "Good"}
          </span>
        </div>
        <p className="font-display text-ok mt-1 text-3xl font-semibold tabular-nums">
          {s.ms}
          <span className="text-ink-3 ml-1 text-sm font-normal">ms</span>
        </p>
        <div className="bg-subtle mt-auto grid grid-cols-4 gap-1 rounded-xl p-1">
          {PROTOCOLS.map((p) => (
            <button
              key={p}
              type="button"
              onClick={() => pick(() => setProtocol(p))}
              className={`cursor-pointer rounded-lg py-1.5 text-[12px] font-medium transition-colors ${
                p === protocol
                  ? "bg-ink text-ground"
                  : "text-ink-3 hover:text-ink"
              }`}
            >
              {p}
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}
