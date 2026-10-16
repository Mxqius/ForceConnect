import { Radio } from "@base-ui/react/radio";
import { RadioGroup } from "@base-ui/react/radio-group";
import type { ReactNode } from "react";

import { Tooltip } from "@/design/primitives/Tooltip";
import { focusRing } from "@/design/styles";
import { cn } from "@/lib/cn";

export interface SegmentedOption<T extends string> {
  value: T;
  label: ReactNode;
  icon?: ReactNode;
  disabled?: boolean;
  /**
   * Shown on hover. Disabled segments should always carry one: a greyed-out
   * choice with no explanation reads as a bug.
   */
  hint?: ReactNode;
}

interface SegmentedProps<T extends string> {
  /** Accessible name for the group. */
  label: string;
  value: T | null;
  onValueChange: (value: T) => void;
  options: readonly SegmentedOption<T>[];
  disabled?: boolean;
  /** Stretch segments to fill the container. */
  block?: boolean;
  className?: string;
}

/**
 * A small set of mutually exclusive choices, all visible at once.
 *
 * A radio group underneath, not a row of toggle buttons: exactly one segment is
 * always selected, and arrow keys move the selection the way they do in every
 * Windows segmented control.
 */
export function Segmented<T extends string>({
  label,
  value,
  onValueChange,
  options,
  disabled,
  block,
  className,
}: SegmentedProps<T>) {
  return (
    <RadioGroup
      aria-label={label}
      value={value}
      onValueChange={(next) => onValueChange(next as T)}
      disabled={disabled}
      className={cn(
        // Pill-in-pill: segment and track share one shape, so the selected
        // segment's ends follow the track's exactly at any height.
        "bg-surface-inset flex gap-1 rounded-full p-1",
        block ? "w-full" : "w-fit",
        disabled && "opacity-50",
        className,
      )}
    >
      {options.map((option) => {
        const segment = (
          <Radio.Root
            key={option.value}
            value={option.value}
            disabled={option.disabled}
            className={cn(
              "text-text-muted flex h-8 cursor-pointer items-center justify-center gap-1.5 rounded-full px-4 text-[13px] font-semibold whitespace-nowrap transition-colors select-none",
              "hover:text-text-primary [&_svg]:size-3.5 [&_svg]:shrink-0",
              "data-checked:bg-surface data-checked:text-text-primary data-checked:shadow-[0_1px_3px_oklch(0_0_0/0.25)]",
              "data-disabled:hover:text-text-muted data-disabled:cursor-not-allowed data-disabled:opacity-45",
              block && "flex-1",
              focusRing,
            )}
          >
            {option.icon}
            {option.label}
          </Radio.Root>
        );

        return option.hint ? (
          <Tooltip key={option.value} content={option.hint}>
            {segment}
          </Tooltip>
        ) : (
          segment
        );
      })}
    </RadioGroup>
  );
}
