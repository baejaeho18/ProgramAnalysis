# Lattice Theory (1) - 강의 슬라이드 해설

CSE552 Program Analysis — Lecture 5
강사: Jaemin Hong

> **이 파일을 읽는 법**: 각 슬라이드는 `원문 내용` → `번역` → `해설`(개념·배경·맥락) → 필요 시 `각주`/`슬라이드 연결` 순서입니다. 누락·왜곡 없이 원문을 모두 담되 처음 보는 사람도 따라올 수 있게 풀어 썼습니다.

---

## 강의 5 전체 조감도 (먼저 큰 그림)

강의 3~4의 타입 분석은 "제약을 모아 단일화로 푼다"는 한 가지 틀이었습니다. 강의 5부터는 **대부분의 정적 분석을 떠받치는 수학적 토대 — 격자 이론(lattice theory)** 을 세웁니다. 왜 필요할까요? 강의 1에서 정적 분석의 필수 목표가 **종료(termination)와 건전성(soundness)**이라 했는데, **"분석이 반드시 끝나고 안전함을 어떻게 보장하는가?"** 에 답하려면 추상값들이 이루는 구조를 수학적으로 다뤄야 하기 때문입니다.

이 강의는 **부호 분석(sign analysis)**을 동기로, 그 추상값들 `{⊥, −, 0, +, ⊤}`이 이루는 구조를 분석하며 격자 이론을 끌어냅니다:
1. **부호 분석과 ⊤·⊥** (슬라이드 2~6): "양수/0/음수"로 추상화, 모를 때 ⊤, 값 없을 때 ⊥.
2. **부분 순서 집합(poset)** (슬라이드 7~11): 추상값 사이의 "정밀도 순서". Hasse 다이어그램.
3. **상한·하한, lub·glb(join·meet)** (슬라이드 12~16): 여러 정보를 합치는 연산. 특히 **join(⊔)이 분기 합류에서 정보를 모으는 핵심**.
4. **격자와 완비 격자** (슬라이드 17~24): 모든 쌍(또는 부분집합)이 join·meet을 갖는 구조. ⊤·⊥과 높이(height).
5. **격자 구성법** (슬라이드 25~32): 멱집합·flat·곱·맵으로 복잡한 격자를 조립. 동형(isomorphism).

핵심 통찰: **추상 도메인은 격자다.** "정밀도 순서(⊑)"가 부분 순서를, "정보 합치기(⊔)"가 join을 이룹니다. 이 격자 구조가 강의 6의 고정점 정리(종료·건전성 보장), 강의 7~9의 데이터플로우, 강의 16~17의 관계형 도메인, 강의 18~20의 추상 해석을 모두 떠받칩니다. 격자는 이 과목 후반의 공용어입니다.

---

## 슬라이드 1: 제목 슬라이드

### 원문 내용
> Lattice Theory (1)
> CSE552 Program Analysis — Lecture 5
> Jaemin Hong

### 번역
> 격자 이론 (1) / CSE552 프로그램 분석 — 강의 5 / 홍재민

### 해설
정적 분석의 수학적 토대인 **격자 이론**의 1편. 부호 분석을 동기로 격자를 끌어냅니다.

---

## 슬라이드 2: Sign Analysis

### 원문 내용
> - An analysis that finds out the possible integer values of variables and expressions
> - In concrete executions, values can be arbitrary integers
> - Our analysis considers an abstraction of the integer values by grouping them into the three categories, or abstract values: negative (−), zero (0), and positive (+)
> - e.g., if a sound analysis concludes that the final value of a variable is +, then the value must be a positive integer in any execution

### 번역
> - **부호 분석(sign analysis)**: 변수·식의 가능한 정수 값을 알아내는 분석
> - 실제 실행에서 값은 임의의 정수일 수 있음
> - 우리 분석은 정수 값을 세 범주(**추상값**)로 묶어 추상화: 음수(−), 0(0), 양수(+)
> - 예: 건전한 분석이 "변수의 최종 값은 +"라 결론내면, 그 값은 **어떤 실행에서도 양의 정수**여야 함

### 해설

**개념 설명 — 추상화의 첫 예: 부호 ★**

부호 분석은 정적 분석의 **가장 단순한 추상화** 예입니다. 무한한 정수 집합(`...,-2,-1,0,1,2,...`)을 단 **세 범주 {−, 0, +}**로 뭉칩니다. 정확한 값 대신 "부호만" 추적 — 계산은 쉬워지고(유한 도메인) 정보는 거칩니다.

"건전(sound)"의 의미가 강의 1·3에서처럼 적용됩니다: 분석이 "+"라 하면 **모든 실행에서 양수**임을 보장(과근사). 이 추상화로 "0으로 나누기"(분모가 +나 −면 안전, 강의 1·4) 등을 검사할 수 있습니다. 그런데 부호를 모르는 경우가 있습니다 — ⊤(슬3)과 ⊥(슬4).

---

## 슬라이드 3: Top (⊤)

### 원문 내용
> - The analysis may not know the sign of some expression
>   - The value is positive in some execution, and it is not in other executions
>   - The value is positive in every execution but the analysis fails to recognize this (which is unavoidable due to undecidability)
> - We add a special abstract value ⊤ representing "don't know"

### 번역
> - 분석이 어떤 식의 부호를 **모를** 수 있다:
>   - 어떤 실행에선 양수, 다른 실행에선 아닐 때
>   - 모든 실행에서 양수지만 분석이 못 알아챌 때(결정 불가능성 때문에 불가피, 강의 1)
> - 이런 "모름"을 나타내는 특수 추상값 **⊤(top)**을 추가

### 해설

**개념 설명 — ⊤ = "모름"(가장 부정밀) ★**

부호를 단정할 수 없는 경우(분기에 따라 다르거나, 결정 불가능해 못 알아낼 때)를 위해 **⊤("don't know")**를 둡니다. ⊤은 "양수일 수도 음수일 수도 0일 수도"라는 **가장 보수적·부정밀한** 추상값입니다(강의 1의 "x는 임의 정수"). 건전성을 위해 모르면 ⊤로 안전하게 처리 — 빠뜨리지 않되 정보가 없음. 반대 극단 ⊥이 슬4.

---

## 슬라이드 4: Bottom (⊥)

### 원문 내용
> - It is beneficial to also have an abstract value ⊥ for expressions
>   - whose values are not numbers (e.g., pointers), or
>   - that have no value in any execution because they are unreachable

### 번역
> - **⊥(bottom)** 추상값도 두면 유용하다:
>   - 값이 숫자가 아닌 식(예: 포인터), 또는
>   - **도달 불가능(unreachable)**해서 어떤 실행에서도 값이 없는 식

### 해설

**개념 설명 — ⊥ = "값 없음"(불가능)**

⊥은 ⊤의 반대 — "**아무 값도 없음**"입니다. 두 경우: ① 숫자가 아니라 부호가 무의미(포인터), ② **도달 불가능한 코드**(절대 실행 안 됨). ⊥은 "이 식은 어떤 정수도 될 수 없다"는 가장 정밀하지만 공허한 정보. 데이터플로우 분석에서 도달 불가 지점이 ⊥가 됩니다(강의 7). ⊤(전부 가능)과 ⊥(아무것도 불가능)이 양 극단을 이룹니다. 예가 슬5.

---

## 슬라이드 5: Sign Analysis Example

### 원문 내용
> ```c
> a = 42;
> b = 87;
> if input() { c = a + b; } else { c = a - b; }
> ```
> A sound analysis may conclude that, at return:
> - a is +
> - b is +
> - c is ⊤

### 번역
> `a=42`(+), `b=87`(+), 분기에서 `c = a+b`(양수+양수=양수) 또는 `c = a-b`(양수−양수는 부호 미정). 합류 후 c는 한쪽에서 +, 다른쪽에서 미정이라 **⊤**.

### 해설

**개념 설명**

a, b는 명확히 +. c는 then 가지에서 `a+b`=+이지만, else 가지에서 `a-b`(42−87=−45, 일반적으로 양수−양수는 부호 미정)라 ⊤. 두 분기가 합류하면서 "+"와 "⊤"가 합쳐져 **⊤**가 됩니다. 이 "분기 합류에서 정보 합치기"가 슬16의 join — 격자가 필요한 핵심 이유입니다. 이 다섯 추상값의 구조가 슬6.

---

## 슬라이드 6: Abstract Domains

### 원문 내용
> - For this analysis, we have an abstract domain consisting of the five abstract values: {⊥, −, 0, +, ⊤}
> - We can organize as follows with the least precise information at the top and the most precise information at the bottom:
> (Hasse diagram: ⊤ at top; −, 0, + in middle; ⊥ at bottom)

### 번역
> - 이 분석의 **추상 도메인**: 다섯 추상값 `{⊥, −, 0, +, ⊤}`
> - 위에 **가장 부정밀**(⊤), 아래에 **가장 정밀**(⊥)이 오도록 배치:
>   - ⊤ (맨 위) / −, 0, + (가운데) / ⊥ (맨 아래)

### 해설

**개념 설명 — 부호 격자 (추상 도메인의 구조) ★**

다섯 추상값을 **정밀도 순서**로 배열합니다: 맨 위 **⊤**(모름, 가장 부정밀), 가운데 **−, 0, +**(구체적 부호, 서로 비교 불가), 맨 아래 **⊥**(값 없음, 가장 정밀).

이 다이아몬드 모양이 **부호 격자(sign lattice)**입니다. "위로 갈수록 정보가 적다(부정밀)", "아래로 갈수록 정보가 많다(정밀)". 이 구조가 곧 **부분 순서**(슬7)이고, "정보 합치기"가 **join**(슬13). 이 도메인의 종료·건전성을 보장하려면 격자 이론이 필요(슬7). 부호 격자는 이후 강의 18~20의 추상 해석에서 표준 예제로 재등장합니다.

---

## 슬라이드 7: Questions — Motivation for Lattice Theory

### 원문 내용
> - How can we ensure termination and soundness of our analysis?
> - What is required for the abstract domain?
> - We need a mathematical foundation: the lattice theory
> - The connection between lattices and program analysis was established in the seminal work by Kildall, Kam, and Ullman¹²
>
> ¹ A unified approach to global program optimization (Kildall, 1973)
> ² Monotone data flow analysis frameworks (Kam and Ullman, 1977)

### 번역
> - 분석의 **종료와 건전성을 어떻게 보장**할까? 추상 도메인에 **무엇이 요구**되는가?
> - 수학적 토대가 필요: **격자 이론**
> - 격자와 프로그램 분석의 연결은 Kildall, Kam, Ullman의 기념비적 연구(1973, 1977)에서 확립됨

### 해설

**개념 설명 — 왜 격자가 필요한가 ★**

강의 1의 필수 목표(종료·건전성)를 **보장하는 조건**이 무엇인지 답하려면 추상 도메인의 수학적 구조가 필요합니다. **격자 이론**이 그 답을 줍니다:
- **종료**: 격자의 **유한 높이**(또는 위드닝)가 고정점 반복의 종료를 보장(슬24, 강의 6·9).
- **건전성**: 격자의 **단조 함수·고정점**이 건전성을 보장(강의 6·18~19).

Kildall(1973)과 Kam-Ullman(1977)이 "데이터플로우 분석 = 격자 위의 고정점 계산"임을 확립했습니다(강의 7~8). 즉 격자는 분석을 **수학적으로 정당화**하는 언어입니다. 가장 기본 개념인 부분 순서가 슬8.

---

## 슬라이드 8: Partially Ordered Sets — Definition

### 원문 내용
> Definition (Partially ordered set). A partially ordered set (poset) is a set S equipped with a binary relation ⊑ where the following conditions are satisfied:
> - Reflexivity: ∀x ∈ S. x ⊑ x
> - Transitivity: ∀x, y, z ∈ S. x ⊑ y ∧ y ⊑ z ⇒ x ⊑ z
> - Anti-symmetry: ∀x, y ∈ S. x ⊑ y ∧ y ⊑ x ⇒ x = y
> - c.f., a total order additionally requires totality (∀x, y ∈ S. x ⊑ y ∨ y ⊑ x)

### 번역
> **부분 순서 집합(poset)**: 집합 S와 이항 관계 ⊑로 이뤄지며 다음을 만족:
> - **반사성**: 모든 x에 대해 x ⊑ x
> - **추이성**: x⊑y이고 y⊑z이면 x⊑z
> - **반대칭성**: x⊑y이고 y⊑x이면 x=y
> - (참고: **전순서(total order)**는 추가로 **전체성**(임의 두 원소가 비교 가능)을 요구)

### 해설

**개념 설명 — 부분 순서(poset) ★**

**부분 순서 ⊑**는 "정밀도 비교"를 형식화합니다. 세 조건: **반사성**(자기 자신과 같은 정밀도), **추이성**(정밀도 비교가 연쇄됨), **반대칭성**(서로 더 정밀하면 같은 것).

핵심은 "**부분**" — 모든 쌍이 비교되진 않습니다. 부호 격자에서 +와 −는 **서로 비교 불가**(둘 다 ⊤보다 정밀, ⊥보다 부정밀이지만 서로 간엔 순서 없음). 이것이 전순서(모든 쌍 비교 가능, 예: 정수 ≤)와의 차이. 추상값들의 정밀도는 부분 순서를 이룹니다. 분석 관점의 직관이 슬9.

---

## 슬라이드 9: Partially Ordered Sets — Intuition and Notation

### 원문 내용
> - From the analysis perspective, when x ⊑ y, we say that "y is a safe approximation of x," or that "x is at least as precise as y"
> - We sometimes write y ⊒ x instead of x ⊑ y

### 번역
> - 분석 관점에서 `x ⊑ y`는 "**y는 x의 안전한 근사(safe approximation)**" 또는 "**x는 y만큼은 정밀**"하다는 뜻
> - `x ⊑ y`를 `y ⊒ x`로도 씀

### 해설

**개념 설명 — ⊑의 의미: 안전한 근사 ★**

순서 ⊑의 분석적 의미가 핵심입니다: `x ⊑ y`는 **"y가 x를 안전하게 근사"**(y가 x보다 크거나 같음 = 더 보수적·부정밀). 예: `+ ⊑ ⊤`("⊤은 +의 안전한 근사" — +를 ⊤로 봐도 안전하지만 정보 손실). 강의 1의 "과근사"가 이 ⊑로 형식화됩니다 — 실제(x)를 분석(y)이 덮으면 `x ⊑ y`. 건전성 = "실제 ⊑ 분석 결과"(강의 18~19). 위로 갈수록 부정밀(안전), 아래로 정밀. 예가 슬10.

---

## 슬라이드 10: Partially Ordered Sets — Examples

### 원문 내용
> Examples:
> - (ℕ, ≤)
> - (𝒫(S), ⊆)
> - ({⊥, −, 0, +, ⊤}, ⊑) where ⊥⊑⊥,−,0,+,⊤; −⊑−,⊤; 0⊑0,⊤; +⊑+,⊤; ⊤⊑⊤

### 번역
> poset 예:
> - **(ℕ, ≤)**: 자연수 크기 순서(전순서)
> - **(𝒫(S), ⊆)**: 멱집합 포함 순서(부분 순서)
> - **부호 도메인**: ⊥이 모두 이하, ⊤이 모두 이상, −·0·+는 서로 비교 불가

### 해설

**개념 설명**

세 가지 poset 예: **(ℕ,≤)**는 전순서, **(𝒫(S),⊆)**는 부분 순서(예: {a}와 {b} 비교 불가, 강의 11·14·18의 토큰/상태 집합), **부호 도메인**은 슬6의 다이아몬드. 멱집합과 부호 도메인이 부분 순서(비교 불가 쌍 존재)임을 봅니다. 시각화 도구가 슬11.

---

## 슬라이드 11: Hasse Diagrams

### 원문 내용
> - A partial order can be illustrated by a Hasse diagram in which the elements are nodes and the order relation is the transitive closure of edges leading from lower to higher nodes
> (예: ℕ 사슬; 𝒫({a,b,c}) 부울 격자; 부호 다이아몬드)

### 번역
> - **Hasse 다이어그램**: 원소를 노드로, 순서 관계를 **아래→위 간선의 추이 폐포**로 그린 그림 (직접 간선만 그리고 추이적 관계는 생략)

### 해설

**개념 설명 — Hasse 다이어그램 (격자의 그림)**

poset 시각화의 표준입니다. 규칙: **바로 위/아래 관계만 간선으로** 그리고, 추이적 관계는 따라 올라가면 됨. 부호 격자는 ⊥→{−,0,+}→⊤ 다이아몬드. 𝒫({a,b,c})는 ∅→...→{a,b,c} 부울 격자(큐브). 이후 모든 격자를 이 다이어그램으로 그립니다(강의 9·16·17·18). 순서가 정의됐으니 여러 원소의 상·하한을 정의(슬12).

---

## 슬라이드 12: Bounds

### 원문 내용
> Definition (Upper bound and lower bound). For X ⊆ S and y ∈ S,
> - y is an upper bound for X, written X ⊑ y, if ∀x ∈ X. x ⊑ y
> - y is a lower bound for X, written y ⊑ X, if ∀x ∈ X. y ⊑ x
> Example: Given (ℕ, ≤), for {5, 7, 10}, 10 and 100 are some of upper bounds, and 1 and 5 are some of lower bounds.

### 번역
> **상한·하한**: 부분집합 X와 y에 대해, y가 X의 **상한**이면 X의 모든 원소가 y 이하; **하한**이면 y가 X의 모든 원소 이하. 예: {5,7,10}의 상한 10·100, 하한 1·5.

### 해설

**개념 설명 — 상한·하한**

여러 원소 집합 X에 대해 **상한**은 "X 모두보다 크거나 같은 원소"(X를 덮음), **하한**은 "X 모두보다 작거나 같은 원소". 여럿 있을 수 있습니다. 그중 **가장 작은 상한·가장 큰 하한**이 분석에 중요(슬13) — 정보를 "딱 필요한 만큼" 합치기 때문.

---

## 슬라이드 13: Least Upper Bound and Greatest Lower Bound

### 원문 내용
> Definition (Least upper bound and greatest lower bound).
> - A least upper bound (lub), written ⨆X, satisfies: X ⊑ ⨆X ∧ ∀y. X ⊑ y ⇒ ⨆X ⊑ y
> - A greatest lower bound (glb), written ⨅X, satisfies: ⨅X ⊑ X ∧ ∀y. y ⊑ X ⇒ y ⊑ ⨅X

### 번역
> **최소 상한(lub) `⨆X`**: X의 상한이면서 다른 모든 상한보다 작거나 같음(가장 작은 상한). **최대 하한(glb) `⨅X`**: X의 하한이면서 다른 모든 하한보다 큼(가장 큰 하한).

### 해설

**개념 설명 — lub·glb = 가장 타이트한 합침 ★**

**최소 상한(lub, ⨆)**은 "X를 덮는 가장 작은(=가장 정밀한) 상한" — 군더더기 없이 딱 필요한 만큼만 합친 것. 부호 도메인에서 `⨆{+,−}=⊤`. **최대 하한(glb, ⨅)**은 반대. 분기 합류에서 "양쪽 정보를 안전하게 합치되 최대한 정밀하게"가 lub(슬16). lub가 곧 join(슬14). 표기가 슬14.

---

## 슬라이드 14: Lub and Glb — Notations

### 원문 내용
> - x ⊔ y := ⨆{x, y} (join of x and y)
> - x ⊓ y := ⨅{x, y} (meet of x and y)
> - ⨆_{a∈A} f(a) := ⨆{f(a) | a ∈ A}
> - ⨅_{a∈A} f(a) := ⨅{f(a) | a ∈ A}

### 번역
> `x ⊔ y` = 두 원소 lub = **조인(join)**; `x ⊓ y` = glb = **미트(meet)**; 함수 적용 버전도 정의.

### 해설

**개념 설명 — join(⊔)과 meet(⊓)**

두 원소의 lub를 **join `x⊔y`**, glb를 **meet `x⊓y`**. **join**은 정보 **합치기**(둘 다 가능, 분기 합류), **meet**은 정보 **교차**(둘 다 만족, 조건 필터). 부호 예: `+⊔0=⊤`, `⊤⊓+=+`. 이 두 연산이 격자의 핵심. 성질이 슬15.

---

## 슬라이드 15: Lub and Glb — Properties

### 원문 내용
> - If ⨆X exists, then it is unique
> - If ⨅X exists, then it is unique
> - If x ⊔ y exists, then x ⊑ y ⟺ x ⊔ y = y
> - If x ⊓ y exists, then x ⊑ y ⟺ x ⊓ y = x

### 번역
> - lub·glb는 **존재하면 유일**
> - `x⊔y=y ⟺ x⊑y` (x가 y 이하면 합쳐도 y)
> - `x⊓y=x ⟺ x⊑y` (x가 y 이하면 교차는 x)

### 해설

**개념 설명 — 순서와 연산의 일치**

핵심: **순서(⊑)와 join/meet이 일관**됩니다. `x⊑y`이면 `x⊔y=y`(더 큰 게 흡수), `x⊓y=x`(더 작은 게 흡수). 순서를 join/meet으로 판정 가능. lub·glb 유일성이 격자를 잘 정의되게 함. 이 일치성이 강의 18(갈루아 연결)·19(건전성 부등식)에서 계속 쓰입니다. 예와 역할이 슬16.

---

## 슬라이드 16: Lub and Glb — Examples and Role in Analysis

### 원문 내용
> - In (ℕ, ≤), ⨆ = max and ⨅ = min
> - In (𝒫, ⊆), ⨆ = ⋃ and ⨅ = ⋂
> - The lub/join operation plays an important role in program analysis
> - We use lub when combining abstract information from multiple sources
>   - e.g., when control flow merges after the branches of if statements

### 번역
> - (ℕ,≤)에서 **join=max, meet=min**; (𝒫,⊆)에서 **join=합집합, meet=교집합**
> - **join(lub)이 프로그램 분석의 핵심** — 여러 출처 정보를 합칠 때(특히 **if 분기 합류**) 사용

### 해설

**개념 설명 — join이 분기 합류를 처리한다 ★**

자연수에선 join=max, 멱집합에선 join=합집합. **분석에서의 역할**이 핵심: **if 분기 합류 시 양쪽 정보를 join으로 합침**(슬5의 c=⊤=+⊔미정). 강의 7~8의 데이터플로우 JOIN, 강의 18의 CJOIN이 모두 이 join. 즉 **join = "여러 경로 정보를 안전하게 모으기"**. join·meet이 항상 존재하는 구조가 격자(슬17).

---

## 슬라이드 17: Lattices — Definition

### 원문 내용
> Definition (Lattice).
> - A lattice is a partial order (S, ⊑) in which x ⊔ y and x ⊓ y exist for all x, y ∈ S
> - A complete lattice is a partial order (S, ⊑) in which ⨆X and ⨅X exist for all X ⊆ S

### 번역
> **격자**: 임의의 두 원소가 join·meet을 갖는 부분 순서. **완비 격자**: 임의의 부분집합이 lub·glb를 갖는 부분 순서.

### 해설

**개념 설명 — 격자 vs 완비 격자 ★**

- **격자**: **어떤 두 원소든** join·meet 존재.
- **완비 격자**: **어떤 부분집합이든**(무한 포함) lub·glb 존재. 더 강함.

**분석엔 완비 격자가 중요** — 임의 개수 경로 정보를 합치고(⨆), 고정점을 정의(강의 6)하려면 임의 부분집합 lub가 필요. 부호 격자·멱집합 격자는 완비 격자. 성질이 슬18.

---

## 슬라이드 18: Lattices — Properties

### 원문 내용
> - Every complete lattice is a lattice
> - What is a lattice that is not a complete lattice?
> - A nonempty finite lattice is complete
> - Where S is a poset, every subset of S has an lub ⟺ every subset of S has a glb
> - Most lattices we encounter in program analysis are complete lattices

### 번역
> - 완비 격자는 모두 격자
> - 완비 아닌 격자 예: (ℕ,≤) — 두 원소 max는 있지만 ℕ 전체의 lub는 없음
> - **유한 격자는 완비**
> - lub 존재 ⟺ glb 존재(쌍대성)
> - 분석에서 대부분 **완비 격자**

### 해설

**개념 설명**

- 완비 격자 ⊂ 격자. **유한 격자는 항상 완비**(부호 격자가 예). (ℕ,≤)는 격자지만 비완비(ℕ 전체에 상한 없음). lub 존재 ⟺ glb 존재.
- **분석엔 대부분 완비 격자** — 고정점 이론(강의 6)이 요구. 격자 판정 예가 슬19~23.

---

## 슬라이드 19~23: Lattice Examples (격자 판정 예제)

### 원문 내용
> - Example 1 (다이아몬드, 가운데 3원소): **Lattice**
> - Example 2 (위가 갈라진 모양): **Not a lattice**
> - Example 3 (사슬+다이아몬드): **Lattice**
> - Example 4 (교차 간선 6원소): **Not a lattice**
> - Example 5 (큐브 8원소): **Lattice**

### 번역
> Hasse 다이어그램으로 격자 여부 판정: 예제 1·3·5는 격자, 예제 2·4는 격자 아님.

### 해설

**개념 설명 — 격자 판정 (시험 단골) ★**

격자 여부는 **"임의의 두 원소가 유일한 lub와 glb를 갖는가"**로 판정합니다.
- **격자 아님(예제 2·4)**: 두 원소의 상한(또는 하한)이 **여럿인데 그중 최소(최대)가 유일하지 않은** 경우 → lub(glb) 없음.
- **격자(예제 1·3·5)**: 모든 쌍이 유일한 lub·glb.

요령: 두 원소를 골라 "공통 위쪽 원소 중 가장 낮은 게 유일?"(lub), "공통 아래쪽 중 가장 높은 게 유일?"(glb)을 확인. 둘 다 항상 성립해야 격자. 시험 단골 유형. ⊤·⊥과 높이가 슬24.

---

## 슬라이드 24: Top and Bottom

### 원문 내용
> - ⊤ = ⨆S
> - ⊥ = ⨅S
> - Height of S: the length of the longest path from ⊥ to ⊤
>   - e.g., the height of the sign lattice is 2
> - Every complete lattice has ⊤ and ⊥

### 번역
> - **⊤ = ⨆S**(전체의 lub = 최대), **⊥ = ⨅S**(전체의 glb = 최소)
> - **높이(height)**: ⊥→⊤ 최장 경로 길이 (부호 격자 = 2)
> - 모든 완비 격자는 ⊤·⊥을 가짐

### 해설

**개념 설명 — ⊤·⊥과 높이 ★**

⊤(최대)=전체 lub, ⊥(최소)=전체 glb. 완비 격자는 항상 둘을 가짐. **높이** = ⊥→⊤ 최장 경로(부호 격자 2).

**높이가 종료를 보장 ★**: 고정점 반복은 값이 ⊥에서 위로 **단조 증가**하는데, 높이가 유한하면 유한 단계에서 멈춥니다(강의 6·7). 높이 무한(구간)이면 위드닝 필요(강의 9·16·17). 즉 **유한 높이 = 종료 보장** — 강의 1의 "종료" 목표의 격자적 조건. 격자 조립법이 슬25~30.

---

## 슬라이드 25: Constructing Lattices — Power Sets

### 원문 내용
> - (𝒫(A), ⊆) is a complete lattice, called the power set lattice: ⨆ = ⋃, ⨅ = ⋂, ⊥ = ∅, ⊤ = A
> - (𝒫(A), ⊇) is also a complete lattice, called the reverse power set lattice: ⨆ = ⋂, ⨅ = ⋃, ⊥ = A, ⊤ = ∅

### 번역
> - **멱집합 격자 (𝒫(A),⊆)**: join=합집합, meet=교집합, ⊥=∅, ⊤=A
> - **역멱집합 격자 (𝒫(A),⊇)**: 순서를 뒤집음 → join=교집합, meet=합집합, ⊥=A, ⊤=∅

### 해설

**개념 설명 — 멱집합 격자 (가장 흔한 격자) ★**

부분집합들의 포함 격자. 분석에 자주 등장: 강의 11·14의 "함수/셀 집합", 강의 7의 "도달 정의/사용 가능 식", 강의 18의 "수집 의미론 𝒫(CState)". **순서를 뒤집으면**(⊇) join/meet도 뒤바뀝니다 — 강의 12의 available guard(⊇, must), 강의 7의 must 분석이 역멱집합. **⊆(may, 합집합) vs ⊇(must, 교집합)** 선택이 분석 성격을 정합니다(강의 7~8 4분면). flat이 슬26.

---

## 슬라이드 26: Constructing Lattices — Flat

### 원문 내용
> - For A = {a1, a2, ...}, flat(A) is a complete lattice with height 2:
> (Hasse: ⊤ at top; a1, a2, ... in middle; ⊥ at bottom)
> - Example: Sign = {⊥, −, 0, +, ⊤} can be expressed as flat({−, 0, +})

### 번역
> - **flat(A)**: 원소들을 가운데 나란히, 위에 ⊤·아래에 ⊥을 붙인 **높이 2 완비 격자**
> - 예: **부호 격자 = flat({−,0,+})**

### 해설

**개념 설명 — flat 격자**

**flat 격자**는 "구체값들을 서로 비교 불가하게 나란히 놓고 ⊤(모름)·⊥(없음)을 양 끝에 붙인" 구조. 부호 격자가 `flat({−,0,+})`. 높이 2. "정확한 값 하나 또는 모름/없음"(상수 전파 등)에 씁니다. 곱이 슬27.

---

## 슬라이드 27: Constructing Lattices — Products (Definition)

### 원문 내용
> - If L1, ..., Ln are complete lattices, then so is the product where the order ⊑ is defined componentwise:
>   - (x1, ..., xn) ⊑ (x1', ..., xn') ⟺ ∀i. xi ⊑ xi'
> - Product of n identical lattices: L^n
> - Lubs and glbs computed componentwise

### 번역
> - 완비 격자들의 **곱 `L1×...×Ln`**도 완비 격자: 순서·lub·glb를 **성분별(componentwise)**로 정의
> - 동일 격자 n개의 곱: `L^n`

### 해설

**개념 설명 — 곱 격자 ★**

여러 격자를 **곱**하면 각 성분을 독립적으로 다루는 격자. 순서·join·meet 모두 **성분별**. 예: `Sign×Sign`은 두 변수 부호 동시. 강의 16의 변수 곱공간, 강의 18 슬38의 곱 격자 갈루아 연결로 이어짐. **각 성분이 격자면 곱도 격자**(모듈성). 높이가 슬28.

---

## 슬라이드 28: Constructing Lattices — Products (Height and Examples)

### 원문 내용
> - height(L1 × ... × Ln) = height(L1) + ... + height(Ln)
> - Examples: Sign × Flat(ℕ): (+, 1), (−, ⊤); Sign^#(Var): (+, 0, ⊤), (⊤, ⊤, −)

### 번역
> - **곱 격자 높이 = 각 높이의 합**
> - 예: `Sign^|Var|`(변수마다 부호)의 원소 (+,0,⊤) 등.

### 해설

**개념 설명**

곱 격자의 **높이는 성분 높이의 합** — 유한 높이 유지(종료). `Sign^|Var|`은 변수마다 부호 하나 = 부호 분석 상태. 변수 n개면 높이 2n(유한 → 종료). 이 곱이 "한 지점의 추상 상태"(슬30의 맵으로 더 자연스럽게). 맵이 슬29.

---

## 슬라이드 29: Constructing Lattices — Maps (Definition)

### 원문 내용
> - If A is a set and L is a complete lattice, then we obtain a complete lattice called a map lattice A → L consisting of the set of functions from A to L, ordered pointwise:
>   - f ⊑ g ⟺ ∀a ∈ A. f(a) ⊑ g(a)
> - Lubs and glbs computed pointwise; height(A → L) = |A| · height(L)

### 번역
> - 집합 A와 완비 격자 L로 **맵 격자 `A→L`**(함수들), **점별(pointwise)** 순서·연산
>   - `f ⊑ g ⟺ 모든 a에서 f(a) ⊑ g(a)`
> - 높이 = |A|·height(L)

### 해설

**개념 설명 — 맵 격자 (상태 = 변수→추상값) ★**

**맵 격자 `A→L`**는 "A의 각 원소에 L의 값을 대응한 함수들". 순서·연산 모두 **점별**. 이것이 **분석 상태의 표준 형태**: `Var→Sign`, `Var→Interval`. 강의 18 슬16의 `State=Var→Sign`, 강의 9의 추상 상태가 모두 맵 격자. 높이 |A|·height(L)(유한 → 종료). 곱 격자와 본질적으로 같음(슬32 동형). 예가 슬30.

---

## 슬라이드 30: Constructing Lattices — Maps (Examples)

### 원문 내용
> Examples:
> - Var → Sign
> - CFG Node → Var → Sign

### 번역
> - **Var → Sign**: 각 변수의 부호 (한 지점의 추상 상태)
> - **CFG Node → Var → Sign**: 각 노드마다 변수 부호 — 프로그램 전체 분석 결과

### 해설

**개념 설명 — 분석 상태의 구조 ★**

두 맵 격자가 핵심 자료구조: **`Var→Sign`**(한 지점 상태), **`CFG Node→(Var→Sign)`**(모든 지점 = 분석 전체 결과). 이중 맵(노드→변수→추상값)이 강의 7~9 데이터플로우 결과, 강의 18의 `State^n`과 일치. 분석 결과 자체가 **격자 원소**이고, 분석은 이 격자에서 고정점을 찾는 것(강의 6). 구조 보존 함수가 슬31.

---

## 슬라이드 31: Homomorphism and Isomorphism — Definitions

### 원문 내용
> Definition (Homomorphism). f : L1 → L2 is a homomorphism if ∀x, y. f(x ⊔ y) = f(x) ⊔ f(y) ∧ f(x ⊓ y) = f(x) ⊓ f(y)
> Definition (Isomorphism). A bijective homomorphism. L1 ≅ L2 if an isomorphism exists. Intuitively, isomorphic lattices are exactly the same (same Hasse diagram) with different names.

### 번역
> - **준동형(homomorphism)**: join·meet을 **보존**하는 함수
> - **동형(isomorphism)**: 전단사 준동형. 동형(`L1≅L2`)인 두 격자는 **이름만 다를 뿐 같은 구조**

### 해설

**개념 설명 — 준동형·동형 (구조 보존)**

**준동형**은 격자 연산(join·meet)을 보존하는 함수 — 강의 18 슬29의 "완전 join 사상"이 이것(α가 join 보존하면 갈루아 연결 존재). **동형**은 일대일 대응 준동형으로, 두 격자가 본질적으로 같음. 예가 슬32.

---

## 슬라이드 32: Homomorphism and Isomorphism — Examples

### 원문 내용
> Examples:
> - L^n ≅ A → L where #(A) = n
> - Sign^#(Var) ≅ Var → Sign

### 번역
> - **`L^n ≅ A→L`** (|A|=n): n개 곱 = n개 키 맵 (본질적으로 같음)
> - **`Sign^|Var| ≅ Var→Sign`**

### 해설

**개념 설명**

곱 격자(슬27)와 맵 격자(슬29)가 **동형**. `L^n`(튜플)과 `A→L`(함수, |A|=n)은 같은 구조(i번째 성분 ↔ i번째 키). "변수 부호들의 곱"과 "변수→부호 맵"은 같은 격자, 표기만 다름 → 자유롭게 교체. 전체 요약이 슬33.

---

## 슬라이드 33: Summary

### 원문 내용
> - Sign analysis abstracts integer values into {−, 0, +} with ⊤ (unknown) and ⊥ (no value)
> - A complete lattice is a poset where every subset has an lub and a glb
> - The lub (join) operation combines abstract information at control flow merge points
> - Complete lattices can be constructed using power sets, flat, products, and maps

### 번역
> - 부호 분석은 정수를 `{−,0,+}`로 추상화 + ⊤·⊥
> - **완비 격자**는 모든 부분집합이 lub·glb를 갖는 poset
> - **join(lub)**이 제어 흐름 합류점에서 정보를 합침
> - 완비 격자는 **멱집합·flat·곱·맵**으로 구성

### 해설

**전체 정리 — 강의 5의 한 장 요약**

1. **부호 분석**: 정수→{−,0,+}, ⊤(가장 부정밀)·⊥(가장 정밀).
2. **부분 순서(⊑)**: 정밀도 순서, `x⊑y`="y는 x의 안전한 근사". 반사·추이·반대칭, 비교 불가 쌍 존재.
3. **lub·glb(join⊔·meet⊓)**: 가장 타이트하게 합치기(⊔, 분기 합류)·교차하기(⊓, 조건 필터).
4. **(완비) 격자**: 두 원소(임의 부분집합)가 join·meet. ⊤·⊥, 높이(유한=종료).
5. **구성**: 멱집합(∪/∩)·flat·곱(성분별)·맵(점별, 상태=변수→추상값). 곱≅맵.

**다른 강의와의 연결 (파일 간 연결성)**

- ← **강의 1**: 종료·건전성을 격자로 형식화(⊑=과근사, 유한 높이=종료).
- ← **강의 3~4**: 멱집합·flat이 타입/제약 도메인.
- → **강의 6 (고정점)**: 단조 함수·고정점 정리로 종료·건전성 완성.
- → **강의 7~9 (데이터플로우)**: JOIN=lub, 상태=맵 격자, 유한 높이=종료. ⊆(may)/⊇(must)가 4분면.
- → **강의 16~17 (관계형)**: 등식·다면체·팔각형이 모두 격자, join=lub.
- → **강의 18~20 (추상 해석)**: 𝒫(CState)·State^n이 격자, 갈루아 연결(준동형), 의미론=고정점.

**가장 큰 교훈**: **추상 도메인은 격자다.** 추상값들의 "정밀도 순서"가 부분 순서(⊑), "정보 합치기"가 join(⊔). 이 격자 구조가 정적 분석의 두 필수 목표를 보장 — **유한 높이가 종료를, 순서(과근사)가 건전성을**. 멱집합·flat·곱·맵으로 분석 상태를 체계적으로 조립합니다. 격자는 강의 6 이후 모든 분석의 공용어입니다.

---

## 마치며

강의 5는 부호 분석을 동기로 **격자 이론**이라는 정적 분석의 수학적 토대를 세웁니다. 핵심 한 줄: **"추상 도메인은 완비 격자이고, 정밀도 순서(⊑)는 안전한 근사를, join(⊔)은 분기 합류의 정보 합치기를, 유한 높이는 종료를 보장한다."** 멱집합·flat·곱·맵으로 분석 상태(변수→추상값, 노드→변수→추상값)를 조립합니다. 이 격자 언어가 강의 6의 고정점, 강의 7~9의 데이터플로우, 강의 18~20의 추상 해석을 모두 떠받칩니다. 시험에서는 (a) poset 세 조건과 부분 순서 vs 전순서(슬8), (b) lub/glb·join/meet 정의와 순서와의 일치(슬13~15), (c) Hasse 다이어그램으로 격자 여부 판정(슬19~23), (d) ⊤·⊥과 높이(종료와의 관계, 슬24), (e) 멱집합·곱·맵 구성과 동형(슬25~32)이 단골입니다.
