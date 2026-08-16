import { Field as BaseField } from "@base-ui/react/field";
import { Input as BaseInput } from "@base-ui/react/input";
import type { ComponentPropsWithoutRef, ReactNode } from "react";

import { cn } from "@/lib/cn";
import { controlBase } from "@/design/styles";

export function Input({
  className,
  ...props
}: ComponentPropsWithoutRef<typeof BaseInput>) {
  return <BaseInput className={cn(controlBase, "h-8", className)} {...props} />;
}

interface FieldProps {
  label?: ReactNode;
  /** Static helper text. Hidden while an error is showing. */
  description?: ReactNode;
  /** Marks the control required and shows the affordance on the label. */
  required?: boolean;
  className?: string;
  children: ReactNode;
}

/**
 * Label, control, description, and validation message as one unit.
 *
 * Base UI wires the `for`/`id`/`aria-describedby` relationships between these
 * parts, which is the whole reason to use it rather than a bare `<label>`.
 */
export function Field({
  label,
  description,
  required,
  className,
  children,
}: FieldProps) {
  return (
    <BaseField.Root className={cn("flex flex-col gap-1.5", className)}>
      {label ? (
        <BaseField.Label className="text-text-secondary flex items-center gap-1 text-xs font-medium">
          {label}
          {required ? (
            <span className="text-danger" aria-hidden>
              *
            </span>
          ) : null}
        </BaseField.Label>
      ) : null}

      {children}

      <BaseField.Error className="text-danger text-xs" />

      {description ? (
        <BaseField.Description className="text-text-muted text-xs">
          {description}
        </BaseField.Description>
      ) : null}
    </BaseField.Root>
  );
}

Field.Root = BaseField.Root;
Field.Label = BaseField.Label;
Field.Control = BaseField.Control;
Field.Description = BaseField.Description;
Field.Error = BaseField.Error;
Field.Validity = BaseField.Validity;
