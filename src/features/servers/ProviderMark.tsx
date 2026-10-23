import { cn } from "@/lib/cn";

/**
 * Provider logos, by key: `src/assets/providers/<provider>.svg` (or `.png`,
 * `.webp`) is picked up at build time, where `<provider>` is the server's
 * `provider` field in `servers.json` — `cloudflare.svg`, `quad9.svg`. Adding a
 * logo is a file drop, with no code to change. SVG is preferred: the marks
 * are drawn at several sizes, and a small bitmap blurs on a HiDPI display.
 */
const LOGOS: Record<string, string> = Object.fromEntries(
  Object.entries(
    import.meta.glob<string>("../../assets/providers/*.{svg,png,webp}", {
      eager: true,
      query: "?url",
      import: "default",
    }),
  ).map(([path, url]) => [
    path.slice(path.lastIndexOf("/") + 1, path.lastIndexOf(".")),
    url,
  ]),
);

/**
 * A provider's logo, or a monogram where there is none.
 *
 * The monogram is honest about being a stand-in while still giving each row a
 * stable, scannable anchor. Its hue is derived from the provider key, so a
 * given provider always looks the same.
 */
export function ProviderMark({
  provider,
  name,
  className,
}: {
  provider: string;
  name: string;
  className?: string;
}) {
  const logo = LOGOS[provider];
  if (logo) {
    return (
      <span
        aria-hidden
        className={cn(
          "bg-surface-inset flex size-8 shrink-0 items-center justify-center overflow-hidden rounded-md",
          className,
        )}
      >
        <img
          src={logo}
          alt=""
          draggable={false}
          className="size-[70%] object-contain"
        />
      </span>
    );
  }

  const hue = hashHue(provider);
  const initial = name.trim().charAt(0).toUpperCase() || "?";

  return (
    <span
      aria-hidden
      className={cn(
        "flex size-8 shrink-0 items-center justify-center rounded-md text-sm font-semibold",
        className,
      )}
      style={{
        // oklch keeps the generated colours at a consistent perceived
        // lightness, so no provider ends up with unreadable contrast.
        backgroundColor: `oklch(0.32 0.06 ${hue})`,
        color: `oklch(0.86 0.11 ${hue})`,
      }}
    >
      {initial}
    </span>
  );
}

function hashHue(value: string): number {
  let hash = 0;
  for (let i = 0; i < value.length; i += 1) {
    hash = (hash * 31 + value.charCodeAt(i)) % 360;
  }
  return hash;
}
