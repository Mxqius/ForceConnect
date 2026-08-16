import { useRef, useState, type KeyboardEvent } from "react";

import { cn } from "@/lib/cn";
import { controlBase } from "@/design/styles";
import { Chip } from "@/design/primitives/Badge";

interface ChipInputProps {
  value: string[];
  onChange: (value: string[]) => void;
  placeholder?: string;
  /** Rejects further entries once reached. */
  max?: number;
  className?: string;
  id?: string;
  "aria-label"?: string;
}

/**
 * Free-text tag entry.
 *
 * Built on a plain input rather than Base UI's Combobox because there is no
 * fixed option set to filter — tags are whatever the user types. Commit with
 * Enter or comma; Backspace on an empty field removes the last chip.
 */
export function ChipInput({
  value,
  onChange,
  placeholder = "Add a tag…",
  max,
  className,
  id,
  "aria-label": ariaLabel,
}: ChipInputProps) {
  const [draft, setDraft] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);

  const atCapacity = max !== undefined && value.length >= max;

  function commit(raw: string) {
    const tag = raw.trim().toLowerCase();
    if (!tag || atCapacity) return;
    // Silently ignoring a duplicate beats an error for something the user can
    // see is already there.
    if (!value.includes(tag)) {
      onChange([...value, tag]);
    }
    setDraft("");
  }

  function handleKeyDown(event: KeyboardEvent<HTMLInputElement>) {
    if (event.key === "Enter" || event.key === ",") {
      // Enter would otherwise submit the surrounding form mid-entry.
      event.preventDefault();
      commit(draft);
      return;
    }

    if (event.key === "Backspace" && draft === "" && value.length > 0) {
      onChange(value.slice(0, -1));
    }
  }

  return (
    <div
      className={cn(
        controlBase,
        "flex min-h-11 cursor-text flex-wrap items-center gap-1.5 px-2.5 py-2",
        "focus-within:border-accent",
        className,
      )}
      onClick={() => inputRef.current?.focus()}
    >
      {value.map((tag) => (
        <Chip
          key={tag}
          removeLabel={`Remove tag ${tag}`}
          onRemove={() => onChange(value.filter((t) => t !== tag))}
        >
          {tag}
        </Chip>
      ))}

      <input
        ref={inputRef}
        id={id}
        aria-label={ariaLabel ?? "Add a tag"}
        value={draft}
        disabled={atCapacity}
        placeholder={value.length === 0 ? placeholder : undefined}
        onChange={(event) => setDraft(event.target.value)}
        onKeyDown={handleKeyDown}
        // Committing on blur avoids the trap where a typed but uncommitted
        // tag is silently discarded when the user clicks Save.
        onBlur={() => commit(draft)}
        className="placeholder:text-text-muted min-w-24 flex-1 bg-transparent text-sm outline-none disabled:cursor-not-allowed"
      />
    </div>
  );
}
