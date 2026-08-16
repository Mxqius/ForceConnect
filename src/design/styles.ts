import { cn } from "@/lib/cn";

/**
 * Style fragments shared across primitives.
 *
 * These live apart from the components so a look is defined once rather than
 * per file — and so component modules export only components, which is what
 * keeps fast refresh working during development.
 */

/** One focus treatment for the whole app. */
export const focusRing =
  "outline-none focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent";

/** Layout and disabled behaviour common to every clickable control. */
export const interactiveBase =
  "inline-flex select-none items-center justify-center whitespace-nowrap font-medium transition-colors disabled:pointer-events-none disabled:opacity-45";

/** Text inputs and anything that should read as a text input. */
export const controlBase = cn(
  "w-full rounded-md border border-border-default bg-surface-inset px-2.5 text-sm text-text-primary",
  "placeholder:text-text-faint transition-colors",
  "hover:border-border-strong",
  "focus:border-accent",
  "disabled:cursor-not-allowed disabled:opacity-50",
  // Base UI marks a failed field with data-invalid; styling the attribute
  // rather than a prop means validity and appearance cannot disagree.
  "data-[invalid]:border-danger",
  focusRing,
);

/** Floating surfaces: popovers, tooltips, menus, select popups. */
export const overlaySurface = cn(
  "z-50 rounded-md border border-border-default bg-surface-elevated shadow-popup",
  "transition-[opacity,transform] duration-100",
  "data-[starting-style]:scale-[0.97] data-[starting-style]:opacity-0",
  "data-[ending-style]:scale-[0.97] data-[ending-style]:opacity-0",
);
