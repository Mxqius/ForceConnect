import { Popover as BasePopover } from "@base-ui/react/popover";
import { Tooltip as BaseTooltip } from "@base-ui/react/tooltip";
import type { ReactElement, ReactNode } from "react";

import { cn } from "@/lib/cn";
import { overlayPositioner, overlaySurface } from "@/design/styles";

/**
 * Wraps the app once so tooltips share a delay group — moving between adjacent
 * controls shows the next tooltip immediately instead of re-waiting.
 */
export function TooltipProvider({ children }: { children: ReactNode }) {
  return (
    <BaseTooltip.Provider delay={400} closeDelay={100}>
      {children}
    </BaseTooltip.Provider>
  );
}

interface TooltipProps {
  content: ReactNode;
  side?: "top" | "bottom" | "left" | "right";
  /** Must be a single element: Base UI clones it to attach trigger behaviour. */
  children: ReactElement;
}

export function Tooltip({ content, side = "top", children }: TooltipProps) {
  return (
    <BaseTooltip.Root>
      <BaseTooltip.Trigger render={children} />
      <BaseTooltip.Portal>
        <BaseTooltip.Positioner
          className={overlayPositioner}
          side={side}
          sideOffset={6}
        >
          <BaseTooltip.Popup
            className={cn(
              overlaySurface,
              "text-text-secondary max-w-64 px-3 py-2 text-xs",
            )}
          >
            {content}
          </BaseTooltip.Popup>
        </BaseTooltip.Positioner>
      </BaseTooltip.Portal>
    </BaseTooltip.Root>
  );
}

interface PopoverProps {
  /** Must be a single element: Base UI clones it to attach trigger behaviour. */
  trigger: ReactElement;
  side?: "top" | "bottom" | "left" | "right";
  align?: "start" | "center" | "end";
  className?: string;
  children: ReactNode;
}

export function Popover({
  trigger,
  side = "bottom",
  align = "start",
  className,
  children,
}: PopoverProps) {
  return (
    <BasePopover.Root>
      <BasePopover.Trigger render={trigger} />
      <BasePopover.Portal>
        <BasePopover.Positioner
          className={overlayPositioner}
          side={side}
          align={align}
          sideOffset={6}
        >
          <BasePopover.Popup className={cn(overlaySurface, "p-4", className)}>
            {children}
          </BasePopover.Popup>
        </BasePopover.Positioner>
      </BasePopover.Portal>
    </BasePopover.Root>
  );
}

Popover.Close = BasePopover.Close;
Popover.Title = BasePopover.Title;
Popover.Description = BasePopover.Description;
