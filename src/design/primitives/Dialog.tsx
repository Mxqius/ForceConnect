import { AlertDialog as BaseAlertDialog } from "@base-ui/react/alert-dialog";
import { Dialog as BaseDialog } from "@base-ui/react/dialog";
import { X } from "lucide-react";
import type { ReactElement, ReactNode } from "react";

import { cn } from "@/lib/cn";
import { IconButton } from "@/design/primitives/Button";

const backdrop = cn(
  "fixed inset-0 z-50 bg-black/60 backdrop-blur-[2px]",
  "transition-opacity duration-150",
  "data-[starting-style]:opacity-0 data-[ending-style]:opacity-0",
);

const popup = cn(
  "fixed top-1/2 left-1/2 z-50 w-[min(32rem,calc(100vw-2rem))] -translate-x-1/2 -translate-y-1/2",
  "rounded-xl border border-border-default bg-surface-elevated shadow-dialog",
  "transition-[opacity,transform] duration-150",
  "data-[starting-style]:scale-[0.98] data-[starting-style]:opacity-0",
  "data-[ending-style]:scale-[0.98] data-[ending-style]:opacity-0",
);

interface DialogProps {
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
  title: ReactNode;
  description?: ReactNode;
  /** Buttons for the footer. Omitted entirely when not provided. */
  footer?: ReactNode;
  /** Must be a single element: Base UI clones it to attach trigger behaviour. */
  trigger?: ReactElement;
  className?: string;
  children?: ReactNode;
}

export function Dialog({
  open,
  onOpenChange,
  title,
  description,
  footer,
  trigger,
  className,
  children,
}: DialogProps) {
  return (
    <BaseDialog.Root open={open} onOpenChange={onOpenChange}>
      {trigger ? <BaseDialog.Trigger render={trigger} /> : null}

      <BaseDialog.Portal>
        <BaseDialog.Backdrop className={backdrop} />
        <BaseDialog.Popup className={cn(popup, className)}>
          <header className="border-border-subtle flex items-start gap-4 border-b px-5 py-4">
            <div className="min-w-0 flex-1">
              <BaseDialog.Title className="text-lg font-semibold tracking-tight">
                {title}
              </BaseDialog.Title>
              {description ? (
                <BaseDialog.Description className="text-text-muted mt-1 text-sm">
                  {description}
                </BaseDialog.Description>
              ) : null}
            </div>

            <BaseDialog.Close
              render={
                <IconButton label="Close" size="sm" className="-mt-1 -mr-1">
                  <X />
                </IconButton>
              }
            />
          </header>

          {children ? <div className="px-5 py-4">{children}</div> : null}

          {footer ? (
            <footer className="border-border-subtle flex items-center justify-end gap-2 border-t px-5 py-3">
              {footer}
            </footer>
          ) : null}
        </BaseDialog.Popup>
      </BaseDialog.Portal>
    </BaseDialog.Root>
  );
}

Dialog.Close = BaseDialog.Close;

interface AlertDialogProps {
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
  title: ReactNode;
  description?: ReactNode;
  /** Must be a single element: Base UI clones it to attach trigger behaviour. */
  trigger?: ReactElement;
  children?: ReactNode;
  footer: ReactNode;
}

/**
 * A dialog that demands an answer.
 *
 * Unlike {@link Dialog} there is no close button and no dismiss-on-backdrop —
 * the user has to pick one of the actions. Reserve it for changes that are
 * disruptive or hard to undo, such as clearing DNS while connected.
 */
export function AlertDialog({
  open,
  onOpenChange,
  title,
  description,
  trigger,
  children,
  footer,
}: AlertDialogProps) {
  return (
    <BaseAlertDialog.Root open={open} onOpenChange={onOpenChange}>
      {trigger ? <BaseAlertDialog.Trigger render={trigger} /> : null}

      <BaseAlertDialog.Portal>
        <BaseAlertDialog.Backdrop className={backdrop} />
        <BaseAlertDialog.Popup className={cn(popup, "w-[min(26rem,90vw)]")}>
          <div className="px-5 py-4">
            <BaseAlertDialog.Title className="text-base font-semibold">
              {title}
            </BaseAlertDialog.Title>
            {description ? (
              <BaseAlertDialog.Description className="text-text-muted mt-1.5 text-sm">
                {description}
              </BaseAlertDialog.Description>
            ) : null}
            {children}
          </div>

          <footer className="border-border-subtle flex items-center justify-end gap-2 border-t px-5 py-3">
            {footer}
          </footer>
        </BaseAlertDialog.Popup>
      </BaseAlertDialog.Portal>
    </BaseAlertDialog.Root>
  );
}

AlertDialog.Close = BaseAlertDialog.Close;
