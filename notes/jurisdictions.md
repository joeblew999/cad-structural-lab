# Jurisdiction roadmap — which ones we can actually do

The wall fit-check is prescriptive (table-lookup). **Prescriptive ICF wall tables
only exist in North America** (US IRC / Canada NBC, via the HUD/PCA Prescriptive
Method). Everywhere else is **engineered-only** — a different, bigger build (ACI 318 /
Eurocode calc engine). So "going global" splits cleanly into two tiers.

## US — the jurisdiction we have done (`us-`)

**Core DONE** (the two governing demand cases):
- Basement vertical reinforcement — flat ICF 5.5/7.5/9.5″ (Tables 3.4/3.5/3.6). Validated.
- Above-grade vertical reinforcement — flat ICF, wind-driven (Tables 4.1 + 4.2).

**Completeness backlog** (same public-domain HUD source, flat ICF, quick adds):
- Table 3.1 footings · 3.2 crawlspace walls · 3.3 horizontal basement reinf · 5.x lintels.
- N/A to Quad-Lock: waffle-grid / screen-grid tables (3.7–3.9, 4.3–4.4) — Quad-Lock is a
  **flat** ICF system, so those never apply.

US is usable now for the load-bearing cases; the backlog rounds it out.

## Feasibility of other jurisdictions

| Jurisdiction | Wall design regime | Prescriptive table? | Data source | Local Quad-Lock eval | Verdict |
|---|---|---|---|---|---|
| **US** | IRC R404/R608 | ✅ yes | HUD Prescriptive Method (public domain) | CCRR-1060 | **DONE** |
| **Canada** | NBC 9.15 + CSA A23.3 | ✅ yes (NBC Part 9) | **NBC — free PDF from NRC**; same method | (CCRR is dual US/Canada) | **DO NEXT** |
| **US high-wind** (FL/HVHZ) | FBC + Miami-Dade NOA | product-specific | **Miami-Dade NOA (held locally)** | NOA ✓ | feasible — extends US above-grade past 80 psf |
| **Caribbean** (PR/Cayman/Jamaica/Bahamas) | mostly IBC-based + local | reuse US + high-wind | local approvals (material) | several ✓ | mostly covered by US + high-wind |
| **Europe** | Eurocode 2 (EN 1992) | ❌ engineered | ETA-06-0189 (product CE basis, not a table) | ETA ✓ | needs the **engineered** path |
| **South Africa** | SANS 10100 | ❌ engineered | SABS report (material test) | SABS ✓ | engineered path |
| **NZ / AU** | NZS 3101 / AS 3600 | ❌ engineered | standards **paywalled**; no local eval | none | engineered + hardest to source |

## What we can do, in order

1. **US** ✅ (done; optional completeness backlog).
2. **Canada** — the clear next: prescriptive tables exist, **NBC is free from NRC**, same
   methodology, and the CCRR we already read is dual US/Canada. Adds `ca-` tables; mostly
   metric conversion + NBC 9.15 foundation specifics.
3. **US high-wind extension** — transcribe the **Miami-Dade NOA** (we hold it) to push
   above-grade past the 80 psf prescriptive cap, covering FL + hurricane Caribbean.
4. **Europe / South Africa / NZ / AU** — **engineered only.** No prescriptive table to
   transcribe; these require the deferred ACI/Eurocode **calculation** engine (out-of-plane
   bending + axial interaction). The ETA/SABS we hold give product parameters, not lookups.
   NZ/AU additionally need paywalled standards.

## The strategic line

- **Prescriptive system tops out at North America + high-wind.** That's US (done) +
  Canada + the Miami-Dade extension — all with accessible/free data.
- **Truly global = the engineered path.** That's a separate, larger project (the wall
  analog to `frame-on-faer`: a code-driven calc), and for NZ/AU it also means buying
  standards. Worth doing only when a real project in those jurisdictions demands it.

**Recommendation:** finish US backlog if you want US airtight, then do **Canada** (cheap,
free data, same method). Treat Europe/NZ/AU as the engineered-path milestone, not a
table-transcription task.
