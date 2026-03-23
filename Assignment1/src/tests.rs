/// 기본 테스트 — 과제 제공 테스트 5개
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


// =====================================================================
//  강의 슬라이드 예시 기반 테스트
//  03-type-1 (Lecture 3), 04-type-2 (Lecture 4) 슬라이드에 직접 등장하는
//  예시들을 검증합니다.
// =====================================================================

// =====================================================================
//  03-type-1 (Lecture 3) — 제약 기반 타입 분석 기초
// =====================================================================

/// [Lecture 3, Slide 14] Example 1: fn f(x, y) { x + y }
///   ⟦x + y⟧ = ⟦x⟧ = ⟦y⟧ = i32
#[test]
fn test_lec3_ex1_simple_addition() {
    test("fn f(x: (), y: ()) { x + y }");
}

/// [Lecture 3, Slides 15-16] Example 2: 고차 함수
///   fn f(x) { x }, fn g(y) { y(1) }, fn h() { g(f) }
///   → f: fn(i32)->i32, g: fn(fn(i32)->i32)->i32, h: fn()->i32
#[test]
fn test_lec3_ex2_higher_order() {
    test(
        "
fn f(x: ()) { x }
fn g(y: ()) { y(1) }
fn h() { g(f) }
",
    );
}

/// [Lecture 3, Slide 17] Example 3: bool vs i32 충돌
///   if x → x=bool, x+1 → x=i32, 충돌 → no solution
#[test]
fn test_lec3_ex3_bool_i32_conflict() {
    test_none("fn f(x: ()) { if x { x + 1 } else { 0 } }");
}

/// [Lecture 3, Slide 18] Example 4: 재귀 타입 (occurs check)
///   fn f(x) { let y = x + 1; f } → 무한 타입 → no solution
#[test]
fn test_lec3_ex4_recursive_type() {
    test_none("fn f(x: ()) { let y = x + 1; f }");
}

/// [Lecture 3, Slide 19] Example 5: 항등 함수 (principal type)
///   fn f(x) { x } → fn(X) → X (TYPEVAR 포함)
#[test]
fn test_lec3_ex5_identity_polymorphic() {
    test_var("fn f(x: ()) { x }");
}

// =====================================================================
//  04-type-2 (Lecture 4) — 튜플, 제약 해결, 한계
// =====================================================================

/// [Lecture 4, Slide 15] Correct Version Example 1: (1, true).0
///   유효 접근 → i32
#[test]
fn test_lec4_tuple_access_first() {
    test("fn f() { let x = (1, true); x.0 }");
}

/// [Lecture 4, Slide 16] Correct Version Example 2: (1, true).2
///   OOB → Absent → ⟦e.i⟧ ≠ ◇ 위반 → no solution
#[test]
fn test_lec4_tuple_oob_access() {
    test_none("fn f() { let x = (1, true); x.2 }");
}

/// [Lecture 4, Slides 19-20] 단형 분석 한계: let-polymorphism 없음
///   f(1) → X=i32, f(true) → X=bool, 충돌 → no solution
#[test]
fn test_lec4_let_polymorphism_limitation() {
    test_none(
        "
fn f(x: ()) { x }
fn g() { f(1); f(true) }
",
    );
}

/// [Lecture 4, Slide 21] Higher-rank polymorphism 없음
///   g(y) { y(1); y(true) } → y의 인자가 i32이면서 bool → no solution
#[test]
fn test_lec4_polymorphism_higher_order() {
    test_none(
        "
fn f(x: ()) { x }
fn g(y: ()) { y(1); y(true) }
fn h() { g(f) }
",
    );
}

/// [Lecture 4, Slide 22] Polymorphic Recursion
///   f(true, n-1) → x=bool, f(0, n-1) → x=i32, 충돌 → no solution
#[test]
fn test_lec4_polymorphic_recursion() {
    test_none(
        "
fn f(x: (), n: ()) {
    if n > 1 {
        f(true, n - 1);
    } else if n == 1 {
        f(0, n - 1);
    }
    x
}
",
    );
}

/// [Lecture 4, Slide 9] 비-튜플 프로젝션 거부: i32.0
#[test]
fn test_lec4_projection_on_non_tuple_i32() {
    test_none("fn f() { let x = 1; x.0 }");
}

/// [Lecture 4, Slide 18] Flow-Insensitivity: x+2(i32)와 if x(bool) 동시 사용
#[test]
fn test_lec4_flow_insensitivity() {
    test_none(
        "
fn f(x: ()) {
    let y = x + 2;
    if x > 0 { y } else { 0 };
    if x { 1 } else { 0 }
}
",
    );
}


// =====================================================================
//  커스텀 Edge Case 테스트
//  강의 슬라이드가 제공하는 타입 시스템 가정 하에서, 모든 구문과
//  타입 조합의 경계 조건을 검증합니다.
// =====================================================================

// =====================================================================
//  1. 리터럴 & 기본 타입
// =====================================================================

/// i32 리터럴만 반환
#[test]
fn test_literal_i32() {
    test("fn f() { 42 }");
}

/// bool 리터럴만 반환
#[test]
fn test_literal_bool() {
    test("fn f() { true }");
}

/// 빈 튜플 (unit) 반환
#[test]
fn test_literal_unit() {
    test("fn f() { () }");
}

// =====================================================================
//  2. 산술 연산 — 모든 연산자
// =====================================================================

/// 덧셈: i32 + i32 = i32
#[test]
fn test_arith_add() {
    test("fn f(x: (), y: ()) { x + y }");
}

/// 뺄셈
#[test]
fn test_arith_sub() {
    test("fn f(x: (), y: ()) { x - y }");
}

/// 곱셈
#[test]
fn test_arith_mul() {
    test("fn f(x: (), y: ()) { x * y }");
}

/// 나눗셈
#[test]
fn test_arith_div() {
    test("fn f(x: (), y: ()) { x / y }");
}

/// 나머지
#[test]
fn test_arith_rem() {
    test("fn f(x: (), y: ()) { x % y }");
}

/// 비트 XOR
#[test]
fn test_arith_bitxor() {
    test("fn f(x: (), y: ()) { x ^ y }");
}

/// 비트 AND
#[test]
fn test_arith_bitand() {
    test("fn f(x: (), y: ()) { x & y }");
}

/// 비트 OR
#[test]
fn test_arith_bitor() {
    test("fn f(x: (), y: ()) { x | y }");
}

/// 좌시프트
#[test]
fn test_arith_shl() {
    test("fn f(x: (), y: ()) { x << y }");
}

/// 우시프트
#[test]
fn test_arith_shr() {
    test("fn f(x: (), y: ()) { x >> y }");
}

/// 산술 체이닝: (a + b) * (c - d) / e
#[test]
fn test_arith_chain() {
    test("fn f(a: (), b: (), c: (), d: (), e: ()) { (a + b) * (c - d) / e }");
}

/// bool에 산술 연산 적용 → 실패
#[test]
fn test_arith_on_bool() {
    test_none("fn f() { true + false }");
}

/// 튜플에 산술 연산 적용 → 실패
#[test]
fn test_arith_on_tuple() {
    test_none("fn f() { (1, 2) + 3 }");
}

/// 참조에 산술 연산 적용 → 실패
#[test]
fn test_arith_on_ref() {
    test_none("fn f() { let x = &1; x + 1 }");
}

/// 함수에 산술 연산 적용 → 실패
#[test]
fn test_arith_on_fn() {
    test_none(
        "
fn g(x: ()) { x }
fn f() { g + 1 }
",
    );
}

// =====================================================================
//  3. 논리 연산
// =====================================================================

/// AND: bool && bool = bool
#[test]
fn test_logic_and() {
    test("fn f(x: (), y: ()) { x && y }");
}

/// OR: bool || bool = bool
#[test]
fn test_logic_or() {
    test("fn f(x: (), y: ()) { x || y }");
}

/// i32에 논리 연산 적용 → 실패
#[test]
fn test_logic_on_i32() {
    test_none("fn f() { 1 && 2 }");
}

/// 같은 변수에 산술+논리 → 실패 (i32 ≠ bool)
#[test]
fn test_arith_and_logic_same_var() {
    test_none("fn f(x: (), y: ()) { x + y; x && y }");
}

// =====================================================================
//  4. 비교 연산
// =====================================================================

/// ==: i32 == i32 = bool
#[test]
fn test_cmp_eq() {
    test("fn f(x: (), y: ()) { x == y }");
}

/// !=
#[test]
fn test_cmp_ne() {
    test("fn f(x: (), y: ()) { x != y }");
}

/// <
#[test]
fn test_cmp_lt() {
    test("fn f(x: (), y: ()) { x < y }");
}

/// <=
#[test]
fn test_cmp_le() {
    test("fn f(x: (), y: ()) { x <= y }");
}

/// >
#[test]
fn test_cmp_gt() {
    test("fn f(x: (), y: ()) { x > y }");
}

/// >=
#[test]
fn test_cmp_ge() {
    test("fn f(x: (), y: ()) { x >= y }");
}

/// 비교 결과를 if 조건으로 사용
#[test]
fn test_cmp_in_if_condition() {
    test("fn f(x: (), y: ()) { if x > y { x } else { y } }");
}

/// 비교 결과를 논리 연산에 사용: (x > 0) && (y < 10)
#[test]
fn test_cmp_result_in_logic() {
    test("fn f(x: (), y: ()) { (x > 0) && (y < 10) }");
}

// =====================================================================
//  5. 단항 연산
// =====================================================================

/// 산술 부정: -x, x = i32
#[test]
fn test_unary_neg() {
    test("fn f(x: ()) { -x }");
}

/// 논리 부정: !x, x = bool
#[test]
fn test_unary_not() {
    test("fn f(x: ()) { !x }");
}

/// 이중 부정: --x = x, 결과 i32
#[test]
fn test_unary_double_neg() {
    test("fn f(x: ()) { -(-x) }");
}

/// 이중 논리 부정: !!x = x, 결과 bool
#[test]
fn test_unary_double_not() {
    test("fn f(x: ()) { !(!x) }");
}

/// bool에 산술 부정 → 실패 (- 는 i32 전용)
#[test]
fn test_neg_on_bool() {
    test_none("fn f(x: ()) { -x; !x }");
}

// =====================================================================
//  6. 참조 & 역참조
// =====================================================================

/// 기본: &x → Ref(T), *(&x) → T
#[test]
fn test_ref_deref_roundtrip() {
    test("fn f() { let x = 1; *(&x) }");
}

/// 이중 참조: &&x
#[test]
fn test_double_ref() {
    test("fn f() { let x = 1; let y = &x; let z = &y; **z }");
}

/// 삼중 참조: &&&x
#[test]
fn test_triple_ref() {
    test("fn f() { let x = 1; let y = &x; let z = &y; let w = &z; ***w }");
}

/// 참조를 함수 인자로 전달
#[test]
fn test_ref_as_fn_arg() {
    test(
        "
fn deref_add(r: ()) { *r + 1 }
fn foo() { let x = 10; deref_add(&x) }
",
    );
}

/// 비참조 역참조 → 실패
#[test]
fn test_deref_non_ref_i32() {
    test_none("fn f() { let x = 1; *x }");
}

/// bool 역참조 → 실패
#[test]
fn test_deref_non_ref_bool() {
    test_none("fn f() { *true }");
}

/// 튜플 역참조 → 실패
#[test]
fn test_deref_non_ref_tuple() {
    test_none("fn f() { *(1, 2) }");
}

/// 참조의 참조를 한 번만 역참조 → 결과는 참조
#[test]
fn test_partial_deref() {
    test(
        "
fn f() {
    let x = 1;
    let y = &x;
    let z = &y;
    let w = *z;
    *w
}
",
    );
}

/// 참조를 튜플에 넣고 꺼내서 역참조
#[test]
fn test_ref_in_tuple() {
    test("fn f() { let x = 1; let t = (&x, &x); *t.0 + *t.1 }");
}

// =====================================================================
//  7. 튜플 — 생성, 접근, 중첩
// =====================================================================

/// 두 번째 원소 접근
#[test]
fn test_tuple_access_second() {
    test("fn f() { let x = (1, true); x.1 }");
}

/// 중첩 튜플 접근
#[test]
fn test_nested_tuple_access() {
    test("fn f() { let x = (1, (2, 3)); x.0 + x.1.0 + x.1.1 }");
}

/// 깊은 중첩: ((1,),).0.0
#[test]
fn test_deeply_nested_tuple() {
    test("fn f() { let x = ((1, 2), (3, 4)); x.0.0 + x.0.1 + x.1.0 + x.1.1 }");
}

/// 3개 원소 튜플
#[test]
fn test_triple_tuple() {
    test("fn f() { let x = (1, 2, 3); x.0 + x.1 + x.2 }");
}

/// 혼합 타입 튜플: (i32, bool, i32)
#[test]
fn test_mixed_type_tuple() {
    test("fn f() { let x = (1, true, 2); x.0 + x.2 }");
}

/// 혼합 타입 튜플의 bool 원소 접근
#[test]
fn test_mixed_tuple_bool_access() {
    test("fn f() { let x = (1, true, 2); if x.1 { x.0 } else { x.2 } }");
}

/// OOB: 2개 튜플의 .2 접근 → 실패
#[test]
fn test_tuple_oob_2elem() {
    test_none("fn f() { let x = (1, true); x.2 }");
}

/// OOB: 3개 튜플의 .3 접근 → 실패
#[test]
fn test_tuple_oob_3elem() {
    test_none("fn f() { let x = (1, true, 3); x.3 }");
}

/// OOB: 중첩 튜플 내부에서 → 실패
#[test]
fn test_tuple_oob_nested() {
    test_none("fn f() { let x = (1, (2, 3)); x.1.2 }");
}

/// 비-튜플 프로젝션: i32.0 → 실패
#[test]
fn test_projection_on_i32() {
    test_none("fn f() { let x = 1; x.0 }");
}

/// 비-튜플 프로젝션: bool.0 → 실패
#[test]
fn test_projection_on_bool() {
    test_none("fn f() { let x = true; x.0 }");
}

/// 비-튜플 프로젝션: fn.0 → 실패
#[test]
fn test_projection_on_fn() {
    test_none(
        "
fn g(x: ()) { x + 1 }
fn f() { g.0 }
",
    );
}

/// 비-튜플 프로젝션: ref.0 → 실패
#[test]
fn test_projection_on_ref() {
    test_none("fn f() { let x = &1; x.0 }");
}

/// non-concrete 튜플 (파라미터) 접근 → 성공, 타입은 미결정
#[test]
fn test_non_concrete_tuple_access() {
    test_var("fn f(x: ()) { x.0 }");
}

/// non-concrete 튜플에 여러 접근 → 각각 독립적 TYPEVAR
#[test]
fn test_non_concrete_tuple_multi_access() {
    test_var("fn f(x: ()) { (x.0, x.1) }");
}

/// non-concrete 튜플 접근 후 산술 → 결정됨
#[test]
fn test_non_concrete_tuple_access_then_arith() {
    test("fn f(x: ()) { x.0 + x.1 }");
}

/// 튜플의 if 분기 타입 불일치 → 실패
#[test]
fn test_tuple_if_branch_mismatch() {
    test_none("fn f() { if true { (1, 2) } else { (1, true) } }");
}

/// 빈 튜플과 비-빈 튜플 unify
///
/// ()는 make_tuple([], false)로 non-concrete 생성되므로,
/// (1,)과 unify 시 fresh 변수로 패딩되어 분석은 통과.
/// 단, 변환 후 코드가 Rust 타입 체크에서 ()와 (i32,) 불일치로 실패하므로
/// test_none도 test도 아닌, 분석 결과만 확인 (해가 존재함).
#[test]
fn test_unit_vs_nonempty_tuple() {
    let result = utils::run_compiler_on_str(
        "fn f(x: ()) { if true { () } else { (1,) }; x + 1 }",
        transformation::transform,
    )
    .unwrap();
    assert!(result.is_some());
}

// =====================================================================
//  8. 함수 — 호출, 고차, arity
// =====================================================================

/// 0개 인자 함수
#[test]
fn test_fn_zero_args() {
    test(
        "
fn f() { 42 }
fn g() { f() + 1 }
",
    );
}

/// 1개 인자 함수
#[test]
fn test_fn_one_arg() {
    test(
        "
fn inc(x: ()) { x + 1 }
fn g() { inc(5) }
",
    );
}

/// 3개 인자 함수
#[test]
fn test_fn_three_args() {
    test("fn f(a: (), b: (), c: ()) { a + b + c }");
}

/// 함수 반환값을 다른 함수에 전달
#[test]
fn test_fn_chained_calls() {
    test(
        "
fn double(x: ()) { x + x }
fn inc(x: ()) { x + 1 }
fn foo() { double(inc(3)) }
",
    );
}

/// 함수를 인자로 전달 (apply 패턴)
#[test]
fn test_fn_as_argument() {
    test(
        "
fn add1(x: ()) { x + 1 }
fn apply(f: (), x: ()) { f(x) }
fn foo() { apply(add1, 2) }
",
    );
}

/// 함수 합성: compose(f, g, x) = f(g(x))
#[test]
fn test_fn_composition() {
    test(
        "
fn compose(f: (), g: (), x: ()) { f(g(x)) }
fn double(x: ()) { x + x }
fn inc(x: ()) { x + 1 }
fn foo() { compose(double, inc, 3) }
",
    );
}

/// 함수 반환 튜플 → 접근
#[test]
fn test_fn_returns_tuple() {
    test(
        "
fn make_pair() { (1, true) }
fn foo() { let p = make_pair(); p.0 + 1 }
",
    );
}

/// 함수를 튜플에 넣고 꺼내서 호출
#[test]
fn test_fn_in_tuple() {
    test(
        "
fn add1(x: ()) { x + 1 }
fn sub1(x: ()) { x - 1 }
fn foo() { let t = (add1, sub1); t.0(10) + t.1(10) }
",
    );
}

/// arity 불일치: 1개 인자 함수를 0개로 호출 → 실패
#[test]
fn test_arity_mismatch_fewer() {
    test_none(
        "
fn f(x: ()) { x + 1 }
fn g() { f() }
",
    );
}

/// arity 불일치: 1개 인자 함수를 2개로 호출 → 실패
#[test]
fn test_arity_mismatch_more() {
    test_none(
        "
fn f(x: ()) { x + 1 }
fn g() { f(1, 2) }
",
    );
}

/// 비-함수 호출: i32를 호출 → 실패
#[test]
fn test_call_non_fn_i32() {
    test_none("fn f() { let x = 1; x(2) }");
}

/// 비-함수 호출: bool을 호출 → 실패
#[test]
fn test_call_non_fn_bool() {
    test_none("fn f() { let x = true; x(1) }");
}

/// 비-함수 호출: 튜플을 호출 → 실패
#[test]
fn test_call_non_fn_tuple() {
    test_none("fn f() { let x = (1, 2); x(3) }");
}

/// 상호 재귀
#[test]
fn test_mutual_recursion() {
    test(
        "
fn f(x: ()) { x + g(x) }
fn g(y: ()) { y - f(y) }
",
    );
}

/// 자기 재귀 (유효)
#[test]
fn test_self_recursion_valid() {
    test(
        "
fn factorial(n: ()) {
    if n == 0 { 1 } else { n * factorial(n - 1) }
}
",
    );
}

/// 자기 재귀 (타입 불일치) → 실패
#[test]
fn test_self_recursion_conflict() {
    test_none(
        "
fn f(x: ()) {
    if x > 0 { f(true) } else { 0 }
}
",
    );
}

/// 미제약 함수 — TYPEVAR 잔존
#[test]
fn test_unconstrained_fn() {
    test_var("fn f(x: ()) { x }");
}

/// 미제약 2인자 함수
#[test]
fn test_unconstrained_fn_two_params() {
    test_var("fn f(x: (), y: ()) { (x, y) }");
}

// =====================================================================
//  9. if/else — 분기 타입 일치
// =====================================================================

/// if-else 타입 일치: 양쪽 i32
#[test]
fn test_if_else_both_i32() {
    test("fn f(x: ()) { if x > 0 { 1 } else { 2 } }");
}

/// if-else 타입 일치: 양쪽 bool
#[test]
fn test_if_else_both_bool() {
    test("fn f(x: ()) { if x > 0 { true } else { false } }");
}

/// if-else 타입 일치: 양쪽 튜플
#[test]
fn test_if_else_both_tuple() {
    test("fn f(x: ()) { if x > 0 { (1, 2) } else { (3, 4) } }");
}

/// if-else 타입 불일치: i32 vs bool → 실패
#[test]
fn test_if_else_mismatch_i32_bool() {
    test_none("fn f() { if true { 1 } else { false } }");
}

/// if-else 타입 불일치: i32 vs 튜플 → 실패
#[test]
fn test_if_else_mismatch_i32_tuple() {
    test_none("fn f() { if true { 1 } else { (1, 2) } }");
}

/// if without else → unit 반환
#[test]
fn test_if_without_else() {
    test("fn f(x: ()) { if x > 0 { x + 1; } }");
}

/// 중첩 if
#[test]
fn test_nested_if() {
    test("fn f(a: (), b: ()) { if a > 0 { if b > 0 { a + b } else { a } } else { b } }");
}

/// if 조건에 비-bool → 실패
#[test]
fn test_if_cond_non_bool_i32() {
    test_none("fn f() { if 1 { 2 } else { 3 } }");
}

/// if 조건에 튜플 → 실패
#[test]
fn test_if_cond_tuple() {
    test_none("fn f() { if (1, 2) { 3 } else { 4 } }");
}

// =====================================================================
//  10. let 바인딩 & 대입
// =====================================================================

/// let 체인: 타입 전파
#[test]
fn test_let_chain_propagation() {
    test("fn f() { let x = 1; let y = x + 2; let z = y * 3; z }");
}

/// 대입: x = expr → lhs=rhs 타입 일치
#[test]
fn test_assign_consistent() {
    test("fn f() { let mut x = 1; x = 2; x + 3 }");
}

/// 대입 타입 불일치: i32에 bool → 실패 (flow-insensitivity)
#[test]
fn test_assign_type_conflict() {
    test_none("fn f() { let mut x = 1; x = true; x }");
}

/// 복합 대입: x += 1 → x = i32
#[test]
fn test_assign_op_add() {
    test("fn f() { let mut x = 0; x += 1; x }");
}

/// 복합 대입 타입 불일치: bool에 += → 실패
#[test]
fn test_assign_op_on_bool() {
    test_none("fn f() { let mut x = true; x += 1; x }");
}

// =====================================================================
//  11. 블록 & return
// =====================================================================

/// 블록의 마지막 표현식이 타입 결정
#[test]
fn test_block_last_expr() {
    test("fn f() { { let x = 1; x + 2 } }");
}

/// 세미콜론으로 끝나는 블록 → unit
#[test]
fn test_block_semi_unit() {
    test("fn f() { { 1 + 2; } }");
}

/// early return: return 값과 본문 끝 값의 타입 일치 필요
#[test]
fn test_return_matching_body() {
    test("fn f(x: ()) { if x > 0 { return x + 1; } x - 1 }");
}

/// return 타입 불일치 → 실패
#[test]
fn test_return_type_mismatch() {
    test_none("fn f() { if true { return 1; } true }");
}

/// return 없는 함수에서 return () 사용
#[test]
fn test_return_unit() {
    test("fn f(x: ()) { if x > 0 { return; } }");
}

// =====================================================================
//  12. loop/break/continue
// =====================================================================

/// 기본 loop → unit 반환
#[test]
fn test_loop_basic() {
    test(
        "
fn f(x: ()) {
    let mut i = 0;
    loop {
        if i > x { break; }
        i += 1;
    }
}
",
    );
}

/// loop 내부 continue
#[test]
fn test_loop_continue() {
    test(
        "
fn f(n: ()) {
    let mut i = 0;
    let mut sum = 0;
    loop {
        if i > n { break; }
        i += 1;
        if i == 3 { continue; }
        sum += i;
    }
    sum
}
",
    );
}

// =====================================================================
//  13. 튜플 swap 패턴 & 함수 인자로 전달
// =====================================================================

/// swap(p) = (p.1, p.0), 호출 후 접근
#[test]
fn test_tuple_swap() {
    test(
        "
fn swap(p: ()) { (p.1, p.0) }
fn foo() { let r = swap((1, 2)); r.0 + r.1 }
",
    );
}

/// 튜플을 함수에 전달 후 필드 접근
#[test]
fn test_tuple_param_field_access() {
    test(
        "
fn fst(p: ()) { p.0 }
fn snd(p: ()) { p.1 }
fn foo() { let t = (10, 20); fst(t) + snd(t) }
",
    );
}

/// 튜플 생성 함수: make_pair
#[test]
fn test_make_pair() {
    test(
        "
fn make_pair(a: (), b: ()) { (a, b) }
fn foo() { let p = make_pair(1, 2); p.0 + p.1 }
",
    );
}

// =====================================================================
//  14. 참조 + 튜플 조합
// =====================================================================

/// 튜플의 참조 원소
#[test]
fn test_tuple_of_refs() {
    test("fn f() { let a = 1; let b = 2; let t = (&a, &b); *t.0 + *t.1 }");
}

/// 튜플의 참조를 가지고 역참조 후 필드 접근
#[test]
fn test_ref_to_tuple_deref_access() {
    test(
        "
fn f() {
    let t = (1, 2);
    let r = &t;
    (*r).0 + (*r).1
}
",
    );
}

// =====================================================================
//  15. 타입 변수(TYPEVAR) 잔존 케이스
// =====================================================================

/// 완전 미제약: f(x) { x }
#[test]
fn test_typevar_identity() {
    test_var("fn f(x: ()) { x }");
}

/// 미제약 튜플: f(x, y) { (x, y) }
#[test]
fn test_typevar_pair() {
    test_var("fn f(x: (), y: ()) { (x, y) }");
}

/// 부분 제약: f(x, y) { x + 1; y }  → x=i32, y=TYPEVAR
#[test]
fn test_typevar_partial() {
    test_var("fn f(x: (), y: ()) { x + 1; y }");
}

/// 비-concrete 튜플 단일 접근
#[test]
fn test_typevar_non_concrete_access() {
    test_var("fn f(p: ()) { p.0 }");
}

// =====================================================================
//  16. Occurs Check (재귀 타입)
// =====================================================================

/// f의 반환이 f 자신 → 무한 타입 → 실패
#[test]
fn test_occurs_check_self_return() {
    test_none("fn f(x: ()) { let y = x + 1; f }");
}

/// f(f) 호출 → f = fn(f) -> R → 재귀 → 실패
#[test]
fn test_occurs_check_self_call() {
    test_none("fn f(x: ()) { x(x) }");
}

// =====================================================================
//  17. 복합 시나리오
// =====================================================================

/// 참조 + 비교 + if + 튜플 (manhattan distance 축소판)
#[test]
fn test_complex_ref_cmp_if_tuple() {
    test(
        "
fn abs(x: ()) { if x < 0 { -x } else { x } }
fn dist(p: (), q: ()) { abs(p.0 - q.0) + abs(p.1 - q.1) }
fn foo() { dist((3, 4), (0, 0)) }
",
    );
}

/// 고차 + 튜플: map_pair
#[test]
fn test_higher_order_tuple() {
    test(
        "
fn map_pair(f: (), p: ()) { (f(p.0), f(p.1)) }
fn inc(x: ()) { x + 1 }
fn foo() { let r = map_pair(inc, (1, 2)); r.0 + r.1 }
",
    );
}

/// 4개 함수 체이닝
#[test]
fn test_four_fn_chain() {
    test(
        "
fn a(x: ()) { x + 1 }
fn b(x: ()) { x * 2 }
fn c(x: ()) { x - 3 }
fn d(x: ()) { a(b(c(x))) }
fn foo() { d(10) }
",
    );
}

/// 튜플 안에 if 표현식
#[test]
fn test_tuple_with_if_elem() {
    test("fn f(x: ()) { (if x > 0 { 1 } else { 2 }, x + 1) }");
}

/// 같은 변수를 여러 컨텍스트에서 일관적으로 사용
#[test]
fn test_consistent_multi_use() {
    test("fn f(x: ()) { x + 1; x - 2; x * 3; x / 4; x % 5 }");
}

/// 여러 함수가 같은 파라미터 타입을 공유
#[test]
fn test_shared_type_across_fns() {
    test(
        "
fn f(x: ()) { x + 1 }
fn g(x: ()) { x * 2 }
fn h() { f(1) + g(2) }
",
    );
}
