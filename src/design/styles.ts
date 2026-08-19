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

/**
 * Text inputs and anything that should read as a text input.
 *
 * Sized to match `Button`'s `md`: a form row where the field and the button
 * beside it differ by two pixels looks like a mistake, and matching them is
 * most of what makes a dialog look considered.
 */
export const controlBase = cn(
  "w-full rounded-xl border border-border-default bg-surface-inset px-4 text-sm text-text-primary",
  "placeholder:text-text-muted transition-colors",
  "hover:border-border-strong",
  "focus:border-accent",
  "disabled:cursor-not-allowed disabled:opacity-50",
  // Base UI marks a failed field with data-invalid; styling the attribute
  // rather than a prop means validity and appearance cannot disagree.
  "data-[invalid]:border-danger",
  focusRing,
);

/**
 * The positioning wrapper every floating part sits in.
 *
 * The z-index belongs *here*, not on the popup. Base UI's `Popup` is a static
 * child of its `Positioner`, and `z-index` on a statically positioned element
 * is ignored — so a popup that appeared to be at `z-50` was really painting at
 * the document's base layer and ended up *behind* any open dialog. That is what
 * made the protocol select in the add-server dialog look like it never opened:
 * it opened, correctly positioned, underneath the dialog.
 *
 * The value is above `--z-dialog` on purpose. These are opened *from* dialogs,
 * so they have to clear them; the toast viewport stays above everything.
 */
export const overlayPositioner = "z-60 outline-none";

/** Floating surfaces: popovers, tooltips, menus, select popups. */
export const overlaySurface = cn(
  "rounded-xl border border-border-default bg-surface-elevated shadow-popup",
  "transition-[opacity,transform] duration-100",
  "data-[starting-style]:scale-[0.97] data-[starting-style]:opacity-0",
  "data-[ending-style]:scale-[0.97] data-[ending-style]:opacity-0",
);
