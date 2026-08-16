import { useQuery } from "@tanstack/react-query";
import { openUrl } from "@tauri-apps/plugin-opener";
// lucide 1.x dropped brand marks, so these are generic by necessity.
import { Code2, ExternalLink, Globe, Send, ShieldCheck } from "lucide-react";

import { Button } from "@/design/primitives";
import { ipc } from "@/lib/ipc";
import { queryKeys } from "@/lib/query-keys";

const LINKS = [
  {
    label: "Source",
    href: "https://github.com/mxqius/forceconnect",
    icon: Code2,
  },
  { label: "Website", href: "https://mxqius.ir", icon: Globe },
  { label: "Telegram", href: "https://t.me/mxqius", icon: Send },
];

export function AboutScreen() {
  const { data } = useQuery({
    queryKey: queryKeys.appInfo,
    queryFn: ipc.appInfo,
  });

  return (
    <div className="mx-auto flex max-w-lg flex-col items-center gap-6 p-6 text-center">
      <div className="bg-accent-surface text-accent flex size-16 items-center justify-center rounded-2xl">
        <ShieldCheck className="size-8" />
      </div>

      <div>
        <h1 className="text-2xl font-semibold tracking-tight">ForceConnect</h1>
        <p className="text-text-muted mt-1 text-sm">
          Version {data?.version ?? "—"}
        </p>
      </div>

      <p className="text-text-secondary text-sm leading-relaxed">
        A DNS manager for Windows. Switch resolvers, see which adapter traffic
        actually leaves by, and route queries through encrypted DNS.
      </p>

      <div className="border-border-subtle bg-surface text-text-muted rounded-2xl border px-4 py-3 text-xs leading-relaxed">
        The interface never runs as administrator. A small, separate helper
        holds the few operations that need it, and only while the app is open —
        the source for both is public and the design is documented.
      </div>

      <div className="flex flex-wrap justify-center gap-2">
        {LINKS.map(({ label, href, icon: Icon }) => (
          <Button
            key={href}
            size="sm"
            startIcon={<Icon />}
            endIcon={<ExternalLink className="opacity-50" />}
            onClick={() => void openUrl(href)}
          >
            {label}
          </Button>
        ))}
      </div>

      <p className="text-text-faint text-xs">MIT licensed · Built by Mxqius</p>
    </div>
  );
}
