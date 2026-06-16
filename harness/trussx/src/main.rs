//! Known-answer probe for `trussx` (3D pin-jointed truss FEA).
//! 1) single bar: u = PL/(AE).  2) statically-determinate 2-bar truss: member
//! forces by method of joints (E,A-independent statics). Exit non-zero on FAIL.
use trussx::{force, point, Truss};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut fails = 0;
    let mut chk = |name: &str, got: f64, want: f64, tol: f64| {
        let err = (got - want).abs();
        if err >= tol { fails += 1; }
        println!("  {name:<34} got={got:+.4e}  want={want:+.4e}  err={err:.2e}  {}",
            if err < tol { "PASS" } else { "FAIL <<<" });
    };

    // Case 1: single bar in tension
    println!("1) single bar, analytical u = P L /(A E)");
    let mut t = Truss::new();
    let a = t.add_joint(point(0.0, 0.0, 0.0));
    let b = t.add_joint(point(1.0, 0.0, 0.0));
    t.set_support(a, [true, true, true])?;
    t.set_support(b, [false, true, true])?;
    t.set_load(b, force(1000.0, 0.0, 0.0))?;
    let ab = t.add_member(a, b);
    t.set_member_properties(ab, 0.01, 200.0e9)?;
    t.evaluate()?;
    chk("u_x at B (PL/AE)", t.joint_displacement(b).unwrap().x, 5.0e-7, 1e-12);
    chk("axial force (=P)", t.member_axial_force(ab).unwrap().abs(), 1000.0, 1e-6);

    // Case 2: 2-bar determinate truss; statics -> F_02=P*sqrt2 (T), F_12=-P (C)
    println!("2) 2-bar determinate truss, method of joints");
    let mut t2 = Truss::new();
    let n0 = t2.add_joint(point(0.0, 0.0, 0.0));
    let n1 = t2.add_joint(point(1.0, 0.0, 0.0));
    let n2 = t2.add_joint(point(1.0, 1.0, 0.0));
    t2.set_support(n0, [true, true, true])?;
    t2.set_support(n1, [true, true, true])?;
    t2.set_support(n2, [false, false, true])?;
    t2.set_load(n2, force(1000.0, 0.0, 0.0))?;
    let m02 = t2.add_member(n0, n2);
    let m12 = t2.add_member(n1, n2);
    t2.set_member_properties(m02, 0.01, 200.0e9)?;
    t2.set_member_properties(m12, 0.01, 200.0e9)?;
    t2.evaluate()?;
    let f02 = t2.member_axial_force(m02).unwrap();
    let f12 = t2.member_axial_force(m12).unwrap();
    chk("|F_02| (=P*sqrt2)", f02.abs(), 1000.0 * 2.0_f64.sqrt(), 1e-3);
    chk("|F_12| (=P)", f12.abs(), 1000.0, 1e-3);
    if f02 * f12 >= 0.0 { fails += 1; }
    println!("  signs F_02={f02:+.2} F_12={f12:+.2} -> opposite (diag T / vert C)? {}",
        if f02 * f12 < 0.0 { "YES PASS" } else { "NO FAIL <<<" });

    println!("\ntrussx probe: {}", if fails == 0 { "ALL PASS" } else { "FAILURES" });
    std::process::exit(if fails == 0 { 0 } else { 1 });
}
