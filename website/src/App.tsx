import {
  Download,
  Feather,
  Gauge,
  Lock,
  RotateCcw,
  ShieldCheck,
  Zap,
} from "lucide-react";
import type { ReactNode } from "react";

import { Demo } from "./Demo";
import logo from "./logo.png";
import { detectPlatform, PLATFORMS, RELEASES, REPO } from "./platforms";

const PROVIDERS = [
  "Cloudflare",
  "Google",
  "Quad9",
  "AdGuard",
  "Control D",
  "OpenDNS",
  "Mullvad",
  "Shecan",
  "Electro",
  "RadarGame",
  "403.Online",
  "Begzar",
];

export function App() {
  return (
    <div className="relative overflow-x-clip">
      <div className="page-glow pointer-events-none absolute inset-x-0 top-0 h-[900px]" />

      <Nav />

      <main className="relative">
        <Hero />
        <Features />
        <Downloads />
      </main>

      <Footer />
    </div>
  );
}

function Nav() {
  return (
    <header className="relative z-10 mx-auto flex max-w-6xl items-center justify-between px-4 py-5 sm:px-6">
      <a href="#" className="flex items-center gap-2.5 font-semibold">
        <img src={logo} alt="" className="size-8" width={32} height={32} />
        ForceConnect
      </a>
      <nav className="flex items-center gap-1.5 text-sm">
        <a
          href="#features"
          className="text-ink-2 hover:text-ink hidden rounded-lg px-3 py-2 transition-colors sm:block"
        >
          Features
        </a>
        <a
          href={REPO}
          className="text-ink-2 hover:text-ink flex items-center gap-2 rounded-lg px-3 py-2 transition-colors"
        >
          <GitHubIcon className="size-4" />
          <span className="hidden sm:inline">GitHub</span>
        </a>
        <a
          href="#download"
          className="bg-ink text-ground hover:bg-ink-2 rounded-full px-4 py-2 font-medium transition-colors"
        >
          Download
        </a>
      </nav>
    </header>
  );
}

function Hero() {
  const detected = PLATFORMS.find((p) => p.id === detectPlatform());
  // Offer a direct download only when this visitor's build exists; everyone
  // else lands on the platform list, which says what is coming.
  const direct = detected?.available ? detected : null;

  return (
    <section className="mx-auto grid max-w-6xl items-center gap-12 px-4 pt-10 pb-20 sm:px-6 lg:grid-cols-[1fr_1.05fr] lg:pt-16 lg:pb-28">
      <div className="rise">
        <span className="border-line bg-tile/70 text-ink-2 inline-flex items-center gap-2 rounded-full border px-3 py-1 text-xs">
          <span className="bg-cyan size-1.5 rounded-full" />
          v2 — rebuilt from the ground up in Rust
        </span>

        <h1 className="font-display mt-6 text-[44px] leading-[1.02] font-semibold tracking-[-0.035em] text-balance sm:text-6xl lg:text-[68px]">
          Faster DNS.
          <br />
          <span className="text-gradient">One click away.</span>
        </h1>

        <p className="text-ink-2 mt-6 max-w-md text-lg leading-relaxed text-pretty">
          ForceConnect is a lightweight desktop DNS manager. Pick a resolver,
          press the button, and every query is routed through it — encrypted, if
          you want.
        </p>

        <div className="mt-8 flex flex-wrap items-center gap-3">
          <a
            href={direct ? RELEASES : "#download"}
            className="bg-accent hover:bg-accent/90 shadow-accent/30 inline-flex items-center gap-2 rounded-full px-6 py-3 font-medium text-white shadow-lg transition-colors"
          >
            <Download className="size-4.5" />
            {direct ? `Download for ${direct.name}` : "Download"}
          </a>
          <a
            href={REPO}
            className="border-line-strong text-ink hover:bg-tile inline-flex items-center gap-2 rounded-full border px-6 py-3 font-medium transition-colors"
          >
            <GitHubIcon className="size-4.5" />
            View source
          </a>
        </div>

        <p className="text-ink-3 mt-5 text-sm">
          Free &amp; open source · Windows, macOS &amp; Linux
        </p>
      </div>

      <div className="rise [animation-delay:120ms]">
        <Demo />
        <p className="text-ink-3 mt-3 text-center text-xs">
          Interactive preview — press the power button.
        </p>
      </div>
    </section>
  );
}

function Features() {
  return (
    <section
      id="features"
      className="mx-auto max-w-6xl scroll-mt-8 px-4 pb-24 sm:px-6"
    >
      <h2 className="font-display max-w-xl text-3xl font-semibold tracking-tight text-balance sm:text-4xl">
        Everything a DNS switcher should be.{" "}
        <span className="text-ink-3">Nothing it shouldn’t.</span>
      </h2>

      <div className="mt-10 grid gap-3 md:grid-cols-6">
        <Tile
          className="md:col-span-4"
          icon={<Lock />}
          title="Encrypted DNS, natively"
          body="DoH, DoT, DoQ and DoH3 through a resolver built into the app. No bundled third-party binaries, no proxies to configure."
        >
          <div className="mt-5 flex flex-wrap gap-2">
            {["DNS-over-HTTPS", "DNS-over-TLS", "DNS-over-QUIC", "DoH3"].map(
              (p) => (
                <span
                  key={p}
                  className="bg-elevated text-ink-2 rounded-full px-3 py-1 font-mono text-xs"
                >
                  {p}
                </span>
              ),
            )}
          </div>
        </Tile>

        <Tile
          className="md:col-span-2"
          icon={<Gauge />}
          title="Latency at a glance"
          body="Every server is pinged live, so the fastest one for your network is obvious."
        />

        <Tile
          className="md:col-span-2"
          icon={<ShieldCheck />}
          title="One prompt, at launch"
          body="Admin rights are asked for once. No service, no scheduled task, nothing left running."
        />

        <Tile
          className="md:col-span-4"
          icon={<Zap />}
          title={`${PROVIDERS.length} providers, ready to go`}
          body="Privacy-first, malware-blocking, family-safe and regional resolvers — or add your own."
        >
          <div className="mt-5 flex flex-wrap gap-1.5">
            {PROVIDERS.map((p) => (
              <span
                key={p}
                className="border-line text-ink-2 rounded-lg border px-2.5 py-1 text-xs"
              >
                {p}
              </span>
            ))}
          </div>
        </Tile>

        <Tile
          className="md:col-span-3"
          icon={<RotateCcw />}
          title="Crash-safe by design"
          body="A restore journal puts your original DNS back — even after a crash, forced shutdown or power loss."
        />

        <Tile
          className="md:col-span-3"
          icon={<Feather />}
          title="Small and native"
          body="Built with Rust and Tauri. A ~3 MB installer that opens instantly and stays light on memory."
        />
      </div>
    </section>
  );
}

function Tile({
  icon,
  title,
  body,
  className = "",
  children,
}: {
  icon: ReactNode;
  title: string;
  body: string;
  className?: string;
  children?: ReactNode;
}) {
  return (
    <article
      className={`border-line bg-tile hover:border-line-strong rounded-(--radius-tile) border p-6 transition-colors ${className}`}
    >
      <div className="bg-accent/12 text-accent-bright grid size-10 place-items-center rounded-xl [&>svg]:size-5">
        {icon}
      </div>
      <h3 className="mt-5 text-lg font-semibold">{title}</h3>
      <p className="text-ink-3 mt-1.5 leading-relaxed text-pretty">{body}</p>
      {children}
    </article>
  );
}

function Downloads() {
  return (
    <section
      id="download"
      className="mx-auto max-w-6xl scroll-mt-8 px-4 pb-20 sm:px-6"
    >
      <div className="relative overflow-hidden rounded-[28px] bg-[linear-gradient(150deg,#1b6bf2_0%,#1590e0_55%,#18b9d9_100%)] px-5 py-12 text-center sm:px-10">
        <div className="pointer-events-none absolute inset-0 bg-[radial-gradient(500px_260px_at_85%_0%,rgb(255_255_255/0.22),transparent_60%)]" />
        <div className="bg-ground relative mx-auto grid size-16 place-items-center rounded-2xl shadow-xl shadow-black/25">
          <img src={logo} alt="" className="size-11" width={44} height={44} />
        </div>
        <h2 className="font-display relative mt-5 text-3xl font-semibold tracking-tight text-white sm:text-4xl">
          Take control of your DNS.
        </h2>
        <p className="relative mx-auto mt-3 max-w-md text-white/85">
          Free and open source. Pick your platform.
        </p>

        <div className="relative mx-auto mt-8 grid max-w-3xl gap-3 text-left sm:grid-cols-3">
          {PLATFORMS.map((p) => {
            const body = (
              <>
                <span
                  className={`grid size-10 shrink-0 place-items-center rounded-xl [&>svg]:size-5 ${
                    p.available ? "bg-accent/10 text-accent" : "bg-white/15"
                  }`}
                >
                  {p.icon}
                </span>
                <span className="min-w-0">
                  <span className="flex items-center gap-2 font-semibold">
                    {p.name}
                    {!p.available && (
                      <span className="rounded-full bg-white/15 px-2 py-0.5 text-[11px] font-medium">
                        Coming soon
                      </span>
                    )}
                    {p.available && p.beta && (
                      <span className="bg-accent/10 text-accent rounded-full px-2 py-0.5 text-[11px] font-medium">
                        Beta
                      </span>
                    )}
                  </span>
                  <span
                    className={`mt-0.5 block text-xs ${
                      p.available ? "text-ink-3" : "text-white/75"
                    }`}
                  >
                    {p.detail}
                  </span>
                </span>
                {p.available && (
                  <Download className="text-accent ml-auto size-4.5 shrink-0" />
                )}
              </>
            );

            return p.available ? (
              <a
                key={p.id}
                href={RELEASES}
                className="text-ground flex items-center gap-3 rounded-2xl bg-white p-4 shadow-xl shadow-black/15 transition-transform hover:scale-[1.02]"
              >
                {body}
              </a>
            ) : (
              <div
                key={p.id}
                aria-disabled
                className="flex items-center gap-3 rounded-2xl border border-white/20 bg-white/8 p-4 text-white"
              >
                {body}
              </div>
            );
          })}
        </div>
      </div>
    </section>
  );
}

function Footer() {
  return (
    <footer className="border-line border-t">
      <div className="text-ink-3 mx-auto flex max-w-6xl flex-col items-center justify-between gap-3 px-4 py-8 text-sm sm:flex-row sm:px-6">
        <p>© {new Date().getFullYear()} Mxqius · MIT License</p>
        <div className="flex gap-5">
          <a href={REPO} className="hover:text-ink transition-colors">
            GitHub
          </a>
          <a
            href={`${REPO}/releases`}
            className="hover:text-ink transition-colors"
          >
            Releases
          </a>
          <a
            href={`${REPO}/issues`}
            className="hover:text-ink transition-colors"
          >
            Report an issue
          </a>
        </div>
      </div>
    </footer>
  );
}

/** lucide dropped brand marks, so the GitHub one is inlined. */
function GitHubIcon({ className }: { className?: string }) {
  return (
    <svg
      viewBox="0 0 24 24"
      fill="currentColor"
      className={className}
      aria-hidden
    >
      <path d="M12 .5a11.5 11.5 0 0 0-3.64 22.41c.58.1.79-.25.79-.56v-2c-3.2.7-3.88-1.36-3.88-1.36-.52-1.33-1.28-1.69-1.28-1.69-1.05-.72.08-.7.08-.7 1.16.08 1.77 1.19 1.77 1.19 1.03 1.77 2.7 1.26 3.36.96.1-.75.4-1.26.73-1.55-2.55-.29-5.24-1.28-5.24-5.68 0-1.26.45-2.28 1.19-3.09-.12-.29-.52-1.46.11-3.05 0 0 .97-.31 3.17 1.18a11 11 0 0 1 5.77 0c2.2-1.49 3.17-1.18 3.17-1.18.63 1.59.23 2.76.11 3.05.74.81 1.19 1.83 1.19 3.09 0 4.41-2.69 5.39-5.25 5.67.41.36.78 1.06.78 2.14v3.17c0 .31.21.67.8.56A11.5 11.5 0 0 0 12 .5Z" />
    </svg>
  );
}
