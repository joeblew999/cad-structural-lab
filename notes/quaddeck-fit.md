# Quad-Deck fit-check — structural SELECTION (not BOM)

Keep three concerns separate:

```
1. DEMAND     (structural mechanics on the CAD/IFC model)
   clear span, tributary area, dead+live loads, is-it-load-bearing, load path
   → "this span must carry W over L"            ← cad-structural-lab computes this

2. CAPACITY / FIT-CHECK   (Quad-Lock's engineering, in their span tables)
   → "which panel+slab has capacity >= demand?" ← `mise run fit` does this

3. BOM        (procurement, AFTER selection)
   panels, ties, metal, concrete volume, quantities  ← factory-customers, downstream
```

The span tables are **capacity data**, not BOM. They are the *interface* between the
structural demand and the product. BOM is produced only after a config is selected
and is intentionally out of scope here.

## `mise run fit <span_ft> <live_psf> [capacity_csv]`

Given a structural demand, lists every Quad-Deck config whose capacity covers it,
lightest first, with span margin. Reads `quad-deck-spans.csv` from the
`factory-customers` repo (via `$env.QUADLOCK_DATA`); pass a path to override.

```
mise run fit 18 40        # uses factory-customers data
mise run fit 18 40 /some/other/quad-deck-spans.csv
```

- Picks the smallest tabulated live-load tier that envelopes the demand (e.g. a
  35 psf demand is checked at the 40 psf tier — conservative).
- Returns the viable set sorted by panel then slab; "lightest fit" = first row.
- **No fit** (over-demand) returns a clear failure: needs a beam, intermediate
  support, or non-standard design — never a wrong answer.

## What it deliberately does NOT do

- It does **not compute the demand** — that is the structural-mechanics / load-takedown
  job (see `harness/frame-on-faer` for the bending solver, available as an independent
  deflection cross-check on the demand side).
- It does **not produce a BOM.**
- It is **estimating/selection only.** Final design requires a licensed PE
  (Quad-Lock requires a sign waiver). Capacity values are Quad-Lock's, baked at
  4000 psi with 2-#7 bottom / 1-#6 top rebar — see the capacity CSV's README.

## Caveats baked into the current capacity table

- Live-load tiers are discrete (20 / 40 / 100 psf). Superimposed dead load (finishes,
  partitions) is **not** separately in the table — the published spans assume only the
  Quad-Deck self-weight as dead load. Account for superimposed dead on the demand side.
- `margin_ft = 0` means the config is exactly at the tabulated limit — prefer some
  margin in real selection.
- Only Regular Floors & Roofs is loaded. Garage floors and green roofs are separate
  tables (not yet transcribed).

## Roadmap

- Demand side: extract `(span, live load)` from graphite-2D / IFC
  `IfcStructuralAnalysisModel`; add superimposed dead load.
- Join `quad-deck-properties.csv` to report floor self-weight (the dead load the
  supporting walls/columns must then carry) and fire rating alongside each fit.
