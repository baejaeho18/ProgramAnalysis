# Interprocedural Analysis - 강의 슬라이드 해설

CSE552 Program Analysis — Lecture 10

---

## 슬라이드 1: 제목 슬라이드

### 원문 내용

> Interprocedural Analysis
> CSE552 Program Analysis — Lecture 10
> Jaemin Hong

### 해설

**개념 설명**

이 강의는 절차간 분석(Interprocedural Analysis)이라는 주요 프로그램 분석 주제를 다룹니다. 지금까지 학습한 절차내 분석(Intraprocedural Analysis)은 각 함수를 독립적으로 분석했지만, 이 강의에서는 함수 호출을 통해 여러 함수 간에 정보가 전파되는 방식을 다룹니다.

**강의의 목표**

- 함수 호출을 포함한 전체 프로그램을 분석하는 방법 학습
- 절차간 분석의 구체적인 접근 방식들 이해
- 정확도와 비용의 트레이드오프 학습
- 실제 분석 도구에서 사용되는 기법들 습득

---

## 슬라이드 2: Interprocedural Analysis 개념

### 원문 내용

> **Intraprocedural analysis**: analyzes the body of each individual function in isolation
>
> **Interprocedural analysis**: analyzes the whole program containing multiple functions and function calls

### 해설

**개념 설명**

절차간 분석(Interprocedural Analysis)의 기본 개념을 정의합니다.

- **절차내 분석(Intraprocedural Analysis)**: 각 함수를 별도로, 다른 함수와의 상호작용을 무시하고 분석합니다. 예를 들어, 함수 내에서 호출하는 함수의 동작을 정확히 알 수 없으므로 보수적으로 처리합니다.

- **절차간 분석(Interprocedural Analysis)**: 전체 프로그램의 함수 호출 관계를 고려하여 분석합니다. 함수 A가 함수 B를 호출할 때, B의 동작이 A의 분석 결과에 영향을 미칩니다.

**왜 이것이 중요한가?**

절차내 분석만으로는 프로그램의 동작을 정확히 이해할 수 없습니다. 예를 들어:
- 함수의 반환값이 무엇인지 알 수 없음
- 함수가 전역 변수를 수정하는지 알 수 없음
- 포인터가 어디를 가리키는지 알 수 없음

절차간 분석은 이러한 정보들을 함수 간에 전파하여 더 정확한 분석을 가능하게 합니다.

**맥락**

절차간 분석은 프로그램 분석의 가장 도전적인 문제 중 하나입니다. 이 강의는 이 문제를 해결하는 여러 가지 방법을 제시합니다.

---

## 슬라이드 3: Interprocedural Analysis를 위한 CFG

### 원문 내용

> [다이어그램]
> - 왼쪽: 호출 함수 내의 코드 `x = f(e1,...,en)` 와 그 이후의 코드들
> - 오른쪽: 호출되는 함수 f의 CFG (entry, 함수 본체, return)
> - 화살표로 호출 지점과 함수의 entry/return을 연결

### 해설

**개념 설명**

절차간 분석을 수행하기 위해서는 CFG(제어 흐름 그래프)를 단일 프로그램 전체를 포함하도록 확장해야 합니다.

**CFG 구조**

- **호출 노드(Call Node)**: 함수를 호출하는 명령문. 예: `x = f(e1,...,en)`
- **진입 노드(Entry Node)**: 호출된 함수의 시작점. 함수 f의 entry node는 `entry(f)`로 표기합니다.
- **반환 노드(Return Node)**: 함수가 반환하는 위치. 모든 return 문은 단일 가상의 return 노드로 통합됩니다.
- **호출 후 노드(After-call Node)**: 함수 호출이 완료된 후 실행될 코드

**구체적 예시**

호출 지점 `x = f(e1,...,en)`이 있을 때:
1. 호출 노드에서 함수 f의 entry node로 점프
2. 함수 f의 본체를 실행
3. return 노드에서 호출 후 노드로 분기하여 계속 실행

**왜 이렇게 구성하는가?**

이러한 구조를 통해 데이터 흐름을 함수 경계를 넘어 추적할 수 있습니다. 호출 함수의 변수가 피호출 함수의 매개변수로 어떻게 매핑되고, 피호출 함수의 반환값이 호출 함수의 변수로 어떻게 매핑되는지 모델링합니다.

---

## 슬라이드 4: Transfer Functions — Call Node

### 원문 내용

> x = f(e1, ..., en):
>
> t_v(σ) = { [x1 ↦ eval(e1,σ), ..., xn ↦ eval(en,σ)], entry(f)), (σ[x ↦ σ_return(RET)], after(v)) }
>
> where
> - x1, ..., xn are the parameters of f
> - entry(f) is the entry node of f
> - σ_return is the state at the return node of f
> - after(v) is the after-call node

### 해설

**개념 설명**

호출 노드(Call Node)에서의 전이 함수(Transfer Function)는 매개변수 전달과 반환값 처리를 정의합니다.

**구체적으로 무엇을 하는가?**

호출 `x = f(e1, ..., en)`이 현재 상태 σ에서 실행될 때:

1. **매개변수 설정**: 함수 f의 매개변수 x1, ..., xn을 호출 지점의 실제 인수 e1, ..., en의 값으로 매핑합니다.
   - 예: `x1 ↦ eval(e1, σ)` - e1을 현재 상태에서 평가하여 x1에 할당

2. **함수 진입**: 설정된 매개변수 상태에서 함수 f의 entry node로 진입합니다.

3. **반환값 처리**: 함수 f가 반환될 때 (σ_return 상태에서), 반환값을 변수 x에 할당하고 호출 후 노드(after(v))로 분기합니다.

**상세한 예시**

```c
fn foo(a) { RET = a + 1; return; }
x = foo(5);  // 호출 지점
```

호출 지점에서:
1. a ↦ eval(5, σ) = 5로 설정하고 foo의 entry로 진입
2. foo 내에서 RET = a + 1 = 6 계산
3. 반환 후 x ↦ 6으로 설정하고 호출 후 노드에서 계속 실행

**배경 지식**

- σ는 추상 상태(abstract state)로, 변수들의 값을 추상적으로 표현합니다
- eval(e, σ)는 표현식 e를 상태 σ에서 평가하는 함수입니다
- [x ↦ v]는 상태를 업데이트하는 표기법으로, 변수 x의 값을 v로 설정합니다

---

## 슬라이드 5: Transfer Functions — Return Node

### 원문 내용

> return:
>
> t_v(σ) = { (σ_v [x_i ↦ σ(RET)], after(v_i)), ... }
>
> where
> - v_i is a call node
> - σ_v is the state at v_i
> - x_i is the variable assigned the return value at v_i
> - after(v_i) is the after-call node for v_i

### 해설

**개념 설명**

반환 노드(Return Node)에서의 전이 함수는 함수가 반환될 때 호출 지점으로 제어를 되돌리는 방식을 정의합니다. 중요한 점은 **모든 호출 지점으로 동시에 정보가 전파된다**는 것입니다.

**구체적으로 무엇을 하는가?**

함수 f의 return 노드에서 상태 σ가 있을 때:

1. 이 함수를 호출한 **모든 호출 지점 v_i**에 대해:
   - v_i에서의 상태 σ_v를 가져옵니다
   - RET(반환값)을 v_i에서 할당받은 변수 x_i에 매핑합니다
   - 결과 상태에서 v_i의 호출 후 노드로 진행합니다

2. 따라서 한 번의 return에서 **여러 호출 지점으로 정보가 분기**됩니다.

**상세한 예시**

```c
fn foo(a) {
  RET = a + 1;
  return;
}

x = foo(5);  // 호출 지점 v1
y = foo(10); // 호출 지점 v2
```

foo의 return 노드에서:
- v1로의 반환: x ↦ RET (v1에서의 상태), after(v1)로 이동
- v2로의 반환: y ↦ RET (v2에서의 상태), after(v2)로 이동

**왜 이렇게 하는가?**

한 함수가 여러 곳에서 호출될 수 있으므로, return에서는 각 호출 지점의 특정 상황에 맞게 정보를 전파해야 합니다. 이를 통해 각 호출 지점 이후의 변수 상태를 정확히 결정할 수 있습니다.

---

## 슬라이드 6: Example 1

### 원문 내용

```c
fn foo(a) {
  // a: [1, 1]
  RET = a + a;
  // RET: [2, 2]
  return;
}

x = 1;
// x: [1, 1]
y = foo(x);
// x: [1, 1], y: [2, 2]
z = x + y;
// z: [3, 3]
```

### 해설

**개념 설명**

첫 번째 간단한 예시로, 절차간 분석의 기본 메커니즘을 보여줍니다. 여기서는 정수 값의 범위(interval)를 추적하는 분석을 사용합니다.

**상세한 추적**

1. **foo 함수 내부**:
   - a의 범위: [1, 1] (정확히 1)
   - RET = a + a = 1 + 1 = 2
   - RET의 범위: [2, 2] (정확히 2)

2. **main 함수**:
   - x = 1 할당, x의 범위: [1, 1]
   - foo(x) 호출: x의 값 [1, 1]을 a로 전달
   - foo가 반환되면 RET의 범위 [2, 2]를 y에 할당
   - y = [2, 2]
   - z = x + y = [1, 1] + [2, 2] = [3, 3]

**왜 이것이 중요한가?**

절차간 분석이 없다면, foo 함수의 동작을 모를 것이므로 y의 범위를 결정할 수 없습니다. 절차간 분석을 통해 foo의 반환값이 정확히 2임을 알 수 있고, 따라서 z의 범위도 정확히 결정할 수 있습니다.

---

## 슬라이드 7: Example 2 — Code and CFG

### 원문 내용

```c
fn foo(a) {
  if a <= 0 {
    RET = 0;
  } else {
    b = foo(a - 1);
    RET = b + 1;
  }
  return;
}

x = foo(10);
```

[CFG 다이어그램: foo 내부의 if-else 구조와 recursive call을 보여줌]

### 해설

**개념 설명**

이제 더 복잡한 예시로, **재귀 함수(Recursive Function)**를 다룹니다. 재귀는 절차간 분석을 어렵게 만드는 주요 요소입니다.

**코드 분석**

```c
fn foo(a) {
  if a <= 0 {           // 조건: a <= 0
    RET = 0;           // 기저 사례 (Base case)
  } else {
    b = foo(a - 1);    // 재귀 호출
    RET = b + 1;       // 재귀 호출의 결과에 1을 더함
  }
  return;
}

x = foo(10);
```

**함수의 의미**

이 함수는 입력된 음이 아닌 정수의 값을 그대로 반환하는 함수입니다:
- foo(0) = 0
- foo(1) = foo(0) + 1 = 1
- foo(2) = foo(1) + 1 = 2
- ...
- foo(10) = 10

**CFG 구조**

- **Entry**: foo의 시작
- **If a ≤ 0**: 조건 분기
  - True 경로: RET = 0 → return
  - False 경로: b = foo(a - 1) 호출 (재귀) → RET = b + 1 → return
- **Return**: 함수의 종료

**재귀의 도전성**

foo가 스스로를 호출하기 때문에, 분석이 무한히 깊어질 수 있습니다:
- foo(10) → foo(9) → foo(8) → ... → foo(0)

다음 슬라이드에서 이 문제를 어떻게 해결하는지 보겠습니다.

---

## 슬라이드 8: Example 2 — Analysis Iteration (1)

### 원문 내용

```c
// 반복 0 (초기 상태)
fn foo(a) {
  // a: [10, 10]
  if a <= 0 {
    RET = 0;
  } else {
    // a: [10, 10]
    b = foo(a - 1);
    RET = b + 1;
  }
  return;
}

x = foo(10);
```

### 해설

**개념 설명**

절차간 분석은 고정점(Fixed Point) 알고리즘을 사용합니다. 초기 상태에서 시작하여 반복적으로 정보를 정제합니다.

**반복 0의 상태**

초기에:
- foo(a)는 a = [10, 10]으로 호출됩니다
- a의 범위는 [10, 10]입니다
- a ≤ 0 조건은 거짓입니다 (10 > 0)
- 따라서 else 분기로 이동

**문제점**

이 단계에서:
- foo(a - 1) = foo(9)를 호출하려고 합니다
- 하지만 아직 foo가 a = 9인 경우의 동작을 분석하지 않았습니다
- 그래서 b의 범위를 모릅니다 → 보수적으로 [⊥, ⊥] (불가능) 또는 [음수, 양수]로 설정
- RET = b + 1도 불확실합니다

**반복의 목적**

다음 반복에서 더 많은 정보를 수집하여 이러한 불확실성을 줄입니다.

---

## 슬라이드 9: Example 2 — Analysis Iteration (2)

### 원문 내용

```c
// 반복 1 (첫 번째 정제)
fn foo(a) {
  // a: [-inf, 10]
  if a <= 0 {
    RET = 0;
  } else {
    // a: [-inf, 10]
    b = foo(a - 1);
    RET = b + 1;
  }
  // RET: [0, 1]
  return;
}

x = foo(10);
// x: [0, inf]
```

### 해설

**개념 설명**

첫 번째 반복에서, 분석은 몇 가지 새로운 정보를 수집합니다.

**반복 1에서 일어나는 일**

1. **a의 범위 확장**:
   - 초기에는 a = [10, 10]
   - 하지만 foo가 a - 1로 스스로를 호출하므로, a = 9, 8, 7, ... 등의 값도 가능합니다
   - 따라서 a의 범위는 [-∞, 10]으로 확장됩니다

2. **조건 분석**:
   - a ≤ 0인 경우: RET = 0
   - a > 0인 경우: RET = b + 1 (b의 범위는 아직 불확실)

3. **RET 범위**:
   - a ≤ 0인 경우: RET = 0
   - a > 0인 경우: RET ≥ 1 (b ≥ 0이면)
   - 따라서 RET: [0, 1] (또는 더 큰 범위)

4. **호출 결과**:
   - x = foo(10)이므로 x의 범위는 [0, ∞]로 확장됩니다

**수렴을 향한 단계**

이 과정은 a의 범위가 고정될 때까지 계속됩니다.

---

## 슬라이드 10: Example 3

### 원문 내용

```c
fn foo(a) {
  // a: [1, 2]
  RET = a + a;
  // RET: [2, 4]
  return;
}

if input() {
  x = 1;
  // x: [1, 1]
  y = foo(x);
  // y: [2, 4]
} else {
  x = 2;
  // x: [2, 2]
  z = foo(x);
  // z: [2, 4]
}
```

### 해설

**개념 설명**

이 예시는 **조건부 실행(Conditional Execution)**을 다룹니다. 같은 함수가 서로 다른 입력값으로 여러 번 호출될 수 있습니다.

**분석 과정**

1. **if 분기**:
   - x = 1
   - foo(1) 호출: a ↦ 1
   - foo 내에서 RET = 1 + 1 = 2
   - y ↦ 2

2. **else 분기**:
   - x = 2
   - foo(2) 호출: a ↦ 2
   - foo 내에서 RET = 2 + 2 = 4
   - z ↦ 4

3. **foo 함수의 일반화**:
   - foo는 a = 1과 a = 2 두 경우로 호출됩니다
   - foo 내의 a의 범위: [1, 2]
   - RET = a + a의 범위: [2, 4]

**중요한 관찰**

만약 절차간 분석이 없다면:
- foo의 매개변수 a가 취할 수 있는 모든 값을 모르므로
- RET의 정확한 범위를 결정할 수 없습니다

절차간 분석은 모든 호출 지점의 정보를 수집하여 함수의 입력/출력 관계를 일반화합니다.

---

## 슬라이드 11: Example 4 — Code and CFG

### 원문 내용

```c
fn foo(a) {
  RET = a + 1;
  return;
}

x = foo(1);
y = foo(x);
```

[CFG 다이어그램: 두 번의 호출이 순차적으로 연결됨]

### 해설

**개념 설명**

이 예시는 **데이터 의존성(Data Dependency)**을 보여줍니다. 한 함수의 결과가 다른 함수의 입력이 되는 경우입니다.

**코드 분석**

```c
fn foo(a) {
  RET = a + 1;   // 입력에 1을 더함
  return;
}

x = foo(1);      // x = 1 + 1 = 2
y = foo(x);      // y = 2 + 1 = 3
```

**CFG 흐름**

1. **첫 번째 호출: x = foo(1)**
   - foo의 entry: a ↦ 1
   - RET = a + 1 = 2
   - return: x ↦ 2, after-call 노드로 이동

2. **두 번째 호출: y = foo(x)**
   - x의 값은 이전 호출의 결과 2
   - foo의 entry: a ↦ 2
   - RET = a + 1 = 3
   - return: y ↦ 3

**왜 이것이 도전적인가?**

첫 번째 호출의 결과(x)가 두 번째 호출의 입력이 됩니다. 절차간 분석은 이러한 데이터 흐름을 올바르게 추적해야 합니다.

**CFG의 특징**

다이어그램에서 볼 수 있듯이:
- x = foo(1) 호출 지점에서 foo의 entry로 연결
- foo의 return에서 after-call 노드로 연결
- 그 다음 y = foo(x) 호출 지점으로 연결
- 이는 순차적 데이터 흐름을 모델링합니다

---

## 슬라이드 12: Example 4 — Analysis Iterations

### 원문 내용

```c
// 반복 0
fn foo(a) {
  // a: [1, 1]
  RET = a + 1;
  // RET: [2, 2]
  return;
}

x = foo(1);
// x: [2, 2]
y = foo(x);
// y: [2, 2]

// 반복 1
fn foo(a) {
  // a: [1, inf]
  RET = a + 1;
  // RET: [2, inf]
  return;
}

x = foo(1);
// x: [2, inf]
y = foo(x);
// y: [2, inf]
```

### 해설

**개념 설명**

이 예시는 고정점 알고리즘의 동작을 명확히 보여줍니다.

**반복 0 (초기 상태)**

```
x = foo(1)
  → a = 1
  → RET = 1 + 1 = 2
  → x = 2

y = foo(x) // x = 2에서의 값을 사용
  → a = 2 (from x)
  → RET = 2 + 1 = 3
  → y = 3
```

이론상 y = [2, 2]이지만, 다음 반복에서 정제됩니다.

**반복 1 (첫 번째 정제)**

foo가 받는 a의 범위를 다시 계산합니다:
- 첫 번째 호출: x = foo(1) → a = [1, 1]
- 두 번째 호출: y = foo(x) → a = [2, 2] (x가 [2, 2]였으므로)

하지만 분석은 보수적이므로, foo의 a는 [1, ∞]로 확장됩니다 (모든 호출을 고려).

실제로는:
- a = [1, 1] (첫 호출) 또는 a = [2, 2] (두 번째 호출)
- 합집합하면 a = [1, 2]

하지만 슬라이드에서는 [1, ∞]로 표기했으므로, 이는 보수적 상한(Conservative Upper Bound)을 나타냅니다.

**왜 반복이 필요한가?**

재귀나 복잡한 함수 호출 패턴이 있을 때, 한 번의 분석으로 정확한 결과를 얻을 수 없습니다. 반복을 통해 점점 더 정확한 결과로 수렴합니다.

---

## 슬라이드 13: Context Sensitivity

### 원문 내용

> **Context-insensitive analysis**
> - Does not distinguish between different calls to the same function
> - Heavily suffers from interprocedurally invalid paths
>   - e.g., dataflow from one call node propagates to all after-call nodes
>
> **Context-sensitive analysis**
> - Distinguishes different calls
> - (Context → (State ∪ {unreachable}))"
> - If Context = {}, it is context-insensitive

### 해설

**개념 설명**

절차간 분석에서 **문맥 민감성(Context Sensitivity)**은 매우 중요한 개념입니다. 같은 함수가 여러 곳에서 호출될 때, 각 호출의 문맥(Context)을 구분할지 여부를 결정합니다.

**Context-Insensitive Analysis (문맥 무시 분석)**

정의:
- 같은 함수에 대한 모든 호출을 구분하지 않습니다
- 모든 호출 지점에서 나온 데이터가 섞입니다

문제점 - 구체적 예:

```c
fn foo(a) {
  RET = a + 1;
  return;
}

// 호출 1
x = foo(1);  // x should be 2

// 호출 2
y = foo(10); // y should be 11
```

Context-insensitive 분석:
- foo의 a: [1, 1] ∪ [10, 10] = [1, 10] (모든 호출 섞임)
- RET: [2, 11] (부정확)
- x: [2, 11], y: [2, 11] (둘 다 부정확)

**Interprocedurally Invalid Paths (절차간 무효 경로)**

더 심각한 문제:

```c
fn foo(a) {
  if (a > 5) {
    RET = a;
  } else {
    RET = 0;
  }
  return;
}

x = foo(1);   // 항상 x = 0
y = foo(10);  // 항상 y = 10

// 그 후 ...
if (x > 5) {  // 항상 거짓인데, context-insensitive에서는 참이 될 수도
  // ...
}
```

Context-insensitive 분석에서:
- foo의 a: [1, 10]
- RET: [0, 10] (0과 10 모두 가능하다고 생각)
- x: [0, 10], y: [0, 10]
- 따라서 `x > 5`가 참일 수 있다고 (잘못) 분석

**Context-Sensitive Analysis (문맥 민감 분석)**

정의:
- 각 호출의 문맥(Context)을 구분합니다
- Context는 호출 스택을 나타냅니다

개선된 분석:
```c
// Context = C1: foo(1) 호출
foo(a) @ C1: a = [1, 1], RET = [0, 0]
x = foo(1); // x: [0, 0]

// Context = C2: foo(10) 호출
foo(a) @ C2: a = [10, 10], RET = [10, 10]
y = foo(10); // y: [10, 10]
```

각 호출마다 정확한 결과를 얻습니다.

**공식 표현**

(Context → (State ∪ {unreachable}))
- 각 문맥에 대해, 그 문맥에서의 상태를 매핑
- unreachable: 그 문맥에서 도달 불가능한 경우

Context = {}인 경우, Context-sensitive 분석이 Context-insensitive로 변합니다 (모든 호출을 하나로 취급).

**왜 중요한가?**

- **정확도**: Context-sensitive 분석이 훨씬 더 정확합니다
- **비용**: 하지만 분석 비용도 훨씬 더 높습니다 (호출 스택 깊이에 따라 exponential)

다음 슬라이드에서 이를 구현하는 방법을 봅니다.

---

## 슬라이드 14: Context-Sensitive Transfer Functions

### 원문 내용

> t_v : State × Context → P(State × Node × Context)
>
> x = e:
>
> t_v(σ, c) = { (unreachable, succ(v), c) } if σ = unreachable
>            { (σ[x ↦ eval(σ, e)], succ(v), c) } if σ ≠ unreachable

### 해설

**개념 설명**

Context-sensitive 분석의 전이 함수(Transfer Function)를 정의합니다. 이제 상태뿐만 아니라 문맥도 함께 전파됩니다.

**함수 시그니처**

```
t_v : State × Context → P(State × Node × Context)
```

이는:
- **입력**: 상태 σ 와 문맥 c
- **출력**: (상태, 노드, 문맥) 튜플의 집합 (P는 Power Set)

즉, 한 번의 전이에서 여러 (상태, 노드, 문맥) 조합이 생성될 수 있습니다.

**일반적인 명령어: x = e**

```c
σ에서 x = e를 실행할 때:
```

경우 1: σ = unreachable (도달 불가능)
- 결과: (unreachable, succ(v), c)
- 설명: 도달 불가능한 상태에서는 계속 도달 불가능합니다

경우 2: σ ≠ unreachable (도달 가능)
- 결과: (σ[x ↦ eval(σ, e)], succ(v), c)
- 설명:
  - x ↦ eval(σ, e): 변수 x를 표현식 e의 평가값으로 설정
  - succ(v): 다음 노드로 이동
  - c: 문맥은 변하지 않음

**예시**

```c
x = 5;  // 명령어
```

현재 상태 σ = {y: [1, 3]}, 문맥 c = C1:

실행 후:
- 새로운 상태: σ' = {y: [1, 3], x: [5, 5]}
- 다음 노드로 이동
- 문맥은 여전히 C1

**왜 Context가 중요한가?**

같은 상태 σ라도, 다른 호출 경로(문맥)에서 나온 것이면 다르게 취급합니다:
- (σ, C1): 호출 경로 C1에서의 상태 σ
- (σ, C2): 호출 경로 C2에서의 상태 σ

이 둘은 구별되어, 각각 다른 이후 노드들로 전파됩니다.

---

## 슬라이드 15: Fixed Point Algorithm

### 원문 내용

> PropagationWithWideningAndContexts(t1, ..., tn, σ_start):
>   ((σ1, ..., σm), ...) := ((σ_start, 1, 1), ...)
>   W := {(v1, c1), ..., (vm, cm), ...}
>   while W ≠ ∅ :
>     (v, c) := W.removeOne()
>     Y := t_v(x_v, c)
>     for (y, v', c') ∈ Y :
>       z := x_v ∇ y
>       if x_v ≠ z :
>         x_v := z
>         W.add((v', c'))
>   return x

### 해설

**개념 설명**

절차간 분석의 핵심 알고리즘입니다. 이는 **고정점 계산(Fixed Point Computation)**을 수행하며, **와이딩(Widening)**을 사용합니다.

**알고리즘 구조**

**1. 초기화**
```
((σ1, ..., σm), ...) := ((σ_start, 1, 1), ...)
```
- 각 노드 (v, c)에 대해 초기 상태를 설정
- 보통은 모두 ⊥ (bottom, 불가능한 상태)로 시작하고, 시작 노드만 σ_start로 설정

**2. 작업 큐 초기화**
```
W := {(v1, c1), ..., (vm, cm), ...}
```
- 분석해야 할 (노드, 문맥) 쌍들을 작업 큐에 추가

**3. 반복 루프**
```
while W ≠ ∅ :
```

루프의 각 반복에서:

a) **작업 선택**
```
(v, c) := W.removeOne()
```
큐에서 하나의 (노드, 문맥) 쌍을 꺼냅니다.

b) **전이 함수 적용**
```
Y := t_v(x_v, c)
```
- 현재 노드 v에서의 상태 x_v와 문맥 c에 대해
- 전이 함수 t_v를 적용하여 결과 Y를 얻습니다
- Y는 (상태, 다음노드, 문맥) 튜플들의 집합

c) **상태 업데이트와 전파**
```
for (y, v', c') ∈ Y :
```
Y의 각 결과에 대해:

- **Widening 적용**
```
z := x_v ∇ y
```
와이딩 연산자 ∇를 사용하여, 새 상태 y와 기존 상태 x_v를 합칩니다.
와이딩의 목적: 무한 상태 공간을 유한한 근사로 축소

- **변화 확인**
```
if x_v ≠ z :
  x_v := z
  W.add((v', c'))
```
상태가 변했으면:
  - 상태를 새로운 z로 업데이트
  - 다음 노드 (v', c')를 작업 큐에 추가

상태가 변하지 않으면 (이미 수렴), 다음 노드를 추가하지 않습니다.

**4. 결과 반환**
```
return x
```
모든 노드와 문맥에 대한 최종 상태들을 반환합니다.

**와이딩의 역할**

와이딩이 없으면:
```c
fn foo(a) {
  x = 0;          // x: [0, 0]
  while (a > 0) {
    x = x + 1;   // 반복 1: x: [0, 1]
                 // 반복 2: x: [0, 2]
                 // 반복 3: x: [0, 3]
                 // ... 무한히 계속
  }
}
```

와이딩 적용:
```
x ∇ [0, 1] = [0, ∞]  // 더 이상 변하지 않음
```

와이딩은 분석을 강제로 수렴시킵니다.

**알고리즘의 특징**

- **Worklist Algorithm**: 변화가 있는 노드만 재처리 (효율적)
- **Sound (건전성)**: 모든 가능한 실행 경로를 고려
- **May Analysis**: 부정확하지만, 항상 안전한 over-approximation

**복잡도 분석**

최악의 경우:
- 노드 수: O(n)
- 문맥 수: O(k^m) where k = call sites, m = context depth
- 각 노드 방문당 O(1) 시간 (위키/링크 추가 제외)
- 전체: O(n × k^m × m)

---

## 슬라이드 16: Call String Approach — Overview

### 원문 내용

> **k-call-site sensitivity**
> **Call = {v ∈ Node | v is a call node}**
> **Context = Call^<k** where k ∈ ℤ⁺
>
> - (v1, v2, ..., vm) identifies the topmost m call sites
> - The abstract state at (v, (v1, v2, ..., vm)) approximates the runtime state at v, where
>   - The function containing v is called from v1
>   - The function containing v1 is called from v2
>   - ...
>   - The function containing v_{m-1} is called from v_m

### 해설

**개념 설명**

**Call String Approach**는 Context-sensitive 분석을 구현하는 가장 직관적인 방법입니다. 문맥을 호출 사이트들의 문자열(시퀀스)로 표현합니다.

**기본 개념**

**Call Site (호출 지점)**
- 함수를 호출하는 프로그램의 위치
- 각 호출 지점은 고유한 노드 v로 식별됩니다

**k-Call-Site Sensitivity**
- 최대 k개의 호출 사이트를 추적합니다
- k가 클수록 더 정확하지만, 비용도 증가합니다

**Context의 구조**

Context = Call^<k 는:
- Call^<k: k 이하의 호출 사이트들의 시퀀스 모음
- 예: Call^<2 = {ε, (v1), (v1, v2)} where ε는 빈 문맥

**구체적 예시 (k=1, Call site 민감도)**

```c
fn foo(a) {         // foo 내의 모든 코드
  RET = a + 1;
  return;
}

fn main() {
  // 호출 지점 v1
  x = foo(1);

  // 호출 지점 v2
  y = foo(10);
}
```

k=1일 때:
- foo 내의 상태는 호출 지점으로 구분됩니다
- (foo의 첫 줄, (v1)): a = 1에서의 상태
- (foo의 첫 줄, (v2)): a = 10에서의 상태
- 두 호출의 영향이 분리됩니다

**호출 스택 추적**

예시 프로그램:

```c
fn foo(x) {
  return x + 1;
}

fn bar(y) {
  return foo(y) * 2;
}

fn main() {
  return bar(5);
}
```

실행 스택:
```
main → bar(5) → foo(5) → return 6 → bar → return 12 → main
```

호출 스택: (v_bar, v_foo)
- v_foo는 foo를 호출하는 bar 내의 위치
- v_bar는 bar를 호출하는 main 내의 위치

k=2일 때:
- (foo의 코드, (v_foo, v_bar)): 이 호출 스택에서의 상태
- 호출 깊이 제한으로 인해, 더 깊은 호출은 truncate됩니다

**k의 의미**

- **k=0**: Context-insensitive (모든 호출을 구분하지 않음)
- **k=1**: 가장 최근 호출 사이트만 추적
- **k=2**: 최근 2개의 호출 사이트 추적
- **k=∞**: 전체 호출 스택 추적 (불가능할 수 있음)

---

## 슬라이드 17: Call String Approach — Context Interpretation

### 원문 내용

> - ε is an empty tuple, representing the initial call context
> - (v1, v2, ..., vm) where m < k represents call stacks of height exactly m
>   - v_m must be a call node in main
> - (v1, v2, ..., v_k) represents call stacks of height at least k
>   - Call strings longer than k are truncated

### 해설

**개념 설명**

Call String 방식에서 문맥이 어떻게 표현되고 해석되는지를 설명합니다.

**문맥의 종류**

**1. 빈 문맥: ε**
- 의미: 프로그램 시작점 (main의 최상위)
- 호출 스택이 없음 (깊이 0)
- 초기 상태에 사용됨

예시:
```c
fn main() {
  x = 1;  // 문맥: ε
}
```

**2. 부분 호출 스택: (v1, v2, ..., vm) where m < k**
- 의미: 정확히 m개의 호출 사이트로 이루어진 호출 스택
- v_m: main의 호출 사이트 (최상단)
- v_{m-1}: v_m에서 호출한 함수 내의 호출 사이트
- ...
- v_1: 현재 함수로의 진입점

예시 (k=3):

```c
fn foo(a) {
  x = a + 1;      // 문맥: (v_call_to_foo)
}

fn bar() {
  foo(5);         // v_call_to_foo: bar 내의 호출 지점
}

fn main() {
  bar();          // v_call_to_bar: main 내의 호출 지점
}
```

주요 코드 실행 위치에서:
- foo의 x = a + 1: 문맥 = (v_call_to_foo) [m=1]
  - bar는 (v_call_to_bar)에서 호출됨을 추적

**3. 전체 호출 스택: (v1, v2, ..., v_k) where m ≥ k**
- 의미: k개 이상의 깊이를 가진 호출 스택
- 가장 깊은 k개 호출 사이트만 추적
- 더 오래된 호출은 버림 (truncate)

예시 (k=2):

```c
fn a() { ... }
fn b() { a(); }
fn c() { b(); }
fn d() { c(); }

fn main() {
  d();
}
```

호출 순서: main → d → c → b → a

a 실행 시 호출 스택:
- 실제 스택: (v_d, v_c, v_b) [깊이 3]
- k=2이므로 가장 최근 2개만 유지: (v_c, v_b)
- v_d는 버림

이유: 분석 비용 제한
- k=∞이면 깊은 재귀에서 무한 많은 문맥이 생성됨
- k를 고정하면 문맥 수를 제한할 수 있음

**v_m이 main의 call node여야 하는 이유**

호출 스택의 최상단 (v_m)은 항상 main에서의 호출이어야 합니다:
- main은 다른 함수에서 호출되지 않습니다
- 따라서 호출 스택의 루트는 main의 호출 사이트여야 합니다

예외:
- ε (빈 문맥): 프로그램 시작, main 자체

**Truncation의 영향**

Call string truncation은:
- **장점**: 분석 비용 감소, 유한한 문맥 수 보장
- **단점**: 정확도 감소, 깊은 재귀의 문맥 정보 손실

예시:

```c
fn foo(x) { RET = x + 1; return; }

fn recursive(n) {
  if (n <= 0) return;
  x = foo(n);
  recursive(n - 1);
}

fn main() {
  recursive(100);
}
```

k=1:
- recursive 내 모든 recursive 호출은 같은 문맥 (v_rec)으로 취급
- 따라서 각 재귀 깊이를 구분하지 못함

k=∞:
- 각 재귀 깊이마다 다른 문맥
- recursive(100) → recursive(99) → ... 각각 구분
- 하지만 무한 문맥이 가능

---

## 슬라이드 18: Call String Transfer Functions — Push and Call

### 원문 내용

> push((v1, ..., vm), v0) = { (v0, v1, ..., vm) } if m < k
>                          { (v0, v1, ..., v_{m-1}) } if m = k
>
> x = f(e1, ..., en):
>
> t_v(σ, c) = { ([x1 ↦ eval(e1, σ), ..., xn ↦ eval(en, σ)], entry(f), c'), ...}
>
> where
> - x1, ..., xn are the parameters of f
> - entry(f) is the entry node of f
> - c' = push(c, v)
> - σ_return,c' is the state at the return node of f with context c'
> - after(v) is the after-call node

### 해설

**개념 설명**

Call string 방식에서 함수 호출 시 문맥을 어떻게 업데이트하는지를 정의합니다.

**Push 함수의 역할**

```
push((v1, ..., vm), v0) = ...
```

이 함수는:
- 입력: 현재 문맥 (v1, ..., vm)과 새로운 호출 지점 v0
- 출력: 업데이트된 문맥

**경우 1: m < k (스택이 아직 k 미만)**

```
push((v1, ..., vm), v0) = (v0, v1, ..., vm)
```

- 새로운 호출 지점 v0을 가장 앞에 추가 (most recent position)
- 기존 문맥은 그대로 유지
- 결과 길이: m + 1

예시 (k=3, m=1):

```c
fn main() {
  foo();          // v1: foo 호출 지점
}

fn foo() {
  bar();          // v2: bar 호출 지점
}

fn bar() {
  x = 1;
}
```

- main에서 foo 호출: c = ε
  - push(ε, v1) = (v1)
- foo에서 bar 호출: c = (v1)
  - push((v1), v2) = (v2, v1)
- bar 실행: c = (v2, v1)

**경우 2: m = k (스택이 이미 k)**

```
push((v1, ..., vm), v0) = (v0, v1, ..., v_{m-1})
```

- 새로운 호출 지점 v0을 맨 앞에 추가
- 가장 오래된 v_m을 제거 (truncate)
- 결과 길이: k (유지)

예시 (k=2, m=2):

```
push((v1, v2), v3) = (v3, v1)  // v2 제거
```

이렇게 하면 k를 초과하는 호출 스택은 추적하지 않습니다.

**함수 호출 시 Transfer Function**

```c
x = f(e1, ..., en):

t_v(σ, c) = { ([x1 ↦ eval(e1, σ), ..., xn ↦ eval(en, σ)], entry(f), c'), ... }
```

함수 호출 `x = f(e1, ..., en)`이 상태 σ와 문맥 c에서 실행될 때:

1. **매개변수 설정**
   ```
   [x1 ↦ eval(e1, σ), ..., xn ↦ eval(en, σ)]
   ```
   - 호출 지점의 실제 인수들을 평가하여
   - 함수의 매개변수 x1, ..., xn에 매핑

2. **함수 진입**
   ```
   entry(f)
   ```
   - 호출된 함수 f의 시작 노드로 이동

3. **문맥 업데이트**
   ```
   c' = push(c, v)
   ```
   - 현재 문맥 c에 호출 지점 v를 추가하여 새로운 문맥 c'를 생성
   - 이제 호출된 함수 내의 모든 분석은 c'를 사용합니다

**반환 처리**

Return 노드에서:
```
σ_return,c'
```
- 함수 f의 return 노드에서의 상태 (c' 문맥)
- 이것이 호출 함수로 반환되는 값입니다

**구체적 예시**

```c
fn foo(a) {
  RET = a + 1;
  return;
}

fn main() {
  x = foo(5);   // v_call: main 내 호출 지점
}
```

1. main에서 foo 호출 (c = ε):
   - push(ε, v_call) = (v_call)
   - foo의 entry로 이동, c' = (v_call)
   - a ↦ 5로 설정

2. foo 내 실행 (c' = (v_call)):
   - RET = a + 1 = 6 계산
   - return 노드에서 σ_return,(v_call) = {a: 5, RET: 6}

3. Return (c' = (v_call)):
   - after-call 노드로 복귀
   - x ↦ 6 설정

---

## 슬라이드 19: Call String Transfer Functions — Return

### 원문 내용

> return:
>
> t_v(σ, c) = { (σ_v,c [x_i ↦ σ(RET)], after(v_i), c_i), ... }
>
> where
> - v_i is a call node and c_i is a context at v_i where push(c_i, v_i) = c
> - σ_v,c is the state at v_i with c_i
> - x_i is the variable assigned the return value at v_i
> - after(v_i) is the after-call node for v_i

### 해설

**개념 설명**

Call string 방식에서 함수 반환 시, 어떻게 문맥에 맞는 호출 지점으로 돌아가는지를 정의합니다.

**핵심 아이디어**

함수 f의 return 노드에서 상태 σ와 문맥 c가 있을 때:

1. **문맥 역추적**
   ```
   push(c_i, v_i) = c
   ```
   - 현재 문맥 c는 호출 지점 v_i에서 push 연산의 결과입니다
   - 따라서 c_i (호출 지점의 문맥)를 역으로 복원해야 합니다
   - 예: c = (v_call), c_i = ε, v_i = v_call

2. **호출 지점으로 돌아가기**
   - v_i (호출 지점)에서의 상태 σ_v,c를 가져옵니다
   - 그 상태에 반환값을 할당합니다

3. **후속 처리**
   - after-call 노드로 분기
   - 원래 문맥 c_i로 복귀

**구체적 동작**

```c
x = f(e1, ..., en):  // 호출 지점: v_i, 문맥: c_i

t_v(σ, c) = { (σ_v,c [x_i ↦ σ(RET)], after(v_i), c_i), ... }
```

f의 return에서:
- σ: 함수 f의 return 상태 (문맥 c = push(c_i, v_i)에서)
- σ(RET): 반환값

처리:
1. σ_v,c: v_i에서 호출할 때의 상태 (정보를 저장해둬야 함)
2. 반환값을 x_i에 할당: σ_v,c [x_i ↦ σ(RET)]
3. after(v_i): 호출 후 노드로 이동
4. c_i: 호출 지점의 원래 문맥으로 복귀

**예시**

```c
fn foo(a) {
  RET = a + 1;
  return;            // return 노드
}

fn main() {
  x = foo(5);       // v_call, 문맥 c = ε
}
```

foo의 return 노드에서:
- 현재 문맥: c = (v_call)
- σ = {a: 5, RET: 6}
- σ(RET) = 6

역추적:
- push(c_i, v_call) = (v_call)를 만족하는 c_i = ε를 찾음
- v_i = v_call (호출 지점)
- σ_v_call,ε: main의 x = foo(5) 직전의 상태, 예: {x: ⊥, y: [1, 3]}

반환:
- σ_v_call,ε [x ↦ 6] = {x: 6, y: [1, 3]}
- after(v_call): foo 호출 후의 다음 명령어로 이동
- 문맥: ε (main으로 복귀)

**왜 σ_v,c를 저장해야 하는가?**

여러 호출 지점이 같은 함수를 호출할 수 있습니다:

```c
fn foo(a) { RET = a + 1; return; }

fn main() {
  x = foo(1);   // v1, c_i = ε
  y = foo(2);   // v2, c_i = ε
}
```

foo의 return에서:
- 경우 1: c = (v1), σ_v1,ε [x ↦ RET] → x값 설정
- 경우 2: c = (v2), σ_v2,ε [y ↦ RET] → y값 설정

각 호출 지점의 상태를 분리 저장하여, return에서 올바른 호출 지점으로 정보를 전파합니다.

**복수 호출 지점 (중첩 호출)**

```c
fn foo(a) { RET = a; return; }
fn bar(a) { b = foo(a); RET = b + 1; return; }

fn main() {
  c = bar(5);   // v1
}
```

bar의 return:
- 현재 문맥: c = (v_foo, v1) (foo 호출 후 bar의 return)
  - v1: bar를 호출하는 main의 위치
  - v_foo: foo를 호출하는 bar의 위치

역추적:
- push(c_i, v_foo) = (v_foo, v1)를 만족하려면 c_i = (v1)
- σ_v_foo,(v1): bar 내에서 foo 호출 직전 상태

이렇게 중첩된 호출도 올바르게 처리됩니다.

---

## 슬라이드 20: Call String Example 1 (k=1)

### 원문 내용

```c
fn foo(a) {
  // (C1) a: [1, 1]
  // (C2) a: [2, 2]
  RET = a + a;
  // (C1) RET: [2, 2]
  // (C2) RET: [4, 4]
  return;
}

if input() {
  x = 1;
  // x: [1, 1]
  y = foo(x);  // C1
  // y: [2, 2]
} else {
  x = 2;
  // x: [2, 2]
  z = foo(x);  // C2
  // z: [4, 4]
}
```

### 해설

**개념 설명**

Call string 방식 (k=1)을 사용한 실제 분석 예시입니다. 두 호출 지점을 구분하여 정확한 결과를 얻습니다.

**호출 지점 정의**

- **C1**: if 분기의 `y = foo(x)` 호출 지점
- **C2**: else 분기의 `z = foo(x)` 호출 지점

**분석 과정**

**1. C1을 통한 호출: y = foo(x) with x = 1**

문맥 c_i = ε (main 또는 if 분기의 시작):
- push(ε, C1) = (C1)
- foo 내에서 c = (C1)
- a = [1, 1]
- RET = 1 + 1 = [2, 2]
- return:
  - reverse_push((C1), C1) = ε
  - y ↦ [2, 2]

**2. C2를 통한 호출: z = foo(x) with x = 2**

문맥 c_i = ε:
- push(ε, C2) = (C2)
- foo 내에서 c = (C2)
- a = [2, 2]
- RET = 2 + 2 = [4, 4]
- return:
  - reverse_push((C2), C2) = ε
  - z ↦ [4, 4]

**foo 함수의 분석 결과**

문맥별 상태:
- (foo 진입, (C1)): a = [1, 1]
- (foo 진입, (C2)): a = [2, 2]
- (RET = a + a, (C1)): RET = [2, 2]
- (RET = a + a, (C2)): RET = [4, 4]

**정확성 비교**

**Context-insensitive (k=0)**
- foo의 a: [1, 1] ∪ [2, 2] = [1, 2]
- RET = a + a: [2, 4] (부정확)
- y: [2, 4], z: [2, 4] (둘 다 부정확)

**Context-sensitive (k=1)**
- (foo, (C1)): a = [1, 1], RET = [2, 2]
- (foo, (C2)): a = [2, 2], RET = [4, 4]
- y: [2, 2], z: [4, 4] (정확)

k=1을 사용하면 각 호출의 영향을 정확히 분리할 수 있습니다.

**언제 k=1이 불충분한가?**

```c
fn foo(a) { return a + 1; }
fn bar(a) { return foo(a) * 2; }

fn main() {
  x = bar(5);   // v1
  y = bar(10);  // v2
}
```

k=1:
- (bar, (v1)): a = 5
- (bar, (v2)): a = 10
- (foo, (v_foo_in_bar)): a = [5, 10] (구분 안 됨)

bar 내의 foo 호출은 같은 호출 지점 v_foo_in_bar이므로, 구분되지 않습니다.

이 경우 k=2가 필요합니다:
- (foo, (v_foo_in_bar, v1)): a = 5
- (foo, (v_foo_in_bar, v2)): a = 10

---

## 슬라이드 21: Call String Example 2 (k=1)

### 원문 내용

```c
fn foo(a) {
  // (C1) a: [1, 1]
  RET = a + 1;
  // (C1) RET: [2, 2]
  return;
}

x = foo(1);   // C1
// x: [2, 2]
y = foo(x);   // C2
```

[오른쪽: 반복 후 결과]

```c
fn foo(a) {
  // (C1) a: [1, 1]
  // (C2) a: [2, 2]
  RET = a + 1;
  // (C1) RET: [2, 2]
  // (C2) RET: [3, 3]
  return;
}

x = foo(1);   // C1
// x: [2, 2]
y = foo(x);   // C2
// y: [3, 3]
```

### 해설

**개념 설명**

이 예시는 **데이터 흐름 의존성(Data Flow Dependency)**에서 call string 방식의 이점을 보여줍니다.

**코드 분석**

```c
fn foo(a) {
  RET = a + 1;
  return;
}

x = foo(1);    // 호출 C1
y = foo(x);    // 호출 C2: x는 C1의 결과에 의존
```

C1의 결과가 C2의 입력이 됩니다.

**왼쪽: 초기 분석 (반복 0)**

**C1: x = foo(1)**
- 문맥: (C1)
- foo의 a: [1, 1]
- RET: [2, 2]
- x: [2, 2]

**C2: y = foo(x)**
- x = [2, 2]에서 foo 호출
- 문맥: (C2)
- foo의 a: [2, 2]
- RET: [3, 3]
- y: [3, 3]

하지만 분석은 반복적입니다. 초기에는 C1의 결과를 모를 수 있으므로:

**오른쪽: 수렴 후 분석**

반복을 통해, 모든 호출의 정보가 수집되면:

**foo의 최종 분석**
- (C1) context: a = [1, 1] → RET = [2, 2]
- (C2) context: a = [2, 2] → RET = [3, 3]

**호출자 입장**
- x = foo(1) → x = [2, 2] (C1 결과)
- y = foo([2, 2]) → y = [3, 3] (C2 결과)

**정확성 분석**

각 호출이 분리되어 분석되므로:
- C1과 C2의 영향이 섞이지 않음
- 각 호출의 정확한 입출력 관계 파악 가능
- 최종 결과가 정확함

**Context-insensitive와의 비교**

Context-insensitive (k=0):
- foo의 a: [1, 1] ∪ [2, 2] = [1, 2] (모든 호출 섞임)
- RET: [2, 3]
- x: [2, 3], y: [2, 3] (부정확)

---

## 슬라이드 22: Call String Example 3 (k=1)

### 원문 내용

```c
fn bar(c) {
  // (C3) c: [1, 2]
  RET = c;
  // (C3) RET: [1, 2]
  return;
}

fn foo(a) {
  // (C1) a: [1, 1]
  // (C2) a: [2, 2]
  // (C1) b: [1, 2]
  // (C2) b: [1, 2]
  b = bar(a);   // C3
  RET = b + b;
  // (C1) RET: [2, 4]
  // (C2) RET: [2, 4]
  return;
}

// C3 정의: foo 내에서의 bar 호출
if input() {
  x = 1;
  // x: [1, 1]
  y = foo(x);   // C1
  // y: [2, 4]
} else {
  x = 2;
  // x: [2, 2]
  z = foo(x);   // C2
  // z: [2, 4]
}
```

### 해설

**개념 설명**

이 예시는 **중첩된 함수 호출(Nested Function Calls)**에서 call string 방식이 어떻게 동작하는지를 보여줍니다.

**호출 관계**

```
main
  ├─ C1: foo(1)
  │  └─ C3: bar(a)
  │     └─ return a
  └─ C2: foo(2)
     └─ C3: bar(a)  (같은 호출 지점이지만 다른 입력)
        └─ return a
```

**분석 단계**

**1. C1: foo(1) 분석**

foo 내에서 문맥 (C1):
- a = [1, 1]
- b = bar(a) 호출:
  - 호출 지점 C3, 문맥 = push((C1), C3) = (C3, C1) (k=1이므로 truncate 안 함)
  - 하지만 k=1이면 (C3)만 유지됨

실제로는 k=1 설정에서:
- push((C1), C3) = (C3) (C1 버림, 길이 제한 k=1)

bar 내에서 문맥 (C3):
- c = a = [1, 1]
- RET = [1, 1]

foo로 돌아와서 (문맥 C1):
- b = [1, 1]
- RET = b + b = [2, 2]

**2. C2: foo(2) 분석**

foo 내에서 문맥 (C2):
- a = [2, 2]
- b = bar(a) 호출:
  - push((C2), C3) = (C3)

bar 내에서 문맥 (C3):
- c = a = [2, 2]
- RET = [2, 2]

foo로 돌아와서 (문맥 C2):
- b = [2, 2]
- RET = b + b = [4, 4]

**슬라이드의 분석 결과**

반복 후:
- (foo, (C1)): a = [1, 1], b = [1, 2], RET = [2, 4]
- (foo, (C2)): a = [2, 2], b = [1, 2], RET = [2, 4]

**주목할 점**

bar는 호출 지점 C3로만 호출되지만:
- C3의 문맥에서 c가 [1, 2]가 됨
- 왜냐하면 C3이 foo 내에서 호출되는데, foo가 (C1), (C2) 두 문맥에서 실행되기 때문

foo 자체는 각 문맥별로 구분되지만, 중첩 호출 C3은 모든 foo의 실행을 섞습니다.

**깊이 제한의 영향**

k=1 제한으로 인해:
- (C3, C1)과 (C3, C2)를 구분하지 않음
- 따라서 bar의 입력을 정확히 분리하지 못함

k=2로 증가시키면:
- (bar, (C3, C1)): c = [1, 1], RET = [1, 1]
- (bar, (C3, C2)): c = [2, 2], RET = [2, 2]
- 각 호출을 정확히 추적

---

## 슬라이드 23: Call String Example 4 (k=2)

### 원문 내용

```c
fn bar(c) {
  // (C3, C1) c: [1, 1]
  // (C3, C2) c: [2, 2]
  RET = c;
  // (C3, C1) RET: [1, 1]
  // (C3, C2) RET: [2, 2]
  return;
}

fn foo(a) {
  // (C1) a: [1, 1]
  // (C2) a: [2, 2]
  b = bar(a);   // C3
  // (C1) b: [1, 1]
  // (C2) b: [2, 2]
  RET = b + b;
  // (C1) RET: [2, 2]
  // (C2) RET: [4, 4]
  return;
}

if input() {
  x = 1;
  // x: [1, 1]
  y = foo(x);   // C1
  // y: [2, 2]
} else {
  x = 2;
  // x: [2, 2]
  z = foo(x);   // C2
  // z: [4, 4]
}
```

### 해설

**개념 설명**

이제 k=2 (2-call-site sensitivity)를 사용하여, 이전 예시를 더 정확히 분석합니다.

**문맥의 구조**

k=2일 때 가능한 문맥들:
- ε: 프로그램 시작
- (C1): foo만 호출
- (C2): foo만 호출
- (C3, C1): foo(C1) 내에서 bar 호출
- (C3, C2): foo(C2) 내에서 bar 호출
- 더 깊은 호출은 truncate

**분석 과정**

**1단계: main에서 foo 호출**

C1: foo(1)
- 문맥: ε
- push(ε, C1) = (C1)
- foo 내에서 문맥 (C1)

C2: foo(2)
- 문맥: ε
- push(ε, C2) = (C2)
- foo 내에서 문맥 (C2)

**2단계: foo 내에서 bar 호출**

foo의 문맥 (C1):
- a = [1, 1]
- b = bar(a) 호출 (C3)
- 문맥: (C1)
- push((C1), C3) = (C3, C1)
- bar 내에서 문맥 (C3, C1)

foo의 문맥 (C2):
- a = [2, 2]
- b = bar(a) 호출 (C3)
- 문맥: (C2)
- push((C2), C3) = (C3, C2)
- bar 내에서 문맥 (C3, C2)

**3단계: bar 분석**

bar의 문맥 (C3, C1):
- c = 1 (foo(1)에서의 a)
- RET = 1

bar의 문맥 (C3, C2):
- c = 2 (foo(2)에서의 a)
- RET = 2

**4단계: 반환 및 결과**

foo(C1)로 돌아옴:
- b = 1 (bar의 (C3, C1) 결과)
- RET = 1 + 1 = 2
- y = 2

foo(C2)로 돌아옴:
- b = 2 (bar의 (C3, C2) 결과)
- RET = 2 + 2 = 4
- z = 4

**정확성 비교**

**k=0 (Context-insensitive)**
- y: [2, 4], z: [2, 4] (부정확)

**k=1 (1-call-site sensitive)**
- foo: 구분 가능
- bar: 구분 불가 (C3 호출만 추적)
- y: [2, 4], z: [2, 4] (여전히 부정확)

**k=2 (2-call-site sensitive)**
- foo: 구분 가능 (C1, C2)
- bar: 구분 가능 ((C3, C1), (C3, C2))
- y: [2, 2], z: [4, 4] (정확)

---

## 슬라이드 24: Precision and Cost

### 원문 내용

> - Larger k gives more precision, but also higher cost
> - e.g., height((Call^<k → Var → Sign)") = O(c^k · m · n)
>   - where c = |Call| and m = |Var|
> - In practice, k = 1 sometimes gives inadequate precision, and k ≥ 2 is generally too expensive
> - It is common to select k individually for each call site, based on heuristics

### 해설

**개념 설명**

Call string 방식의 정확도와 비용의 트레이드오프를 분석합니다.

**정확도 증가**

k가 클수록:
- 더 깊은 호출 스택을 추적
- 더 많은 호출 문맥을 구분
- 분석 결과의 정확도 증가

예시:
- k=0: 모든 호출 구분 안 함
- k=1: 최근 1개 호출 추적
- k=2: 최근 2개 호출 추적
- k=∞: 전체 호출 스택 추적

**비용 증가**

복잡도 분석:

```
height((Call^<k → Var → Sign)") = O(c^k · m · n)
```

여기서:
- c = |Call|: 호출 지점의 수
- m = |Var|: 변수의 수
- n = 노드의 수
- Sign: 부호 분석 (Sign = {+, -, 0})

**복잡도 해석**

- **c^k**: Call string의 가능한 조합
  - 각 깊이에서 c개의 호출 지점 선택 가능
  - k개 깊이 → c^k 개의 문맥

- **m · n**: 각 (노드, 문맥)마다 m개 변수의 상태

**구체적 예시**

프로그램에 호출 지점 10개, 변수 5개, 노드 100개, Sign 분석:

- **k=0**: 1 · 5 · 100 = 500 상태
- **k=1**: 10 · 5 · 100 = 5,000 상태
- **k=2**: 100 · 5 · 100 = 50,000 상태
- **k=3**: 1,000 · 5 · 100 = 500,000 상태

k가 1 증가하면 10배 증가합니다!

**실무에서의 고려사항**

**k = 1의 문제점**
```c
fn foo(a) { return a + 1; }
fn bar(a) { return foo(a) * 2; }

fn main() {
  x = bar(5);   // 35
  y = bar(10);  // 70
}
```

k=1에서:
- bar의 두 호출을 구분하지만
- foo(a)의 a = [5, 10]이 되어 부정확

**k ≥ 2의 문제점**
- 비용이 exponential
- 깊은 재귀에서 특히 문제
- 메모리 폭발 가능

**실무 해결책**

```
선택적 k 설정 (Selective k)
```

각 호출 지점마다 서로 다른 k를 사용:

```c
fn foo(a) { ... }      // 호출 지점별로
fn bar(a) {
  return foo(a);       // k=1 (충분)
}
fn baz(a) {
  return bar(a);       // k=2 (필요)
}
```

**Heuristics 예시**

1. **콜드 함수(Cold Function)**: k=0 (비용 절감)
2. **핫 함수(Hot Function)**: k=2 이상 (정확도 중요)
3. **재귀 함수**: k=1 (깊이 제한)
4. **라이브러리 함수**: k=0 (분석 외부)

**정리**

- 정확도와 비용은 수학적으로 연관 (trade-off)
- 현실적으로는 k=1이 자주 사용됨 (불완전하지만 관리 가능)
- k=2는 일반적으로 비용이 너무 높음
- 선택적 k 설정이 현실적인 해결책

---

## 슬라이드 25: Call String Example 5 (k=1, Sign Domain)

### 원문 내용

```c
fn foo(a) {
  // (C1) a: +
  // (C2) a: +
  RET = a + a;
  // (C1) RET: +
  // (C2) RET: +
  return;
}

if input() {
  x = 1;
  // x: +
  y = foo(x);  // C1
  // y: +
} else {
  x = 2;
  // x: +
  z = foo(x);  // C2
  // z: +
}
```

### 해설

**개념 설명**

이 예시는 더 간단한 추상 도메인인 **Sign Domain**을 사용합니다. Sign Domain은 각 변수가 양수(+), 음수(-), 또는 0인지를 추적합니다.

**Sign Domain**

- **+**: 양수 (양의 정수)
- **-**: 음수 (음의 정수)
- **0**: 정확히 0
- **⊥**: 불가능 (도달 불가능한 상태)

Join 연산:
- + ⊔ - = (양수 또는 음수, 즉 모든 정수)
- + ⊔ 0 = ≥0 (0 포함 양수)
- 등등

**분석 과정**

**Input 분석**
```c
x = 1;
// x: +  (양의 정수 1)

x = 2;
// x: +  (양의 정수 2)
```

**C1: foo(x) with x = +**
- 문맥: (C1)
- a = + (foo의 입력)
- RET = a + a:
  - + + + = +
  - 양수 + 양수 = 양수
- RET: +

**C2: foo(x) with x = +**
- 문맥: (C2)
- a = + (foo의 입력)
- RET = a + a = +
- RET: +

**최종 결과**
- y: + (C1의 반환값)
- z: + (C2의 반환값)

**Sign Domain의 장점**

1. **단순성**
   - Interval [1, 1], [2, 2]보다 간단
   - Sign {+, -, 0}만 추적

2. **효율성**
   - 상태 공간이 작음
   - 계산 빠름

3. **실용성**
   - 부호 검사 (양수/음수 여부)
   - null 검사와 함께 자주 사용
   - 범위 검사는 필요 없을 때

**제한사항**

```c
x = 5;   // x: +
y = 10;  // y: +
z = x + y;
// z: + (정확)

but:
x = 1;   // x: +
y = 2;   // y: +
z = x / y;
// z: +? 아니면 다른가?
```

Sign 분석만으로는:
- 5 / 10 = 0.5 (실수)
- 1 / 2 = 0 (정수 나눗셈)
- 구분 불가

하지만 Sign 분석의 목표가 정수 범위가 아닌 부호만 추적하는 것이므로, 이는 의도된 제한입니다.

---

## 슬라이드 26: Functional Approach — Overview

### 원문 내용

> **Context = State**
>
> A lattice element for a CFG node v is a map
> m_v : State → State ∪ unreachable
>
> m_v(σ) approximates the runtime state at v, where the function containing v is called with σ as the abstract state at the call site
>
> If v is the exit node of a function f, m_v is a summary of f, mapping abstract entry states to abstract exit states
> - Transfer function: models the effect of executing a single instruction
> - Function summary: models the effect of executing an entire function

### 해설

**개념 설명**

지금까지 다룬 Call String Approach와는 다른 접근법인 **Functional Approach**를 소개합니다. 여기서 문맥은 호출 스택이 아닌 **상태 자체(State)**입니다.

**핵심 아이디어**

Call String Approach:
- 문맥 = 호출 스택 (v1, v2, ..., vk)
- 같은 함수가 다른 호출 경로에서는 다른 분석

Functional Approach:
- 문맥 = 호출 지점의 상태 σ
- 같은 함수가 같은 상태로 호출되면 같은 결과
- 함수를 독립적인 상태 변환자(State Transformer)로 모델링

**Lattice Element**

```
m_v : State → State ∪ unreachable
```

각 노드 v에 대해:
- m_v는 함수 (입력: 추상 상태, 출력: 추상 상태)
- m_v(σ): σ 상태에서 v에 도달했을 때의 상태

**예시**

```c
x = y + 1;  // 명령어 v
```

m_v:
- m_v(σ) = σ[x ↦ eval(σ[y]) + 1]
- "현재 상태 σ에서 x를 y+1로 설정한 상태"

**함수 요약 (Function Summary)**

함수 f의 exit 노드에서:
```
m_exit = 함수 f의 요약
```

이는:
- 입력: 함수 진입 시의 추상 상태
- 출력: 함수 종료 시의 추상 상태
- 기능: 함수 내부 모든 명령어의 합성 효과

**예시**

```c
fn foo(a) {
  b = a + 1;
  RET = b * 2;
  return;
}
```

함수 요약 m_foo:
- 입력: a의 범위 (예: [1, 5])
- 처리:
  - b = a + 1 → [2, 6]
  - RET = b * 2 → [4, 12]
- 출력: RET = [4, 12]

즉, m_foo(σ) = σ[RET ↦ (σ(a) + 1) * 2]

**Transfer Function vs Function Summary**

**Transfer Function** (단일 명령어)
```c
x = y + 1;
t_v(σ) = σ[x ↦ σ(y) + 1]
```

**Function Summary** (전체 함수)
```c
fn foo(a) {
  RET = a * 2 + 1;
  return;
}
m_foo(σ) = σ[RET ↦ σ(a) * 2 + 1]
```

함수 요약은 함수의 모든 transfer function을 합성합니다.

**왜 "Functional"이라 불리는가?**

함수를 컴퓨터 과학의 함수 개념으로 모델링합니다:
- 함수의 입출력 관계를 부분 함수(Partial Function)로 표현
- 함수의 동작을 상태 변환으로 캡슐화
- 함수 합성으로 프로그램 분석

이는 함수형 프로그래밍의 철학과 유사합니다.

---

## 슬라이드 27: Functional Approach — Transfer Functions

### 원문 내용

> x = f(e1, ..., en):
>
> t_v(σ, c) = { (σ', entry(f), σ'),
>              (σ[x ↦ σ_return,σ'(RET)], after(v), c) }
>
> where σ' = [x1 ↦ eval(e1, σ), ..., xn ↦ eval(en, σ)]
>
> return:
>
> t_v(σ, c) = { (σ_v,c [x_i ↦ σ(RET)], after(v_i), c_i), ... }
>
> where
> - v_i is a call node and c_i is a context at v_i where the call context is σ
> - σ_v,c is the state at v_i with c_i
> - x_i is the variable assigned the return value at v_i
> - after(v_i) is the after-call node for v_i

### 해설

**개념 설명**

Functional Approach에서 함수 호출과 반환을 처리하는 방식을 정의합니다.

**함수 호출: x = f(e1, ..., en)**

```
t_v(σ, c) = { (σ', entry(f), σ'),
              (σ[x ↦ σ_return,σ'(RET)], after(v), c) }
```

두 개의 산출 (output)이 있습니다:

**산출 1: 함수 진입**
```
(σ', entry(f), σ')
```
- σ': 매개변수를 설정한 새로운 상태
  ```
  σ' = [x1 ↦ eval(e1, σ), ..., xn ↦ eval(en, σ)]
  ```
- entry(f): 함수 f의 진입 노드로 이동
- 문맥 c로 σ'을 전파 (여기서 c는 호출 지점의 상태)

**산출 2: 호출 후 처리**
```
(σ[x ↦ σ_return,σ'(RET)], after(v), c)
```

이 부분은 고급 설명입니다:
- σ_return,σ': 함수 f의 반환 노드에서, 입력 상태 σ'일 때의 상태
- σ_return,σ'(RET): 반환값
- σ[x ↦ ...]: 호출 함수의 상태를 x = RET로 업데이트
- after(v): 호출 후 노드로 이동

**핵심 차이점: Call String vs Functional**

Call String:
```
push(c, v): 호출 스택에 v 추가
```

Functional:
```
entry(f)의 문맥: 호출 지점의 상태 σ'
```

상태 자체가 문맥 정보를 담습니다.

**구체적 예시**

```c
fn foo(a) {
  b = a + 1;
  RET = b * 2;
  return;
}

x = foo(5);  // 호출 지점
```

**함수 호출 시:**
1. σ = {x: ⊥}
2. σ' = [a ↦ eval(5, σ)] = {a: 5}
3. entry(foo)로 이동, 문맥 c = σ' = {a: 5}

**함수 내부:**
- 문맥 c = {a: 5}에서:
  - b = a + 1 = 6
  - RET = b * 2 = 12

**Return:**
- σ_return,{a:5}(RET) = 12
- 호출자로 돌아와서 x ↦ 12

**반환: return**

```
t_v(σ, c) = { (σ_v,c [x_i ↦ σ(RET)], after(v_i), c_i), ... }
```

함수 f의 return에서:
- 각 호출 지점 v_i에 대해:
  - σ_v,c: v_i에서의 상태 (문맥 c_i)
  - σ(RET): 반환값
  - σ_v,c [x_i ↦ σ(RET)]: x_i = RET로 업데이트
  - after(v_i), c_i: 호출 후 노드로, 호출자 문맥으로 복귀

**왜 σ_return,σ'를 사용하는가?**

함수를 상태 변환자로 보기:
```
함수 f: State → State
m_f(σ) = 함수 f가 상태 σ에서 시작할 때의 최종 상태
```

호출 지점에서:
1. σ'로 매개변수 설정
2. m_f를 적용하여 m_f(σ')를 계산 (= σ_return,σ')
3. 반환값을 호출자 상태에 반영

---

## 슬라이드 28: Functional Approach — Example

### 원문 내용

```c
fn foo(a) {
  // [a: +] a: +
  RET = a + a;
  // [a: +] RET: +
  return;
}

if input() {
  x = 1;
  // x: +
  y = foo(x);
  // y: +
} else {
  x = 2;
  // x: +
  z = foo(x);
  // z: +
}
```

### 해설

**개념 설명**

Functional Approach를 사용하여 이전 Call String Example과 동일한 코드를 분석합니다.

**분석 방식의 차이**

Call String Approach (이전):
- 호출 지점 C1, C2로 문맥 구분
- foo를 여러 문맥에서 분석

Functional Approach:
- 함수 foo를 "상태 변환자"로 모델링
- 입력 상태에 따라 출력 결과 결정

**분석 과정**

**1단계: 함수 foo의 분석**

```c
fn foo(a) {
  RET = a + a;
  return;
}
```

입력 상태별 분석:
- 입력: [a: +]
  - RET = a + a = + + + = +
  - 출력: [RET: +]

함수 요약 m_foo:
```
m_foo([a: +]) = [RET: +]
```

(a가 음수나 0일 수도 있지만, 호출에서는 a: +로 호출되므로 그 경우만 분석)

**2단계: C1 호출 - y = foo(x) with x = 1**

```c
x = 1;
// x: +
y = foo(x);
```

분석:
1. x = 1이므로 x: +
2. foo를 호출할 때 입력: a: +
3. m_foo([a: +]) = [RET: +]를 적용
4. y ↦ RET = +

**3단계: C2 호출 - z = foo(x) with x = 2**

```c
x = 2;
// x: +
z = foo(x);
```

분석:
1. x = 2이므로 x: +
2. foo를 호출할 때 입력: a: +
3. m_foo([a: +]) = [RET: +]를 적용
4. z ↦ RET = +

**최종 결과**

```
y: +
z: +
```

**Call String vs Functional 비교**

**정확도**

이 예시에서는 둘 다 동일한 정확도:
- Call String: (foo, (C1)), (foo, (C2)) 구분
- Functional: m_foo([a: +])는 하나이지만, 호출 시점의 상태로 결정

**복잡도**

더 복잡한 예시에서 차이 발생:

```c
fn foo(a) { return a + 1; }

fn main() {
  // 100가지 다른 입력으로 foo 호출
  for (i = 0; i < 100; i++) {
    x = foo(i);
  }
}
```

Call String (k=1):
- 100개 호출 지점, 각각 구분
- 100개 문맥에서 foo 분석

Functional:
- foo를 한 번 분석
- m_foo(σ) 함수 정의
- 각 호출에서 m_foo 적용

이 경우 Functional이 더 효율적입니다.

---

## 슬라이드 29: Precision and Cost

### 원문 내용

> - The functional approach gives optimal precision
>   - As precise as if inlining all function calls
>   - Completely avoids the problem with dataflow along interprocedurally invalid paths
>
> - However, very expensive
>   - e.g., State = Var → Sign
>   - height((State → State ∪ unreachable)") = O(5^m · m · n)
>   - where m = |Var|

### 해설

**개념 설명**

Functional Approach의 정확도와 비용을 분석합니다.

**정확도: 최적(Optimal Precision)**

Functional Approach는:
- 함수를 정확하게 모델링
- 각 입력 상태에 따른 정확한 출력 결정
- 절차간 무효 경로 완전히 제거

**비교: Function Inlining**

```c
fn foo(a) { return a + 1; }

x = foo(5);
y = foo(10);
```

인라인 버전:
```c
// Inlined foo(5)
x = 5 + 1;  // x = 6

// Inlined foo(10)
y = 10 + 1; // y = 11
```

분석:
- x = 6 (정확)
- y = 11 (정확)

Functional Approach:
- foo를 함수 객체로 모델링
- m_foo([a: [5, 5]]) → [RET: [6, 6]]
- m_foo([a: [10, 10]]) → [RET: [11, 11]]
- 인라인과 동일한 정확도

**비용: 매우 비쌈**

복잡도:

```
height((State → State ∪ unreachable)") = O(5^m · m · n)
```

여기서:
- m = |Var|: 변수의 수
- n: 노드의 수
- State = Var → Sign: 각 변수의 부호
- Sign = {+, -, 0, ⊥, T}: 5개 원소

**복잡도 분석**

State → State: 상태에서 상태로의 함수
- State 공간: 5^m (각 변수마다 5가지 선택)
- State → State: 5^m에서 5^m로의 함수
- 가능한 함수 개수: (5^m)^(5^m) = 5^(m · 5^m)

매우 큽니다!

더 실용적인 상한:

height((State → State)") = O(5^m · m · n)
- 5^m: 상태 공간 크기
- m: 변수 개수
- n: 노드 개수

**구체적 예시**

m = 5개 변수, n = 100 노드:

```
O(5^5 · 5 · 100) = O(3125 · 5 · 100) = O(1,562,500)
```

k=1인 Call String Approach:

```
O(10 (호출 지점) ^ 1 · 5 · 100) = O(5,000)
```

Functional이 300배 이상 비쌉니다!

**문제점들**

1. **상태 공간 폭발**
   - m이 증가하면 exponential 폭발
   - 10개 변수: 5^10 = 9,765,625 상태

2. **함수 표현**
   - State → State를 어떻게 표현할 것인가?
   - 명시적: 모든 입력 상태마다 출력 정의 (불가능)
   - 암시적: 추상 함수 (복잡함)

3. **분석 비용**
   - 각 함수마다 상태 공간의 모든 조합을 고려
   - 깊은 재귀에서 특히 문제

---

## 슬라이드 30: Practical Variants

### 원문 내용

> - In practice, the functional approach is often applied selectively
>   - Only on some functions, or
>   - Using call contexts that only consider some of the program variables
>
> - **Parameter sensitivity**: call contexts are defined by the abstract values of the function parameters
>   - If no pointers or global variables, it is equivalent to the current approach
>
> - **Object sensitivity**: call contexts are defined by the abstract values of the receiver objects
>   - Popular when analyzing object-oriented programs

### 해설

**개념 설명**

완전한 Functional Approach는 너무 비싸므로, 실무에서는 선택적으로 적용하는 변형들이 있습니다.

**선택적 적용**

완전한 Functional Approach 대신:
1. **함수 선택**: 특정 함수에만 적용
   - 중요한 함수: Functional
   - 단순한 함수: Context-insensitive

2. **변수 선택**: 일부 변수만 추적
   - 중요한 변수: 상태에 포함
   - 무관한 변수: 무시

**Parameter Sensitivity (매개변수 민감도)**

문맥 = 함수 매개변수의 추상값

**정의**

```
Context = 함수 매개변수의 값들
```

예시:

```c
fn foo(a, b) {
  RET = a + b;
  return;
}
```

매개변수: a, b

호출들:
```c
x = foo(1, 2);     // context: (1, 2)
y = foo(1, 3);     // context: (1, 3)
z = foo(10, 20);   // context: (10, 20)
```

각 (매개변수 값) 조합이 다른 문맥입니다.

**구현**

```c
fn foo(a, b) { ... }

// foo의 분석
분석 대상:
- (foo, ([1, 1], [2, 2])):
  a: [1, 1], b: [2, 2]
  ...

- (foo, ([1, 1], [3, 3])):
  a: [1, 1], b: [3, 3]
  ...

- (foo, ([10, 10], [20, 20])):
  a: [10, 10], b: [20, 20]
  ...
```

각 매개변수 조합별로 별도 분석.

**제약: 포인터/전역변수 없을 때만 동등**

```c
int global = 0;

fn foo(a) {
  global = a;
  RET = global;
  return;
}
```

Parameter Sensitivity만:
- 문맥: (a의 값)
- global의 값을 무시하면 부정확

따라서:
- 포인터 있음 → Object Sensitivity 사용
- 전역변수 있음 → 추가 상태 추적

**Object Sensitivity (객체 민감도)**

객체지향 프로그래밍에서:
- 같은 메서드가 다른 객체에서 호출될 수 있음
- 각 객체의 상태는 다름

문맥 = 수신자 객체(Receiver Object)의 추상값

예시 (Java/Python):

```java
class Counter {
  int value = 0;

  void increment() {
    value++;
  }
}

Counter c1 = new Counter();
Counter c2 = new Counter();

c1.increment();  // c1의 value 증가
c2.increment();  // c2의 value 증가 (별개)
```

Object Sensitivity:
- (increment, c1): c1에서의 메서드
- (increment, c2): c2에서의 메서드
- 각각 별도 분석

**복잡도**

Parameter Sensitivity:
```
O(P^p · m · n)
```
- P: 매개변수 범위 (예: Sign이면 5)
- p: 매개변수 개수

Object Sensitivity:
```
O(O^k · m · n)
```
- O: 객체 개수
- k: context depth

**실무 사용**

- **Java/C# 분석**: Object Sensitivity (객체지향)
- **C/Rust 분석**: Parameter Sensitivity (절차지향)
- **함수형 언어**: 전체 Functional Approach (부작용 최소)

**Trade-off**

Call String:
- 정확도: 중간 (호출 깊이로 제한)
- 비용: 낮음
- 구현: 간단

Parameter/Object Sensitivity:
- 정확도: 높음 (언어 특성 반영)
- 비용: 중간
- 구현: 복잡

Functional:
- 정확도: 최고 (최적)
- 비용: 매우 높음
- 구현: 매우 복잡

---

## 슬라이드 31: Summary

### 원문 내용

> - Interprocedural analysis extends intraprocedural analysis to handle function calls
> - Context-insensitive analysis merges information from all call sites, leading to imprecision from interprocedurally invalid paths
> - Context-sensitive analysis distinguishes different calling contexts
> - The call string approach uses sequences of call sites as contexts; k controls the trade-off between precision and cost
> - The functional approach uses abstract entry states as contexts, achieving optimal precision but at potentially high cost

### 해설

**개념 설명**

이 강의의 주요 내용을 정리합니다.

**1. 절차간 분석의 필요성**

절차간 분석(Interprocedural Analysis):
- 절차내 분석(Intraprocedural Analysis): 각 함수를 독립적으로 분석
- 절차간 분석: 함수 호출을 통한 정보 전파를 추적

필요한 이유:
- 함수의 반환값을 결정하기 위해
- 함수의 부작용을 추적하기 위해
- 포인터 흐름을 정확히 파악하기 위해

**2. Context-Insensitive의 문제**

정의: 같은 함수에 대한 모든 호출을 구분하지 않음

문제점:
- 절차간 무효 경로(Interprocedurally Invalid Path) 발생
  - 한 호출에서만 가능한 데이터가 다른 호출에 전파
  - 실제로 발생하지 않는 경로도 분석됨
- 정확도 저하

예:
```c
fn foo(a) {
  if (a > 5) { ... }
}

foo(1);   // a는 항상 ≤ 5
foo(10);  // a는 항상 > 5
```

Context-insensitive:
- a: [1, 10] (모든 호출 섞임)
- 실제로 불가능한 경로 분석

**3. Context-Sensitive의 해결책**

정의: 다른 호출 지점의 문맥을 구분

장점:
- 절차간 무효 경로 제거
- 정확도 증가

**4. Call String Approach**

문맥 = 호출 스택의 시퀀스

특징:
- 직관적: 호출 스택을 직접 모델링
- 구현 용이
- 복잡도 제어 가능 (k로 제한)

k의 의미:
- k=0: Context-insensitive
- k=1: 최근 1개 호출 추적
- k=∞: 전체 호출 스택

Trade-off:
- k ↑: 정확도 ↑, 비용 ↑ (exponential)
- 실무: k=1이 일반적

**5. Functional Approach**

문맥 = 호출 지점의 상태

특징:
- 함수를 상태 변환자(State Transformer)로 모델링
- 최적 정확도 (인라인만큼)
- 절차간 무효 경로 완전 제거

단점:
- 매우 높은 비용 (exponential)
- 상태 공간의 크기로 인한 폭발

실무 변형:
- Parameter Sensitivity: 매개변수 값으로 문맥 정의
- Object Sensitivity: 객체지향 프로그램에서 수신자 객체로 문맥 정의
- 선택적 적용: 일부 함수/변수에만 적용

**6. Trade-off 요약**

정확도 순서:
```
Functional > Call String (큰 k) > Call String (작은 k) > Context-insensitive
```

비용 순서:
```
Functional > Call String (큰 k) > Call String (작은 k) > Context-insensitive
```

**7. 실무 선택**

- 빠른 분석 필요: k=0 (Context-insensitive)
- 합리적 균형: k=1 (Call String)
- 고정확도 필요: Parameter/Object Sensitivity 또는 Functional
- 객체지향 코드: Object Sensitivity
- 절차지향 코드: Parameter Sensitivity 또는 Call String

**다음 강의를 위한 준비**

이제 절차간 분석의 기본을 이해했습니다.
다음 주제들:
- 실제 구현: 어떤 접근 방식이 가장 효율적인가?
- 특정 분석: 포인터 분석, 타입 분석 등에서의 절차간 분석
- 도구와 사례: 실제 분석 도구에서의 적용

**중요 개념 복습**

- **Interprocedural**: 함수 경계를 넘는 정보 흐름
- **Context-sensitive**: 호출 문맥 구분
- **Call String**: 호출 스택 기반 문맥
- **Functional**: 상태 기반 함수 모델링
- **Trade-off**: 정확도 vs 비용의 균형

---

## 마치며

이 강의는 프로그램 분석에서 가장 도전적이면서도 중요한 주제인 절차간 분석(Interprocedural Analysis)을 다루었습니다.

핵심은:
1. **문제 인식**: Context-insensitive의 부정확함 이해
2. **해결책 탐색**: 다양한 문맥 민감 분석 기법 학습
3. **Trade-off 이해**: 정확도와 효율성의 균형 인식
4. **실무 적용**: 각 상황에 맞는 접근 방식 선택

이러한 지식은 정적 분석 도구, 컴파일러 최적화, 보안 분석 등 다양한 분야에서 실제로 활용됩니다.
