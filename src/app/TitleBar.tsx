import { isTauri } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Minus, X } from "lucide-react";

import logoIcon from "@/assets/brand/logo-icon.png";
import { cn } from "@/lib/cn";

/**
 * Resolve the window on use rather than at module scope.
 *
 * `getCurrentWindow()` reads Tauri internals off `window`, so calling it while
 * the module loads throws in a plain browser and takes the whole app down with
 * it — which would silently cost the browser dev path the IPC layer goes out of
 * its way to support (see the fixtures branch in `lib/ipc`).
 */
function withWindow(
  action: (win: ReturnType<typeof getCurrentWindow>) => void,
) {
  if (!isTauri()) return;
  action(getCurrentWindow());
}

/**
 * There is deliberately no maximize control.
 *
 * The window is a fixed-size utility panel — every screen is laid out for that
 * one size, and stretching it to the full display would leave a field of empty
 * surface rather than showing more. `maximizable: false` in the window config
 * closes the other routes to the same state (the system menu, and the
 * double-click a drag region would otherwise handle).
 */
export function TitleBar() {
  return (
    <header
      data-tauri-drag-region
      // No fill and no rule: in the bento layout the title bar is part of the
      // ground the tiles sit on, and a band across the top would be a tile
      // that holds nothing.
      className="flex h-10 shrink-0 items-center select-none"
    >
      <div
        data-tauri-drag-region
        className="text-text-muted flex flex-1 items-center gap-2.5 self-stretch px-5"
      >
        <img src={logoIcon} alt="" className="size-[18px] shrink-0" />
        <span className="text-[13px] font-medium">ForceConnect</span>
      </div>

      <div className="flex h-full shrink-0 items-stretch">
        <WindowButton
          label="Minimize"
          onClick={() => withWindow((win) => void win.minimize())}
        >
          <Minus className="size-4" />
        </WindowButton>
        <WindowButton
          label="Close"
          danger
          onClick={() => withWindow((win) => void win.close())}
        >
          <X className="size-4" />
        </WindowButton>
      </div>
    </header>
  );
}

interface WindowButtonProps {
  label: string;
  danger?: boolean;
  onClick: () => void;
  children: React.ReactNode;
}

function WindowButton({ label, danger, onClick, children }: WindowButtonProps) {
  return (
    <button
      type="button"
      aria-label={label}
      onClick={onClick}
      className={cn(
        "text-text-muted inline-flex w-12 items-center justify-center transition-colors",
        danger
          ? "hover:bg-danger hover:text-white"
          : "hover:bg-surface-hover hover:text-text-primary",
      )}
    >
      {children}
    </button>
  );
}
