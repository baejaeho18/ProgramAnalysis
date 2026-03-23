# Type Analysis (2) - 상세 해설
## CSE552 Program Analysis — Lecture 4

---

## 슬라이드 1: 제목 페이지

### 원문 내용
> Type Analysis (2)
> CSE552 Program Analysis — Lecture 4
> Jaemin Hong

### 해설

이 강의는 Type Analysis의 두 번째 부분으로, 제약 조건(constraints)을 해결하는 메커니즘과 알고리즘에 대해 다룹니다.

---

## 슬라이드 2: Solving Constraints — Mechanisms

### 원문 내용
> **Solving Constraints — Mechanisms**
>
> We use two mechanisms to express equivalences:
> - Equivalence between type variables is expressed using union-find
> - Equivalence between a type variable and a proper type is expressed using a mapping M from type variables to types (initially undefined for all)
>
> Assume that MakeSet(X) for each type variable X that occurs in the constraints is called in advance

### 해설

**개념 설명**

제약 조건을 풀기 위해 두 가지 주요 메커니즘을 사용합니다:

1. **Union-Find**: 두 타입 변수가 동등하다는 정보를 관리하기 위해 사용됩니다. 예를 들어 `X = Y`라는 제약이 생기면 union-find를 통해 X와 Y를 같은 집합으로 병합합니다.

2. **Mapping M**: 타입 변수를 구체적인 타입(proper type)에 연결하는 맵입니다. 예를 들어 `X = int`라면 `M[X] = int`로 저장합니다. 초기에는 모든 타입 변수에 대해 정의되지 않은 상태입니다.

**배경 지식** (학부 2학년 수준)

Union-Find는 disjoint set을 관리하는 자료구조로, 두 원소가 같은 집합에 속하는지 빠르게 확인하고, 두 집합을 합칠 수 있습니다. 이를 통해 타입 변수 간의 동등성 관계를 효율적으로 관리합니다.

**전체적인 맥락**

제약 조건 해결(constraint solving)은 타입 추론의 핵심입니다. 프로그램을 분석하면서 수많은 등식 제약(equality constraints)이 생기는데, 이들을 체계적으로 풀어야 최종 타입 할당을 얻을 수 있습니다.

---

## 슬라이드 3: Solving Constraints — Resolve

### 원문 내용
> **Solving Constraints — Resolve**
>
> Resolve(T): resolves a type to its representative
>
> Resolve(T):
> ```
> if T is a type variable X :
>     X' ← Find(X)
>     if M[X'] is defined :
>         return M[X']
>     else
>         return X'
> else
>     return T
> ```

### 해설

**개념 설명**

`Resolve(T)` 함수는 주어진 타입 T를 그것의 대표 타입(representative)으로 변환합니다.

동작 과정:
- T가 타입 변수 X인 경우: Union-Find의 Find(X)를 통해 X가 속한 집합의 대표 원소 X'를 찾습니다.
  - M[X']가 정의되어 있으면(즉, X'가 구체적인 타입에 이미 할당되었으면) 그 타입을 반환합니다.
  - M[X']가 정의되지 않았으면 X' 자체(대표 타입 변수)를 반환합니다.
- T가 타입 변수가 아닌 경우: T를 그대로 반환합니다.

**배경 지식** (학부 2학년 수준)

Find(X) 연산은 Union-Find에서 경로 압축(path compression)을 사용하여 X가 속한 집합의 대표 원소를 O(α(n)) 시간에 찾습니다(α는 Ackermann 함수의 역함수로 매우 작은 값).

**수식/기호/코드 설명**

- X': Find(X)로 얻은 대표 타입 변수
- M[X']: 대표 타입 변수 X'에 할당된 구체적인 타입

**전체적인 맥락**

Resolve는 Unify 알고리즘과 DeepResolve 함수의 기본 구성 요소입니다. 타입 변수의 복잡한 연쇄를 따라가면서 최종 타입을 결정합니다.

---

## 슬라이드 4: Solving Constraints — Unify Algorithm (Part 1)

### 원문 내용
> **Solving Constraints — Unify Algorithm (Part 1)**
>
> For each constraint T₁ = T₂, we invoke Unify(T₁, T₂)
>
> Unify(T₁, T₂):
> ```
> T₁' ← Resolve(T₁)
> T₂' ← Resolve(T₂)
> if T₁' = T₂' :
>     return
> if both T₁' and T₂' are type variables :
>     Union(T₁', T₂')
> else if T₁' is a type variable :
>     if Occurs(T₁', T₂') :
>         unification fails
>     else
>         M[T₁'] ← T₂'
> ```

### 해설

**개념 설명**

Unify 알고리즘은 두 타입이 동등해야 한다는 제약을 처리합니다.

동작 단계:
1. 두 타입을 각각 Resolve하여 최종 형태로 변환합니다.
2. 변환된 두 타입이 같으면 제약이 이미 만족되었으므로 반환합니다.
3. 둘 다 타입 변수인 경우: Union-Find의 Union 연산으로 두 타입 변수를 같은 집합으로 묶습니다.
4. T₁'이 타입 변수이고 T₂'는 아닌 경우:
   - Occurs 체크를 통해 무한 타입을 방지합니다.
   - 문제가 없으면 M[T₁'] ← T₂'로 할당합니다.

**배경 지식** (학부 2학년 수준)

Robinson's unification algorithm이 이것의 기초입니다. 두 항(term)이 동등하게 만드는 치환(substitution)을 찾는 과정입니다.

**수식/기호/코드 설명**

- T₁', T₂': Resolve된 타입들
- Union(T₁', T₂'): 두 집합을 합치는 Union-Find 연산

**전체적인 맥락**

Unify는 타입 제약 해결의 핵심 알고리즘입니다. 프로그램 분석 단계에서 생성된 모든 등식 제약에 대해 호출되며, 이를 통해 점진적으로 타입 변수의 값을 결정합니다.

**추가 설명** (선택)

실제로는 조건이 하나 더 있습니다: T₂'가 타입 변수이고 T₁'이 아닌 경우도 대칭적으로 처리해야 합니다. 이는 Part 2에서 다룹니다.

---

## 슬라이드 5: Solving Constraints — Occurs Check

### 원문 내용
> **Solving Constraints — Occurs Check**
>
> Occurs check: checks if a type variable X occurs in a type T
> - Prevents infinite types (e.g., X = fn(i32) → X)
>
> Occurs(X, T):
> ```
> if T is a proper type C(T₁, ..., Tₙ) :
>     return Occurs(X, T₁) ∨ · · · ∨ Occurs(X, Tₙ)
> if T is a type variable Y :
>     Y' ← Find(Y)
>     if M[Y'] is defined :
>         return Occurs(X, M[Y'])
>     else
>         return Y' = X
> ```

### 해설

**개념 설명**

Occurs 체크는 타입 변수 X가 타입 T 내에 포함되어 있는지 확인하는 함수입니다.

동작 과정:
- T가 구체적인 타입 C(T₁, ..., Tₙ)인 경우: 모든 부분 타입 T₁, ..., Tₙ에 대해 재귀적으로 Occurs를 확인합니다. (OR 논리)
- T가 타입 변수 Y인 경우:
  - Y의 대표 원소 Y'를 Find로 찾습니다.
  - M[Y']가 정의되어 있으면 그 타입에 대해 재귀적으로 확인합니다.
  - 그렇지 않으면 Y'과 X가 같은지 확인합니다.

**배경 지식** (학부 2학년 수준)

Occurs 체크는 unification에서 중요한 부분입니다. 이 체크가 없으면 `X = fn(i32) → X` 같은 무한 타입(infinite type)이 허용되어 타입 시스템이 깨집니다.

**수식/기호/코드 설명**

- ∨: 논리 OR 연산
- M[Y']: Y'에 할당된 타입

**전체적인 맥락**

Occurs 체크는 Unify 함수 내에서 타입 변수를 구체적인 타입에 할당하기 전에 호출됩니다. 이를 통해 타입 시스템의 건전성(soundness)을 보장합니다.

**추가 설명** (선택)

어떤 언어(예: Prolog)에서는 효율성을 위해 occurs 체크를 생략하기도 합니다. 그러나 타입 추론에서는 정확성이 중요하므로 대부분 포함합니다.

---

## 슬라이드 6: Solving Constraints — Unify Algorithm (Part 2)

### 원문 내용
> **Solving Constraints — Unify Algorithm (Part 2)**
>
> Continuing Unify:
> ```
> if ...(previous cases) :
> else if T₂' is a type variable :
>     if Occurs(T₂', T₁') :
>         unification fails
>     else
>         M[T₂'] ← T₁'
> else if T₁' = C(T₁', ..., Tₙ') and T₂' = C(T₁'', ..., Tₙ') :
>     for i in 1..n :
>         Unify(Tᵢ', Tᵢ'')
> else
>     unification fails
> ```

### 해설

**개념 설명**

Unify 알고리즘의 Part 2는 Part 1의 나머지 경우들을 다룹니다.

경우들:
1. T₂'이 타입 변수이고 T₁'이 아닌 경우: Part 1의 대칭 경우입니다. Occurs 체크 후 M[T₂'] ← T₁'로 할당합니다.

2. 둘 다 같은 구조의 구체적인 타입인 경우: 예를 들어 둘 다 `C(...)` 형태라면, 각 인자에 대해 재귀적으로 Unify를 호출합니다. 예: `fn(int) → bool`과 `fn(X) → Y`를 Unify하면 `Unify(int, X)`와 `Unify(bool, Y)`를 호출합니다.

3. 그 외의 경우: 예를 들어 `int`와 `bool`, 또는 다른 구조의 함수 타입끼리는 unify될 수 없으므로 실패합니다.

**배경 지식** (학부 2학년 수준)

함수형 언어나 타입 추론 시스템에서 자주 사용되는 표준적인 unification 알고리즘입니다.

**수식/기호/코드 설명**

- C(T₁', ..., Tₙ'): 생성자 C를 가진 구체적인 타입
- for i in 1..n: 각 인자에 대해 반복적으로 unify

**전체적인 맥락**

Unify는 등식 제약을 해결하는 메인 알고리즘입니다. Part 1과 Part 2를 합쳐 모든 가능한 경우를 커버합니다.

---

## 슬라이드 7: Solving Constraints — Obtaining the Solution

### 원문 내용
> **Solving Constraints — Obtaining the Solution**
>
> Once Unify is called for each constraint, calling DeepResolve on each type variable gives the solution
>
> DeepResolve(T):
> ```
> if T is a proper type C(T₁, ..., Tₙ) :
>     return C(DeepResolve(T₁), ..., DeepResolve(Tₙ))
> if T is a type variable X :
>     X' ← Find(X)
>     if M[X'] is defined :
>         return DeepResolve(M[X'])
>     else
>         return X'
> ```

### 해설

**개념 설명**

DeepResolve는 Resolve와 유사하지만, 구체적인 타입 내의 모든 부분 타입도 재귀적으로 해결합니다.

동작 과정:
- T가 구체적인 타입 C(T₁, ..., Tₙ)인 경우: 각 인자 T₁, ..., Tₙ에 대해 DeepResolve를 재귀적으로 호출합니다.
- T가 타입 변수 X인 경우:
  - Find(X)로 대표 원소 X'를 찾습니다.
  - M[X']가 정의되어 있으면 그 타입에 대해 재귀적으로 DeepResolve를 호출합니다.
  - 그렇지 않으면 X'를 반환합니다.

**배경 지식** (학부 2학년 수준)

Resolve는 한 단계만 해결하지만(shallow), DeepResolve는 모든 중첩 타입 변수까지 완전히 해결합니다(deep).

**수식/기호/코드 설명**

- C(DeepResolve(T₁), ..., DeepResolve(Tₙ)): 구조를 유지하면서 모든 부분 타입을 재귀적으로 해결

**전체적인 맥락**

타입 추론의 마지막 단계입니다. 모든 Unify 호출이 완료된 후, 각 타입 변수에 대해 DeepResolve를 호출하면 최종적인 구체적인 타입(또는 타입 변수)을 얻을 수 있습니다.

**추가 설명** (선택)

예를 들어, M[X] = Y이고 M[Y] = int인 경우:
- Resolve(X)는 Y를 반환합니다.
- DeepResolve(X)는 int를 반환합니다.

---

## 슬라이드 8: Solving Constraints — Implementation

### 원문 내용
> **Solving Constraints — Implementation**
>
> - Each constraint is processed only once
> - For implementation, we can interleave the collection and solving phases, solving the constraints on-the-fly, as they are being generated

### 해설

**개념 설명**

실제 구현에서의 효율성과 유연성에 관한 설명입니다.

두 가지 중요한 사항:
1. **각 제약은 한 번만 처리됩니다**: 같은 제약이 여러 번 생성되지 않도록 주의하거나, 생성되더라도 한 번만 처리하도록 합니다.

2. **온더플라이(on-the-fly) 해결**: 전통적으로는 먼저 모든 제약을 수집한 후 풀지만, 실제 구현에서는 제약이 생성되는 즉시 Unify를 호출할 수 있습니다. 이는:
   - 메모리 사용을 줄일 수 있습니다.
   - 조기에 에러를 감지할 수 있습니다.
   - 성능이 더 좋을 수 있습니다.

**배경 지식** (학부 2학년 수준)

이는 컴파일러 설계에서 자주 사용되는 최적화 기법입니다. 컴파일 시간과 메모리를 절약할 수 있습니다.

**전체적인 맥락**

이제 일반적인 제약 해결 메커니즘을 완료했습니다. 다음부터는 특정 언어 기능(튜플 타입)을 지원하기 위해 제약과 알고리즘을 확장하는 방법을 배웁니다.

---

## 슬라이드 9: Tuple Types

### 원문 내용
> **Tuple Types**
>
> Expression e ::= ... | (e, ..., e) | e.i
> Type T ::= ... | (T, ..., T)
>
> We want to extend the type analysis to support tuple types
> - Projection on non-tuple types should be rejected
> - Projection on non-existent elements of tuples should be rejected

### 해설

**개념 설명**

이제 프로그래밍 언어에 튜플 타입을 추가합니다.

문법:
- 튜플 표현식: `(e₁, e₂, ..., eₙ)` - 여러 값을 하나의 튜플로 묶음
- 튜플 투영(projection): `e.i` - 튜플의 i번째 원소에 접근
- 튜플 타입: `(T₁, T₂, ..., Tₙ)` - 각 원소의 타입을 명시

목표:
1. 비튜플 타입에 대한 투영을 거부해야 합니다. 예: `(int).0` 또는 `int.0`
2. 존재하지 않는 원소에 대한 투영을 거부해야 합니다. 예: `(int, bool).2`

**배경 지식** (학부 2학년 수준)

튜플은 구조체(struct)와 유사한 복합 데이터 타입입니다. 여러 타입의 값을 함께 저장할 수 있습니다.

**전체적인 맥락**

지금까지는 단순한 타입(정수, 함수 타입 등)만 다뤘습니다. 이제 더 복잡한 구조를 지원하기 위해 제약 생성 및 해결 메커니즘을 확장해야 합니다.

---

## 슬라이드 10: Tuple Type Constraints (First Attempt)

### 원문 내용
> **Tuple Type Constraints (First Attempt)**
>
> - (e₁, ..., eₙ): [(e₁, ..., eₙ)] = ([e₁], ..., [eₙ])
> - e.i: [e] = (λ₀, ..., λᵢ₋₁, [e.i], λᵢ₊₁, ..., λₙ)
>   - where λ₀, ..., λᵢ₋₁ are fresh type variables

### 해설

**개념 설명**

튜플 타입을 지원하기 위한 첫 번째 시도로서 타입 제약을 생성하는 규칙입니다.

규칙 분석:

1. **튜플 생성**: `(e₁, ..., eₙ)` 표현식
   - 전체 튜플의 타입 `[e₁, ..., eₙ]`은 각 원소의 타입들의 튜플 `([e₁], ..., [eₙ])`과 같아야 합니다.
   - 예: `(1, true)`의 타입은 `(int, bool)`이어야 합니다.

2. **튜플 투영**: `e.i` 표현식 (i번째 원소 접근)
   - `[e]` (e의 타입)는 정확히 n개의 원소를 가진 튜플이어야 합니다.
   - i번째 원소의 타입은 `[e.i]`와 같아야 합니다.
   - i-1개의 "이전" 위치는 λ₀, ..., λᵢ₋₁로 채워집니다 (이들은 아직 무엇인지 모르는 fresh type variables).
   - i-1개의 "이후" 위치는 λᵢ₊₁, ..., λₙ으로 채워집니다.

**배경 지식** (학부 2학년 수준)

Fresh type variable은 제약 생성 과정에서 새로 도입되는 타입 변수입니다. 각 fresh variable은 고유(unique)합니다.

**수식/기호/코드 설명**

- [e]: 표현식 e의 타입 (이전 강의에서의 표기)
- λ: 임의의 타입을 대표하는 fresh type variable
- 첨수: 튜플 투영의 인덱스 (1부터 시작하거나 0부터 시작할 수 있음)

**전체적인 맥락**

이 규칙은 직관적이지만, 다음 슬라이드의 예제에서 문제가 발생합니다.

---

## 슬라이드 11: Tuple Type Constraints (First Attempt) — Example

### 원문 내용
> **Tuple Type Constraints (First Attempt) — Example**
>
> ```
> fn f() {
>   let x = (1, true);
>   x.0
> }
> ```
>
> Constraints:
> - fn f(){...}: [f] = fn() → [x.0]
> - let x = (1, true): [x] = ([1], [true])
> - 1: [1] = i32
> - true: [true] = bool
> - x.0: [x] = ([x.0], λ)
>
> No solution exists because [x] cannot be both ([x.0]) and (i32, bool).

### 해설

**개념 설명**

이 예제는 첫 번째 시도 규칙의 문제점을 보여줍니다.

코드 분석:
- 함수 f는 튜플 `(1, true)`를 변수 x에 할당합니다.
- 함수는 `x.0` (튜플의 첫 번째 원소)을 반환합니다.

생성된 제약들:
1. `[f] = fn() → [x.0]`: 함수 f의 타입
2. `[x] = ([1], [true])`: x의 타입은 (i32, bool) 형태의 튜플
3. `[1] = i32`: 리터럴 1의 타입
4. `[true] = bool`: 리터럴 true의 타입
5. `[x] = ([x.0], λ)`: x.0 투영에서 생성된 제약 - [x]는 정확히 2개 원소의 튜플이고, 첫 번째 원소가 [x.0]

**문제점**

제약 2와 5가 모순입니다:
- 제약 2: `[x] = (i32, bool)`
- 제약 5: `[x] = ([x.0], λ)`

Unify 결과:
- `([x.0], λ) = (i32, bool)`이어야 하므로
- `[x.0] = i32`이고 `λ = bool`이어야 합니다.

하지만 제약 2는 `[x] = ([1], [true])`이므로, 결국 제약이 만족될 수 없습니다.

실제로는 `[x] = (i32, bool)`이고 `[x.0] = i32`이어야 하므로, 두 번째 규칙이 잘못되었습니다.

**전체적인 맥락**

첫 번째 시도는 튜플의 길이를 정확히 지정하지 못합니다. 투영에서 몇 개의 원소가 있는지 미리 알 수 없으므로, 이를 처리하기 위해 다른 접근이 필요합니다.

---

## 슬라이드 12: Tuple Type Constraints (Second Attempt)

### 원문 내용
> **Tuple Type Constraints (Second Attempt)**
>
> Let N be the largest tuple length or projection index in the program
>
> - (e₁, ..., eₙ): [(e₁, ..., eₙ)] = ([e₁], ..., [eₙ], λₙ₊₁, ..., λₙ)
> - e.i: [e] = (λ₀, ..., λᵢ₋₁, [e.i], λᵢ₊₁, ..., λₙ₋₁)

### 해설

**개념 설명**

두 번째 시도는 고정된 크기의 튜플을 사용합니다.

핵심 아이디어: 프로그램의 모든 투영 인덱스와 튜플 길이 중 최댓값 N을 찾고, 모든 튜플을 정확히 N개의 원소를 가진 것으로 취급합니다.

규칙 수정:

1. **튜플 생성**: `(e₁, ..., eₙ)` (n < N인 경우)
   - 튜플 타입은 N개 원소를 가지며, 처음 n개는 각 표현식의 타입, 나머지는 fresh variables입니다.
   - `[x] = ([e₁], ..., [eₙ], λₙ₊₁, ..., λₙ)`

2. **튜플 투영**: `e.i`
   - `[e]`는 N개 원소의 튜플이어야 합니다.
   - i번째 원소의 타입이 `[e.i]`입니다.

**배경 지식** (학부 2학년 수준)

이는 static 분석에서 자주 사용하는 기법입니다. 동적 크기를 고정된 크기로 "정규화(normalization)"하면 분석이 단순해집니다.

**전체적인 맥락**

고정 크기 접근으로 문제를 해결하지만, 여전히 문제가 있을 수 있습니다. 다음 예제를 보겠습니다.

---

## 슬라이드 13: Tuple Type Constraints (Second Attempt) — Example

### 원문 내용
> **Tuple Type Constraints (Second Attempt) — Example**
>
> ```
> fn f() {
>   let x = (1, true);
>   x.2
> }
> ```
>
> Constraints:
> - fn f(){...}: [f] = fn() → [x.2]
> - let x = (1, true): [x] = ([1], [true], λ₃)
> - 1: [1] = i32
> - true: [true] = bool
> - x.2: [x] = (λ₀, λ₁, [x.2])
>
> Solution: [x] = (i32, bool, λ₃), [f] = fn() → λ₃

### 해설

**개념 설명**

이 예제는 N = 3인 경우입니다(최대 투영 인덱스가 2, 즉 세 번째 원소).

코드 분석:
- 함수 f는 튜플 `(1, true)` (2개 원소)를 생성합니다.
- 함수는 `x.2` (세 번째 원소)를 반환합니다.

생성된 제약들:
1. `[f] = fn() → [x.2]`
2. `[x] = (i32, bool, λ₃)`: 2개 원소로 생성했으므로 3번째는 fresh variable
3. `[1] = i32`
4. `[true] = bool`
5. `[x] = (λ₀, λ₁, [x.2])`: 투영 제약

해결:
- 제약 2와 5를 Unify하면:
  - `i32 = λ₀`
  - `bool = λ₁`
  - `λ₃ = [x.2]`
- 따라서 `[x.2]` 타입은 `λ₃` (여전히 미결정)입니다.
- 최종: `[f] = fn() → λ₃`

**배경 지식** (학부 2학년 수준)

이는 유효한 해이지만, 여전히 세 번째 원소의 타입을 추론하지 못했습니다. 즉, 범위를 벗어난 투영을 감지하지 못합니다.

**전체적인 맥락**

고정 크기 접근도 완벽하지 않습니다. 다음 슬라이드에서 더 나은 해결책을 제시합니다.

---

## 슬라이드 14: Tuple Type Constraints (Correct)

### 원문 내용
> **Tuple Type Constraints (Correct)**
>
> Add a type to represent an absent element of a tuple
>
> T ::= ... | ⋄
>
> - (e₁, ..., eₙ): [(e₁, ..., eₙ)] = ([e₁], ..., [eₙ], ⋄, ..., ⋄)
> - e.i: [e] = (λ₀, ..., λᵢ₋₁, [e.i], λᵢ₊₁, ..., λₙ₋₁) ∧ [e.i] ≠ ⋄

### 해설

**개념 설명**

올바른 해결책은 absent(부재) 타입 ⋄ (바닥/bottom 기호)를 도입합니다.

핵심 아이디어:
- ⋄는 존재하지 않는 원소를 나타내는 특수한 타입입니다.
- 튜플 생성 시, 지정된 원소들은 해당 표현식의 타입으로 채워지고, 나머지는 ⋄로 채웁니다.
- 투영 시에는 접근하는 원소의 타입이 ⋄가 아니어야 한다는 부등식 제약이 추가됩니다.

규칙:

1. **튜플 생성**: `(e₁, ..., eₙ)` (n ≤ N)
   - `[x] = ([e₁], ..., [eₙ], ⋄, ..., ⋄)`: n개 위치는 표현식 타입, 나머지는 ⋄

2. **튜플 투영**: `e.i`
   - 기존: `[e] = (λ₀, ..., λᵢ₋₁, [e.i], λᵢ₊₁, ..., λₙ₋₁)`
   - 추가: `[e.i] ≠ ⋄` (부등식 제약)

**배경 지식** (학부 2학년 수준)

⋄는 타입 이론에서 "bottom" 또는 "impossible" 타입이라고 불립니다. 값이 존재할 수 없는 타입을 나타냅니다.

**수식/기호/코드 설명**

- ⋄: absent 원소를 나타내는 타입
- [e.i] ≠ ⋄: 부등식 제약 (e.i 타입이 ⋄가 아니어야 함)

**전체적인 맥락**

이제 범위 밖의 투영을 올바르게 감지할 수 있습니다. 다음 예제들을 봅시다.

---

## 슬라이드 15: Tuple Type Constraints (Correct) — Example 1

### 원문 내용
> **Tuple Type Constraints (Correct) — Example 1**
>
> ```
> fn f() {
>   let x = (1, true);
>   x.0
> }
> ```
>
> Constraints:
> - fn f(){...}: [f] = fn() → [x.0]
> - let x = (1, true): [x] = ([1], [true])
> - 1: [1] = i32
> - true: [true] = bool
> - x.0: [x] = ([x.0], λ)
>
> Solution: [x] = (i32, bool), [f] = fn() → i32

### 해설

**개념 설명**

이 예제는 유효한 투영입니다 (0번째 원소에 접근).

코드 분석:
- N = 1 (최대 투영 인덱스)
- 튜플 `(1, true)` 생성: 2개 원소이므로 N보다 크지만, 표기는 간단하게 ([1], [true])로 표현

생성된 제약들:
1. `[f] = fn() → [x.0]`
2. `[x] = ([1], [true])`: 기본 제약
3-4. 리터럴 타입
5. `[x] = ([x.0], λ)`: 투영 제약
6. `[x.0] ≠ ⋄`: 부등식 제약

해결:
- `([1], [true])` = `([x.0], λ)`를 Unify하면:
  - `[x.0] = [1] = i32`
  - `λ = [true] = bool`
- `[x.0] ≠ ⋄`는 `i32 ≠ ⋄`로 자동으로 만족됩니다.

최종 해:
- `[x] = (i32, bool)`
- `[f] = fn() → i32`

**배경 지식** (학부 2학년 수준)

이 예제는 정상적인 범위 내의 투영이므로 문제없이 해결됩니다.

**전체적인 맥락**

정상 케이스입니다. 다음 예제는 범위를 벗어난 투영을 보여줍니다.

---

## 슬라이드 16: Tuple Type Constraints (Correct) — Example 2

### 원문 내용
> **Tuple Type Constraints (Correct) — Example 2**
>
> ```
> fn f() {
>   let x = (1, true);
>   x.2
> }
> ```
>
> Constraints:
> - fn f(){...}: [f] = fn() → [x.2]
> - let x = (1, true): [x] = ([1], [true], ⋄)
> - 1: [1] = i32
> - true: [true] = bool
> - x.2: [x] = (λ₁, λ₂, [x.2]) ∧ [x.2] ≠ ⋄
>
> No solution exists because [x.2] should not be ⋄.

### 해설

**개념 설명**

이 예제는 범위를 벗어난 투영입니다 (튜플은 2개 원소인데 3번째 원소에 접근).

코드 분석:
- N = 2 (최대 투영 인덱스는 2)
- 실제로는 인덱스가 0, 1, 2이므로 N = 3일 수도 있지만, 튜플 생성 시 2개만 지정했으므로 세 번째는 ⋄

생성된 제약들:
1. `[f] = fn() → [x.2]`
2. `[x] = (i32, bool, ⋄)`: 튜플 생성에서 2개 지정, 나머지는 ⋄
3-4. 리터럴 타입
5. `[x] = (λ₁, λ₂, [x.2])`: 투영 제약
6. `[x.2] ≠ ⋄`: 부등식 제약

**문제점**

제약 2와 5를 Unify하려면:
- `i32 = λ₁`
- `bool = λ₂`
- `⋄ = [x.2]`

그런데 제약 6은 `[x.2] ≠ ⋄`를 요구합니다.

따라서 제약 6이 위반되므로 **해가 존재하지 않습니다**. 타입 분석기는 "not ok"를 반환합니다.

**배경 지식** (학부 2학년 수준)

부등식 제약(inequality constraint)을 확인하는 것이 핵심입니다. 앞의 슬라이드 17에서 구현을 다룹니다.

**전체적인 맥락**

이제 범위를 벗어난 투영을 올바르게 거부할 수 있습니다!

---

## 슬라이드 17: Tuple Type Constraints — Implementation

### 원문 내용
> **Tuple Type Constraints — Implementation**
>
> - Unify is for solving equalities
> - We first apply Unify to solve all the equalities, and then check the inequalities
> - If any inequality is violated, the analysis says "not ok"
> - Otherwise, the analysis says "ok"

### 해설

**개념 설명**

튜플 타입 제약을 구현하는 방식을 설명합니다.

절차:
1. **1단계**: 모든 등식 제약을 Unify로 해결합니다.
2. **2단계**: 모든 부등식 제약을 확인합니다.
   - 부등식 제약: `[e.i] ≠ ⋄`
   - 1단계에서 얻은 타입 할당을 사용하여, `[e.i]`를 계산(DeepResolve)합니다.
   - 만약 결과가 ⋄라면, 부등식이 위반되었으므로 "not ok"를 반환합니다.
   - 모든 부등식이 만족되면 "ok"를 반환합니다.

**배경 지식** (학부 2학년 수준)

부등식 제약은 등식으로 변환하기 어렵기 때문에 별도로 처리합니다. 이를 두 단계로 나누는 것이 효율적입니다.

**전체적인 맥락**

이제 튜플 타입 지원이 완료되었습니다. 다음부터는 타입 분석의 한계와 문제점들을 다룹니다.

---

## 슬라이드 18: Limitations — Flow-Insensitivity

### 원문 내용
> **Limitations — Flow-Insensitivity**
>
> ```
> fn f() {
>   let x = 1;
>   let y = x + 2;
>   x = true;
>   if x { y } else { 0 }
> }
> ```
>
> - It does not incur a type error at runtime
>   - Common pattern in dynamically typed languages
>   - However, the analysis will say "not ok"
>
> - The analysis ignores the order of execution and computes a single type for each identifier regardless of the program point at which it is used
> - Such an analysis is called a flow-insensitive analysis
> - We will cover flow-sensitive analyses later in the course

### 해설

**개념 설명**

현재의 타입 분석(flow-insensitive)은 프로그램의 실행 흐름(control flow)을 무시합니다.

코드 분석:
1. `let x = 1;`: x를 정수로 초기화
2. `let y = x + 2;`: y는 정수 (1 + 2)
3. `x = true;`: x를 재할당하여 불린 값 할당
4. `if x { y } else { 0 }`: 조건이 참이면 y (정수), 거짓이면 0 (정수)

실제 실행:
- 런타임에는 문제가 없습니다: x는 true이고, 조건이 참이므로 y (정수)를 반환합니다.

타입 분석의 문제:
- 분석기는 x의 타입을 정해야 하는데, x는 한 번은 1 (정수), 다시는 true (불린)입니다.
- Flow-insensitive 분석은 모든 할당을 고려하므로, x의 타입을 `int` 또는 `bool` 모두를 만족해야 합니다.
- 이는 불가능하므로 "not ok"를 반환합니다.

**배경 지식** (학부 2학년 수준)

Flow-insensitive 분석은 단순하고 빠르지만, 거짓 양성(false positive)이 많습니다. Flow-sensitive 분석은 각 프로그램 지점에서 다른 타입을 허용하지만 더 복잡합니다.

**전체적인 맥락**

이것은 현재 분석의 첫 번째 한계입니다. 나중에 flow-sensitive 분석을 배우면 이 문제를 해결할 수 있습니다.

---

## 슬라이드 19: Limitations — Polymorphism (Problem)

### 원문 내용
> **Limitations — Polymorphism (Problem)**
>
> ```
> fn f(x) { x }
> fn g() {
>   f(1);
>   f(true)
> }
> ```
>
> - It does not incur a type error at runtime
>   - f is polymorphic (generic function)
>   - We can type it as fn f<T>(x: T) → T { x } in Rust
>
> - However, the analysis will say "not ok"
>   - [x] needs to be both i32 and bool

### 해설

**개념 설명**

현재의 monomorphic 타입 분석은 다형성(polymorphism)을 지원하지 않습니다.

코드 분석:
- 함수 `f(x)`는 입력을 그대로 반환합니다 (항등 함수).
- `g()`에서 `f(1)` 호출: 정수를 전달
- `g()`에서 `f(true)` 호출: 불린을 전달

실제 실행:
- 함수 f는 일반적(generic)이어서, 어떤 타입이든 받을 수 있습니다.
- Rust에서는 `fn f<T>(x: T) → T`로 정의할 수 있습니다.

타입 분석의 문제:
- 현재 분석은 f의 매개변수 x에 대해 단 하나의 타입만 할당합니다 (monomorphic).
- `f(1)`에서: `[x] = i32`
- `f(true)`에서: `[x] = bool`
- 두 제약이 모순되므로 "not ok"를 반환합니다.

**배경 지식** (학부 2학년 수준)

다형성(polymorphism)은 동일한 코드가 여러 타입에서 작동할 수 있도록 하는 기능입니다. 매개변수 다형성(parametric polymorphism) 또는 제네릭(generics)이라고도 불립니다.

**전체적인 맥락**

이것은 두 번째 주요 한계입니다. 다음 슬라이드에서 해결책을 제시합니다.

---

## 슬라이드 20: Limitations — Polymorphism (Solution)

### 원문 내용
> **Limitations — Polymorphism (Solution)**
>
> - To address this, we need to instantiate a function with different types at different call sites
>   - [f] := ∀X. fn(X) → X
>   - In f(1), [f] is instantiated to fn(i32) → i32
>   - In f(true), [f] is instantiated to fn(bool) → bool
>
> - This is the key idea of Hindley-Miller algorithm and often called let-polymorphism
> - The time complexity is exponential in the worst case¹²
>
> - Later in the course, we will cover context-sensitive analyses, which distinguish different call sites of a function and are similar to let-polymorphism
>
> ¹Deciding ML typability is complete for deterministic exponential time (Mairon, 1990)
> ²ML typability is Dexptime-complete (Kfoury et al., 1990)

### 해설

**개념 설명**

다형성을 지원하기 위한 해결책은 호출 지점(call site)에 따라 함수를 다른 타입으로 인스턴스화하는 것입니다.

핵심 아이디어:
- 함수 f의 타입을 다형 타입(polymorphic type) `∀X. fn(X) → X`로 정의합니다.
  - ∀는 전칭 정량자(universal quantifier)로 "모든 타입 X에 대해"를 의미합니다.
  - X는 타입 변수(type variable)이며, 호출 지점마다 구체적인 타입으로 치환될 수 있습니다.

- 각 호출 지점에서 함수를 인스턴스화합니다:
  - `f(1)` 호출: X를 i32로 치환 → `fn(i32) → i32`
  - `f(true)` 호출: X를 bool로 치환 → `fn(bool) → bool`

**배경 지식** (학부 2학년 수준)

이것은 **Hindley-Milner(HM) 타입 시스템**의 핵심입니다. ML, Haskell 등의 함수형 언어에서 사용됩니다.

용어:
- **Let-polymorphism**: let 바인딩(함수 정의)에서만 다형성을 허용하는 제한된 형태의 다형성입니다.
- **Parametric polymorphism**: 타입 변수를 사용하여 일반적인 함수를 정의합니다.

**수식/기호/코드 설명**

- ∀X: 전칭 정량자 (for all types X)
- fn(X) → X: X 타입을 받아 X 타입을 반환하는 함수
- Instantiation: 각 호출 지점에서 타입 변수를 구체적인 타입으로 대체

**복잡도**

지만, 이것은 계산 비용이 높습니다:
- ML의 타입 추론은 **EXPTIME-complete** (결정론적 지수 시간 완료)입니다.
- 최악의 경우 지수 시간이 소요됩니다.

**전체적인 맥락**

Let-polymorphism은 강력하지만 복잡합니다. 나중에 **context-sensitive analysis**를 배우면 다른 접근법을 볼 수 있습니다.

**추가 설명** (선택)

Hindley-Milner 알고리즘은 다음과 같이 작동합니다:
1. 함수를 일반적인 타입(다형 타입)으로 정의합니다.
2. 각 호출 지점에서 타입 변수를 '신선한' 타입 변수로 인스턴스화합니다.
3. 호출 인자와 함수 매개변수를 Unify하여 인스턴스화된 타입 변수들을 구체화합니다.

---

## 슬라이드 21: Let-Polymorphism

### 원문 내용
> **Let-Polymorphism**
>
> - Even with let polymorphism, false alarms are unavoidable
> - No higher-rank polymorphism
>
> ```
> fn f(x) { x }
> fn g(y) {
>   y(1);
>   y(true)
> }
> fn h() { g(f) }
> ```
>
> - y is a parameter and cannot be polymorphically instantiated at different call sites

### 해설

**개념 설명**

Let-polymorphism도 모든 문제를 해결하지 못합니다. 특히 higher-rank polymorphism이 없기 때문입니다.

코드 분석:
```
fn f(x) { x }           // 항등 함수
fn g(y) {
  y(1);                  // y를 정수 1로 호출
  y(true)                // y를 불린 true로 호출
}
fn h() { g(f) }         // g에 항등 함수 f를 전달
```

실제 실행:
- `h()` 호출 → `g(f)` 호출
- `g` 내에서 y = f (항등 함수)
- `y(1)` → `f(1)` → 정수 반환
- `y(true)` → `f(true)` → 불린 반환
- 모두 성공합니다.

Let-polymorphism의 문제:
- Let-polymorphism은 **함수 정의(let binding)**에서만 다형성을 허용합니다.
- 함수 **매개변수(parameter)**는 다형성으로 처리되지 않습니다.
- y는 g의 매개변수이므로, 서로 다른 호출 지점 `y(1)`과 `y(true)`에서 다양하게 인스턴스화될 수 없습니다.
- 따라서 분석기는 "not ok"를 반환합니다.

**배경 지식** (학부 2학년 수준)

Higher-rank polymorphism은 매개변수 자체가 다형 타입일 수 있게 하는 기능입니다. 예: `fn g(y: ∀T. fn(T) → T)`

이를 지원하려면 타입 추론 알고리즘이 훨씬 복잡해집니다.

**전체적인 맥락**

Let-polymorphism의 한계를 보여주는 예제입니다. 다음 슬라이드는 다른 한계를 다룹니다.

---

## 슬라이드 22: Let-Polymorphism (cont.)

### 원문 내용
> **Let-Polymorphism (cont.)**
>
> - No polymorphic recursion
>
> ```
> fn f(x, n) {
>   if n > 1 {
>     f(true, n - 1);
>   } else if n == 1 {
>     f(0, n - 1);
>   }
>   x
> }
> ```
>
> - Each recursive call requires x to have a different type
> - With unrestricted higher-rank polymorphism or polymorphic recursion, solving the constraints becomes undecidable

### 해설

**개념 설명**

Let-polymorphism은 다형 재귀(polymorphic recursion)도 지원하지 않습니다.

코드 분석:
```
fn f(x, n) {
  if n > 1 {
    f(true, n - 1);      // 첫 번째 재귀 호출: x = true (불린)
  } else if n == 1 {
    f(0, n - 1);         // 두 번째 재귀 호출: x = 0 (정수)
  }
  x                        // 기본 케이스: x를 반환
}
```

실제 실행:
- n = 2일 때: `f(true, 2)` → n > 1이므로 `f(true, 1)` 호출
- n = 1일 때: `f(true, 1)` → n == 1이므로 `f(0, 0)` 호출
- n = 0일 때: `f(0, 0)` → 기본 케이스, 0 반환

각 재귀 호출에서 x의 타입이 다릅니다:
- 첫 호출: x는 bool
- 두 번째 호출: x는 i32

Let-polymorphism의 문제:
- 같은 함수의 여러 호출에서 매개변수 x가 다른 타입이어야 합니다.
- 하지만 x는 함수 정의에서의 매개변수이므로, 단 하나의 타입만 가질 수 있습니다.
- 따라서 제약이 충돌하여 "not ok"를 반환합니다.

**배경 지식** (학부 2학년 수준)

Polymorphic recursion을 지원하려면 각 재귀 호출에 대해 명시적인 타입 정보를 요구해야 합니다(예: Haskell의 RAML 언어).

**복잡도 문제**

무제한 higher-rank polymorphism 또는 polymorphic recursion을 지원하면:
- 제약 해결이 **Undecidable** (결정 불가능)이 됩니다.
- 즉, 어떤 알고리즘도 모든 입력에 대해 정답을 줄 수 없습니다.

**전체적인 맥락**

이것은 다형성 지원의 또 다른 한계입니다. 실용적인 타입 시스템은 이러한 한계를 인식하고 설계됩니다.

---

## 슬라이드 23: Limitations — Other Runtime Errors

### 원문 내용
> **Limitations — Other Runtime Errors**
>
> - Division by zero
> - Stack-use-after-scope
>   - Catching dangling references requires lifetime/region analysis, which is beyond the scope of this type analysis
>
> ```
> fn f() {
>   let x = 0;
>   &x
> }
> fn g() {
>   *f()
> }
> ```

### 해설

**개념 설명**

타입 분석이 감지할 수 없는 다른 런타임 에러들이 있습니다.

**1. Division by Zero (0으로 나누기)**

예:
```c
int x = 0;
int y = 5 / x;  // 런타임 에러
```

타입 검사만으로는 0으로 나누는 것을 감지할 수 없습니다. 이를 위해서는:
- 값 범위 분석(range analysis)
- 상수 전파(constant propagation)
등이 필요합니다.

**2. Stack-use-after-scope (스택 사용 후 범위 벗어남)**

코드 분석:
```
fn f() {
  let x = 0;      // x는 f의 스택에 할당됨
  &x              // x의 참조를 반환
}
fn g() {
  *f()            // f의 참조를 역참조
}
```

문제:
- f()가 반환될 때, f의 로컬 변수 x는 스택에서 해제됩니다.
- g()에서 `*f()`는 해제된 메모리에 접근합니다 (dangling reference).
- 이는 메모리 안전 에러입니다.

타입 분석만으로 감지 불가:
- 이를 감지하려면 **lifetime/region analysis**가 필요합니다.
- 이는 변수의 생명주기(lifetime)를 추적하는 고급 분석입니다.

**배경 지식** (학부 2학년 수준)

이는 **메모리 안전(memory safety)** 문제입니다. Rust는 lifetime 시스템으로 이를 컴파일 타임에 감지합니다.

**전체적인 맥락**

타입 분석은 강력하지만, 모든 런타임 에러를 감지할 수 없습니다. 다른 종류의 분석(정수 범위 분석, lifetime 분석 등)이 필요합니다.

---

## 슬라이드 24: Summary

### 원문 내용
> **Summary**
>
> - Constraints are solved by the Unify algorithm, which uses union-find for type variable equivalences and a mapping for variable-to-type bindings
> - Tuple types require an absent-element type ⋄ and inequality constraints to correctly reject out-of-bounds projections
> - The analysis is flow-insensitive and monomorphic; let-polymorphism addresses the latter but with exponential worst-case complexity

### 해설

**개념 설명**

이 강의의 주요 내용을 요약합니다.

**1. 제약 해결 메커니즘**

- **Unify 알고리즘**: 등식 제약(equality constraints)을 해결합니다.
  - Union-Find: 타입 변수 간의 동등성 관리
  - Mapping M: 타입 변수를 구체적인 타입에 바인딩
  - Occurs 체크: 무한 타입 방지
  - DeepResolve: 최종 타입 계산

**2. 튜플 타입 지원**

- **Absent-element 타입 ⋄**: 존재하지 않는 튜플 원소를 나타냅니다.
- **부등식 제약**: `[e.i] ≠ ⋄`를 통해 범위 밖의 투영을 거부합니다.
- **구현**: 모든 등식을 먼저 해결한 후, 부등식을 확인합니다.

**3. 분석의 한계**

- **Flow-insensitive**: 실행 순서를 무시하므로 재할당된 변수에서 문제가 생깁니다.
  - 해결: Flow-sensitive 분석 (나중에 배움)

- **Monomorphic**: 함수의 단 하나 타입만 허용합니다.
  - 해결: Let-polymorphism (Hindley-Milner 알고리즘)
  - 비용: 최악의 경우 지수 시간 복잡도

**4. 여전히 감지 불가능한 오류**

- 0으로 나누기
- Stack-use-after-scope (dangling references)
- 이들을 위해서는 다른 종류의 분석이 필요합니다.

**전체적인 맥락**

이제 기본적인 타입 추론의 원리를 완전히 이해했습니다. 다음 강의에서는 flow-sensitive 분석이나 더 고급 주제를 다룰 것입니다.

---

## 요약 정보

**강의 주제**: Type Analysis (2) - 제약 해결과 튜플 타입

**핵심 개념들**:
1. Union-Find와 Mapping을 이용한 제약 해결
2. Unify, Resolve, DeepResolve 알고리즘
3. Occurs 체크를 통한 무한 타입 방지
4. 튜플 타입과 범위 체크 (⋄ 타입과 부등식 제약)
5. 현재 분석의 한계 (flow-insensitivity, monomorphism)
6. Let-polymorphism을 통한 다형성 지원

**적용 수준**: 학부 2학년 이상 (타입 시스템 이해 필수)

**다음 강의 선행 주제**: Flow-sensitive 분석, Context-sensitive 분석
