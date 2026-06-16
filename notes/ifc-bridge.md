# IFC → structural-mechanics bridge — design & test plan

**Verdict:** realistic and standardized **if** scoped to IFC files that contain an
`IfcStructuralAnalysisModel`. IFC has a first-class structural analysis domain
(`IfcStructuralAnalysisDomain`, ISO 16739, current in IFC4.3) carrying nodes,
members, supports, loads, load cases, materials, sections and even results. The
trap is assuming an arbitrary (architectural) IFC has that analytical data — it
usually does not.

## The decisive fork

- **Case A — IFC carries `IfcStructuralAnalysisModel`** (exported from a structural
  tool: SCIA, RFEM, Robot, SAP2000, Tekla). The bridge is a **clean, mostly
  deterministic schema mapping** → our solver. **Build this first.**
- **Case B — IFC carries only the physical model** (`IfcBeam`/`IfcColumn`/`IfcSlab`
  solids, the common Revit/ArchiCAD export). No analytical model exists; you must
  **derive** it from solid geometry (axis extraction, connectivity & node
  inference, slab idealization, support guessing). This is a hard, lossy,
  semi-automated geometry problem — **a separate project, not a standards mapping.**
  Evidence: coordination/reference-view IFC yields "unconnected slabs, disconnected
  nodes, missing constraint information"; even native Revit→Robot needs "manual
  correction of analytical axes, connections, boundary conditions."

`ifc-ubuntu` (OCCT/IfcOpenShell) reads **physical geometry** → we are in **Case B
by default**. So: target Case-A IFC first; treat Case B as a later research item.

## Entity mapping (Case A) → our solver model

Target data model = what `harness/frame-on-faer` already consumes
(`nodes: [[f64;2|3]]`, `Elem{i,j,E,A,I}`, `fixed: [dof]`, `load: [(dof,val)]`).

| Our model | IFC source entity |
|---|---|
| node coordinate | topology vertex of `IfcStructuralPointConnection` / member endpoints |
| member (i, j) | `IfcStructuralCurveMember` (1D beam/column); `IfcStructuralSurfaceMember` (2D) |
| E, ν, ρ | `IfcMaterial` via `IfcMaterialProfileSet` |
| A, I (section) | `IfcProfileDef` via `IfcMaterialProfileSet` |
| support / fixed DOFs | `IfcStructuralPointConnection` + `IfcBoundaryNodeCondition` (per-DOF stiffness/fixity) |
| point load | `IfcStructuralLoadSingleForce` (via `IfcStructuralPointAction`) |
| distributed load | `IfcStructuralLoadLinearForce` / `IfcStructuralLoadPlanarForce` |
| load case / combo | `IfcStructuralLoadGroup` / `IfcStructuralLoadCase` |
| expected reactions/results (for checking) | `IfcStructuralReaction`, `IfcStructuralPointReaction` |

## Test plan (known-answer, same discipline as every other probe here)

**Tier 1 — synthetic, closed-form (do this first; fully deterministic).**
Hand-author (or generate) a minimal IFC file containing an
`IfcStructuralAnalysisModel` for a problem we already validated in
`harness/frame-on-faer`: a **cantilever beam, end load**. Then:

1. bridge parses the IFC → builds `nodes/Elem/fixed/load`
2. solve with `frame-on-faer`
3. assert tip deflection == `PL³/3EI` (machine precision)

This makes the *whole bridge* testable against a closed-form answer end-to-end,
with no dependency on any external tool. Add a 2-element simply-supported beam
(`PS³/48EI`) and a portal frame as it matures.

**Tier 2 — real-world fixtures (round-trip).**
Use public IFC samples that include analytical models. Import → solve → compare
against the results embedded in the IFC (`IfcStructuralReaction`) when present, or
against the originating tool's reported values. Flag any drift; expect MVD/version
inconsistency (IFC2x3 vs IFC4 vs IFC4.3) and validate every import.

## Open dependency: IFC parsing in Rust

Reading these entities needs an IFC/STEP toolkit. IFC is STEP (ISO 10303-21) text,
so the structural-domain entities are parseable. Options to evaluate (probe them
like everything else before trusting):
- **ifc-ubuntu / IfcOpenShell** (C++/Python) — full schema, but out-of-Rust.
- **Rust STEP parsers** (`ruststep`, `iso-10303`-family) — parse STEP; would need
  the IFC structural-domain entities navigated by hand. Maturity unverified.

Decide parser strategy as a separate catalog probe before writing the bridge.

## Where it will live (scaffold ready)

```
fixtures/ifc/            sample IFC files (analytical-bearing) + hand-authored cantilever
harness/ifc-bridge/      (future) IFC -> frame-on-faer model -> solve -> known-answer check
```

## Sources

- IfcStructuralAnalysisModel — IFC4.3: https://ifc43-docs.standards.buildingsmart.org/IFC/RELEASE/IFC4x3/HTML/lexical/IfcStructuralAnalysisModel.htm
- Interpretation of structural analytical models from the coordination view (Automation in Construction): https://www.sciencedirect.com/science/article/abs/pii/S0926580518301286
- On BIM Interoperability via the IFC Standard — structural viewpoint (MDPI Applied Sciences): https://www.mdpi.com/2076-3417/11/23/11430
- CYPE — IFC requirements for the analytical model generator: https://learning.cype.com/en/faq/ifc-requirements-for-the-analytical-model-generator/
