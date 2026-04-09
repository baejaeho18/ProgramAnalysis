use std::collections::HashMap;

use Interval::Range;
use rustc_middle::{
    mir::{BasicBlock, Local, Location},
    ty::TyCtxt,
};

use crate::{analysis, domains::*, utils};

const BOT: Interval = Interval::Bot;
const TOP: Interval = Range(None, None);

fn print_mir_with_res(res: &HashMap<Location, AbsState>, tcx: TyCtxt<'_>) {
    for def_id in tcx.hir_body_owners() {
        if tcx.item_name(def_id.to_def_id()).as_str() == "f" {
            let body = tcx.optimized_mir(def_id);
            for (bb, bbd) in body.basic_blocks.iter_enumerated() {
                println!("{bb:?}");
                for (i, stmt) in bbd.statements.iter().enumerate() {
                    let location = Location {
                        block: bb,
                        statement_index: i,
                    };
                    println!("  // {:?}", res.get(&location));
                    println!("  {i}: {stmt:?}");
                }
                let location = Location {
                    block: bb,
                    statement_index: bbd.statements.len(),
                };
                println!("  // {:?}", res.get(&location));
                println!("  {}: {:?}", bbd.statements.len(), bbd.terminator().kind);
            }
        }
    }
}

fn embed_mir(params: &str, body: &str) -> String {
    format!(
        r#"
        #![feature(core_intrinsics, custom_mir)]
        #![allow(internal_features)]
        use core::intrinsics::mir::*;
        #[custom_mir(dialect = "runtime", phase = "optimized")]
        fn f({params}) -> i32 {{
            mir! {{ {body} }}
        }}
        "#,
    )
}

fn test(params: &str, body: &str, expected: &[(usize, usize, &[(usize, Interval)])]) {
    let code = embed_mir(params, body);
    utils::run_compiler_on_str(&code, |tcx| {
        let res = analysis::analyze(tcx);
        print_mir_with_res(&res, tcx);
        for (bb, stmt_idx, intervals) in expected {
            let location = Location {
                block: BasicBlock::from_usize(*bb),
                statement_index: *stmt_idx,
            };
            let state = res.get(&location);
            for (local, interval) in *intervals {
                let expected = state
                    .and_then(|state| state.0.get(&Local::from_usize(*local)))
                    .unwrap_or(&BOT);
                assert_eq!(interval, expected, "bb{bb}:{stmt_idx} _{local}");
            }
        }
    })
    .unwrap();
}

#[test]
fn test_simple() {
    let params = "";
    let body = "
        {
            RET = 1 + 2;
            RET = RET + 1;
            RET = RET + RET;
            Return()
        }
    ";
    let expected: &[(usize, usize, &[(usize, Interval)])] = &[
        (0, 0, &[(0, BOT)]),
        (0, 1, &[(0, Range(Some(3), Some(3)))]),
        (0, 2, &[(0, Range(Some(4), Some(4)))]),
        (0, 3, &[(0, Range(Some(8), Some(8)))]),
    ];
    test(params, body, expected);
}

#[test]
fn test_lit() {
    let params = "";
    let body = "
        {
            RET = 0;
            Return()
        }
    ";
    let expected: &[(usize, usize, &[(usize, Interval)])] =
        &[(0, 0, &[(0, BOT)]), (0, 1, &[(0, Range(Some(0), Some(0)))])];
    test(params, body, expected);
}

#[test]
fn test_params() {
    let params = "x: i32, y: i32";
    let body = "
        {
            Return()
        }
    ";
    let expected: &[(usize, usize, &[(usize, Interval)])] = &[(0, 0, &[(1, TOP), (2, TOP)])];
    test(params, body, expected);
}

#[test]
fn test_join() {
    let params = "x: i32";
    let body = "
        let b;
        {
            b = x == 0;
            match b {
                true => bb1,
                _ => bb2,
            }
        }
        bb1 = {
            RET = 0;
            Goto(bb3)
        }
        bb2 = {
            RET = 10;
            Goto(bb3)
        }
        bb3 = {
            Return()
        }
    ";
    let expected: &[(usize, usize, &[(usize, Interval)])] = &[
        (1, 1, &[(0, Range(Some(0), Some(0)))]),
        (2, 1, &[(0, Range(Some(10), Some(10)))]),
        (3, 0, &[(0, Range(Some(0), Some(10)))]),
    ];
    test(params, body, expected);
}

#[test]
fn test_lt() {
    let params = "x: i32";
    let body = "
        let y;
        let b1;
        let b2;
        {
            b1 = x == 0;
            match b1 {
                true => bb2,
                _ => bb1,
            }
        }
        bb1 = {
            Return()
        }
        bb2 = {
            y = 0;
            Goto(bb4)
        }
        bb3 = {
            y = 10;
            Goto(bb4)
        }
        bb4 = {
            b2 = y < 5;
            match b2 {
                true => bb5,
                _ => bb6,
            }
        }
        bb5 = {
            Goto(bb1)
        }
        bb6 = {
            Goto(bb1)
        }
    ";
    let expected: &[(usize, usize, &[(usize, Interval)])] = &[
        (5, 0, &[(2, Range(Some(0), Some(4)))]),
        (6, 0, &[(2, Range(Some(5), Some(10)))]),
    ];
    test(params, body, expected);
}

#[test]
fn test_add() {
    let params = "x: i32, y: i32";
    let body = "
        let b1;
        let b2;
        let b3;
        let b4;
        {
            RET = 0;
            b1 = x <= 7;
            match b1 {
                true => bb2,
                _ => bb1,
            }
        }
        bb1 = {
            Return()
        }
        bb2 = {
            b2 = x >= 2;
            match b2 {
                true => bb3,
                _ => bb1,
            }
        }
        bb3 = {
            b3 = y <= 8;
            match b3 {
                true => bb4,
                _ => bb1,
            }
        }
        bb4 = {
            b4 = y >= 5;
            match b4 {
                true => bb5,
                _ => bb1,
            }
        }
        bb5 = {
            RET = x + y;
            Goto(bb1)
        }
    ";
    let expected: &[(usize, usize, &[(usize, Interval)])] = &[
        (
            5,
            0,
            &[(1, Range(Some(2), Some(7))), (2, Range(Some(5), Some(8)))],
        ),
        (5, 1, &[(0, Range(Some(7), Some(15)))]),
    ];
    test(params, body, expected);
}

#[test]
fn test_loop_widening() {
    let params = "y: i32";
    let body = "
        let x;
        let b;
        {
            x = 0;
            Goto(bb1)
        }
        bb1 = {
            b = y <= 10;
            match b {
                true => bb2,
                _ => bb3,
            }
        }
        bb2 = {
            x = x + 1;
            Goto(bb1)
        }
        bb3 = {
            Return()
        }
    ";
    let expected: &[(usize, usize, &[(usize, Interval)])] = &[
        (0, 0, &[(1, TOP), (2, BOT)]),
        (0, 1, &[(1, TOP), (2, Range(Some(0), Some(0)))]),
        (1, 0, &[(1, TOP), (2, Range(Some(0), None))]),
        (
            2,
            0,
            &[(1, Range(None, Some(10))), (2, Range(Some(0), None))],
        ),
        (
            2,
            1,
            &[(1, Range(None, Some(10))), (2, Range(Some(1), None))],
        ),
        (
            3,
            0,
            &[(1, Range(Some(11), None)), (2, Range(Some(0), None))],
        ),
    ];
    test(params, body, expected);
}

#[test]
fn test_loop_widening_and_narrowing() {
    let params = "";
    let body = "
        let x;
        let b;
        {
            x = 0;
            Goto(bb1)
        }
        bb1 = {
            b = x <= 10;
            match b {
                true => bb2,
                _ => bb3,
            }
        }
        bb2 = {
            x = x + 1;
            Goto(bb1)
        }
        bb3 = {
            Return()
        }
    ";
    let expected: &[(usize, usize, &[(usize, Interval)])] = &[
        (0, 0, &[(1, BOT)]),
        (0, 1, &[(1, Range(Some(0), Some(0)))]),
        (1, 0, &[(1, Range(Some(0), Some(11)))]),
        (2, 0, &[(1, Range(Some(0), Some(10)))]),
        (2, 1, &[(1, Range(Some(1), Some(11)))]),
        (3, 0, &[(1, Range(Some(11), Some(11)))]),
    ];
    test(params, body, expected);
}

// =====================================================================
// Additional test cases from lecture slides and edge cases
// =====================================================================

/// Lecture slide 10: Branch with constants, join at merge point
/// if input() { x = 0 } else { x = 2 }; y = x + 2
#[test]
fn test_lecture_branch_join() {
    let params = "c: i32";
    let body = "
        let x;
        let y;
        let b;
        {
            b = c == 0;
            match b {
                true => bb1,
                _ => bb2,
            }
        }
        bb1 = {
            x = 0;
            Goto(bb3)
        }
        bb2 = {
            x = 2;
            Goto(bb3)
        }
        bb3 = {
            y = x + 2;
            Return()
        }
    ";
    let expected: &[(usize, usize, &[(usize, Interval)])] = &[
        (1, 1, &[(2, Range(Some(0), Some(0)))]),
        (2, 1, &[(2, Range(Some(2), Some(2)))]),
        // After join: x = [0, 2]
        (3, 0, &[(2, Range(Some(0), Some(2)))]),
        // y = x + 2 = [0,2] + [2,2] = [2, 4]
        (3, 1, &[(3, Range(Some(2), Some(4)))]),
    ];
    test(params, body, expected);
}

/// Subtraction: x - y with constrained ranges
#[test]
fn test_subtraction() {
    let params = "x: i32, y: i32";
    let body = "
        let b1;
        let b2;
        let b3;
        let b4;
        {
            b1 = x <= 8;
            match b1 {
                true => bb2,
                _ => bb1,
            }
        }
        bb1 = {
            Return()
        }
        bb2 = {
            b2 = x >= 5;
            match b2 {
                true => bb3,
                _ => bb1,
            }
        }
        bb3 = {
            b3 = y <= 3;
            match b3 {
                true => bb4,
                _ => bb1,
            }
        }
        bb4 = {
            b4 = y >= 1;
            match b4 {
                true => bb5,
                _ => bb1,
            }
        }
        bb5 = {
            RET = x - y;
            Goto(bb1)
        }
    ";
    let expected: &[(usize, usize, &[(usize, Interval)])] = &[
        // x=[5,8], y=[1,3]
        (
            5,
            0,
            &[(1, Range(Some(5), Some(8))), (2, Range(Some(1), Some(3)))],
        ),
        // x - y = [5-3, 8-1] = [2, 7]
        (5, 1, &[(0, Range(Some(2), Some(7)))]),
    ];
    test(params, body, expected);
}

/// Multiplication with positive ranges
#[test]
fn test_multiplication() {
    let params = "x: i32, y: i32";
    let body = "
        let b1;
        let b2;
        let b3;
        let b4;
        {
            b1 = x <= 4;
            match b1 {
                true => bb2,
                _ => bb1,
            }
        }
        bb1 = {
            Return()
        }
        bb2 = {
            b2 = x >= 2;
            match b2 {
                true => bb3,
                _ => bb1,
            }
        }
        bb3 = {
            b3 = y <= 5;
            match b3 {
                true => bb4,
                _ => bb1,
            }
        }
        bb4 = {
            b4 = y >= 3;
            match b4 {
                true => bb5,
                _ => bb1,
            }
        }
        bb5 = {
            RET = x * y;
            Goto(bb1)
        }
    ";
    let expected: &[(usize, usize, &[(usize, Interval)])] = &[
        // x=[2,4], y=[3,5]
        // x * y: corners = 2*3=6, 2*5=10, 4*3=12, 4*5=20 → [6, 20]
        (5, 1, &[(0, Range(Some(6), Some(20)))]),
    ];
    test(params, body, expected);
}

/// Loop with decrement and lower bound narrowing
/// x = 100; while x >= 0 { x = x - 1 }
#[test]
fn test_loop_decrement_narrowing() {
    let params = "";
    let body = "
        let x;
        let b;
        {
            x = 100;
            Goto(bb1)
        }
        bb1 = {
            b = x >= 0;
            match b {
                true => bb2,
                _ => bb3,
            }
        }
        bb2 = {
            x = x - 1;
            Goto(bb1)
        }
        bb3 = {
            Return()
        }
    ";
    let expected: &[(usize, usize, &[(usize, Interval)])] = &[
        // After widening + narrowing: loop head x = [-1, 100]
        (1, 0, &[(1, Range(Some(-1), Some(100)))]),
        // True branch: x >= 0 → x = [0, 100]
        (2, 0, &[(1, Range(Some(0), Some(100)))]),
        // After x = x - 1: [-1, 99]
        (2, 1, &[(1, Range(Some(-1), Some(99)))]),
        // False branch: x < 0 → x = [-1, -1]
        (3, 0, &[(1, Range(Some(-1), Some(-1)))]),
    ];
    test(params, body, expected);
}

/// Nested filtering: chain of conditions narrowing the same variable
#[test]
fn test_nested_filtering() {
    let params = "x: i32";
    let body = "
        let b1;
        let b2;
        {
            b1 = x >= 0;
            match b1 {
                true => bb1,
                _ => bb3,
            }
        }
        bb1 = {
            b2 = x <= 100;
            match b2 {
                true => bb2,
                _ => bb3,
            }
        }
        bb2 = {
            RET = x;
            Goto(bb3)
        }
        bb3 = {
            Return()
        }
    ";
    let expected: &[(usize, usize, &[(usize, Interval)])] = &[
        // After x >= 0: x = [0, ∞]
        (1, 0, &[(1, Range(Some(0), None))]),
        // After x <= 100: x = [0, 100]
        (2, 0, &[(1, Range(Some(0), Some(100)))]),
        (2, 1, &[(0, Range(Some(0), Some(100)))]),
    ];
    test(params, body, expected);
}

/// Equality filter: exact value
#[test]
fn test_eq_filter() {
    let params = "x: i32";
    let body = "
        let b;
        {
            b = x == 5;
            match b {
                true => bb1,
                _ => bb2,
            }
        }
        bb1 = {
            RET = x;
            Goto(bb3)
        }
        bb2 = {
            RET = 0;
            Goto(bb3)
        }
        bb3 = {
            Return()
        }
    ";
    let expected: &[(usize, usize, &[(usize, Interval)])] = &[
        // True: x == 5 → x = [5, 5]
        (1, 0, &[(1, Range(Some(5), Some(5)))]),
        (1, 1, &[(0, Range(Some(5), Some(5)))]),
        // False: x != 5 → x = TOP (can't narrow ne on TOP)
        (2, 0, &[(1, TOP)]),
    ];
    test(params, body, expected);
}

/// Inequality filter on a known singleton [5,5] with != 5 → Bot
#[test]
fn test_ne_filter_singleton() {
    let params = "x: i32";
    let body = "
        let b1;
        let b2;
        {
            b1 = x == 5;
            match b1 {
                true => bb1,
                _ => bb3,
            }
        }
        bb1 = {
            b2 = x != 5;
            match b2 {
                true => bb2,
                _ => bb3,
            }
        }
        bb2 = {
            RET = x;
            Goto(bb3)
        }
        bb3 = {
            Return()
        }
    ";
    let expected: &[(usize, usize, &[(usize, Interval)])] = &[
        // After x == 5 (true): x = [5, 5]
        (1, 0, &[(1, Range(Some(5), Some(5)))]),
        // x != 5 on [5,5]: true branch → Bot (impossible), false → [5,5]
        // Since true_iv is Bot, no edge to bb2. bb2 stays Bot.
    ];
    test(params, body, expected);
}

/// Loop counting up to a bound: x = 0; while x < 100 { x = x + 1 }
#[test]
fn test_loop_count_to_100() {
    let params = "";
    let body = "
        let x;
        let b;
        {
            x = 0;
            Goto(bb1)
        }
        bb1 = {
            b = x < 100;
            match b {
                true => bb2,
                _ => bb3,
            }
        }
        bb2 = {
            x = x + 1;
            Goto(bb1)
        }
        bb3 = {
            Return()
        }
    ";
    let expected: &[(usize, usize, &[(usize, Interval)])] = &[
        // After widening + narrowing: loop head x = [0, 100]
        (1, 0, &[(1, Range(Some(0), Some(100)))]),
        // True branch: x < 100 → x = [0, 99]
        (2, 0, &[(1, Range(Some(0), Some(99)))]),
        // After x = x + 1: [1, 100]
        (2, 1, &[(1, Range(Some(1), Some(100)))]),
        // False branch: x >= 100 → x = [100, 100]
        (3, 0, &[(1, Range(Some(100), Some(100)))]),
    ];
    test(params, body, expected);
}

/// Two variables incremented in a loop
#[test]
fn test_two_vars_loop() {
    let params = "";
    let body = "
        let x;
        let y;
        let b;
        {
            x = 0;
            y = 10;
            Goto(bb1)
        }
        bb1 = {
            b = x <= 5;
            match b {
                true => bb2,
                _ => bb3,
            }
        }
        bb2 = {
            x = x + 1;
            y = y + 2;
            Goto(bb1)
        }
        bb3 = {
            RET = y;
            Return()
        }
    ";
    let expected: &[(usize, usize, &[(usize, Interval)])] = &[
        // After widening + narrowing at loop head:
        // x: [0, 6] (narrowed from [0,∞]), y: [10, ∞] (no constraint on y in loop guard)
        (1, 0, &[(1, Range(Some(0), Some(6))), (2, Range(Some(10), None))]),
        // True branch: x <= 5 → x = [0, 5]
        (2, 0, &[(1, Range(Some(0), Some(5)))]),
        // After x = x+1: [1, 6]
        (2, 1, &[(1, Range(Some(1), Some(6)))]),
        // False branch: x > 5 → x = [6, 6]
        (3, 0, &[(1, Range(Some(6), Some(6)))]),
    ];
    test(params, body, expected);
}
