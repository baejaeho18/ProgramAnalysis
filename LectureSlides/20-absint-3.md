# Abstract Interpretation (3) - 강의 슬라이드 해설

CSE552 Program Analysis — Lecture 20
강사: Jaemin Hong

> **이 파일을 읽는 법**: 각 슬라이드는 `원문 내용` → `번역` → `해설`(개념·배경·맥락) → 필요 시 `각주`/`슬라이드 연결` 순서입니다. 누락·왜곡 없이 원문을 모두 담되 처음 보는 사람도 따라올 수 있게 풀어 썼습니다.

---

## 강의 20 전체 조감도 (먼저 큰 그림)

강의 18~19는 **수집 의미론(collecting semantics)** — "각 지점에서 도달 가능한 상태 집합" — 을 구체 의미론으로 삼아 건전성을 증명했습니다. 그런데 이 수집 의미론에는 한계가 있습니다: **"각 지점의 상태"만 알 뿐, 프로그램이 *어떻게* 그 상태에 도달했는지(실행 경로, 어떤 식이 평가됐는지, 지점 간 상태 관계)는 모릅니다.** 부호·구간 분석엔 충분하지만, **도달 정의 분석(reaching definitions)·사용 가능 식 분석(available expressions)** 같은 일부 분석은 "실행의 역사"가 필요해 수집 의미론으로는 건전성을 증명할 수 없습니다.

강의 20은 더 풍부한 구체 의미론 — **트레이스 의미론(trace semantics)** — 을 도입합니다. 트레이스는 **(프로그램 지점, 상태) 쌍의 유한 수열**로, 실행의 *전체 역사*를 담습니다. 줄거리:

1. **수집 의미론의 한계**와 트레이스 의미론의 정의 (슬라이드 2~9). 트레이스 집합을 `tf(T) = I ∪ grow(T)`의 최소 고정점으로 정의.
2. **트레이스 → 수집 → 추상**의 갈루아 연결 사슬 (슬라이드 10~14). 수집 의미론은 트레이스 의미론의 *추상*이고, 갈루아 연결은 **합성(compose)**되므로, 트레이스에서 분석 도메인까지 한 줄로 연결됩니다.
3. **건전성의 합성** (슬라이드 15). 갈루아 연결이 합성되니 건전성 증명도 합성됨.
4. **도달 정의 분석** — 트레이스 의미론이 *필요한* 분석의 실제 사례. 그 추상화 함수 αRD와 건전성 증명 (슬라이드 16~19).

핵심 통찰: **"어떤 구체 의미론을 쓰느냐가 어떤 분석의 건전성을 증명할 수 있느냐를 정한다."** 분석이 실행의 역사를 보면(도달 정의), 의미론도 역사를 담아야(트레이스) 합니다. 그리고 **갈루아 연결의 합성성**(강의 18 슬37~38의 확장)이 여러 추상화 층을 매끄럽게 잇는 핵심 도구입니다. 이 강의는 추상 해석 3부작의 마무리이자, "의미론의 선택"이라는 깊은 주제를 다룹니다.

---

## 슬라이드 1: 제목 슬라이드

### 원문 내용
> Abstract Interpretation (3)
> CSE552 Program Analysis — Lecture 20
> Jaemin Hong

### 번역
> 추상 해석 (3) / CSE552 프로그램 분석 — 강의 20 / 홍재민

### 해설
추상 해석 3편(마지막). **트레이스 의미론**과 "더 풍부한 의미론이 필요한 분석"(도달 정의)을 다룹니다.

---

## 슬라이드 2: Limitations of Collecting Semantics

### 원문 내용
> - Reachable state collecting semantics
>   - Collects states that are possible when program execution reaches each program point for some input
> - Precisely captures the meaning of programs in a way that allows us to prove soundness of some analyses, e.g., sign analysis
> - However, for other analyses, it is insufficient because it does not capture all the information about how programs execute

### 번역
> - **도달 상태 수집 의미론(reachable state collecting semantics)**:
>   - 어떤 입력에서 실행이 각 지점에 도달했을 때 가능한 상태들을 모음
> - 부호 분석 같은 **일부 분석의 건전성 증명**에는 충분히 정확하다
> - 그러나 **다른 분석**에는 불충분하다 — 프로그램이 *어떻게* 실행되는지에 대한 모든 정보를 담지 못하기 때문

### 해설

**개념 설명 — 수집 의미론이 못 담는 것**

강의 18~19의 수집 의미론은 "각 지점에서 가능한 상태 집합"입니다. 이것은 **"무엇(what)"** — 그 지점에서 변수들이 어떤 값일 수 있나 — 만 담고, **"어떻게(how)"** — 어떤 경로로 왔나, 도중에 무슨 식이 평가됐나, 지점들 사이 상태가 어떻게 연결되나 — 는 버립니다.

부호·구간 분석은 "그 지점의 변수 값"만 보면 되니 수집 의미론으로 충분합니다. 하지만 **도달 정의(어느 대입이 여기까지 살아 왔나), 사용 가능 식(어떤 식이 이미 계산됐나)** 같은 분석은 **실행 역사**가 필요해 수집 의미론으로는 부족합니다. 구체 예가 슬라이드 3.

---

## 슬라이드 3: Example

### 원문 내용
> ```
> y = 0;
> if input() { x = 1; } else { x = y + 1; }
> // {[x: 1, y: 0]}
> ```
> - Only tells the state in the last line
> - Does not tell which expression is evaluated
>   - Relevant to available expression analysis
> - Does not provide information about how one state at a program point is related to states at other program points

### 번역
> 코드 끝 지점의 수집 의미론은 `{[x:1, y:0]}`(가능한 상태)만 알려 준다.
> - **마지막 줄의 상태만** 말해 줌
> - **어떤 식이 평가됐는지** 모름 (사용 가능 식 분석에 필요)
> - 한 지점의 상태가 **다른 지점의 상태와 어떻게 관련되는지** 모름

### 해설

**개념 설명 — 잃어버린 정보 세 가지**

이 예에서 끝 지점 상태는 두 분기에서 모두 `[x:1, y:0]`입니다(then은 x=1, else는 x=0+1=1). 수집 의미론은 이 결과 상태만 알 뿐:
1. **어떤 경로로 왔나**(then이냐 else냐) 모름.
2. **어떤 식이 평가됐나**(`1`이냐 `y+1`이냐) 모름 — 사용 가능 식 분석은 "`y+1`이 이미 계산됐나"를 알아야 하는데 이 정보가 없음.
3. **지점 간 상태 관계** 모름.

이런 "실행의 역사·구조" 정보를 담는 것이 트레이스 의미론(슬4)입니다. 수집 의미론은 트레이스에서 이 정보를 *지워* 얻은 것임이 슬라이드 10에서 밝혀집니다.

---

## 슬라이드 4: Trace Semantics

### 원문 내용
> - Expresses the meaning of a program as the set of traces that can appear when the program runs
> - A trace is a finite sequence of pairs of program points and states
>   - Trace = (Node × CState)*
>   - A* = ⋃_{i∈ℕ} A^i = A^0 ∪ A^1 ∪ A^2 ∪ ...

### 번역
> - 프로그램의 의미를, 실행 시 나타날 수 있는 **트레이스(trace)들의 집합**으로 표현
> - **트레이스** = (프로그램 지점, 상태) 쌍의 **유한 수열**: `Trace = (Node × CState)*`
> - `A*` = 길이 0, 1, 2, ... 인 모든 수열의 합집합 (Kleene star)

### 해설

**개념 설명 — 트레이스: 실행의 역사 ★**

**트레이스**는 실행을 한 걸음씩 기록한 **(지점, 상태)의 수열**입니다. 예: `(v0, σ0)·(v1, σ1)·(v2, σ2)...` — "v0에서 상태 σ0, 그다음 v1에서 σ1, ...". 즉 프로그램이 **어디를 거쳐 어떤 상태로 변해 왔는지**를 통째로 담습니다.

`(Node × CState)*`의 `*`(Kleene 별)은 "유한 길이의 모든 수열"을 뜻합니다. 트레이스 의미론 `《P》`는 그런 트레이스들의 집합 — 실행마다 다른 경로가 다른 트레이스가 됩니다. 수집 의미론이 "각 지점의 상태"였다면, 트레이스 의미론은 "실행 경로 전체" — 훨씬 풍부합니다. 한 노드의 전이가 슬라이드 5.

---

## 슬라이드 5: Concrete Transfer Functions

### 원문 내용
> - We define the semantics of a single CFG node as a function from a concrete state to a set of concrete states
> - ctv : CState → 𝒫(CState)
> - v : x = e → ctv(ρ) = {ρ[x ↦ z] | z ∈ ceval(ρ, e)}
> - Others → ctv(ρ) = {ρ}

### 번역
> - 한 CFG 노드의 의미를, **구체 상태 → 구체 상태 집합** 함수로 정의: `ctv : CState → 𝒫(CState)`
> - 대입 `x=e`: ρ에서 x를 e의 가능한 값으로 바꾼 상태들
> - 그 외 노드: 상태 변화 없음 `{ρ}`

### 해설

**개념 설명**

`ctv`는 "한 상태가 노드 v를 거치면 어떤 상태들이 되나"입니다(강의 18의 구체 전이를 상태 하나 단위로). 대입은 x를 갱신, 나머지는 그대로. 이것이 트레이스를 한 걸음씩 늘릴 때(슬8의 grow) 쓰입니다. 전이의 표기가 슬라이드 6.

---

## 슬라이드 6: Notation

### 원문 내용
> - We write a transition between configurations as
>   (v, ρ) → (v', ρ') if v' ∈ csucc(ρ, v) and ρ' ∈ ctv'(ρ)

### 번역
> - **설정(configuration) 간 전이**를 `(v, ρ) → (v', ρ')`로 표기:
>   - `v'`이 ρ에서 v의 가능한 후속이고(`v'∈csucc(ρ,v)`), `ρ'`이 v'의 전이 결과(`ρ'∈ct_{v'}(ρ)`)일 때

### 해설

**개념 설명 — 한 걸음 실행**

`(v, ρ) → (v', ρ')`는 "지점 v·상태 ρ에서 한 걸음 실행하면 지점 v'·상태 ρ'이 된다"는 한 스텝 전이입니다. 후속 노드(csucc, 강의 18)로 다음 지점을, 전이(ctv', 슬5)로 다음 상태를 정합니다. 트레이스는 이 전이를 **이어 붙인** 수열. 이 전이로 트레이스 의미론을 정의하는 게 슬라이드 7.

---

## 슬라이드 7: Trace Semantics of Programs

### 원문 내용
> - 《P》 ∈ 𝒫(Trace)
> - 《P》 is the set of finite traces that start at the program entry and in each step proceed according to the CFG
>   - We do not require that the traces reach the program exit
> - 《P》 is the least solution to:
>   - I ⊆ 《P》 where I = {(entry, [x1↦z1,...,xn↦zn]) | z1,...,zn ∈ ℤ}
>   - (π·s ∈ 《P》 ∧ s → s') ⟹ π·s·s' ∈ 《P》

### 번역
> - `《P》`: 프로그램의 트레이스 의미론 (트레이스 집합)
> - **진입에서 시작해 CFG를 따라 한 걸음씩 진행하는** 모든 유한 트레이스 (끝까지 도달 안 해도 됨)
> - `《P》`는 다음 제약의 **최소 해**:
>   - 초기 트레이스 `I` ⊆ 《P》 (진입 지점·임의 초기 상태인 길이 1 트레이스들)
>   - **확장**: 트레이스 `π·s`가 《P》에 있고 `s → s'`(한 걸음 가능)이면, 늘린 트레이스 `π·s·s'`도 《P》에 있음

### 해설

**개념 설명 — 트레이스 의미론도 최소 고정점**

`《P》`는 두 규칙의 최소 해입니다:
- **기저(I)**: "진입에서 임의 초기 상태"인 길이 1 트레이스들(모든 실행의 시작점).
- **확장(grow)**: 이미 있는 트레이스를 한 걸음(`s→s'`) 늘린 것도 포함.

즉 "진입에서 시작해 가능한 모든 방식으로 한 걸음씩 늘린 모든 유한 트레이스"입니다. 강의 18의 수집 의미론처럼 **최소 고정점**으로 정의 — 단지 도메인이 "상태 집합"이 아니라 "트레이스 집합"일 뿐. "끝까지 안 가도 됨"은 무한 루프·중간 지점도 트레이스로 포함하기 위함. 단조 함수 형태가 슬라이드 8.

---

## 슬라이드 8: Definition with a Monotone Function

### 원문 내용
> - tf : 𝒫(Trace) → 𝒫(Trace); tf(T) = I ∪ grow(T)
> - where grow(T) = {π·s·s' | π·s ∈ T ∧ s → s'}
> - 《P》 = lfp(tf)

### 번역
> - **트레이스 전이 함수** `tf(T) = I ∪ grow(T)`:
>   - I: 초기 트레이스, grow(T): T의 각 트레이스를 한 걸음 늘린 것들
> - `《P》 = lfp(tf)` (tf의 최소 고정점)

### 해설

**개념 설명 — 트레이스 의미론을 lfp로**

슬라이드 7의 두 규칙을 한 함수 `tf(T) = I ∪ grow(T)`로 묶습니다. tf는 단조(T가 커지면 grow(T)도 커짐)이므로 Tarski 정리(강의 19 슬21)로 최소 고정점 존재. `《P》 = lfp(tf)`. 강의 18의 구체 의미론(`{|P|}=lfp(cf)`)과 **완전히 같은 구조** — 도메인만 트레이스로 바뀜. 그래서 강의 19의 건전성 증명 기법이 그대로 적용됩니다. 예가 슬라이드 9.

---

## 슬라이드 9: Example — Trace Semantics

### 원문 내용
> ```
> // v0
> y = 0;  // v1
> if input() {  // v2
>   x = 1;  // v3
> } else {
>   x = y + 1;  // v4
> }
> return;  // v5
> ```
> 《P》 = { 길이 1~4의 모든 트레이스들: (v0,...)·(v1,...)·(v2,...)·(v3 또는 v4,...)·(v5,...) }

### 번역
> 트레이스 의미론은 길이별로 모든 부분 실행을 담는다: 진입 v0만, v0·v1, v0·v1·v2, then 경로 ...·v3, else 경로 ...·v4, 그리고 끝 v5까지. 각 트레이스가 지점·상태의 수열로 실행 역사를 기록.

### 해설

**개념 설명 — 트레이스가 경로를 구분한다**

슬라이드 3에서 수집 의미론은 끝 상태 `[x:1,y:0]` 하나로 then/else를 못 구분했습니다. 트레이스 의미론은 **then 경로 트레이스(...·(v3,...))와 else 경로 트레이스(...·(v4,...))를 별개로** 담아, "어느 경로로 왔나", "v4에서 `y+1`을 평가했나"를 구분합니다. 슬라이드 3에서 잃었던 "어떻게(how)" 정보가 트레이스엔 보존됩니다. 수집 의미론과의 관계가 슬라이드 10.

---

## 슬라이드 10: Trace Semantics and Reachable State Semantics

### 원문 내용
> - Their relation can be expressed as a Galois connection:
> - αt : 𝒫(Trace) → 𝒫(CState)^n; αt(T) = (R1,...,Rn) where Ri = {ρ | ∃π. π·(vi, ρ) ∈ T}
> - γt : 𝒫(CState)^n → 𝒫(Trace); γt((R1,...,Rn)) = {π·(vi, ρ) | ρ ∈ Ri}
> - Reachable state semantics is an abstraction of trace semantics

### 번역
> - 두 의미론의 관계는 **갈루아 연결**로 표현됨:
> - `αt : 트레이스집합 → 상태집합곱` — 각 지점 vi에서, **그 지점으로 끝나는 트레이스의 마지막 상태들**을 모음 (트레이스에서 "역사"를 지우고 "각 지점의 상태"만 남김)
> - `γt`: 반대 방향
> - **도달 상태 의미론은 트레이스 의미론의 추상이다**

### 해설

**개념 설명 — 수집 의미론 = 트레이스 의미론의 추상 ★**

핵심 통찰: **수집(도달 상태) 의미론은 트레이스 의미론을 추상화한 것**입니다. 추상화 함수 αt는 트레이스 집합에서 **"역사를 지우고 각 지점의 상태만 추출"**합니다 — 각 지점 vi에 대해, 그 지점으로 끝나는 모든 트레이스의 마지막 상태를 모아 Ri. 이러면 "어떻게 왔나"는 사라지고 "그 지점에서 가능한 상태"만 남죠(= 수집 의미론).

αt·γt가 **갈루아 연결**을 이룹니다(트레이스가 더 구체, 상태집합이 더 추상). 즉 추상 해석의 **추상화 한 층이 더** 생긴 것:
$$\mathcal{P}(\text{Trace}) \;\xrightarrow{\alpha_t}\; \mathcal{P}(\text{CState})^n \;\xrightarrow{\alpha_c}\; \text{State}^n$$
트레이스(가장 구체) → 수집(중간) → 분석 도메인(가장 추상). 이 두 갈루아 연결을 합치는 게 슬라이드 11.

---

## 슬라이드 11: Composition of Galois Connections

### 원문 내용
> - Let α1:L1→L2, γ1:L2→L1 form a Galois connection, and α2:L2→L3, γ2:L3→L2 form a Galois connection
> - Then α2∘α1:L1→L3 and γ1∘γ2:L3→L1 form a Galois connection
> - We have a Galois connection between 𝒫(Trace) and 𝒫(CState)^n and a Galois connection between 𝒫(CState)^n and State^n
> - We also have a Galois connection between 𝒫(Trace) and State^n

### 번역
> - 두 갈루아 연결 (α1,γ1), (α2,γ2)이 사슬로 이어지면, **합성 `(α2∘α1, γ1∘γ2)`도 갈루아 연결**
> - 트레이스↔수집, 수집↔분석 도메인 두 갈루아 연결이 있으므로, **트레이스↔분석 도메인** 갈루아 연결도 성립

### 해설

**개념 설명 — 갈루아 연결의 합성성 ★**

갈루아 연결의 강력한 성질: **합성 가능(composable)**합니다. 두 연결을 사슬로 이으면 그 합성도 갈루아 연결입니다(α는 정방향 합성 α2∘α1, γ는 역방향 합성 γ1∘γ2).

따라서:
- 트레이스 → 수집 (αt, 슬10)
- 수집 → 분석 도메인 (αc, 강의 18)
이 둘을 합치면 **트레이스 → 분석 도메인** 갈루아 연결이 한 번에 생깁니다. 즉 가장 구체적인 의미론(트레이스)에서 분석 결과까지 **하나의 추상화 사슬**로 연결. 이 합성성 덕분에 여러 추상화 층을 따로 다루지 않고 통합할 수 있습니다. 그림이 슬라이드 12.

---

## 슬라이드 12: Composition of Galois Connections (cont.)

### 원문 내용
> (그림) 𝒫(Trace) ⇄[αt, γt] 𝒫(CState)^n ⇄[αc, γc] State^n; 바깥 사슬: αc∘αt (정방향), γt∘γc (역방향)

### 번역
> 세 격자(트레이스·수집·분석)와 두 갈루아 연결을 그림으로. 바깥 화살표가 합성된 연결: 추상화 `αc∘αt`, 구체화 `γt∘γc`.

### 해설

**개념 설명**

세 격자를 잇는 그림입니다. 안쪽 두 갈루아 연결(αt/γt, αc/γc)이 합성되어 바깥의 트레이스↔분석 연결(αc∘αt, γt∘γc)이 됩니다. 슬라이드 11을 시각화 — 추상화가 층층이 쌓여도 갈루아 연결이 유지됨을 보여 줍니다. 이 합성 연결 위에서 건전성을 증명합니다(슬13~15). 수집 의미론이 트레이스의 건전한 추상임이 슬라이드 13~14.

---

## 슬라이드 13: Soundness of Reachable State Semantics

### 원문 내용
> - αt(tf(T)) ⊑ cf(αt(T))
> - Let αt(T) = (R1,...,Rn). Need to prove αt(tf(T))i ⊆ cfi(R1,...,Rn)
> - Take ρ ∈ αt(tf(T))i. Then π·(vi, ρ) ∈ tf(T) for some π
> - Case 1: π·(vi, ρ) ∈ I → vi = entry and ρ is an initial state → ρ ∈ cfi(R1,...,Rn)

### 번역
> - **수집 의미론(cf)이 트레이스 의미론(tf)의 건전한 추상** ⟺ `αt(tf(T)) ⊑ cf(αt(T))`
> - 증명: ρ가 αt(tf(T))i에 있다 하면, ρ로 끝나는 트레이스가 tf(T)에 있음. 경우 나눔.
> - 경우 1 (초기 트레이스 I): vi=진입, ρ는 초기 상태 → cf의 진입 규칙으로 포함.

### 해설

**개념 설명 — 수집이 트레이스의 건전한 추상임을 증명**

강의 19의 "한 스텝 건전성"(`cg∘γ ⊑ γ∘g`, 또는 동치인 α형 `α∘cg ⊑ g∘α`)을 트레이스↔수집 층에 적용합니다:
$$\alpha_t(tf(T)) \;\sqsubseteq\; cf(\alpha_t(T))$$
"트레이스를 한 걸음 늘린 뒤 상태만 추출" ⊆ "상태만 추출한 뒤 수집 한 스텝". 증명은 ρ가 어디서 왔는지 경우를 나눕니다. 경우 1(초기 트레이스)은 자명. 경우 2(확장)가 슬라이드 14.

---

## 슬라이드 14: Soundness of Reachable State Semantics (cont.)

### 원문 내용
> - Case 2: π·(vi, ρ) ∈ grow(T)
>   - Then π = π'·(vj, ρ') where π' ∈ T and (vj, ρ') → (vi, ρ)
>   - Since π ∈ T, ρ' ∈ Rj
>   - Since (vj, ρ') → (vi, ρ), vi ∈ csucc(ρ', vj) and ρ ∈ ctvi(ρ')
>   - Thus, ρ ∈ cfi(R1,...,Rn)

### 번역
> 경우 2 (확장된 트레이스): ρ로 끝나는 트레이스가 더 짧은 트레이스(ρ'로 끝남)를 한 걸음 늘린 것. ρ'은 Rj에 있고(귀납), 전이 `(vj,ρ')→(vi,ρ)`가 성립하므로, ρ는 수집 전이 cfi의 결과에 포함됨.

### 해설

**개념 설명**

확장 경우: ρ로 끝나는 트레이스는 ρ'로 끝나는 트레이스를 한 걸음(`(vj,ρ')→(vi,ρ)`) 늘린 것. ρ'은 추출하면 Rj에 들어가고(αt 정의), 전이가 성립하니(csucc·ctvi) ρ는 수집 전이 cfi가 Rj에서 만들어 내는 상태에 포함됩니다. 따라서 `αt(tf(T)) ⊑ cf(αt(T))` 성립 → 수집은 트레이스의 건전한 추상. 이제 이를 분석까지 합성하는 게 슬라이드 15.

---

## 슬라이드 15: Soundness of Analysis

### 원문 내용
> - tf ∘ γt ⊑ γt ∘ cf
> - cf ∘ γc ⊑ γc ∘ f
> - tf ∘ (γt ∘ γc) = (tf ∘ γt) ∘ γc ⊑ (γt ∘ cf) ∘ γc = γt ∘ (cf ∘ γc) ⊑ γt ∘ (γc ∘ f) = (γt ∘ γc) ∘ f
> - lfp(tf) ⊑ (γ1 ∘ γ2)(lfp(f))

### 번역
> - 트레이스↔수집 건전성 `tf∘γt ⊑ γt∘cf` (슬13~14)
> - 수집↔분석 건전성 `cf∘γc ⊑ γc∘f` (강의 19)
> - **두 부등식을 합성**: `tf∘(γt∘γc) ⊑ (γt∘γc)∘f` → 건전성 정리(강19 슬22)로 `lfp(tf) ⊑ (γt∘γc)(lfp(f))`
> - 즉 트레이스 의미론(가장 구체)이 분석 결과의 구체화에 포함됨 → **건전**

### 해설

**개념 설명 — 건전성의 합성 ★**

갈루아 연결이 합성되듯(슬11), **건전성도 합성**됩니다. 두 층의 한 스텝 건전성:
- 트레이스→수집: `tf∘γt ⊑ γt∘cf`,
- 수집→분석: `cf∘γc ⊑ γc∘f`,
을 부등식 사슬로 이으면 **트레이스→분석** 한 스텝 건전성 `tf∘(γt∘γc) ⊑ (γt∘γc)∘f`이 나옵니다. 그러면 강의 19의 건전성 정리(한 스텝 → 고정점)로:
$$lfp(tf) \;\sqsubseteq\; (\gamma_t \circ \gamma_c)(lfp(f))$$
즉 **트레이스 의미론(`《P》=lfp(tf)`)이 분석 결과의 구체화에 포함** → 분석이 트레이스 의미론에 대해서도 건전. 강의 19의 기법이 더 풍부한 의미론으로 매끄럽게 확장됨을 보여 줍니다. 트레이스가 *필요한* 분석의 예가 슬라이드 16.

---

## 슬라이드 16: Reaching Definition Analysis

### 원문 내용
> - State = (𝒫(Node), ⊆)
> - JOIN(v) = ⋃_{u∈pred(v)} [|u|]
> - x = e: [|v|] = (JOIN(v) ↓ x) ∪ {v}; (↓x removes all definitions of x)
> - if x: [|v|] = JOIN(v); entry: [|v|] = JOIN(v); return: [|v|] = JOIN(v)

### 번역
> - **도달 정의 분석(reaching definitions)**: 상태 = 정의(노드) 집합, 순서 ⊆
> - JOIN: 선행자들의 합집합 (전방·may)
> - 대입 `x=e` 노드 v: `(JOIN에서 x의 옛 정의들 제거) ∪ {v}` — v가 x를 새로 정의하므로
> - 그 외: JOIN 그대로

### 해설

**개념 설명 — 트레이스가 필요한 분석 (강의 7~8 복습)**

**도달 정의 분석**은 "각 지점에 어떤 대입(정의)들이 *살아서 도달*하나"를 추적합니다(강의 7~8). 상태가 **노드(정의) 집합**입니다 — 변수 값이 아니라 "어느 대입이 여기까지 유효한가". 대입 `x=e`는 x의 옛 정의를 죽이고(`↓x`) 자기를 추가(`∪{v}`, kill/gen).

**핵심**: 이 분석의 상태(정의 집합)는 "그 지점의 변수 값"이 아니라 **"어떤 대입들이 거쳐 왔나"**라는 *실행 역사*입니다. 그래서 수집 의미론(값만 담음)으로는 건전성을 증명할 수 없고, **트레이스 의미론이 필요**합니다(슬2~3의 한계가 현실이 되는 지점). 추상화 함수가 슬라이드 17.

---

## 슬라이드 17: Abstraction Function for Reaching Definition Analysis

### 원문 내용
> - def(v, x) = v defines x (i.e., v is x = e)
> - last(π·s) = s
> - αRD : 𝒫(Trace) → 𝒫(Node)^n
> - αRD(T) = (D1,...,Dn) where Di = {vj | π·(vj,_)·π' ∈ T ∧ def(vj,x) ∧ ∀(v,_)∈π'. ¬def(v,x) ∧ last(π·(vj,_)·π') = (vi,_)}

### 번역
> - `def(v,x)`: 노드 v가 x를 정의함(즉 v가 `x=e`)
> - `last(π·s) = s`: 트레이스의 마지막 쌍
> - `αRD : 트레이스집합 → 정의집합곱`:
>   - 지점 vi에서 도달 정의 Di = "트레이스에서 vj가 x를 정의했고(`def(vj,x)`), **그 이후로 x를 재정의한 노드가 없으며**(`∀(v,_)∈π'. ¬def(v,x)`), 그 트레이스가 vi로 끝나는" 모든 vj

### 해설

**개념 설명 — 트레이스에서 도달 정의 추출 ★**

αRD는 **트레이스에서 "살아남은 정의"를 추출**합니다. 지점 vi로 끝나는 트레이스를 보고, "x를 정의한 노드 vj 중, 그 뒤로 x가 **재정의되지 않은** 것"을 모읍니다. "재정의 안 됨"(`∀(v,_)∈π'. ¬def(v,x)`)이 핵심 — 도중에 다른 대입이 x를 덮으면 그 정의는 죽었으니 제외.

**이것이 왜 트레이스가 필요한지를 보여 줍니다**: "vj 이후 x가 재정의됐나"는 **실행 경로(트레이스)를 봐야** 알 수 있습니다. 수집 의미론(끝 상태만)으론 이 역사를 못 봐 αRD를 정의할 수조차 없습니다. 트레이스 의미론이 있어야 비로소 도달 정의 분석의 추상화 함수를 세우고 건전성을 증명할 수 있습니다. 건전성 증명이 슬라이드 18~19.

---

## 슬라이드 18: Soundness of Reaching Definition Analysis

### 원문 내용
> - αRD(tf(T)) ⊑ f(αRD(T))
> - Let αRD(T) = (D1,...,Dn). Need to prove αRD(tf(T))i ⊆ fi(D1,...,Dn)
> - Take vj ∈ αRD(tf(T))i. Then π·(vj,ρj)·π' ∈ tf(T), def(vj,x), no node in π' defines x, and last(π·(vj,ρj)·π') = (vi,ρi)

### 번역
> - **도달 정의 분석 f가 트레이스 tf의 건전한 추상** ⟺ `αRD(tf(T)) ⊑ f(αRD(T))`
> - 증명: vj가 αRD(tf(T))i에 있다 하면, vj가 x를 정의하고 그 이후 x 재정의 없이 vi로 끝나는 트레이스가 tf(T)에 있음. 경우 나눔(슬19).

### 해설

**개념 설명**

강의 19의 건전성 레시피(슬23)를 도달 정의 분석에 적용합니다. 한 스텝 건전성 `αRD(tf(T)) ⊑ f(αRD(T))`를 보이면, 건전성 정리가 전체 분석의 건전성을 보장합니다. 증명은 "살아남은 정의 vj"가 어떤 트레이스에서 왔는지(초기/확장) 경우를 나눠 진행 — 슬라이드 19.

---

## 슬라이드 19: Soundness of Reaching Definition Analysis (cont.)

### 원문 내용
> - Case 1: the trace is in I → vi = entry, no such vj exists
> - Case 2: the trace is in grow(T) → π·(vj, ρj)·π'' ∈ T where π''·s = π'
>   - If vj = vi, then def(vi, x), so vj ∈ fi(D1,...,Dn)
>   - Otherwise, vj ∈ Dk where vk is a predecessor of vi
>   - Since no node after vj defines x, vi does not redefine x
>   - Thus, vj is not removed by ↓x and vj ∈ fi(D1,...,Dn)

### 번역
> 경우 1 (초기 트레이스): vi=진입, 그런 vj 없음(자명).
> 경우 2 (확장): 더 짧은 트레이스의 마지막 한 걸음. (a) vj=vi면 vi가 x를 정의하므로 gen으로 fi에 포함. (b) 아니면 vj는 선행 Dk에 있고, vj 이후 x 재정의가 없으니 vi가 x를 다시 정의하지 않음 → `↓x`로 제거 안 되어 fi에 포함.

### 해설

**개념 설명 — kill/gen이 건전한 추상임을 증명**

확장 경우의 핵심:
- **vj=vi(자기 자신이 정의)**: vi가 x를 새로 정의하니 `∪{v}`(gen)로 fi에 들어감.
- **vj≠vi**: vj는 선행 지점의 도달 정의(Dk)에 있었고, "vj 이후 x 재정의 없음"이므로 현재 vi도 x를 덮지 않음 → `↓x`(kill)에 걸리지 않아 살아남음 → fi에 포함.

즉 도달 정의의 **kill/gen 전이가 트레이스 의미론에 대해 건전**함이 증명됩니다. "재정의 없음" 조건이 트레이스(역사)에서 직접 검증되어, αRD와 f가 잘 맞물림을 보여 줍니다. 트레이스 의미론이 있어야 이 증명이 가능 — 강의 20의 결론. 전체 요약이 슬라이드 20.

---

## 슬라이드 20: Summary

### 원문 내용
> - Reachable-state collecting semantics is insufficient for analyses that need information about how programs execute, motivating a richer semantics
> - Trace semantics expresses meaning as the set of finite traces (sequences of program-point/state pairs) and is defined as the least fixpoint of tf(T) = I ∪ grow(T)
> - Reachable-state semantics is an abstraction of trace semantics, connected by the Galois connection (αt, γt)
> - Galois connections compose, yielding a connection from 𝒫(Trace) all the way to the analysis domain State^n and a corresponding soundness argument
> - Reaching definition analysis is naturally justified against trace semantics: its abstraction function αRD and soundness proof rely on tracking which definitions survive along a trace

### 번역
> - **도달 상태 수집 의미론은** 프로그램이 *어떻게* 실행되는지가 필요한 분석엔 불충분 → 더 풍부한 의미론이 필요
> - **트레이스 의미론**: 의미를 (지점,상태) 쌍의 유한 수열 집합으로 표현, `tf(T)=I∪grow(T)`의 최소 고정점
> - 도달 상태 의미론은 **트레이스 의미론의 추상**, 갈루아 연결 (αt,γt)로 연결
> - **갈루아 연결은 합성**되어, 트레이스에서 분석 도메인까지 연결과 건전성 논증을 제공
> - **도달 정의 분석**은 트레이스 의미론에 대해 자연스럽게 정당화됨 — αRD와 건전성 증명이 "트레이스를 따라 어떤 정의가 살아남나"에 의존

### 해설

**전체 정리 — 강의 20의 한 장 요약**

1. **동기**: 수집 의미론은 "각 지점의 상태"만 담아 "어떻게 실행됐나"(경로·역사)를 모름 → 도달 정의·사용 가능 식 같은 분석엔 부족(슬2~3).
2. **트레이스 의미론**: 실행을 (지점,상태) 수열로 통째 기록. `《P》=lfp(tf)`, `tf=I∪grow` (슬4~9).
3. **추상화 사슬**: 트레이스 →(αt)→ 수집 →(αc)→ 분석. 수집은 트레이스의 추상. **갈루아 연결 합성**으로 트레이스↔분석 직접 연결(슬10~12).
4. **건전성 합성**: 두 층의 한 스텝 건전성을 합쳐(슬15), 건전성 정리로 `lfp(tf) ⊑ γ(lfp(f))`.
5. **도달 정의**: 트레이스가 *필요한* 실제 분석. αRD가 "트레이스에서 재정의 안 된 정의 추출", 건전성이 트레이스의 역사에 의존(슬16~19).

**다른 강의와의 연결 (파일 간 연결성)**

- ← **강의 18 (추상 해석 1)**: α·γ·갈루아 연결, 의미론=lfp. 트레이스는 또 하나의 구체 의미론. 갈루아 연결의 합성성은 강18 슬37~38(맵·곱 확장)의 연장.
- ← **강의 19 (추상 해석 2)**: 건전성 정리(한 스텝→고정점), `cg∘γ⊑γ∘g`의 한 스텝 건전성, 구조적 증명이 그대로 트레이스 층에 적용·합성.
- ← **강의 6 (고정점)**: tf의 최소 고정점, Tarski.
- ← **강의 7~8 (데이터플로우)**: 도달 정의·사용 가능 식 분석. 이 강의가 그들의 건전성을 *왜 트레이스 의미론이 있어야* 증명할 수 있는지 밝힘.
- → (과목 종합): 추상 해석 3부작(18~20)이 강의 5~17의 모든 분석에 "왜 건전한가"의 이론적 토대를 완성. 의미론의 선택(수집 vs 트레이스)이 증명 가능 범위를 정함.

**가장 큰 교훈**: **"분석이 보는 정보의 깊이가 구체 의미론의 깊이를 정한다."** 분석이 단순히 "각 지점의 값"만 보면(부호·구간) 수집 의미론으로 충분하지만, "실행의 역사"를 보면(도달 정의 — 어떤 정의가 살아 왔나) 의미론도 역사를 담는 **트레이스 의미론**이어야 건전성을 증명할 수 있습니다. 그리고 **갈루아 연결의 합성성**이 트레이스→수집→분석의 여러 추상화 층을 하나의 건전성 논증으로 매끄럽게 잇습니다 — 추상 해석이 layered abstraction을 다루는 우아함의 정점입니다.

---

## 마치며

강의 20은 추상 해석 3부작을 마무리하며, **"어떤 구체 의미론을 쓸 것인가"**라는 깊은 질문을 던집니다. 강의 18~19의 수집 의미론은 부호·구간 분석엔 충분하지만, 도달 정의처럼 *실행의 역사*를 보는 분석엔 부족합니다. 더 풍부한 **트레이스 의미론**(실행을 (지점,상태) 수열로 기록)을 도입하고, **갈루아 연결의 합성성**으로 트레이스→수집→분석의 추상화 사슬과 그 건전성 논증을 한 줄로 잇습니다. 핵심 한 줄: **"수집 의미론은 트레이스 의미론의 추상이며, 분석이 실행 역사를 보면 의미론도 트레이스여야 건전성을 증명할 수 있다."** 시험에서는 (a) 수집 의미론이 못 담는 정보와 트레이스 의미론의 필요성(슬2~3), (b) 트레이스 의미론을 `lfp(I∪grow)`로 정의(슬7~8), (c) αt로 수집이 트레이스의 추상임을 보이기(슬10), (d) 갈루아 연결의 합성과 건전성 합성(슬11·15), (e) 도달 정의 분석이 왜 트레이스를 요구하며 αRD가 어떻게 정의되는지(슬16~17)가 단골입니다.
