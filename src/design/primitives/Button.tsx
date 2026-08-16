import { Button as BaseButton } from "@base-ui/react/button";
import type { ComponentPropsWithoutRef, ReactNode } from "react";

import { cn } from "@/lib/cn";
import { focusRing, interactiveBase } from "@/design/styles";

export type ButtonVariant =
  "primary" | "secondary" | "ghost" | "danger" | "link";
export type ButtonSize = "sm" | "md" | "lg";

const VARIANTS: Record<ButtonVariant, string> = {
  primary:
    "bg-accent text-text-on-accent border border-transparent hover:bg-accent-hover active:bg-accent-active",
  secondary:
    "bg-surface text-text-primary border border-border-default hover:bg-surface-hover hover:border-border-strong active:bg-surface-active",
  ghost:
    "bg-transparent text-text-secondary border border-transparent hover:bg-surface-hover hover:text-text-primary active:bg-surface-active",
  danger:
    "bg-danger text-white border border-transparent hover:bg-danger-hover",
  link: "bg-transparent text-accent border border-transparent underline-offset-2 hover:underline p-0 h-auto",
};

const SIZES: Record<ButtonSize, string> = {
  sm: "h-7 px-2.5 text-xs gap-1.5 rounded-sm",
  md: "h-8 px-3 text-sm gap-2 rounded-md",
  lg: "h-10 px-4 text-base gap-2 rounded-md",
};

interface ButtonProps extends ComponentPropsWithoutRef<typeof BaseButton> {
  variant?: ButtonVariant;
  size?: ButtonSize;
  /** Rendered before the label. Sized to match the text, not the button. */
  startIcon?: ReactNode;
  endIcon?: ReactNode;
  /** Occupies the full width of its container. */
  block?: boolean;
}

export function Button({
  variant = "secondary",
  size = "md",
  startIcon,
  endIcon,
  block,
  className,
  children,
  ...props
}: ButtonProps) {
  return (
    <BaseButton
      className={cn(
        interactiveBase,
        focusRing,
        VARIANTS[variant],
        variant !== "link" && SIZES[size],
        block && "w-full",
        // Sizing icons here rather than at every call site keeps a stray
        // 24px lucide icon from silently changing the button's height.
        "[&_svg]:size-4 [&_svg]:shrink-0",
        className,
      )}
      {...props}
    >
      {startIcon}
      {children}
      {endIcon}
    </BaseButton>
  );
}

interface IconButtonProps extends Omit<ButtonProps, "startIcon" | "endIcon"> {
  /** Required: an icon-only control is invisible to screen readers without it. */
  label: string;
}

const ICON_SIZES: Record<ButtonSize, string> = {
  sm: "size-7 rounded-sm",
  md: "size-8 rounded-md",
  lg: "size-10 rounded-md",
};

export function IconButton({
  variant = "ghost",
  size = "md",
  label,
  className,
  children,
  ...props
}: IconButtonProps) {
  return (
    <BaseButton
      aria-label={label}
      className={cn(
        interactiveBase,
        focusRing,
        VARIANTS[variant],
        ICON_SIZES[size],
        "p-0 [&_svg]:size-4 [&_svg]:shrink-0",
        className,
      )}
      {...props}
    >
      {children}
    </BaseButton>
  );
}
