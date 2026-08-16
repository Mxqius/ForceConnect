import { X } from "lucide-react";
import type { ReactNode } from "react";

import { cn } from "@/lib/cn";
import { focusRing } from "@/design/styles";

export type BadgeTone =
  "neutral" | "accent" | "success" | "warning" | "danger" | "encrypted";

const TONES: Record<BadgeTone, string> = {
  neutral: "bg-surface-active text-text-secondary border-border-subtle",
  accent: "bg-accent-surface text-accent border-accent-muted",
  success: "bg-success-surface text-success border-success/30",
  warning: "bg-warning-surface text-warning border-warning/30",
  danger: "bg-danger-surface text-danger border-danger/30",
  encrypted:
    "bg-protocol-encrypted/12 text-protocol-encrypted border-protocol-encrypted/30",
};

interface BadgeProps {
  tone?: BadgeTone;
  /** Monospace + tabular figures. Use for addresses, ports, and latencies. */
  mono?: boolean;
  className?: string;
  children: ReactNode;
}

export function Badge({
  tone = "neutral",
  mono,
  className,
  children,
}: BadgeProps) {
  return (
    <span
      className={cn(
        "text-2xs inline-flex shrink-0 items-center gap-1 rounded-sm border px-1.5 py-0.5 leading-none font-medium",
        TONES[tone],
        mono && "font-mono tabular-nums",
        className,
      )}
    >
      {children}
    </span>
  );
}

interface ChipProps {
  onRemove?: () => void;
  /** Shown as the remove button's accessible name, e.g. "Remove tag fast". */
  removeLabel?: string;
  className?: string;
  children: ReactNode;
}

/** A badge the user can dismiss. Used for tags on custom servers. */
export function Chip({
  onRemove,
  removeLabel = "Remove",
  className,
  children,
}: ChipProps) {
  return (
    <span
      className={cn(
        "bg-surface-active text-text-secondary border-border-subtle inline-flex items-center gap-1 rounded-sm border py-0.5 pr-0.5 pl-1.5 text-xs",
        className,
      )}
    >
      {children}
      {onRemove ? (
        <button
          type="button"
          aria-label={removeLabel}
          onClick={onRemove}
          className={cn(
            "hover:bg-surface-hover hover:text-text-primary rounded-xs p-0.5 transition-colors",
            focusRing,
          )}
        >
          <X className="size-3" />
        </button>
      ) : null}
    </span>
  );
}

interface StatusDotProps {
  tone: "success" | "warning" | "danger" | "neutral";
  /** Adds a slow halo. Reserve it for genuinely live states. */
  pulse?: boolean;
  className?: string;
}

const DOT_TONES = {
  success: "bg-success",
  warning: "bg-warning",
  danger: "bg-danger",
  neutral: "bg-text-faint",
} as const;

export function StatusDot({ tone, pulse, className }: StatusDotProps) {
  return (
    <span className={cn("relative flex size-2 shrink-0", className)}>
      {pulse ? (
        <span
          className={cn(
            "absolute inline-flex size-full animate-ping rounded-full opacity-60",
            DOT_TONES[tone],
          )}
        />
      ) : null}
      <span
        className={cn(
          "relative inline-flex size-2 rounded-full",
          DOT_TONES[tone],
        )}
      />
    </span>
  );
}
