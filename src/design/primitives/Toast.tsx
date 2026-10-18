import { Toast as BaseToast } from "@base-ui/react/toast";
import {
  AlertTriangle,
  CheckCircle2,
  Info,
  X,
  XCircle,
  type LucideIcon,
} from "lucide-react";
import type { ReactNode } from "react";

import { cn } from "@/lib/cn";
import { IconButton } from "@/design/primitives/Button";

export type ToastTone = "success" | "error" | "warning" | "info";

/**
 * Module-level manager.
 *
 * Deliberately not a hook: connect/disconnect results, adapter-change events,
 * and command failures all need to raise a toast from places that are not
 * React components. A hook-only API would push a context read into every one
 * of them.
 */
const manager = BaseToast.createToastManager();

interface NotifyOptions {
  description?: ReactNode;
  /** Milliseconds. `0` keeps the toast until dismissed. */
  timeout?: number;
  action?: { label: string; onClick: () => void };
}

function notify(
  tone: ToastTone,
  title: ReactNode,
  options: NotifyOptions = {},
) {
  return manager.add({
    title,
    description: options.description,
    type: tone,
    // Matches the 5s hold the previous app used, which read about right.
    // Errors stay until dismissed: they usually need the user to do something.
    timeout: options.timeout ?? (tone === "error" ? 0 : 5000),
    ...(options.action
      ? {
          actionProps: {
            children: options.action.label,
            onClick: options.action.onClick,
          },
        }
      : {}),
  });
}

export const toast = {
  success: (title: ReactNode, options?: NotifyOptions) =>
    notify("success", title, options),
  error: (title: ReactNode, options?: NotifyOptions) =>
    notify("error", title, options),
  warning: (title: ReactNode, options?: NotifyOptions) =>
    notify("warning", title, options),
  info: (title: ReactNode, options?: NotifyOptions) =>
    notify("info", title, options),
  close: (id?: string) => manager.close(id),
  promise: manager.promise,
};

const TONE_ICONS: Record<ToastTone, LucideIcon> = {
  success: CheckCircle2,
  error: XCircle,
  warning: AlertTriangle,
  info: Info,
};

const TONE_COLORS: Record<ToastTone, string> = {
  success: "text-success",
  error: "text-danger",
  warning: "text-warning",
  info: "text-accent-bright",
};

function ToastList() {
  const { toasts } = BaseToast.useToastManager();

  return toasts.map((item) => {
    const tone = (item.type as ToastTone | undefined) ?? "info";
    const Icon = TONE_ICONS[tone] ?? Info;

    return (
      <BaseToast.Root
        key={item.id}
        toast={item}
        className={cn(
          "border-border-default bg-surface-elevated shadow-popup absolute right-0 bottom-0 w-88 rounded-lg border p-3",
          "flex items-center gap-2.5",
          "transition-[opacity,transform] duration-250 ease-out",
          // Base UI publishes stack geometry as CSS variables; the newest
          // toast sits at index 0 and older ones slide down behind it.
          "transform-[translateY(calc(var(--toast-offset-y)*-1))]",
          "data-starting-style:translate-y-4 data-starting-style:opacity-0",
          "data-ending-style:translate-y-4 data-ending-style:opacity-0",
          "data-expanded:transform-[translateY(calc(var(--toast-offset-y)*-1))]",
        )}
      >
        <Icon className={cn("mt-0.5 size-4 shrink-0", TONE_COLORS[tone])} />

        <div className="min-w-0 flex-1">
          <BaseToast.Title className="text-sm font-medium" />
          <BaseToast.Description className="text-text-muted mt-0.5 text-xs" />
          {item.actionProps ? (
            <BaseToast.Action className="text-accent-bright mt-2 text-xs font-medium hover:underline" />
          ) : null}
        </div>

        <BaseToast.Close
          render={
            <IconButton label="Dismiss" size="sm" className="-mt-1 -mr-1">
              <X />
            </IconButton>
          }
        />
      </BaseToast.Root>
    );
  });
}

export function ToastProvider({ children }: { children: ReactNode }) {
  return (
    <BaseToast.Provider toastManager={manager} limit={4}>
      {children}
      <BaseToast.Portal>
        <BaseToast.Viewport className="fixed right-4 bottom-4 z-100 flex w-[min(22rem,calc(100vw-2rem))] outline-none">
          <ToastList />
        </BaseToast.Viewport>
      </BaseToast.Portal>
    </BaseToast.Provider>
  );
}
