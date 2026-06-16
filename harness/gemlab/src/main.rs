//! Known-answer probe for `gemlab` (FEM toolkit) + faer.
//! 1) 2nd moment of area of a disk/ring by FE integration vs analytical.
//! 2) elastic patch test: 2 CST triangles, uniaxial tension solved with faer,
//!    displacements vs exact elasticity (eps_xx=sigma/E). Exit non-zero on FAIL.
//! Requires C system libs (russell -> suite-sparse/openblas/lapack).
use gemlab::integ::{scalar_field, AnalyticalTri3, Gauss};
use gemlab::prelude::*;
use gemlab::recovery::get_points_coords;
use gemlab::shapes::{GeoKind, Scratchpad};
use faer::Mat;
use faer::linalg::solvers::Solve;
use russell_lab::math::PI;

const E: f64 = 200.0e9;
const NU: f64 = 0.3;
const TH: f64 = 1.0;
const SIGMA: f64 = 1.0e6;

fn ana_u(x: f64, y: f64) -> (f64, f64) { let c = SIGMA / E; (c*x, -NU*c*y) }

fn main() -> Result<(), StrError> {
    let mut fails = 0;

    // --- 1) disk second moment of area: pi r^4 / 4 ---
    let r = 3.0;
    let kind = GeoKind::Qua17;
    let mesh = Structured::quarter_disk_2d_a(r, 3, 3, kind, false)?;
    let gauss = Gauss::new(kind);
    let mut pad = Scratchpad::new(2, kind)?;
    let mut second = 0.0;
    for cell in &mesh.cells {
        mesh.set_pad(&mut pad, &cell.points);
        let xips = get_points_coords(&mut pad, &gauss)?;
        second += scalar_field(&mut pad, &gauss, |p| { let y = xips[p][1]; Ok(y*y) })?;
    }
    second *= 4.0;
    let want = r.powi(4) * PI / 4.0;
    let e1 = (second - want).abs();
    if e1 >= 1e-4 { fails += 1; }
    println!("  disk 2nd-moment pi r^4/4   got={second:.6}  want={want:.6}  err={e1:.2e}  {}",
        if e1 < 1e-4 { "PASS" } else { "FAIL <<<" });

    // --- 2) elastic patch test: 2 CST triangles, uniaxial tension, faer solve ---
    let nodes = [[0.0,0.0],[1.0,0.0],[1.0,1.0],[0.0,1.0]];
    let tris = [[0usize,1,2],[0usize,2,3]];
    let mut k = [[0.0f64;8];8];
    for t in &tris {
        let mut p = Scratchpad::new(2, GeoKind::Tri3)?;
        for (li,&gn) in t.iter().enumerate() { p.set_xx(li,0,nodes[gn][0]); p.set_xx(li,1,nodes[gn][1]); }
        let ke = AnalyticalTri3::new(&p).mat_10_bdb(E, NU, true, TH)?;
        for li in 0..6 { let gi = 2*t[li/2]+(li%2);
            for lj in 0..6 { let gj = 2*t[lj/2]+(lj%2); k[gi][gj] += ke.get(li,lj); } }
    }
    let pres = [0usize,1,6];               // node0{x,y}=0, node3 x=0
    let free = [2usize,3,4,5,7];
    let mut force = [0.0f64;8];
    force[2] = SIGMA/2.0; force[4] = SIGMA/2.0;   // consistent edge traction
    let nf = free.len();
    let kff = Mat::from_fn(nf,nf,|i,j| k[free[i]][free[j]]);
    let rhs = Mat::from_fn(nf,1,|i,_| force[free[i]]);
    let uf = kff.partial_piv_lu().solve(&rhs);
    let mut u = [0.0f64;8];
    for (i,&d) in free.iter().enumerate() { u[d] = uf[(i,0)]; }
    let _ = pres;
    let mut maxe = 0.0f64;
    for n in 0..4 { let (ax,ay)=ana_u(nodes[n][0],nodes[n][1]);
        maxe = maxe.max((u[2*n]-ax).abs()).max((u[2*n+1]-ay).abs()); }
    if maxe >= 1e-18 { fails += 1; }
    println!("  elastic patch test          max_err={maxe:.2e}  (eps_xx={:.3e}=sigma/E)  {}",
        u[4], if maxe < 1e-18 { "PASS" } else { "FAIL <<<" });

    println!("\ngemlab probe: {}", if fails == 0 { "ALL PASS" } else { "FAILURES" });
    std::process::exit(if fails == 0 { 0 } else { 1 });
}
