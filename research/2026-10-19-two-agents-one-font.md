# Two agents, one font: headless editing and the live editor

Date: 2026-10-19. Answers the question behind decision 2 (one file per
glyph): what layout lets a coding agent work on a font with no editor
running, and an in-editor agent work on the live document, without
the two fighting.

## 1. Who touches the font

Three actors, one source tree:

- The designer in Runebender, with unsaved edits in memory.
- The in-editor agent, through `runebender mcp --live`, on the same
  unsaved document, with receipts and revision preconditions.
- The headless agent (Claude Code, Codex, a script, CI), through
  files, a CLI, and git. No editor process.

Any layout must let the third actor change glyph `b` on disk while
the first two hold unsaved changes to glyph `a`, and have that be
boring.

## 2. What Runebender does today (checked in source, 2026-10-19)

- In memory, the model is already glyph-has-sources: `VariableGlyph`
  holds every layer of a glyph across sources, and geometry is a
  babelfont `GlyphList`. Runebender depends on babelfont-rs directly.
- Point, contour, and component IDs (`EntityId`) are a session
  counter. They are never written to disk and differ every launch.
  The MCP recipe contract hands agents these IDs, so a point named in
  one session means nothing in the next, and the live agent and the
  headless agent cannot name the same point.
- Save rewrites every UFO in full: stage a complete copy, swap it in.
  There is no per-glyph write.
- A file watcher fingerprints the whole source tree. On a change with
  no unsaved edits, the whole project reloads. With unsaved edits, it
  stops: "sources changed on disk; Save As preserves your edits, or
  Revert to Saved accepts disk changes." Conflict granularity is the
  whole font.
- A headless MCP mode exists (`runebender mcp` without `--live`). It
  reads files and offers `font_info`, `read_glyph`, `analyze_glyph`,
  `proof`, `propose`, `propose_edits`, `graph_run`, and proposal
  tools. No tool writes the font; proposals are UFO layers the
  designer installs in the editor.

So the headless path already exists for reading, analysis, proofs,
and proposals. What is missing is headless writing with stable names
for things, and per-glyph conflict handling in the editor. Both are
format problems before they are editor problems.

## 3. What each actor needs from the layout

Headless agent:

1. `ls glyphs/` lists what exists; the glyph name maps to its file by
   a reversible rule, no index file to consult.
2. One glyph, every source, in a few hundred lines. One read.
3. A unique text anchor for every editable value, so a string-replace
   edit matches exactly once, or a CLI that applies a named operation
   and writes canonical bytes.
4. Sight: render glyphs and text to PNG from files. Analysis as
   numbers. Both exist in Runebender's headless MCP today.
5. Validate and build from the CLI.
6. Small, canonical files so git diffs and merges stay legible.

Live editor and its agent:

1. IDs on disk equal IDs in memory, so a receipt from the live agent
   and a diff from the headless agent name the same point.
2. Save writes only the glyph files that changed.
3. Reload only the glyph files that changed on disk, and only when the
   editor has no unsaved edits to that glyph.
4. On a true conflict (both changed glyph `a`), merge by ID at the row
   level, and show the designer only the rows both sides touched.

Every item on both lists is satisfied by: the glyph as the unit of a
file, stable IDs inside the file, and a canonical row-oriented form.
None is satisfied by one file per font. One file per master (UFO)
satisfies the headless list only partly (item 2 fails) and the editor
list only with a whole-UFO rewrite on every save, which is what blocks
item 3 today.

## 4. Granularity options, scored

| | File per font | Dir per master (UFO) | File per glyph | Dir per glyph, file per source |
|---|---|---|---|---|
| Read one glyph, all masters | whole font in context | N reads, N dirs | 1 read | N+1 reads, one `cat` |
| Conflict unit in editor | font | master (today: font) | glyph | glyph x source |
| Diff for one point move | 1 hunk in huge file | 1 hunk | 1 row | 1 row |
| Diff for adding a master | every glyph entry | new dir, clean | every glyph file | new files only, clean |
| Two people, two masters, same glyph | conflict | clean | clean if rows apart, driver makes it certain | clean |
| Compatibility visible in one place | yes | no | yes | with `cat` |
| File count, Virtua Grotesk (870 glyphs, 2 masters) | 1 | 1,740 glifs | 870 | 2,610 |

File per glyph and directory per glyph are the two live options. The
difference is decided by the serialization spike, not here: a "wide"
table (one row per point, a column per source) only works in one
file and is the most legible for interpolation; a "long" table (one
row per source per point) splits naturally into one file per source
and adds a master without touching existing rows.

## 5. The setup that serves both modes

1. **One Rust crate owns the format.** Read, write, canonicalize,
   validate, merge. Runebender links it; the `lilufo` CLI wraps it;
   the headless MCP server wraps the CLI. Same bytes from every path.
2. **A babelfont-rs convertor.** Runebender's geometry is already
   babelfont. The existing Fontra convertor is about 1,000 lines, UFO
   about 1,350. A lilufo convertor of that size gives Runebender
   native open and save, and gives every other format import and
   export, with one piece of code.
3. **IDs come from the file.** Runebender's `EntityId` is replaced
   or seeded by the persisted ID on load, and new IDs are minted
   once and written on save. Live receipts and file diffs then agree.
4. **Per-glyph fingerprints in the editor.** Save writes dirty glyph
   files only, each guarded by the fingerprint read at load (the
   runebender-web ETag rule). Reload accepts changed files for glyphs
   with no unsaved edits. A conflict on one glyph is a row-level
   three-way merge by ID; the same code is the git merge driver.
5. **The CLI is the headless agent's hands and eyes.** `show`,
   `edit` (named operations with preconditions, canonical output),
   `check`, `proof` and `specimen` to PNG, `build`, `diff`. The agent
   may still string-replace a row directly; the canonical form makes
   that safe, and `check` catches what the schema cannot.
6. **Routing rule, later.** When the editor has the font open, the
   CLI can hand edits to the live session instead of writing files,
   so there is one truth at a time. Not needed for the first version;
   per-glyph reload makes the file path safe enough.

## 6. What this changes in decision 2

Decision 2 stands, sharpened: the glyph is the unit of a file or a
directory; the spike decides which. The reason is no longer only Git.
It is that per-glyph conflict detection is the only way a headless
agent and a live editor can share a font without a modal "Save As or
Revert" on every external write.

## 7. Open items for the spike

- Wide vs long tables: diff size for one-point move, diff for adding a
  master, git merge of two different-master edits to one glyph,
  tokens to read one glyph, agent edit success.
- Proposal storage: today a UFO layer. In lilufo, a source flagged
  `proposal` inside the glyph file, or a sibling `proposals/`
  directory excluded from builds. Decide after the spike shows the
  file shape.
- Whether the headless MCP server and the CLI should be one binary.
