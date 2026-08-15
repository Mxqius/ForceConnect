import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

/**
 * Merge Tailwind classes so later ones win on conflict. Every primitive takes a
 * `className` prop and runs it through here, which is what makes the design
 * system overridable at the call site without `!important`.
 */
export function cn(...inputs: ClassValue[]): string {
  return twMerge(clsx(inputs));
}
