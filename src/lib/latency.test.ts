import { describe, expect, it } from "vitest";

import {
  LATENCY_FAIR_MS,
  LATENCY_GOOD_MS,
  classifyLatency,
  formatLatency,
} from "@/lib/latency";

describe("classifyLatency", () => {
  it("colours by the 1.x thresholds, inclusive at each boundary", () => {
    expect(classifyLatency(0)).toBe("success");
    expect(classifyLatency(LATENCY_GOOD_MS)).toBe("success");
    expect(classifyLatency(LATENCY_GOOD_MS + 1)).toBe("warning");
    expect(classifyLatency(LATENCY_FAIR_MS)).toBe("warning");
    expect(classifyLatency(LATENCY_FAIR_MS + 1)).toBe("danger");
  });

  it("treats unreachable as neutral rather than slow", () => {
    // A server that never answered is a different problem from a slow one,
    // and colouring it red would tell the user to look in the wrong place.
    expect(classifyLatency(null)).toBe("neutral");
    expect(classifyLatency(undefined)).toBe("neutral");
    expect(classifyLatency(-1)).toBe("neutral");
    expect(classifyLatency(Number.NaN)).toBe("neutral");
    expect(classifyLatency(Number.POSITIVE_INFINITY)).toBe("neutral");
  });
});

describe("formatLatency", () => {
  it("rounds to whole milliseconds", () => {
    expect(formatLatency(12.4)).toBe("12 ms");
    expect(formatLatency(12.6)).toBe("13 ms");
  });

  it("does not render a sub-millisecond result as 0 ms", () => {
    expect(formatLatency(0.4)).toBe("<1 ms");
    expect(formatLatency(0)).toBe("<1 ms");
  });

  it("renders unreachable as an em dash", () => {
    expect(formatLatency(null)).toBe("—");
    expect(formatLatency(-1)).toBe("—");
  });
});
