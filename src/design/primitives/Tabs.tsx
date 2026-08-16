import { Tabs as BaseTabs } from "@base-ui/react/tabs";
import type { ComponentPropsWithoutRef, ReactNode } from "react";

import { cn } from "@/lib/cn";
import { focusRing } from "@/design/styles";

export function Tabs({
  className,
  ...props
}: ComponentPropsWithoutRef<typeof BaseTabs.Root>) {
  return (
    <BaseTabs.Root className={cn("flex flex-col", className)} {...props} />
  );
}

export function TabList({
  className,
  ...props
}: ComponentPropsWithoutRef<typeof BaseTabs.List>) {
  return (
    <BaseTabs.List
      className={cn(
        "border-border-subtle relative flex items-center gap-1 border-b",
        className,
      )}
      {...props}
    >
      {props.children}
      <BaseTabs.Indicator
        className={cn(
          "bg-accent absolute bottom-0 left-0 h-px",
          // Base UI publishes the active tab's geometry as CSS variables, so
          // the indicator can animate without any measuring in JS.
          "w-[var(--active-tab-width)] translate-x-[var(--active-tab-left)]",
          "transition-[translate,width] duration-200 ease-out",
        )}
      />
    </BaseTabs.List>
  );
}

interface TabProps extends ComponentPropsWithoutRef<typeof BaseTabs.Tab> {
  /** Rendered as a muted count beside the label, e.g. a result total. */
  badge?: ReactNode;
}

export function Tab({ className, badge, children, ...props }: TabProps) {
  return (
    <BaseTabs.Tab
      className={cn(
        "text-text-muted relative -mb-px flex cursor-pointer items-center gap-1.5 rounded-t-sm px-3 py-2 text-sm font-medium transition-colors",
        "hover:text-text-secondary",
        "data-[selected]:text-text-primary",
        focusRing,
        className,
      )}
      {...props}
    >
      {children}
      {badge !== undefined ? (
        <span className="bg-surface-active text-text-muted text-2xs rounded-full px-1.5 py-0.5 tabular-nums">
          {badge}
        </span>
      ) : null}
    </BaseTabs.Tab>
  );
}

export function TabPanel({
  className,
  ...props
}: ComponentPropsWithoutRef<typeof BaseTabs.Panel>) {
  return (
    <BaseTabs.Panel className={cn("outline-none", className)} {...props} />
  );
}
