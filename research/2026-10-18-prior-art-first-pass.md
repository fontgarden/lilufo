# Prior art, first pass

Date: 2026-10-18. Quick survey before any schema work. Facts only;
the argument lives in the discussion, not here.

## UFO 4

- Status on unifiedfontobject.org: "In development. UFO 4 will be an
  extension of UFO 3." No proposals listed as under consideration,
  approved, or rejected on the UFO 4 page.
- Work happens in GitHub issues on `unified-font-object/ufo-spec`.
  About 56 open issues. The ones that overlap with lilufo goals:
  - #175 (2021) Variable-first UFO: one designspace + one UFO with
    layers as sources; variable kerning and features; merge or adopt
    designspace into the UFO spec. Discussion stalled in April 2021.
  - #114 (2020) Diff-ability as an explicit goal. Points raised:
    canonical formatting, no data stored in two places
    (contents.plist), separate canonical source from derived caches.
    Actionable items proposed by Tal Leming, not landed.
  - #238 (2025) Single-file UFO (UFOX). Driver is I/O on network
    drives. Prototype exists as a gist. Active as of Dec 2025.
  - #77 (2019) Move unicode out of GLIF into a font-level cmap.
    Driver: preview text without parsing every glyph file.
  - #164 Reversible glyph file naming. #244 (2026, 49 comments)
    ordered overlapping kerning groups. #106 XML feature format.
    #117 validation suite. #228 license for the spec text.
- Nothing in the tracker about stable element IDs, semantic edit
  operations, agent editing, provenance, or non-outline glyph
  representations. UFO 3 already has `identifier` attributes on
  points, contours, components, anchors, and guidelines, optional
  and unique per glyph.

## Fontra

- Native backend is a directory (`.fontra`): `font-data.json`,
  `glyph-info.csv`, `kerning.csv`, `features.txt`, `glyphs/` with one
  JSON file per glyph, `background-images/`. Source:
  `fontra/fontra/src/fontra/backends/fontra.py`.
- One glyph file holds all sources of that glyph (`VariableGlyph`),
  with glyph-local axes. This is the closest existing thing to the
  per-glyph grouping in the lilufo proposal.
- Fontra also publishes a convention for variable components inside
  UFO, stored in glyph lib keys:
  `github.com/fontra/variable-components-in-ufo`.
- Backends for UFO, Designspace, Glyphs, OpenType, and `.rcjk`.

## Glyphs

- `.glyphspackage`: same data as `.glyphs`, split into one file per
  glyph under `glyphs/`, plus `fontinfo.plist`, `order.plist`,
  `UIState.plist`. Per-glyph file holds all masters as layers.
- Format documented in the GlyphsSDK repo with JSON Schema files.
- Proprietary origin, openly documented, NeXTSTEP plist syntax.

## Designspace 5

- Already covers axes, mappings, discrete axes, sources, layers as
  sources, sparse sources, instances, rules, variable-font subsets,
  STAT labels. Not examined field by field yet.

## Open questions this pass raised

- How much of the lilufo list is already in Fontra's model.
- Whether Fontra's JSON-per-glyph format has any spec document, or is
  defined only by its Python classes.
- What the UFO maintainers would accept as UFO 4 proposals versus
  what needs a new format.

## Addendum: Fontra, babelfont, and the "why not UFO" question

Checked 2026-10-18 against source, not docs.

### Fontra and UFO

- Fontra's primary storage is still UFO + Designspace. The `.fontra`
  directory is an alternative backend, documented nowhere I could
  find: not on fontra.xyz, docs.fontra.xyz, or the blog. Behdad's
  "State of Text Rendering 2024" says the format is not stable and
  its structure is similar to `.glyphspackage`.
- Fontra does not depend on babelfont. Its Python dependencies are
  fontTools, ufo2ft, ufomerge, cattrs, aiohttp, skia-pathops, Pillow.
  The data model is `src/fontra/core/classes.py`, Python dataclasses
  serialized with cattrs. There is no schema document.
- No written rationale for the native format from Just van Rossum was
  found. Inferred from the code: `VariableGlyph` with `axes`,
  `sources`, `layers` cannot be expressed in UFO without lib hacks,
  which is exactly what `fontra/variable-components-in-ufo` does.
  The native format exists so the model does not have to be
  flattened into GLIF lib keys.
- Google's `fontc` reads `.fontra` directly via `fontra2fontir`, so
  the format has a second, independent reader.

### babelfont

- Two implementations, both by Simon Cozens: Python `babelfont`
  (3.x rewrite) and Rust `babelfont-rs` (crates `babelfont`,
  `fontmerge`, `sr-aef`, plus a TypeScript mirror `babelfont-ts`).
  Both active, last pushed October 2026.
- Native `.babelfont` is a single JSON file, versioned `[1, 0]`:
  `upm`, `axes`, `cross_axis_mappings`, `instances`, `masters`,
  `glyphs` (each with `layers`, where each layer is a master's
  drawing), `names` (i18n dictionaries), `features`, kern groups,
  `custom_ot_values`, `variation_sequences`, and a `format_specific`
  bag on every object for lossless round trips.
- Rust model: `Font`, `Master`, `Glyph { layers, component_axes }`,
  `Layer { shapes, anchors, guides, location, smart_component_location }`,
  `Shape = Path | Component`. Layers carry an optional `id`, nothing
  below that has an ID. Nodes are positional.
- Convertors: UFO/Designspace (read; write in progress), Glyphs 2/3,
  Fontra (read and write), FontLab VFJ, FontForge SFD, RoboCJK, TTF,
  and fontc IR (`fontir`).
- Design intent is interchange: a lossless superset of every editor's
  model, not an editing or Git source format. One file, so no
  per-glyph diffs.
