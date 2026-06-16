# Wall capacity — prescriptive ICF reinforcement (code-pluggable)

The wall analog to the floor span table. Walls have **no proprietary Quad-Lock
table** — capacity is in the building code. This is the prescriptive (table-lookup)
path; engineered (ACI 318 calc) is a separate future path for out-of-limit walls.

## Source & data location

All DATA lives in the **`factory-customers`** repo (this `cad-structural-lab` repo is
just the tooling). The wall code tables are at
`factory-customers/customers/quadlock/catalogue/`:
`wall-capacity-prescriptive-icf.csv` (basement), `-abovegrade.csv`, `wall-wind-pressure-icf.csv`.
The tooling finds them via `$env.QUADLOCK_DATA` (set in `mise.toml`, defaults to the
sibling checkout).

Transcribed from the **HUD/PCA/NAHB "Prescriptive Method for Insulating Concrete Forms
in Residential Construction, Second Edition"** (public domain) — the document IRC R404
derives from. URL: https://www.huduser.gov/publications/pdf/icf_2ed.pdf

**Full provenance + legal status is in `factory-customers/.../catalogue/code-tables-SOURCES.md`**
(the `code` column is the provenance key). Public-domain HUD source chosen deliberately
over the ICC-copyrighted IRC. Not legal advice; verify against the governing local code.

- **Table 3.4** (5.5″ flat ICF basement walls) — TRANSCRIBED & VALIDATED.
  Self-check: the doc's own Appendix-A worked example (6″ wall, 8 ft, 6 ft backfill,
  30 pcf → `#3@12; #4@22; #5@30; #6@40`) is reproduced exactly by `fit-wall`.
- TODO (same source, same schema): Table 3.5 (7.5″), 3.6 (9.5″) basement; Table 4.2
  flat above-grade (+ 4.1 wind pressure); 3.2 crawlspace; 3.3 horizontal.

## Schema

`code, application, wall_thickness_in, max_wall_height_ft, max_backfill_ft, soil_pcf,
steel_grade, rebar_options` — `code` makes it pluggable (add NBC/PCA-100 rows under
new codes). `rebar_options` = `;`-separated equivalent choices (`#4@22` = No.4 @ 22″ oc);
pick one bar size.

## Rules baked into `fit-wall`

`mise run fit-wall <core_in> <height_ft> <backfill_ft> [soil_pcf] [csv]`

- **soil_pcf** = equivalent fluid density: **30** (well-drained GW/GP/SW/SP), **45**
  (GM/GC/SM/ML), **60** (SC/MH/CL clays). Default 60 = worst case.
- **No interpolation** (Table footnote 4): demand is rounded **UP** to the next
  tabulated wall height / backfill height, and to the next-higher soil class.
- **Steel grade:** table values are **Grade 40**; with **Grade 60** the spacing may be
  ×1.5 (min #4 @ 48″). `fit-wall` reports the Grade-40 value + this note.
- Over-limit (height/backfill beyond the table) → refuses, points to engineered design.
- Core thinner than 5.5″ → not a prescriptive basement case (too thin).

## Quad-Lock core ↔ prescriptive thickness

`fit-wall` uses the nearest tabulated thickness **not greater** than the actual core
(conservative). Quad-Lock cores map: 5.75″/6″ → 5.5″ table; 7.75″/8″ → 7.5″;
9.75″/10″ → 9.5″. The required core then maps to a Quad-Lock tie+panel via
`factory-customers/.../catalogue/quad-lock-wall-cores.csv` (BOM side).

## Jurisdiction reality

Prescriptive ICF wall tables exist for **US (IRC/this Prescriptive Method) and Canada
(NBC) / PCA 100**. **NZ/AU (NZS 3101 / AS 3600) and Eurocode 2 have no equivalent
prescriptive ICF table** — those require engineered design. So "as many codes as
possible" prescriptively = the North American set; other jurisdictions need the
engineered path.

## Caveat

Estimating/selection only; final design requires a licensed PE. Demand side (wall
height, unbalanced backfill, soil class, axial, wind/seismic) comes from the building
+ site, not just geometry.
