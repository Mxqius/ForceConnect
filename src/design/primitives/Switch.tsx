import { Checkbox as BaseCheckbox } from "@base-ui/react/checkbox";
import { Switch as BaseSwitch } from "@base-ui/react/switch";
import { Check, Minus } from "lucide-react";
import type { ComponentPropsWithoutRef } from "react";

import { cn } from "@/lib/cn";
import { focusRing } from "@/design/styles";

export function Switch({
  className,
  ...props
}: ComponentPropsWithoutRef<typeof BaseSwitch.Root>) {
  return (
    <BaseSwitch.Root
      className={cn(
        "bg-surface-active border-border-default relative h-5 w-9 shrink-0 cursor-pointer rounded-full border transition-colors",
        "data-[checked]:bg-accent data-[checked]:border-accent",
        "disabled:cursor-not-allowed disabled:opacity-50",
        focusRing,
        className,
      )}
      {...props}
    >
      <BaseSwitch.Thumb
        className={cn(
          "block size-4 rounded-full bg-white shadow-sm transition-[translate]",
          "translate-x-0.5 data-[checked]:translate-x-4",
        )}
      />
    </BaseSwitch.Root>
  );
}

export function Checkbox({
  className,
  ...props
}: ComponentPropsWithoutRef<typeof BaseCheckbox.Root>) {
  return (
    <BaseCheckbox.Root
      className={cn(
        "group border-border-strong bg-surface-inset flex size-4 shrink-0 cursor-pointer items-center justify-center rounded-sm border transition-colors",
        "data-[checked]:bg-accent data-[checked]:border-accent data-[checked]:text-text-on-accent",
        "data-[indeterminate]:bg-accent data-[indeterminate]:border-accent data-[indeterminate]:text-text-on-accent",
        "disabled:cursor-not-allowed disabled:opacity-50",
        focusRing,
        className,
      )}
      {...props}
    >
      <BaseCheckbox.Indicator className="flex items-center justify-center">
        <Check
          className="size-3 group-data-[indeterminate]:hidden"
          strokeWidth={3}
        />
        <Minus
          className="hidden size-3 group-data-[indeterminate]:block"
          strokeWidth={3}
        />
      </BaseCheckbox.Indicator>
    </BaseCheckbox.Root>
  );
}
