import { useEffect, useRef, useState } from "react";

/**
 * A pixel phoenix — ققنوس — perched on the ledge above the fact strip on the
 * About screen.
 *
 * It is small on purpose: a 24x16 grid at 2x, which is about a third the area
 * of a favicon. At that size a bird is three shapes and nothing else, so the
 * three it gets are the ones that say phoenix and not sparrow — a crest, two
 * long trailing tail streamers, and a body that runs ember at the back to gold
 * at the breast. Everything else was cut.
 *
 * It hops, sheds the odd ember, and crosses the strip. Click it and it goes up
 * in a flare of white heat and a burst of embers, then settles back onto the
 * same perch.
 *
 * Cost matters more than it looks like it should here: this is a decoration on
 * a screen nobody stares at, so it must not cost a frame anywhere else.
 *
 * 1. The ambient routine — perch, hop, fly right, perch, fly back — is one CSS
 *    timeline shared by every moving part, so React renders the sprite once and
 *    then never touches it again. The only state in the file is the click
 *    flight: one boolean and one class.
 * 2. Only `transform` and `opacity` are animated, so every step runs on the
 *    compositor and never invalidates layout or paint.
 * 3. The sprites are pixel grids, and adjacent pixels of the same colour are
 *    merged into one <rect> at module load (see `toRects`) — built once per
 *    process rather than per render.
 *
 * Colours are CSS variables rather than literals so the fire can be deepened on
 * the light theme, where a gold bright enough to glow against #141417 is
 * invisible on a white card. See phoenix.css.
 */

const PALETTE: Record<string, string> = {
  o: "var(--phx-o)", // outline — a dark ember, not black
  r: "var(--phx-r)", // deep ember: the back, the rump, the folded wing
  f: "var(--phx-f)", // flame
  a: "var(--phx-a)", // amber
  y: "var(--phx-y)", // gold — the face, the breast, the feather tips
  h: "var(--phx-h)", // white-hot: the glint in the eye
  e: "var(--phx-o)", // the eye, the same ink as the outline
  H: "var(--phx-flare)", // the flare silhouette, on click only
};

type PixelRect = { x: number; y: number; w: number; fill: string };

/**
 * Expand a pixel grid into rects, merging horizontal runs of one colour. Dots
 * are transparent. Called at module scope only — never inside a render.
 */
function toRects(rows: string[]): PixelRect[] {
  const rects: PixelRect[] = [];

  rows.forEach((row, y) => {
    let x = 0;
    while (x < row.length) {
      const ch = row[x];
      if (ch === undefined || ch === ".") {
        x += 1;
        continue;
      }
      let run = 1;
      while (row[x + run] === ch) run += 1;
      rects.push({ x, y, w: run, fill: PALETTE[ch] ?? "transparent" });
      x += run;
    }
  });

  return rects;
}

/**
 * Every grid is drawn on the same 24x16 canvas rather than as a tight sub-grid
 * with an offset. It costs nothing — blank cells produce no rects — and it
 * means the layers can be read against each other by eye, which is the only
 * way pixel art this small gets checked.
 */

/** One step down the fire, for the wing on the far side of the body. */
const COOLER: Record<string, string> = { y: "a", a: "f", f: "r", r: "r" };
const dim = (rows: string[]): string[] =>
  rows.map((row) => row.replace(/[yafr]/g, (ch) => COOLER[ch] ?? ch));

/** Every lit pixel at once, for the flash it goes up in. */
const flare = (rows: string[]): string[] =>
  rows.map((row) => row.replace(/[^.]/g, "H"));

/**
 * Head and body, facing right. The eye is at (17,4) with its white-hot glint
 * beside it; that one pixel is most of why the bird reads as alive rather than
 * as a shape.
 */
const BODY_ROWS = [
  "........................",
  "........................",
  "...............ooooo....",
  "..............oyyyyyo...",
  "..............oyyehyoff.",
  "..............oyyyyyofo.",
  "..............oyyyyyo...",
  ".............oayyyyao...",
  "...........offaaaaayo...",
  "..........offfaaaaayo...",
  "..........offfaaaayyo...",
  "..........orrffaaayyo...",
  "...........orrfffayo....",
  "............ooooooo.....",
];

/**
 * Two plumes off the crown. Drawn *under* the body so the head keeps its own
 * outline and the plumes read as rising out from behind it.
 */
const CREST_ROWS = [
  "...........yy..y........",
  "...........yya.af.......",
  "............yaf.........",
];

/**
 * The two tail streamers, which are the whole reason this reads as a phoenix
 * and not as a small orange bird. Both run dark at the rump and gold at the
 * tip, so they look like they are burning away from the body.
 */
const TAIL_ROWS = [
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  ".yya....................",
  "..yaf...................",
  "...aff..................",
  "....affr................",
  "......ffrr..............",
  "........frrr............",
  "........................",
  ".......afrrr............",
  ".....yaff...............",
  "...yyaf.................",
];

/**
 * The wing at rest: a deep-ember mass on the flank with only a trailing edge
 * outlined. A fully outlined wing at this size puts two dark lines within a
 * pixel of each other and the whole flank turns to mud.
 */
const WING_FOLDED_ROWS = [
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  ".............frrr.......",
  "............frrrr.......",
  "............frrro.......",
  ".............oooo.......",
];

/**
 * The two halves of a wingbeat, pivoting on the shoulder at (14,9). Swapped by
 * a hard opacity cut rather than a rotation: rotated pixel art stops being
 * pixel art.
 */
const WING_UP_ROWS = [
  "........................",
  "........................",
  "........................",
  "..yya...................",
  "..oaffo.................",
  "...oaffao...............",
  "....oaffaao.............",
  ".....oaffaaao...........",
  "......ooaffaaao.........",
  "........oooooooo........",
];

const WING_DN_ROWS = [
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........oooooooo........",
  "......ooaffaaao.........",
  ".....oaffaaao...........",
  "....oaffaao.............",
  "...oaffao...............",
  "..oaffo.................",
  "..yya...................",
];

const LEGS_ROWS = [
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  ".............a...a......",
  "............aaa.aaa.....",
];

/** Talons drawn up under the belly — at this size, the clearest tell that it
    has left the ledge. */
const FEET_TUCKED_ROWS = [
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "........................",
  "..............aa........",
  ".............aa.........",
];

const BODY = toRects(BODY_ROWS);
const CREST = toRects(CREST_ROWS);
const TAIL = toRects(TAIL_ROWS);
const WING_FOLDED = toRects(WING_FOLDED_ROWS);
const WING_UP = toRects(WING_UP_ROWS);
const WING_DN = toRects(WING_DN_ROWS);
const WING_UP_FAR = toRects(dim(WING_UP_ROWS));
const WING_DN_FAR = toRects(dim(WING_DN_ROWS));
const LEGS = toRects(LEGS_ROWS);
const FEET_TUCKED = toRects(FEET_TUCKED_ROWS);

/**
 * The flash, which is every part at once — both wingbeats included, since the
 * wings are already open by the time it fires and the union of the two strokes
 * is a wider burst than either. Drawn last, so it covers the bird rather than
 * leaving parts of it showing through at their normal colour.
 *
 * The wings are split out because they have to be laid down twice: once in
 * place, and once at the far wing's offset. Miss the second and the far wing's
 * two-pixel rim stays cold while everything inside it is white.
 */
const FLARE_WINGS = [
  ...toRects(flare(WING_UP_ROWS)),
  ...toRects(flare(WING_DN_ROWS)),
];

const FLARE_CORE = [
  ...toRects(flare(TAIL_ROWS)),
  ...toRects(flare(CREST_ROWS)),
  ...toRects(flare(BODY_ROWS)),
  ...toRects(flare(FEET_TUCKED_ROWS)),
];

const EMBER = toRects(["y"]);

/** The embers it sheds standing still — slow, and only two of them. */
const IDLE_EMBERS = [
  { x: 12, y: 11, delay: "0s" },
  { x: 16, y: 12, delay: "3.4s" },
];

/** The burst it leaves behind on take-off. Drift is in sprite units. */
const BURST_EMBERS = [
  { x: 11, y: 11, dx: "-6px", dy: "-5px", delay: "0ms" },
  { x: 15, y: 12, dx: "5px", dy: "-4px", delay: "40ms" },
  { x: 13, y: 13, dx: "-2px", dy: "4px", delay: "90ms" },
  { x: 17, y: 10, dx: "6px", dy: "2px", delay: "140ms" },
  { x: 9, y: 12, dx: "-7px", dy: "1px", delay: "190ms" },
];

function Pixels({ rects }: { rects: PixelRect[] }) {
  return (
    <>
      {rects.map((r) => (
        <rect
          key={`${r.x}-${r.y}`}
          x={r.x}
          y={r.y}
          width={r.w}
          height={1}
          fill={r.fill}
        />
      ))}
    </>
  );
}

/** One wing: both strokes stacked, swapped by an opacity flip in CSS. */
function Wing({ up, down }: { up: PixelRect[]; down: PixelRect[] }) {
  return (
    <>
      <g className="phx-flap phx-flap--up">
        <Pixels rects={up} />
      </g>
      <g className="phx-flap phx-flap--dn">
        <Pixels rects={down} />
      </g>
    </>
  );
}

export function PixelPhoenix() {
  const [rising, setRising] = useState(false);
  const settleTimer = useRef<number | undefined>(undefined);

  useEffect(() => () => window.clearTimeout(settleTimer.current), []);

  /**
   * Click: leave the ambient timeline exactly where it is and play the rise
   * over the top of it. Pausing rather than cancelling is what lets the bird
   * pick its patrol back up mid-stride — see `.is-rising` in phoenix.css.
   */
  const rise = () => {
    if (rising) return;
    // Reduced motion suppresses the rise animation entirely, which means there
    // is no animationend to land on. Don't take off at all.
    if (window.matchMedia("(prefers-reduced-motion: reduce)").matches) return;

    setRising(true);
    // Belt and braces: if the animationend is ever missed — the tab is
    // backgrounded mid-flight, say — it would hang in the air forever.
    settleTimer.current = window.setTimeout(() => setRising(false), 1800);
  };

  const settle = (event: React.AnimationEvent<HTMLDivElement>) => {
    // Every animation under the wrapper bubbles through here; only the rise
    // itself ends the flight.
    if (event.target !== event.currentTarget) return;
    window.clearTimeout(settleTimer.current);
    setRising(false);
  };

  return (
    <div
      className={["phx-stage", rising && "is-rising"].filter(Boolean).join(" ")}
    >
      <div className="phx-walker">
        <div className="phx-arc">
          <button
            type="button"
            className="phx-hit"
            onClick={rise}
            aria-label="Make the phoenix rise"
          >
            <div className="phx-facing">
              {/* The burst stays behind at the perch instead of riding along,
                  so it sits outside the rise wrapper. The positioning translate
                  and the drift have to be on different elements: a CSS
                  transform replaces an SVG transform attribute outright rather
                  than composing with it. */}
              <svg
                className="phx-dust"
                viewBox="-1 -2 26 19"
                focusable="false"
                aria-hidden="true"
              >
                {BURST_EMBERS.map((e) => (
                  <g
                    key={`${e.x}-${e.y}`}
                    transform={`translate(${e.x} ${e.y})`}
                  >
                    <g
                      className="phx-spark"
                      style={
                        {
                          "--spark-x": e.dx,
                          "--spark-y": e.dy,
                          animationDelay: e.delay,
                        } as React.CSSProperties
                      }
                    >
                      <Pixels rects={EMBER} />
                    </g>
                  </g>
                ))}
              </svg>

              <div className="phx-rise" onAnimationEnd={settle}>
                <div className="phx-hop">
                  <svg
                    className="phx-sprite"
                    viewBox="-1 -2 26 19"
                    width="52"
                    height="38"
                    shapeRendering="crispEdges"
                    focusable="false"
                    aria-hidden="true"
                  >
                    {/* Far wing first — the only part that belongs behind the
                        body. Offset back and up so the two wings read as a pair
                        rather than as one thick one. */}
                    <g className="phx-pose phx-pose--air">
                      <g className="phx-wing--far">
                        <Wing up={WING_UP_FAR} down={WING_DN_FAR} />
                      </g>
                    </g>

                    <g className="phx-tail">
                      <Pixels rects={TAIL} />
                    </g>

                    {/* Under the body, so the crown keeps its outline. */}
                    <g className="phx-crest">
                      <Pixels rects={CREST} />
                    </g>

                    <g className="phx-breath">
                      <Pixels rects={BODY} />
                      {/* An eyelid the colour of the face: shut for a frame or
                          two, rarely. */}
                      <rect
                        className="phx-blink"
                        x={17}
                        y={4}
                        width={2}
                        height={1}
                        fill={PALETTE.y}
                      />
                    </g>

                    <g className="phx-pose phx-pose--perch">
                      <Pixels rects={LEGS} />
                      <Pixels rects={WING_FOLDED} />
                    </g>

                    <g className="phx-pose phx-pose--air">
                      <Pixels rects={FEET_TUCKED} />
                      <Wing up={WING_UP} down={WING_DN} />
                    </g>

                    {/* Two embers drifting off it while it stands there. */}
                    {IDLE_EMBERS.map((e) => (
                      <g
                        key={`${e.x}-${e.y}`}
                        transform={`translate(${e.x} ${e.y})`}
                      >
                        <g
                          className="phx-ember"
                          style={{ animationDelay: e.delay }}
                        >
                          <Pixels rects={EMBER} />
                        </g>
                      </g>
                    ))}

                    {/* The whole bird at white heat, for the instant it goes. */}
                    <g className="phx-flare">
                      <g className="phx-wing--far">
                        <Pixels rects={FLARE_WINGS} />
                      </g>
                      <Pixels rects={FLARE_WINGS} />
                      <Pixels rects={FLARE_CORE} />
                    </g>
                  </svg>
                </div>
              </div>
            </div>
          </button>
        </div>
      </div>
    </div>
  );
}
