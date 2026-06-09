# Pointer Analysis (2) - 강의 슬라이드 해설

CSE552 Program Analysis — Lecture 15
강사: Jaemin Hong

> **이 파일을 읽는 법**: 각 슬라이드는 `원문 내용` → `번역` → `해설`(개념·배경·맥락) → 필요 시 `각주`/`슬라이드 연결` 순서입니다. 누락·왜곡 없이 원문을 모두 담되 처음 보는 사람도 따라올 수 있게 풀어 썼습니다.

---

## 강의 15 전체 조감도 (먼저 큰 그림)

강의 14의 포인터 분석(Andersen·Steensgaard)은 **흐름 무감각(flow-insensitive)** — 문장 순서를 무시하고 "프로그램 어딘가에서 가리킬 수 있는 모든 것"을 한 번에 구했습니다. 강의 15는 더 정밀한 **흐름 감각(flow-sensitive)** 포인터 분석으로 나아갑니다. 이는 **각 프로그램 지점마다** 메모리 상태를 따로 추적하는, 강의 7~8의 데이터플로우 분석 스타일입니다.

이 강의의 가장 중요한 개념은 **강한 갱신(strong update) vs 약한 갱신(weak update)**입니다. 포인터를 통한 간접 대입 `*p = e`를 할 때:
- 포인터 p가 **확실히 하나의 메모리 셀**을 가리키면 → 그 셀의 값을 **덮어쓸(strong)** 수 있음(정밀).
- p가 **여러 셀 중 어느 것일지 모르면** → 어느 것도 확실히 못 덮으므로 기존 값과 **합쳐야(weak)** 함(보수적·안전).

강의의 줄거리는 "처음엔 무조건 strong update를 했다가(잘못됨) → 점점 조건을 붙여 가며 → 결국 **추상 셀이 단 하나의 런타임 셀을 나타낼 때만 strong update가 건전(sound)**하다"는 깨달음에 도달하는 과정입니다(슬라이드 8~15). 마지막엔 절차간 + 재귀에서 이 조건이 어떻게 더 까다로워지는지(슬라이드 16~22) 봅니다.

핵심 교훈: **건전성(soundness)을 지키면서 정밀도를 얻는 것은 미묘하며, 잘못하면 unsound가 된다.** 이 강의는 그 함정과 올바른 해법을 단계적으로 보여 줍니다.

---

## 슬라이드 1: 제목 슬라이드

### 원문 내용
> Pointer Analysis (2)
> CSE552 Program Analysis — Lecture 15
> Jaemin Hong

### 번역
> 포인터 분석 (2) / CSE552 프로그램 분석 — 강의 15 / 홍재민

### 해설
포인터 분석 2편. **흐름 감각** 포인터 분석과 그 핵심 난제인 **strong/weak update**를 다룹니다.

---

## 슬라이드 2: Flow-Sensitive Pointer Analysis

### 원문 내용
> - Flow-insensitive pointer analysis is sometimes too imprecise
> ```
> x = &a;
> *x = 1;
> x = &b;
> *x = 2;
> ```
> - Flow-sensitive pointer analysis provides better precision

### 번역
> - **흐름 무감각** 포인터 분석은 때때로 너무 부정확하다
> - 코드: x가 a를 가리킬 때 `*x=1`(a에 1), 그 뒤 x가 b를 가리킬 때 `*x=2`(b에 2)
> - **흐름 감각** 포인터 분석은 더 나은 정밀도를 제공한다

### 해설

**개념 설명 — 순서를 봐야 정확하다**

흐름 무감각 분석(강의 14)은 문장 순서를 무시하므로, "x는 a 또는 b를 가리킨다(`pt(x)={a,b}`)"고만 봅니다. 그러면 `*x=1`이 a와 b 둘 다 건드릴 수 있다고 보수적으로 처리 → 부정확.

하지만 **순서를 보면** 명확합니다: 첫 `*x=1` 시점엔 x=&a이므로 a에만 1, 둘째 `*x=2` 시점엔 x=&b이므로 b에만 2. **흐름 감각** 분석은 각 지점의 x값을 따로 추적해 이를 정확히 잡아냅니다. 강의 7~8의 흐름 감각 데이터플로우를 포인터에 적용한 것.

**슬라이드 연결**: 더 나아가 포인터 분석을 값 분석과 결합하는 동기가 슬라이드 3.

---

## 슬라이드 3: Combining Pointer Analysis with Another Analysis

### 원문 내용
> - Pointer analysis can be performed as a pre-analysis, and the points-to information can be utilized in the main analysis
> - However, performing pointer analysis as a part of the main analysis can provide additional precision

### 번역
> - 포인터 분석을 **사전 분석(pre-analysis)**으로 먼저 수행하고, 그 points-to 정보를 본 분석에서 활용할 수 있다
> - 하지만 포인터 분석을 **본 분석의 일부로** 함께 수행하면 추가적인 정밀도를 얻을 수 있다

### 해설

**개념 설명 — 따로 vs 함께**

포인터 정보를 쓰는 두 방식:
1. **사전 분석으로 분리**: 강의 14처럼 포인터 분석을 먼저 끝내고, 그 결과(pt)를 값 분석 등 본 분석에 입력으로 줌. 간단하지만, 포인터 분석이 흐름 무감각이면 본 분석의 정밀도도 제한됨.
2. **본 분석과 통합**: 포인터 정보와 값 정보를 **함께** 흐름 감각으로 추적. 서로의 정밀도를 높여 줌(예: 값 분석이 분기를 쳐내면 포인터도 정밀해짐). 강의 14 슬라이드 21의 "CFA와 pt를 동시에"와 같은 통합 철학.

이 강의는 방식 2를 택해, **값(interval) × 포인터(points-to)**를 한 상태에 담아 흐름 감각으로 분석합니다(슬라이드 4).

---

## 슬라이드 4: Abstract Domains

### 원문 내용
> - Cell = Var ∪ Alloc
> - Value = Interval × 𝒫(Cell)
>   - ([l, h], {c, ...}) represents an integer between l and h or a pointer to c
> - State = Cell → Value
> - JOIN(v) = ⨆_{u∈pred(v)} ⟦u⟧

### 번역
> - **Cell = Var ∪ Alloc**: 추상 셀 = 변수 셀과 할당 셀의 합 (강의 14)
> - **Value = Interval × 𝒫(Cell)**: 추상값 = (구간, 셀 집합) 쌍
>   - `([l,h], {c,...})`는 "l과 h 사이의 정수" **또는** "c를 가리키는 포인터"를 나타냄
> - **State = Cell → Value**: 상태 = 각 셀에 추상값을 대응
> - **JOIN(v) = v의 선행자들의 결합** (전방 분석)

### 해설

**개념 설명 — 정수와 포인터를 한 값에**

추상값이 **구간(Interval, 강의 9)과 셀 집합(𝒫(Cell), 강의 14)의 쌍**입니다. 왜 쌍인가? 한 변수가 어떤 경로에선 정수, 다른 경로에선 포인터일 수 있어, 둘 다 담아야 안전하기 때문입니다. 예: `([1,1], ∅)`은 정수 1, `(⊥, {x})`는 x를 가리키는 포인터, `([1,1], {x})`는 "정수 1 또는 x 포인터"(합류 결과).

상태는 `State = Cell → Value` — 각 메모리 셀에 추상값을 매핑. 이것이 강의 7~8·9의 흐름 감각 상태(각 지점마다 변수→추상값)를 포인터까지 포함하도록 확장한 것. JOIN은 선행자 결합(전방). 강의 9의 interval 분석에 points-to를 얹은 구조입니다.

**슬라이드 연결**: 이 상태를 바꾸는 전이 함수가 슬라이드 5(포인터 무관)·6(포인터 관련).

---

## 슬라이드 5: Transfer Functions — Pointer-Irrelevant

### 원문 내용
> - x = e: t_v(σ) = σ[x ↦ eval(σ, e)]
> - eval(σ, x) = σ(x)
> - eval(σ, n) = ([n, n], ⊥)
> - eval(σ, input()) = (⊤, ⊥)
> - eval(σ, e1 op e2) = op̂(eval(σ, e1), eval(σ, e2))

### 번역
> - `x = e` (직접 대입): 상태 σ에서 x를 `eval(σ,e)`로 갱신
> - `eval`(식 평가):
>   - 변수 x → σ(x) (현재 값)
>   - 정수 상수 n → `([n,n], ⊥)` (정수 n, 포인터 성분 없음)
>   - `input()` → `(⊤, ⊥)` (어떤 정수든, 즉 구간 ⊤)
>   - `e1 op e2` → 추상 연산 op̂를 두 평가 결과에 적용

### 해설

**개념 설명 — 정수 부분은 강의 9 그대로**

포인터와 무관한 연산(직접 대입, 산술)은 강의 9의 interval 분석과 동일합니다. `eval`이 식을 추상값으로 평가하고, 직접 대입은 그 값을 변수 셀에 넣습니다. 상수는 점 구간, `input()`은 ⊤(미지의 정수), 이항 연산은 추상 연산자 op̂(강의 9의 interval 덧셈 등). 포인터 성분은 ⊥(없음)로 둡니다. 익숙한 부분이라 빠르게 넘어가고, 진짜 새로운 건 슬라이드 6의 포인터 연산.

---

## 슬라이드 6: Transfer Functions — Pointer-Relevant

### 원문 내용
> - eval(σ, &x) = (⊥, {x})
> - eval(σ, alloc()) = (⊥, {alloc-i})
> - eval(σ, *e) = fetch(σ, eval(σ, e))
> - fetch(σ, (_, C)) = ⨆_{c∈C} σ(c)

### 번역
> - `eval(σ, &x)` = `(⊥, {x})` (x를 가리키는 포인터)
> - `eval(σ, alloc())` = `(⊥, {alloc-i})` (할당 셀을 가리키는 포인터)
> - `eval(σ, *e)` = `fetch(σ, eval(σ, e))` (e를 평가해 포인터를 얻고, 그것이 가리키는 셀의 값을 가져옴)
> - `fetch(σ, (_, C))` = `⨆_{c∈C} σ(c)` (포인터가 가리킬 수 있는 모든 셀 c의 값을 결합)

### 해설

**개념 설명 — 역참조 읽기 = fetch**

포인터 연산:
- `&x`: x의 주소 → 포인터 성분 {x}.
- `alloc()`: 새 할당 셀 → {alloc-i}.
- `*e` (역참조 읽기): e를 평가해 "가리키는 셀 집합 C"를 얻은 뒤, **fetch**로 그 셀들의 값을 가져옵니다. C에 셀이 여럿이면(어느 것일지 모름) **모두 결합(⨆)** — 보수적으로 합칩니다.

예: `w = *z`에서 z가 {x,y}를 가리키면, `fetch`는 `σ(x) ⨆ σ(y)`(x와 y의 값 합집합)를 반환. 슬라이드 7에서 확인. 역참조 **쓰기**(`*e=...`)는 훨씬 미묘해서 슬라이드 8부터 따로 다룹니다 — 여기가 strong/weak update의 무대.

**슬라이드 연결**: 슬라이드 7이 읽기까지의 전이를 예시.

---

## 슬라이드 7: Transfer Functions — Example

### 원문 내용
> ```
> x = 1;
> // x: ([1,1], bot)
> y = 2;
> // x: ([1,1], bot), y: ([2,2], bot)
> if ... {
>   z = &x;
>   // ..., z: (bot, {x})
> } else {
>   z = &y;
>   // ..., z: (bot, {y})
> }
> // ..., z: (bot, {x, y})
> w = *z;
> // ..., w: ([1,2], bot)
> ```

### 번역
> x=1, y=2 후 각각 점 구간. 분기에서 z=&x 또는 z=&y → 합류 시 z는 `(⊥, {x,y})`(x 또는 y 가리킴). `w=*z`는 fetch로 x값([1,1])과 y값([2,2])을 결합 → `w: ([1,2], ⊥)`.

### 해설

**개념 설명**

흐름 감각 상태가 각 지점마다 갱신됩니다. 핵심 장면은 `w=*z`: z가 {x,y}를 가리키므로(어느 것일지 모름), fetch가 `σ(x)⨆σ(y) = [1,1]⨆[2,2] = [1,2]`. w는 1 또는 2일 수 있다고 정확히(읽기는 합쳐도 안전) 추론. **읽기는 합집합으로 OK**지만, 쓰기는 다릅니다 — 슬라이드 8.

---

## 슬라이드 8: Indirect Assignment (Incorrect)

### 원문 내용
> - *e1 = e2: t_v(σ) = update(σ, eval(σ, e1), eval(σ, e2))
> - update(σ, (_, C), v) = σ[c1 ↦ v, ..., cn ↦ v] where C = {c1, ..., cn}

### 번역
> - `*e1 = e2` (역참조 쓰기): e1을 평가해 가리키는 셀 집합 C를, e2를 평가해 값 v를 얻고, `update`로 C의 모든 셀에 v를 씀
> - `update(σ, (_,C), v)` = C의 **모든 셀 c1,...,cn에 v를 덮어씀** (σ[c1↦v, ..., cn↦v])

### 해설

**개념 설명 — 첫 번째(틀린) 시도**

역참조 쓰기 `*e1=e2`를 어떻게 처리할까요? 첫 시도: e1이 가리킬 수 있는 **모든 셀에 v를 덮어쓰기**. 이를 "무조건 strong update"라 부를 수 있습니다.

문제는 이게 **틀렸다(incorrect)**는 것입니다(슬라이드 제목이 "Incorrect"). e1이 여러 셀을 가리킬 수 있을 때(어느 것일지 모름), 그 *전부*를 덮으면 안 됩니다 — 실제로는 그중 하나만 바뀌니까요. 슬라이드 9~10이 이 오류를 구체적으로 보여 줍니다.

**슬라이드 연결**: 왜 틀렸는지 슬라이드 9(단일 셀은 OK)·10(다중 셀은 unsound).

---

## 슬라이드 9: Indirect Assignment (Incorrect) — Example 1

### 원문 내용
> ```
> x = 1;
> // x: ([1,1], bot)
> y = &x;
> // x: ([1,1], bot), y: (bot, {x})
> *y = 2;
> // x: ([2,2], bot), y: (bot, {x})
> ```

### 번역
> x=1, y=&x(y가 x 가리킴). `*y=2`는 y가 가리키는 셀(x)에 2를 씀 → x: `([2,2], ⊥)`.

### 해설

**개념 설명 — 단일 셀: 덮어쓰기가 맞다**

여기서는 y가 **확실히 x 하나만** 가리킵니다(`pt(y)={x}`). 그러니 `*y=2`는 x를 2로 **덮어써도** 정확합니다(x는 이제 확실히 2). 이 경우 슬라이드 8의 무조건 덮어쓰기가 옳게 작동합니다. 문제는 가리키는 셀이 **여럿**일 때 — 슬라이드 10.

---

## 슬라이드 10: Indirect Assignment (Incorrect) — Example 2

### 원문 내용
> ```
> x = 1;
> y = 2;
> if ... {
>   z = &x;
>   // ..., z: (bot, {x})
> } else {
>   z = &y;
>   // ..., z: (bot, {y})
> }
> // ..., z: (bot, {x, y})
> *z = 3;
> // x: ([3,3], bot), y: ([3,3], bot), z: (bot, {x, y})
> ```
> - Unsound results: [3, 3] for x and y
>   - However, x may be 1, and y may be 2

### 번역
> z가 {x,y}를 가리키는 상태에서 `*z=3`. 무조건 덮어쓰기 규칙은 x와 y **둘 다** 3으로 만듦.
> - **불건전한(unsound) 결과**: x와 y가 모두 [3,3]
>   - 그러나 실제로는 **x가 1일 수도, y가 2일 수도** 있다

### 해설

**개념 설명 — 다중 셀 덮어쓰기는 unsound (핵심 오류)**

z는 x **또는** y를 가리킵니다(둘 중 어느 것인지는 분기에 달림). `*z=3`은 실행 시 **둘 중 하나만** 3으로 바꿉니다:
- 만약 z=&x였다면: x=3, y=2 (그대로).
- 만약 z=&y였다면: x=1 (그대로), y=3.

그런데 무조건 덮어쓰기는 **둘 다** 3으로 만들어, "x는 반드시 3, y는 반드시 3"이라 결론냅니다. 이는 **건전성 위반(unsound)**입니다 — 실제로 x가 1일 수 있는데(z=&y인 경우) 분석은 그 가능성을 **버려** 버렸습니다.

**왜 unsound가 치명적인가**: 정적 분석의 생명은 "실제로 가능한 모든 경우를 포함"하는 것(강의 5~6). x=1 가능성을 누락하면 분석이 거짓말이 되고, 이를 믿는 최적화·검증이 틀립니다. → 다중 셀일 땐 덮어쓰면 안 되고 **합쳐야** 합니다. 그 수정이 슬라이드 11.

**슬라이드 연결**: 수정안(strong vs weak 구분)이 슬라이드 11.

---

## 슬라이드 11: Indirect Assignment (Still Incorrect)

### 원문 내용
> - *e1 = e2: t_v(σ) = update(σ, eval(σ, e1), eval(σ, e2))
> - update(σ, (_, C), v) =
>   - σ[c ↦ v]  if C = {c}
>   - ⨆_{c∈C} σ[c ↦ σ(c) ⊔ v]  otherwise
> - Strong update: the new value overwrites the memory
> - Weak update: the new value is joined with the old value

### 번역
> - `update(σ, (_,C), v)`:
>   - **C가 단일 셀 `{c}`이면** → `σ[c↦v]` (**강한 갱신(strong update)**: 덮어쓰기)
>   - **그 외(여러 셀)이면** → 각 셀 c에 대해 `σ(c) ⊔ v` (**약한 갱신(weak update)**: 기존 값과 합침)
> - **Strong update**: 새 값이 메모리를 덮어쓴다
> - **Weak update**: 새 값이 기존 값과 결합된다

### 해설

**개념 설명 — strong vs weak update (이 강의의 핵심 개념)**

슬라이드 10의 오류를 고치는 핵심 아이디어:
- **강한 갱신(strong update)**: 가리키는 셀이 **딱 하나**(`C={c}`)면, 그 셀이 확실히 바뀌므로 **덮어쓰기**(`c↦v`). 정밀.
- **약한 갱신(weak update)**: 가리키는 셀이 **여럿**이면, 그중 하나만 바뀔 뿐 어느 것인지 모르므로, 각 셀에 대해 **기존 값과 새 값을 합침**(`σ(c)⊔v`). 그러면 x는 "1 또는 3", y는 "2 또는 3"으로 남아 실제 가능성을 보존(sound).

이 구분으로 슬라이드 10의 unsound가 해결됩니다. **하지만** 슬라이드 제목이 여전히 "Still Incorrect" — 이 규칙도 아직 완전하지 않습니다. 단일 셀이라고 무조건 strong update하면 또 다른 함정이 있기 때문(슬라이드 12~13).

**슬라이드 연결**: 새 규칙도 틀리는 경우가 슬라이드 12·13.

---

## 슬라이드 12: Indirect Assignment (Still Incorrect) — Example 1

### 원문 내용
> ```
> x = 1; y = 2;
> if ... { z = &x; } else { z = &y; }
> // ..., z: (bot, {x, y})
> *z = 3;
> // x: ([1,3], bot), y: ([2,3], bot), z: (bot, {x, y})
> ```

### 번역
> 슬라이드 10과 같은 코드. 이제 z가 {x,y}(다중)이므로 **weak update** → x: `[1,3]`(1 또는 3), y: `[2,3]`(2 또는 3). 건전한 결과.

### 해설

**개념 설명 — weak update가 제대로 작동**

슬라이드 10의 unsound([3,3])가 슬라이드 11의 규칙으로 고쳐졌습니다. z가 다중 셀이라 weak update: x는 `[1,1]⊔[3,3]=[1,3]`, y는 `[2,2]⊔[3,3]=[2,3]`. 실제 가능성(x∈{1,3}, y∈{2,3})을 모두 포함 → sound. 여기까진 잘 작동합니다. 하지만 슬라이드 13이 strong update가 단일 셀에서도 틀릴 수 있음을 보입니다.

---

## 슬라이드 13: Indirect Assignment (Still Incorrect) — Example 2

### 원문 내용
> ```
> for i in [0, 1] {
>   x = alloc();
>   // x: (bot, {alloc-0})
>   *x = 0;
>   // ..., alloc-0: ([0, 0], bot)
>   if i == 0 {
>     y = x;
>     // ..., y: (bot, {alloc-0}), alloc-0: ([0,0], bot)
>   }
> }
> *x = 1;
> // ..., alloc-0: ([1, 1], bot)
> ```
> - Unsound results: [1, 1] for alloc-0
>   - However, *y is 0

### 번역
> 루프가 두 번 도는데, 매 반복 `x=alloc()`로 새 메모리를 만듦. 추상적으로는 둘 다 같은 셀 `alloc-0`. i=0일 때 `y=x`로 y가 첫 alloc-0을 가리킴. 루프 후 `*x=1`은 x가 **단일 셀 {alloc-0}**을 가리키므로 strong update → alloc-0: `[1,1]`.
> - **불건전한 결과**: alloc-0이 [1,1]
>   - 그러나 실제로 `*y`(첫 번째 할당 메모리)는 **0**이다

### 해설

**개념 설명 — 추상 셀 ≠ 단일 런타임 셀 (두 번째 함정)**

이게 미묘한 함정입니다. `pt(x)={alloc-0}`이라 "단일 셀"로 보여 strong update를 했지만, **추상 셀 alloc-0은 실제로 두 개의 런타임 메모리**(루프 1차·2차에서 만든 것)를 나타냅니다(할당 지점 추상화의 한계, 강의 14 슬라이드 3 각주).

루프 후 `*x=1`은 **두 번째** 런타임 메모리만 1로 바꿉니다. 첫 번째 메모리(y가 가리키는 것)는 여전히 0. 그런데 strong update는 alloc-0 전체를 1로 덮어, "`*y`도 1"이라 (잘못) 결론 → `*y`가 실제로 0인데 unsound.

**핵심 통찰**: `C={c}`라는 것(추상 셀이 하나)과 "그 셀이 단일 런타임 메모리"라는 것은 **다릅니다**. 추상 셀 하나가 여러 런타임 셀을 뭉친 경우(루프 할당, 재귀 변수), strong update는 unsound. → 진짜 조건은 "추상 셀이 단 하나의 런타임 셀을 나타낼 때"입니다. 그 정정이 슬라이드 14.

**슬라이드 연결**: 올바른 규칙이 슬라이드 14.

---

## 슬라이드 14: Indirect Assignment (Correct)

### 원문 내용
> - x = e: t_v(σ) = update(σ, (⊥, {x}), eval(σ, e))
> - *e1 = e2: t_v(σ) = update(σ, eval(σ, e1), eval(σ, e2))
> - update(σ, (_, C), v) =
>   - σ[x ↦ v]  if C = {x}
>   - ⨆_{c∈C} σ[c ↦ σ(c) ⊔ v]  otherwise
> - Strong update: the new value overwrites the memory
>   - Used when the abstract cell represents a single runtime cell
> - Weak update: the new value is joined with the old value
>   - Used when the abstract cell represents possibly multiple runtime cells

### 번역
> - 직접 대입 `x=e`도 update로 통일: `update(σ, (⊥,{x}), eval(σ,e))`
> - `update(σ, (_,C), v)`:
>   - **C가 단일 셀이고 그것이 변수 `{x}`이면** → strong update `σ[x↦v]`
>   - 그 외 → weak update `σ(c)⊔v`
> - **Strong update**: 추상 셀이 **단 하나의 런타임 셀**을 나타낼 때 사용 (덮어쓰기)
> - **Weak update**: 추상 셀이 **여러 런타임 셀**을 나타낼 수 있을 때 사용 (합침)

### 해설

**개념 설명 — 올바른 조건: "단일 런타임 셀"**

슬라이드 13의 교훈을 반영한 최종 규칙입니다. strong update의 조건이 "추상 셀이 하나(`C={c}`)"에서 **"그 셀이 단 하나의 런타임 셀을 확실히 나타낼 때"**로 강화됩니다.

핵심 구분:
- **변수 셀 x**: 보통 런타임에 단 하나(그 변수 한 칸). → strong update 가능. (단, 재귀 함수의 지역 변수는 예외 — 슬라이드 19~21.)
- **할당 셀 alloc-i**: 루프·재귀에서 여러 런타임 메모리를 뭉칠 수 있음. → **항상 weak update** (안전하게).

그래서 규칙은 "`C={x}` (단일 변수 셀)이면 strong, 그 외(다중 셀이거나 할당 셀)이면 weak". 이로써 슬라이드 13의 unsound가 해결됩니다(alloc-0은 할당 셀이라 weak → alloc-0: `[0,1]`로 *y=0 가능성 보존).

**배경 지식 — 요약 셀(summary cell)**: 여러 런타임 객체를 뭉친 추상 셀을 "요약 셀"이라 하고, 요약 셀엔 strong update가 금지됩니다. 단 하나를 확실히 나타내는 셀(보통 비재귀 함수의 지역 변수)만 strong update 허용. 정밀도와 건전성의 균형점.

**슬라이드 연결**: 올바른 규칙의 예가 슬라이드 15.

---

## 슬라이드 15: Indirect Assignment (Correct) — Example

### 원문 내용
> ```
> for i in [0, 1] {
>   x = alloc();
>   // x: (bot, {alloc-0})
>   *x = 0;
>   // ..., alloc-0: ([0, 0], bot)
>   if i == 0 { y = x; ... }
> }
> *x = 1;
> // ..., alloc-0: ([0, 1], bot)
> ```

### 번역
> 슬라이드 13과 같은 코드. 이제 alloc-0은 **할당 셀**(여러 런타임 셀 가능)이므로 `*x=1`에 **weak update** → alloc-0: `[0,1]`(0 또는 1). `*y`가 0일 가능성을 보존 → 건전.

### 해설

**개념 설명**

슬라이드 13의 unsound([1,1])가 올바른 규칙으로 고쳐졌습니다. alloc-0이 할당 셀이라 weak update → `[0,0]⊔[1,1]=[0,1]`. 이제 분석은 "첫 메모리는 0(y가 봄), 둘째 메모리는 1일 수 있음"을 모두 포함. 정밀도는 조금 잃지만(0인지 1인지 단정 못 함) 건전성을 지킵니다. 슬라이드 8→11→14의 세 단계 수정이 완성되는 순간.

**슬라이드 연결**: 절차내 흐름 감각 분석 완성. 슬라이드 16부터 절차간으로 확장.

---

## 슬라이드 16: Interprocedural Analysis — Without Pointers

### 원문 내용
> x = f(e1, ..., en):
> t_v(σ) = {
>   ([x1 ↦ eval(e1,σ), ..., xn ↦ eval(en,σ)], entry(f)),
>   (σ[x ↦ σ_return(RET)], after(v))
> }
> where x1,...,xn are parameters of f; entry(f) is entry node; σ_return is state at return node; after(v) is the after-call node
>
> return:
> t_v(σ) = {(σ_vi[x1 ↦ σ(RET)], after(v1)), ...}
> where vi is a call node; σ_vi is state at vi; xi is variable assigned the return value; after(vi) is after-call node for vi

### 번역
> **호출 `x = f(e1,...,en)`**: 두 가지 정보 흐름을 만듦:
> - (1) 실인자들을 매개변수에 바인딩한 상태를 **f의 진입 노드**로 보냄
> - (2) f의 반환 상태에서 RET를 x에 받은 상태를 **호출 다음 노드(after-call)**로 보냄
> **반환 `return`**: 각 호출 지점 vi에 대해, 호출 시점 상태 σ_vi에 반환값을 합쳐 그 호출의 다음 노드로 보냄

### 해설

**개념 설명 — 절차간 데이터플로우 복습 (강의 10)**

포인터를 빼고 보면 강의 10의 절차간 분석입니다. 호출은 두 간선을 만듭니다: **호출→피호출 진입**(인자 전달)과 **피호출 반환→호출 다음**(반환값 전달). 반환 노드는 모든 호출 지점으로 정보를 되돌려 보냅니다. 흐름 감각 + 절차간이라, 호출 전후 상태를 정확히 이어 줍니다. 여기까진 강의 10의 복습이고, 포인터가 끼면 추가 처리가 필요(슬라이드 17).

---

## 슬라이드 17: Interprocedural Analysis — With Pointers

### 원문 내용
> - The values of address-taken variables and heap addresses should be passed across function boundaries as well
> x = f(e1, ..., en):
> t_v(σ) = {
>   ([x1 ↦ eval(e1,σ), ..., c1 ↦ σ(c1), ...], entry(f)),
>   (σ[x ↦ σ_return(RET), c1 ↦ σ_return(c1), ...], after(v))
> }
> where ci is an address-taken variable or a heap address
>
> return:
> t_v(σ) = {(σ_vi[x1 ↦ σ(RET), c1 ↦ σ(c1), ...], after(v1)), ...}

### 번역
> - **주소가 취해진 변수(address-taken)와 힙 주소의 값도 함수 경계를 넘어 전달**되어야 한다
> - 호출 시: 인자뿐 아니라 그런 셀 ci들의 값 `σ(ci)`도 함께 f의 진입으로 보냄. 반환 시: 매개변수뿐 아니라 ci들의 (피호출자가 바꿨을 수 있는) 값도 호출 측으로 되받음.

### 해설

**개념 설명 — 포인터가 함수 경계를 넘게 한다**

포인터가 있으면 함수가 **인자 외의 메모리도 바꿀 수 있습니다**. 예: `foo(p)`가 `*p=1`을 하면, 호출자의 어떤 변수가 바뀝니다. 그래서:
- **호출 시**: 인자뿐 아니라 **주소가 취해진 변수(누군가 `&`로 주소를 딴 변수)와 힙 셀**의 값도 피호출자에게 넘김(피호출자가 그것들을 포인터로 건드릴 수 있으니).
- **반환 시**: 피호출자가 그 셀들을 바꿨을 수 있으므로, **바뀐 값을 호출자에게 되돌려** 줍니다.

"address-taken variable"만 넘기는 이유: 주소가 안 취해진 변수는 포인터로 접근 불가하니 함수가 못 건드림 → 넘길 필요 없음(효율화). 슬라이드 18이 예.

---

## 슬라이드 18: Interprocedural Analysis — Example

### 원문 내용
> ```
> fn foo(p) {
>   // x: ([0,0], bot), p: (bot, {x})
>   *p = 1;
>   // x: ([1,1], bot), p: (bot, {x})
> }
>
> x = 0;
> // x: ([0,0], bot)
> y = &x;
> // x: ([0,0], bot), y: (bot, {x})
> foo(y);
> // x: ([1,1], bot), y: (bot, {x})
> ```

### 번역
> x=0, y=&x 후 `foo(y)` 호출. foo는 매개변수 p=y={x}를 받고, x의 값([0,0])도 함께 받음. foo 안 `*p=1`은 p가 단일 셀 {x}이므로 strong update → x: [1,1]. 반환 시 바뀐 x값이 호출자로 돌아와 x: [1,1].

### 해설

**개념 설명**

`foo(y)`가 호출되면:
- p=y가 가리키는 {x}를 받고, **x의 값도 함께** foo 진입으로 전달(슬라이드 17).
- foo 안에서 `*p=1`이 x를 1로 strong update(p가 단일 변수 셀 x를 가리키므로 — 슬라이드 14 조건 충족).
- 반환 시 바뀐 x([1,1])가 호출자로 되돌아옴 → 호출 후 x: [1,1].

포인터를 통한 함수 간 부수 효과(side effect)가 정확히 추적됐습니다. 단, 이 strong update가 항상 안전한 건 아닙니다 — 재귀가 끼면 위험(슬라이드 19).

**슬라이드 연결**: strong update의 마지막 함정인 재귀가 슬라이드 19~22.

---

## 슬라이드 19: Recursive Functions

### 원문 내용
> - When x is a variable of a (mutually) recursive function, multiple runtime cells denoted by the abstract cell x can coexist

### 번역
> - x가 **(상호)재귀 함수의 변수**일 때, 추상 셀 x가 나타내는 **여러 런타임 셀이 동시에 존재**할 수 있다

### 해설

**개념 설명 — 재귀가 변수 셀을 요약 셀로 만든다**

슬라이드 14에서 "변수 셀은 보통 단일 런타임 셀이라 strong update 가능"이라 했습니다. 그런데 **재귀 함수**에서는 예외입니다. 재귀 호출이 진행되면 **같은 함수의 여러 활성화(activation)가 동시에 스택에 쌓이고**, 각 활성화마다 지역 변수 x의 인스턴스가 따로 존재합니다. 즉 추상 셀 x가 **여러 런타임 셀**을 나타내게 되어 — 할당 셀처럼 **요약 셀**이 됩니다.

따라서 재귀 함수의 지역 변수에 strong update를 하면 슬라이드 13과 같은 unsound가 발생합니다. 예가 슬라이드 20.

**슬라이드 연결**: 구체적 unsound 예가 슬라이드 20, 수정이 슬라이드 21.

---

## 슬라이드 20: Recursive Functions — Example

### 원문 내용
> ```
> fn foo() {
>   let x = 0;
>   // x: ([0,0], bot)
>   let p = &x; ...
>   if ... {
>     // x: ([0,0], bot)
>     foo();
>     // x: ([1,1], bot)
>   } else {
>     x = 1;
>     // x: ([1,1], bot)
>   }
>   // x: ([1,1], bot)
> }
> foo();
> ```
> - Unsound results: [1, 1] for x after foo returns
>   - However, x is 0

### 번역
> `foo`는 지역 x=0을 두고, 재귀 호출하거나 x=1로 둠. strong update를 쓰면 재귀 후 x가 [1,1]로 보임.
> - **불건전한 결과**: foo 반환 후 x가 [1,1]
>   - 그러나 (바깥 활성화의) x는 실제로 0이다

### 해설

**개념 설명 — 재귀에서 strong update가 unsound**

재귀 호출 `foo()` 안에서 (안쪽 활성화의) x가 1이 되어도, **바깥 활성화의 x는 여전히 0**입니다(각 활성화의 x는 별개 메모리). 그런데 추상 셀 x는 이 둘을 뭉치므로, strong update로 x=1을 덮어쓰면 "바깥 x도 1"이라 (잘못) 결론 → 바깥 x가 실제 0인데 unsound. 슬라이드 13(루프 할당)과 같은 구조의 오류가, 이번엔 **재귀 지역 변수**에서 발생.

**슬라이드 연결**: 해법(재귀 변수엔 weak update)이 슬라이드 21.

---

## 슬라이드 21: Updates in Interprocedural Analysis

### 원문 내용
> update(σ, (_, C), v) =
>   - σ[x ↦ v]  if C = {x} ∧ x is a local variable of a non-recursive function
>   - ⨆_{c∈C} σ[c ↦ σ(c) ⊔ v]  otherwise
> - Strong update: used when the abstract cell represents a single runtime cell
> - Weak update: used when the abstract cell represents possibly multiple runtime cells

### 번역
> - `update(σ, (_,C), v)`:
>   - **C가 단일 셀 {x}이고 x가 비재귀(non-recursive) 함수의 지역 변수이면** → strong update
>   - 그 외 → weak update
> - Strong update: 추상 셀이 단일 런타임 셀을 나타낼 때
> - Weak update: 여러 런타임 셀을 나타낼 수 있을 때

### 해설

**개념 설명 — 최종 조건: 비재귀 함수의 지역 변수만 strong**

슬라이드 14의 조건이 한 번 더 강화됩니다. strong update가 허용되는 건 오직:
> **C가 단일 변수 셀이고, 그 변수가 비재귀 함수의 지역 변수일 때.**

이 경우에만 추상 셀이 확실히 단 하나의 런타임 셀을 나타냅니다. 나머지 — 다중 셀, 할당 셀(루프), **재귀 함수의 변수**, (보수적으로) 전역 변수 등 — 는 모두 요약 셀일 수 있어 weak update. 슬라이드 8→11→14→21로 이어진 조건 강화의 종착점입니다.

**배경 지식**: 실무에선 "재귀 여부"를 호출 그래프의 SCC(강의 11·12)로 판정합니다 — SCC에 속한 함수의 지역 변수는 재귀적이므로 strong update 금지. 강의 11~12의 SCC 처리가 여기서도 쓰입니다.

**슬라이드 연결**: 수정된 규칙의 예가 슬라이드 22.

---

## 슬라이드 22: Updates in Interprocedural Analysis — Example

### 원문 내용
> ```
> fn foo() {
>   let x = 0;
>   // x: ([0,0], bot)
>   let p = &x; ...
>   if ... {
>     // x: ([0,0], bot)
>     foo();
>     // x: ([0,1], bot)
>   } else {
>     x = 1;
>     // x: ([0,1], bot)
>   }
>   // x: ([0,1], bot)
> }
> foo();
> ```

### 번역
> 슬라이드 20과 같은 코드. 이제 foo가 재귀 함수이므로 x에 **weak update** → x: `[0,1]`(0 또는 1). 바깥 활성화의 x=0 가능성을 보존 → 건전.

### 해설

**개념 설명**

슬라이드 20의 unsound([1,1])가 슬라이드 21 규칙으로 고쳐졌습니다. foo가 재귀라 x는 요약 셀 → weak update → `[0,0]⊔[1,1]=[0,1]`. 이제 "바깥 x는 0, 안쪽 x는 1일 수 있음"을 모두 포함 → sound. 정밀도(0인지 1인지 단정)는 잃지만 건전성을 지킵니다. 강의 전체의 결론이 완성됩니다.

**슬라이드 연결**: 전체 요약이 슬라이드 23.

---

## 슬라이드 23: Summary

### 원문 내용
> - Flow-sensitive pointer analysis provides better precision than flow-insensitive analysis
> - Indirect assignment through a pointer requires a case split: strong update overwrites the cell, while weak update joins with the old value
> - Strong update is sound only when the abstract cell denotes a single runtime cell; otherwise weak update must be used
> - Local variables of (mutually) recursive functions may denote multiple coexisting runtime cells, so strong update is restricted to local variables of non-recursive functions

### 번역
> - **흐름 감각** 포인터 분석은 흐름 무감각보다 정밀하다
> - 포인터를 통한 간접 대입은 **경우 분리**가 필요: strong update는 셀을 덮어쓰고, weak update는 기존 값과 합친다
> - strong update는 **추상 셀이 단 하나의 런타임 셀을 나타낼 때만** 건전하다; 그 외엔 weak update를 써야 한다
> - (상호)재귀 함수의 지역 변수는 여러 런타임 셀을 동시에 나타낼 수 있으므로, strong update는 **비재귀 함수의 지역 변수로 제한**된다

### 해설

**전체 정리 — 강의 15의 한 장 요약**

1. **흐름 감각**: 각 지점마다 상태(`Cell→Value`, Value=Interval×𝒫(Cell))를 추적 → 흐름 무감각보다 정밀(슬라이드 2~7).
2. **strong vs weak update**: 역참조 쓰기 `*p=e`의 핵심 결정.
   - strong(덮어쓰기): 정밀, **단일 런타임 셀일 때만 건전**.
   - weak(합치기): 보수적, 항상 안전.
3. **건전성 조건의 단계적 발견**(이 강의의 서사): 무조건 덮어쓰기(unsound, 슬14다중) → 단일 추상셀이면 strong(여전히 unsound, 슬13루프) → **단일 런타임 셀**이면 strong(슬14) → 재귀 변수 제외, **비재귀 함수 지역 변수만** strong(슬21).
4. **절차간 + 포인터**: address-taken 변수·힙 셀의 값도 함수 경계를 넘겨야(슬16~18). 재귀는 SCC로 판정해 weak update.

**다른 강의와의 연결 (파일 간 연결성)**

- ← **강의 5~6 (건전성·격자)**: 이 강의 전체가 "정밀도를 얻되 건전성(과근사)을 깨지 않기"의 사례. weak update의 ⊔는 격자 join.
- ← **강의 7~8 (흐름 감각 데이터플로우)**: 흐름 감각 상태, JOIN, 전이 함수의 직접 확장.
- ← **강의 9 (interval·위드닝)**: Value의 Interval 성분, eval/op̂.
- ← **강의 10 (절차간)**: 절차간 호출·반환 전이(슬16), 재귀 고정점.
- ← **강의 11~12 (cubic·SCC)**: 재귀 판정에 호출 그래프 SCC 사용.
- ← **강의 14 (포인터 1)**: 할당 지점 추상화, Cell, points-to. 흐름 무감각(14) → 흐름 감각(15)의 정밀화.

**가장 큰 교훈**: **strong update는 강력하지만 위험하다.** "추상 셀이 단 하나의 런타임 셀을 확실히 나타낼 때"만 건전하며, 루프 할당·재귀 변수처럼 하나의 추상 셀이 여러 런타임 셀을 요약하는 경우엔 반드시 weak update를 써야 합니다. 정밀도를 향한 욕심이 어떻게 건전성을 깨뜨릴 수 있는지를 보여 주는, 정적 분석 설계의 교과서적 사례입니다.

---

## 마치며

강의 15는 포인터 분석을 흐름 감각·절차간으로 끌어올리면서, **strong/weak update**라는 정밀도-건전성의 핵심 긴장을 깊이 파고듭니다. "무조건 덮어쓰기 → 단일 셀이면 덮어쓰기 → 단일 *런타임* 셀이면 덮어쓰기 → 비재귀 지역 변수만 덮어쓰기"로 조건이 단계적으로 강화되는 서사는, 정적 분석에서 **하나의 unsound 예제가 어떻게 규칙을 정교하게 다듬게 하는지**를 보여 줍니다. 시험에서는 (a) 주어진 코드에서 strong/weak update를 판정하고 결과 상태 추적(슬라이드 9~15), (b) 왜 특정 strong update가 unsound인지(루프 할당·재귀, 슬라이드 13·20), (c) strong update의 정확한 건전성 조건 서술(슬라이드 21)이 단골입니다.
