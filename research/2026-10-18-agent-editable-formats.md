# Agent-editable structured formats: what is known

Date: 2026-10-18. Agent research pass. Primary sources where
possible; marked where not.

## Edit reliability: whole file vs diff vs structured ops

- Anthropic's SWE-bench write-up: highest reliability came from string
  replacement that applies only when `old_str` matches exactly once;
  otherwise the model gets an error and retries.
  https://www.anthropic.com/engineering/swe-bench-sonnet
- Althoubi, "Look Before You Leap" (arXiv 2609.11957, Aug 2026, one
  preprint): 640 edits across 224 files. Content-anchored formats fail
  loudly. Line-number edits corrupted 99.1% of files under a one-line
  shift. Function-name anchoring hit the wrong function 12.7% of the
  time. An anchor-and-verify applier that refuses when unsure had 1
  silent misapplication in 8,320 trials.
  https://arxiv.org/abs/2609.11957
- Aider: moving to a simplified unified diff raised a refactor
  benchmark from 20% to 61%. What mattered: no line numbers, hunks
  treated as search/replace, whole-block rewrites, lenient applier.
  The JSON function-call format for edits did worse than plain text.
  https://aider.chat/docs/unified-diffs.html
- "To Diff or Not to Diff?" (ACL Findings 2026, arXiv 2604.27296):
  block- and function-level rewrites match whole-file accuracy at
  >30% lower cost. Line-numbered diffs are hard to generate. AST diff
  tools do not suit token-by-token generation.
- Takeaway: the model names what to change by content or stable name;
  a deterministic applier verifies uniqueness and refuses rather than
  guesses. Index addressing is the documented failure mode.

## Schema-constrained generation and its limits

- JSONSchemaBench (arXiv 2501.10868, ICML 2025): real-world schema
  coverage on the hard set ranges from 3% to 41% across engines.
  Closed providers support a conservative subset.
- Tam et al. (EMNLP 2024 Industry): strict format constraints can
  hurt reasoning; "reason in prose, then convert" mitigates. Conflicts
  with JSONSchemaBench on quality; treat as model- and task-dependent.
- Provider subsets as of 2026-10-18: Anthropic strict mode rejects
  `minimum`/`maximum`, `minLength`/`maxLength`, recursion. OpenAI
  allows numeric bounds but caps properties, nesting, enum size, and
  requires all fields required.
- Implication: a schema guarantees shape, not semantics (coordinate
  ranges, closed contours, ID existence). Those checks belong in a
  validator, not the grammar.

## MCP tool design guidance

- MCP spec 2025-06-18: `readOnlyHint`, `destructiveHint`,
  `idempotentHint`, `openWorldHint`; `outputSchema`; errors in
  `isError` results so the model can recover.
- Anthropic, "Writing effective tools for agents": few consolidated
  tools; agents handle names better than opaque UUIDs (resolving UUIDs
  to names measurably cut hallucinations); concise/detailed response
  modes; errors must state the fix.
  https://www.anthropic.com/engineering/writing-tools-for-agents
- Dry-run, receipts, idempotency keys: no MCP-level standard. Seen in
  practice in thierryc/Glyphs-mcp (job preview then accept) and
  Runebender `agent_apply` with `operation_key`.

## Optimistic concurrency precedents

- Google Docs `batchUpdate` `WriteControl`: `requiredRevisionId`
  rejects stale writes; the whole batch is atomic.
- GitHub Contents API: update requires the blob `sha`; 409 on
  conflict.
- Monperrus, "Lost update problem for humans and AI agents" (July
  2026): whole-file writes race with editor autosave; fix is small
  operations or making the agent a peer in a CRDT/OT session.
  https://www.monperrus.net/martin/lost-update-problem-humans-ai-agents

## Creative-tool agent integrations

- Blender MCP: central tool is `execute_blender_code`; objects by name.
- Penpot official MCP: 5 tools, one is `execute_code`.
- Figma MCP: nodes addressed by node ID from "Copy link".
- Rhino/Grasshopper (LAMCP): nickname + GUID; arbitrary Python; no
  undo.
- Glyphs: thierryc/Glyphs-mcp uses a job model (start, preview,
  apply/accept, undo, recovery copy before Python runs).
  nmassi/glyphs-mcp edits by replacing a whole layer's paths.
- Fontra: no MCP found. Backend protocol is whole-object
  `getGlyph(name)` / `putGlyph(name, glyph, codePoints)`. Points,
  contours, components have no IDs.
- Pattern: most servers converge on one arbitrary-code tool plus a
  few readers. The font-specific ones that matured moved to
  proposal/receipt workflows.

## Scripting API addressing in font editors

- Glyphs: glyphs by name; layers by opaque `layerId` equal to the
  master id "chosen by Glyphs.app"; nodes by index only; anchors by
  name.
- RoboFont/FontParts: points, contours, components, anchors have
  optional UFO 3 `identifier`; `getIdentifier()` mints one; `index`
  shifts on insert/delete.
- FontForge: contours and points by integer index only; references go
  stale after `removeGlyph`.
- UFO 3 GLIF: optional `identifier` on point, contour, component,
  anchor, guideline; unique per glyph; retained across remove-overlap
  "when possible"; may change on paste.
- What breaks for agents: index addressing after any insertion;
  opaque master IDs looked up before every edit; whole-layer
  replacement clobbering concurrent user edits; execute-code tools
  with no precondition or receipt.

## Token cost and numeric data

- TOON project benchmark (self-reported): JSON 4,308 tokens vs YAML
  3,487 vs compact JSON 2,892 vs TOON 2,474 vs CSV 1,851 on the same
  data; accuracy within overlapping confidence intervals except CSV
  on one small model. https://github.com/toon-format/toon
- No controlled study of LLM coordinate-reading accuracy by format.
  Adjacent: SVGEditBench V2 errors cluster in `<path>` numeric
  sequences; integer rounding shortened sequences without visual
  loss. https://arxiv.org/abs/2502.19453

## Semantic patch precedent

- Coccinelle, Comby, OpenRewrite LST, ast-grep. Shared lesson: express
  the edit against structure, keep a lossless mapping back to text so
  unrelated bytes do not change.

## Multi-agent editing

- AgentRoom (arXiv 2608.23740): CRDT workspace plus claim/release
  tools; authors say explicit coordination, not CRDT merging, drove
  gains. CRDTs guarantee byte convergence, not semantic validity.

## Conclusions

**Format-level properties (must be in the file):**
- Stable, human-readable identity for every addressable unit: glyphs
  by name, sources by name (not opaque IDs), optional per-point,
  contour, component identifiers retained across edits.
- Deterministic canonical serialization (key order, number format,
  one object per line or per file) so content-anchored replacement
  matches exactly once and diffs are small.
- Lossless round-trip with no tool-specific noise, so an applier can
  rewrite one unit without touching others.
- Structural validity expressible in a schema the strict-mode subsets
  accept, plus a documented invariant list for a validator.
- Compact numeric encoding: integers where possible, tabular point
  rows rather than one JSON object per point.

**Protocol-level properties (tool layer, not the file):** revision
preconditions, atomic batches, dry-run with receipts and idempotency
keys, undo, concise/detailed read modes, MCP annotations, semantic
validation after schema validation, claim/release for multiple
agents. These vary per client and model, so they must not be baked
into the file.

**Strongest evidence:** Althoubi 2026 (line anchoring corrupts 99.1%
under a one-line shift; anchor-and-verify 0.01% silent failure);
Anthropic SWE-bench (exact-one-match replace is the most reliable
primitive); JSONSchemaBench (schemas cannot carry semantics).

**Unknown:** LLM accuracy reading or writing glyph coordinates by
format; any public eval of a font MCP; whether constrained decoding
hurts edit quality; whether per-point IDs help agents more than they
cost in tokens.
