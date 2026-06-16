# Findings — Rust structural mechanics for CAD/IFC

Probed 2026-06-15/16. Every claim below was checked by cloning the source and
running known-answer (closed-form) validation, not taken from a README.

## The engine: faer — AUDITED

Pure-Rust dense linear algebra (LU/QR/Cholesky/SVD/eigen/lstsq). Ran 9 closed-form
checks (symmetric eigenvalues, SVD, determinant, inverse, solve, Cholesky,
Laplacian spectrum, QR least-squares) → all machine precision (~1e-16).

- **WASM/Workers gotcha:** default features include `rayon` → `spindle` →
  `atomic-wait`, which has no `wasm32` impl, so stock `faer = "0.24"` fails to
  build for Workers. Fix: `default-features = false, features = ["std","linalg"]`.
  Verified by building a 214 KB `.wasm` with a working LU solve.

## Trusses: trussx — AUDITED

3D pin-jointed truss FEA, pure-Rust (nalgebra + petgraph), runs on WASM.
Member forces in a statically-determinate 2-bar truss matched a method-of-joints
hand calc to machine precision (F = P√2 tension, −P compression), tension-positive.
Only 2★ / single author, but the math is correct. **Axial only — no bending.**

## The building-frame question

A truss = bars that only push/pull. A building frame = beams/columns that **bend**
at rigid connections, plus code-based loads and design checks — a different
discipline. The Rust ecosystem has **no mature frame/structural package**:

- `fe-engine`, `quick-fea` — aim at frames (load cases, material codes) but are
  1–2★ WIP, not on crates.io, GPU solver "not validated", no real tests.
- `finite_element_method` — author-declared educational; no load cases.
- `gemlab`/`pmsim` — continuum solid mechanics, not frames; C-lib/container bound.

### We proved the frame path is buildable on faer

`harness/frame-on-faer` is a ~80-line 2D Euler-Bernoulli frame solver (3 DOF/node:
ux, uy, θ; axial + bending + coordinate transform) built on faer. Validated to
machine precision vs textbook closed-form:

- horizontal cantilever: δ = PL³/3EI, θ = PL²/2EI
- vertical column (tests the 90° coordinate transform): δ = PL³/3EI
- simply-supported beam, central load, 2 elements (tests assembly): δ = PS³/48EI

Note: this is the exact cantilever case `oxiphysics` got **57% wrong and stamped
PASS**. A correct beam element nails it. So the hard part (the solver/element math)
is real and buildable on faer.

## What's left to "calculate loads for a building"

Not hard math — engineering scaffolding:

1. **Distributed member loads** (uniform/triangular → consistent nodal loads).
2. **3D frames** (12-DOF elements, torsion, biaxial bending).
3. **Code-based load generation + combinations** (ASCE 7 / Eurocode: dead, live,
   wind, snow, seismic; 1.2D+1.6L etc.). This is the big one and is mostly rules,
   not algebra — strong candidate for wrapping an established source.
4. **Member design checks** (capacity, drift, deflection limits).
5. **CAD bridge:** extract structural model (nodes/members/sections/supports/loads)
   from graphite-2D geometry and from IFC/STEP (ifc-ubuntu). IFC carries
   `IfcStructuralAnalysisModel` — a natural import target.

## Rejected

`oxiphysics` (cool-japan): claims to replace Bullet+OpenFOAM+LAMMPS+CalculiX in
pure Rust. Validation gates are rigged — FEM cantilever 57% off → "PASS"; MD total
energy 0 → 21.7 kJ/mol → "energy conserved". "60,115 tests" carry near-zero
validation signal. Do not use.
