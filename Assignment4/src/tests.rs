use std::collections::HashMap;

use Interval::Range;
use rustc_middle::{
    mir::{BasicBlock, Local, Location},
    ty::TyCtxt,
};

use crate::{analysis, domains::*, utils};

const BOT: Interval = Interval::Bot;
const TOP: Interval = Range(None, None);

fn print_mir_with_res(
    res: &HashMap<String, HashMap<Location, AbsState>>,
    tcx: TyCtxt<'_>,
) {
    for def_id in tcx.hir_body_owners() {
        let name = tcx.item_name(def_id.to_def_id()).to_string();
        let Some(fn_states) = res.get(&name) else {
            continue;
        };
        let body = tcx.optimized_mir(def_id);
        println!("---- {name} ----");
        for (bb, bbd) in body.basic_blocks.iter_enumerated() {
            println!("{bb:?}");
            for (i, stmt) in bbd.statements.iter().enumerate() {
                let location = Location {
                    block: bb,
                    statement_index: i,
                };
                println!("  // {:?}", fn_states.get(&location));
                println!("  {i}: {stmt:?}");
            }
            let location = Location {
                block: bb,
                statement_index: bbd.statements.len(),
            };
            println!("  // {:?}", fn_states.get(&location));
            println!(
                "  {}: {:?}",
                bbd.statements.len(),
                bbd.terminator().kind
            );
        }
    }
}

fn embed_funcs(funcs: &[(&str, &str, &str)]) -> String {
    let mut code = String::from(
        r#"
        #![feature(core_intrinsics, custom_mir)]
        #![allow(internal_features)]
        use core::intrinsics::mir::*;
    "#,
    );
    for (name, params, body) in funcs {
        code.push_str(&format!(
            r#"
            #[custom_mir(dialect = "runtime", phase = "optimized")]
            fn {name}({params}) -> i32 {{
                mir! {{ {body} }}
            }}
            "#
        ));
    }
    code
}

fn test(
    funcs: &[(&str, &str, &str)],
    expected: &[(&str, usize, usize, &[(usize, Interval)])],
) {
    let code = embed_funcs(funcs);
    utils::run_compiler_on_str(&code, |tcx| {
        let res = analysis::analyze(tcx);
        print_mir_with_res(&res, tcx);
        for (func, bb, stmt_idx, intervals) in expected {
            let location = Location {
                block: BasicBlock::from_usize(*bb),
                statement_index: *stmt_idx,
            };
            let fn_states = res.get(*func);
            let state = fn_states.and_then(|m| m.get(&location));
            for (local, interval) in *intervals {
                let actual = state
                    .and_then(|state| state.0.get(&Local::from_usize(*local)))
                    .unwrap_or(&BOT);
                assert_eq!(
                    interval, actual,
                    "{func}: bb{bb}:{stmt_idx} _{local}"
                );
            }
        }
    })
    .unwrap();
}

// =====================================================================
// Basic intraprocedural sanity (carried over from Assignment 3)
// =====================================================================

#[test]
fn test_simple_main_only() {
    let funcs: &[(&str, &str, &str)] = &[(
        "main",
        "",
        "
        {
            RET = 1 + 2;
            RET = RET + 1;
            RET = RET + RET;
            Return()
        }
        ",
    )];
    let expected: &[(&str, usize, usize, &[(usize, Interval)])] = &[
        ("main", 0, 0, &[(0, BOT)]),
        ("main", 0, 1, &[(0, Range(Some(3), Some(3)))]),
        ("main", 0, 2, &[(0, Range(Some(4), Some(4)))]),
        ("main", 0, 3, &[(0, Range(Some(8), Some(8)))]),
    ];
    test(funcs, expected);
}

#[test]
fn test_main_params_top() {
    // At main's entry, all parameters are TOP, all other locals are BOT.
    let funcs: &[(&str, &str, &str)] = &[(
        "main",
        "x: i32, y: i32",
        "
        {
            Return()
        }
        ",
    )];
    let expected: &[(&str, usize, usize, &[(usize, Interval)])] = &[(
        "main",
        0,
        0,
        &[(0, BOT), (1, TOP), (2, TOP)],
    )];
    test(funcs, expected);
}

// =====================================================================
// Interprocedural tests
// =====================================================================

/// foo(a) returns a + 1; main calls foo(10).
/// The returned value of foo at the call site should be [11, 11].
#[test]
fn test_call_simple_constant() {
    let funcs: &[(&str, &str, &str)] = &[
        (
            "main",
            "",
            "
            let r;
            {
                Call(r = foo(10), ReturnTo(bb1), UnwindContinue())
            }
            bb1 = {
                RET = r;
                Return()
            }
            ",
        ),
        (
            "foo",
            "a: i32",
            "
            {
                RET = a + 1;
                Return()
            }
            ",
        ),
    ];
    let expected: &[(&str, usize, usize, &[(usize, Interval)])] = &[
        // In foo, entry sees a = [10,10] from main's call site.
        ("foo", 0, 0, &[(1, Range(Some(10), Some(10)))]),
        // After RET = a + 1, RET = [11, 11].
        ("foo", 0, 1, &[(0, Range(Some(11), Some(11)))]),
        // Back in main: after the call, r = foo's RET = [11, 11].
        ("main", 1, 0, &[(1, Range(Some(11), Some(11)))]),
        // After RET = r, main's RET = [11, 11].
        ("main", 1, 1, &[(0, Range(Some(11), Some(11)))]),
    ];
    test(funcs, expected);
}

/// foo(a) returns a + 1; main calls foo(x) where x is the parameter (TOP).
/// foo's input is TOP, so its output is TOP.
#[test]
fn test_call_top_arg() {
    let funcs: &[(&str, &str, &str)] = &[
        (
            "main",
            "x: i32",
            "
            let r;
            {
                Call(r = foo(x), ReturnTo(bb1), UnwindContinue())
            }
            bb1 = {
                RET = r;
                Return()
            }
            ",
        ),
        (
            "foo",
            "a: i32",
            "
            {
                RET = a + 1;
                Return()
            }
            ",
        ),
    ];
    let expected: &[(&str, usize, usize, &[(usize, Interval)])] = &[
        ("foo", 0, 0, &[(1, TOP)]),
        // a + 1 on TOP stays TOP.
        ("foo", 0, 1, &[(0, TOP)]),
        ("main", 1, 0, &[(2, TOP)]),
    ];
    test(funcs, expected);
}

/// Two distinct call sites to the same function. In a context-insensitive
/// analysis they are joined at foo's entry. We expect a's interval at foo's
/// entry to be the join of {3, 7} = [3, 7], and RET = [4, 8].
#[test]
fn test_context_insensitive_join() {
    let funcs: &[(&str, &str, &str)] = &[
        (
            "main",
            "",
            "
            let r1;
            let r2;
            {
                Call(r1 = foo(3), ReturnTo(bb1), UnwindContinue())
            }
            bb1 = {
                Call(r2 = foo(7), ReturnTo(bb2), UnwindContinue())
            }
            bb2 = {
                RET = r1 + r2;
                Return()
            }
            ",
        ),
        (
            "foo",
            "a: i32",
            "
            {
                RET = a + 1;
                Return()
            }
            ",
        ),
    ];
    let expected: &[(&str, usize, usize, &[(usize, Interval)])] = &[
        // foo's input is joined: [3, 7].
        ("foo", 0, 0, &[(1, Range(Some(3), Some(7)))]),
        // foo's RET is the join: [4, 8].
        ("foo", 0, 1, &[(0, Range(Some(4), Some(8)))]),
        // Each call site in main sees foo's RET = [4, 8] (context-insensitive).
        ("main", 1, 0, &[(1, Range(Some(4), Some(8)))]),
        ("main", 2, 0, &[
            (1, Range(Some(4), Some(8))),
            (2, Range(Some(4), Some(8))),
        ]),
        // r1 + r2 = [4,8] + [4,8] = [8, 16].
        ("main", 2, 1, &[(0, Range(Some(8), Some(16)))]),
    ];
    test(funcs, expected);
}

/// add(a, b) returns a + b. main calls add(2, 3).
/// foo's entry: a=[2,2], b=[3,3]; RET = [5,5].
#[test]
fn test_call_two_args() {
    let funcs: &[(&str, &str, &str)] = &[
        (
            "main",
            "",
            "
            let r;
            {
                Call(r = add(2, 3), ReturnTo(bb1), UnwindContinue())
            }
            bb1 = {
                RET = r;
                Return()
            }
            ",
        ),
        (
            "add",
            "a: i32, b: i32",
            "
            {
                RET = a + b;
                Return()
            }
            ",
        ),
    ];
    let expected: &[(&str, usize, usize, &[(usize, Interval)])] = &[
        (
            "add",
            0,
            0,
            &[(1, Range(Some(2), Some(2))), (2, Range(Some(3), Some(3)))],
        ),
        ("add", 0, 1, &[(0, Range(Some(5), Some(5)))]),
        ("main", 1, 0, &[(1, Range(Some(5), Some(5)))]),
        ("main", 1, 1, &[(0, Range(Some(5), Some(5)))]),
    ];
    test(funcs, expected);
}

/// foo branches on its parameter and returns either 0 or 10.
/// Caller observes foo's RET = [0, 10].
#[test]
fn test_callee_with_branch() {
    let funcs: &[(&str, &str, &str)] = &[
        (
            "main",
            "x: i32",
            "
            let r;
            {
                Call(r = foo(x), ReturnTo(bb1), UnwindContinue())
            }
            bb1 = {
                RET = r;
                Return()
            }
            ",
        ),
        (
            "foo",
            "a: i32",
            "
            let b;
            {
                b = a == 0;
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
            ",
        ),
    ];
    let expected: &[(&str, usize, usize, &[(usize, Interval)])] = &[
        // foo at the merge: RET ∈ [0, 10].
        ("foo", 3, 0, &[(0, Range(Some(0), Some(10)))]),
        // main sees r ∈ [0, 10].
        ("main", 1, 0, &[(2, Range(Some(0), Some(10)))]),
    ];
    test(funcs, expected);
}

/// Two callers: foo(2) and foo(8). Inside foo, parameter a ∈ [2, 8].
/// The callee returns a, so each caller sees foo = [2, 8].
#[test]
fn test_multiple_callers_join() {
    let funcs: &[(&str, &str, &str)] = &[
        (
            "main",
            "",
            "
            let r1;
            let r2;
            {
                Call(r1 = foo(2), ReturnTo(bb1), UnwindContinue())
            }
            bb1 = {
                Call(r2 = foo(8), ReturnTo(bb2), UnwindContinue())
            }
            bb2 = {
                RET = r1;
                Return()
            }
            ",
        ),
        (
            "foo",
            "a: i32",
            "
            {
                RET = a;
                Return()
            }
            ",
        ),
    ];
    let expected: &[(&str, usize, usize, &[(usize, Interval)])] = &[
        ("foo", 0, 0, &[(1, Range(Some(2), Some(8)))]),
        ("foo", 0, 1, &[(0, Range(Some(2), Some(8)))]),
        // After the first call, r1 has the joined value [2, 8].
        ("main", 1, 0, &[(1, Range(Some(2), Some(8)))]),
    ];
    test(funcs, expected);
}

/// A chain of calls: main -> bar -> foo.
/// foo(a) = a + 1; bar(x) = foo(x) * 2; main calls bar(3).
/// foo: a=[3,3], RET=[4,4]; bar: t=foo(x)=[4,4], RET=[8,8]; main: r=[8,8].
#[test]
fn test_call_chain() {
    let funcs: &[(&str, &str, &str)] = &[
        (
            "main",
            "",
            "
            let r;
            {
                Call(r = bar(3), ReturnTo(bb1), UnwindContinue())
            }
            bb1 = {
                RET = r;
                Return()
            }
            ",
        ),
        (
            "bar",
            "x: i32",
            "
            let t;
            {
                Call(t = foo(x), ReturnTo(bb1), UnwindContinue())
            }
            bb1 = {
                RET = t * 2;
                Return()
            }
            ",
        ),
        (
            "foo",
            "a: i32",
            "
            {
                RET = a + 1;
                Return()
            }
            ",
        ),
    ];
    let expected: &[(&str, usize, usize, &[(usize, Interval)])] = &[
        ("foo", 0, 0, &[(1, Range(Some(3), Some(3)))]),
        ("foo", 0, 1, &[(0, Range(Some(4), Some(4)))]),
        ("bar", 1, 0, &[(2, Range(Some(4), Some(4)))]),
        ("bar", 1, 1, &[(0, Range(Some(8), Some(8)))]),
        ("main", 1, 0, &[(1, Range(Some(8), Some(8)))]),
    ];
    test(funcs, expected);
}

/// Recursion: foo(a) = if a == 0 { 0 } else { foo(a - 1) + 1 }.
/// With widening, the recursive call's input widens to [-∞, ∞] eventually
/// (since a - 1 keeps decreasing without bound on the over-approximated
/// recursive edge). The return value, however, narrows back to a sound
/// over-approximation. We just check soundness: foo's RET must contain 0
/// for any input.
#[test]
fn test_recursive_call() {
    let funcs: &[(&str, &str, &str)] = &[
        (
            "main",
            "",
            "
            let r;
            {
                Call(r = foo(3), ReturnTo(bb1), UnwindContinue())
            }
            bb1 = {
                RET = r;
                Return()
            }
            ",
        ),
        (
            "foo",
            "a: i32",
            "
            let b;
            let t;
            let m;
            {
                b = a == 0;
                match b {
                    true => bb1,
                    _ => bb2,
                }
            }
            bb1 = {
                RET = 0;
                Goto(bb4)
            }
            bb2 = {
                m = a - 1;
                Call(t = foo(m), ReturnTo(bb3), UnwindContinue())
            }
            bb3 = {
                RET = t + 1;
                Goto(bb4)
            }
            bb4 = {
                Return()
            }
            ",
        ),
    ];
    // Soundness check: verify analysis terminates and produces a non-bot
    // state at foo's return location. We don't pin the exact widened result.
    let funcs_owned = funcs;
    let code = embed_funcs(funcs_owned);
    utils::run_compiler_on_str(&code, |tcx| {
        let res = analysis::analyze(tcx);
        print_mir_with_res(&res, tcx);
        let foo_states = res.get("foo").expect("foo should be analyzed");
        // The Return block of foo is bb4.
        let ret_loc = Location {
            block: BasicBlock::from_usize(4),
            statement_index: 0,
        };
        let st = foo_states.get(&ret_loc).expect("foo's return location");
        let ret_iv = st.0.get(&Local::from_usize(0)).copied().unwrap_or(BOT);
        // The returned interval must contain 0 (the base case).
        match ret_iv {
            Range(lo, hi) => {
                assert!(
                    lo.unwrap_or(i32::MIN) <= 0 && hi.unwrap_or(i32::MAX) >= 0,
                    "expected interval to contain 0, got {:?}",
                    ret_iv
                );
            }
            BOT => panic!("foo's RET is BOT (unsound for terminating recursion)"),
        }
    })
    .unwrap();
}

/// Two distinct call sites with literal arguments; context-insensitive
/// merging gives inc's parameter the joined interval [5, 6] and return
/// value [6, 7]. Uses literals (not r1) so widening at inc's entry stays
/// stable across iterations.
#[test]
fn test_call_uses_result() {
    let funcs: &[(&str, &str, &str)] = &[
        (
            "main",
            "",
            "
            let r1;
            let r2;
            {
                Call(r1 = inc(5), ReturnTo(bb1), UnwindContinue())
            }
            bb1 = {
                Call(r2 = inc(6), ReturnTo(bb2), UnwindContinue())
            }
            bb2 = {
                RET = r2;
                Return()
            }
            ",
        ),
        (
            "inc",
            "a: i32",
            "
            {
                RET = a + 1;
                Return()
            }
            ",
        ),
    ];
    let expected: &[(&str, usize, usize, &[(usize, Interval)])] = &[
        ("inc", 0, 0, &[(1, Range(Some(5), Some(6)))]),
        ("inc", 0, 1, &[(0, Range(Some(6), Some(7)))]),
        ("main", 2, 0, &[(2, Range(Some(6), Some(7)))]),
        ("main", 2, 1, &[(0, Range(Some(6), Some(7)))]),
    ];
    test(funcs, expected);
}

/// Loop with widening inside main, no calls. Sanity check that the existing
/// widening/narrowing machinery still works under the interprocedural driver.
#[test]
fn test_loop_widening_narrowing_main() {
    let funcs: &[(&str, &str, &str)] = &[(
        "main",
        "",
        "
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
        ",
    )];
    let expected: &[(&str, usize, usize, &[(usize, Interval)])] = &[
        ("main", 0, 0, &[(1, BOT)]),
        ("main", 0, 1, &[(1, Range(Some(0), Some(0)))]),
        ("main", 1, 0, &[(1, Range(Some(0), Some(11)))]),
        ("main", 2, 0, &[(1, Range(Some(0), Some(10)))]),
        ("main", 2, 1, &[(1, Range(Some(1), Some(11)))]),
        ("main", 3, 0, &[(1, Range(Some(11), Some(11)))]),
    ];
    test(funcs, expected);
}

/// Call inside a loop. main runs a counter and calls foo each iteration.
/// foo just returns its argument. The argument to foo across iterations
/// joins to the widened/narrowed interval of the counter.
#[test]
fn test_call_inside_loop() {
    let funcs: &[(&str, &str, &str)] = &[
        (
            "main",
            "",
            "
            let i;
            let s;
            let b;
            {
                i = 0;
                s = 0;
                Goto(bb1)
            }
            bb1 = {
                b = i < 5;
                match b {
                    true => bb2,
                    _ => bb4,
                }
            }
            bb2 = {
                Call(s = id(i), ReturnTo(bb3), UnwindContinue())
            }
            bb3 = {
                i = i + 1;
                Goto(bb1)
            }
            bb4 = {
                RET = s;
                Return()
            }
            ",
        ),
        (
            "id",
            "x: i32",
            "
            {
                RET = x;
                Return()
            }
            ",
        ),
    ];
    let expected: &[(&str, usize, usize, &[(usize, Interval)])] = &[
        // id's parameter joins all loop-iteration values: [0, 4].
        ("id", 0, 0, &[(1, Range(Some(0), Some(4)))]),
        ("id", 0, 1, &[(0, Range(Some(0), Some(4)))]),
        // After widening and narrowing, the counter at the loop head is [0, 5].
        ("main", 1, 0, &[(1, Range(Some(0), Some(5)))]),
        // Exit edge: i >= 5, so i = [5, 5].
        ("main", 4, 0, &[(1, Range(Some(5), Some(5)))]),
    ];
    test(funcs, expected);
}

/// Call with zero parameters. foo() returns 42.
#[test]
fn test_call_no_args() {
    let funcs: &[(&str, &str, &str)] = &[
        (
            "main",
            "",
            "
            let r;
            {
                Call(r = answer(), ReturnTo(bb1), UnwindContinue())
            }
            bb1 = {
                RET = r;
                Return()
            }
            ",
        ),
        (
            "answer",
            "",
            "
            {
                RET = 42;
                Return()
            }
            ",
        ),
    ];
    let expected: &[(&str, usize, usize, &[(usize, Interval)])] = &[
        ("answer", 0, 1, &[(0, Range(Some(42), Some(42)))]),
        ("main", 1, 0, &[(1, Range(Some(42), Some(42)))]),
        ("main", 1, 1, &[(0, Range(Some(42), Some(42)))]),
    ];
    test(funcs, expected);
}
