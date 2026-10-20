# Decisions

One entry per decision. Newest at the bottom. A decision can be
reversed by a later entry that names it.

## 2026-10-18 — Repository created

The spec, planning, and docs for lilufo live in one repository,
`fontgarden/lilufo`. The earlier `fontgarden/lilufo` (a Rust UFO
command-line tool) was renamed to `fontgarden/lilufo-cli`.

## 2026-10-18 — Open question: new format or contribute to UFO/Designspace

Not decided. The first research task is to compare UFO 3, the UFO 4
work, Designspace 5, Glyphs packages, and Fontra against the goals in
the proposal, and find out what is really missing.

## 2026-10-18 — Open question: license for the specification text

Code in this repository is Apache-2.0. Specification text may need a
different license (for example CC BY 4.0). Decide before the first
public spec draft.

## 2026-10-18 — Fontra is an import target, not an upstream

lilufo will read `.fontra` projects through converters, but will not
be built as a contribution to Fontra or on top of its data model.
Fontra's model is still a useful checklist of what a variable glyph
needs to hold.

## 2026-10-19 — lilufo is a new format with converters

Not a UFO extension. Lossless import of UFO and Designspace and
export with a loss report are required. babelfont-rs is the planned
converter layer.

## 2026-10-19 — The glyph is the unit of a file

One file, or one directory, per glyph, holding every source of that
glyph. The spike decides file versus directory. The reason is
per-glyph conflict detection between a headless agent and the live
editor, as well as Git. See research/2026-10-19-two-agents-one-font.md.

## 2026-10-19 — Topology/geometry split is the central hypothesis

Point structure, IDs, and types declared once per glyph; each source
is a coordinate table against those IDs. To be tested by a
serialization spike before any schema is written. Not yet proven.

## 2026-10-19 — Serialization candidates: XML, strict JSON, line-oriented text

Readability is a scored criterion, judged by Eli. The spike encodes
the same glyphs all three ways and compares bytes, tokens, diff size,
merge behaviour, agent edit success, and readability. JSON is not the
default.

## 2026-10-19 — The edit protocol is its own spec

The file format describes a font on disk. A separate, smaller spec
describes how an agent changes it: target by stable ID, precondition
on the file hash that was read, dry-run, receipt. Runebender's MCP
tools are the first draft. Provisional: Eli accepted this as worth
trying and it can change if it causes problems.

## 2026-10-19 — OpenType outlines are the core; neural and procedural glyphs are extensions

The core format is outlines and components. Each glyph source has a
geometry slot shaped so a model reference plus a cached outline can
sit there under an extension namespace later. Core tools use the
cached outline and ignore the model.
