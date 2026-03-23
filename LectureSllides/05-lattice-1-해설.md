# Lattice Theory (1) - 상세 해설

CSE552 Program Analysis — Lecture 5
강사: Jaemin Hong

---

## 슬라이드 1: 제목 슬라이드

### 원문 내용
> Lattice Theory (1)
> CSE552 Program Analysis — Lecture 5
> Jaemin Hong

### 해설

이 강의는 프로그램 분석의 수학적 기초가 되는 **격자 이론(Lattice Theory)**을 소개합니다. 프로그램 분석에서 추상 영역(abstract domain)을 설계하고 그 속성을 보장하기 위해서는 격자 이론의 개념이 필수적입니다.

---

## 슬라이드 2: Sign Analysis

### 원문 내용
> **Sign Analysis**
>
> - An analysis that finds out the possible integer values of variables and expressions
> - In concrete executions, values can be arbitrary integers
> - Our analysis considers an abstraction of the integer values by grouping them into the three categories, or abstract values: negative (−), zero (0), and positive (+)
> - e.g., if a sound analysis concludes that the final value of a variable is +, then the value must be a positive integer in any execution

### 해설

**개념 설명**

Sign Analysis(부호 분석)는 프로그램 내 변수와 식의 값이 음수, 0, 또는 양수 중 어디에 속하는지를 판단하는 정적 분석입니다. 이는 무한한 정수 집합을 3개의 범주로 추상화(abstraction)하는 과정입니다.

**배경 지식**

- **추상화(Abstraction)**: 프로그램의 실제 동작을 정확하게 추적하지 못하더라도, 분석 목표에 필요한 정보는 유지하면서 계산 복잡도를 낮추는 기법입니다.
- **정적 분석(Static Analysis)**: 프로그램을 실행하지 않고 코드를 분석하여 가능한 동작을 예측합니다.
- **건전성(Soundness)**: 분석 결과가 모든 가능한 실행에서 참이면 분석이 건전합니다.

**전체적인 맥락**

Sign Analysis는 가장 간단한 추상 영역의 예시로, 이후 배울 격자 이론의 개념들을 구체적으로 설명하는 데 사용됩니다.

---

## 슬라이드 3: Top (⊤)

### 원문 내용
> **Top**
>
> - The analysis may not know the sign of some expression
>   - The value is positive in some execution, and it is not in other executions
>   - The value is positive in every execution but the analysis fails to recognize this (which is unavoidable due to undecidability)
> - We add a special abstract value ⊤ representing "don't know"

### 해설

**개념 설명**

분석기는 항상 정확한 정보를 얻을 수 없습니다. 값의 부호를 확실히 알 수 없을 때를 표현하기 위해 **Top(⊤)** 값을 도입합니다. 이는 "+", "−", "0" 중 어느 것인지 알 수 없다는 의미입니다.

**배경 지식**

- **결정 불가능성(Undecidability)**: 프로그램의 성질 중 일부는 원리적으로 정적 분석으로 정확히 판단할 수 없습니다 (Rice의 정리).
- **보수적 근사**: 분석이 확실하지 않을 때는 "모를 수도 있다"는 정보를 반환하는 것이 건전합니다.

**추가 설명** (보충)

예를 들어, 조건문의 동적 조건에 따라 같은 변수가 양수일 수도, 음수일 수도 있다면, 분석기는 이를 ⊤으로 표현합니다.

---

## 슬라이드 4: Bottom (⊥)

### 원문 내용
> **Bottom**
>
> - It is beneficial to also have an abstract value ⊥ for expressions
>   - whose values are not numbers (e.g., pointers), or
>   - that have no value in any execution because they are unreachable

### 해설

**개념 설명**

Top과 반대로, **Bottom(⊥)** 값은 두 가지 경우를 나타냅니다:
1. 값이 정수가 아닌 경우 (예: 포인터)
2. 그 식이 실행 가능한 경로에서 절대 도달하지 않는 경우 (도달 불가능한 코드)

**배경 지식**

- **타입 안전성**: 부호 분석은 정수 값만을 대상으로 하므로, 다른 타입의 식은 ⊥로 표현합니다.
- **도달 불가능 코드**: 분석 과정에서 어떤 코드 지점이 도달 불가능함을 발견할 수 있습니다.

**추가 설명** (보충)

⊥는 "이 정보는 sign analysis의 대상이 아니다" 또는 "이 지점은 실행될 수 없다"를 의미합니다.

---

## 슬라이드 5: Sign Analysis Example

### 원문 내용
> **Sign Analysis Example**
>
> ```
> a = 42;
> b = 87;
> if input() {
>   c = a + b;
> } else {
>   c = a - b;
> }
> ```
>
> A sound analysis may conclude that, at return,
> - a is +
> - b is +
> - c is ⊤

### 해설

**개념 설명**

구체적인 예시를 통해 Sign Analysis를 설명합니다. 입력값에 따라 `c`의 값이 달라질 수 있으므로 (`a + b`는 양수, `a - b`는 음수일 수 있음), 분석기는 `c`의 부호를 ⊤으로 결론짓습니다.

**배경 지식**

- **제어 흐름 병합**: if 문의 두 가지 분기가 모두 가능하므로, 병합된 상태에서는 두 분기의 결과를 모두 고려해야 합니다.

**전체적인 맥락**

이 예시는 추상 영역에서 **join 연산**(후에 배울 개념)이 왜 필요한지를 설명합니다.

---

## 슬라이드 6: Abstract Domains

### 원문 내용
> **Abstract Domains**
>
> - For this analysis, we have an abstract domain consisting of the five abstract values: {⊥, −, 0, +, ⊤}
> - We can organize as follows with the least precise information at the top and the most precise information at the bottom:
>
> ```
>        ⊤
>       /|\
>      / | \
>     −  0  +
>      \ | /
>       \|/
>        ⊥
> ```

### 해설

**개념 설명**

Sign Analysis의 **추상 영역(Abstract Domain)**은 5개의 값으로 구성됩니다:
- **⊤**: 정보 없음
- **−, 0, +**: 구체적 정보
- **⊥**: 해당 없음

이들을 다이아몬드 형태의 구조로 배치하는데, 이는 후에 배울 **부분 순서(partial order)**입니다.

**배경 지식**

- **정확성(Precision)**: 아래쪽이 더 정확하고, 위쪽이 더 일반적입니다.
- **격자 구조(Lattice Structure)**: 이 다이아몬드 구조는 격자의 한 예입니다.

**수식/기호 설명**

- 위에서 아래로의 화살표는 "더 정확하다"는 의미의 순서 관계를 나타냅니다.

---

## 슬라이드 7: Questions — Motivation for Lattice Theory

### 원문 내용
> **Questions — Motivation for Lattice Theory**
>
> - How can we ensure termination and soundness of our analysis?
> - What is required for the abstract domain?
>
> - We need a mathematical foundation: the lattice theory
> - The connection between lattices and program analysis was established in the seminal work by Kildall, Kam, and Ullman¹²

### 해설

**개념 설명**

프로그램 분석의 건전성과 종료성을 수학적으로 보장하기 위해, 추상 영역의 구조에 대한 엄밀한 정의가 필요합니다. 이것이 **격자 이론(Lattice Theory)**을 학습해야 하는 이유입니다.

**배경 지식**

- **종료성(Termination)**: 분석이 유한한 시간 내에 완료되어야 합니다.
- **건전성(Soundness)**: 분석 결과가 모든 가능한 실행에서 참이어야 합니다.

**전체적인 맥락**

Kildall (1973), Kam & Ullman (1977)의 논문들이 데이터 흐름 분석(data flow analysis)과 격자 이론의 관계를 확립했습니다. 이는 프로그램 분석 이론의 기초가 되었습니다.

---

## 슬라이드 8: Partially Ordered Sets — Definition

### 원문 내용
> **Partially Ordered Sets — Definition**
>
> **Definition** (Partially ordered set). A partially ordered set (poset) is a set S equipped with a binary relation ⊑ where the following conditions are satisfied:
> - Reflexivity: ∀x ∈ S. x ⊑ x
> - Transitivity: ∀x, y, z ∈ S. x ⊑ y ∧ y ⊑ z ⟹ x ⊑ z
> - Anti-symmetry: ∀x, y ∈ S. x ⊑ y ∧ y ⊑ x ⟹ x = y
>
> c.f., a total order additionally requires totality
> (∀x, y ∈ S. x ⊑ y ∨ y ⊑ x)

### 해설

**개념 설명**

**부분 순서 집합(Partially Ordered Set, poset)**은 원소들 사이의 순서 관계를 정의한 집합입니다. 세 가지 공리를 만족해야 합니다:

1. **반사성(Reflexivity)**: 모든 원소는 자기 자신과 비교할 수 있습니다.
2. **추이성(Transitivity)**: 순서 관계가 연쇄적으로 전이됩니다.
3. **반대칭성(Anti-symmetry)**: 양방향으로 비교 가능하면 같은 원소입니다.

**배경 지식** (학부 2학년 수준)

- **동치관계(Equivalence Relation)**: 반사, 대칭, 추이성을 만족합니다.
- **순서관계(Order Relation)**: 반사, 추이, 반대칭을 만족합니다.
- **전체 순서(Total Order)**: 추가로 모든 두 원소가 비교 가능합니다 (예: 자연수의 ≤).

**전체적인 맥락**

부분 순서는 격자의 기반이 되는 개념입니다. Sign Analysis의 다이아몬드 구조도 부분 순서입니다.

---

## 슬라이드 9: Partially Ordered Sets — Intuition and Notation

### 원문 내용
> **Partially Ordered Sets — Intuition and Notation**
>
> - From the analysis perspective, when x ⊑ y, we say that "y is a safe approximation of x," or that "x is at least as precise as y"
> - We sometimes write y ⊒ x instead of x ⊑ y

### 해설

**개념 설명**

프로그램 분석 관점에서 `x ⊑ y`는 다음을 의미합니다:
- **x가 y보다 정확함** (x is more precise than y)
- **y가 x의 안전한 근사임** (y safely over-approximates x)

**배경 지식**

- **안전한 근사(Safe Approximation)**: 실제보다 더 일반적이어서 참일 가능성이 높습니다. 프로그램 분석에서는 보수적이어야 하므로 안전한 근사를 사용합니다.

**추가 설명** (보충)

Sign Analysis 예시에서:
- `-` ⊑ `⊤` (음수는 ⊤보다 정확함)
- `0` ⊑ `⊤` (0은 ⊤보다 정확함)
- `+` ⊑ `⊤` (양수는 ⊤보다 정확함)

---

## 슬라이드 10: Partially Ordered Sets — Examples

### 원문 내용
> **Partially Ordered Sets — Examples**
>
> Examples:
> - (ℕ, ≤)
> - (𝒫(S), ⊆)
> - ({⊥, −, 0, +, ⊤}, ⊑) where
>   - ⊥ ⊑ −, 0, +, ⊤
>   - − ⊑ −, ⊤
>   - 0 ⊑ 0, ⊤
>   - + ⊑ +, ⊤
>   - ⊤ ⊑ ⊤

### 해설

**개념 설명**

세 가지 구체적인 부분 순서 집합의 예시:

1. **(ℕ, ≤)**: 자연수와 일반적인 수의 크기 비교. 이는 전체 순서입니다.

2. **(𝒫(S), ⊆)**: 집합 S의 멱집합(power set)과 부분집합 관계. 이는 부분 순서이지만 전체 순서는 아닙니다 (비교 불가능한 집합들이 있음).

3. **Sign Analysis의 poset**: ⊥는 모든 값의 아래에, ⊤는 모든 값의 위에 위치합니다.

**배경 지식** (학부 2학년 수준)

- **멱집합(Power Set)**: 주어진 집합의 모든 부분집합들의 집합입니다.
- **부분집합(Subset)**: A ⊆ B는 A의 모든 원소가 B에도 속함을 의미합니다.

---

## 슬라이드 11: Hasse Diagrams

### 원문 내용
> **Hasse Diagrams**
>
> - A partial order can be illustrated by a Hasse diagram in which the elements are nodes and the order relation is the transitive closure of edges leading from lower to higher nodes

### 해설

**개념 설명**

**하세 다이어그램(Hasse Diagram)**은 부분 순서를 시각화하는 방법입니다. 다음 규칙을 따릅니다:

- 원소는 노드로 표현
- 낮은 순서의 원소는 아래에, 높은 순서의 원소는 위에 배치
- 직접 연결된 간선(edge)만 그리고, 추이성에 의해 자명한 간선은 생략
- 순서 관계는 경로의 추이적 폐포(transitive closure)

**배경 지식** (학부 2학년 수준)

- **추이적 폐포(Transitive Closure)**: 모든 간접 경로를 암묵적으로 포함하는 것입니다.

**전체적인 맥락**

하세 다이어그램은 복잡한 부분 순서를 이해하기 쉽게 표현하는 도구입니다. 강의의 여러 예시에서 사용됩니다.

---

## 슬라이드 12: Bounds

### 원문 내용
> **Bounds**
>
> **Definition** (Upper bound and lower bound). For X ⊆ S and y ∈ S,
> - y is an upper bound for X, written X ⊑ y, if ∀x ∈ X. x ⊑ y
> - y is a lower bound for X, written y ⊑ X, if ∀x ∈ X. y ⊑ x
>
> Example: Given (ℕ, ≤), for {5, 7, 10}, 10 and 100 are some of upper bounds, and 1 and 5 are some of lower bounds.

### 해설

**개념 설명**

부분 순서 집합에서 **상한(upper bound)**과 **하한(lower bound)**의 개념을 정의합니다:

- **상한(Upper Bound)**: 집합 X의 모든 원소보다 크거나 같은 원소
- **하한(Lower Bound)**: 집합 X의 모든 원소보다 작거나 같은 원소

한 집합은 여러 개의 상한과 하한을 가질 수 있습니다.

**예시**

(ℕ, ≤)에서 {5, 7, 10}을 생각해봅시다:
- 상한: 10, 100, 1000, ... (모두 10 이상)
- 하한: 1, 2, 3, 4, 5 (모두 5 이하)

**전체적인 맥락**

상한과 하한 중에서 가장 적절한 것들을 찾는 것이 프로그램 분석에서 매우 중요합니다.

---

## 슬라이드 13: Least Upper Bound and Greatest Lower Bound

### 원문 내용
> **Least Upper Bound and Greatest Lower Bound**
>
> **Definition** (Least upper bound and greatest lower bound).
> - A least upper bound (lub), written ⊔X, satisfies
>   - X ⊑ ⊔X ∧ ∀y. X ⊑ y → ⊔X ⊑ y
> - A greatest lower bound (glb), written ⊓X, satisfies
>   - ⊓X ⊑ X ∧ ∀y. y ⊑ X → y ⊑ ⊓X

### 해설

**개념 설명**

상한과 하한 중에서 **최적의** 것을 선택하는 개념입니다:

- **최소상한(Least Upper Bound, lub, ⊔X)**:
  - X의 모든 상한 중에서 가장 작은 것
  - X의 모든 원소보다 크고, 다른 상한보다는 작음

- **최대하한(Greatest Lower Bound, glb, ⊓X)**:
  - X의 모든 하한 중에서 가장 큰 것
  - X의 모든 원소보다 작고, 다른 하한보다는 큼

**수식 설명**

⊔X의 정의 분석:
- `X ⊑ ⊔X`: ⊔X는 X의 상한이다
- `∀y. X ⊑ y → ⊔X ⊑ y`: 다른 모든 상한 y에 대해, ⊔X는 y보다 작거나 같다

**배경 지식**

- **유일성(Uniqueness)**: 최소상한과 최대하한이 존재하면 유일합니다 (반대칭성에 의해).

**전체적인 맥락**

lub와 glb는 프로그램 분석에서 **join 연산**과 **meet 연산**으로 사용되는 핵심 개념입니다.

---

## 슬라이드 14: Lub and Glb — Notations

### 원문 내용
> **Lub and Glb — Notations**
>
> Notations:
> - x ⊔ y := ⊔{x, y} (join of x and y)
> - x ⊓ y := ⊓{x, y} (meet of x and y)
> - ⊔ₐ∈ₐ f(a) = ⊔{f(a) | a ∈ A}
> - ⊓ₐ∈ₐ f(a) = ⊓{f(a) | a ∈ A}

### 해설

**개념 설명**

두 원소와 여러 원소의 lub, glb를 편리하게 표기하기 위한 기호들입니다:

- **⊔** (join): 두 원소의 최소상한. 직관적으로 "둘 다 만족하는 가장 작은 상한"
- **⊓** (meet): 두 원소의 최대하한. 직관적으로 "둘 다보다 작은 가장 큰 하한"

**수식/기호 설명**

- `x ⊔ y`: 집합 {x, y}의 최소상한
- `⊔ₐ∈ₐ f(a)`: 함수 f를 집합 A의 모든 원소에 적용한 결과들의 최소상한

**추가 설명** (보충)

Sign Analysis 예시:
- `− ⊔ + = ⊤` (음수와 양수의 join은 "부호 미정")
- `− ⊓ + = ⊥` (음수와 양수의 meet는 "교집합이 없으므로 ⊥")

---

## 슬라이드 15: Lub and Glb — Properties

### 원문 내용
> **Lub and Glb — Properties**
>
> Important properties:
> - If ⊔X exists, then it is unique
> - If ⊓X exists, then it is unique
> - If x ⊔ y exists, then x ⊑ y ⟺ x ⊔ y = y
> - If x ⊓ y exists, then x ⊑ y ⟺ x ⊓ y = x

### 해설

**개념 설명**

lub와 glb의 중요한 성질들입니다:

1. **유일성**: 최소상한과 최대하한이 존재하면 반드시 유일합니다.

2. **순서 관계와의 동치성**:
   - `x ⊑ y ⟺ x ⊔ y = y`: x가 y 이상이면, 둘의 join은 y다
   - `x ⊑ y ⟺ x ⊓ y = x`: x가 y 이상이면, 둘의 meet는 x다

**배경 지식**

이들 성질은 부분 순서의 정의(특히 반대칭성)로부터 자연스럽게 따라옵니다.

**전체적인 맥락**

이 성질들은 나중에 배울 격자의 정의와 성질에서 핵심적으로 사용됩니다.

---

## 슬라이드 16: Lub and Glb — Examples and Role in Analysis

### 원문 내용
> **Lub and Glb — Examples and Role in Analysis**
>
> Examples:
> - In (ℕ, ≤), ⊔ = max and ⊓ = min
> - In (𝒫(S), ⊆), ⊔ = ∪ and ⊓ = ∩
>
> - The lub/join operation plays an important role in program analysis
> - We use lub when combining abstract information from multiple sources
>   - e.g., when control flow merges after the branches of if statements

### 해설

**개념 설명**

join과 meet 연산의 구체적인 예시:

1. **(ℕ, ≤)**:
   - `⊔ = max` (최댓값)
   - `⊓ = min` (최솟값)

2. **(𝒫(S), ⊆)**:
   - `⊔ = ∪` (합집합)
   - `⊓ = ∩` (교집합)

**전체적인 맥락**

프로그램 분석에서 **control flow가 병합**될 때 join 연산을 사용합니다:
- if-then-else 문의 두 분기 후에 변수의 추상 정보를 합칩니다
- 루프의 종료 후 여러 경로의 정보를 합칩니다

**추가 설명** (보충)

Sign Analysis에서 `− ⊔ 0 ⊔ + = ⊤`는 "음수, 0, 양수 모두 가능하면 ⊤"을 의미합니다.

---

## 슬라이드 17: Lattices — Definition

### 원문 내용
> **Lattices — Definition**
>
> **Definition** (Lattice).
> - A lattice is a partial order (S, ⊑) in which x ⊔ y and x ⊓ y exist for all x, y ∈ S
> - A complete lattice is a partial order (S, ⊑) in which ⊔X and ⊓X exist for all X ⊆ S

### 해설

**개념 설명**

드디어 **격자(Lattice)**를 정의합니다:

- **격자(Lattice)**: 모든 두 원소 쌍이 join과 meet을 가지는 부분 순서 집합
- **완전 격자(Complete Lattice)**: 모든 부분집합(임의의 크기)이 join과 meet을 가지는 부분 순서 집합

**배경 지식**

- 격자는 부분 순서 집합보다 더 강한 제약입니다.
- 완전 격자는 격자보다 더 강한 제약입니다.

**수식 설명**

- **격자의 조건**: ∀x, y ∈ S, ∃(x ⊔ y) ∧ ∃(x ⊓ y)
- **완전 격자의 조건**: ∀X ⊆ S, ∃(⊔X) ∧ ∃(⊓X)

**전체적인 맥락**

프로그램 분석에서 사용하는 추상 영역들은 대부분 완전 격자 구조를 가집니다.

---

## 슬라이드 18: Lattices — Properties

### 원문 내용
> **Lattices — Properties**
>
> Important properties:
> - Every complete lattice is a lattice
>   - What is a lattice that is not a complete lattice?
> - A nonempty finite lattice is complete
> - Where S is a poset, every subset of S has an lub ⟺ every subset of S has a glb
> - Most lattices we encounter in program analysis are complete lattices

### 해설

**개념 설명**

격자의 중요한 성질들:

1. **완전 격자 ⊃ 격자**: 모든 완전 격자는 격자이지만, 역은 성립하지 않습니다.

2. **유한 격자는 완전**: 공집합이 아닌 유한 격자는 항상 완전 격자입니다.

3. **lub와 glb의 쌍대성**: "모든 부분집합이 lub를 가짐" ⟺ "모든 부분집합이 glb를 가짐"

4. **실용성**: 프로그램 분석에서는 거의 항상 완전 격자를 다룹니다.

**배경 지식**

- **쌍대성(Duality)**: 부분 순서의 역방향도 부분 순서이므로, lub와 glb의 역할이 대칭입니다.

**추가 설명** (보충)

"격자이지만 완전 격자가 아닌" 예시: 유리수의 일반적인 순서 (⟨ℚ, ≤⟩). 모든 유한 부분집합은 lub와 glb를 가지지만, 무한 부분집합 중 일부는 상한이 존재하지 않습니다.

---

## 슬라이드 19: Lattice Example 1

### 원문 내용
> **Lattice Example 1**
>
> [다이아몬드 형태의 격자 다이어그램]
> Lattice

### 해설

**개념 설명**

5개의 원소로 이루어진 다이아몬드 형태의 격자입니다. 이는 Sign Analysis의 추상 영역과 동일한 구조입니다:
- 맨 위: ⊤
- 중간: −, 0, +
- 맨 아래: ⊥

**전체적인 맥락**

이 구조는 프로그램 분석에서 가장 흔하게 사용되는 격자 중 하나입니다. 모든 쌍이 join과 meet을 가지므로 격자이며, 유한이므로 완전 격자입니다.

---

## 슬라이드 20: Lattice Example 2

### 원문 내용
> **Lattice Example 2**
>
> [세 개의 독립적인 노드와 한 개의 아래 노드로 구성된 구조]
> Not a lattice

### 해설

**개념 설명**

이 구조는 격자가 **아닙니다**. 이유는 위의 세 노드 중 서로 다른 두 개 (예: 왼쪽과 중간)가:
- join을 가지지 않음 (두 노드보다 큰 노드가 없음)
- meet을 가짐 (아래 노드만 가능)

따라서 모든 쌍이 join과 meet을 동시에 가지지 않으므로 격자가 아닙니다.

**배경 지식**

격자 정의의 필요충분조건: 모든 두 원소가 join과 meet을 모두 가져야 합니다.

---

## 슬라이드 21: Lattice Example 3

### 원문 내용
> **Lattice Example 3**
>
> [위에서 아래로: 최상단 노드, 중간층 4개 노드(다이아몬드), 최하단 노드]
> Lattice

### 해설

**개념 설명**

이 구조는 격자입니다. 두 개의 다이아몬드가 위아래로 연결된 형태로:
- 모든 두 원소가 join을 가짐
- 모든 두 원소가 meet을 가짐

**배경 지식**

다이아몬드 형태의 부분은 격자의 기본 단위이며, 이들을 위아래로 연결해도 여전히 격자입니다.

**추가 설명** (보충)

이것은 높이 3인 완전 격자의 예입니다.

---

## 슬라이드 22: Lattice Example 4

### 원문 내용
> **Lattice Example 4**
>
> [여러 경로로 교차하는 구조]
> Not a lattice

### 해설

**개념 설명**

이 구조는 격자가 **아닙니다**. 왼쪽 중간과 오른쪽 중간 노드가:
- 공통의 상한이 여러 개 (유일하지 않은 join)
- 공통의 하한도 여러 개 (유일하지 않은 meet)

따라서 join이나 meet이 **유일하지 않으므로** 격자가 아닙니다.

**배경 지식**

격자에서 join과 meet은 유일해야 합니다. 부분 순서 관계에서 추이성에 의해 가장 작은 상한과 가장 큰 하한이 유일하게 결정되어야 합니다.

---

## 슬라이드 23: Lattice Example 5

### 원문 내용
> **Lattice Example 5**
>
> [여러 층으로 구성된 복잡한 구조]
> Lattice

### 해설

**개념 설명**

이 구조는 격자입니다. 더 복잡한 형태이지만, 모든 두 원소 쌍이 유일한 join과 meet을 가집니다.

**전체적인 맥락**

이 예시는 격자가 복잡한 구조도 가능함을 보여줍니다. 중요한 것은 모든 두 원소가 유일한 join과 meet을 가지는지 여부입니다.

---

## 슬라이드 24: Top and Bottom

### 원문 내용
> **Top and Bottom**
>
> - ⊤ = ⊔S
> - ⊥ = ⊓S
> - Height of S: the length of the longest path from ⊥ to ⊤
>   - e.g., the height of the sign lattice is 2
> - Every complete lattice has ⊤ and ⊥

### 해설

**개념 설명**

완전 격자에서 특별한 원소들:

- **Top (⊤)**: 전체 집합 S의 최소상한. 가장 "일반적인" 원소
- **Bottom (⊥)**: 전체 집합 S의 최대하한. 가장 "구체적인" 원소
- **높이(Height)**: ⊥에서 ⊤까지의 최장 경로의 길이

**배경 지식**

완전 격자의 정의에서 모든 부분집합이 join과 meet을 가지므로, 특히 전체 집합도 이를 만족합니다.

**수식 설명**

Sign Analysis의 경우:
- ⊤ = ⊔{⊥, −, 0, +, ⊤} = ⊤
- ⊥ = ⊓{⊥, −, 0, +, ⊤} = ⊥
- Height = 2 (⊥ → {−,0,+} → ⊤의 최장 경로)

---

## 슬라이드 25: Constructing Lattices — Power Sets

### 원문 내용
> **Constructing Lattices — Power Sets**
>
> - (𝒫(A), ⊆) is a complete lattice, called the power set lattice:
>   - ⊔ = ∪, ⊓ = ∩, ⊥ = ∅, ⊤ = A
> - (𝒫(A), ⊇) is also a complete lattice, called the reverse power set lattice:
>   - ⊔ = ∩, ⊓ = ∪, ⊥ = A, ⊤ = ∅

### 해설

**개념 설명**

멱집합을 사용하여 완전 격자를 구성할 수 있습니다:

1. **(𝒫(A), ⊆)**: 부분집합 관계로 정렬
   - join = 합집합 (더 일반적인 정보)
   - meet = 교집합 (더 구체적인 정보)
   - ⊤ = A (모든 원소 포함, 가장 일반적)
   - ⊥ = ∅ (원소 없음, 가장 구체적)

2. **(𝒫(A), ⊇)**: 부분집합 관계를 역순으로 정렬
   - join과 meet이 반대로 됨 (대칭성)

**배경 지식**

- **멱집합**: 집합 A의 모든 부분집합들의 집합
- **𝒫(A)의 크기**: |A| = n이면 |𝒫(A)| = 2ⁿ

**전체적인 맥락**

멱집합 격자는 프로그램 분석에서 매우 자주 사용됩니다. 예를 들어, 도달 가능한 변수들의 집합을 나타냅니다.

---

## 슬라이드 26: Constructing Lattices — Flat

### 원문 내용
> **Constructing Lattices — Flat**
>
> - For A = {a₁, a₂, ...}, flat(A) is a complete lattice with height 2:
>
> ```
>        ⊤
>       /|\...
>      / | \
>    a₁  a₂  ...
>      \ | /
>       \|/
>        ⊥
> ```
>
> Example:
> - Sign = {−, 0, +} can be expressed as flat({−, 0, +})

### 해설

**개념 설명**

**Flat Lattice**는 주어진 원소들을 가지고 만드는 가장 단순한 완전 격자입니다:

- 원소들 {a₁, a₂, ...} 사이에는 순서 관계가 없음
- 모든 원소를 위에 놓은 ⊤과 아래에 놓은 ⊥만 존재
- 높이는 항상 2

**배경 지식**

- flat 격자는 원소들 사이의 자연스러운 순서가 없을 때 사용합니다.

**전체적인 맥락**

Sign Analysis의 추상 영역은 실제로 flat({−, 0, +})에 ⊤과 ⊥를 추가한 것입니다.

---

## 슬라이드 27: Constructing Lattices — Products (Definition)

### 원문 내용
> **Constructing Lattices — Products (Definition)**
>
> - If L₁, L₂, ..., Lₙ are complete lattices, then so is the product where the order ⊑ is defined componentwise:
>   - L₁ × L₂ × ... × Lₙ = {(x₁, x₂, ..., xₙ) | xᵢ ∈ Lᵢ}
>   - (x₁, ..., xₙ) ⊑ (x₁', ..., xₙ') ⟺ ∀i. xᵢ ⊑ xᵢ'
> - Product of n identical lattices can be written concisely as Lⁿ = L × L × ... × L
> - Lubs and glbs can be computed componentwise:
>   - ⊔ᵢ∈ᵢ (xᵢ₁, ..., xᵢₙ) = (⊔ᵢ∈ᵢ xᵢ₁, ..., ⊔ᵢ∈ᵢ xᵢₙ)
>   - ⊓ᵢ∈ᵢ (xᵢ₁, ..., xᵢₙ) = (⊓ᵢ∈ᵢ xᵢ₁, ..., ⊓ᵢ∈ᵢ xᵢₙ)

### 해설

**개념 설명**

여러 격자를 **곱(product)**하여 새로운 격자를 만들 수 있습니다:

- 각 성분(component)을 독립적으로 정렬
- 두 튜플을 비교할 때는 모든 위치에서 만족해야 함
- join과 meet은 각 위치마다 독립적으로 계산

**수식 설명**

- **성분별 순서(componentwise order)**: 모든 위치 i에서 xᵢ ⊑ xᵢ'이어야 함
- **성분별 join/meet**: 각 위치의 join/meet을 계산한 후 다시 묶음

**배경 지식**

- **곱 격자의 높이**: height(L₁ × ... × Lₙ) = height(L₁) + ... + height(Lₙ)

**전체적인 맥락**

프로그램 분석에서 여러 변수를 동시에 추적할 때 곱 격자를 사용합니다.

---

## 슬라이드 28: Constructing Lattices — Products (Height and Examples)

### 원문 내용
> **Constructing Lattices — Products (Height and Examples)**
>
> - height(L₁ × ⋯ × Lₙ) = height(L₁) + ⋯ + height(Lₙ)
>
> Examples:
> - Sign × Flat(ℕ)
>   - (+, 1)
>   - (−, ⊤)
> - Signᴾ⁽ᴸᵒᶜᵒᶜ⁾
>   - (+, 0, ⊤)
>   - (⊤, ⊤, −)

### 해설

**개념 설명**

곱 격자의 높이와 구체적인 예시:

1. **Sign × Flat(ℕ)**:
   - 각 변수에 대해 부호와 자연수 값을 추적
   - (+, 1): 양수 값 1
   - (−, ⊤): 음수이지만 구체적 값 불명

2. **Signᴾ⁽ᴸᵒᶜᵒᶜ⁾**:
   - 각 프로그램 위치(location)에서의 부호 정보
   - 3개 위치에서의 부호를 튜플로 표현

**배경 지식**

- **프로그램 위치(Program Location)**: 제어 흐름 그래프의 노드

**전체적인 맥락**

이러한 곱 격자들은 실제 프로그램 분석에서 사용되는 추상 영역들입니다.

---

## 슬라이드 29: Constructing Lattices — Maps (Definition)

### 원문 내용
> **Constructing Lattices — Maps (Definition)**
>
> - If A is a set and L is a complete lattice, then we obtain a complete lattice called a map lattice A → L consisting of the set of functions from A to L, ordered pointwise:
>   - A → L = {[a₁ ↦ x₁, a₂ ↦ x₂, ...] | [a₁, a₂, ...] = A ∧ xᵢ ∈ L}
>   - f ⊑ g ⟺ ∀a ∈ A. f(a) ⊑ g(a)
> - Lubs and glbs can be computed pointwise:
>   - ⊔ᵢ∈ᵢ fᵢ = λa. ⊔ᵢ∈ᵢ fᵢ(a)
>   - ⊓ᵢ∈ᵢ fᵢ = λa. ⊓ᵢ∈ᵢ fᵢ(a)
> - height(A → L) = |A| · height(L)

### 해설

**개념 설명**

**함수 격자(Map Lattice)** A → L은 집합 A에서 격자 L로의 모든 함수들의 집합입니다:

- **점별 순서(pointwise order)**: 모든 a ∈ A에서 f(a) ⊑ g(a)이어야 f ⊑ g
- **점별 join/meet**: 각 입력값 a에서의 join/meet을 계산

**수식 설명**

- 함수를 [a₁ ↦ x₁, a₂ ↦ x₂, ...]로 표기 (a₁ → x₁, a₂ → x₂, ... 매핑)
- λa. ⊔ᵢ∈ᵢ fᵢ(a): 각 a에서 fᵢ들의 값을 join하는 새 함수

**배경 지식**

- **함수 공간(Function Space)**: 두 집합 사이의 모든 함수들의 집합
- **점별 순서**: 함수들의 순서를 정하는 가장 자연스러운 방식

**전체적인 맥락**

이는 프로그램 분석의 **데이터 흐름 분석(data flow analysis)**에서 가장 중요한 격자입니다. 각 프로그램 점에서의 추상 상태를 함수로 표현합니다.

---

## 슬라이드 30: Constructing Lattices — Maps (Examples)

### 원문 내용
> **Constructing Lattices — Maps (Examples)**
>
> Examples:
> - Var → Sign
> - CFG Node → Var → Sign

### 해설

**개념 설명**

실제 프로그램 분석에서 사용되는 함수 격자의 예:

1. **Var → Sign**:
   - 프로그램의 모든 변수를 정의역으로 함
   - 각 변수의 부호(Sign 격자)를 값으로 함
   - 예: [x ↦ +, y ↦ −, z ↦ ⊤]

2. **CFG Node → Var → Sign**:
   - 제어 흐름 그래프의 각 노드에서
   - 각 변수의 부호 정보를 추적
   - 이중 함수 격자

**전체적인 맥락**

이들은 실제 프로그램 분석 도구에서 사용되는 추상 영역입니다. Var → Sign은 "이 프로그램 점에서 각 변수의 부호는?"이라는 질문에 답합니다.

---

## 슬라이드 31: Homomorphism and Isomorphism — Definitions

### 원문 내용
> **Homomorphism and Isomorphism — Definitions**
>
> **Definition** (Homomorphism). Where L₁ and L₂ are lattices, a function f : L₁ → L₂ is a homomorphism if
> ∀x, y ∈ L₁. f(x ⊔ y) = f(x) ⊔ f(y) ∧ f(x ⊓ y) = f(x) ⊓ f(y)
>
> **Definition** (Isomorphism).
> - If a homomorphism f is bijective, f is an isomorphism
> - If there exists an isomorphism f : L₁ → L₂, L₁ and L₂ are isomorphic, written L₁ ≅ L₂
> - Intuitively, isomorphic lattices are exactly the same (e.g., same Hasse diagram), only with different names for elements

### 해설

**개념 설명**

두 격자 사이의 구조 보존 함수들을 정의합니다:

- **동형(Homomorphism) f**: 격자의 연산을 보존합니다
  - f(x ⊔ y) = f(x) ⊔ f(y) (join 보존)
  - f(x ⊓ y) = f(x) ⊓ f(y) (meet 보존)

- **동형사상(Isomorphism)**: 전단사(bijective) 동형. 두 격자가 본질적으로 같음을 의미

**배경 지식** (학부 2학년 수준)

- **전단사(Bijective)**: 단사(injective, 일대일)이면서 동시에 전사(surjective, 전체 적용)
- **동형사상의 의미**: 구조적으로 완전히 같지만 라벨만 다름

**전체적인 맥락**

동형사상은 분석에서 추상 영역의 "동등성"을 확인하는 데 사용됩니다.

---

## 슬라이드 32: Homomorphism and Isomorphism — Examples

### 원문 내용
> **Homomorphism and Isomorphism — Examples**
>
> Examples:
> - Lⁿ ≅ A → L where #(A) = n
> - Signᴾ⁽ᵛᵃʳ⁾ ≅ Var → Sign

### 해설

**개념 설명**

동형사상의 구체적인 예:

1. **Lⁿ ≅ A → L** (|A| = n):
   - n개 원소를 가진 집합 A에서 격자 L로의 함수 격자
   - n개 격자의 곱과 동형
   - 이는 선택한 순서에 따라 같은 구조를 나타낼 수 있음을 보여줍니다

2. **Signᴾ⁽ᵛᵃʳ⁾ ≅ Var → Sign**:
   - 각 변수별로 Sign 격자를 갖는 곱 격자
   - 변수 집합 Var에서 Sign으로의 함수 격자와 동형
   - 실제로는 같은 추상 영역이지만 표현 방식이 다름

**배경 지식**

- #(A) = n: 집합 A의 원소 개수가 n

**전체적인 맥락**

이는 곱 격자와 함수 격자가 본질적으로 같은 개념임을 보여줍니다. 상황에 따라 더 편한 표현을 선택할 수 있습니다.

---

## 슬라이드 33: Summary

### 원문 내용
> **Summary**
>
> - Sign analysis abstracts integer values into {−, 0, +} with ⊤ (unknown) and ⊥ (no value)
> - A complete lattice is a poset where every subset has an lub and a glb
> - The lub (join) operation combines abstract information at control flow merge points
> - Complete lattices can be constructed using power sets, flat, products, and maps

### 해설

**개념 설명**

강의 전체의 핵심을 요약합니다:

1. **Sign Analysis**: 정수를 5개 값 {⊥, −, 0, +, ⊤}으로 추상화하는 기본 예시

2. **Complete Lattice**: 모든 부분집합이 최소상한과 최대하한을 가지는 부분 순서 집합

3. **Join Operation**: 여러 제어 흐름이 만날 때 추상 정보를 합치는 방법

4. **Lattice Construction**:
   - **Power Sets**: 멱집합과 부분집합 관계
   - **Flat**: 순서 없는 원소들로 높이 2인 격자 구성
   - **Products**: 여러 격자를 곱하여 고차원 격자 구성
   - **Maps**: 함수 격자로 프로그램 상태 표현

**전체적인 맥락**

이 강의는 프로그램 분석의 수학적 기초를 제공합니다. 다음 강의에서는 이 격자 이론을 사용하여 실제 분석 알고리즘(고정점 계산)이 어떻게 작동하는지 배울 것입니다.

**추가 설명** (보충)

격자 이론은 단순한 수학이 아니라, 프로그램 분석의 건전성(correctness)과 종료성(termination)을 **증명**하기 위한 필수적인 도구입니다. 고정점 정리(fixed point theorem)와 결합하면, 데이터 흐름 분석이 항상 올바른 결과를 유한 시간 내에 산출함을 보장할 수 있습니다.

---

## 추가 학습 자료

### 핵심 개념 정리

**부분 순서 집합 → 격자의 진행:**
- 부분 순서 집합: 기본 구조 (3가지 공리)
- 상한/하한: 원소들 사이의 비교 (여러 개 가능)
- 최소상한/최대하한: 최적의 상한/하한 (유일)
- 격자: 모든 두 원소가 join과 meet을 가짐
- 완전 격자: 모든 부분집합이 join과 meet을 가짐

**프로그램 분석과의 연결:**
- 추상 영역은 완전 격자여야 함
- Join은 제어 흐름 병합에서 사용
- Lattice의 높이는 분석 고정점 수렴의 상한
- 무한 승강 체인(infinite ascending chain)이 없으면 분석이 항상 종료

### 다음 강의 준비

다음 강의 "Lattice Theory (2)"에서는:
- 고정점(fixed point)의 개념
- 단조 함수(monotone function)와 연속 함수(continuous function)
- Knaster-Tarski 고정점 정리
- 데이터 흐름 분석 알고리즘의 수렴성

을 배울 것으로 예상됩니다.
