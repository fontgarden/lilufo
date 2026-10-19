# Synthesis: what a better font source format looks like

Date: 2026-10-18. Draws on the four research notes in this directory.
Nothing here is decided. Candidate decisions are listed at the end
for Eli to accept, reject, or change.

## 1. The gap is real and specific

Every existing format gives up at least one of these:

| Property | Who has it | Who lacks it |
|---|---|---|
| Open, written spec | UFO, Designspace, Glyphs (documented) | Fontra, .rcjk, VFJ, SFD |
| All sources of a glyph in one file | Glyphs, Fontra, VFJ, .rcjk | UFO |
| Glyph-local axes, variable components | Glyphs 4, Fontra, .rcjk, VARC | UFO, Designspace |
| Stable IDs below the glyph | UFO (per glyph only, optional) | everyone else |
| Normative canonical serialization | Glyphs (sorted keys) | everyone else |
| Variable kerning, metrics, features in one model | nobody | everyone |
| Source separated from caches and UI state | nobody fully | everyone |
| Non-outline glyph representations | nobody | everyone |

The first three rows are solved problems with known answers. Rows
four to eight are open in every format. That is where a new format
earns its existence.

## 2. Settled requirements

These come straight from the research and need no debate.

1. **Glyph has sources, on disk.** One file per glyph holding every
   master, intermediate, and conditional source. Four formats chose
   this independently. UFO is alone on the other side and its own
   authors wanted to move (issue 175).
2. **Every number can vary.** Kerning, metrics, anchors, feature
   values: one value is a constant, N values are a variation. Just's
   "kerning as CSV with a column per source" is the proven shape.
3. **Layer meaning is structured data, not a name.** A source has a
   name, a location, a sparse flag, a background-of link. Names are
   derived from data when a tool needs one (Belluz, issue 175).
4. **One source can sit at several locations.** The location belongs
   to the designspace entry, not to the layer (Van Blokland).
5. **Decomposed component transforms** (translate, rotate, scale,
   skew, center) and per-component axis locations, matching VARC.
6. **Derived data is not source.** Outline snapshots of neural
   glyphs, thumbnails, hint hashes, UI state live outside the package
   or in an ignored cache directory. Jupyter, SFD, and Glyphs
   `UIState.plist` are the warnings.
7. **Namespaced extension bags on every object; readers preserve
   what they do not understand.** Date-based format version plus a
   generator string (KiCad).
8. **Binary assets by content hash.** Text pointer files in the
   package, bytes in a content-addressed store (DVC, Xet, Penpot).
9. **A canonical form written into the spec**, including key order,
   number precision, indentation. Pretty-printed, not one line.
10. **The edit protocol is a separate, versioned spec** layered on
    the file format. Preconditions, receipts, dry-run, atomic batches
    belong there. Runebender's `agent_apply` is the prototype.

## 3. The one idea that might be the better way

**Separate topology from geometry.**

Today every format stores, per master, a list of contours, each a
list of points with x, y, type, smooth. Interpolation compatibility
is then a check run afterwards: do all masters have the same contour
count, point count, point types, start points? Compatibility errors
are the single largest class of variable-font build failure.

A compiled variable font does not work that way. `gvar` stores one
outline structure and per-region deltas against point indices. The
structure is shared; only coordinates vary.

A source format can do the same:

```
glyph a
  topology            declared once
    contour c1: points p1 (curve, smooth) p2 (offcurve) p3 ... 
    contour c2: ...
    component k1: base o, transform
    anchor top
  geometry            one table per source
    regular:  p1 120 0   p2 80 0   p3 ...   k1 ...   top 250 540
    bold:     p1 140 0   p2 95 0   p3 ...
```

What this buys:

- **Compatibility by construction.** Every source fills in the same
  point IDs. A missing or extra point is a parse error, not a build
  error found an hour later.
- **Stable IDs for free.** Point IDs exist because the topology needs
  them. They are the same ID in every source, so "move p7 in bold" is
  unambiguous. UFO's per-glyph identifiers never managed this across
  layers.
- **Tiny, legible diffs.** A design change to the bold master is a
  row of numbers in one table. Topology edits (add a point) are a
  separate, rarer diff.
- **Token-compact for agents.** Coordinates are a table, not a tree
  of objects. The research found row-oriented layouts cost half the
  tokens of nested JSON, and LLM coordinate errors cluster in long
  nested numeric runs.
- **Three-way merge by ID.** A merge driver (UnityYAMLMerge style)
  matches rows by point ID. Two people editing different masters of
  the same glyph do not conflict. This addresses the main risk the
  proposal listed for per-glyph files.
- **Representation slot falls out naturally.** Geometry for a source
  can be a coordinate table (outline), or a reference to an evaluator
  plus a cached table (neural, procedural). The topology still says
  what shape is expected.

What it costs:

- **Incompatible masters need an explicit model.** Glyphs allows
  different point structures per layer. The answer is probably named
  topology variants, each with its own set of sources, plus rules for
  which variant applies where. Bracket layers already work this way.
- **Point types per master.** Rare, forbidden in compiled VFs. Make
  types topology-level and give sparse sources no say.
- **Every editor must mint IDs on point creation and keep them
  through remove-overlap, reverse, and start-point changes.** UFO
  shows this is implementable; Runebender already has stable point
  IDs internally.
- **Identical drawings produce different bytes** when IDs differ.
  Accept; deterministic ID assignment on import mitigates.

This is testable without a spec. Encode three real variable glyphs
from Virtua Grotesk this way and the conventional way, then compare
diff size, token count, and agent edit success.

## 4. Serialization: keep the decision open, narrow the options

Evidence from the research:

- JSON with a strict profile keeps every parser in every language.
  Its costs (no comments, verbose point objects) go away if points
  are rows and comments move to a `note` field.
- A custom text grammar (KiCad S-expressions, KDL) reads better and
  diffs well, and costs a parser per language. Fonts have a Python
  and a Rust ecosystem and nothing else that matters; two parsers is
  not a large cost, but it is a permanent one.
- XML and GLIF buy UFO tooling and nothing else. Once the model
  diverges from UFO, that tooling needs rewriting anyway.

Narrowed to two candidates: strict-profile JSON, or a line-oriented
text format. Decide by the serialization spike, not by taste.

## 5. Package layout, revised from the proposal

```
Family.lilufo/
  font.json            format version, generator, names, metrics zones
  axes.json            axes, mappings, sources (name, location,
                       sparse), instances, rules, variable-font subsets
  glyphs/
    a.json             topology + geometry tables for every source
    beh-ar.json
  kerning.csv          side1, side2, one column per source
  features/            fea with variable scalar syntax
  assets/              pointer files: {hash, size, type, source}
  LICENSE, README      optional, human
```

Not in the package: `.cache/` for snapshots and thumbnails, editor
state, build outputs. Glyph file names derived from glyph names by
a reversible algorithm (UFO issue 164); glyph identity is the name,
rename tracked in a `renames` log so history survives.

## 6. UFO and Designspace: what to do about them

Not a contribution target for the core. UFO 4 has no process, the
ideas lilufo needs have sat in its tracker for five years, and the
model difference (layer-has-glyphs vs glyph-has-sources) is not
bridgeable by extension.

Still an obligation: lossless import of UFO + Designspace, and export
with a documented loss report. babelfont-rs is the import layer; a
lilufo convertor there gets Glyphs, Fontra, VFJ, SFD, and fontc IR as
well. Export to UFO + Designspace is the compatibility story for
fontmake users.

Possible later contribution: once the topology/geometry model is
proven, a UFO 4 proposal for shared identifiers across layers would
be a small, concrete thing to offer, with lilufo as the existence
proof.

## 7. Candidate decisions

For Eli to accept or change. Each becomes a `DECISIONS.md` entry.

- A. lilufo is a new format with converters, not a UFO extension.
- B. Glyph-has-sources, one file per glyph.
- C. Topology/geometry split is the central hypothesis, to be
  tested by a serialization spike before any schema is written.
- D. Serialization is JSON strict profile or a line-oriented text
  format; the spike decides.
- E. The edit protocol is a separate spec. Runebender's MCP surface
  is its first draft.
- F. Neural and procedural representations are an extension, not
  core, but the core geometry slot is designed to hold them.

## 8. Next step

Serialization spike. Take three glyphs from Virtua Grotesk (one
simple, one with components, one with an intermediate master).
Encode each as UFO today, as nested JSON, and as topology plus
geometry tables. Measure: bytes, tokens, size of the diff for a
one-point move in one master, and whether an agent can make that
move correctly from each encoding. Write the result as
`research/serialization-spike.md`. No spec until the numbers exist.
