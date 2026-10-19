# Font source formats, field-level comparison

Date: 2026-10-18. Agent research pass. Primary sources: UFO 3 spec
pages, fontTools designspaceLib docs, GlyphsSDK GlyphsFileFormat v3
and v4 docs and JSON schemas, FontForge sfdformat.html, Fontra
`core/classes.py` and `backends/fontra.py`, fontra-rcjk backend, the
variable-components-in-ufo README, boring-expansion VARC.md, one CC0
sample `.vfj` plus babelfont's `vfj.py`. [unverified] marks the rest.

## Per-format notes

**UFO 3.** Directory package, XML plists, GLIF 2, `features.fea`. One
`.glif` per glyph per layer; `contents.plist` duplicates the
name-to-file map (ufo-spec #114). One UFO per master, no variation
concept inside. Optional `identifier` on point, contour, component,
anchor, guideline: 1 to 100 printable ASCII, unique within one glyph
only, not across layers or glyphs; `public.objectLibs` hangs data off
them. Glyphs identified by name; rename not tracked. No glyph-local
axes. Kerning and groups font-level, one set per master, `public.kern1.`
and `public.kern2.` prefixes. Extension: reverse-domain `lib` keys at
font, layer, glyph, object level; `data/` and `images/` (PNG only).
Normalizer is a tool, not spec: ufonormalizer (tabs, integer-valued
floats to int, floats rounded to 10 digits).

**Designspace 5.x.** One XML file. Sources are paths to UFOs; the
`layer` attribute makes a UFO layer a source. The spec says nothing
about what a layer source contributes; fontmake's code comment is the
only statement ("sparse layer sources only contributing glyph outlines
but no info, kerning, features"). Sparse kerning and anchors still
open: fonttools #3168 (2023), #3988. Axes: continuous and discrete
(5.0), `<map>` (avar1), `<mappings>` (5.1, avar2), STAT labels,
`<variable-fonts>` with axis subsets. Rules with condition sets.
Documented non-support: two sources at one location, anisotropic
label locations, extrapolation. IDs: source and instance `name`
strings only. No normalizer.

**Glyphs 3 and .glyphspackage.** OpenStep plist, UTF-8, keys sorted
alphabetically, booleans 1/0, empty containers omitted: a canonical
form is specified. Package: `fontinfo.plist`, `order.plist`,
`UIState.plist`, `glyphs/*.glyph`; v4 adds `kerning.plist`,
`note.md`, `features/*.fea`. Per glyph, `layers[]` each with
`layerId` (equals master `id` for master layers) and
`associatedMasterId`. Brace layers via `attr.coordinates`, bracket
layers via `attr.axisRules`; v4 adds glyph-local axis coordinates.
Smart components: `partsSettings`, `partSelection`, component
`piece`; v4 `glyph.axes`. Nodes are `(x, y, "TYPE[s]", {attrs})`.
Metrics keys per glyph; master `metricValues` with overshoot; kerning
keyed master id, left, right; features font-level with no per-master
variation. Extension: `customParameters`, `userData`. IDs on masters
and layers, none on nodes or anchors. glyphsLib pain: brace and
bracket to designspace needs synthetic glyphs (#468, #715), smart
component incompatibility and nesting (#907, #948).

**FontLab VFJ.** Single JSON, 2-space indent. Per glyph `layers[]`
with `name` matching a master name, so grouping is per glyph.
Elements hold contours or components; nodes are strings `"x y"`.
Kerning lives inside each master. Element `id` used for shared
element references [semantics unverified]. No public schema.

**FontForge SFD.** Single ASCII file, keyword and value header,
`StartChar` to `EndChar` blocks; the page says the authoritative
reference is `sfd.c`. MM via per-instance subfonts, per master not per
glyph. `Refer:` uses glyph index. No element IDs. Stores all OT layout
natively. No normalizer (fontforge #111 open since 2012; #2711
timestamps and #2715 UI state in the file cause spurious diffs).

**RoboCJK .rcjk.** Directory: `designspace.json`, `fontLib.json`,
three glyph sets (`characterGlyph/`, `deepComponent/`,
`atomicElement/`) as `.glif` plus layer subdirs. Glyph-local axes in
`lib["robocjk.axes"]`; sources in `robocjk.variationGlyphs`; variable
components in `robocjk.deepComponents` with a decomposed transform.
Hierarchy is by folder, not schema. No format README.

**Fontra variable-components-in-ufo.** Two glyph lib keys:
`com.black-foundry.glyph-designspace` (axes, sources with location
and layername) and `com.black-foundry.variable-components` (base,
decomposed transformation, location). No version or status; last
commit January 2024.

**Fontra `.fontra`.** `font-data.json`, `glyph-info.csv`,
`kerning.csv` (one value column per source), `features.txt`,
`glyphs/*.json`. Written with `json.dumps(indent=0)`, keys unsorted,
writes not atomic. `VariableGlyph{axes, sources[{name, layerName,
location, locationBase, inactive}], layers}`; `FontSource.isSparse`.
Points carry identifiers only in `attrs`.

**VARC (compiled).** Coverage, `MultiItemVariationStore`,
`ConditionList`, per-component records: gid, condition, axis values
with variation, transform (translate, rotate, scale, skew, center),
`RESET_UNSPECIFIED_AXES`. Components recurse. Not in OpenType 1.9.1;
fontTools implements the HarfBuzz draft.

## Table

| | Shape | Granularity | Master grouping | Stable IDs | Local axes, smart comps | Kerning, features, metrics per master | Extension | Canonical form |
|---|---|---|---|---|---|---|---|---|
| UFO 3 | dir, XML | glyph x layer | per master | per-glyph optional identifiers | none | one set per UFO | lib, data/ | tool only |
| Designspace 5 | one XML | family | points to UFOs | names | none | layer sources drop kerning (tool behaviour) | lib | none |
| Glyphs 3 | plist, file or package | glyph | per glyph | master, layer ids | smart glyphs, brace, bracket | kerning and metrics per master; features font-level | customParameters, userData | specified |
| VFJ | one JSON | font | per glyph | element ids [partial] | none found | kerning inside master | none public | none |
| SFD | one text or .sfdir | font or glyph | per master | none | none | per subfont | free keywords | none |
| .rcjk | dir, GLIF + JSON | glyph x layer | per glyph via lib | none | local axes, deep components | n/a | lib | none |
| VC-in-UFO | UFO lib keys | glyph x layer | per master UFO | UFO identifiers | local axes, variable components | as UFO | lib | tool only |
| .fontra | dir, JSON + CSV | glyph | per glyph | sources by name | local axes, variable components | kerning per source column | customData | none |
| VARC | binary | n/a | deltas | gid | yes | n/a | n/a | n/a |

## Concepts in two or more formats (core candidates)

- Per-glyph grouping of all masters.
- Glyph-local axes with min, default, max and sources at local
  locations.
- Decomposed component transform (translate, rotate, scale, skew,
  center), not a matrix.
- Per-component axis location, sparse, inherited from the parent.
- Intermediate (brace) and conditional (bracket, rules) sources.
- Sparse sources flagged explicitly.
- Kerning as side1 and side2 groups plus pair values per master.
- Metrics as named zones per master with overshoot.
- Localized names on axes, labels, instances.
- Namespaced extension bags on every object.
- Glyph order and cmap stored at font level.

## What no format does well

- Stable IDs below glyph level that survive across masters and
  layers. Nothing tracks glyph renames.
- Variable kerning and features in the same model as variable
  outlines.
- A normative canonical serialization. Only Glyphs specifies sorting.
- Specified semantics of sparse sources.
- Separation of derived caches and UI state from source.
- A written schema for glyph-local variation in an open format.
- Non-outline glyph representations. VARC is the only format whose
  components carry conditions and locations natively, and it is
  compiled-only.
