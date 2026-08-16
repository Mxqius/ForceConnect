import { Progress as BaseProgress } from "@base-ui/react/progress";
import { Separator as BaseSeparator } from "@base-ui/react/separator";
import type { ComponentPropsWithoutRef } from "react";

import { cn } from "@/lib/cn";

interface ProgressProps extends ComponentPropsWithoutRef<
  typeof BaseProgress.Root
> {
  /** Accessible name — required, since a bare bar tells assistive tech nothing. */
  label: string;
  tone?: "accent" | "success" | "danger";
}

const TONES = {
  accent: "bg-accent",
  success: "bg-success",
  danger: "bg-danger",
} as const;

export function Progress({
  label,
  tone = "accent",
  className,
  ...props
}: ProgressProps) {
  return (
    <BaseProgress.Root
      aria-label={label}
      className={cn("w-full", className)}
      {...props}
    >
      <BaseProgress.Track className="bg-surface-active h-1 w-full overflow-hidden rounded-full">
        <BaseProgress.Indicator
          className={cn(
            "h-full rounded-full transition-[width] duration-300 ease-out",
            TONES[tone],
          )}
        />
      </BaseProgress.Track>
    </BaseProgress.Root>
  );
}

export function Separator({
  className,
  ...props
}: ComponentPropsWithoutRef<typeof BaseSeparator>) {
  return (
    <BaseSeparator
      className={cn(
        "bg-border-subtle shrink-0",
        "data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full",
        "data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px",
        className,
      )}
      {...props}
    />
  );
}
