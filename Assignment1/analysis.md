# analysis.rs 전체 설계 및 상세 구현 해설

## 전체 아키텍처

analysis.rs는 **제약 기반 타입 분석(constraint-based type analysis)**을 구현합니다. 슬라이드 03-type-1과 04-type-2에서 다루는 이론을 실제 Rust HIR(High-level Intermediate Representation) 위에 구현한 것입니다.

전체 흐름은 3단계입니다:

1. **Phase 1 — 함수 시그니처 수집** (`FnCollector`): 모든 함수에 대해 타입 변수를 만들어둠
2. **Phase 2 — 제약 생성** (`ConstraintVisitor`): 모든 표현식을 순회하며 제약(equality)을 생성하고, 즉시 Unify로 풀기
3. **Phase 3 — 결과 추출** (`analyze` 함수 후반부): 부등식(inequality) 검사 후, 타입 변수를 실제 타입으로 해석(resolve)

이것은 슬라이드 04-type-2, 슬라이드 17의 구현 전략과 정확히 일치합니다:
> "We first apply Unify to solve all the equalities, and then check the inequalities"

---

## 1. 외부 의존성 및 임포트 (1~10행)

```rust
use std::collections::HashMap;

use rustc_ast::{BinOpKind, LitKind};
use rustc_hir::{
    Block, Expr, ExprKind, HirId, Item, ItemKind, PatKind, QPath, Stmt, StmtKind, UnOp, def::Res,
    def_id::LocalDefId, intravisit,
};
use rustc_middle::{hir::nested_filter, ty::TyCtxt};

use crate::types::Type;
```

Rust 컴파일러 내부 크레이트(`rustc_ast`, `rustc_hir`, `rustc_middle`)를 직접 사용합니다. 이 분석기는 Rust 컴파일러 플러그인으로 동작하기 때문에, 소스 코드를 직접 파싱하는 것이 아니라 컴파일러가 이미 만들어 놓은 **HIR(고수준 중간 표현)**을 순회합니다.

주요 타입들:
- `ExprKind`: 표현식의 종류 (리터럴, 함수 호출, if문, 튜플 등)
- `HirId`: HIR 노드의 고유 식별자 (각 표현식/변수마다 하나씩)
- `LocalDefId`: 함수 정의의 고유 식별자
- `intravisit`: HIR 트리를 순회하는 Visitor 패턴 인프라

---

## 2. 분석 결과 구조체 (12~18행)

```rust
pub struct AnalysisResult {
    pub locals: HashMap<HirId, Type>,
    pub fn_rets: HashMap<LocalDefId, Type>,
}
```

분석의 최종 출력물입니다. 슬라이드에서 "Solution"이라고 표현되는 것에 해당합니다.

- `locals`: 각 지역 변수(`HirId`)에 대한 추론된 타입. 예를 들어 슬라이드 03 Example 1에서 `⟦x⟧ = i32, ⟦y⟧ = i32`
- `fn_rets`: 각 함수의 반환 타입. 예를 들어 `⟦f⟧ = fn(i32, i32) → i32`에서 반환 부분인 `i32`

---

## 3. 메인 분석 함수 — `analyze()` (20~71행)

```rust
pub fn analyze<'tcx>(tcx: TyCtxt<'tcx>) -> Option<AnalysisResult> {
```

`Option<AnalysisResult>`를 반환합니다. `Some(...)` = 해가 존재, `None` = "no solution"(제약 모순).

### Phase 1: 함수 시그니처 수집 (24~32행)

```rust
    let mut collector = FnCollector {
        tcx,
        infer: TypeInfer::new(),
        fn_ret_ty: HashMap::new(),
        fn_ty: HashMap::new(),
        local_ty: HashMap::new(),
    };
    tcx.hir_visit_all_item_likes_in_crate(&mut collector);
```

모든 함수를 먼저 한 번 순회해서, 각 함수에 대한 **타입 변수**를 미리 만들어 둡니다.

이것이 필요한 이유: 슬라이드 03, Example 2를 보면 `fn h() { g(f) }`에서 h의 본문이 f를 참조합니다. 만약 함수들을 순서대로 하나씩 처리하면, h를 분석할 때 f의 타입 변수가 아직 없을 수 있습니다. 따라서 **모든 함수의 타입 변수를 먼저 만들고**, 그 다음에 제약을 생성합니다.

이것은 또한 **상호 재귀(mutual recursion)** 함수도 처리할 수 있게 합니다: `fn f(x) { x + g(x) }`, `fn g(y) { y - f(y) }` — 두 함수가 서로를 참조하므로, 둘 다 타입 변수가 미리 있어야 합니다.

### Phase 2: 제약 생성 및 풀기 (34~48행)

```rust
    let mut visitor = ConstraintVisitor {
        tcx,
        infer: collector.infer,
        expr_ty: HashMap::new(),
        local_ty: collector.local_ty,
        fn_ret_ty: collector.fn_ret_ty,
        fn_ty: collector.fn_ty,
        field_access_vars: Vec::new(),
    };
    tcx.hir_visit_all_item_likes_in_crate(&mut visitor);

    if visitor.infer.failed {
        return None;
    }
```

`collector.infer`를 그대로 이어받아서 두 번째 순회를 합니다. 이번에는 **모든 표현식**을 방문하면서 제약을 생성합니다.

슬라이드에서 "Constraints:" 아래에 나열되는 등식들(예: `⟦x + y⟧ = ⟦x⟧ = ⟦y⟧ = i32`)이 이 단계에서 생성됩니다. 그리고 생성과 동시에 `unify()`를 호출해서 바로 풀어나갑니다.

`failed`가 true이면 제약 모순이 발생한 것이므로 즉시 `None` 반환.

### Phase 3-1: 부등식 검사 (50~55행)

```rust
    for &var in &visitor.field_access_vars {
        if visitor.infer.is_absent(var) {
            return None;
        }
    }
```

이것이 슬라이드 04-type-2, 슬라이드 14 "Correct Version"의 핵심 구현입니다:
> `e.i: ⟦e⟧ = (X₀, ..., Xᵢ₋₁, ⟦e.i⟧, Xᵢ₊₁, ..., X_{N-1}) ∧ ⟦e.i⟧ ≠ ◇`

등식(`=`)은 Phase 2에서 `unify()`로 이미 풀었고, 여기서는 **부등식(`≠ ◇`)**을 검사합니다. 튜플 필드 접근(`e.i`)의 결과 타입이 Absent(◇)이면, 그것은 존재하지 않는 원소에 접근한 것이므로 "no solution"입니다.

**왜 즉시가 아니라 나중에 검사하는가?** 슬라이드 17에서 설명하듯이: "We first apply Unify to solve all the equalities, and then check the inequalities." Unify 과정에서 나중에 Absent가 구체적 타입으로 바뀔 수 있기 때문입니다. 예를 들어 `swap(p)` 함수에서 p.1이 처음에는 Absent여도, 나중에 `swap((1, 2))` 호출로 인해 i32로 통합될 수 있습니다.

### Phase 3-2: 타입 해석 (57~71행)

```rust
    let mut locals = HashMap::new();
    for (hir_id, var) in &visitor.local_ty {
        let ty = visitor.infer.resolve(*var)?;
        locals.insert(*hir_id, ty);
    }

    let mut fn_rets = HashMap::new();
    for (def_id, var) in &visitor.fn_ret_ty {
        let ty = visitor.infer.resolve(*var)?;
        fn_rets.insert(*def_id, ty);
    }

    Some(AnalysisResult { locals, fn_rets })
```

모든 제약이 풀린 후, 각 타입 변수를 실제 `Type` 값으로 변환합니다. `resolve()`가 `None`을 반환하면 (재귀 타입 = 순환 발생), 전체 분석도 `None`을 반환합니다.

이것이 슬라이드 03, Example 4의 `fn f(x) { let y = x + 1; f }` 케이스를 처리합니다. f의 타입이 `μX. fn(i32) → X`와 같은 재귀 타입이 되어야 하지만, "Our language does not have a recursive type, so no solution exists."

---

## 4. 타입 추론 엔진 — `TypeInfo`와 `TypeInfer` (73~342행)

이것이 슬라이드 03-type-1의 **Unify 알고리즘**을 구현하는 핵심 자료구조입니다.

### 4.1 TypeInfo 열거형 (77~87행)

```rust
#[derive(Clone, Debug)]
enum TypeInfo {
    Bool,
    I32,
    Ref(usize),
    Tuple(Vec<usize>, bool),
    FnPtr(Vec<usize>, usize),
    Absent,
}
```

타입 변수에 바인딩될 수 있는 **구체적 타입 정보**입니다. 슬라이드에서 "variable-to-binding mapping"에 해당합니다.

각 variant의 의미:
- `Bool`, `I32`: 기본 타입. `⟦1⟧ = i32`, `⟦true⟧ = bool`
- `Ref(usize)`: 참조 타입 `&T`. 내부의 `usize`는 T에 해당하는 타입 변수 번호
- `Tuple(Vec<usize>, bool)`: 튜플 타입. 첫 번째 필드는 원소 타입 변수들의 리스트, 두 번째는 **concrete 플래그**
- `FnPtr(Vec<usize>, usize)`: 함수 포인터 타입. `fn(params) → ret`
- `Absent`: 슬라이드 04, 슬라이드 14의 **◇ (absent element type)**

**concrete 플래그가 왜 필요한가?**

슬라이드 04의 "Second Attempt"와 "Correct Version"의 차이를 구현하기 위함입니다.

- `(1, true)` → Tuple([i32, bool], **true**): 실제 코드에서 만든 튜플. 크기가 확정됨. 부족한 원소를 채울 때 **Absent**로 채움
- `ensure_tuple(x, 3)` → Tuple([V0, V1, V2], **false**): 프로젝션 `x.2`에서 x가 튜플이어야 한다는 제약에 의해 생성. 크기가 불확정. 부족한 원소를 채울 때 **fresh 변수**로 채움

이 구분이 없으면 슬라이드 04의 Second Attempt 문제가 발생합니다: `(1, true).2`에서 x.2를 접근할 때 padding된 원소가 fresh 변수이므로 어떤 타입이든 될 수 있어서 잘못 통과합니다.

### 4.2 TypeInfer 구조체 (89~104행)

```rust
struct TypeInfer {
    parent: Vec<usize>,
    rank: Vec<usize>,
    info: Vec<Option<TypeInfo>>,
    failed: bool,
}
```

이것이 슬라이드 03-type-1에서 설명하는 **Union-Find 자료구조**입니다.

- `parent`: 각 타입 변수의 부모. `parent[x] == x`이면 x가 자기 집합의 대표(root)
- `rank`: Union by Rank를 위한 트리 높이 추정치. 슬라이드 03에서 "balance the trees"라고 설명
- `info`: 각 대표 원소에 바인딩된 타입 정보. `None`이면 아직 미결정(free variable). 이것이 슬라이드에서 "variable-to-binding mapping"에 해당
- `failed`: 제약 모순 발생 여부. 한 번이라도 충돌하면 true로 설정되고, 이후 모든 unify가 무시됨

슬라이드 03에서 설명하는 구조:
> "Unify algorithm uses union-find for type variable equivalences and a mapping for variable-to-bindings"

이 구조체가 **둘 다** 하나로 합친 것입니다. Union-Find(`parent`, `rank`)로 동치 관계를 관리하고, `info`로 바인딩을 관리합니다.

### 4.3 fresh() — 타입 변수 생성 (106~112행)

```rust
fn fresh(&mut self) -> usize {
    let id = self.parent.len();
    self.parent.push(id);
    self.rank.push(0);
    self.info.push(None);
    id
}
```

슬라이드에서 `X₀, X₁, ...` 같은 **fresh 타입 변수**를 만드는 연산입니다. 슬라이드 03, Example 2에서:
> "⟦f⟧ = fn(⟦x⟧) → ⟦x⟧ ... where each ⟦·⟧ is a fresh type variable"

- `parent[id] = id`: 자기 자신이 부모 = 새 집합의 유일한 원소
- `rank[id] = 0`: 초기 높이 0
- `info[id] = None`: 아직 아무 타입에도 바인딩되지 않음

### 4.4 fresh_with() 및 make_* 메서드 (114~144행)

```rust
fn fresh_with(&mut self, ti: TypeInfo) -> usize {
    let id = self.fresh();
    self.info[id] = Some(ti);
    id
}

fn make_bool(&mut self) -> usize { self.fresh_with(TypeInfo::Bool) }
fn make_i32(&mut self) -> usize { self.fresh_with(TypeInfo::I32) }
fn make_ref(&mut self, inner: usize) -> usize { self.fresh_with(TypeInfo::Ref(inner)) }
fn make_tuple(&mut self, elems: Vec<usize>) -> usize {
    self.fresh_with(TypeInfo::Tuple(elems, false))
}
fn make_concrete_tuple(&mut self, elems: Vec<usize>) -> usize {
    self.fresh_with(TypeInfo::Tuple(elems, true))
}
fn make_absent(&mut self) -> usize { self.fresh_with(TypeInfo::Absent) }
fn is_absent(&self, var: usize) -> bool {
    let rep = self.find(var);
    matches!(self.info[rep], Some(TypeInfo::Absent))
}
fn make_fn_ptr(&mut self, params: Vec<usize>, ret: usize) -> usize {
    self.fresh_with(TypeInfo::FnPtr(params, ret))
}
```

"이미 타입이 알려진" 타입 변수를 만드는 편의 메서드들입니다. 예를 들어 `make_i32()`는 슬라이드에서 `⟦1⟧ = i32` 제약을 만들 때 사용됩니다.

핵심 구분:
- `make_tuple`: **non-concrete** 튜플. `ensure_tuple`에서 "x가 튜플이어야 한다"는 제약으로 생성. 슬라이드 04의 Second Attempt 규칙에 해당
- `make_concrete_tuple`: **concrete** 튜플. `(e₁, ..., eₙ)` 표현식에서 생성. 슬라이드 04의 Correct Version 규칙에 해당

### 4.5 find() — Union-Find의 Find 연산 (146~151행)

```rust
fn find(&self, mut x: usize) -> usize {
    while self.parent[x] != x {
        x = self.parent[x];
    }
    x
}
```

타입 변수 x가 속한 집합의 **대표 원소(root)**를 찾습니다. `parent` 체인을 따라 올라가서 `parent[x] == x`인 곳에 도달하면 그것이 root입니다.

이것이 슬라이드 03에서 설명하는 "equivalence class representative"를 찾는 연산입니다. 예를 들어 `⟦x⟧ = ⟦y⟧`를 unify한 후, find(x)와 find(y)는 같은 값을 반환합니다.

이 버전은 `&self`를 받으므로 경로 압축 없이 단순 탐색만 합니다. `info`를 읽기만 할 때 사용됩니다.

### 4.6 find_compress() — 경로 압축 Find (153~162행)

```rust
fn find_compress(&mut self, x: usize) -> usize {
    let root = self.find(x);
    let mut v = x;
    while self.parent[v] != v {
        let p = self.parent[v];
        self.parent[v] = root;
        v = p;
    }
    root
}
```

**경로 압축(path compression)**이 적용된 Find입니다. 슬라이드 03에서:
> "Path compression: makes every node point directly to the root"

root를 찾은 후, 경로 상의 모든 노드의 parent를 root로 직접 연결합니다. 이렇게 하면 다음번 find가 O(1)에 가까워집니다.

두 번의 패스를 사용합니다:
1. 첫 번째 패스 (`self.find(x)`): root를 찾음
2. 두 번째 패스 (`while` 루프): 경로 상 모든 노드를 root에 직접 연결

이것과 아래의 Union by Rank를 결합하면, n번의 연산에 대해 **O(n · α(n))** 시간 복잡도를 달성합니다 (α는 역 아커만 함수, 실질적으로 상수).

### 4.7 union_sets() — Union by Rank (164~178행)

```rust
fn union_sets(&mut self, x: usize, y: usize) {
    let xr = self.find(x);
    let yr = self.find(y);
    if xr == yr {
        return;
    }
    if self.rank[xr] < self.rank[yr] {
        self.parent[xr] = yr;
    } else if self.rank[xr] > self.rank[yr] {
        self.parent[yr] = xr;
    } else {
        self.parent[yr] = xr;
        self.rank[xr] += 1;
    }
}
```

슬라이드 03의 **Union by Rank**를 구현합니다:
> "Always attach the shorter tree to the root of the taller tree"

규칙:
- `rank[xr] < rank[yr]`: x의 트리가 더 낮으므로 x를 y 아래에 붙임
- `rank[xr] > rank[yr]`: 반대
- `rank[xr] == rank[yr]`: 아무 쪽이나 붙이고, 붙인 쪽의 rank를 1 증가

이것이 트리의 높이를 O(log n)으로 유지해서, 경로 압축과 함께 거의 O(1) 연산을 보장합니다.

### 4.8 unify() — 핵심 통합 연산 (180~205행)

```rust
fn unify(&mut self, a: usize, b: usize) {
    if self.failed {
        return;
    }
    let ra = self.find_compress(a);
    let rb = self.find_compress(b);
    if ra == rb {
        return;
    }

    let ia = self.info[ra].take();
    let ib = self.info[rb].take();

    self.union_sets(ra, rb);
    let rep = self.find(ra);

    match (ia, ib) {
        (None, None) => {}
        (Some(info), None) | (None, Some(info)) => {
            self.info[rep] = Some(info);
        }
        (Some(ia), Some(ib)) => {
            self.merge_info(rep, ia, ib);
        }
    }
}
```

이것이 슬라이드 03에서 설명하는 **Unify 알고리즘**의 핵심입니다. `⟦a⟧ = ⟦b⟧`라는 등식 제약을 처리합니다.

단계별 동작:
1. **failed 검사** (181~183): 이미 모순이 발생했으면 더 이상 아무것도 하지 않음
2. **대표 원소 찾기** (184~185): 경로 압축과 함께 각 변수의 root를 찾음
3. **이미 같은 집합이면 종료** (186~188): `ra == rb`이면 이미 같은 동치류에 속해 있으므로 할 일 없음
4. **타입 정보 추출** (190~191): `.take()`로 기존 info를 꺼냄 (잠시 None으로 만듦)
5. **집합 합치기** (193~194): Union by Rank로 두 집합을 합치고, 새 대표 원소를 찾음
6. **타입 정보 병합** (196~204):
   - 둘 다 `None` (미결정 + 미결정): 아무것도 안 함. 두 변수가 "같다"는 것만 기록
   - 하나만 `Some` (바인딩 + 미결정): 바인딩된 정보를 새 대표에 저장
   - 둘 다 `Some` (바인딩 + 바인딩): `merge_info()`로 두 타입 정보를 병합. 여기서 충돌이 발생할 수 있음

### 4.9 merge_info() — 타입 정보 병합 (207~265행)

```rust
fn merge_info(&mut self, rep: usize, ia: TypeInfo, ib: TypeInfo) {
    match (ia, ib) {
        (TypeInfo::Bool, TypeInfo::Bool) => {
            let r = self.find(rep);
            self.info[r] = Some(TypeInfo::Bool);
        }
        (TypeInfo::I32, TypeInfo::I32) => {
            let r = self.find(rep);
            self.info[r] = Some(TypeInfo::I32);
        }
```

**Bool + Bool = Bool**, **I32 + I32 = I32**: 같은 타입끼리는 성공.

```rust
        (TypeInfo::Ref(a), TypeInfo::Ref(b)) => {
            let r = self.find(rep);
            self.info[r] = Some(TypeInfo::Ref(a));
            self.unify(a, b);
        }
```

**Ref(a) + Ref(b)**: `&T₁`과 `&T₂`를 합치려면 T₁ = T₂여야 함. 재귀적으로 내부 타입을 unify.

```rust
        (TypeInfo::Tuple(mut as_, ac), TypeInfo::Tuple(mut bs, bc)) => {
            let concrete = ac || bc;
            while as_.len() < bs.len() {
                if ac {
                    as_.push(self.make_absent());
                } else {
                    as_.push(self.fresh());
                }
            }
            while bs.len() < as_.len() {
                if bc {
                    bs.push(self.make_absent());
                } else {
                    bs.push(self.fresh());
                }
            }
            let r = self.find(rep);
            self.info[r] = Some(TypeInfo::Tuple(as_.clone(), concrete));
            for i in 0..as_.len() {
                self.unify(as_[i], bs[i]);
            }
        }
```

**Tuple + Tuple 병합**: 가장 복잡한 부분. 슬라이드 04의 "Correct Version" 규칙을 구현합니다.

크기가 다를 때의 패딩 전략이 핵심:
- **concrete 튜플** (`ac == true`): `(1, true)` 같은 실제 튜플. 부족한 원소를 **Absent(◇)**로 채움. 이것이 슬라이드 14의 `(⟦e₁⟧, ..., ⟦eₙ⟧, ◇, ..., ◇)` 규칙
- **non-concrete 튜플** (`ac == false`): `e.i`에서 "e는 최소 i+1개 원소의 튜플이어야 한다"로 생성된 것. 부족한 원소를 **fresh 변수**로 채움. 나중에 더 많은 정보가 올 수 있으므로

`concrete = ac || bc`: 둘 중 하나라도 concrete이면 결과도 concrete. 이것은 `(1, true)`와 `x` (x.0 접근으로 생성된 non-concrete 튜플)를 unify할 때, 결과가 concrete가 되어야 하기 때문입니다.

패딩 후에는 원소별로 재귀적으로 unify합니다.

```rust
        (TypeInfo::FnPtr(ap, ar), TypeInfo::FnPtr(bp, br)) => {
            if ap.len() != bp.len() {
                self.failed = true;
                return;
            }
            let r = self.find(rep);
            self.info[r] = Some(TypeInfo::FnPtr(ap.clone(), ar));
            for i in 0..ap.len() {
                self.unify(ap[i], bp[i]);
            }
            self.unify(ar, br);
        }
```

**FnPtr + FnPtr 병합**: `fn(A₁, ..., Aₙ) → R₁`과 `fn(B₁, ..., Bₘ) → R₂`를 합침.

인자 개수가 다르면 즉시 `failed = true`. 이것이 `test_arity_mismatch`가 실패하는 이유입니다: `fn f(x)` (1개 인자)을 `f(1, 2)` (2개 인자)로 호출하면 FnPtr의 길이가 달라서 충돌.

인자 개수가 같으면 각 인자와 반환 타입을 재귀적으로 unify.

```rust
        (TypeInfo::Absent, other) | (other, TypeInfo::Absent) => {
            let r = self.find(rep);
            self.info[r] = Some(other);
        }
        _ => {
            self.failed = true;
        }
```

**Absent + 다른 타입**: Absent는 "없는 원소"이므로, 구체적 타입이 오면 양보합니다. 이것은 `swap((1, 2))` 같은 케이스를 위한 것입니다. swap 함수 내부에서 p.1이 처음에는 Absent가 될 수 있지만, 나중에 `(1, 2)`와 unify되면서 i32로 바뀌어야 합니다.

패턴 `(TypeInfo::Absent, other)`: Absent와 Absent가 만나면 `other`가 `TypeInfo::Absent`이 되어 그대로 Absent가 유지됩니다.

**catch-all `_ => failed = true`**: 서로 다른 종류의 타입이 만나면 모순. 예: Bool + I32, I32 + Tuple, Ref + FnPtr 등. 이것이 슬라이드 03 Example 3의 "⟦x⟧ cannot be both bool and i32"를 처리합니다.

### 4.10 ensure_tuple() — 튜플 프로젝션 제약 (267~299행)

```rust
fn ensure_tuple(&mut self, var: usize, min_len: usize) -> Option<Vec<usize>> {
    if self.failed {
        return None;
    }
    let rep = self.find_compress(var);
    let info = self.info[rep].take();
    match info {
        None => {
            let elems: Vec<usize> = (0..min_len).map(|_| self.fresh()).collect();
            let r = self.find(var);
            self.info[r] = Some(TypeInfo::Tuple(elems.clone(), false));
            Some(elems)
        }
        Some(TypeInfo::Tuple(mut elems, concrete)) => {
            while elems.len() < min_len {
                if concrete {
                    elems.push(self.make_absent());
                } else {
                    elems.push(self.fresh());
                }
            }
            let r = self.find(var);
            self.info[r] = Some(TypeInfo::Tuple(elems.clone(), concrete));
            Some(elems)
        }
        Some(other) => {
            let r = self.find(var);
            self.info[r] = Some(other);
            self.failed = true;
            None
        }
    }
}
```

`e.i` (튜플 프로젝션) 표현식을 처리할 때 사용됩니다. 슬라이드 04의 제약 규칙:
> `e.i: ⟦e⟧ = (X₀, ..., Xᵢ₋₁, ⟦e.i⟧, Xᵢ₊₁, ..., X_{N-1})`

세 가지 경우:
1. **None (미결정)**: 변수에 아직 아무 정보 없음 → 크기 `min_len`의 **non-concrete 튜플** 생성. fresh 변수로 채움. 나중에 더 많은 정보가 올 수 있으므로.
2. **이미 Tuple**: 현재 크기가 `min_len`보다 작으면 패딩. concrete 여부에 따라 Absent 또는 fresh로.
3. **다른 타입 (I32, Bool, Ref, FnPtr 등)**: 튜플이 아닌 것에 프로젝션 시도 → `failed = true`. 이것이 슬라이드 04, 슬라이드 9의 "Projection on non-tuple types should be rejected"를 구현.

### 4.11 resolve() — 타입 해석 (301~341행)

```rust
fn resolve(&self, var: usize) -> Option<Type> {
    self.resolve_inner(var, &mut Vec::new())
}

fn resolve_inner(&self, var: usize, stack: &mut Vec<usize>) -> Option<Type> {
    let rep = self.find(var);
    if stack.contains(&rep) {
        return None; // cycle
    }
    stack.push(rep);
    let result = match &self.info[rep] {
        None => Some(Type::Var(rep)),
        Some(TypeInfo::Bool) => Some(Type::Bool),
        Some(TypeInfo::I32) => Some(Type::I32),
        Some(TypeInfo::Ref(inner)) => {
            let inner = *inner;
            Some(Type::Ref(Box::new(self.resolve_inner(inner, stack)?)))
        }
        Some(TypeInfo::Absent) => Some(Type::Absent),
        Some(TypeInfo::Tuple(elems, _)) => {
            let elems = elems.clone();
            let mut types = Vec::new();
            for e in &elems {
                types.push(self.resolve_inner(*e, stack)?);
            }
            Some(Type::Tuple(types))
        }
        Some(TypeInfo::FnPtr(params, ret)) => {
            let params = params.clone();
            let ret = *ret;
            let mut param_types = Vec::new();
            for p in &params {
                param_types.push(self.resolve_inner(*p, stack)?);
            }
            let ret_type = self.resolve_inner(ret, stack)?;
            Some(Type::FnPtr(param_types, Box::new(ret_type)))
        }
    };
    stack.pop();
    result
}
```

제약 풀기가 끝난 후, 내부 표현(`usize` 인덱스)을 외부 타입(`Type` enum)으로 변환합니다.

**`stack`의 역할: Occurs Check (순환 검사)**

`stack`은 현재 resolve 중인 타입 변수들의 스택입니다. 만약 resolve 도중 이미 스택에 있는 변수를 다시 만나면, 그것은 **재귀 타입**을 의미합니다.

슬라이드 03, Example 4: `fn f(x) { let y = x + 1; f }`에서 `⟦f⟧ = fn(i32) → ⟦f⟧`로 해석하면 f가 자기 자신을 포함하는 무한 타입이 됩니다. `stack.contains(&rep)`이 이것을 감지하고 `None`을 반환합니다.

각 case의 매핑:
- `None` → `Type::Var(rep)`: 미결정 변수 → TYPEVAR. 슬라이드 03 Example 5의 `fn f(x) { x }` → `fn(X) → X`에서 X에 해당
- `TypeInfo::Bool` → `Type::Bool`
- `TypeInfo::Tuple(elems, _)` → `Type::Tuple(...)`: concrete 플래그는 더 이상 필요 없으므로 무시

---

## 5. Phase 1: 함수 시그니처 수집 — `FnCollector` (344~387행)

```rust
struct FnCollector<'tcx> {
    tcx: TyCtxt<'tcx>,
    infer: TypeInfer,
    fn_ret_ty: HashMap<LocalDefId, usize>,
    fn_ty: HashMap<LocalDefId, usize>,
    local_ty: HashMap<HirId, usize>,
}
```

- `fn_ret_ty`: 각 함수의 반환 타입 변수
- `fn_ty`: 각 함수의 전체 타입 변수 (FnPtr)
- `local_ty`: 각 매개변수의 타입 변수

```rust
impl<'tcx> intravisit::Visitor<'tcx> for FnCollector<'tcx> {
    type NestedFilter = nested_filter::OnlyBodies;

    fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
        self.tcx
    }

    fn visit_item(&mut self, item: &'tcx Item<'tcx>) -> Self::Result {
        let ItemKind::Fn { body, .. } = item.kind else {
            return;
        };

        let def_id = item.owner_id.def_id;
        let body = self.tcx.hir_body(body);

        let ret_var = self.infer.fresh();
        self.fn_ret_ty.insert(def_id, ret_var);

        let mut param_vars = Vec::new();
        for param in body.params {
            let PatKind::Binding(_, hir_id, _, _) = param.pat.kind else {
                panic!("unsupported")
            };
            let var = self.infer.fresh();
            self.local_ty.insert(hir_id, var);
            param_vars.push(var);
        }

        let fn_var = self.infer.make_fn_ptr(param_vars, ret_var);
        self.fn_ty.insert(def_id, fn_var);
    }
}
```

슬라이드 03의 제약 생성 규칙 중 함수 정의에 해당하는 부분:
> `fn f(x) {...}: ⟦f⟧ = fn(⟦x⟧) → ⟦본문⟧`

예를 들어 `fn f(x, y) { x + y }`에 대해:
1. `ret_var = fresh()` → V0 (반환 타입)
2. `var = fresh()` → V1 (x의 타입), V2 (y의 타입)
3. `fn_var = make_fn_ptr([V1, V2], V0)` → V3: FnPtr([V1, V2], V0)

이 시점에서 V0, V1, V2는 모두 `None` (미결정)입니다. 실제 제약은 Phase 2에서 생성됩니다.

`PatKind::Binding(_, hir_id, _, _)`: Rust 패턴 중 단순 변수 바인딩만 지원. `fn f((a, b): ())` 같은 구조 분해는 `panic!("unsupported")`.

---

## 6. Phase 2: 제약 생성 — `ConstraintVisitor` (389~661행)

### 6.1 구조체와 Visitor 설정 (393~409행)

```rust
struct ConstraintVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    infer: TypeInfer,
    expr_ty: HashMap<HirId, usize>,
    local_ty: HashMap<HirId, usize>,
    fn_ret_ty: HashMap<LocalDefId, usize>,
    fn_ty: HashMap<LocalDefId, usize>,
    field_access_vars: Vec<usize>,
}
```

- `expr_ty`: **모든 표현식**에 대한 타입 변수 매핑. 슬라이드에서 `⟦e⟧`를 구현
- `field_access_vars`: `.0`, `.1` 등의 필드 접근 결과 타입 변수. Phase 3에서 Absent 검사용

### 6.2 visit_item() — 함수 본문과 반환 타입 연결 (411~426행)

```rust
fn visit_item(&mut self, item: &'tcx Item<'tcx>) -> Self::Result {
    let ItemKind::Fn { body, .. } = item.kind else {
        intravisit::walk_item(self, item);
        return;
    };

    intravisit::walk_item(self, item);

    let def_id = item.owner_id.def_id;
    let ret_var = self.fn_ret_ty[&def_id];
    let body = self.tcx.hir_body(body);
    let body_ty = self.expr_ty[&body.value.hir_id];
    self.infer.unify(body_ty, ret_var);
}
```

슬라이드 03의 규칙:
> `fn f(x) {body}: ⟦f⟧ = fn(⟦x⟧) → ⟦body⟧`

`intravisit::walk_item()`이 먼저 함수 내부의 모든 표현식을 방문하고, 그 후에 본문의 마지막 표현식 타입(`body_ty`)을 반환 타입 변수(`ret_var`)와 unify합니다.

순서가 중요합니다: 먼저 walk하여 내부 표현식들의 타입 변수를 모두 만들고 제약을 생성한 후, 마지막에 반환 타입 제약을 추가합니다.

### 6.3 visit_stmt() — 지역 변수 바인딩 (428~449행)

```rust
fn visit_stmt(&mut self, stmt: &'tcx Stmt<'tcx>) -> Self::Result {
    match stmt.kind {
        StmtKind::Let(l) => {
            let PatKind::Binding(_, hir_id, _, _) = l.pat.kind else {
                panic!("unsupported")
            };
            let var = self.infer.fresh();
            self.local_ty.insert(hir_id, var);

            intravisit::walk_stmt(self, stmt);

            if let Some(init) = l.init {
                let init_ty = self.expr_ty[&init.hir_id];
                self.infer.unify(var, init_ty);
            }
        }
        StmtKind::Expr(_) | StmtKind::Semi(_) => {
            intravisit::walk_stmt(self, stmt);
        }
        _ => panic!("unsupported"),
    }
}
```

`let x = expr;` 구문을 처리합니다. 슬라이드 03의 규칙:
> `let x = e: ⟦x⟧ = ⟦e⟧`

1. fresh 변수 생성 → `⟦x⟧`
2. walk하여 init 표현식의 타입 변수 생성
3. `unify(var, init_ty)` → `⟦x⟧ = ⟦e⟧`

### 6.4 visit_expr() — 표현식별 제약 생성 (451~656행)

모든 표현식에 대해 호출되는 핵심 메서드입니다.

```rust
fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) -> Self::Result {
    intravisit::walk_expr(self, expr);

    let ty = self.infer.fresh();
    self.expr_ty.insert(expr.hir_id, ty);

    if self.infer.failed {
        return;
    }
```

**모든 표현식**에 대해 먼저 fresh 타입 변수(`ty`)를 만듭니다. 이것이 슬라이드에서 `⟦e⟧`에 해당합니다.

`walk_expr()`을 먼저 호출하는 이유: 하위 표현식들의 타입 변수가 먼저 만들어져 있어야 현재 표현식의 제약을 생성할 수 있습니다.

#### 함수 호출 (463~469행)

```rust
ExprKind::Call(callee, args) => {
    let callee_ty = self.expr_ty[&callee.hir_id];
    let arg_tys: Vec<usize> =
        args.iter().map(|a| self.expr_ty[&a.hir_id]).collect();
    let fn_ty = self.infer.make_fn_ptr(arg_tys, ty);
    self.infer.unify(callee_ty, fn_ty);
}
```

슬라이드 03의 규칙:
> `f(e₁, ..., eₙ): ⟦f⟧ = fn(⟦e₁⟧, ..., ⟦eₙ⟧) → ⟦f(e₁, ..., eₙ)⟧`

`callee_ty`가 `fn(arg_tys) → ty` 형태여야 한다는 제약을 생성합니다. 예를 들어 `g(f)`에서 g의 타입이 `fn(⟦f⟧) → ⟦g(f)⟧`여야 합니다.

#### 튜플 생성 (470~475행)

```rust
ExprKind::Tup(elems) => {
    let elem_tys: Vec<usize> =
        elems.iter().map(|e| self.expr_ty[&e.hir_id]).collect();
    let tup_ty = self.infer.make_concrete_tuple(elem_tys);
    self.infer.unify(ty, tup_ty);
}
```

슬라이드 04의 "Correct Version" 규칙:
> `(e₁, ..., eₙ): ⟦(e₁, ..., eₙ)⟧ = (⟦e₁⟧, ..., ⟦eₙ⟧, ◇, ..., ◇)`

**`make_concrete_tuple`**을 사용합니다 (concrete=true). 이 튜플의 크기는 확정되었으므로, 나중에 패딩할 때 Absent로 채워집니다.

#### 이항 연산 (476~518행)

```rust
ExprKind::Binary(op, lhs, rhs) => {
    let lhs_ty = self.expr_ty[&lhs.hir_id];
    let rhs_ty = self.expr_ty[&rhs.hir_id];
    match op.node {
        BinOpKind::Add | BinOpKind::Sub | BinOpKind::Mul
        | BinOpKind::Div | BinOpKind::Rem | BinOpKind::BitXor
        | BinOpKind::BitAnd | BinOpKind::BitOr
        | BinOpKind::Shl | BinOpKind::Shr => {
            let i = self.infer.make_i32();
            self.infer.unify(lhs_ty, i);
            let i = self.infer.make_i32();
            self.infer.unify(rhs_ty, i);
            let i = self.infer.make_i32();
            self.infer.unify(ty, i);
        }
```

슬라이드 03의 규칙:
> `e₁ + e₂: ⟦e₁ + e₂⟧ = ⟦e₁⟧ = ⟦e₂⟧ = i32`

산술 연산은 세 제약을 동시에 생성: 좌항 = i32, 우항 = i32, 결과 = i32.

```rust
        BinOpKind::And | BinOpKind::Or => {
            let b = self.infer.make_bool();
            self.infer.unify(lhs_ty, b);
            let b = self.infer.make_bool();
            self.infer.unify(rhs_ty, b);
            let b = self.infer.make_bool();
            self.infer.unify(ty, b);
        }
```

논리 연산 `&&`, `||`: 좌항 = bool, 우항 = bool, 결과 = bool.

이것과 산술 연산이 같은 변수에 적용되면 충돌: `x + y; x || y` → x가 i32이면서 bool → no solution.

```rust
        BinOpKind::Eq | BinOpKind::Ne | BinOpKind::Lt
        | BinOpKind::Le | BinOpKind::Gt | BinOpKind::Ge => {
            let i = self.infer.make_i32();
            self.infer.unify(lhs_ty, i);
            let i = self.infer.make_i32();
            self.infer.unify(rhs_ty, i);
            let b = self.infer.make_bool();
            self.infer.unify(ty, b);
        }
```

비교 연산: 피연산자는 i32, **결과는 bool**. 이것이 `if x > 0 { ... }`에서 조건식의 타입이 bool이 되는 이유입니다.

#### 단항 연산 (520~539행)

```rust
ExprKind::Unary(op, operand) => {
    let op_ty = self.expr_ty[&operand.hir_id];
    match op {
        UnOp::Not => {  // !x
            let b = self.infer.make_bool();
            self.infer.unify(op_ty, b);
            let b = self.infer.make_bool();
            self.infer.unify(ty, b);
        }
        UnOp::Neg => {  // -x
            let i = self.infer.make_i32();
            self.infer.unify(op_ty, i);
            let i = self.infer.make_i32();
            self.infer.unify(ty, i);
        }
        UnOp::Deref => {  // *x
            let r = self.infer.make_ref(ty);
            self.infer.unify(op_ty, r);
        }
    }
}
```

역참조(`*x`)가 특히 흥미롭습니다: `*x`의 타입이 `ty`이면, x의 타입은 `&ty` (`Ref(ty)`)여야 합니다. 역으로 참조를 벗기는 제약입니다.

#### 리터럴 (541~551행)

```rust
ExprKind::Lit(lit) => match lit.node {
    LitKind::Int(..) => {
        let i = self.infer.make_i32();
        self.infer.unify(ty, i);
    }
    LitKind::Bool(_) => {
        let b = self.infer.make_bool();
        self.infer.unify(ty, b);
    }
    _ => panic!("unsupported"),
},
```

슬라이드 03의 가장 기본적인 규칙:
> `1: ⟦1⟧ = i32`
> `true: ⟦true⟧ = bool`

#### if 표현식 (556~576행)

```rust
ExprKind::If(cond, then_expr, else_opt) => {
    let cond_ty = self.expr_ty[&cond.hir_id];
    let b = self.infer.make_bool();
    self.infer.unify(cond_ty, b);

    let then_ty = self.expr_ty[&then_expr.hir_id];

    match else_opt {
        Some(else_expr) => {
            let else_ty = self.expr_ty[&else_expr.hir_id];
            self.infer.unify(ty, then_ty);
            self.infer.unify(ty, else_ty);
        }
        None => {
            let unit = self.infer.make_tuple(vec![]);
            self.infer.unify(ty, unit);
            let unit = self.infer.make_tuple(vec![]);
            self.infer.unify(then_ty, unit);
        }
    }
}
```

슬라이드 03의 규칙:
> `if e₁ { e₂ } else { e₃ }: ⟦if ...⟧ = ⟦e₂⟧ = ⟦e₃⟧, ⟦e₁⟧ = bool`

조건식은 반드시 bool이어야 하고, then/else 브랜치의 타입이 같아야 합니다. 이것이 슬라이드 03 Example 3에서 `if x { x + 1 } else { 0 }` → x가 bool이면서 i32여야 해서 충돌하는 이유입니다.

**else가 없는 경우**: `if x > 0 { x + 1; }` → 결과 타입이 `()` (unit = 빈 튜플). then 브랜치도 unit이어야 합니다.

#### 대입 (590~606행)

```rust
ExprKind::Assign(lhs, rhs, _) => {
    let lhs_ty = self.expr_ty[&lhs.hir_id];
    let rhs_ty = self.expr_ty[&rhs.hir_id];
    self.infer.unify(lhs_ty, rhs_ty);
    let unit = self.infer.make_tuple(vec![]);
    self.infer.unify(ty, unit);
}
```

`x = expr`: 좌변과 우변의 타입이 같아야 합니다. 대입 표현식 자체의 타입은 unit.

이것이 슬라이드 04 슬라이드 18의 **flow-insensitivity**를 자연스럽게 구현합니다. `let mut x = 1; x = true;`에서 x는 i32로 바인딩되어 있는데, `x = true`로 인해 `⟦x⟧ = bool` 제약이 추가되어 → i32 ≠ bool → 충돌. 분석은 실행 순서를 무시하고 모든 제약을 동시에 풀기 때문입니다.

#### 튜플 필드 접근 (607~614행)

```rust
ExprKind::Field(base, ident) => {
    let index = ident.name.as_str().parse::<usize>().expect("unsupported");
    let base_ty = self.expr_ty[&base.hir_id];
    if let Some(elems) = self.infer.ensure_tuple(base_ty, index + 1) {
        self.infer.unify(ty, elems[index]);
        self.field_access_vars.push(ty);
    }
}
```

슬라이드 04의 "Correct Version" 규칙:
> `e.i: ⟦e⟧ = (X₀, ..., Xᵢ₋₁, ⟦e.i⟧, Xᵢ₊₁, ..., X_{N-1}) ∧ ⟦e.i⟧ ≠ ◇`

1. `ident`에서 인덱스 파싱: `.0` → 0, `.1` → 1, `.2` → 2
2. `ensure_tuple(base_ty, index + 1)`: base가 최소 `index+1`개 원소의 튜플이어야 한다는 제약
3. `unify(ty, elems[index])`: 결과 타입 = 해당 인덱스의 원소 타입
4. `field_access_vars.push(ty)`: 나중에 `⟦e.i⟧ ≠ ◇` 검사를 위해 기록

`ensure_tuple`이 `None`을 반환하면 (base가 비-튜플 타입이거나 이미 failed), 아무 제약도 추가하지 않고 넘어갑니다. failed 상태이므로 최종적으로 None이 반환됩니다.

#### 변수/함수 참조 (615~626행)

```rust
ExprKind::Path(QPath::Resolved(_, path)) => match path.res {
    Res::Local(hir_id) => {
        let local_var = self.local_ty[&hir_id];
        self.infer.unify(ty, local_var);
    }
    Res::Def(_, def_id) => {
        let def_id = def_id.expect_local();
        let fn_var = self.fn_ty[&def_id];
        self.infer.unify(ty, fn_var);
    }
    _ => panic!("unsupported"),
},
```

변수 이름이 나타나면:
- **지역 변수** (`Res::Local`): `x`가 나타나면 `⟦이 표현식⟧ = ⟦x⟧` (같은 x는 항상 같은 타입 변수를 공유)
- **함수 이름** (`Res::Def`): `f`가 나타나면 `⟦이 표현식⟧ = ⟦f⟧` (Phase 1에서 만든 FnPtr)

이것이 **flow-insensitivity**의 근본 원인입니다: 같은 변수 x가 여러 곳에서 나타나면, 모두 같은 타입 변수로 unify됩니다. 프로그램 위치에 관계없이 하나의 타입만 갖습니다.

#### 참조 생성 (627~631행)

```rust
ExprKind::AddrOf(_, _, inner) => {
    let inner_ty = self.expr_ty[&inner.hir_id];
    let r = self.infer.make_ref(inner_ty);
    self.infer.unify(ty, r);
}
```

`&e`의 타입 = `Ref(⟦e⟧)`. 단순하게 참조 타입을 감싸는 것.

#### 블록 표현식 (581~589행)

```rust
ExprKind::Block(block, _) => {
    if let Some(expr) = block.expr {
        let expr_ty = self.expr_ty[&expr.hir_id];
        self.infer.unify(ty, expr_ty);
    } else {
        let unit = self.infer.make_tuple(vec![]);
        self.infer.unify(ty, unit);
    }
}
```

Rust에서 블록 `{ stmt; ...; expr }`의 타입은 마지막 표현식의 타입. 마지막 표현식이 없으면 (모든 줄이 세미콜론으로 끝나면) unit.

#### return 표현식 (639~653행)

```rust
ExprKind::Ret(ret_val) => {
    let f = expr.hir_id.owner.def_id;
    let ret_ty = self.fn_ret_ty[&f];
    match ret_val {
        Some(e) => {
            let e_ty = self.expr_ty[&e.hir_id];
            self.infer.unify(ret_ty, e_ty);
        }
        None => {
            let unit = self.infer.make_tuple(vec![]);
            self.infer.unify(ret_ty, unit);
        }
    }
}
```

`return e;`가 나타나면 `⟦e⟧`를 해당 함수의 반환 타입 변수와 unify합니다. `return;` (값 없음)이면 반환 타입이 unit이어야 합니다.

---

## 전체 흐름 예시: `fn f() { let x = (1, true); x.2 }`

이 예시를 통해 전체 동작을 추적해 봅니다.

**Phase 1:**
- f에 대해: ret_var = V0, fn_var = V1: FnPtr([], V0)

**Phase 2:**
1. `let x = (1, true);`
   - x → V2 (fresh)
   - `1` → V3, unify(V3, i32) → V3: I32
   - `true` → V4, unify(V4, bool) → V4: Bool
   - `(1, true)` → V5, make_concrete_tuple([V3, V4]) = V6: Tuple([V3, V4], **true**)
   - unify(V5, V6) → V5: Tuple([V3, V4], true)
   - unify(V2, V5) → V2 ≡ V5: Tuple([V3, V4], true) ... **concrete 튜플!**

2. `x.2`
   - V7 (이 표현식의 타입)
   - ensure_tuple(V2, 3): V2는 Tuple([V3, V4], **true**), len=2 < 3
   - **concrete이므로** Absent로 패딩: V8 = make_absent()
   - V2: Tuple([V3, V4, V8], true)
   - unify(V7, V8) → V7 ≡ V8: Absent
   - field_access_vars에 V7 추가

3. 본문의 타입 = V7, unify(V7, V0) → V0 ≡ V7: Absent

**Phase 3:**
- `for &var in field_access_vars`: V7의 대표를 찾으면 Absent → **return None**

결과: **no solution** ✅

---

## 핵심 설계 결정 요약

| 설계 포인트 | 구현 | 근거 (슬라이드) |
|---|---|---|
| Union-Find + 바인딩 통합 | `parent` + `rank` + `info` 하나의 구조체 | 03-type-1: "union-find for equivalences, mapping for bindings" |
| 경로 압축 + Union by Rank | `find_compress` + `union_sets` | 03-type-1: O(n·α(n)) 복잡도 |
| concrete/non-concrete 구분 | Tuple의 bool 플래그 | 04-type-2: Second Attempt vs Correct Version |
| Absent 타입 | TypeInfo::Absent + 지연 검사 | 04-type-2 슬라이드 14: ◇ 타입과 ⟦e.i⟧ ≠ ◇ |
| 등식 먼저, 부등식 나중 | Phase 2 unify → Phase 3 is_absent | 04-type-2 슬라이드 17: "first Unify, then inequalities" |
| 재귀 타입 거부 (Occurs Check) | resolve_inner의 stack | 03-type-1 Example 4: "no recursive type" |
| Flow-insensitive | 같은 변수 = 같은 타입 변수 | 04-type-2 슬라이드 18: "single type regardless of program point" |
| Monomorphic (다형성 없음) | 함수당 하나의 FnPtr | 04-type-2 슬라이드 19: "⟦x⟧ needs to be both i32 and bool" |
