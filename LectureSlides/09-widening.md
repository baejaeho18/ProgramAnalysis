# Widening - 강의 슬라이드 해설

CSE552 Program Analysis — Lecture 9

---

## 슬라이드 1: Widening

### 원문 내용

> Widening
> CSE552 Program Analysis — Lecture 9
> Jaemin Hong

### 해설

**강의 개요**

이 강의는 프로그램 분석에서 무한 높이(infinite height)를 가진 격자(lattice)에서의 고정점(fixed point) 계산 문제를 해결하는 핵심 기법인 확대(Widening)를 다룬다. 프로그램 분석은 프로그램의 모든 가능한 실행 경로를 추상화하여 분석하는데, 반복문과 재귀가 있으면 고정점 계산이 수렴하지 않을 수 있다. 이를 해결하기 위한 기법이 바로 widening이다.

**강의 목표**

- 무한 높이 격자에서의 고정점 계산의 문제점 이해
- Widening 연산자의 정의와 작동 원리 학습
- Widening을 이용한 수렴 보장 알고리즘 구현
- Widening의 한계와 이를 보완하는 narrowing 기법 학습
- 실제 구간 분석(interval analysis) 예제를 통한 이해

---

## 슬라이드 2: Interval Analysis

### 원문 내용

> Interval Analysis
>
> - An interval analysis computes a lower and an upper bound for the possible values of each integer variable
> - Sound answers can be used for optimizations and bug detection related to array bounds checking, numerical overflows, and integer representations
> - Involves a lattice of infinite height

### 해설

**개념 설명**

구간 분석(Interval Analysis)은 정수 변수가 가질 수 있는 모든 가능한 값들을 구간 형태로 추상화하여 분석하는 기법이다. 예를 들어, 변수 x가 0부터 10 사이의 값을 가질 수 있다면 이를 [0, 10]으로 표현한다. 이는 구체적인 값들의 무한한 집합을 유한한 표현으로 압축한다.

**주요 특징**

1. **상한과 하한(Lower and Upper Bound)**: 각 정수 변수에 대해 가질 수 있는 최솟값과 최댓값을 추적한다. 예를 들어 [2, 8]은 변수가 2 이상 8 이하의 값을 가진다는 뜻이다.

2. **건전성(Soundness)**: 구간 분석의 결과는 항상 프로그램의 실제 동작을 포함한다. 즉, 실제 변수 값이 분석 결과 구간 밖에 있을 수 없다.

**응용 분야**

- **배열 경계 검사(Array Bounds Checking)**: 배열 접근 인덱스가 배열 크기를 벗어나지 않는지 검증
- **숫자 오버플로우 감지**: 정수 연산 결과가 정수 표현 범위를 벗어나는지 감지
- **정수 표현 검증**: 비트 길이가 부족하지 않은지 확인

**무한 높이 격자의 문제**

구간의 범위가 무한할 수 있으므로 (예: [-∞, 5], [10, ∞]) 격자의 높이가 무한해진다. 이는 고정점 계산 반복에서 수렴이 보장되지 않음을 의미한다.

---

## 슬라이드 3: Interval Domain — Definitions

### 원문 내용

> Interval Domain — Definitions
>
> Z = Z ∪ {-∞, ∞}
> Interval = {[l, h] | l, h ∈ Z ∧ l ≤ h} ∪ {⊥}
> [l₁, h₁] ⊑ [l₂, h₂] ⟺ l₂ ≤ l₁ ∧ h₁ ≤ h₂
> ⊤ = [-∞, ∞]

### 해설

**개념 설명**

이 슬라이드는 구간 도메인의 형식적 정의를 제시한다.

1. **확장된 정수(Z)**: 일반 정수 집합에 무한대 기호 -∞와 ∞를 추가한다. 이를 통해 제한 없는 범위를 표현할 수 있다.

2. **구간(Interval)**:
   - l ≤ h를 만족하는 [l, h] 형태의 구간
   - ⊥(bottom): 불가능한 값을 나타내는 최하단 원소 (프로그램이 도달 불가능한 경로를 의미)
   - 예: [3, 7], [-∞, 5], [10, ∞], [-∞, ∞]

3. **부분순서(⊑)**: 구간 간의 근사 관계 정의
   - [l₁, h₁] ⊑ [l₂, h₂]는 "l₂ ≤ l₁ ∧ h₁ ≤ h₂"를 의미
   - 직관: [l₁, h₁]이 [l₂, h₂]보다 정확하다(더 좁다)는 뜻
   - 예: [3, 5] ⊑ [0, 10] (왼쪽이 더 정확)

4. **⊤ (Top)**: 가장 덜 정확한 구간으로 모든 가능한 값을 포함한다.

**직관적 이해**

부분순서 정의에서 범위가 좁을수록 더 많은 정보를 담고 있으므로 순서 관계에서 위에 있다고 생각한다. 예를 들어:
- [5, 5] ⊑ [4, 6] ⊑ [0, 10] ⊑ [-∞, ∞]

---

## 슬라이드 4: Interval Domain — Join and Meet

### 원문 내용

> Interval Domain — Join and Meet
>
> [l₁, h₁] ∪ [l₂, h₂] = [min(l₁, l₂), max(h₁, h₂)]
>
> [l₁, h₁] ⊓ [l₂, h₂] = {
>   [max(l₁, l₂), min(h₁, h₂)]  if max(l₁, l₂) ≤ min(h₁, h₂)
>   ⊥                           otherwise
> }

### 해설

**개념 설명**

Join(⊔)과 Meet(⊓) 연산은 격자 이론의 핵심 연산이다. 프로그램 분석에서는 여러 실행 경로의 상태를 합치거나 교집합을 취할 때 사용된다.

**Join 연산 (∪)**

- 정의: [l₁, h₁] ∪ [l₂, h₂] = [min(l₁, l₂), max(h₁, h₂)]
- 의미: 두 구간을 포함하는 가장 작은 구간을 구한다 (상한)
- 예시:
  - [1, 5] ∪ [8, 10] = [1, 10]
  - [2, 4] ∪ [3, 6] = [2, 6]
  - [0, 5] ∪ [-∞, 3] = [-∞, 5]
- 역할: 제어 흐름의 여러 경로가 합쳐질 때 두 상태 정보를 합친다

**Meet 연산 (⊓)**

- 정의: 두 구간의 교집합을 구한다
- 만약 교집합이 존재하면: [max(l₁, l₂), min(h₁, h₂)]
- 교집합이 없으면 (max(l₁, l₂) > min(h₁, h₂)): ⊥
- 예시:
  - [1, 5] ⊓ [3, 7] = [3, 5]
  - [1, 5] ⊓ [6, 10] = ⊥ (교집합 없음)
  - [0, 10] ⊓ [5, ∞] = [5, 10]
- 역할: 조건문의 true/false 브랜치를 분석할 때 추가 정보를 반영한다

**중요성**

이 연산들이 정확하게 정의되어야 프로그램 분석 알고리즘이 올바르게 작동한다.

---

## 슬라이드 5: Interval Domain — Hasse Diagram

### 원문 내용

> Interval Domain — Hasse Diagram
>
> [다양한 구간들의 계층 구조를 보여주는 다이어그램]
> 최상단: [-∞, ∞]
> 중간: [-∞, 0], [-2, 2], [1, ∞] 등
> 최하단: [0, 0], [1, 1] 등

### 해설

**개념 설명**

Hasse 다이어그램은 부분순서 관계를 시각화하는 그래프이다. 이를 통해 격자 구조를 직관적으로 이해할 수 있다.

**다이어그램 해석**

- **노드**: 각 구간을 나타냄
- **방향선**: 부분순서 관계를 나타냄
  - 아래에서 위로의 화살표는 ⊑ 관계를 의미
  - a에서 b로 가는 경로가 있으면 a ⊑ b
- **최상단 (⊤)**: [-∞, ∞] - 모든 구간을 포함
- **최하단 (⊥)**: 공집합 - 어떤 구간에도 포함됨

**주요 특징**

1. **무한 높이**: 다이어그램의 높이가 무한하다
   - 예: [0, 0] ⊑ [0, 1] ⊑ [0, 2] ⊑ ... ⊑ [0, ∞]
   - 이 체인이 무한히 계속된다

2. **폭넓은 구조**: 각 높이에 무한히 많은 구간이 존재할 수 있다

3. **고정점 계산의 문제**: 이 무한 구조 때문에 단순한 반복적 고정점 계산이 수렴하지 않을 수 있다

---

## 슬라이드 6: Interval Domain — Infinite Height

### 원문 내용

> Interval Domain — Infinite Height
>
> [0, 0] ⊑ [0, 1] ⊑ [0, 2] ⊑ [0, 3] ⊑ [0, 4] ⊑ [0, 5] ⊑ ···

### 해설

**개념 설명**

이 슬라이드는 구간 도메인이 무한 높이를 가지는 이유를 명확히 보여준다.

**무한 체인의 예**

- [0, 0] ⊑ [0, 1] ⊑ [0, 2] ⊑ [0, 3] ⊑ ...
  - 각 구간은 점점 더 넓어진다
  - 상한(upper bound)이 계속 증가한다
  - 이 체인이 무한히 계속된다

**왜 문제인가?**

프로그램 분석에서 고정점을 구할 때, 반복마다 더 큰 구간을 얻을 수 있다면:
- x₀ = [0, 0]
- x₁ = [0, 1]
- x₂ = [0, 2]
- x₃ = [0, 3]
- ...

이 수열은 절대 수렴하지 않는다. 따라서 유한 시간 내에 고정점을 찾을 수 없다.

**해결책**

이 문제를 해결하기 위해 widening이 필요하다. Widening은 일부러 더 큰 근사값으로 점프하여 수렴을 강제한다.

---

## 슬라이드 7: Transfer Functions

### 원문 내용

> Transfer Functions
>
> State = Var → Interval
>
> x = e :    t_v(σ) = σ[x ↦ eval(σ, e)]
> if x :     t_v(σ) = σ
> return :   t_v(σ) = σ
>
> σ_start = ⊤

### 해설

**개념 설명**

Transfer Function은 프로그램의 각 연산이 추상 상태(abstract state)를 어떻게 변환하는지 정의한다.

**상태(State) 정의**

- State = Var → Interval
- 각 변수가 가질 수 있는 값의 범위를 매핑하는 함수
- 예: σ(x) = [0, 10], σ(y) = [5, 15]는 "x는 0~10, y는 5~15 범위의 값"을 의미

**Transfer Function들**

1. **할당문 (x = e)**
   - t_v(σ) = σ[x ↦ eval(σ, e)]
   - 변수 x에 식 e의 평가 결과를 할당한다
   - 다른 변수들은 변경하지 않는다
   - 예: σ(x) = [0, 5], e = x + 2일 때 eval(σ, e) = [2, 7]이므로 결과는 σ[x ↦ [2, 7]]

2. **조건문 (if x)**
   - t_v(σ) = σ
   - 조건 자체는 상태를 변경하지 않는다
   - 나중에 제어 흐름 민감성(control sensitivity)에서 더 정교한 버전이 나온다

3. **return 문**
   - t_v(σ) = σ
   - 상태를 변경하지 않는다

**초기 상태**

σ_start = ⊤ = [-∞, ∞]는 프로그램 시작 시 모든 변수가 임의의 값을 가질 수 있음을 의미한다.

---

## 슬라이드 8: Abstract Evaluation

### 원문 내용

> Abstract Evaluation
>
> eval(σ, x) = σ(x)
> eval(σ, n) = [n, n]
> eval(σ, input()) = ⊤ = [-∞, ∞]
> eval(σ, e₁ op e₂) = ⊕op(eval(σ, e₁), eval(σ, e₂))

### 해설

**개념 설명**

Abstract Evaluation은 구간 도메인에서 식(expression)의 값을 평가하는 방법을 정의한다.

**평가 규칙들**

1. **변수 평가: eval(σ, x)**
   - 현재 상태 σ에서 변수 x의 구간을 반환한다
   - 예: σ(x) = [3, 5]이면 eval(σ, x) = [3, 5]

2. **상수 평가: eval(σ, n)**
   - 상수 n을 정확한 값 [n, n]으로 표현한다
   - 예: eval(σ, 7) = [7, 7], eval(σ, -3) = [-3, -3]

3. **입력 평가: eval(σ, input())**
   - 사용자 입력은 임의의 값이 될 수 있으므로 ⊤ = [-∞, ∞]를 반환한다
   - 즉, 입력은 정수 범위 전체를 가능한 값으로 본다

4. **이항 연산: eval(σ, e₁ op e₂)**
   - 양쪽 식을 먼저 평가하여 구간을 얻는다
   - 그 후 추상 연산 ⊕op를 적용한다
   - 예: eval(σ, x + 3)에서 σ(x) = [2, 5]이면
     - eval(σ, x) = [2, 5]
     - eval(σ, 3) = [3, 3]
     - ⊕+(eval(σ, x), eval(σ, 3)) = [2, 5] + [3, 3] = [5, 8]

**평가의 건전성**

추상 평가는 건전해야 한다. 즉, 실제 값의 범위가 항상 평가 결과 구간에 포함되어야 한다.

---

## 슬라이드 9: Abstract Operations

### 원문 내용

> Abstract Operations
>
> ⊕op([l₁, h₁], [l₂, h₂]) = [min(x op y), max(x op y)]
>                           x∈[l₁,h₁]        x∈[l₁,h₁]
>                           y∈[l₂,h₂]        y∈[l₂,h₂]
>
> ⊕+([l₁, h₁], [l₂, h₂]) = [l₁ + l₂, h₁ + h₂]
> ⊕-([l₁, h₁], [l₂, h₂]) = [l₁ - h₂, h₁ - l₂]

### 해설

**개념 설명**

Abstract Operation은 구간 값들 사이의 연산을 정의한다. 일반 정수 연산을 구간으로 일반화하는 것이다.

**일반적 정의**

⊕op([l₁, h₁], [l₂, h₂]) = [min(x op y), max(x op y)]

의미:
- 첫 번째 구간의 모든 값과 두 번째 구간의 모든 값에 대해 연산을 수행한다
- 그 결과들의 최솟값과 최댓값을 구한다
- 이렇게 하면 실제 연산 결과가 항상 포함된다 (건전성)

**구체적 예시**

1. **덧셈: ⊕+([l₁, h₁], [l₂, h₂]) = [l₁ + l₂, h₁ + h₂]**
   - 직관: 가장 작은 값들을 더하면 최솟값, 가장 큰 값들을 더하면 최댓값
   - 예: [2, 5] + [3, 7] = [2+3, 5+7] = [5, 12]
   - 검증: 2+3=5, 5+7=12, 2+7=9, 5+3=8 → 범위는 [5, 12] ✓

2. **뺄셈: ⊕-([l₁, h₁], [l₂, h₂]) = [l₁ - h₂, h₁ - l₂]**
   - 직관: l₁에서 가장 큰 값(h₂)을 빼면 최솟값, h₁에서 가장 작은 값(l₂)을 빼면 최댓값
   - 예: [5, 8] - [2, 3] = [5-3, 8-2] = [2, 6]
   - 검증: 5-3=2, 8-2=6, 5-2=3, 8-3=5 → 범위는 [2, 6] ✓

**곱셈이 복잡한 이유**

일반적으로 곱셈은 더 복잡하다. [l₁, h₁] × [l₂, h₂]의 결과는 4개 경우의 곱셈 중 최솟값과 최댓값을 찾아야 한다:
- l₁ × l₂, l₁ × h₂, h₁ × l₂, h₁ × h₂

음수를 포함하면 더욱 복잡해진다.

---

## 슬라이드 10: Interval Analysis Example (1)

### 원문 내용

> Interval Analysis Example (1)
>
> // x: [-inf, inf]
> if input() {
>   // x: [-inf, inf]
>   x = 0;
>   // x: [0, 0]
> } else {
>   // x: [-inf, inf]
>   x = 2;
>   // x: [2, 2]
> }
> // x: [0, 2]
> y = x + 2;
> // y: [2, 4]

### 해설

**단계별 분석**

이 예제는 조건문이 있을 때 구간 분석이 어떻게 작동하는지 보여준다.

**1단계: 프로그램 시작**
- x: [-∞, ∞] (초기 상태)

**2단계: if 조건 평가**
- 두 분기(branch)로 나뉜다
- 두 분기 모두 진입 가능하다고 가정

**3단계: if-true 분기**
- 입력 조건이 참인 경우
- x = 0 할당
- x: [0, 0] (정확한 값)

**4단계: if-false 분기**
- 입력 조건이 거짓인 경우
- x = 2 할당
- x: [2, 2] (정확한 값)

**5단계: 두 분기 병합**
- if-true 분기의 x: [0, 0]
- if-false 분기의 x: [2, 2]
- Join 연산: [0, 0] ⊔ [2, 2] = [0, 2]
- 결과: x: [0, 2]

**6단계: 후속 할당**
- y = x + 2
- eval(σ, x) = [0, 2]
- eval(σ, 2) = [2, 2]
- ⊕+([0, 2], [2, 2]) = [0+2, 2+2] = [2, 4]
- 결과: y: [2, 4]

**의미**

분석은 건전하다: y가 실제로 가질 수 있는 값은 2 또는 4이고, [2, 4]는 이를 포함한다.

---

## 슬라이드 11: Interval Analysis Example (2)

### 원문 내용

> Interval Analysis Example (2)
>
> // x: [-inf, inf]
> if x < 0 {
>   // x: [-inf, inf]
>   y = -x;
>   // y: [-inf, inf]
> } else {
>   // x: [-inf, inf]
>   y = x;
>   // y: [-inf, inf]
> }
> // y: [-inf, inf]

### 해설

**개념 설명**

이 예제는 기본 구간 분석의 한계를 보여준다. 조건문의 정보가 충분히 활용되지 않는다.

**단계별 분석**

**1단계: 프로그램 시작**
- x: [-∞, ∞]

**2단계: if x < 0 조건**
- 기본 구간 분석은 조건을 무시한다
- 두 분기 모두 x: [-∞, ∞]로 유지

**3단계: if-true 분기 (x < 0인 경우)**
- y = -x 할당
- eval(σ, -x) = -eval(σ, x) = -[-∞, ∞] = [-∞, ∞]
- 결과: y: [-∞, ∞]

**4단계: if-false 분기 (x ≥ 0인 경우)**
- y = x 할당
- eval(σ, x) = [-∞, ∞]
- 결과: y: [-∞, ∞]

**5단계: 두 분기 병합**
- if-true: y: [-∞, ∞]
- if-false: y: [-∞, ∞]
- Join: [-∞, ∞] ⊔ [-∞, ∞] = [-∞, ∞]
- 결과: y: [-∞, ∞]

**문제점**

조건 x < 0의 정보가 전혀 사용되지 않았다. 실제로는:
- x < 0인 분기: y = -x > 0
- x ≥ 0인 분기: y = x ≥ 0
- 따라서 y는 항상 ≥ 0이어야 한다

하지만 기본 분석은 y: [-∞, ∞]를 반환한다. 이를 개선하는 것이 제어 흐름 민감성(Control Sensitivity)이다.

---

## 슬라이드 12: Control Sensitivity — Transfer Functions

### 원문 내용

> Control Sensitivity — Transfer Functions
>
> We can exploit the information available in conditionals
>
> t_v : State → P(State × Node)
>
> x = e :     t_v(σ) = {(σ[x ↦ eval(σ, e)], succ(v))}
> if x ≤ n :  t_v(σ) = {(σ[x ↦ filter_≤(σ(x), n)], true(v)),
>                       (σ[x ↦ filter_>(σ(x), n)], false(v))}
> return :    t_v(σ) = ∅

### 해설

**개념 설명**

제어 민감성은 조건문에서 조건이 참 또는 거짓일 때의 제약을 상태에 반영한다.

**Transfer Function 재정의**

기본 transfer function은 각 명령어에서 상태를 단순히 변환한다. 하지만 제어 민감성에서는:
- 반환값이 (State, Node) 쌍의 집합이다
- Node는 실행이 이동할 다음 프로그램 위치이다
- 이를 통해 true/false 분기를 명시적으로 구분한다

**구체적 규칙들**

1. **할당문 (x = e)**
   - 기본과 동일하게 상태를 변환한다
   - 다음 위치는 succ(v) (다음 명령)

2. **조건문 (if x ≤ n)**
   - 두 개의 (State, Node) 쌍을 반환한다
   - True 분기: σ[x ↦ filter_≤(σ(x), n)]
     - x의 범위를 n 이하로 필터링한다
     - 다음 위치: true(v)
   - False 분기: σ[x ↦ filter_>(σ(x), n)]
     - x의 범위를 n 초과로 필터링한다
     - 다음 위치: false(v)

3. **return 문**
   - 다음 위치가 없으므로 공집합을 반환한다

**전략**

σ[x ↦ filter_rel(σ(x), n)]는:
- 현재 상태 σ에서
- 변수 x만 수정한다
- x의 범위를 조건에 맞게 좁힌다(filter)
- 다른 변수는 그대로 둔다

---

## 슬라이드 13: Control Sensitivity — Filter Functions

### 원문 내용

> Control Sensitivity — Filter Functions
>
> filter_≤([l, h], n) = [l, h] ⊓ [-∞, n]
> filter_>([l, h], n) = [l, h] ⊓ [n + 1, ∞]

### 해설

**개념 설명**

Filter Function은 조건 정보를 이용해 구간을 좁혀주는 연산이다.

**필터링 규칙**

1. **filter_≤([l, h], n)**
   - 조건: x ≤ n이 참인 경우
   - 현재 구간 [l, h]와 [-∞, n]의 교집합을 구한다
   - 결과: [max(l, -∞), min(h, n)] = [l, min(h, n)]
   - 예: filter_≤([5, 15], 10) = [5, 15] ⊓ [-∞, 10] = [5, 10]

2. **filter_>([l, h], n)**
   - 조건: x > n인 경우
   - 현재 구간 [l, h]와 [n+1, ∞]의 교집합을 구한다
   - 결과: [max(l, n+1), min(h, ∞)] = [max(l, n+1), h]
   - 예: filter_>([5, 15], 10) = [5, 15] ⊓ [11, ∞] = [11, 15]

**필터링의 작용**

필터링은 조건의 참/거짓 여부에 따라 불가능한 값들을 제외한다:
- x ≤ 10이 참이면, 10보다 큰 값은 불가능하다
- x > 10이 참이면, 10 이하의 값은 불가능하다

**건전성**

필터 함수는 항상 건전하다:
- 조건을 만족하는 실제 값들은 필터 결과에 포함된다
- 조건을 만족하지 않는 값들은 제외된다

---

## 슬라이드 14: Control Sensitivity — Propagation Algorithm

### 원문 내용

> Control Sensitivity — Propagation Algorithm
>
> PropagationWorkListAlgorithm(t₁, . . . , tₙ, σ_start):
>   (x₁, x₂, . . . , xₙ) ← (σ_start, ⊥, . . . , ⊥)
>   W ← {v₁, . . . , vₙ}
>   while W ≠ ∅ :
>     vᵢ ← W.removeOne()
>     Y ← t_v(xᵢ)
>     for (y, vⱼ) ∈ Y :
>       z ← xⱼ ∪ y
>       if xⱼ ≠ z :
>         xⱼ ← z
>         W.add(vⱼ)
>   return x

### 해설

**개념 설명**

이 알고리즘은 제어 흐름 민감성을 사용하여 프로그램의 모든 위치에서의 상태를 계산한다.

**알고리즘 구성**

**초기화**
- (x₁, x₂, . . . , xₙ): 각 위치에서의 상태 저장
- x₁ = σ_start: 첫 위치는 초기 상태
- 나머지는 ⊥ (정보 없음)
- W: 처리해야 할 위치들의 Worklist

**Main Loop**

```
while W ≠ ∅:
  1. vᵢ ← W.removeOne()     // worklist에서 하나 제거
  2. Y ← t_v(xᵢ)            // transfer function 적용
  3. for (y, vⱼ) ∈ Y:       // 결과의 각 (상태, 다음위치) 쌍에 대해
     4. z ← xⱼ ∪ y          // 기존 상태와 join
     5. if xⱼ ≠ z:          // 변화가 있으면
        6. xⱼ ← z           // 상태 업데이트
        7. W.add(vⱼ)        // 다음 위치를 worklist에 추가
```

**동작 메커니즘**

1. Worklist 기반 처리로 필요한 위치만 반복 처리
2. 상태가 변할 때만 다음 위치를 worklist에 추가
3. 모든 상태가 안정화되면 (worklist가 비면) 종료
4. 최종 결과 x는 각 위치에서의 도달 가능한 상태를 나타냄

**수렴 보장**

격자가 유한 높이를 가지면, 상태가 계속 증가(⊑ 관계에서)할 수 없으므로 알고리즘이 반드시 종료된다.

---

## 슬라이드 15: Control Sensitivity — Example

### 원문 내용

> Control Sensitivity — Example
>
> // x: [-inf, inf]
> if x < 0 {
>   // x: [-inf, 0]
>   y = -x;
>   // y: [0, inf]
> } else {
>   // x: [1, inf]
>   y = x;
>   // y: [1, inf]
> }
> // y: [0, inf]

### 해설

**단계별 분석**

이제 제어 민감성을 사용하면 더 정확한 결과를 얻는다.

**1단계: 초기 상태**
- x: [-∞, ∞]

**2단계: if x < 0 조건 평가**
- True 분기: x < 0이 참
  - filter_<([-∞, ∞], 0) = [-∞, ∞] ⊓ [-∞, -1] = [-∞, -1]
  - x: [-∞, -1]
- False 분기: x ≥ 0이 참
  - filter_≥([-∞, ∞], 0) = [-∞, ∞] ⊓ [0, ∞] = [0, ∞]
  - x: [0, ∞]

(슬라이드는 x: [1, ∞]로 표시했으므로, 조건이 x ≤ 0이 아니라 x < 0이고 더 정확한 필터링을 하는 것으로 보임)

**3단계: True 분기에서 y 계산**
- y = -x
- eval(σ, -x)에서 x ∈ [-∞, -1]
- -x ∈ [1, ∞]
- y: [1, ∞] (또는 근사적으로 [0, ∞])

**4단계: False 분기에서 y 계산**
- y = x
- x ∈ [0, ∞]이므로
- y: [0, ∞]

**5단계: 두 분기 병합**
- True: y: [1, ∞]
- False: y: [0, ∞]
- Join: [1, ∞] ⊔ [0, ∞] = [0, ∞]
- 결과: y: [0, ∞]

**정확성 향상**

기본 분석과 비교:
- 기본: y: [-∞, ∞] (부정확)
- 제어 민감: y: [0, ∞] (더 정확)

조건 정보 덕분에 y가 항상 음이 아님을 알 수 있다.

---

## 슬라이드 16: Fixed Point — Tasuki's Theorem

### 원문 내용

> Fixed Point — Tasuki's Theorem
>
> Theorem (Tasuki¹)
> If L is a complete lattice and f : L → L is monotone, then f has a
> least fixed point, given by
>
>   lfp(f) = ⊓ {x ∈ L | f(x) ⊑ x}
>
> - However, we may not find the lfp within finite iterations
> - If f(x) ⊑ x (x is a pre-fixed point), then lfp(f) ⊑ x

### 해설

**개념 설명**

Tarski의 정리는 고정점 이론의 기초이다. 프로그램 분석에서 고정점 존재를 보장한다.

**주요 용어**

1. **완전 격자(Complete Lattice)**: 모든 부분집합이 상한(supremum)과 하한(infimum)을 가지는 격자

2. **단조 함수(Monotone Function)**: x ⊑ y이면 f(x) ⊑ f(y)를 만족하는 함수

3. **최소 고정점(Least Fixed Point)**: 모든 고정점 중 가장 작은 (⊑ 관계에서) 고정점

4. **고정점(Fixed Point)**: f(x) = x를 만족하는 x

**정리의 의미**

Tarski의 정리는 다음을 보장한다:
- 완전 격자에서 단조 함수는 최소 고정점을 가진다
- 최소 고정점은 lfp(f) = ⊓{x ∈ L | f(x) ⊑ x}로 계산할 수 있다
- 즉, "f(x) ⊑ x를 만족하는 모든 x"의 교집합

**문제점**

실제로 lfp(f)를 계산하는 것은 어렵다:
- 조건을 만족하는 모든 x를 찾아야 한다
- 격자가 무한하면 이는 불가능하다

**Pre-fixed Point**

f(x) ⊑ x를 만족하는 x를 pre-fixed point라 한다:
- lfp(f) ⊑ x를 만족한다 (최소 고정점의 정의상)
- 반복적 고정점 계산 (x_{i+1} = f(x_i))에서 x_i ⊑ x_{i+1}이 되다 어느 시점에 수렴하면, 그 x_i는 pre-fixed point이다

---

## 슬라이드 17: Fixed Point — Example

### 원문 내용

> Fixed Point — Example
>
> x = 0;
> // [0, 0]
> while input() {
>   // [0, 1]
>   x = x + 1;
>   // [1, 1]
> }
>
> =>
>
> x = 0;
> // [0, 0]
> while input() {
>   // [0, 1]
>   x = x + 1;
>   // [1, 2]
> }
>
> ...

### 해설

**개념 설명**

이 예제는 무한 높이 격자에서 단순한 반복이 수렴하지 않는 문제를 보여준다.

**초기 상태**

```
x = 0;         // [0, 0]
while input() {
  x = x + 1;
}
```

**첫 번째 반복**

1. x = 0: x ∈ [0, 0]
2. while 조건: 반복 가능 (input() 반환값 알 수 없음)
3. while 본문 시작: x ∈ [0, 0]
4. x = x + 1: x ∈ [0, 0] + [1, 1] = [1, 1]
5. while로 돌아옴

**while 루프 포인트에서의 상태**

- 첫 진입: x ∈ [0, 0]
- 반복 후: x ∈ [1, 1]
- Join: [0, 0] ⊔ [1, 1] = [0, 1]

**두 번째 반복**

1. while에 도착: x ∈ [0, 1]
2. while 본문: x = x + 1 = [0, 1] + [1, 1] = [1, 2]
3. while로 돌아옴
4. Join: [0, 1] ⊔ [1, 2] = [0, 2]

**세 번째 반복**

1. while에 도착: x ∈ [0, 2]
2. while 본문: x = x + 1 = [0, 2] + [1, 1] = [1, 3]
3. Join: [0, 2] ⊔ [1, 3] = [0, 3]

**패턴**

- 반복 0: [0, 0]
- 반복 1: [0, 1]
- 반복 2: [0, 2]
- 반복 3: [0, 3]
- ...
- 반복 n: [0, n]

**문제**

상한이 계속 증가하므로:
- [0, 0] ⊑ [0, 1] ⊑ [0, 2] ⊑ ...
- 이 체인은 무한하다
- 유한 시간 내에 수렴하지 않는다
- 따라서 일반적인 fixed point 계산은 무한히 반복된다

**해결책**

이 문제를 해결하기 위해 widening이 필요하다. Widening은 어느 시점에 [0, ∞]로 "점프"하여 상한을 무한대로 설정함으로써 수렴을 강제한다.

---

## 슬라이드 18: Widening — Definition

### 원문 내용

> Widening — Definition
>
> - Widening² ensures termination even when the lattice has an infinite height
> - Involves a binary operator, ∇ : L × L → L, which satisfies:
>   1. ∀x, y ∈ L x ⊔ y ⊑ x ∇ y
>   2. ∀z₀ ⊑ z₁ ⊑ z₂ ⊑ ··· : the sequence y₀, y₁, y₂, ... defined by
>      y₀ = z₀ and y_{i+1} = y_i ∇ z_{i+1} for i = 0, 1, ... converges after a finite number of steps

### 해설

**개념 설명**

Widening은 무한 높이 격자에서 고정점 계산을 수렴하도록 강제하는 연산자이다.

**Widening 연산자의 정의**

∇ : L × L → L은 다음 두 조건을 만족해야 한다:

1. **Join보다 크거나 같음 (x ⊔ y ⊑ x ∇ y)**
   - Widening의 결과는 join의 결과보다 크거나 같다 (덜 정확하다)
   - 왜? 근사를 통해 수렴을 강제하기 위해
   - x ∇ y는 x와 y 모두를 포함하지만, join보다 더 큰 범위일 수 있다
   - 예: [0, 1] ⊔ [2, 3] = [0, 3]이지만, widening은 [0, ∞]일 수 있다

2. **수렴성 (Convergence)**
   - z₀ ⊑ z₁ ⊑ z₂ ⊑ ...의 증가 체인이 있을 때
   - y₀ = z₀, y_{i+1} = y_i ∇ z_{i+1}로 정의하면
   - 이 수열 y₀, y₁, y₂, ...는 유한 단계 후에 수렴한다
   - 즉, 어떤 k에 대해 y_k = y_{k+1} = y_{k+2} = ...

**직관**

- Join은 정확하지만 무한 높이에서 수렴하지 않을 수 있다
- Widening은 덜 정확하지만 반드시 수렴한다
- Tradeoff: 정확성을 포기해서 수렴을 얻는다

**중요성**

Widening이 없으면 무한 높이 격자에서 고정점을 찾을 수 없다.

---

## 슬라이드 19: Widening — Approximating the Least Fixed Point

### 원문 내용

> Widening — Approximating the Least Fixed Point
>
> We can approximate lfp(f) by computing:
>
>   x₀ = ⊥
>   x_{i+1} = x_i ∇ f(x_i)
>
> For some k, we have x_{k+1} = x_k, and lfp(f) ⊑ x_k
>
> - f(x_k) ⊑ x_k ∇ f(x_k) = x_{k+1} = x_k
> - ∇ does not need to be symmetric (x ∇ y ≠ y ∇ x in general)
> - We combine abstract information from the previous (left-hand) and the current iteration (right-hand)
> - Idea: to coarsen abstract values that are unstable

### 해설

**개념 설명**

Widening을 사용하여 최소 고정점을 근사하는 방법을 제시한다.

**알고리즘**

```
x₀ = ⊥
x_{i+1} = x_i ∇ f(x_i)
```

**동작 원리**

1. **초기 상태**: x₀ = ⊥ (정보 없음)

2. **반복 계산**: x_{i+1} = x_i ∇ f(x_i)
   - x_i: 이전 반복의 결과
   - f(x_i): 이번 반복에서 새로 계산된 값
   - x_i ∇ f(x_i): 둘을 합치되, widening으로 근사한다

3. **수렴**: 어떤 k에 대해 x_{k+1} = x_k
   - Widening의 정의에 의해 유한 단계 후 수렴한다
   - 이때 x_k가 근사된 고정점이다

**정확성 보장**

f(x_k) ⊑ x_k ∇ f(x_k) = x_{k+1} = x_k

따라서:
- f(x_k) ⊑ x_k
- x_k는 pre-fixed point이다
- Tarski의 정리에 의해 lfp(f) ⊑ x_k

즉, x_k는 실제 최소 고정점보다 큰 근사값이다 (더 덜 정확하지만 건전하다).

**비대칭성**

∇는 대칭일 필요가 없다:
- x ∇ y ≠ y ∇ x일 수 있다
- 왜? 알고리즘에서 의미가 있기 때문
- x_i: 좌측(이전) 값
- f(x_i): 우측(현재) 값
- 우측 값이 좌측보다 크면 widening을 적용해서 더 넓혀야 한다

**안정화되지 않은 값 처리**

Idea: "to coarsen abstract values that are unstable"
- 계속 변하는 값들(불안정한)은 widening으로 근사한다
- 이를 통해 수렴을 강제한다

---

## 슬라이드 20: Widening — Widening Operator

### 원문 내용

> Widening — Widening Operator
>
> ∇ : Interval × Interval → Interval
>
> ⊥ ∇ y = y
> x ∇ ⊥ = x
> [l₁, h₁] ∇ [l₂, h₂] = [l₃, h₃]
>
> where
>
> l₃ = { l₁     if l₁ ≤ l₂
>      { -∞    otherwise
>
> h₃ = { h₁     if h₁ ≥ h₂
>      { ∞     otherwise
>
> ∇: State × State → State
> (σ₁ ∇ σ₂)(x) = σ₁(x) ∇ σ₂(x)

### 해설

**개념 설명**

구간 도메인에서의 구체적인 widening 연산자 정의이다.

**기본 규칙**

1. **Bottom 처리**
   - ⊥ ∇ y = y: 정보가 없으면 우측값 사용
   - x ∇ ⊥ = x: 새 정보가 없으면 좌측값 유지

2. **일반적인 경우 [l₁, h₁] ∇ [l₂, h₂]**
   - 좌측(이전) 구간: [l₁, h₁]
   - 우측(현재) 구간: [l₂, h₂]
   - 결과: [l₃, h₃]

**하한 결정**

```
l₃ = { l₁     if l₁ ≤ l₂
     { -∞    otherwise
```

의미:
- 좌측 하한이 우측 하한보다 작거나 같으면: 좌측 유지
- 그렇지 않으면: -∞로 확대
- 즉, 하한이 올라가려면 무한으로 설정 (수렴 강제)

예:
- [0, 5] ∇ [1, 6] = [0, ∞] (l₁=0 ≤ l₂=1이므로 l₃=0, h₁=5 < h₂=6이므로 h₃=∞)
- [1, 5] ∇ [0, 6] = [-∞, 5] (l₁=1 > l₂=0이므로 l₃=-∞, h₁=5 < h₂=6이므로 h₃=∞)

**상한 결정**

```
h₃ = { h₁     if h₁ ≥ h₂
     { ∞     otherwise
```

의미:
- 좌측 상한이 우측 상한보다 크거나 같으면: 좌측 유지
- 그렇지 않으면: ∞로 확대

예:
- [0, 10] ∇ [1, 5] = [0, 10] (h₁=10 ≥ h₂=5이므로 h₃=10)
- [0, 5] ∇ [1, 6] = [0, ∞] (h₁=5 < h₂=6이므로 h₃=∞)

**상태(State) 레벨**

```
∇: State × State → State
(σ₁ ∇ σ₂)(x) = σ₁(x) ∇ σ₂(x)
```

- 각 변수별로 widening을 적용한다
- σ₁(x)와 σ₂(x)에 interval widening을 적용하고
- 결과를 새 상태에 할당한다

**직관**

- 범위가 좁혀지고 있으면(수렴 중이면) 그대로 유지
- 범위가 벌어지고 있으면(발산 중이면) 무한으로 설정
- 이를 통해 불안정한 값들을 조기에 무한으로 근사하여 수렴을 강제한다

---

## 슬라이드 21: Widening — Algorithm

### 원문 내용

> Widening — Algorithm
>
> PropagationWithWidening(t₁, . . . , tₙ, σ_start):
>   (x₁, x₂, . . . , xₙ) ← (σ_start, ⊥, . . . , ⊥)
>   W ← {v₁, . . . , vₙ}
>   while W ≠ ∅ :
>     vᵢ ← W.removeOne()
>     Y ← t_v(xᵢ)
>     for (y, vⱼ) ∈ Y :
>       z ← xⱼ ∇ y
>       if xⱼ ≠ z :
>         xⱼ ← z
>         W.add(vⱼ)
>   return x

### 해설

**개념 설명**

제어 민감성 알고리즘에서 join(⊔) 대신 widening(∇)을 사용한 버전이다.

**알고리즘 비교**

기본 알고리즘:
```
z ← xⱼ ⊔ y
```

Widening 알고리즘:
```
z ← xⱼ ∇ y
```

단 한 줄의 차이이지만, 이것이 무한 높이 격자에서 수렴을 보장한다.

**동작 메커니즘**

1. **초기화**: (x₁, x₂, . . . , xₙ) ← (σ_start, ⊥, . . . , ⊥)
   - 첫 위치는 초기 상태
   - 나머지는 정보 없음

2. **Worklist 처리**: 다른 것은 제어 민감성 알고리즘과 동일

3. **상태 병합**: z ← xⱼ ∇ y
   - Join 대신 widening 사용
   - 더 큰 근사값으로 계산

4. **변화 감지**: if xⱼ ≠ z
   - 변화가 있으면 worklist에 추가

5. **수렴**: Widening 정의에 의해 반드시 종료된다

**수렴 보장**

- Widening의 정의 조건 2에 의해 y_i 수열은 유한 단계 후 수렴한다
- 따라서 이 알고리즘도 유한 단계 후에 종료된다
- 각 위치에서의 상태가 안정화된다

**근사 정확성**

- 최종 결과는 실제 최소 고정점의 상한이다 (덜 정확)
- 하지만 건전성은 유지된다 (항상 실제 값을 포함)

---

## 슬라이드 22: Widening — Example (1)

### 원문 내용

> Widening — Example (1)
>
> x = 0;
> // [0, 0]
> while input() {
>   // [0, 0]
>   x = x + 1;
>   // [1, 1]
> }
>
> =>
>
> x = 0;
> // [0, inf]
> while input() {
>   // [0, inf]
>   x = x + 1;
>   // [1, inf]
> }

### 해설

**단계별 분석**

이 예제는 widening이 무한 루프를 어떻게 처리하는지 보여준다.

**초기 반복 (Join 사용 시)**

```
x = 0;                    // [0, 0]
while input() {
  x = x + 1;              // [1, 1]
}
// while 복귀: [0, 0] ⊔ [1, 1] = [0, 1]
```

다시 반복:
```
while input() {           // [0, 1]
  x = x + 1;              // [1, 2]
}
// while 복귀: [0, 1] ⊔ [1, 2] = [0, 2]
```

이렇게 계속 증가한다.

**Widening 적용**

첫 번째 반복은 같지만, 두 번째부터 widening을 사용한다:

```
// 첫 번째 반복
x = 0;                    // x = [0, 0]
while input() {           // x = [0, 0]
  x = x + 1;              // x = [1, 1]
}
// [0, 0] ∇ [1, 1] = ?
```

**Widening 계산**

[0, 0] ∇ [1, 1]:
- l₁ = 0, l₂ = 1: l₁ ≤ l₂이므로 l₃ = 0
- h₁ = 0, h₂ = 1: h₁ < h₂이므로 h₃ = ∞
- 결과: [0, ∞]

**두 번째 반복**

```
while input() {           // x = [0, ∞]
  x = x + 1;              // x = [1, ∞]
}
// [0, ∞] ∇ [1, ∞] = ?
```

[0, ∞] ∇ [1, ∞]:
- l₁ = 0, l₂ = 1: l₁ ≤ l₂이므로 l₃ = 0
- h₁ = ∞, h₂ = ∞: h₁ ≥ h₂이므로 h₃ = ∞
- 결과: [0, ∞]

**수렴**

이제 [0, ∞] ∇ [1, ∞] = [0, ∞]이므로 상태가 변하지 않는다.

따라서 알고리즘이 종료된다!

**최종 결과**

```
x = 0;
// [0, inf]
while input() {
  // [0, inf]
  x = x + 1;
  // [1, inf]
}
```

Join을 사용했으면 [0, 0] ⊑ [0, 1] ⊑ [0, 2] ⊑ ... (수렴 안 됨)
Widening을 사용하면 두 번째 반복에서 [0, ∞]로 점프하여 수렴한다.

---

## 슬라이드 23: Widening — Example (2)

### 원문 내용

> Widening — Example (2)
>
> x = 0;
> // [0, 0]
> while x <= 10 {
>   // [0, 0]
>   x = x + 1;
>   // [1, 1]
> }
> // bot
>
> =>
>
> x = 0;
> // [0, inf]
> while x <= 10 {
>   // [0, inf]
>   x = x + 1;
>   // [1, inf]
> }
> // [11, inf]

### 해설

**개념 설명**

이 예제는 조건이 있는 루프에서 widening이 어떻게 작동하는지 보여준다.

**기본 분석 (Join 사용)**

```
x = 0;                    // [0, 0]
while x <= 10 {           // 조건: x ≤ 10
  filter_≤([0, 0], 10) = [0, 0] // 조건 만족
  x = x + 1;              // [1, 1]
}
```

반복 진입 후 다시 루프 헤드로:
```
x = [0, 0] ⊔ [1, 1] = [0, 1]
while x <= 10 {
  filter_≤([0, 1], 10) = [0, 1] // 모두 ≤ 10 만족
  x = x + 1;              // [1, 2]
}
```

계속 진행하면... x가 11이 되면:
```
while x <= 10 {
  filter_≤([0, 10], 10) = [0, 10]
  x = x + 1;              // [1, 11]
}
```

Join으로 진행하면 최종적으로:
```
x = [0, 10] ⊔ [1, 11] = [0, 11]
while x <= 10 {
  filter_≤([0, 11], 10) = [0, 10]
  x = x + 1;              // [1, 11]
}
```

이는 수렴하지 않는다.

**Widening 적용**

두 번째 반복부터 widening을 사용:

```
x = 0;
while x <= 10 {           // [0, 0]
  x = x + 1;              // [1, 1]
}
// [0, 0] ∇ [1, 1] = [0, ∞]

while x <= 10 {           // [0, ∞]
  filter_≤([0, ∞], 10) = [0, 10]
  x = x + 1;              // [1, 11]
}
// [0, ∞] ∇ [1, 11] = ?
```

[0, ∞] ∇ [1, 11]:
- l₃: l₁=0 ≤ l₂=1이므로 l₃=0
- h₃: h₁=∞ ≥ h₂=11이므로 h₃=∞
- 결과: [0, ∞]

수렴했다!

**루프 탈출**

while 루프 탈출 후 (x > 10인 경우):
```
filter_>([0, ∞], 10) = [11, ∞]
```

따라서 최종적으로 x: [11, ∞]

**해석**

- Join: 수렴 안 함
- Widening: [0, ∞]에서 수렴하고, 루프 탈출 후 [11, ∞]로 정확히 계산

---

## 슬라이드 24: Narrowing — Motivation

### 원문 내용

> Narrowing — Motivation
>
> - After widening, narrowing can make the result more precise
> - Where x is the analysis result with widening:
>
>   lfp(f) ⊑ f^{i+1}(x) ⊑ f^i(x) ⊑ x
>
>   - lfp(f) ⊑ f(x) ⊑ x
> - In general, x ⊓ f(x) ⊓ f²(x) ⊓ ··· may not converge in finite steps

### 해설

**개념 설명**

Widening은 정확성을 포기하고 수렴을 얻는다. Narrowing은 widening 결과에서 정확성을 회복하는 기법이다.

**문제 상황**

Widening을 사용한 결과를 x라 하자:
- x는 최소 고정점보다 크다 (덜 정확)
- 예: x = [0, ∞]는 실제로는 [0, 10]과 [11, ∞]를 포함하지만 더 넓다

**개선 가능성**

lfp(f) ⊑ f^{i+1}(x) ⊑ f^i(x) ⊑ x 관계에서:
- f^i(x)는 감소 체인을 형성한다
- 즉, f(x) ⊑ x, f²(x) ⊑ f(x), ...
- 따라서 f(x), f²(x), f³(x), ...는 하강 체인이다

**예**

Widening 후 x = [0, ∞]:
- f(x): 루프 본문을 적용한 후의 값
- f²(x): 한 번 더 적용
- ...
- 이들은 점점 작아져서 실제 고정점에 가까워진다

**수렴 문제**

그러나:
- x ⊓ f(x) ⊓ f²(x) ⊓ ···가 반드시 유한 단계에 수렴하지 않을 수 있다
- 이를 보장하려면 narrowing 연산자가 필요하다

---

## 슬라이드 25: Narrowing — Definition

### 원문 내용

> Narrowing — Definition
>
> We can define a binary operator, △ : L × L → L, which satisfies:
>   1. ∀x, y ∈ L y ⊑ x ==> y ⊑ x △ y ⊑ x
>   2. ∀z₀ ⊒ z₁ ⊒ z₂ ⊒ ··· : the sequence y₀, y₁, y₂, ... defined by
>      y₀ = z₀ and y_{i+1} = y_i △ z_{i+1} for i = 0, 1, ... converges after a finite number of steps

### 해설

**개념 설명**

Narrowing은 widening과 쌍을 이루는 연산자이다.

**조건 1: 범위 내 유지**

∀x, y ∈ L: y ⊑ x ==> y ⊑ x △ y ⊑ x

의미:
- y가 x 내에 있으면 (y ⊑ x)
- x △ y의 결과도 y와 x 사이에 있어야 한다
- y ⊑ (x △ y) ⊑ x
- 즉, narrowing은 y를 x 범위 내에서 좁혀준다

예:
- x = [0, ∞], y = f(x) = [1, 11]
- y ⊑ x이므로 (실제로 [1, 11] ⊑ [0, ∞])
- x △ y는 [y, x] 사이의 값이어야 한다
- 예를 들어 [1, ∞]는 가능 ([1, 11] ⊑ [1, ∞] ⊑ [0, ∞])

**조건 2: 하강 수렴**

z₀ ⊒ z₁ ⊒ z₂ ⊒ ···의 하강 체인에 대해:
- y₀ = z₀, y_{i+1} = y_i △ z_{i+1}로 정의한 수열
- 이 수열이 유한 단계 후 수렴한다

의미:
- Widening은 상승 수열을 상승시켜서 빠르게 수렴시킨다
- Narrowing은 하강 수열을 하강시켜서 빠르게 수렴시킨다

---

## 슬라이드 26: Narrowing — Narrowing Operator

### 원문 내용

> Narrowing — Narrowing Operator
>
> △ : Interval × Interval → Interval
>
> ⊥ △ y = ⊥
> x △ ⊥ = ⊥
> [l₁, h₁] △ [l₂, h₂] = [l₂, h₃]
>
> where
>
> l₂ = { l₂ if l₁ = -∞
>      { l₁ otherwise
>
> h₃ = { h₂ if h₁ = ∞
>      { h₁ otherwise

### 해설

**개념 설명**

구간 도메인에서의 구체적인 narrowing 연산자이다.

**기본 규칙**

1. **Bottom 처리**
   - ⊥ △ y = ⊥
   - x △ ⊥ = ⊥
   - 정보가 없으면 결과도 정보 없음

2. **일반적인 경우 [l₁, h₁] △ [l₂, h₂]**
   - 결과: [l₂', h₂']
   - 여기서 l₂', h₃은 다음 규칙에 따라 결정

**하한 결정 (l₂')**

```
l₂' = { l₂ if l₁ = -∞
      { l₁ otherwise
```

의미:
- x (좌측)의 하한이 -∞이면 (제약 없음): y (우측)의 하한 사용
- 그렇지 않으면: x의 하한 사용
- 즉, 더 구체적인 제약 정보를 유지한다

예:
- [0, 10] △ [1, 5]: l₁=0 ≠ -∞이므로 l₂'=0
- [-∞, 10] △ [1, 5]: l₁=-∞이므로 l₂'=1

**상한 결정 (h₃)**

```
h₃ = { h₂ if h₁ = ∞
     { h₁ otherwise
```

의미:
- x의 상한이 ∞이면 (제약 없음): y의 상한 사용
- 그렇지 않으면: x의 상한 사용

예:
- [0, 10] △ [1, 5]: h₁=10 ≠ ∞이므로 h₃=10
- [0, ∞] △ [1, 5]: h₁=∞이므로 h₃=5

**직관**

- Widening: 범위가 벌어지면 무한으로 (상승 강제)
- Narrowing: 무한인 범위를 좁혀서 (하강 강제)

**상태 레벨**

Widening과 마찬가지로 각 변수별로 적용된다.

---

## 슬라이드 27: Narrowing — Algorithm

### 원문 내용

> Narrowing — Algorithm
>
> Narrowing(t₁, . . . , tₙ, x):
>   W ← {v₁, . . . , vₙ}
>   while W ≠ ∅ :
>     vᵢ ← W.removeOne()
>     y ← ⊓(σ | vⱼ ∈ dep^{-1}(vᵢ) ∧ (σ, v) ∈ t_v(xⱼ))
>     z ← xᵢ △ y
>     if z ≠ xᵢ :
>       xᵢ ← z
>       W ∪ dep(vᵢ)

### 해설

**개념 설명**

Widening 후 narrowing으로 정확성을 회복하는 알고리즘이다.

**입력**

- t₁, ..., tₙ: transfer functions
- x: widening 단계에서 얻은 근사 고정점

**알고리즘 구조**

1. **초기화**
   - W ← {v₁, ..., vₙ}
   - 모든 위치를 처리 대상으로 설정

2. **Main Loop**
   ```
   while W ≠ ∅:
   ```

3. **위치 선택**
   ```
   vᵢ ← W.removeOne()
   ```

4. **의존성 정보 수집**
   ```
   y ← ⊓(σ | vⱼ ∈ dep^{-1}(vᵢ) ∧ (σ, v) ∈ t_v(xⱼ))
   ```

   의미:
   - dep^{-1}(vᵢ): vᵢ에 데이터를 제공하는 위치들
   - 이들 각각에서 vᵢ로 들어오는 값들을 meet로 합친다
   - 더 정확한 정보를 얻는다

5. **Narrowing 적용**
   ```
   z ← xᵢ △ y
   ```
   - widening 결과 xᵢ를 새 정보 y로 좁힌다

6. **변화 감지 및 전파**
   ```
   if z ≠ xᵢ:
     xᵢ ← z
     W ∪ dep(vᵢ)
   ```
   - 변화가 있으면 이 위치에 의존하는 다른 위치들도 재계산

**수렴 보장**

Narrowing의 정의에 의해 하강 수열이 유한 단계 후 수렴한다.

---

## 슬라이드 28: Narrowing — Example

### 원문 내용

> Narrowing — Example
>
> x = 0;
> // [0, inf]
> while x <= 10 {
>   // [0, inf]
>   x = x + 1;
>   // [1, inf]
> }
> // [11, inf]
>
> =>
>
> x = 0;
> // [0, 11]
> while x <= 10 {
>   // [0, 10]
>   x = x + 1;
>   // [1, 11]
> }
> // [11, 11]

### 해설

**개념 설명**

Widening의 결과를 narrowing으로 정교하게 처리한다.

**Widening 결과 (입력)**

```
x = 0;                    // [0, ∞]
while x <= 10 {
  // [0, ∞]
  x = x + 1;              // [1, ∞]
}
```

루프 탈출 후 x: [11, ∞]

**Narrowing 단계 1**

while 루프 포인트에서:
- Widening 결과: [0, ∞]
- 들어오는 정보:
  - 초기: [0, 0]
  - 루프 본문 후: [1, ∞]
- Meet: [0, 0] ⊓ [1, ∞] = ⊥ (불가능)

아, 잠깐. 실제로는 다시 생각해보자.

**올바른 해석**

While 루프는 다음과 같이 작동한다:
1. 루프 입구 도달 (여러 경로에서)
2. 조건 필터링
3. 루프 본문 실행
4. 루프 입구로 돌아감

**Narrowing 수행**

Widening 결과에서 시작:
- x (루프 입구): [0, ∞]
- 루프 본문 실행: x = x + 1으로 [1, ∞]

이제 narrowing을 적용한다면:

1. **첫 반복**
   - x (루프 입구): [0, ∞]
   - 조건 필터: x ≤ 10이므로 filter_≤([0, ∞], 10) = [0, 10]
   - 루프 본문: [0, 10] + [1, 1] = [1, 11]
   - Narrowing: [0, ∞] △ [1, 11]
     - h₁=∞이므로 h₃=11
     - l₁=0 ≠ -∞이므로 l₂=0
     - 결과: [0, 11]

2. **두 번째 반복**
   - x (루프 입구): [0, 11]
   - 조건 필터: filter_≤([0, 11], 10) = [0, 10]
   - 루프 본문: [1, 11]
   - Narrowing: [0, 11] △ [1, 11]
     - 결과: [1, 11]? 아니다...

실제로 narrowing 연산 재정의:
   [l₁, h₁] △ [l₂, h₂] = [l', h']
   l' = if l₁ = -∞ then l₂ else l₁
   h' = if h₁ = ∞ then h₂ else h₁

따라서:
   [0, 11] △ [1, 11]:
   - l₁=0 ≠ -∞이므로 l'=0
   - h₁=11 ≠ ∞이므로 h'=11
   - 결과: [0, 11] (변화 없음, 수렴)

**최종 결과**

루프 탈출 후:
- x: [0, 11]
- 조건: x > 10이므로 filter_>([0, 11], 10) = [11, 11]

따라서:
```
x = 0;
// [0, 11]
while x <= 10 {
  // [0, 10]
  x = x + 1;
  // [1, 11]
}
// [11, 11]
```

**정확성 개선**

- Widening: x: [0, ∞] → x: [11, ∞]
- Narrowing: x: [0, 11] → x: [11, 11]

Narrowing으로 인해 루프 탈출 후 정확한 값을 얻을 수 있다!

---

## 슬라이드 29: Summary

### 원문 내용

> Summary
>
> - Interval analysis tracks lower and upper bounds of integer variables using a lattice of infinite height
> - Control sensitivity refines analysis precision by exploiting conditional branch information
> - Tasuki's theorem guarantees a least fixed point exists, but infinite-height lattices may prevent finite convergence
> - Widening (∇) ensures termination by over-approximating unstable values, at the cost of precision
> - Narrowing (△) recovers precision after widening by iteratively tightening the over-approximation

### 해설

**강의 요약**

이 슬라이드는 Lecture 9 전체의 핵심 내용을 정리한다.

**5가지 주요 개념**

1. **구간 분석 (Interval Analysis)**
   - 정수 변수의 값을 구간으로 추상화
   - 무한 높이 격자 사용
   - 배열 경계 검사, 오버플로우 감지 등에 활용

2. **제어 민감성 (Control Sensitivity)**
   - 조건문의 정보를 활용
   - 참/거짓 분기에서 변수 범위 좁히기 (필터링)
   - 기본 분석보다 정확한 결과 제공

3. **Tarski의 정리 (Tarski's Theorem)**
   - 완전 격자의 단조 함수는 최소 고정점을 가진다
   - 하지만 무한 높이 격자에서 유한 시간에 찾을 수 없을 수 있다

4. **확대 (Widening)**
   - 무한 높이 문제 해결
   - 불안정한 값을 의도적으로 근사 (무한으로)
   - 수렴을 보장하지만 정확성 감소

5. **좁혀짐 (Narrowing)**
   - Widening의 부정확함을 회복
   - 하강 반복으로 정확한 고정점에 가까워짐
   - Widening의 근사값을 정교하게 처리

**흐름**

Widening과 Narrowing은 쌍을 이룬다:
- **Widening 단계**: 빠른 수렴을 위해 과다 근사
- **Narrowing 단계**: 근사값을 개선하여 정확성 회복

이를 통해 무한 높이 격자에서도 효율적이고 정확한 고정점 계산이 가능해진다.

**실무 의미**

- 정적 분석 도구의 핵심 기법
- 프로그램의 런타임 오류 감지 (버그 검출)
- 컴파일러 최적화 기초
- 형식 검증(formal verification)에도 활용

**다음 내용으로의 연결**

이 강의에서 배운 widening과 narrowing 기법은:
- 다른 추상 해석 도메인에도 적용 가능
- 더 정교한 분석 기법의 기초
- 프로그램 검증 이론의 중요한 부분

---

## 강의 전체 정리

**CSE552 Program Analysis - Lecture 9: Widening 완전 학습 가이드**

### 핵심 문제와 해결책

**문제**: 무한 높이 격자에서 고정점 계산이 수렴하지 않는다
- 예: x를 계속 넓혀가는 구간 [0, 0] ⊑ [0, 1] ⊑ [0, 2] ⊑ ...

**해결책 1 - Widening**: 의도적인 근사로 수렴 강제
- [0, 0] ∇ [1, 1] = [0, ∞] (한 번에 점프)
- 몇 번의 반복 후 안정화

**해결책 2 - Narrowing**: Widening의 근사를 정교하게
- [0, ∞]에서 시작하여 점진적으로 좁혀짐
- 더 정확한 고정점 근사

### 학습 경로

1. **기초 (Slides 1-3)**: 구간 분석의 정의와 부분순서
2. **연산 (Slides 4-9)**: Join, Meet, Filter, Abstract Operations
3. **예제 (Slides 10-11)**: 기본 분석의 한계 이해
4. **향상 (Slides 12-15)**: 제어 민감성으로 정확성 증대
5. **이론 (Slides 16-17)**: Tarski 정리와 무한 높이 문제
6. **해결책 (Slides 18-23)**: Widening으로 수렴 보장
7. **정교화 (Slides 24-28)**: Narrowing으로 정확성 회복
8. **정리 (Slide 29)**: 전체 내용 요약

### 실전 활용

**Widening을 사용하는 이유**:
- 무한 반복되는 고정점 계산을 유한 단계에 멈추게 한다
- 비용이 중요한 정적 분석에서 필수적이다

**Narrowing을 사용하는 이유**:
- Widening의 과다 근사를 정정한다
- 더 정확한 분석 결과를 얻을 수 있다

**함께 사용하는 패턴**:
```
1. 기본 고정점 계산 (Widening 사용) - 빠르게 수렴
2. 결과 정교화 (Narrowing 사용) - 정확성 회복
```

