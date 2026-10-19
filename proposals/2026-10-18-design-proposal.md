# lilufo — An Open Font Source Format for Variable, Procedural, and AI-Assisted Typography

**Status:** Exploratory working proposal / agent handoff document  
**Working name:** lilufo  
**Proposed extension:** `.lilufo` (directory package)  
**Initial implementation target:** Runebender (Rust)  
**Date:** 2026-10-18

> **Elevator pitch:** lilufo is a proposed open, human-readable, cross-platform, cross-application font source format inspired by UFO. It brings variable-font authoring into a coherent project model and is designed for precise agent-based editing, reproducible font engineering, and optional procedural and neural glyph representations.

**Important:** Everything labeled *proposed* or presented in examples below is design exploration, not an existing specification. The example JSON is illustrative, not normative. This document is intended to be iterated on with engineers, type designers, and AI agents.

## 1. Goals and motivation

Preserve the best characteristics of [Unified Font Object (UFO)](https://unifiedfontobject.org/):

- Openly specified, long-lived, platform- and application-independent.
- Human-readable and accessible to ordinary developer tools.
- Modular glyph data that lends itself to Git and programmatic processing.
- Extensible without making every application implement every extension.
- Easy to inspect, edit, validate, and exchange.

Borrow useful characteristics of [Glyphs](https://glyphsapp.com/), especially the integrated source-family model used by `.glyphs` and `.glyphspackage`:

- First-class variable-font axes, masters/sources, and instances.
- All sources for one glyph accessible together.
- Efficient family-wide and glyph-specific editing.
- Support for layers, interpolation, sparse masters, and alternate glyph construction methods.

Design for new requirements:

- Safe, targeted changes by AI agents, scripts, and human collaborators.
- A representation-independent model: outlines, components, strokes, parametric constructions, and learned models can coexist.
- Explicit provenance, validation, reproducible builds, and dependency tracking.
- Suitability for Runebender's experiments with neural and post-OpenType fonts, while retaining utility for conventional fonts.

### Non-goals

- Replacing OpenType as a compiled distribution format in v1.
- Requiring neural networks, a cloud service, or AI for ordinary usage.
- Baking a particular editor's internal UI state into the interchange standard.
- Requiring arbitrary code execution merely to open or inspect a project.
- Claiming direct binary or file-level backward compatibility with UFO without defining it.

## 2. Prior art and interoperability

| Format / system | Strengths to examine | Implications for lilufo |
| --- | --- | --- |
| [UFO](https://unifiedfontobject.org/) / GLIF | Open specification, individual glyph files, layers, object identifiers, extension mechanisms | Preserve openness and granular editing; reuse concepts rather than duplicate blindly |
| [Glyphs](https://glyphsapp.com/) `.glyphs` / `.glyphspackage` | Unified font family, masters, variation and per-glyph layers | Consider one font project with all sources grouped per glyph |
| [Designspace](https://fonttools.readthedocs.io/en/latest/designspaceLib/) | Axis mappings, locations, sources, instances, variation rules | Treat these capabilities as foundational, not just min/default/max axes |
| [Fontra](https://fontra.xyz/) | Variable glyph model, local axes, editing and collaboration | Study its current model closely before creating a competing schema |
| [fontTools](https://github.com/fonttools/fonttools) | Interchange and compilation infrastructure | Target import/export and build integration |
| Git | Text diffs, provenance, collaboration | Canonical formatting and small, independently modifiable files |

**Context:** UFO plus Designspace already supports variable-font workflows. The novelty of lilufo should not be described as inventing variability; it is integrating variability, agent-editable structure, and extensible representations within one self-contained project format.

Review the evolving UFO specification (including any UFO 4 work) and current Fontra and Designspace specifications before freezing a schema.

## 3. Core design principles

1. **Human-readable and agent-editable.** Ordinary text tools should inspect and modify metadata and glyphs.
2. **Git-native.** Stable IDs, deterministic serialization, small diffs, per-glyph isolation, and straightforward merges.
3. **Variation first.** Axes, mappings, masters, instances, sparse sources, and glyph-specific variability belong to the coherent core model.
4. **Representation-independent.** An outline is one possible implementation of a glyph, not always the only source of truth.
5. **Reproducible.** Sources, asset hashes, and explicit build requirements should suffice to reproduce results where possible.
6. **Interoperable and optional by design.** Simple editors can support the conventional core without evaluating neural models.

**Guiding rule:** The format describes font data, not the internal state of a particular app, agent, training framework, or model runtime.

## 4. Proposed on-disk organization

A `.lilufo` project starts as a directory containing UTF-8 structured files, with binary assets explicitly separated. JSON is a candidate, not a settled requirement.

```text
MyFont.lilufo/
├── font.json                 # Format version, family metadata, capabilities
├── designspace.json          # Axes, mappings, sources, instances, rules
├── glyphs/
│   ├── a.json                # All variable sources of the glyph 'a'
│   ├── beh-ar.json
│   └── noon-ar.json
├── features/
│   ├── liga.fea
│   └── calt.fea
├── kerning/                  # Kerning data, grouped/segmented as appropriate
├── models/                   # OPTIONAL: manifests and possibly local weights
├── datasets/                 # OPTIONAL: manifests/references, not necessarily data
├── workflows/                # OPTIONAL: build/training/generation recipes
├── tests/                    # OPTIONAL: validation/QA fixtures
└── assets/                   # OPTIONAL: images and other supporting assets
```

This structure is a sketch, not a mandate. A final design must resolve layer naming, filesystem portability, escape/case rules, stable paths through glyph renames, kerning granularity, feature order, localized names, metadata, and package archival. Consider an optional deterministic ZIP transport while keeping the unpacked package canonical for editing.

### Why per-glyph grouping matters

Conventional UFO + Designspace can represent a family as:

```text
MyFont.designspace
Regular.ufo/glyphs/a.glif
Bold.ufo/glyphs/a.glif
Light.ufo/glyphs/a.glif
```

A lilufo alternative is:

```text
MyFont.lilufo/glyphs/a.json
```

The latter can put all sources and representations of one glyph in one context, minimizing cross-directory traversal for agents and font editors. **Trade-off:** edits to different sources of the same glyph now share a file and can conflict; this should be tested with real repositories.

## 5. Illustrative variable-font data

`designspace.json` (abbreviated; intentionally omits many necessary Designspace features):

```json
{
  "axes": [
    {
      "tag": "wght",
      "name": "Weight",
      "min": 100,
      "default": 400,
      "max": 900
    }
  ],
  "sources": [
    {"id": "regular", "location": {"wght": 400}},
    {"id": "bold", "location": {"wght": 900}}
  ],
  "instances": [
    {"name": "Semibold", "location": {"wght": 600}}
  ]
}
```

A real schema should additionally handle user/design axis coordinates, axis mapping, discrete axes, defaults, source layering, glyph-specific axes, sparse sources, instances, rules, and compatible interpolation behavior.

`glyphs/a.json` (partial pseudo-glyph, **not valid production outline geometry**):

```json
{
  "name": "a",
  "unicodes": ["0061"],
  "sources": {
    "regular": {
      "advance": 520,
      "representation": "outline",
      "contours": [
        {
          "id": "contour-01",
          "points": [
            {"id": "point-01", "x": 120, "y": 0, "type": "line"}
          ]
        }
      ]
    },
    "bold": {
      "advance": 560,
      "representation": "outline",
      "contours": []
    }
  }
}
```

### Stable identity and targeted changes

Use stable object IDs for glyphs, contours, points, components, anchors, and other addressable elements where meaningful. An editing tool can then request a small operation (e.g., move `point-01` by +12 units in the regular source of `a`) instead of replacing the entire document. Define exact ID uniqueness and lifecycle rules, including contour reordering, splitting, interpolation compatibility, duplication, and merges.

## 6. Representation-independent glyphs

The glyph object should separate **identity and semantics** (name, Unicode associations, features, anchors, etc.) from **how geometry and behavior are produced**.

Candidate representation types:

- **`outline`:** Bézier paths, contours, components, anchors, metrics.
- **`parametric`:** Strokes, dimensions, and constrained constructions.
- **`procedural`:** Deterministic generation recipe with declared inputs and outputs.
- **`neural`:** A learned model predicts geometry, metrics, positioning, or other properties.

A single glyph could possess more than one representation. The format must explicitly identify:

- Which representation or source is **authoritative**.
- Which results are derived caches or snapshots.
- Which representations particular applications can edit vs. merely display/export.
- What coordinate system, units, conditioning variables, and interpolation semantics apply.
- When caches are stale and how to regenerate them.

### Example neural reference

```json
{
  "name": "beh-ar",
  "sources": {
    "default": {
      "representation": "neural",
      "model": "models/nastaliq-geometry",
      "inputs": {
        "glyph": "beh-ar",
        "style": "regular"
      }
    }
  }
}
```

This is **an example of an extension concept, not a specification for Runebender's current NTF data**. Model manifests would need versioning, input/output tensor contracts, shapes, units, conditioning semantics, architecture/runtime identifiers, hashes, and ideally test vectors.

Binary model weights should live separately from text metadata, as local files or content-addressed external references. The project should remain inspectable when the model runtime or weights are unavailable.

### Proposed evaluation boundary

```text
Glyph identity + variation location + selected representation
                           │
                           ▼
                    Evaluator/runtime
                           │
                           ▼
   Evaluated geometry + metrics + anchors + positioning
```

A traditional editor may use a generated outline snapshot while preserving the original neural or procedural source. Whether this evaluation interface belongs in the base spec or an extension is open.

## 7. Agent-friendly architecture

**Good AI support does not mean adding a free-text prompt field to every object.** The essential property is a precise, discoverable, validated editing surface.

| Feature | Purpose |
| --- | --- |
| Stable IDs | Target a glyph, source, contour, point, anchor, or component unambiguously |
| Machine-readable schema | Validate and constrain generated edits |
| Canonical serialization | Stable diffs; predictable hashes; reduced agent churn |
| Semantic patch operations | Record intent separately from file rewrites |
| Incremental validation | Check the edited glyph and its affected dependents |
| Dependency graph | Rebuild components, composites, kerning, and layout impacts |
| Provenance | Distinguish authorship, imports, transformations, and outputs |
| Reproducible builds | Connect source revision, tools, assets, and compiled artifacts |

### Optional editing tool protocol

Develop an API or tool protocol alongside the on-disk spec rather than requiring it inside the file format:

```text
font.inspect
font.get_glyph
font.get_dependencies
font.edit_point
font.add_component
font.interpolate
font.validate
font.build
font.compare
```

Example workflow:

> Inspect Arabic initial forms, identify inconsistent entry-stroke heights, propose targeted adjustments, test interpolation and shaping, and return a reviewable patch and preview.

An agent should be able to inspect a plan, produce changes, validate invariants, and seek human review before applying extensive geometry edits. Tool callers must not assume that a syntactically valid edit is a good design.

### Semantic edit example (conceptual)

```json
{
  "operation": "move_point",
  "glyph": "a",
  "source": "regular",
  "point_id": "point-01",
  "delta": {"x": 12, "y": 0}
}
```

This would be a separate, versioned operation protocol. It should define concurrency and revision preconditions, atomic transactions, conflict detection, undo, and validation outcomes.

## 8. Optional extensions

### Core (required)

Conventional font metadata, glyph identities, outlines, components, anchors, metrics, layers, kerning, OpenType feature references, variation axes/sources/instances and standard interchange semantics.

### Neural extension (optional)

- Model manifests and weight references, possibly content-addressed.
- Dataset references with provenance, licensing and checksums.
- Configuration and conditioning parameters.
- Input/output tensor or semantic contracts.
- Training/evaluation metadata and deterministic test vectors where feasible.
- Derived outline caches with source-of-truth and invalidation rules.

### Workflows extension (optional)

- Build recipes, generation pipelines, and optional node graphs.
- Declared tool/runtime versions and inputs/outputs.
- Reproducibility information, failure records, and validation artifacts.

The **core must not require** executing scripts, loading models, using a network service, or granting untrusted documents execution rights. Extensions must use namespaced identifiers and explicit feature detection, with graceful degradation in unsupported applications.

## 9. Interoperability and compatibility

A credible adoption path depends on high-quality round trips:

1. **UFO 3 ↔ lilufo** for conventional outline sources and metadata.
2. **UFO + Designspace ↔ lilufo** for full variable-family behavior.
3. **Glyphs `.glyphs` / `.glyphspackage` ↔ lilufo** where licensing, public format behavior, and feature mappings permit.
4. **Fontra ↔ lilufo** or shared data model components where feasible.
5. **lilufo → OpenType variable fonts** via established compilers/toolchains.

Document losses explicitly. A converter should report unsupported constructs rather than silently dropping them. For neural/procedural glyphs, outline export will often be a *baked approximation* rather than a lossless round trip.

Interoperability test families should include at least Latin and complex-script fonts, including Arabic shaping and Nastaʿlīq behavior, multiple masters, sparse sources, components, kerning and OpenType layout features.

## 10. Serialization and package design questions

Do not commit prematurely to JSON just because it is convenient for coding agents. Compare JSON, UFO-compatible XML/GLIF, and other readable encodings against actual editing, version control, and tooling needs.

Decide:

- Is the directory package canonical? Is there a deterministic archive/ZIP form?
- How are glyph names mapped to portable filenames (Unicode, case, reserved names, path length)?
- Are source IDs local or globally scoped? Are glyph identities stable through rename?
- What are canonical key order, whitespace, numeric precision, and newline rules?
- How are coordinates, points, segments, components, and interpolation compatibility encoded?
- How are glyph-level local axes, variation deltas, and sparse sources represented?
- What parts of the OpenType layout model are source data vs build outputs?
- How are comments, unknown extensions, and user-defined metadata preserved?
- How does incremental validation and caching work across applications?
- What is the extension capability/version negotiation policy?
- Can untrusted model assets be inspected without execution, and how is integrity enforced?
- How do separate agents safely make concurrent edits?

## 11. Proposed development milestones

### Milestone 1 — Minimal core and Rust parser/serializer

- Draft a concise schema for metadata, glyph IDs, paths, components, anchors and metrics.
- Support deterministic read/write, validation, and error reporting in an independent Rust crate.
- Create sample packages and round-trip tests using standard UFO sources.

### Milestone 2 — Integrated variable-font model

- Support axes, mappings, masters/sources, layers, sparse masters and instances.
- Import/export real UFO + Designspace families.
- Measure behavior and data loss, especially for interpolation and complex scripts.

### Milestone 3 — Agent editing tools

- Version semantic edit operations with preconditions and atomic application.
- Expose inspect, modify, validate, compare and build tools.
- Test precise multi-glyph changes with coding agents and human review.

### Milestone 4 — Neural and procedural prototype

- Import one Runebender NTF experiment using a model manifest and asset reference.
- Evaluate one glyph and cache a conventional outline snapshot.
- Confirm that traditional tools can inspect or export the snapshot while preserving the authoritative source.

### Milestone 5 — Interoperability and public specification

- Publish format and extension schemas, conformance tests, example fonts and explicit loss reports.
- Seek feedback from UFO, Glyphs, Designspace, Fontra, fontTools, and other font-engineering communities.
- Keep the format independent of Runebender even if Runebender is the initial reference editor.

## 12. Naming explorations

| Name | Potential expansion | Character / caveat |
| --- | --- | --- |
| **lilufo** | Can remain a proper name | Friendly, memorable, resembles UFO, and does not bind the standard to AI |
| **NUFO** | New Unified Font Object | Direct UFO lineage; clear but generic |
| **AUFO** | Augmented Unified Font Object | Suggests enhanced and AI-assisted UFO |
| **FUFO** | Future Unified Font Object | Playful, potentially overly informal |
| **LUFO** | Learned Unified Font Object | Good for ML work but narrower than the general-purpose format |

**Current preferred name:** `lilufo`.

**Suggested public description:**

> lilufo is an open, human-readable font source format designed for variable fonts, programmatic typography, and AI-assisted font development. Inspired by UFO, it extends the traditional font source model to support new representations of typography, including learned and procedural fonts.

The format may begin as an experimental UFO fork, but if its schema diverges significantly, it should be presented as *UFO-inspired and interoperable*, not automatically UFO-compatible.

## 13. Architectural trade-offs worth testing

| Decision | Advantage | Cost / risk |
| --- | --- | --- |
| One file per glyph containing all masters | Easy glyph-centric editing, agent context | Conflicts when multiple people edit distinct masters of same glyph |
| Per-master glyph files | Familiar UFO pattern, independent edits | Scattered cross-master glyph state |
| JSON | Broad tool support, schemas, easy agent manipulation | Verbose point data, no comments in standard JSON, noisy numeric diffs |
| GLIF/XML evolution | Reuses UFO tooling and idioms | Less straightforward for integrated multi-master and novel representation data |
| Model-native glyph representations | Enables genuinely new font technology | Portability, trust, reproducibility and evaluation contracts are much harder |
| Generated outline snapshots | Legacy compatibility and previews | Cache invalidation and risk of treating lossy export as authoritative |
| Semantic patch protocol | More precise automated edits and reviews | Extra specification and implementation burden |

## 14. Next research and design tasks for collaborating agents

1. **Prior-art matrix:** Compare UFO 3, ongoing UFO proposals, Designspace 5.x, Glyphs package formats, Fontra data models and NTF requirements at field/schema level. Identify what is genuinely missing.
2. **Minimal schema sketch:** Design a lossless conventional glyph/variation schema; specify stable IDs and portable filename mapping.
3. **Serialization spike:** Encode three real variable glyphs in JSON and GLIF-like alternatives; compare size, Git diffs, read/write speed and human editability.
4. **Interop spike:** Round-trip a variable UFO + Designspace project and log exact structural and semantic losses.
5. **Agent editing experiment:** Have agents make a small anchored geometry change through JSON replacement versus semantic patch; compare correctness, conflict handling and diff readability.
6. **Neural extension proof of concept:** Define a model manifest and reference architecture for one Nastaʿlīq NTF glyph. Evaluate and cache an outline without requiring a model to inspect the package.
7. **Threat model:** Decide safe handling of external assets, hashes, path traversal, unsupported extensions and untrusted workflows.
8. **Governance:** Define versioning, namespace registration, conformance, open specification licensing, and an independent Rust reference implementation.

## 15. Working decision log

### Provisional choices

- Name: **lilufo**.
- Initial form: **directory package**.
- Primary objective: **open, inspectable source-of-truth for traditional and future fonts**.
- Guiding division: **small interoperable core + optional representation/workflow extensions**.
- Initial reference implementation: **Rust / Runebender**, with the spec deliberately app-independent.

### Decisions explicitly **not yet made**

- JSON vs evolved GLIF/XML or another readable encoding.
- Exact glyph/contour/point and variable-source schemas.
- Representation evaluation API, caches and trust boundary.
- How much Designspace semantics to mirror versus generalize.
- How layers, features, glyph-local axes, nonlinear interpolation and compiled output relate.
- File-level vs semantic edit granularity and concurrency rules.
- Extent of lossless UFO/Glyphs/Fontra interoperability.
- Whether NTF and other learned-font formats should live as lilufo extensions or independently referenced resources.

## References / starting points

- UFO project and specification: https://unifiedfontobject.org/
- Glyphs app / source-format ecosystem: https://glyphsapp.com/
- fontTools Designspace documentation: https://fonttools.readthedocs.io/en/latest/designspaceLib/
- Fontra: https://fontra.xyz/
- fontTools repository: https://github.com/fonttools/fonttools
- Runebender context and neural font experiments: https://elih.net/blog/nastaliq-distilled/

---

## Suggested kickoff prompt for another agent

> Read this document as an exploratory design brief, not an accepted specification. First review the current UFO, Designspace, Glyphs and Fontra formats and list concrete interoperability requirements. Then propose a **minimal lilufo v0.1 schema** for conventional variable fonts, including glyph IDs, point/contour/component IDs, variable sources, package layout and canonical serialization. Clearly mark changes that would break UFO round trips, and distinguish normative choices from unresolved research questions. Do not introduce neural or workflow features into the required core unless there is a demonstrable need. Include three example glyph files, a Rust implementation sketch, and a validation/test plan.
