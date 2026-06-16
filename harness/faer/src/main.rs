//! Known-answer probe for `faer` (dense linear algebra engine).
//! Checks LU solve, inverse, determinant, Cholesky, symmetric eigenvalues,
//! SVD, and QR least-squares against closed-form answers. Exit non-zero on FAIL.
use faer::{mat, Side};
use faer::linalg::solvers::{Solve, SolveLstsq, DenseSolveCore};

fn main() {
    let mut fails = 0;
    let mut pass = |name: &str, err: f64, tol: f64| {
        let ok = err < tol && err.is_finite();
        if !ok { fails += 1; }
        println!("  {name:<40} err={err:.3e}  tol={tol:.0e}  {}", if ok { "PASS" } else { "FAIL <<<" });
    };

    // A = [[2,1],[1,2]] (SPD): eig {1,3}; svd {3,1}; det 3; inv (1/3)[[2,-1],[-1,2]]; A[1,1]^T=[3,3].
    let a = mat![[2.0_f64, 1.0], [1.0, 2.0]];

    let mut ev = a.self_adjoint_eigenvalues(Side::Lower).unwrap();
    ev.sort_by(|x, y| x.partial_cmp(y).unwrap());
    pass("sym eigenvalues {1,3}", ((ev[0]-1.0).abs()).max((ev[1]-3.0).abs()), 1e-12);

    let mut sv = a.singular_values().unwrap();
    sv.sort_by(|x, y| y.partial_cmp(x).unwrap());
    pass("singular values {3,1}", ((sv[0]-3.0).abs()).max((sv[1]-1.0).abs()), 1e-12);

    pass("determinant = 3", (a.determinant()-3.0).abs(), 1e-12);

    let inv = a.partial_piv_lu().inverse();
    let inv_true = mat![[2.0/3.0, -1.0/3.0], [-1.0/3.0, 2.0/3.0]];
    let mut e = 0.0_f64;
    for i in 0..2 { for j in 0..2 { e = e.max((inv[(i,j)]-inv_true[(i,j)]).abs()); } }
    pass("inverse closed-form", e, 1e-12);

    let b = mat![[3.0_f64], [3.0]];
    let x = a.partial_piv_lu().solve(&b);
    let r = &a * &x - &b;
    pass("solve residual ||Ax-b||", (r[(0,0)].powi(2)+r[(1,0)].powi(2)).sqrt(), 1e-12);
    pass("solve x = [1,1]", ((x[(0,0)]-1.0).abs()).max((x[(1,0)]-1.0).abs()), 1e-12);

    let llt = a.llt(Side::Lower).unwrap();
    let l = llt.L();
    let recon = &l * l.transpose();
    let mut ec = 0.0_f64;
    for i in 0..2 { for j in 0..2 { ec = ec.max((recon[(i,j)]-a[(i,j)]).abs()); } }
    pass("Cholesky A=LL^T reconstruct", ec, 1e-12);

    // 1D Laplacian eigenvalues {2-sqrt2, 2, 2+sqrt2}
    let lap = mat![[2.0_f64,-1.0,0.0],[-1.0,2.0,-1.0],[0.0,-1.0,2.0]];
    let mut lev = lap.self_adjoint_eigenvalues(Side::Lower).unwrap();
    lev.sort_by(|x,y| x.partial_cmp(y).unwrap());
    let s2 = 2.0_f64.sqrt();
    let want = [2.0-s2, 2.0, 2.0+s2];
    let mut el = 0.0_f64;
    for k in 0..3 { el = el.max((lev[k]-want[k]).abs()); }
    pass("Laplacian-3 eigenvalues {2+-sqrt2,2}", el, 1e-12);

    // QR least-squares: y=2x+1 exact-fit -> coeffs [1,2]
    let xm = mat![[1.0_f64,0.0],[1.0,1.0],[1.0,2.0],[1.0,3.0]];
    let y = mat![[1.0_f64],[3.0],[5.0],[7.0]];
    let coef = xm.qr().solve_lstsq(&y);
    pass("QR least-squares fit [1,2]", ((coef[(0,0)]-1.0).abs()).max((coef[(1,0)]-2.0).abs()), 1e-12);

    println!("\nfaer probe: {}", if fails == 0 { "ALL PASS" } else { "FAILURES" });
    std::process::exit(if fails == 0 { 0 } else { 1 });
}
