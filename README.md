# lilufo

lilufo is a proposed open source format for font sources. It is
built for variable fonts, for editing by AI agents, and for version
control. It is inspired by UFO and is not an extension of UFO.

Nothing is built yet. This repository holds the research, the
decision log, and the arguments. The goal of this stage is feedback.
If an existing format can do the job, this project stops. If you
think that is the case, open an issue and say which format and why.

## The problem

A font source today is spread across formats that were designed
before variable fonts and before AI agents. The common pain points:

- **Masters are separate files.** In UFO and Designspace, one glyph
  lives in one file per master. A change to one glyph touches many
  files, and the designspace file ties them together from outside.
- **Nothing below the glyph has a stable name.** A point is "the
  seventh point of the second contour". When a contour is reversed
  or a point is added, every tool and every diff loses track.
- **Interpolation compatibility is checked after the fact.** Each
  master stores its own point list. Mismatched masters are found at
  build time, not at edit time.
- **Only outlines can vary.** Kerning, metrics, anchors, and feature
  values each have their own partial answer, or none.
- **No canonical form.** Two tools write the same font as different
  bytes. Diffs show noise, and merges fail on files that did not
  change in meaning.
- **Source and cache are mixed.** Editor state, thumbnails, and build
  output end up next to the drawings and in version control.
- **Editing is by whole file.** An agent and a human who edit the
  same font at the same time have no way to detect a conflict
  smaller than the whole font.

Each existing format solves some of these. None solves stable names,
compatibility by construction, variation of every number, or a
canonical form, and none solves all of them together. The comparison is in
`research/2026-10-18-font-formats-field-comparison.md` and the gap
table is in `research/2026-10-18-synthesis.md`.

## The idea

### A glyph is a file, and it holds all of its sources

One file per glyph. Every master, intermediate, and conditional
source of that glyph lives in that file. Glyphs, Fontra, FontLab,
and RoboCJK each chose this independently. The authors of UFO
proposed it for UFO 4 in 2021 (ufo-spec issue 175) and it did not
land.

This gives per-glyph diffs, per-glyph conflict detection, and
per-glyph atomic writes. An agent can edit one glyph while a person
edits another in a live editor, and both sides can tell when the
same glyph changed under them.

### Structure is declared once, masters supply only numbers

This is the central hypothesis and it is not yet proven.

A glyph declares its point structure once: contours, point IDs,
point types, smooth flags, components, anchors. Each source is then
a table of coordinates against those IDs.

```
glyph a
  structure            declared once
    contour c1: p1 curve smooth, p2 offcurve, p3 offcurve, p4 curve
    anchor top
  sources              one row set per master
    regular:  p1 120 0   p2 80 0   p3 60 20   p4 40 60   top 250 540
    bold:     p1 140 0   p2 95 0   p3 70 25   p4 48 70   top 250 580
```

A compiled variable font already works this way. The `gvar` table
stores one outline and per-region deltas. The source format stops
pretending otherwise.

What this buys:

- **Compatibility by construction.** Every source fills in the same
  IDs. A missing point is a parse error, not a build error.
- **Stable IDs across masters for free.** "Move p7 in bold" is
  unambiguous. UFO has optional per-glyph identifiers, but not shared
  across layers.
- **Small diffs.** A design change to one master is one row of
  numbers. A structure change is a separate, rarer diff.
- **Merge by ID.** A merge driver can match rows by point ID. Two
  people who edit different masters of one glyph do not conflict.
- **Compact for agents.** A coordinate table costs about half the
  tokens of nested JSON objects, and keeps numbers in short rows,
  which is where models make fewer mistakes.

What it costs:

- Masters with different point structures need an explicit model,
  such as named structure variants with their own source sets.
- Every editor must mint IDs when it creates points and keep them
  through reverse, remove-overlap, and start-point changes.
- Identical drawings can produce different bytes when IDs differ.

### Every number can vary

Kerning, metrics, anchors, and feature values use one model: one
value is a constant, N values are a variation. Kerning is a table
with one column per source. Nothing varies by a different mechanism
than outlines do.

### Source is separate from cache and from edits

The package holds source and nothing else. Thumbnails, editor state,
build output, and cached outlines of procedural glyphs live outside
it. The spec defines a canonical form, including key order and
number precision, so two tools write the same bytes.

How an agent changes a font is a separate, smaller spec layered on
the file format: target by stable ID, precondition on the hash of
the file that was read, dry run, receipt. Runebender's MCP tools are
the first draft of that spec.

### OpenType outlines are the core, everything else is an extension

The core is outlines and components, the same as every font tool
expects. Each glyph source has a geometry slot. The slot is shaped so
that a reference to a procedural or neural model plus a cached outline
can sit there later, under an extension namespace. Core tools use the
cached outline and ignore the model. Every object carries namespaced
extension data, and readers preserve what they do not understand.

## How this compares

| | UFO + Designspace | Glyphs 3 | Fontra | lilufo |
|---|---|---|---|---|
| Open written spec | yes | documented | no | yes, planned |
| All sources of a glyph in one file | no | yes | yes | yes |
| Stable IDs below the glyph, shared across masters | no | no | no | yes |
| Compatibility by construction | no | no | no | yes |
| Variable kerning, metrics, features in one model | no | partial | partial | yes |
| Canonical form in the spec | no | sorted keys | no | yes |
| Source separated from cache and UI state | partial | no | partial | yes |
| Slot for non-outline glyphs | no | no | no | extension |
| Edit protocol with preconditions | no | no | no | separate spec |
| Editor independent | yes | no | no | yes |

The lilufo column describes the design, not working code.

## Why not contribute to UFO instead

UFO 4 has no active process and no listed proposals. The ideas
lilufo needs have sat in its tracker for five years. The model change
from "a layer has glyphs" to "a glyph has sources" is not bridgeable
by an extension. The details are in
`research/2026-10-18-ufo-issue-175.md`.

lilufo still owes UFO users a path: lossless import of UFO and
Designspace, and export to them with a loss report. The planned
converter layer is babelfont-rs, which also gives Glyphs, Fontra,
FontLab, and FontForge import.

## What is decided and what is not

Decided, in `DECISIONS.md`:

- New format with converters, not a UFO extension.
- The glyph is the unit of a file.
- Structure declared once per glyph is the central hypothesis, to be
  tested before any schema is written.
- Serialization candidates are XML, strict JSON, and line-oriented
  text. Readability is a scored criterion. JSON is not the default.
- The edit protocol is its own spec. This one is provisional.
- OpenType outlines are the core. Neural and procedural glyphs are
  extensions.

Open:

- The serialization. A spike will encode three real variable glyphs
  each way and compare bytes, tokens, diff size, merge behavior,
  agent edit success, and readability.
- Whether the structure split holds up on real incompatible masters.
- Whether a glyph is one file or one directory.
- The license for the specification text.

## Feedback wanted

The questions this repository exists to answer:

1. Is there an existing format that already does this? If so, which
   one, and what does it lack?
2. Does the structure-once, numbers-per-master split break on real
   fonts you know? Which glyphs?
3. What did your tooling need from a source format that no format
   gave you?

Open an issue at <https://github.com/fontgarden/lilufo/issues>.

## Layout

- `proposals/` — design sketches, dated
- `research/` — prior art, comparisons, and the synthesis
- `spec/` — the specification, once a draft exists
- `DECISIONS.md` — the decision log

## Related

- [Runebender](https://github.com/fontgarden/runebender) — planned
  reference implementation
- [lilufo-cli](https://github.com/fontgarden/lilufo-cli) — an older,
  unrelated UFO command-line tool that used this name first

## License

Apache-2.0. The license for the specification text is an open
decision in `DECISIONS.md`.
