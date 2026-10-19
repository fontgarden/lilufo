# Lessons from file formats outside fonts

Date: 2026-10-18. Agent research pass, primary sources where
possible. Items marked [unverified] came from secondary sources.

## Case studies

**KiCad, legacy to S-expressions.** One grammar for boards,
schematics, symbols, footprints. Every object carries a random `uuid`.
Date-stamped `version` token plus `generator`. Fixed decimal precision
(6 places for boards, 4 for schematics), no exponents. Lesson: a small
homogeneous grammar plus a UUID per object and fixed precision makes
a CAD file scriptable and diffable. Rationale comes from forum posts
[partly unverified].
https://dev-docs.kicad.org/en/file-formats/sexpr-intro/

**Blender .blend, why not Git-friendly.** In-memory C structs dumped
with their old memory addresses, plus a DNA schema block. One small
edit changes most bytes. No stable identity, no canonical form, so
VCS falls back to lock-or-overwrite. [partly unverified]
https://formats.kaitai.io/blender_blend/

**Figma and Penpot.** Figma: tree of objects, each an ID and a
property map, last-writer-wins per property, parent and fractional
index stored as one atomic property. Penpot (open): UUID on every
file, page, shape, component, color, media object. Edits are a closed
vocabulary of typed changes (`add-obj`, `mod-obj`, `del-obj`,
`mov-objects`) validated by schemas. v3 export is a ZIP with
`manifest.json`, one JSON file per shape, binaries under
`objects/{uuid}.{ext}` with a blake2b hash. v2 (SQLite) was never
shipped. Lesson: UUID per object plus a typed change vocabulary is
what makes agents and multiplayer possible.
https://help.penpot.app/technical-guide/developer/data-model/penpot-file-format/
https://github.com/penpot/penpot/blob/develop/common/src/app/common/files/changes.cljc

**Typst.** Text-first markup; users version with Git. Lesson: when
the source is the document, every generic tool works; IDs and binary
assets must be layered on by convention.

**Unity YAML scenes.** Each object is its own YAML document headed
`--- !u!<classID> &<fileID>`. Cross-file references need a GUID in a
sidecar `.meta`; losing it breaks every reference. UnityYAMLMerge is
a semantic three-way merge driven by `mergerules.txt`: arrays matched
as sets by `fileID`, float tolerances, fallback to a text merge.
Lesson: stable in-file IDs let a tiny rules file make generic YAML
mergeable; IDs in sidecars are fragile.
https://docs.unity3d.com/Manual/SmartMerge.html

**Godot .tscn.** INI-like text, file-level `uid`, node `unique_id`
that survives rename and move, resources referenced by id. The editor
strips comments on save. Lesson: file uid tracks moved files without a
sidecar; destroying comments is a cost.
https://docs.godotengine.org/en/4.6/engine_details/file_formats/tscn.html

**SQLite as application file format.** Transactions, queries, schema
as documentation. Binary, page-based, so Git stores whole copies and
cannot merge. Penpot abandoned it. Lesson: good for a cache or undo
store, bad as the Git-tracked source.
https://sqlite.org/appfileformat.html

**Automerge / CRDTs.** Every op has an ID (actor, counter). An
object's ID is the op that created it. List elements are addressed by
the ID of their insert op, not by index. Change chunks are hashed and
preserve unknown bytes. Lesson: identity by creation op and position
by predecessor ID is the general answer to merge; history grows
forever and the file is binary.
https://automerge.org/automerge-binary-format-spec/

**RFC 8785 JSON Canonicalization.** No whitespace, sorted keys,
ECMAScript number printing. Output is one unreadable line. Lesson:
put the canonical form in the spec (key order, precision,
indentation) but keep it pretty-printed.
https://www.rfc-editor.org/rfc/rfc8785.html

**RFC 6902 JSON Patch, RFC 7386 Merge Patch.** Patch: ordered
`add/remove/replace/move/copy/test`, atomic, `test` as precondition,
weak on arrays because paths are index-based. Lesson: give agents a
patch vocabulary with preconditions, address by object ID.

**Dolt.** Rows in prolly trees keyed by primary key. Cell-wise merge:
conflict only if both sides change the same column of the same row.
Conflicts stored as tables, not markers. Lesson: cell-level merge
needs only a stable key per record and one attribute per column.
https://www.dolthub.com/docs/concepts/dolt/git/conflicts

**Jupyter and nbdime (cautionary).** Outputs and execution counts
churn every run. Cell `id` added only in nbformat 4.5; the JEP lists
the cost: identical content now differs byte for byte. nbdime does
structural three-way merge. Lesson: keep derived output out of the
source, add IDs from day one, ship the merge driver with the format.
https://raw.githubusercontent.com/jupyter/enhancement-proposals/master/62-cell-id/cell-id.md

**Pijul / Darcs.** Independent changes commute; conflicts are
first-class state. Lesson: a format that records edits against stable
IDs gets most of patch theory without changing VCS.

**Content-addressed assets.** git-annex (symlink to hash), DVC
(`.dvc` pointer files with MD5), Hugging Face Xet (content-defined
chunking, ~64 KB, so a re-saved checkpoint uploads only changed
chunks). Lesson: text pointer files with hashes in Git, bytes
elsewhere; use CDC if weights are re-saved often.
https://huggingface.co/docs/hub/en/xet/deduplication

**KDL and RON vs JSON/TOML/YAML.** KDL 2.0: node syntax, comments,
slashdash to disable a node, lossless mapping to JSON and XML. RON:
Rust-shaped, not self-describing. Lesson: comments help humans, but an
unusual format costs parsers in every language; JSON with a strict
profile keeps the ecosystem.
https://kdl.dev/

## Ranked patterns to adopt

1. Stable UUID on every object, inside the object. (KiCad, Penpot.)
   Cost: identical content differs by bytes; mint at creation, never
   regenerate on save.
2. Canonical serialization written into the spec: key order, fixed
   decimal precision, indentation, no exponents. (KiCad, RFC 8785.)
   Cost: precision choice is permanent.
3. One object per file inside a directory or ZIP. (Penpot v3, Unity.)
   Cost: many small files; needs a manifest.
4. Position by neighbor ID, not array index. (Automerge, Figma.)
   Cost: ordering becomes a field to maintain.
5. Typed change vocabulary with preconditions. (Penpot, JSON Patch
   `test`.) Cost: a second schema to version.
6. Derived data outside the source. (Jupyter as the negative case.)
7. Ship a merge driver with the format, configured by key rules.
   (UnityYAMLMerge, nbdime.)
8. Content-addressed pointer files for weights and images. (DVC, Xet,
   Penpot objects.)
9. Date-based format version plus generator; preserve unknown fields.
   (KiCad, Automerge, nbformat.)
10. Conflicts as data, not text markers. (Dolt, Pijul.)

Avoid: runtime memory dumps (Blender), SQLite as the tracked source,
IDs in sidecar files (Unity .meta), editors that strip comments on
save (Godot).
