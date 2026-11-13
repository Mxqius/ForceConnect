import { useQuery } from "@tanstack/react-query";
import { openUrl } from "@tauri-apps/plugin-opener";
import { Download, ExternalLink, Sparkles } from "lucide-react";

import { AlertDialog, Button, Progress } from "@/design/primitives";
import { summariseNotes } from "@/features/settings/releaseNotes";
import {
  useUpdaterStatus,
  useUpdaterStore,
} from "@/features/settings/useUpdater";
import { ipc } from "@/lib/ipc";
import { queryKeys } from "@/lib/query-keys";

const REPO = "https://github.com/mxqius/forceconnect";

/**
 * Tells the user a new version exists, when the app found it on its own.
 *
 * It asks rather than acts: the update replaces the running binary of an
 * app that runs elevated, so the install is the user's click every time. The
 * three answers are distinct on purpose — "Later" is until the next launch,
 * "Skip this version" is for good, and neither disables checking.
 *
 * Not shown for a build that cannot verify updates (see `useUpdaterStatus`):
 * offering an install that is certain to fail at the last step is worse than
 * saying nothing, and Settings already explains that case.
 */
export function UpdatePrompt() {
  const stage = useUpdaterStore((state) => state.stage);
  const pending = useUpdaterStore((state) => state.pending);
  const promptVersion = useUpdaterStore((state) => state.promptVersion);
  const install = useUpdaterStore((state) => state.install);
  const dismiss = useUpdaterStore((state) => state.dismissPrompt);
  const skip = useUpdaterStore((state) => state.skipVersion);

  const { data: status } = useUpdaterStatus();
  const { data: info } = useQuery({
    queryKey: queryKeys.appInfo,
    queryFn: ipc.appInfo,
  });

  const version = pending?.version ?? promptVersion;
  const open =
    promptVersion !== null &&
    status?.configured === true &&
    (stage.kind === "available" ||
      stage.kind === "downloading" ||
      stage.kind === "installing" ||
      stage.kind === "error");

  const notes = summariseNotes(pending?.body);
  const working = stage.kind === "downloading" || stage.kind === "installing";

  return (
    <AlertDialog
      open={open}
      onOpenChange={(next) => {
        // Escape must not walk away from a download that is in progress.
        if (!next && !working) dismiss();
      }}
      title={
        <span className="flex items-center gap-2.5">
          <span className="bg-accent-surface text-accent-bright flex size-8 shrink-0 items-center justify-center rounded-xl">
            <Sparkles className="size-4" />
          </span>
          ForceConnect {version} is available
        </span>
      }
      description={
        info
          ? `You are on ${info.version}. Installing closes ForceConnect and opens it again when it finishes.`
          : "Installing closes ForceConnect and opens it again when it finishes."
      }
      footer={
        stage.kind === "available" ? (
          <>
            <Button variant="ghost" size="sm" onClick={skip}>
              Skip this version
            </Button>
            <Button size="sm" onClick={dismiss}>
              Later
            </Button>
            <Button
              variant="primary"
              size="sm"
              startIcon={<Download />}
              onClick={() => void install()}
            >
              Update now
            </Button>
          </>
        ) : stage.kind === "error" ? (
          <Button size="sm" onClick={dismiss}>
            Close
          </Button>
        ) : (
          <Button size="sm" disabled>
            {stage.kind === "installing" ? "Installing…" : "Downloading…"}
          </Button>
        )
      }
    >
      <div className="mt-4 flex flex-col gap-3">
        {stage.kind === "error" ? (
          <p className="text-danger text-sm">{stage.message}</p>
        ) : null}

        {stage.kind === "downloading" ? (
          <div className="flex flex-col gap-2">
            <Progress
              label={`Downloading ${version}`}
              value={stage.percent}
              max={100}
            />
            <p className="text-text-muted text-xs">
              Downloading
              {stage.percent === null ? "…" : ` — ${stage.percent}%`}
            </p>
          </div>
        ) : null}

        {stage.kind === "installing" ? (
          <p className="text-text-muted text-sm">
            Installing. ForceConnect will close and reopen on its own.
          </p>
        ) : null}

        {stage.kind === "available" && notes.items.length > 0 ? (
          <ul className="fc-scroll bg-surface-inset flex max-h-48 flex-col gap-2 overflow-y-auto rounded-xl px-3.5 py-3 text-[13px] leading-snug">
            {notes.items.map((item) => (
              <li key={item.lead + item.text} className="flex gap-2">
                <span
                  aria-hidden
                  className="bg-accent-bright mt-[7px] size-1 shrink-0 rounded-full"
                />
                <span className="text-text-secondary">
                  {item.lead ? (
                    <strong className="text-text-primary font-semibold">
                      {item.lead}
                      {item.text ? " — " : ""}
                    </strong>
                  ) : null}
                  {item.text}
                </span>
              </li>
            ))}
            {notes.more > 0 ? (
              <li className="text-text-muted pl-3 text-xs">
                and {notes.more} more in the release notes
              </li>
            ) : null}
          </ul>
        ) : null}

        {stage.kind === "available" ? (
          <button
            type="button"
            onClick={() => void openUrl(`${REPO}/releases/tag/v${version}`)}
            className="text-accent-bright focus-visible:outline-accent flex w-fit items-center gap-1.5 text-xs font-medium outline-none hover:underline focus-visible:outline-2"
          >
            Full release notes
            <ExternalLink className="size-3" />
          </button>
        ) : null}
      </div>
    </AlertDialog>
  );
}
