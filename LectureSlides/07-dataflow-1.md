# CSE552 프로그램 분석 - 강의 7: 데이터흐름 분석 (1)

강의자: Jaemin Hong

---

## Slide 1: 제목

> Dataflow Analysis (1), CSE552 Program Analysis — Lecture 7, Jaemin Hong

### 강의 개요

이 강의는 데이터흐름 분석(Dataflow Analysis)의 기초를 다룹니다. 데이터흐름 분석은 프로그램의 각 포인트에서 변수들이 가질 수 있는 값의 범위를 추상적으로 계산하는 정적 분석 기법입니다. 이를 통해 컴파일러 최적화, 버그 탐지, 보안 분석 등 다양한 응용이 가능합니다.

---

## Slide 2: 데이터흐름 분석 개요

> - Starts with a CFG and a complete lattice (with a finite height)
> - A lattice element represents abstract information for a CFG node
> - The lattice may be: fixed for all programs, or parameterized by the program
> - To each node v, we assign a constraint variable ⟦v⟧ ranging over the lattice elements

### 개념 설명

데이터흐름 분석은 다음 세 가지 핵심 요소로 구성돼요:

1. **제어 흐름 그래프(CFG)**: 프로그램의 실행 경로를 노드와 간선으로 표현한 그래프입니다.
2. **완전 격자(Complete Lattice)**: 프로그램의 추상적 상태를 나타내는 수학적 구조입니다. 유한한 높이를 가져야 고정점 알고리즘이 종료돼요.
3. **제약 변수**: 각 CFG 노드 v에 대해 ⟦v⟧이라는 변수를 할당하며, 이는 그 노드에서의 추상적 정보를 나타내요.

### 배경 지식

완전 격자(Complete Lattice)는 다음을 만족하는 순서 집합이에요:
- 모든 부분집합에 대해 최소상한(supremum)과 최대하한(infimum)이 존재
- 이를 ⊔(join)과 ⊓(meet) 연산으로 표현합니다.

### 전체적 맥락

격자는 고정되거나 프로그램에 따라 매개변수화될 수 있어요:
- **고정 격자**: 모든 프로그램에 대해 동일한 격자를 사용 (예: 부호 분석)
- **매개변수화된 격자**: 프로그램의 입력 크기 등에 따라 달라지는 격자 (예: 상수 전파)

---

## Slide 3: 데이터흐름 제약과 고정점

> - For each node, we define a dataflow constraint that relates the variable to those of other nodes
> - If all constraints are (in-) equations with monotone right-hand sides, we can use the fixed-point algorithm

### 개념 설명

각 CFG 노드에 대해 제약(constraint)을 정의해요. 이 제약은 해당 노드의 추상 상태를 다른 노드들의 상태와 연결하는 방정식입니다.

핵심은 이 제약들이 **단조 함수(monotone function)**의 형태여야 한다는 거예요:
- 입력이 증가하면 출력도 증가하는 성질
- 이 성질이 보장되면 고정점 알고리즘으로 해를 구할 수 있습니다.

### 배경 지식

고정점(Fixed Point)은 f(x) = x를 만족하는 x를 의미해요. 최소 고정점(lfp - least fixed point)은 모든 고정점 중 가장 작은 것으로, 프로그램 분석에서 우리가 원하는 해입니다.

### 전체적 맥락

단조성이 중요한 이유는:
- 보장된 수렴: 고정점에 반드시 도달합니다.
- 유일성: 최소 고정점이 유일하게 결정됩니다.
- 효율성: 고정점에 도달할 때까지의 반복 횟수가 제한돼요.

---

## Slide 4: 단조 프레임워크

> - The combination of a complete lattice and a space of monotone functions
> - Can be instantiated by specifying the CFG and the rules for assigning dataflow constraints

### 개념 설명

단조 프레임워크(Monotone Framework)는 데이터흐름 분석의 일반적인 틀이에요:

1. **격자**: 추상적 정보를 표현하는 수학적 구조
2. **단조 함수**: 각 CFG 노드에 대한 전이 함수(transfer function)

이 프레임워크를 특정 분석(부호 분석, 상수 전파 등)에 맞게 인스턴스화하면 돼요.

### 배경 지식

프레임워크의 강점은 일반성이에요:
- 다양한 분석 기법을 통일된 방식으로 다룰 수 있습니다.
- 수렴성, 안전성 등의 이론적 성질을 한 번에 증명할 수 있어요.

### 전체적 맥락

이 강의에서는 단조 프레임워크 내에서:
- 부호 분석(Sign Analysis)
- 상수 전파(Constant Propagation)

을 살펴볼 거예요. 둘 다 같은 프레임워크를 기반으로 하지만, 다른 격자와 규칙을 사용합니다.

---

## Slide 5: 문법

> Node v ::= x=e | if x | entry | return
> Expression e ::= x | n | input() | e op e
> (No functions, pointers, and compound types for now)

### 개념 설명

강의에서 분석할 간단한 프로그래밍 언어의 문법이에요:

**노드(Statement) 종류:**
- `x=e`: 변수 x에 식 e의 값을 할당
- `if x`: 변수 x의 값에 따라 분기
- `entry`: 프로그램 시작점
- `return`: 프로그램 종료점

**식(Expression) 종류:**
- `x`: 변수
- `n`: 정수 상수
- `input()`: 외부 입력 (값을 모르므로 추상화)
- `e op e`: 이항 연산 (덧셈, 뺄셈 등)

### 배경 지식

이렇게 간단한 언어를 사용하는 이유:
- 핵심 개념에 집중할 수 있어요
- 함수, 포인터, 복합 타입은 분석을 크게 복잡하게 만드니까요

### 전체적 맥락

실제 프로그램 분석은 이보다 훨씬 복잡한 언어를 다루지만, 원리는 같습니다:
1. 복잡한 구조를 단순하게 모델링
2. 필수적인 정보만 추상화
3. 같은 분석 기법을 적용

---

## Slide 6: 부호 분석 - 격자

> ⟦v⟧ ∈ State = Var → Sign
> [Diamond-shaped Hasse diagram: ⊤ at top, -, 0, + in middle, ⊥ at bottom]
> Stateⁿ expresses information for all CFG nodes, where n is the number of nodes

### 개념 설명

부호 분석(Sign Analysis)은 변수가 음수, 0, 양수 중 어느 범위에 있는지 추적하는 분석이에요.

**Sign 격자:**
```
     ⊤ (unknown/anything)
   /   |   \
  -    0    +  (negative, zero, positive)
   \   |   /
     ⊥ (impossible/bottom)
```

**State의 정의:**
- `State = Var → Sign`: 각 변수가 어떤 부호를 가지는지 매핑
- 예: {a → +, b → -, c → ⊤}는 a는 양수, b는 음수, c는 부호를 알 수 없음을 의미해요

**Stateⁿ:**
- n개의 CFG 노드 각각에 대해 State를 할당한 것
- 전체 프로그램의 추상적 상태를 표현합니다.

### 배경 지식

격자의 순서 관계(≤)는:
- ⊥ ≤ -, 0, +, ⊤ (⊥는 가장 작은 원소)
- -, 0, + ≤ ⊤ (⊤는 가장 큰 원소)
- -, 0, + 사이에는 비교 불가능

이 순서는 정보의 정확성을 나타내요: ⊤은 가장 정보가 적고, -, 0, +는 더 정확해요.

### 전체적 맥락

부호 분석의 용도:
- 0으로 나누기 검출: 나누는 수가 0 또는 ⊤이면 경고
- 경고가 너무 많아서 실무에서는 더 정교한 분석이 필요해요

---

## Slide 7: 부호 분석 - JOIN 연산

> - JOIN(v) combines the abstract states from the predecessors of v:
>   JOIN(v) = ⊔_{u∈pred(v)} ⟦u⟧
> - Precisely speaking, JOIN(v) is a function
>   JOIN(v) : Stateⁿ → State
>   JOIN(v)(⟦v₁⟧, ..., ⟦vₙ⟧) = ⟦vᵢ⟧ ⊔ ⟦vⱼ⟧ ⊔ ···

### 개념 설명

JOIN 연산은 여러 경로에서 노드 v에 도달할 때 이들 경로의 추상 상태를 합치는 작업이에요.

**직관적 의미:**
- 노드 v의 이전 노드들(pred(v)) 모두로부터의 상태를 병합
- ⊔(join) 연산은 두 원소를 모두 포함하는 가장 작은 원소를 찾아요

**수학적 정의:**
```
JOIN(v) : Stateⁿ → State
JOIN(v)(state) = ⊔(⟦u⟧ | u ∈ pred(v))
```

### 배경 지식

JOIN이 필요한 이유:
- 제어 흐름 그래프에는 분기와 합병이 있어요
- 합병점에서 여러 경로가 만날 때, 모든 경로를 고려해야 안전합니다

### 배경 예시

```
    if x
    /  \
   v₃  v₄
    \  /
     v₅
```

v₅에 도착하는 추상 상태는:
- v₃에서의 상태와
- v₄에서의 상태

를 합쳐야 해요. 둘 다 가능하니까요.

---

## Slide 8: 부호 분석 - JOIN 예시

> [CFG diagram with v₁: entry, v₂: if x, v₃: y=z, v₄: y=-5, v₅: return]
> - JOIN(v₁) = ⊥
> - JOIN(v₂) = ⟦v₁⟧
> - JOIN(v₃) = ⟦v₂⟧
> - JOIN(v₄) = ⟦v₂⟧
> - JOIN(v₅) = ⟦v₃⟧ ⊔ ⟦v₄⟧

### 개념 설명

구체적인 CFG에서 JOIN의 계산을 보여줘요:

```
v₁: entry
  ↓
v₂: if x
  / \
v₃:   v₄:
y=z   y=-5
  \ /
v₅: return
```

**각 노드의 JOIN 값:**

| 노드 | 이전 노드 | JOIN 값 | 설명 |
|------|---------|--------|------|
| v₁ | 없음 | ⊥ | 시작점이므로 불가능한 상태(bottom) |
| v₂ | v₁ | ⟦v₁⟧ | v₁에서만 올 수 있음 |
| v₃ | v₂ | ⟦v₂⟧ | v₂의 첫 번째 분기 |
| v₄ | v₂ | ⟦v₂⟧ | v₂의 두 번째 분기 |
| v₅ | v₃, v₄ | ⟦v₃⟧ ⊔ ⟦v₄⟧ | v₃과 v₄ 둘 다에서 올 수 있음 |

### 전체적 맥락

v₅에서의 JOIN이 중요해요:
- y의 값이 v₃을 거쳐 올 수도, v₄를 거쳐 올 수도 있어요
- 따라서 두 경로의 상태를 합쳐야 안전합니다

---

## Slide 9: 부호 분석 - 제약 규칙

> - x=e: ⟦v⟧ = JOIN(v)[x ↦ eval(JOIN(v), e)]
> - eval : (Var → Sign) × Expression → Sign
>   - eval(σ, x) = σ(x)
>   - eval(σ, n) = sign(n)
>   - eval(σ, input()) = ⊤
>   - eval(σ, e₁ op e₂) = ôp(eval(σ, e₁), eval(σ, e₂))

### 개념 설명

각 노드 타입에 대한 제약을 정의하는 규칙이에요.

**할당문 x=e의 규칙:**
```
⟦v⟧ = JOIN(v)[x ↦ eval(JOIN(v), e)]
```

의미: 노드 v에서 x에 e를 할당하면,
1. JOIN(v)로 이전 상태를 구하고
2. e를 평가(eval)해서
3. x의 값을 업데이트

**eval 함수 정의:**

| 식 | 결과 | 설명 |
|----|------|------|
| eval(σ, x) | σ(x) | 변수의 부호는 σ에서 조회 |
| eval(σ, n) | sign(n) | 상수의 부호 (양수면 +, 0이면 0, 음수면 -) |
| eval(σ, input()) | ⊤ | 입력값은 부호를 모르므로 ⊤ |
| eval(σ, e₁ op e₂) | ôp(...) | 연산의 추상적 결과 |

### 배경 지식

상태 업데이트 표기: σ[x ↦ v]는 σ의 복사본인데 x의 값만 v로 바꾼 거예요.

---

## Slide 10: 부호 분석 - 추상적 덧셈

> [5×5 table for abstract addition ˆ+]
> ˆ+  | ⊥  -  0  +  ⊤
> ⊥   | ⊥  ⊥  ⊥  ⊥  ⊥
> -   | ⊥  -  -  ⊤  ⊤
> 0   | ⊥  -  0  +  ⊤
> +   | ⊥  ⊤  +  +  ⊤
> ⊤   | ⊥  ⊤  ⊤  ⊤  ⊤

### 개념 설명

부호의 덧셈을 추상적으로 정의한 표예요. 각 칸은 두 부호의 합의 부호 범위를 나타내요.

**몇 가지 중요한 경우:**

- `- + - = -`: 음수 + 음수 = 항상 음수 ✓
- `+ + + = +`: 양수 + 양수 = 항상 양수 ✓
- `- + + = ⊤`: 음수 + 양수 = 음수일 수도, 양수일 수도, 0일 수도 (어느 쪽이 더 큰지 모르니까)
- `⊥ + anything = ⊥`: 불가능한 값과의 연산도 불가능

### 배경 지식

이 표는 **안전성(soundness)**을 보장하기 위해 설계돼요:
- 실제 가능한 결과를 모두 포함하되,
- 불가능한 결과도 포함할 수 있어요 (과근사, over-approximation)

예: `- + +`의 결과가 항상 ⊤는 아니지만, ⊤로 표현하면 안전해요.

### 예시

```
a = -3, b = 5
a + b = 2 (양수)
하지만 a + b가 음수 또는 양수일 수 있으므로 ⊤로 표현
```

---

## Slide 11: 부호 분석 - 나머지 규칙

> - entry: ⟦v⟧ = ⊤
> - Others: ⟦v⟧ = JOIN(v)
> - While ⊓ exists, we only use ⊔. This is common.

### 개념 설명

다른 노드 타입들에 대한 규칙이에요:

**entry 노드:**
```
⟦entry⟧ = ⊤
```
의미: 프로그램 시작점에서 변수들의 부호는 완전히 미결정 (⊤)

**if x, return 등 다른 노드:**
```
⟦v⟧ = JOIN(v)
```
의미: 변수 할당이 없으므로 상태는 그대로 전파

### 배경 지식

**왜 ⊓(meet)를 사용하지 않을까?**

격자 이론에서:
- ⊔는 최소상한(supremum) - 여러 원소를 포함하는 가장 작은 원소
- ⊓은 최대하한(infimum) - 여러 원소에 포함되는 가장 큰 원소

앞쪽 분석(forward analysis)에서는 ⊔만 필요해요:
- 여러 경로의 정보를 합칠 때 가능성을 모두 포함해야 하니까요

### 전체적 맥락

뒤쪽 분석(backward analysis - 예: 사용 가능성 분석)에서는 ⊓을 사용합니다.

---

## Slide 12: 부호 분석 - 제약 예시 (코드)

> [Code: a=42; b=87; if x { c=a+b; } else { c=a-b; } return;]
> [CFG: v₁:entry → v₂:a=42 → v₃:b=87 → v₄:if x → v₅:c=a+b / v₆:c=a-b → v₇:return]

### 개념 설명

구체적인 코드에서 CFG와 제약을 구성하는 과정을 보여줘요.

**원본 코드:**
```
a = 42;
b = 87;
if x {
  c = a + b;
} else {
  c = a - b;
}
return;
```

**CFG 구조:**
```
v₁: entry
  ↓
v₂: a=42
  ↓
v₃: b=87
  ↓
v₄: if x
  / \
v₅:  v₆:
c=a+b c=a-b
  \ /
v₇: return
```

### 배경 지식

각 노드는 제어 흐름에서 특별한 점을 나타내요:
- **entry**: 프로그램 시작
- **return**: 프로그램 종료
- **분기점**: if 문에서 경로가 나뉘는 곳
- **합병점**: 경로가 다시 만나는 곳 (v₇)

### 전체적 맥락

이 예시에서 특히 중요한 부분은 v₇이에요:
- v₅와 v₆에서 서로 다른 c 값을 가지고 도착해요
- 따라서 JOIN으로 합쳐야 해요

---

## Slide 13: 부호 분석 - 제약 예시 (제약식)

> ⟦v₁⟧ = ⊤
> ⟦v₂⟧ = ⟦v₁⟧[a ↦ +]
> ⟦v₃⟧ = ⟦v₂⟧[b ↦ +]
> ⟦v₄⟧ = ⟦v₃⟧
> ⟦v₅⟧ = ⟦v₄⟧[c ↦ ˆ+(⟦v₄⟧(a), ⟦v₄⟧(b))]
> ⟦v₆⟧ = ⟦v₄⟧[c ↦ ˆ-(⟦v₄⟧(a), ⟦v₄⟧(b))]
> ⟦v₇⟧ = ⟦v₅⟧ ⊔ ⟦v₆⟧

### 개념 설명

이전 CFG에 대한 구체적인 제약식이에요. 각 변수의 값을 단계별로 추적해요.

**제약식 해석:**

| 제약식 | 의미 |
|--------|------|
| ⟦v₁⟧ = ⊤ | entry이므로 모든 변수가 ⊤ |
| ⟦v₂⟧ = ⟦v₁⟧[a ↦ +] | v₁의 상태에서 a만 +(양수)로 업데이트 (42는 양수) |
| ⟦v₃⟧ = ⟦v₂⟧[b ↦ +] | v₂의 상태에서 b만 +(양수)로 업데이트 (87은 양수) |
| ⟦v₄⟧ = ⟦v₃⟧ | if 문은 할당이 없으므로 상태 유지 |
| ⟦v₅⟧ = ⟦v₄⟧[c ↦ ˆ+(⟦v₄⟧(a), ⟦v₄⟧(b))] | c = a + b를 계산: (+) ˆ+ (+) = (+) |
| ⟦v₆⟧ = ⟦v₄⟧[c ↦ ˆ-(⟦v₄⟧(a), ⟦v₄⟧(b))] | c = a - b를 계산: (+) ˆ- (+) = ⊤ |
| ⟦v₇⟧ = ⟦v₅⟧ ⊔ ⟦v₆⟧ | v₅와 v₆의 상태를 합침 |

### 배경 지식

구체적으로 계산하면:
- v₅에서: a=+, b=+, c=+
- v₆에서: a=+, b=+, c=⊤
- v₇에서: a=+, b=+, c=⊤ (두 경로의 c가 다르므로 ⊤으로 합침)

### 예시 분석

```
실제 값:
v₂: a=42 (양수)
v₃: a=42, b=87
v₅: a=42, b=87, c=129 (양수)
v₆: a=42, b=87, c=-45 (음수)
v₇: a=42, b=87, c는 129 또는 -45

추상 값:
v₇: a=+, b=+, c=⊤ (정확히 모든 가능성을 포함)
```

---

## Slide 14: 단조성

> - Function composition preserves monotonicity
> - ⊔ is monotone
> - Map update is monotone
> - ôp is monotone
> - eval(_, e) : (Var → Sign) → Sign is monotone for every e

### 개념 설명

부호 분석의 모든 연산이 단조 함수(monotone function)임을 증명하는 슬라이드예요.

**단조 함수의 정의:**
```
f가 단조함수 ⟺ x ≤ y이면 f(x) ≤ f(y)
```

**부호 분석에서의 단조성:**

1. **함수 합성의 단조성**: f와 g가 단조이면 f ∘ g도 단조
2. **⊔의 단조성**: x ≤ x'이고 y ≤ y'이면 x ⊔ y ≤ x' ⊔ y'
3. **맵 업데이트의 단조성**: x ≤ y이면 x[a ↦ v] ≤ y[a ↦ v]
4. **추상 연산의 단조성**: ôp가 단조함수 (Slide 10의 표를 보면 확인 가능)
5. **eval의 단조성**: eval(σ, e)는 σ에 대해 단조

### 배경 지식

**왜 단조성이 중요한가:**

고정점 정리(Knaster-Tarski Fixed Point Theorem):
- f : L → L가 단조함수이고 L이 완전 격자이면,
- f의 최소 고정점이 존재하고 유일해요.

### 전체적 맥락

이 정리 덕분에:
- 고정점 알고리즘이 항상 종료돼요 (격자의 높이가 유한하므로)
- 결과가 유일해요
- 우리의 분석 결과가 정확해요

---

## Slide 15: 덧셈의 단조성

> [Same 5×5 table as slide 10, showing monotonicity verification]

### 개념 설명

Slide 10의 추상 덧셈 표가 실제로 단조인지 검증하는 과정이에요.

**단조성 확인:**

ˆ+이 단조라는 것은: a ≤ a'이고 b ≤ b'이면 a ˆ+ b ≤ a' ˆ+ b'

**표에서 확인:**

| 경우 | 확인 |
|------|------|
| ⊥ ≤ anything | ⊥ ˆ+ b = ⊥ ≤ a' ˆ+ b' ✓ |
| - ≤ + | - ˆ+ 0 = - ≤ + ˆ+ 0 = + ✓ |
| - ≤ ⊤ | - ˆ+ b ≤ ⊤ ˆ+ b ✓ (모든 b에 대해) |

### 배경 지식

표를 자세히 보면:
- 오른쪽으로 갈수록 큰 수를 더할수록 결과가 증가하거나 같아요
- 아래로 갈수록 작은 수를 더할수록 결과가 증가하거나 같아요
- 이것이 단조성의 정의입니다.

### 전체적 맥락

모든 추상 연산(뺄셈, 곱셈 등)도 마찬가지로 단조여야 해요:
- 그렇지 않으면 고정점에 도달하지 않을 수 있어요
- 따라서 분석 설계 시 단조성 검증이 중요합니다.

---

## Slide 16: 제약 해결

> f : Stateⁿ → Stateⁿ
> f(⟦v₁⟧, ..., ⟦vₙ⟧) = (f₁(⟦v₁⟧), ..., fₙ(⟦vₙ⟧))
> We can compute lfp(f) using:
> NaiveFixedPointAlgorithm(f):
>   x ← ⊥
>   while x ≠ f(x) do
>     x ← f(x)
>   return x

### 개념 설명

모든 제약을 하나의 함수 f : Stateⁿ → Stateⁿ로 표현해요.

**함수의 구성:**
```
f(x₁, ..., xₙ) = (f₁(x₁,...,xₙ), ..., fₙ(x₁,...,xₙ))
```

각 fᵢ는 노드 i의 제약 규칙이에요.

**소박한 고정점 알고리즘:**

```python
def NaiveFixedPointAlgorithm(f):
    x = ⊥  # 모든 변수를 ⊥로 초기화
    while x ≠ f(x):  # 고정점에 도달할 때까지
        x = f(x)     # 한 번 전이
    return x
```

### 배경 지식

**수렴성 증명:**

f가 단조이고 L이 유한 높이의 격자라면:
- 수열 ⊥ ≤ f(⊥) ≤ f²(⊥) ≤ ... ≤ lfp(f)
- 이 수열은 엄격히 증가하므로 반드시 수렴해요

### 전체적 맥락

시간 복잡도는 O(n · h · k):
- n: CFG 노드 수
- h: 격자의 높이 (⊥에서 ⊤까지의 최대 경로 길이)
- k: 한 번 전이(f 계산)의 비용

예: 부호 분석에서 Sign 격자의 높이는 3입니다 (⊥ → 중간 → ⊤).

---

## Slide 17: 정밀도 - 개선된 부호 격자

> - Adding abstract values can improve precision
>   - e.g., -/0, -/+, 0/+
> [Extended Hasse diagram: ⊤ at top, -/0, -/+, 0/+ in middle layer, -, 0, + below, ⊥ at bottom]

### 개념 설명

원래 부호 격자의 정밀도를 높이기 위해 더 많은 추상 값을 추가할 수 있어요.

**개선된 격자:**

```
      ⊤
   / | \
  -/0 -/+ 0/+
    \ | /
   -  0  +
     \ | /
       ⊥
```

새로운 원소들:
- `-/0`: 음수 또는 0
- `-/+`: 음수 또는 양수 (0은 아님)
- `0/+`: 0 또는 양수

### 배경 지식

이들 값은 특정 부호 조합만 가능할 때 유용해요:

**예:**
```python
x = 0
if (x > 0):  # 거짓, x는 양수가 될 수 없음
    c = x + 10
else:        # 참, x는 음수 또는 0
    c = x - 10
```

여기서 c의 값:
- 원래 부호 격자: ⊤ (정밀도 낮음)
- 개선된 격자: `-/0` (더 정확)

### 전체적 맥락

정밀도를 높일수록:
- 거짓 경보(false alarm)가 줄어들어요
- 대신 격자의 높이가 증가해서 수렴에 더 오래 걸려요
- 실무에서는 정밀도와 효율성의 균형을 맞춰야 합니다.

---

## Slide 18: 부호 분석의 응용

> - In theory, can detect division-by-zero errors
>   - Identify division whose divisor is 0 or ⊤
>   - Would have too many false alarms
> - More powerful analysis techniques can be useful
>   - Interval domain
>   - Path sensitivity

### 개념 설명

부호 분석의 실제 적용 가능성과 한계를 다루는 슬라이드예요.

**0으로 나누기 검출:**

이론적으로는 부호 분석으로 나누는 수가 0 또는 ⊤일 때를 감지할 수 있어요:

```python
if (x == 0):
    # 이 부분에서는 x가 0임을 알아야 함
    y = a / x  # 에러 검출 가능
```

하지만 실제로는 거짓 경보가 많아요:

```python
x = input()  # x는 ⊤
y = a / x    # 항상 경고 (실제로는 0이 아닐 수도)
```

### 배경 지식

**더 강력한 기법들:**

1. **구간 분석(Interval Domain)**: 변수가 [1, 100] 범위라는 식으로 정확한 범위 추적
2. **경로 민감성(Path Sensitivity)**: if문의 조건을 고려해서 서로 다른 경로에 다른 정보 할당

예:
```python
if (x > 0):    # 이 브랜치에서는 x > 0
    y = 1 / x  # 안전
else:
    y = a / x  # x ≤ 0이므로 위험 가능
```

### 전체적 맥락

부호 분석은 간단하지만 정확도가 낮아요. 실무에서는 더 복잡한 도메인과 기법이 필요합니다.

---

## Slide 19: 상수 전파 - 격자

> State = Var → flat(ℤ)
> [Hasse diagram: ⊤ at top, ..., -2, -1, 0, 1, 2, ... in middle, ⊥ at bottom]

### 개념 설명

상수 전파(Constant Propagation)는 변수의 정확한 정수 값을 추적하는 분석이에요.

**flat(ℤ) 격자:**

```
        ⊤ (unknown)
       / | | | \
    -2 -1 0 1 2  ... (모든 정수)
       \ | | | /
        ⊥ (impossible)
```

**특징:**
- ⊥는 불가능한 상태
- 각 정수 n은 변수가 정확히 n이라는 뜻
- ⊤는 정확한 값을 모를 때

### 배경 지식

이를 **평탄 격자(flat lattice)**라고 불러요:
- 최소 원소: ⊥
- 최대 원소: ⊤
- 중간 원소들: 모두 비교 불가능
- 높이: 2 (⊥에서 ⊤까지 최대 2단계)

부호 격자와 달리:
- 정수값 자체를 추적하므로 훨씬 정확해요
- 하지만 입력값이 불확실하면 빠르게 ⊤가 돼요

### 전체적 맥락

컴파일러 최적화에서 매우 유용해요:
```python
x = 5
y = x * 2   # y가 항상 10이므로 컴파일 타임에 계산 가능
```

---

## Slide 20: 상수 전파 - 제약 규칙

> - x=e: ⟦v⟧ = JOIN(v)[x ↦ eval(JOIN(v), e)]
> - entry: ⟦v⟧ = ⊤
> - Others: ⟦v⟧ = JOIN(v)
> - eval : (Var → Flat(ℤ)) × Expression → Flat(ℤ)
>   - eval(σ, x) = σ(x)
>   - eval(σ, n) = n
>   - eval(σ, input()) = ⊤
>   - eval(σ, e₁ op e₂) = ôp(eval(σ, e₁), eval(σ, e₂))

### 개념 설명

상수 전파의 제약 규칙은 부호 분석과 거의 같아요. 다른 점은 eval 함수뿐이에요.

**제약 규칙:**

부호 분석과 동일:
- `x=e`: 할당 규칙
- `entry`: ⊤로 초기화
- 기타: JOIN만 적용

**eval 함수 (핵심 차이):**

| 식 | 결과 | 설명 |
|----|------|------|
| eval(σ, x) | σ(x) | 변수의 값을 그대로 조회 |
| eval(σ, n) | n | 상수는 그 값 자체 |
| eval(σ, input()) | ⊤ | 입력은 미지수 |
| eval(σ, e₁ op e₂) | ôp(...) | 연산 결과 계산 |

### 배경 지식

**join(v)의 결과:**

```
join({5, 10}) = ⊤  // 다른 값이므로 합칠 수 없음
join({5, 5}) = 5   // 같은 값이므로 5 유지
```

### 전체적 맥락

상수 전파의 한계:
```python
if x:
    a = 1
else:
    a = 2
# a는 ⊤ (1 또는 2, 정확히 알 수 없음)
```

---

## Slide 21: 상수 전파 - 추상 연산자

> a ôp b = { ⊥ if a=⊥ or b=⊥; ⊤ otherwise, if a=⊤ or b=⊤; a op b otherwise }

### 개념 설명

상수 전파에서의 추상 연산 정의예요.

**규칙:**

```
a ôp b를 계산할 때:
1. a=⊥ 또는 b=⊥이면 → ⊥ (불가능한 값과의 연산은 불가능)
2. a=⊤ 또는 b=⊤이면 → ⊤ (미지수와의 연산 결과는 미지수)
3. 둘 다 구체적인 값이면 → a op b (실제 연산 수행)
```

### 배경 지식

**최적화:**

```python
x = 5
y = 10
z = x + y  # z = 15 (컴파일 타임에 계산 가능)
```

상수 전파 후:
- x의 상태: 5
- y의 상태: 10
- z의 상태: 15

컴파일러가 `z = x + y` 코드를 `z = 15`로 최적화 가능해요.

### 예시

```python
x = input()      # x: ⊤
y = 5            # y: 5
z = x + y        # ⊤ ô+ 5 = ⊤
w = 3 * 4        # 3 ô* 4 = 12
```

---

## Slide 22: 상수 전파 - 응용

> Compiler optimization:
> Before: a=3; b=a*2; c=a+input(); a=a*b; e=a+c;
> After: a=3; b=6; c=3+input(); a=18; e=18+c;

### 개념 설명

상수 전파를 이용한 컴파일러 최적화의 구체적인 예시예요.

**상수 전파 분석 과정:**

```python
# 분석 결과
a=3      → a: 3
b=a*2    → a: 3, b: 6 (3*2 = 6 컴파일 타임에 계산)
c=a+input() → a: 3, b: 6, c: ⊤ (3 + 미지수 = 미지수)
a=a*b    → a: 18, b: 6, c: ⊤ (3*6 = 18)
e=a+c    → a: 18, b: 6, c: ⊤, e: ⊤ (18 + 미지수 = 미지수)
```

**최적화:**

1. `b=a*2` → `b=6`: 상수 폴딩(constant folding)
2. `a=a*b` → `a=18`: 상수 전파 후 상수 폴딩
3. `e=a+c` → `e=18+c`: a가 상수이므로 미리 계산

### 배경 지식

**컴파일러 최적화의 효과:**

- 런타임 계산 감소
- 캐시 효율 개선
- 분기 예측 개선

### 전체적 맥락

상수 전파는 매우 실용적인 최적화예요:
- 많은 컴파일러 (GCC, LLVM)에서 기본으로 수행
- 간단하면서도 효과적
- 이후 다른 최적화의 기반이 되기도 해요

---

## Slide 23: 고정점 알고리즘 - 동기

> - We need to find lfp(f) where f : Stateⁿ → Stateⁿ
> - NaiveFixedPointAlgorithm computes every fᵢ in each iteration — much of the computation is redundant
> - Example: x = (x₁,...,x₇) with f₁(x)=⊤, f₂(x)=x₁[a↦+], f₃(x)=x₂[b↦+], f₄(x)=x₃, f₅(x)=x₄[c↦ˆ+(x₄(a),x₄(b))], f₆(x)=x₄[c↦ˆ-(x₄(a),x₄(b))], f₇(x)=x₅⊔x₆

### 개념 설명

소박한 고정점 알고리즘이 왜 비효율적인지 보여주는 슬라이드예요.

**문제:**

```python
NaiveFixedPointAlgorithm(f):
    x = ⊥
    while x ≠ f(x):
        x = f(x)  # f 전체를 매번 계산
    return x
```

매 반복마다 **모든** f₁, ..., fₙ을 계산해요:

```
반복 1: f(x) = (f₁(x), f₂(x), ..., f₇(x))
반복 2: f(x) = (f₁(x), f₂(x), ..., f₇(x))
...
```

### 배경 지식

**불필요한 계산:**

예시에서:
- f₂는 x₁에만 의존
- f₃은 x₂에만 의존
- ...

하지만 매번 모든 것을 재계산해요:

```
반복 1: f₁(⊥) = ⊤ (변화)
반복 2: f₂(x) = ⊤[a↦+] (변화)
반복 3: f₃(x) = ... (변화)
반복 4: f₄(x) = ... (변화)
반복 5: f₅(x) = ... (변화)
반복 6: f₆(x) = ... (변화)
반복 7: f₇(x) = ... (변화)
반복 8: f₁(x) = ⊤ (변화 없음) ← 불필요!
반복 9: ...
```

### 전체적 맥락

실제 대규모 프로그램에서:
- n이 수천 개 이상일 수 있어요
- 각 fᵢ 계산도 비쌀 수 있어요
- 효율적인 알고리즘이 필수입니다

---

## Slide 24: 고정점 알고리즘 - 구조 활용

> Same example.
> - e.g., f₂ depends only on x₁, but the value of x₁ does not change in most iterations
> - We can exploit the fact that our lattice is Lⁿ and f consists of f₁, ..., fₙ

### 개념 설명

구조를 활용해서 불필요한 계산을 줄이는 아이디어를 소개하는 슬라이드예요.

**핵심 관찰:**

1. **의존성**: f₂는 x₁에만 의존
2. **안정성**: x₁이 변하지 않으면 f₂(x)도 변하지 않음
3. **결론**: x₁이 변하지 않을 때는 f₂를 계산할 필요 없음

### 배경 지식

**격자 구조 활용:**

f : Lⁿ → Lⁿ를 각 좌표별로 분해하면:
```
f(x₁, ..., xₙ) = (f₁(x₁,...,xₙ), ..., fₙ(x₁,...,xₙ))
```

각 fᵢ가 모든 변수에 의존할 필요는 없어요.

### 전체적 맥락

이 아이디어로부터 발전된 알고리즘들:
- Round Robin
- Chaotic Iteration
- Worklist Algorithm

각 알고리즘은 의존성을 다르게 활용해요.

---

## Slide 25: Round Robin 알고리즘

> x = (x₁,...,xₙ), f(x) = (f₁(x),...,fₙ(x))
> RoundRobin(f₁,...,fₙ):
>   x ← ⊥
>   while x ≠ f(x) do
>     for i in 1...n:
>       xᵢ ← fᵢ(x)
>   return x
> - One iteration of the while loop does not give the same result as one iteration of NaiveFixedPointAlgorithm in general
> - However, always terminates and produces lfp(f)
> - The number of iterations until the fixed point may be smaller

### 개념 설명

Round Robin 알고리즘은 각 노드를 차례대로 하나씩 업데이트해요.

**알고리즘:**

```python
def RoundRobin(f₁, ..., fₙ):
    x = ⊥
    while x ≠ f(x):
        for i in 1 to n:
            xᵢ = fᵢ(x)  # 각 노드를 하나씩 업데이트
    return x
```

**특징:**

1. 외부 while 루프: 고정점 도달까지
2. 내부 for 루프: 1부터 n까지 순서대로 업데이트
3. 각 반복마다 모든 노드를 한 번씩 방문

### 배경 지식

**수렴성:**

Round Robin도 고정점에 도달해요:
- f가 단조함수이고 L이 유한 높이 격자라면
- 항상 수렴하고 lfp(f)를 계산합니다.

**효율성:**

소박한 알고리즘보다 빠를 수 있어요:
- 하지만 반복 횟수는 같을 수도, 적을 수도 있어요
- 가장 좋은 경우: 반복이 크게 줄어듦

### 예시

```python
# 초기: x₁=⊥, x₂=⊥, x₃=⊥, ..., x₇=⊥
# 반복 1의 for 루프:
#   x₁ ← f₁(x) = ⊤
#   x₂ ← f₂(x) = ⊤[a↦+]
#   x₃ ← f₃(x) = ...
#   ...
#   x₇ ← f₇(x) = ...
# 반복 2의 for 루프: (다시 1부터 7까지)
#   x₁ ← f₁(x) = ⊤ (변화 없음)
#   x₂ ← f₂(x) = ... (변화 있을 수도)
```

---

## Slide 26: Round Robin - 관찰

> [Same pseudocode]
> - The order of the iterations i := 1...n is irrelevant with the final result
> - We need to update xᵢ if xᵢ ≠ fᵢ(x) to reach the fixed point
> - We do not need to update xᵢ if xᵢ = fᵢ(x)

### 개념 설명

Round Robin의 중요한 성질들을 정리하는 슬라이드예요.

**성질 1: 순서 무관성**

```python
for i in [1,2,3,...,n]:     # 이 순서로 업데이트하나
for i in [n,...,3,2,1]:     # 역순으로 업데이트하나
for i in [3,1,4,1,5,...]:   # 임의의 순서로 하나
```

최종 결과는 같아요! (수렴 후)

다만, 수렴 속도는 다를 수 있어요.

**성질 2: 필요한 업데이트만**

```python
if xᵢ ≠ fᵢ(x):    # 값이 변할 경우만
    xᵢ ← fᵢ(x)
# else: 이미 고정점 조건 만족
```

### 배경 지식

**고정점 조건:**

x가 고정점 ⟺ x = f(x) ⟺ x₁ = f₁(x) AND x₂ = f₂(x) AND ... AND xₙ = fₙ(x)

따라서 모든 i에 대해 xᵢ = fᵢ(x)일 때 고정점에 도달해요.

### 전체적 맥락

이 관찰로부터:
- 불필요한 업데이트를 건너뛸 수 있어요
- 어떤 노드를 먼저 업데이트할지도 선택 가능해요
- 이것이 Chaotic Iteration의 토대가 됩니다.

---

## Slide 27: Chaotic Iteration

> ChaoticIteration(f₁,...,fₙ):
>   x ← ⊥
>   while x ≠ f(x) do
>     choose i ∈ {1,...,n} s.t. xᵢ ≠ fᵢ(x)
>     xᵢ ← fᵢ(x)
>   return x
> - Always terminates and produces lfp(f)
> - The number of assignments until the fixed point may be smaller

### 개념 설명

Chaotic Iteration은 Round Robin에서 한 발 더 나아가 **필요한 노드만** 업데이트해요.

**알고리즘:**

```python
def ChaoticIteration(f₁, ..., fₙ):
    x = ⊥
    while x ≠ f(x):
        # xᵢ ≠ fᵢ(x)인 i를 하나 선택
        i를 선택  # 어떤 i든 상관없음
        xᵢ = fᵢ(x)  # 그 노드만 업데이트
    return x
```

**특징:**

1. for 루프가 없음 (하나씩만 선택)
2. 고정점을 만족하지 않는 노드만 업데이트
3. 순서는 임의로 선택 가능

### 배경 지식

**수렴성:**

Chaotic Iteration도 항상 수렴해요:
- 각 노드를 기껏해야 h번(h = 격자의 높이) 업데이트
- 전체 업데이트 횟수 ≤ n·h

### 예시

```python
# 초기: x = (⊥, ⊥, ⊥, ..., ⊥)

# 반복 1:
#   x₁ ≠ f₁(x) (⊥ ≠ ⊤)이므로 선택
#   x₁ ← ⊤

# 반복 2:
#   x₂ ≠ f₂(x) (⊥ ≠ ...)이므로 선택
#   x₂ ← f₂(x)

# 반복 3:
#   x₃, x₄, x₇ 중 업데이트 필요한 것 선택
#   x₅ ← f₅(x)  # 예를 들어 x₅ 선택

# ...계속...
```

### 전체적 맥락

이 알고리즘의 문제점:
- 어떤 i를 선택할지 매번 결정해야 해요
- 이를 알아내는 것 자체가 비싼 연산이에요
- 다음 슬라이드에서 이를 해결해요

---

## Slide 28: Chaotic Iteration - 문제점

> [Same pseudocode]
> - Not practical, as efficiency depends on the choice of i
> - Finding i requires computing fᵢ's so it is expensive

### 개념 설명

Chaotic Iteration의 이론적 장점이 실제로 구현하기 어려운 이유를 설명해요.

**문제 1: 선택의 어려움**

```python
# 어떤 i를 선택할 것인가?
while x ≠ f(x):
    choose i s.t. xᵢ ≠ fᵢ(x)  # ← 이 선택이 어려워!
```

좋은 선택이라면:
- 수렴을 빠르게 해요
- 최소한의 반복으로 고정점 도달

나쁜 선택이라면:
- 불필요한 계산이 많아져요
- 수렴까지 오래 걸려요

**문제 2: 선택 비용**

조건 `xᵢ ≠ fᵢ(x)`를 확인하려면:
```python
for i in 1 to n:           # 모든 i를 확인?
    if xᵢ ≠ fᵢ(x):        # fᵢ(x) 계산 필요
        # ...이 i를 선택
```

이렇게 하면 모든 fᵢ를 계산하게 되어 원래 알고리즘과 별 다를 게 없어요!

### 배경 지식

**효율성 vs 구현 난이도**

- 이론적으로: 업데이트 횟수가 최소 (O(n·h))
- 실제로: 좋은 선택 전략을 찾기 어려움

### 전체적 맥락

이 문제를 해결하기 위해 **Worklist 알고리즘**이 등장해요:
- 선택을 명시적으로 관리 (worklist)
- 업데이트가 필요한 노드만 기록
- 효율적이면서도 구현 가능

---

## Slide 29: Worklist 알고리즘 - 관찰

> - fᵢ typically uses only a few of x₁,...,xₙ
> - We can record the nodes that need recomputation based on what we updated, rather than newly finding them every time
> [Same example equations]

### 개념 설명

Worklist 알고리즘의 핵심 아이디어를 소개해요.

**관찰 1: 제한된 의존성**

```
f₁(x) = ⊤                                    # x에 의존하지 않음
f₂(x) = x₁[a ↦ +]                          # x₁에만 의존
f₃(x) = x₂[b ↦ +]                          # x₂에만 의존
f₄(x) = x₃                                  # x₃에만 의존
f₅(x) = x₄[c ↦ ˆ+(x₄(a), x₄(b))]          # x₄에만 의존
f₆(x) = x₄[c ↦ ˆ-(x₄(a), x₄(b))]          # x₄에만 의존
f₇(x) = x₅ ⊔ x₆                            # x₅와 x₆에 의존
```

각 fᵢ는 전체 x가 아니라 **특정 원소들**에만 의존해요.

**관찰 2: 역 의존성 활용**

xᵢ가 변하면, fᵢ(x)가 변할 수 있는 모든 j를 찾아요:

```
만약 x₄가 변하면:
  → f₅가 변할 수 있음
  → f₆이 변할 수 있음

만약 x₅가 변하면:
  → f₇이 변할 수 있음
```

따라서 x₄를 업데이트했을 때는 f₅, f₆만 재계산하면 돼요.

### 배경 지식

**의존성 그래프:**

```
f₁
 ↓
f₂
 ↓
f₃
 ↓
f₄
 ↙  ↘
f₅  f₆
 ↘  ↙
  f₇
```

노드 i에서 노드 j로의 간선: fⱼ가 xᵢ에 의존

### 전체적 맥락

이 의존성을 명시적으로 관리하면:
- 어떤 노드를 업데이트할지 쉽게 결정 가능
- 불필요한 계산 회피
- 효율적이면서도 구현 가능한 알고리즘 완성

---

## Slide 30: Worklist 알고리즘 - dep

> - We introduce a map dep : Node → P(Node)
>   - dep(v) = the set of nodes whose information depends on the information of v
> - For the sign analysis and constant propagation analysis, dep = succ
> - When the information of v is updated, only the nodes in dep(v) need to be recomputed

### 개념 설명

의존성을 명시적으로 표현하는 dep 함수를 정의해요.

**dep 함수:**

```
dep : Node → P(Node)
dep(v) = {u | fᵤ가 xᵥ에 의존}
```

예를 들어:
- `dep(v₁) = {v₂}`: f₂가 x₁에 의존하므로
- `dep(v₄) = {v₅, v₆}`: f₅, f₆이 x₄에 의존하므로
- `dep(v₅) = {v₇}`: f₇이 x₅에 의존하므로

**CFG 분석에서의 dep:**

부호 분석과 상수 전파에서:
```
dep(v) = succ(v)  (v의 후계자들)
```

왜냐하면:
- fᵥ의 결과가 JOIN(succ(v))로 이어져요
- 따라서 v의 후계자들만 영향받음

### 배경 지식

**일반적인 경우:**

```
if condition:
    x = ...
    ↓ (CFG 간선)
y = x + ...  (succ 노드는 이 할당)
```

x를 업데이트하면 그 후계자인 y 계산에 영향을 줘요.

### 전체적 맥락

dep을 계산하는 방법:
1. **정적 분석**: 코드를 훑어 의존성 파악
2. **동적 추적**: 실제 계산 중 의존성 기록
3. **경험적 추정**: 보수적으로 모든 노드 포함

정확한 dep일수록 효율성이 높아요.

---

## Slide 31: Worklist 알고리즘 - dep 예시

> ⟦v₅⟧ = ⟦v₄⟧[c ↦ ˆ+(⟦v₄⟧(a), ⟦v₄⟧(b))]
> ⟦v₆⟧ = ⟦v₄⟧[c ↦ ˆ-(⟦v₄⟧(a), ⟦v₄⟧(b))]
> dep(v₄) = {v₅, v₆}

### 개념 설명

구체적인 제약 규칙에서 dep를 계산하는 예시예요.

**분석:**

v₅의 제약: `⟦v₅⟧ = ⟦v₄⟧[c ↦ ˆ+(⟦v₄⟧(a), ⟦v₄⟧(b))]`
- ⟦v₅⟧가 ⟦v₄⟧에 의존 ✓
- ⟦v₄⟧가 변하면 ⟦v₅⟧도 변할 수 있음

v₆의 제약: `⟦v₆⟧ = ⟦v₄⟧[c ↦ ˆ-(⟦v₄⟧(a), ⟦v₄⟧(b))]`
- ⟦v₆⟧가 ⟦v₄⟧에 의존 ✓
- ⟦v₄⟧가 변하면 ⟦v₆⟧도 변할 수 있음

v₇의 제약: `⟦v₇⟧ = ⟦v₅⟧ ⊔ ⟦v₆⟧`
- ⟦v₇⟧는 ⟦v₅⟧과 ⟦v₆⟧에 의존
- ⟦v₄⟧에는 직접 의존하지 않음

따라서: `dep(v₄) = {v₅, v₆}`

### 배경 지식

**CFG와의 연결:**

데이터흐름 분석에서 dep은 보통:
```
dep(v) = succ(v)  (CFG에서 v의 후계자)
```

CFG 구조가 명확하면 dep도 명확해요.

### 전체적 맥락

이제 worklist 알고리즘을 구현할 준비가 됐어요:
- v를 업데이트하면 dep(v)를 worklist에 추가
- 효율적이고 정확한 고정점 계산 가능

---

## Slide 32: Worklist 알고리즘 - 의사코드

> SimpleWorkListAlgorithm(f₁,...,fₙ):
>   x ← ⊥
>   W ← {v₁,...,vₙ}
>   while W ≠ ∅ do
>     vᵢ ← W.removeOne()
>     y ← fᵢ(x)
>     if y ≠ xᵢ:
>       xᵢ ← y
>       W ← W ∪ dep(vᵢ)
>   return x
> - W is called the worklist
> - Always terminates and produces lfp(f)

### 개념 설명

실제 구현 가능한 효율적인 Worklist 알고리즘이에요.

**알고리즘:**

```python
def SimpleWorkListAlgorithm(f₁, ..., fₙ):
    x = ⊥                      # 모든 노드를 ⊥로 초기화
    W = {v₁, ..., vₙ}         # worklist: 모든 노드로 시작

    while W ≠ ∅:               # worklist가 공백이 아닌 동안
        vᵢ = W.removeOne()      # worklist에서 하나 꺼냄
        y = fᵢ(x)              # 노드 i의 함수 계산

        if y ≠ xᵢ:             # 값이 변한 경우만
            xᵢ = y              # 업데이트
            W = W ∪ dep(vᵢ)    # 의존하는 노드들을 worklist에 추가

    return x
```

**주요 단계:**

1. **초기화**: 모든 변수를 ⊥로, worklist를 모든 노드로
2. **루프**: worklist가 공백이 될 때까지
   - 노드 하나를 꺼냄
   - 함수 계산
   - 변화가 있으면 영향받는 노드들을 worklist에 추가

### 배경 지식

**Worklist의 역할:**

- **추적(Tracking)**: 어떤 노드를 재계산해야 하는지 기록
- **지연(Postponement)**: 필요할 때만 계산
- **자동 종료**: worklist가 비면 고정점 도달

### 전체적 맥락

이 알고리즘의 강점:
- **정확성**: 고정점을 반드시 계산
- **효율성**: 필요한 노드만 계산
- **실무성**: 실제로 구현하고 사용 가능

대부분의 정적 분석 도구가 이 알고리즘을 사용해요.

---

## Slide 33: Worklist 알고리즘 - 시간 복잡도

> If |dep(v)| is bounded by a constant for all nodes v, the worst-case time complexity is O(n · h · k)
> where:
> - n is the number of CFG nodes
> - h is the height of the lattice L = State
> - k is the worst-case time required to compute fᵢ

### 개념 설명

Worklist 알고리즘의 성능을 분석한 슬라이드예요.

**시간 복잡도: O(n · h · k)**

- **n**: CFG 노드 개수
- **h**: 격자의 높이
- **k**: 한 노드 함수 계산 시간

**복잡도 분석:**

1. **노드별 업데이트 횟수**: 각 노드는 최대 h번 변함
   - ⊥에서 ⊤로 가는 경로: 최대 h단계
   - 각 노드는 단조 함수이므로 한 번 증가하면 다시 감소 안 함

2. **영향받는 노드**: 각 노드를 업데이트하면 dep(v)의 노드들이 worklist에 추가
   - |dep(v)| ≤ c (상수)라고 가정
   - 따라서 총 추가 횟수 ≤ n·h·c

3. **총 계산 비용**: n·h번의 함수 계산 × k = O(n·h·k)

### 배경 지식

**격자의 높이:**

- 부호 분석: h = 3 (⊥ → 부호 → ⊤)
- 상수 전파 (flat): h = 2 (⊥ → 상수/⊤ → 상수/⊤)
- 구간 분석: h = ∞ (무한 가능)

### 예시

```
n = 1000 (CFG 노드)
h = 3 (부호 분석 격자의 높이)
k = 10 (함수 계산 시간)
dep의 최대 크기 = 2 (CFG에서 후계자는 최대 2개)

시간 복잡도 = 1000 × 3 × 10 = 30,000 단위
```

실제로는 훨씬 빨라요 (모든 노드가 h번 변하는 건 아니므로).

### 전체적 맥락

이 복잡도는:
- 선형적으로 프로그램 크기(n)에 비례
- 격자가 유한하면 항상 수렴
- 실무적으로 충분히 빨라요

---

## Slide 34: Worklist 알고리즘 - 가능한 개선

> - Handle strongly connected components (cycles) separately
> - Use a priority queue for the worklist
> - Make the dependence information more precise by allowing dep to consider x₁,...,xₙ in addition to v

### 개념 설명

Worklist 알고리즘의 효율성을 더욱 높이기 위한 개선 기법들이에요.

**개선 1: 강연결 성분(SCC) 처리**

```
제어 흐름 그래프의 루프(사이클):

v₁ → v₂ → v₃ ↘
          ↗ v₄
```

루프가 있는 부분을 특별히 처리해요:
- 루프를 하나의 블록으로 취급
- 루프 내부의 고정점을 먼저 계산
- 그 후 다른 부분 계산

장점: 루프로 인한 불필요한 반복 감소

**개선 2: 우선순위 큐**

```python
# 기본: 임의의 노드 처리
W = {v₁, v₃, v₇}  # 어떤 순서든 괜찮음

# 개선: 좋은 순서로 처리
W = PriorityQueue()
# 예: 아직 변하지 않은 노드를 우선 처리
# 또는: 영향 범위가 넓은 노드를 우선 처리
```

효과: 수렴 속도 향상

**개선 3: 정확한 의존성**

```python
# 기본: 정적 의존성
dep(v₄) = {v₅, v₆}  # 항상 동일

# 개선: 동적 의존성
# v₄의 특정 변수(예: a)가 변했다면
# 그것을 사용하는 노드만 추가
# v₄의 a와 b 중 a만 변했다면
# a를 사용하는 함수만 재계산
```

효과: 불필요한 계산 제거

### 배경 지식

**강연결 성분(SCC):**

CFG에서 서로 도달 가능한 노드들의 집합

```
CFG:
v₁ → v₂ ← v₃
     ↓ ↗

SCC: {v₁}과 {v₂, v₃}
```

### 전체적 맥락

이들 개선은:
- 이론적 복잡도는 변하지 않음
- 실제 성능은 크게 개선
- 현대 분석 도구들이 사용하는 기법들이에요

---

## Slide 35: 요약

> - Dataflow analysis assigns constraint variables over a lattice to CFG nodes and solves monotone constraints via fixed-point computation
> - Sign analysis tracks the sign of variables; constant propagation tracks exact integer values
> - The naive fixed-point algorithm recomputes all nodes each iteration; Round Robin and Chaotic Iteration improve on this
> - The worklist algorithm uses dependency information (dep) to recompute only affected nodes, achieving O(n·h·k) complexity

### 개념 설명

강의 전체를 정리하는 최종 슬라이드예요.

**데이터흐름 분석의 핵심:**

1. **격자를 이용한 추상화**: 프로그램의 복잡한 상태를 수학적 구조로 표현
2. **제약 변수**: CFG의 각 노드에 하나씩 할당
3. **고정점 계산**: 단조성을 이용해 반복으로 고정점 도달

**구체적 분석들:**

- **부호 분석**: -, 0, + 를 구분하는 간단한 격자
  - 용도: 0으로 나누기 검출 등
  - 한계: 정밀도가 낮음

- **상수 전파**: 변수의 정확한 정수값 추적
  - 용도: 컴파일러 최적화 (상수 폴딩)
  - 한계: 입력이 있으면 빠르게 ⊤가 됨

**알고리즘의 진화:**

1. **소박한 알고리즘**: 매 반복마다 모든 함수 계산 → 비효율적
2. **Round Robin**: 노드를 순서대로 처리 → 약간의 개선
3. **Chaotic Iteration**: 필요한 노드만 선택 → 이론적으로 최적이지만 구현 어려움
4. **Worklist 알고리즘**: 의존성을 명시적으로 관리 → 효율적이면서도 구현 가능

**성능:**

- **시간 복잡도**: O(n·h·k)
  - n: 노드 수, h: 격자 높이, k: 함수 계산 시간
  - 실무적으로 충분히 빠름

### 배경 지식

**데이터흐름 분석의 적용 분야:**

1. **컴파일러 최적화**: 상수 전파, 데드 코드 제거
2. **버그 탐지**: 불가능한 값 추적 (0으로 나누기 등)
3. **보안 분석**: 위험한 연산 추적
4. **코드 이해**: 변수의 범위 파악

### 전체적 맥락

이 강의에서 배운 개념들:
- 격자와 단조성
- 고정점 이론
- 효율적인 알고리즘 설계

은 단순히 데이터흐름 분석뿐 아니라, **정적 분석 전반**의 기초를 이루고 있습니다.

다음 강의에서는 이를 더욱 발전시켜 다양한 분석 기법들을 살펴볼 거예요:
- 정확도 개선 (경로 민감성, 문맥 민감성)
- 효율성 개선 (점진적 분석, 병렬화)
- 실제 프로그래밍 언어에의 적용

---

## 핵심 용어 정리

| 용어 | 한국어 | 설명 |
|------|--------|------|
| Dataflow Analysis | 데이터흐름 분석 | 프로그램의 변수 값 범위를 추적하는 정적 분석 기법 |
| Lattice | 격자 | 순서 관계를 가진 수학적 구조 |
| Transfer Function | 전이 함수 | 각 노드에서의 추상 상태 변화를 정의 |
| Fixed Point | 고정점 | f(x) = x를 만족하는 점 |
| Monotone Function | 단조 함수 | x ≤ y이면 f(x) ≤ f(y)를 만족 |
| JOIN | 조인 | 여러 경로의 추상 상태를 합침 |
| Sign Analysis | 부호 분석 | 변수의 부호(-/0/+)를 추적 |
| Constant Propagation | 상수 전파 | 변수의 정확한 정수값을 추적 |
| Worklist | 작업 목록 | 재계산이 필요한 노드들의 목록 |
| CFG | 제어흐름 그래프 | 프로그램의 실행 경로를 나타내는 그래프 |

