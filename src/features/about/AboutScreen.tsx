import { useQuery } from "@tanstack/react-query";
import { openUrl } from "@tauri-apps/plugin-opener";
// lucide 1.x dropped brand marks, so these are generic by necessity.
import { Bug, Code2, Globe, Heart, type LucideIcon } from "lucide-react";

import { Button } from "@/design/primitives";
import { ipc } from "@/lib/ipc";
import { queryKeys } from "@/lib/query-keys";

import { PixelPhoenix } from "./PixelPhoenix";
import "./phoenix.css";

const REPO = "https://github.com/mxqius/forceconnect";

/** The donation page. The Donate link is shown once this is set. */
const DONATE_URL = "";

const LINKS: { label: string; href: string; icon: LucideIcon }[] = [
  { label: "Source code", href: REPO, icon: Code2 },
  { label: "Website", href: "https://mxqius.ir", icon: Globe },
  { label: "Report an issue", href: `${REPO}/issues`, icon: Bug },
];

export function AboutScreen() {
  const { data } = useQuery({
    queryKey: queryKeys.appInfo,
    queryFn: ipc.appInfo,
  });

  return (
    // `my-auto` rather than centring the container: auto margins give way
    // when the content is taller than the window, so it scrolls instead of
    // overflowing off the top.
    <div className="flex min-h-full flex-col px-8 pt-8 pb-(--dock-clearance)">
      <div className="mx-auto my-auto w-full max-w-md">
        {/* The rule under the bird is its perch. */}
        <PixelPhoenix />
        <div className="border-border-subtle border-t pt-6">
          <div className="flex items-baseline gap-3">
            <h1 className="text-3xl font-semibold tracking-tight">
              ForceConnect
            </h1>
            <span className="text-text-muted font-mono text-sm">
              {data ? `v${data.version}` : ""}
            </span>
          </div>

          <p className="text-text-secondary mt-2 text-sm leading-relaxed">
            Encrypted DNS for Windows, macOS and Linux. Pick a resolver and send
            every lookup over DoH, DoT, DoQ or DoH3.
          </p>

          <p className="text-text-muted mt-4 text-sm">
            Built by <span className="text-text-primary">MXQIUS</span>
          </p>

          <nav aria-label="Links" className="mt-6 flex flex-wrap gap-2">
            {LINKS.map(({ label, href, icon: Icon }) => (
              <Button
                key={href}
                size="sm"
                className="rounded-full"
                startIcon={<Icon />}
                onClick={() => void openUrl(href)}
              >
                {label}
              </Button>
            ))}
            {DONATE_URL ? (
              <Button
                size="sm"
                variant="primary"
                className="rounded-full"
                startIcon={<Heart />}
                onClick={() => void openUrl(DONATE_URL)}
              >
                Donate
              </Button>
            ) : null}
          </nav>
        </div>
      </div>
    </div>
  );
}
