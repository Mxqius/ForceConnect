import { cn } from "@/lib/cn";

/**
 * Stand-in provider mark.
 *
 * Real logos are pending — the 1.x app shipped 431-byte PNGs that would not
 * survive a HiDPI display, so they were not worth carrying over. A generated
 * monogram is honest about being a placeholder while still giving each row a
 * stable, scannable anchor. The hue is derived from the provider key so a
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
