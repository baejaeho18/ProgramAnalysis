# Lattice Theory (2) - 강의 슬라이드 해설

CSE552 Program Analysis — Lecture 6
강사: Jaemin Hong

> **이 파일을 읽는 법**: 각 슬라이드는 `원문 내용` → `번역` → `해설`(개념·배경·맥락) → 필요 시 `각주`/`슬라이드 연결` 순서입니다. 누락·왜곡 없이 원문을 모두 담되 처음 보는 사람도 따라올 수 있게 풀어 썼습니다.

---

## 강의 6 전체 조감도 (먼저 큰 그림)

강의 5는 "추상 도메인은 격자다"라는 구조를 세웠습니다. 강의 6은 그 격자 위에서 **분석이 어떻게 계산되고, 왜 종료하며, 무엇이 정답인가**에 답합니다. 핵심은 **고정점(fixed point)**입니다.

흐름은 이렇습니다:
1. **동기** (슬라이드 2~6): 부호 분석을 흐름 감각(flow-sensitive)으로 — 각 지점마다 상태(변수→부호)를 둠. 그러면 분석은 **연립방정식 `x = f(x)`**가 되고, 루프가 있으면 방정식이 **상호 재귀**가 되어 단순 대입으로 못 풉니다. 그 해가 **f의 고정점**.
2. **단조 함수** (슬라이드 7~11): 고정점이 잘 존재하려면 전이 함수 f가 **단조(monotone)** — "더 정밀한 입력 → 더 정밀한 출력"(정보를 거꾸로 뒤집지 않음) — 여야 합니다.
3. **고정점 정리** (슬라이드 12~17): **Tarski 정리**(단조 함수는 완비 격자에서 최소 고정점을 가짐)가 존재를, **Kleene 정리**(유한 높이면 `⊥, f(⊥), f²(⊥), ...`를 반복하면 lfp에 도달)가 **계산 방법**을 줍니다. 이것이 **단순 고정점 알고리즘**.
4. **정밀도·복잡도·부등식** (슬라이드 18~19): 최소 고정점이 "방정식의 가장 정밀한 해"지만 실제 의미보단 거칠 수 있음. 복잡도는 격자 높이에 비례. 부등식 제약은 등식으로 변환.

핵심 통찰: **분석 = 격자 위 단조 함수의 최소 고정점 계산**이고, **⊥에서 시작해 f를 반복하면(유한 높이) 종료가 보장**됩니다. 이것이 강의 1의 "종료·건전성" 목표가 격자+고정점으로 완성되는 순간이며, 강의 7~9의 데이터플로우 분석, 강의 18~20의 의미론·건전성 정리가 모두 이 위에 섭니다.

---

## 슬라이드 1: 제목 슬라이드

### 원문 내용
> Lattice Theory (2)
> CSE552 Program Analysis — Lecture 6
> Jaemin Hong

### 번역
> 격자 이론 (2) / CSE552 프로그램 분석 — 강의 6 / 홍재민

### 해설
격자 이론 2편. **고정점(fixed point)**으로 분석을 계산하는 법과 그 종료·정확성의 근거를 다룹니다.

---

## 슬라이드 2: Motivating Example — Sign⁶

### 원문 내용
> ```c
> 1 a = 42;
> 2 b = a + input();
> ```
> Abstract domain: Sign⁶
> - a0 = ⊤, b0 = ⊤, a1 = +, b1 = b0, a2 = a1, b2 = a1 + ⊤ (we will define + later)

### 번역
> 코드: `a=42; b=a+input()`. 추상 도메인 **Sign⁶**(지점 0·1·2 × 변수 a·b = 6개 부호).
> - 진입(0): a0=⊤, b0=⊤ (아무것도 모름)
> - 1행 후(1): a1=+(42), b1=b0(b 안 바뀜)
> - 2행 후(2): a2=a1, b2=a1+⊤ (`a + 임의값`의 부호)

### 해설

**개념 설명 — 흐름 감각 부호 분석의 첫 형태**

각 프로그램 지점(0·1·2)마다, 각 변수(a·b)의 부호를 따로 추적합니다 → 6개 부호값. 각 지점의 부호는 이전 지점과 그 줄의 연산으로 정해집니다(`a1=+` 등). 주목: 이 식들은 **이전 지점에 의존**하는 방정식 — 분석은 이 방정식들을 푸는 것입니다. 더 자연스러운 표기(맵 격자)가 슬3.

---

## 슬라이드 3: Motivating Example — (Var → Sign)³

### 원문 내용
> ```c
> a = 42; b = a + input();
> ```
> Abstract domain: (Var → Sign)³
> - x0 = [a ↦ ⊤, b ↦ ⊤]
> - x1 = x0[a ↦ +]
> - x2 = x1[b ↦ x1(a) + ⊤]

### 번역
> 같은 코드를 **맵 격자 (Var → Sign)³**로 표현(지점 3개 × 각 지점의 "변수→부호" 상태).
> - x0 = [a↦⊤, b↦⊤]
> - x1 = x0에서 a만 +로 갱신 (`x0[a↦+]`)
> - x2 = x1에서 b만 `x1(a)+⊤`로 갱신

### 해설

**개념 설명 — 상태 = 맵 격자 (강의 5 슬30 연결)**

강의 5의 **맵 격자 `Var→Sign`**(한 지점의 상태)와 그 곱(지점마다 하나)이 분석 상태입니다. `x0[a↦+]`는 "x0와 같되 a만 +로 바꾼 상태"(함수 갱신). 각 지점 상태가 이전 지점에서 그 줄의 효과를 적용해 나옵니다. 이것이 **흐름 감각 분석**(슬4) — 강의 2의 Sign⁶ 표기보다 깔끔. 정의가 슬4.

---

## 슬라이드 4: Flow-Sensitive Analysis

### 원문 내용
> (같은 코드·도메인)
> - Flow-sensitive analysis
>   - The order of statements is taken into account
>   - The sign of variables is determined for each program point

### 번역
> - **흐름 감각(flow-sensitive) 분석**:
>   - 문장의 **순서**를 고려한다
>   - 변수의 부호를 **각 프로그램 지점마다** 결정한다

### 해설

**개념 설명 — 흐름 감각 ★**

각 지점마다 별도 상태를 두므로, **문장 순서를 반영**합니다. 강의 4 슬18의 "흐름 무감각 타입 분석"(변수마다 단 하나의 타입)의 한계를 극복한 것 — 같은 변수도 지점마다 다른 부호를 가질 수 있습니다(`a`가 진입엔 ⊤, 1행 후엔 +). 강의 7~9의 데이터플로우 분석, 강의 15의 흐름 감각 포인터 분석이 모두 이 방식. 이 방정식을 어떻게 푸는지가 슬5.

---

## 슬라이드 5: Solving by Substitution

### 원문 내용
> (같은 방정식)
> - For this example program, each equation only depends on preceding ones
>   - The solution can be found by simple substitution
>   - x0 = [a↦⊤, b↦⊤]; x1 = [a↦+, b↦⊤]; x2 = [a↦+, b↦⊤]
> - In general, mutually recursive equations may appear, e.g., for programs that contain loops

### 번역
> - 이 예제는 각 방정식이 **앞선 것에만 의존** → **단순 대입(substitution)**으로 풀림
>   - x0=[a↦⊤,b↦⊤], x1=[a↦+,b↦⊤], x2=[a↦+,b↦⊤]
> - 그러나 일반적으로 **상호 재귀(mutually recursive) 방정식**이 나타남 — 특히 **루프**가 있는 프로그램에서

### 해설

**개념 설명 — 루프가 문제다 ★**

직선 코드는 방정식이 "앞 지점만" 참조하므로 위에서 아래로 대입하면 풀립니다. 그런데 **루프**가 있으면 방정식이 **순환**합니다 — 루프 헤드의 상태가 루프 본문(뒤쪽)에 의존하고, 본문은 다시 헤드에 의존(`x_head = ... x_body ...`, `x_body = ... x_head ...`). 단순 대입으로는 못 풉니다(닭과 달걀). 이 순환 방정식의 해가 **고정점**(슬6). 강의 2 슬33의 while 루프 MIR이 정확히 이 상황.

---

## 슬라이드 6: Fixed Point Formulation

### 원문 내용
> - Solving this system requires finding the fixed point for function f : (Var → Sign)³ → (Var → Sign)³ defined as follows:
>   - f(x0, x1, x2) = ([a↦⊤, b↦⊤], x0[a↦+], x1[b↦x1(a) + ⊤])
>   - A fixed point for f is x that satisfies f(x) = x
> - How can we find a fixed point for a function over a lattice?

### 번역
> - 이 방정식 시스템을 푸는 것은 함수 **f의 고정점**을 찾는 것: `f : (Var→Sign)³ → (Var→Sign)³`
>   - f가 모든 지점 상태를 한꺼번에 "한 번 갱신"하는 함수
>   - **고정점**: `f(x) = x`인 x (넣어도 그대로 나오는 상태)
> - 격자 위 함수의 고정점을 어떻게 찾을까?

### 해설

**개념 설명 — 분석 = 고정점 찾기 ★**

전체 방정식 시스템을 **하나의 함수 f**로 묶습니다: f는 "현재 모든 지점 상태 x를 받아, 한 번 갱신한 새 상태"를 줍니다. **고정점 `f(x)=x`**는 "더 갱신해도 안 변하는 상태" = 방정식의 해 = 분석 결과입니다.

즉 **정적 분석 = 전이 함수 f의 고정점 계산**. 이것이 이 강의의 핵심 프레임이고, 강의 7~9·18~20에서 계속 등장합니다. 고정점이 존재하고 계산 가능하려면 f가 **단조**여야 합니다(슬7).

---

## 슬라이드 7: Monotone Functions — Definition

### 원문 내용
> Definition (Monotone function). A function f : L1 → L2 where L1 and L2 are lattices is monotone (or order-preserving) when ∀x, y ∈ L1. x ⊑ y ⇒ f(x) ⊑ f(y)
> - From the analysis perspective, the intuition of monotonicity is that more precise input does not result in less precise output

### 번역
> **단조 함수(monotone function)**: `x ⊑ y ⇒ f(x) ⊑ f(y)` (순서를 보존)
> - 분석 관점 직관: **더 정밀한 입력이 덜 정밀한 출력을 내지 않는다**

### 해설

**개념 설명 — 단조성 ★**

**단조 함수**는 순서를 보존합니다: 입력이 커지면(부정밀해지면) 출력도 커지거나 같음. 분석적 직관: **"입력이 더 정밀하면 출력도 적어도 그만큼 정밀"** — 정보를 거꾸로 뒤집지 않음.

왜 중요한가? ① **고정점 존재**(Tarski, 슬14)와 ② **반복의 단조 증가**(Kleene, 슬15)가 단조성에 의존합니다. 비단조 함수는 고정점 반복이 진동·발산할 수 있습니다. 강의 18 슬17·19의 α·γ·전이 함수가 모두 단조여야 했던 이유가 여기 있습니다 — 단조성은 분석 정당화의 전제. 관련 함수 종류가 슬8.

---

## 슬라이드 8: Extensive and Distributive Functions

### 원문 내용
> Definition (Extensive function). f : L → L is extensive when ∀x ∈ L. x ⊑ f(x)
> Definition (Distributive function). f : L1 → L2 is distributive when ∀x, y. f(x) ⊔ f(y) = f(x ⊔ y)
> - Every distributive function is also monotone
> - Not every monotone function is also distributive

### 번역
> - **확장적(extensive) 함수**: `x ⊑ f(x)` (입력보다 출력이 크거나 같음 — 키우기만)
> - **분배적(distributive) 함수**: `f(x) ⊔ f(y) = f(x ⊔ y)` (join을 보존)
> - **분배적 ⇒ 단조** (분배적이면 단조), 하지만 **단조 ⇏ 분배적**

### 해설

**개념 설명 — 확장적·분배적 함수**

- **확장적**: 항상 위로 보냄(`x⊑f(x)`). **위드닝**(강의 9)이 이 성질 — 고정점보다 위로 보내 종료를 강제. 강의 19 슬22의 "γ∘α 확장적"도 같은 개념.
- **분배적**: join을 보존(`f(x⊔y)=f(x)⊔f(y)`). 강의 5 슬31의 준동형, 강의 18 슬29의 완전 join 사상이 이것. 분배적이면 "분기 합류 후 분석 = 각각 분석 후 합침"이 정확히 같아져 **정밀도 손실이 없음**(MOP=MFP, 강의 8 주제).

위계: **분배적 ⊂ 단조**. 분배적이면 단조지만 역은 아님(예: 곱셈 추상 연산은 단조지만 비분배적). 단조 함수의 성질이 슬9~11.

---

## 슬라이드 9: Monotone Functions — Properties (1)

### 원문 내용
> Important properties:
> - Every constant function is monotone
> - f is monotone ⟺ ∀x, y. f(x) ⊔ f(y) ⊑ f(x ⊔ y)
> - If f and g are monotone, then so is their composition g ∘ f
> - ⊔ : L² → L and ⊓ : L² → L are monotone

### 번역
> - **상수 함수는 단조**
> - `f 단조 ⟺ f(x)⊔f(y) ⊑ f(x⊔y)` (분배적의 부등식 버전 — 등식이 아니라 ⊑)
> - 단조 함수의 **합성도 단조**
> - **join(⊔)·meet(⊓) 자체가 단조**

### 해설

**개념 설명 — 단조성은 합성·조합에 닫혀 있다 ★**

핵심 성질: **단조 함수를 합성·조합해도 단조**입니다. 상수·join·meet이 단조이고, 단조 함수의 합성이 단조이므로, **이들로 만든 복잡한 전이 함수 f도 자동으로 단조**입니다(슬11에서 확인). 이 "단조성의 모듈성" 덕분에, 분석 설계자는 기본 조각이 단조임만 확인하면 전체 f의 단조성이 따라옵니다(강의 19의 "조각별 건전성 → 전체"와 같은 모듈성). 분배적 부등식(`f(x)⊔f(y)⊑f(x⊔y)`)이 단조성과 동치 — 분배는 등호, 단조는 ⊑. 더 많은 성질이 슬10.

---

## 슬라이드 10: Monotone Functions — Properties (2)

### 원문 내용
> Important properties (cont.):
> - If f : L1 → (A → L2) and g : L1 → L2 are monotone, then so is h : L1 → (A → L2) defined by h(x) = f(x)[a ↦ g(x)]
> - f1 : L → L1, ..., fn : L → Ln are monotone ⟺ f : L → L1 × ... × Ln defined by f(x) = (f1(x), ..., fn(x)) is monotone

### 번역
> - 맵 갱신 `h(x) = f(x)[a↦g(x)]`도 단조(f, g가 단조면)
> - 곱으로의 함수 `f(x)=(f1(x),...,fn(x))`는 **각 성분이 단조 ⟺ 전체가 단조**

### 해설

**개념 설명**

맵 갱신(`[a↦...]`)과 곱 함수도 단조성을 보존합니다. 즉 슬3의 `x1=x0[a↦+]` 같은 맵 갱신, 슬6의 곱 함수 `f(x0,x1,x2)=(...)`가 각 성분이 단조면 전체 단조. 이로써 분석 전이 함수가 단조임을 성분별로 쉽게 보일 수 있습니다(슬11). 예가 슬11.

---

## 슬라이드 11: Monotone Functions — Example

### 원문 내용
> - f(x0, x1, x2) = ([a↦⊤, b↦⊤], x0[a↦+], x1[b↦x1(a) + ⊤]) = (f0, f1, f2)
> - f0 is monotone because it is a constant function
> - f1 is monotone because (x0,x1,x2)↦x0 is monotone and (x0,x1,x2)↦+ is monotone
> - f2 is monotone (we will show it later)
> - f is monotone because f0, f1, and f2 are monotone

### 번역
> 슬6의 f를 세 성분(f0,f1,f2)으로 나눠 각각 단조임을 보임: f0(상수)·f1(투영+상수 갱신)·f2 모두 단조 → 곱 f도 단조(슬10).

### 해설

**개념 설명 — 단조성 증명의 실제**

슬9~10의 성질로 f의 단조성을 **조각별로** 증명합니다: f0은 상수(단조), f1은 투영(x0 꺼내기)과 상수 갱신의 조합(단조), f2도 마찬가지. 각 성분이 단조이므로 곱 f가 단조(슬10). 이렇게 **기본 단조 조각의 조합**으로 전이 함수의 단조성이 확립됩니다 — 강의 19의 건전성 증명과 같은 "분해" 전략. 이제 단조 f의 고정점을 봅니다(슬12).

---

## 슬라이드 12: Fixed Points — Definition

### 원문 내용
> Definition (Fixed point).
> - x ∈ L is a fixed point for f if f(x) = x
> - A least fixed point (lfp) x for f is a fixed point for f where x ⊑ y for every fixed point y for f

### 번역
> - **고정점**: `f(x)=x`인 x
> - **최소 고정점(lfp)**: 모든 고정점 y보다 작거나 같은(`x⊑y`) 고정점 — **가장 작은(가장 정밀한) 고정점**

### 해설

**개념 설명 — 최소 고정점(lfp) ★**

고정점은 여럿일 수 있습니다(슬13). 그중 **최소 고정점(lfp)**은 "모든 고정점보다 작은(=가장 정밀한)" 고정점입니다. 강의 5의 순서로, 작을수록 정밀(아래쪽). 분석에서는 **lfp가 정답** — "방정식을 만족하는 가장 정밀한 해"이기 때문(슬13). 강의 18~20에서 의미론·분석 결과를 모두 lfp로 정의했던 그 lfp입니다. 분석에서의 역할이 슬13.

---

## 슬라이드 13: Fixed Points — Role in Analysis

### 원문 내용
> Where the constraints are expressed as an equation system x = f(x),
> - A solution to the system is the same as a fixed point for f
> - For carefully designed constraints, every fixed point provides a sound result
> - Among all fixed points, the lfp provides the most precise result

### 번역
> 제약을 방정식 `x = f(x)`로 표현할 때:
> - 시스템의 해 = **f의 고정점**
> - 잘 설계된 제약이면 **모든 고정점이 건전한(sound) 결과**
> - 모든 고정점 중 **lfp가 가장 정밀한** 결과

### 해설

**개념 설명 — 왜 lfp인가 ★**

핵심 통찰 세 가지:
1. **분석의 해 = f의 고정점**(슬6).
2. **모든 고정점은 건전**(잘 설계된 제약이면) — 강의 18 슬12에서 "큰 고정점도 안전하지만 부정밀"이라 한 그것.
3. **lfp가 가장 정밀** — 군더더기 없이 도달 가능한 것만 담음.

따라서 분석은 **최소 고정점**을 목표로 합니다(건전하면서 가장 정밀). 강의 18~20의 `{|P|}=lfp(cf)`, `[|P|]=lfp(f)`가 이것. lfp의 존재가 슬14.

---

## 슬라이드 14: Tarski's Fixed Point Theorem

### 원문 내용
> Theorem (Tarski¹). If L is a complete lattice and f : L → L is monotone, then f has a least fixed point
> - The most precise solution is guaranteed to exist, but how can we find it?
>
> ¹ A lattice-theoretical fixpoint theorem and its applications (Tarski, 1955)

### 번역
> **Tarski 고정점 정리**: L이 **완비 격자**이고 f가 **단조**이면, f는 **최소 고정점을 가진다(존재 보장)**.
> - 가장 정밀한 해의 존재는 보장되나, 어떻게 찾을까?

### 해설

**개념 설명 — Tarski: lfp 존재 보장 ★**

**Tarski 정리**가 분석의 정당성을 떠받칩니다: **완비 격자 + 단조 함수 ⇒ 최소 고정점 존재**. 즉 "분석이 추구하는 가장 정밀한 해가 반드시 존재한다"는 보장. 두 전제가 강의 5(완비 격자)와 강의 6 슬7(단조)에서 준비됐습니다.

강의 19 슬21~22의 건전성 정리가 이 Tarski 정리(lfp = 후고정점들의 만남)를 핵심 도구로 썼습니다. 단 Tarski는 **존재만** 보장하고 **계산법**은 안 줍니다. 계산법이 Kleene 정리(슬15).

---

## 슬라이드 15: Kleene's Fixed Point Theorem

### 원문 내용
> Theorem (Kleene²). If L is a complete lattice with a finite height and f : L → L is monotone, then lfp(f) = ⨆_{i≥0} f^i(⊥)
> - If the lattice has a finite height, we can find the lfp by computing the increasing chain ⊥ ⊑ f(⊥) ⊑ f²(⊥) ⊑ ... until the fixed point is reached
>
> ² Introduction to metamathematics (Kleene, 1952)

### 번역
> **Kleene 고정점 정리**: L이 **유한 높이** 완비 격자이고 f가 단조이면, `lfp(f) = ⨆_{i≥0} f^i(⊥)`.
> - 유한 높이면 **증가 사슬 `⊥ ⊑ f(⊥) ⊑ f²(⊥) ⊑ ...`를 고정점에 도달할 때까지** 계산해 lfp를 구할 수 있다.

### 해설

**개념 설명 — Kleene: lfp 계산법 ★★**

**Kleene 정리**가 lfp를 **실제로 계산하는 법**을 줍니다: **⊥에서 시작해 f를 반복** — `⊥, f(⊥), f²(⊥), ...`. 단조성 덕분에 이 사슬은 **단조 증가**(⊑로 커짐)하고, **유한 높이**이면 유한 단계에서 멈춰 lfp에 도달합니다.

이것이 강의 1의 두 목표를 격자로 완성하는 순간:
- **종료**: 유한 높이 → 증가 사슬이 유한 → 멈춤(강의 5 슬24의 "유한 높이=종료"의 정리).
- **건전성**: lfp가 가장 정밀한 건전한 해(슬13).

높이 무한(구간 도메인)이면 이 반복이 안 멈출 수 있어 **위드닝**이 필요(강의 9). 이 반복이 곧 **데이터플로우 워크리스트 알고리즘**(강의 7~8)의 이론적 근거. 알고리즘 형태가 슬16.

---

## 슬라이드 16: Naive Fixed Point Algorithm

### 원문 내용
> ```
> NaiveFixedPointAlgorithm(f):
>   x ← ⊥
>   while x ≠ f(x):
>     x ← f(x)
>   return x
> ```
> (그림: ⊥에서 출발해 지그재그로 위로 올라가 lfp에 수렴)

### 번역
> **단순 고정점 알고리즘**: x를 ⊥로 시작, `x ≠ f(x)`인 동안 `x ← f(x)` 반복, 변화 없으면 반환. (⊥에서 출발해 고정점까지 단조 증가하며 수렴)

### 해설

**개념 설명 — Kleene 반복의 알고리즘 ★**

Kleene 정리(슬15)를 그대로 코드로: **⊥에서 시작해 f를 변화가 없을 때까지 반복**. 각 반복에서 x가 ⊑로 커지다가(단조), `x=f(x)`(고정점)에 도달하면 멈춤. 그림처럼 ⊥에서 lfp까지 올라갑니다.

이것이 모든 데이터플로우 분석의 **기본 골격**입니다 — 강의 7~8의 워크리스트, Assignment 4의 `find_fixed_point`(`old != widened`인 동안 반복)가 정확히 이 패턴. 단 "전체 x를 매번 다시 계산"은 비효율적이라, 실전에선 "바뀐 부분만 다시"(워크리스트, 강의 7)로 최적화. 예가 슬17.

---

## 슬라이드 17: Fixed Point Algorithm — Example

### 원문 내용
> - f(x0, x1, x2) = ([a↦⊤, b↦⊤], x0[a↦+], x1[b↦x1(a) + ⊤])
> - ⊥ = ([a↦⊥, b↦⊥], [a↦⊥, b↦⊥], [a↦⊥, b↦⊥])
> - f(⊥) = ([a↦⊤, b↦⊤], [a↦+, b↦⊥], [a↦⊥, b↦⊥])
> - f²(⊥) = ([a↦⊤, b↦⊤], [a↦+, b↦⊤], [a↦+, b↦⊤])
> - f³(⊥) = ([a↦⊤, b↦⊤], [a↦+, b↦⊤], [a↦+, b↦⊤]) = f²(⊥)

### 번역
> ⊥(모든 변수 ⊥)에서 f를 반복:
> - f(⊥): x0 채워짐(진입은 ⊤), x1의 a=+ 채워짐
> - f²(⊥): 정보가 더 전파되어 x1·x2 완성
> - f³(⊥) = f²(⊥) → **고정점 도달**, lfp = f²(⊥)

### 해설

**개념 설명 — 반복이 정보를 전파한다**

⊥(아무것도 모름)에서 시작해, f를 적용할 때마다 **정보가 한 단계씩 앞으로 전파**됩니다: 1회차에 진입과 1행, 2회차에 2행까지 채워지고, 3회차엔 변화 없음(고정점). 직선 코드라 2회 만에 수렴(루프가 있으면 더 걸림). 이 "정보 전파 반복"이 데이터플로우 분석의 실제 동작. 슬5의 단순 대입 결과와 같은 답에 도달하되, **루프가 있어도 작동**하는 일반적 방법입니다. 정밀도·복잡도 논의가 슬18.

---

## 슬라이드 18: Precision and Complexity

### 원문 내용
> - Even though we find the most precise possible solution to the equation system, the equation system is merely a conservative approximation of the actual program behavior
> - The semantically most precise answer can be below the lfp in the lattice
> - The time complexity of computing a fixed point with this algorithm depends on:
>   - The height of the lattice (bound for the number of iterations)
>   - The cost of computing f(x) and testing equality, performed in each iteration

### 번역
> - lfp는 **방정식 시스템의 가장 정밀한 해**지만, 방정식 자체가 실제 동작의 **보수적 근사**일 뿐
> - **의미적으로 가장 정밀한 답은 lfp보다 아래(더 정밀)일 수 있음**
> - 시간 복잡도: **격자 높이**(반복 횟수 상한) × **f(x) 계산·동치 검사 비용**(매 반복)

### 해설

**개념 설명 — lfp가 "최선"이 아닌 이유**

미묘한 점: lfp는 **"우리가 세운 방정식의" 가장 정밀한 해**이지, **실제 프로그램의** 가장 정밀한 답은 아닙니다. 방정식(추상 전이 함수)이 이미 근사라서, 실제 정답은 lfp보다 더 아래(더 정밀)일 수 있습니다. 이는 강의 8의 **MOP vs MFP**(경로별 vs 합류 후) 정밀도 격차, 강의 20의 "수집 의미론도 트레이스보다 거침"과 통합니다 — **추상화 단계마다 정밀도를 잃습니다**(강의 18 갈루아 연결의 정보 손실).

**복잡도**: 반복 횟수는 격자 **높이**에 비례(높이가 곧 ⊥→lfp 거리). 그래서 유한 높이가 종료·효율의 핵심. 높이가 크거나 무한이면 위드닝으로 줄임(강의 9). 부등식 처리가 슬19.

---

## 슬라이드 19: Inequality Constraints

### 원문 내용
> - Some analyses can yield inequations
> - We can rewrite them as equations
>   - x ⊒ f(x) is equivalent to x = x ⊔ f(x)
>   - x ⊑ f(x) is equivalent to x = x ⊓ f(x)

### 번역
> - 일부 분석은 **부등식 제약**을 낳음
> - 등식으로 변환 가능:
>   - `x ⊒ f(x)` ⟺ `x = x ⊔ f(x)`
>   - `x ⊑ f(x)` ⟺ `x = x ⊓ f(x)`

### 해설

**개념 설명 — 부등식을 등식으로**

제약이 등식(`x=f(x)`)이 아니라 부등식(`x ⊒ f(x)`, "x는 f(x)를 덮어야")일 때도 있습니다(강의 11·14의 `[y]⊆[x]` 같은 포함 제약). 이를 **등식으로 변환**: `x ⊒ f(x)`는 `x = x⊔f(x)`(x에 f(x)를 합쳐도 x). 그러면 고정점 알고리즘을 그대로 적용. 강의 11의 cubic, 강의 14의 Andersen이 포함 제약을 이렇게 고정점으로 푼 것. 전체 요약이 슬20.

---

## 슬라이드 20: Summary

### 원문 내용
> - Solving constraints can be formulated as finding a fixed point for a function over a lattice
> - Monotone functions on complete lattices have a least fixed point
> - The naive fixed point algorithm iterates f from ⊥ until convergence

### 번역
> - 제약 풀이 = **격자 위 함수의 고정점 찾기**
> - **완비 격자 위 단조 함수는 최소 고정점을 가짐**(Tarski)
> - **단순 고정점 알고리즘**은 ⊥에서 f를 반복해 수렴

### 해설

**전체 정리 — 강의 6의 한 장 요약**

1. **분석 = 고정점 계산**: 흐름 감각 분석은 연립방정식 `x=f(x)`가 되고(루프면 순환), 그 해가 f의 고정점(슬2~6).
2. **단조 함수**: `x⊑y ⇒ f(x)⊑f(y)`("정밀 입력→정밀 출력"). 합성·조합에 닫혀 있어 전이 함수가 자동 단조(슬7~11). 확장적(위드닝)·분배적(정밀 보존)도 등장.
3. **고정점 정리**: lfp가 정답(건전+가장 정밀). **Tarski**(완비+단조→lfp 존재), **Kleene**(유한 높이→`⊥,f(⊥),f²(⊥),...`로 계산)(슬12~17).
4. **종료·정밀도·복잡도**: 유한 높이=종료, 반복 횟수∝높이. lfp는 방정식의 최선이나 실제보단 거칠 수 있음. 부등식은 등식으로 변환(슬18~19).

**다른 강의와의 연결 (파일 간 연결성)**

- ← **강의 5**: 완비 격자·맵 격자·⊤·⊥·높이가 이 강의의 전제. 유한 높이=종료가 Kleene으로 정리화.
- ← **강의 4**: 흐름 무감각 타입 분석의 한계(슬4 흐름 감각으로 극복).
- → **강의 7~9 (데이터플로우)**: 워크리스트 알고리즘 = Kleene 반복의 효율화. JOIN=lub. 유한 높이/위드닝.
- → **강의 9 (위드닝)**: 무한 높이 격자에서 종료를 위해 확장적 위드닝(슬8) 사용.
- → **강의 18~20 (추상 해석)**: 의미론·분석 결과가 모두 lfp(`{|P|}=lfp(cf)`), 건전성 정리가 Tarski(후고정점)를 사용. 단조성이 전제.
- → **강의 11·14 (cubic·Andersen)**: 포함 제약(부등식)을 고정점으로 풂(슬19).

**가장 큰 교훈**: **정적 분석은 격자 위 단조 함수의 최소 고정점을 계산하는 것**입니다. Tarski가 그 답(lfp)의 존재를, Kleene이 계산법(⊥에서 f 반복)을 보장하며, **유한 높이가 종료를** 줍니다. 이로써 강의 1의 "종료·건전성" 목표가 격자+고정점으로 완성됩니다 — 이 프레임이 강의 7~9의 데이터플로우와 강의 18~20의 추상 해석 전체를 떠받칩니다.

---

## 마치며

강의 6은 강의 5의 격자 위에서 **"분석을 어떻게 계산하고 왜 끝나는가"**에 답합니다. 핵심 한 줄: **"흐름 감각 분석은 방정식 `x=f(x)`가 되고, 단조 함수 f의 최소 고정점이 그 답이며(Tarski 존재 보장), 유한 높이 격자에서 ⊥부터 f를 반복하면(Kleene) 종료가 보장된다."** 이 고정점 프레임은 데이터플로우(강의 7~9)와 추상 해석(강의 18~20)의 공통 엔진입니다. 시험에서는 (a) 흐름 감각 분석이 왜 고정점 문제가 되는가(루프=순환 방정식, 슬5~6), (b) 단조성의 정의와 분석적 의미·합성 보존(슬7·9), (c) Tarski(존재) vs Kleene(계산)의 역할 구분(슬14~15), (d) `⊥,f(⊥),f²(⊥),...` 반복 추적(슬16~17), (e) 유한 높이와 종료·복잡도의 관계, 부등식→등식 변환(슬18~19)이 단골입니다.
