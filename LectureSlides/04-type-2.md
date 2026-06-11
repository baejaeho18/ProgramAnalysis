# Type Analysis (2) - 강의 슬라이드 해설

CSE552 Program Analysis — Lecture 4
강사: Jaemin Hong

> **이 파일을 읽는 법**: 각 슬라이드는 `원문 내용` → `번역` → `해설`(개념·배경·맥락) → 필요 시 `각주`/`슬라이드 연결` 순서입니다. 누락·왜곡 없이 원문을 모두 담되 처음 보는 사람도 따라올 수 있게 풀어 썼습니다.

---

## 강의 4 전체 조감도 (먼저 큰 그림)

강의 3은 "타입 제약을 모은다"까지 했습니다. 강의 4는 그 제약을 **실제로 푸는 단일화(unification) 알고리즘**을 완성하고, 분석을 확장(튜플)하며, 마지막으로 이 단순 타입 분석의 **근본적 한계**를 정직하게 짚습니다.

세 부분:
1. **단일화 알고리즘 구현** (슬라이드 2~8): 타입 변수 동치는 **Union-Find**로, 타입 변수↔고유 타입 바인딩은 **사상 M**으로 관리. `Resolve`(대표 찾기) → `Unify`(등식 처리, occurs check 포함) → `DeepResolve`(최종 타입 추출).
2. **튜플 타입 확장** (슬라이드 9~17): 튜플과 프로젝션(`e.i`)을 다루는데, 세 번의 시도 끝에 **"없는 원소" 타입 ◇와 부등식 제약**으로 올바른 분석을 완성. 분석 확장의 시행착오를 보여 줌.
3. **한계** (슬라이드 18~24): 이 분석은 **흐름 무감각(flow-insensitive)**이고 **단형(monomorphic)**입니다. let-다형성으로 다형성 일부를 해결하지만 지수 시간이 들고, 고차 다형성·다형 재귀는 결정 불가능. 또 0으로 나누기·dangling 참조는 못 잡음.

핵심 통찰: **분석을 만드는 것은 시행착오의 과정**(튜플 3번 시도)이고, **모든 분석엔 한계가 있다**(흐름 무감각·단형). 이 한계 인식이 이후 강의의 동기가 됩니다 — 흐름 감각(강의 7~9), 문맥 민감(강의 10, let-다형성과 유사), 영역/수명 분석(강의 2의 lifetime). 강의 1의 "건전하되 정밀하게, 하지만 완벽할 순 없다"가 구체적으로 드러나는 강의입니다.

---

## 슬라이드 1: 제목 슬라이드

### 원문 내용
> Type Analysis (2)
> CSE552 Program Analysis — Lecture 4
> Jaemin Hong

### 번역
> 타입 분석 (2) / CSE552 프로그램 분석 — 강의 4 / 홍재민

### 해설
타입 분석 2편. 단일화 알고리즘의 완성, 튜플 확장, 그리고 이 분석의 한계를 다룹니다.

---

## 슬라이드 2: Solving Constraints — Mechanisms

### 원문 내용
> - We use two mechanisms to express equivalences:
>   - Equivalence between type variables is expressed using union-find
>   - Equivalence between a type variable and a proper type is expressed using a mapping M from type variables to types (initially undefined for all)
> - Assume that MakeSet(X) for each type variable X that occurs in the constraints is called in advance

### 번역
> 동치를 표현하는 **두 메커니즘**:
> - **타입 변수끼리의 동치** → **Union-Find**(강의 3)
> - **타입 변수 ↔ 고유 타입(proper type)의 동치** → **사상 M**(타입 변수→타입, 처음엔 모두 미정의)
> - 제약에 나오는 각 타입 변수 X에 대해 MakeSet(X)가 미리 호출됐다고 가정

### 해설

**개념 설명 — 두 가지 동치를 두 도구로 ★**

단일화는 두 종류의 등식을 다룹니다:
1. **변수 = 변수** (`⟦x⟧ = ⟦y⟧`): 어느 게 진짜 타입인지 아직 모름 → **Union-Find로 한 그룹으로 묶기**(강의 3의 Union).
2. **변수 = 고유 타입** (`⟦x⟧ = i32`): 변수의 타입이 정해짐 → **사상 M에 기록**(`M[⟦x⟧] = i32`).

즉 Union-Find(동치류)와 사상 M(구체 타입 바인딩)을 **함께** 씁니다. 이 둘을 조합한 연산들이 슬라이드 3~7. 첫 연산 Resolve가 슬3.

---

## 슬라이드 3: Solving Constraints — Resolve

### 원문 내용
> - Resolve(T): resolves a type to its representative
> ```
> Resolve(T):
>   if T is a type variable X:
>     X_r ← Find(X)
>     if M[X_r] is defined: return M[X_r]
>     else: return X_r
>   else: return T
> ```

### 번역
> - **Resolve(T)**: 타입 T를 그 **대표(representative)**로 해석
>   - T가 타입 변수 X면: 루트 X_r = Find(X); M[X_r]이 정의됐으면 그 고유 타입 반환, 아니면 루트 변수 X_r 반환
>   - T가 고유 타입이면 그대로 반환

### 해설

**개념 설명 — Resolve: "이 타입은 현재 무엇으로 알려졌나"**

`Resolve`는 타입 변수가 **지금까지 무엇으로 밝혀졌는지** 한 단계 알아냅니다:
- 변수면 Union-Find로 대표(루트)를 찾고, 그 대표에 고유 타입 바인딩(M)이 있으면 그 타입을, 없으면 아직 미정이니 대표 변수를 반환.
- 고유 타입이면 그대로.

이것이 Unify가 두 타입을 비교하기 전에 "현재 알려진 형태"로 정규화하는 도구입니다(슬4). 한 단계만 푸므로(얕은 해석) 최종 타입 추출엔 DeepResolve(슬7)가 필요. 핵심 알고리즘 Unify가 슬4.

---

## 슬라이드 4: Solving Constraints — Unify Algorithm (Part 1)

### 원문 내용
> - For each constraint T1 = T2, we invoke Unify(T1, T2)
> ```
> Unify(T1, T2):
>   T1_r ← Resolve(T1); T2_r ← Resolve(T2)
>   if T1_r = T2_r: return
>   if both T1_r and T2_r are type variables: Union(T1_r, T2_r)
>   else if T1_r is a type variable:
>     if Occurs(T1_r, T2_r): unification fails
>     else: M[T1_r] ← T2_r
> ```

### 번역
> - 각 제약 `T1 = T2`마다 `Unify(T1, T2)` 호출
> - 절차: 둘을 Resolve로 정규화(T1_r, T2_r). 같으면 끝. **둘 다 변수면 Union**(한 그룹으로). **한쪽만 변수면**: occurs check 통과 시 그 변수를 상대 타입으로 바인딩(M에 기록), 자기 포함이면 실패.

### 해설

**개념 설명 — Unify: 등식 하나를 처리하는 핵심 ★**

`Unify(T1, T2)`는 "T1과 T2가 같다"를 강제합니다:
1. 둘을 Resolve로 현재 형태로 정규화.
2. 이미 같으면 할 일 없음.
3. **둘 다 변수**면 → `Union`으로 동치류 합치기(슬2 메커니즘 1).
4. **한쪽만 변수**면 → 그 변수를 상대 타입으로 **바인딩**(M에 기록, 메커니즘 2). 단 자기 자신을 포함하면(`X = fn(...)→X`) **occurs check**로 막아 무한 타입 방지(슬5).

나머지 경우(둘 다 고유 타입)가 슬6. 이것이 강의 3의 Union-Find에 "변수↔타입 바인딩"과 "구조 분해"를 더한 완전한 단일화입니다.

---

## 슬라이드 5: Solving Constraints — Occurs Check

### 원문 내용
> - Occurs check: checks if a type variable X occurs in a type T
>   - Prevents infinite types (e.g., X = fn(i32) → X)
> ```
> Occurs(X, T):
>   if T is a proper type C(T1, ..., Tn): return Occurs(X, T1) ∨ ... ∨ Occurs(X, Tn)
>   if T is a type variable Y:
>     Y_r ← Find(Y)
>     if M[Y_r] is defined: return Occurs(X, M[Y_r])
>     else: return Y_r = X
> ```

### 번역
> - **occurs check**: 타입 변수 X가 타입 T 안에 **나타나는지** 검사
>   - **무한 타입 방지**(예: `X = fn(i32) → X` — X가 자기 안에 등장)
> - T가 고유 타입이면 각 인자에 재귀적으로 검사; 변수 Y면 그 대표가 X인지(또는 바인딩된 타입 안에 X가 있는지) 확인

### 해설

**개념 설명 — occurs check: 무한 타입 차단**

강의 3 슬18의 `fn(i32)→fn(i32)→...`(무한 중첩) 문제를 막는 검사입니다. `X = T`로 바인딩하기 전에, **T 안에 X가 들어 있으면** 그것은 자기 자신을 포함하는 무한 타입(`X = ...X...`)이므로 **단일화 실패**(타입 오류). 이 검사가 재귀 타입을 금지합니다(강의 3 슬18의 "재귀 타입 없으면 해 없음"의 구현). Unify의 나머지 경우가 슬6.

---

## 슬라이드 6: Solving Constraints — Unify Algorithm (Part 2)

### 원문 내용
> Continuing Unify:
> ```
>   else if T2_r is a type variable:
>     if Occurs(T2_r, T1_r): unification fails
>     else: M[T2_r] ← T1_r
>   else if T1_r = C(T1', ..., Tn') and T2_r = C(T1'', ..., Tn''):
>     for i in 1..n: Unify(Ti', Ti'')
>   else: unification fails
> ```

### 번역
> Unify 계속:
> - T2_r만 변수면 → (occurs check 후) T2_r를 T1_r로 바인딩 (대칭)
> - **둘 다 같은 생성자 C(...)면** → 각 인자끼리 재귀적으로 Unify (구조 분해)
> - 그 외(다른 생성자) → **단일화 실패**

### 해설

**개념 설명 — 구조 분해와 실패**

Unify의 마지막 경우들:
- **둘 다 고유 타입이고 같은 생성자**(예: `fn(A,B)→C`와 `fn(D,E)→F`) → **인자끼리 재귀 Unify**(A=D, B=E, C=F). 함수 타입·참조 타입을 구조적으로 맞춰 나감.
- **다른 생성자**(예: `i32`와 `bool`, 또는 `fn`과 `&`) → **즉시 실패**(타입 오류). 강의 3 슬17의 "x가 i32이자 bool"이 여기서 검출됩니다.

이 재귀적 구조 분해가 단일화의 본질 — 복잡한 타입을 부분으로 쪼개 맞춥니다. 최종 타입 추출이 슬7.

---

## 슬라이드 7: Solving Constraints — Obtaining the Solution

### 원문 내용
> - Once Unify is called for each constraint, calling DeepResolve on each type variable gives the solution
> ```
> DeepResolve(T):
>   if T is a proper type C(T1, ..., Tn): return C(DeepResolve(T1), ..., DeepResolve(Tn))
>   if T is a type variable X:
>     X_r ← Find(X)
>     if M[X_r] is defined: return DeepResolve(M[X_r])
>     else: return X_r
> ```

### 번역
> - 모든 제약에 Unify를 호출한 뒤, 각 타입 변수에 **DeepResolve**를 호출하면 최종 해(타입)가 나옴
> - DeepResolve는 Resolve와 달리 **재귀적으로 끝까지** 풀어, 타입 안의 모든 변수까지 구체화

### 해설

**개념 설명 — DeepResolve: 최종 타입 추출**

`Resolve`(슬3)는 한 단계만 풀지만, `DeepResolve`는 **타입 전체를 재귀적으로** 풉니다. 예: `⟦f⟧`가 `fn(X)→X`로 바인딩됐고 X가 i32로 밝혀졌으면, DeepResolve가 `fn(i32)→i32`를 반환. 모든 제약을 Unify로 처리한 뒤 각 변수에 DeepResolve하면 최종 타입 할당(= 강의 3 슬13의 해)이 나옵니다. 변수가 끝까지 남으면 다형(주 타입, 강의 3 슬19). 구현 팁이 슬8.

---

## 슬라이드 8: Solving Constraints — Implementation

### 원문 내용
> - Each constraint is processed only once
> - For implementation, we can interleave the collection and solving phases, solving the constraints on-the-fly, as they are being generated

### 번역
> - 각 제약은 **한 번만** 처리됨
> - 구현 시 **수집과 풀이를 번갈아** 할 수 있음 — 제약을 생성하면서 즉석에서(on-the-fly) 풀기

### 해설

**개념 설명**

효율 팁: 제약을 다 모은 뒤 푸는 대신, **코드를 훑으며 제약을 만들 때마다 즉시 Unify**해도 됩니다(단일화는 순서 무관). 메모리를 아끼고 모순을 일찍 발견할 수 있습니다. (강의 11 cubic의 "전파와 처리 인터리빙"과 같은 발상.) 여기까지가 단일화 완성. 슬9부터 튜플 확장.

---

## 슬라이드 9: Tuple Types

### 원문 내용
> - Expression e ::= ... | (e, ..., e) | e.i
> - Type T ::= ... | (T, ..., T)
> - We want to extend the type analysis to support tuple types
>   - Projection on non-tuple types should be rejected
>   - Projection on non-existent elements of tuples should be rejected

### 번역
> - 식에 **튜플 `(e,...,e)`**과 **프로젝션 `e.i`**(i번째 원소 접근) 추가, 타입에 **튜플 타입 `(T,...,T)`** 추가
> - 분석 확장 목표: (1) 튜플이 아닌 것에 `.i` 접근은 거부, (2) 튜플의 **없는 원소** 접근(`(1,2).5`)은 거부

### 해설

**개념 설명 — 튜플과 프로젝션**

튜플 `(1, true)`은 곱 타입(강의 2). `e.i`는 i번째 원소를 꺼냅니다(`(1,true).0 = 1`). 분석이 잡아야 할 오류 두 가지: ① 튜플 아닌 것에 `.i`(예: `5.0`), ② 범위 밖 인덱스(`(1,2).5`). 이를 제약으로 표현하는 데 **세 번의 시도**가 필요합니다(슬10~16) — 분석 설계의 시행착오를 보여 주는 좋은 사례. 첫 시도가 슬10.

---

## 슬라이드 10: Tuple Type Constraints (First Attempt)

### 원문 내용
> - (e1, ..., en): ⟦(e1, ..., en)⟧ = (⟦e1⟧, ..., ⟦en⟧)
> - e.i: ⟦e⟧ = (X0, ..., X_{i-1}, ⟦e.i⟧)
>   - where X0, ..., X_{i-1} are fresh type variables

### 번역
> **1차 시도**:
> - 튜플 `(e1,...,en)`: 그 타입은 각 원소 타입의 튜플 `(⟦e1⟧,...,⟦en⟧)`
> - 프로젝션 `e.i`: e는 **길이 i+1인 튜플**이고 그 마지막(i번째) 원소가 `⟦e.i⟧`라고 제약 — `⟦e⟧ = (X0,...,X_{i-1}, ⟦e.i⟧)`

### 해설

**개념 설명 — 1차 시도의 발상**

`e.i`를 다루려면 "e는 적어도 i+1개 원소를 가진 튜플"이라 해야 합니다. 1차 시도: `⟦e⟧ = (X0, ..., X_{i-1}, ⟦e.i⟧)` — "e는 i+1개짜리 튜플, 그 i번째가 결과"라고 못 박음. 그럴듯하지만 **문제**가 있습니다 — 같은 튜플을 두 번 다른 인덱스로 접근하면 길이가 충돌합니다(슬11). 분석 설계의 첫 함정.

---

## 슬라이드 11: Tuple Type Constraints (First Attempt) — Example

### 원문 내용
> ```rust
> fn f() { let x = (1, true); x.0 }
> ```
> Constraints:
> - ⟦f⟧ = fn() → ⟦x.0⟧
> - let x = (1,true): ⟦x⟧ = (⟦1⟧, ⟦true⟧)
> - 1: ⟦1⟧ = i32; true: ⟦true⟧ = bool
> - x.0: ⟦x⟧ = (⟦x.0⟧)
> - No solution exists because ⟦x⟧ cannot be both (⟦x.0⟧) and (i32, bool).

### 번역
> `x = (1,true); x.0`: `let`에서 `⟦x⟧=(i32,bool)`(길이 2). 그런데 `x.0`(1차 규칙)은 `⟦x⟧=(⟦x.0⟧)`(길이 1)을 요구. **길이 2 vs 길이 1 충돌 → 해 없음**(틀린 거부!).

### 해설

**개념 설명 — 1차 시도의 실패 ★**

`x`는 실제로 `(1, true)`(길이 2)인데, `x.0`을 1차 규칙으로 처리하면 "x는 길이 1 튜플"을 강제합니다 → 길이 2와 모순 → 분석이 "타입 오류"라 **잘못 거부**합니다. 문제의 핵심: **`e.i`가 "정확히 i+1개"를 요구**하는데, 실제론 "**적어도** i+1개"여야 합니다. 길이를 고정한 게 잘못. 2차 시도가 이를 고칩니다(슬12).

---

## 슬라이드 12: Tuple Type Constraints (Second Attempt)

### 원문 내용
> - Let N be the largest tuple length or projection index in the program.
> - (e1, ..., en): ⟦(e1, ..., en)⟧ = (⟦e1⟧, ..., ⟦en⟧, X_{n+1}, ..., X_N)
> - e.i: ⟦e⟧ = (X0, ..., X_{i-1}, ⟦e.i⟧, X_{i+1}, ..., X_{N-1})

### 번역
> **2차 시도**: 프로그램의 **최대 튜플 길이/인덱스 N**으로 모든 튜플을 **길이 N으로 패딩**.
> - 튜플 `(e1,...,en)`: `(⟦e1⟧,...,⟦en⟧, X_{n+1},...,X_N)` (나머지를 fresh 변수로 채움)
> - 프로젝션 `e.i`: `(X0,...,X_{i-1}, ⟦e.i⟧, X_{i+1},...,X_{N-1})` (i번째만 결과, 나머지 fresh)

### 해설

**개념 설명 — 2차 시도: 모든 튜플을 길이 N으로 통일**

1차의 길이 충돌을 피하려고, **모든 튜플을 프로그램 내 최대 길이 N으로 패딩**합니다(빈 자리는 fresh 변수). 그러면 길이가 항상 N으로 통일되어 충돌이 없어집니다. `x.0`도 `(⟦x.0⟧, X1,...,X_{N-1})`로 길이 N. 이러면 슬11의 잘못된 거부가 사라집니다(슬13). 하지만 **새 문제** — 범위 밖 접근을 못 잡습니다(슬16). 부분적 해결.

---

## 슬라이드 13: Tuple Type Constraints (Second Attempt) — Example

### 원문 내용
> ```rust
> fn f() { let x = (1, true); x.2 }
> ```
> Constraints:
> - ⟦f⟧ = fn() → ⟦x.2⟧
> - let x = (1,true): ⟦x⟧ = (⟦1⟧, ⟦true⟧, X3)   // N=3, 패딩
> - x.2: ⟦x⟧ = (X1, X2, ⟦x.2⟧)
> - Solution: ⟦x⟧ = (i32, bool, X3), ⟦f⟧ = fn() → X3

### 번역
> `x=(1,true); x.2`(범위 밖! x는 길이 2인데 .2 접근). N=3으로 패딩하면 `⟦x⟧=(i32,bool,X3)`, `x.2`가 X3를 가리킴 → **해가 존재해 ok라고 함**. 그런데 실제론 x.2는 범위 밖이라 **거부해야 함**(2차 시도의 결함).

### 해설

**개념 설명 — 2차 시도의 결함**

2차 시도는 패딩 때문에 `x.2`(범위 밖 접근)도 X3(패딩 변수)를 가리켜 **통과시켜 버립니다**. 실제로는 길이 2 튜플에 `.2`는 오류인데 잡지 못함 → **unsound**(놓침). 1차는 너무 엄격(잘못 거부), 2차는 너무 느슨(못 잡음). 둘 다 틀렸습니다. 올바른 해법은 "패딩 자리"와 "진짜 원소"를 구분하는 것(슬14).

---

## 슬라이드 14: Tuple Type Constraints (Correct)

### 원문 내용
> - Add a type to represent an absent element of a tuple: T ::= ... | ◇
> - (e1, ..., en): ⟦(e1, ..., en)⟧ = (⟦e1⟧, ..., ⟦en⟧, ◇, ..., ◇)
> - e.i: ⟦e⟧ = (X0, ..., X_{i-1}, ⟦e.i⟧, X_{i+1}, ..., X_{N-1}) ∧ ⟦e.i⟧ ≠ ◇

### 번역
> **올바른 시도**: 튜플의 **없는 원소**를 나타내는 타입 **◇**를 추가.
> - 튜플 `(e1,...,en)`: 진짜 원소 뒤를 **◇로 패딩** (`(⟦e1⟧,...,⟦en⟧, ◇,...,◇)`)
> - 프로젝션 `e.i`: 패딩은 fresh 변수로 하되, **`⟦e.i⟧ ≠ ◇`라는 부등식 제약** 추가 (접근하는 원소는 ◇(없는 원소)가 아니어야 함)

### 해설

**개념 설명 — 올바른 해법: ◇ + 부등식 ★**

핵심 아이디어 두 가지:
1. **◇ ("없는 원소" 타입)**: 튜플을 N으로 패딩하되, 진짜 원소 뒤는 **◇**로 채웁니다. 그러면 "이 자리는 원래 없는 원소"임이 타입에 드러남.
2. **부등식 `⟦e.i⟧ ≠ ◇`**: `e.i`로 접근하는 i번째 원소가 ◇(없는 원소)이면 안 된다는 제약. 범위 밖 접근(◇를 가리킴)이 이 부등식을 위반해 거부됩니다.

이제 `(1,true).0`은 `⟦x.0⟧=i32≠◇`(OK), `(1,true).2`는 `⟦x.2⟧=◇`인데 `≠◇` 위반(거부) → **둘 다 올바르게 판정**. 등식만으론 부족해 **부등식**을 도입한 점이 핵심(단일화는 등식만 풀므로 부등식은 따로 검사 — 슬17). 예가 슬15~16.

---

## 슬라이드 15: Tuple Type Constraints (Correct) — Example 1

### 원문 내용
> ```rust
> fn f() { let x = (1, true); x.0 }
> ```
> - let x = (1,true): ⟦x⟧ = (⟦1⟧, ⟦true⟧)   // ◇ 패딩
> - x.0: ⟦x⟧ = (⟦x.0⟧, T2)
> - Solution: ⟦x⟧ = (i32, bool), ⟦f⟧ = fn() → i32

### 번역
> `x.0`(정상 접근): `⟦x.0⟧=i32`이고 `i32≠◇`(OK) → 해 존재, `f: fn()→i32`. 올바르게 ok.

### 해설

**개념 설명**

정상적인 `x.0` 접근은 i32를 가리키고 `i32≠◇` 부등식을 만족 → ok. 1차 시도가 잘못 거부했던(슬11) 것이 올바르게 통과됩니다. 범위 밖 접근이 슬16.

---

## 슬라이드 16: Tuple Type Constraints (Correct) — Example 2

### 원문 내용
> ```rust
> fn f() { let x = (1, true); x.2 }
> ```
> - let x = (1,true): ⟦x⟧ = (⟦1⟧, ⟦true⟧, ◇)
> - x.2: ⟦x⟧ = (X1, X2, ⟦x.2⟧) ∧ ⟦x.2⟧ ≠ ◇
> - No solution exists because ⟦x.2⟧ should not be ◇.

### 번역
> `x.2`(범위 밖): 패딩으로 `⟦x⟧=(i32,bool,◇)`이라 3번째가 ◇ → `⟦x.2⟧=◇`. 그런데 부등식 `⟦x.2⟧≠◇` 위반 → **해 없음 → 올바르게 거부**.

### 해설

**개념 설명**

범위 밖 `x.2`는 패딩된 ◇를 가리키게 되고, `⟦x.2⟧≠◇` 부등식을 위반 → 거부. 2차 시도가 놓쳤던(슬13) 오류가 올바르게 잡힙니다. **◇ + 부등식**이 1차(과엄격)·2차(과관대)의 문제를 모두 해결. 구현 방법이 슬17.

---

## 슬라이드 17: Tuple Type Constraints — Implementation

### 원문 내용
> - Unify is for solving equalities
> - We first apply Unify to solve all the equalities, and then check the inequalities
>   - If any inequality is violated, the analysis says "not ok"
>   - Otherwise, the analysis says "ok"

### 번역
> - Unify는 **등식** 풀이용
> - 먼저 Unify로 모든 등식을 풀고, **그 다음 부등식들을 검사**
>   - 부등식 위반이 있으면 "not ok", 없으면 "ok"

### 해설

**개념 설명 — 등식 먼저, 부등식 나중**

단일화는 등식만 다루므로, 부등식(`≠◇`)은 별도 단계로 처리합니다: **① 모든 등식을 Unify로 풀어 타입을 확정 → ② 부등식들을 검사**. 등식을 다 풀어야 각 변수의 최종 타입을 알고 ◇인지 판정할 수 있기 때문. 이렇게 등식·부등식을 분리 처리합니다. 여기까지가 분석 확장. 슬18부터 이 분석의 **한계**.

---

## 슬라이드 18: Limitations — Flow-Insensitivity

### 원문 내용
> ```rust
> fn f() { let x = 1; let y = x + 2; x = true; if x { y } else { 0 } }
> ```
> - It does not incur a type error at runtime
>   - Common pattern in dynamically typed languages
> - However, the analysis will say "not ok"
> - The analysis ignores the order of execution and computes a single type for each identifier regardless of the program point at which it is used
> - Such an analysis is called a flow-insensitive analysis
> - We will cover flow-sensitive analyses later in the course

### 번역
> 코드: x를 처음 1(i32)로 썼다가 나중에 `x = true`(bool)로 재대입. 런타임 타입 오류는 없지만(동적 언어 흔한 패턴), **분석은 "not ok"**.
> - 분석은 **실행 순서를 무시**하고 각 식별자에 **단 하나의 타입**을 부여 → x가 i32이자 bool이어야 해 모순.
> - 이런 분석을 **흐름 무감각(flow-insensitive)** 분석이라 함.

### 해설

**개념 설명 — 한계 1: 흐름 무감각 ★**

이 타입 분석은 **흐름 무감각**입니다 — 변수마다 **위치와 무관하게 단 하나의 타입**을 부여합니다. 그래서 "처음엔 i32였다가 나중에 bool"인 코드(동적 언어에선 정상)를 "x가 i32이자 bool"이라 보고 잘못 거부합니다(헛경보).

이를 고치려면 **흐름 감각(flow-sensitive)** 분석 — 각 프로그램 지점마다 타입을 따로 추적 — 이 필요한데, 그것이 강의 7~9의 데이터플로우 분석, 강의 15의 흐름 감각 포인터 분석입니다. 이 한계가 이후 강의의 동기. 두 번째 한계가 슬19.

---

## 슬라이드 19: Limitations — Polymorphism (Problem)

### 원문 내용
> ```rust
> fn f(x) { x }
> fn g() { f(1); f(true) }
> ```
> - It does not incur a type error at runtime
>   - f is polymorphic (generic function); We can type it fn f<T>(x: T) -> T { x } in Rust
> - However, the analysis will say "not ok"
>   - ⟦x⟧ needs to be both i32 and bool

### 번역
> `f(x){x}`(항등 함수)를 `f(1)`(i32)과 `f(true)`(bool)로 호출. f는 다형(제네릭 `fn f<T>`)이라 런타임 오류 없음. 그런데 **분석은 "not ok"** — `f(1)`이 ⟦x⟧=i32를, `f(true)`이 ⟦x⟧=bool을 강제해 모순.

### 해설

**개념 설명 — 한계 2: 단형(monomorphism) ★**

이 분석은 **단형(monomorphic)**입니다 — 함수 f에 **하나의 타입만** 부여합니다. 그런데 항등 함수 f는 다형(`fn f<T>(x:T)->T`)이라 i32로도 bool로도 쓸 수 있어야 합니다. 분석은 `f(1)`과 `f(true)`가 같은 ⟦x⟧에 다른 타입을 요구해 모순이라 거부 → 헛경보. 다형 함수를 못 다루는 것. 해법(let-다형성)이 슬20.

---

## 슬라이드 20: Limitations — Polymorphism (Solution)

### 원문 내용
> - To address this, we need to instantiate a function with different types at different call sites
>   - ⟦f⟧ = ∀X. fn(X) → X
>   - In f(1), ⟦f⟧ is instantiated to fn(i32) → i32
>   - In f(true), ⟦f⟧ is instantiated to fn(bool) → bool
> - This is the key idea of Hindley-Milner algorithm and often called let-polymorphism
> - The time complexity is exponential in the worst case¹²
> - Later in the course, we will cover context-sensitive analyses, which distinguish different call sites of a function and are similar to let-polymorphism

### 번역
> - 해법: 함수를 **호출 지점마다 다른 타입으로 인스턴스화(instantiate)**
>   - `⟦f⟧ = ∀X. fn(X)→X` (전칭 한정, 다형 타입)
>   - `f(1)`에선 `fn(i32)→i32`로, `f(true)`에선 `fn(bool)→bool`로 인스턴스화
> - 이것이 **Hindley-Milner의 핵심, let-다형성(let-polymorphism)**
> - 최악의 경우 **지수 시간**
> - 나중에 배울 **문맥 민감(context-sensitive) 분석**이 호출 지점을 구분하는 점에서 let-다형성과 유사

### 해설

**개념 설명 — let-다형성과 문맥 민감의 연결 ★**

해법은 함수에 **다형 타입 `∀X. fn(X)→X`**(주 타입, 강의 3 슬19)를 주고, **각 호출 지점에서 X를 다르게 인스턴스화**하는 것입니다. `f(1)`→X=i32, `f(true)`→X=bool. 이것이 **let-다형성**(Hindley-Milner의 핵심).

**중요한 연결**: "호출 지점마다 다르게 다룬다"는 발상은 **강의 10의 문맥 민감(context-sensitive) 분석**과 똑같습니다 — 같은 함수라도 호출 문맥별로 분석을 분리. 즉 let-다형성은 문맥 민감의 타입 버전. 단 **지수 시간**이 들 수 있어(슬20 각주) 정밀도-비용 trade-off(강의 1 슬29). let-다형성도 한계가 있습니다(슬21~22).

---

## 슬라이드 21: Let-Polymorphism

### 원문 내용
> - Even with let polymorphism, false alarms are unavoidable
> - No higher-rank polymorphism
> ```rust
> fn f(x) { x }
> fn g(y) { y(1); y(true) }
> fn h() { g(f) }
> ```
> - y is a parameter and cannot be polymorphically instantiated at different call sites

### 번역
> - let-다형성으로도 **헛경보를 완전히 없앨 순 없다**
> - **고차 다형성(higher-rank polymorphism)이 없음**
> - `g(y){ y(1); y(true) }`: y는 **매개변수**라 호출 지점마다 다르게 인스턴스화 못 함 → 여전히 거부

### 해설

**개념 설명 — let-다형성의 한계 1: 고차 다형성 없음**

let-다형성은 **let으로 묶인 함수(이름 붙은 함수)**만 다형으로 인스턴스화합니다. **매개변수로 받은 함수 y**는 안 됩니다 — y는 g 호출 시 하나의 타입으로 고정되므로, g 안에서 `y(1)`과 `y(true)`를 둘 다 하면 모순. 이것이 **고차 다형성(higher-rank polymorphism)**의 부재 — 다형 함수를 인자로 받아 여러 타입으로 쓰는 것은 불가. Rust도 기본적으로 이 제약이 있습니다. 또 다른 한계가 슬22.

---

## 슬라이드 22: Let-Polymorphism (cont.)

### 원문 내용
> - No polymorphic recursion
> ```rust
> fn f(x, n) {
>   if n > 1 { f(true, n - 1); } else if n == 1 { f(0, n - 1); }
>   x
> }
> ```
> - Each recursive call requires x to have a different type
> - With unrestricted higher-rank polymorphism or polymorphic recursion, solving the constraints becomes undecidable³⁴⁵

### 번역
> - **다형 재귀(polymorphic recursion)가 없음**
> - 위 코드: 재귀 호출마다 x가 다른 타입(true→bool, 0→i32)을 요구 → 단일 타입으로 못 맞춤 → 거부
> - **무제한 고차 다형성이나 다형 재귀를 허용하면 제약 풀이가 결정 불가능(undecidable)**

### 해설

**개념 설명 — let-다형성의 한계 2: 다형 재귀 없음, 그리고 결정 불가능성 ★**

**다형 재귀**(재귀 호출마다 다른 타입)도 let-다형성으로 안 됩니다. 더 중요한 점: **이런 고급 다형성을 무제한 허용하면 타입 추론 자체가 결정 불가능(undecidable)**해집니다(각주의 Henglein 1993 등). 즉 강의 1의 Rice 정리·결정 불가능성이 타입 추론에서도 나타납니다 — 표현력을 높이면 결정 가능성을 잃습니다. 그래서 실용 언어는 다형성을 **결정 가능한 범위**(let-다형성)로 제한합니다. 또 다른 종류의 한계(타입 외 오류)가 슬23.

---

## 슬라이드 23: Limitations — Other Runtime Errors

### 원문 내용
> - Division by zero
> - Stack-use-after-scope
>   - Catching dangling references requires lifetime/region analysis, which is beyond the scope of this type analysis
> ```rust
> fn f() { let x = 0; &x }
> fn g() { *f() }
> ```

### 번역
> - **0으로 나누기**: 타입 분석으로는 못 잡음(값 분석 필요)
> - **스코프 벗어난 스택 사용(dangling 참조)**: `f()`가 지역 변수 x의 참조 `&x`를 반환 → x는 f 종료 시 사라지므로 `*f()`는 무효 참조. 이를 잡으려면 **수명/영역 분석(lifetime/region analysis)**이 필요(이 타입 분석의 범위 밖)

### 해설

**개념 설명 — 타입 분석이 못 잡는 오류들**

타입 분석은 **타입 오류만** 잡습니다. 다른 런타임 오류는 못 잡습니다:
- **0으로 나누기**: 값(범위)을 추적해야 함 → 구간 분석(강의 9).
- **dangling 참조**(스코프 벗어난 참조 사용): 참조의 수명을 추적해야 함 → 수명/영역 분석(강의 2의 lifetime, Rust 빌림 검사). `f()`가 지역 변수 주소를 반환하는 것은 타입은 맞지만(둘 다 `&i32`) 의미적으론 위험.

즉 **각 분석은 자기 영역의 오류만 잡습니다** — 타입 분석은 타입, 구간 분석은 값 범위, 포인터 분석은 앨리어싱. 강의 1의 "분석은 특정 성질만 알아낸다"의 구체화. 전체 요약이 슬24.

---

## 슬라이드 24: Summary

### 원문 내용
> - Constraints are solved by the Unify algorithm, which uses union-find for type variable equivalences and a mapping for variable-to-type bindings
> - Tuple types require an absent-element type ◇ and inequality constraints to correctly reject out-of-bounds projections
> - The analysis is flow-insensitive and monomorphic; let-polymorphism addresses the latter but with exponential worst-case complexity

### 번역
> - 제약은 **Unify 알고리즘**으로 풀림 — 변수 동치는 Union-Find, 변수↔타입 바인딩은 사상 M
> - 튜플 타입은 **없는 원소 타입 ◇와 부등식 제약**으로 범위 밖 프로젝션을 올바르게 거부
> - 이 분석은 **흐름 무감각·단형**; let-다형성이 단형을 해결하나 최악 지수 복잡도

### 해설

**전체 정리 — 강의 4의 한 장 요약**

1. **단일화 알고리즘**: Resolve(대표 찾기) → Unify(등식 처리: 변수-변수는 Union, 변수-타입은 M 바인딩, occurs check로 무한 타입 차단, 같은 생성자는 구조 분해) → DeepResolve(최종 타입). Union-Find(동치) + 사상 M(바인딩).
2. **튜플 확장**: 1차(과엄격)·2차(과관대) 실패 후, **◇ + 부등식**으로 올바르게 해결. 등식은 Unify, 부등식은 사후 검사.
3. **한계**: 흐름 무감각(위치별 타입 못 함), 단형(let-다형성으로 일부 해결, 지수 시간), 고차 다형성·다형 재귀는 결정 불가능, 타입 외 오류(0 나누기·dangling)는 못 잡음.

**다른 강의와의 연결 (파일 간 연결성)**

- ← **강의 3**: Union-Find·제약 수집의 직접 연장. 이 강의가 단일화를 완성.
- ← **강의 1**: 헛경보(흐름 무감각·단형의 false alarm), 결정 불가능성(다형 재귀)이 Rice 정리와 통함. 각 분석은 특정 성질만.
- → **강의 7~9 (데이터플로우)**: 흐름 무감각의 한계(슬18)를 흐름 감각으로 해결.
- → **강의 10 (절차간·문맥 민감)**: let-다형성(호출 지점별 인스턴스화, 슬20)이 문맥 민감 분석과 동형. "호출 지점 구분".
- → **강의 14 (Steensgaard)**: 같은 Unify(Union-Find + occurs check)를 포인터에 적용. 함수 타입 단일화 ↔ 포인터 항 단일화.
- → **강의 9 (구간)·강의 2 (수명)**: 타입 분석이 못 잡는 0 나누기·dangling을 각각 값 분석·수명 분석이 담당(슬23).

**가장 큰 교훈**: **분석을 만드는 것은 시행착오**(튜플 3번 시도)이고, **모든 분석엔 한계가 있습니다**(흐름 무감각·단형·타입 외 오류 못 잡음). 흐름 무감각·단형이라는 두 한계는 각각 흐름 감각 분석(강의 7~9)과 문맥 민감 분석(강의 10)의 동기가 되며, "표현력을 높이면 결정 가능성을 잃는다"(다형 재귀의 결정 불가능성)는 강의 1의 근본 한계가 타입에서 재현되는 모습입니다.

---

## 마치며

강의 4는 단일화 알고리즘을 완성하고, 튜플 확장의 시행착오를 통해 **분석 설계의 현실(한 번에 완벽하지 않다)**을 보여 주며, 이 단순 타입 분석의 한계를 정직하게 드러냅니다. 핵심 한 줄: **"타입 제약은 Union-Find와 사상으로 단일화해 풀고, 분석을 확장할 땐 ◇·부등식 같은 장치가 필요하며, 흐름 무감각·단형이라는 근본 한계는 이후의 흐름 감각·문맥 민감 분석으로 보완된다."** 시험에서는 (a) Unify/Resolve/DeepResolve/Occurs의 동작과 occurs check의 역할(슬3~7), (b) 튜플 분석의 세 시도와 ◇·부등식의 필요성(슬10~16), (c) 흐름 무감각이 무엇이고 왜 헛경보를 내는가(슬18), (d) 단형 vs let-다형성과 문맥 민감과의 유사성(슬19~20), (e) 고차 다형성·다형 재귀가 없는 이유(결정 불가능성, 슬21~22)가 단골입니다.
