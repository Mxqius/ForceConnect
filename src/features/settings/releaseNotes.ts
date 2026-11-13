export interface NoteItem {
  /** The bolded lead-in of a bullet, e.g. "Backup server"; empty if it has none. */
  lead: string;
  text: string;
}

export interface NotesSummary {
  items: NoteItem[];
  /** Bullets that did not fit, so the popup can say there is more. */
  more: number;
}

/** Sections of a release description that are not changes. */
const NOT_CHANGES = /download|good to know|feedback|requirements/i;

function plain(text: string): string {
  return text
    .replace(/\[([^\]]+)\]\([^)]*\)/g, "$1")
    .replace(/\*\*|`/g, "")
    .replace(/\s+/g, " ")
    .trim();
}

/**
 * The few lines worth showing in a popup, from a release's markdown.
 *
 * The description is written for the GitHub release page — headings, a
 * download table, caveats — and a dialog has room for none of that. What it
 * does have room for is the list of changes, so this takes the bullets and
 * leaves out the sections that are about installing rather than about what
 * changed. Anything unrecognised produces an empty list, and the popup falls
 * back to a link to the full notes rather than showing raw markdown.
 */
export function summariseNotes(
  notes: string | null | undefined,
  max = 5,
): NotesSummary {
  const all: NoteItem[] = [];
  let skipping = false;

  for (const raw of (notes ?? "").split(/\r?\n/)) {
    const line = raw.trim();

    const heading = /^#{1,6}\s+(.*)$/.exec(line);
    if (heading) {
      skipping = NOT_CHANGES.test(heading[1]!);
      continue;
    }
    if (skipping) continue;

    const bullet = /^[-*]\s+(.*)$/.exec(line);
    if (!bullet) continue;

    const body = bullet[1]!;
    const split = body.indexOf(" — ");
    all.push(
      split === -1
        ? { lead: "", text: plain(body) }
        : {
            lead: plain(body.slice(0, split)),
            text: plain(body.slice(split + 3)),
          },
    );
  }

  return { items: all.slice(0, max), more: Math.max(0, all.length - max) };
}
