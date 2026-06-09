# Control Flow Analysis - 강의 슬라이드 해설

CSE552 Program Analysis — Lecture 11
강사: Jaemin Hong

> **이 파일을 읽는 법**
> 각 슬라이드는 `원문 내용`(영어 슬라이드의 충실한 인용) → `번역`(한국어 직역) → `해설`(개념 설명·배경 지식·맥락) → 필요한 경우 `각주`와 `슬라이드 연결`의 순서로 구성됩니다. 각주(¹²³)는 신입생이 모를 수 있는 용어나, 더 깊이 들어가고 싶은 사람을 위한 보충입니다. 누락이나 왜곡 없이 원문의 모든 정보를 담되, 처음 보는 사람도 따라올 수 있도록 풀어 썼습니다.

---

## 강의 11 전체 조감도 (먼저 큰 그림부터)

지금까지(강의 1~10)의 흐름을 한 줄로 요약하면 이렇습니다. **추상화(추상 영역·갈루아 연결, 강의 5~6) → 데이터플로우 분석(강의 7~8) → 위드닝/내로잉(강의 9) → 절차간 분석(강의 10)**. 그런데 강의 10의 절차간 분석에는 숨은 전제가 하나 있었습니다. **"각 호출 지점(call site)에서 어떤 함수가 불리는지 이미 알고 있다"**는 것이죠. C의 평범한 `foo()` 호출처럼 함수 이름이 코드에 박혀 있으면 이 전제는 공짜로 성립합니다.

하지만 **함수가 값(value)으로 취급되는 언어**(함수 포인터를 쓰는 C, 일급 함수가 있는 Rust·OCaml·JavaScript, 가상 메서드가 있는 Java·C++)에서는 "이 호출 지점에서 실제로 어떤 함수가 불릴까?"가 더 이상 자명하지 않습니다. 이 질문에 **안전하게(=실제로 불릴 수 있는 함수를 하나도 빠뜨리지 않고) 답하는 것**이 바로 **제어 흐름 분석(Control Flow Analysis, CFA)**이고, 그 산출물이 **호출 그래프(call graph)**입니다. 즉 강의 11은 강의 10이 당연하게 가정했던 "호출 그래프"를 **어떻게 만들어 내는가**를 채워 넣는 강의입니다.

이 강의의 뼈대는 세 부분입니다:
1. **제약식 규칙(Constraint Rules)** — CFA 문제를 "변수마다 가능한 함수 집합"에 대한 포함관계 제약식으로 번역 (슬라이드 2~7)
2. **3차 알고리즘(Cubic Algorithm)** — 그런 제약식을 O(n³)에 푸는 일반적인 알고리즘 (슬라이드 8~17)
3. **객체지향 언어에서의 제어 흐름** — 가상 메서드 호출(CHA, RTA) (슬라이드 18)

---

## 슬라이드 1: 제목 슬라이드

### 원문 내용

> Control Flow Analysis
> CSE552 Program Analysis — Lecture 11
> Jaemin Hong

### 번역

> 제어 흐름 분석
> CSE552 프로그램 분석 — 강의 11
> 홍재민

### 해설

**개념 설명**

이 강의의 주제는 **제어 흐름 분석(Control Flow Analysis)**입니다. 여기서 "제어 흐름"이라는 단어에 주의가 필요합니다. 강의 7~8의 데이터플로우 분석에서도 "제어 흐름 그래프(CFG)"라는 말이 나왔지만, 그때의 제어 흐름은 **한 함수 내부**에서 문장들이 어떤 순서로 실행되는가(분기·반복)였습니다. 반면 이 강의의 제어 흐름은 **함수와 함수 사이**, 즉 "어느 호출 지점이 어느 함수로 이어지는가"라는 **절차간(interprocedural)** 제어 흐름입니다.¹

**각주**

¹ 용어가 혼동되기 쉽습니다. 정리하면: (a) **CFG (Control Flow Graph)** = 한 함수 안의 기본 블록 사이 흐름(강의 7), (b) **Call Graph (호출 그래프)** = 함수 사이의 호출 관계(이번 강의의 산출물). "Control Flow Analysis"라는 이름은 역사적으로 함수형 언어 커뮤니티에서 "어떤 람다가 어디서 호출되는가"를 분석하던 데서 왔고, 그래서 결과물이 호출 그래프입니다.

---

## 슬라이드 2: Control Flow Analysis (문제 정의)

### 원문 내용

> - When functions are values, it is nontrivial to see which function is being called at each call site
> - Control flow analysis conservatively approximates the interprocedural control flow (i.e., call graph)
> - A call graph shows which functions may be called for each call site
> - Call graphs provide a foundation for subsequently performing interprocedural dataflow analysis

### 번역

> - 함수가 값일 때는, 각 호출 지점에서 어떤 함수가 호출되는지 파악하는 일이 자명하지 않다
> - 제어 흐름 분석은 절차간 제어 흐름(즉, 호출 그래프)을 보수적으로 근사한다
> - 호출 그래프는 각 호출 지점에서 호출될 수 있는 함수들이 무엇인지 보여 준다
> - 호출 그래프는 이후에 수행할 절차간 데이터플로우 분석의 토대를 제공한다

### 해설

**개념 설명**

"**함수가 값(functions are values)**"이라는 표현이 핵심입니다. 함수를 변수에 담고, 인자로 넘기고, 반환할 수 있다는 뜻입니다(일급 함수, first-class function). 이때 `x()`라는 호출을 봐도 `x`에 어떤 함수가 들어 있는지는 코드만 봐서는 즉시 알 수 없습니다 — 프로그램 실행 경로에 따라 `foo`일 수도 `bar`일 수도 있죠.

"**보수적으로 근사한다(conservatively approximates)**"는 정적 분석의 핵심 안전 원칙입니다. 실제로 호출될 수 있는 함수를 **하나도 빠뜨리지 않는** 대신, 실제로는 절대 불리지 않을 함수를 일부 포함해도 괜찮다는 뜻입니다(과근사, over-approximation²). 빠뜨리면 위험하고(분석이 거짓말이 됨), 좀 더 포함하는 건 안전하지만 정밀도가 떨어질 뿐입니다.

"**각 호출 지점에서 호출될 수 있는(may be called)**"의 *may*도 같은 맥락입니다. "반드시 불린다(must)"가 아니라 "불릴 가능성이 있다(may)"는 집합을 구합니다.

마지막 줄이 강의 10과의 연결고리입니다. 절차간 데이터플로우 분석(강의 10)을 하려면 "호출 지점 → 대상 함수" 간선이 그려진 호출 그래프가 **먼저** 있어야 합니다. 그래서 CFA는 절차간 분석의 **전처리(선행 단계)**입니다.

**각주**

² 강의 5~6의 격자 용어로 말하면, 우리는 "가능한 함수들의 집합"이라는 영역에서 **상한(upper bound)**을 구하는 것입니다. 집합이 클수록(함수가 많이 포함될수록) 더 안전하지만 덜 정밀합니다. 가장 안전하지만 쓸모없는 답은 "모든 함수가 모든 호출 지점에서 불릴 수 있다"이고, 우리는 그보다 **가능한 한 작은(least)** 안전한 답을 원합니다(슬라이드 8의 "least solution"으로 이어짐).

**슬라이드 연결**

이 슬라이드가 던진 문제 — "변수에 어떤 함수가 들어갈 수 있는가?" — 를 슬라이드 3에서 **제약식**으로 형식화합니다.

---

## 슬라이드 3: Constraint Rules (제약식 규칙)

### 원문 내용

> For each program variable x, we introduce a constraint variable [x] denoting the set of possible functions that x may refer to
>
> - `fn x(...) { ... }`: x ∈ [x]
> - `x = y`: [y] ⊆ [x]
> - `x = y(z1, ..., zn)`: ∀f. f ∈ [y] ⇒ ([z1] ⊆ [a_f^1] ∧ ··· ∧ [zn] ⊆ [a_f^n] ∧ [RET_f] ⊆ [x])

### 번역

> 각 프로그램 변수 x에 대해, x가 가리킬 수 있는 함수들의 집합을 나타내는 제약 변수 [x]를 도입한다
>
> - `fn x(...) { ... }` (함수 정의): x ∈ [x]
> - `x = y` (대입): [y] ⊆ [x]
> - `x = y(z1, ..., zn)` (호출): 모든 함수 f에 대해, f ∈ [y]이면 ([z1] ⊆ [a_f^1] ∧ ··· ∧ [zn] ⊆ [a_f^n] ∧ [RET_f] ⊆ [x])

### 해설

**개념 설명 — 기호부터 천천히**

- `[x]` (대괄호로 감싼 변수): "프로그램 변수 `x`가 가리킬 수 있는 **함수들의 집합**"을 담는 그릇입니다. 우리가 구하려는 미지수예요. 예를 들어 분석 결과 `[x] = {foo, bar}`이면 "실행 중 `x`는 `foo` 또는 `bar`를 가리킬 수 있다"는 뜻입니다.
- `∈` (원소), `⊆` (부분집합): 집합 기호 그대로입니다.
- `a_f^i`: 함수 `f`의 **i번째 매개변수(parameter)**. (예: `f`의 첫째 인자 자리)
- `RET_f`: 함수 `f`의 **반환값(return value)**을 담는 가상의 변수.

**규칙 하나씩**

1. **함수 정의 `fn x(...) {...}` ⟹ `x ∈ [x]`**
   함수 `x`를 정의하면, 그 이름 `x` 자체는 자기 자신(함수 `x`)을 가리킵니다. 즉 함수의 이름은 그 함수를 값으로 담고 있는 변수처럼 취급됩니다. 이것이 모든 함수 집합에 "씨앗(token)"을 뿌리는 출발점입니다.

2. **대입 `x = y` ⟹ `[y] ⊆ [x]`**
   `y`가 가리킬 수 있는 함수는 모두 `x`도 가리킬 수 있게 됩니다. **방향이 중요합니다**: `y`의 가능성이 `x`로 **흘러 들어갑니다**(`[y] ⊆ [x]`, 반대 아님). 대입의 화살표 방향(오른쪽 → 왼쪽)과 같다고 기억하세요.

3. **호출 `x = y(z1,...,zn)` ⟹ 조건부 제약**
   이게 핵심이자 가장 복잡한 규칙입니다. `y`가 함수일 텐데 어떤 함수인지 모르므로, **"만약 `y`가 함수 `f`를 가리킬 수 있다면(`f ∈ [y]`)"**이라는 가정 아래, 그 경우 일어나는 일을 적습니다:
   - `[z_i] ⊆ [a_f^i]`: 실인자 `z_i`의 가능 함수들이 `f`의 i번째 **매개변수**로 흘러 들어간다 (인자 전달).
   - `[RET_f] ⊆ [x]`: `f`의 반환값이 호출 결과 `x`로 흘러 나온다 (반환값 전달).

   즉 "`y`가 `f`일 수도 있다"가 **참이 될 때에만** 인자·반환 흐름이 활성화됩니다. 이 **조건부(conditional) 구조** `f∈[y] ⇒ ...`가 슬라이드 8의 세 번째 제약식 형태 `t∈x ⇒ y⊆z`와 정확히 대응됩니다(뒤에서 이 구조 덕분에 효율적으로 풀립니다).

**왜 이렇게 하나? (배경 지식)**

이것은 **제약 기반 분석(constraint-based analysis)**의 전형입니다. 강의 3~4의 타입 분석에서 이미 "코드 조각마다 제약식을 만들고, 그 제약식들을 한꺼번에 만족하는 해를 구한다"는 패턴을 봤습니다. CFA도 똑같은 철학입니다: 코드를 훑으며 위 규칙대로 제약식을 쏟아낸 뒤, 그 모두를 만족하는 **가장 작은(least)** 집합 할당을 찾습니다.

**슬라이드 연결**

슬라이드 4~5는 규칙 1·2만 쓰는 간단한 예, 슬라이드 6~7은 규칙 3(호출)까지 쓰는 예입니다. 규칙 3의 조건부 구조는 슬라이드 8에서 "3차 알고리즘이 푸는 일반 문제"로 추상화됩니다.

---

## 슬라이드 4: Example 1 — Code and Constraints

### 원문 내용

> ```
> fn foo() { ... }
> fn bar() { ... }
>
> if ... {
>   x = foo;
> } else {
>   x = bar;
> }
> x();
> ```
> Constraints:
> - `fn foo`: foo ∈ [foo]
> - `fn bar`: bar ∈ [bar]
> - `x = foo`: [foo] ⊆ [x]
> - `x = bar`: [bar] ⊆ [x]

### 번역

> 코드: `foo`와 `bar` 두 함수를 정의하고, 조건에 따라 `x`에 `foo` 또는 `bar`를 대입한 뒤 `x()`를 호출한다.
> 제약식:
> - `fn foo` 정의로부터: foo ∈ [foo]
> - `fn bar` 정의로부터: bar ∈ [bar]
> - `x = foo` 로부터: [foo] ⊆ [x]
> - `x = bar` 로부터: [bar] ⊆ [x]

### 해설

**개념 설명**

슬라이드 3의 규칙을 코드에 기계적으로 적용한 예입니다.
- `fn foo {...}`, `fn bar {...}` → 규칙 1로 `foo ∈ [foo]`, `bar ∈ [bar]`.
- `x = foo` → 규칙 2로 `[foo] ⊆ [x]`.
- `x = bar` → 규칙 2로 `[bar] ⊆ [x]`.

여기서 `if...else`의 조건은 **무시**됩니다. 정적 분석은 어느 분기가 실제로 실행될지 모르므로, **양쪽 다 일어날 수 있다**고 보수적으로 가정합니다(그래서 `[x]`는 두 대입을 모두 받습니다). 마지막 `x()`는 인자도 반환 사용도 없어 추가 제약을 만들지 않지만, "`x`가 가리키는 함수들이 곧 이 호출 지점의 대상"이 됩니다 — 즉 `[x]`가 이 호출 지점의 호출 그래프 간선을 결정합니다.

**슬라이드 연결**

이 제약식들의 해가 슬라이드 5입니다.

---

## 슬라이드 5: Example 1 — Solution

### 원문 내용

> [foo] = {foo}
> [bar] = {bar}
> [x] = {foo, bar}

### 번역

> [foo] = {foo}, [bar] = {bar}, [x] = {foo, bar}

### 해설

**개념 설명**

제약식을 만족하는 **가장 작은** 해입니다.
- `foo ∈ [foo]` 때문에 `[foo]`는 최소한 `{foo}`. 다른 제약이 `[foo]`를 키우지 않으므로 `[foo]={foo}`.
- 마찬가지로 `[bar]={bar}`.
- `[foo]⊆[x]`와 `[bar]⊆[x]` 때문에 `[x] ⊇ {foo,bar}`. 더 키울 제약이 없으니 `[x]={foo,bar}`.

**해석**: 호출 지점 `x()`는 **`foo` 또는 `bar`로 갈 수 있다.** 따라서 호출 그래프에는 이 호출 지점에서 `foo`로 가는 간선과 `bar`로 가는 간선이 모두 그려집니다. 이것이 우리가 원한 "보수적 근사"입니다 — 실제 실행에서는 둘 중 하나만 불리지만, 분석은 둘 다 가능하다고 봅니다.

---

## 슬라이드 6: Example 2 — Code and Constraints

### 원문 내용

> ```
> fn foo(x) {
>   RET_foo = x;
>   return;
> }
> fn bar(y) {
>   RET_bar = y;
>   return;
> }
>
> z = foo;
> w = z(bar);
> ```
> Constraints:
> - `fn foo`: foo ∈ [foo]
> - `RET_foo = x`: [x] ⊆ [RET_foo]
> - `fn bar`: bar ∈ [bar]
> - `RET_bar = y`: [y] ⊆ [RET_bar]
> - `z = foo`: [foo] ⊆ [z]
> - `w = z(bar)`:
>   foo ∈ [z] ⇒ ([bar] ⊆ [x] ∧ [RET_foo] ⊆ [w])
>   ∧ bar ∈ [z] ⇒ ([bar] ⊆ [y] ∧ [RET_bar] ⊆ [w])

### 번역

> 코드: `foo(x)`는 인자 `x`를 그대로 반환하고, `bar(y)`도 인자 `y`를 그대로 반환한다. 그 뒤 `z = foo`로 함수를 변수에 담고, `w = z(bar)`로 `z`(=foo)를 호출하며 인자로 **함수 `bar`**를 넘긴다.
> 제약식:
> - 함수 정의: foo ∈ [foo], bar ∈ [bar]
> - 반환 대입: [x] ⊆ [RET_foo], [y] ⊆ [RET_bar]
> - `z = foo`: [foo] ⊆ [z]
> - 호출 `w = z(bar)`: "z가 foo이면 (bar가 foo의 매개변수 x로 들어가고, foo의 반환이 w로 나온다)" 그리고 "z가 bar이면 (bar가 bar의 매개변수 y로 들어가고, bar의 반환이 w로 나온다)"

### 해설

**개념 설명 — 이 예제가 노리는 함정**

여기서는 **함수가 인자로 전달**됩니다(`z(bar)`에서 `bar`는 호출이 아니라 *값으로서의 함수*). 그래서 매개변수 `x`, `y`에도 "함수 집합"이 흐릅니다.

호출 `w = z(bar)`를 규칙 3에 대입해 봅시다. `y`(피호출자 자리) = `z`, 실인자는 `bar` 하나, 결과는 `w`. 그런데 `z`가 어떤 함수인지 모르므로, 후보 함수 `foo`와 `bar` 각각에 대해 조건부 제약을 만듭니다:
- **만약 `foo ∈ [z]`** (z가 foo일 수 있다면): 실인자 `bar`가 `foo`의 매개변수 `x`로 → `[bar] ⊆ [x]`; `foo`의 반환이 `w`로 → `[RET_foo] ⊆ [w]`.
- **만약 `bar ∈ [z]`** (z가 bar일 수 있다면): `[bar] ⊆ [y]`, `[RET_bar] ⊆ [w]`.

이 **조건부**가 핵심입니다. 실제로는 `[z]={foo}`라서 첫 번째 조건만 발동되고 두 번째는 잠든 채로 남습니다. 어느 조건이 깨어날지는 `[z]`의 해가 정해져야 알 수 있으므로, 제약을 푸는 과정에서 **점진적으로** 활성화됩니다(슬라이드 10~11의 `cond` 자료구조가 바로 이 "잠든 조건부 제약"을 저장합니다).

**슬라이드 연결**

이 조건부 제약 구조가 어떻게 효율적으로 풀리는지가 슬라이드 8~15의 주제이고, 그 해가 슬라이드 7입니다.

---

## 슬라이드 7: Example 2 — Solution

### 원문 내용

> [foo] = {foo}
> [bar] = {bar}
> [x] = {bar}
> [RET_foo] = {bar}
> [z] = {foo}
> [w] = {bar}

### 번역

> [foo]={foo}, [bar]={bar}, [x]={bar}, [RET_foo]={bar}, [z]={foo}, [w]={bar}

### 해설

**단계별 유도 (직접 따라가 보기)**

1. 함수 정의: `[foo]={foo}`, `[bar]={bar}`.
2. `z = foo`: `[foo]⊆[z]` → `[z] ⊇ {foo}`. 따라서 `[z]={foo}`.
3. 이제 `foo ∈ [z]`가 **참**이 되었으므로 첫 번째 조건부가 깨어납니다: `[bar]⊆[x]` → `[x] ⊇ {bar}` → `[x]={bar}`. 그리고 `[RET_foo]⊆[w]`도 활성화(아직 `[RET_foo]`가 비어 다음 단계에서 채워짐).
4. `RET_foo = x`: `[x]⊆[RET_foo]` → `[RET_foo] ⊇ {bar}` → `[RET_foo]={bar}`.
5. 다시 3번에서 활성화된 `[RET_foo]⊆[w]`를 통해 `[w] ⊇ {bar}` → `[w]={bar}`.
6. `bar ∈ [z]`는 끝까지 **거짓**(`[z]={foo}`이므로)이라, `[y]`와 `[RET_bar]`는 비어 있고 두 번째 조건부는 발동되지 않습니다. (그래서 `[y]`, `[RET_bar]`는 ∅.)

**해석**: 호출 `w = z(bar)`는 `foo`로만 이어집니다(`[z]={foo}`). 결과적으로 `w`에는 `bar`라는 함수가 담깁니다(foo가 인자 bar를 그대로 반환했으므로). 분석이 "함수가 인자로 흘러가서 반환으로 나오는" 데이터 흐름을 정확히 추적했습니다.

이처럼 손으로 푸는 과정은 직관적이지만, 변수와 함수가 많아지면 **체계적이고 효율적인 알고리즘**이 필요합니다. 그것이 슬라이드 8부터 등장하는 **3차 알고리즘**입니다.

---

## 슬라이드 8: Cubic Algorithm (3차 알고리즘 — 일반 문제 정의)

### 원문 내용

> - The constraints for control flow analysis are an instance of a general class that can be solved in cubic time
> - Many problems fall into this category
> - A finite set of tokens T = {t1, ..., tk}
> - A finite set of variables V = {x1, ..., xn} whose values are sets of tokens
> - Each constraint has one of the following forms:
>   - t ∈ x
>   - x ⊆ y
>   - t ∈ x ⇒ y ⊆ z
> - Goal: for a given collection of constraints, produce the least solution

### 번역

> - 제어 흐름 분석의 제약식들은 **3차 시간(cubic time)**에 풀 수 있는 일반적인 문제 부류의 한 사례다
> - 많은 문제들이 이 부류에 속한다
> - 유한한 **토큰** 집합 T = {t1, ..., tk}
> - 값이 토큰들의 집합인 유한한 **변수** 집합 V = {x1, ..., xn}
> - 각 제약식은 다음 세 형태 중 하나다:
>   - t ∈ x (토큰 t가 변수 x에 속한다)
>   - x ⊆ y (x의 모든 토큰이 y에 속한다)
>   - t ∈ x ⇒ y ⊆ z (t가 x에 있으면, y가 z에 포함된다)
> - 목표: 주어진 제약식 모음에 대해 **최소 해(least solution)**를 구한다

### 해설

**개념 설명 — 추상화의 힘**

여기서 멋진 전환이 일어납니다. CFA의 구체적 개념(함수, 매개변수, 반환값)을 모두 지워 버리고, **추상적인 일반 문제**로 바꿉니다:
- **토큰(token)** `t` = "집합에 들어갈 수 있는 원소". CFA에서는 *각각의 함수*가 하나의 토큰입니다(`foo`, `bar` 등).
- **변수(variable)** `x` = "토큰들의 집합을 담는 그릇". CFA의 `[x]`, `[foo]` 등이 여기 해당.
- 세 가지 제약식 형태:
  - `t ∈ x` ← CFA의 규칙 1 (`foo ∈ [foo]`)에 대응
  - `x ⊆ y` ← CFA의 규칙 2 (`[y] ⊆ [x]`)에 대응
  - `t ∈ x ⇒ y ⊆ z` ← CFA의 규칙 3의 조건부 (`f∈[y] ⇒ [z_i]⊆[a]`)에 대응

세 형태만 있으면 CFA뿐 아니라 **포인터 분석(강의 14~15에서 만남), 타입 추론** 등 수많은 분석이 표현됩니다. "Many problems fall into this category"가 그 말입니다. 그래서 이 한 알고리즘을 익혀 두면 두고두고 씁니다.

**"최소 해(least solution)"의 의미**

제약식을 만족하는 집합 할당은 여러 개일 수 있습니다(예: 모든 변수에 모든 토큰을 다 넣어도 `⊆` 제약은 만족됨). 그중 **각 변수의 집합이 가장 작은** 해를 원합니다. 이것이 격자 이론(강의 5~6)의 **최소 고정점(least fixpoint)** 개념과 같습니다 — 안전하면서도 가장 정밀한 답.³

**"3차 시간(cubic, O(n³))"**

변수·토큰 개수를 n이라 할 때, 이 알고리즘은 O(n³)에 끝납니다(슬라이드 16에서 증명). 다항 시간이라 실용적입니다.

**각주**

³ 왜 최소 해가 존재하고 유일한가? 제약식들이 모두 "집합을 키우는 방향(monotone)"이고, 집합 포함관계는 완비 격자(complete lattice)를 이루므로, **Knaster–Tarski 고정점 정리**(강의 6)에 의해 최소 고정점이 유일하게 존재합니다. 알고리즘은 빈 집합에서 시작해 제약을 만족할 때까지 토큰을 더하기만 하므로, 자연히 최소 해에 도달합니다.

**슬라이드 연결**

슬라이드 9~11은 이 일반 문제를 푸는 자료구조와 알고리즘, 슬라이드 12~15는 실행 예제, 슬라이드 16은 복잡도 분석입니다.

---

## 슬라이드 9: Data Structures (자료구조)

### 원문 내용

> The algorithm maintains a directed graph where
> - Nodes correspond to constraint variables
> - Edges reflect inclusion constraints
>
> For each constraint variable x,
> - x.sol ⊆ T holds the solution for x
> - x.succ ⊆ V is the set of successors of x (i.e., the edges of the graph)
> - x.cond(t) ⊆ V × V represents a set of conditional constraints for x and t
>
> We additionally have
> - W ⊆ T × V is a worklist
>
> All sets are initially empty

### 번역

> 알고리즘은 다음과 같은 **방향 그래프**를 유지한다:
> - 노드 = 제약 변수
> - 간선 = 포함(inclusion) 제약
>
> 각 제약 변수 x에 대해:
> - `x.sol ⊆ T`: x의 현재 해(담고 있는 토큰 집합)
> - `x.succ ⊆ V`: x의 후속 노드 집합(즉 그래프의 간선들)
> - `x.cond(t) ⊆ V × V`: 토큰 t와 변수 x에 대한 조건부 제약들의 집합
>
> 추가로:
> - `W ⊆ T × V`: 작업 목록(worklist)
>
> 모든 집합은 처음에 비어 있다

### 해설

**개념 설명 — 각 자료구조가 무엇을 위한 것인가**

- **`x.sol`**: 지금까지 알아낸 "x에 들어가는 토큰들". 알고리즘이 끝나면 이게 곧 `[x]`의 답입니다.
- **`x.succ` (후속자, successors)**: `x ⊆ y` 제약을 그래프 간선 `x → y`로 표현합니다. "x에 토큰이 들어오면 y에도 흘려보내야 한다"는 통로입니다. `x.succ`는 x에서 나가는 간선들의 끝점 집합.
- **`x.cond(t)`**: 아직 **잠들어 있는 조건부 제약** `t∈x ⇒ y⊆z`를 저장합니다. 핵심은 "`t`가 아직 `x.sol`에 없어서 발동 못 한 제약"을 `(y,z)` 쌍으로 보관해 두는 것입니다. 나중에 `t`가 `x`에 들어오는 순간 이 잠든 제약을 깨워 간선 `y→z`를 추가합니다(슬라이드 10의 Propagate). 이것이 슬라이드 6의 "조건부 제약"을 효율적으로 다루는 비결입니다.
- **`W` (worklist, 작업 목록)**: "새로 발견된 `(토큰 t, 변수 x)` 쌍" — 즉 "방금 t가 x에 추가됐으니, 이 변화를 후속 노드들에 전파해야 함"을 적어 두는 대기열. 데이터플로우 분석(강의 7~8)의 워크리스트 알고리즘과 같은 아이디어입니다.⁴

**"모든 집합은 처음에 비어 있다"** — 최소 해를 구하므로 ∅에서 출발해 **더하기만** 합니다(절대 빼지 않음). 이 단조 증가(monotone) 성질이 종료와 최소성을 보장합니다.

**각주**

⁴ 강의 7~8의 데이터플로우 워크리스트는 "값이 바뀐 노드"를 큐에 넣고 그 후속을 재방문했습니다. 여기서는 "토큰이 추가된 (t,x) 쌍"을 큐에 넣고 그 변화를 전파합니다. 본질적으로 같은 **고정점 반복(fixpoint iteration)**이며, 워크리스트는 "바뀐 것만 다시 보자"는 효율화 장치입니다.

**슬라이드 연결**

이 자료구조 위에서 동작하는 연산이 슬라이드 10의 세 보조 함수입니다.

---

## 슬라이드 10: Helper Functions (보조 함수)

### 원문 내용

> We introduce helper functions for adding tokens and edges and propagating tokens
>
> **AddToken(t, x):**
> ```
> if t ∉ x.sol:
>     x.sol.add(t)
>     W.add(t, x)
> ```
> **AddEdge(x, y):**
> ```
> if x ≠ y ∧ y ∉ x.succ:
>     x.succ.add(y)
>     for t ∈ x.sol:
>         AddToken(t, y)
> ```
> **Propagate():**
> ```
> while W ≠ ∅:
>     (t, x) ← W.removeOne()
>     for (y, z) ∈ x.cond(t):
>         AddEdge(y, z)
>     for y ∈ x.succ:
>         AddToken(t, y)
> ```

### 번역

> 토큰과 간선을 추가하고 토큰을 전파하기 위한 보조 함수들을 도입한다.
>
> **AddToken(t, x)** — 토큰 t를 변수 x에 추가:
> 만약 t가 아직 x.sol에 없으면, x.sol에 t를 넣고, 작업 목록 W에 (t,x)를 추가한다. (이미 있으면 아무것도 안 함)
>
> **AddEdge(x, y)** — 포함 간선 x→y 추가:
> 만약 x≠y이고 y가 아직 x.succ에 없으면, x.succ에 y를 추가하고, **x가 이미 갖고 있는 모든 토큰 t를 y로도 흘려보낸다**(AddToken(t,y)).
>
> **Propagate()** — 작업 목록을 비울 때까지 전파:
> W가 빌 때까지: (t,x)를 하나 꺼내, (1) x.cond(t)에 잠들어 있던 조건부 제약 (y,z)마다 간선 y→z를 추가하고, (2) x의 모든 후속 y에 토큰 t를 전파한다.

### 해설

**개념 설명 — 세 함수의 협동**

이 세 함수가 알고리즘의 엔진입니다. 각각의 "if" 조건이 **중복 작업을 막아** 종료와 효율을 보장합니다.

- **AddToken**: "토큰을 집합에 넣기"의 단일 창구. **이미 있으면 무시**(중복 방지). 새로 넣었을 때만 `W`에 기록해 "이 변화를 전파해야 함"을 알립니다.
- **AddEdge**: "`x ⊆ y` 통로 만들기". `x≠y` 검사는 자기 자신으로의 무의미한 간선을 막고, `y∉x.succ` 검사는 중복 간선을 막습니다. **중요**: 간선을 새로 놓는 순간, x가 *이미* 갖고 있던 토큰들을 즉시 y로 부어 줍니다(`for t∈x.sol: AddToken(t,y)`). 통로를 늦게 뚫었어도 과거의 물이 새 통로로 흐르게 하는 것이죠.
- **Propagate**: 작업 목록을 소비하는 루프. 꺼낸 `(t,x)`(="방금 t가 x에 들어왔다")에 대해 두 가지를 합니다:
  1. **조건부 깨우기**: `x.cond(t)`에 잠든 `(y,z)`들 — 즉 "`t∈x`이면 `y⊆z`"였던 제약들 — 이 이제 조건(`t∈x`)이 충족됐으니 `AddEdge(y,z)`로 발동.
  2. **토큰 전파**: x의 후속 y들에게 t를 흘려보냄(`AddToken(t,y)`).

  AddToken/AddEdge가 다시 W를 키우거나 토큰을 추가할 수 있으므로, 이 루프는 변화가 더 없을 때까지(=고정점) 돕니다.

**왜 종료하는가**

토큰은 추가만 되고(최대 |T|×|V|개 쌍), 간선도 추가만 됩니다(최대 |V|²개). 모두 유한하고 단조 증가하므로 언젠가 W가 비고 멈춥니다.

**슬라이드 연결**

이 보조 함수들을 **어떤 순서로 호출해 제약식을 처리하는가**가 슬라이드 11입니다.

---

## 슬라이드 11: Processing Constraints (제약식 처리)

### 원문 내용

> We process each constraint using the helper functions
>
> **t ∈ x:**
> ```
> AddToken(t, x)
> Propagate()
> ```
> **y ⊆ z:**
> ```
> AddEdge(y, z)
> Propagate()
> ```
> **t ∈ x ⇒ y ⊆ z:**
> ```
> if t ∈ x.sol:
>     AddEdge(y, z)
>     Propagate()
> else:
>     x.cond(t).add(y, z)
> ```

### 번역

> 각 제약식을 보조 함수로 처리한다.
> - **t ∈ x**: AddToken(t,x) 후 Propagate().
> - **y ⊆ z**: AddEdge(y,z) 후 Propagate().
> - **t ∈ x ⇒ y ⊆ z**: 만약 t가 이미 x.sol에 있으면 즉시 AddEdge(y,z) 후 Propagate(); 아직 없으면 이 제약을 x.cond(t)에 저장(잠재워 둠).

### 해설

**개념 설명 — 세 형태를 처리하는 법**

슬라이드 8의 세 제약식 형태를 각각 어떻게 다루는지 보여 줍니다:

1. **`t ∈ x`** (무조건 토큰): 그냥 `t`를 `x`에 넣고 전파.
2. **`y ⊆ z`** (무조건 포함): 간선 `y→z`를 놓고 전파(AddEdge가 기존 토큰도 흘려보냄).
3. **`t ∈ x ⇒ y ⊆ z`** (조건부): **두 갈래**가 핵심입니다.
   - 이미 `t ∈ x.sol`이면(조건이 벌써 참) → 즉시 `AddEdge(y,z)`로 발동.
   - 아직 아니면 → **나중을 위해 `x.cond(t)`에 저장**. 이후 누군가 `AddToken(t,x)`를 호출해 `t`가 `x`에 들어오는 순간, Propagate가 `x.cond(t)`를 훑어 이 잠든 제약을 깨웁니다(슬라이드 10).

이 "지금 발동 vs 나중을 위해 저장"의 분기가 조건부 제약을 **딱 필요한 만큼만** 처리하게 해 줍니다. 이미 모든 제약식을 한 번씩 처리하면, 이후의 전파는 모두 Propagate 안에서 연쇄적으로 일어납니다.

**슬라이드 연결**

슬라이드 12~15는 이 처리 규칙을 작은 제약식 모음에 적용해 **손으로 추적(trace)**하는 예제들입니다. 시험에 자주 나오는 유형이니 직접 따라가 보세요.

---

## 슬라이드 12: Cubic Algorithm — Example 1

### 원문 내용

> Constraints: t ∈ x, x ⊆ y
> - **t ∈ x**
>   - AddToken(t, x): x.sol.add(t)
> - **x ⊆ y**
>   - AddEdge(x, y): x.succ.add(y)
>   - t ∈ x.sol
>   - AddToken(t, y): y.sol.add(t)

### 번역

> 제약식: t∈x, x⊆y (이 순서로 처리)
> - **t∈x 처리**: AddToken(t,x)로 x.sol에 t 추가 → x.sol={t}.
> - **x⊆y 처리**: AddEdge(x,y)로 x.succ에 y 추가. 이때 x.sol에 이미 t가 있으므로(t∈x.sol), AddToken(t,y)가 호출되어 y.sol에도 t 추가 → y.sol={t}.

### 해설

**개념 설명 — 순서가 만드는 묘미**

여기서는 `t∈x`를 **먼저** 처리합니다. 그래서 `x⊆y`로 간선을 놓는 순간, x에는 이미 t가 있어 AddEdge가 **그 자리에서** t를 y로 흘려보냅니다(슬라이드 10의 `for t∈x.sol: AddToken(t,y)`). 결과: `x.sol={t}`, `y.sol={t}`.

**핵심 교훈**: AddEdge는 "통로를 놓을 때 *이미 고여 있던* 토큰도 함께 흘린다"는 것. 그래서 제약식 처리 순서가 달라도 최종 해는 같습니다(슬라이드 13에서 반대 순서 확인).

---

## 슬라이드 13: Cubic Algorithm — Example 2

### 원문 내용

> Constraints: x ⊆ y, t ∈ x
> - **x ⊆ y**
>   - AddEdge(x, y): x.succ.add(y)
> - **t ∈ x**
>   - AddToken(t, x):
>     - x.sol.add(t)
>     - W.add(t, x)
>   - Propagate():
>     - (t, x) = W.removeOne()
>     - y ∈ x.succ
>     - AddToken(t, y): y.sol.add(t)

### 번역

> 제약식: x⊆y, t∈x (이번엔 반대 순서)
> - **x⊆y 처리**: AddEdge(x,y)로 x.succ에 y 추가. 이 시점엔 x.sol이 비어 있어 흘려보낼 토큰이 없음.
> - **t∈x 처리**: AddToken(t,x)로 x.sol에 t 추가하고 W에 (t,x) 기록. 이어 Propagate()가 (t,x)를 꺼내, x의 후속 y에게 AddToken(t,y) → y.sol={t}.

### 해설

**개념 설명 — 같은 결과, 다른 경로**

슬라이드 12와 **제약식은 같고 처리 순서만 반대**입니다. 이번엔 간선을 먼저 놓았는데, 그 순간 x가 비어 있어 아무 일도 안 일어납니다. 대신 나중에 `t∈x`가 처리되어 t가 x에 들어올 때, **Propagate가 후속 y로 t를 전파**합니다. 최종 결과는 슬라이드 12와 동일한 `x.sol={t}, y.sol={t}`.

**핵심 교훈**: 토큰이 먼저든 간선이 먼저든, **최소 해는 처리 순서에 무관**합니다. 이는 제약 시스템이 **단조(monotone)**이고 최소 고정점이 유일하기 때문입니다(슬라이드 8 각주³). Propagate의 역할 — "나중에 들어온 토큰을 기존 간선 따라 전파" — 이 잘 보이는 예제입니다.

---

## 슬라이드 14: Cubic Algorithm — Example 3

### 원문 내용

> Constraints: t ∈ x, t ∈ x ⇒ y ⊆ z
> - **t ∈ x**
>   - AddToken(t, x): x.sol.add(t)
> - **t ∈ x ⇒ y ⊆ z**
>   - AddEdge(y, z): y.succ.add(z)

### 번역

> 제약식: t∈x, 그리고 조건부 t∈x ⇒ y⊆z (이 순서)
> - **t∈x 처리**: x.sol={t}.
> - **조건부 처리**: 조건 t∈x를 검사하니 이미 x.sol에 t가 있음(참) → 즉시 AddEdge(y,z)로 y.succ에 z 추가.

### 해설

**개념 설명 — 조건이 미리 충족된 경우**

조건부 `t∈x ⇒ y⊆z`를 처리하는 시점에 **이미 `t∈x.sol`이 참**입니다(앞서 `t∈x`를 처리했으므로). 슬라이드 11의 첫 갈래에 해당 → 곧장 간선 `y→z`를 놓습니다. (이 예제는 y에 토큰이 없어 z로 흐를 건 없지만, 통로는 열렸습니다.)

**핵심 교훈**: 조건부 제약은 조건이 이미 참이면 **무조건 제약처럼** 즉시 발동됩니다. 슬라이드 15와 대비하세요.

---

## 슬라이드 15: Cubic Algorithm — Example 4

### 원문 내용

> Constraints: t ∈ x ⇒ y ⊆ z, t ∈ x
> - **t ∈ x ⇒ y ⊆ z**
>   - x.cond(t).add(y, z)
> - **t ∈ x**
>   - AddToken(t, x): W.add(t, x)
>   - Propagate():
>     - (t, x) = W.removeOne()
>     - (y, z) ∈ x.cond(t)
>     - AddEdge(y, z): y.succ.add(z)

### 번역

> 제약식: 조건부 t∈x⇒y⊆z 를 **먼저**, 그 다음 t∈x (슬라이드 14와 반대 순서)
> - **조건부 처리**: 이 시점엔 t가 x.sol에 없음(거짓) → 발동하지 않고 x.cond(t)에 (y,z)를 저장(잠재움).
> - **t∈x 처리**: AddToken(t,x)로 t를 넣고 W에 (t,x) 기록. Propagate()가 (t,x)를 꺼내 x.cond(t)에 잠들어 있던 (y,z)를 발견 → AddEdge(y,z)로 깨움.

### 해설

**개념 설명 — 조건이 나중에 충족되는 경우 (cond의 진가)**

이번엔 조건부를 **먼저** 만났는데 조건이 아직 거짓이라, 발동 대신 `x.cond(t)`에 **저장**합니다(슬라이드 11의 둘째 갈래). 그러다 나중에 `t∈x`가 처리되어 t가 x에 들어오면, Propagate가 `x.cond(t)`를 뒤져 잠든 `(y,z)`를 깨우고 간선을 놓습니다.

**핵심 교훈**: `x.cond(t)`는 "**조건이 아직 거짓인 조건부 제약을 미래를 위해 보관**"하는 장치입니다. 슬라이드 14(조건 이미 참 → 즉시 발동)와 슬라이드 15(조건 아직 거짓 → 저장 후 나중에 발동)를 짝지어 보면, 슬라이드 11의 두 갈래가 완성됩니다. 그리고 결과는 두 순서 모두 `y.succ={z}`로 동일 — 다시 한 번 **순서 무관성**을 확인합니다.

**슬라이드 연결 (12~15 종합)**

네 예제는 (토큰·간선의 순서) × (조건부의 조건이 이미/나중에 충족)의 모든 조합을 보여 줍니다. 시험에서 "다음 제약식들을 처리해 각 변수의 sol/succ을 구하라"는 추적 문제로 자주 나옵니다.

---

## 슬라이드 16: Time Complexity (시간 복잡도)

### 원문 내용

> Assumptions:
> - Number of tokens = O(n)
> - Number of variables = O(n)
> - Number of t ∈ x constraints = O(n)
> - Number of x ⊆ y constraints = O(n²)
>
> - Each pair (t, v) can be added at most once to the worklist, so the number of iterations of Propagate is O(n²)
> - AddEdge is called O(n²) times because there are O(n²) conditional constraints
> - AddToken is called O(n³) times in total because there are O(n²) edges and O(n) tokens
> - Updating a set takes O(1)
> - In total, the algorithm runs in O(n³) time

### 번역

> 가정:
> - 토큰 수 = O(n)
> - 변수 수 = O(n)
> - `t∈x` 형태 제약 수 = O(n)
> - `x⊆y` 형태 제약 수 = O(n²)
>
> 분석:
> - 각 (t,v) 쌍은 작업 목록에 **최대 한 번**만 들어갈 수 있으므로, Propagate의 반복 횟수는 O(n²) (토큰 O(n) × 변수 O(n)).
> - AddEdge는 O(n²)번 호출된다 (조건부 제약이 O(n²)개이므로).
> - AddToken은 전체적으로 O(n³)번 호출된다 (간선 O(n²)개 × 토큰 O(n)개).
> - 집합 갱신은 O(1).
> - 따라서 전체 알고리즘은 **O(n³)** 시간에 동작한다.

### 해설

**개념 설명 — "왜 3차인가"를 한 줄씩**

지배항(가장 비싼 부분)은 **AddToken의 총 호출 횟수**입니다.
- AddEdge로 만들 수 있는 간선은 최대 변수쌍 수 = O(n²)개.
- 각 간선 `x→y`를 따라, x의 토큰(최대 O(n)개) 하나하나가 y로 AddToken을 유발.
- 따라서 "간선 × 토큰" = O(n²) × O(n) = **O(n³)**.

각 AddToken/집합 연산은 (비트벡터·해시 등으로) O(1)이라 가정하므로, 총합 O(n³).

**왜 (t,v)가 한 번만 worklist에 들어가나?** AddToken은 `if t∉x.sol`일 때만 W에 넣습니다 — 같은 토큰을 같은 변수에 두 번 넣지 않으므로, (t,v) 쌍은 평생 최대 한 번 W에 등장합니다. 이것이 **단조성(monotone)**이 종료와 다항 시간을 보장하는 메커니즘입니다.

**배경 지식**: O(n³)은 "느려 보여도" **다항식이라 다루기 쉽고**, 큰 프로그램에선 부담스러울 수 있습니다. 그래서 슬라이드 17의 개선책이 나옵니다. 이 cubic 한계는 포인터 분석(강의 14~15)에서도 그대로 등장하는 유명한 결과입니다("cubic bottleneck").

---

## 슬라이드 17: Possible Improvements (가능한 개선)

### 원문 내용

> - Cycle elimination
> - Maintaining the worklist in topological order
> - Interleaving solution propagation and constraint processing
> - Using shared bit vectors

### 번역

> - **사이클 제거(Cycle elimination)**
> - 작업 목록을 **위상 정렬 순서(topological order)**로 유지
> - 해 전파와 제약식 처리를 **번갈아(interleave)** 수행
> - **공유 비트 벡터(shared bit vectors)** 사용

### 해설

**개념 설명 — 각 개선의 직관**

이론적 최악 O(n³)을 실전에서 줄이는 휴리스틱들입니다.

- **사이클 제거**: 그래프에 `x→y→...→x` 같은 순환이 있으면, 그 안의 모든 변수는 **결국 같은 해**를 갖게 됩니다(서로 토큰을 무한히 주고받으니까). 그러니 순환을 하나의 노드로 **합쳐(collapse)** 버리면 중복 전파가 사라집니다. 가장 효과 큰 최적화로 알려져 있습니다.⁵
- **위상 정렬 순서**: 토큰을 "흐름의 상류 → 하류" 순서로 전파하면, 한 번 처리한 노드를 다시 건드리는 일이 줄어 재방문이 감소합니다.
- **전파와 처리의 인터리빙**: 모든 제약을 다 읽은 뒤 전파하는 대신, 제약을 읽으면서 전파를 섞어 하면 작업 목록이 비대해지지 않습니다.
- **공유 비트 벡터**: 해 집합 `x.sol`을 비트 벡터로 표현하면, 합집합(전파)이 비트 OR 한 번으로 끝나 상수가 작아집니다. 게다가 똑같은 집합을 여러 변수가 공유하면 메모리도 절약됩니다.

**각주**

⁵ 사이클 제거는 Fähndrich 등이 포인터 분석에서 제안해 큰 속도 향상을 얻은 기법입니다. 강한 연결 요소(SCC)를 찾아 한 노드로 축약하는 것으로, 그래프 알고리즘(Tarjan SCC)과 직접 연결됩니다.

---

## 슬라이드 18: Control Flow in Object-Oriented Languages

### 원문 내용

> For each `x.m(...)`, we want to decide which method implementations may be called.
> - Simple solution: select any method named m whose signature matches the argument types
> - Better solution: class hierarchy analysis (CHA), which considers only the part of the class hierarchy spanned by the declared type of x
> - More refined solution: rapid type analysis (RTA), which decides objects of which classes are actually instantiated

### 번역

> 각 메서드 호출 `x.m(...)`에 대해, 실제로 어떤 메서드 구현이 호출될 수 있는지 결정하고자 한다.
> - **단순한 방법**: 인자 타입과 시그니처가 맞는, 이름이 m인 **모든** 메서드를 후보로 선택.
> - **더 나은 방법 — 클래스 계층 분석(CHA)**: x의 **선언된 타입(declared type)**이 포괄하는 클래스 계층의 일부만 고려.
> - **더 정밀한 방법 — 신속 타입 분석(RTA)**: 프로그램에서 **실제로 인스턴스화(instantiated)되는** 클래스의 객체들만 고려.

### 해설

**개념 설명 — 가상 메서드라는 또 다른 "함수가 값" 상황**

Java·C++ 같은 객체지향 언어에서 `x.m()`은 **동적 디스패치(dynamic dispatch)**됩니다. `x`의 **실제(런타임) 타입**에 따라 어느 클래스의 `m` 구현이 불릴지 결정되죠. 이것도 "호출 지점에서 대상이 코드만으로 자명하지 않다"는 점에서 일급 함수와 **같은 문제**입니다 — 그래서 같은 강의에 묶였습니다.

세 가지 정밀도:
- **단순(이름·시그니처 매칭)**: 이름이 같은 `m`을 전부 후보로. 매우 보수적(많은 거짓 간선).
- **CHA (Class Hierarchy Analysis)**: `x`의 **선언 타입 T**를 보고, T와 그 **서브클래스들**의 `m`만 후보로. 런타임 타입은 T이거나 T의 자손일 수밖에 없으니 이렇게 좁혀도 안전합니다. 호출 그래프만으로 빠르게 계산.
- **RTA (Rapid Type Analysis)**: CHA에 더해, 프로그램에서 `new C()`로 **실제로 만들어지는** 클래스만 후보로 남깁니다. 한 번도 인스턴스화되지 않는 클래스의 메서드는 절대 불릴 수 없으니 제외 → CHA보다 정밀.⁶

정밀도 순서: **단순 < CHA < RTA** (좁을수록 정밀, 모두 안전한 과근사).

**각주**

⁶ 더 정밀한 방법으로는 변수마다 가능한 타입 집합을 추적하는 **VTA(Variable Type Analysis)**, 그리고 이번 강의 앞부분의 **포인터/제약 기반 CFA(0-CFA 등)**가 있습니다. 즉 슬라이드 2~17의 제약 기법을 객체에 적용하면 RTA보다 더 정밀한 호출 그래프를 얻습니다. 정밀도↑ = 비용↑의 트레이드오프(강의 10의 정밀도-비용 균형과 동일한 주제).

**슬라이드 연결**

슬라이드 2~17(일급 함수의 CFA)과 슬라이드 18(OO 가상 메서드)은 **"호출 대상이 동적으로 정해진다"는 같은 문제의 두 얼굴**입니다. 둘 다 보수적인 호출 그래프를 만들어 강의 10의 절차간 분석에 넘겨 줍니다.

---

## 슬라이드 19: Summary (요약)

### 원문 내용

> - Control flow analysis conservatively approximates call graphs when functions are first-class values
> - Constraint rules relate each variable to the set of functions it may hold, including flow through calls and returns
> - The constraint system is an instance of a general class solvable in cubic time via a worklist-based algorithm on a graph of inclusion edges
> - Key data structures: per-variable solution sets, successor edges, and conditional constraints, driven by a token–variable worklist

### 번역

> - 제어 흐름 분석은 함수가 일급 값일 때 호출 그래프를 **보수적으로 근사**한다.
> - 제약식 규칙은 각 변수를 그것이 담을 수 있는 함수 집합과 연결하며, **호출과 반환을 통한 흐름**도 포함한다.
> - 이 제약 시스템은 포함 간선 그래프 위의 **워크리스트 기반 알고리즘**으로 **3차 시간**에 풀 수 있는 일반 문제 부류의 한 사례다.
> - 핵심 자료구조: 변수별 해 집합(sol), 후속 간선(succ), 조건부 제약(cond), 그리고 이를 구동하는 **토큰–변수 작업 목록(W)**.

### 해설

**전체 정리 — 강의 11의 한 장 요약**

1. **문제**: 함수가 값이거나(일급 함수) 메서드가 가상일 때, "이 호출 지점은 어디로?"가 자명하지 않다 → **호출 그래프**를 안전하게 만들어야 한다(슬라이드 2, 18).
2. **형식화**: 변수마다 "가능한 함수 집합" `[x]`를 두고, 정의·대입·호출을 **세 가지 제약식**(`t∈x`, `x⊆y`, `t∈x⇒y⊆z`)으로 번역(슬라이드 3~7, 8).
3. **해법**: 포함 간선 그래프 + 워크리스트로 **최소 해**를 **O(n³)**에 계산(슬라이드 9~16).
4. **개선·확장**: 사이클 제거 등 최적화(17), 객체지향의 CHA/RTA(18).

**다른 강의와의 연결 (파일 간 연결성)**

- ← **강의 3~4 (타입 분석)**: "제약식을 모아 한꺼번에 푼다"는 제약 기반 분석의 철학을 공유.
- ← **강의 5~6 (격자·고정점)**: "최소 해 = 최소 고정점"이며, 단조성이 종료를 보장.
- ← **강의 7~8 (데이터플로우·워크리스트)**: Propagate의 워크리스트는 데이터플로우 워크리스트와 같은 고정점 반복.
- → **강의 10 (절차간 분석)**: CFA가 만든 호출 그래프가 절차간 데이터플로우의 **입력**. (강의 11은 강의 10의 빠진 전제를 채운다.)
- → **강의 14~15 (포인터 분석)**: 똑같은 세 가지 제약식과 cubic 알고리즘이 "변수가 가리킬 수 있는 **메모리 위치 집합**"을 구하는 데 재사용됨. CFA는 포인터 분석의 특수한 형태(토큰=함수)로 볼 수 있다.

---

## 마치며

강의 11은 강의 10의 절차간 분석이 **암묵적으로 가정했던 호출 그래프를 어떻게 만드는가**를 채워 주는 다리입니다. 한 문장으로: **"함수가 값이 되는 순간 호출 대상이 불확실해지고, 이를 제약식으로 형식화해 워크리스트 기반 3차 알고리즘으로 최소 해를 구하면, 안전한 호출 그래프가 나온다."**

이 강의에서 익힌 **세 가지 제약식 형태와 cubic 알고리즘**은 이 과목에서 가장 재사용성이 높은 도구입니다 — 포인터 분석(14~15)에서 거의 그대로 다시 만나게 됩니다. 슬라이드 12~15의 추적 예제와 슬라이드 16의 복잡도 유도는 시험 단골이므로, 손으로 직접 돌려 보길 권합니다.
