import { Select as BaseSelect } from "@base-ui/react/select";
import { Check, ChevronsUpDown } from "lucide-react";
import type { ReactNode } from "react";

import { cn } from "@/lib/cn";
import { focusRing, overlayPositioner, overlaySurface } from "@/design/styles";

export interface SelectOption<T extends string> {
  value: T;
  label: ReactNode;
  /** Secondary line, e.g. an adapter's description or address. */
  hint?: ReactNode;
  disabled?: boolean;
}

interface SelectProps<T extends string> {
  value: T | null;
  /**
   * Receives `null` when the selection is cleared, so callers have to decide
   * what an empty selection means rather than having one silently invented.
   */
  onValueChange: (value: T | null) => void;
  options: SelectOption<T>[];
  placeholder?: string;
  disabled?: boolean;
  className?: string;
  /** Accessible name when the select is not wrapped in a Field. */
  label?: string;
}

export function Select<T extends string>({
  value,
  onValueChange,
  options,
  placeholder = "Select…",
  disabled,
  className,
  label,
}: SelectProps<T>) {
  return (
    <BaseSelect.Root
      value={value}
      onValueChange={onValueChange}
      disabled={disabled}
      items={options}
    >
      <BaseSelect.Trigger
        aria-label={label}
        className={cn(
          // Matches `Input` and `Button`'s `md`, so a select never looks like
          // the runt of a form row.
          "border-border-default bg-surface-inset text-text-primary flex h-11 w-full items-center justify-between gap-2 rounded-xl border px-4 text-sm",
          "hover:border-border-strong transition-colors",
          "data-[popup-open]:border-accent",
          "disabled:cursor-not-allowed disabled:opacity-50",
          focusRing,
          className,
        )}
      >
        <BaseSelect.Value className="truncate">
          {(selectedValue: T | null) => {
            const selected = options.find(
              (option) => option.value === selectedValue,
            );
            return selected ? (
              selected.label
            ) : (
              <span className="text-text-muted">{placeholder}</span>
            );
          }}
        </BaseSelect.Value>
        <BaseSelect.Icon>
          <ChevronsUpDown className="text-text-muted size-4 shrink-0" />
        </BaseSelect.Icon>
      </BaseSelect.Trigger>

      <BaseSelect.Portal>
        <BaseSelect.Positioner
          className={overlayPositioner}
          sideOffset={4}
          alignItemWithTrigger={false}
        >
          <BaseSelect.Popup
            className={cn(
              overlaySurface,
              "fc-scroll max-h-80 min-w-[var(--anchor-width)] overflow-y-auto p-1.5",
            )}
          >
            {/* `List` is not optional decoration: it owns the composite
                roving-focus context, so keyboard navigation and typeahead only
                work for items inside it. Scrolling stays on the popup rather
                than moving to the list, because the popup is the element with
                the border and the padding — clipping inside it would cut the
                first and last rows off mid-highlight. */}
            <BaseSelect.List>
              {options.map((option) => (
                <BaseSelect.Item
                  key={option.value}
                  value={option.value}
                  disabled={option.disabled}
                  className={cn(
                    "group flex cursor-pointer items-center gap-2.5 rounded-lg px-3 py-2.5 text-sm outline-none",
                    "data-[highlighted]:bg-surface-hover",
                    "data-[disabled]:pointer-events-none data-[disabled]:opacity-40",
                  )}
                >
                  {/* Fixed-width slot: the indicator unmounts when the item is
                      not selected, and without reserved space every other row
                      would shift left. */}
                  <span className="flex w-4 shrink-0 justify-center">
                    <BaseSelect.ItemIndicator className="text-accent-bright flex">
                      <Check className="size-4" strokeWidth={3} />
                    </BaseSelect.ItemIndicator>
                  </span>
                  <span className="flex min-w-0 flex-1 flex-col group-data-[selected]:font-medium">
                    <BaseSelect.ItemText className="truncate">
                      {option.label}
                    </BaseSelect.ItemText>
                    {option.hint ? (
                      <span className="text-text-muted truncate text-xs">
                        {option.hint}
                      </span>
                    ) : null}
                  </span>
                </BaseSelect.Item>
              ))}
            </BaseSelect.List>
          </BaseSelect.Popup>
        </BaseSelect.Positioner>
      </BaseSelect.Portal>
    </BaseSelect.Root>
  );
}
