import { Check, Copy } from "lucide-react";
import { useEffect, useRef, useState, type ReactNode } from "react";

import { cn } from "@/lib/cn";
import { focusRing } from "@/design/styles";

interface EmptyStateProps {
  icon?: ReactNode;
  title: ReactNode;
  description?: ReactNode;
  action?: ReactNode;
  className?: string;
}

export function EmptyState({
  icon,
  title,
  description,
  action,
  className,
}: EmptyStateProps) {
  return (
    <div
      className={cn(
        "flex flex-col items-center justify-center gap-2 px-6 py-12 text-center",
        className,
      )}
    >
      {icon ? (
        <div className="text-text-faint mb-1 [&_svg]:size-6">{icon}</div>
      ) : null}
      <p className="text-text-secondary text-sm font-medium">{title}</p>
      {description ? (
        <p className="text-text-muted max-w-xs text-xs">{description}</p>
      ) : null}
      {action ? <div className="mt-2">{action}</div> : null}
    </div>
  );
}

export function Skeleton({ className }: { className?: string }) {
  return (
    <div
      aria-hidden
      className={cn("bg-surface-active animate-pulse rounded-sm", className)}
    />
  );
}

interface CopyableValueProps {
  value: string;
  /** Defaults to the value. Use when the display form differs, e.g. truncated. */
  children?: ReactNode;
  className?: string;
}

/**
 * Click to copy, with brief inline confirmation.
 *
 * The previous app copied a DNS address on click and flashed "Address copied"
 * for a second. It was a genuinely nice touch on a screen full of values people
 * need to paste elsewhere, so it survives the rewrite as a primitive.
 */
export function CopyableValue({
  value,
  children,
  className,
}: CopyableValueProps) {
  const [copied, setCopied] = useState(false);
  const timer = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);

  // Without this, unmounting mid-confirmation leaves a timer that sets state
  // on a gone component.
  useEffect(() => () => clearTimeout(timer.current), []);

  async function copy() {
    try {
      await navigator.clipboard.writeText(value);
      setCopied(true);
      clearTimeout(timer.current);
      timer.current = setTimeout(() => setCopied(false), 1200);
    } catch {
      // Clipboard access can be refused; silently leaving the value
      // selectable is a better outcome than an error toast for something
      // this incidental.
    }
  }

  return (
    <button
      type="button"
      onClick={() => void copy()}
      title={copied ? "Copied" : `Copy ${value}`}
      className={cn(
        "group text-text-secondary hover:text-text-primary inline-flex items-center gap-1.5 rounded-sm font-mono text-xs tabular-nums transition-colors",
        focusRing,
        className,
      )}
    >
      <span data-selectable>{children ?? value}</span>
      {copied ? (
        <Check className="text-success size-3 shrink-0" />
      ) : (
        <Copy className="size-3 shrink-0 opacity-0 transition-opacity group-hover:opacity-60" />
      )}
      <span className="sr-only" role="status">
        {copied ? "Copied to clipboard" : ""}
      </span>
    </button>
  );
}
