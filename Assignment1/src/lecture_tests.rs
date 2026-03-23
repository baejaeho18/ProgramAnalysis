/// 강의 슬라이드 예시 기반 추가 테스트
///
/// 03-type-1 (Lecture 3), 04-type-2 (Lecture 4) 슬라이드의 예시들을
/// 실제 구현에 넣었을 때 올바르게 동작하는지 검증합니다.
///
/// test(code)     — 해가 존재하고, 모든 타입이 구체적으로 결정됨 (TYPEVAR 없음)
/// test_var(code)  — 해가 존재하지만, 일부 타입 변수가 미결정 (TYPEVAR 있음)
/// test_none(code) — 제약이 모순되어 해가 없음 ("no solution")
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

// =====================================================================
//  03-type-1 (Lecture 3) — 제약 기반 타입 분석 기초
// =====================================================================

/// [Lecture 3, Slide 14] Example 1: fn f(x, y) { x + y }
///
/// 제약:
///   ⟦f⟧ = fn(⟦x⟧, ⟦y⟧) → ⟦x + y⟧
///   ⟦x + y⟧ = ⟦x⟧ = ⟦y⟧ = i32
///
/// 기대 결과: f: fn(i32, i32) -> i32 — 모든 타입 결정됨
#[test]
fn test_lec3_ex1_simple_addition() {
    test("fn f(x: (), y: ()) { x + y }");
}

/// [Lecture 3, Slides 15-16] Example 2: 고차 함수
///
/// fn f(x) { x }           → f: fn(X) → X
/// fn g(y) { y(1) }        → y: fn(i32) → A, g: fn(fn(i32)→A) → A
/// fn h() { g(f) }         → f가 y에 대입되므로 X = i32, A = i32
///
/// 기대 결과: f: fn(i32)->i32, g: fn(fn(i32)->i32)->i32, h: fn()->i32
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

/// [Lecture 3, Slide 17] Example 3: 타입 충돌 — bool vs i32
///
/// fn f(x) { if x { x + 1 } else { 0 } }
///
/// 제약 충돌:
///   if x  → ⟦x⟧ = bool  (조건식은 bool이어야 함)
///   x + 1 → ⟦x⟧ = i32   (덧셈의 피연산자는 i32)
///   bool ≠ i32 → 해 없음
///
/// 기대 결과: no solution
#[test]
fn test_lec3_ex3_bool_i32_conflict() {
    test_none("fn f(x: ()) { if x { x + 1 } else { 0 } }");
}

/// [Lecture 3, Slide 18] Example 4: 재귀 타입 (occurs check)
///
/// fn f(x) { let y = x + 1; f }
///
/// f의 본문이 f 자신을 반환하므로:
///   return_type(f) = type(f) = fn(i32) → fn(i32) → fn(i32) → ...
///   무한 타입 → occurs check 실패 → 해 없음
///
/// 기대 결과: no solution (resolve에서 cycle 감지)
#[test]
fn test_lec3_ex4_recursive_type() {
    test_none("fn f(x: ()) { let y = x + 1; f }");
}

/// [Lecture 3, Slide 19] Example 5: 항등 함수 (다형성)
///
/// fn f(x) { x }
///
/// 제약: ⟦f⟧ = fn(⟦x⟧) → ⟦x⟧
/// x에 대한 추가 제약이 없어 무한히 많은 해 존재:
///   fn(X) → X, fn(i32) → i32, fn(bool) → bool, ...
/// 가장 일반적인 해(principal type): fn(X) → X
///
/// 기대 결과: 해 존재, 타입 변수(TYPEVAR) 포함
#[test]
fn test_lec3_ex5_identity_polymorphic() {
    test_var("fn f(x: ()) { x }");
}

// =====================================================================
//  04-type-2 (Lecture 4) — 제약 해결, 튜플, 한계
// =====================================================================

/// [Lecture 4, Slide 11 상황] 튜플 접근 — 올바른 경우
///
/// fn f() { let x = (1, true); x.0 }
///
/// 슬라이드의 "First Attempt" 규칙으로는 no solution이었지만,
/// 구현체는 ensure_tuple로 패딩하여 올바르게 처리해야 함.
///
/// 제약:
///   ⟦x⟧ = (i32, bool)        (let 바인딩에서)
///   x.0 → ensure_tuple(x, 1)  (최소 1개 원소 보장)
///   ⟦x.0⟧ = elems[0] = i32
///
/// 기대 결과: f: fn() -> i32 — 모든 타입 결정됨
#[test]
fn test_lec4_tuple_access_first() {
    test("fn f() { let x = (1, true); x.0 }");
}

/// 튜플 두 번째 원소 접근
///
/// fn f() { let x = (1, true); x.1 }
///
/// 기대 결과: f: fn() -> bool — 모든 타입 결정됨
#[test]
fn test_lec4_tuple_access_second() {
    test("fn f() { let x = (1, true); x.1 }");
}

/// [Lecture 4, Slides 14-16] 튜플 범위 밖 접근 — Absent 타입으로 거부
///
/// fn f() { let x = (1, true); x.2 }
///
/// (1, true)는 2개 원소의 concrete 튜플.
/// x.2 접근 시 ensure_tuple이 Absent로 패딩: (i32, bool, ◇)
/// x.2 = ◇ (Absent) → 강의의 ⟦e.i⟧ ≠ ◇ 제약 위반 → no solution
///
/// 기대 결과: no solution
#[test]
fn test_lec4_tuple_oob_access() {
    test_none("fn f() { let x = (1, true); x.2 }");
}

/// [Lecture 4, Slides 20-21] Let-다형성 한계 — 단형(monomorphic) 분석
///
/// fn f(x) { x }       → f: fn(X) → X
/// fn g() { f(1); f(true) }
///
/// f(1)    → X = i32
/// f(true) → X = bool  (이미 X = i32인데 bool과 충돌!)
///
/// 단형 타입 분석에서는 f를 호출할 때마다 같은 X를 공유하므로
/// 다른 타입으로 사용할 수 없음.
///
/// 기대 결과: no solution
#[test]
fn test_lec4_let_polymorphism_limitation() {
    test_none(
        "
fn f(x: ()) { x }
fn g() { f(1); f(true) }
",
    );
}

/// 단형 분석 한계 변형: 고차 함수 인자로 전달
///
/// fn f(x) { x }
/// fn g(y) { y(1); y(true) }
/// fn h() { g(f) }
///
/// g 내부에서 y(1)과 y(true)가 충돌:
///   y(1)    → y = fn(i32) → A
///   y(true) → y = fn(bool) → B
///   i32 ≠ bool → 해 없음
///
/// 기대 결과: no solution
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

// =====================================================================
//  참조(reference)와 역참조(dereference) 관련 테스트
// =====================================================================

/// 기본 참조/역참조
///
/// let x = 1; let y = &x; *y
///
/// 제약:
///   ⟦x⟧ = i32, ⟦&x⟧ = &⟦x⟧ = &i32, ⟦y⟧ = &i32
///   *y → ⟦y⟧ = &⟦*y⟧ → ⟦*y⟧ = i32
///
/// 기대 결과: f: fn() -> i32
#[test]
fn test_ref_deref_basic() {
    test("fn f() { let x = 1; let y = &x; *y }");
}

/// 이중 참조/역참조
///
/// let x = 1; let y = &x; let z = &y; **z
///
/// ⟦z⟧ = &&i32, **z = i32
///
/// 기대 결과: f: fn() -> i32
#[test]
fn test_ref_deref_nested() {
    test("fn f() { let x = 1; let y = &x; let z = &y; **z }");
}

/// 비참조 타입에 역참조 적용 — 타입 오류
///
/// let x = 1; *x
///
/// *x 규칙: ⟦x⟧ = &⟦*x⟧ → x는 참조여야 함
/// 그런데 x = i32이므로 &T ≠ i32 → 해 없음
///
/// 기대 결과: no solution
#[test]
fn test_deref_non_reference() {
    test_none("fn f() { let x = 1; *x }");
}

/// 참조를 함수 인자로 전달
///
/// fn deref_fn(r) { *r }
/// fn foo() { let x = 1; deref_fn(&x) }
///
/// r = &i32, *r = i32, deref_fn: fn(&i32) -> i32
///
/// 기대 결과: 모든 타입 결정됨
#[test]
fn test_ref_as_argument() {
    test(
        "
fn deref_fn(r: ()) { *r }
fn foo() { let x = 1; deref_fn(&x) }
",
    );
}

// =====================================================================
//  산술/논리 연산 관련 테스트
// =====================================================================

/// 논리 연산: && (and)
///
/// ⟦x && y⟧ = bool, ⟦x⟧ = bool, ⟦y⟧ = bool
///
/// 기대 결과: f: fn(bool, bool) -> bool
#[test]
fn test_bool_and_operation() {
    test("fn f(x: (), y: ()) { x && y }");
}

/// 비교 연산이 bool을 반환하고 if 조건으로 사용
///
/// x > y → ⟦x⟧ = ⟦y⟧ = i32, 결과 = bool
/// if ... { x } else { y } → then/else 타입 일치 → i32
///
/// 기대 결과: f: fn(i32, i32) -> i32
#[test]
fn test_comparison_in_if() {
    test("fn f(x: (), y: ()) { if x > y { x } else { y } }");
}

/// 산술 + 논리 혼합 — 타입 충돌
///
/// x + y → x, y = i32
/// x || y → x, y = bool
/// i32 ≠ bool → 해 없음
///
/// 기대 결과: no solution (기존 test_fail_var_add_and_or와 동일)
#[test]
fn test_arithmetic_logic_conflict() {
    test_none("fn f(x: (), y: ()) { x + y; x || y }");
}

/// 튜플에 산술 연산 적용 — 타입 오류
///
/// (1, 2)의 타입 = (i32, i32) ≠ i32 → + 적용 불가
///
/// 기대 결과: no solution
#[test]
fn test_tuple_in_arithmetic() {
    test_none("fn f() { (1, 2) + 3 }");
}

// =====================================================================
//  복합 테스트: 강의에서 다룬 패턴 조합
// =====================================================================

/// 함수 포인터를 인자로 전달 (apply 패턴)
///
/// add1: fn(i32) -> i32
/// apply(f, x): f(x) → f = fn(typeof_x) -> result
/// foo(): apply(add1, 2) → 모두 i32로 결정
///
/// 기대 결과: 모든 타입 결정됨
#[test]
fn test_fn_ptr_apply() {
    test(
        "
fn add1(x: ()) { x + 1 }
fn apply(f: (), x: ()) { f(x) }
fn foo() { apply(add1, 2) }
",
    );
}

/// 튜플 swap 함수
///
/// swap(p): (p.1, p.0) → 튜플 원소 순서 교환
/// foo(): swap((1, 2)) → r = (i32, i32), r.0 + r.1 = i32
///
/// 기대 결과: 모든 타입 결정됨
#[test]
fn test_tuple_swap() {
    test(
        "
fn swap(p: ()) { (p.1, p.0) }
fn foo() { let r = swap((1, 2)); r.0 + r.1 }
",
    );
}

/// 상호 재귀 함수 (서로를 호출)
///
/// f(x) = x + g(x): x = i32, g(x) = i32
/// g(y) = y - f(y): y = i32, f(y) = i32
/// 제약이 일관되므로 해 존재.
///
/// 기대 결과: f: fn(i32)->i32, g: fn(i32)->i32
#[test]
fn test_mutual_recursion() {
    test(
        "
fn f(x: ()) { x + g(x) }
fn g(y: ()) { y - f(y) }
",
    );
}

/// 제약이 없는 함수 — 타입 변수 잔존
///
/// f(x, y): (x, y)를 반환하지만 x, y에 대한 제약 없음
///
/// 기대 결과: 해 존재, TYPEVAR 포함
#[test]
fn test_unconstrained_tuple() {
    test_var("fn f(x: (), y: ()) { (x, y) }");
}

/// let 바인딩 체인
///
/// let x = 1; let y = x + 2; y
/// 모든 변수가 i32로 결정됨
///
/// 기대 결과: 모든 타입 결정됨
#[test]
fn test_let_chain() {
    test("fn f() { let x = 1; let y = x + 2; y }");
}

/// if without else — unit 타입 반환
///
/// if 조건 { 본문; } (else 없음) → 결과는 () (unit)
/// then 브랜치도 unit이어야 함
/// x > 0 → x = i32
///
/// 기대 결과: 모든 타입 결정됨 (return = (), x = i32)
#[test]
fn test_if_without_else() {
    test("fn f(x: ()) { if x > 0 { x + 1; } }");
}

/// 나눗셈 (i32 산술)
///
/// x / y → ⟦x⟧ = ⟦y⟧ = ⟦x/y⟧ = i32
///
/// 기대 결과: 모든 타입 결정됨
#[test]
fn test_division() {
    test("fn f(x: (), y: ()) { x / y }");
}

/// 부정 연산자
///
/// -x → x = i32, 결과 = i32
/// !y → y = bool, 결과 = bool
/// 두 함수가 독립적이므로 각각 해 존재
///
/// 기대 결과: 모든 타입 결정됨
#[test]
fn test_unary_operators() {
    test(
        "
fn neg(x: ()) { -x }
fn not_fn(y: ()) { !y }
fn foo() { neg(1) + neg(2); !not_fn(true) }
",
    );
}

/// 빈 튜플 (unit type)
///
/// () = 빈 튜플 = unit type
///
/// 기대 결과: 모든 타입 결정됨
#[test]
fn test_unit_type() {
    test("fn f() { () }");
}

/// 중첩 튜플
///
/// (1, (2, 3)) = (i32, (i32, i32))
///
/// 기대 결과: 모든 타입 결정됨
#[test]
fn test_nested_tuple() {
    test("fn f() { let x = (1, (2, 3)); x.0 + x.1.0 + x.1.1 }");
}

/// 함수 합성 패턴
///
/// compose(f, g, x) = f(g(x))
/// double(x) = x + x
/// inc(x) = x + 1
/// foo() = compose(double, inc, 3) = double(inc(3)) = double(4) = 8
///
/// 기대 결과: 모든 타입 결정됨 (i32)
#[test]
fn test_function_composition() {
    test(
        "
fn compose(f: (), g: (), x: ()) { f(g(x)) }
fn double(x: ()) { x + x }
fn inc(x: ()) { x + 1 }
fn foo() { compose(double, inc, 3) }
",
    );
}

/// 인자 개수 불일치 — 타입 오류
///
/// f는 1개 인자, 2개로 호출 → fn 타입의 param 수 불일치 → 해 없음
///
/// 기대 결과: no solution
#[test]
fn test_arity_mismatch() {
    test_none(
        "
fn f(x: ()) { x + 1 }
fn g() { f(1, 2) }
",
    );
}
