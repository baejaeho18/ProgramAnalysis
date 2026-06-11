# Type Analysis (1) - 강의 슬라이드 해설

CSE552 Program Analysis — Lecture 3
강사: Jaemin Hong

> **이 파일을 읽는 법**: 각 슬라이드는 `원문 내용` → `번역` → `해설`(개념·배경·맥락) → 필요 시 `각주`/`슬라이드 연결` 순서입니다. 누락·왜곡 없이 원문을 모두 담되 처음 보는 사람도 따라올 수 있게 풀어 썼습니다.

---

## 강의 3 전체 조감도 (먼저 큰 그림)

이 강의는 **첫 번째 구체적 정적 분석 — 타입 분석(type analysis)** 을 다룹니다. 강의 1에서 "건전한 분석을 만든다"고 했는데, 그 첫 실습입니다. 질문은 단순합니다: **"이 프로그램이 실행 중 타입 오류를 낼까?"**(예: 함수가 아닌 걸 호출, 숫자가 아닌 걸 더하기).

이 강의가 가르치는 것은 단순히 타입 검사가 아니라, **제약 기반 분석(constraint-based analysis)이라는 강력한 일반 기법**입니다. 패턴은 이렇습니다:
1. 프로그램의 각 식·변수에 **타입 변수**를 도입 (슬라이드 9~10)
2. 코드를 훑으며 **등식 제약**을 모음 (슬라이드 11~12)
3. 모든 제약을 만족하는 해를 **단일화(unification)**로 찾음 (슬라이드 20~32)

이 "제약 모으기 → 단일화로 풀기" 패턴은 이후 강의에서 반복됩니다 — 특히 **강의 14의 Steensgaard 포인터 분석**이 정확히 같은 단일화를 쓰고, **강의 11·13·14의 cubic 알고리즘**도 제약 기반의 사촌입니다. 그리고 단일화를 효율적으로 구현하는 **Union-Find 자료구조**(슬라이드 21~32)는 거의 선형 시간을 내는 핵심 도구로, 이후 SCC·재귀 처리에도 쓰입니다.

핵심 통찰: **타입 분석 = "각 식의 타입은 무엇인가"라는 연립방정식을 푸는 것**이고, 그 방정식은 선형 등식 풀이(슬20)와 똑같은 구조 — 유일해/무해/무한해의 세 경우가 그대로 나타납니다.

---

## 슬라이드 1: 제목 슬라이드

### 원문 내용
> Type Analysis (1)
> CSE552 Program Analysis — Lecture 3
> Jaemin Hong

### 번역
> 타입 분석 (1) / CSE552 프로그램 분석 — 강의 3 / 홍재민

### 해설
첫 구체 분석인 **타입 분석**의 1편. 제약 기반 접근과 단일화(Union-Find)를 배웁니다.

---

## 슬라이드 2: Type Errors

### 원문 내용
> - Using non-numbers for arithmetic operations
> - Calling non-functions
> - Providing wrong number of arguments to functions
> - Having field accesses on non-structs
> - Accessing non-existent fields of structs

### 번역
> **타입 오류(type error)**의 예:
> - 숫자가 아닌 것으로 산술 연산
> - 함수가 아닌 것을 호출
> - 함수에 잘못된 개수의 인자 전달
> - 구조체가 아닌 것에 필드 접근
> - 구조체의 없는 필드에 접근

### 해설

**개념 설명 — 타입 오류란**

타입 오류는 "어떤 값을, 그 타입이 허용하지 않는 방식으로 쓰는" 실수입니다. 예: `true + 1`(불리언을 더함), `(5)()`(숫자를 함수처럼 호출). 이런 오류는 런타임에 크래시를 내거나(동적 언어) 컴파일이 안 됩니다(정적 언어). **타입 분석**은 이런 오류를 **실행 전에** 잡아냅니다(슬라이드 3). 강의 1의 "버그 찾기" 동기의 가장 기본적인 사례.

---

## 슬라이드 3: Type Analysis

### 원문 내용
> - Decides whether a program will result in a type error at runtime
> - Sound type analysis
>   - If the analysis concludes that the program is ok, then it will not result in a type error at runtime
>   - Some programs that will not result in a type error at runtime may be classified "not ok" by the analysis (false alarms)

### 번역
> - 프로그램이 런타임에 **타입 오류를 낼지** 판정
> - **건전한(sound) 타입 분석**:
>   - 분석이 "ok"라 하면 → 런타임에 타입 오류가 **나지 않음**(보장)
>   - 실제로는 타입 오류가 안 나는 프로그램을 "not ok"라 할 수도 있음(**헛경보, false alarm**)

### 해설

**개념 설명 — 건전한 타입 분석 (강의 1의 구체화)**

타입 분석은 **건전성**을 목표로 합니다(강의 1 슬19). "ok"라는 판정은 **항상 믿을 만함**(타입 오류 없음 보장 = 거짓 음성 없음). 대신 멀쩡한 프로그램을 "not ok"라 거부할 수 있음(헛경보 = 거짓 양성 가능, incomplete). 이것이 강의 1의 "변환·검증엔 건전성"의 전형 — 타입 검사를 통과하면 안전을 보장하되, 일부 안전한 프로그램은 거부될 수 있습니다(Rice 정리상 불가피). 용어 정리가 슬라이드 4~5.

---

## 슬라이드 4: Type Analysis and Type Checking

### 원문 내용
> - The terms "type analysis" and "type checking" can be used interchangeably; type checking is one example of static analysis
> - The term "type checking" often refers to the language-default type analysis, as in statically typed languages (e.g., C, Java, Rust)
> - The term "type analysis" often refers to analysis for dynamically typed languages (e.g., Python, JavaScript) or more precise analysis than the default type checking

### 번역
> - "타입 분석"과 "타입 검사(type checking)"는 호환적으로 쓰임; 타입 검사는 정적 분석의 한 예
> - **"타입 검사"**는 보통 정적 타입 언어(C·Java·Rust)의 **언어 기본** 타입 분석을 가리킴
> - **"타입 분석"**은 동적 타입 언어(Python·JS)용 분석이나 기본 검사보다 **더 정밀한** 분석을 가리킴

### 해설

**개념 설명 — 용어 구분**

둘은 거의 같은 말이지만 뉘앙스 차이: "타입 검사"는 컴파일러가 기본으로 하는 것(Rust 컴파일이 그 예), "타입 분석"은 그 외(동적 언어에 타입을 추론해 주거나, 기본보다 똑똑한 분석). 본질은 같은 정적 분석 — 이 과목은 "타입 분석"이라 부르며 일반 원리를 다룹니다. 타입 추론과의 관계가 슬라이드 5.

---

## 슬라이드 5: Type Analysis and Type Inference

### 원문 내용
> - In some languages (e.g., C), the types of variables are explicitly annotated by the programmer; type analysis can just utilize this information
> - In many other languages, such type annotations are optional or not available; type analysis needs to decide the types of variables
> - When saying "type checking," this process is called "type inference"
> - When saying "type analysis," inferring types is often considered as part of the analysis

### 번역
> - C 같은 언어는 변수 타입을 **명시(annotation)**하므로, 타입 분석이 그 정보를 그냥 활용
> - 많은 언어는 타입 명시가 선택적/없으므로, 타입 분석이 **변수 타입을 결정**해야 함
> - 이 과정을 "타입 검사" 맥락에선 **타입 추론(type inference)**이라 부름

### 해설

**개념 설명 — 타입 추론**

타입이 코드에 안 적혀 있으면, 분석이 **추론**해야 합니다(예: `let x = 5`에서 x는 i32). 이 강의의 핵심이 바로 이 추론 — 명시 없이 "각 식의 타입은 무엇이어야 하는가"를 제약으로부터 알아냅니다. 강의 2의 IDE 마우스호버(타입 표시)가 이 추론의 응용. 이 강의의 구체적 범위가 슬라이드 6.

---

## 슬라이드 6: Type Analysis in This Lecture — Scope

### 원문 내용
> - We target Rust
> - Rust already has type checking
> - Parameters have type annotations (except for anonymous functions)
> - We will consider a subset of Rust with all type annotations removed
> - While this is not practically useful, it allows us to understand the basics of type analysis

### 번역
> - 대상: Rust (이미 타입 검사가 있음)
> - 매개변수는 타입 명시됨(익명 함수 제외)
> - 우리는 **타입 명시를 모두 제거한 Rust 부분집합**을 다룸
> - 실용적이진 않지만, 타입 분석의 기초를 이해하는 데 좋음

### 해설

**개념 설명 — 단순화된 설정**

학습 목적으로, 타입 명시를 다 지운 Rust 조각을 다룹니다. 그러면 "타입을 처음부터 추론"하는 순수한 문제가 됩니다(이미 명시돼 있으면 추론할 게 없으니까). 실전이 아니라 원리 학습용. 접근법이 슬라이드 7.

---

## 슬라이드 7: Type Analysis in This Lecture — Approach

### 원문 내용
> - Our type analysis will collect constraints and solve them using unification, which is a general technique for solving constraints in various analyses (e.g., Steensgaard-style pointer analysis)
> - This type analysis is a simpler version of the Hindley-Milner algorithm, used in real-world languages (e.g., OCaml, Haskell)¹²
>
> ¹ The principal type-scheme of an object in combinatory logic (Hindley, 1969)
> ² A theory of type polymorphism in programming (Milner, 1978)

### 번역
> - 우리의 타입 분석은 **제약을 모아 단일화(unification)로 푼다** — 단일화는 여러 분석(예: Steensgaard 포인터 분석)에서 쓰는 일반 기법
> - 이것은 실전 언어(OCaml·Haskell)가 쓰는 **Hindley-Milner 알고리즘의 단순 버전**

### 해설

**개념 설명 — 제약 + 단일화 (이 강의의 방법론) ★**

방법론을 명시합니다: **"제약을 모으고 단일화로 푼다."** 이는 두 가지 점에서 중요합니다:
1. **단일화는 범용 기법**: 타입 분석뿐 아니라 **강의 14의 Steensgaard 포인터 분석**도 똑같은 단일화를 씁니다(이 강의가 그 예고편).
2. **Hindley-Milner**: OCaml·Haskell의 실제 타입 추론 알고리즘의 단순화 버전. 학술적으로 견고한 토대.

각주의 Hindley(1969)·Milner(1978)는 타입 추론 이론의 창시자들. 이제 다룰 언어의 문법이 슬라이드 8.

---

## 슬라이드 8: Syntax

### 원문 내용
> - Function f ::= fn x(x, ..., x) { s ... s e }
> - Statement s ::= let x = e; | e;
> - Expression e ::= x | n | e + e | e == e | x = e | if e { e } else { e } | e(e, ..., e) | &x | *e

### 번역
> 다룰 언어의 문법:
> - 함수: `fn 이름(매개변수들) { 문장들 식 }`
> - 문장: `let x = e;` 또는 `e;`
> - 식: 변수 x, 정수 n, 덧셈, 비교(==), 대입(`x=e`), if-else, 함수 호출, 주소(`&x`), 역참조(`*e`)

### 해설

**개념 설명**

분석 대상 미니 언어의 문법입니다. Rust의 핵심 식들(산술·비교·대입·조건·호출·포인터)을 추렸습니다. 각 식 형태마다 타입 제약 규칙이 하나씩 붙습니다(슬11~12). `&x`(주소)·`*e`(역참조)가 있어 포인터 타입도 다룹니다. 타입의 종류가 슬라이드 9.

---

## 슬라이드 9: Types

### 원문 내용
> - T ::= i32 | bool | () | &T | fn(T, ..., T) → T | X
> - (X is a type variable)
> - A proper type is a type that is not a type variable (i.e., built from type constructors)
> - Type constructors: i32, bool, () (nullary), & (unary), fn(_, ..., _) → _ (n+1-ary)
> - Notation: write a proper type as C(T1, ..., Tn) where C is the constructor and n is its arity

### 번역
> - 타입 `T`: `i32`, `bool`, `()`(유닛), `&T`(참조), `fn(T,...) → T`(함수), 또는 **타입 변수 X**
> - **고유 타입(proper type)**: 타입 변수가 아닌 타입(타입 생성자로 만든 것)
> - **타입 생성자**: i32·bool·()(0항), &(1항), fn(...)→_(n+1항)
> - 표기: 고유 타입을 `C(T1,...,Tn)`로 (C는 생성자, n은 항수)

### 해설

**개념 설명 — 타입 변수와 타입 생성자**

두 가지 타입:
- **타입 변수 `X`**: "아직 모르는 타입"(추론할 미지수). 선형방정식의 x, y 같은 것.
- **고유 타입**: i32, &i32, fn(i32)→bool 등 구체 타입. **타입 생성자**(i32, &, fn)로 만듭니다.

`&T`는 "T를 가리키는 참조", `fn(T1,T2)→T3`는 "T1,T2를 받아 T3를 반환하는 함수". 타입 변수를 두고 제약으로 그 값을 알아내는 것이 타입 추론 — 슬라이드 10에서 어떤 타입 변수를 도입하는지 봅니다.

---

## 슬라이드 10: Constraints — Type Variables

### 원문 내용
> - For each identifier (local variable, parameter, and function name) x, we introduce a type variable ⟦x⟧
> - For each occurrence of a non-identifier expression e, we introduce a type variable ⟦e⟧
> - Here, ⟦e⟧ refers to a concrete node in the tree, not to the syntax it corresponds to

### 번역
> - 각 **식별자**(지역 변수·매개변수·함수 이름) x마다 타입 변수 `⟦x⟧`를 도입
> - 각 **비식별자 식** e의 발생마다 타입 변수 `⟦e⟧`를 도입
> - `⟦e⟧`는 구문이 아니라 **트리의 구체 노드**를 가리킴(같은 모양 식이라도 다른 위치면 다른 변수)

### 해설

**개념 설명 — 모든 것에 타입 변수를**

추론의 첫걸음: 프로그램의 **모든 식별자와 식에 타입 변수를 붙입니다**. `⟦x⟧`는 "x의 타입", `⟦e1+e2⟧`는 "그 덧셈 식의 타입". 이 타입 변수들이 우리가 풀 미지수입니다. 중요한 점: `⟦e⟧`는 **트리의 특정 위치**를 가리킴 — 같은 `x+1`이 두 군데 있으면 서로 다른 타입 변수(다른 노드). 이 타입 변수들 사이의 제약을 모으는 규칙이 슬라이드 11~12. (강의 11의 `[x]`, 강의 14의 `⟦c⟧`와 같은 발상.)

---

## 슬라이드 11: Constraints — Collection Rules (Part 1)

### 원문 내용
> The constraints are collected from each function, statement, and expression in the given program.
> - fn x(x1, ..., xn) { ...; e }: ⟦x⟧ = fn(⟦x1⟧, ..., ⟦xn⟧) → ⟦e⟧
> - let x = e: ⟦x⟧ = ⟦e⟧
> - n: ⟦n⟧ = i32
> - e1 + e2: ⟦e1 + e2⟧ = ⟦e1⟧ = ⟦e2⟧ = i32
> - e1 == e2: ⟦e1 == e2⟧ = bool ∧ ⟦e1⟧ = ⟦e2⟧ = i32

### 번역
> 코드의 각 함수·문장·식에서 제약을 수집:
> - 함수 정의: `⟦f⟧ = fn(매개변수 타입들) → 본문 마지막 식의 타입`
> - `let x = e`: `⟦x⟧ = ⟦e⟧` (x는 e와 같은 타입)
> - 정수 n: `⟦n⟧ = i32`
> - `e1 + e2`: `⟦합⟧ = ⟦e1⟧ = ⟦e2⟧ = i32` (둘 다 정수, 결과도 정수)
> - `e1 == e2`: `⟦비교⟧ = bool ∧ ⟦e1⟧ = ⟦e2⟧ = i32` (피연산자는 정수, 결과는 불리언)

### 해설

**개념 설명 — 제약 수집 규칙 (전반부) ★**

각 코드 조각이 타입 변수들 사이의 **등식**을 만듭니다. 직관:
- **함수 정의**: 함수의 타입은 "매개변수 타입들 → 반환 타입". 반환 타입은 본문 마지막 식의 타입.
- **`let x = e`**: x는 e와 같은 타입.
- **정수**: i32.
- **덧셈**: 두 피연산자와 결과 모두 i32(정수끼리만 더할 수 있고 결과도 정수).
- **비교**: 피연산자는 i32, 결과는 bool.

각 식 형태가 "그 식이 타입에 부과하는 조건"을 등식으로 적습니다. 나머지 규칙(대입·조건·호출·포인터)이 슬라이드 12.

---

## 슬라이드 12: Constraints — Collection Rules (Part 2)

### 원문 내용
> - x = e: ⟦x = e⟧ = () ∧ ⟦x⟧ = ⟦e⟧
> - if e1 { e2 } else { e3 }: ⟦if e1 { e2 } else { e3 }⟧ = ⟦e2⟧ = ⟦e3⟧ ∧ ⟦e1⟧ = bool
> - e(e1, ..., en): ⟦e⟧ = fn(⟦e1⟧, ..., ⟦en⟧) → ⟦e(e1, ..., en)⟧
> - &x: ⟦&x⟧ = &⟦x⟧
> - *e: &⟦*e⟧ = ⟦e⟧

### 번역
> - 대입 `x = e`: `⟦대입식⟧ = ()`(유닛) ∧ `⟦x⟧ = ⟦e⟧`
> - `if e1 { e2 } else { e3 }`: 두 가지가 같은 타입(`⟦e2⟧=⟦e3⟧`)이고 조건은 bool(`⟦e1⟧=bool`), 전체는 그 타입
> - 호출 `e(e1,...,en)`: `⟦e⟧ = fn(인자 타입들) → ⟦호출 결과⟧` (e는 그 시그니처의 함수여야)
> - `&x`: `⟦&x⟧ = &⟦x⟧` (x의 참조 타입)
> - `*e`: `&⟦*e⟧ = ⟦e⟧` (e는 역참조 결과의 참조 타입)

### 해설

**개념 설명 — 제약 수집 규칙 (후반부)**

이어지는 규칙:
- **대입식**은 값이 `()`(유닛, Rust에서 대입은 유닛을 반환), 좌우변 같은 타입.
- **if-else**: 두 가지의 타입이 같아야 하고(전체 식의 타입), 조건은 bool.
- **호출**: 호출 대상 e가 "인자들 → 결과" 함수 타입이어야 함. (강의 11 CFA·강의 14 포인터의 호출 규칙과 같은 구조!)
- **`&x`**: x를 가리키는 참조. **`*e`**: e는 결과의 참조여야(역참조).

이 규칙들로 프로그램을 훑으면 등식 제약 모음이 생깁니다. 그걸 푸는 게 슬라이드 13.

---

## 슬라이드 13: Constraints — Solution

### 원문 내용
> - Solving the constraints gives the type of each identifier and expression
> - If a solution exists, the analysis says "ok"
> - If no solution exists, the analysis says "not ok"

### 번역
> - 제약을 풀면 **각 식별자·식의 타입**이 나옴
> - **해가 있으면 "ok"**(타입 오류 없음), **해가 없으면 "not ok"**(타입 오류)

### 해설

**개념 설명 — 해의 존재 = 타입 안전**

핵심 아이디어: 등식 제약들을 **모두 만족하는 타입 할당이 있으면** 프로그램은 타입이 잘 맞음("ok"). 없으면 어딘가 모순(예: 같은 변수가 i32이자 bool이어야 함) → 타입 오류("not ok"). 즉 **"타입 검사 = 제약 연립방정식의 해 존재 판정"**입니다. 예제들(슬14~19)이 이를 보여 줍니다.

---

## 슬라이드 14: Example 1

### 원문 내용
> ```rust
> fn f(x, y) { x + y }
> ```
> Constraints:
> - fn f(x,y) {x+y}: ⟦f⟧ = fn(⟦x⟧, ⟦y⟧) → ⟦x+y⟧
> - x + y: ⟦x+y⟧ = ⟦x⟧ = ⟦y⟧ = i32
> Solution:
> - ⟦f⟧ = fn(i32, i32) → i32; ⟦x⟧ = i32; ⟦y⟧ = i32

### 번역
> `f(x,y) { x+y }`: 덧셈 규칙으로 x,y,결과 모두 i32 → `f: fn(i32,i32)→i32`. 해가 존재 → ok.

### 해설

**개념 설명**

가장 단순한 예. `x+y`가 x,y를 i32로 강제하고, f의 타입이 그로부터 `fn(i32,i32)→i32`로 결정됩니다. 타입 명시를 지웠지만 **덧셈 한 번으로 타입이 추론**됩니다. 더 복잡한 함수 타입 예가 슬라이드 15~16.

---

## 슬라이드 15: Example 2 — Code and Constraints

### 원문 내용
> ```rust
> fn f(x) { x }
> fn g(y) { y(1) }
> fn h() { g(f) }
> ```
> Constraints:
> - fn f(x) {x}: ⟦f⟧ = fn(⟦x⟧) → ⟦x⟧
> - fn g(y) {y(1)}: ⟦g⟧ = fn(⟦y⟧) → ⟦y(1)⟧
> - y(1): ⟦y⟧ = fn(⟦1⟧) → ⟦y(1)⟧
> - 1: ⟦1⟧ = i32
> - fn h() {g(f)}: ⟦h⟧ = fn() → ⟦g(f)⟧
> - g(f): ⟦g⟧ = fn(⟦f⟧) → ⟦g(f)⟧

### 번역
> `f(x){x}`(항등 함수), `g(y){y(1)}`(y를 함수로 호출), `h(){g(f)}`(g에 f 전달). 호출 규칙으로 제약 수집.

### 해설

**개념 설명 — 고차 함수의 타입 추론**

함수를 인자로 넘기는(고차) 예입니다. `g(y){y(1)}`에서 y는 "i32를 받는 함수"여야 함(`y(1)` 때문). `h(){g(f)}`에서 g는 "f를 받는 함수"여야 함. 이 제약들을 풀면 각 함수 타입이 줄줄이 결정됩니다(슬16). 호출 규칙(슬12)이 함수 타입을 엮는 모습.

---

## 슬라이드 16: Example 2 — Solution

### 원문 내용
> Solution:
> - ⟦f⟧ = fn(i32) → i32
> - ⟦g⟧ = fn(fn(i32) → i32) → i32
> - ⟦y⟧ = fn(i32) → i32
> - ⟦h⟧ = fn() → i32

### 번역
> 해: f는 `fn(i32)→i32`(i32 항등), g는 `fn(fn(i32)→i32)→i32`(함수를 받음), h는 `fn()→i32`. 모두 일관 → ok.

### 해설

**개념 설명**

연립 제약을 풀면: `y(1)`로 y=`fn(i32)→?`, `g(f)`로 g가 f를 받으니 ⟦y⟧=⟦f⟧, f는 항등이라 `fn(i32)→i32`... 식으로 전파됩니다. 결과는 일관된 타입 할당 → ok. **고차 함수 타입도 단일화로 자동 추론**됨을 보여 줍니다. 해가 없는 경우가 슬라이드 17.

---

## 슬라이드 17: Example 3

### 원문 내용
> ```rust
> fn f(x) { if x { x + 1 } else { 0 } }
> ```
> Constraints:
> - ⟦f⟧ = fn(⟦x⟧) → ⟦if ...⟧
> - if x {x+1} else {0}: ⟦if⟧ = ⟦x+1⟧ = ⟦0⟧ ∧ ⟦x⟧ = bool
> - x + 1: ⟦x+1⟧ = ⟦x⟧ = ⟦1⟧ = i32
> - 1: ⟦1⟧ = i32; 0: ⟦0⟧ = i32
> - No solution exists because ⟦x⟧ cannot be both bool and i32.

### 번역
> `if x {x+1} else {0}`: 조건 x는 bool이어야(if 규칙), 그런데 `x+1`에서 x는 i32여야 함(덧셈 규칙). **x가 bool이자 i32일 순 없음 → 해 없음 → not ok**(타입 오류).

### 해설

**개념 설명 — 모순 = 타입 오류 ★**

이 예가 "해 없음 = 타입 오류"를 보여 줍니다. `if x`는 x를 bool로, `x+1`은 x를 i32로 강제 → **`⟦x⟧=bool`이자 `⟦x⟧=i32`라는 모순**. 단일화가 이 모순을 검출해 "해 없음"을 반환 → 분석은 "not ok"(타입 오류). 실제로 x를 조건으로도 쓰고 산술에도 쓰면 타입이 안 맞으니 타당합니다. 또 다른 해 없음 사례(재귀 타입)가 슬라이드 18.

---

## 슬라이드 18: Example 4

### 원문 내용
> ```rust
> fn f(x) { let y = x + 1; f }
> ```
> Constraints:
> - ⟦f⟧ = fn(⟦x⟧) → ⟦f⟧
> - let y = x+1: ⟦y⟧ = ⟦x+1⟧; x+1: ⟦x+1⟧ = ⟦x⟧ = ⟦1⟧ = i32
> Solution?
> - ⟦x⟧ = i32, ⟦y⟧ = i32, ⟦f⟧ = fn(i32) → fn(i32) → ...
> - f is a function that can take an integer infinitely many times
> - In a language with a recursive type, we can find a solution: μX. fn(i32) → X
> - Our language does not have a recursive type, so no solution exists

### 번역
> `f(x){ let y=x+1; f }`: 본문이 f 자신을 반환 → `⟦f⟧ = fn(i32) → ⟦f⟧`. 즉 f의 타입 안에 f가 무한히 중첩(`fn(i32)→fn(i32)→...`). **재귀 타입(`μX. fn(i32)→X`)이 있으면 해가 있지만, 우리 언어엔 없어 해 없음.**

### 해설

**개념 설명 — 재귀 타입과 occurs check**

`f`가 자기 자신을 반환하면 `⟦f⟧ = fn(i32) → ⟦f⟧` — 타입 변수가 **자기 자신을 포함**합니다(무한 중첩). 이는 단일화의 **occurs check**(변수가 자기를 포함하면 실패)에 걸립니다. **재귀 타입(`μX. fn(i32)→X`, "자기 참조 타입")**이 있는 언어(이론적)라면 해가 있지만, 보통 언어는 이를 금지해 "해 없음". 이 예는 "단일화가 무한 타입을 막는다"는 점을 보여 줍니다. 반대로 해가 **무한히 많은** 경우가 슬라이드 19.

---

## 슬라이드 19: Example 5

### 원문 내용
> ```rust
> fn f(x) { x }
> ```
> Constraints: ⟦f⟧ = fn(⟦x⟧) → ⟦x⟧
> Solutions:
> - ⟦f⟧ = fn(X) → X
> - ⟦f⟧ = fn(i32) → i32
> - ⟦f⟧ = fn(bool) → bool
> - ...
> - There are infinitely many solutions
> - fn(X) → X (the one with a type variable) is the most general solution, often called the principal type

### 번역
> `f(x){x}`(항등 함수): `⟦f⟧ = fn(⟦x⟧) → ⟦x⟧`. x의 타입이 안 정해져 **해가 무한히 많음**(fn(i32)→i32, fn(bool)→bool, ...). 그중 **타입 변수를 남긴 `fn(X)→X`가 가장 일반적인 해 = 주 타입(principal type)**.

### 해설

**개념 설명 — 주 타입(principal type)과 다형성**

항등 함수는 "어떤 타입이든" 받아 그대로 반환 → 타입이 하나로 안 정해지고 **무한히 많은 해**(각 구체 타입마다). 그중 **`fn(X)→X`**(타입 변수를 남긴 것)가 **가장 일반적인 해**로, 모든 구체 해를 포괄합니다 — 이를 **주 타입(principal type)**이라 합니다. 이것이 **다형성(polymorphism)**의 핵심(강의 2의 제네릭 `<T>`와 같은 발상). Hindley-Milner는 항상 주 타입을 찾아냅니다. 선형방정식과의 멋진 유추가 슬라이드 20.

---

## 슬라이드 20: Analogy

### 원문 내용
> Constraints: x+y=4, x−y=2 → Solution: x=3, y=1
> Constraints: x+y=4, 2x+2y=5 → No solution
> Constraints: x+y=4, 2x+2y=8 → Infinitely many solutions: x=4−y

### 번역
> **선형방정식과의 유추**:
> - `x+y=4, x−y=2` → 유일해 (x=3,y=1)
> - `x+y=4, 2x+2y=5` → 해 없음(모순)
> - `x+y=4, 2x+2y=8` → 무한히 많은 해(x=4−y)

### 해설

**개념 설명 — 타입 추론 = 연립방정식 풀이 ★**

이 유추가 강의 3의 핵심 통찰입니다. **타입 제약 = 연립방정식**이고, 세 가지 결과가 정확히 대응합니다:
- **유일해**(예제 1·2): 타입이 하나로 결정. "ok".
- **해 없음**(예제 3·4): 모순. "not ok"(타입 오류).
- **무한히 많은 해**(예제 5): 주 타입(다형성). "ok"이되 가장 일반적인 타입.

즉 타입 추론은 "타입에 대한 연립방정식을 푸는 것"입니다. 다른 점은 변수가 숫자가 아니라 **타입(트리 구조)**이라는 것 — 그래서 푸는 방법이 가우스 소거가 아니라 **단일화(unification)**입니다(슬21~). 이 유추가 강의 16(관계형 분석, 선형 등식 도메인)에서 다시 등장합니다.

---

## 슬라이드 21: Union-Find — Introduction

### 원문 내용
> - Union-find data structure (a.k.a. disjoint-set data structure)³
> - Represents and manipulates equivalence relations
> - Consists of a directed graph of nodes that each have exactly one edge to their parent node
>   - Which may be the node itself, in which case it is called a root
> - Two nodes are equivalent if they have a common ancestor, and each root is the canonical representative of its equivalence class
>
> ³ An improved equivalence algorithm (Galler and Fischer, 1964)

### 번역
> - **Union-Find(서로소 집합) 자료구조**: **동치 관계**를 표현·조작
> - 각 노드가 부모로 가는 간선을 정확히 하나 가진 방향 그래프(자기 자신이면 **루트**)
> - 두 노드가 **공통 조상**을 가지면 동치, 각 루트가 그 동치류의 **대표(canonical representative)**

### 해설

**개념 설명 — Union-Find = 동치류 관리 ★**

단일화는 "이 타입 변수와 저 타입 변수가 같다"는 등식을 처리하는데, 이는 곧 **동치 관계를 관리**하는 것입니다. **Union-Find**가 그 도구입니다:
- 각 원소가 부모를 가리키는 트리 구조.
- 같은 트리(공통 루트)에 속하면 **동치**.
- 루트가 그 그룹의 대표.

`⟦x⟧ = ⟦y⟧`를 만나면 두 그룹을 합치고(Union), "x와 z가 같은가?"는 루트 비교(Find)로 답합니다. **강의 14의 Steensgaard 포인터 분석이 정확히 이 Union-Find를 써서 거의 선형 시간**을 냅니다. 세 연산(MakeSet/Find/Union)이 슬23~27.

---

## 슬라이드 22: Union-Find — Example

### 원문 내용
> (그림) A는 루트(자기 자신 가리킴), B·C는 A를 부모로. D는 루트, E는 D를 부모로. → {A,B,C}와 {D,E} 두 동치류.

### 번역
> 예: A를 루트로 B,C가 한 그룹 {A,B,C}, D를 루트로 E가 한 그룹 {D,E}. 두 서로소 집합.

### 해설

**개념 설명**

트리로 동치류를 표현한 그림. A가 {A,B,C}의 대표(루트), D가 {D,E}의 대표. B와 C는 공통 조상 A를 가지니 동치, B와 D는 다른 루트라 비동치. 이 구조 위에서 세 연산이 동작합니다. 첫 연산 MakeSet이 슬23.

---

## 슬라이드 23: Union-Find — MakeSet

### 원문 내용
> - MakeSet(x): adds a new node x that initially is its own parent
> ```
> MakeSet(x):
>   x.parent ← x
> ```

### 번역
> - **MakeSet(x)**: 새 노드 x를 추가, 처음엔 **자기 자신이 부모**(혼자 한 그룹, 루트)

### 해설

**개념 설명**

MakeSet은 새 원소를 "혼자만의 그룹"으로 만듭니다(자기 자신이 루트). 타입 분석에선 새 타입 변수를 도입할 때 호출. 모든 타입 변수가 처음엔 각자 별개 그룹. 그룹의 대표를 찾는 Find가 슬24.

---

## 슬라이드 24: Union-Find — Find

### 원문 내용
> - Find(x): finds the canonical representative of x by traversing the path to the root
> ```
> Find(x):
>   while x.parent ≠ x:
>     x ← x.parent
>   return x
> ```

### 번역
> - **Find(x)**: x가 속한 그룹의 **대표(루트)**를 찾음 — 부모를 따라 루트까지 올라감

### 해설

**개념 설명**

Find는 x에서 부모를 계속 따라가 **루트(자기 자신을 가리키는 노드)**에 도달합니다. 그 루트가 x가 속한 그룹의 대표. "x와 y가 같은 그룹인가?"는 `Find(x) == Find(y)`로 답합니다. 단순하지만 트리가 깊으면 느릴 수 있어, 슬29의 경로 압축으로 최적화합니다. 예가 슬25.

---

## 슬라이드 25: Union-Find — Find (Example)

### 원문 내용
> (그림) A←B←C←D 사슬에서 Find(A), Find(B), Find(C), Find(D) 모두 A를 반환.

### 번역
> A를 루트로 B,C,D가 사슬로 연결된 경우, 어느 노드에서 Find해도 루트 A를 반환(모두 같은 그룹).

### 해설

**개념 설명**

깊은 사슬 A←B←C←D에서 Find(D)는 D→C→B→A로 올라가 A 반환. 모두 같은 그룹임을 확인. 단 이 사슬이 길면 Find가 느림(O(깊이)) — 슬28~30의 최적화 동기. 그룹을 합치는 Union이 슬26.

---

## 슬라이드 26: Union-Find — Union

### 원문 내용
> - Union(x, y): finds the canonical representatives of x and y, and makes one the parent of the other unless they are already equivalent
> ```
> Union(x, y):
>   x_r ← Find(x)
>   y_r ← Find(y)
>   if x_r ≠ y_r:
>     x_r.parent ← y_r
> ```

### 번역
> - **Union(x, y)**: x와 y의 대표(루트)를 찾아, 이미 같은 그룹이 아니면 **한 루트를 다른 루트의 자식으로** 만들어 두 그룹을 합침

### 해설

**개념 설명 — 등식을 처리하는 연산 ★**

Union이 타입 분석의 핵심입니다. `⟦x⟧ = ⟦y⟧`라는 등식을 만나면 `Union(⟦x⟧, ⟦y⟧)`를 호출해 두 동치류를 합칩니다. 구현: 두 루트를 찾아, 하나를 다른 하나의 자식으로 연결. 이미 같은 그룹이면 아무것도 안 함. 이렇게 등식 제약을 차례로 Union하면 동치류가 형성되고, 모순(다른 고유 타입을 같은 그룹으로 합치려 함)이 생기면 타입 오류. 예가 슬27.

---

## 슬라이드 27: Union-Find — Union (Example)

### 원문 내용
> Union(B, D): (Before) A←B, C←D 두 그룹. (After) Find(B)=A, Find(D)=C, A를 C의 자식으로 → C가 {A,B,C,D}의 루트.

### 번역
> Union(B,D): B의 루트 A와 D의 루트 C를 찾아, A를 C 아래로 붙임 → 두 그룹 {A,B}와 {C,D}가 {A,B,C,D} 하나로 합쳐짐(루트 C).

### 해설

**개념 설명**

Union(B,D)는 B의 그룹과 D의 그룹을 합칩니다. 루트끼리(A와 C) 연결 — A를 C의 자식으로. 이제 네 노드가 한 그룹(루트 C). 이 연산이 타입 등식 `⟦B⟧=⟦D⟧`를 반영. 복잡도와 최적화가 슬28~32.

---

## 슬라이드 28: Union-Find — Complexity

### 원문 내용
> - We can express equivalence between type variables using union-find
> - Using union-find allows us to solve constraints in O(n²)
> - With path compression and union-by-rank optimizations, the time complexity becomes O(n·α(n)), where α is the inverse Ackermann function

### 번역
> - Union-Find로 타입 변수 간 동치를 표현
> - 기본 Union-Find로 제약 풀이가 **O(n²)**
> - **경로 압축(path compression)·랭크 기반 합치기(union by rank)** 최적화로 **O(n·α(n))** — α는 역 아커만 함수(사실상 상수)

### 해설

**개념 설명 — 거의 선형 시간 ★**

기본 Union-Find도 O(n²)이지만, 두 최적화로 **거의 선형 O(n·α(n))**이 됩니다. α(역 아커만 함수)는 실용 범위에서 **4 이하의 사실상 상수** — 즉 거의 O(n). 이 효율이 **강의 14의 Steensgaard 포인터 분석이 "거의 선형 시간"**(강의 14 슬11·14)인 이유입니다. 같은 Union-Find가 두 분석을 빠르게 만듭니다. 두 최적화가 슬29~32.

---

## 슬라이드 29: Path Compression

### 원문 내용
> - During Find, make every node on the path point directly to the root
> - Flattens the tree structure, speeding up future Find operations
> ```
> Find(x):
>   if x.parent ≠ x:
>     x.parent ← Find(x.parent)
>   return x.parent
> ```

### 번역
> - **경로 압축**: Find 중에 경로의 모든 노드가 **루트를 직접 가리키게** 만듦
> - 트리를 평탄화 → 이후 Find가 빨라짐 (재귀로 구현)

### 해설

**개념 설명 — 최적화 1: 경로 압축**

Find로 루트까지 올라가는 김에, **거쳐 간 모든 노드를 루트에 직접 연결**합니다. 그러면 다음 Find는 한 번에 루트에 도달(트리가 납작해짐). 한 번 비용을 들여 미래를 빠르게 하는 amortization. 예가 슬30.

---

## 슬라이드 30: Path Compression (Example)

### 원문 내용
> (그림) Before Find(D): A←B←C←D 사슬. After Find(D): A 아래 B,C,D가 모두 직접 연결(평탄화).

### 번역
> 사슬 A←B←C←D에서 Find(D)를 하면, D·C·B가 모두 루트 A를 직접 가리키게 평탄화됨. 이후 Find가 O(1).

### 해설

**개념 설명**

깊은 사슬이 Find(D) 후 납작한 트리(A 아래 B,C,D 나란히)가 됩니다. 한 번의 Find로 미래의 모든 Find가 빨라집니다. 두 번째 최적화(union by rank)가 슬31.

---

## 슬라이드 31: Union by Rank

### 원문 내용
> - Each node has a rank (an upper bound on its height)
> - When unioning, attach the smaller-rank tree under the root of the larger-rank tree
> - If ranks are equal, choose one as the new root and increment its rank
> ```
> MakeSet(x): x.parent ← x; x.rank ← 0
> Union(x, y):
>   x_r ← Find(x); y_r ← Find(y)
>   if x_r = y_r: return
>   if x_r.rank < y_r.rank: x_r.parent ← y_r
>   else if x_r.rank > y_r.rank: y_r.parent ← x_r
>   else: y_r.parent ← x_r; x_r.rank ← x_r.rank + 1
> ```

### 번역
> - **랭크 기반 합치기(union by rank)**: 각 노드에 **랭크**(높이 상한)를 둠
> - 합칠 때 **랭크 작은 트리를 큰 트리 아래에** 붙임(트리가 깊어지지 않게)
> - 랭크가 같으면 하나를 루트로 삼고 그 랭크를 +1

### 해설

**개념 설명 — 최적화 2: 랭크 기반 합치기**

Union 시 아무렇게나 붙이면 트리가 깊어질 수 있습니다. **작은 트리를 큰 트리 아래에** 붙이면 전체 높이가 덜 늘어납니다(균형 유지). 랭크는 높이의 상한 추정치. 경로 압축(슬29)과 union by rank(슬31)를 **함께** 쓰면 O(n·α(n)) 달성. 예가 슬32.

---

## 슬라이드 32: Union by Rank (Example)

### 원문 내용
> Union(B, C): Find(B)=A (rank 1), Find(C)=C (rank 0), so C goes under A. (Before) A(r=1)←B, C(r=0). (After) A(r=1) 아래 B,C.

### 번역
> Union(B,C): B의 루트 A(랭크 1)가 C(랭크 0)보다 크므로, 작은 쪽 C를 A 아래로 붙임. A의 랭크는 그대로 1(트리 높이 안 늘어남).

### 해설

**개념 설명**

랭크 1인 A 그룹과 랭크 0인 C를 합칠 때, **작은 C를 A 아래로** 붙입니다. 그러면 A의 높이가 안 늘어나(랭크 유지) 트리가 얕게 유지됩니다. 만약 반대로 A를 C 아래 붙였다면 높이가 늘었을 것. 전체 요약이 슬33.

---

## 슬라이드 33: Summary

### 원문 내용
> - Type analysis determines whether a program may produce type errors at runtime; a sound analysis guarantees no false negatives
> - Constraint-based type analysis introduces type variables for each identifier and expression, then collects equality constraints from the program's syntax
> - These constraints can be solved using a union-find data structure in almost linear time

### 번역
> - 타입 분석은 런타임 타입 오류 가능성을 판정; 건전한 분석은 **거짓 음성 없음**(놓치지 않음)을 보장
> - **제약 기반** 타입 분석은 각 식별자·식에 **타입 변수**를 도입하고 구문에서 **등식 제약**을 수집
> - 이 제약들은 **Union-Find**로 **거의 선형 시간**에 풀 수 있음

### 해설

**전체 정리 — 강의 3의 한 장 요약**

1. **목표**: "타입 오류가 날까?"를 건전하게 판정. ok=오류 없음 보장(거짓 음성 없음), 헛경보 가능.
2. **제약 기반 접근**: 모든 식별자·식에 타입 변수 → 구문에서 등식 제약 수집 → 해 존재 판정.
3. **세 결과**(선형방정식 유추): 유일해(ok), 해 없음(타입 오류), 무한해(주 타입=다형성).
4. **단일화 = Union-Find**: 등식을 동치류 합치기로 처리. 경로 압축 + 랭크 합치기로 거의 선형 O(n·α(n)).

**다른 강의와의 연결 (파일 간 연결성)**

- ← **강의 1**: 건전한 타입 분석은 "ok 판정 신뢰, 헛경보 가능" — 강의 1의 건전성·헛경보 개념의 첫 구체화.
- ← **강의 2**: 분석 대상이 Rust(타입 명시 제거 버전), HIR 사용(강2 슬34~35).
- → **강의 4 (타입 분석 2)**: 이 기초 위에 더 복잡한 타입(구조체·다형성 등) 확장 예상.
- → **강의 11·13·14 (cubic·제약 기반)**: "타입 변수에 제약을 모아 푼다"는 패턴이 CFA(`[x]`)·출처능력·Andersen(`⟦c⟧`)에서 재등장.
- → **강의 14 (Steensgaard 포인터)**: **같은 단일화(Union-Find)**를 써서 포인터 등식을 푼다. 이 강의가 그 직접적 토대. 함수 타입 단일화 ↔ 포인터 항 단일화.
- → **강의 16 (선형 등식 도메인)**: 슬라이드 20의 "연립방정식 유추"가 관계형 분석에서 실제 선형 등식으로.

**가장 큰 교훈**: 타입 분석은 **"각 식의 타입은 무엇인가"라는 등식 연립방정식을, 단일화(Union-Find)로 푸는 것**입니다. 이 **제약 수집 + 단일화** 패턴과 **Union-Find** 자료구조는 타입을 넘어 포인터 분석(강의 14 Steensgaard)까지 재사용되는 범용 도구입니다. 유일해/무해/무한해라는 선형방정식의 세 결과가 타입의 ok/오류/다형성에 정확히 대응한다는 점이 가장 우아한 통찰입니다.

---

## 마치며

강의 3은 **첫 구체 분석(타입 분석)**을 통해 정적 분석의 핵심 기법인 **제약 기반 분석과 단일화**를 소개합니다. 핵심 한 줄: **"모든 식에 타입 변수를 붙이고 구문에서 등식 제약을 모은 뒤, Union-Find로 단일화하면 — 해가 있으면 타입 안전(ok), 모순이면 타입 오류, 변수가 남으면 다형(주 타입)이다."** 이 패턴은 강의 14의 Steensgaard 포인터 분석으로 거의 그대로 이어지며, Union-Find의 거의 선형 효율이 그 분석을 빠르게 만듭니다. 시험에서는 (a) 주어진 코드의 타입 제약 수집과 풀이(슬14~16), (b) 해가 없는 경우(모순·재귀 타입)를 찾기(슬17~18), (c) 주 타입과 다형성(슬19), (d) 선형방정식 유추(유일/무/무한해, 슬20), (e) Union-Find의 Find/Union과 경로 압축·랭크 최적화 및 복잡도(슬24~32)가 단골입니다.
