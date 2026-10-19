# Eli's method for AI vector work, and what the format owes it

Date: 2026-10-19. Source: elih.net posts "Virtua Grotesk: Grid Systems
for Dataset Engineering" (2026-07-20), "img2bez" (2026-06-24),
"NeuralType" (2026-07-29), "Nasta'liq Distilled" (2026-07-30).

The headless survey found "AI fails at vector data" as the field's
consensus. That is the consensus for unconstrained outlines handed to
a general model. Eli's work takes a different route and reports
results. The format should not be built around this method, but it
must not get in its way, and a few cheap choices make it fit.

## The method, in short

- **Dyadic self-labeling grid.** 1024 UPM. Structure points on an
  8-unit grid, optical corrections on a 2-unit subgrid. Grid
  membership records intent, so the sources are labeled training
  data with no annotation step. Human-drawn Virtua sits on the 8-unit
  grid 85% of the time; random coordinates would 6%.
- **Lossless tokenization.** Each legal grid coordinate is one token.
  Commands are MOVE, LINE, CURVE, CLOSE. A glyph is about 90 tokens.
  The codec is lossless when every coordinate is on the grid.
- **Small bounded moves, not whole glyphs.** Virtua-12M predicts a
  per-point delta from Regular to Bold. Output is a draft a person
  corrects on the 2-unit grid. IoU 0.745 against a 0.564 no-model
  baseline, 10 held-out glyphs, 5 seeds.
- **A QA loop the harness can run.** `make grid-qa` reports grid
  violations and popcount labels; the AI harness fixes, renders,
  reruns.
- **Rasters traced into structured outlines.** img2bez places points
  at extrema, corners, inflections so masters stay compatible, with
  small compiled models deciding, not drawing.
- **Neural glyphs predict fields, not curves.** NeuralType models emit
  an SDF; a tracer makes the outline, so a small error shifts an edge
  instead of breaking the path. Direct outline-token prediction failed
  in those runs.

Stated plan: AI vector work solved by the end of 2026.

## What this asks of lilufo

1. **Integer coordinates, declared grid.** Coordinates are integers
   by default. A font may declare its drawing grid (UPM, structure
   step, correction step) in font metadata. `check` can then report
   grid compliance and popcount labels. This is the first piece of
   "AI-era" metadata that has an existing consumer.
2. **Rows that tokenize.** One row per point with integer x, y, and
   type is already the token stream Virtua-12M reads. The
   topology/geometry split makes a model's output (a per-point delta
   table for one source) exactly one geometry table diff.
3. **Compatibility by construction helps training.** Regular-Bold
   pairs with shared point IDs are the training pairs. Today those
   pairs are recovered by matching; with shared IDs they are given.
4. **The neural representation slot has a concrete shape.** A source
   whose geometry is `{model: assets/<hash>.ntf, inputs: {...}}` plus
   a cached outline traced by img2bez. The cache is derived, so it
   lives outside the canonical source or is marked derived inside it.
5. **The QA contract is the headless loop.** `lilufo check` with grid
   rules, `lilufo render`, and a receipt from `lilufo edit` are the
   three calls the harness already makes in another shape.

## What it does not ask

- No grid requirement in the core. Fonts on 1000 UPM with arbitrary
  coordinates are first-class. The grid is declared, optional, and
  checked only when declared.
- No model dependency to open or validate a package.
- No assumption that agents edit through tokens. The text form stays
  the interface; the token codec is a tool on top.
