import { describe, expect, it } from "vitest";

import { summariseNotes } from "@/features/settings/releaseNotes";

const NOTES = `## ForceConnect 2.2 — encrypted DNS that actually connects

Intro paragraph.

> Already on 2.1.2? Install from inside the app.

### ✨ What's new

- **Encrypted DNS, for real** — pick DoH and connect. See [docs](https://example.com).
- **Backup server** — choose one in **Settings**.
- Plain bullet with \`code\`

### 🛠 Fixes

- DNSSEC answers are passed through.

### 📥 Download

| Platform | File |
| --- | --- |

### ⚠️ Good to know

- Windows SmartScreen may warn about the installer.
`;

describe("summariseNotes", () => {
  it("takes the changes and splits the lead-in from the sentence", () => {
    const { items } = summariseNotes(NOTES, 10);

    expect(items[0]).toEqual({
      lead: "Encrypted DNS, for real",
      text: "pick DoH and connect. See docs.",
    });
    expect(items[1]).toEqual({
      lead: "Backup server",
      text: "choose one in Settings.",
    });
    expect(items[2]).toEqual({ lead: "", text: "Plain bullet with code" });
  });

  it("includes fixes but not the sections about installing", () => {
    const text = summariseNotes(NOTES, 10).items.map((i) => i.text);

    expect(text).toContain("DNSSEC answers are passed through.");
    expect(text.join(" ")).not.toMatch(/SmartScreen/);
  });

  it("caps the list and counts what was left out", () => {
    const summary = summariseNotes(NOTES, 2);

    expect(summary.items).toHaveLength(2);
    expect(summary.more).toBe(2);
  });

  it("returns nothing for missing or bullet-less notes", () => {
    expect(summariseNotes(null)).toEqual({ items: [], more: 0 });
    expect(summariseNotes("Just a sentence.")).toEqual({ items: [], more: 0 });
  });
});
