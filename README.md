# cad-structural-lab

A **validated** catalog of Rust structural-mechanics tooling for our CAD models —
graphite-overlay (2D) and ifc-ubuntu (IFC/STEP / BIM). The goal: make it easy to
run real structural mechanics off those CAD models, on a foundation we have
actually probed rather than trusted from READMEs.

The soul of this repo is the **probe harness**: for each crate, a small Rust
program that runs *known-answer* checks (closed-form solutions) against the real
published source. A crate is only `AUDITED` once its probe passes here.

## Layout

```
catalog.jsonl        registry: every crate + status + one-line verdict
harness/<name>/      a probe crate: known-answer checks vs the real source
notes/               findings, tiers, the building-frame gap analysis
.src/                vendored upstream sources (gitignored; `mise run src-get`)
.mise/tasks/         nushell tasks (catalog, probe, probe-all, src-get)
```

## Usage

```sh
mise run catalog            # show the catalog as a table
mise run probe faer         # run one probe against the real crate source
mise run probe-all          # run all pure-Rust probes (no system libs)
mise run probe gemlab       # needs C libs (see below)
mise run src-get gemlab     # vendor upstream source into .src/ for inspection
```

`probe`/`probe-all` use `cargo run`, which pulls each crate's source from
crates.io (or a git dep) and runs our checks against it — "pull the source, run
our code against it."

## Status snapshot (probed, not assumed)

| Crate | Role | Status | WASM | Notes |
|---|---|---|---|---|
| **faer** | linear-algebra engine | ✅ AUDITED | yes | 9 closed-form checks, machine precision |
| **trussx** | truss solver (axial only) | ✅ AUDITED | yes | member forces = method-of-joints |
| **frame-on-faer** | our 2D frame reference | ✅ AUDITED | yes | beam-columns that bend; in-repo |
| **gemlab** | FEM toolkit (continuum) | ✅ AUDITED | no (C libs) | patch test machine precision |
| **pmsim** | full nonlinear FEM solver | credible / unbuilt | no | Linux-x86 + MKL + MUMPS only |
| structural-shapes | section properties | adjunct | yes | helper, not probed |
| finite_element_method | educational FEM | educational | ? | learning crate |
| fe-engine / quick-fea | frame solvers | WIP | ? | 1–2★, not on crates.io |
| oxiphysics | "replaces everything" | ❌ REJECTED | ? | rigged validation gates |

## Tiers (where faer fits)

- **faer** is the engine: dense linear algebra, pure-Rust, runs on Workers/WASM.
- **trussx** does trusses (axial-only bars).
- **frame-on-faer** is our proof that the *building-frame* path (bars that bend)
  is buildable on faer — validated to machine precision against textbook beams.
- The remaining gap to "calculate loads for a building" is **engineering
  scaffolding, not hard math**: distributed loads, 3D frames, code-based load
  combinations (ASCE 7 / Eurocode), member capacity/drift checks.

See [notes/findings.md](notes/findings.md) for the full analysis.

## System libs (gemlab / pmsim only)

These wrap validated C/Fortran solvers and are not pure-Rust / not WASM:

- macOS: `brew install suite-sparse openblas lapack`
- Linux: `apt-get install libsuitesparse-dev libopenblas-dev liblapack-dev`

faer, trussx and frame-on-faer need none of this.
