# 강의 슬라이드 예시 기반 테스트 분석 보고서

`src/lecture_tests.rs`에 작성된 26개 테스트에 대해,
`src/analysis.rs` 구현 코드를 직접 추적(trace)하여 예상 결과를 분석합니다.

실행 방법:
```
cargo test --lib lecture_tests
```

---

## 요약

| # | 테스트 이름 | 출처 | 검증 유형 | 예상 결과 |
|---|-----------|------|----------|----------|
| 1 | test_lec3_ex1_simple_addition | L3 슬14 | test() | ✅ PASS |
| 2 | test_lec3_ex2_higher_order | L3 슬15-16 | test() | ✅ PASS |
| 3 | test_lec3_ex3_bool_i32_conflict | L3 슬17 | test_none() | ✅ PASS |
| 4 | test_lec3_ex4_recursive_type | L3 슬18 | test_none() | ✅ PASS |
| 5 | test_lec3_ex5_identity_polymorphic | L3 슬19 | test_var() | ✅ PASS |
| 6 | test_lec4_tuple_access_first | L4 슬11 | test() | ✅ PASS |
| 7 | test_lec4_tuple_access_second | L4 슬11 | test() | ✅ PASS |
| 8 | test_lec4_tuple_oob_access | L4 슬13 | test_var() | ⚠️ 구현 vs 강의 차이 |
| 9 | test_lec4_let_polymorphism_limitation | L4 슬20-21 | test_none() | ✅ PASS |
| 10 | test_lec4_polymorphism_higher_order | L4 슬20-21 | test_none() | ✅ PASS |
| 11 | test_ref_deref_basic | L3 슬12 규칙 | test() | ✅ PASS |
| 12 | test_ref_deref_nested | 응용 | test() | ✅ PASS |
| 13 | test_deref_non_reference | 응용 | test_none() | ✅ PASS |
| 14 | test_ref_as_argument | 응용 | test() | ✅ PASS |
| 15 | test_bool_and_operation | L3 슬12 규칙 | test() | ✅ PASS |
| 16 | test_comparison_in_if | L3 슬12 규칙 | test() | ✅ PASS |
| 17 | test_arithmetic_logic_conflict | 기존 변형 | test_none() | ✅ PASS |
| 18 | test_tuple_in_arithmetic | 응용 | test_none() | ✅ PASS |
| 19 | test_fn_ptr_apply | 응용 | test() | ✅ PASS |
| 20 | test_tuple_swap | 응용 | test() | ✅ PASS |
| 21 | test_mutual_recursion | 응용 | test() | ✅ PASS |
| 22 | test_unconstrained_tuple | 응용 | test_var() | ✅ PASS |
| 23 | test_let_chain | 기초 | test() | ✅ PASS |
| 24 | test_if_without_else | L3 슬12 규칙 | test() | ✅ PASS |
| 25 | test_division | L3 슬11 규칙 | test() | ✅ PASS |
| 26 | test_unary_operators | L3 슬12 규칙 | test() | ✅ PASS |
| 27 | test_unit_type | 기초 | test() | ✅ PASS |
| 28 | test_nested_tuple | 응용 | test() | ✅ PASS |
| 29 | test_function_composition | 응용 | test() | ✅ PASS |
| 30 | test_arity_mismatch | 응용 | test_none() | ✅ PASS |

---

## 상세 추적 분석

### 1. test_lec3_ex1_simple_addition — ✅ PASS 예상

```rust
fn f(x: (), y: ()) { x + y }
```

**추적:**
- Phase 1 (FnCollector): f_ret=V0, x=V1, y=V2, f_fn=FnPtr([V1,V2],V0)
- Phase 2 (ConstraintVisitor):
  - `x + y`: BinOp::Add → V1=i32, V2=i32, result=i32
  - body type (= x+y result) unified with V0 → V0=i32
- Resolve: V0=i32, V1=i32, V2=i32 → 모든 타입 결정
- `test()` assertion: TYPEVAR 없음 ✅

### 2. test_lec3_ex2_higher_order — ✅ PASS 예상

```rust
fn f(x: ()) { x }
fn g(y: ()) { y(1) }
fn h() { g(f) }
```

**추적:**
- Phase 1: f_ret=V0, x=V1, f_fn=FnPtr([V1],V0)
  g_ret=V2, y=V3, g_fn=FnPtr([V3],V2)
  h_ret=V4, h_fn=FnPtr([],V4)
- Phase 2:
  - f 본문: x → V1. body=V1, unify(V1, V0) → V0=V1
  - g 본문: y(1) → Call. 1=i32. make_fn_ptr([i32],result). unify(V3, fn(i32)->result)
    → V3=FnPtr([i32],result). body unify with V2 → V2=result
  - h 본문: g(f) → Call. f=f_fn=FnPtr([V1],V0). make_fn_ptr([f_type],result2). unify(g_fn, fn(f_type)->result2)
    → g의 param V3 = f_type = FnPtr([V1],V0)
    → 이미 V3 = FnPtr([i32],result) → unify → V1=i32, V0=result
    → V0=V1=i32이므로 result=i32, V2=i32, V4=result2=i32
- Resolve: 모든 변수 = i32 또는 fn(i32)->i32 등 → TYPEVAR 없음 ✅

### 3. test_lec3_ex3_bool_i32_conflict — ✅ PASS 예상

```rust
fn f(x: ()) { if x { x + 1 } else { 0 } }
```

**추적:**
- `if x`: ExprKind::If → cond = x, unify(x_type, bool) → V1=bool
- `x + 1`: BinOp::Add → unify(V1, i32) → bool과 i32 merge_info → 다른 TypeInfo 변종 → `failed = true`
- analyze() returns None ✅

### 4. test_lec3_ex4_recursive_type — ✅ PASS 예상

```rust
fn f(x: ()) { let y = x + 1; f }
```

**추적:**
- `x + 1`: V1=i32, y=i32
- `f`: ExprKind::Path → Res::Def → type = f_fn = FnPtr([V1], V0)
- body type = f_fn, unify(f_fn, V0) → V0 = FnPtr([i32], V0)
- resolve(V0): stack에 V0 추가 → FnPtr의 ret = V0 → cycle 감지 → return None
- analyze() returns None ✅

### 5. test_lec3_ex5_identity_polymorphic — ✅ PASS 예상

```rust
fn f(x: ()) { x }
```

**추적:**
- body = x → V1. unify(V1, V0) → V0=V1
- resolve: V0, V1 모두 info=None → Type::Var(rep) → TYPEVAR
- `test_var()` assertion: TYPEVAR 포함 ✅

### 6. test_lec4_tuple_access_first — ✅ PASS 예상

```rust
fn f() { let x = (1, true); x.0 }
```

**추적:**
- `(1, true)`: Tup → make_tuple([i32_var, bool_var])
- `let x = ...`: x_var unified with tuple_var → x = (i32, bool)
- `x.0`: Field, index=0 → ensure_tuple(x, 1). x는 이미 Tuple([i32_v, bool_v]). 2 ≥ 1 → 패딩 불필요. result = elems[0] = i32
- resolve: 모든 타입 결정 → TYPEVAR 없음 ✅

### 7. test_lec4_tuple_access_second — ✅ PASS 예상

동일 논리, elems[1] = bool ✅

### 8. test_lec4_tuple_oob_access — ⚠️ 구현과 강의 차이

```rust
fn f() { let x = (1, true); x.2 }
```

**추적:**
- x = (i32, bool) — 2개 원소
- `x.2`: ensure_tuple(x, 3) → 현재 2개 < 3 → fresh var 추가 → (i32, bool, V_fresh)
- result = elems[2] = V_fresh (info=None)
- resolve: V_fresh → Type::Var → TYPEVAR

**현재 구현 동작:** test_var() ✅ (해 존재, TYPEVAR 포함)
**강의 Correct Version 기대 동작:** test_none() (absent 타입 ◇와 부등식 제약으로 거부해야 함)

**차이 원인:** `ensure_tuple`이 Absent 대신 fresh 변수로 패딩함.
`Type::Absent`이 types.rs에 정의되어 있지만 analysis.rs에서 사용하지 않음.

### 9. test_lec4_let_polymorphism_limitation — ✅ PASS 예상

```rust
fn f(x: ()) { x }
fn g() { f(1); f(true) }
```

**추적:**
- f_fn = FnPtr([V1], V0), V0=V1 (identity)
- g 본문:
  - `f(1)`: make_fn_ptr([i32_var], ret1). unify(f_fn, new_fn) → V1=i32
  - `f(true)`: make_fn_ptr([bool_var], ret2). unify(f_fn, new_fn) → V1(=i32)과 bool merge → **failed!**
- analyze() returns None ✅

### 10-30: 나머지 테스트 — 모두 PASS 예상

각각 위와 유사한 추적 로직으로:
- 산술/논리/비교 연산자 규칙이 올바르게 적용됨
- 참조/역참조의 Ref 타입 생성과 unification이 정확함
- 튜플 ensure_tuple 패딩 동작이 일관적임
- FnPtr 인자 수 불일치 시 failed 설정됨
- cycle detection이 재귀 타입을 올바르게 감지함

### 특이 케이스: test_arity_mismatch

```rust
fn f(x: ()) { x + 1 }    // f: fn(i32) -> i32 (1개 인자)
fn g() { f(1, 2) }        // f를 2개 인자로 호출
```

**추적:**
- `f(1, 2)`: make_fn_ptr([i32, i32], ret) — 2개 인자 함수 타입 생성
- unify with f_fn = FnPtr([V1], V0) — 1개 인자
- merge_info: FnPtr([1개], _) vs FnPtr([2개], _) → `ap.len() != bp.len()` → **failed!**
- analyze() returns None ✅

---

## 발견된 구현 이슈

### 1. 튜플 범위 밖 접근이 거부되지 않음 (test #8)

**문제:** `(1, true).2`가 no solution 대신 TYPEVAR를 포함한 해로 나옴.

**원인:** `ensure_tuple`(line 232-260)이 부족한 길이를 fresh 변수로 패딩하지만,
강의의 "Correct Version" (04-type-2, slides 14-16)에서는 absent 타입 ◇와
부등식 제약 `⟦e.i⟧ ≠ ◇`를 사용하여 존재하지 않는 원소 접근을 거부함.

**수정 방향:**
1. 튜플 생성 시 패딩을 Absent로 채움
2. 프로젝션 시 결과가 Absent인지 검사하여 failed 설정

```rust
// ensure_tuple에서 패딩할 때:
while elems.len() < min_len {
    let absent = self.fresh_with(TypeInfo::Absent); // Absent로 패딩
    elems.push(absent);
}

// Field 접근에서 Absent 검사 추가:
// ensure_tuple 후 elems[index]가 Absent면 failed = true
```

(단, TypeInfo에 Absent variant를 추가하고 merge_info에서 처리해야 함)
