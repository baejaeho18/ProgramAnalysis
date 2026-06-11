# Dataflow Analysis (2) - 강의 슬라이드 해설

CSE552 Program Analysis — Lecture 8
강사: Jaemin Hong

> **이 파일을 읽는 법**: 각 슬라이드는 `원문 내용` → `번역` → `해설`(개념·배경·맥락) → 필요 시 `각주`/`슬라이드 연결` 순서입니다. 누락·왜곡 없이 원문을 모두 담되 처음 보는 사람도 따라올 수 있게 풀어 썼습니다.
>
> *참고: 이 강의는 두 버전의 PDF(37쪽·40쪽)가 있었는데, 더 완전한 40쪽 버전(`08-dataflow-2(1).pdf`)을 기준으로 작성했습니다.*

---

## 강의 8 전체 조감도 (먼저 큰 그림)

강의 7은 단조 프레임워크로 부호·상수전파 분석을 만들었습니다(둘 다 전방·값 추적). 강의 8은 같은 틀로 **네 가지 고전 데이터플로우 분석**을 만들며, 그것들이 **두 축(전방/후방 × may/must)으로 분류되는 4분면(four-quadrant)**을 이룸을 보입니다 — 데이터플로우 분석의 가장 유명한 정리(整理)입니다.

네 분석과 그 위치:
| | **may (∪, ⊆)** | **must (∩, ⊇)** |
|---|---|---|
| **forward(전방)** | 도달 정의(reaching definitions) | 사용 가능 식(available expressions) |
| **backward(후방)** | 살아있는 변수(live variables) | 매우 바쁜 식(very busy expressions) |

흐름:
1. **네 분석 각각** (슬라이드 2~22): live variable(레지스터 할당), available expression(중복 계산 제거), very busy expression(코드 호이스팅), reaching definition(def-use 그래프). 각각의 격자·JOIN·전이 규칙.
2. **두 축 분류** (슬라이드 24~29): **전방/후방**(과거/미래 정보, JOIN이 pred/succ), **may/must**(가능/필연, 멱집합 ⊆/역멱집합 ⊇). 모두 건전하되 불완전.
3. **전이 함수와 효율** (슬라이드 30~39): 전이 함수 `t_v` 추상화, **전파 워크리스트(propagation worklist)** 알고리즘으로 JOIN 중복 제거.

핵심 통찰: **하나의 단조 프레임워크가 네 분석을 모두 만들고, 그들은 (전방/후방)×(may/must)의 4분면으로 깔끔히 분류된다.** 이 분류는 강의 12의 guard 분석(live=후방may, available=전방must)이 어디서 왔는지, 강의 20의 reaching definition이 왜 트레이스 의미론을 요구하는지를 이해하는 열쇠입니다. **may=합집합·멱집합, must=교집합·역멱집합**이라는 대응이 핵심입니다.

---

## 슬라이드 1: 제목 슬라이드

### 원문 내용
> Dataflow Analysis (2)
> CSE552 Program Analysis — Lecture 8
> Jaemin Hong

### 번역
> 데이터플로우 분석 (2) / CSE552 프로그램 분석 — 강의 8 / 홍재민

### 해설
데이터플로우 분석 2편. **네 가지 고전 분석**과 그들의 **4분면 분류**(전방/후방 × may/must)를 다룹니다.

---

## 슬라이드 2: Live Variable Analysis

### 원문 내용
> - A variable is live at a program point if there exists an execution where its value is read later in the execution without it being written to in between
> ```c
> // nothing is live
> x = input();  // x is live
> if input() {  // x is live
>   y = x;      // y is live
> } else {
>   y = 1;      // y is live
> }
> // y is live
> z = y;
> ```

### 번역
> - 어떤 변수가 한 지점에서 **살아있다(live)**: 그 지점 이후에 (중간에 다시 쓰이지 않고) 그 값이 **읽히는 실행이 존재**할 때
> - 예: `x=input()` 후 x는 살아있음(나중에 `y=x`에서 읽힘). `z=y` 직전엔 y가 살아있음.

### 해설

**개념 설명 — 살아있는 변수 ★**

변수가 **살아있다(live)** = "지금 값이 **나중에 읽힐** 가능성이 있다"(다시 쓰이기 전에). 즉 "이 값이 미래에 쓸모 있는가". `x=input()` 후 x는 `y=x`에서 읽히니 살아있고, `y=1`로 덮인 뒤 옛 x값은 죽음(dead).

이것은 **미래(future)** 에 관한 정보(나중에 읽히나?)라 **후방(backward)** 분석입니다 — 정보가 뒤에서 앞으로 흐릅니다. 강의 12의 **live guard 분석**(진입 시 잡힌 락)이 이것을 락에 적용한 것. 동기가 슬3.

---

## 슬라이드 3: Live Variable Analysis — Motivation

### 원문 내용
> - We can approximate the set of live variables using dataflow analysis
> - Application: register allocation
> - We want: the answer "not live" can be trusted and "live" is safe but useless

### 번역
> - 데이터플로우 분석으로 살아있는 변수 집합을 근사
> - 응용: **레지스터 할당(register allocation)**
> - 목표: **"not live(죽음)" 판정은 신뢰할 수 있고**, "live(살아있음)"은 안전하지만 쓸모없음

### 해설

**개념 설명 — 레지스터 할당과 may 분석**

응용: **레지스터 할당** — 죽은 변수는 레지스터를 비워도 되므로, "죽음" 판정이 정확해야 합니다. 그런데 live variable은 **may 분석**이라 "살아있을 *수* 있다"를 과근사합니다 — 안전을 위해 살아있을 가능성이 조금이라도 있으면 "live". 따라서 **"not live"가 확실한 정보**(죽었다고 하면 진짜 죽음), "live"는 보수적(헛되이 살아있다 할 수 있음). 강의 1의 건전성·과근사가 여기 적용. 상태·JOIN이 슬4.

---

## 슬라이드 4: Live Variable Analysis — Abstract States

### 원문 내용
> (CFG: v1 → v2, v3; ⟦v1⟧={x,y}, ⟦v2⟧={x}, ⟦v3⟧={y})
> - State = (𝒫(Var), ⊆) — Power set lattice
> - For each CFG node v, ⟦v⟧ denotes the set of variables live before the node
> - JOIN(v) = ⨆_{u∈succ(v)} ⟦u⟧ = ⋃_{u∈succ(v)} ⟦u⟧
>   - This combines abstract states from the successors

### 번역
> - 상태 = `(𝒫(Var), ⊆)` — **멱집합 격자**(변수 집합, 포함 순서)
> - `⟦v⟧` = 노드 v **앞에서** 살아있는 변수 집합
> - **JOIN(v) = 후속자(successor)들의 합집합(⋃)** — 후방 분석이라 후속에서 정보가 옴

### 해설

**개념 설명 — 후방·may = 후속자 합집합 ★**

live variable의 구조:
- **격자 = 멱집합 `(𝒫(Var), ⊆)`** (변수 집합). may 분석의 전형(강의 5 슬25).
- **JOIN = 후속자(succ)들의 합집합(⋃)**. 두 가지가 핵심:
  - **후방(backward)**: 정보가 뒤(미래)에서 앞으로 → JOIN이 **후속자**(강의 7의 전방 JOIN=선행자와 반대).
  - **may(합집합)**: 어느 후속 경로에서든 살아있으면 살아있음(합쳐서 보존).

이 "후방+합집합"이 live variable을 4분면의 **후방-may** 칸에 놓습니다. 강의 12 슬15의 live guard 분석(후방·may·합집합)과 정확히 동일 구조. 전이 규칙이 슬5.

---

## 슬라이드 5: Live Variable Analysis — Constraint Rule (Assignment)

### 원문 내용
> - x=e: ⟦v⟧ = JOIN(v) \ {x} ∪ vars(e)
> ```c
> // y and z are live
> x = y + z;
> // x is live
> ```

### 번역
> - **대입 `x=e`**: `⟦v⟧ = (JOIN(v)에서 x 제거) ∪ e의 변수들`
>   - x를 쓰므로(덮으므로) x는 더 이상 옛 값으로 살아있지 않음 → 제거(kill)
>   - e를 읽으므로 e의 변수들은 살아있음 → 추가(gen)
> - 예: `x=y+z` 뒤(아래)에서 x가 살아있다면, 그 **앞**에서는 y·z가 살아있음(x는 곧 덮이니 죽음)

### 해설

**개념 설명 — kill/gen (후방으로 읽기) ★**

대입 `x=e`의 전이를 **뒤에서 앞으로** 읽습니다:
- **kill**: x를 새로 쓰므로(덮음), 옛 x는 죽음 → JOIN에서 x 제거.
- **gen**: e를 평가하며 e의 변수들을 읽으므로, 그것들은 살아있음 → 추가(`vars(e)`).

예: `x=y+z` 다음에 x가 필요하면, 이 줄 *앞*에서는 y·z가 필요(읽힘)하고 x는 곧 덮일 거라 불필요. 이 kill/gen 패턴이 강의 8의 모든 분석의 공통 골격이고, 강의 20의 reaching definition(`↓x ∪ {x=e}`)과 같은 구조. 나머지 규칙이 슬6.

---

## 슬라이드 6: Live Variable Analysis — Constraint Rules (Remaining)

### 원문 내용
> - if x: ⟦v⟧ = JOIN(v) ∪ {x}
> - entry: ⟦v⟧ = JOIN(v)
> - return: ⟦v⟧ = JOIN(v) = ∅

### 번역
> - **조건 `if x`**: x를 읽으므로 `JOIN(v) ∪ {x}` (x 추가)
> - **진입(entry)**: `JOIN(v)` 그대로
> - **반환(return)**: `JOIN(v) = ∅` (반환 후엔 아무 변수도 안 읽힘 → 후방 분석의 시작점)

### 해설

**개념 설명**

나머지 규칙: 조건 `if x`는 x를 읽으니 추가, 반환은 ∅(미래에 읽힐 게 없음 — **후방 분석의 출발점**, 강의 12 슬15의 `return: ⟦v⟧=∅`과 동일). 후방 분석은 return(끝)에서 ⊥(∅)로 시작해 앞으로 전파됩니다. 다음 분석(available expression)이 슬7~12.

---

## 슬라이드 7: Available Expression Analysis

### 원문 내용
> - A nontrivial expression (not a literal, not a variable) in a program is available at a program point if its current value has already been computed earlier in the execution
> ```c
> // nothing is available
> x = y + 1;       // y + 1 is available
> if input() {
>   y = z + 1;     // y + 1 unavailable (y changed), z + 1 available
>   ...
> } else {
>   x = z + 1;     // y + 1, z + 1 available
> }
> // z + 1 is available
> w = (z + 1) + (y + 1);
> ```

### 번역
> - **비자명한 식**(리터럴·변수 아님)이 한 지점에서 **사용 가능(available)**: 그 값이 실행 중 **이미 더 일찍 계산**되어 있을 때
> - 예: `x=y+1` 후 `y+1`이 available. 단 `y=z+1`로 y가 바뀌면 `y+1`은 더 이상 available 아님(값 변함).

### 해설

**개념 설명 — 사용 가능 식 ★**

식이 **사용 가능(available)** = "이미 계산되어 있어 **다시 계산 안 해도 되는**" 식. `x=y+1` 후 `y+1`은 available(또 쓰면 재계산 불필요). 단 `y`가 바뀌면 `y+1`의 값도 바뀌니 더 이상 available 아님(무효화).

이것은 **과거(past)** 정보(이미 계산했나?)라 **전방(forward)** 분석이고, **모든 경로에서** 계산돼 있어야 안전하므로 **must** 분석입니다 — 강의 12의 **available guard 분석**(반환 시 잡힌 락)과 정확히 같은 구조. 동기가 슬8.

---

## 슬라이드 8: Available Expression Analysis — Motivation

### 원문 내용
> - We can approximate the set of available expressions using dataflow analysis
> - Application: optimization (eliminating redundant computations)
> - We want: the answer "available" can be trusted and "not available" is safe but useless

### 번역
> - 응용: **최적화(중복 계산 제거)**
> - 목표: **"available" 판정은 신뢰**(이미 계산됨 보장), "not available"은 안전하지만 쓸모없음

### 해설

**개념 설명 — 중복 계산 제거와 must 분석**

응용: **중복 계산 제거** — 식이 확실히 available이면, 다시 계산하지 말고 저장된 값을 재사용(슬9). 그러려면 "available" 판정이 **확실**해야 합니다(틀리면 잘못된 값 재사용 → 버그). 그래서 **must 분석**: "모든 경로에서 확실히 계산됐을 때만 available"(과소근사 방향, 교집합). live variable(may)과 정반대 — **"available"이 확실, "not available"이 보수적**. 강의 1의 건전성이 must 쪽으로 적용. 응용 예가 슬9.

---

## 슬라이드 9: Available Expression Analysis — Optimization Example

### 원문 내용
> ```c
> Before:                After:
> x = y + 1;             x = y + 1;
> if input() {           if input() {
>   y = z + 1;             zplus1 = z + 1; y = zplus1;
> } else {               } else {
>   x = z + 1;             zplus1 = z + 1; x = zplus1;
> }                      }
> w = (z + 1) + (y + 1); w = zplus1 + (y + 1);
> ```

### 번역
> `z+1`이 두 분기 모두에서 계산되므로(합류 후 available), 그 값을 `zplus1`에 저장해 **마지막 `w=...`에서 재계산 없이 재사용**. 중복 계산 제거 최적화.

### 해설

**개념 설명**

`z+1`이 양쪽 분기에서 계산되니 합류 후 available — 그 값을 변수에 저장(`zplus1`)해 마지막에 재사용합니다(재계산 제거). 단 `y+1`은 then 가지에서 y가 바뀌어 무효화되므로 재사용 못 함. 컴파일러 최적화(강의 1 슬5~7)의 실례. 상태·JOIN이 슬10.

---

## 슬라이드 10: Available Expression Analysis — Abstract States

### 원문 내용
> (CFG: v1, v2 → v3; ⟦v1⟧={x+1}, ⟦v2⟧={x+1, y+1}, ⟦v3⟧={x+1})
> - State = (𝒫(Expr), ⊇) — Reverse power set lattice
> - For each CFG node v, ⟦v⟧ denotes the set of expressions available after the node
> - JOIN(v) = ⨆_{u∈pred(v)} ⟦u⟧ = ⋂_{u∈pred(v)} ⟦u⟧
>   - This combines abstract states from the predecessors

### 번역
> - 상태 = `(𝒫(Expr), ⊇)` — **역멱집합 격자**(식 집합, **역포함** 순서)
> - `⟦v⟧` = 노드 v **뒤에서** 사용 가능한 식 집합
> - **JOIN(v) = 선행자(predecessor)들의 교집합(⋂)** — 전방 분석, must이라 교집합

### 해설

**개념 설명 — 전방·must = 선행자 교집합 ★**

available expression의 구조:
- **격자 = 역멱집합 `(𝒫(Expr), ⊇)`** — 순서가 **뒤집힘**(⊇). must 분석의 전형(강의 5 슬25). 큰 집합(더 많이 available)이 더 정밀(아래쪽).
- **JOIN = 선행자(pred)들의 교집합(⋂)**:
  - **전방(forward)**: 정보가 앞에서 뒤로 → JOIN이 **선행자**.
  - **must(교집합)**: **모든** 선행 경로에서 available해야 available(교집합으로 공통만).

이 "전방+교집합"이 available expression을 4분면의 **전방-must** 칸에 놓습니다. 강의 12 슬17의 available guard(전방·must·교집합)와 동일. live variable(후방·합집합)과 정확히 쌍대. 전이 규칙이 슬11.

---

## 슬라이드 11: Available Expression Analysis — Constraint Rule (Assignment)

### 원문 내용
> - x=e: ⟦v⟧ = (JOIN(v) ∪ exprs(e)) ↓x
>   - ↓x removes all expressions containing x
>   - exprs collects all nontrivial expressions
> - exprs(x)=∅; exprs(n)=∅; exprs(input())=∅; exprs(e1 op e2) = {e1 op e2} ∪ exprs(e1) ∪ exprs(e2)

### 번역
> - **대입 `x=e`**: `⟦v⟧ = (JOIN(v) ∪ e의 부분식들) ↓x`
>   - **gen**: e를 계산하므로 e의 비자명 부분식들이 available 추가
>   - **kill (`↓x`)**: x가 바뀌므로 **x를 포함한 모든 식 제거**(값이 무효화됨)

### 해설

**개념 설명 — gen/kill (전방으로 읽기)**

available의 전이를 **앞에서 뒤로** 읽습니다:
- **gen**: e를 계산하니 e의 부분식들이 available(`exprs(e)`).
- **kill (`↓x`)**: x가 바뀌면 **x를 포함한 식**(`x+1` 등)은 값이 변해 무효 → 제거.

예: `x = x+(y+z)` 후 `y+z`는 available(계산됨), 하지만 `x+...`는 x가 바뀌어 무효. live variable의 kill/gen과 같은 골격이되 방향(전방)과 연산(교집합)이 다름. 나머지 규칙이 슬12.

---

## 슬라이드 12: Available Expression Analysis — Constraint Rules (Remaining)

### 원문 내용
> - if x: ⟦v⟧ = JOIN(v)
> - entry: ⟦v⟧ = ⊤ = ∅
> - return: ⟦v⟧ = JOIN(v)

### 번역
> - 조건: `JOIN(v)` 그대로
> - **진입(entry)**: `⊤ = ∅` (시작 시 아무 식도 계산 안 됨 — must 격자에서 ⊤=∅=가장 보수적)
> - 반환: `JOIN(v)`

### 해설

**개념 설명 — must 격자의 ⊤=∅**

진입에서 `⊤=∅`인 점이 중요합니다. **역멱집합 격자(⊇)에서 ⊤은 ∅**(강의 5 슬25) — "아무것도 available 안 함"이 가장 보수적(시작점). live variable(멱집합, ⊥=∅)과 반대. 이 ∅에서 전방으로 식이 쌓입니다. 강의 12 슬24의 재귀 바닥 요약(available 반환=LockVar=⊤)과 같은 격자 방향. 세 번째 분석(very busy)이 슬13~18.

---

## 슬라이드 13: Very Busy Expression Analysis

### 원문 내용
> - An expression is very busy if it will definitely be evaluated before its value changes
> ```c
> // nothing is very busy
> x = input();   // x + 1 is very busy
> if input() {
>   y = x + 1;   // x + 1 is very busy
> } else {
>   z = x + 1;   // x + 1 and y + 1 are very busy
>   w = y + 1;   // y + 1 is very busy
> }
> ```

### 번역
> - 식이 **매우 바쁘다(very busy)**: 그 값이 바뀌기 전에 **반드시 평가될** 것일 때(미래에 확실히 계산됨)
> - 예: `x=input()` 후 `x+1`은 very busy(양쪽 분기 모두에서 `x+1`이 곧 계산됨)

### 해설

**개념 설명 — 매우 바쁜 식 ★**

식이 **very busy** = "값이 바뀌기 전에 **반드시(모든 경로에서) 미래에 계산될**" 식. `x+1`이 then·else 양쪽에서 곧 계산되면 합류 전에 very busy. 

이것은 **미래(future)** 정보(반드시 계산될까?)라 **후방(backward)**, **모든 경로에서** 반드시이므로 **must** — 4분면의 **후방-must** 칸. (live=후방may, available=전방must와 비교.) 동기가 슬14.

---

## 슬라이드 14: Very Busy Expression Analysis — Motivation

### 원문 내용
> - Application: optimization (code hoisting)
> - We want: the answer "very busy" can be trusted and "not very busy" is safe but useless

### 번역
> - 응용: **코드 호이스팅(code hoisting)** — 반드시 계산될 식을 미리(분기 전으로) 끌어올림
> - 목표: **"very busy" 판정 신뢰**, "not very busy"는 보수적

### 해설

**개념 설명 — 코드 호이스팅**

응용: **코드 호이스팅** — 양쪽 분기에서 반드시 계산될 식(`x+1`)을 분기 *앞으로* 끌어올려 한 번만 계산(코드 크기↓). must이라 "very busy"가 확실해야 안전(아니면 안 쓸 경로에 불필요 계산). 슬15~16에서 if·while 최적화 예. 예가 슬15~16.

---

## 슬라이드 15~16: Very Busy — Optimization Examples (if/while)

### 원문 내용
> ```c
> (if) Before:           After:
> x = input();           x = input(); xplus1 = x + 1;
> if input() {           if input() {
>   y = x + 1;             y = xplus1;
> } else {               } else {
>   z = x + 1; w = y + 1;  z = xplus1; w = y + 1;
> }                      }
> ```
> ```c
> (while) Before:        After:
> x = input();           x = input(); xplus1 = x + 1;
> while input() {        while input() {
>   y = x + 1;             y = xplus1;
> }                      }
> z = x + 1;             z = xplus1;
> ```

### 번역
> `x+1`이 분기/루프 안팎에서 반드시 계산되므로, 분기·루프 **앞으로 끌어올려(hoist)** `xplus1`에 한 번 계산하고 재사용. 코드 호이스팅.

### 해설

**개념 설명**

`x+1`이 모든 경로에서 반드시 계산되니(very busy), 분기/루프 *앞*으로 끌어올려 한 번만 계산합니다. if 예는 양쪽 분기 공통 식을, while 예는 루프 안과 뒤에서 쓰이는 식을 앞으로. 코드 크기·중복 감소. 상태가 슬17.

---

## 슬라이드 17: Very Busy Expression Analysis — Abstract States

### 원문 내용
> (CFG: v1 → v2, v3; ⟦v1⟧={x+1}, ⟦v2⟧={x+1,y+1}, ⟦v3⟧={x+1})
> - State = (𝒫(Expr), ⊇) — Reverse power set lattice
> - ⟦v⟧ denotes the set of expressions very busy before the node
> - JOIN(v) = ⨆_{u∈succ(v)} ⟦u⟧ = ⋂_{u∈succ(v)} ⟦u⟧
>   - This combines abstract states from the successors

### 번역
> - 격자 = **역멱집합 (𝒫(Expr), ⊇)** (must)
> - **JOIN = 후속자들의 교집합(⋂)** — 후방·must

### 해설

**개념 설명 — 후방·must = 후속자 교집합 ★**

very busy의 구조: **역멱집합 격자(⊇, must)** + **JOIN = 후속자(succ) 교집합(⋂)**. 즉 **후방(succ)** + **must(교집합)**. available(전방·must)과 비교하면 방향만 다르고(후속자), live(후방·may)와 비교하면 연산만 다릅니다(교집합). 4분면의 마지막 칸을 채웁니다. 전이가 슬18.

---

## 슬라이드 18: Very Busy Expression Analysis — Constraint Rules

### 원문 내용
> - x=e: ⟦v⟧ = (JOIN(v)↓x) ∪ exprs(e)
> - if x: ⟦v⟧ = JOIN(v)
> - entry: ⟦v⟧ = JOIN(v)
> - return: ⟦v⟧ = ⊤ = ∅

### 번역
> - **대입 `x=e`**: `(JOIN(v)에서 x 포함 식 제거 ↓x) ∪ e의 부분식들`
> - **반환(return)**: `⊤ = ∅` (후방 분석이라 끝에서 시작, 미래에 계산될 게 없음)

### 해설

**개념 설명**

very busy 전이: x가 바뀌면 x 포함 식 무효화(`↓x`), e 계산은 부분식 추가(`exprs(e)`). available과 식은 비슷하나 방향이 후방. **반환에서 `⊤=∅`**(후방 must의 시작점). 네 분석 중 마지막. 다음은 reaching definition(슬19~22).

---

## 슬라이드 19: Reaching Definition Analysis

### 원문 내용
> - Reaching definitions for a program point are those assignments that may have defined the current values of variables
> ```c
> if input() {
>   x = y;      // x = y is a reaching definition
>   x = y + 1;  // x = y + 1 is a reaching definition
> } else {
>   x = z + 1;  // x = z + 1 is a reaching definition
> }
> // x = y + 1 and x = z + 1 are reaching definitions
> return x;
> ```

### 번역
> - **도달 정의(reaching definitions)**: 한 지점에서, 변수들의 **현재 값을 정의했을 수 있는 대입들**
> - 예: 합류 후 x의 값은 `x=y+1`(then) 또는 `x=z+1`(else)에서 옴 → 둘 다 도달 정의

### 해설

**개념 설명 — 도달 정의 ★**

**도달 정의** = "이 지점의 변수 값을 만든(정의한) 대입 문장들". `x=y+1`이 그 뒤 x를 안 덮으면 그 정의가 "도달". 합류 후엔 양쪽 분기의 마지막 정의들이 모두 도달.

이것은 **과거(past)** 정보(어느 대입이 값을 만들었나)라 **전방(forward)**, **어느 경로에서든** 도달 가능하면 도달이라 **may** — 4분면의 **전방-may** 칸. 강의 20 슬16의 reaching definition이 이것(그리고 트레이스 의미론이 필요한 이유). 동기가 슬20.

---

## 슬라이드 20: Reaching Definition Analysis — Motivation

### 원문 내용
> - Application: def-use graph (useful for optimizations)
> - We want: the answer "not reaching" can be trusted and "reaching" is safe but useless

### 번역
> - 응용: **def-use 그래프**(정의-사용 그래프, 최적화에 유용)
> - 목표: **"not reaching" 판정 신뢰**, "reaching"은 보수적(may)

### 해설

**개념 설명 — def-use 그래프**

응용: **def-use 그래프**(어느 정의가 어느 사용에 도달하나)는 상수 전파·dead code 제거 등 많은 최적화의 기반입니다. may 분석이라 "도달할 *수* 있다"를 과근사 → "not reaching"이 확실. live variable처럼 may·멱집합. 상태가 슬21.

---

## 슬라이드 21: Reaching Definition Analysis — Abstract States

### 원문 내용
> (CFG: v1, v2 → v3; ⟦v1⟧={x=y}, ⟦v2⟧={x=y+1}, ⟦v3⟧={x=y, x=y+1})
> - State = (𝒫(Def), ⊆) — Power set lattice; Def d ::= x=e
> - ⟦v⟧ denotes the set of definitions that may define values at the point after the node
> - JOIN(v) = ⨆_{u∈pred(v)} ⟦u⟧ = ⋃_{u∈pred(v)} ⟦u⟧
>   - This combines abstract states from the predecessors

### 번역
> - 격자 = **멱집합 (𝒫(Def), ⊆)** (정의 집합, may); 정의 `d ::= x=e`
> - **JOIN = 선행자들의 합집합(⋃)** — 전방·may

### 해설

**개념 설명 — 전방·may = 선행자 합집합 ★**

reaching definition: **멱집합 격자(⊆, may)** + **JOIN = 선행자(pred) 합집합(⋃)**. **전방(pred)** + **may(합집합)**. live(후방·may)와 연산은 같고(합집합) 방향이 다름(선행자). available(전방·must)과 방향은 같고(선행자) 연산이 다름(합집합). 4분면 완성. 전이가 슬22.

---

## 슬라이드 22: Reaching Definition Analysis — Constraint Rules

### 원문 내용
> - x=e: ⟦v⟧ = (JOIN(v)↓x) ∪ {x=e}
>   - ↓x removes all definitions of x
> - if x: ⟦v⟧ = JOIN(v)
> - entry: ⟦v⟧ = JOIN(v) = ∅
> - return: ⟦v⟧ = JOIN(v)

### 번역
> - **대입 `x=e`**: `(JOIN(v)에서 x의 옛 정의 제거 ↓x) ∪ {x=e}`
>   - **kill**: x를 다시 정의하므로 x의 옛 정의들 제거
>   - **gen**: 이 정의 `x=e` 추가
> - **진입**: `∅` (시작 시 정의 없음)

### 해설

**개념 설명 — kill/gen (강의 20과 연결) ★**

reaching definition의 kill/gen: x를 새로 정의하면 x의 **옛 정의를 죽이고(`↓x`)** 이 정의를 추가(`{x=e}`). 이것이 강의 20 슬16~19의 reaching definition 분석과 **정확히 동일** — 그리고 강의 20은 이 분석의 건전성을 증명하려면 **트레이스 의미론**(실행 역사)이 필요함을 보입니다(αRD가 "재정의 안 된 정의" 추적). 복잡도가 슬23.

---

## 슬라이드 23: Time Complexity

### 원문 내용
> - For SimpleWorkListAlgorithm, if |dep(v)| is bounded by a constant, the worst-case time complexity is O(n · h · k)
> - O(n · m²) where n = CFG nodes, m = number of variables/expressions/definitions
>   - Because h = m, k = O(m)

### 번역
> - 워크리스트 복잡도 **O(n·h·k)**(강의 7), 그리고 이 분석들은 **O(n·m²)** (h=m, k=O(m), m=변수/식/정의 수)

### 해설

**개념 설명**

강의 7의 O(n·h·k)를 이 분석들에 적용. 멱집합 격자의 높이 h = 원소 수 m(집합이 ∅→전체로 m번 커짐), 전이 비용 k=O(m) → **O(n·m²)**. 멱집합 분석의 표준 복잡도. 두 축 분류가 슬24~29.

---

## 슬라이드 24: Forward vs Backward Analyses

### 원문 내용
> - A forward analysis computes information about the past behavior
>   - Examples: sign, constant propagation, available expression, reaching definition
>   - Starts at the entry node, propagates forward; JOIN uses pred; dep = succ
> - A backward analysis computes information about the future behavior
>   - Examples: live variables, very busy expressions
>   - Starts at the return node, propagates backward; JOIN uses succ; dep = pred

### 번역
> - **전방(forward) 분석**: **과거** 동작 정보. 진입에서 시작, 앞으로 전파. **JOIN=선행자(pred), dep=후속자(succ)**. (부호·상수전파·available·reaching)
> - **후방(backward) 분석**: **미래** 동작 정보. 반환에서 시작, 뒤로 전파. **JOIN=후속자(succ), dep=선행자(pred)**. (live·very busy)

### 해설

**개념 설명 — 첫 번째 축: 전방/후방 ★**

분류의 첫 축. **전방**은 "과거"(이미 일어난 일 — 어느 정의가 값을 만들었나, 어떤 식이 계산됐나)를 진입에서 앞으로 추적. **후방**은 "미래"(앞으로 일어날 일 — 나중에 읽힐까, 반드시 계산될까)를 반환에서 뒤로 추적. JOIN과 dep의 방향이 정반대(전방: JOIN=pred, dep=succ / 후방: JOIN=succ, dep=pred). 강의 7 슬30의 dep=succ가 전방이었던 이유. 두 번째 축이 슬25.

---

## 슬라이드 25: May vs Must Analyses

### 원문 내용
> - A may analysis describes information that may possibly be true
>   - Examples: live variables, reaching definitions; Typically uses a power set lattice
> - A must analysis describes information that must definitely be true
>   - Examples: available expression, very busy expression; Typically uses a reverse power set lattice

### 번역
> - **may 분석**: **가능한(possibly)** 정보. 멱집합 격자(⊆, join=합집합). (live·reaching)
> - **must 분석**: **반드시(definitely)** 참인 정보. 역멱집합 격자(⊇, join=교집합). (available·very busy)

### 해설

**개념 설명 — 두 번째 축: may/must ★**

분류의 둘째 축. **may**는 "가능성"(어느 경로에서든 성립하면) → **멱집합(⊆), 합집합**. **must**는 "필연"(모든 경로에서 성립해야) → **역멱집합(⊇), 교집합**. 이 대응이 핵심:
- **may = 멱집합 = 합집합 = ⊆**,
- **must = 역멱집합 = 교집합 = ⊇**.

강의 12의 live guard(may·합집합)·available guard(must·교집합), 강의 13의 R(may)·W(must)가 모두 이 대응. 건전성 측면이 슬26~28.

---

## 슬라이드 26: May vs Must Analyses — Soundness

### 원문 내용
> - May ≠ Sound, Must ≠ Complete
> - All these analyses are sound but not complete

### 번역
> - **may ≠ 건전, must ≠ 완전** (혼동 주의!)
> - 이 분석들은 모두 **건전하되 불완전**(강의 1)

### 해설

**개념 설명 — may/must와 건전/완전은 다르다 ★**

흔한 오해를 바로잡습니다: **may/must는 "무엇을 추적하나"(가능/필연)이고, sound/complete는 "근사 방향"(과근사/과소근사)**으로 별개입니다. **네 분석 모두 건전(sound)하되 불완전(incomplete)**입니다 — may든 must든 정적 분석은 건전성을 목표로 함(강의 1). 왜 may도 must도 건전한지가 슬27~28의 핵심(헷갈리기 쉬운 부분). live·available 사례로 설명(슬27~28).

---

## 슬라이드 27: May vs Must — Soundness (Live Variables)

### 원문 내용
> - Live variables = {x}
>   - Set of possible behavior: any execution that does not require any variable other than x to be live
>     - Can have false positives (some such executions may be actually impossible)
>   - Set of impossible behavior: any execution that requires some variable other than x to be live
>     - No false negatives (such executions are indeed impossible)

### 번역
> - live={x}일 때: "x 외엔 살아있을 필요 없는 실행"(가능 동작)을 추정 — **거짓 양성 가능**(실제론 불가능한 실행 포함 가능). "x 외 변수가 살아야 하는 실행"(불가능 동작) — **거짓 음성 없음**(그런 실행은 진짜 불가능).

### 해설

**개념 설명 — may 분석의 건전성**

live variable(may)이 "live={x}"라 하면: **"x만 살아있으면 충분한 실행"을 (보수적으로) 본다** — 실제론 더 많은 변수가 필요할 수 있어 이쪽으로 **과근사**(거짓 양성 가능). 하지만 **"live가 아닌 변수"(not live) 판정은 확실**(그 변수는 진짜 안 쓰임 = 거짓 음성 없음). 즉 may 분석의 "not in the set"이 신뢰됨 = 건전. 슬3의 "not live는 신뢰"가 이것. available은 반대 방향(슬28).

---

## 슬라이드 28: May vs Must — Soundness (Available Expressions)

### 원문 내용
> - Available expressions = {x+y}
>   - Set of possible behavior: any execution that has already computed x+y
>     - Can have false positives (some such executions may be actually impossible)
>   - Set of impossible behavior: any execution that has not computed x+y
>     - No false negatives (such executions are indeed impossible)

### 번역
> - available={x+y}일 때: "x+y를 이미 계산한 실행"을 추정 — 거짓 양성 가능. "x+y를 계산 안 한 실행"(불가능) — 거짓 음성 없음.

### 해설

**개념 설명 — must 분석도 건전**

available(must)이 "available={x+y}"라 하면: "x+y가 이미 계산된 실행"만 본다 — 안전을 위해 **확실히 계산된 것만** 포함(과소근사 방향이지만 건전). "available"이라 한 건 진짜 계산됨(신뢰). 즉 **must 분석의 "in the set"이 신뢰됨 = 건전**. may든 must든 "신뢰할 수 있는 쪽"이 다르지만 둘 다 건전. 4분면 표가 슬29.

---

## 슬라이드 29: Classification of Dataflow Analyses

### 원문 내용
> | | Forward | Backward |
> |---|---|---|
> | May | Reaching definition analysis | Live variable analysis |
> | Must | Available expression analysis | Very busy expression analysis |

### 번역
> **데이터플로우 분석 4분면 분류표**:
> | | 전방(forward) | 후방(backward) |
> |---|---|---|
> | **may** | 도달 정의 | 살아있는 변수 |
> | **must** | 사용 가능 식 | 매우 바쁜 식 |

### 해설

**개념 설명 — 4분면 표 (이 강의의 핵심 정리) ★★**

데이터플로우 분석의 가장 유명한 정리(整理)입니다. 두 축(전방/후방 × may/must)으로 네 고전 분석이 깔끔히 분류됩니다:
- **전방-may**: 도달 정의(어느 정의가 값을 만들었나, 과거·가능).
- **전방-must**: 사용 가능 식(어떤 식이 계산됐나, 과거·필연).
- **후방-may**: 살아있는 변수(나중에 읽히나, 미래·가능).
- **후방-must**: 매우 바쁜 식(반드시 계산되나, 미래·필연).

각 칸이 (JOIN 방향, 격자, 연산)을 결정합니다: 전방→pred/후방→succ, may→멱집합·합집합/must→역멱집합·교집합. **강의 12의 두 guard 분석이 이 표의 대각선(live=후방may, available=전방must)을 차지**합니다(강의 12 슬17의 4분면 표). 추가 분석 예가 슬30~31.

---

## 슬라이드 30~31: Example — Initialized Variable Analysis

### 원문 내용
> ```c
> if input() { x = 1; y = x + 1; } else { y = 2; }
> z = y + x;
> ```
> - We want to know whether a certain variable is definitely initialized at a program point
>   - Must analysis — State = (𝒫(Var), ⊇)
> - Initialization is a property of past — Forward analysis — JOIN(v) = ⨆_{u∈pred(v)} ⟦u⟧
> - x=e: ⟦v⟧ = JOIN(v) ∪ {x}; entry: ⟦v⟧ = ∅; ...

### 번역
> **초기화된 변수 분석**: 어떤 변수가 한 지점에서 **반드시 초기화됐는지** 판정.
> - **must**(반드시) → 역멱집합 (𝒫(Var),⊇); **전방**(과거 속성) → JOIN=선행자
> - 위 코드에서 합류 후 y는 반드시 초기화(양쪽 다), x는 then만 → x는 미초기화 가능.

### 해설

**개념 설명 — 4분면으로 새 분석 설계하기 ★**

새 분석(초기화된 변수)을 4분면으로 **설계**하는 연습입니다. "변수가 **반드시** 초기화됐나" → **must**(역멱집합·교집합). "초기화는 **과거** 속성" → **전방**(JOIN=pred). 따라서 **전방-must**(available과 같은 칸). 미초기화 변수 사용(버그)을 잡는 데 유용. 이처럼 **"무엇을 묻나"로 4분면 위치가 정해지고, 그것이 격자·JOIN·전이를 결정**합니다 — 분석 설계의 강력한 가이드. 전이 함수 추상화가 슬32.

---

## 슬라이드 32: Transfer Functions

### 원문 내용
> - All constraint functions are of the form ⟦v⟧ = t_v(JOIN(v)) where t_v : L → L
> - Example: live variable analysis
>   - x=e: ⟦v⟧ = JOIN(v) \ {x} ∪ vars(e)
>   - if x: ⟦v⟧ = JOIN(v) ∪ {x}
>   - entry/return: ⟦v⟧ = JOIN(v)

### 번역
> - 모든 제약 함수는 **`⟦v⟧ = t_v(JOIN(v))`** 형태 — `t_v`가 노드 v의 **전이 함수(transfer function)**
> - 즉 "JOIN으로 합친 뒤 t_v를 적용"

### 해설

**개념 설명 — 전이 함수 t_v ★**

모든 분석이 **`⟦v⟧ = t_v(JOIN(v))`** 형태로 통일됩니다: ① JOIN으로 합치고 → ② 노드별 전이 함수 t_v 적용. live의 `\{x}∪vars(e)`, available의 `(...∪exprs(e))↓x` 등이 모두 t_v. 이 추상화로 알고리즘을 통일·효율화합니다(슬34~). 강의 18~19의 전이 함수, Assignment 4의 transfer_stmt/transfer_term이 이것. 위치 설명이 슬33.

---

## 슬라이드 33: Transfer Functions (cont.)

### 원문 내용
> - t_v is called a transfer function for the CFG node
>   - Forward: input represents the state immediately before the node, output the state immediately after
>   - Backward: input represents the state immediately after, output the state immediately before
> - Example: t_{x=e}(s) = s \ {x} ∪ vars(e)

### 번역
> - **전이 함수 t_v**: 전방은 "노드 직전 상태→직후 상태", 후방은 "직후 상태→직전 상태"

### 해설

**개념 설명**

전이 함수의 의미: 전방은 노드를 "지나면서" 상태가 어떻게 변하나(직전→직후), 후방은 거꾸로(직후→직전). 같은 t_v 형식이지만 방향에 따라 입출력이 뒤바뀜. 이 통일된 형식이 효율적 알고리즘의 토대(슬34~39). 효율화 동기가 슬34.

---

## 슬라이드 34: Transfer Functions — Redundancy in SimpleWorkListAlgorithm

### 원문 내용
> - In SimpleWorkListAlgorithm, JOIN(v) = ⨆⟦u⟧ is computed in each iteration
> - However, ⟦u⟧ often has not changed, so much of the computation is redundant
> - We can use transfer functions to avoid redundancy
> - Now, x_i = ⟦v_i⟧ is the state before v_i in forward analyses, and the state after v_i in backward analyses

### 번역
> - 단순 워크리스트는 매번 JOIN(전체 선행자 합치기)을 다시 계산 → 대부분 안 바뀌어 중복
> - **전이 함수로 중복 제거**: 변화를 점진적으로 전파

### 해설

**개념 설명 — JOIN 재계산의 중복**

단순 워크리스트(강의 7 슬32)는 노드를 처리할 때마다 JOIN(모든 선행자 합치기)을 다시 합니다 — 대부분 선행자는 안 바뀌었는데도. 이 중복을 **전이 함수의 결과를 점진적으로 전파**해 제거합니다(슬38). 상태 위치 그림이 슬35~37, 개선 알고리즘이 슬38.

---

## 슬라이드 35~37: Transfer Functions — State Positions

### 원문 내용
> (전방·후방·합류에서 전이 함수 적용 전후 상태 위치를 보여주는 그림)
> - Forward: w/o transfer functions 상태가 노드 "뒤"에, w/ transfer functions 상태가 노드 "앞"에
> - Join: 전이 함수를 쓰면 각 선행자에서 t를 적용한 뒤 합류

### 번역
> 전이 함수 사용 시, 상태를 "노드 사이(간선)"에 두어, 각 노드에서 t를 적용한 결과를 후속/선행으로 전파하는 구조를 그림으로 설명.

### 해설

**개념 설명**

전이 함수를 쓰면 상태의 "위치"가 노드에서 **간선(노드 사이)**로 옮겨갑니다 — "노드 i의 상태에 t_i를 적용한 결과를 다음 노드로 전파". 합류(슬37)에서는 각 선행자에서 t를 적용한 뒤 join. 이로써 JOIN을 매번 재계산하지 않고 변화만 흘려보냅니다. 알고리즘이 슬38.

---

## 슬라이드 38: PropagationWorkListAlgorithm

### 원문 내용
> ```
> PropagationWorkListAlgorithm(t1, ..., tn, s_start):
>   (x1, ..., xn) ← (s_start, ⊥, ..., ⊥)
>   W ← {v1, ..., vn}
>   while W ≠ ∅:
>     vi ← W.removeOne()
>     y ← t_{vi}(xi)
>     for vj ∈ dep(vi):
>       z ← xj ⊔ y
>       if xj ≠ z:
>         xj ← z
>         W.add(vj)
>   return x
> ```

### 번역
> **전파 워크리스트 알고리즘**: 각 노드에서 전이 함수 t를 적용한 결과 y를, 의존 노드(dep)들에 **⊔로 흘려보냄**. 변하면 갱신하고 워크리스트에 추가. (JOIN 전체 재계산 없이 점진적 전파)

### 해설

**개념 설명 — 전파 워크리스트 (효율적 표준) ★**

단순 워크리스트(JOIN 재계산)를 개선한 표준 알고리즘입니다: 노드 vi를 처리할 때 **t_{vi}(xi)를 계산해 그 결과를 의존 노드들에 ⊔로 직접 전파**(JOIN 전체를 다시 안 함). 강의 11 cubic의 Propagate, Assignment 4의 전파가 이 구조와 동형. JOIN 중복을 제거해 더 효율적. 정당성이 슬39.

---

## 슬라이드 39: PropagationWorkListAlgorithm — Intuition

### 원문 내용
> - This gives the same analysis results
> - Intuition:
>   - SimpleWorkListAlgorithm computes x3 = t1(x1) ⊔ t2(x2)
>   - PropagationWorkListAlgorithm computes x3 = x3 ⊔ t1(x1) and x3 = x3 ⊔ t2(x2)
>   - If f is monotone and g(x) = f(x) ⊔ x, then lfp(g) = lfp(f)

### 번역
> - **같은 결과**를 줌
> - 직관: 단순 알고리즘은 `x3 = t1(x1)⊔t2(x2)`(한 번에 합침), 전파 알고리즘은 `x3 = x3⊔t1(x1)`, `x3 = x3⊔t2(x2)`(점진적으로 합침). 단조 f에 대해 `g(x)=f(x)⊔x`면 `lfp(g)=lfp(f)`(강의 6 슬19).

### 해설

**개념 설명 — 점진적 합치기가 같은 lfp를 줌**

전파 워크리스트가 단순 워크리스트와 **같은 lfp**를 산출함을 보입니다: 한꺼번에 join하든(`t1⊔t2`) 점진적으로 join하든(`x3⊔t1`, `x3⊔t2`) 결과 같음. 근거는 강의 6 슬19의 `x⊒f(x) ⟺ x=x⊔f(x)`(부등식↔등식). 즉 전파 방식은 부등식 제약을 푸는 것과 같고, 단조성 덕에 같은 고정점. 효율은 좋고 정답은 같음. 전체 요약이 슬40.

---

## 슬라이드 40: Summary

### 원문 내용
> - Live variable analysis: which variables may be needed in the future (backward, may)
> - Available expression analysis: which expressions have already been computed (forward, must)
> - Very busy expression analysis: which expressions will definitely be evaluated (backward, must)
> - Reaching definition analysis: which assignments may define current values (forward, may)
> - Dataflow analyses are classified along two axes: forward/backward and may/must
> - PropagationWorkListAlgorithm avoids redundant JOIN recomputation by propagating transfer function results incrementally

### 번역
> - **live**(후방·may), **available**(전방·must), **very busy**(후방·must), **reaching**(전방·may)
> - 두 축(전방/후방 × may/must)으로 분류
> - **전파 워크리스트**가 JOIN 중복을 점진 전파로 제거

### 해설

**전체 정리 — 강의 8의 한 장 요약**

1. **네 고전 분석**: live(나중에 읽히나)·available(이미 계산됐나)·very busy(반드시 계산되나)·reaching(어느 정의가 값을 만들었나). 각각 응용(레지스터·중복제거·호이스팅·def-use).
2. **4분면 분류**: (전방/후방)×(may/must). 전방=과거·JOIN(pred)·dep(succ), 후방=미래·JOIN(succ)·dep(pred). **may=멱집합·합집합·⊆, must=역멱집합·교집합·⊇**. 모두 건전하되 불완전.
3. **설계 가이드**: "무엇을 묻나"(과거/미래, 가능/필연)로 4분면 위치가 정해지고, 그것이 격자·JOIN·전이를 결정(슬30 초기화 분석).
4. **효율**: 전이 함수 t_v 통일, 전파 워크리스트로 JOIN 중복 제거(같은 lfp).

**다른 강의와의 연결 (파일 간 연결성)**

- ← **강의 5**: 멱집합(may)·역멱집합(must) 격자, ⊆/⊇, ⊥/⊤.
- ← **강의 6~7**: 고정점·워크리스트·전이 함수·O(n·h·k).
- → **강의 12 (락 응용)**: **live guard = live variable(후방·may), available guard = available expression(전방·must)**. 강의 12 슬17의 4분면 표가 이 강의에서 옴.
- → **강의 13 (출력 매개변수)**: R(읽기집합, may·⊆) vs W(쓰기집합, must·⊇)가 이 may/must 대응.
- → **강의 20 (트레이스 의미론)**: reaching definition(전방·may)의 건전성 증명이 왜 트레이스 의미론을 요구하는지(αRD).
- → **강의 18~19 (추상 해석)**: 전이 함수·JOIN·lfp·건전성이 추상 해석으로 일반화.

**가장 큰 교훈**: **하나의 단조 프레임워크가 네 고전 분석을 모두 만들고, 그들은 (전방/후방)×(may/must)의 4분면으로 분류된다.** 핵심 대응은 **may=멱집합·합집합, must=역멱집합·교집합**, **전방=과거·선행자JOIN, 후방=미래·후속자JOIN**. 이 4분면은 새 분석을 설계하는 가이드("무엇을 묻나→어느 칸→격자·JOIN·전이")이자, 강의 12의 guard 분석·강의 20의 reaching definition을 이해하는 열쇠입니다. **may/must는 추적 대상(가능/필연)이지 근사 방향(건전/완전)이 아니다** — 넷 다 건전합니다.

---

## 마치며

강의 8은 데이터플로우 분석의 **4분면 분류**라는 가장 유명한 정리를 제시합니다. 핵심 한 줄: **"live(후방may)·available(전방must)·very busy(후방must)·reaching(전방may) 네 분석은 (전방/후방)×(may/must)로 분류되며, may는 멱집합·합집합, must는 역멱집합·교집합, 전방은 선행자 JOIN, 후방은 후속자 JOIN을 쓴다."** 이 표는 강의 12의 두 guard 분석이 어디서 왔는지, 강의 20의 reaching definition이 왜 트레이스 의미론을 요구하는지를 설명합니다. 시험에서는 (a) 네 분석 각각의 정의·응용·격자·JOIN·전이 규칙(슬2~22), (b) 4분면 분류와 각 칸의 특성(전방/후방·may/must·격자·연산, 슬24~29), (c) 새 분석을 4분면으로 설계하기(초기화 변수, 슬30~31), (d) may/must와 sound/complete의 구분(넷 다 건전, 슬25~28), (e) 전이 함수와 전파 워크리스트(슬32~39)가 단골입니다.
