# Dataflow Analysis (2) - 강의 설명

## Slide 1: 제목 슬라이드

> Dataflow Analysis (2), CSE552 Program Analysis — Lecture 8, Jaemin Hong

### 개념 설명
이 강의는 데이터플로우 분석의 두 번째 부분입니다. 프로그램 분석에서 변수들이 프로그램의 어느 지점에서 어떤 상태인지 파악하는 것은 매우 중요해요. 이번 강의에서는 여러 종류의 데이터플로우 분석 기법들을 배우게 됩니다.

### 전체적 맥락
첫 번째 강의에서는 데이터플로우 분석의 기본 개념(lattice, monotone function, fixed point 등)을 배웠어요. 이번 강의부터는 구체적인 분석 기법들을 하나씩 살펴봅니다.

---

## Slide 2: Live Variable Analysis (활성 변수 분석)

> A variable is *live* at a program point if there exists an execution where its value is read later in the execution without it being written to in between

### 개념 설명
활성 변수 분석(live variable analysis)은 어느 변수가 나중에 사용될 가능성이 있는지를 파악하는 분석이에요. 어떤 프로그램 지점에서 변수가 "활성(live)"이라는 것은, 그 변수의 현재 값이 앞으로 다시 쓰여지기 전에 읽혀질 가능성이 있다는 뜻입니다.

### 상세한 예시
```
// nothing is live
x = input();
// x is live (x의 값이 나중에 사용될 수 있음)
if input() {
  // x is live
  y = x;
  // y is live (y의 값이 나중에 사용될 수 있음)
} else {
  // nothing is live (이 경로에서는 사용 안 함)
  y = 1;
  // y is live
}
// y is live
z = y;
```

예를 들어 첫 번째 분기에서 x를 읽고, 두 번째 분기에서는 1을 할당해요. 두 분기 이후에는 y가 활성인데, 첫 번째 분기에서 y = x로 할당되므로 x도 활성이어야 해요.

---

## Slide 3: Live Variable Analysis — 동기 (Motivation)

> - We can approximate the set of live variables using dataflow analysis
>   - Application: register allocation
>   - We want: the answer "not live" can be trusted and "live" is safe but useless

### 개념 설명
활성 변수 분석이 왜 필요한지 알아봅시다. 주요 응용은 **레지스터 할당(register allocation)**이에요. 컴파일러는 제한된 수의 레지스터가 있으므로, 활성이 아닌 변수들의 값을 안전하게 재사용할 수 있습니다.

### 배경 지식
- "not live"라는 답변은 신뢰할 수 있어요(truly not live)
- "live"라는 답변은 보수적(conservative)이어요. 실제로는 활성이 아닐 수도 있지만, 활성이 아니라고 잘못 판단하는 것보다는 낫습니다
- 이를 **may analysis** 또는 **sound but incomplete analysis**라고 합니다

---

## Slide 4: Live Variable Analysis — 추상 상태 (Abstract States)

> [CFG: v₁ at top splits to v₂ and v₃ which merge]
> ⟦v₁⟧ = {x, y}, ⟦v₂⟧ = {x}, ⟦v₃⟧ = {y}
> - State = (P(Var), ⊆) — Power set lattice
> - For each CFG node v, ⟦v⟧ denotes the set of variables live before the node
> - JOIN(v) = ⊔_{u∈succ(v)} ⟦u⟧ = ∪_{u∈succ(v)} ⟦u⟧
>   - This combines abstract states from the **successors**

### 개념 설명
활성 변수 분석에서 추상 상태는 변수들의 집합이에요. Lattice는 Power set lattice with ⊆ (부분집합)을 사용합니다. 각 CFG 노드 v에서 ⟦v⟧는 그 노드 "앞에서" 활성인 변수들의 집합이에요.

### 상세한 예시
- 만약 v₁이 분기점이고 v₂, v₃ 두 경로로 나뉜다면:
  - v₂에서 x가 활성이고, v₃에서 y가 활성이면
  - v₁에서는 {x, y}가 모두 활성이어야 합니다 (union을 취함)

### 배경 지식
**활성 변수 분석은 백워드 분석(backward analysis)**입니다. 왜냐하면 미래의 정보를 이용해서 현재의 상태를 결정하기 때문이에요. 따라서 JOIN은 **successors**로부터 정보를 모읍니다.

---

## Slide 5: Live Variable Analysis — 제약 규칙 (Constraint Rule) - 할당

> - x=e: ⟦v⟧ = JOIN(v) \ {x} ∪ vars(e)

### 개념 설명
변수 할당 x=e에서:
1. JOIN(v)는 이 노드 다음에서 활성인 변수들
2. 할당 후 x는 새로 정의되므로, 이전의 x 값은 필요 없어요 (\ {x})
3. 하지만 우변의 식 e에서 사용되는 변수들은 활성이어야 해요 (∪ vars(e))

### 상세한 예시
```
// y and z are live (y와 z가 활성)
x = y + z;
// x is live (x가 새로 정의되었고, y와 z는 더 이상 활성 아님)
```

실제로 x = y + z 이전에 y와 z가 활성이어야 이들이 읽혀질 수 있어요.

---

## Slide 6: Live Variable Analysis — 제약 규칙 (나머지)

> - if x: ⟦v⟧ = JOIN(v) ∪ {x}
> - entry: ⟦v⟧ = JOIN(v)
> - return: ⟦v⟧ = JOIN(v) = ∅

### 개념 설명
다른 종류의 노드들에 대한 규칙들입니다:

1. **if x**: 조건 검사에서 x를 읽으므로 x는 활성이어야 해요
2. **entry**: 입구 노드에서는 successors의 활성 변수들만 고려해요
3. **return**: 반환 노드에서는 더 이상 활성인 변수가 없어요 (프로그램이 끝나니까)

---

## Slide 7: Available Expression Analysis (가용 표현식 분석)

> A nontrivial expression (not a literal, not a variable) in a program is *available* at a program point if its current value has already been computed earlier in the execution

### 개념 설명
가용 표현식 분석(available expression analysis)은 어떤 식이 이미 계산되었는지를 파악하는 분석이에요. 표현식 e가 어떤 지점에서 "가용(available)"이라는 것은, 그 값이 이미 계산되었고 그 이후로 입력값이 변하지 않았다는 뜻입니다.

"비자명한 표현식"이란 리터럴이나 단순 변수가 아닌 것, 즉 실제로 계산이 필요한 식을 말해요 (예: y + 1).

### 상세한 예시
```
// nothing is available
x = y + 1;
// y + 1 is available (방금 계산했으므로 가용함)
if input() {
  // y + 1 is available
  y = z + 1;
  // z + 1 is available
} else {
  // y + 1 is available
  x = z + 1;
  // y + 1 and z + 1 are available
}
// z + 1 is available (양쪽 경로 모두에서 계산됨)
w = (z + 1) + (y + 1);
```

두 분기 이후에 z + 1이 가용한 이유는, 양쪽 분기 모두에서 계산되었기 때문이에요.

---

## Slide 8: Available Expression Analysis — 동기

> - We can approximate the set of available expressions using dataflow analysis
>   - Application: optimization (eliminating redundant computations)
>   - We want: the answer "available" can be trusted and "not available" is safe but useless

### 개념 설명
가용 표현식 분석의 주요 응용은 **최적화(optimization)**입니다. 특히 중복된 계산을 제거할 수 있어요.

### 배경 지식
- "available"이라는 답변은 신뢰할 수 있어요(truly available)
- "not available"이라는 답변은 보수적이에요. 실제로는 가용할 수도 있지만, 가용하지 않다고 가정해도 안전합니다
- 이를 **must analysis**라고 합니다

---

## Slide 9: Available Expression Analysis — 최적화 예시

> Before:
> ```
> x = y + 1;
> if input() {
>   y = z + 1;
> } else {
>   x = z + 1;
> }
> w = (z + 1) + (y + 1);
> ```
> After:
> ```
> x = y + 1;
> if input() {
>   zplus1 = z + 1;
>   y = zplus1;
> } else {
>   zplus1 = z + 1;
>   x = zplus1;
> }
> w = zplus1 + (y + 1);
> ```

### 개념 설명
가용 표현식 분석을 이용하면 다음과 같은 최적화가 가능해요:
1. 양쪽 분기에서 z + 1을 계산하는 중복을 제거
2. 분기 앞에서 z + 1을 한 번 계산하고 변수에 저장
3. 양쪽 분기에서 그 변수를 재사용

이렇게 하면 z + 1 계산을 한 번으로 줄일 수 있어요!

---

## Slide 10: Available Expression Analysis — 추상 상태

> [CFG: v₁ and v₂ at top merge into v₃]
> ⟦v₁⟧ = {x+1}, ⟦v₂⟧ = {x+1, y+1}, ⟦v₃⟧ = {x+1}
> - State = (P(Expr), ⊇) — **Reverse** power set lattice
> - For each CFG node v, ⟦v⟧ denotes the set of expressions available after the node
> - JOIN(v) = ⊔_{u∈pred(v)} ⟦u⟧ = ∩_{u∈pred(v)} ⟦u⟧
>   - This combines abstract states from the **predecessors**

### 개념 설명
가용 표현식 분석의 추상 상태도 표현식들의 집합이지만, lattice가 **역순(reverse)** power set이에요. 즉, ⊇ 관계를 사용합니다. 왜냐하면:
- 더 많은 표현식이 가용하다는 것은 더 적은 정보를 의미해요 (더 보수적)
- 더 적은 표현식이 가용하다는 것이 더 정확한 정보입니다

각 노드 v에서 ⟦v⟧는 그 노드 "뒤에서" 가용한 표현식들의 집합이에요.

### 상세한 예시
- v₁에서 {x+1}이 가용하고, v₂에서 {x+1, y+1}이 가용하면
- v₃ (merge point)에서는 {x+1}만 가용해요
- 왜냐하면 y+1은 v₁ 경로에서 계산되지 않았으니까요
- 따라서 JOIN은 **intersection** (∩)을 사용합니다

### 배경 지식
**가용 표현식 분석은 포워드 분석(forward analysis)**입니다. 과거의 정보를 이용해서 현재의 가능성을 판단하니까요. JOIN은 **predecessors**로부터 정보를 모웁니다.

---

## Slide 11: Available Expression Analysis — 제약 규칙 - 할당

> - x=e: ⟦v⟧ = (JOIN(v) ∪ exprs(e))↓x
>   - ↓x removes all expressions containing x
>   - exprs collects all nontrivial expressions
> - exprs(x) = ∅
> - exprs(n) = ∅
> - exprs(input()) = ∅
> - exprs(e₁ op e₂) = {e₁ op e₂} ∪ exprs(e₁) ∪ exprs(e₂)

### 개념 설명
할당 x=e에서의 규칙:
1. JOIN(v)는 이 노드 이전에 가용한 표현식들
2. exprs(e)는 식 e에서 새로 계산되는 표현식들
3. 하지만 x를 포함하는 모든 표현식은 제거해야 해요 (↓x)
   - 왜냐하면 x가 다시 정의되면, x를 포함하는 표현식의 값이 변할 수 있기 때문

exprs 함수는 비자명한 표현식들을 수집해요:
- 변수나 리터럴, input()은 계산할 필요가 없으므로 ∅
- 연산식은 연산식 자체와 부분 표현식들을 수집

### 상세한 예시
```
// x + 1 is available
x = x + (y + z);
// y + z is available (x가 재정의되므로 x+1은 제거됨)
```

이 예시에서:
- 이전에 x + 1이 가용했지만, x를 재정의하므로 x + 1은 더 이상 가용하지 않아요
- 새로 계산된 y + z는 가용해요
- exprs(x + (y + z)) = {x + (y + z), y + z}

---

## Slide 12: Available Expression Analysis — 제약 규칙 (나머지)

> - if x: ⟦v⟧ = JOIN(v)
> - entry: ⟦v⟧ = ⊤ = ∅
> - return: ⟦v⟧ = JOIN(v)

### 개념 설명
다른 종류의 노드들에 대한 규칙들입니다:

1. **if x**: 조건 검사는 어떤 표현식도 새로 계산하지 않으므로, 가용한 표현식은 그대로 유지돼요
2. **entry**: 프로그램 진입점에서는 아직 어떤 표현식도 계산되지 않았어요. ⊤은 reverse power set lattice에서 ∅을 의미합니다
3. **return**: 반환 노드도 새로운 표현식을 계산하지 않아요

---

## Slide 13: Very Busy Expression Analysis (매우 바쁜 표현식 분석)

> An expression is *very busy* if it will definitely be evaluated before its value changes

### 개념 설명
매우 바쁜 표현식 분석(very busy expression analysis)은 어떤 식이 앞으로 반드시 계산될 것인지를 파악하는 분석이에요. 표현식 e가 "매우 바쁘다(very busy)"는 것은, 그 값이 변하기 전에 반드시 평가될 것이라는 뜻입니다.

### 상세한 예시
```
// nothing is very busy
x = input();
// x + 1 is very busy (앞으로 반드시 계산될 것)
if input() {
  // x + 1 is very busy
  y = x + 1;
} else {
  // x + 1 and y + 1 are very busy
  z = x + 1;
  // y + 1 is very busy
  w = y + 1;
}
```

분석해 보면:
- x = input() 이후 양쪽 분기 모두에서 x + 1을 계산하므로, x + 1은 매우 바빠요
- else 분기의 w = y + 1 이전에는 y + 1도 매우 바빠요

---

## Slide 14: Very Busy Expression Analysis — 동기

> - We can approximate the set of very busy expressions using dataflow analysis
>   - Application: optimization (code hoisting)
>   - We want: the answer "very busy" can be trusted and "not very busy" is safe but useless

### 개념 설명
매우 바쁜 표현식 분석의 주요 응용은 **코드 호이스팅(code hoisting)**이에요. 양쪽 분기에서 모두 계산하는 코드를 분기 앞으로 올려서 중복을 제거할 수 있습니다.

### 배경 지식
- "very busy"라는 답변은 신뢰할 수 있어요(truly very busy)
- "not very busy"라는 답변은 보수적이에요
- 이것도 **must analysis**입니다

---

## Slide 15: Very Busy Expression Analysis — 최적화 예시 (if 문)

> Before:
> ```
> x = input();
> if input() {
>   y = x + 1;
> } else {
>   z = x + 1;
>   w = y + 1;
> }
> ```
> After:
> ```
> x = input();
> xplus1 = x + 1;
> if input() {
>   y = xplus1;
> } else {
>   z = xplus1;
>   w = y + 1;
> }
> ```

### 개념 설명
코드 호이스팅 최적화:
1. 양쪽 분기에서 x + 1을 계산하는 중복이 있어요
2. 분기 앞에서 미리 한 번 계산해서 변수에 저장
3. 양쪽 분기에서 그 변수를 재사용

이렇게 하면 x + 1 계산을 한 번으로 줄일 수 있어요!

---

## Slide 16: Very Busy Expression Analysis — 최적화 예시 (while 문)

> Before:
> ```
> x = input();
> while input() {
>   y = x + 1;
> }
> z = x + 1;
> ```
> After:
> ```
> x = input();
> xplus1 = x + 1;
> while input() {
>   y = xplus1;
> }
> z = xplus1;
> ```

### 개념 설명
반복문에서도 같은 최적화가 가능해요:
1. while 루프 안에서와 바깥에서 x + 1을 모두 계산해요
2. 루프 전에 미리 한 번 계산하면, 루프 안에서도 재사용할 수 있어요
3. 하지만 루프를 다시 실행하므로 x + 1의 값이 변할 수 없어요 (x가 루프 안에서 변하지 않으므로)

---

## Slide 17: Very Busy Expression Analysis — 추상 상태

> [CFG: v₁ at top splits to v₂ and v₃]
> ⟦v₁⟧ = {x+1}, ⟦v₂⟧ = {x+1, y+1}, ⟦v₃⟧ = {x+1}
> - State = (P(Expr), ⊇) — **Reverse** power set lattice
> - For each CFG node v, ⟦v⟧ denotes the set of expressions very busy before the node
> - JOIN(v) = ⊔_{u∈succ(v)} ⟦u⟧ = ∩_{u∈succ(v)} ⟦u⟧
>   - This combines abstract states from the **successors**

### 개념 설명
매우 바쁜 표현식 분석의 추상 상태:
- Lattice는 reverse power set (⊇)이에요
- 각 노드 v에서 ⟦v⟧는 그 노드 "앞에서" 매우 바쁜 표현식들의 집합
- JOIN은 **successors**로부터 정보를 모웁니다

### 배경 지식
**매우 바쁜 표현식 분석은 백워드 분석(backward analysis)**입니다. 미래의 정보(앞으로 반드시 계산될 것)를 이용해서 현재를 판단하니까요.

---

## Slide 18: Very Busy Expression Analysis — 제약 규칙

> - x=e: ⟦v⟧ = (JOIN(v)↓x) ∪ exprs(e)

### 개념 설명
할당 x=e에서의 규칙:
1. JOIN(v)는 이 노드 이후에 매우 바쁜 표현식들
2. x를 포함하는 표현식은 제거해요 (↓x)
   - 왜냐하면 x가 재정의되니까
3. 새로 계산되는 식의 표현식들을 추가해요

### 상세한 예시
```
// x + (y + z) and y + z are very busy
x = x + (y + z);
// x + 1 is very busy
```

이 예시에서:
- x + (y + z) 이후에 x + (y + z)와 y + z가 매우 바빠요
- x 재정의 이전에는 x가 포함된 표현식 (예: x + 1)도 매우 바쁜데
- x = x + (y + z) 이후에는 x가 변경되므로 이전의 x 포함 표현식은 제거돼요

### 배경 지식
나머지 규칙들:
- **if x**: ⟦v⟧ = JOIN(v) (새로 계산하지 않음)
- **entry**: ⟦v⟧ = JOIN(v)
- **return**: ⟦v⟧ = ⊤ = ∅ (반환 후에는 아무것도 매우 바쁘지 않음)

---

## Slide 19: Reaching Definition Analysis (도달하는 정의 분석)

> *Reaching definitions* for a program point are those assignments that may have defined the current values of variables

### 개념 설명
도달하는 정의 분석(reaching definition analysis)은 어떤 할당이 현재 변수의 값을 정의했을 수 있는지를 파악하는 분석이에요. 정의 d가 프로그램 지점에 "도달한다"는 것은, 그 정의가 현재 변수의 값을 설정했을 가능성이 있다는 뜻입니다.

### 상세한 예시
```
if input() {
  x = y;
  // x = y is a reaching definition
  x = y + 1;
  // x = y + 1 is a reaching definition (이전의 x = y는 덮어씌워짐)
} else {
  x = z + 1;
  // x = z + 1 is a reaching definition
}
// x = y + 1 and x = z + 1 are reaching definitions
// (양쪽 경로에서 온 정의들이 도달)
return x;
```

merge point에서:
- 첫 번째 분기에서 온 x = y + 1
- 두 번째 분기에서 온 x = z + 1
- 이 두 정의가 모두 도달해요

---

## Slide 20: Reaching Definition Analysis — 동기

> - We can approximate the set of reaching definitions using dataflow analysis
>   - Application: def-use graph (useful for optimizations)
>   - We want: the answer "not reaching" can be trusted and "reaching" is safe but useless

### 개념 설명
도달하는 정의 분석의 주요 응용은 **def-use graph 구성**이에요. 이를 통해:
- 각 변수의 정의와 사용 관계를 파악할 수 있어요
- 이를 기반으로 다양한 최적화를 수행할 수 있습니다

### 배경 지식
- "not reaching"이라는 답변은 신뢰할 수 있어요
- "reaching"이라는 답변은 보수적이에요
- 이를 **may analysis**라고 합니다

---

## Slide 21: Reaching Definition Analysis — 추상 상태

> [CFG: v₁ and v₂ merge into v₃]
> ⟦v₁⟧ = {x=y}, ⟦v₂⟧ = {x=y+1}, ⟦v₃⟧ = {x=y, x=y+1}
> - State = (P(Def), ⊆) — Power set lattice
>   - Def d ::= x=e
> - For each CFG node v, ⟦v⟧ denotes the set of definitions that may define values of variables at the program point after the node
> - JOIN(v) = ⊔_{u∈pred(v)} ⟦u⟧ = ∪_{u∈pred(v)} ⟦u⟧
>   - This combines abstract states from the **predecessors**

### 개념 설명
도달하는 정의 분석의 추상 상태:
- 상태는 정의들의 집합이에요 (정의 = 할당문)
- Lattice는 Power set with ⊆입니다
- 각 노드 v에서 ⟦v⟧는 그 노드 "뒤에서" 도달하는 정의들의 집합
- JOIN은 **predecessors**로부터 정보를 모웁니다 (union)

### 배경 지식
**도달하는 정의 분석은 포워드 분석(forward analysis)**입니다. 과거의 정의들을 이용해서 현재의 도달 가능성을 판단하니까요.

---

## Slide 22: Reaching Definition Analysis — 제약 규칙

> - x=e: ⟦v⟧ = (JOIN(v)↓x) ∪ {x=e}
>   - ↓x removes all definitions of x

### 개념 설명
할당 x=e에서의 규칙:
1. JOIN(v)는 이 노드 이전에 도달하는 정의들
2. 이전의 x에 대한 정의들은 제거해요 (↓x)
   - 왜냐하면 x가 새로 정의되니까 이전 정의는 더 이상 도달하지 않아요
3. 새로운 정의 {x=e}를 추가해요

### 상세한 예시
```
// x = y + 1 is a reaching definition
x = y;
// x = y is a reaching definition (이전의 x = y + 1은 제거됨)
```

x를 재정의하면:
- 이전의 x = y + 1은 더 이상 도달하지 않아요
- 새로운 정의 x = y가 도달하게 돼요

### 배경 지식
나머지 규칙들:
- **if x**: ⟦v⟧ = JOIN(v) (새 정의 없음)
- **entry**: ⟦v⟧ = JOIN(v) = ∅ (프로그램 시작)
- **return**: ⟦v⟧ = JOIN(v)

---

## Slide 23: 시간 복잡도 (Time Complexity)

> - For SimpleWorkListAlgorithm, if |dep(v)| is bounded by a constant for all nodes v, the worst-case time complexity is O(n · h · k) where:
>   - n is the number of CFG nodes
>   - h is the height of the lattice L = State
>   - k is the worst-case time required to compute fᵢ
> - O(n · m²) where:
>   - n is the number of CFG nodes
>   - m is the number of variables/expressions/definitions
>   - Because h = m, k = O(m)

### 개념 설명
SimpleWorkListAlgorithm의 시간 복잡도:
- 각 노드의 의존성이 상수로 제한되어 있으면: O(n · h · k)
  - n: CFG 노드 수
  - h: lattice의 높이
  - k: 각 함수 계산 시간
- 실제로는 보통: O(n · m²)
  - 각 분석에서 m은 변수/표현식/정의의 개수
  - Lattice 높이 h = m, 함수 계산 k = O(m)

### 배경 지식
이 복잡도는 최악의 경우(worst case)입니다. 대부분의 실제 프로그램은 이보다 훨씬 빨라요.

---

## Slide 24: Forward vs Backward 분석

> - A *forward* analysis computes information about the past behavior
>   - Examples: sign analysis, constant propagation analysis, available expression analysis, reaching definition analysis
>   - The analysis starts at the entry node and propagates information forward in the CFG
>   - JOIN is defined using pred
>   - dep = succ
> - A *backward* analysis computes information about the future behavior
>   - Examples: live variables analysis and very busy expressions analysis
>   - The analysis starts at the return node and propagates information backward in the CFG
>   - JOIN is defined using succ
>   - dep = pred

### 개념 설명
데이터플로우 분석을 두 가지 방향으로 분류할 수 있어요:

**포워드 분석 (Forward Analysis)**:
- 과거의 정보를 이용해서 현재를 판단
- 프로그램 시작점에서부터 끝점으로 진행하며 정보 전파
- JOIN은 predecessors로부터 정보 수집
- 의존성은 successors (dep = succ)

**백워드 분석 (Backward Analysis)**:
- 미래의 정보를 이용해서 현재를 판단
- 프로그램 끝점에서부터 시작점으로 역진행하며 정보 전파
- JOIN은 successors로부터 정보 수집
- 의존성은 predecessors (dep = pred)

### 상세한 예시
- Forward: "지금까지 어떤 식들을 계산했는가?" (available expression)
- Backward: "앞으로 어떤 변수들을 사용할 것인가?" (live variables)

---

## Slide 25: May vs Must 분석

> - A *may* analysis describes information that may possibly be true
>   - Examples: live variables analysis and reaching definitions analysis
>   - Typically uses a power set lattice
> - A *must* analysis describes information that must definitely be true
>   - Examples: available expression analysis and very busy expression analysis
>   - Typically uses a reverse power set lattice

### 개념 설명
데이터플로우 분석을 다른 방향으로 분류할 수도 있어요:

**May 분석 (May Analysis)**:
- "이것이 참일 가능성이 있는가?"를 묻습니다
- 여러 경로 중 하나라도 만족하면 "yes"
- Power set lattice (⊆)를 사용하고 JOIN은 union
- 보수적(conservative): 가능한 것을 모두 포함

**Must 분석 (Must Analysis)**:
- "이것이 반드시 참인가?"를 묻습니다
- 모든 경로에서 만족해야만 "yes"
- Reverse power set lattice (⊇)를 사용하고 JOIN은 intersection
- 보수적(conservative): 확실한 것만 포함

### 상세한 예시
- May: "이 변수가 나중에 사용될 수도 있나?" (live variable)
- Must: "이 식이 반드시 이미 계산되었나?" (available expression)

---

## Slide 26: May vs Must 분석 — 건전성 (Soundness)

> - May ≠ Sound, Must ≠ Complete
> - All these analyses are sound but not complete

### 개념 설명
흔한 오해를 바로잡아봅시다:
- May 분석이 항상 sound(건전)한 것은 아니에요
- Must 분석이 항상 complete(완전)한 것도 아니에요
- 중요한 것은 **모든 이 분석들이 sound하다**는 거예요!

### 배경 지식
- **Sound (건전)**: 분석이 보수적이어서 위험한 상황을 놓치지 않아요
- **Complete (완전)**: 분석이 정확해서 거짓 경보가 없어요
- 대부분의 정적 분석은 sound를 우선하고 complete는 포기합니다 (false negative보다 false positive가 낫습니다)

---

## Slide 27: May vs Must 분석 — 건전성 (Live Variables)

> - Live variables = {x}
>   - Set of possible behavior: any execution that does not require any variable other than x to be live
>     - Can have false positives (some such executions may be actually impossible)
>   - Set of impossible behavior: any execution that requires some variable other than x to be live
>     - No false negatives (such executions are indeed impossible)

### 개념 설명
활성 변수 분석의 건전성을 이해해 봅시다.

분석 결과: "오직 x만 활성이다" ({x})
- 실제로는 x와 y가 모두 활성일 수도 있어요 (false positive 가능)
- 하지만 x만 활성이라고 판단하면, "y는 활성이 아니다"라는 결론을 내릴 수 있어요
- 이 경우 "y는 나중에 사용되지 않는다"는 결론은 **절대 틀리지 않아요** (no false negatives)

### 상세한 예시
만약 분석이 "x만 활성"이라고 하면:
- 실제로 y도 나중에 사용될 수 있어요 (분석은 보수적)
- 하지만 "y는 사용 안 됨"이라고 결론 내려도 안전해요 (레지스터 재사용 가능)

---

## Slide 28: May vs Must 분석 — 건전성 (Available Expressions)

> - Available expressions = {x + y}
>   - Set of possible behavior: any execution that has already computed x + y
>     - Can have false positives (some such executions may be actually impossible)
>   - Set of impossible behavior: any execution that has not computed x + y
>     - No false negatives (such executions are indeed impossible)

### 개념 설명
가용 표현식 분석의 건전성:

분석 결과: "{x + y}가 가용하다"
- 실제로는 특정 경로에서 x + y가 계산되지 않았을 수도 있어요 (false positive 가능)
- 하지만 "x + y는 가용하다"고 판단하면, 그 값을 재사용할 수 있어요
- 최악의 경우 불필요한 계산을 하지 않는 것이므로 안전해요 (no false negatives)

### 상세한 예시
만약 분석이 "x + y는 가용"이라고 하면:
- 일부 경로에서는 실제로 계산되지 않았을 수도 있어요
- 하지만 "x + y는 계산되지 않았다"고 결론 내려도 안전해요 (최악의 경우 불필요한 계산만 추가)

---

## Slide 29: 데이터플로우 분석 분류 (Classification)

> |       | Forward | Backward |
> |-------|---------|----------|
> | May   | Reaching definition analysis | Live variable analysis |
> | Must  | Available expression analysis | Very busy expression analysis |

### 개념 설명
이제까지 배운 네 가지 분석을 2×2 표로 분류할 수 있어요:

**Forward + May**: 도달하는 정의 분석
- 과거의 정의들이 현재까지 도달했는가? (may)
- 프로그램 시작부터 끝까지 진행 (forward)

**Forward + Must**: 가용 표현식 분석
- 이 식이 반드시 이미 계산되었는가? (must)
- 프로그램 시작부터 끝까지 진행 (forward)

**Backward + May**: 활성 변수 분석
- 이 변수가 나중에 사용될 수도 있는가? (may)
- 프로그램 끝부터 시작으로 역진행 (backward)

**Backward + Must**: 매우 바쁜 표현식 분석
- 이 식이 반드시 앞으로 계산될 것인가? (must)
- 프로그램 끝부터 시작으로 역진행 (backward)

---

## Slide 30: 예시 — 초기화된 변수 분석 (Initialized Variable Analysis)

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
초기화된 변수 분석(initialized variable analysis)은 새로운 예시입니다. 이는 어떤 변수가 확실하게 초기화되었는지를 파악하는 분석이에요.

### 상세한 예시
코드를 분석하면:
- if 분기: x와 y가 모두 초기화됨
- else 분기: y만 초기화됨
- merge point: 양쪽 분기를 거친 후에는?
  - y는 양쪽 분기에서 모두 초기화되므로 확실히 초기화됨
  - x는 if 분기에서만 초기화되므로 불확실함
- z = y + x: y는 안전하지만 x는 초기화되지 않았을 수 있음!

---

## Slide 31: 예시 — 초기화된 변수 분석 (계속)

> - We want to know whether a certain variable is definitely initialized at a program point
>   - Must analysis — State = (P(Var), ⊇)
> - Initialization is a property of past
>   - Forward analysis — JOIN(v) = ⊔_{u∈pred(v)} ⟦u⟧
> - x=e: ⟦v⟧ = JOIN(v) ∪ {x}
> - if x: ⟦v⟧ = JOIN(v)
> - entry: ⟦v⟧ = ∅
> - return: ⟦v⟧ = JOIN(v)

### 개념 설명
초기화된 변수 분석의 특성:
- **Must 분석**: 변수가 "반드시" 초기화되었는지를 묻습니다
- **Forward 분석**: 과거의 할당들을 이용해서 판단합니다
- Lattice: Reverse power set (⊇) — 더 적은 변수가 초기화되면 더 정확한 정보

### 상세한 예시
제약 규칙:
- x=e: 할당하면 x가 초기화됨 (JOIN(v) ∪ {x})
- if x: 조건 검사는 초기화 상태를 바꾸지 않음
- entry: 프로그램 시작에는 아무것도 초기화되지 않음 (∅)
- return: 반환 시점의 초기화 상태는 그대로

### 배경 지식
이는 Slide 29의 표에서 "Forward + Must"에 해당하는 분석의 또 다른 예시입니다.

---

## Slide 32: 전달 함수 (Transfer Functions)

> - All constraint functions are of the form ⟦v⟧ = t_v(JOIN(v)) where t_v : L → L

### 개념 설명
모든 제약 규칙을 다음과 같은 형태로 표현할 수 있어요:

⟦v⟧ = t_v(JOIN(v))

여기서 t_v는 "전달 함수(transfer function)"라고 불립니다. 이는:
- 입력: JOIN(v) (이전 노드들의 정보)
- 출력: 현재 노드에서의 상태 ⟦v⟧
- 함수: 노드 v의 특성에 따라 정의됩니다

### 상세한 예시
활성 변수 분석의 전달 함수들:
```
x=e: t_{x=e}(s) = s \ {x} ∪ vars(e)
if x: t_{if x}(s) = s ∪ {x}
entry: t_{entry}(s) = s
return: t_{return}(s) = s
```

각 전달 함수는 CFG 노드의 의미를 정확히 반영해요.

---

## Slide 33: 전달 함수 (계속)

> - t_v is called a *transfer function* for the CFG node
>   - Forward analysis: input to the transfer function represents the state immediately before the node, and the output represents the state immediately after the node
>   - Backward analysis: input to the transfer function represents the state immediately after the node, and the output represents the state immediately before the node

### 개념 설명
전달 함수의 입출력이 포워드/백워드 분석에서 다르게 해석돼요:

**포워드 분석**:
- 입력: 노드 "앞의" 상태
- 출력: 노드 "뒤의" 상태
- 예: 가용 표현식 (노드를 지나가면서 새로운 식이 가용해짐)

**백워드 분석**:
- 입력: 노드 "뒤의" 상태
- 출력: 노드 "앞의" 상태
- 예: 활성 변수 (앞으로 필요한 변수를 역으로 추적)

### 상세한 예시
활성 변수 분석에서:
```
t_{x=e}(s) = s \ {x} ∪ vars(e)
```
- 입력 s: 이 노드 뒤에서 활성인 변수들
- 출력: 이 노드 앞에서 활성이어야 하는 변수들
  - x는 할당되므로 뒤에서 활성이어도 제거됨
  - 하지만 e의 변수들은 먼저 읽혀야 하므로 추가됨

---

## Slide 34: 전달 함수 — SimpleWorkListAlgorithm의 중복 (Redundancy)

> - In SimpleWorkListAlgorithm, JOIN(v) = ⊔⟦u⟧ is computed in each iteration
> - However, ⟦u⟧ often has not changed since last iteration, so much of the computation is redundant
> - We can use transfer functions to avoid redundancy
> - Now, xᵢ = ⟦vᵢ⟧ is the state *before* vᵢ in forward analyses, and the state *after* vᵢ in backward analyses

### 개념 설명
SimpleWorkListAlgorithm은 매 반복마다 JOIN(v)를 다시 계산해요. 하지만 이전의 ⟦u⟧값이 변하지 않았으면, JOIN도 변하지 않을 가능성이 높아요. 이는 불필요한 계산을 낭비하는 거예요.

**해결책**: 전달 함수를 이용해서 증분적으로 정보를 전파하기!

### 배경 지식
이제부터는 xᵢ = ⟦vᵢ⟧이 다음을 의미합니다:
- 포워드 분석: 노드 vᵢ "앞의" 상태
- 백워드 분석: 노드 vᵢ "뒤의" 상태

이렇게 하면 전달 함수를 일관되게 적용할 수 있어요!

---

## Slide 35: PropagationWorkListAlgorithm

> ```
> PropagationWorkListAlgorithm(t₁, ..., tₙ):
>   x ← ⊥
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

### 개념 설명
PropagationWorkListAlgorithm은 SimpleWorkListAlgorithm의 최적화 버전이에요.

**알고리즘 흐름**:
1. 모든 변수를 ⊥로 초기화
2. 모든 노드를 worklist에 추가
3. 반복:
   - Worklist에서 노드 vᵢ를 꺼냄
   - 전달 함수를 적용: y = t_{vᵢ}(xᵢ)
   - 의존하는 노드들을 업데이트
   - 상태가 변하면 worklist에 다시 추가

**핵심 아이디어**: 전달 함수의 결과를 증분적으로 전파해서 JOIN 재계산을 피함!

### 상세한 예시
```
노드 v1에서: y = t_v1(x1) 계산
↓
v1의 의존 노드 v3으로: x3 = x3 ⊔ y (증분 업데이트)
↓
x3이 변했으면 v3을 worklist에 추가
```

---

## Slide 36: PropagationWorkListAlgorithm — 직관 (Intuition)

> - This computes the same result
> - Intuition:
>   - SimpleWorkListAlgorithm computes x₃ = t₁(x₁) ⊔ t₂(x₂)
>   - PropagationWorkListAlgorithm computes x₃ = x₃ ⊔ t₁(x₁) and x₃ = x₃ ⊔ t₂(x₂), which is x₃ = x₃ ⊔ t₁(x₁) ⊔ t₂(x₂)
>   - If f is monotone and g(x) = f(x) ⊔ x, then lfp(g) = lfp(f)

### 개념 설명
두 알고리즘이 같은 결과를 계산하는 이유를 이해해 봅시다.

**SimpleWorkListAlgorithm**:
- 매번: x₃ = t₁(x₁) ⊔ t₂(x₂) 계산 (JOIN 전체 재계산)

**PropagationWorkListAlgorithm**:
- 처음: x₃ = x₃ ⊔ t₁(x₁) (v1의 결과 추가)
- 다음: x₃ = x₃ ⊔ t₂(x₂) (v2의 결과 추가)
- 결과: x₃ = x₃ ⊔ t₁(x₁) ⊔ t₂(x₂)

수학적으로, f가 monotone이고 g(x) = f(x) ⊔ x라면:
- lfp(g) = lfp(f)
- 즉, 증분적 업데이트와 전체 재계산이 같은 고정점에 도달해요!

### 배경 지식
이는 Tarski의 부동점 정리의 응용입니다. Monotone 함수는 고정점에서의 계산 순서에 상관없이 같은 결과를 얻습니다.

---

## Slide 37: 요약 (Summary)

> - Live variable analysis determines which variables may be needed in the future (backward, may analysis)
> - Available expression analysis determines which expressions have already been computed (forward, must analysis)
> - Very busy expression analysis determines which expressions will definitely be evaluated (backward, must analysis)
> - Reaching definition analysis determines which assignments may define current variable values (forward, may analysis)
> - Dataflow analyses are classified along two axes: forward/backward and may/must
> - PropagationWorkListAlgorithm avoids redundant JOIN recomputation by propagating transfer function results incrementally

### 개념 설명
이번 강의에서 배운 내용을 정리해요:

**활성 변수 분석** (Backward + May):
- 변수가 앞으로 사용될 수도 있는가?
- 레지스터 할당에 유용

**가용 표현식 분석** (Forward + Must):
- 식이 이미 계산되었는가?
- 중복 계산 제거에 유용

**매우 바쁜 표현식 분석** (Backward + Must):
- 식이 반드시 앞으로 계산될 것인가?
- 코드 호이스팅에 유용

**도달하는 정의 분석** (Forward + May):
- 할당이 현재 값을 정의할 수 있는가?
- def-use graph 구성에 유용

### 전체적 맥락
데이터플로우 분석은 두 축으로 분류됩니다:
- **Forward vs Backward**: 정보의 전파 방향 (과거 vs 미래)
- **May vs Must**: 정보의 신뢰성 (가능성 vs 확실성)

PropagationWorkListAlgorithm은 이러한 분석들을 효율적으로 계산하기 위한 알고리즘이에요. 전달 함수의 결과를 증분적으로 전파함으로써 불필요한 JOIN 재계산을 피합니다.

### 배경 지식
이번 강의는 정적 분석의 핵심 기법 중 하나인 데이터플로우 분석을 깊이 있게 다루었어요. 이 기법들은:
- 컴파일러 최적화
- 프로그램 검증
- 보안 분석 (정보 유출 탐지 등)
등에 광범위하게 사용됩니다.
