use crate::{transformation, utils};

fn test(code: &str) {
    let code = utils::run_compiler_on_str(code, transformation::transform)
        .unwrap()
        .unwrap();
    println!("{code}");
    utils::run_compiler_on_str(&code, utils::type_check).unwrap();
    assert!(!code.contains("TYPEVAR"));
}

fn test_var(code: &str) {
    let code = utils::run_compiler_on_str(code, transformation::transform)
        .unwrap()
        .unwrap();
    println!("{code}");
    utils::run_compiler_on_str(&code, utils::type_check).unwrap();
    assert!(code.contains("TYPEVAR"));
}

fn test_none(code: &str) {
    let result = utils::run_compiler_on_str(code, transformation::transform).unwrap();
    assert!(result.is_none());
}

#[test]
fn test_binary_search() {
    test(
        "
fn binary_search(arr: (), len: (), target: ()) {
    let mut lo = 0;
    let mut hi = len - 1;
    loop {
        if lo > hi {
            return (false, 0);
        }
        let mid = lo + (hi - lo) / 2;
        let val = arr(mid);
        if val == target {
            return (true, mid);
        }
        if val < target {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    (false, 0)
}
fn my_arr(i: ()) { i * 2 }
fn foo() {
    let r = binary_search(my_arr, 10, 6);
    if r.0 { r.1 } else { -1 }
}",
    );
}

#[test]
fn test_manhattan() {
    test(
        "
fn make_point(x: (), y: ()) { (x, y) }
fn manhattan(p: (), q: ()) {
    let dx = p.0 - q.0;
    let dy = p.1 - q.1;
    let adx = if dx < 0 { -dx } else { dx };
    let ady = if dy < 0 { -dy } else { dy };
    adx + ady
}
fn closer(p: (), q: (), origin: ()) {
    let dp = &manhattan(p, origin);
    let dq = &manhattan(q, origin);
    if *dp <= *dq { p } else { q }
}
fn foo() {
    let c = closer(make_point(3, 4), make_point(1, 1), make_point(0, 0));
    c.0 + c.1
}",
    );
}

#[test]
fn test_var_identity() {
    test_var("fn foo(x: ()) { x }");
}

#[test]
fn test_fail_add_bool() {
    test_none("fn foo() { 1 + true }");
}

#[test]
fn test_fail_var_add_and_or() {
    test_none("fn foo(x: (), y: ()) { x + y; x || y; }");
}
