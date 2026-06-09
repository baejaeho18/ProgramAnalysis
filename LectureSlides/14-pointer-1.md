# Pointer Analysis (1) - 강의 슬라이드 해설

CSE552 Program Analysis — Lecture 14
강사: Jaemin Hong

> **이 파일을 읽는 법**: 각 슬라이드는 `원문 내용` → `번역` → `해설`(개념·배경·맥락) → 필요 시 `각주`/`슬라이드 연결` 순서입니다. 누락·왜곡 없이 원문을 모두 담되 처음 보는 사람도 따라올 수 있게 풀어 썼습니다.

---

## 강의 14 전체 조감도 (먼저 큰 그림)

이번 강의부터 **포인터 분석(Pointer Analysis)**이라는 정적 분석의 핵심 주제로 들어갑니다. 질문은 단순합니다: **"각 포인터 변수는 어떤 메모리를 가리킬 수 있는가?"** 이 정보(points-to)를 알면 **앨리어싱(aliasing, 두 포인터가 같은 곳을 가리킴)**을 판단할 수 있고, 그러면 거의 모든 다른 분석(데이터플로우·최적화·버그 검출)이 포인터를 통한 메모리 접근을 정확히 다룰 수 있게 됩니다.

이 강의의 뼈대:
1. **추상화의 두 축** — 무한한 런타임 메모리를 유한하게 다루는 **할당 지점 추상화(allocation-site abstraction)**, 그리고 분석 목표인 **points-to 함수** (슬라이드 2~4)
2. **Andersen 방식** (슬라이드 5~10) — 대입을 **포함(subset) 제약**으로 보고 **cubic 알고리즘(강의 11)**으로 풂. 정밀하지만 O(n³).
3. **Steensgaard 방식** (슬라이드 11~20) — 대입을 **양방향(등식)**으로 보고 **단일화(unification)**로 풂. 거의 선형(almost linear)이지만 덜 정밀.
4. **절차간 + 함수 포인터** (슬라이드 21~25) — 함수 포인터가 있으면 **제어 흐름 분석(강의 11)과 points-to 분석이 서로 의존**하므로 **동시에** 풀어야 함.

핵심 대비축은 **Andersen(정밀·느림) vs Steensgaard(거침·빠름)** — 강의 10에서 본 정밀도-비용 트레이드오프의 또 다른 사례입니다. 그리고 강의 11의 cubic 알고리즘이 또 등장합니다(Andersen이 그 직접 응용).

---

## 슬라이드 1: 제목 슬라이드

### 원문 내용
> Pointer Analysis (1)
> CSE552 Program Analysis — Lecture 14
> Jaemin Hong

### 번역
> 포인터 분석 (1) / CSE552 프로그램 분석 — 강의 14 / 홍재민

### 해설
포인터 분석 2부작의 1편. 이번 편은 **흐름 무감각(flow-insensitive)** 포인터 분석의 두 고전(Andersen, Steensgaard)을 다룹니다.

---

## 슬라이드 2: Motivation

### 원문 내용
> ```
> *x = 42;
> *y = -87;
> z = *x;
> ```
> - The value of z depends on whether x and y are aliases or not

### 번역
> 코드: `*x`에 42를, `*y`에 -87을 쓴 뒤 `z = *x`로 `*x`를 읽는다.
> - **z의 값은 x와 y가 앨리어스인지 아닌지에 달려 있다**

### 해설

**개념 설명 — 왜 포인터 분석이 필요한가**

이 세 줄이 포인터 분석의 존재 이유를 압축합니다. `z = *x`의 결과는:
- **x와 y가 다른 곳을 가리키면(앨리어스 아님)**: `*x`는 여전히 42 → z=42.
- **x와 y가 같은 곳을 가리키면(앨리어스)**: `*y=-87`이 `*x`도 -87로 바꿈 → z=-87.

**앨리어싱(aliasing)**: 둘 이상의 포인터(또는 이름)가 **같은 메모리 위치**를 가리키는 현상. 앨리어스 여부를 모르면, 컴파일러는 `*x`에 무엇이 들었는지 확신할 수 없어 어떤 분석·최적화도 보수적으로 망가집니다. 그래서 "각 포인터가 무엇을 가리키나"를 알아내는 **points-to 분석**이 필요합니다.

**배경 지식**: 강의 12에서 락 분석이 "앨리어싱을 무시해 unsound"였던 것을 떠올리세요. 이 강의가 바로 그 앨리어싱을 제대로 다루는 법입니다.

**슬라이드 연결**: 무한한 메모리를 유한하게 다루는 추상화가 슬라이드 3.

---

## 슬라이드 3: Allocation-Site Abstraction

### 원문 내용
> - A program can have infinitely many memory cells (memory locations) at runtime
> - We introduce
>   - an abstract cell x for every program variable x
>   - an abstract cell alloc-i for each occurrence of an alloc operation in the program, where i is a unique index
> - Each abstract cell represents the set of cells at runtime that are allocated at the corresponding source location
> - We use Cell to denote the set of abstract cells for the given program

### 번역
> - 프로그램은 런타임에 **무한히 많은 메모리 셀(위치)**을 가질 수 있다
> - 우리는 다음을 도입한다:
>   - 각 프로그램 변수 x마다 **추상 셀 x**
>   - 프로그램 내 각 `alloc` 연산 발생마다 **추상 셀 alloc-i** (i는 고유 인덱스)
> - 각 추상 셀은, 그에 대응하는 소스 위치에서 할당된 **런타임 셀들의 집합**을 나타낸다
> - 주어진 프로그램의 추상 셀들의 집합을 **Cell**이라 표기한다

### 해설

**개념 설명 — 무한을 유한으로 (핵심 추상화)**

런타임 메모리는 무한할 수 있습니다(예: 루프 안 `malloc`은 매 반복마다 새 메모리). 유한한 분석으로 다루려면 **추상화**가 필요합니다. 아이디어:
- **변수 셀**: 각 변수 x는 추상 셀 `x` 하나로.
- **할당 셀**: 코드의 각 `alloc()` *위치*(소스 줄)는 추상 셀 `alloc-i` 하나로. 그 위치에서 런타임에 몇 개가 만들어지든, **모두 하나의 추상 셀로 뭉뚱그립니다**.

이것이 **할당 지점 추상화(allocation-site abstraction)**입니다. "어디서 할당됐는가(소스 위치)"로 메모리를 분류 → 위치는 유한하므로 추상 셀도 유한(`Cell`). 강의 5~6의 추상화 철학(무한 구체 → 유한 추상)의 메모리판입니다.¹

**각주**

¹ 비용: 같은 줄에서 만든 두 객체를 구분 못 함(둘 다 `alloc-i`). 그래서 루프 안 할당은 정밀도가 떨어집니다. 더 정밀하게 하려면 호출 문맥별로 셀을 나누는 등(강의 10의 문맥 민감) 기법이 필요. 트레이드오프의 시작.

**슬라이드 연결**: 이 추상 셀들 위에서 정의되는 분석 목표가 슬라이드 4.

---

## 슬라이드 4: Points-To Analysis

### 원문 내용
> - Points-to analysis determines the set of abstract memory cells that each pointer variable may point to
>   - computes a function pt : Var → 𝒫(Cell)
> - Points-to information can be used to approximate facts related to pointers
>   - e.g., x and y may be aliases if pt(x) ∩ pt(y) ≠ ∅

### 번역
> - **Points-to 분석**은 각 포인터 변수가 가리킬 수 있는 **추상 메모리 셀들의 집합**을 결정한다
>   - 함수 **pt : Var → 𝒫(Cell)**을 계산 (변수 → 셀 집합)
> - Points-to 정보로 포인터 관련 사실을 근사할 수 있다
>   - 예: `pt(x) ∩ pt(y) ≠ ∅`이면 x와 y는 **앨리어스일 수 있다**

### 해설

**개념 설명 — 분석의 산출물 pt**

points-to 분석의 결과는 함수 `pt`입니다: 각 변수에 "그 변수가 가리킬 수 있는 셀들의 집합"을 대응. 예) `pt(x) = {a, alloc-1}`이면 "x는 변수 a 또는 1번 할당 위치의 메모리를 가리킬 수 있다".

**앨리어스 판정**: `pt(x)`와 `pt(y)`가 **하나라도 공통 셀**을 가지면(`∩≠∅`), x와 y는 같은 곳을 가리킬 *수 있음* → 앨리어스 가능. 슬라이드 2의 핵심 질문에 답하는 도구. (반대로 교집합이 비면 절대 앨리어스 아님 — 확실한 정보.)

"may point to"의 *may*에 주목: 보수적 과근사(강의 5~6). 실제보다 더 많이 가리킨다고 볼 수 있어도 안전.

**슬라이드 연결**: pt를 계산하는 첫 방법(Andersen)이 슬라이드 5.

---

## 슬라이드 5: Andersen-Style Analysis — Nodes and Constraint Variables

### 원문 내용
> Node v ::= x = alloc()
>          | x = &y
>          | x = y
>          | x = *y
>          | *x = y
>
> - For each abstract cell c, we introduce a constraint variable ⟦c⟧, which ranges over sets of abstract cells
>
> (각주 1: Program analysis and specialization for the C programming language (Andersen, 1994))

### 번역
> 분석이 다루는 다섯 가지 포인터 문장(노드):
> - `x = alloc()` (메모리 할당)
> - `x = &y` (주소 취득)
> - `x = y` (포인터 복사)
> - `x = *y` (역참조 읽기)
> - `*x = y` (역참조 쓰기)
> - 각 추상 셀 c마다 **제약 변수 ⟦c⟧**를 도입하며, 이는 추상 셀들의 집합 위에서 값을 가진다

### 해설

**개념 설명 — 다섯 가지 기본 포인터 문장**

모든 포인터 조작은 이 다섯 형태로 환원됩니다:
- `x = alloc()`: 새 메모리를 만들고 x가 그것을 가리킴.
- `x = &y`: y의 주소를 x에. (x가 셀 y를 가리킴)
- `x = y`: y가 가리키는 것을 x도 가리킴(복사).
- `x = *y`: y가 가리키는 셀의 *내용*(포인터)을 x에. (한 단계 역참조)
- `*x = y`: x가 가리키는 셀에 y가 가리키는 것을 써넣음.

각 셀 c마다 제약 변수 `⟦c⟧`("c가 가리킬 수 있는 셀들의 집합")를 둡니다 — 강의 11의 `[x]`(가능한 함수 집합)와 똑같은 발상, 토큰이 "함수"에서 "셀"로 바뀐 것뿐.

**각주**: Andersen(1994)의 박사 논문이 출처. **포함 기반(inclusion-based)** 포인터 분석의 원조.

**슬라이드 연결**: 다섯 문장 각각의 제약 규칙이 슬라이드 6.

---

## 슬라이드 6: Andersen-Style Analysis — Constraint Rules

### 원문 내용
> - x = alloc(): alloc-i ∈ ⟦x⟧
> - x = &y: y ∈ ⟦x⟧
> - x = y: ⟦y⟧ ⊆ ⟦x⟧
> - x = *y: ∀c ∈ Cell. c ∈ ⟦y⟧ ⇒ ⟦c⟧ ⊆ ⟦x⟧
> - *x = y: ∀c ∈ Cell. c ∈ ⟦x⟧ ⇒ ⟦y⟧ ⊆ ⟦c⟧
> - Constraints can be solved using the cubic algorithm
> - pt(x) = ⟦x⟧

### 번역
> - `x = alloc()` ⟹ alloc-i ∈ ⟦x⟧ (x는 그 할당 셀을 가리킴)
> - `x = &y` ⟹ y ∈ ⟦x⟧ (x는 셀 y를 가리킴)
> - `x = y` ⟹ ⟦y⟧ ⊆ ⟦x⟧ (y가 가리키는 것을 x도)
> - `x = *y` ⟹ 모든 셀 c에 대해, c ∈ ⟦y⟧이면 ⟦c⟧ ⊆ ⟦x⟧ (y가 c를 가리키면, c가 가리키는 것을 x도)
> - `*x = y` ⟹ 모든 셀 c에 대해, c ∈ ⟦x⟧이면 ⟦y⟧ ⊆ ⟦c⟧ (x가 c를 가리키면, y가 가리키는 것을 c도)
> - 제약식은 **cubic 알고리즘**(강의 11)으로 풀 수 있다
> - **pt(x) = ⟦x⟧**

### 해설

**개념 설명 — 다섯 규칙을 직관으로**

핵심은 마지막 두 규칙(역참조)에 **조건부**가 등장한다는 것입니다 — 강의 11의 `t∈x⇒y⊆z` 형태!
- `x=alloc()`/`x=&y`: 토큰 심기(`t∈⟦x⟧`). 강의 11의 `t∈x`.
- `x=y`: 포함 전파(`⟦y⟧⊆⟦x⟧`). 강의 11의 `x⊆y`.
- `x=*y` (읽기): y가 무엇을 가리키는지 모르므로, "**만약** y가 c를 가리키면(`c∈⟦y⟧`), 그 c가 가리키는 것(`⟦c⟧`)을 x도 가리킨다"는 조건부. 강의 11의 `t∈x⇒y⊆z`.
- `*x=y` (쓰기): "만약 x가 c를 가리키면(`c∈⟦x⟧`), y가 가리키는 것을 c에 써넣는다(`⟦y⟧⊆⟦c⟧`)"는 조건부.

세 형태(`t∈x`, `x⊆y`, `t∈x⇒y⊆z`)뿐이므로 **강의 11의 cubic 알고리즘이 그대로** 적용됩니다 → O(n³). 그리고 `pt(x)=⟦x⟧`로 해가 곧 points-to 정보.

**중요**: Andersen은 **흐름 무감각(flow-insensitive)**입니다 — 문장 순서를 무시하고 제약을 한꺼번에 모아 풉니다(강의 7~8의 흐름 감각 데이터플로우와 대조). 그래서 "프로그램 어딘가에서 x=y이면 영원히 ⟦y⟧⊆⟦x⟧".

**슬라이드 연결**: 네 개의 예제(슬라이드 7~10)가 각 규칙을 구체화.

---

## 슬라이드 7: Andersen — Example 1

### 원문 내용
> ```
> x = &a;
> y = &b;
> if ... {
>   z = x;
> } else {
>   z = y;
> }
> ```
> Constraints:
> - x = &a: a ∈ ⟦x⟧
> - y = &b: b ∈ ⟦y⟧
> - z = x: ⟦x⟧ ⊆ ⟦z⟧
> - z = y: ⟦y⟧ ⊆ ⟦z⟧
> Solutions:
> - ⟦x⟧ = {a}
> - ⟦y⟧ = {b}
> - ⟦z⟧ = {a, b}

### 번역
> x는 a를, y는 b를 가리킴. z는 분기에 따라 x 또는 y가 됨 → z는 a와 b 둘 다 가리킬 수 있음.
> 해: pt(x)={a}, pt(y)={b}, **pt(z)={a,b}**.

### 해설

**개념 설명**

`x=&a`→`a∈⟦x⟧`, `y=&b`→`b∈⟦y⟧`. 분기는 둘 다 가능하므로 `z=x`와 `z=y` 양쪽 제약 모두 생성 → `⟦z⟧⊇⟦x⟧∪⟦y⟧={a,b}`. z가 a와 b를 **구분해서** 가리킴(그래프: z→a, z→b 별개). 이 "구분"이 Andersen의 정밀함이며, 슬라이드 16에서 Steensgaard와 대비됩니다.

---

## 슬라이드 8: Andersen — Example 2

### 원문 내용
> ```
> x = &a;
> y = &x;
> z = *y;
> ```
> Constraints:
> - x = &a: a ∈ ⟦x⟧
> - y = &x: x ∈ ⟦y⟧
> - z = *y: x ∈ ⟦y⟧ ⇒ ⟦x⟧ ⊆ ⟦z⟧ ∧ y ∈ ⟦y⟧ ⇒ ⟦y⟧ ⊆ ⟦z⟧ ∧ z ∈ ⟦y⟧ ⇒ ⟦z⟧ ⊆ ⟦z⟧ ∧ a ∈ ⟦y⟧ ⇒ ⟦a⟧ ⊆ ⟦z⟧
> Solutions:
> - ⟦x⟧ = {a}
> - ⟦y⟧ = {x}
> - ⟦z⟧ = {a}

### 번역
> x→a, y→x. `z=*y`(역참조 읽기): y가 가리키는 셀의 내용을 z로. y={x}이므로 활성화되는 조건은 `x∈⟦y⟧ ⇒ ⟦x⟧⊆⟦z⟧` → ⟦z⟧⊇⟦x⟧={a}.
> 해: pt(x)={a}, pt(y)={x}, **pt(z)={a}**.

### 해설

**개념 설명 — 역참조 규칙의 작동**

`z=*y`는 모든 셀 c에 대한 조건부를 만들지만, 실제로 발동되는 건 `⟦y⟧`에 든 셀(여기선 x)에 대한 것뿐입니다. `x∈⟦y⟧`가 참 → `⟦x⟧⊆⟦z⟧` 발동 → `⟦z⟧={a}`. 나머지 조건(y,z,a∈⟦y⟧)은 거짓이라 잠든 채로. 강의 11의 조건부 제약 처리(`cond` 자료구조)가 이 "필요한 것만 발동"을 효율적으로 처리합니다.

**해석**: `z = *y = *(&x) = x`이고 x는 a를 가리키므로 z도 a를 가리킴. 분석이 한 단계 역참조를 정확히 추적.

---

## 슬라이드 9: Andersen — Example 3

### 원문 내용
> ```
> x = &a;
> y = &b;
> *x = y;
> ```
> Constraints:
> - x = &a: a ∈ ⟦x⟧
> - y = &b: b ∈ ⟦y⟧
> - *x = y: x ∈ ⟦x⟧ ⇒ ⟦y⟧ ⊆ ⟦x⟧ ∧ y ∈ ⟦x⟧ ⇒ ⟦y⟧ ⊆ ⟦y⟧ ∧ a ∈ ⟦x⟧ ⇒ ⟦y⟧ ⊆ ⟦a⟧ ∧ b ∈ ⟦x⟧ ⇒ ⟦y⟧ ⊆ ⟦b⟧
> Solutions:
> - ⟦x⟧ = {a}
> - ⟦y⟧ = {b}
> - ⟦a⟧ = {b}

### 번역
> x→a, y→b. `*x = y`(역참조 쓰기): x가 가리키는 셀에 y가 가리키는 것을 씀. x={a}이므로 발동되는 조건은 `a∈⟦x⟧ ⇒ ⟦y⟧⊆⟦a⟧` → ⟦a⟧⊇⟦y⟧={b}.
> 해: pt(x)={a}, pt(y)={b}, **pt(a)={b}**.

### 해설

**개념 설명 — 역참조 쓰기의 작동**

`*x=y`는 "x가 가리키는 곳에 쓰기"입니다. x={a}이므로 셀 a에 y의 내용(b)을 써넣음 → `⟦a⟧={b}` (즉 a가 b를 가리키게 됨). 그래프: x→a→b. 이제 `a` 자신이 포인터로서 b를 가리킵니다(`*x=*(&a)=a`에 b를 대입한 효과). 슬라이드 8(읽기)과 9(쓰기)가 조건부의 두 방향(`⟦c⟧⊆⟦x⟧` vs `⟦y⟧⊆⟦c⟧`)을 보여 줍니다.

---

## 슬라이드 10: Andersen — Example 4

### 원문 내용
> ```
> x = alloc(); // 0
> y = alloc(); // 1
> *x = y;
> ```
> Constraints:
> - x = alloc(): alloc-0 ∈ ⟦x⟧
> - y = alloc(): alloc-1 ∈ ⟦y⟧
> - *x = y: alloc-0 ∈ ⟦x⟧ ⇒ ⟦y⟧ ⊆ ⟦alloc-0⟧ ∧ alloc-1 ∈ ⟦x⟧ ⇒ ⟦y⟧ ⊆ ⟦alloc-1⟧
> Solutions:
> - ⟦x⟧ = {alloc-0}
> - ⟦y⟧ = {alloc-1}
> - ⟦alloc-0⟧ = {alloc-1}
> - ⟦alloc-1⟧ = ∅

### 번역
> x는 0번 할당 셀을, y는 1번 할당 셀을 가리킴. `*x=y`로 alloc-0에 y의 내용(alloc-1)을 씀.
> 해: pt(x)={alloc-0}, pt(y)={alloc-1}, pt(alloc-0)={alloc-1}, pt(alloc-1)=∅.

### 해설

**개념 설명 — 할당 셀의 등장**

슬라이드 9와 구조는 같지만 변수 셀 대신 **할당 셀(alloc-0, alloc-1)**을 씁니다. 각 `alloc()` 위치가 고유 셀이 됩니다(슬라이드 3). 그래프 x→alloc-0→alloc-1. 힙 메모리(동적 할당)도 추상 셀로 똑같이 다뤄짐을 보여 줍니다.

**슬라이드 연결**: 여기까지가 Andersen(정밀). 슬라이드 11부터 더 빠르지만 거친 Steensgaard.

---

## 슬라이드 11: Steensgaard-Style Analysis — Characteristics

### 원문 내용
> - Views assignments as being bidirectional
> - Less precise but more efficient than Andersen-style analysis
>
> (각주 2: Points-to analysis in almost linear time (Steensgaard, 1996))

### 번역
> - 대입을 **양방향(bidirectional)**으로 본다
> - Andersen 방식보다 **덜 정밀하지만 더 효율적**이다
> - (각주: Steensgaard 1996, "거의 선형 시간의 points-to 분석")

### 해설

**개념 설명 — 정밀도를 버리고 속도를 얻다**

Andersen은 `x=y`를 **단방향** 포함(`⟦y⟧⊆⟦x⟧`)으로 봤습니다. Steensgaard는 이를 **양방향 등식**(`⟦x⟧=⟦y⟧`)으로 봅니다 — "x와 y는 같은 것을 가리킨다"고 **합쳐 버림**. 이러면 정보가 거칠어지지만(한쪽만 가리켜도 둘 다 가리킨다고 봄), **단일화(unification)**로 거의 선형 시간(α(n), 거의 O(n))에 풀립니다.

이것이 강의 10의 정밀도-비용 트레이드오프의 극적 사례입니다: O(n³) Andersen vs 거의 O(n) Steensgaard. 큰 프로그램(수백만 줄)에선 속도가 결정적이라 Steensgaard가 실용적일 수 있습니다.

**각주**: Steensgaard(1996). **단일화 기반(unification-based)** 포인터 분석. union-find 자료구조로 거의 선형.

**슬라이드 연결**: 양방향을 어떻게 제약으로 표현하는지가 슬라이드 12.

---

## 슬라이드 12: Steensgaard — Constraint Rules

### 원문 내용
> - For each abstract cell c, we introduce a term variable ⟦c⟧
> - & is the only term constructor
>
> - x = alloc(): ⟦x⟧ = &⟦alloc-i⟧
> - x = &y: ⟦x⟧ = &⟦y⟧
> - x = y: ⟦x⟧ = ⟦y⟧
> - x = *y: ⟦y⟧ = &⟦x⟧
> - *x = y: ⟦x⟧ = &⟦y⟧

### 번역
> - 각 추상 셀 c마다 **항 변수(term variable) ⟦c⟧**를 도입
> - **&**가 유일한 항 생성자(term constructor)
> - `x = alloc()` ⟹ ⟦x⟧ = &⟦alloc-i⟧
> - `x = &y` ⟹ ⟦x⟧ = &⟦y⟧
> - `x = y` ⟹ ⟦x⟧ = ⟦y⟧
> - `x = *y` ⟹ ⟦y⟧ = &⟦x⟧
> - `*x = y` ⟹ ⟦x⟧ = &⟦y⟧

### 해설

**개념 설명 — 항(term)과 단일화**

Steensgaard는 각 셀에 **항 변수**를 주고, **`&t`** 라는 단 하나의 생성자로 "가리킴"을 표현합니다. `⟦x⟧ = &⟦y⟧`는 "x는 y를 가리킨다"는 뜻(슬라이드 13에서 직관). 모든 규칙이 **등식(=)**임에 주목하세요 — Andersen의 `⊆`(포함)와 대조.

- `x=&y`/`x=alloc()`: x는 y(또는 할당 셀)를 가리킴 → `⟦x⟧=&⟦y⟧`.
- `x=y`: x와 y가 같은 항 → `⟦x⟧=⟦y⟧`(단일화로 둘을 합침).
- `x=*y`: y가 가리키는 게 x가 가리키는 것과 같음 → `⟦y⟧=&⟦x⟧`.
- `*x=y`: x가 가리키는 게 y가 가리키는 것과 같음 → `⟦x⟧=&⟦y⟧`.

등식을 푸는 알고리즘이 **단일화(unification)** — 타입 추론(강의 3~4)에서 쓰던 바로 그 기법입니다.

**슬라이드 연결**: `&` 항의 직관이 슬라이드 13.

---

## 슬라이드 13: Steensgaard — Intuition

### 원문 내용
> - ⟦x⟧ = &⟦y⟧ intuitively means x points to y
>
> x = &y: ⟦x⟧ = &⟦y⟧     (x → y)
> x = *y: ⟦y⟧ = &⟦x⟧     (x ↓ ; y → · → ·)
> *x = y: ⟦x⟧ = &⟦y⟧     (x → · → · ; y ↗)

### 번역
> - `⟦x⟧ = &⟦y⟧`는 직관적으로 **"x가 y를 가리킨다"**는 뜻
> - 각 대입의 그림(화살표): `x=&y`는 x→y; `x=*y`와 `*x=y`는 한 단계 더 들어간 가리킴 관계를 만든다

### 해설

**개념 설명**

`&⟦y⟧`라는 항은 "y라는 셀을 가리키는 포인터"를 뜻합니다. `⟦x⟧=&⟦y⟧`이면 x의 항이 "y를 가리킴"이니 곧 x→y. 단일화는 이런 `&t` 항들을 맞춰 가며, 같다고 강제된 셀들을 union-find로 한 덩어리로 합칩니다. 그림은 각 대입이 만드는 가리킴 구조를 시각화한 것 — 직관적 이해를 돕습니다.

**슬라이드 연결**: 푸는 알고리즘이 슬라이드 14.

---

## 슬라이드 14: Steensgaard — Constraint Solving

### 원문 내용
> - Constraints can be solved using the unification algorithm
> - pt(x) = {c | ⟦x⟧ = &⟦c⟧}

### 번역
> - 제약식은 **단일화(unification) 알고리즘**으로 풀 수 있다
> - **pt(x) = {c | ⟦x⟧ = &⟦c⟧}** (x의 항이 &⟦c⟧와 같아지는 모든 셀 c)

### 해설

**개념 설명 — 단일화로 풀기**

단일화는 "두 항이 같다"는 등식들을 받아, 모순 없이 같아지도록 변수를 맞추는 알고리즘(union-find 기반, 거의 선형). `⟦x⟧=⟦y⟧`를 만나면 x와 y를 한 그룹으로 합치고, `&t1=&t2`면 t1과 t2를 합칩니다.

points-to 정보는 `pt(x) = {c | ⟦x⟧=&⟦c⟧}` — x의 항이 `&⟦c⟧`가 되는 셀들. 단일화로 합쳐진 셀들이 한꺼번에 pt에 들어가므로, **Andersen보다 큰(거친) pt 집합**이 나옵니다(슬라이드 16에서 확인).

**배경 지식 — 거의 선형의 비결**: union-find의 거의 상수 시간 연산(역 아커만 함수 α) 덕분에 전체가 거의 O(n). 강의 11의 cubic(O(n³))과 극명한 대비.

**슬라이드 연결**: 슬라이드 15~20이 같은 예제들을 Steensgaard로 풀고 Andersen과 비교.

---

## 슬라이드 15: Steensgaard — Example 1

### 원문 내용
> ```
> x = &a;
> y = &b;
> if ... { z = x; } else { z = y; }
> ```
> Constraints:
> - x = &a: ⟦x⟧ = &⟦a⟧
> - y = &b: ⟦y⟧ = &⟦b⟧
> - z = x: ⟦z⟧ = ⟦x⟧
> - z = y: ⟦z⟧ = ⟦y⟧
> Solutions:
> - ⟦z⟧ = ⟦x⟧ = ⟦y⟧ = &⟦a⟧ = &⟦b⟧
> - ⟦a⟧ = ⟦b⟧
> - pt(z) = pt(x) = pt(y) = {a, b}

### 번역
> `z=x`와 `z=y`가 모두 등식이라 ⟦z⟧=⟦x⟧=⟦y⟧로 셋이 합쳐짐. 그러면 &⟦a⟧=&⟦b⟧ → ⟦a⟧=⟦b⟧(a,b도 합쳐짐).
> 해: **pt(z)=pt(x)=pt(y)={a,b}** — 셋 다 같은 points-to.

### 해설

**개념 설명 — 양방향의 대가**

`z=x`를 등식 `⟦z⟧=⟦x⟧`로 보니 x와 z가 합쳐지고, `z=y`로 y도 합쳐져 **x=y=z가 한 덩어리**가 됩니다. 그 결과 그들이 가리키는 a와 b까지 합쳐져(`⟦a⟧=⟦b⟧`), **pt(x)=pt(y)=pt(z)={a,b}**.

여기서 정밀도 손실이 드러납니다: 실제로 x는 a만, y는 b만 가리키는데, Steensgaard는 셋을 합쳐 **x도 b를 가리킬 수 있다**고 (과하게) 봅니다. 슬라이드 16이 이 손실을 Andersen과 직접 비교.

---

## 슬라이드 16: Steensgaard Example 1 vs. Andersen

### 원문 내용
> **Steensgaard-style**: pt(z) = pt(x) = pt(y) = {a, b}  (x,y,z → a,b)
> **Andersen-style**: pt(z) = {a, b}, pt(x) = {a}, pt(y) = {b}

### 번역
> - **Steensgaard**: x,y,z 모두 {a,b} (셋이 하나로 뭉침)
> - **Andersen**: z={a,b}이지만 x={a}, y={b} (각자 구분)

### 해설

**개념 설명 — 정밀도 차이의 시각화**

같은 코드, 다른 답:
- **Andersen**: x→a, y→b를 **구분 유지**. z만 둘을 합침(분기 때문에 정당). pt(x)={a}는 정확.
- **Steensgaard**: x=y=z를 모두 합쳐 셋 다 {a,b}. pt(x)={a,b}는 **과근사**(x는 실제로 b를 안 가리킴).

핵심: Steensgaard는 **단방향이어야 할 정보를 양방향으로** 처리해 불필요하게 합칩니다. 정밀도↓ 대신 속도↑. "x와 y가 앨리어스인가?"를 물으면 Andersen은 "아니오"(정확), Steensgaard는 "예일 수도"(보수적). 둘 다 안전(sound)하지만 정밀도가 다름.

---

## 슬라이드 17: Steensgaard — Example 2

### 원문 내용
> ```
> x = &a;
> y = &x;
> z = *y;
> ```
> Constraints:
> - x = &a: ⟦x⟧ = &⟦a⟧
> - y = &x: ⟦y⟧ = &⟦x⟧
> - z = *y: ⟦y⟧ = &⟦z⟧
> Solutions:
> - ⟦y⟧ = &⟦x⟧ = &⟦z⟧
> - ⟦x⟧ = ⟦z⟧ = &⟦a⟧
> - pt(y) = {x, z}
> - pt(x) = pt(z) = {a}

### 번역
> `z=*y` ⟹ ⟦y⟧=&⟦z⟧. 그런데 ⟦y⟧=&⟦x⟧이기도 하므로 &⟦x⟧=&⟦z⟧ → ⟦x⟧=⟦z⟧(x,z 합쳐짐).
> 해: pt(y)={x,z}, pt(x)=pt(z)={a}.

### 해설

**개념 설명**

`z=*y`(읽기)가 `⟦y⟧=&⟦z⟧`를 강제하는데, 이미 `⟦y⟧=&⟦x⟧`이므로 **x와 z가 단일화**됩니다. 결과로 y는 {x,z}를 가리키고(x와 z가 합쳐졌으니), x=z={a}. Andersen이라면 pt(y)={x}였을 텐데(z는 y가 안 가리킴), Steensgaard는 역참조 때문에 x,z를 합쳐 pt(y)={x,z}로 거칠어집니다(슬라이드 18).

---

## 슬라이드 18: Steensgaard Example 2 vs. Andersen

### 원문 내용
> **Steensgaard**: pt(y) = {x, z}, pt(x) = pt(z) = {a}  (y → x,z → a)
> **Andersen**: pt(x) = {a}, pt(y) = {x}, pt(z) = {a}  (y → x → a, z → a)

### 번역
> - **Steensgaard**: y={x,z}, x=z={a} (x,z 뭉침)
> - **Andersen**: y={x}, x={a}, z={a} (x,z 구분)

### 해설

Andersen은 pt(y)={x}로 정확(y는 x만 가리킴)하지만, Steensgaard는 역참조에서 x,z를 합쳐 pt(y)={x,z}. 또 한 번 정밀도-속도 트레이드오프. 시험에서 "같은 코드에 두 분석을 적용해 pt를 구하고 차이를 설명하라"가 단골입니다.

---

## 슬라이드 19: Steensgaard — Example 3

### 원문 내용
> ```
> x = &a;
> y = &b;
> *x = y;
> ```
> Constraints:
> - x = &a: ⟦x⟧ = &⟦a⟧
> - y = &b: ⟦y⟧ = &⟦b⟧
> - *x = y: ⟦x⟧ = &⟦y⟧
> Solutions:
> - ⟦x⟧ = &⟦y⟧ = &⟦a⟧
> - ⟦y⟧ = ⟦a⟧ = &⟦b⟧
> - pt(x) = {a, y}
> - pt(y) = pt(a) = {b}

### 번역
> `*x=y` ⟹ ⟦x⟧=&⟦y⟧. 이미 ⟦x⟧=&⟦a⟧이므로 &⟦y⟧=&⟦a⟧ → ⟦y⟧=⟦a⟧(y,a 합쳐짐).
> 해: pt(x)={a,y}, pt(y)=pt(a)={b}.

### 해설
`*x=y`가 `⟦x⟧=&⟦y⟧`를 강제 → y와 a가 단일화. 그래서 x는 {a,y}(=합쳐진 한 셀)를 가리키고, y=a={b}. Andersen(슬라이드 9)에선 pt(x)={a}, pt(a)={b}로 더 정밀했죠. 슬라이드 20에서 비교.

---

## 슬라이드 20: Steensgaard Example 3 vs. Andersen

### 원문 내용
> **Steensgaard**: pt(x) = {a, y}, pt(z) = pt(y) = {b}  (x → a,y → b)
> **Andersen**: pt(x) = {a}, pt(y) = {b}, pt(a) = {b}  (x → a → b, y ↗ b)

### 번역
> - **Steensgaard**: x={a,y}, y=a={b} (a,y 뭉침)
> - **Andersen**: x={a}, y={b}, a={b} (구분)

### 해설
역참조 쓰기에서도 Steensgaard는 a,y를 합쳐 거칠어집니다. 세 예제(16,18,20)가 일관되게 보여 주는 결론: **Steensgaard ⊇ Andersen** (Steensgaard의 pt가 항상 Andersen의 pt를 포함, 즉 더 큰 과근사). 둘 다 sound, Andersen이 더 정밀.

**슬라이드 연결**: 여기까지 절차내. 슬라이드 21부터 함수 포인터가 있는 절차간 — 강의 11과 14가 만나는 지점.

---

## 슬라이드 21: Interprocedural Pointer Analysis

### 원문 내용
> ```
> *x = f;        fn f(a) { *a = &y; }
> (*y)(z);       fn g(b) { *b = &z; }
>                h(&x);
> ```
> - Deciding control flow requires points-to information
> - Deciding points-to information requires control flow information
> - We need to perform points-to analysis and control flow analysis simultaneously when we have pointers and function pointers

### 번역
> 함수 포인터가 섞인 코드. (`*x=f`로 함수를 메모리에 저장, `(*y)(z)`로 포인터를 통해 호출)
> - **제어 흐름을 결정하려면 points-to 정보가 필요**하다 (어떤 함수가 불릴지 알려면 포인터가 무엇을 가리키는지 알아야)
> - **points-to 정보를 결정하려면 제어 흐름 정보가 필요**하다 (함수 호출로 인자가 전달되는 흐름을 알아야)
> - 포인터와 함수 포인터가 함께 있으면 **points-to 분석과 제어 흐름 분석을 동시에** 수행해야 한다

### 해설

**개념 설명 — 닭과 달걀 (강의 11과 14의 상호 의존)**

이 슬라이드가 강의 11과 14를 묶는 핵심 통찰입니다. 함수 포인터가 있으면:
- **CFA(강의 11)는 pt가 필요**: `(*y)(z)`가 어느 함수를 부를지 = `pt(y)`가 어떤 함수 셀을 담는가.
- **pt(강의 14)는 CFA가 필요**: 그 함수 호출로 인자 z가 매개변수 a로 흐르는 points-to 전파를 알려면 호출 대상을 알아야.

서로가 서로를 필요로 하는 **순환 의존**입니다. 해법: 둘을 **하나의 통합 제약 시스템**으로 만들어 **동시에** 고정점까지 풉니다. 따로 풀면 한쪽 정보가 없어 진행 불가. 강의 11에서 "CFA가 절차간 분석의 토대"라 했지만, 함수 포인터가 있으면 그 토대(CFA)조차 pt 없이는 못 만든다는 것.

**슬라이드 연결**: 통합 방법이 Andersen(슬라이드 22~23)과 Steensgaard(24~25) 각각으로.

---

## 슬라이드 22: Andersen-Style Analysis (함수 포인터)

### 원문 내용
> - Can be directly combined with the control flow analysis
> - fn f(...) {...}: f ∈ ⟦f⟧
> - x = y(z1, ..., zn): ∀f. f ∈ ⟦y⟧ ⇒ (⟦z1⟧ ⊆ ⟦a_f^1⟧ ∧ ... ∧ ⟦zn⟧ ⊆ ⟦a_f^n⟧ ∧ ⟦RET_f⟧ ⊆ ⟦x⟧)

### 번역
> - **제어 흐름 분석(강의 11)과 직접 결합**할 수 있다
> - `fn f(...) {...}` ⟹ f ∈ ⟦f⟧ (함수 셀을 토큰으로)
> - `x = y(z1,...,zn)` ⟹ 모든 f에 대해, f ∈ ⟦y⟧이면 (인자 ⟦zi⟧⊆⟦a_f^i⟧, 반환 ⟦RET_f⟧⊆⟦x⟧)

### 해설

**개념 설명 — 강의 11의 규칙이 그대로**

이 두 규칙은 **강의 11 슬라이드 3의 CFA 제약 규칙과 글자 그대로 동일**합니다! 차이는 `⟦·⟧`가 "가능한 함수 집합"이 아니라 "가능한 셀 집합"(함수도 셀의 일종)이라는 것뿐. 즉 함수를 셀로 취급하면, **CFA가 points-to 분석에 자연히 흡수**됩니다.

`f∈⟦y⟧`(y가 함수 f를 가리킬 수 있음)이면 그 호출이 가능하다고 보고 인자·반환 흐름을 활성화. 강의 11의 cubic 알고리즘이 이 통합 제약을 푸므로, **pt와 CFA가 한 번의 고정점 계산으로 동시에** 나옵니다. 슬라이드 21의 순환 의존이 이렇게 해결됩니다.

**슬라이드 연결**: 예제가 슬라이드 23.

---

## 슬라이드 23: Andersen-Style Analysis — Example

### 원문 내용
> ```
> fn f(a) {}
> fn g(b) {}
> x = f;
> y = g;
> if ... { z = &x; } else { z = &y; }
> w = *z;
> u = &v;
> w(u);
> ```
> Constraints:
> - fn f: f ∈ ⟦f⟧;  fn g: g ∈ ⟦g⟧
> - x = f: ⟦f⟧ ⊆ ⟦x⟧;  y = g: ⟦g⟧ ⊆ ⟦y⟧
> - z = &x: x ∈ ⟦z⟧;  z = &y: y ∈ ⟦z⟧
> - w = *z: ∀c. c ∈ ⟦z⟧ ⇒ ⟦c⟧ ⊆ ⟦w⟧
> - u = &v: v ∈ ⟦u⟧
> - w(u): ∀f. f ∈ ⟦w⟧ ⇒ ⟦u⟧ ⊆ ⟦a_f^1⟧
> Solutions:
> - ⟦f⟧ = {f}, ⟦g⟧ = {g}
> - ⟦x⟧ = {f}, ⟦y⟧ = {g}
> - ⟦z⟧ = {x, y}, ⟦w⟧ = {f, g}
> - ⟦u⟧ = {v}, ⟦a⟧ = {v}, ⟦b⟧ = {v}

### 번역
> 함수 f,g를 변수에 담고, z는 x 또는 y의 주소, w=*z(역참조), 마지막에 w(u)로 포인터 호출. 분석 결과 w가 f,g 둘 다 가리킬 수 있으므로(⟦w⟧={f,g}), w(u) 호출은 f와 g 양쪽으로 갈 수 있어 인자 u(={v})가 a,b 둘 다로 흐름 → ⟦a⟧=⟦b⟧={v}.

### 해설

**개념 설명 — pt와 CFA가 함께 풀리는 전 과정**

단계를 따라가면 순환 의존이 풀리는 게 보입니다:
1. pt 계산: ⟦x⟧={f}, ⟦y⟧={g}, ⟦z⟧={x,y}, 역참조로 ⟦w⟧={f,g}.
2. 이제 ⟦w⟧={f,g}라는 **points-to 정보가 호출 대상(CFA)**을 결정: `w(u)`는 f와 g 둘 다 호출 가능.
3. 그 CFA 정보로 다시 **points-to 전파**: u={v}가 f의 매개변수 a와 g의 매개변수 b 양쪽으로 → ⟦a⟧=⟦b⟧={v}.

points-to(1)가 제어 흐름(2)을 정하고, 제어 흐름이 다시 points-to(3)를 정하는 — 슬라이드 21의 상호 의존이 한 제약 시스템 안에서 자연히 해소됩니다.

---

## 슬라이드 24: Steensgaard-Style Analysis (함수 포인터)

### 원문 내용
> - The term constructors represent both variable pointers and function pointers
>   - ⟦x⟧ = (&⟦y⟧, fn(⟦z1⟧, ..., ⟦zn⟧) → ⟦w⟧)
> - x = alloc(): ⟦x⟧ = (&⟦alloc-i⟧, _)
> - x = &y: ⟦x⟧ = (&⟦y⟧, _)
> - x = y: ⟦x⟧ = ⟦y⟧
> - x = *y: ⟦y⟧ = (&⟦x⟧, _)
> - *x = y: ⟦x⟧ = (&⟦y⟧, _)
> - fn f(x1,...,xn) { return y; }: ⟦f⟧ = (_, fn(⟦x1⟧,...,⟦xn⟧) → ⟦y⟧)
> - x = y(z1,...,zn): ⟦y⟧ = (_, fn(⟦z1⟧,...,⟦zn⟧) → ⟦x⟧)

### 번역
> - 항 생성자가 **변수 포인터와 함수 포인터를 모두** 표현: 각 셀의 항은 쌍 `(&⟦y⟧, fn(...)→⟦w⟧)` — 첫째는 "무엇을 가리키나(포인터)", 둘째는 "함수라면 어떤 시그니처인가".
> - 기존 다섯 규칙은 포인터 성분(`&`)만 다루고 함수 성분은 `_`(무관).
> - `fn f(...) {return y;}` ⟹ ⟦f⟧의 함수 성분 = fn(매개변수들)→⟦y⟧
> - `x = y(...)` ⟹ ⟦y⟧의 함수 성분과 인자·반환을 단일화

### 해설

**개념 설명 — 항에 함수 성분 추가**

Steensgaard도 함수 포인터를 다룹니다: 각 셀의 항을 **쌍 `(포인터 성분, 함수 성분)`**으로 확장합니다. 포인터 성분은 기존대로 `&⟦y⟧`, 함수 성분은 `fn(인자들)→반환`. 함수 정의와 호출이 이 함수 성분을 **단일화**합니다 — 타입 추론에서 함수 타입을 단일화하던 것과 똑같습니다(강의 3~4). 호출 `x=y(...)`는 y의 함수 성분의 인자·반환을 실인자·결과와 단일화 → CFA와 pt가 동시에 거의 선형으로 풀림.

**슬라이드 연결**: 예제가 슬라이드 25.

---

## 슬라이드 25: Steensgaard-Style Analysis — Example

### 원문 내용
> ```
> fn f(a) {}
> fn g(b) {}
> x = f; y = g;
> if ... { z = &x; } else { z = &y; }
> w = *z;
> u = &v;
> w(u);
> ```
> Solutions:
> - ⟦f⟧ = ⟦g⟧ = ⟦x⟧ = ⟦y⟧ = ⟦w⟧ = (_, fn(⟦a⟧))
> - ⟦z⟧ = (&⟦x⟧, _)
> - ⟦u⟧ = ⟦a⟧ = ⟦b⟧ = (&⟦v⟧, _)
> - func(f) = func(g) = func(x) = func(y) = func(w) = {f, g}
> - pt(z) = {x, y}
> - pt(u) = pt(a) = pt(b) = {v}

### 번역
> 슬라이드 23과 같은 코드를 Steensgaard로. x=f, y=g가 등식이라 f,g,x,y,w가 모두 단일화(함수 성분 공유), a,b도 합쳐짐. 호출 대상 func(w)={f,g}, pt(u)=pt(a)=pt(b)={v}.

### 해설

**개념 설명 — 함수도 합쳐 버린다**

Steensgaard는 `x=f`,`y=g`를 등식으로 보아 **f와 g까지 한 덩어리로** 단일화합니다(func(f)=func(g)={f,g}). 그래서 어떤 함수 변수든 호출하면 f,g 둘 다 가능하다고 봅니다 — Andersen(슬라이드 23)보다 거칠지만 거의 선형. pt 결과(pt(u)=pt(a)=pt(b)={v})는 우연히 Andersen과 같지만, 함수 셀 합침에서 정밀도 손실이 일어났습니다. 다시 한 번 **정밀(Andersen) vs 속도(Steensgaard)** 대비.

**슬라이드 연결**: 전체 요약이 슬라이드 26.

---

## 슬라이드 26: Summary

### 원문 내용
> - Allocation-site abstraction lets us reason about unbounded memory with a finite set of abstract cells
> - Andersen-style analysis encodes pointer statements as subset constraints over ⟦c⟧ and solves them with the cubic algorithm
> - Steensgaard-style analysis treats assignments bidirectionally, uses term equalities with the & constructor, and solves them via unification — faster but less precise
> - With function pointers, control flow and points-to information are mutually dependent and must be computed together

### 번역
> - **할당 지점 추상화**로 무한 메모리를 유한한 추상 셀 집합으로 다룬다
> - **Andersen**: 포인터 문장을 ⟦c⟧에 대한 **포함(subset) 제약**으로 인코딩, **cubic 알고리즘**으로 풂
> - **Steensgaard**: 대입을 양방향으로, **& 생성자의 항 등식**으로 보고 **단일화**로 풂 — 더 빠르지만 덜 정밀
> - 함수 포인터가 있으면 제어 흐름과 points-to 정보가 **상호 의존**이라 **함께 계산**해야 한다

### 해설

**전체 정리 — 강의 14의 한 장 요약**

1. **추상화**: 무한 메모리 → 유한 추상 셀(변수 셀 + 할당 셀). 분석 목표는 `pt: Var→𝒫(Cell)`.
2. **Andersen (포함 기반)**: `x=y`→`⟦y⟧⊆⟦x⟧`, 역참조는 조건부. 세 형태 제약 → **강의 11 cubic**, O(n³), 정밀.
3. **Steensgaard (단일화 기반)**: `x=y`→`⟦x⟧=⟦y⟧`(양방향), `&` 항 단일화, 거의 선형, 거침. **Steensgaard pt ⊇ Andersen pt**.
4. **함수 포인터**: CFA(강의 11)와 pt가 순환 의존 → 통합 제약으로 동시 해결. Andersen은 강의 11 규칙을 셀로 일반화해 흡수.

**다른 강의와의 연결 (파일 간 연결성)**

- ← **강의 3~4 (타입·단일화)**: Steensgaard의 단일화는 타입 추론의 단일화와 동일. 함수 항 단일화도 함수 타입 단일화와 같음.
- ← **강의 5~6 (격자·추상화)**: 할당 지점 추상화 = 무한→유한 추상, pt는 𝒫(Cell) 격자 위 과근사.
- ← **강의 7~8 (데이터플로우)**: 이 강의는 **흐름 무감각**(문장 순서 무시) — 흐름 감각 데이터플로우와 대조되는 축.
- ← **강의 10 (절차간·정밀도-비용)**: Andersen vs Steensgaard = 정밀-비용 트레이드오프. 절차간 함수 인자·반환 전파.
- ← **강의 11 (제어 흐름·cubic)**: Andersen = 강의 11 cubic의 직접 응용(토큰=셀). 함수 포인터에서 CFA·pt 통합.
- → **강의 15 (포인터 2)**: 흐름 감각·문맥 민감 등 더 정밀한 포인터 분석으로 확장 예상.

**가장 큰 교훈**: points-to 분석은 (1) 정밀하지만 느린 **포함 기반(Andersen, cubic)**과 (2) 거칠지만 빠른 **단일화 기반(Steensgaard, 거의 선형)**의 두 갈래로 나뉘며, 함수 포인터가 있으면 제어 흐름 분석과 떼려야 뗄 수 없습니다. 강의 11의 cubic 알고리즘이 또다시 핵심 도구로 등장합니다.

---

## 마치며

강의 14는 "포인터가 무엇을 가리키나"라는 근본 질문에 답하는 두 고전 알고리즘을 대비시킵니다. **Andersen(포함·cubic·정밀)과 Steensgaard(단일화·선형·거침)**의 비교는 정적 분석 전체를 관통하는 정밀도-비용 트레이드오프의 교과서적 예입니다. 또한 함수 포인터를 통해 강의 11(제어 흐름)과 이 강의(points-to)가 하나로 묶이는 것을 보았습니다. 시험에서는 (a) 주어진 코드에 두 분석을 각각 적용해 pt 구하고 차이 설명(슬라이드 7~10, 15~20), (b) 왜 Steensgaard가 덜 정밀하지만 빠른지 서술(슬라이드 11,16), (c) 함수 포인터에서 CFA와 pt의 상호 의존 설명(슬라이드 21)이 단골입니다.
