# Abstract Interpretation (1) - 강의 슬라이드 해설

CSE552 Program Analysis — Lecture 18
강사: Jaemin Hong

> **이 파일을 읽는 법**: 각 슬라이드는 `원문 내용` → `번역` → `해설`(개념·배경·맥락) → 필요 시 `각주`/`슬라이드 연결` 순서입니다. 누락·왜곡 없이 원문을 모두 담되 처음 보는 사람도 따라올 수 있게 풀어 썼습니다.

---

## 강의 18 전체 조감도 (먼저 큰 그림)

지금까지(강의 5~17) 우리는 수많은 분석을 "설계"했습니다 — 부호·구간·포인터·관계형 등. 그때마다 "이 분석은 **건전(sound)**하다"고 말했지만, **"건전하다는 것이 정확히 무슨 뜻인가?"**는 직관적으로만 다뤘습니다. 강의 18~20의 **추상 해석(Abstract Interpretation, Cousot & Cousot 1977)**은 이 질문에 **수학적으로 엄밀한 답**을 줍니다.

핵심 아이디어는 두 세계를 연결하는 것입니다:
1. **구체 세계(concrete)** — 프로그램의 *진짜* 의미. 각 지점에서 실행 중 나타날 수 있는 **모든 구체 상태의 집합**(collecting semantics). 이것은 정확하지만 **계산 불가능(non-computable)**합니다.
2. **추상 세계(abstract)** — 우리가 설계한 분석. 구체 상태를 추상값(부호·구간 등)으로 근사. 계산 가능하지만 부정확.

두 세계를 잇는 다리가 **추상화 함수 α(concrete→abstract)**와 **구체화 함수 γ(abstract→concrete)**이고, 이 둘이 만족해야 할 황금 조건이 **갈루아 연결(Galois connection)**입니다. 갈루아 연결이 성립하면 "분석이 건전하다"가 **수학적으로 보장**됩니다.

이 강의의 줄거리:
1. **구체 의미론**을 최소 고정점으로 정의 (슬라이드 2~14)
2. **α, γ**와 단조성 (슬라이드 15~20)
3. **갈루아 연결**의 정의·성질·예제 (슬라이드 21~34)
4. **표현 함수 β**와 곱·맵 격자로의 확장 (슬라이드 35~38)

이 강의는 지금까지의 모든 분석에 **이론적 토대**를 부여하는, 가장 추상적이지만 가장 근본적인 강의입니다. 강의 5~6의 격자 이론이 본격적으로 빛을 발합니다.

---

## 슬라이드 1: 제목 슬라이드

### 원문 내용
> Abstract Interpretation (1)
> CSE552 Program Analysis — Lecture 18
> Jaemin Hong

### 번역
> 추상 해석 (1) / CSE552 프로그램 분석 — 강의 18 / 홍재민

### 해설
추상 해석 3부작의 1편. **건전성의 수학적 정의**와 **갈루아 연결**이 주제입니다.

---

## 슬라이드 2: Formalizing Soundness

### 원문 내용
> - So far, we have used the term soundness informally
>   - If an analysis is sound, the properties it infers for a given program hold in all actual executions of the program
> - Abstract interpretation
>   - Provides a mathematical foundation for what it means for an analysis to be sound
>   - Relates the analysis specification to the formal semantics of the programming language
>
> (각주 1: Abstract interpretation: A unified lattice model for static analysis of programs by construction or approximation of fixpoints (Cousot and Cousot, 1977))

### 번역
> - 지금까지 우리는 **건전성(soundness)**을 비형식적으로 써 왔다
>   - 건전한 분석이란, 그것이 추론한 성질이 그 프로그램의 **모든 실제 실행에서 성립**하는 분석
> - **추상 해석**은:
>   - 분석이 건전하다는 것의 의미에 **수학적 토대**를 제공
>   - 분석 명세를 프로그래밍 언어의 **형식 의미론(formal semantics)**과 연결
> - (각주: Cousot & Cousot 1977 — 추상 해석의 창시 논문)

### 해설

**개념 설명 — "건전하다"를 엄밀히 하기**

지금까지 "건전 = 추론한 성질이 모든 실행에서 참(과근사)"이라고 직관적으로 썼습니다. 추상 해석은 이를 **수학적으로 증명 가능한 명제**로 만듭니다. 방법: 분석(추상)과 언어의 진짜 의미(구체 의미론)를 **격자와 함수로 연결**하고, 그 연결이 특정 성질(갈루아 연결)을 만족하면 건전성이 따라 나옴을 보입니다.

**각주**: Cousot 부부의 1977년 POPL 논문은 프로그램 분석 이론의 기초를 놓은 기념비적 업적입니다. "고정점의 구성·근사를 통한 통합 격자 모델"이라는 제목 그대로, 이 강의의 모든 내용이 그 논문에서 나옵니다.

**슬라이드 연결**: 먼저 "구체 의미론"과 "추상 의미론"을 구분합니다(슬라이드 3).

---

## 슬라이드 3: Concrete vs. Abstract Semantics

### 원문 내용
> - Concrete semantics: captures the meaning of programs in ordinary execution without any approximations
>   - The usual notion of semantics
>   - Denotational semantics, operational semantics, axiomatic semantics, etc.
> - Abstract semantics: the approximated meaning of programs used by the static analysis under consideration
>   - What we have defined using constraint rules

### 번역
> - **구체 의미론(concrete semantics)**: 근사 없이 보통 실행에서의 프로그램 의미를 포착
>   - 통상적인 의미론 개념 (표시적·작용적·공리적 의미론 등)
> - **추상 의미론(abstract semantics)**: 고려 중인 정적 분석이 사용하는 **근사된** 프로그램 의미
>   - 우리가 제약식으로 정의해 온 것

### 해설

**개념 설명 — 두 가지 의미론**

- **구체 의미론**: 프로그램의 *진짜* 동작. "x=3이면 실행 후 y는 정확히 7" 같은 정확한 의미. 정확하지만 (튜링 정지 문제 때문에) 일반적으로 계산 불가능.
- **추상 의미론**: 우리가 분석으로 정의한 *근사* 의미. "x는 양수, y는 [0,10]" 같은 추상값. 계산 가능하지만 부정확.

지금까지 강의에서 "제약식으로 정의한 것"이 추상 의미론입니다. 추상 해석은 이 둘을 나란히 놓고 비교해 "추상이 구체를 안전하게 근사하는가(건전)"를 따집니다. 먼저 구체 의미론을 형식화합니다(슬라이드 4~).

---

## 슬라이드 4: Collecting Semantics

### 원문 내용
> - Concrete state: CState = Var → ℤ
> - For each CFG node v, {|v|} is a constraint variable that ranges over sets of concrete states: {|v|} ∈ 𝒫(CState)
> - {|v|} shall denote the set of concrete states that are possible at the program point immediately after the instruction represented by v, in some execution of the program

### 번역
> - **구체 상태(concrete state)**: `CState = Var → ℤ` (각 변수에 정수값을 대응한 메모리 스냅샷)
> - 각 CFG 노드 v에 대해, `{|v|}`는 **구체 상태들의 집합** 위에서 값을 갖는 제약 변수: `{|v|} ∈ 𝒫(CState)`
> - `{|v|}`는, 어떤 실행에서 v가 나타내는 명령 **직후** 프로그램 지점에서 가능한 **모든 구체 상태의 집합**

### 해설

**개념 설명 — 수집 의미론 (collecting semantics)**

구체 의미론을 정적 분석과 비교하기 좋은 형태로 만든 것이 **수집 의미론**입니다. 핵심:
- **구체 상태** = "그 순간 모든 변수의 정확한 값"(메모리 스냅샷). `Var→ℤ`.
- 각 프로그램 지점 v에서, **그 지점에 도달할 수 있는 모든 구체 상태를 모은 집합** `{|v|}`을 추적.

왜 "집합"인가? 입력·분기에 따라 한 지점에 여러 상태가 도달할 수 있으니(예: `if input()` 분기), 가능한 모든 상태를 모읍니다. 이 집합값 의미론은 정적 분석(각 지점에 추상값 하나)과 구조가 같아(각 지점에 집합 하나) 비교가 쉽습니다. `𝒫(CState)`는 강의 5~6의 멱집합 격자.

**슬라이드 연결**: 이 집합을 어떻게 계산하는지 — 식 평가(슬5), 후속 노드(슬6), JOIN(슬7), 제약식(슬8).

---

## 슬라이드 5: Expression Evaluation

### 원문 내용
> - ceval : CState × Expr → 𝒫(ℤ)
> - ceval(ρ, x) = {ρ(x)}
> - ceval(ρ, n) = {n}
> - ceval(ρ, input()) = ℤ
> - ceval(ρ, e1 + e2) = {z1 + z2 | z1 ∈ ceval(ρ, e1) ∧ z2 ∈ ceval(ρ, e2)}
> - ceval : 𝒫(CState) × Expr → 𝒫(ℤ)
> - ceval(R, e) = ⋃_{ρ∈R} ceval(ρ, e)

### 번역
> - **구체 식 평가** `ceval : 구체상태 × 식 → 정수집합`:
>   - 변수 x → `{ρ(x)}` (그 상태에서 x의 값)
>   - 상수 n → `{n}`
>   - `input()` → `ℤ` (어떤 정수든 가능)
>   - `e1 + e2` → 두 평가 결과의 모든 합
> - 상태 집합으로 확장: `ceval(R, e) = ⋃_{ρ∈R} ceval(ρ, e)` (집합 안 모든 상태에서 평가해 합집합)

### 해설

**개념 설명 — 구체 식 평가**

`ceval`은 식을 구체 상태에서 평가해 **가능한 값들의 집합**을 줍니다. 변수·상수는 단일값, `input()`은 ℤ 전체(미정), 덧셈은 가능한 합들. 상태 집합 R에 대해선 모든 상태의 결과를 합칩니다. 이것이 슬라이드 18의 추상 식 평가(부호 도메인 등)와 **대응**되며, 둘 사이의 관계가 건전성을 정합니다. 분기 처리가 슬라이드 6.

---

## 슬라이드 6: Successor Nodes

### 원문 내용
> - csucc : CState × Node → 𝒫(Node)
> - v : if e → csucc(ρ, v) = T ∪ F where
>   - T = {true(v)} if z ∈ ceval(ρ, e) for some z ≠ 0; ∅ otherwise
>   - F = {false(v)} if 0 ∈ ceval(ρ, e); ∅ otherwise
> - v: other nodes → csucc(ρ, v) = succ(v)
> - csucc(R, v) = ⋃_{ρ∈R} csucc(ρ, v)

### 번역
> - **구체 후속 노드** `csucc`: 상태 ρ에서 노드 v 다음에 갈 수 있는 노드들
> - `if e` 노드: 조건 e가 ρ에서 **0이 아닌 값**일 수 있으면 참 분기(true), **0**일 수 있으면 거짓 분기(false). 둘 다 가능하면 둘 다.
> - 그 외 노드: 일반 후속 `succ(v)`
> - 상태 집합으로 확장: 합집합

### 해설

**개념 설명 — 조건 분기의 구체 의미**

`csucc`는 "이 상태에서 다음에 어느 노드로 가는가"를 정합니다. `if e`에서 e가 참(≠0)일 수 있으면 참 분기로, 거짓(=0)일 수 있으면 거짓 분기로 갈 수 있습니다. 한 상태가 양쪽 다 가능하진 않지만(e가 정해진 값), 상태 *집합*에선 양쪽 다 나타날 수 있습니다(상태마다 다른 분기). 이것이 분석의 control-flow 처리(어느 분기로 정보가 흐르나)의 구체 버전입니다. 합류 처리가 슬라이드 7.

---

## 슬라이드 7: Join (CJOIN)

### 원문 내용
> - CJOIN(v) = {ρ ∈ CState | ∃w ∈ Node. ρ ∈ {|w|} ∧ v ∈ csucc(ρ, w)}

### 번역
> - **구체 합류** `CJOIN(v)` = "어떤 선행 노드 w에서, 상태 ρ가 w에 가능하고(`ρ∈{|w|}`) 그 상태에서 v로 갈 수 있는(`v∈csucc(ρ,w)`)" 모든 ρ의 집합

### 해설

**개념 설명 — 구체 의미론의 JOIN**

`CJOIN(v)`는 "v에 도달할 수 있는 상태들을 모든 선행 노드로부터 모은 것"입니다. 각 선행 w에 가능했던 상태 중, 거기서 v로 가는 것들을 모읍니다. 이것이 데이터플로우 분석(강의 7~8)의 JOIN(선행자 결합)의 **구체 버전** — 추상 JOIN(부호 합집합 등)이 이 구체 JOIN을 근사합니다. 제약식이 슬라이드 8.

---

## 슬라이드 8: Constraint Rules

### 원문 내용
> - v : x = e → {|v|} = {ρ[x ↦ z] | ρ ∈ CJOIN(v) ∧ z ∈ ceval(ρ, e)}
> - v: entry → {|v|} = {[x0 ↦ z0, ...] | zi ∈ ℤ}
> - v: other nodes → {|v|} = CJOIN(v)

### 번역
> - 대입 노드 `x = e`: CJOIN(v)의 각 상태에서 x를 e의 가능한 값 z로 바꾼 상태들의 집합
> - 진입 노드: 모든 변수가 임의 정수인 모든 상태 (초기엔 아무것도 모름)
> - 그 외 노드: `{|v|} = CJOIN(v)` (그냥 합류된 상태)

### 해설

**개념 설명 — 구체 의미론의 전이 함수**

각 노드가 상태 집합을 어떻게 바꾸는지 정의합니다. 대입은 CJOIN으로 들어온 각 상태에서 x를 갱신, 진입은 "모든 가능한 초기 상태"(아무 정보 없음), 나머지는 합류 그대로. 이것이 분석 전이 함수(강의 7~9)의 **구체 원본**입니다. 추상 전이 함수가 이를 근사. 이 제약식들이 정의하는 의미론의 격자 구조가 슬라이드 9.

---

## 슬라이드 9: Lattices

### 원문 내용
> - (𝒫(ℤ), ⊆) is a lattice
> - (𝒫(CState), ⊆) is a lattice
> - 𝒫(CState)^n is a lattice (n = |Node|)

### 번역
> - `(𝒫(ℤ), ⊆)`는 격자 (정수 집합들의 포함 격자)
> - `(𝒫(CState), ⊆)`는 격자 (구체 상태 집합들)
> - `𝒫(CState)^n`도 격자 (n = 노드 수; 각 노드마다 상태 집합)

### 해설

**개념 설명 — 구체 의미론도 격자 위에 산다**

강의 5~6의 격자 이론이 등장합니다. 멱집합 `𝒫(...)`는 포함 순서 ⊆로 완비 격자(join=합집합, meet=교집합). 프로그램 전체 의미는 "각 노드마다 상태 집합 하나"이므로 `𝒫(CState)^n`(곱 격자)에 삽니다. 격자라야 고정점 이론(슬10~13)을 적용해 의미론을 정의할 수 있습니다. 추상 도메인(부호·구간)도 격자였듯, 구체 도메인도 격자 — 둘을 격자 사상으로 연결하는 것이 추상 해석의 핵심. 프로그램 의미를 고정점으로 정의하는 게 슬라이드 10.

---

## 슬라이드 10: Semantics of Program

### 원문 내용
> - {|v1|} = cf1({|v1|}, {|v2|}, ..., {|vn|}), ... (모든 노드에 대한 연립 제약)
> - cf : 𝒫(CState)^n → 𝒫(CState)^n
> - cf((x1,...,xn)) = (cf1(x1,...,xn), ..., cfn(x1,...,xn))
> - The least fixed point for cf describes the semantics of the program
>   - The set of all concrete states that can occur at each program point in some execution

### 번역
> - 모든 노드의 제약식 `{|vi|} = cfi(...)`을 하나의 함수 `cf`로 묶음 (`𝒫(CState)^n → 𝒫(CState)^n`)
> - **cf의 최소 고정점(least fixed point)**이 프로그램의 의미를 기술
>   - 즉 각 지점에서 어떤 실행에서든 나타날 수 있는 모든 구체 상태의 집합

### 해설

**개념 설명 — 의미론 = 최소 고정점**

슬라이드 8의 제약식들은 서로 의존하는 연립방정식 `{|v|} = cf(...)`입니다. 이 방정식을 만족하는 해가 **고정점**(넣으면 그대로 나오는 점). 프로그램 의미는 그중 **최소 고정점(lfp)** — "꼭 도달 가능한 상태만 담은 가장 작은 해"입니다. 강의 6의 Kleene/Tarski 고정점 이론이 그대로 적용됩니다. 데이터플로우 분석도 전이 함수의 고정점을 구했듯(강의 7~9), 구체 의미론도 고정점으로 정의 — **분석과 의미론이 같은 수학 구조(고정점)**임이 추상 해석의 출발점. 왜 *최소* 고정점인지가 슬라이드 11~12.

---

## 슬라이드 11: Example: Concrete Semantics of a Loop

### 원문 내용
> (CFG: v1 entry → v2 x=0 → v3 if input() → [T] v4 x=x+2 →(다시 v3) / [F] v5 return)
> Least fixed point:
> - {|v1|} = {[x↦z] | z ∈ ℤ}
> - {|v2|} = [x↦0]
> - {|v3|} = {[x↦z] | z ∈ {0,2,4,...}}
> - {|v4|} = {[x↦z] | z ∈ {2,4,6,...}}
> - {|v5|} = {[x↦z] | z ∈ {0,2,4,...}}

### 번역
> 루프 `x=0; while(input()) x+=2;`의 구체 의미론(최소 고정점):
> - 진입: x 임의
> - x=0 후: x=0
> - 루프 헤드 v3: x ∈ **{0,2,4,...}** (짝수 비음수)
> - 루프 본문 후 v4: x ∈ {2,4,6,...}
> - return v5: x ∈ {0,2,4,...}

### 해설

**개념 설명 — 구체 의미론의 실제 모습**

이 루프의 *정확한* 의미는 "x는 0,2,4,...(짝수)"입니다. 최소 고정점이 이를 정확히 포착: v3에서 x∈{0,2,4,...}. 주목할 점은 이것이 **무한 집합**이고 일반적으로 닫힌 형태로 못 적을 수 있다는 것 — 그래서 구체 의미론은 정확하지만 계산 불가능(슬13). 분석(구간 도메인)이라면 위드닝으로 [0,∞] 같은 근사를 줬겠죠. 왜 *최소* 고정점이어야 하는지가 슬라이드 12.

---

## 슬라이드 12: Why Least Fixed Point?

### 원문 내용
> (같은 루프)
> Another fixed point:
> - {|v3|} = {[x↦z] | z ∈ ℤ}
> - {|v4|} = {[x↦z] | z ∈ ℤ}
> - {|v5|} = {[x↦z] | z ∈ ℤ}

### 번역
> 같은 루프의 **다른 고정점**: v3,v4,v5 모두 x∈ℤ(모든 정수). 이것도 제약식을 만족하지만(고정점), 실제 도달 불가능한 상태(x=1 등)까지 포함 → **너무 큼(부정확)**.

### 해설

**개념 설명 — 최소 고정점이 정확한 의미인 이유**

제약식을 만족하는 고정점은 여러 개입니다. 예에서 "모든 x∈ℤ"도 고정점이지만, 실제로는 절대 도달 못 하는 상태(x=1, x=−2 등)까지 포함해 **과도하게 큽니다**. **최소 고정점**만이 "실제로 도달 가능한 상태만" 담은 정확한 의미입니다. 더 큰 고정점들은 안전하지만(상위 근사) 부정확. 그래서 의미론은 **lfp**로 정의. (분석에서도 lfp를 구했음 — 강의 6~9.) lfp의 존재가 슬라이드 13.

---

## 슬라이드 13: Existence of the Least Fixed Point

### 원문 내용
> - Since cf is monotone, the lfp exists according to Tarski's fixed-point theorem
> - 𝒫(CState)^n has an infinite height
> - In general, the lfp is non-computable

### 번역
> - cf는 **단조(monotone)**이므로, **Tarski의 고정점 정리**에 의해 lfp가 존재한다
> - `𝒫(CState)^n`는 **무한 높이** 격자
> - 일반적으로 lfp는 **계산 불가능(non-computable)**하다

### 해설

**개념 설명 — 존재하지만 계산은 불가능**

- **존재**: cf가 단조(입력이 커지면 출력도 커짐)이고 격자가 완비이므로, **Tarski 고정점 정리**(강의 6)가 lfp의 존재를 보장.
- **계산 불가능**: 격자 높이가 무한(상태 집합이 무한히 커질 수 있음)이라 Kleene 반복이 안 끝날 수 있고, 본질적으로 정지 문제와 얽혀 lfp를 일반적으로 계산할 수 없습니다.

이것이 **정적 분석이 필요한 근본 이유**입니다: 정확한 의미(구체 lfp)는 계산 불가능하니, **추상화로 근사**해 계산 가능하게 만드는 것. 그 추상이 건전한가를 따지는 게 이 강의. 기호 정리가 슬라이드 14.

---

## 슬라이드 14: Notations

### 원문 내용
> - {|P|} = lfp(cf) where cf is the semantic constraint function
>   - {|P|} denotes the (concrete) semantics of P
> - [|P|] = lfp(f) where f is the analysis constraint function
>   - [|P|] denotes the analysis result (abstract semantics) of P

### 번역
> - `{|P|} = lfp(cf)`: 프로그램 P의 **구체 의미론** (의미 제약 함수 cf의 최소 고정점)
> - `[|P|] = lfp(f)`: P의 **분석 결과(추상 의미론)** (분석 제약 함수 f의 최소 고정점)

### 해설

**개념 설명 — 두 세계의 기호**

이제 두 세계가 모두 "최소 고정점"으로 정의됩니다:
- 구체: `{|P|} = lfp(cf)` — 정확하지만 계산 불가.
- 추상: `[|P|] = lfp(f)` — 근사지만 계산 가능(우리가 설계한 분석).

**건전성**이란 곧 `[|P|]`가 `{|P|}`를 안전하게 근사함(`{|P|} ⊆ γ([|P|])`)을 뜻합니다. 이를 보장하는 다리(α, γ)와 그 조건(갈루아 연결)이 슬라이드 15부터의 주제. 두 세계를 잇는 첫 함수 — 추상화 α가 슬라이드 15.

---

## 슬라이드 15: Abstraction Functions

### 원문 내용
> - Define how each element from the semantic lattice is most precisely described by an element in the analysis lattice
> - αa : 𝒫(ℤ) → Sign
> - αa(D) = ⊥ if D=∅; + if D≠∅ ∧ ∀z∈D. z>0; − if D≠∅ ∧ ∀z∈D. z<0; 0 if D={0}; ⊤ otherwise

### 번역
> - **추상화 함수**: 의미(구체) 격자의 각 원소를, 분석(추상) 격자의 원소로 **가장 정밀하게** 기술
> - `αa : 𝒫(ℤ) → Sign` (정수 집합 → 부호):
>   - 빈 집합 → ⊥
>   - 모두 양수 → +
>   - 모두 음수 → −
>   - {0} → 0
>   - 그 외(섞임) → ⊤

### 해설

**개념 설명 — 추상화 함수 α (구체 → 추상)**

**추상화 함수 α**는 구체 원소(정수 집합)를 **가장 정밀하게 표현하는** 추상값으로 보냅니다. 부호 예: {3,5}→+(다 양수), {0}→0, {-1,2}→⊤(섞여서 부호 단정 불가), ∅→⊥. "가장 정밀하게"가 핵심 — {3}을 ⊤로 보내도 안전하지만 정보 손실이 크니, 가능한 가장 구체적인 +로 보냅니다. 이 α가 슬라이드 18의 γ(반대 방향)와 짝을 이뤄 갈루아 연결을 만듭니다. 상태·곱으로의 확장이 슬라이드 16.

---

## 슬라이드 16: Abstraction Functions (cont.)

### 원문 내용
> - αb : 𝒫(CState) → State; αb(R) = σ where σ(x) = αa({ρ(x) | ρ ∈ R}) (State = Var → Sign)
> - αc : 𝒫(CState)^n → State^n; αc((R1,...,Rn)) = (αb(R1),...,αb(Rn))

### 번역
> - `αb : 구체상태집합 → 추상상태` — 각 변수 x에 대해, R 안 모든 상태에서의 x값들을 모아 αa로 추상화한 부호를 줌
> - `αc : 곱 → 곱` — 노드별로 αb 적용

### 해설

**개념 설명 — 층층이 쌓는 추상화**

α를 세 층으로 정의합니다:
- `αa`: 값 수준(정수 집합 → 부호).
- `αb`: 상태 수준(상태 집합 → 변수별 부호). 각 변수의 가능한 값들을 모아 αa.
- `αc`: 프로그램 수준(노드별 곱).

이 층층 구조는 슬라이드 37~38(맵·곱 격자의 갈루아 연결)에서 "α가 부분에서 전체로 자동 확장됨"으로 정당화됩니다. γ도 같은 층(슬18~19). 추상화의 필수 성질이 슬라이드 17.

---

## 슬라이드 17: Monotonicity of Abstraction

### 원문 내용
> - Abstraction functions should be monotone
>   - A larger set of concrete elements should not be represented by a smaller abstract element in the lattice order

### 번역
> - 추상화 함수는 **단조(monotone)**여야 한다
>   - 더 큰 구체 원소 집합이 더 작은 추상 원소로 표현되면 안 된다 (구체가 커지면 추상도 커지거나 같아야)

### 해설

**개념 설명 — 단조성의 필요**

α가 단조라는 것은 "구체 정보가 많아지면(집합이 커지면) 추상값도 커지거나 같다(더 보수적)"는 뜻입니다. 예: {3}→+, {3,−1}→⊤. 집합이 커지자 추상값도 +에서 ⊤로 커짐(⊑ 순서). 만약 큰 집합이 작은 추상값으로 가면 정보가 거꾸로 흘러 건전성이 깨집니다. 단조성은 고정점 이론 적용과 건전성의 전제. 반대 방향 함수 γ가 슬라이드 18.

---

## 슬라이드 18: Concretization Functions

### 원문 내용
> - Express the meaning of the analysis lattice elements in terms of the semantic lattice elements
> - γa : Sign → 𝒫(ℤ)
> - γa(⊥)=∅; γa(+)={z|z>0}; γa(−)={z|z<0}; γa(0)={0}; γa(⊤)=ℤ

### 번역
> - **구체화 함수(concretization)**: 분석(추상) 격자 원소의 의미를 구체 격자 원소로 표현
> - `γa : Sign → 𝒫(ℤ)`:
>   - ⊥ → ∅
>   - + → 양의 정수 전체
>   - − → 음의 정수 전체
>   - 0 → {0}
>   - ⊤ → ℤ 전체

### 해설

**개념 설명 — 구체화 함수 γ (추상 → 구체)**

**γ는 α의 반대 방향**입니다. 추상값이 "실제로 어떤 구체 값들을 의미하는가"를 줍니다. +는 "양의 정수 전체", ⊤는 "모든 정수"(아무 제약 없음), ⊥는 "없음". 즉 γ는 추상값을 그것이 포함하는 모든 구체값으로 펼칩니다. α(접기)와 γ(펼치기)가 짝을 이뤄 갈루아 연결(슬21)을 형성. 상태·곱 확장이 슬라이드 19.

---

## 슬라이드 19: Concretization Functions (cont.)

### 원문 내용
> - γb : State → 𝒫(CState); γb(σ) = {ρ | ∀x. ρ(x) ∈ γa(σ(x))}
> - γc : State^n → 𝒫(CState)^n; γc((σ1,...,σn)) = (γb(σ1),...,γb(σn))

### 번역
> - `γb : 추상상태 → 구체상태집합` — 각 변수 x의 값이 σ(x)의 구체화(γa(σ(x))) 안에 드는 모든 구체 상태
> - `γc` — 노드별 γb 적용

### 해설

**개념 설명**

γ도 α처럼 세 층(값·상태·곱). `γb`: 추상 상태 σ가 의미하는 구체 상태들 = "모든 변수가 자기 추상값의 구체화 범위에 드는 상태". 예: σ=(x↦+, y↦0)이면 γb(σ)={x>0이고 y=0인 모든 상태}. α(슬16)와 정확히 대칭. γ의 필수 성질이 슬라이드 20.

---

## 슬라이드 20: Monotonicity of Concretization

### 원문 내용
> - Concretization functions should be monotone
>   - A greater abstract value should not express a smaller set of concrete elements

### 번역
> - 구체화 함수도 **단조**여야 한다
>   - 더 큰 추상값이 더 작은 구체 원소 집합을 표현하면 안 된다

### 해설

**개념 설명**

γ 단조: 추상값이 커지면(더 보수적) 구체화 집합도 커지거나 같다. 예: +의 γ={z>0}, ⊤의 γ=ℤ. ⊤⊒+이고 ℤ⊇{z>0} — 큰 추상값이 큰 집합을. 직관적: 추상값이 "더 모호할수록" 그것이 포함하는 구체값도 더 많다. α·γ가 둘 다 단조라야 갈루아 연결(슬21)이 성립. 이제 두 함수를 묶는 핵심 개념 — 갈루아 연결.

---

## 슬라이드 21: Galois Connections

### 원문 내용
> - L1 and L2 are complete lattices
> - α : L1 → L2 is an abstraction function
> - γ : L2 → L1 is a concretization function
> - α and γ should satisfy:
>   - γ ∘ α is extensive: ∀x ∈ L1. x ⊑ γ(α(x))
>   - α ∘ γ is reductive: ∀y ∈ L2. α(γ(y)) ⊑ y
> - The pair of monotone functions (α, γ) is called a Galois connection

### 번역
> - L1(구체), L2(추상)가 완비 격자
> - α: L1→L2 (추상화), γ: L2→L1 (구체화)
> - 두 조건:
>   - **γ∘α는 확장적(extensive)**: 모든 x에 대해 `x ⊑ γ(α(x))` (추상화했다 되돌리면 원래보다 크거나 같음 = 안전)
>   - **α∘γ는 축소적(reductive)**: 모든 y에 대해 `α(γ(y)) ⊑ y` (구체화했다 추상화하면 원래보다 작거나 같음 = 최대한 정밀)
> - 이 단조 함수 쌍 (α, γ)를 **갈루아 연결(Galois connection)**이라 한다

### 해설

**개념 설명 — 갈루아 연결 (이 강의의 심장) ★**

갈루아 연결은 α와 γ가 "잘 짝지어졌다"는 조건으로, **두 부등식**으로 요약됩니다:

1. **γ∘α 확장적: `x ⊑ γ(α(x))`** — "x를 추상화(α)했다가 구체화(γ)로 되돌리면, 원래 x보다 **크거나 같다**". 즉 추상화는 정보를 잃을 수 있지만 **절대 잃지 않은 척하지 않는다**(과근사, 안전). 건전성의 본질.
2. **α∘γ 축소적: `α(γ(y)) ⊑ y`** — "y를 구체화(γ)했다가 추상화(α)하면 원래 y보다 **작거나 같다**". 즉 α는 **가장 정밀한** 추상값을 준다(필요 이상으로 모호하게 하지 않음).

직관: γ∘α는 **안전(과근사 보장)**, α∘γ는 **정밀(최선의 근사)**. 이 두 조건을 만족하는 단조 쌍이 갈루아 연결이며, 갈루아 연결이 있으면 **분석의 건전성이 수학적으로 따라 나옵니다**. 강의 18의 핵심 정리.

**배경 지식**: 갈루아 연결은 순서 이론의 일반 개념(두 단조 함수의 수반 adjunction)으로, 추상 해석에 응용된 것입니다. 시각화가 슬라이드 23.

**슬라이드 연결**: 두 조건의 직관(슬22), 그림(슬23), 부호 예제(슬24~25).

---

## 슬라이드 22: Intuitions

### 원문 내용
> - x ⊑ γ(α(x))
>   - Abstraction may lose precision but must be safe
> - α(γ(y)) ⊑ y
>   - Abstraction should always give the most precise possible description

### 번역
> - `x ⊑ γ(α(x))`: 추상화는 정밀도를 잃을 수 있지만 **반드시 안전**해야 한다
> - `α(γ(y)) ⊑ y`: 추상화는 항상 **가능한 가장 정밀한** 기술을 줘야 한다

### 해설

**개념 설명 — 두 부등식의 의미**

- **확장(안전)**: 추상화로 정보를 잃어도(γ(α(x))가 x보다 클 수 있어도), **실제를 빠뜨리진 않는다**(x를 포함). 부호 예: x={3}, α(x)=+, γ(+)={z>0}⊇{3} — 원래를 포함하니 안전(과근사).
- **축소(정밀)**: γ로 펼친 걸 다시 α로 접으면 원래보다 정밀해진다 — α가 군더더기 없이 가장 타이트한 추상값을 줌을 보장.

"안전하되 최대한 정밀"이 갈루아 연결의 정신. 그림이 슬라이드 23.

---

## 슬라이드 23: Intuitions, Visually

### 원문 내용
> (그림) 왼쪽: L1의 x → α → α(x) (L2) → γ → γ(α(x)) (L1), x ⊑ γ(α(x))
> 오른쪽: L2의 y → γ → γ(y) (L1) → α → α(γ(y)) (L2), α(γ(y)) ⊑ y

### 번역
> 두 다이어그램으로 확장·축소를 시각화: x를 α로 올렸다 γ로 내리면 x 위(또는 같음); y를 γ로 내렸다 α로 올리면 y 아래(또는 같음).

### 해설

**개념 설명**

왕복 여행의 그림입니다. 구체에서 출발(x)해 추상 갔다 돌아오면 **올라감**(x⊑γα(x), 정보 손실=커짐). 추상에서 출발(y)해 구체 갔다 돌아오면 **내려감 또는 제자리**(αγ(y)⊑y, 최선의 추상). 두 화살표의 방향이 갈루아 연결의 비대칭을 보여 줍니다. 부호 도메인에서 실제로 성립하는지 슬라이드 24~25가 확인.

---

## 슬라이드 24: Example: Sign Galois Connection (Extensive)

### 원문 내용
> - γ(α(∅)) = γ(⊥) = ∅ ⊇ ∅
> - γ(α({1})) = γ(+) = {z|z>0} ⊇ {1}
> - γ(α({0})) = γ(0) = {0} ⊇ {0}
> - γ(α({−1})) = γ(−) = {z|z<0} ⊇ {−1}
> - γ(α({1,−1})) = γ(⊤) = ℤ ⊇ {1,−1}
> - γ ∘ α is typically not the identity function (abstraction may lose precision)

### 번역
> 부호 도메인에서 **확장성** 확인: 각 정수 집합 D에 대해 `D ⊆ γ(α(D))`. 예: {1}→+→{z>0}⊇{1}. γ∘α는 보통 항등함수가 아니다(추상화가 정밀도를 잃으므로 — {1}이 {z>0}로 커짐).

### 해설

**개념 설명**

부호 도메인에서 γ∘α가 확장적임을 사례로 확인: 모든 D에 대해 D⊆γ(α(D)). {1}→+→{모든 양수}처럼 **커지지만(정밀도 손실) 원래를 포함(안전)**. γ∘α가 항등이 아닌 것은 정상 — 추상화는 본디 정보를 잃습니다. 축소성은 슬라이드 25.

---

## 슬라이드 25: Example: Sign Galois Connection (Reductive)

### 원문 내용
> - α(γ(⊥)) = α(∅) = ⊥ ⊑ ⊥
> - α(γ(+)) = α({z>0}) = + ⊑ +
> - α(γ(−)) = − ⊑ −; α(γ(0)) = 0 ⊑ 0; α(γ(⊤)) = ⊤ ⊑ ⊤
> - α ∘ γ is typically the identity function

### 번역
> 부호 도메인에서 **축소성** 확인: 각 추상값 y에 대해 `α(γ(y)) ⊑ y`. 여기선 모두 등호(`α(γ(y))=y`). α∘γ는 보통 항등함수다.

### 해설

**개념 설명**

부호 도메인에서 α∘γ는 항등(`α(γ(+))=+` 등). 이는 부호 도메인의 각 추상값이 서로 다른 구체 집합을 의미해(중복 없음), 펼쳤다 접으면 제자리로 옴을 뜻합니다. γ∘α(확장, 보통 항등 아님)와 α∘γ(축소, 보통 항등)의 비대칭에 주목. α∘γ가 항등이 아닐 수도 있는 경우가 슬라이드 26.

---

## 슬라이드 26: When is α∘γ Not the Identity?

### 원문 내용
> - When is α∘γ not the identity function?
> - γ(α(γ(x))) = γ(x) for all x
>   - γ(x) ⊑ γ(α(γ(x))); α(γ(x)) ⊑ x and γ is monotone, so γ(α(γ(x))) ⊑ γ(x)
> - Even if α(γ(x)) ≠ x, they have the same concretization
> - If two abstract elements have the same concretization, α∘γ may not be the identity function

### 번역
> - α∘γ가 항등이 아닌 경우는?
> - 항상 `γ(α(γ(x))) = γ(x)`가 성립한다 (확장성·축소성·단조성으로 유도)
> - 즉 `α(γ(x)) ≠ x`여도 둘은 **같은 구체화**를 가진다
> - **두 추상 원소가 같은 구체화를 가지면**, α∘γ가 항등이 아닐 수 있다

### 해설

**개념 설명 — 중복 추상값이 있을 때**

`α(γ(y))=y`(축소가 등호)가 깨지는 건, **여러 추상값이 같은 구체 집합을 의미할 때**입니다. 예컨대 두 추상값 a,b가 모두 γ로 같은 집합이 되면, γ(b)를 다시 α로 접으면 a로 갈 수 있어 `α(γ(b))=a≠b`. 하지만 γ(a)=γ(b)이므로 **의미는 같습니다**(슬26의 등식). 즉 α∘γ가 항등이 아닌 건 "추상 격자에 의미가 겹치는 군더더기 원소가 있다"는 신호. 이상적 도메인은 그런 중복이 없어 α∘γ=항등. α,γ가 서로를 유일하게 정함이 슬라이드 27.

---

## 슬라이드 27: Unique Determination

### 원문 내용
> If L1, L2 are complete lattices and α, γ form a Galois connection, then
> - γ is uniquely determined by α: γ(y) = ⨆{x ∈ L1 | α(x) ⊑ y}
> - α is uniquely determined by γ: α(x) = ⨅{y ∈ L2 | x ⊑ γ(y)}

### 번역
> 갈루아 연결에서:
> - **γ는 α로 유일하게 결정**: `γ(y) = ⨆{x | α(x) ⊑ y}` (α가 y 이하로 보내는 모든 x의 결합)
> - **α는 γ로 유일하게 결정**: `α(x) = ⨅{y | x ⊑ γ(y)}` (x를 포함하는 모든 y의 만남)

### 해설

**개념 설명 — α와 γ는 서로의 짝이 유일하다 (실용적 함의)**

갈루아 연결의 강력한 성질: **α만 정하면 γ가 자동으로(유일하게) 정해지고, 역도 성립**합니다. 공식:
- `α(x) = ⨅{y | x ⊑ γ(y)}`: x를 안전히 덮는 추상값들 중 **가장 작은(정밀한)** 것.
- `γ(y) = ⨆{x | α(x) ⊑ y}`: y로 추상화되는 구체값들을 모두 모은 것.

**실용적 의미**: 분석 설계자는 α나 γ 중 **하나만** 정하면 됩니다 — 나머지는 수학이 정해 줍니다. 보통 더 직관적인 쪽(γ: "이 추상값이 뭘 의미하나")을 정합니다. 둘 중 하나만 명세하면 됨이 슬라이드 28.

---

## 슬라이드 28: Implication

### 원문 내용
> - Once the analysis designer has specified the concrete and abstract semantics, the relation between the concrete domain and the abstract domain may be specified
>   - using an abstraction function α or using a concretization function γ
> - This holds only when α and γ form a Galois connection
> - Question: When does α have γ such that they form a Galois connection, and vice versa?

### 번역
> - 구체·추상 의미론을 정한 뒤, 두 도메인의 관계를 **α 또는 γ 하나로** 명세할 수 있다
> - 단, α와 γ가 갈루아 연결을 이룰 때만 성립
> - **질문**: α는 언제 짝 γ를 가져 갈루아 연결을 이루는가? (역도)

### 해설

**개념 설명**

슬라이드 27 덕분에 설계자는 α나 γ 하나만 명세하면 됩니다. 그런데 아무 α나 짝 γ를 갖는 건 아닙니다 — 어떤 조건이라야 갈루아 연결의 짝이 존재할까요? 그 답이 슬라이드 29~30(완전 join/meet 사상). 이 질문은 "내가 정의한 α가 올바른 갈루아 연결을 이루는가"를 검사하는 실용적 기준이 됩니다.

---

## 슬라이드 29: Complete Join/Meet Morphisms

### 원문 내용
> - f : L1 → L2 is a complete join morphism if f(⨆A) = ⨆_{a∈A} f(a) for all A ⊆ L1
> - f : L1 → L2 is a complete meet morphism if f(⨅A) = ⨅_{a∈A} f(a) for all A ⊆ L1

### 번역
> - f가 **완전 join 사상(complete join morphism)**: 임의 부분집합 A에 대해 `f(⨆A) = ⨆ f(a)` (join을 보존)
> - f가 **완전 meet 사상(complete meet morphism)**: `f(⨅A) = ⨅ f(a)` (meet을 보존)

### 해설

**개념 설명 — join/meet을 보존하는 함수**

함수가 격자 연산을 보존하는 성질입니다:
- **완전 join 사상**: "여럿을 합친 것의 상(像) = 각각의 상을 합친 것". 합집합과 잘 어울림.
- **완전 meet 사상**: 교집합과 잘 어울림.

이 성질이 갈루아 연결 짝의 **존재 조건**입니다(슬30). 직관: α가 join을 보존해야(여러 구체 정보를 합친 뒤 추상화 = 각각 추상화 뒤 합침), 짝 γ가 잘 정의됨. 존재 정리가 슬라이드 30.

---

## 슬라이드 30: Existence of Galois Connections

### 원문 내용
> - If α is a complete join morphism, then there exists γ such that α and γ form a Galois connection
> - If γ is a complete meet morphism, then there exists α such that α and γ form a Galois connection

### 번역
> - **α가 완전 join 사상이면**, 갈루아 연결을 이루는 γ가 **존재**한다
> - **γ가 완전 meet 사상이면**, 갈루아 연결을 이루는 α가 존재한다

### 해설

**개념 설명 — 존재 정리 (설계자의 검사 기준)**

슬라이드 28의 질문에 답합니다: **α가 join을 보존하면(완전 join 사상) 짝 γ가 반드시 존재**합니다(γ는 슬27 공식으로 구성). 마찬가지로 γ가 meet을 보존하면 짝 α 존재.

**실용적 의미**: 분석 설계자가 α를 정의했다면, "이 α가 join을 보존하는가?"만 확인하면 갈루아 연결(따라서 건전성 기반)이 보장됩니다. 이것이 도메인 설계의 정당성 검사. 그런데 갈루아 연결이 없어도 분석이 망가지는 건 아니라는 점이 슬라이드 31~34.

---

## 슬라이드 31: Galois Connection as Sanity Check

### 원문 내용
> - A certain analysis lattice may not have a Galois connection with the semantic lattice
> - This does not necessarily mean that the design is wrong
>   - The analysis still may be sound
>   - The analysis still can terminate
> - However, the analysis may produce surprising results

### 번역
> - 어떤 분석 격자는 의미 격자와 **갈루아 연결을 갖지 못할** 수 있다
> - 이것이 곧 설계가 틀렸다는 뜻은 아니다
>   - 분석은 여전히 건전할 수 있고
>   - 여전히 종료할 수 있다
> - 다만, **놀라운(직관에 반하는) 결과**를 낼 수 있다

### 해설

**개념 설명 — 갈루아 연결은 "필수"가 아니라 "건전성 검사"**

갈루아 연결이 없어도 분석이 곧장 틀린 건 아닙니다. 건전하고 종료할 수도 있습니다. 다만 갈루아 연결은 "α가 항상 **유일한 최선의** 추상값을 준다"를 보장하는데, 이게 없으면 그 보장이 깨져 **직관에 반하는 결과**(예: 더 정밀한 분석이 더 나쁜 결과 — 슬34)가 나올 수 있습니다. 즉 갈루아 연결은 도메인 설계가 "잘 짜였는지" 점검하는 **건전성 검사(sanity check)** 역할. 갈루아 연결이 없는 예가 슬라이드 32~33.

---

## 슬라이드 32: Example: A Lattice Without a Galois Connection

### 원문 내용
> - Sign' = {⊥, 0⁻, 0⁺, ⊤}
> - γ(⊥)=∅; γ(0⁻)={z|z≤0}; γ(0⁺)={z|z≥0}; γ(⊤)=ℤ
> - How should we define eval(σ, 0)?
> - How should we define α({0})?

### 번역
> - 변형 부호 격자 `Sign' = {⊥, 0⁻(≤0), 0⁺(≥0), ⊤}` (0이 0⁻와 0⁺ 양쪽에 속함)
> - γ: 0⁻→{z≤0}, 0⁺→{z≥0}, ⊤→ℤ
> - 문제: `α({0})`을 무엇으로 정의해야 하나? (0은 0⁻에도 0⁺에도 속함)

### 해설

**개념 설명 — 0이 두 곳에 걸쳐 갈루아 연결이 깨지는 격자**

이 변형 격자에선 0이 `0⁻(≤0)`과 `0⁺(≥0)` **둘 다**에 속합니다. 그래서 `{0}`을 추상화할 때 0⁻로 갈지 0⁺로 갈지 **유일한 최선이 없습니다**. 갈루아 연결은 "x를 덮는 추상값들의 **만남(⨅)**"으로 α를 정하는데(슬27), 0⁻와 0⁺의 만남은 ⊥인데 ⊥는 {0}을 안 덮어 모순. 즉 **최선의 추상값이 유일하게 정해지지 않아** 갈루아 연결이 없습니다. 그 결과가 슬라이드 33.

---

## 슬라이드 33: Example: No Galois Connection Exists

### 원문 내용
> - Let α({0}) = 0⁻: {0}⊑{0,1}, but 0⁻ ⋢ 0⁺ (즉 단조성·연결 깨짐)
> - Let α({0}) = 0⁺: {0}⊑{−1,0}, but 0⁺ ⋢ 0⁻
> - No Galois connection exists between Sign' and 𝒫(ℤ) with the given γ

### 번역
> - α({0})=0⁻로 정하면: {0}⊆{0,1}인데 0⁻⋢0⁺ → 갈루아 연결 조건 위반
> - α({0})=0⁺로 정하면: {0}⊆{−1,0}인데 0⁺⋢0⁻ → 역시 위반
> - 따라서 주어진 γ로는 Sign'과 𝒫(ℤ) 사이에 갈루아 연결이 **존재하지 않는다**

### 해설

**개념 설명**

α({0})을 어느 쪽으로 정하든 모순이 생깁니다. 0⁻로 정하면 {0,1}(0⁺로 추상화됨)을 덮는 데 실패하고, 0⁺로 정해도 {−1,0}에서 실패. 둘 다 단조성/연결 조건을 위반 → **갈루아 연결 없음**. 핵심 원인: 격자에 "0을 가장 정밀하게 표현하는 유일한 원소"가 없음(0⁻·0⁺가 겹침). 이런 도메인의 실제 폐해가 슬라이드 34.

---

## 슬라이드 34: Example: Counter-Intuitive Imprecision

### 원문 내용
> ```
> x = e1 != e2;  // e1 always equals e2
> // is x non-negative?
> ```
> - Initially, the analysis fails to recognize that e1 always equals e2, so it assigns 0⁺ to x, proving that x is non-negative
> - After improving the analysis, it recognizes that e1 always equals e2, so it assigns 0⁻ to x, failing to prove that x is non-negative
> - More precise analysis can lead to less precise results, which is counter-intuitive

### 번역
> 코드: `x = (e1 != e2)`인데 e1과 e2가 항상 같음 → x는 항상 0(false).
> - 처음엔 분석이 e1=e2를 못 알아 x에 **0⁺** 부여 → "x는 비음수"를 증명함
> - 분석을 개선해 e1=e2를 알아채면 x에 **0⁻** 부여(0이지만 0⁻로) → "x는 비음수"를 **증명 못 함**
> - **더 정밀한 분석이 더 부정확한 결과**를 낼 수 있다 — 직관에 반함

### 해설

**개념 설명 — 갈루아 연결이 없을 때의 기이한 현상 ★**

이것이 슬라이드 31의 "놀라운 결과"의 구체 예입니다. x는 실제로 항상 0인데:
- **덜 정밀한 분석**: x를 0⁺로 추상화 → "x≥0" 증명 성공.
- **더 정밀한 분석**: x=0임을 알아채고도 0을 0⁻(≤0)로 추상화 → "x≥0" 증명 **실패**.

**정밀도를 높였더니 결과가 나빠지는** 역설! 원인은 0이 0⁻·0⁺에 겹쳐 갈루아 연결이 없어, "0을 가장 정밀하게 표현하는 유일한 추상값"이 없기 때문. 갈루아 연결이 있으면 α가 항상 최선의 추상값을 주어 이런 **비단조적 정밀도 역전**이 안 생깁니다. 이 예가 "왜 갈루아 연결이 중요한가"를 가장 설득력 있게 보여 줍니다(시험 단골). 다른 명세 방법 — 표현 함수가 슬라이드 35.

---

## 슬라이드 35: Representation Functions

### 원문 내용
> - We can also specify the connection between concrete elements and abstract elements using a representation function
> - β : ℤ → Sign; β(z) = 0 if z=0; + if z>0; − if z<0
> - α can be derived from β: α(D) = ⨆_{z∈D} β(z)

### 번역
> - 구체·추상 원소의 관계를 **표현 함수(representation function)**로도 명세할 수 있다
> - `β : ℤ → Sign` — **개별 구체값** 하나를 추상값으로: β(0)=0, β(양수)=+, β(음수)=−
> - β로부터 α 유도: `α(D) = ⨆_{z∈D} β(z)` (집합 안 각 원소를 β로 보내 결합)

### 해설

**개념 설명 — 가장 간단한 명세: 표현 함수 β**

α는 "집합 → 추상값"이라 정의가 다소 번거롭습니다. **표현 함수 β는 "개별 원소 하나 → 추상값"**이라 훨씬 간단합니다(β(3)=+ 등). 그리고 α는 β로부터 자동 유도: 집합의 각 원소를 β로 보낸 뒤 모두 join. 예: α({3,5})=β(3)⨆β(5)=+⨆+=+; α({−1,2})=−⨆+=⊤.

**실용적 의미**: 설계자는 가장 직관적인 β(원소 하나를 어떻게 추상화)만 정하면, α·γ가 자동으로 따라오고 갈루아 연결도 보장됩니다(슬36). 가장 쉬운 도메인 명세법. β→갈루아 연결 유도가 슬라이드 36.

---

## 슬라이드 36: Deriving Galois Connections from Representation Functions

### 원문 내용
> - Let β : V → L where V is a set and L is a complete lattice
> - α : 𝒫(V) → L; α(S) = ⨆_{v∈S} β(v)
> - γ : L → 𝒫(V); γ(x) = {v ∈ V | β(v) ⊑ x}
> - Then, α and γ form a Galois connection

### 번역
> - 표현 함수 `β : V → L`(원소 → 추상값)가 주어지면:
>   - `α(S) = ⨆_{v∈S} β(v)` (집합의 각 원소를 β로 보내 join)
>   - `γ(x) = {v | β(v) ⊑ x}` (추상값 x로 덮이는 원소들)
> - 그러면 α와 γ는 **갈루아 연결을 이룬다** (자동 보장)

### 해설

**개념 설명 — β만 정하면 갈루아 연결 공짜**

이것이 도메인 설계의 가장 실용적인 도구입니다. 표현 함수 β(원소 하나를 추상화하는 직관적 함수)만 정하면:
- α는 join으로(슬35), γ는 "x로 덮이는 원소 모음"으로 자동 정의되고,
- 이 α,γ는 **항상 갈루아 연결**(증명은 β가 원소별이라 α가 자동으로 완전 join 사상이 됨 — 슬30).

즉 **β를 정하는 순간 건전성 기반이 공짜로 따라옵니다**. 부호·구간 등 대부분의 도메인이 이렇게 β로 정의됩니다. 곱·맵 격자로의 확장이 슬라이드 37~38.

---

## 슬라이드 37: Galois Connections and Map Lattices

### 원문 내용
> - Let L1, L2 be complete lattices, S a set, and α:L1→L2, γ:L2→L1 a Galois connection
> - Then α', γ' form a Galois connection between S→L1 and S→L2:
>   - α'(x)(s) = α(x(s))
>   - γ'(y)(s) = γ(y(s))

### 번역
> - 격자 L1,L2 사이에 갈루아 연결 (α,γ)가 있으면, **맵 격자** `S→L1`과 `S→L2` 사이에도 점별(pointwise) 갈루아 연결 (α',γ')이 성립:
>   - α'은 각 키 s에서 α를 적용, γ'은 γ를 적용

### 해설

**개념 설명 — 갈루아 연결의 자동 확장 (맵)**

값 수준 갈루아 연결(예: 𝒫(ℤ)↔Sign)이 있으면, **상태 수준**(State=Var→Sign, 즉 변수→값의 맵)으로 **자동 확장**됩니다 — 각 변수(키)에 점별로 α/γ를 적용. 이것이 슬라이드 16·19에서 αa→αb, γa→γb로 층을 쌓은 것의 정당화입니다. "값에서 잘 되면 상태에서도 잘 된다"가 수학적으로 보장. 곱 격자 확장이 슬라이드 38.

---

## 슬라이드 38: Galois Connections and Product Lattices

### 원문 내용
> - Let (α,γ) be a Galois connection between L1, L2, and (α',γ') between L1', L2'
> - Then α'', γ'' form a Galois connection between L1×L1' and L2×L2':
>   - α''((x,x')) = (α(x), α'(x'))
>   - γ''((y,y')) = (γ(y), γ'(y'))

### 번역
> - 두 갈루아 연결 (α,γ), (α',γ')이 있으면, **곱 격자** L1×L1'과 L2×L2' 사이에도 성분별 갈루아 연결 (α'',γ'')이 성립.

### 해설

**개념 설명 — 갈루아 연결의 자동 확장 (곱)**

상태 수준 갈루아 연결이 있으면 **프로그램 수준**(노드별 곱 State^n) 갈루아 연결로 **자동 확장**됩니다 — 각 성분(노드)에 성분별로. 이것이 슬라이드 16·19의 αb→αc, γb→γc 확장의 정당화. 슬라이드 37(맵)·38(곱)을 합치면, **값 하나에 대한 갈루아 연결만 세우면 프로그램 전체로 자동 확장**된다는 강력한 모듈성이 보장됩니다. 즉 설계자는 가장 작은 단위(β, 값)만 신경 쓰면 됩니다. 전체 요약이 슬라이드 39.

---

## 슬라이드 39: Summary

### 원문 내용
> - Abstract interpretation provides a mathematical foundation for the soundness of static analyses by relating abstract semantics to concrete semantics
> - The concrete semantics of a program can be defined as the least fixed point of a system of constraints over 𝒫(CState)^n
> - Abstraction (α) and concretization (γ) functions connect the semantic and analysis lattices
> - α and γ form a Galois connection when they are monotone, γ∘α is extensive, and α∘γ is reductive

### 번역
> - **추상 해석**은 추상 의미론을 구체 의미론과 연결해 정적 분석의 **건전성에 수학적 토대**를 제공한다
> - 프로그램의 **구체 의미론**은 `𝒫(CState)^n` 위 제약 시스템의 **최소 고정점**으로 정의된다
> - **추상화(α)·구체화(γ)** 함수가 의미 격자와 분석 격자를 연결한다
> - α,γ가 **단조이고, γ∘α가 확장적, α∘γ가 축소적**이면 **갈루아 연결**을 이룬다

### 해설

**전체 정리 — 강의 18의 한 장 요약**

1. **목표**: "건전하다"를 수학적으로 정의 — 추상 의미론이 구체 의미론을 안전히 근사함(슬2~3).
2. **구체 의미론**: 각 지점의 도달 가능 상태 집합(collecting semantics)을 `𝒫(CState)^n` 위 제약의 **최소 고정점**으로 정의. 정확하지만 계산 불가능(슬4~14).
3. **α(추상화)·γ(구체화)**: 두 격자를 잇는 단조 함수. 층층이(값→상태→프로그램) 확장(슬15~20).
4. **갈루아 연결**: γ∘α 확장적(안전) + α∘γ 축소적(정밀). 있으면 건전성 보장, α↔γ가 서로 유일하게 결정, 없으면 정밀도 역전 등 기이한 결과(슬21~34).
5. **실용 도구**: 표현 함수 β(원소 하나만 정하면 α·γ·갈루아 연결 자동), 맵·곱 격자로 자동 확장(슬35~38).

**다른 강의와의 연결 (파일 간 연결성)**

- ← **강의 5~6 (격자·고정점)**: 완비 격자, 단조 함수, Tarski/Kleene 고정점이 이 강의의 언어. 의미론도 분석도 lfp.
- ← **강의 7~9 (데이터플로우·위드닝)**: CJOIN/csucc/전이 함수의 추상이 우리가 만든 분석. 구체 lfp가 계산 불가라 추상(위드닝 등)이 필요.
- ← **강의 5 (부호 분석)**: 부호 도메인이 α·γ·갈루아 연결의 표준 예제로 재등장.
- ← **강의 16~17 (관계형 도메인)**: 구간·다면체·팔각형 모두 𝒫(CState)와 갈루아 연결로 정당화되는 추상 도메인. 위드닝도 추상 해석의 한 장치.
- → **강의 19~20 (추상 해석 2·3)**: 갈루아 연결 위에서 **건전한 추상 전이 함수의 유도**, 최적 추상 연산, 고정점 전이(transfer) 등으로 이어짐.

**가장 큰 교훈**: 추상 해석은 지금까지 만든 모든 분석에 **"왜 건전한가"의 수학적 근거**를 줍니다. 핵심은 **갈루아 연결** — γ∘α 확장(안전)과 α∘γ 축소(정밀)라는 두 부등식. 설계자는 표현 함수 β 하나만 잘 정하면, α·γ·갈루아 연결·건전성 기반이 자동으로 따라오고 프로그램 전체로 확장됩니다. "정밀하게 만들었더니 결과가 나빠지는" 역설(슬34)은 갈루아 연결이 없을 때의 경고이며, 잘 설계된 도메인이 왜 갈루아 연결을 가져야 하는지를 보여 줍니다.

---

## 마치며

강의 18은 이 과목의 **이론적 정점**입니다. 그동안 "건전하다"고 말해 온 모든 분석을, 구체 의미론(계산 불가능한 정확한 lfp)과 추상 의미론(계산 가능한 근사 lfp)을 **갈루아 연결**로 잇는 엄밀한 틀 안에 놓습니다. 핵심 한 줄: **"α(추상화)와 γ(구체화)가 갈루아 연결(γ∘α 확장·α∘γ 축소)을 이루면 분석의 건전성이 수학적으로 보장된다."** 표현 함수 β로 도메인을 간단히 명세하고 맵·곱으로 자동 확장하는 모듈성도 핵심입니다. 시험에서는 (a) 갈루아 연결의 두 조건 진술·증명, (b) 부호 도메인에서 확장성/축소성 확인(슬24~25), (c) 갈루아 연결이 없는 격자와 정밀도 역전 현상 설명(슬32~34), (d) β로부터 α·γ 유도(슬35~36), (e) 의미론을 최소 고정점으로 정의하는 이유(슬11~13)가 단골입니다.
