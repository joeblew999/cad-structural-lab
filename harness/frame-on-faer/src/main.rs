//! Reference 2D Euler-Bernoulli FRAME solver (beam-columns that BEND) on faer.
//! 3 DOF/node [u_x,u_y,theta]; element = axial + bending + coordinate transform.
//! This is the in-repo proof that the building-frame path is buildable on faer.
//! Validated to machine precision vs textbook closed-form. Exit non-zero on FAIL.
use faer::Mat;
use faer::linalg::solvers::Solve;

pub struct Elem { pub i: usize, pub j: usize, pub e: f64, pub a: f64, pub inertia: f64 }

/// 6x6 global stiffness of one frame element.
pub fn ke_global(n: &[[f64; 2]], el: &Elem) -> [[f64; 6]; 6] {
    let (dx, dy) = (n[el.j][0] - n[el.i][0], n[el.j][1] - n[el.i][1]);
    let l = (dx * dx + dy * dy).sqrt();
    let (c, s) = (dx / l, dy / l);
    let a = el.e * el.a / l;
    let ei = el.e * el.inertia;
    let (b1, b2, b3, b4) = (12.0*ei/l.powi(3), 6.0*ei/l.powi(2), 4.0*ei/l, 2.0*ei/l);
    let k = [
        [ a,  0.0, 0.0, -a,  0.0, 0.0],
        [0.0,  b1,  b2, 0.0, -b1,  b2],
        [0.0,  b2,  b3, 0.0, -b2,  b4],
        [-a, 0.0, 0.0,  a,  0.0, 0.0],
        [0.0, -b1, -b2, 0.0,  b1, -b2],
        [0.0,  b2,  b4, 0.0, -b2,  b3],
    ];
    let t = [
        [  c,   s, 0.0, 0.0, 0.0, 0.0],
        [ -s,   c, 0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0,   c,   s, 0.0],
        [0.0, 0.0, 0.0,  -s,   c, 0.0],
        [0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
    ];
    let mut kt = [[0.0; 6]; 6];
    for r in 0..6 { for cc in 0..6 { let mut v=0.0; for m in 0..6 { v += k[r][m]*t[m][cc]; } kt[r][cc]=v; } }
    let mut kg = [[0.0; 6]; 6];
    for r in 0..6 { for cc in 0..6 { let mut v=0.0; for m in 0..6 { v += t[m][r]*kt[m][cc]; } kg[r][cc]=v; } }
    kg
}

/// Assemble + solve with faer; returns full global displacement vector.
pub fn solve(nodes: &[[f64;2]], elems: &[Elem], fixed: &[usize], load: &[(usize,f64)]) -> Vec<f64> {
    let ndof = nodes.len()*3;
    let mut kk = vec![vec![0.0f64; ndof]; ndof];
    for el in elems {
        let kg = ke_global(nodes, el);
        let map = [3*el.i, 3*el.i+1, 3*el.i+2, 3*el.j, 3*el.j+1, 3*el.j+2];
        for r in 0..6 { for c in 0..6 { kk[map[r]][map[c]] += kg[r][c]; } }
    }
    let free: Vec<usize> = (0..ndof).filter(|d| !fixed.contains(d)).collect();
    let mut f = vec![0.0f64; ndof];
    for &(d,v) in load { f[d]=v; }
    let nf = free.len();
    let kff = Mat::from_fn(nf, nf, |i,j| kk[free[i]][free[j]]);
    let rhs = Mat::from_fn(nf, 1, |i,_| f[free[i]]);
    let uf = kff.partial_piv_lu().solve(&rhs);
    let mut u = vec![0.0f64; ndof];
    for (i,&d) in free.iter().enumerate() { u[d] = uf[(i,0)]; }
    u
}

fn main() {
    let mut fails = 0;
    let mut chk = |name: &str, got: f64, want: f64| {
        let err = (got-want).abs() / want.abs().max(1e-30);
        if err >= 1e-9 { fails += 1; }
        println!("  {name:<44} got={got:+.6e}  want={want:+.6e}  rel={err:.2e}  {}",
            if err < 1e-9 { "PASS" } else { "FAIL <<<" });
    };
    let (e,a,i) = (200.0e9, 0.01, 8.333333333e-6);
    let p = 1000.0;

    println!("1) horizontal cantilever (bending)");
    let u = solve(&[[0.0,0.0],[2.0,0.0]], &[Elem{i:0,j:1,e,a,inertia:i}], &[0,1,2], &[(4,-p)]);
    chk("tip deflection PL^3/3EI", u[4], -p*8.0/(3.0*e*i));
    chk("tip rotation   PL^2/2EI", u[5], -p*4.0/(2.0*e*i));

    println!("2) vertical column (coordinate transform)");
    let u = solve(&[[0.0,0.0],[0.0,2.0]], &[Elem{i:0,j:1,e,a,inertia:i}], &[0,1,2], &[(3,p)]);
    chk("tip horiz. deflection PL^3/3EI", u[3], p*8.0/(3.0*e*i));

    println!("3) simply-supported beam, central load, 2 elements (assembly)");
    let u = solve(&[[0.0,0.0],[2.0,0.0],[4.0,0.0]],
        &[Elem{i:0,j:1,e,a,inertia:i}, Elem{i:1,j:2,e,a,inertia:i}], &[0,1,7], &[(4,-p)]);
    chk("midspan deflection PS^3/48EI", u[4], -p*64.0/(48.0*e*i));

    println!("\nframe-on-faer probe: {}", if fails == 0 { "ALL PASS" } else { "FAILURES" });
    std::process::exit(if fails == 0 { 0 } else { 1 });
}
