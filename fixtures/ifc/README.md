# IFC test fixtures

Sample IFC files for testing the IFC → structural-mechanics bridge.
See [../../notes/ifc-bridge.md](../../notes/ifc-bridge.md) for the design + test plan.

## What goes here

- **`cantilever.ifc`** (to author) — a minimal hand-written `IfcStructuralAnalysisModel`
  for a cantilever beam with end load. The bridge must parse it and, via
  `harness/frame-on-faer`, reproduce `δ = PL³/3EI` to machine precision.
  This is the Tier-1 closed-form fixture: no external tool needed.

- **analytical-bearing real-world IFC** (Case A) — files that actually contain an
  `IfcStructuralAnalysisModel` (exported from SCIA / RFEM / Robot / Tekla).

## Where to get real analytical-bearing samples

- buildingSMART sample test files: https://github.com/buildingSMART/Sample-Test-Files
- Vendor exports (SCIA Engineer, Dlubal RFEM, Autodesk Robot) — these populate the
  `IfcStructuralAnalysisDomain`, unlike architectural (Revit/ArchiCAD) exports which
  usually carry only the physical model (Case B — no analytical model present).

## Note

Most IFC files in the wild are architectural and contain **no** analytical model.
Verify a candidate file actually has `IfcStructuralAnalysisModel` before using it
as a Case-A fixture (grep the STEP text for `IFCSTRUCTURALANALYSISMODEL`).
