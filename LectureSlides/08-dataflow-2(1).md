# 데이터플로우 분석 (2) - 강의 설명 자료

## Slide 1: Dataflow Analysis (2)

> **Dataflow Analysis (2)**
> CSE552 Program Analysis — Lecture 8
> Jaemin Hong

### 개념 설명
이 강의는 프로그램 분석 (Program Analysis) 과정의 8번째 강의로, 데이터플로우 분석의 두 번째 부분을 다룹니다. 첫 번째 강의에서 배운 기초 개념을 바탕으로 더 다양한 종류의 데이터플로우 분석 기법들을 학습하게 됩니다.

### 상세한 예시
이 강의를 통해 배우게 될 내용들:
- Live Variable Analysis (생존 변수 분석)
- Available Expression Analysis (가용 식 분석)
- Very Busy Expression Analysis (매우 바쁜 식 분석)
- Reaching Definition Analysis (도달하는 정의 분석)
- 이들 분석 기법들의 분류 및 특성

---

## Slide 2: Live Variable Analysis

> **Live Variable Analysis**
> - A variable is *live* at a program point if there exists an execution where its value is read later in the execution without it being written to in between
> - Code:
> ```
> // nothing is live
> x = input();
> // x is live
> if input() {
>   // x is live
>   y = x;
>   // y is live
> } else {
>   // nothing is live
>   y = 1;
>   // y is live
> }
> // y is live
> z = y;
> ```

### 개념 설명
생존 변수 (Live Variable)는 어떤 프로그램 지점에서 그 값이 나중에 읽혀질 가능성이 있는 변수를 말합니다. 중요한 점은 **"존재할 수 있는 실행 경로"**를 고려한다는 것입니다. 변수가 생존하려면, 현재 지점부터 나중에 그 변수가 읽혀질 때까지의 사이에 **다시 쓰여지지 않아야** 합니다.

### 상세한 예시
위 코드를 분석해봅시다:
- 첫 줄 `x = input();` 바로 뒤: x는 생존합니다. 왜냐하면 if 문의 true 분기에서 `y = x;`로 읽혀질 수 있기 때문입니다.
- if 문 진입 후: x는 여전히 생존합니다.
- true 분기에서 `y = x;` 후: y가 생존합니다. (마지막 줄에서 `z = y;`로 읽힐 것)
- false 분기에서: x는 읽혀지지 않으므로 생존하지 않습니다.
- else 블록에서 `y = 1;` 후: y가 생존합니다.
- 마지막: y는 생존하며 `z = y;`로 읽혀집니다.

### 배경 지식
프로그램의 조건문은 여러 실행 경로를 만들기 때문에, 변수가 "어떤 경로에서라도" 나중에 읽혀질 수 있다면 생존하는 것으로 봅니다. 이는 **May Analysis** (가능한 분석)에 해당합니다.

### 전체적 맥락
생존 변수 분석은 레지스터 할당 (Register Allocation) 최적화에 사용됩니다. 생존하지 않는 변수는 그 값을 레지스터에 유지할 필요가 없으므로 레지스터를 절약할 수 있습니다.

---

## Slide 3: Live Variable Analysis — Motivation

> **Live Variable Analysis — Motivation**
> - We can approximate the set of live variables using dataflow analysis
>   - Application: register allocation
>   - We want: the answer "not live" can be trusted and "live" is safe but useless

### 개념 설명
생존 변수 분석의 목표는 각 프로그램 지점에서 어떤 변수들이 생존하는지를 근사적으로 계산하는 것입니다. 여기서 "근사적으로"라는 것은 정확한 실행 경로를 모두 추적할 수 없으므로 안전한 (Safe) 과다근사를 하게 된다는 의미입니다.

### 상세한 예시
분석의 신뢰성:
- **"not live"라고 결론**: 이것은 신뢰할 수 있습니다 (신뢰성 - Soundness). 우리가 "생존하지 않는다"고 판단한 변수는 정말로 어떤 실행 경로에서도 나중에 읽혀지지 않습니다.
- **"live라고 결론**: 이것은 안전하지만 덜 유용합니다. 실제로는 생존하지 않을 수도 있지만 (거짓 양성), 우리는 안전을 위해 생존한다고 가정합니다.

### 배경 지식
최적화 관점에서 보면:
- "not live" → 안전하게 레지스터 재할당 가능
- "live" → 반드시 보존해야 함 (과도한 보존도 괜찮음)

---

## Slide 4: Live Variable Analysis — Abstract States

> **Live Variable Analysis — Abstract States**
> [CFG: v₁ splits to v₂, v₃]
> ⟦v₁⟧ = {x,y}, ⟦v₂⟧ = {x}, ⟦v₃⟧ = {y}
> - State = (P(Var), ⊆) — Power set lattice
> - For each CFG node v, ⟦v⟧ denotes the set of variables live before the node
> - JOIN(v) = ⊔_{u∈succ(v)} ⟦u⟧ = ∪_{u∈succ(v)} ⟦u⟧
>   - This combines abstract states from the **successors**

### 개념 설명
생존 변수 분석에서 추상 상태 (Abstract State)는 변수들의 거듭제곱 집합 (Power Set) 격자입니다. 각 CFG 노드 v에 대해 ⟦v⟧는 **노드 이전에 생존하는 변수들의 집합**을 나타냅니다.

### 상세한 예시
CFG에서 노드 v₁이 v₂와 v₃로 분기한다고 하면:
- v₁ 이후에는 v₂로 갈 수도, v₃로 갈 수도 있습니다.
- v₂에서 필요한 변수들: {x}
- v₃에서 필요한 변수들: {y}
- 따라서 v₁에서는 x와 y 모두 생존해야 합니다: ⟦v₁⟧ = {x, y}

### 배경 지식
**JOIN 연산**은 successor (후속 노드)들로부터 온 상태를 합칩니다. 생존 변수의 경우, 어느 분기로 가든 생존할 수 있는 변수는 모두 생존하는 것으로 봅니다. 따라서 합집합 (∪)을 사용합니다.

생존 변수 분석은 **후진 분석 (Backward Analysis)**이므로, JOIN은 **후속 노드 (successor)**들의 상태를 합칩니다.

### 전체적 맥락
격자 구조 (P(Var), ⊆)에서:
- ⊤ (Top) = 모든 변수
- ⊥ (Bottom) = 공집합 (아무 변수도 생존하지 않음)
- 연산: ⊔ = ∪ (합집합)

---

## Slide 5: Live Variable Analysis — Constraint Rule (Assignment)

> **Live Variable Analysis — Constraint Rule (Assignment)**
> - x=e: ⟦v⟧ = JOIN(v) \ {x} ∪ vars(e)
> ```
> // y and z are live
> x = y + z;
> // x is live
> ```

### 개념 설명
변수 할당 (Assignment) `x=e`에서 제약 규칙 (Constraint Rule)을 적용합니다:
- 할당 **이후** (노드 다음): x는 생존합니다 (방금 정의되었으므로)
- 할당 **이전** (노드 이전): x는 제거되고, e에 나타나는 모든 변수들이 추가됩니다

### 상세한 예시
`x = y + z;`를 분석해봅시다:
- 이 문장 이후 노드에서 필요한 변수들이 {y, z}라고 하면:
- 이 문장 이전에는?
  - x를 {y, z}에서 제거: {y, z} \ {x} = {y, z} (x가 없었으므로 변화 없음)
  - 식 (y + z)의 변수들을 추가: {y, z} ∪ {y, z} = {y, z}
- 결과: y와 z가 이전에 필요합니다.

또 다른 예시: `x = y + z;` 이후 x만 필요하다면:
- 이전에 필요한 변수: {x} \ {x} ∪ {y, z} = ∅ ∪ {y, z} = {y, z}
- y와 z가 생성되므로 먼저 계산되어야 합니다.

### 배경 지식
이 규칙의 논리:
- **JOIN(v) \ {x}**: 이 노드 이후에 필요한 변수 중 x를 제거 (이 노드가 x를 정의했으므로)
- **∪ vars(e)**: 식 e를 계산하기 위해 필요한 변수들을 추가

### 전체적 맥락
**후진 분석**에서 우리는 노드 이후의 상태에서 시작하여 노드 이전의 상태를 계산합니다. 따라서 제약 규칙은 ⟦v⟧ (노드 이전)를 JOIN(v) (노드 이후)로부터 계산합니다.

---

## Slide 6: Live Variable Analysis — Constraint Rules (Remaining)

> **Live Variable Analysis — Constraint Rules (Remaining)**
> - if x: ⟦v⟧ = JOIN(v) ∪ {x}
> - entry: ⟦v⟧ = JOIN(v)
> - return: ⟦v⟧ = JOIN(v) = ∅

### 개념 설명
할당 외의 다른 노드들에 대한 제약 규칙들입니다.

### 상세한 예시
- **if x**: 조건문에서 x의 값을 읽으므로, JOIN(v)에 x를 추가합니다.
  - 조건을 평가하려면 x가 필요합니다.

- **entry**: 프로그램 진입점에서는 정의된 변수가 없으므로, 후속 노드에서 필요한 변수들을 그대로 전달합니다.

- **return**: 반환문도 정의가 아니므로 JOIN(v)를 그대로 사용합니다.
  - 반환 이후는 없으므로 JOIN(v) = ∅입니다.

### 배경 지식
모든 노드에 대해 제약 규칙을 정의하여 고정점 (Fixed Point)을 찾으면, 모든 프로그램 지점에서의 생존 변수를 계산할 수 있습니다.

---

## Slide 7: Available Expression Analysis

> **Available Expression Analysis**
> - A nontrivial expression (not a literal, not a variable) in a program is *available* at a program point if its current value has already been computed earlier in the execution
> ```
> // nothing is available
> x = y + 1;
> // y + 1 is available
> if input() {
>   // y + 1 is available
>   y = z + 1;
>   // z + 1 is available
> } else {
>   // y + 1 is available
>   x = z + 1;
>   // y + 1 and z + 1 are available
> }
> // z + 1 is available
> w = (z + 1) + (y + 1);
> ```

### 개념 설명
가용 식 (Available Expression)은 어떤 프로그램 지점에서 그 값이 이미 이전에 계산된 식을 말합니다. **중요한 조건**: 그 이후 식의 변수들이 재정의되지 않아야 합니다.

"가용"이라는 것은 그 식의 값을 다시 계산할 필요 없이 **재사용할 수 있다**는 의미입니다.

### 상세한 예시
코드를 단계별로 분석합니다:
- 초기: 아무것도 가용하지 않음
- `x = y + 1;` 후: y + 1은 가용합니다 (방금 계산됨)
- true 분기 진입: y + 1은 여전히 가용합니다 (y가 재정의되지 않음)
- `y = z + 1;` 후: y를 포함하는 식들 (y + 1 등)은 가용하지 않습니다. 하지만 z + 1은 가용합니다
- false 분기: y + 1은 여전히 가용합니다
- `x = z + 1;` 후: y + 1과 z + 1 모두 가용합니다
- 합치기 (merge): 양쪽 분기 모두에서 가용한 식만 가용합니다. 즉, z + 1만 가용합니다 (y + 1은 일부 분기에서만 가용)

### 배경 지식
가용 식 분석은 **전진 분석 (Forward Analysis)**입니다. 우리는 프로그램 진입부터 시작하여 앞으로 진행하면서 어떤 식들이 가용해지는지 추적합니다.

식을 가용하게 유지하려면, 그 식에 포함된 변수들이 재정의되지 않아야 합니다. 예를 들어 y + 1이 가용한데 y가 재정의되면, y + 1은 더 이상 같은 값을 갖지 않으므로 가용하지 않습니다.

### 전체적 맥락
가용 식 분석은 **Must Analysis** (반드시 참인 분석)입니다. 우리는 **모든 가능한 경로에서** 이미 계산되었을 때만 식을 가용하다고 봅니다. 합치기는 교집합 (∩)을 사용합니다.

---

## Slide 8: Available Expression Analysis — Motivation

> **Available Expression Analysis — Motivation**
> - Application: optimization (eliminating redundant computations)
> - We want: "available" can be trusted; "not available" is safe but useless

### 개념 설명
가용 식 분석의 목표는 중복된 계산을 제거하는 최적화에 사용하는 것입니다.

분석 결과의 신뢰성:
- **"available"이라고 결론**: 신뢰할 수 있습니다. 그 식은 정말로 이미 계산되었으므로 재계산할 필요가 없습니다.
- **"not available"이라고 결론**: 안전하지만 덜 유용합니다. 실제로는 가용할 수도 있지만, 우리는 안전하게 "가용하지 않다"고 가정합니다.

### 상세한 예시
"available"을 믿을 수 있다는 것의 중요성:
- 만약 우리가 z + 1을 가용하다고 판단하고 실제로는 계산되지 않았다면, 정의되지 않은 값을 재사용하게 되어 프로그램이 오작동합니다.
- 따라서 가용 식 분석은 보수적으로, "확실한 경우에만" 가용하다고 판단합니다 (Must Analysis).

---

## Slide 9: Available Expression Analysis — Optimization Example

> **Available Expression Analysis — Optimization Example**
> Before: `x = y+1; if input() { y = z+1; } else { x = z+1; } w = (z+1) + (y+1);`
> After: `x = y+1; if input() { zplus1 = z+1; y = zplus1; } else { zplus1 = z+1; x = zplus1; } w = zplus1 + (y+1);`

### 개념 설명
가용 식 분석을 사용한 실제 최적화 예시입니다. z + 1이 두 분기 모두에서 계산되므로, 이를 공통 부분식 (Common Subexpression Elimination, CSE)으로 추출합니다.

### 상세한 예시
**최적화 전:**
- if 분기: `y = z+1;`에서 z+1을 계산
- else 분기: `x = z+1;`에서 z+1을 계산
- 마지막: `w = (z+1) + (y+1);`에서 z+1을 또 계산
- 총 3번의 z+1 계산

**최적화 후:**
- if 분기: 먼저 z+1을 계산하여 zplus1에 저장, 그 후 y를 정의
- else 분기: 마찬가지로 z+1을 먼저 계산
- 마지막: 미리 계산한 zplus1을 재사용
- 총 2번의 z+1 계산 (또는 1번의 z+1 계산 + 재사용)

### 배경 지식
공통 부분식 제거 (CSE)는 효과적인 최적화 기법입니다. 가용 식 분석은 정확히 어디에서 이 최적화를 안전하게 적용할 수 있는지를 알려줍니다.

---

## Slide 10: Available Expression Analysis — Abstract States

> **Available Expression Analysis — Abstract States**
> [CFG: v₁, v₂ merge into v₃]
> ⟦v₁⟧ = {x+1}, ⟦v₂⟧ = {x+1, y+1}, ⟦v₃⟧ = {x+1}
> - State = (P(Expr), ⊇) — **Reverse** power set lattice
> - ⟦v⟧ denotes expressions available after the node
> - JOIN(v) = ⊔_{u∈pred(v)} ⟦u⟧ = ∩_{u∈pred(v)} ⟦u⟧ (predecessors)

### 개념 설명
가용 식 분석에서 추상 상태는 **역방향 거듭제곱 집합** 격자입니다. 생존 변수 분석과 달리, 여기서는 교집합 (∩)을 사용합니다.

이는 **Must Analysis** (반드시 참인 분석)의 특징입니다. 모든 선행 경로 (predecessor path)에서 가용해야만 가용하다고 판단합니다.

### 상세한 예시
CFG에서 v₁과 v₂가 v₃으로 merge된다고 하면:
- v₁ 이후: x + 1만 가용 → ⟦v₁⟧ = {x+1}
- v₂ 이후: x + 1과 y + 1 가용 → ⟦v₂⟧ = {x+1, y+1}
- v₃ 이전 (JOIN): 둘 다에서 가용한 것만 → JOIN(v₃) = {x+1} ∩ {x+1, y+1} = {x+1}
- 따라서 ⟦v₃⟧ = {x+1}

### 배경 지식
역방향 격자 (P(Expr), ⊇)에서:
- ⊤ (Top) = 공집합 (모든 식이 가용) - 가장 유리한 상태
- ⊥ (Bottom) = 모든 식 (아무 식도 가용하지 않음) - 가장 불리한 상태
- 연산: ⊔ = ∩ (교집합)

### 전체적 맥락
가용 식 분석은 **전진 분석**이므로 JOIN은 **선행 노드 (predecessor)**들의 상태를 합칩니다. 그리고 **Must Analysis**이므로 교집합을 사용합니다.

---

## Slide 11: Available Expression Analysis — Constraint Rule (Assignment)

> **Available Expression Analysis — Constraint Rule (Assignment)**
> - x=e: ⟦v⟧ = (JOIN(v) ∪ exprs(e))↓x
>   - ↓x removes all expressions containing x
>   - exprs collects all nontrivial expressions
> - exprs(x) = ∅, exprs(n) = ∅, exprs(input()) = ∅
> - exprs(e₁ op e₂) = {e₁ op e₂} ∪ exprs(e₁) ∪ exprs(e₂)
> ```
> // x + 1 is available
> x = x + (y + z);
> // y + z is available
> ```

### 개념 설명
할당 `x=e`에서:
- **JOIN(v) ∪ exprs(e)**: 이전에 가용한 식들에 현재 계산하는 식들을 추가
- **↓x**: x를 포함하는 모든 식을 제거 (x를 재정의했으므로 그 값은 변함)

↓x 연산은 중요합니다. x를 포함하는 모든 식 (x + 1, 2 * x, f(x) 등)은 더 이상 같은 값을 갖지 않으므로 가용하지 않습니다.

### 상세한 예시
`x = x + (y + z);`를 분석합니다:
- 이 문장 이전에 x + 1이 가용하다고 하면:
- JOIN(v) = {x+1}
- exprs(x + (y + z)) = {x + (y + z), y + z}
- 합치기: {x+1} ∪ {x + (y + z), y + z} = {x+1, x + (y + z), y + z}
- ↓x 적용: {x+1, x + (y + z), y + z}에서 x를 포함하는 모든 식 제거
  - x + 1 제거 (x 포함)
  - x + (y + z) 제거 (x 포함)
  - y + z 보존 (x 미포함)
- 결과: {y + z}

### 배경 지식
exprs 함수는 식에서 모든 비자명 부분식 (Nontrivial Subexpression)을 추출합니다:
- 변수나 상수는 비자명하지 않음 (exprs(x) = ∅)
- 복합 식은 식 자체와 부분식들을 모두 포함

예: exprs(2 * (x + 1)) = {2 * (x + 1), x + 1}

### 전체적 맥락
이 규칙은 **전진 분석**의 특징을 보여줍니다. 우리는 노드 이전의 상태 (JOIN(v))로부터 노드 이후의 상태 (⟦v⟧)를 계산합니다.

---

## Slide 12: Available Expression Analysis — Constraint Rules (Remaining)

> **Available Expression Analysis — Constraint Rules (Remaining)**
> - if x: ⟦v⟧ = JOIN(v)
> - entry: ⟦v⟧ = ⊤ = ∅
> - return: ⟦v⟧ = JOIN(v)

### 개념 설명
할당이 아닌 다른 노드들에 대한 제약 규칙들입니다.

### 상세한 예시
- **if x**: 조건문은 식을 계산하지만 변수를 정의하지 않습니다. 따라서 JOIN(v)를 그대로 사용합니다.

- **entry**: 프로그램 진입점에서는 아무 식도 이미 계산되지 않았으므로 ⊤ = ∅입니다 (가용한 식이 없음).

- **return**: 반환문도 마찬가지로 JOIN(v)를 사용합니다.

### 배경 지식
역방향 격자에서 ⊤ = ∅라는 것이 직관적입니다:
- 일반 격자 (P(Var), ⊆)에서 ⊤ = 모든 변수 (최악의 경우)
- 역방향 격자 (P(Expr), ⊇)에서 ⊤ = 공집합 (모든 식이 가용, 가장 유리한 경우)

---

## Slide 13: Very Busy Expression Analysis

> **Very Busy Expression Analysis**
> - An expression is *very busy* if it will definitely be evaluated before its value changes
> ```
> // nothing is very busy
> x = input();
> // x + 1 is very busy
> if input() {
>   // x + 1 is very busy
>   y = x + 1;
> } else {
>   // x + 1 and y + 1 are very busy
>   z = x + 1;
>   // y + 1 is very busy
>   w = y + 1;
> }
> ```

### 개념 설명
매우 바쁜 식 (Very Busy Expression)은 어떤 프로그램 지점에서 **확실히** (definitely) 나중에 평가될 것이 보장되는 식을 말합니다. 또한 그 식의 변수들이 그 사이에 재정의되지 않아야 합니다.

### 상세한 예시
코드를 단계별로 분석합니다:
- `x = input();` 후: x + 1은 매우 바쁩니다 (양쪽 분기 모두에서 평가됨)
- true 분기: x + 1은 여전히 매우 바쁩니다
- true 분기에서 `y = x + 1;`: x + 1이 평가됩니다
- false 분기: x + 1과 y + 1이 매우 바쁩니다 (둘 다 평가될 것)
- false 분기에서 `z = x + 1;`: x + 1이 평가됨
- false 분기에서 `y = 1;` 후: y + 1은 여전히 매우 바쁩니다
- false 분기에서 `w = y + 1;`: y + 1이 평가됨

### 배경 지식
"매우 바쁘다"는 것은 **모든 가능한 실행 경로에서** 그 식이 평가된다는 의미입니다. 이는 **Must Analysis**의 특징입니다.

또한 이는 **후진 분석 (Backward Analysis)**입니다. 우리는 프로그램의 끝에서부터 시작하여 역방향으로 진행하면서, 어떤 식들이 나중에 확실히 평가될 것인지를 추적합니다.

### 전체적 맥락
Very Busy Expression Analysis는:
- **후진 분석**: 끝에서부터 역방향으로
- **Must Analysis**: 모든 경로에서 평가되어야 함
- 따라서 JOIN은 **후속 노드 (successor)**들의 교집합 (∩)입니다

---

## Slide 14: Very Busy Expression Analysis — Motivation

> **Very Busy Expression Analysis — Motivation**
> - Application: optimization (code hoisting)
> - "very busy" can be trusted; "not very busy" is safe but useless

### 개념 설명
매우 바쁜 식 분석은 **코드 호이스팅 (Code Hoisting)**이라는 최적화 기법에 사용됩니다. 식이 매우 바쁘다면, 그 식을 프로그램의 더 앞 부분으로 이동시켜도 안전합니다.

### 상세한 예시
분석 결과의 신뢰성:
- **"very busy"라고 결론**: 신뢰할 수 있습니다. 그 식은 정말로 어떤 실행 경로에서도 나중에 평가됩니다.
- **"not very busy"라고 결론**: 안전하지만 덜 유용합니다. 실제로는 매우 바쁠 수도 있지만, 우리는 안전하게 "매우 바쁘지 않다"고 가정합니다.

---

## Slide 15: Very Busy Expression Analysis — Optimization Example (if)

> **Very Busy Expression Analysis — Optimization Example (if)**
> Before: `x = input(); if input() { y = x+1; } else { z = x+1; w = y+1; }`
> After: `x = input(); xplus1 = x+1; if input() { y = xplus1; } else { z = xplus1; w = y+1; }`

### 개념 설명
if 문의 양쪽 분기 모두에서 x + 1이 평가되므로, 이를 조건문 이전으로 호이스팅할 수 있습니다.

### 상세한 예시
**최적화 전:**
- if의 true 분기: `y = x+1;`에서 x+1을 계산
- if의 false 분기: `z = x+1;`에서 x+1을 계산
- x+1이 두 번 계산됨

**최적화 후:**
- if 이전: x+1을 한 번 계산하여 xplus1에 저장
- if의 true 분기: `y = xplus1;`으로 변경 (재계산 불필요)
- if의 false 분기: `z = xplus1;`으로 변경 (재계산 불필요)
- x+1이 한 번만 계산됨

### 배경 지식
코드 호이스팅은 루프 불변식 제거 (Loop-Invariant Code Motion, LICM)와 유사한 최적화 기법입니다. 불필요한 계산을 제거하여 성능을 개선합니다.

---

## Slide 16: Very Busy Expression Analysis — Optimization Example (while)

> **Very Busy Expression Analysis — Optimization Example (while)**
> Before: `x = input(); while input() { y = x+1; } z = x+1;`
> After: `x = input(); xplus1 = x+1; while input() { y = xplus1; } z = xplus1;`

### 개념 설명
while 루프 내에서 x + 1이 평가되고, 루프 이후에도 x + 1이 평가됩니다. x가 루프 내에서 변경되지 않으면, x + 1을 루프 이전으로 호이스팅할 수 있습니다.

### 상세한 예시
**최적화 전:**
- 루프의 매 반복마다 `y = x+1;`에서 x+1을 계산
- 루프 이후 `z = x+1;`에서 x+1을 또 계산
- n번의 루프 반복 + 1번의 루프 이후 = n+1번의 x+1 계산

**최적화 후:**
- 루프 이전: x+1을 한 번 계산하여 xplus1에 저장
- 루프의 매 반복마다 `y = xplus1;`으로 변경
- 루프 이후 `z = xplus1;`으로 변경
- 1번의 x+1 계산 + n번의 재사용

### 배경 지식
이 최적화는 루프 불변식 제거 (LICM)의 전형적인 예입니다. x가 루프 내에서 변경되지 않으므로, x + 1은 루프 불변식입니다.

---

## Slide 17: Very Busy Expression Analysis — Abstract States

> **Very Busy Expression Analysis — Abstract States**
> [CFG: v₁ splits to v₂, v₃]
> ⟦v₁⟧ = {x+1}, ⟦v₂⟧ = {x+1, y+1}, ⟦v₃⟧ = {x+1}
> - State = (P(Expr), ⊇) — **Reverse** power set lattice
> - ⟦v⟧ = expressions very busy before the node
> - JOIN(v) = ⊔_{u∈succ(v)} ⟦u⟧ = ∩_{u∈succ(v)} ⟦u⟧ (successors)

### 개념 설명
매우 바쁜 식 분석도 역방향 거듭제곱 집합 격자를 사용합니다. 여기서 ⟦v⟧는 **노드 이전에** 매우 바쁜 식들을 나타냅니다.

후진 분석이므로 JOIN은 **후속 노드 (successor)**들의 교집합을 취합니다.

### 상세한 예시
CFG에서 v₁이 v₂와 v₃으로 분기한다고 하면:
- v₂에서 매우 바쁜 식: {x+1, y+1}
- v₃에서 매우 바쁜 식: {x+1}
- v₁에서 매우 바쁜 식: 양쪽 모두에서 매우 바쁜 것만
  - JOIN(v₁) = {x+1, y+1} ∩ {x+1} = {x+1}
- ⟦v₁⟧ = {x+1}

### 배경 지식
**후진 분석**에서는:
- ⟦v⟧는 노드 **이전**의 상태
- JOIN은 **후속 노드**들의 정보를 합침
- 이는 "이 노드를 지난 후 어떤 상태가 되는가?"를 뒤에서부터 추적하는 방식

**Must Analysis**에서는:
- 교집합 (∩)을 사용하여 모든 경로에서 참인 것만 선택
- 역방향 격자 (P(Expr), ⊇)를 사용

---

## Slide 18: Very Busy Expression Analysis — Constraint Rules

> **Very Busy Expression Analysis — Constraint Rules**
> - x=e: ⟦v⟧ = (JOIN(v)↓x) ∪ exprs(e)
> ```
> // x + (y + z) and y + z are very busy
> x = x + (y + z);
> // x + 1 is very busy
> ```
> - if x: ⟦v⟧ = JOIN(v)
> - entry: ⟦v⟧ = JOIN(v)
> - return: ⟦v⟧ = ⊤ = ∅

### 개념 설명
할당 `x=e`에서:
- **JOIN(v)↓x**: 후속 노드에서 매우 바쁜 식 중 x를 포함하는 것들을 제거 (x를 재정의하므로)
- **∪ exprs(e)**: 현재 계산하는 식들을 추가 (이 노드에서 평가됨)

### 상세한 예시
`x = x + (y + z);`를 분석합니다:
- 이 문장 이후에 x + 1이 매우 바쁘다고 하면:
- JOIN(v) = {x+1}
- JOIN(v)↓x = {x+1}에서 x를 포함하는 식 제거 → ∅
- exprs(x + (y + z)) = {x + (y + z), y + z}
- 결과: ∅ ∪ {x + (y + z), y + z} = {x + (y + z), y + z}

### 배경 지식
이 규칙의 논리:
- 할당 이후에 매우 바쁘던 식 중에서, x를 포함하는 것들은 (값이 바뀌므로) 이전에는 매우 바쁘지 않습니다
- 대신 현재 문장에서 평가되는 식들은 매우 바쁩니다 (적어도 이 노드에서는 확실히 평가됨)

**후진 분석**에서는 우리가 노드 이후의 상태 (JOIN(v))로부터 노드 이전의 상태 (⟦v⟧)를 계산합니다.

### 전체적 맥락
일반적인 규칙들:
- **if x**: 조건문은 변수를 정의하지 않으므로 JOIN(v)를 그대로 사용
- **entry**: 프로그램 진입점, 후속 노드가 없으므로 JOIN(v) = ∅. 따라서 ⟦entry⟧ = ∅
- **return**: 반환 이후는 아무것도 평가되지 않으므로 ⊤ = ∅에서 시작

---

## Slide 19: Reaching Definition Analysis

> **Reaching Definition Analysis**
> - *Reaching definitions* are those assignments that may have defined the current values of variables
> ```
> if input() {
>   x = y;
>   // x = y is a reaching definition
>   x = y + 1;
>   // x = y + 1 is a reaching definition
> } else {
>   x = z + 1;
>   // x = z + 1 is a reaching definition
> }
> // x = y + 1 and x = z + 1 are reaching definitions
> return x;
> ```

### 개념 설명
도달하는 정의 (Reaching Definition)는 현재 프로그램 지점에서 **어떤 변수의 현재 값을 정의했을 가능성이 있는 할당들**을 말합니다.

예를 들어, x의 현재 값은 `x = y + 1` 또는 `x = z + 1` 중 하나의 할당에서 나왔을 것입니다.

### 상세한 예시
코드를 단계별로 분석합니다:
- true 분기에서 `x = y;`: 이제 x = y가 도달하는 정의
- true 분기에서 `x = y + 1;`: 이전의 x = y는 더 이상 x를 정의하지 않으므로 제거됨. 이제 x = y + 1이 도달하는 정의
- false 분기에서 `x = z + 1;`: x = z + 1이 도달하는 정의
- 합치기: 어느 분기를 따랐는지 알 수 없으므로, x = y + 1과 x = z + 1 모두 도달하는 정의 (어느 것일지 모르지만 둘 중 하나)

### 배경 지식
Reaching Definition Analysis는:
- **전진 분석**: 프로그램 진입부터 앞으로 진행
- **May Analysis**: 가능한 정의들을 모두 추적
- 따라서 JOIN은 **선행 노드 (predecessor)**들의 합집합 (∪)

### 전체적 맥락
이 분석은 변수가 어디서 정의되었는지를 추적합니다. 이는 def-use 그래프 구성에 사용되며, 여러 최적화 기법의 기초가 됩니다.

---

## Slide 20: Reaching Definition Analysis — Motivation

> **Reaching Definition Analysis — Motivation**
> - Application: def-use graph (useful for optimizations)
> - "not reaching" can be trusted; "reaching" is safe but useless

### 개념 설명
도달하는 정의 분석의 목표는 각 변수 사용 지점에서 어떤 정의들이 도달하는지를 파악하는 것입니다. 이를 통해 def-use 그래프 (Definition-Use Graph)를 구성할 수 있습니다.

### 상세한 예시
분석 결과의 신뢰성:
- **"not reaching"이라고 결론**: 신뢰할 수 있습니다. 그 정의는 정말로 이 사용 지점에 도달하지 않습니다.
- **"reaching"이라고 결론**: 안전하지만 덜 유용합니다. 실제로는 도달하지 않을 수도 있지만, 우리는 안전하게 "도달한다"고 가정합니다.

### 배경 지식
Def-use 그래프는 다양한 최적화에 활용됩니다:
- 데드 코드 제거 (Dead Code Elimination)
- 상수 전파 (Constant Propagation)
- 변수 사용 분석

---

## Slide 21: Reaching Definition Analysis — Abstract States

> **Reaching Definition Analysis — Abstract States**
> [CFG: v₁, v₂ merge into v₃]
> ⟦v₁⟧ = {x=y}, ⟦v₂⟧ = {x=y+1}, ⟦v₃⟧ = {x=y, x=y+1}
> - State = (P(Def), ⊆) — Power set lattice, Def d ::= x=e
> - ⟦v⟧ = definitions that may define variable values after the node
> - JOIN(v) = ⊔_{u∈pred(v)} ⟦u⟧ = ∪_{u∈pred(v)} ⟦u⟧ (predecessors)

### 개념 설명
도달하는 정의 분석에서 추상 상태는 거듭제곱 집합 격자 (P(Def), ⊆)입니다. 여기서 Def는 `x=e` 형태의 정의입니다.

JOIN은 선행 노드들의 **합집합**을 취합니다 (May Analysis).

### 상세한 예시
CFG에서 v₁과 v₂가 v₃으로 merge된다고 하면:
- v₁ 이후: x = y가 도달하는 정의
- v₂ 이후: x = y + 1이 도달하는 정의
- v₃ 이전 (JOIN): 어느 경로를 따랐는지 모르므로 둘 다 도달할 수 있음
  - JOIN(v₃) = {x=y} ∪ {x=y+1} = {x=y, x=y+1}

### 배경 지식
**전진 분석**에서는:
- ⟦v⟧는 노드 **이후**의 상태 (노드가 끝난 후의 정의들)
- JOIN은 **선행 노드 (predecessor)**들의 정보를 합침

**May Analysis**에서는:
- 합집합 (∪)을 사용하여 가능한 모든 경우를 추적
- 일반 격자 (P(Def), ⊆)를 사용

---

## Slide 22: Reaching Definition Analysis — Constraint Rules

> **Reaching Definition Analysis — Constraint Rules**
> - x=e: ⟦v⟧ = (JOIN(v)↓x) ∪ {x=e} (↓x removes all definitions of x)
> ```
> // x = y + 1 is a reaching definition
> x = y;
> // x = y is a reaching definition
> ```
> - if x: ⟦v⟧ = JOIN(v)
> - entry: ⟦v⟧ = JOIN(v) = ∅
> - return: ⟦v⟧ = JOIN(v)

### 개념 설명
할당 `x=e`에서:
- **JOIN(v)↓x**: 선행 노드로부터 도달한 정의 중 x에 대한 것들을 제거 (이 할당이 x를 재정의하므로)
- **∪ {x=e}**: 현재 할당을 새로운 도달하는 정의로 추가

### 상세한 예시
`x = y;` 할당 직전에 x = y + 1이 도달한다고 하면:
- JOIN(v) = {x = y + 1}
- JOIN(v)↓x = {x = y + 1}에서 x에 대한 정의 제거 → ∅
- 현재 할당 추가: ∅ ∪ {x = y} = {x = y}
- 결과: x = y가 도달하는 정의

이것은 직관적입니다. 새로운 할당이 이전의 모든 x 정의를 "죽입니다" (Kill Definitions).

### 배경 지식
Reaching Definition Analysis의 용어:
- **Gen** (Generate): 이 노드가 만드는 정의 → {x=e}
- **Kill**: 이 노드가 없애는 정의 → x를 정의하는 모든 다른 정의들

따라서 규칙은: ⟦v⟧ = (JOIN(v) - Kill) ∪ Gen = (JOIN(v)↓x) ∪ {x=e}

### 전체적 맥락
다른 노드들:
- **if x**: 정의하지 않으므로 JOIN(v)를 그대로 사용
- **entry**: 선행 노드가 없으므로 JOIN(v) = ∅
- **return**: 정의하지 않으므로 JOIN(v)를 그대로 사용

---

## Slide 23: Time Complexity

> **Time Complexity**
> - SimpleWorkListAlgorithm: O(n · h · k)
>   - n = CFG nodes, h = lattice height, k = worst-case fᵢ computation
> - O(n · m²): n = CFG nodes, m = number of variables/expressions/definitions
>   - Because h = m, k = O(m)

### 개념 설명
WorkList 알고리즘의 시간 복잡도는 세 가지 요소에 의존합니다:
- **n**: CFG의 노드 개수
- **h**: 격자의 높이 (최대 깊이)
- **k**: 전달 함수 계산의 최악 경우 시간

### 상세한 예시
시간 복잡도 분석:
- 각 노드는 최대 **h번** 업데이트됩니다 (격자에서 ⊥에서 ⊤으로 올라가는 경로의 길이)
- 각 업데이트마다 전달 함수를 계산하는 데 **k 시간** 필요
- n개 노드가 있으므로 최대 **n · h · k 시간**

### 배경 지식
구체적인 분석들의 복잡도:
- **Live Variable Analysis**: h = 변수 개수, k = O(변수 개수) → O(n · m²)
- **Available Expression Analysis**: h = 식 개수, k = O(식 개수) → O(n · m²)
- **Reaching Definition Analysis**: h = 정의 개수, k = O(정의 개수) → O(n · m²)

실제로는 대부분의 경우 h가 작으므로 (CFG가 깊지 않으면), 실제 성능은 이론적 상한보다 훨씬 좋습니다.

---

## Slide 24: Forward vs Backward Analyses

> **Forward vs Backward Analyses**
> - Forward: computes info about past behavior
>   - Examples: sign, constant propagation, available expression, reaching definition
>   - Starts at entry, propagates forward, JOIN uses pred, dep = succ
> - Backward: computes info about future behavior
>   - Examples: live variables, very busy expressions
>   - Starts at return, propagates backward, JOIN uses succ, dep = pred

### 개념 설명
데이터플로우 분석은 크게 두 가지로 분류됩니다: 전진 분석과 후진 분석.

**전진 분석 (Forward Analysis)**:
- 프로그램의 **과거 동작**에 대한 정보 계산
- 프로그램 진입점 (entry)에서 시작
- 앞쪽으로 진행하면서 정보 전파
- JOIN은 선행 노드 (predecessor)들의 정보 합침

**후진 분석 (Backward Analysis)**:
- 프로그램의 **미래 동작**에 대한 정보 계산
- 프로그램 종료점 (return)에서 시작
- 뒤쪽으로 진행하면서 정보 전파
- JOIN은 후속 노드 (successor)들의 정보 합침

### 상세한 예시
예시 분석들을 분류합니다:

**전진 분석:**
1. Sign Propagation: x = 2; y = x + 3; → y는 양수 (과거: x가 정의되었음)
2. Available Expression: x = y + 1; → y + 1이 가용 (과거: 이미 계산됨)
3. Reaching Definition: if (...) { x = 1; } else { x = 2; } → 어느 정의가 도달하는가 (과거: 어느 경로를 따랐는가)

**후진 분석:**
1. Live Variables: z = y; → y가 생존 (미래: 나중에 y가 읽힐 것)
2. Very Busy Expression: if (...) { ... x+1 ... } else { ... x+1 ... } → x+1은 매우 바쁨 (미래: 어느 경로든 x+1이 평가됨)

### 배경 지식
전진/후진 분석의 선택:
- 정보가 "이전에 계산된 것"이나 "과거의 상태"와 관련되면 → 전진 분석
- 정보가 "나중에 사용될 것"이나 "미래의 상태"와 관련되면 → 후진 분석

### 전체적 맥락
전진 분석과 후진 분석은 구현 측면에서도 다릅니다:
- 전진: ⟦v⟧는 노드 **이후** 상태
- 후진: ⟦v⟧는 노드 **이전** 상태

---

## Slide 25: May vs Must Analyses

> **May vs Must Analyses**
> - May: info that may possibly be true. Examples: live variables, reaching definitions. Power set lattice.
> - Must: info that must definitely be true. Examples: available expressions, very busy expressions. Reverse power set lattice.

### 개념 설명
데이터플로우 분석은 또 다른 방식으로도 분류됩니다: May Analysis와 Must Analysis.

**May Analysis (가능한 분석)**:
- 정보가 **가능할 수 있는** 경우를 추적
- 예: 생존 변수, 도달하는 정의
- 거듭제곱 집합 격자 (P(·), ⊆) 사용
- JOIN = ∪ (합집합): 어느 경로든 가능한 것을 모두 포함

**Must Analysis (반드시인 분석)**:
- 정보가 **반드시 참인** 경우를 추적
- 예: 가용 식, 매우 바쁜 식
- 역방향 거듭제곱 집합 격자 (P(·), ⊇) 사용
- JOIN = ∩ (교집합): 모든 경로에서 참인 것만 포함

### 상세한 예시
May vs Must의 직관적 차이:

**May: "생존 변수"**
- 변수 x가 생존한다 = 어떤 실행 경로에서는 나중에 읽힐 수 있다
- 보수적 해석: 어느 경로에서든 읽힐 가능성이 있으면 생존으로 취급
- 결과: 과다근사 (Overapproximation) - false positives 가능

**Must: "가용 식"**
- 식 x+1이 가용하다 = 모든 실행 경로에서 이미 계산되었다
- 보수적 해석: 모든 경로에서 계산되었을 때만 가용으로 취급
- 결과: 과소근사 (Underapproximation) - false negatives 가능

### 배경 지식
격자 구조의 차이:

**May Analysis - (P(·), ⊆):**
- ⊤ = 전체 집합 (모든 변수/정의)
- ⊥ = 공집합
- ⊔ = ∪ (합집합)

**Must Analysis - (P(·), ⊇):**
- ⊤ = 공집합 (모든 것이 가용함)
- ⊥ = 전체 집합 (아무것도 가용하지 않음)
- ⊔ = ∩ (교집합)

### 전체적 맥락
May와 Must는 정반대의 논리입니다:
- May: 작은 집합에서 시작, 정보를 추가하면서 집합 확대
- Must: 큰 집합에서 시작, 정보를 제거하면서 집합 축소

---

## Slide 26: May vs Must Analyses — Soundness

> **May vs Must Analyses — Soundness**
> - May ≠ Sound, Must ≠ Complete
> - All these analyses are sound but not complete

### 개념 설명
음이론적 (Soundness)과 완전성 (Completeness):

**Soundness (음이론성)**:
- 분석 결과에 거짓 음성이 없음 (False Negatives ×)
- "이 정보는 확실하지 않다"는 결론을 내린 것은 신뢰할 수 있음

**Completeness (완전성)**:
- 분석 결과에 거짓 양성이 없음 (False Positives ×)
- "이 정보는 확실하다"는 결론을 내린 것이 정확함

### 상세한 예시
May Analysis의 경우:
- **Unsound에서** false negatives 가능: "생존하지 않는다"고 했는데 실제로는 생존
- **Incomplete**: false positives 가능: "생존한다"고 했는데 실제로는 생존하지 않음
- 우리가 구현하는 것은 **Sound하고 Incomplete**함: 거짓 음성은 없지만 거짓 양성은 있을 수 있음

Must Analysis의 경우:
- **Incomplete에서** false positives 가능: "가용하다"고 했는데 실제로는 가용하지 않음
- **Unsound**: false negatives 가능: "가용하지 않다"고 했는데 실제로는 가용
- 우리가 구현하는 것은 **Complete하고 Unsound**... 아니, 잠깐.

실제로 우리가 구현하는 모든 분석은 **Sound**합니다. 보수적으로 설계되어 있기 때문입니다.

### 배경 지식
정확한 분류:

**May Analysis (우리의 구현):**
- Sound: 거짓 음성 ×
- Incomplete: 거짓 양성 가능
- "not live"는 신뢰할 수 있음, "live"는 과다근사

**Must Analysis (우리의 구현):**
- Sound: 거짓 음성 ×... 아니 거짓 양성 ×
- Incomplete: 거짓 음성 가능
- "available"은 신뢰할 수 있음, "not available"은 과소근사

### 전체적 맥락
모든 정적 분석은 근본적으로 불완전합니다 (Halting Problem 때문에). 우리는 Sound하지만 Incomplete한 분석을 만들어서, 최적화에서 생기는 오류를 방지합니다.

---

## Slide 27: May vs Must Analyses — Soundness (Live Variables)

> **May vs Must Analyses — Soundness (Live Variables)**
> - Live variables = {x}
>   - Possible behavior: executions not requiring variables other than x to be live (can have false positives)
>   - Impossible behavior: executions requiring other variables to be live (no false negatives)

### 개념 설명
생존 변수 분석에서 우리가 계산한 결과 ⟦v⟧ = {x}라고 하면, 이것의 의미를 정확히 이해해야 합니다.

### 상세한 예시
⟦v⟧ = {x}라는 결과는:

**가능한 실행:**
- x만 필요하고 다른 변수는 필요 없는 실행 경로가 존재할 수 있음 (우리는 그것을 알 수 없음)
- 거짓 양성 (False Positive) 가능: 실제로는 y도 필요하지만 {x}에 포함되지 않음

**불가능한 실행:**
- 어떤 실행 경로에서도 x 외에 다른 변수가 나중에 반드시 읽혀져야 하는 경우는 없음
- 거짓 음성 (False Negative) 없음: {x}에 포함된 x는 반드시 나중에 읽혀질 수 있음

### 배경 지식
May Analysis의 보수성:
- 변수가 생존할 가능성이 조금이라도 있으면 생존으로 취급
- 결과적으로 생존 변수의 집합을 **과다근사** (Overapproximation)
- 레지스터 할당에서: 불필요한 변수도 살려둘 수 있지만, 필요한 변수를 죽이지는 않음 (안전)

### 전체적 맥락
Sound하다는 것의 의미:
- "not live"라는 결론은 **신뢰할 수 있음**: 정말로 어떤 경로에서도 읽혀지지 않음
- "live"라는 결론은 **안전하지만 보수적**: 실제로는 "not live"일 수도 있음

---

## Slide 28: May vs Must Analyses — Soundness (Available Expressions)

> **May vs Must Analyses — Soundness (Available Expressions)**
> - Available expressions = {x + y}
>   - Possible behavior: executions that already computed x+y (can have false positives)
>   - Impossible behavior: executions that haven't computed x+y (no false negatives)

### 개념 설명
가용 식 분석에서 우리가 계산한 결과 ⟦v⟧ = {x + y}라고 하면, 그 의미를 정확히 이해해야 합니다.

### 상세한 예시
⟦v⟧ = {x + y}라는 결과는:

**가능한 실행:**
- x + y가 이미 계산된 실행 경로가 존재할 수 있음 (모든 경로에서는 아닐 수도)
- 거짓 양성 (False Positive) 가능: 일부 경로에서만 계산되었지만, 우리는 모든 경로에서 계산되었다고 가정

**불가능한 실행:**
- x + y가 계산되지 않은 경로는 불가능 (모든 경로에서 계산됨)
- 거짓 음성 (False Negative) 없음: {x + y}에 포함된 x + y는 반드시 이미 계산되었음

### 배경 지식
Must Analysis의 보수성:
- 식이 모든 경로에서 계산되었을 때만 가용으로 취급
- 결과적으로 가용 식의 집합을 **과소근사** (Underapproximation)
- 공통 부분식 제거에서: 가능한 최적화를 일부 놓칠 수 있지만, 잘못된 최적화는 하지 않음 (안전)

### 전체적 맥락
Sound하다는 것의 의미:
- "available"이라는 결론은 **신뢰할 수 있음**: 정말로 모든 경로에서 계산됨
- "not available"이라는 결론은 **안전하지만 보수적**: 실제로는 "available"일 수도 있음

---

## Slide 29: Classification of Dataflow Analyses

> **Classification of Dataflow Analyses**
> |       | Forward | Backward |
> |-------|---------|----------|
> | May   | Reaching definition | Live variable |
> | Must  | Available expression | Very busy expression |

### 개념 설명
데이터플로우 분석의 4가지 분류를 표로 정리하면 이렇게 됩니다. 각 사분면에는 대표적인 분석 기법이 하나씩 있습니다.

### 상세한 예시
각 분류의 특징:

**May + Forward (Reaching Definition):**
- 정보: 과거에 어떤 정의가 이루어졌는가
- 목표: 변수의 현재 값이 어디서 정의되었는가
- 전파: 프로그램 진입에서 시작하여 앞으로
- JOIN: 선행 노드들의 합집합 (∪)

**May + Backward (Live Variable):**
- 정보: 미래에 어떤 변수가 사용될 것인가
- 목표: 현재 어떤 변수가 나중에 필요한가
- 전파: 프로그램 종료에서 시작하여 뒤로
- JOIN: 후속 노드들의 합집합 (∪)

**Must + Forward (Available Expression):**
- 정보: 과거에 어떤 식이 계산되었는가
- 목표: 현재 어떤 식의 값을 재사용할 수 있는가
- 전파: 프로그램 진입에서 시작하여 앞으로
- JOIN: 선행 노드들의 교집합 (∩)

**Must + Backward (Very Busy Expression):**
- 정보: 미래에 어떤 식이 평가될 것인가
- 목표: 현재 어떤 식을 앞으로 호이스팅할 수 있는가
- 전파: 프로그램 종료에서 시작하여 뒤로
- JOIN: 후속 노드들의 교집합 (∩)

### 배경 지식
이 분류는 완전하지 않습니다. 다른 4가지 조합도 가능합니다:
- May + Forward, May + Backward, Must + Forward, Must + Backward
- 하지만 위의 4가지가 가장 일반적이고 실용적입니다

### 전체적 맥락
이 분류 표는 데이터플로우 분석의 설계 공간을 나타냅니다. 새로운 분석을 설계할 때, 먼저 다음을 결정합니다:
1. Forward인가 Backward인가?
2. May인가 Must인가?
3. 그에 따라 JOIN 연산과 제약 규칙을 결정

---

## Slide 30: Example — Initialized Variable Analysis

> **Example — Initialized Variable Analysis**
> ```
> if input() {
>   x = 1
>   // x is initialized
>   y = x + 1
>   // x and y are initialized
> } else {
>   y = 2
>   // y is initialized
> }
> // y is initialized
> z = y + x
> ```

### 개념 설명
초기화된 변수 분석 (Initialized Variable Analysis)은 어떤 프로그램 지점에서 어떤 변수들이 확실히 초기화되었는지를 파악합니다. 이는 사용되지 않은 변수 (Uninitialized Variable) 오류를 찾는 데 사용됩니다.

### 상세한 예시
코드를 단계별로 분석합니다:
- if 진입: 아무 변수도 초기화되지 않음
- true 분기에서 `x = 1`: x가 초기화됨
- true 분기에서 `y = x + 1`: y도 초기화됨 (x는 이미 초기화됨)
- false 분기에서 `y = 2`: y가 초기화됨
- 병합 지점: x는 초기화되지 않을 수 있음 (false 분기에서 초기화 안 됨), y는 확실히 초기화됨
- `z = y + x`: y는 안전하지만 x는 사용되지 않은 변수 오류!

### 배경 지식
이 분석은 여러 면에서 흥미롭습니다:
- **Must Analysis**: 모든 경로에서 초기화되어야만 초기화된 것으로 취급
- **Forward Analysis**: 프로그램 진입부터 앞으로 진행
- **역방향 격자**: (P(Var), ⊇) 사용 (초기화된 변수들의 집합)

---

## Slide 31: Example — Initialized Variable Analysis (cont.)

> **Example — Initialized Variable Analysis (cont.)**
> - Must analysis — State = (P(Var), ⊇)
> - Forward analysis — JOIN(v) = ⊔_{u∈pred(v)} ⟦u⟧
> - x=e: ⟦v⟧ = JOIN(v) ∪ {x}
> - if x: ⟦v⟧ = JOIN(v)
> - entry: ⟦v⟧ = ∅
> - return: ⟦v⟧ = JOIN(v)

### 개념 설명
초기화된 변수 분석의 공식적인 정의입니다.

### 상세한 예시
각 규칙의 의미:

- **State = (P(Var), ⊇)**: 역방향 격자. 더 많은 변수가 초기화될수록 ⊤에 가까움 (좋은 상태)

- **JOIN(v) = ∩_{u∈pred(v)} ⟦u⟧**: 모든 선행 경로에서 초기화된 변수들만 merge 후에도 초기화된 것으로 취급 (Must)

- **x=e: ⟦v⟧ = JOIN(v) ∪ {x}**: 할당은 변수를 초기화. 따라서 JOIN(v)에 x를 추가. (생존 변수 분석과 반대!)

- **if x: ⟦v⟧ = JOIN(v)**: 조건문은 초기화하지 않음

- **entry: ⟦v⟧ = ∅**: 프로그램 진입점에서는 아무 변수도 초기화되지 않음

- **return: ⟦v⟧ = JOIN(v)**: 반환문은 초기화하지 않음

### 배경 지식
흥미로운 비교:
- **생존 변수 분석** (May, Backward): ⟦v⟧ = JOIN(v) \ {x} ∪ vars(e)
  - 할당을 통해 변수가 "죽음" (더 이상 필요 없음)
  - 역순으로 추적

- **초기화 분석** (Must, Forward): ⟦v⟧ = JOIN(v) ∪ {x}
  - 할당을 통해 변수가 "태어남" (초기화됨)
  - 정순으로 추적

---

## Slide 32: Transfer Functions

> **Transfer Functions**
> - All constraint functions: ⟦v⟧ = t_v(JOIN(v)) where t_v : L → L
> - Example (live variable analysis):
>   - x=e: ⟦v⟧ = JOIN(v) \ {x} ∪ vars(e)
>   - if x: ⟦v⟧ = JOIN(v) ∪ {x}
>   - entry: ⟦v⟧ = JOIN(v)
>   - return: ⟦v⟧ = JOIN(v)

### 개념 설명
전달 함수 (Transfer Function)는 각 CFG 노드에서 JOIN 결과를 입력받아 그 노드의 출력 상태를 계산하는 함수입니다.

모든 제약 규칙은 ⟦v⟧ = t_v(JOIN(v)) 형태로 표현할 수 있습니다. 여기서 t_v는 노드 v의 전달 함수입니다.

### 상세한 예시
생존 변수 분석의 전달 함수들:

**할당 x=e:**
- t_{x=e}(s) = s \ {x} ∪ vars(e)
- 입력 s (노드 이후 상태): {x, y}
- x를 제거: {y}
- vars(y+1) = {y} 추가: {y}
- 출력: {y}

**조건문 if x:**
- t_{if x}(s) = s ∪ {x}
- 입력 s: {y}
- x 추가: {x, y}
- 출력: {x, y}

**진입점 entry:**
- t_{entry}(s) = s
- 항등 함수 (Identity Function)

**반환점 return:**
- t_{return}(s) = s
- 항등 함수

### 배경 지식
전달 함수의 성질:
1. **모노톤성 (Monotonicity)**: s₁ ⊆ s₂이면 t_v(s₁) ⊆ t_v(s₂)
   - 더 많은 정보를 입력하면 더 많은 정보가 출력됨
   - 고정점 계산이 수렴함을 보장

2. **합성성 (Compositionality)**: 전달 함수들을 합성하여 경로의 전체 효과를 계산할 수 있음

### 전체적 맥락
전달 함수는 단순히 수학적 추상화가 아니라, 실제 알고리즘 구현의 핵심입니다. 다음 슬라이드에서 보게 될 PropagationWorkListAlgorithm은 전달 함수를 직접 사용하여 효율성을 높입니다.

---

## Slide 33: Transfer Functions (cont.)

> **Transfer Functions (cont.)**
> - t_v is a *transfer function* for the CFG node
>   - Forward: input = state before node, output = state after node
>   - Backward: input = state after node, output = state before node
> - Example: t_{x=e}(s) = s \ {x} ∪ vars(e)

### 개념 설명
전달 함수의 입출력이 전진 분석과 후진 분석에서 어떻게 다른지를 명확히 합니다.

### 상세한 예시
전진 분석 (예: 초기화 분석):
- 입력: 노드 **이전** 상태 (이 노드 진입 시 초기화된 변수들)
- 처리: 노드 실행 (할당 등)
- 출력: 노드 **이후** 상태 (노드 실행 후 초기화된 변수들)

후진 분석 (예: 생존 변수 분석):
- 입력: 노드 **이후** 상태 (이 노드 이후 필요한 변수들)
- 역처리: 노드가 역으로 어떻게 영향을 미치는가
- 출력: 노드 **이전** 상태 (노드 실행 전에 필요한 변수들)

### 배경 지식
전달 함수 t_{x=e}(s) = s \ {x} ∪ vars(e)의 의미:
- 생존 변수 분석에서: 노드 이후 필요한 변수들 (s)로부터 시작
- x를 제거: 이 노드가 x를 정의하므로, 이전에 필요했던 x는 이제 불필요 (또는 이전 정의 참조)
- vars(e)를 추가: 식 e를 계산하려면 e의 변수들이 필요

### 전체적 맥락
전달 함수의 추상화는 실제 구현을 단순화합니다. JOIN과 전달 함수를 분리하여 생각하면:
1. 먼저 JOIN으로 진입하는 모든 경로의 정보를 합침
2. 그 후 전달 함수로 현재 노드의 효과를 적용

---

## Slide 34: Transfer Functions — Redundancy in SimpleWorkListAlgorithm

> **Transfer Functions — Redundancy in SimpleWorkListAlgorithm**
> - JOIN(v) = ⊔⟦u⟧ is computed each iteration, but ⟦u⟧ often hasn't changed → redundant
> - Use transfer functions to avoid redundancy
> - Now xᵢ = ⟦vᵢ⟧ is state *before* vᵢ (forward) or *after* vᵢ (backward)

### 개념 설명
SimpleWorkListAlgorithm의 주요 비효율성은 JOIN(v)를 매번 재계산한다는 것입니다. 만약 선행/후행 노드들의 상태 ⟦u⟧가 변하지 않았다면, JOIN(v)도 변하지 않으므로 재계산은 낭비입니다.

### 상세한 예시
SimpleWorkListAlgorithm의 문제:
```
x₃ = t₁(x₁) ⊔ t₂(x₂)  // 첫 반복
x₃ = t₁(x₁) ⊔ t₂(x₂)  // 두 번째 반복 (x₁, x₂가 변하지 않았다면 불필요)
```

전달 함수를 사용하면:
- x₃ 자체를 직접 업데이트할 수 있음
- x₁이나 x₂가 변할 때만 x₃을 재계산하면 됨
- 불변 상태에서는 연쇄 효과를 계산하지 않음

### 배경 지식
상태의 의미 변화:
- SimpleWorkListAlgorithm: xᵢ = ⟦vᵢ⟧ (노드 이후 상태, 전진 분석 기준)
- PropagationWorkListAlgorithm: xᵢ도 의미가 같지만, 계산 순서가 다름
  - Forward: xᵢ = ⟦vᵢ⟧ (노드 이전 상태로 재정의)
  - Backward: xᵢ = ⟦vᵢ⟧ (노드 이후 상태로 재정의)

### 전체적 맥락
이 최적화는 단순해 보이지만 매우 효과적입니다. 실제로는:
- SimpleWorkListAlgorithm: O(n · h · k)
- PropagationWorkListAlgorithm: 여전히 O(n · h · k) 최악의 경우이지만, 평균적으로 훨씬 빠름

다음 슬라이드에서 PropagationWorkListAlgorithm을 자세히 보겠습니다.

---

## Slide 35: Transfer Functions — State Positions (Forward)

> **Transfer Functions — State Positions (Forward)** ← NEW SLIDE
> Two diagrams side by side showing a linear CFG (entry → v₁ → v₂ → v₃ → return):
> - W/o transfer functions: ⟦v⟧ is positioned AFTER each node (entry then ⟦entry⟧, v₁ then ⟦v₁⟧, etc.)
> - W/ transfer functions: ⟦v⟧ is positioned BEFORE each node (⟦entry⟧ then entry, ⟦v₁⟧ then v₁, etc.)
> This visually shows how the meaning of ⟦v⟧ shifts when using transfer functions in forward analysis.

### 개념 설명
전진 분석에서 전달 함수를 사용할 때 상태 위치가 어떻게 변하는지를 시각적으로 보여줍니다.

**전달 함수 없이:**
- ⟦v⟧는 노드 **이후**의 상태를 나타냄
- CFG에서 노드 다음에 상태가 위치
- 예: entry → ⟦entry⟧ → v₁ → ⟦v₁⟧ → v₂ → ⟦v₂⟧

**전달 함수 사용:**
- ⟦v⟧는 노드 **이전**의 상태를 나타냄으로 재정의
- CFG에서 노드 이전에 상태가 위치
- 예: ⟦entry⟧ → entry → ⟦v₁⟧ → v₁ → ⟦v₂⟧ → v₂

### 상세한 예시
초기화 분석 (전진 분석)의 예:

**전달 함수 없이:**
```
[어떤 변수도 초기화 안 됨]
        ↓
      entry
        ↓
[어떤 변수도 초기화 안 됨]
        ↓
      x = 1
        ↓
    [{x}는 초기화됨]
```

여기서 ⟦entry⟧는 "entry 실행 후" 상태이고, ⟦x=1⟧는 "x=1 실행 후" 상태입니다.

**전달 함수 사용:**
```
[어떤 변수도 초기화 안 됨]
        ↓
      entry  (t_entry를 적용)
        ↓
[어떤 변수도 초기화 안 됨]
        ↓
      x = 1  (t_{x=1}을 적용)
        ↓
    [{x}는 초기화됨]
```

여기서는 ⟦entry⟧를 "entry 진입 시" 상태로 생각하고, 전달 함수 t_entry를 적용하면 "entry 이후" 상태가 됩니다.

### 배경 지식
이것은 단순한 표기법 변화가 아니라 **개념적 변화**입니다:

**SimpleWorkListAlgorithm (전달 함수 없이):**
- ⟦v⟧ = t_v(JOIN(v))를 반복 계산
- JOIN(v)를 매번 전체 재계산 (비효율)

**PropagationWorkListAlgorithm (전달 함수 사용):**
- 상태 xᵢ를 "노드 이전" 상태로 재정의
- 이웃 노드들을 직접 업데이트
- JOIN 계산이 암시적으로 됨 (전달된 값들을 합치기)

### 전체적 맥락
이 시각화는 다음 슬라이드 (36, 37)과 연결됩니다:
- Slide 35 (Forward): 전진 분석에서 상태 위치 변화
- Slide 36 (Backward): 후진 분석에서 상태 위치 변화 (거울상)
- Slide 37 (Join): JOIN 연산이 어디에서 일어나는지

---

## Slide 36: Transfer Functions — State Positions (Backward)

> **Transfer Functions — State Positions (Backward)** ← NEW SLIDE
> Two diagrams side by side showing same linear CFG:
> - W/o transfer functions: ⟦v⟧ is positioned BEFORE each node (⟦entry⟧ then entry, ⟦v₁⟧ then v₁, etc.)
> - W/ transfer functions: ⟦v⟧ is positioned AFTER each node (entry then ⟦entry⟧, v₁ then ⟦v₁⟧, etc.)
> This is the mirror image of slide 35 for backward analysis.

### 개념 설명
후진 분석에서 전달 함수를 사용할 때 상태 위치가 어떻게 변하는지를 보여줍니다. 이것은 전진 분석 (Slide 35)의 거울상입니다.

**전달 함수 없이:**
- ⟦v⟧는 노드 **이전**의 상태를 나타냄
- CFG에서 노드 이전에 상태가 위치
- 예: ⟦entry⟧ → entry → ⟦v₁⟧ → v₁ → ⟦v₂⟧ → v₂

**전달 함수 사용:**
- ⟦v⟧는 노드 **이후**의 상태를 나타냄으로 재정의
- CFG에서 노드 다음에 상태가 위치
- 예: entry → ⟦entry⟧ → v₁ → ⟦v₁⟧ → v₂ → ⟦v₂⟧

### 상세한 예시
생존 변수 분석 (후진 분석)의 예:

**전달 함수 없이:**
```
[어떤 변수도 생존하지 않음]
        ↑
    z = y
        ↑
    [{y}는 생존]
        ↑
      y = 1
        ↑
[어떤 변수도 생존하지 않음]
```

여기서 ⟦z=y⟧는 "z=y 이전" 상태입니다 (후진이므로 거꾸로).

**전달 함수 사용:**
```
[어떤 변수도 생존하지 않음]
        ↑
    z = y  (t_{z=y}을 적용)
        ↑
    [{y}는 생존]
        ↑
      y = 1  (t_{y=1}을 적용)
        ↑
[어떤 변수도 생존하지 않음]
```

여기서 ⟦z=y⟧를 "z=y 이후" 상태로 생각하고, 역으로 전달 함수를 적용하면 "z=y 이전" 상태가 됩니다.

### 배경 지식
Slide 35와의 대칭성:
- **Forward (Slide 35)**: 전달 함수 없이는 ⟦v⟧가 노드 **이후**, 전달 함수 사용 시 노드 **이전**
- **Backward (Slide 36)**: 전달 함수 없이는 ⟦v⟧가 노드 **이전**, 전달 함수 사용 시 노드 **이후**

이는 전진과 후진의 방향성 때문입니다:
- 전진: 앞쪽으로 가면서 상태가 변함 (입력 → 처리 → 출력)
- 후진: 뒤쪽으로 가면서 상태가 변함 (출력 ← 역처리 ← 입력)

### 전체적 맥락
이 슬라이드는 **왜 PropagationWorkListAlgorithm이 구현 관점에서 효율적인지**를 설명합니다:
- 상태 위치를 일관되게 정의 (항상 노드의 한쪽)
- 이웃 노드들과의 정보 전파가 단순해짐
- SimpleWorkListAlgorithm의 JOIN 재계산 문제 해결

---

## Slide 37: Transfer Functions — State Positions (Join)

> **Transfer Functions — State Positions (Join)** ← NEW SLIDE
> Two diagrams showing a merge CFG (v₁ and v₂ merge into v₃):
> - W/o transfer functions: ⟦v₁⟧ and ⟦v₂⟧ are after their nodes, then merge into ⟦v₃⟧ below v₃
> - W/ transfer functions: ⟦v₁⟧ and ⟦v₂⟧ are after their nodes, then merge BEFORE v₃ as ⟦v₃⟧
> Shows how JOIN operates: without transfer functions, ⟦v₃⟧ = t_{v₃}(⟦v₁⟧ ⊔ ⟦v₂⟧). With transfer functions, the join happens first (⟦v₃⟧ = ⟦v₁⟧ ⊔ ⟦v₂⟧) and then transfer function is applied separately.

### 개념 설명
JOIN 연산이 전달 함수 사용 여부에 따라 어디에서 일어나는지를 시각적으로 보여줍니다. 이것이 SimpleWorkListAlgorithm과 PropagationWorkListAlgorithm의 핵심 차이입니다.

### 상세한 예시
전진 분석 (예: 초기화 분석)에서 노드들이 merge되는 경우를 봅시다:

**전달 함수 없이:**
```
[{x} 초기화]         [{y} 초기화]
    v₁ ────┐  v₂ ────┘
           v₃
           ↓
    ⟦v₃⟧ = t_{v₃}(⟦v₁⟧ ⊔ ⟦v₂⟧)
           ↓
    [{x,y} 초기화? 또는 ∩?]
```

SimpleWorkListAlgorithm은:
1. ⟦v₁⟧과 ⟦v₂⟧을 먼저 계산 (각각 노드 이후 상태)
2. 이들을 JOIN (⊔)으로 합침: JOIN(v₃) = ⟦v₁⟧ ⊔ ⟦v₂⟧
3. 전달 함수 t_{v₃}를 적용: ⟦v₃⟧ = t_{v₃}(JOIN(v₃))

**전달 함수 사용:**
```
⟦v₁⟧=[...]                ⟦v₂⟧=[...]
  v₁                        v₂
  ↓                         ↓
────────┐  JOIN(v₃)  ┌────────
        └─────⊔──────┘
         (여기서 merge)
              ↓
         ⟦v₃⟧ (v₃ 이전 상태)
              ↓
            t_{v₃} 적용
              ↓
         다음 노드로 전파
```

PropagationWorkListAlgorithm은:
1. ⟦v₁⟧과 ⟦v₂⟧을 계산하면서 바로 v₃에 전파
2. v₃에서 JOIN (⊔): x₃ = x₃ ⊔ (t_{v₁}(x₁))
3. v₃에서 JOIN (⊔): x₃ = x₃ ⊔ (t_{v₂}(x₂))
4. 다음 노드들로 전파

### 배경 지식
두 알고리즘의 수학적 관계:

**SimpleWorkListAlgorithm:**
- ⟦v₃⟧ = t_{v₃}(⊔_{v∈pred(v₃)} ⟦v⟧)

**PropagationWorkListAlgorithm:**
- x₃ = ⊔_{v∈pred(v₃)} t_v(x_v)

이 두 식이 같은 고정점에 도달한다는 것이 알고리즘의 정당성입니다 (전달 함수가 분배법칙을 만족할 때).

### 전체적 맥락
이 시각화는 **효율성의 핵심**을 보여줍니다:

**SimpleWorkListAlgorithm의 문제:**
- 각 노드 v₃에 진입할 때마다, JOIN(v₃)을 **전체 재계산**
- 선행 노드들의 상태가 변하지 않았어도 재계산 (낭비)

**PropagationWorkListAlgorithm의 개선:**
- 상태 변화가 있을 때만 이웃 노드에 전파
- 자동으로 JOIN의 효과가 누적됨
- 불필요한 재계산 제거

---

## Slide 38: PropagationWorkListAlgorithm

> **PropagationWorkListAlgorithm** ← UPDATED (added s_start)
> ```
> PropagationWorkListAlgorithm(t₁, ..., tₙ, s_start):
>   (x₁, x₂, ..., xₙ) ← (s_start, ⊥, ..., ⊥)
>   W ← {v₁, ..., vₙ}
>   while W ≠ ∅ do
>     vᵢ ← W.removeOne()
>     y ← t_{vᵢ}(xᵢ)
>     for vⱼ ∈ dep(vᵢ):
>       z ← xⱼ ⊔ y
>       if xⱼ ≠ z:
>         xⱼ ← z
>         W.add(vⱼ)
>   return x
> ```
> Note: now takes s_start as parameter. First element initialized to s_start instead of ⊥.

### 개념 설명
PropagationWorkListAlgorithm은 전달 함수를 직접 사용하여 SimpleWorkListAlgorithm의 비효율성을 개선한 알고리즘입니다. **UPDATED 부분**: 이제 s_start를 파라미터로 받아서 첫 번째 노드 (진입점 또는 종료점)를 특별히 초기화합니다.

### 상세한 예시
알고리즘의 동작을 단계별로 설명합니다:

**초기화 (Initialization):**
- x₁ = s_start (진입점 또는 종료점의 초기 상태)
- x₂, ..., xₙ = ⊥ (다른 모든 노드는 최악의 상태)
- W = {v₁, ..., vₙ} (모든 노드를 WorkList에 추가)

**반복 (Main Loop):**
1. WorkList에서 노드 vᵢ를 제거
2. 전달 함수 적용: y = t_{vᵢ}(xᵢ)
   - 이것은 vᵢ의 출력 상태입니다 (또는 후진에서는 입력 상태)
3. 각 의존 노드 vⱼ ∈ dep(vᵢ)에 대해:
   - JOIN: z = xⱼ ⊔ y
   - 상태가 변했으면: xⱼ = z, vⱼ를 WorkList에 추가
4. WorkList가 빌 때까지 반복

**종료:**
- 고정점 도달: 모든 상태가 변하지 않음
- 반환: 최종 상태들 (x₁, ..., xₙ)

### 상세한 예시 (구체적 실행)
전진 분석 (초기화 분석) 예시:

```
CFG: entry → v₁ → v₂ → return

초기화:
x_entry = ∅  (s_start = ∅, 시작점에서 아무것도 초기화 안 됨)
x_v₁ = ⊥     (역순 격자에서는 전체 집합)
x_v₂ = ⊥
x_return = ⊥

첫 번째 반복 (vᵢ = entry):
y = t_entry(x_entry) = t_entry(∅) = ∅
dep(entry) = {v₁}
z = x_v₁ ⊔ y = ⊥ ⊔ ∅ = ∅ (May에서는 ∪, Must에서는 ∩)
x_v₁ = ∅, W에 v₁ 추가

두 번째 반복 (vᵢ = v₁, 가정: v₁은 x=1):
y = t_{x=1}(x_v₁) = t_{x=1}(∅) = ∅ ∪ {x} = {x}
dep(v₁) = {v₂}
z = x_v₂ ⊔ y = ⊥ ⊔ {x} = {x}
x_v₂ = {x}, W에 v₂ 추가
...
```

### 배경 지식
**s_start 파라미터의 의미:**
- 전진 분석: entry 노드의 초기 상태 (보통 ∅ 또는 특정 초기값)
- 후진 분석: return 노드의 초기 상태 (보통 ∅)

예: reaching definition 분석에서 entry에 도달하는 정의가 있다면, s_start = {그 정의들}로 설정

### 전체적 맥락
SimpleWorkListAlgorithm과의 핵심 차이점:

**SimpleWorkListAlgorithm:**
```python
while W ≠ ∅:
    vᵢ ← W.removeOne()
    y ← JOIN(vᵢ)  # 모든 선행/후행 노드의 상태를 매번 합침
    y' ← t_{vᵢ}(y)
    if xᵢ ≠ y':
        xᵢ ← y'
        for vⱼ ∈ dep(vᵢ):
            W.add(vⱼ)
```

**PropagationWorkListAlgorithm:**
```python
while W ≠ ∅:
    vᵢ ← W.removeOne()
    y ← t_{vᵢ}(xᵢ)  # xᵢ는 이미 JOIN 결과를 포함
    for vⱼ ∈ dep(vᵢ):
        z ← xⱼ ⊔ y  # 증분 UPDATE
        if xⱼ ≠ z:
            xⱼ ← z
            W.add(vⱼ)
```

**효율성 개선:**
- JOIN을 명시적으로 재계산하지 않음
- 이웃 노드들에게 변화된 정보만 전파
- 실제로는 JOIN이 암시적으로 이루어짐 (y ⊔ xⱼ)

---

## Slide 39: PropagationWorkListAlgorithm — Intuition

> **PropagationWorkListAlgorithm — Intuition**
> - This gives the same analysis results
> - Intuition:
>   - SimpleWorkListAlgorithm computes x₃ = t₁(x₁) ⊔ t₂(x₂)
>   - PropagationWorkListAlgorithm computes x₃ = x₃ ⊔ t₁(x₁) and x₃ = x₃ ⊔ t₂(x₂)
>   - If f is monotone and g(x) = f(x) ⊔ x, then lfp(g) = lfp(f)

### 개념 설명
왜 두 알고리즘이 같은 결과를 주는지, 그리고 PropagationWorkListAlgorithm이 효율적인지를 직관적으로 설명합니다.

### 상세한 예시
merge point v₃에서 두 선행 노드 v₁, v₂로부터 정보가 들어오는 경우:

**SimpleWorkListAlgorithm:**
- 한 번에 계산: x₃ = t₁(x₁) ⊔ t₂(x₂)
- JOIN(v₃) = t₁(x₁) ⊔ t₂(x₂)을 먼저 구한 후
- 전달 함수를 적용: x₃' = t₃(JOIN(v₃))

**PropagationWorkListAlgorithm:**
- 단계별 계산:
  1. v₁이 업데이트될 때: x₃ = x₃ ⊔ t₁(x₁) (첫 번째 입력)
  2. v₂가 업데이트될 때: x₃ = x₃ ⊔ t₂(x₂) (두 번째 입력 누적)
- 결과: x₃ = ⊥ ⊔ t₁(x₁) ⊔ t₂(x₂) = t₁(x₁) ⊔ t₂(x₂)

### 배경 지식
**수학적 정당성:**

함수 f가 **단조함수 (Monotone Function)**라면:
- x ⊆ y ⟹ f(x) ⊆ f(y)
- 모든 전달 함수는 단조함수

함수 g(x) = f(x) ⊔ x를 정의하면 (f(x)를 누적하는 함수):
- lfp(g) = lfp(f) (같은 최소 고정점)

**직관:**
- f(x)를 한 번에 전체 계산 (SimpleWorkListAlgorithm) vs.
- f(x)를 증분으로 누적 (PropagationWorkListAlgorithm)
- 같은 고정점에 도달

### 전체적 맥락
이 슬라이드는 두 알고리즘의 이론적 동등성을 보여줍니다. 둘 다:
1. 같은 최소 고정점을 계산
2. 같은 분석 결과를 제공
3. 하지만 구현 효율성은 다름

**실제 성능:**
- SimpleWorkListAlgorithm: 모든 노드와 가지를 매번 확인
- PropagationWorkListAlgorithm: 변화가 있는 부분만 처리

이것이 프로그램 분석 도구에서 PropagationWorkListAlgorithm을 선호하는 이유입니다.

---

## Slide 40: Summary

> **Summary**
> - Live variable analysis: which variables may be needed in the future (backward, may)
> - Available expression analysis: which expressions already computed (forward, must)
> - Very busy expression analysis: which expressions will definitely be evaluated (backward, must)
> - Reaching definition analysis: which assignments may define current variable values (forward, may)
> - Classified along forward/backward and may/must axes
> - PropagationWorkListAlgorithm avoids redundant JOIN recomputation by propagating transfer function results incrementally

### 개념 설명
이 강의 전체를 정리하는 요약 슬라이드입니다. 4가지 주요 데이터플로우 분석 기법을 다시 한 번 정리하고, 그들의 특성과 구현 방법을 요약합니다.

### 상세한 예시
각 분석 기법의 요약:

**1. Live Variable Analysis (생존 변수 분석)**
- 방향: **Backward** (미래 행동 추적)
- 종류: **May** (가능한 변수들)
- 정격: (P(Var), ⊆) 일반 격자
- 응용: 레지스터 할당, 데드 코드 제거
- 직관: "이 변수가 나중에 필요할까?"

**2. Available Expression Analysis (가용 식 분석)**
- 방향: **Forward** (과거 행동 추적)
- 종류: **Must** (확실히 계산된 식들)
- 격자: (P(Expr), ⊇) 역방향 격자
- 응용: 공통 부분식 제거 (CSE)
- 직관: "이 식의 값을 재사용할 수 있을까?"

**3. Very Busy Expression Analysis (매우 바쁜 식 분석)**
- 방향: **Backward** (미래 행동 추적)
- 종류: **Must** (확실히 평가될 식들)
- 격자: (P(Expr), ⊇) 역방향 격자
- 응용: 루프 불변식 제거, 코드 호이스팅
- 직관: "이 식을 더 앞으로 이동시킬 수 있을까?"

**4. Reaching Definition Analysis (도달하는 정의 분석)**
- 방향: **Forward** (과거 행동 추적)
- 종류: **May** (가능한 정의들)
- 격자: (P(Def), ⊆) 일반 격자
- 응용: Def-use 그래프 구성, 변수 추적
- 직관: "이 변수의 현재 값은 어디서 정의되었을까?"

### 배경 지식
**분류 표 (2×2 Matrix):**

|        | Forward | Backward |
|--------|---------|----------|
| May    | Reaching Definition | Live Variable |
| Must   | Available Expression | Very Busy Expression |

이 표는:
- 데이터플로우 분석의 설계 공간을 나타냄
- 각 사분면은 서로 다른 특성을 가짐
- 새로운 분석을 설계할 때 이 틀을 사용할 수 있음

### 전체적 맥락
**강의의 핵심 메시지:**

1. **다양한 분석 기법**: 프로그램 특성을 추적하는 방법은 하나가 아님
   - 같은 문제도 다양한 각도에서 분석 가능

2. **구조적 접근**: Forward/Backward, May/Must로 분류하면 이해하기 쉬움
   - 새로운 분석을 설계할 때도 이 틀을 사용 가능

3. **실용적 구현**: PropagationWorkListAlgorithm
   - 이론과 실제 구현 사이의 간극을 메움
   - 효율성을 고려한 알고리즘 설계

4. **최적화와의 연결**: 모든 분석은 최적화를 목표로 함
   - 분석 결과를 실제 컴파일러 최적화로 변환

**다음 주제로의 확장:**
- 더 정교한 분석 기법 (interprocedural analysis)
- 동적 분석 (Runtime analysis)
- 추상 해석 (Abstract Interpretation)의 일반화
