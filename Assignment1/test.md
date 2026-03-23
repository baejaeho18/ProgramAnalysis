# 전체 테스트 케이스 상세 해설

이 문서는 tests.rs, lecture_tests.rs, custom_tests.rs의 모든 테스트 케이스에 대해, 어떤 상황을 검증하는지, 어떤 제약이 생성되어 어떻게 풀리는지, 왜 그 결과가 나오는지를 상세하게 설명합니다.

## 테스트 헬퍼 함수

세 파일 모두 동일한 헬퍼를 사용합니다:

- **`test(code)`**: 분석이 성공(`Some`)하고, Rust 타입 체크도 통과하며, 결과에 `TYPEVAR`이 없는 경우 (모든 타입이 구체적으로 결정됨)
- **`test_var(code)`**: 분석이 성공하고 Rust 타입 체크도 통과하지만, 결과에 `TYPEVAR`이 남아 있는 경우 (일부 타입 변수가 미결정)
- **`test_none(code)`**: 분석이 "no solution" (`None`)을 반환하는 경우 (제약 모순 또는 재귀 타입)

---

# Part 1: tests.rs — 기본 제공 테스트 (5개)

---

### test_binary_search

```rust
fn binary_search(arr: (), len: (), target: ()) {
    let mut lo = 0;
    let mut hi = len - 1;
    loop {
        if lo > hi { return (false, 0); }
        let mid = lo + (hi - lo) / 2;
        let val = arr(mid);
        if val == target { return (true, mid); }
        if val < target { lo = mid + 1; } else { hi = mid - 1; }
    }
    (false, 0)
}
fn my_arr(i: ()) { i * 2 }
fn foo() {
    let r = binary_search(my_arr, 10, 6);
    if r.0 { r.1 } else { -1 }
}
```

**상정 상황**: 실제 프로그램 수준의 복잡한 코드에서 모든 타입이 올바르게 추론되는지 검증.

**제약 생성 과정**:

1. `lo = 0`, `hi = len - 1` → `⟦lo⟧ = i32`, `⟦len⟧ = i32` (산술 연산) → `⟦hi⟧ = i32`
2. `lo > hi` → 비교 → `⟦lo⟧ = ⟦hi⟧ = i32`, 결과 bool → if 조건 OK
3. `return (false, 0)` → `⟦ret⟧ = (bool, i32)` (concrete 튜플)
4. `lo + (hi - lo) / 2` → 모든 피연산자 i32 → `⟦mid⟧ = i32`
5. `arr(mid)` → `⟦arr⟧ = fn(i32) → ⟦val⟧` → 이미 arr의 타입 변수 존재
6. `val == target` → `⟦val⟧ = ⟦target⟧ = i32`, 결과 bool
7. `return (true, mid)` → `⟦ret⟧ = (bool, i32)` → 이전 return과 일치
8. `lo = mid + 1` → Assign: `⟦lo⟧ = ⟦mid + 1⟧ = i32` → 일관
9. `(false, 0)` (마지막) → 본문 타입 = (bool, i32) → `⟦ret⟧`와 일치
10. `my_arr(i) { i * 2 }` → `⟦i⟧ = i32` → `⟦my_arr⟧ = fn(i32) → i32`
11. `binary_search(my_arr, 10, 6)` → arr = my_arr이므로 `⟦arr⟧ = fn(i32) → i32`, val = i32 → target = i32 확인
12. `r.0` → bool, `if r.0 { r.1 } else { -1 }` → then = i32, else = i32 → OK

**결과**: ✅ test — 모든 타입 결정됨. binary_search: fn(fn(i32)->i32, i32, i32) -> (bool, i32)

---

### test_manhattan

```rust
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
}
```

**상정 상황**: 튜플, 필드 접근, 참조, 역참조, 비교, if-else, 함수 체이닝이 모두 조합된 실전적 코드.

**제약 생성 과정**:

1. `make_point(x, y) { (x, y) }` → `⟦make_point⟧ = fn(X, Y) → (X, Y)`, x와 y는 미결정
2. `p.0 - q.0` → `ensure_tuple(p, 1)`, `ensure_tuple(q, 1)` → p와 q는 non-concrete 튜플. p.0 = i32, q.0 = i32 (뺄셈)
3. `p.1 - q.1` → p의 두 번째 원소도 i32, q의 두 번째 원소도 i32 → p, q = (i32, i32)
4. `dx < 0` → dx = i32 (이미 확인), 결과 bool → if 조건 OK
5. `-dx` → dx = i32, `dx` → i32 → then/else 둘 다 i32 → adx = i32
6. `adx + ady` → i32 → manhattan 반환 = i32
7. `&manhattan(p, origin)` → `⟦dp⟧ = &i32`
8. `*dp <= *dq` → `*dp` = i32, `*dq` = i32 → 비교 → bool
9. `if ... { p } else { q }` → then = p, else = q → p와 q의 타입이 같아야 함 (이미 둘 다 (i32, i32))
10. `make_point(3, 4)` → X = i32, Y = i32 → make_point: fn(i32, i32) → (i32, i32)
11. `c.0 + c.1` → c = (i32, i32) → i32 + i32 = i32

**결과**: ✅ test — make_point: fn(i32,i32)->(i32,i32), manhattan: fn((i32,i32),(i32,i32))->i32, closer: fn((i32,i32),(i32,i32),(i32,i32))->(i32,i32)

---

### test_var_identity

```rust
fn foo(x: ()) { x }
```

**상정 상황**: 인자에 아무 연산도 하지 않으면 타입이 결정되지 않는지 검증.

**제약**: `⟦foo⟧ = fn(⟦x⟧) → ⟦x⟧`. x에 대한 추가 제약 없음.

**결과**: ✅ test_var — `fn(TYPEVAR) → TYPEVAR`. 슬라이드 03 Example 5의 principal type `fn(X) → X`에 대응.

---

### test_fail_add_bool

```rust
fn foo() { 1 + true }
```

**상정 상황**: i32와 bool을 더하면 실패하는지 검증.

**제약**:
- `⟦1⟧ = i32`
- `⟦true⟧ = bool`
- `1 + true` → `⟦1⟧ = ⟦true⟧ = i32` (산술 규칙: 양쪽 i32)
- `unify(bool, i32)` → **I32 ≠ Bool → failed!**

**결과**: ✅ test_none — merge_info의 catch-all `_ => failed = true`에 걸림.

---

### test_fail_var_add_and_or

```rust
fn foo(x: (), y: ()) { x + y; x || y; }
```

**상정 상황**: 같은 변수를 산술과 논리 연산에 동시에 사용하면 충돌하는지 검증.

**제약**:
- `x + y` → `⟦x⟧ = ⟦y⟧ = i32`
- `x || y` → `⟦x⟧ = ⟦y⟧ = bool`
- `unify(i32, bool)` → **충돌!**

**결과**: ✅ test_none

---

# Part 2: lecture_tests.rs — 슬라이드 직접 인용 (10개)

---

### test_lec3_ex1_simple_addition

```rust
fn f(x: (), y: ()) { x + y }
```

**슬라이드**: 03-type-1, Slide 14, Example 1

**제약**:
- Phase 1: `⟦f⟧ = fn(⟦x⟧, ⟦y⟧) → V_ret`
- Phase 2: `x + y` → `⟦x⟧ = i32`, `⟦y⟧ = i32`, `⟦x+y⟧ = i32`
- 본문 타입 = `⟦x+y⟧ = i32` → `unify(V_ret, i32)`

**결과**: ✅ test — `f: fn(i32, i32) → i32`

---

### test_lec3_ex2_higher_order

```rust
fn f(x: ()) { x }
fn g(y: ()) { y(1) }
fn h() { g(f) }
```

**슬라이드**: 03-type-1, Slides 15-16, Example 2

**제약**:
- Phase 1: `⟦f⟧ = fn(V_x) → V_rf`, `⟦g⟧ = fn(V_y) → V_rg`, `⟦h⟧ = fn() → V_rh`
- f의 본문 `x` → `V_rf = V_x` → f: fn(X) → X
- g의 본문 `y(1)`:
  - `⟦1⟧ = i32`
  - `y(1)` → `⟦y⟧ = fn(i32) → ⟦y(1)⟧` → `unify(V_y, fn(i32) → A)`
  - `V_rg = A`
- h의 본문 `g(f)`:
  - `⟦g(f)⟧`: `⟦g⟧ = fn(⟦f⟧) → ⟦g(f)⟧`
  - `unify(V_y, ⟦f⟧)` → `fn(i32) → A = fn(V_x) → V_x` → `V_x = i32, A = i32`

**결과**: ✅ test — `f: fn(i32)->i32`, `g: fn(fn(i32)->i32)->i32`, `h: fn()->i32`

---

### test_lec3_ex3_bool_i32_conflict

```rust
fn f(x: ()) { if x { x + 1 } else { 0 } }
```

**슬라이드**: 03-type-1, Slide 17, Example 3

**제약**:
- `if x` → `⟦x⟧ = bool` (조건식은 bool)
- `x + 1` → `⟦x⟧ = i32` (산술)
- `unify(bool, i32)` → **충돌!**

**결과**: ✅ test_none — "⟦x⟧ cannot be both (⟦x.0⟧) and (i32, bool)"

---

### test_lec3_ex4_recursive_type

```rust
fn f(x: ()) { let y = x + 1; f }
```

**슬라이드**: 03-type-1, Slide 18, Example 4

**제약**:
- `x + 1` → `⟦x⟧ = i32, ⟦y⟧ = i32`
- 본문의 마지막 표현식이 `f` → `⟦본문⟧ = ⟦f⟧`
- `⟦f⟧ = fn(i32) → ⟦f⟧` → 재귀 타입!
- `resolve_inner`에서 stack에 이미 f의 대표가 있으므로 → `None` 반환

**결과**: ✅ test_none — occurs check. `μX. fn(i32) → X`는 우리 타입 시스템에 없음.

---

### test_lec3_ex5_identity_polymorphic

```rust
fn f(x: ()) { x }
```

**슬라이드**: 03-type-1, Slide 19, Example 5

**제약**: `⟦f⟧ = fn(V_x) → V_x`. x에 대한 추가 제약 없음 → V_x는 `None` (미결정).

**resolve 결과**: `fn(TYPEVAR) → TYPEVAR` — principal type.

**결과**: ✅ test_var

---

### test_lec4_tuple_access_first

```rust
fn f() { let x = (1, true); x.0 }
```

**슬라이드**: 04-type-2, Slide 15, Correct Version Example 1

**제약**:
- `(1, true)` → `make_concrete_tuple([i32, bool])` → x = Tuple([V_i32, V_bool], **true**)
- `x.0` → `ensure_tuple(x, 1)` → 이미 Tuple이고 len(2) ≥ 1 → 패딩 불필요
- `⟦x.0⟧ = elems[0]` → `unify(ty, V_i32)` → ty = i32
- Phase 3: `field_access_vars`에서 ty 확인 → i32 ≠ Absent → OK

**결과**: ✅ test — `f: fn() → i32`

---

### test_lec4_tuple_oob_access

```rust
fn f() { let x = (1, true); x.2 }
```

**슬라이드**: 04-type-2, Slide 16, Correct Version Example 2

**제약**:
- `(1, true)` → x = Tuple([i32, bool], **true**) — concrete
- `x.2` → `ensure_tuple(x, 3)` → len(2) < 3 → **concrete이므로 Absent로 패딩**
  - x = Tuple([i32, bool, **Absent**], true)
- `⟦x.2⟧ = elems[2]` → ty = Absent
- Phase 3: `is_absent(ty)` → **true** → return None

**결과**: ✅ test_none — `⟦e.i⟧ ≠ ◇` 위반.

---

### test_lec4_let_polymorphism_limitation

```rust
fn f(x: ()) { x }
fn g() { f(1); f(true) }
```

**슬라이드**: 04-type-2, Slides 19-20

**제약**:
- `⟦f⟧ = fn(V_x) → V_x`
- `f(1)` → `⟦f⟧ = fn(i32) → A` → `unify(V_x, i32)` → V_x = i32
- `f(true)` → `⟦f⟧ = fn(bool) → B` → `unify(V_x, bool)` → V_x(=i32)와 bool 통합 → **충돌!**

단형(monomorphic) 분석이므로 f의 V_x는 프로그램 전체에서 하나. 다형성(let-polymorphism)이 없으면 같은 함수를 다른 타입으로 호출할 수 없음.

**결과**: ✅ test_none

---

### test_lec4_polymorphism_higher_order

```rust
fn f(x: ()) { x }
fn g(y: ()) { y(1); y(true) }
fn h() { g(f) }
```

**슬라이드**: 04-type-2, Slide 21

**제약**:
- `y(1)` → `⟦y⟧ = fn(i32) → A`
- `y(true)` → `⟦y⟧ = fn(bool) → B`
- `unify(fn(i32)→A, fn(bool)→B)` → 인자 `unify(i32, bool)` → **충돌!**

y가 파라미터(higher-rank position)이므로 다형적으로 인스턴스화할 수 없음.

**결과**: ✅ test_none

---

### test_lec4_polymorphic_recursion

```rust
fn f(x: (), n: ()) {
    if n > 1 { f(true, n - 1); }
    else if n == 1 { f(0, n - 1); }
    x
}
```

**슬라이드**: 04-type-2, Slide 22

**제약**:
- `n > 1` → `⟦n⟧ = i32`
- `f(true, n-1)` → `⟦f⟧ = fn(bool, i32) → R` → `unify(V_x, bool)` → V_x = bool
- `f(0, n-1)` → `⟦f⟧ = fn(i32, i32) → R` → `unify(V_x, i32)` → V_x(=bool)와 i32 → **충돌!**

각 재귀 호출이 x에 다른 타입을 요구하므로, 다형적 재귀(polymorphic recursion) 없이는 불가능.

**결과**: ✅ test_none

---

### test_lec4_projection_on_non_tuple_i32

```rust
fn f() { let x = 1; x.0 }
```

**슬라이드**: 04-type-2, Slide 9 — "Projection on non-tuple types should be rejected"

**제약**:
- `⟦x⟧ = i32` (리터럴)
- `x.0` → `ensure_tuple(x, 1)` → info = Some(TypeInfo::I32) → `Some(other)` 분기 → **failed = true**

**결과**: ✅ test_none — ensure_tuple이 i32를 튜플로 만들 수 없음.

---

### test_lec4_flow_insensitivity

```rust
fn f(x: ()) {
    let y = x + 2;
    if x > 0 { y } else { 0 };
    if x { 1 } else { 0 }
}
```

**슬라이드**: 04-type-2, Slide 18

**제약**:
- `x + 2` → `⟦x⟧ = i32`
- `x > 0` → `⟦x⟧ = i32` (일관)
- `if x { 1 } else { 0 }` → `⟦x⟧ = bool` (if 조건)
- `unify(i32, bool)` → **충돌!**

분석은 flow-insensitive이므로 프로그램 순서를 무시하고 x에 대한 모든 제약을 동시에 적용. x가 i32이면서 bool일 수 없음.

**결과**: ✅ test_none

---

# Part 3: custom_tests.rs — Edge Case 테스트

---

## 1. 리터럴 & 기본 타입

### test_literal_i32

```rust
fn f() { 42 }
```

**상정**: 가장 단순한 케이스. i32 리터럴만 반환하는 함수.

**제약**: `⟦42⟧ = i32`, `⟦f⟧ = fn() → i32`.

**결과**: ✅ test

---

### test_literal_bool

```rust
fn f() { true }
```

**상정**: bool 리터럴만 반환.

**제약**: `⟦true⟧ = bool`, `⟦f⟧ = fn() → bool`.

**결과**: ✅ test

---

### test_literal_unit

```rust
fn f() { () }
```

**상정**: 빈 튜플(unit) 반환.

**제약**: `⟦()⟧ = Tuple([], false)`, `⟦f⟧ = fn() → ()`.

**결과**: ✅ test

---

## 2. 산술 연산 — 모든 연산자

### test_arith_add ~ test_arith_shr (10개)

```rust
fn f(x: (), y: ()) { x + y }   // add
fn f(x: (), y: ()) { x - y }   // sub
fn f(x: (), y: ()) { x * y }   // mul
fn f(x: (), y: ()) { x / y }   // div
fn f(x: (), y: ()) { x % y }   // rem
fn f(x: (), y: ()) { x ^ y }   // bitxor
fn f(x: (), y: ()) { x & y }   // bitand
fn f(x: (), y: ()) { x | y }   // bitor
fn f(x: (), y: ()) { x << y }  // shl
fn f(x: (), y: ()) { x >> y }  // shr
```

**상정**: 분석이 지원하는 모든 산술/비트 연산자가 올바르게 `⟦lhs⟧ = ⟦rhs⟧ = ⟦결과⟧ = i32` 제약을 생성하는지 검증.

**제약 (모두 동일 패턴)**:
- `⟦x⟧ = i32`, `⟦y⟧ = i32`, `⟦x op y⟧ = i32`
- 이들은 `BinOpKind::Add | Sub | Mul | Div | Rem | BitXor | BitAnd | BitOr | Shl | Shr` 분기에서 처리

**결과**: ✅ test — 각각 `f: fn(i32, i32) → i32`

---

### test_arith_chain

```rust
fn f(a: (), b: (), c: (), d: (), e: ()) { (a + b) * (c - d) / e }
```

**상정**: 중첩 산술이 올바르게 전파되는지 검증.

**제약**: 모든 부분식이 i32 규칙을 따름.
- `a + b` → a=b=i32, 결과 i32
- `c - d` → c=d=i32, 결과 i32
- `(a+b) * (c-d)` → i32 * i32 = i32
- `... / e` → e = i32, 결과 i32

**결과**: ✅ test — `f: fn(i32, i32, i32, i32, i32) → i32`

---

### test_arith_on_bool

```rust
fn f() { true + false }
```

**상정**: bool에 산술 연산 적용 시 실패 확인.

**제약**: `⟦true⟧ = bool`, `+` 규칙 → `⟦true⟧ = i32` → `unify(bool, i32)` → **충돌**

**결과**: ✅ test_none

---

### test_arith_on_tuple

```rust
fn f() { (1, 2) + 3 }
```

**상정**: 튜플에 산술 연산 적용 시 실패 확인.

**제약**: `⟦(1,2)⟧ = Tuple([i32, i32], true)`, `+` 규칙 → `⟦(1,2)⟧ = i32` → `unify(Tuple, I32)` → **충돌**

**결과**: ✅ test_none — merge_info catch-all

---

### test_arith_on_ref

```rust
fn f() { let x = &1; x + 1 }
```

**상정**: 참조 타입에 산술 연산 적용 시 실패.

**제약**: `⟦&1⟧ = Ref(i32)`, `x = Ref(i32)`, `+` 규칙 → `⟦x⟧ = i32` → `unify(Ref(i32), i32)` → **충돌**

**결과**: ✅ test_none

---

### test_arith_on_fn

```rust
fn g(x: ()) { x }
fn f() { g + 1 }
```

**상정**: 함수 타입에 산술 연산 적용 시 실패.

**제약**: `⟦g⟧ = FnPtr([V_x], V_x)`, `+` 규칙 → `⟦g⟧ = i32` → `unify(FnPtr, I32)` → **충돌**

**결과**: ✅ test_none

---

## 3. 논리 연산

### test_logic_and / test_logic_or

```rust
fn f(x: (), y: ()) { x && y }
fn f(x: (), y: ()) { x || y }
```

**상정**: 논리 연산이 `⟦lhs⟧ = ⟦rhs⟧ = ⟦결과⟧ = bool` 제약을 생성하는지 확인.

**제약**: `⟦x⟧ = bool`, `⟦y⟧ = bool`, `⟦x && y⟧ = bool`

**결과**: ✅ test — `f: fn(bool, bool) → bool`

---

### test_logic_on_i32

```rust
fn f() { 1 && 2 }
```

**상정**: i32에 논리 연산 적용 시 실패.

**제약**: `⟦1⟧ = i32`, `&&` 규칙 → `⟦1⟧ = bool` → `unify(i32, bool)` → **충돌**

**결과**: ✅ test_none

---

### test_arith_and_logic_same_var

```rust
fn f(x: (), y: ()) { x + y; x && y }
```

**상정**: 같은 변수를 산술+논리에 동시 사용 → 충돌.

**제약**: `x + y` → x=i32. `x && y` → x=bool. `unify(i32, bool)` → **충돌**

**결과**: ✅ test_none

---

## 4. 비교 연산

### test_cmp_eq ~ test_cmp_ge (6개)

```rust
fn f(x: (), y: ()) { x == y }  // eq
fn f(x: (), y: ()) { x != y }  // ne
fn f(x: (), y: ()) { x < y }   // lt
fn f(x: (), y: ()) { x <= y }  // le
fn f(x: (), y: ()) { x > y }   // gt
fn f(x: (), y: ()) { x >= y }  // ge
```

**상정**: 모든 비교 연산자가 `⟦lhs⟧ = ⟦rhs⟧ = i32, ⟦결과⟧ = bool` 제약을 생성하는지 검증.

**결과**: ✅ test — 각각 `f: fn(i32, i32) → bool`. 반환 타입이 bool이므로 TYPEVAR 없이 결정.

---

### test_cmp_in_if_condition

```rust
fn f(x: (), y: ()) { if x > y { x } else { y } }
```

**상정**: 비교 결과(bool)가 if 조건으로 연결되고, then/else의 타입이 일치하는지 확인.

**제약**:
- `x > y` → x=y=i32, 결과=bool → if 조건 OK
- then = x(i32), else = y(i32) → `unify(ty, i32)` 두 번 → 일관

**결과**: ✅ test — `f: fn(i32, i32) → i32`

---

### test_cmp_result_in_logic

```rust
fn f(x: (), y: ()) { (x > 0) && (y < 10) }
```

**상정**: 비교 결과(bool)를 논리 연산에 사용하는 조합.

**제약**:
- `x > 0` → x=i32, 결과=bool
- `y < 10` → y=i32, 결과=bool
- `bool && bool` → bool

**결과**: ✅ test — `f: fn(i32, i32) → bool`

---

## 5. 단항 연산

### test_unary_neg

```rust
fn f(x: ()) { -x }
```

**제약**: `UnOp::Neg` → `⟦x⟧ = i32`, `⟦-x⟧ = i32`

**결과**: ✅ test — `f: fn(i32) → i32`

---

### test_unary_not

```rust
fn f(x: ()) { !x }
```

**제약**: `UnOp::Not` → `⟦x⟧ = bool`, `⟦!x⟧ = bool`

**결과**: ✅ test — `f: fn(bool) → bool`

---

### test_unary_double_neg

```rust
fn f(x: ()) { -(-x) }
```

**제약**: 내부 `-x` → x=i32, 결과=i32. 외부 `-(i32)` → i32. 이중 부정.

**결과**: ✅ test — `f: fn(i32) → i32`

---

### test_unary_double_not

```rust
fn f(x: ()) { !(!x) }
```

**제약**: 내부 `!x` → x=bool, 결과=bool. 외부 `!(bool)` → bool.

**결과**: ✅ test — `f: fn(bool) → bool`

---

### test_neg_on_bool

```rust
fn f(x: ()) { -x; !x }
```

**상정**: 같은 변수에 `-` (i32 요구)와 `!` (bool 요구)를 동시 적용 → 충돌.

**제약**: `-x` → x=i32. `!x` → x=bool. `unify(i32, bool)` → **충돌**

**결과**: ✅ test_none

---

## 6. 참조 & 역참조

### test_ref_deref_roundtrip

```rust
fn f() { let x = 1; *(&x) }
```

**제약**: `x = i32`. `&x` → Ref(i32). `*(&x)` → Deref 규칙: `⟦operand⟧ = Ref(⟦결과⟧)` → `Ref(i32) = Ref(ty)` → ty = i32.

**결과**: ✅ test — `f: fn() → i32`. 참조→역참조 왕복.

---

### test_double_ref

```rust
fn f() { let x = 1; let y = &x; let z = &y; **z }
```

**제약**: x=i32, y=&i32, z=&&i32. `*z` → &i32. `**z` → i32.

**결과**: ✅ test — `f: fn() → i32`

---

### test_triple_ref

```rust
fn f() { let x = 1; let y = &x; let z = &y; let w = &z; ***w }
```

**제약**: x=i32, y=&i32, z=&&i32, w=&&&i32. `***w` → i32.

**결과**: ✅ test — 3중 참조 역참조.

---

### test_ref_as_fn_arg

```rust
fn deref_add(r: ()) { *r + 1 }
fn foo() { let x = 10; deref_add(&x) }
```

**제약**:
- `*r` → r = Ref(A). `*r + 1` → A = i32 → r = &i32
- `deref_add(&x)` → &x = &i32 → x = i32 → 인자 타입 일치

**결과**: ✅ test — `deref_add: fn(&i32) → i32`

---

### test_deref_non_ref_i32 / test_deref_non_ref_bool / test_deref_non_ref_tuple

```rust
fn f() { let x = 1; *x }       // i32 역참조
fn f() { *true }                 // bool 역참조
fn f() { *(1, 2) }              // 튜플 역참조
```

**상정**: Ref가 아닌 타입에 역참조(*)를 적용하면 실패하는지 검증.

**제약 (test_deref_non_ref_i32 예시)**:
- `x = i32`
- `*x` → Deref 규칙: `⟦x⟧ = Ref(ty)` → `make_ref(ty)` 생성 후 `unify(i32, Ref(ty))` → **I32 ≠ Ref → 충돌**

**결과**: ✅ test_none (세 개 모두)

---

### test_partial_deref

```rust
fn f() {
    let x = 1;
    let y = &x;
    let z = &y;
    let w = *z;   // w = &i32
    *w             // i32
}
```

**상정**: 이중 참조를 한 번만 역참조하면 아직 참조가 남는지 확인.

**제약**: z = &&i32. `*z` → &i32 → w = &i32. `*w` → i32.

**결과**: ✅ test

---

### test_ref_in_tuple

```rust
fn f() { let x = 1; let t = (&x, &x); *t.0 + *t.1 }
```

**상정**: 튜플에 참조를 넣고 꺼내서 역참조하는 조합.

**제약**: x=i32. t = (&i32, &i32). t.0 = &i32. `*t.0` = i32. `*t.0 + *t.1` = i32.

**결과**: ✅ test

---

## 7. 튜플

### test_tuple_access_second

```rust
fn f() { let x = (1, true); x.1 }
```

**제약**: x = (i32, bool, true). x.1 = bool.

**결과**: ✅ test — `f: fn() → bool`

---

### test_nested_tuple_access

```rust
fn f() { let x = (1, (2, 3)); x.0 + x.1.0 + x.1.1 }
```

**제약**: x = (i32, (i32, i32)). x.0 = i32, x.1 = (i32, i32), x.1.0 = i32, x.1.1 = i32. 전부 i32 덧셈.

**결과**: ✅ test

---

### test_deeply_nested_tuple

```rust
fn f() { let x = ((1, 2), (3, 4)); x.0.0 + x.0.1 + x.1.0 + x.1.1 }
```

**상정**: 2×2 중첩 튜플의 모든 원소 접근.

**결과**: ✅ test

---

### test_triple_tuple

```rust
fn f() { let x = (1, 2, 3); x.0 + x.1 + x.2 }
```

**상정**: 3개 원소 concrete 튜플의 모든 유효 인덱스 접근.

**결과**: ✅ test

---

### test_mixed_type_tuple / test_mixed_tuple_bool_access

```rust
fn f() { let x = (1, true, 2); x.0 + x.2 }
fn f() { let x = (1, true, 2); if x.1 { x.0 } else { x.2 } }
```

**상정**: 혼합 타입 튜플에서 i32 원소는 산술에, bool 원소는 if 조건에 사용.

**제약 (두 번째)**:
- x.1 = bool → if 조건 OK
- x.0 = i32, x.2 = i32 → then/else 일치

**결과**: ✅ test (둘 다)

---

### test_tuple_oob_2elem / test_tuple_oob_3elem

```rust
fn f() { let x = (1, true); x.2 }       // 2개에서 .2
fn f() { let x = (1, true, 3); x.3 }    // 3개에서 .3
```

**상정**: 다양한 크기의 concrete 튜플에서 OOB 접근이 거부되는지 확인.

**제약**: `ensure_tuple` 시 concrete이므로 Absent로 패딩 → 접근 결과 Absent → `⟦e.i⟧ ≠ ◇` 위반.

**결과**: ✅ test_none

---

### test_tuple_oob_nested

```rust
fn f() { let x = (1, (2, 3)); x.1.2 }
```

**상정**: 중첩 튜플 내부에서 OOB 접근.

**제약**: x.1 = (i32, i32) — concrete. `x.1.2` → ensure_tuple((i32,i32), 3) → Absent 패딩 → 결과 Absent → **실패**

**결과**: ✅ test_none

---

### test_projection_on_i32 / bool / fn / ref

```rust
fn f() { let x = 1; x.0 }           // i32.0
fn f() { let x = true; x.0 }        // bool.0
fn g(x: ()) { x + 1 } fn f() { g.0 }  // fn.0
fn f() { let x = &1; x.0 }          // ref.0
```

**상정**: 모든 비-튜플 타입(I32, Bool, FnPtr, Ref)에 대해 프로젝션이 거부되는지 체계적 검증.

**제약**: `ensure_tuple` 호출 시 info가 Some(I32/Bool/FnPtr/Ref) → `Some(other)` 분기 → `failed = true`.

**결과**: ✅ test_none (4개 모두)

---

### test_non_concrete_tuple_access

```rust
fn f(x: ()) { x.0 }
```

**상정**: 파라미터(아직 아무 타입 정보 없음)에 필드 접근 → non-concrete 튜플이 자동 생성.

**제약**:
- x는 `None` (미결정)
- `x.0` → `ensure_tuple(x, 1)` → info = None → Tuple([V_fresh], **false**) 생성
- `⟦x.0⟧ = V_fresh` → V_fresh에 추가 제약 없음 → 미결정

**결과**: ✅ test_var — `f: fn((TYPEVAR,)) → TYPEVAR`

---

### test_non_concrete_tuple_multi_access

```rust
fn f(x: ()) { (x.0, x.1) }
```

**상정**: 같은 non-concrete 튜플에 여러 접근 시, 각 원소가 독립적 TYPEVAR이 되는지.

**제약**:
- `x.0` → ensure_tuple(x, 1) → Tuple([V0], false)
- `x.1` → ensure_tuple(x, 2) → 이미 Tuple → len(1) < 2 → fresh로 패딩 → Tuple([V0, V1], false)
- 두 결과 모두 미결정

**결과**: ✅ test_var

---

### test_non_concrete_tuple_access_then_arith

```rust
fn f(x: ()) { x.0 + x.1 }
```

**상정**: non-concrete 튜플 접근 후 산술에 사용 → 원소 타입이 i32로 결정.

**제약**: x.0 + x.1 → x.0 = i32, x.1 = i32 → x = (i32, i32)

**결과**: ✅ test — 모든 타입 결정.

---

### test_tuple_if_branch_mismatch

```rust
fn f() { if true { (1, 2) } else { (1, true) } }
```

**상정**: if-else 양쪽의 튜플 원소 타입이 다르면 실패.

**제약**:
- then = (i32, i32), else = (i32, bool)
- `unify(then_ty, else_ty)` → 원소별 unify: 0번째 i32=i32 OK, 1번째 `unify(i32, bool)` → **충돌**

**결과**: ✅ test_none

---

### test_unit_vs_nonempty_tuple

```rust
fn f(x: ()) { if true { () } else { (1,) }; x + 1 }
```

**상정**: 빈 튜플 `()`과 1-원소 튜플 `(1,)`의 unify 동작 확인.

**분석 과정**:
- `()` → `make_tuple([], false)` — **non-concrete**, 빈 튜플
- `(1,)` → `make_concrete_tuple([i32])` — concrete
- `unify(Tuple([], false), Tuple([i32], true))` → 길이 맞추기: 빈 쪽은 non-concrete이므로 **fresh** 변수로 패딩 → Tuple([V_fresh], true) vs Tuple([i32], true)
- `unify(V_fresh, i32)` → V_fresh = i32 → 성공
- **분석은 통과** (Some). 단 변환 후 코드에서 Rust 타입 체크가 `()` vs `(i32,)` 불일치로 실패하므로, `test`나 `test_none`이 아닌 `result.is_some()` 직접 확인.

**결과**: ✅ `assert!(result.is_some())` — 분석은 해가 존재한다고 판단 (flow-insensitive의 한계).

---

## 8. 함수

### test_fn_zero_args / one_arg / three_args

```rust
fn f() { 42 }  fn g() { f() + 1 }            // 0개
fn inc(x: ()) { x + 1 }  fn g() { inc(5) }    // 1개
fn f(a: (), b: (), c: ()) { a + b + c }        // 3개
```

**상정**: 다양한 인자 수의 함수가 올바르게 처리되는지 확인.

**제약 (zero_args)**:
- `f() { 42 }` → `⟦f⟧ = fn() → i32`
- `g() { f() + 1 }` → `f()` 결과 = i32, `+ 1` → i32

**결과**: ✅ test (세 개 모두)

---

### test_fn_chained_calls

```rust
fn double(x: ()) { x + x }
fn inc(x: ()) { x + 1 }
fn foo() { double(inc(3)) }
```

**상정**: 함수 반환값을 다른 함수에 직접 전달하는 체이닝.

**제약**: inc(3) → i32. double(i32) → i32.

**결과**: ✅ test

---

### test_fn_as_argument

```rust
fn add1(x: ()) { x + 1 }
fn apply(f: (), x: ()) { f(x) }
fn foo() { apply(add1, 2) }
```

**상정**: 함수를 인자로 전달하는 고차 함수 패턴.

**제약**:
- `f(x)` → f = fn(typeof_x) → result. `apply = fn(fn(X)→R, X) → R`
- `apply(add1, 2)` → add1 = fn(i32)→i32, 2 = i32 → X=i32, R=i32

**결과**: ✅ test

---

### test_fn_composition

```rust
fn compose(f: (), g: (), x: ()) { f(g(x)) }
fn double(x: ()) { x + x }
fn inc(x: ()) { x + 1 }
fn foo() { compose(double, inc, 3) }
```

**상정**: 함수 합성 `f(g(x))`.

**제약**: g(x) → g = fn(X)→A. f(g(x)) → f = fn(A)→B. compose = fn(fn(A)→B, fn(X)→A, X) → B. 구체화: X=i32, A=i32, B=i32.

**결과**: ✅ test

---

### test_fn_returns_tuple

```rust
fn make_pair() { (1, true) }
fn foo() { let p = make_pair(); p.0 + 1 }
```

**제약**: make_pair = fn() → (i32, bool). p = (i32, bool). p.0 = i32. `+ 1` → i32.

**결과**: ✅ test

---

### test_fn_in_tuple

```rust
fn add1(x: ()) { x + 1 }
fn sub1(x: ()) { x - 1 }
fn foo() { let t = (add1, sub1); t.0(10) + t.1(10) }
```

**상정**: 함수를 튜플에 넣고 꺼내서 호출하는 패턴.

**제약**:
- t = (fn(i32)→i32, fn(i32)→i32) — concrete 튜플
- t.0 = fn(i32)→i32 → t.0(10) = i32
- t.1 = fn(i32)→i32 → t.1(10) = i32
- i32 + i32 = i32

**결과**: ✅ test

---

### test_arity_mismatch_fewer / test_arity_mismatch_more

```rust
fn f(x: ()) { x + 1 }  fn g() { f() }      // 0개로 호출 (1개 필요)
fn f(x: ()) { x + 1 }  fn g() { f(1, 2) }  // 2개로 호출 (1개 필요)
```

**상정**: 인자 수 불일치 시 실패.

**제약 (fewer)**:
- `f()` → `make_fn_ptr([], ty)` → `unify(fn(V_x)→V_r, fn()→ty)` → FnPtr 길이 1 ≠ 0 → **failed**

**제약 (more)**:
- `f(1, 2)` → `make_fn_ptr([i32, i32], ty)` → FnPtr 길이 1 ≠ 2 → **failed**

**결과**: ✅ test_none (둘 다)

---

### test_call_non_fn_i32 / bool / tuple

```rust
fn f() { let x = 1; x(2) }              // i32 호출
fn f() { let x = true; x(1) }           // bool 호출
fn f() { let x = (1, 2); x(3) }         // 튜플 호출
```

**상정**: 함수가 아닌 타입을 호출하면 실패.

**제약 (i32)**:
- x = i32. `x(2)` → `make_fn_ptr([i32], ty)` → `unify(i32, fn(i32)→ty)` → **I32 ≠ FnPtr → 충돌**

**결과**: ✅ test_none (세 개 모두)

---

### test_mutual_recursion

```rust
fn f(x: ()) { x + g(x) }
fn g(y: ()) { y - f(y) }
```

**상정**: 상호 재귀가 타입 일관적이면 통과.

**제약**:
- f: x+g(x) → x=i32, g(x)=i32 → g = fn(i32)→i32
- g: y-f(y) → y=i32, f(y)=i32 → f = fn(i32)→i32
- Phase 1에서 두 함수의 타입 변수가 미리 생성되어 있으므로 교차 참조 가능.

**결과**: ✅ test

---

### test_self_recursion_valid

```rust
fn factorial(n: ()) {
    if n == 0 { 1 } else { n * factorial(n - 1) }
}
```

**상정**: 유효한 자기 재귀 — 타입 일관.

**제약**:
- `n == 0` → n=i32, 결과=bool → if 조건 OK
- then = 1 → i32. else = `n * factorial(n - 1)` → n=i32, n-1=i32, factorial(i32)=i32, n*i32=i32
- then/else 둘 다 i32 → 일관.

**결과**: ✅ test — `factorial: fn(i32) → i32`

---

### test_self_recursion_conflict

```rust
fn f(x: ()) {
    if x > 0 { f(true) } else { 0 }
}
```

**상정**: 재귀 호출 시 인자 타입 불일치.

**제약**:
- `x > 0` → x=i32
- `f(true)` → `⟦f⟧ = fn(bool)→R` → `unify(V_x, bool)` → V_x(=i32)와 bool → **충돌**

**결과**: ✅ test_none

---

### test_unconstrained_fn / test_unconstrained_fn_two_params

```rust
fn f(x: ()) { x }
fn f(x: (), y: ()) { (x, y) }
```

**상정**: 인자에 아무 연산 없으면 TYPEVAR로 남는지 확인.

**결과**: ✅ test_var

---

## 9. if/else

### test_if_else_both_i32 / both_bool / both_tuple

```rust
fn f(x: ()) { if x > 0 { 1 } else { 2 } }
fn f(x: ()) { if x > 0 { true } else { false } }
fn f(x: ()) { if x > 0 { (1, 2) } else { (3, 4) } }
```

**상정**: then/else 타입 일치 시 다양한 타입으로 성공.

**제약**: `x > 0` → x=i32, 조건=bool. then과 else의 타입이 동일 → unify 성공.

**결과**: ✅ test (세 개 모두)

---

### test_if_else_mismatch_i32_bool / mismatch_i32_tuple

```rust
fn f() { if true { 1 } else { false } }
fn f() { if true { 1 } else { (1, 2) } }
```

**상정**: then/else 타입 불일치 → 실패.

**제약**: `unify(then_ty, else_ty)` → `unify(i32, bool)` or `unify(i32, Tuple)` → **충돌**

**결과**: ✅ test_none

---

### test_if_without_else

```rust
fn f(x: ()) { if x > 0 { x + 1; } }
```

**상정**: else 없는 if → unit 반환.

**제약**: else 없음 → `⟦if⟧ = ()`, `⟦then⟧ = ()`. then 내부: `x + 1;` (세미콜론) → 블록 결과 unit. x > 0 → x=i32.

**결과**: ✅ test — `f: fn(i32) → ()`

---

### test_nested_if

```rust
fn f(a: (), b: ()) { if a > 0 { if b > 0 { a + b } else { a } } else { b } }
```

**상정**: 중첩 if의 모든 분기 타입이 일관적인지 확인.

**제약**: a=b=i32 (비교/산술). 내부 if: then=a+b=i32, else=a=i32 → i32. 외부 if: then=i32, else=b=i32 → i32.

**결과**: ✅ test

---

### test_if_cond_non_bool_i32 / test_if_cond_tuple

```rust
fn f() { if 1 { 2 } else { 3 } }
fn f() { if (1, 2) { 3 } else { 4 } }
```

**상정**: if 조건에 bool이 아닌 타입 → 실패.

**제약**: if 조건 → `unify(cond_ty, bool)`. 1은 i32 → `unify(i32, bool)` → **충돌**. (1,2)는 Tuple → **충돌**.

**결과**: ✅ test_none

---

## 10. let & 대입

### test_let_chain_propagation

```rust
fn f() { let x = 1; let y = x + 2; let z = y * 3; z }
```

**제약**: x=i32 → y=i32 → z=i32. 체인을 통한 타입 전파.

**결과**: ✅ test

---

### test_assign_consistent

```rust
fn f() { let mut x = 1; x = 2; x + 3 }
```

**제약**: x=i32 (from 1). `x = 2` → `unify(x, i32)` → i32=i32 OK. `x + 3` → i32.

**결과**: ✅ test

---

### test_assign_type_conflict

```rust
fn f() { let mut x = 1; x = true; x }
```

**상정**: 변수에 다른 타입 대입 → flow-insensitivity로 인한 충돌.

**제약**: `x = 1` → x=i32. `x = true` → `unify(x, bool)` → `unify(i32, bool)` → **충돌**

**결과**: ✅ test_none

---

### test_assign_op_add

```rust
fn f() { let mut x = 0; x += 1; x }
```

**제약**: x=i32. `x += 1` → AssignOp: lhs=i32, rhs=i32 → 일관.

**결과**: ✅ test

---

### test_assign_op_on_bool

```rust
fn f() { let mut x = true; x += 1; x }
```

**제약**: x=bool. `x += 1` → AssignOp: `⟦x⟧ = i32` → `unify(bool, i32)` → **충돌**

**결과**: ✅ test_none

---

## 11. 블록 & return

### test_block_last_expr

```rust
fn f() { { let x = 1; x + 2 } }
```

**제약**: 블록의 마지막 표현식 `x + 2` = i32 → 블록 타입 = i32 → f 반환 = i32.

**결과**: ✅ test

---

### test_block_semi_unit

```rust
fn f() { { 1 + 2; } }
```

**제약**: `1 + 2;` — 세미콜론이 있으므로 Semi 문. 블록에 tail expr 없음 → unit.

**결과**: ✅ test — `f: fn() → ()`

---

### test_return_matching_body

```rust
fn f(x: ()) { if x > 0 { return x + 1; } x - 1 }
```

**제약**:
- `return x + 1` → `⟦ret_f⟧ = i32`
- 본문 끝: `x - 1` = i32 → `unify(ret_f, i32)` → 일관.

**결과**: ✅ test

---

### test_return_type_mismatch

```rust
fn f() { if true { return 1; } true }
```

**상정**: return 값(i32)과 본문 끝 값(bool)이 충돌.

**제약**: `return 1` → ret_f = i32. 본문 끝 = true → `unify(ret_f, bool)` → `unify(i32, bool)` → **충돌**

**결과**: ✅ test_none

---

### test_return_unit

```rust
fn f(x: ()) { if x > 0 { return; } }
```

**제약**: `return;` → ret_f = (). if without else → 블록 결과 = () → 일관.

**결과**: ✅ test

---

## 12. loop/break/continue

### test_loop_basic

```rust
fn f(x: ()) {
    let mut i = 0;
    loop {
        if i > x { break; }
        i += 1;
    }
}
```

**제약**: loop → unit 반환. i=i32, x=i32 (비교/산술). break → divergent. 함수 반환 = unit.

**결과**: ✅ test

---

### test_loop_continue

```rust
fn f(n: ()) {
    let mut i = 0; let mut sum = 0;
    loop {
        if i > n { break; }
        i += 1;
        if i == 3 { continue; }
        sum += i;
    }
    sum
}
```

**제약**: n=i32, i=i32, sum=i32. continue → divergent. loop → unit. 하지만 마지막에 `sum` → i32 반환.

**결과**: ✅ test — `f: fn(i32) → i32`

---

## 13. 튜플 패턴

### test_tuple_swap

```rust
fn swap(p: ()) { (p.1, p.0) }
fn foo() { let r = swap((1, 2)); r.0 + r.1 }
```

**상정**: non-concrete 튜플 파라미터를 접근 후, concrete 튜플로 호출하는 패턴.

**제약**:
- swap 내부: p는 미결정. p.1 → ensure_tuple(p, 2) → Tuple([V0, V1], false). p.0 → V0
- `(p.1, p.0)` → concrete (V1, V0)
- `swap((1, 2))` → (1,2) = Tuple([i32, i32], true) → `unify(p, (i32, i32))` → V0=i32, V1=i32
- r = (i32, i32). r.0 + r.1 = i32.

**결과**: ✅ test

---

### test_tuple_param_field_access

```rust
fn fst(p: ()) { p.0 }
fn snd(p: ()) { p.1 }
fn foo() { let t = (10, 20); fst(t) + snd(t) }
```

**제약**: fst(t) → p = (i32, i32) → p.0 = i32. snd(t) → p.1 = i32. 덧셈 = i32.

**결과**: ✅ test

---

### test_make_pair

```rust
fn make_pair(a: (), b: ()) { (a, b) }
fn foo() { let p = make_pair(1, 2); p.0 + p.1 }
```

**제약**: make_pair(1, 2) → a=i32, b=i32 → 결과 (i32, i32). p.0 + p.1 = i32.

**결과**: ✅ test

---

## 14. 참조 + 튜플 조합

### test_tuple_of_refs

```rust
fn f() { let a = 1; let b = 2; let t = (&a, &b); *t.0 + *t.1 }
```

**제약**: a=b=i32. t = (&i32, &i32). t.0 = &i32. `*t.0` = i32. 덧셈 = i32.

**결과**: ✅ test

---

### test_ref_to_tuple_deref_access

```rust
fn f() {
    let t = (1, 2);
    let r = &t;
    (*r).0 + (*r).1
}
```

**제약**: t = (i32, i32). r = &(i32, i32). `*r` = (i32, i32). `(*r).0` = i32. 덧셈 = i32.

**결과**: ✅ test

---

## 15. TYPEVAR 잔존

### test_typevar_identity / pair / partial / non_concrete_access

```rust
fn f(x: ()) { x }                         // 완전 미제약
fn f(x: (), y: ()) { (x, y) }             // 미제약 쌍
fn f(x: (), y: ()) { x + 1; y }           // x=i32, y=미결정
fn f(p: ()) { p.0 }                        // non-concrete 접근
```

**상정**: TYPEVAR이 남아야 하는 모든 상황을 검증.

**제약 (partial)**: `x + 1` → x=i32. y에 대한 제약 없음 → y = TYPEVAR.

**결과**: ✅ test_var (네 개 모두)

---

## 16. Occurs Check

### test_occurs_check_self_return

```rust
fn f(x: ()) { let y = x + 1; f }
```

(test_lec3_ex4_recursive_type와 동일) `⟦f⟧ = fn(i32) → ⟦f⟧` → 재귀 타입 → resolve에서 cycle 감지.

**결과**: ✅ test_none

---

### test_occurs_check_self_call

```rust
fn f(x: ()) { x(x) }
```

**상정**: 자기 자신을 자기 자신에 인자로 전달하는 패턴.

**제약**:
- `x(x)` → `⟦x⟧ = fn(⟦x⟧) → ⟦결과⟧`
- x가 fn(x) → R 형태 → x 안에 x 자신이 포함 → 재귀 타입
- resolve에서 cycle 감지 → None

**결과**: ✅ test_none

---

## 17. 복합 시나리오

### test_complex_ref_cmp_if_tuple

```rust
fn abs(x: ()) { if x < 0 { -x } else { x } }
fn dist(p: (), q: ()) { abs(p.0 - q.0) + abs(p.1 - q.1) }
fn foo() { dist((3, 4), (0, 0)) }
```

**상정**: manhattan 거리의 축소 버전. 참조 없이 비교+if+단항+산술+튜플 조합.

**제약**: abs: fn(i32)→i32. p, q = (i32, i32). dist: fn((i32,i32),(i32,i32))→i32.

**결과**: ✅ test

---

### test_higher_order_tuple

```rust
fn map_pair(f: (), p: ()) { (f(p.0), f(p.1)) }
fn inc(x: ()) { x + 1 }
fn foo() { let r = map_pair(inc, (1, 2)); r.0 + r.1 }
```

**상정**: 고차 함수가 튜플 원소에 함수를 적용하는 패턴.

**제약**: f(p.0) → f = fn(A)→B. f(p.1) → 같은 f. map_pair(inc, (1,2)) → A=i32, B=i32.

**결과**: ✅ test

---

### test_four_fn_chain

```rust
fn a(x: ()) { x + 1 }
fn b(x: ()) { x * 2 }
fn c(x: ()) { x - 3 }
fn d(x: ()) { a(b(c(x))) }
fn foo() { d(10) }
```

**상정**: 4단계 함수 체이닝. 각 함수가 i32→i32.

**결과**: ✅ test

---

### test_tuple_with_if_elem

```rust
fn f(x: ()) { (if x > 0 { 1 } else { 2 }, x + 1) }
```

**상정**: 튜플 원소 자리에 if 표현식을 넣는 패턴.

**제약**: `if x > 0 { 1 } else { 2 }` → x=i32, 결과=i32. `x + 1` → i32. 튜플 = (i32, i32).

**결과**: ✅ test

---

### test_consistent_multi_use

```rust
fn f(x: ()) { x + 1; x - 2; x * 3; x / 4; x % 5 }
```

**상정**: 같은 변수를 5개의 서로 다른 산술 연산에 일관적으로 사용 → 모두 i32 요구 → 충돌 없음.

**결과**: ✅ test

---

### test_shared_type_across_fns

```rust
fn f(x: ()) { x + 1 }
fn g(x: ()) { x * 2 }
fn h() { f(1) + g(2) }
```

**상정**: 서로 다른 함수의 같은 이름 파라미터는 독립적인 타입 변수를 갖는지 확인.

**제약**: f의 x ≠ g의 x (별도의 fresh 변수). 각각 i32. f(1)=i32, g(2)=i32, 덧셈=i32.

**결과**: ✅ test
