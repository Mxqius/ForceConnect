import { Collapsible as BaseCollapsible } from "@base-ui/react/collapsible";
import { ChevronDown } from "lucide-react";
import type { ComponentPropsWithoutRef, ReactNode } from "react";

import { cn } from "@/lib/cn";
import { focusRing } from "@/design/styles";

export function Collapsible({
  className,
  ...props
}: ComponentPropsWithoutRef<typeof BaseCollapsible.Root>) {
  return <BaseCollapsible.Root className={cn(className)} {...props} />;
}

interface CollapsibleTriggerProps extends ComponentPropsWithoutRef<
  typeof BaseCollapsible.Trigger
> {
  /** Rendered before the label, e.g. a section glyph. */
  startIcon?: ReactNode;
}

/**
 * A row that reads as a label at rest and a control on hover — the chevron is
 * the only hint this opens something, which is the point for a section most
 * people will never need to touch.
 */
export function CollapsibleTrigger({
  className,
  startIcon,
  children,
  ...props
}: CollapsibleTriggerProps) {
  return (
    <BaseCollapsible.Trigger
      className={cn(
        "text-text-muted hover:text-text-secondary group flex w-full cursor-pointer items-center justify-between rounded-lg py-1 text-left text-xs font-semibold tracking-wide uppercase transition-colors",
        focusRing,
        className,
      )}
      {...props}
    >
      <span className="inline-flex items-center gap-2">
        {startIcon}
        {children}
      </span>
      <ChevronDown className="size-3.5 shrink-0 transition-transform duration-200 data-[panel-open]:rotate-180" />
    </BaseCollapsible.Trigger>
  );
}

export function CollapsiblePanel({
  className,
  ...props
}: ComponentPropsWithoutRef<typeof BaseCollapsible.Panel>) {
  return (
    <BaseCollapsible.Panel
      className={cn(
        "h-[var(--collapsible-panel-height)] overflow-hidden transition-[height] duration-200 ease-out",
        "data-[ending-style]:h-0 data-[starting-style]:h-0",
        className,
      )}
      {...props}
    />
  );
}
