# Headless font source editing today

Date: 2026-10-19. Agent research pass. How sources get edited with no
GUI editor, and what breaks for coding agents. [unverified] marks
items not confirmed from a primary source.

## Google Fonts toolchain

Build-and-fix, not source-edit. `gftools-builder` reads `config.yaml`
and runs fontmake or fontc, then `gftools fix-*` on the binaries.
Sources are not rewritten by the builder. The gftools commands that
do rewrite UFOs are defcon helpers: `ufo-setter`, `ufo-set-order`,
`ufo-transfer-data`, `ufo-fix-instances`, `ufo-merge`. fontbakery
only reads. fontc only compiles.

Real headless source edits in the wild:

- googlefonts/googlesans-flex `scripts/internal/normalize.py`: loads
  the designspace with ufoLib2, strips non-public lib keys, expands
  bracket kerning, rewrites every source.
- arrowtype/recursive `src/01-shell-scripts-for-sources/`: fontParts
  scripts for unicodes, suffix renames, export flags, decompose,
  kerning remaps, then `normalizeUFO`.
- arrowtype/shantell-sans `scripts--build/prep-build.py`: glyphsLib
  `build_masters`, fontParts edits, `ufonormalizer` at the end.

## Libraries

| Library | Addressing | Write granularity | Canonical |
|---|---|---|---|
| fontTools ufoLib/glifLib | glyphset per layer dir, glyph by name | single `.glif` | stable order, differs from ufonormalizer |
| defcon / fontParts | `font[name]`, `glyph[ci][pi]`, optional `identifier` | `Layer.save` writes only dirty glyphs in place | no |
| ufoLib2 | same model | rewrites every loaded glyph, deletes missing | no |
| glyphsLib | `glyphs["a"].layers[masterId]`, masters by UUID | whole file | sorts keys |
| norad (Rust) | glyph by name, layers by name | `Font::save` removes the directory then rewrites the whole UFO | own serializer, differs from ufonormalizer (norad #189) |
| babelfont-rs | unified model | whole font; UFO write in progress | none stated |
| Fontra backend | `getGlyph(name)` / `putGlyph(name, glyph)` | per glyph for `.fontra`; designspace backend writes every UFO for info and features | `indent=0`, insertion order |
| Glyphs remote | `glyphs-cli` (2026-03) runs Python without the app; edits cannot be saved yet | n/a | app writes whole file |
| RoboFont remote | `roboFont -p script.py` inside the running app | defcon rules | no |

## Agents editing sources: the evidence

- Glyphs forum, 2025: "AI fails miserably with vector data"; LLM SVG
  output "nothing usable"; models hallucinate Glyphs API methods;
  warnings against letting AI edit live files directly.
- Both Glyphs MCP servers are live-editor only, never raw files.
  thierryc/Glyphs-mcp 2.1.0: Keep/Save/Restore, snapshot recovery,
  git checkpoints of authorized saves.
- Other: LHxis/fontforge-mcp (every op can render a PNG proof sheet),
  Dithilli/fontlab-mcp-server, shift-editor/shift (issue 122:
  per-master kerning, fontinfo, lib and undeclared layers lost at
  read time).
- Agent-facing repo rules: twardoch/brex-font CLAUDE.md "do not make
  design changes unless explicitly asked"; tmke8/math-core-fonts
  "no glyph shape changes", edits live in `patches.py` applied at
  build so diffs are readable; typedev/DSSketch (compact designspace
  text format) whose agent-written audit found multi-conditionset
  rules silently corrupted.
- Fontra issue 17: a user generating `.glif` with norad had Fontra
  read a half-written UFO because norad deletes the folder first.
- Compiled-font edits: Jon Shamir used Claude Code to tighten
  punctuation sidebearings in a compiled VF with no sources.
- No GitHub issue or PR in google/fonts, glyphsLib, fontmake, or
  ufoLib2 reporting an agent-authored source edit gone wrong was
  found. Point-level design edits by agents are essentially absent
  from the public record.

## Seeing results headlessly

`hb-view font.ttf "text" -o out.png` after a compile.
`fontTools.pens.freetypePen` renders a glyph from any pen source
without compiling. `svgPathPen` gives SVG from a UFO glyph. DrawBot
scripts. `diffenator2` for binaries. MCP-native rendering in
fontforge-mcp (PNG), glyphs-mcp (SVG), Runebender (`proof`,
`graph_image`). Nothing renders a UFO glyph to PNG in one CLI command
without a script or a compile.

## Concurrent editing

- Fontra: `watchfiles` per backend; self-writes suppressed by recorded
  mtime. An external change reloads only data the server holds and
  broadcasts `externalChange`; pending client edits are not merged,
  server data is replaced. Issue 132: a self-save misread as external
  cleared the undo stack. Write bugs filed 2026-10-01: kerning save
  deletes non-kern groups (#2790), font info overwrites per-master
  values in every UFO (#2791), features written into every UFO with
  includes inlined, intentional per Just (#2792).
- Glyphs: dialog "Reload" / "Keep Glyphs Version"; users report Reload
  discarding unsaved work.
- RoboFont: polls while active; "Update found" dialog; External
  Changes window accepts per glyph with a `.glif` XML diff popover.
  defcon underneath: unsaved in-memory objects win.
  https://www.robofont.com/documentation/reference/workspace/external-changes-window/
- runebender-web: ETag is sha256 of content; PUT requires `If-Match`,
  409 on mismatch; self-write suppression copied from Fontra. Stale
  writer loses; no merge.

## Headless edit operations by observed frequency

1. Font info and metrics across all masters.
2. Lib and groups scrubbing, normalization.
3. Glyph order, unicode assignment, renames, export flags.
4. Kerning and group transfers.
5. Feature code generation, include flattening.
6. Component decomposition, alt generation by interpolation.
7. Designspace instance and axis fixes.
8. Outline geometry: rare, mostly sidebearings and translate.

## Three biggest agent failure modes

1. Whole-font rewrite side effects: writing one thing rewrites
   everything. Diff noise hides the real edit; half-written states
   break watchers.
2. Lossy read-modify-write: libraries drop what they do not model,
   and the agent cannot see the loss.
3. No cheap feedback loop on geometry: verification needs compile
   plus render, so agents skip it or claim success.

## What a format and CLI must provide

- One file per glyph, written atomically (temp file plus rename),
  never a directory wipe; a per-file content hash the CLI accepts as
  a precondition, 409 on mismatch.
- A canonical serializer in the spec; `fmt` and `check` commands.
- Stable IDs for points, contours, anchors, sources.
- Partial-write CLI verbs touching only the named object, with
  `--dry-run` and a receipt.
- Lossless preservation of unknown keys, with a loss report on
  import and export.
- One-command render to PNG or SVG with no compile step, plus a
  before/after image diff.
- A watch and notify protocol with self-write suppression by content
  hash, so an open editor and an agent share the package without a
  reload dialog.
