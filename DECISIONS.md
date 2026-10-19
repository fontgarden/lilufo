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
