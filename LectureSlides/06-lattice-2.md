# 격자 이론 (2) - CSE552 프로그램 분석 강의 6

## Slide 1: 제목

> Lattice Theory (2), CSE552 Program Analysis — Lecture 6, Jaemin Hong

### 개념 설명

이 강의는 격자(lattice) 이론의 두 번째 부분으로, 프로그램 분석에서 고정점(fixed point) 이론을 다룹니다. 첫 번째 강의에서 배운 격자의 기본 구조를 바탕으로, 이번에는 프로그램의 의미를 어떻게 계산하는지에 대한 수학적 기초를 제공합니다.

---

## Slide 2: 동기 부여 예제 — Sign⁶

> Code: `a = 42; b = a + input();`
> Abstract domain: Sign⁶
> - a₀ = ⊤, b₀ = ⊤
> - a₁ = +
> - b₁ = b₀
> - a₂ = a₁
> - b₂ = a₁ + ⊤ (we will define + later)

### 개념 설명

이 예제는 프로그램 지점(program point)마다 변수의 부호(sign)를 추적하는 프로그램 분석입니다. Sign 추상 영역은 음수(−), 양수(+), 0, 그리고 ⊤(모든 부호 가능)을 포함합니다.

### 상세한 예시

프로그램이 실행되면서:
- 초기 상태(a₀, b₀): 두 변수 모두 어떤 값이든 가능하므로 ⊤
- a₁: `a = 42`를 실행한 후, a는 양수(+)로 결정됩니다
- b₁: b는 아직 변하지 않아서 b₀과 같이 ⊤입니다
- a₂: a는 이전 값 a₁을 유지하므로 +입니다
- b₂: b는 a₁(양수) + ⊤(unknown input)의 결과로, 정확한 부호를 알 수 없으므로 ⊤입니다

### 배경 지식

추상 영역(abstract domain)이란 실제 프로그램의 값들을 더 단순한 형태로 대표하는 수학적 구조입니다. Sign 추상 영역은 정수의 무한 집합을 단 5개의 추상 값으로 축약합니다.

---

## Slide 3: 동기 부여 예제 — (Var → Sign)³

> Code: `a = 42; b = a + input();`
> Abstract domain: (Var → Sign)³
> - x₀ = [a ↦ ⊤, b ↦ ⊤]
> - x₁ = x₀[a ↦ +]
> - x₂ = x₁[b ↦ x₁(a) + ⊤]

### 개념 설명

이제 분석을 더 일반적으로 표현합니다. 각 프로그램 지점에서의 상태를 `x₀, x₁, x₂`로 나타내며, 각 상태는 변수에서 부호 값으로의 함수(function)입니다.

### 상세한 예시

- **x₀**: 프로그램 시작 시 모든 변수가 ⊤ (unknown)
- **x₁**: `a = 42` 실행 후, x₀를 복사하고 a의 값만 +로 업데이트
  - `x₀[a ↦ +]` 표기법은 "x₀의 a 항목을 +로 재할당한다"는 의미
  - 결과: [a ↦ +, b ↦ ⊤]
- **x₂**: `b = a + input()` 실행 후
  - x₁(a)는 +를 조회
  - ⊤과의 합은 ⊤
  - 결과: [a ↦ +, b ↦ ⊤]

### 배경 지식

(Var → Sign)은 변수 집합에서 Sign 값으로의 함수 공간을 나타냅니다. 이러한 함수 공간 자체도 격자를 이루며, 위에서 정의된 순서 관계를 가집니다.

---

## Slide 4: 흐름 민감 분석

> Same code and abstract domain as slide 3.
> - Flow-sensitive analysis: The order of statements is taken into account. The sign of variables is determined for each program point.

### 개념 설명

흐름 민감 분석(flow-sensitive analysis)은 프로그램의 제어 흐름(control flow)을 고려하여 각 프로그램 지점에서의 변수 값을 추적하는 분석 방법입니다.

### 상세한 예시

같은 프로그램을 분석할 때:
- 흐름 민감: 각 문장 이후의 상태를 별도로 추적
  - 첫 문장 후: a는 양수
  - 두 번째 문장 후: b는 unknown
- 흐름 비민감: 모든 할당을 합치서 처리
  - a는 양수이거나 input값 (정확하지 않음)

### 배경 지식

흐름 민감 분석은 더 정확한 결과를 제공하지만 계산 비용이 더 높습니다. 반대로 흐름 비민감 분석은 빠르지만 정확도가 낮습니다.

---

## Slide 5: 대입을 통한 풀이

> Same code and equations.
> - For this example program, each equation only depends on preceding ones
> - The solution can be found by simple substitution: x₀ = [a↦⊤, b↦⊤], x₁ = [a↦+, b↦⊤], x₂ = [a↦+, b↦⊤]
> - In general, mutually recursive equations may appear, e.g., for programs that contain loops

### 개념 설명

이 간단한 선형(loop이 없는) 프로그램의 경우, 연립 방정식을 순차적으로 풀 수 있습니다. 각 방정식이 이전 방정식에만 의존하기 때문입니다.

### 상세한 예시

순환 구조가 없으므로:
1. x₀를 초기 상태로 설정: [a↦⊤, b↦⊤]
2. x₀을 사용하여 x₁ 계산: [a↦+, b↦⊤]
3. x₁을 사용하여 x₂ 계산: [a↦+, b↦⊤]

프로그램에 루프가 있으면? 예를 들어:
```
x = 0;
while (x < 10) {
  x = x + 1;
}
```
이 경우 x의 값이 루프에 의존하므로 상호 순환적 방정식이 생깁니다.

### 배경 지식

프로그램이 루프를 포함하면 방정식 체계는 상호 순환적(mutually recursive)이 되며, 단순한 순차 풀이는 불가능합니다. 이때는 고정점 이론을 사용해야 합니다.

---

## Slide 6: 고정점 공식화

> - Solving this system requires finding the fixed point for function f : (Var → Sign)³ → (Var → Sign)³ defined as:
>   f(x₀, x₁, x₂) = ([a↦⊤, b↦⊤], x₀[a↦+], x₁[b↦x₁(a)+⊤])
> - A fixed point for f is x that satisfies f(x) = x
> - How can we find a fixed point for a function over a lattice?

### 개념 설명

프로그램 분석 문제를 고정점 찾기 문제로 재공식화합니다. 함수 f는 현재 상태에서 다음 상태로의 변환을 나타냅니다.

### 상세한 예시

함수 f의 정의를 해석하면:
- 첫 번째 항: 항상 초기 상태 [a↦⊤, b↦⊤]를 반환 (상수 함수)
- 두 번째 항: x₀에서 a를 +로 업데이트
- 세 번째 항: x₁의 a 값에 ⊤을 더하고, x₁의 b를 그 결과로 업데이트

고정점이란 f(x) = x를 만족하는 x입니다. 즉:
```
f(x₀, x₁, x₂) = (x₀, x₁, x₂)
```

### 배경 지식

프로그램 분석 문제를 수학적으로 표현하면, 복잡한 제어 흐름도 함수의 고정점으로 단순하게 표현할 수 있습니다.

---

## Slide 7: 단조 함수 — 정의

> Definition (Monotone function). A function f : L₁ → L₂ where L₁ and L₂ are lattices is monotone (or order-preserving) when ∀x,y ∈ L₁. x ⊑ y ⇒ f(x) ⊑ f(y)
> - From the analysis perspective, the intuition of monotonicity is that more precise input does not result in less precise output

### 개념 설명

단조 함수(monotone function)는 입력이 더 정확해지면 출력도 더 정확해지는(또는 같은 정확도의) 함수입니다. 이는 프로그램 분석에서 매우 중요한 성질입니다.

### 상세한 예시

Sign 격자에서의 예:
- x = ⊤, y = +라고 하면 x ⊑ y (⊤은 가장 덜 정확한 값)
- 함수 f(z) = "z와 1을 더한다"를 생각해봅시다
- f(⊤) = ⊤ (unknown + 1 = unknown)
- f(+) = + (positive + 1 = positive)
- f(⊤) ⊑ f(+)이므로 f는 단조입니다

반대로, 단조가 아닌 함수의 예:
- f(z) = "z와 반대 부호"라는 함수는 단조가 아닙니다
- + ⊑ ⊤이지만 f(+) = −이고 f(⊤) = ⊤
- −은 ⊤과 비교할 수 없는 관계이므로 단조성 위반

### 배경 지식

프로그램 분석에서 단조성은 알고리즘의 수렴성(convergence)을 보장합니다. 단조 함수는 격자를 통해 정확한 방향으로 "이동"하므로, 고정점에 도달할 것이 보장됩니다.

---

## Slide 8: 광범위 함수와 분배 함수

> Definition (Extensive function). A function f : L → L where L is a lattice is extensive when ∀x ∈ L. x ⊑ f(x)
> Definition (Distributive function). A function f : L₁ → L₂ where L₁ and L₂ are lattices is distributive when ∀x,y ∈ L₁. f(x) ⊔ f(y) = f(x ⊔ y)
> - Every distributive function is also monotone
> - Not every monotone function is also distributive

### 개념 설명

광범위 함수(extensive function)는 입력보다 항상 더 덜 정확한(또는 같은) 출력을 생성합니다. 분배 함수(distributive function)는 합(join) 연산을 보존합니다.

### 상세한 예시

광범위 함수의 예:
- f(x) = ⊤은 모든 x에 대해 f(x) = ⊤이므로, x ⊑ ⊤입니다 (광범위)

분배 함수의 예:
- f(x) = 2x (정수에 대해)
  - f(3) ⊔ f(5) = 6 ⊔ 10 = max(6, 10) = 10
  - f(3 ⊔ 5) = f(max(3, 5)) = f(5) = 10
  - 분배 함수입니다

단조이지만 분배가 아닌 함수:
- Sign 격자에서 f(+) = +, f(−) = +, f(0) = 0, f(⊤) = ⊤
- + ⊑ ⊤이므로 + ⊔ − ⊑ ⊤
- f(+ ⊔ −) = f(⊤) = ⊤
- f(+) ⊔ f(−) = + ⊔ + = +
- ⊤ ≠ +이므로 분배가 아닙니다

### 배경 지식

모든 분배 함수는 단조이지만, 역은 성립하지 않습니다. 프로그램 분석에서는 보통 단조성만 필요하며, 분배성은 추가적인 정밀성을 제공합니다.

---

## Slide 9: 단조 함수 — 성질 (1)

> Important properties:
> - Every constant function is monotone
> - f is monotone ⟺ ∀x,y. f(x) ⊔ f(y) ⊑ f(x ⊔ y)
> - If f and g are monotone, then so is their composition g ∘ f, defined by (g ∘ f)(x) = g(f(x))
> - ⊔ : L² → L and ⊓ : L² → L are monotone

### 개념 설명

단조 함수는 여러 좋은 성질을 가지고 있습니다. 이러한 성질들은 복잡한 함수가 단조임을 증명하는 데 사용됩니다.

### 상세한 예시

각 성질을 살펴봅시다:

1. **상수 함수는 단조**: f(x) = c (모든 x에 대해)
   - 어떤 x, y를 선택해도 f(x) = c = f(y)이므로 f(x) ⊑ f(y)

2. **단조성의 동등 조건**:
   - 정의: x ⊑ y ⇒ f(x) ⊑ f(y)
   - 동등한 형태: f(x) ⊔ f(y) ⊑ f(x ⊔ y)
   - 이유: x ⊑ (x ⊔ y)이고 y ⊑ (x ⊔ y)이므로
   - f(x) ⊑ f(x ⊔ y)이고 f(y) ⊑ f(x ⊔ y)
   - 따라서 f(x) ⊔ f(y) ⊑ f(x ⊔ y)

3. **단조 함수의 합성**:
   - f: x ⊑ y ⇒ f(x) ⊑ f(y)
   - g: f(x) ⊑ f(y) ⇒ g(f(x)) ⊑ g(f(y))
   - 따라서 g ∘ f도 단조

4. **Join과 Meet 연산**:
   - Join: x ⊑ x ⊔ y, y ⊑ x ⊔ y (정의에서)
   - Meet: x ⊓ y ⊑ x, x ⊓ y ⊑ y (정의에서)
   - 이들 연산 자체가 단조입니다

### 배경 지식

이러한 성질들을 사용하면 프로그램 분석 함수가 단조임을 귀납적으로 증명할 수 있습니다.

---

## Slide 10: 단조 함수 — 성질 (2)

> Important properties (cont.):
> - If f : L₁ → (A → L₂) and g : L₁ → L₂ are monotone, then so is the function h : L₁ → (A → L₂) defined by h(x) = f(x)[a ↦ g(x)]
> - f₁ : L → L₁, ..., fₙ : L → Lₙ are monotone ⟺ f : L → L₁ × ··· × Lₙ defined by f(x) = (f₁(x), ..., fₙ(x)) is monotone

### 개념 설명

더 복잡한 함수의 단조성을 보장하는 조건들입니다. 첫 번째는 함수 공간의 업데이트에 관한 것이고, 두 번째는 곱(product) 구조에 관한 것입니다.

### 상세한 예시

첫 번째 성질 (함수 업데이트):
```
h(x) = f(x)[a ↦ g(x)]
```
이는 "f(x)의 a 위치에 g(x)를 할당한다"는 의미입니다.

단조성 증명:
- x ⊑ y라고 하자
- f(x) ⊑ f(y) (f가 단조)
- g(x) ⊑ g(y) (g가 단조)
- f(x)[a ↦ g(x)] ⊑ f(y)[a ↦ g(y)] (함수 업데이트도 단조)
- 따라서 h(x) ⊑ h(y)

두 번째 성질 (곱 구조):
```
f(x) = (f₁(x), ..., fₙ(x))
```

단조성:
- (x₁, y₁) ⊑ (x₂, y₂) ⟺ x₁ ⊑ x₂ and y₁ ⊑ y₂
- 각 fᵢ가 단조이면, 전체 f도 단조입니다

### 배경 지식

이 성질들은 프로그램 분석 함수(변수 상태들을 튜플로 묶은 형태)의 단조성을 증명하는 데 매우 유용합니다.

---

## Slide 11: 단조 함수 — 예제

> - f(x₀, x₁, x₂) = ([a↦⊤, b↦⊤], x₀[a↦+], x₁[b↦x₁(a)+⊤])
>   = (f₀(x₀, x₁, x₂), f₁(x₀, x₁, x₂), f₂(x₀, x₁, x₂))
> - f₀ is monotone because it is a constant function
> - f₁ is monotone because (x₀, x₁, x₂) ↦ x₀ is monotone and (x₀, x₁, x₂) ↦ + is monotone
> - f₂ is monotone (we will show it later)
> - f is monotone because f₀, f₁, and f₂ are monotone

### 개념 설명

Slide 6의 예제 함수가 실제로 단조임을 증명합니다. 이를 위해 함수를 세 개의 성분으로 분해합니다.

### 상세한 예시

**f₀ = [a↦⊤, b↦⊤]**:
- 상수 함수이므로 항상 단조입니다

**f₁ = x₀[a↦+]**:
- 이는 두 개의 단조 함수의 합성입니다:
  1. (x₀, x₁, x₂) ↦ x₀: 투영(projection)은 단조
  2. x₀ ↦ x₀[a↦+]: 함수 업데이트는 단조 (Slide 10의 성질)
- 두 단조 함수의 합성은 단조

**f₂ = x₁[b↦x₁(a)+⊤]**:
- x₁(a)는 x₁에 대한 투영 (단조)
- x₁(a) + ⊤는 두 개의 단조 함수를 적용한 것
  - Sign에서의 덧셈 연산 + : Sign² → Sign도 단조입니다
- x₁[b↦...] 함수 업데이트는 단조
- 모두 단조 함수의 합성이므로 f₂도 단조

**전체 함수 f**:
- Slide 10의 두 번째 성질에 의해, f₀, f₁, f₂가 모두 단조이면 f = (f₀, f₁, f₂)도 단조입니다

### 배경 지식

프로그램 분석 함수의 단조성은 고정점 이론을 적용할 수 있는 조건입니다. 실제 프로그램 분석에서는 대부분의 연산(대입, 덧셈, Join, Meet 등)이 단조이므로, 복잡한 분석 함수도 단조임을 보이기는 어렵지 않습니다.

---

## Slide 12: 고정점 — 정의

> Definition (Fixed point).
> - x ∈ L is a fixed point for f if f(x) = x
> - A least fixed point (lfp) x for f is a fixed point for f where x ⊑ y for every fixed point y for f

### 개념 설명

고정점(fixed point)은 함수의 입출력이 같은 원소입니다. 최소 고정점(least fixed point)은 모든 고정점 중에서 가장 정확한(가장 낮은) 값입니다.

### 상세한 예시

간단한 예: f(x) = x² (실수에서)
- 고정점: f(0) = 0, f(1) = 1
- 두 개의 고정점이 있습니다

프로그램 분석 맥락에서:
```
f(x₀, x₁, x₂) = ([a↦⊤, b↦⊤], x₀[a↦+], x₁[b↦x₁(a)+⊤])
```

만약 x = (x₀, x₁, x₂)가 고정점이면:
- x₀ = [a↦⊤, b↦⊤]
- x₁ = x₀[a↦+] = [a↦+, b↦⊤]
- x₂ = x₁[b↦x₁(a)+⊤] = [a↦+, b↦⊤]

최소 고정점은:
- 정의에 따라 다른 모든 고정점보다 작거나 같은 고정점입니다
- 프로그램 분석에서는 최소 고정점이 가장 정밀한 분석 결과를 제공합니다

### 배경 지식

여러 고정점이 존재할 수 있습니다. 예를 들어, f(x) = ⊤은 모든 x가 고정점입니다 (f(x) = ⊤ = x? 아니, x ≠ ⊤일 수 있으므로 고정점이 아닙니다).

정확한 예: f(x) = x ⊔ ⊤이면
- 모든 x에 대해 f(x) = ⊤이므로, x = ⊤일 때만 고정점입니다

최소 고정점의 존재성은 보장되지 않을 수 있으므로, 이를 위한 정리들이 필요합니다.

---

## Slide 13: 고정점 — 분석에서의 역할

> Where the constraints are expressed as an equation system x = f(x),
> - A solution to the system is the same as a fixed point for f
> - For carefully designed constraints, every fixed point provides a sound result
> - Among all fixed points, the lfp provides the most precise result

### 개념 설명

프로그램 분석에서 제약 조건들은 방정식 시스템으로 표현되며, 이 시스템의 해는 함수의 고정점과 동일합니다. 최소 고정점이 가장 정확한 분석 결과입니다.

### 상세한 예시

방정식 시스템:
```
x₀ = [a↦⊤, b↦⊤]
x₁ = x₀[a↦+]
x₂ = x₁[b↦x₁(a)+⊤]
```

이는 다음과 같이 다시 쓸 수 있습니다:
```
(x₀, x₁, x₂) = f(x₀, x₁, x₂)
```

여기서 f는 Slide 6에서 정의한 함수입니다. 이 방정식의 해는 f의 고정점입니다.

최소 고정점의 의미:
- 만약 다른 해 y가 있다면, lfp(f) ⊑ y입니다
- 더 정밀한 정보를 제공합니다

### 배경 지식

프로그램 분석에서 제약 조건이 "조심스럽게 설계된다"는 것은 분석의 단조성과 건전성(soundness)을 보장한다는 의미입니다. 모든 고정점이 프로그램의 행동에 대한 보수적인(conservative) 근사를 제공합니다.

---

## Slide 14: 타르스키의 고정점 정리

> Theorem (Tarski¹). If L is a complete lattice and f : L → L is monotone, then f has a least fixed point
> - The most precise solution is guaranteed to exist, but how can we find it?
> ¹A lattice-theoretical fixpoint theorem and its applications (Tarski, 1955)

### 개념 설명

타르스키(Tarski)의 고정점 정리는 완전 격자(complete lattice) 위의 단조 함수는 항상 최소 고정점을 가진다는 것을 보장합니다.

### 상세한 예시

정리의 의미:
- 완전 격자: 모든 부분집합이 상한(supremum)과 하한(infimum)을 가지는 격자
  - 유한 격자는 자동으로 완전
  - 무한 격자도 완전할 수 있음
- 단조 함수: Slide 7의 정의에 따름
- 결론: 최소 고정점이 존재합니다

예시:
```
f(x) = ⊥ ⊔ x (모든 x에 대해 더 큰 값을 반환)
```
- 이 함수는 단조입니다
- 고정점: f(⊥) = ⊥ ⊔ ⊥ = ⊥이므로 ⊥는 고정점입니다
- 실제로 ⊥는 최소 고정점입니다

### 배경 지식

타르스키의 정리는 존재성만을 보장합니다. 실제로 최소 고정점을 찾으려면 추가 정리(클린의 정리)가 필요합니다. 타르스키의 정리는 1955년 발표되었으며, 현대 프로그램 분석의 수학적 기초입니다.

---

## Slide 15: 클린의 고정점 정리

> Theorem (Kleene²). If L is a complete lattice with a finite height and f : L → L is monotone, then
>   lfp(f) = ⊔ᵢ≥₀ fⁱ(⊥)
> - If the lattice has a finite height, we can find the lfp by computing the increasing chain ⊥ ⊑ f(⊥) ⊑ f²(⊥) ⊑ ··· until the fixed point is reached
> ²Introduction to metamathematics (Kleene, 1952)

### 개념 설명

클린(Kleene)의 고정점 정리는 최소 고정점을 실제로 계산하는 방법을 제시합니다. 격자의 높이가 유한하면, 단순한 반복을 통해 최소 고정점에 도달할 수 있습니다.

### 상세한 예시

높이가 유한한 격자에서:
- ⊥부터 시작
- f(⊥)를 계산
- f(f(⊥)) = f²(⊥)를 계산
- ...
- f^n(⊥) = f^(n+1)(⊥)일 때 멈춤 (고정점 도달)

이 과정은 증가하는 수열입니다:
```
⊥ ⊑ f(⊥) ⊑ f²(⊥) ⊑ f³(⊥) ⊑ ...
```

유한 높이이므로 반드시 안정화됩니다.

예시 (Slide 17에서 자세히):
```
⊥ = ([a↦⊥, b↦⊥], [a↦⊥, b↦⊥], [a↦⊥, b↦⊥])
f(⊥) = ([a↦⊤, b↦⊤], [a↦+, b↦⊥], [a↦⊥, b↦⊥])
f²(⊥) = ([a↦⊤, b↦⊤], [a↦+, b↦⊤], [a↦+, b↦⊤])
f³(⊥) = f²(⊥) (고정점)
```

### 배경 지식

클린의 정리는 계산 가능성을 보장합니다. 무한 높이의 격자에서는 이 방법이 작동하지 않을 수 있으므로, 프로그램 분석에서는 유한 높이의 격자를 의도적으로 설계합니다. 클린의 정리는 1952년 발표되었습니다.

---

## Slide 16: 순진한 고정점 알고리즘

> NaiveFixedPointAlgorithm(f):
>   x ← ⊥
>   while x ≠ f(x):
>     x ← f(x)
>   return x
> [Diagram showing zigzag convergence path inside a lattice triangle]

### 개념 설명

클린의 정리를 기반으로 한 실제 알고리즘입니다. 이 알고리즘은 구현이 간단하지만, 실제로는 여러 최적화가 필요합니다.

### 상세한 예시

알고리즘의 실행 흐름:

1. **초기화**: x ← ⊥
   - 모든 변수가 unknown 상태

2. **반복**:
   - x의 현재 값을 함수 f에 입력
   - 결과가 이전 값과 다르면 업데이트
   - 같으면 고정점에 도달한 것이므로 종료

3. **반환**: x (최소 고정점)

다이어그램의 의미:
- 삼각형은 격자를 나타냅니다
- 아래쪽이 ⊥, 위쪽이 ⊤
- 지그재그 경로는 f(⊥) → f²(⊥) → ... → lfp(f)의 진행을 보여줍니다

### 배경 지식

이 알고리즘은 직관적이지만 비효율적일 수 있습니다:
- 매 반복마다 전체 상태를 비교해야 함
- 변수가 많으면 비교 비용이 높음
- 실제 구현에서는 worklist 알고리즘 등의 최적화를 사용합니다

---

## Slide 17: 고정점 알고리즘 — 예제

> - f(x₀, x₁, x₂) = ([a↦⊤, b↦⊤], x₀[a↦+], x₁[b↦x₁(a)+⊤])
> - ⊥ = ([a↦⊥, b↦⊥], [a↦⊥, b↦⊥], [a↦⊥, b↦⊥])
> - f(⊥) = ([a↦⊤, b↦⊤], [a↦+, b↦⊥], [a↦⊥, b↦⊥])
> - f²(⊥) = ([a↦⊤, b↦⊤], [a↦+, b↦⊤], [a↦+, b↦⊤])
> - f³(⊥) = ([a↦⊤, b↦⊤], [a↦+, b↦⊤], [a↦+, b↦⊤]) = f²(⊥)

### 개념 설명

Slide 6의 예제 함수에 대해 실제로 알고리즘을 실행해봅니다. 몇 번의 반복 후에 고정점에 도달합니다.

### 상세한 예시

**초기 상태**:
```
⊥ = ([a↦⊥, b↦⊥], [a↦⊥, b↦⊥], [a↦⊥, b↦⊥])
```
모든 변수의 부호가 unknown입니다.

**첫 번째 반복 f(⊥)**:
```
f₀(⊥) = [a↦⊤, b↦⊤]  (상수)
f₁(⊥) = ⊥[a↦+] = [a↦+, b↦⊥]
f₂(⊥) = ⊥[b↦⊥(a)+⊤] = ⊥[b↦⊥+⊤] = ⊥[b↦⊤]
       = [a↦⊥, b↦⊤]  (아니, ⊥에서 b를 업데이트)
       = [a↦⊥, b↦⊥]  (⊥ + ⊤ = ⊤인데, f₂ 정의를 다시 보면)
```

실제로:
```
f₂(⊥) = ⊥[b↦⊥(a)+⊤]
⊥(a) = ⊥ (변수 a의 값이 unknown)
⊥ + ⊤ = ⊤ (unknown과 어떤 것의 합도 unknown)
따라서 f₂(⊥) = ⊥[b↦⊤] = [a↦⊥, b↦⊤]
```

아니 다시, ⊥는 함수이므로:
```
⊥ = [a↦⊥, b↦⊥]
⊥(a) = ⊥
⊥ + ⊤ = ⊤
⊥[b↦⊤] = [a↦⊥, b↦⊤]

아니... f₂는 x₁에 대한 것이므로:
f₂ = x₁[b↦x₁(a)+⊤]
```

명확히 하기 위해, x = (x₀, x₁, x₂)라 하면:
```
⊥ = (
  x₀ = [a↦⊥, b↦⊥],
  x₁ = [a↦⊥, b↦⊥],
  x₂ = [a↦⊥, b↦⊥]
)

f(⊥) = (
  f₀ = [a↦⊤, b↦⊤],
  f₁ = x₀[a↦+] = [a↦+, b↦⊥],
  f₂ = x₁[b↦x₁(a)+⊤] = [a↦⊥, b↦⊥+⊤] = [a↦⊥, b↦⊤]
)

잠깐, f₂ 정의에서 x₁(a)를 사용합니다.
x₁ = [a↦⊥, b↦⊥]이므로:
x₁(a) = ⊥
⊥ + ⊤ = ⊤
⊥[b↦⊤] = [a↦⊥, b↦⊤]
```

실제로 Slide에 있는 값으로 재검토:
```
f(⊥) = ([a↦⊤, b↦⊤], [a↦+, b↦⊥], [a↦⊥, b↦⊥])
```

이를 해석하면:
- f₀ = [a↦⊤, b↦⊤]: 항상 초기 상태
- f₁ = [a↦+, b↦⊥]: x₀[a↦+]에서 x₀ = ⊥이지만... 아, x₀는 (x₀, x₁, x₂) 튜플의 첫 번째 원소입니다!

다시 정리:
```
x = (x₀, x₁, x₂)는 세 개의 프로그램 지점에서의 상태

⊥ = ([a↦⊥, b↦⊥], [a↦⊥, b↦⊥], [a↦⊥, b↦⊥])

f(x₀, x₁, x₂) = (
  [a↦⊤, b↦⊤],      (항상 초기 상태)
  x₀[a↦+],         (x₀를 a로 +로 업데이트)
  x₁[b↦x₁(a)+⊤]   (x₁을 b로 업데이트)
)

f(⊥) = (
  [a↦⊤, b↦⊤],
  [a↦⊥, b↦⊥][a↦+] = [a↦+, b↦⊥],
  [a↦⊥, b↦⊥][b↦[a↦⊥, b↦⊥](a)+⊤] = [a↦⊥, b↦⊥+⊤]
)

⊥ + ⊤ = ⊤
따라서: [a↦⊥, b↦⊤]
```

다만 Slide의 표현이 x₁의 상태를 추적하므로, 다시:
```
f₂(⊥) = x₁[b↦x₁(a)+⊤]

하지만 ⊥에서 x₁ = [a↦⊥, b↦⊥]이므로:
f₂(⊥) = [a↦⊥, b↦⊥][b↦⊥+⊤] = [a↦⊥, b↦⊤]
```

Slide와 맞춤을 위해 재표현:
```
x₁ = [a↦⊥, b↦⊥] (⊥의 x₁ 부분)
x₁(a) = ⊥
⊥ + ⊤ = ⊤ (Sign 격자에서 부호가 unknown)
x₁[b↦⊤] = [a↦⊥, b↦⊤]

그런데 왜 Slide는 [a↦⊥, b↦⊥]라고 했나?
```

Slide 17을 다시 읽으니 f₂의 결과가 명확하지 않습니다. 논리적으로:
```
f(⊥) = ([a↦⊤, b↦⊤], [a↦+, b↦⊥], x₁[b↦x₁(a)+⊤])

x₁ = [a↦⊥, b↦⊥]에서:
x₁(a) = ⊥
⊥ + ⊤ = ⊤
[a↦⊥, b↦⊥][b↦⊤] = [a↦⊥, b↦⊤]

따라서: f(⊥) = ([a↦⊤, b↦⊤], [a↦+, b↦⊥], [a↦⊥, b↦⊤])

그런데 Slide는 f(⊥) = ([a↦⊤, b↦⊤], [a↦+, b↦⊥], [a↦⊥, b↦⊥])

이것은 x₁(a) + ⊤ = ⊤가 아니라 ⊤ + ⊤ = ⊤로 해석하면... 아니다.
```

아마 Slide의 표기가 실수일 수 있습니다. 논리적 흐름으로 계속하겠습니다:

**두 번째 반복 f²(⊥)**:
```
f(⊥) = ([a↦⊤, b↦⊤], [a↦+, b↦⊥], [a↦⊥, b↦⊤])

f₂를 다시 계산:
x₁ = [a↦+, b↦⊥] (f(⊥)의 x₁)
x₁(a) = +
+ + ⊤ = ⊤
[a↦+, b↦⊥][b↦⊤] = [a↦+, b↦⊤]

f²(⊥) = ([a↦⊤, b↦⊤], [a↦+, b↦⊤], [a↦+, b↦⊤])
```

**세 번째 반복 f³(⊥)**:
```
f(f²(⊥)):
x₀ = [a↦⊤, b↦⊤]
x₀[a↦+] = [a↦+, b↦⊤]

x₁ = [a↦+, b↦⊤]
x₁(a) = +
+ + ⊤ = ⊤
[a↦+, b↦⊤][b↦⊤] = [a↦+, b↦⊤]

f³(⊥) = ([a↦⊤, b↦⊤], [a↦+, b↦⊤], [a↦+, b↦⊤]) = f²(⊥)
```

고정점에 도달했습니다!

### 배경 지식

이 예제는 프로그램 분석의 최소 고정점이 몇 번의 반복 후에 도달됨을 보여줍니다. Sign 격자는 높이가 작으므로(5개 원소) 빠르게 수렴합니다.

---

## Slide 18: 정밀도와 복잡도

> - Even though we find the most precise possible solution to the equation system, the equation system is merely a conservative approximation of the actual program behavior
> - The semantically most precise answer can be below the lfp in the lattice
> - Time complexity depends on: the height of the lattice (bounds iterations), the cost of computing f(x) and testing equality (performed each iteration)

### 개념 설명

최소 고정점은 우리가 찾을 수 있는 가장 정확한 해이지만, 여전히 실제 프로그램 행동보다는 덜 정확할 수 있습니다. 이는 추상 영역의 한계 때문입니다.

### 상세한 예시

정밀도의 한계:

프로그램:
```
x = input();
if (x > 0) {
  // x는 양수
} else {
  // x는 음수 또는 0
}
```

Sign 격자에서:
- x = ⊤ (input의 결과는 unknown)
- 조건부 분기에서도 Sign 격자는 이를 구별하지 못함
- 최소 고정점: x = ⊤
- 의미상 정확한 답: x는 양수 또는 음수 (두 가지 경로로 분리)

또 다른 예:
```
x = 5;
y = x + 1;
```

Sign 격자에서:
- x = + (양수)
- y = + (양수 + 양수 = 양수)
- 의미상 정확한 답: y = 6 (정수 격자)
- 최소 고정점: y = + (덜 정확함)

복잡도 분석:

1. **격자 높이**:
   - 높이 h라면, 최대 h번의 반복 필요
   - Sign 격자: 높이 5, 최대 5번
   - (Var → Sign)³: 높이 5³ = 125, 최대 125번

2. **f(x) 계산 비용**:
   - 변수 개수 n에 대해 O(n) 이상
   - 복잡한 연산들의 합

3. **동등성 검사**:
   - 두 상태의 동등성 비교: O(n)
   - 매 반복마다 수행

총 시간 복잡도:
```
O(height × (cost(f) + cost(equality)))
```

### 배경 지식

프로그램 분석은 추상화로 인한 정밀도 손실과 계산 비용 사이의 트레이드오프입니다. 더 정교한 추상 영역(예: 구간 격자, 정다각형)은 더 정밀하지만 계산 비용이 높습니다.

---

## Slide 19: 부등식 제약 조건

> - Some analyses can yield inequations
> - We can rewrite them as equations:
>   - x ⊒ f(x) is equivalent to x = x ⊔ f(x)
>   - x ⊑ f(x) is equivalent to x = x ⊓ f(x)

### 개념 설명

어떤 분석에서는 등식이 아니라 부등식 제약 조건이 나타날 수 있습니다. 이들은 간단한 변환을 통해 등식으로 변환할 수 있습니다.

### 상세한 예시

**경우 1: x ⊒ f(x) (x가 f(x)보다 크거나 같음)**

이를 등식으로 변환:
```
x = x ⊔ f(x)
```

왜 이것이 동등한가?
- x ⊒ f(x) ⟺ x ⊔ f(x) = x
- 따라서 x = x ⊔ f(x)

구체적 예:
```
x ⊒ ⊤라는 제약이 있다면:
x = x ⊔ ⊤ = ⊤
따라서 x ≥ ⊤, 즉 x = ⊤
```

**경우 2: x ⊑ f(x) (x가 f(x)보다 작거나 같음)**

이를 등식으로 변환:
```
x = x ⊓ f(x)
```

왜 이것이 동등한가?
- x ⊑ f(x) ⟺ x ⊓ f(x) = x
- 따라서 x = x ⊓ f(x)

구체적 예:
```
x ⊑ +라는 제약이 있다면:
x = x ⊓ +
```

### 배경 지식

부등식 제약은 일부 분석에서 자연스럽게 나타납니다. 예를 들어, 흐름 불감지 분석(flow-insensitive analysis)에서는 "x의 최종 값은 최소한 어떤 값 이상"이라는 형태의 제약이 생깁니다. 이들을 등식으로 변환하면 같은 고정점 알고리즘을 사용할 수 있습니다.

---

## Slide 20: 요약

> - Solving constraints can be formulated as finding a fixed point for a function over a lattice
> - Monotone functions on complete lattices have a least fixed point
> - The naive fixed point algorithm iterates f from ⊥ until convergence

### 개념 설명

이 강의의 핵심 메시지를 세 가지로 정리합니다.

### 상세한 예시 및 종합

**1. 제약 조건을 고정점 문제로**
```
프로그램 분석 제약:
  x₀ = initial state
  x₁ = transfer function for statement 1
  x₂ = transfer function for statement 2
  ...

고정점 공식화:
  x = f(x)

이를 통해 루프가 있는 복잡한 제어 흐름도 단순한 함수의 고정점으로 표현
```

**2. 타르스키의 정리 + 클린의 정리**
- 타르스키: 완전 격자 위의 단조 함수는 최소 고정점을 가짐
- 클린: 유한 높이의 경우 알고리즘으로 계산 가능

**3. 순진한 알고리즘**
```
x ← ⊥
while x ≠ f(x):
  x ← f(x)
```
이 간단한 알고리즘이 최소 고정점을 찾을 수 있음

### 전체적인 맥락

프로그램 분석의 수학적 기초:
1. **추상 영역**: 프로그램의 값을 단순화한 격자
2. **전이 함수**: 각 프로그램 지점에서의 상태 변환
3. **제약 조건 시스템**: 전이 함수들의 연립
4. **고정점**: 제약 조건 시스템의 해
5. **최소 고정점**: 가장 정밀한 분석 결과

이 모든 것이 격자 이론과 고정점 이론으로 수학적으로 정당화됩니다.

### 배경 지식

프로그램 분석의 건전성(soundness)은 전이 함수의 단조성에서 비롯됩니다. 단조 함수는 "더 정밀한 입력이 덜 정밀한 출력을 만들지 않는다"는 직관을 보장하며, 이는 분석 결과가 보수적(conservative)이라는 것을 의미합니다.

---

## 추가 학습 자료

### 주요 개념 복습
- **격자(Lattice)**: 부분 순서를 가지고 join과 meet 연산이 정의된 대수 구조
- **단조 함수(Monotone Function)**: 순서를 보존하는 함수, 프로그램 분석의 핵심 성질
- **고정점(Fixed Point)**: f(x) = x를 만족하는 x
- **최소 고정점(Least Fixed Point)**: 모든 고정점보다 작은 고정점, 가장 정밀한 분석 결과

### 역사적 배경
- 타르스키(Alfred Tarski, 1955): 고정점의 존재성 증명
- 클린(Stephen Kleene, 1952): 고정점의 계산 방법 제시
- 현대 프로그램 분석: 1970년대 쿠신(Patrick Cousot)의 추상 해석(Abstract Interpretation) 이론으로 발전
