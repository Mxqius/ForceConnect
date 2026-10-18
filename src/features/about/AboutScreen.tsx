import { useQuery } from "@tanstack/react-query";
import { openUrl } from "@tauri-apps/plugin-opener";
// lucide 1.x dropped brand marks, so these are generic by necessity.
import {
  Code2,
  ExternalLink,
  Globe,
  Send,
  type LucideIcon,
} from "lucide-react";
import type { ReactNode } from "react";

import { Badge, Button } from "@/design/primitives";
import { ipc } from "@/lib/ipc";
import { queryKeys } from "@/lib/query-keys";

import { PixelPhoenix } from "./PixelPhoenix";
import "./phoenix.css";

const LINKS: { label: string; href: string; icon: LucideIcon; hint: string }[] =
  [
    {
      label: "Source",
      href: "https://github.com/mxqius/forceconnect",
      icon: Code2,
      hint: "github.com/mxqius",
    },
    {
      label: "Website",
      href: "https://mxqius.ir",
      icon: Globe,
      hint: "mxqius.ir",
    },
    {
      label: "Telegram",
      href: "https://t.me/mxqius",
      icon: Send,
      hint: "@mxqius",
    },
  ];

/**
 * What the app is, who wrote it, and what it does with the rights it asks for.
 *
 * Laid out top-down and scrollable rather than vertically centred. The window
 * is a fixed 800×600 and the previous version centred a fixed-height column in
 * it, which meant that the moment the content grew past the viewport it
 * overflowed *both* ways — the top was pushed out of reach and no amount of
 * scrolling brought it back. Content that can grow has to start at the top.
 *
 * The app icon is gone. It appears in the title bar, the taskbar, the tray, and
 * the Start menu already; a fourth copy at 80px was the largest element on a
 * page whose actual subject is a paragraph of text about privileges.
 */
export function AboutScreen() {
  const { data } = useQuery({
    queryKey: queryKeys.appInfo,
    queryFn: ipc.appInfo,
  });

  return (
    <div className="mx-auto flex w-full max-w-2xl flex-col gap-8 p-8 pb-(--dock-clearance)">
      <header className="flex flex-col gap-3">
        <div className="flex flex-wrap items-center gap-3">
          <h1 className="text-2xl font-semibold tracking-tight">
            ForceConnect
          </h1>
          <Badge tone="accent" mono>
            {data?.version ?? "—"}
          </Badge>
        </div>

        <p className="text-text-secondary max-w-prose text-sm leading-relaxed">
          A DNS manager for Windows, macOS and Linux. Switch resolvers, see
          which adapter traffic actually leaves by, and route queries through
          encrypted DNS.
        </p>
      </header>

      {/* The fact strip doubles as a ledge: the phoenix perches on its top edge,
          which is why the two are grouped rather than spaced apart. */}
      <div className="flex flex-col">
        <PixelPhoenix />
        <section className="border-border-subtle bg-surface divide-border-subtle grid divide-y rounded-2xl border sm:grid-cols-3 sm:divide-x sm:divide-y-0">
          <Fact label="Platform">{data?.os ?? "—"}</Fact>
          <Fact label="Licence">MIT</Fact>
          <Fact label="Built by">MXQIUS</Fact>
        </section>
      </div>

      <section className="flex flex-col gap-3">
        <h2 className="text-text-secondary text-xs font-semibold tracking-widest uppercase">
          Administrator rights
        </h2>
        <p className="text-text-muted max-w-prose text-sm leading-relaxed">
          ForceConnect asks for administrator rights once, when it starts,
          because changing an adapter&rsquo;s DNS cannot be done without them.
          It holds them for that session and nothing else: there is no service,
          nothing left installed, and whatever it changes goes back when you
          disconnect or close it. The source is public and the design is
          documented.
        </p>
      </section>

      <section className="flex flex-col gap-3">
        <h2 className="text-text-secondary text-xs font-semibold tracking-widest uppercase">
          Links
        </h2>
        <div className="grid gap-2 sm:grid-cols-3">
          {LINKS.map(({ label, href, icon: Icon, hint }) => (
            <Button
              key={href}
              onClick={() => void openUrl(href)}
              // Stacked label and address, so a link says where it goes before
              // it is clicked — which matters more than usual for a control
              // that hands the user off to an external browser.
              className="h-auto justify-start gap-3 px-4 py-3 text-left"
            >
              <Icon className="text-text-muted" />
              <span className="flex min-w-0 flex-col">
                <span className="text-sm font-medium">{label}</span>
                <span className="text-text-faint truncate text-xs font-normal">
                  {hint}
                </span>
              </span>
              <ExternalLink className="text-text-faint ml-auto size-3.5 shrink-0" />
            </Button>
          ))}
        </div>
      </section>
    </div>
  );
}

function Fact({ label, children }: { label: string; children: ReactNode }) {
  return (
    <div className="flex min-w-0 flex-col gap-1.5 px-5 py-4">
      <span className="text-text-faint text-2xs font-semibold tracking-widest uppercase">
        {label}
      </span>
      <span className="text-text-secondary truncate text-sm">{children}</span>
    </div>
  );
}
