# Dataflow Analysis (1) - 강의 슬라이드 해설

CSE552 Program Analysis — Lecture 7
강사: Jaemin Hong

> **이 파일을 읽는 법**: 각 슬라이드는 `원문 내용` → `번역` → `해설`(개념·배경·맥락) → 필요 시 `각주`/`슬라이드 연결` 순서입니다. 누락·왜곡 없이 원문을 모두 담되 처음 보는 사람도 따라올 수 있게 풀어 썼습니다.

---

## 강의 7 전체 조감도 (먼저 큰 그림)

강의 5(격자)·6(고정점)은 "분석은 격자 위 단조 함수의 최소 고정점"이라는 이론을 세웠습니다. 강의 7은 그 이론을 **실제 분석 프레임워크 — 데이터플로우 분석(dataflow analysis)** 으로 구체화합니다. 한 문장으로: **CFG의 각 노드에 격자 변수를 두고, 노드 간 관계를 단조 제약식으로 적은 뒤, 고정점 알고리즘으로 푼다.**

흐름:
1. **단조 프레임워크(monotone framework)** (슬라이드 2~4): CFG + 완비 격자 + 단조 전이 함수의 조합. 분석을 만드는 일반 틀.
2. **부호 분석을 데이터플로우로** (슬라이드 5~18): 격자(Var→Sign), **JOIN(선행자 합치기)**, 전이 함수(eval·추상 연산), 단조성, 고정점 풀이. 강의 5~6을 CFG 위에서 실현.
3. **상수 전파(constant propagation)** (슬라이드 19~22): 같은 틀에 다른 격자(flat(ℤ))를 끼운 또 하나의 분석. 컴파일러 최적화 응용.
4. **효율적 고정점 알고리즘** (슬라이드 23~34): 단순 반복 → Round Robin → Chaotic Iteration → **워크리스트(worklist)**. "바뀐 것만 다시 계산"으로 O(n·h·k) 달성.

핵심 통찰: **데이터플로우 분석 = "격자 + JOIN + 전이 함수"를 CFG에 끼우고 고정점을 푸는 일반 프레임워크.** 부호·상수전파는 같은 틀의 다른 인스턴스일 뿐. 그리고 **워크리스트 알고리즘**은 Kleene 반복(강의 6)을 "의존성 기반으로 바뀐 노드만 재계산"해 효율화한 것 — Assignment 4와 강의 11의 cubic이 모두 이 워크리스트입니다.

---

## 슬라이드 1: 제목 슬라이드

### 원문 내용
> Dataflow Analysis (1)
> CSE552 Program Analysis — Lecture 7
> Jaemin Hong

### 번역
> 데이터플로우 분석 (1) / CSE552 프로그램 분석 — 강의 7 / 홍재민

### 해설
강의 5~6의 격자·고정점 이론을 **CFG 위의 데이터플로우 분석**으로 구체화하는 강의입니다.

---

## 슬라이드 2: Dataflow Analysis Overview

### 원문 내용
> - Starts with a CFG and a complete lattice (with a finite height)
>   - A lattice element represents abstract information for a CFG node
>   - The lattice may be: fixed for all programs, or parameterized by the program
> - To each node v, we assign a constraint variable ⟦v⟧ ranging over the lattice elements

### 번역
> - **CFG**와 **완비 격자(유한 높이)**에서 출발
>   - 격자 원소 = 한 CFG 노드의 추상 정보
>   - 격자는 모든 프로그램에 고정이거나, 프로그램에 따라 매개변수화됨
> - 각 노드 v에 격자 원소를 값으로 갖는 **제약 변수 ⟦v⟧**를 부여

### 해설

**개념 설명 — 데이터플로우 분석의 뼈대 ★**

데이터플로우 분석의 구성 요소:
1. **CFG**(강의 2의 MIR): 노드(기본 블록/문장)와 간선(제어 흐름).
2. **완비 격자**(강의 5): 추상 정보의 도메인(부호·구간 등). **유한 높이**라야 종료(강의 6 Kleene).
3. **각 노드에 변수 ⟦v⟧**: "노드 v에서의 추상 상태".

즉 "각 프로그램 지점의 추상 상태를 격자 변수로 두고, 그 값들을 푼다". 강의 6 슬3의 `(Var→Sign)³`이 정확히 이것. 제약식과 고정점이 슬3.

---

## 슬라이드 3: Dataflow Constraints and Fixed Points

### 원문 내용
> - For each node, we define a dataflow constraint that relates the variable to those of other nodes
> - If all constraints are (in-) equations with monotone right-hand sides, we can use the fixed-point algorithm

### 번역
> - 각 노드마다 **데이터플로우 제약식**을 정의 — 그 변수를 다른 노드 변수들과 연결
> - 모든 제약이 **단조 우변을 가진 (부)등식**이면, **고정점 알고리즘**을 쓸 수 있다

### 해설

**개념 설명 — 제약 = 단조 방정식 = 고정점**

각 노드의 제약식 `⟦v⟧ = (다른 노드 변수들의 함수)`가 강의 6의 `x=f(x)`를 이룹니다. **우변이 단조**이면(슬14) 고정점 알고리즘(강의 6 슬16)으로 풀 수 있습니다. 부등식이면 등식으로 변환(강의 6 슬19). 이 "단조 제약 → 고정점"이 데이터플로우의 핵심. 이 일반 틀이 단조 프레임워크(슬4).

---

## 슬라이드 4: Monotone Framework

### 원문 내용
> - The combination of a complete lattice and a space of monotone functions
> - Can be instantiated by specifying the CFG and the rules for assigning dataflow constraints

### 번역
> - **단조 프레임워크(monotone framework)**: 완비 격자 + 단조 함수 공간의 조합
> - CFG와 데이터플로우 제약 규칙을 지정해 **인스턴스화**할 수 있음

### 해설

**개념 설명 — 단조 프레임워크 = 분석 제조 틀 ★**

**단조 프레임워크**(Kam-Ullman, 강의 5 슬7)는 분석을 찍어내는 **일반 틀**입니다. "완비 격자 + 단조 전이 함수"라는 빈 틀에, **격자와 제약 규칙만 채우면** 구체 분석이 됩니다:
- 격자=Sign, 규칙=부호 전이 → **부호 분석**(슬5~18).
- 격자=flat(ℤ) → **상수 전파**(슬19~22).
- 격자=멱집합 → **도달 정의·사용 가능 식**(강의 8).

즉 **하나의 틀로 수많은 분석**을 만듭니다. 강의 1 슬30의 "분석 설계 프레임워크"가 이것. 부호 분석으로 인스턴스화를 시작합니다(슬5~).

---

## 슬라이드 5: Syntax

### 원문 내용
> - Node v ::= x=e | if x | entry | return
> - Expression e ::= x | n | input() | e op e
> - (No functions, pointers, and compound types for now)

### 번역
> - 노드: 대입(`x=e`)·조건(`if x`)·진입(entry)·반환(return)
> - 식: 변수·정수·`input()`·이항 연산
> - (지금은 함수·포인터·복합 타입 없음)

### 해설

**개념 설명**

분석 대상 미니 언어. CFG 노드는 대입·조건·진입·반환 네 종류. 강의 2의 MIR을 단순화한 것(함수·포인터는 강의 10·14에서). 이 위에 부호 분석을 정의합니다(슬6~). 격자가 슬6.

---

## 슬라이드 6: Sign Analysis — Lattice

### 원문 내용
> (부호 격자 다이아몬드: ⊤ / −, 0, + / ⊥)
> - ⟦v⟧ ∈ State = Var → Sign
> - State^n expresses information for all CFG nodes, where n is the number of nodes

### 번역
> - 부호 격자(강의 5): {⊥, −, 0, +, ⊤}
> - 각 노드 상태 `⟦v⟧ ∈ State = Var → Sign` (변수→부호, 맵 격자)
> - **State^n**이 모든 CFG 노드의 정보(n=노드 수)

### 해설

**개념 설명**

강의 5의 부호 격자(flat)와 맵 격자(`Var→Sign`)를 그대로 씁니다. 한 노드 상태는 "각 변수의 부호", 전체는 그 곱 `State^n`(강의 5 슬30, 강의 6 슬6과 동일). 노드 간 정보를 합치는 JOIN이 슬7.

---

## 슬라이드 7: Sign Analysis — JOIN

### 원문 내용
> - JOIN(v) combines the abstract states from the predecessors of v: JOIN(v) = ⨆_{u∈pred(v)} ⟦u⟧
> - Precisely speaking, JOIN(v) is a function: JOIN(v) : State^n → State; JOIN(v)(⟦v1⟧, ..., ⟦vn⟧) = ⟦vi⟧ ⊔ ⟦vj⟧ ⊔ ...

### 번역
> - **JOIN(v)**: v의 **선행자(predecessor)들의 추상 상태를 합침(⊔)**: `JOIN(v) = ⨆_{u∈pred(v)} ⟦u⟧`
> - 엄밀히는 전체 상태를 받아 v의 합류 상태를 주는 함수

### 해설

**개념 설명 — JOIN = 선행자 합치기 (전방 분석) ★**

**JOIN(v)**는 "v로 들어오는 모든 경로의 정보를 합치는" 연산입니다 — v의 **선행자(predecessor)** 상태들을 **join(⊔)**으로 모읍니다(강의 5 슬16의 "분기 합류에서 lub"). 부호 분석은 **전방(forward)** 분석이라 선행자에서 정보가 흘러옵니다. 이것이 강의 18 슬7의 CJOIN(구체 버전), 강의 12·15의 JOIN과 같은 연산. 예가 슬8.

---

## 슬라이드 8: Sign Analysis — JOIN Example

### 원문 내용
> (CFG: v1 entry → v2 if x → [T] v3 y=z, [F] v4 y=−5 → v5 return)
> - JOIN(v1) = ⊥
> - JOIN(v2) = ⟦v1⟧
> - JOIN(v3) = ⟦v2⟧
> - JOIN(v4) = ⟦v2⟧
> - JOIN(v5) = ⟦v3⟧ ⊔ ⟦v4⟧

### 번역
> 분기 CFG에서 각 노드의 JOIN: 진입은 ⊥(선행자 없음), 단일 선행자 노드는 그 선행자 상태, **합류 노드 v5는 두 분기(v3, v4)를 ⊔로 합침**.

### 해설

**개념 설명**

JOIN 계산 예. v5는 then(v3)·else(v4) 두 경로가 합류하므로 `⟦v3⟧⊔⟦v4⟧` — 양쪽 정보를 안전하게 합침(강의 5 슬5의 c=⊤). 진입 v1은 선행자가 없어 ⊥(이후 entry 규칙으로 ⊤, 슬11). 단일 선행자는 그대로 전달. JOIN 후 각 노드의 전이가 슬9.

---

## 슬라이드 9: Sign Analysis — Constraint Rules

### 원문 내용
> - x=e: ⟦v⟧ = JOIN(v)[x ↦ eval(JOIN(v), e)]
> - eval : (Var → Sign) × Expression → Sign
>   - eval(σ, x) = σ(x); eval(σ, n) = sign(n); eval(σ, input()) = ⊤; eval(σ, e1 op e2) = op̂(eval(σ,e1), eval(σ,e2))

### 번역
> - **대입 `x=e`**: `⟦v⟧ = JOIN(v)에서 x를 eval 결과로 갱신` (`JOIN(v)[x↦eval(JOIN(v),e)]`)
> - **eval**(식의 추상 평가): 변수→그 부호, 정수→sign(n), `input()`→⊤(모름), 이항 연산→추상 연산자 op̂

### 해설

**개념 설명 — 전이 함수: JOIN 후 갱신 ★**

대입 노드의 전이: **① 선행자 정보를 JOIN으로 합치고 → ② 그 상태에서 식 e를 평가(eval)해 → ③ x를 그 값으로 갱신**. `eval`은 식의 부호를 계산(변수는 현재 부호, 상수는 그 부호, input은 ⊤). 이항 연산은 **추상 연산자 op̂**(슬10). 이 "JOIN → eval → 갱신" 패턴이 강의 18 슬8의 구체 전이, Assignment 4의 transfer_stmt와 동일 구조. 추상 덧셈표가 슬10.

---

## 슬라이드 10: Sign Analysis — Abstract Addition

### 원문 내용
> (덧셈표 +̂: 행·열이 ⊥,−,0,+,⊤)
> - ⊥ +̂ x = ⊥ (모두)
> - − +̂ − = −, − +̂ 0 = −, − +̂ + = ⊤, − +̂ ⊤ = ⊤
> - 0 +̂ x = x
> - + +̂ + = +, + +̂ − = ⊤
> - ⊤ +̂ x = ⊤ (단 ⊥ 제외)

### 번역
> **추상 덧셈 +̂** 표: ⊥은 흡수(⊥+x=⊥), 0은 항등(0+x=x), 같은 부호 덧셈은 그 부호(−+−=−), **다른 부호(+와 −)는 ⊤**(미정), ⊤은 전파.

### 해설

**개념 설명 — 추상 연산자 ★**

추상 덧셈 `+̂`는 부호끼리의 덧셈 결과를 정의합니다. 핵심:
- `0 +̂ x = x`(0은 항등), 같은 부호는 유지(`+ +̂ + = +`),
- **`+ +̂ − = ⊤`**(양수+음수는 부호 미정 — 강의 19 문제 19-2의 그 건전성),
- `⊥`(불가능)은 흡수.

이 표가 **건전한 추상 연산**(강의 19 슬7의 `+̂` 건전성 조건을 만족)이어야 분석이 건전합니다. 강의 18 슬18의 부호 덧셈표와 동일. 나머지 규칙이 슬11.

---

## 슬라이드 11: Sign Analysis — Remaining Rules

### 원문 내용
> - entry: ⟦v⟧ = ⊤
> - Others: ⟦v⟧ = JOIN(v)
> - While ⊓ exists, we only use ⊔ (This is common)

### 번역
> - **진입(entry)**: `⟦v⟧ = ⊤` (모든 변수 부호 미정 — 아무 입력이나 가능)
> - **그 외 노드**(if·return 등): `⟦v⟧ = JOIN(v)` (그냥 합류)
> - meet(⊓)도 있지만 **join(⊔)만** 사용 (흔한 경우)

### 해설

**개념 설명**

나머지 노드 규칙: **진입은 ⊤**(시작 시 모든 변수가 임의값 가능 — 강의 18 슬14의 진입 규칙), 그 외는 JOIN만(대입 아닌 노드는 상태를 안 바꿈). "join만 쓴다"는 데이터플로우의 흔한 특징 — 정보를 합치기만(전방 may 분석). 전체 예가 슬12~13.

---

## 슬라이드 12~13: Sign Analysis — Constraint Example

### 원문 내용
> ```c
> a = 42; b = 87;
> if x { c = a + b; } else { c = a - b; }
> return;
> ```
> Constraints (CFG v1~v7):
> - ⟦v1⟧ = ⊤
> - ⟦v2⟧ = ⟦v1⟧[a ↦ +]
> - ⟦v3⟧ = ⟦v2⟧[b ↦ +]
> - ⟦v4⟧ = ⟦v3⟧
> - ⟦v5⟧ = ⟦v4⟧[c ↦ +̂(⟦v4⟧(a), ⟦v4⟧(b))]
> - ⟦v6⟧ = ⟦v4⟧[c ↦ −̂(⟦v4⟧(a), ⟦v4⟧(b))]
> - ⟦v7⟧ = ⟦v5⟧ ⊔ ⟦v6⟧

### 번역
> 강의 5 슬5의 코드를 CFG·제약식으로. 각 노드가 이전 상태에서 그 줄의 효과를 적용(a=42→a↦+, c=a+b→c↦+̂(...)), 합류 v7은 두 분기를 ⊔.

### 해설

**개념 설명 — 분석의 전체 제약 시스템**

강의 5~6의 예제를 완전한 데이터플로우 제약으로 적었습니다. 각 노드 제약이 강의 6 슬6의 함수 f를 이루고, 고정점이 분석 결과. v7에서 `⟦v5⟧⊔⟦v6⟧`로 then(c=+)·else(c=⊤, 양수−양수) 합류 → c=⊤(강의 5 슬5와 일치). 이 제약을 고정점으로 풉니다(슬16). 그 전에 단조성 확인(슬14).

---

## 슬라이드 14: Monotonicity

### 원문 내용
> - Function composition preserves monotonicity
> - ⊔ is monotone
> - Map update is monotone
> - op̂ is monotone
> - eval(_, e) : (Var → Sign) → Sign is monotone for every e

### 번역
> 단조성 확인: 합성·⊔·맵 갱신·추상 연산자 op̂가 모두 단조(강의 6 슬9~10) → 따라서 **eval과 전체 전이 함수가 단조**.

### 해설

**개념 설명 — 전이 함수가 단조임을 조각별로 ★**

강의 6 슬9~10의 단조성 성질로, 전이 함수의 단조성을 **조각별로** 확인합니다: ⊔·맵 갱신·op̂·합성이 모두 단조이므로, 이들로 만든 eval과 전체 f가 단조. 단조성이 확보되어 **Tarski(lfp 존재)·Kleene(반복 계산)** 적용 가능(강의 6). 핵심 조각 op̂의 단조성이 슬15.

---

## 슬라이드 15: Monotonicity of Addition

### 원문 내용
> (슬10과 같은 +̂ 표 재게시, 단조성 확인용)

### 번역
> 추상 덧셈표를 다시 보며 `+̂`가 단조임을 확인: 어느 인자가 격자에서 올라가도(예: 0→+) 결과가 내려가지 않음.

### 해설

**개념 설명**

`+̂`가 단조임을 표로 확인합니다. 단조 = "입력이 ⊑로 커지면 결과도 ⊑로 커지거나 같다". 예: `+ +̂ 0 = +`이고 `+ +̂ + = +`, `+ +̂ ⊤ = ⊤`(0⊑+⊑⊤이고 결과 +⊑+⊑⊤). 표의 각 행·열이 위로 갈수록 결과도 위로 → 단조. 이로써 분석 건전성·종료의 전제가 갖춰집니다. 풀이가 슬16.

---

## 슬라이드 16: Solving Constraints

### 원문 내용
> - f : State^n → State^n; f(⟦v1⟧, ..., ⟦vn⟧) = (f1(⟦v1⟧), ..., fn(⟦vn⟧))
> - We can compute lfp(f) using NaiveFixedPointAlgorithm: x←⊥; while x≠f(x): x←f(x); return x

### 번역
> 제약을 함수 f로 묶고(강의 6 슬6), **단순 고정점 알고리즘**(⊥에서 f 반복)으로 lfp 계산.

### 해설

**개념 설명**

강의 6 슬16의 단순 고정점 알고리즘을 그대로 적용 — ⊥에서 f를 반복해 lfp(분석 결과)를 구합니다. 부호 분석이 강의 5~6의 이론 위에 완전히 올라섰습니다. 단 이 단순 알고리즘은 비효율적이라(슬23~) 워크리스트로 개선합니다. 먼저 정밀도 개선(슬17)·응용(슬18)을 봅니다.

---

## 슬라이드 17: Precision — Refined Sign Lattice

### 원문 내용
> - Adding abstract values can improve precision (e.g., −/0, −/+, 0/+)
> (확장 부호 격자: ⊤ / −/0, −/+, 0/+ / −, 0, + / ⊥)

### 번역
> - 추상값을 추가하면 **정밀도 향상**: `−/0`(음수 또는 0), `−/+`(0 아님), `0/+`(양수 또는 0) 등 중간 값 추가 → 더 세밀한 격자

### 해설

**개념 설명 — 도메인을 키워 정밀도 ↑**

기본 부호 격자는 `+ ⊔ 0 = ⊤`로 정보를 많이 잃습니다. **중간 추상값**(`0/+` = "0 또는 양수" = 비음수, `−/+` = "0 아님")을 추가하면 `+ ⊔ 0 = 0/+`로 더 정밀해집니다. 예: `0/+`이면 "0 이상"이라 0으로 나누기 검사에 유용. **도메인을 키우면 정밀↑ 비용↑**(강의 1 trade-off). 이는 강의 16의 구간·다면체로 가는 방향. 응용이 슬18.

---

## 슬라이드 18: Applications of Sign Analysis

### 원문 내용
> - In theory, e.g., can detect division-by-zero errors
>   - Identify division whose divisor is 0 or ⊤
>   - Would have too many false alarms
> - More powerful analysis techniques can be useful: Interval domain, Path sensitivity

### 번역
> - 이론적으로 **0으로 나누기 오류 검출** 가능: 분모가 0이나 ⊤인 나눗셈을 잡음
>   - 단 **헛경보가 너무 많음**(부호만으론 부정밀)
> - 더 강력한 기법이 유용: **구간 도메인**(강의 9), **경로 민감(path sensitivity)**

### 해설

**개념 설명**

부호 분석으로 0 나누기를 검사할 수 있지만(분모가 0/⊤이면 위험), 부호만으론 부정밀해 헛경보가 많습니다(강의 4의 false alarm). 더 정밀한 **구간 분석**(강의 9, 정확한 범위)이나 경로 민감 분석이 필요. 강의 1의 "정밀도가 중요"의 실례. 같은 틀에 다른 격자를 끼운 또 다른 분석이 슬19~22(상수 전파).

---

## 슬라이드 19: Constant Propagation — Lattice

### 원문 내용
> - State = Var → flat(ℤ)
> (flat 격자: ⊤ / ..., −2, −1, 0, 1, 2, ... / ⊥)

### 번역
> - **상수 전파(constant propagation)**: 상태 = `Var → flat(ℤ)`
> - flat 격자: 가운데 모든 정수, 위 ⊤(상수 아님), 아래 ⊥(값 없음)

### 해설

**개념 설명 — 같은 틀, 다른 격자 ★**

**상수 전파**는 "각 변수가 **상수**인가, 상수면 몇인가"를 추적합니다. 격자는 `flat(ℤ)`(강의 5 슬26) — 각 정수가 별개 추상값, ⊤(상수 아님/여러 값), ⊥(미정). 부호 분석과 **완전히 같은 프레임워크**(JOIN·전이·고정점)에 **격자만 flat(ℤ)로 교체**한 것입니다. 이것이 단조 프레임워크의 위력(슬4). 규칙이 슬20.

---

## 슬라이드 20: Constant Propagation — Constraint Rules

### 원문 내용
> - x=e: ⟦v⟧ = JOIN(v)[x ↦ eval(JOIN(v), e)]
> - entry: ⟦v⟧ = ⊤; Others: ⟦v⟧ = JOIN(v)
> - eval: eval(σ,x)=σ(x); eval(σ,n)=n; eval(σ,input())=⊤; eval(σ,e1 op e2)=op̂(...)

### 번역
> 제약 규칙이 **부호 분석과 글자 그대로 같음** — JOIN·eval·갱신 구조 동일. 차이는 eval(σ,n)=n(정수 그대로, 부호 아님)과 op̂의 정의(슬21).

### 해설

**개념 설명**

상수 전파의 제약 규칙이 부호 분석(슬9·11)과 **동일한 형태**입니다 — `JOIN→eval→갱신`. 바뀐 건 격자(flat(ℤ))와 그에 맞는 eval/op̂뿐. **프레임워크 재사용**의 명확한 예. 추상 연산자가 슬21.

---

## 슬라이드 21: Constant Propagation — Abstract Operator

### 원문 내용
> - a op̂ b = ⊥ if a=⊥ or b=⊥; ⊤ otherwise if a=⊤ or b=⊤; a op b otherwise

### 번역
> **추상 연산자 op̂**: 한쪽이 ⊥이면 ⊥; 한쪽이 ⊤(상수 아님)이면 ⊤; **둘 다 구체 상수면 실제로 연산**(`a op b`).

### 해설

**개념 설명**

상수 전파의 op̂: **둘 다 알려진 상수면 실제 계산**(3 +̂ 2 = 5), 하나라도 ⊤(상수 아님)면 ⊤, ⊥이면 ⊥. 즉 "둘 다 상수일 때만 결과가 상수". 단순하지만 컴파일러 최적화에 강력(슬22). 응용이 슬22.

---

## 슬라이드 22: Constant Propagation — Application

### 원문 내용
> ```c
> Before:          After:
> a = 3;           a = 3;
> b = a * 2;       b = 6;
> c = a + input(); c = 3 + input();
> a = a * b;       a = 18;
> e = a + c;       e = 18 + c;
> ```

### 번역
> 상수 전파로 **컴파일러 최적화**: 알려진 상수를 미리 계산해 대입(`b=a*2 → b=6`, `a=a*b → a=18`). 단 `input()`이 섞인 c는 ⊤라 일부만 치환(`3+input()`).

### 해설

**개념 설명 — 상수 전파의 실용성 (강의 1 동기)**

상수 전파는 **컴파일러 최적화**(강의 1 슬5~7의 변환)의 핵심입니다. 알려진 상수를 미리 계산해 런타임 연산을 줄입니다(`a*2`를 `6`으로). `input()`이 섞여 ⊤인 변수는 그대로 둠. 강의 1의 "변환엔 건전한 분석"이 실현 — 분석이 "a는 확실히 18"이라 보장해야 치환이 안전. 이제 효율적 고정점 알고리즘(슬23~). 단순 알고리즘의 문제가 슬23.

---

## 슬라이드 23: Fixed-Point Algorithm — Motivation

### 원문 내용
> - We need to find lfp(f) where f : State^n → State^n
> - NaiveFixedPointAlgorithm computes every fi in each iteration
>   - Much of the computation is redundant
> (예: x=(x1,...,x7), f1~f7의 정의)

### 번역
> - 단순 고정점 알고리즘은 **매 반복마다 모든 fi를 재계산** → 대부분 중복(redundant)
> - 대부분의 노드는 안 바뀌는데도 다시 계산하는 낭비

### 해설

**개념 설명 — 단순 알고리즘의 비효율**

강의 6 슬16의 단순 알고리즘은 매 반복 **모든 노드 fi를 다시 계산**합니다. 그런데 대부분 노드는 한 반복에서 안 바뀝니다(예: f2는 x1에만 의존하는데 x1이 안 바뀌면 f2 재계산 무의미). 이 **중복 계산**을 줄이는 게 슬24~34의 목표. 구조 활용이 슬24.

---

## 슬라이드 24: Fixed-Point Algorithm — Exploiting Structure

### 원문 내용
> - e.g., f2 depends only on x1, but the value of x1 does not change in most iterations
> - We can exploit the fact that our lattice is L^n and f consists of f1, ..., fn

### 번역
> - f2는 x1에만 의존하는데, x1은 대부분 반복에서 안 바뀜
> - 격자가 **곱 `L^n`**이고 f가 **성분별 f1,...,fn**으로 나뉜다는 구조를 활용

### 해설

**개념 설명 — 곱 구조를 활용**

핵심 관찰: 격자가 곱 `L^n`이고 전이가 성분별(`f1,...,fn`)이므로, **각 성분을 따로 갱신**할 수 있습니다. 그리고 각 fi는 **일부 성분에만 의존**(f2는 x1만). 따라서 "바뀐 성분이 의존하는 fi만 다시 계산"하면 됩니다. 이 발상이 Round Robin(슬25)·Chaotic(슬27)·워크리스트(슬29~)로 발전. Round Robin이 슬25.

---

## 슬라이드 25: Round Robin Algorithm

### 원문 내용
> ```
> RoundRobin(f1, ..., fn):
>   x ← ⊥
>   while x ≠ f(x):
>     for i in 1..n: xi ← fi(x)
>   return x
> ```
> - One iteration of the while loop does not give the same result as one iteration of NaiveFixedPointAlgorithm
> - Always terminates and produces lfp(f); The number of iterations may be smaller

### 번역
> **Round Robin**: ⊥에서 시작, 수렴할 때까지 각 성분을 **순서대로 즉석 갱신**(`xi ← fi(x)`, 갱신된 값을 바로 다음 계산에 사용).
> - 단순 알고리즘과 한 반복 결과는 다르지만, **항상 종료하고 lfp 산출**, 반복 횟수가 더 적을 수 있음

### 해설

**개념 설명 — Round Robin: 즉석 갱신 ★**

단순 알고리즘은 "모든 fi를 옛 x로 계산 후 한꺼번에 갱신"(Jacobi 방식)이지만, **Round Robin**은 "각 xi를 갱신하며 그 새 값을 바로 다음 fi에 사용"(Gauss-Seidel 방식). 갱신된 정보가 같은 반복 안에서 더 빨리 전파되어 **수렴이 빨라집니다**. Assignment 4의 `find_fixed_point`가 정확히 이 Gauss-Seidel Round Robin. 여전히 lfp에 수렴(순서 무관, 슬26). 관찰이 슬26.

---

## 슬라이드 26: Round Robin — Observations

### 원문 내용
> - The order of the iterations i := 1...n is irrelevant with the final result
> - We need to update xi if xi ≠ fi(x) to reach the fixed point
> - We do not need to update xi if xi = fi(x)

### 번역
> - 성분 갱신 **순서는 최종 결과와 무관**
> - `xi ≠ fi(x)`인 성분만 갱신하면 됨(바뀐 것만), `xi = fi(x)`이면 갱신 불필요

### 해설

**개념 설명**

두 관찰: ① **순서 무관**(어떤 순서로 갱신해도 같은 lfp — 단조성·고정점 유일성). ② **바뀌는 것만 갱신하면 충분**. 이 둘이 더 똑똑한 알고리즘(Chaotic·워크리스트)의 근거입니다. "안 바뀌는 성분은 건너뛰자"가 슬27. Assignment 4가 `old != widened`일 때만 갱신하는 것도 이 관찰.

---

## 슬라이드 27: Chaotic Iteration

### 원문 내용
> ```
> ChaoticIteration(f1, ..., fn):
>   x ← ⊥
>   while x ≠ f(x):
>     choose i ∈ {1,...,n} s.t. xi ≠ fi(x)
>     xi ← fi(x)
>   return x
> ```
> - Always terminates and produces lfp(f); The number of assignments may be smaller

### 번역
> **Chaotic Iteration**: 매번 **바뀔 성분(`xi ≠ fi(x)`) 하나를 골라** 갱신. 항상 종료·lfp 산출, 갱신 횟수가 더 적을 수 있음.

### 해설

**개념 설명 — Chaotic: 바뀔 것만 골라 갱신**

Round Robin이 순서대로 다 도는 대신, **Chaotic**은 "바뀔 성분 하나를 골라" 갱신합니다(슬26의 관찰 ②). 불필요한 갱신을 더 줄입니다. 순서 무관(슬26 관찰 ①)이라 어느 걸 고르든 lfp에 도달. 단 "바뀔 i를 어떻게 고르나"가 문제(슬28). 강의 11 cubic의 Propagate가 이 발상.

---

## 슬라이드 28: Chaotic Iteration — Problems

### 원문 내용
> - Not practical, as efficiency depends on the choice of i
> - Finding i requires computing fi's so it is expensive

### 번역
> - **비실용적**: 효율이 i 선택에 좌우됨
> - 바뀔 i를 찾으려면 결국 fi들을 계산해 봐야 해서 비쌈(닭과 달걀)

### 해설

**개념 설명**

Chaotic의 문제: "바뀔 성분을 고르려면" 모든 fi를 계산해 봐야 하는데, 그게 바로 피하려던 비용입니다. 즉 **선택 자체가 비싸** 실용적이지 않습니다. 해법: **무엇이 바뀌면 무엇을 다시 계산해야 하는지 의존성을 미리 기록**해 두기 → 워크리스트(슬29~). 관찰이 슬29.

---

## 슬라이드 29: Worklist Algorithm — Observation

### 원문 내용
> - fi typically uses only a few of x1, ..., xn
> - We can record the nodes that need recomputation based on what we updated, rather than newly finding them every time

### 번역
> - 각 fi는 보통 **x1,...,xn 중 일부만** 사용
> - 매번 새로 찾는 대신, "**무엇을 갱신하면 어떤 노드를 다시 계산해야 하는지**"를 **미리 기록**

### 해설

**개념 설명 — 워크리스트의 핵심 아이디어 ★**

Chaotic의 "매번 찾기" 문제를, **의존성을 미리 기록**해 해결합니다: "xi가 바뀌면 그것에 의존하는 노드들만 다시 계산하면 된다"를 알고 있으면, 바뀐 노드의 의존 노드들을 **대기열(worklist)에 넣어** 처리하면 됩니다. 새로 탐색할 필요 없음. 이 의존성 맵이 `dep`(슬30). 강의 11 cubic의 워크리스트 W, Assignment 4의 워크리스트 분석과 같은 발상.

---

## 슬라이드 30: Worklist Algorithm — dep

### 원문 내용
> - dep : Node → 𝒫(Node)
> - dep(v) = the set of nodes whose information depends on the information of v
> - For the sign analysis and constant propagation analysis, dep = succ
> - When the information of v is updated, only the nodes in dep(v) need to be recomputed

### 번역
> - **`dep : Node → 𝒫(Node)`**: `dep(v)` = **v의 정보에 의존하는 노드들의 집합**
> - 부호 분석·상수 전파에서는 **dep = succ**(후속자) — 전방 분석이라 v가 바뀌면 그 후속이 영향받음
> - v가 갱신되면 **`dep(v)`의 노드만 다시 계산**

### 해설

**개념 설명 — dep = 의존성 맵 ★**

**`dep(v)`**는 "v가 바뀌면 다시 계산해야 할 노드들"입니다. **전방 분석(부호·상수)에선 `dep = succ`**(후속자) — v의 상태가 바뀌면 v를 선행자로 갖는 후속 노드들의 JOIN이 영향받기 때문. (후방 분석이면 dep=pred — 강의 12의 live guard.) 이 의존성으로 "바뀐 노드의 후속만" 워크리스트에 넣습니다. 예가 슬31.

---

## 슬라이드 31: Worklist Algorithm — dep Example

### 원문 내용
> - ⟦v5⟧ = ⟦v4⟧[c ↦ +̂(⟦v4⟧(a), ⟦v4⟧(b))]
> - ⟦v6⟧ = ⟦v4⟧[c ↦ −̂(⟦v4⟧(a), ⟦v4⟧(b))]
> - dep(v4) = {v5, v6}

### 번역
> v5, v6이 모두 ⟦v4⟧에 의존 → `dep(v4) = {v5, v6}`. v4가 갱신되면 v5·v6만 다시 계산.

### 해설

**개념 설명**

v5·v6의 제약이 ⟦v4⟧를 참조하므로 `dep(v4)={v5,v6}`. v4가 바뀌면 v5·v6만 워크리스트에 넣어 재계산 — 나머지 노드는 건드리지 않음. 이 의존성 추적이 효율의 핵심. 알고리즘이 슬32.

---

## 슬라이드 32: Worklist Algorithm — Pseudocode

### 원문 내용
> ```
> SimpleWorkListAlgorithm(f1, ..., fn):
>   x ← ⊥
>   W ← {v1, ..., vn}
>   while W ≠ ∅:
>     vi ← W.removeOne()
>     y ← fi(x)
>     if y ≠ xi:
>       xi ← y
>       W ← W ∪ dep(vi)
>   return x
> ```
> - W is called the worklist; Always terminates and produces lfp(f)

### 번역
> **워크리스트 알고리즘**: 모든 노드를 워크리스트 W에 넣고 시작. W가 빌 때까지: 노드 하나 꺼내 fi 계산 → **바뀌었으면(`y≠xi`) 갱신하고 그 의존 노드(`dep(vi)`)를 W에 추가**. 항상 종료·lfp 산출.

### 해설

**개념 설명 — 워크리스트 알고리즘 ★★**

데이터플로우 분석의 **표준 알고리즘**입니다:
1. 모든 노드를 워크리스트 W에 넣고 시작.
2. W에서 노드를 꺼내 그 제약(fi)을 계산.
3. **값이 바뀌었으면** 갱신하고, 그 노드에 의존하는 노드들(`dep`)을 W에 다시 넣음(재계산 예약).
4. W가 빌 때(=고정점) 종료.

"바뀐 것만, 영향받는 것만" 계산해 중복을 제거합니다. 강의 11 cubic의 Propagate(W에서 (t,x) 꺼내 전파), Assignment 4의 고정점 반복이 모두 이 구조. 복잡도가 슬33.

---

## 슬라이드 33: Worklist Algorithm — Time Complexity

### 원문 내용
> If |dep(v)| is bounded by a constant for all nodes v, the worst-case time complexity is O(n · h · k) where
> - n is the number of CFG nodes
> - h is the height of the lattice L = State
> - k is the worst-case time required to compute fi

### 번역
> `|dep(v)|`가 상수 이내일 때 최악 복잡도 **O(n·h·k)**:
> - n = CFG 노드 수
> - h = 격자 State의 높이
> - k = fi 계산 비용

### 해설

**개념 설명 — O(n·h·k) ★**

워크리스트의 복잡도: 각 노드는 값이 바뀔 때만 재처리되는데, **한 노드의 값은 격자에서 최대 h번**(높이만큼) 올라갈 수 있습니다(단조 증가, 강의 6). 노드 n개, 각 재처리에 fi 계산 k → **O(n·h·k)**. 핵심: **유한 높이 h가 종료와 복잡도를 좌우**(강의 5 슬24, 강의 6 Kleene). 높이가 크거나 무한이면 위드닝으로 줄임(강의 9). 개선책이 슬34.

---

## 슬라이드 34: Worklist Algorithm — Potential Improvements

### 원문 내용
> - Handle strongly connected components (cycles) separately
> - Use a priority queue for the worklist
> - Make the dependence information more precise by allowing dep to consider x1, ..., xn in addition to v

### 번역
> 개선책: **강한 연결 요소(SCC, 사이클) 별도 처리**, 워크리스트에 **우선순위 큐** 사용, **더 정밀한 의존성**(노드뿐 아니라 변수 단위로).

### 해설

**개념 설명**

워크리스트 최적화: **SCC(사이클) 단위 처리**(강의 11 cubic의 사이클 제거, 강의 12·15의 SCC와 같은 발상), 우선순위 큐(좋은 순서로 처리해 재방문↓), 변수 단위 의존성(더 세밀하게). 이론적 O(n·h·k)를 실전에서 줄이는 기법들. 전체 요약이 슬35.

---

## 슬라이드 35: Summary

### 원문 내용
> - Dataflow analysis assigns constraint variables over a lattice to CFG nodes and solves monotone constraints via fixed-point computation
> - Sign analysis tracks the sign of variables; constant propagation tracks exact integer values
> - The naive fixed-point algorithm recomputes all nodes each iteration; Round Robin and Chaotic Iteration improve on this
> - The worklist algorithm uses dependency information (dep) to recompute only affected nodes, achieving O(n·h·k) complexity

### 번역
> - 데이터플로우 분석은 CFG 노드에 격자 변수를 두고 **단조 제약을 고정점으로** 풂
> - 부호 분석은 변수의 부호를, 상수 전파는 정확한 정수값을 추적
> - 단순 고정점 알고리즘은 매번 모든 노드 재계산; **Round Robin·Chaotic**이 개선
> - **워크리스트 알고리즘**은 의존성(dep)으로 영향받는 노드만 재계산해 **O(n·h·k)** 달성

### 해설

**전체 정리 — 강의 7의 한 장 요약**

1. **단조 프레임워크**: CFG + 완비 격자 + 단조 전이 함수. 격자·규칙만 바꾸면 다른 분석(부호·상수전파·도달정의...).
2. **부호 분석**: 격자 Var→Sign, JOIN(선행자 ⊔), 전이(JOIN→eval→갱신), 추상 연산자 op̂(건전·단조). 고정점으로 풀이.
3. **상수 전파**: 같은 틀에 flat(ℤ) 격자. 컴파일러 최적화 응용.
4. **알고리즘 진화**: 단순(전부 재계산) → Round Robin(즉석 갱신) → Chaotic(바뀔 것만) → **워크리스트**(의존성 dep으로 영향 노드만). O(n·h·k).

**다른 강의와의 연결 (파일 간 연결성)**

- ← **강의 5~6**: 격자(JOIN=lub), 고정점(Kleene 반복=워크리스트), 단조성, 유한 높이=종료/복잡도.
- ← **강의 2**: CFG/MIR이 분석 대상. Assignment 4가 이 워크리스트(Gauss-Seidel)를 MIR에 적용.
- → **강의 8 (데이터플로우 2)**: 도달 정의·사용 가능 식 등 멱집합 도메인, may/must·전방/후방 4분면.
- → **강의 9 (위드닝)**: 무한 높이 격자(구간)에서 종료 위해 위드닝. 슬17·18의 정밀도 동기.
- → **강의 11 (cubic)**: 워크리스트 W·전파가 cubic 알고리즘과 동형(토큰 단위).
- → **강의 12·15 (응용·포인터)**: live/available guard(후방, dep=pred), 흐름 감각 포인터가 같은 데이터플로우 틀.
- → **강의 18~20 (추상 해석)**: JOIN=CJOIN, 전이 함수, lfp가 의미론·건전성 정리의 토대.

**가장 큰 교훈**: **데이터플로우 분석은 "격자 + JOIN + 전이 함수"를 CFG에 끼우고 고정점을 푸는 일반 프레임워크**입니다. 부호·상수전파는 같은 틀의 다른 인스턴스(격자만 교체). 그리고 **워크리스트 알고리즘**이 강의 6의 Kleene 반복을 "의존성 기반으로 바뀐 노드만 재계산"해 O(n·h·k)로 효율화합니다 — 이 워크리스트가 강의 11의 cubic, Assignment 4, 거의 모든 실전 분석의 엔진입니다.

---

## 마치며

강의 7은 강의 5~6의 추상 이론을 **데이터플로우 분석**이라는 실전 프레임워크로 구체화합니다. 핵심 한 줄: **"CFG 각 노드에 격자 변수를 두고, JOIN(선행자 합치기)과 전이 함수로 단조 제약을 세운 뒤, 워크리스트 알고리즘으로 영향받는 노드만 재계산하며 최소 고정점을 O(n·h·k)에 구한다."** 부호 분석과 상수 전파는 같은 틀의 두 인스턴스이며, 워크리스트는 이후 모든 분석(cubic·포인터·Assignment 4)의 공통 엔진입니다. 시험에서는 (a) 단조 프레임워크의 구성요소와 인스턴스화(슬2~4), (b) JOIN의 정의와 분기 합류 처리(슬7~8), (c) 전이 함수(JOIN→eval→갱신)와 추상 연산자 op̂의 건전·단조성(슬9~10·14~15), (d) 단순/Round Robin/Chaotic/워크리스트 알고리즘의 차이와 dep의 역할(슬23~32), (e) O(n·h·k) 복잡도와 높이의 역할(슬33)이 단골입니다.
