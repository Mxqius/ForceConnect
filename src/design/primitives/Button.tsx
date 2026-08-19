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
    "bg-danger-strong text-white border border-transparent hover:bg-danger-hover",
  link: "bg-transparent text-accent-bright border border-transparent underline-offset-2 hover:underline p-0 h-auto",
};

/**
 * Heights are on a 4px grid and start at 36px.
 *
 * The previous scale topped out where this one begins. A 28px button with 12px
 * of padding is a web-page density; this is a desktop utility people click
 * quickly and often, and every size here now clears the 32px minimum target
 * that Windows' own controls use. Padding grew faster than height on purpose —
 * cramped horizontal padding is what made the old buttons read as small even
 * at the sizes where they were not.
 */
const SIZES: Record<ButtonSize, string> = {
  sm: "h-9 px-4 text-xs gap-2 rounded-lg",
  md: "h-11 px-5 text-sm gap-2 rounded-xl",
  lg: "h-13 px-7 text-base gap-2.5 rounded-xl",
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
        GLYPHS[size],
        "[&_svg]:shrink-0",
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

/**
 * Glyph size per button size.
 *
 * Pinned here rather than at the call sites so a stray 24px lucide icon cannot
 * silently change a button's height, and so icons scale with the control
 * instead of staying 16px inside a 52px button.
 */
const GLYPHS: Record<ButtonSize, string> = {
  sm: "[&_svg]:size-4",
  md: "[&_svg]:size-[1.125rem]",
  lg: "[&_svg]:size-5",
};

/** Square counterparts to `SIZES`, so an icon button lines up in a row of them. */
const ICON_SIZES: Record<ButtonSize, string> = {
  sm: "size-9 rounded-lg",
  md: "size-11 rounded-xl",
  lg: "size-13 rounded-xl",
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
        GLYPHS[size],
        "p-0 [&_svg]:shrink-0",
        className,
      )}
      {...props}
    >
      {children}
    </BaseButton>
  );
}
