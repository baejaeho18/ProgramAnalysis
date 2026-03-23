# Type Analysis (1) - 강의 슬라이드 해설

CSE552 Program Analysis — Lecture 3
Jaemin Hong

---

## 슬라이드 1: 제목 슬라이드

### 원문 내용

> Type Analysis (1)
> CSE552 Program Analysis — Lecture 3
> Jaemin Hong

### 해설

**개념 설명**

이 강의는 프로그램 분석의 세 번째 강의로, 타입 분석(Type Analysis)의 기초를 다룬다. 타입 분석은 정적 분석 기법 중 하나로, 프로그램이 실행 시 타입 에러를 일으킬 수 있는지를 사전에 판단하는 분석 방법이다.

**배경 지식**

프로그램 분석 분야에서 타입 분석은 가장 기본적인 정적 분석 기법이며, 이를 통해 다른 더 복잡한 분석 기법들(예: 포인터 분석, 데이터 흐름 분석)을 이해할 수 있는 토대가 마련된다.

---

## 슬라이드 2: Type Errors

### 원문 내용

> **Type Errors**
>
> - Using non-numbers for arithmetic operations
> - Calling non-functions
> - Providing wrong number of arguments to functions
> - Having field accesses on non-structs
> - Accessing non-existent fields of structs

### 해설

**개념 설명**

타입 에러는 프로그램이 특정 값에 대해 그 타입이 허용하지 않는 연산을 시도할 때 발생한다. 슬라이드에서 제시한 다섯 가지는 프로그래밍에서 가장 흔하게 나타나는 타입 에러의 예시들이다.

**상세한 예시들**

1. **산술 연산 오류**: `"hello" + 5`처럼 문자열에 숫자를 더하려고 할 때
2. **함수 호출 오류**: `x = 42; x()`처럼 숫자를 함수처럼 호출할 때
3. **인자 개수 오류**: 함수가 2개의 인자를 기대하는데 3개를 전달할 때
4. **구조체 필드 접근 오류**: 구조체가 아닌 값(예: 정수)에 대해 `.field` 접근을 시도할 때
5. **존재하지 않는 필드 접근**: 구조체 `Point {x, y}`에서 정의되지 않은 필드 `.z`에 접근할 때

**배경 지식** (학부 2학년 수준)

정적 타입 시스템이 있는 언어(C, Java, Rust)에서는 이런 에러들이 컴파일 시에 감지된다. 하지만 동적 타입 언어(Python, JavaScript)에서는 이 에러들이 런타임에만 나타난다. 이 강의에서 학습하는 타입 분석 기법은 동적 타입 언어에서도 이런 에러들을 미리 감지하려는 목표를 가진다.

---

## 슬라이드 3: Type Analysis

### 원문 내용

> **Type Analysis**
>
> - Decides whether a program will result in a type error at runtime
> - Sound type analysis
>   - If the analysis concludes that the program is ok, then it will not result in a type error at runtime
>   - Some programs that will not result in a type error at runtime may be classified "not ok" by the analysis (false alarms)

### 해설

**개념 설명**

타입 분석의 핵심 목표는 주어진 프로그램이 런타임에 타입 에러를 일으킬 가능성이 있는지를 판단하는 것이다. "Sound(건전한)" 타입 분석은 거짓 음성(False Negative)이 없음을 보장한다.

**Soundness의 의미**

- **보장하는 것**: 분석이 "OK"라고 판단하면, 그 프로그램은 절대로 타입 에러를 일으키지 않는다.
- **보장하지 않는 것**: 분석이 "NOT OK"라고 판단해도, 실제로는 타입 에러가 없을 수 있다 (거짓 양성/False Positive).

이를 수학적으로 표현하면: 분석 결과 OK ⟹ 실제로도 OK

**전체적인 맥락**

이 강의에서 구현할 타입 분석은 sound 분석이다. 즉, 정확성보다는 안전성을 우선하므로, 실제로는 안전한 프로그램도 "NOT OK"라고 판정할 수 있다. 이는 실무에서 "보수적인" 분석이라고 불린다.

---

## 슬라이드 4: Type Analysis and Type Checking

### 원문 내용

> **Type Analysis and Type Checking**
>
> - The terms "type analysis" and "type checking" can be used interchangeably; type checking is one example of static analysis
> - The term "type checking" often refers to the language-default type analysis, as in statically typed languages (e.g., C, Java, Rust)
> - The term "type analysis" often refers to analysis for dynamically typed languages (e.g., Python, JavaScript) or more precise analysis than the default type checking

### 해설

**개념 설명**

"타입 분석(Type Analysis)"과 "타입 검사(Type Checking)"는 같은 개념을 나타낼 수 있지만, 문맥에 따라 구분되어 사용된다.

**용어의 구분**

| 용어 | 주요 사용 맥락 | 예시 |
|------|--------------|------|
| Type Checking | 정적 타입 언어의 기본 검사 | C, Java, Rust의 컴파일러 타입 검사 |
| Type Analysis | 동적 타입 언어의 추가 분석 | Python, JavaScript에 대한 타입 추론 |
| Type Analysis | 기본 검사보다 더 정밀한 분석 | Rust의 borrow checker보다도 더 정교한 분석 |

**배경 지식**

학부 2학년 수준에서는 이 두 용어를 거의 동일하게 봐도 된다. 차이를 이해하는 것은 고급 분석 기법으로 나아갈 때 중요해진다.

---

## 슬라이드 5: Type Analysis and Type Inference

### 원문 내용

> **Type Analysis and Type Inference**
>
> - In some languages (e.g., C), the types of variables are explicitly annotated by the programmer; type analysis can just utilize this information
> - In many other languages, such type annotations are optional or not available; type analysis needs to decide the types of variables
> - When saying "type checking," this process is called "type inference"
> - When saying "type analysis," inferring types is often considered as part of the analysis

### 해설

**개념 설명**

타입 분석이 타입 추론(Type Inference)과 어떻게 다른지를 설명한다. 언어의 설계 방식에 따라 분석기가 처리해야 할 작업의 범위가 달라진다.

**두 가지 상황**

1. **명시적 타입 지정 (Explicit Type Annotation)**
   - 예: C의 `int x = 5;`
   - 프로그래머가 이미 모든 변수의 타입을 지정
   - 분석기는 이 정보를 활용하기만 하면 됨

2. **암시적 타입 (Implicit Type)**
   - 예: Python의 `x = 5`
   - 프로그래머가 타입을 명시하지 않음
   - 분석기가 타입을 직접 추론해야 함

**타입 추론의 위치**

- "타입 검사" 맥락: 타입 추론은 검사 전 별도의 전처리 단계
- "타입 분석" 맥락: 타입 추론은 분석 과정의 필수 부분

**전체적인 맥락**

이 강의에서 다루는 Rust 부분 집합은 명시적 타입 지정을 요구하므로, 타입 추론 문제 없이 제약조건 풀이(constraint solving)에만 집중할 수 있다.

---

## 슬라이드 6: Type Analysis in This Lecture — Scope

### 원문 내용

> **Type Analysis in This Lecture — Scope**
>
> - We target Rust
> - Rust already has type checking
> - Parameters have type annotations (except for anonymous functions)
> - We will consider a subset of Rust with all type annotations removed
> - While this is not practically useful, it allows us to understand the basics of type analysis

### 해설

**개념 설명**

이 강의에서 다루는 타입 분석의 범위와 목표를 명확히 한다. 실무 Rust와 다른 단순화된 버전을 사용하는 이유는 교육적이다.

**실무 Rust vs. 강의용 Rust**

실무 Rust:
- 컴파일러가 이미 타입 검사 수행
- 모든 타입이 명시되거나 컴파일러가 추론
- 타입 안전 보장

강의용 Rust:
- 모든 타입 annotation 제거
- 타입을 다시 추론해야 함
- Rust의 기본 문법은 유지

**배경 지식**

Rust의 간단한 부분 집합만 다루므로, Rust의 고급 기능들(lifetime, trait, generic constraints)은 배제된다. 이렇게 단순화하는 이유는 학생들이 타입 분석의 핵심 알고리즘(제약조건 수집 및 풀이)에 집중할 수 있도록 하기 위함이다.

---

## 슬라이드 7: Type Analysis in This Lecture — Approach

### 원문 내용

> **Type Analysis in This Lecture — Approach**
>
> - Our type analysis will collect constraints and solve them using unification, which is a general technique for solving constraints in various analyses (e.g., Steensgaard-style pointer analysis)
> - This type analysis is a simpler version of the Hindley-Milner algorithm, used in real-world languages (e.g., OCaml, Haskell)¹²
>
> ¹ The principal type-scheme of an object in combinatory logic (Hindley, 1969)
> ² A theory of type polymorphism in programming (Milner, 1978)

### 해설

**개념 설명**

이 강의에서 사용할 타입 분석 접근법을 소개한다. 핵심은 두 단계: (1) 제약조건 수집, (2) 제약조건 풀이이다.

**Unification의 개념**

Unification은 두 항(term)이 같은지 확인하고, 같게 만들기 위한 치환(substitution)을 찾는 과정이다.
- 예: `[x] = 132` 에서 `x`를 `132`로 치환
- 예: `[x] = [y]`에서 `x`와 `y`가 같은 타입이어야 함을 인식

**Hindley-Milner 알고리즘과의 관계**

이 강의의 타입 분석:
- Hindley-Milner보다 단순화
- 핵심 아이디어는 동일: 제약조건 기반 타입 추론
- 실제로 OCaml, Haskell 같은 언어들도 이 알고리즘 사용

**배경 지식**

Hindley-Milner는 함수형 프로그래밍 언어의 타입 추론 기초로서, 1970년대부터 사용되어 온 알고리즘이다. 참고 문헌에서 보듯이 70년대에 확립되었고, 여전히 현대 컴파일러에서 활용된다.

**전체적인 맥락**

제약조건 기반 접근법은 포인터 분석, 데이터 흐름 분석 등 다양한 정적 분석 기법의 기초가 된다. 따라서 이 강의에서 배우는 기법들은 이후 더 고급 분석 주제로 확장될 수 있다.

---

## 슬라이드 8: Syntax

### 원문 내용

> **Syntax**
>
> ```
> Function f ::= fn x(x,...,x) { s...s }
> Statement s ::= let x = e; | e;
> Expression e ::= x | n | e + e | e == e | x = e
>                | if e { e } else { e }
>                | e(e,...,e) | &x | *e
> ```

### 해설

**개념 설명**

이 강의에서 분석할 프로그래밍 언어의 문법을 정의한다. BNF(Backus-Naur Form) 표기법으로 함수, 문장, 식의 구조를 나타낸다.

**문법 구성 요소**

1. **함수 정의**: `fn x(x,...,x) { s...s }`
   - `fn`: 함수 선언 키워드
   - 첫 번째 `x`: 함수 이름
   - `(x,...,x)`: 매개변수 리스트
   - `{ s...s }`: 함수 본체 (0개 이상의 statement)

2. **문장**: `let x = e; | e;`
   - `let x = e;`: 변수 선언 및 초기화
   - `e;`: 식 자체로 이루어진 문장

3. **식**:
   - `x`: 변수 참조
   - `n`: 정수 리터럴
   - `e + e`: 덧셈
   - `e == e`: 동등 비교
   - `x = e`: 변수 할당
   - `if e { e } else { e }`: 조건식
   - `e(e,...,e)`: 함수 호출
   - `&x`: 참조 (주소 취득)
   - `*e`: 역참조

**배경 지식**

BNF 표기법은 형식 언어 정의의 표준 방식이다. `::=`는 "정의된다"를 의미하고, `|`는 "또는"을 의미한다.

**추가 설명**

이 언어는 매우 단순화되어 있어서:
- 복잡한 데이터 구조 없음 (구조체는 제약조건 예제에서만 언급)
- 타입 annotation 없음 (명시적 타입 정보가 없음)
- 포인터 연산 포함 (`&`, `*`)

---

## 슬라이드 9: Types

### 원문 내용

> **Types**
>
> ```
> T ::= 132 | bool | () | & T | fn(T,...,T) → T | X
> ```
>
> (X is a type variable)
>
> - A proper type is a type that is not a type variable (i.e., built from type constructors)
> - Type constructors: 132, bool, () (nullary), & (unary), fn(...) → ... (n+1-ary)
> - Notation: write a proper type as C(T₁,...,Tₙ) where C is the constructor and n is its arity

### 해설

**개념 설명**

타입 언어는 실제 타입(`132`, `bool` 등)과 타입 변수(`X`)로 구성된다. 이를 통해 프로그램의 타입 구조를 표현한다.

**타입의 종류**

1. **기본 타입들**
   - `132`: 정수 타입 (구체적으로는 i32로 생각할 수 있음)
   - `bool`: 부울 타입
   - `()`: 단위 타입 (값이 없음, 함수의 반환값이 없을 때)

2. **구성 타입들**
   - `& T`: 참조 타입 (T에 대한 포인터)
   - `fn(T₁,...,Tₙ) → T`: 함수 타입 (n개의 매개변수를 받아 T를 반환)

3. **타입 변수**
   - `X`: 아직 결정되지 않은 타입을 나타냄
   - 나중에 구체적인 타입으로 치환될 것

**수식/기호/코드 설명**

- **Proper Type**: 타입 변수를 포함하지 않는 타입. 예: `132`, `bool`, `&132`, `fn(132, bool) → ()`
- **Type Constructor**: 타입을 만드는 연산. 예: `&`, `fn`, `bool`은 0-ary 생성자
- **Notation**: `fn(132, bool) → 132`를 `fn(T₁, T₂) → T₃`로 나타낼 때, 생성자는 `fn`이고 arity는 3

**배경 지식**

타입 시스템의 형식적 정의에서는 타입을 항(term)으로 취급한다. 이는 논리학, 언어학에서 오는 개념으로, 타입 간의 관계(unification 등)를 수학적으로 다룰 수 있게 한다.

**추가 설명**

정수 타입을 `132`로 표기하는 것은 Rust의 실제 정수 크기를 나타내는 것이다. 실제로는 i32, i64 등이 있지만, 이 강의에서는 단순화하여 `132` (비트 수)로 표기한다.

---

## 슬라이드 10: Constraints — Type Variables

### 원문 내용

> **Constraints — Type Variables**
>
> - For each identifier (local variable, parameter, and function name) x, we introduce a type variable [x]
> - For each occurrence of a non-identifier expression e, we introduce a type variable [e]
> - Here, [e] refers to a concrete node in the tree, not to the syntax it corresponds to

### 해설

**개념 설명**

제약조건 기반 타입 분석의 첫 단계는 프로그램의 모든 "위치"에 타입 변수를 할당하는 것이다. 이 타입 변수들이 제약조건을 통해 어떤 타입을 가져야 하는지 결정될 것이다.

**타입 변수의 할당**

1. **식별자**: 각 변수명, 매개변수, 함수명에 대해 `[x]` 할당
   - 예: `let x = 5;`에서 `x` → `[x]`

2. **비식별자 식**: 복합식의 각 위치에 `[e]` 할당
   - 예: `e1 + e2` 에서 `e1` → `[e1]`, `e2` → `[e2]`, 그리고 전체 `e1+e2` → `[e1+e2]`

**[e]의 의미**

중요한 점: `[e]`는 구문(syntax)이 같은 모든 식에 대해 하나의 타입을 가지지 않는다. 프로그램의 추상 구문 트리(AST)의 구체적인 노드마다 다른 타입 변수를 할당한다.

예를 들어:
```rust
if cond {
    x = 5
} else {
    x = 10
}
```
두 `x = ...` 식이 있어도, 각각 다른 노드이므로 다른 타입 변수를 가진다.

**배경 지식**

AST는 프로그램을 트리 구조로 표현한 것이다. 컴파일러나 정적 분석 도구는 항상 이 AST를 기반으로 작업한다. 소스 코드의 문자 수열보다는 구조화된 표현을 다룬다.

---

## 슬라이드 11: Constraints — Collection Rules (Part 1)

### 원문 내용

> **Constraints — Collection Rules (Part 1)**
>
> The constraints are collected from each function, statement, and expression in the given program.
>
> - `fn x(x₁,...,xₙ) {...e}: [x] = fn([x₁],...,[xₙ]) → [e]`
> - `let x = e: [x] = [e]`
> - `n: [n] = 132`
> - `e₁ + e₂: [e₁ + e₂] = [e₁] = [e₂] = 132`
> - `e₁ == e₂: [e₁ == e₂] = bool ∧ [e₁] = [e₂] = 132`

### 해설

**개념 설명**

제약조건 수집의 첫 번째 부분으로, 각 프로그램 구조마다 어떤 제약조건을 생성해야 하는지를 규칙으로 정의한다.

**규칙별 설명**

1. **함수 정의**: `fn x(x₁,...,xₙ) {...e}: [x] = fn([x₁],...,[xₙ]) → [e]`
   - 함수 `x`의 타입은 함수 타입
   - 매개변수들의 타입: `[x₁]`, ..., `[xₙ]`
   - 반환 타입: 함수 본체의 최종식 `e`의 타입 `[e]`

2. **변수 할당**: `let x = e: [x] = [e]`
   - 변수 `x`의 타입은 우변 식 `e`의 타입과 같음

3. **정수 리터럴**: `n: [n] = 132`
   - 정수 리터럴은 항상 `132` 타입

4. **덧셈**: `e₁ + e₂: [e₁ + e₂] = [e₁] = [e₂] = 132`
   - 덧셈의 결과 타입: `132`
   - 좌측 피연산자의 타입: `132`
   - 우측 피연산자의 타입: `132`
   - 즉, 양쪽 모두 정수여야 하고 결과도 정수

5. **동등 비교**: `e₁ == e₂: [e₁ == e₂] = bool ∧ [e₁] = [e₂] = 132`
   - 비교의 결과 타입: `bool`
   - 양쪽 피연산자: 모두 `132` 타입
   - 즉, 두 정수를 비교하면 부울 값을 반환

**배경 지식**

이 규칙들은 언어의 타입 규칙(type rules)을 형식화한 것이다. 형식적 의미론(formal semantics)이나 타입 이론에서 자주 사용되는 표기법이다. 각 규칙은 프로그래밍 언어의 정의에 따라 달라진다.

**추가 설명**

`∧` 기호는 "그리고"를 의미하는 논리 AND이다. 따라서 `[e₁ == e₂] = bool ∧ [e₁] = [e₂] = 132`는 다음을 의미한다:
- 조건 1: `[e₁ == e₂] = bool`
- 조건 2: `[e₁] = 132`
- 조건 3: `[e₂] = 132`

이 세 조건을 모두 만족해야 한다.

---

## 슬라이드 12: Constraints — Collection Rules (Part 2)

### 원문 내용

> **Constraints — Collection Rules (Part 2)**
>
> - `x = e: [x = e] = () ∧ [x] = [e]`
> - `if e₁ { e₂ } else { e₃ }: [if e₁ { e₂ } else { e₃ }] = [e₂] = [e₃] ∧ [e₁] = bool`
> - `e(e₁,...,eₙ): [e] = fn([e₁],...,[eₙ]) → [e(e₁,...,eₙ)]`
> - `&x: [&x] = &[x]`
> - `*e: [*e] = [e]`

### 해설

**개념 설명**

제약조건 수집의 두 번째 부분으로, 할당, 조건식, 함수 호출, 참조, 역참조 연산의 규칙을 정의한다.

**규칙별 설명**

1. **할당**: `x = e: [x = e] = () ∧ [x] = [e]`
   - 할당문의 타입: `()`(단위 타입, 반환값 없음)
   - 할당된 변수와 우변 식이 같은 타입이어야 함

2. **조건식**: `if e₁ { e₂ } else { e₃ }: [if e₁ { e₂ } else { e₃ }] = [e₂] = [e₃] ∧ [e₁] = bool`
   - 조건식의 타입: then/else 분기의 타입과 같음 (둘 다 같은 타입이어야 함)
   - 조건(e₁)의 타입: 반드시 `bool`

3. **함수 호출**: `e(e₁,...,eₙ): [e] = fn([e₁],...,[eₙ]) → [e(e₁,...,eₙ)]`
   - 호출되는 식 `e`는 함수 타입이어야 함
   - 매개변수 타입: `[e₁]`, ..., `[eₙ]`
   - 함수 호출의 반환 타입: `[e(e₁,...,eₙ)]`

4. **참조(Reference)**: `&x: [&x] = &[x]`
   - 변수 `x`에 대한 참조의 타입은 `&[x]` (x의 참조 타입)

5. **역참조(Dereference)**: `*e: [*e] = [e]`
   - 역참조의 결과 타입은... [e]?
   - 주의: 이것은 단순화된 규칙임. 실제로는 [e] = &[*e]이어야 함.
   - 여기서는 [e]가 이미 참조 타입이라고 가정하고, 역참조는 그 내부 타입을 반환

**배경 지식**

함수 호출의 제약조건은 특히 중요하다. 이 규칙은 함수형 프로그래밍과 고차 함수(higher-order functions)를 지원하는 언어의 기초가 된다. 함수를 값처럼 다룰 수 있기 때문에, 함수 변수의 타입도 함수 타입이어야 한다.

**추가 설명**

역참조 규칙 `*e: [*e] = [e]`는 [e]가 참조 타입 `&T` 형태일 때를 가정한다. 따라서 완전한 규칙은 다음과 같다:
- `[e] = &T`이면 `[*e] = T`

하지만 이 강의의 단순화된 표기에서는 이 부분이 암묵적으로 포함되어 있다.

---

## 슬라이드 13: Constraints — Solution

### 원문 내용

> **Constraints — Solution**
>
> - Solving the constraints gives the type of each identifier and expression
> - If a solution exists, the analysis says "ok"
> - If no solution exists, the analysis says "not ok"

### 해설

**개념 설명**

모든 제약조건을 수집한 후, 이들을 만족하는 해를 찾는 과정이 제약조건 풀이(constraint solving)이다.

**해의 해석**

1. **해가 존재하는 경우**: "OK"
   - 모든 제약조건을 만족하는 타입 할당이 존재
   - 즉, 프로그램이 타입 에러를 일으키지 않음

2. **해가 존재하지 않는 경우**: "NOT OK"
   - 어떻게 타입을 할당해도 만족할 수 없는 모순된 제약조건 존재
   - 즉, 프로그램이 반드시 타입 에러를 일으킴

**예시**

```
제약조건: [x] = 132 ∧ [x] = bool
```
- [x]는 동시에 정수이면서 부울일 수 없으므로 해가 없음
- 따라서 이 프로그램은 "NOT OK"

**배경 지식**

이것은 만족 가능성(satisfiability) 문제로, 논리학과 컴퓨터 과학에서 중요한 연구 주제이다. SAT 문제, SMT 문제 등이 유사한 구조를 가진다.

---

## 슬라이드 14: Example 1

### 원문 내용

> **Example 1**
>
> ```
> fn f(x, y) { x + y }
> ```
>
> Constraints:
> - `fn f(x, y) { x + y }: [f] = fn([x], [y]) → [x + y]`
> - `x + y: [x + y] = [x] = [y] = 132`
>
> Solution:
> - `[f] = fn(132, 132) → 132`
> - `[x] = 132`
> - `[y] = 132`

### 해설

**개념 설명**

가장 단순한 예제로, 두 정수를 더하는 함수의 타입을 분석한다.

**제약조건 수집 과정**

프로그램: `fn f(x, y) { x + y }`

1. 함수 정의 규칙 적용: 함수 f의 타입은 `fn([x], [y]) → [x + y]`
2. 덧셈 규칙 적용: `[x + y] = [x] = [y] = 132`

결과:
```
[f] = fn([x], [y]) → [x + y]
[x + y] = [x] = [y] = 132
```

**제약조건 풀이**

- `[x] = 132`
- `[y] = 132`
- `[x + y] = 132`을 대입하면: `[f] = fn(132, 132) → 132`

**배경 지식**

이 함수는 타입이 안전하다. 어떤 정수를 입력하든 결과는 항상 정수이고, 타입 에러는 발생하지 않는다.

---

## 슬라이드 15: Example 2 — Code and Constraints

### 원문 내용

> **Example 2 — Code and Constraints**
>
> ```
> fn f(x) { x }
> fn g(y) { y(1) }
> fn h() { g(f) }
> ```
>
> Constraints:
> - `fn f(x) { x }: [f] = fn([x]) → [x]`
> - `fn g(y) { y(1) }: [g] = fn([y]) → [y(1)]`
> - `y(1): [y] = fn([1]) → [y(1)]`
> - `1: [1] = 132`
> - `fn h() { g(f) }: [h] = fn() → [g(f)]`
> - `g(f): [g] = fn([f]) → [g(f)]`

### 해설

**개념 설명**

고차 함수(higher-order function)를 포함하는 더 복잡한 예제이다. 함수를 인자로 받고 함수를 반환하는 함수들의 타입을 분석한다.

**프로그램 분석**

1. **함수 f**: 항등함수(identity function)
   - 입력: 임의의 타입 [x]
   - 출력: 같은 타입 [x]

2. **함수 g**: 함수를 받아서 호출하는 함수
   - 입력: 함수 [y] (매개변수 타입이 132)
   - 내부: `y(1)`로 y를 호출 (인자는 정수 1)
   - 따라서 y는 함수이고, 정수를 받아서 뭔가를 반환

3. **함수 h**: f와 g를 조합하는 함수
   - 입력: 없음
   - 내부: `g(f)`로 g에 f를 전달

**제약조건 분석**

```
[y] = fn([1]) → [y(1)]  에서
[1] = 132이므로
[y] = fn(132) → [y(1)]

[g] = fn([y]) → [y(1)]에서
[y] = fn(132) → [y(1)]을 대입하면
[g] = fn(fn(132) → [y(1)]) → [y(1)]

g(f): [g] = fn([f]) → [g(f)]에서
[f] = fn([x]) → [x]이므로
fn([x]) → [x] = fn(132) → [y(1)]
```

**배경 지식**

이것은 함수형 언어의 고전적인 예제이다. 함수를 값처럼 다루는 언어에서는 함수의 타입도 명확히 정의되어야 한다.

---

## 슬라이드 16: Example 2 — Solution

### 원문 내용

> **Example 2 — Solution**
>
> ```
> fn f(x) { x }
> fn g(y) { y(1) }
> fn h() { g(f) }
> ```
>
> Solution:
> - `[f] = fn(132) → 132`
> - `[g] = fn(fn(132) → 132) → 132`
> - `[y] = fn(132) → 132`
> - `[h] = fn() → 132`

### 해설

**개념 설명**

예제 2의 제약조건을 풀어서 나온 해이다. 이 해는 모든 제약조건을 만족한다.

**해의 의미**

1. `[f] = fn(132) → 132`
   - f는 정수를 받아서 정수를 반환하는 항등함수

2. `[g] = fn(fn(132) → 132) → 132`
   - g는 "정수를 받아서 정수를 반환하는 함수"를 받아서 정수를 반환
   - g(f)를 호출하면 f는 fn(132) → 132이므로 g의 입력 타입과 일치

3. `[y] = fn(132) → 132`
   - g의 매개변수 y는 정수를 받아서 정수를 반환하는 함수

4. `[h] = fn() → 132`
   - h는 입력이 없고 정수를 반환

**제약조건 검증**

- `fn f(x) { x }`: [f] = fn([x]) → [x] = fn(132) → 132 ✓
- `fn g(y) { y(1) }`: [g] = fn([y]) → [y(1)] = fn(fn(132) → 132) → 132 ✓
- `y(1)`: [y] = fn(132) → [y(1)] = fn(132) → 132 ✓
- `g(f)`: [g] = fn([f]) → [g(f)] = fn(fn(132) → 132) → 132 ✓

모든 제약조건이 만족되므로 이 프로그램은 "OK"이다.

---

## 슬라이드 17: Example 3

### 원문 내용

> **Example 3**
>
> ```
> fn f(x) { if x { x + 1 } else { 0 } }
> ```
>
> Constraints:
> - `fn f(x) { if x { x + 1 } else { 0 } }: [f] = fn([x]) → [if x { x + 1 } else { 0 }]`
> - `if x { x + 1 } else { 0 }: [if x { x + 1 } else { 0 }] = [x + 1] = [0] ∧ [x] = bool`
> - `x + 1: [x + 1] = [x] = [1] = 132`
> - `1: [1] = 132`
> - `0: [0] = 132`
>
> No solution exists because [x] cannot be both bool and 132.

### 해설

**개념 설명**

이 예제는 모순된 제약조건을 가지는 프로그램으로, "NOT OK"라고 판정된다. 타입 분석의 soundness를 보여주는 중요한 예제이다.

**문제 분석**

프로그램: `fn f(x) { if x { x + 1 } else { 0 } }`

논리적 오류:
1. `if x { ... }`: x가 조건이므로 x는 `bool` 타입이어야 함
2. `x + 1`: x를 숫자와 더하므로 x는 `132` 타입이어야 함

이 두 요구사항은 동시에 만족될 수 없다.

**제약조건 수집**

```
[if x { x + 1 } else { 0 }] = [x + 1] = [0] ∧ [x] = bool
[x + 1] = [x] = [1] = 132
```

결합하면:
```
[x] = bool  (if 조건에서)
[x] = 132   (x + 1에서)
```

이는 모순이다.

**배경 지식**

이 예제는 정적 타입 검사의 가치를 보여준다. 동적 타입 언어에서 이 함수를 실행하면:
- 정수 0을 전달: 조건이 거짓이므로 then 분기 실행 안 됨 → 0 반환
- 부울 true를 전달: then 분기 실행 시도 → `true + 1` 에서 런타임 에러

정적 타입 분석은 이런 오류를 미리 감지한다.

---

## 슬라이드 18: Example 4

### 원문 내용

> **Example 4**
>
> ```
> fn f(x) { let y = x + 1; f }
> ```
>
> Constraints:
> - `fn f(x) { let y = x + 1; f }: [f] = fn([x]) → [f]`
> - `let y = x + 1: [y] = [x + 1]`
> - `x + 1: [x + 1] = [x] = [1] = 132`
> - `1: [1] = 132`
>
> Solution?
> - `[x] = 132, [y] = 132, [f] = fn(132) → fn(132) → fn(132) → ...`
> - f is a function that can take an integer infinitely many times
> - In a language with a recursive type, we can find a solution: `μX. fn(132) → X`
> - Our language does not have a recursive type, so no solution exists

### 해설

**개념 설명**

이 예제는 무한 타입(infinite type)이 필요한 경우를 보여준다. 함수 f가 자기 자신을 반환하므로, f의 타입을 정의하려면 무한 중첩이 필요하다.

**프로그램 분석**

프로그램: `fn f(x) { let y = x + 1; f }`

- f는 정수 x를 받음
- y를 계산 (사용하지 않음)
- f 자신을 반환

따라서 f는:
- 정수를 받아서 f를 반환
- f는 정수를 받아서 f를 반환
- f는 정수를 받아서 f를 반환
- ...

**필요한 타입**

수학적으로는: `μX. fn(132) → X` (무한 재귀 타입)

이는 다음과 같이 전개된다:
```
fn(132) → fn(132) → fn(132) → ...
```

**배경 지식**

최신 프로그래밍 언어들 중 많은 것이 재귀 타입(recursive type)을 지원한다:
- Haskell, OCaml, Rust: 지원
- Java, C#: 제한적 지원
- Python, JavaScript: 지원하지 않음

재귀 타입을 지원하려면 타입 시스템과 구현이 매우 복잡해지므로, 이 강의의 단순화된 언어에서는 제외했다.

**전체적인 맥락**

이 예제는 이론적 한계를 보여준다. 모든 프로그램의 타입을 정할 수 없는 경우가 있으며, 이는 언어의 표현력과 복잡성의 트레이드오프를 보여준다.

---

## 슬라이드 19: Example 5

### 원문 내용

> **Example 5**
>
> ```
> fn f(x) { x }
> ```
>
> Constraints:
> - `fn f(x) { x }: [f] = fn([x]) → [x]`
>
> Solutions:
> - `[f] = fn(132) → 132`
> - `[f] = fn(bool) → bool`
> - `[f] = fn(fn() → 132) → fn() → 132`
> - `...`
> - There are infinitely many solutions
> - `fn(X) → X` (the one with a type variable) is the most general solution, often called the principal type

### 해설

**개념 설명**

이 예제는 하나의 제약조건이 여러 해를 가질 수 있음을 보여준다. 항등함수는 어떤 타입이든 받아들일 수 있기 때문에, 다양한 타입 할당이 가능하다.

**모든 가능한 해**

1. `fn(132) → 132`: 정수 항등함수
2. `fn(bool) → bool`: 부울 항등함수
3. `fn(fn() → 132) → fn() → 132`: 함수 항등함수 (반환 타입이 정수인 함수)
4. ... (무한히 많음)

모든 해가 유효하다. 왜냐하면 함수 f는 다형적(polymorphic)이기 때문이다.

**주 타입(Principal Type)**

```
fn(X) → X
```

이 타입이 특별한 이유:
- 가장 일반적이다 (most general)
- 다른 모든 해는 X를 구체적인 타입으로 치환하여 얻을 수 있다

예: X = 132로 치환 → `fn(132) → 132`

**배경 지식**

주 타입(principal type) 개념은 Hindley-Milner 타입 시스템의 핵심이다. 모든 프로그램이 주 타입을 가지면, 해석기는 프로그래머의 명시적 타입 지정 없이도 가장 일반적인 타입을 자동으로 추론할 수 있다.

**배경 지식 (대학 2학년 수준)**

다형성(polymorphism)은 같은 함수/자료구조가 다양한 타입을 다룰 수 있는 성질이다:
- 매개변수 다형성(parametric polymorphism): 타입 변수를 사용 (예: `List<T>`)
- 임시 다형성(ad-hoc polymorphism): 오버로딩 (예: `+` 연산자)

이 강의의 항등함수는 매개변수 다형성을 보여주는 예이다.

---

## 슬라이드 20: Analogy

### 원문 내용

> **Analogy**
>
> ```
> Constraints:      Constraints:        Constraints:
> x + y = 4         x + y = 4           x + y = 4
> x - y = 2         2x + 2y = 5         2x + 2y = 8
>
> Solution:         No solution         Infinitely many
> x = 3, y = 1                          solutions:
>                                       x = 4 - y
> ```

### 해설

**개념 설명**

타입 제약조건의 풀이를 선형 연립방정식의 풀이와 유추하여 설명한다. 두 분야 모두 "제약조건을 만족하는 해를 찾는" 문제이기 때문이다.

**세 가지 경우의 비유**

1. **유일한 해가 존재하는 경우**
   - 수학: `x + y = 4, x - y = 2` → 해: `x = 3, y = 1`
   - 타입: 제약조건들이 일관성 있고 충분히 강함

2. **해가 없는 경우**
   - 수학: `x + y = 4, 2x + 2y = 5` → 모순
   - 타입: 제약조건이 모순 → "NOT OK"

3. **무한히 많은 해가 존재하는 경우**
   - 수학: `x + y = 4, 2x + 2y = 8` → `x = 4 - y` (y는 자유변수)
   - 타입: `fn(X) → X` (X는 타입변수)

**배경 지식**

이 유추는 형식 언어와 자동화된 정리 증명(automated theorem proving) 분야에서 자주 사용된다. 제약조건 풀이(constraint solving)는 수학의 방정식 풀이, 논리학의 만족 가능성 문제, 프로그래밍 언어의 타입 추론 등 다양한 분야에서 나타나는 공통 패턴이다.

---

## 슬라이드 21: Union-Find — Introduction

### 원문 내용

> **Union-Find — Introduction**
>
> - Union-find data structure (a.k.a. disjoint-set data structure)³
> - Represents and manipulates equivalence relations
> - Consists of a directed graph of nodes that each have exactly one edge to its parent node
>   - Which may be the node itself, in which case it is called a root
> - Two nodes are equivalent if they have a common ancestor, and each root is the canonical representative of its equivalence class
>
> ³ An improved equivalence algorithm (Galler and Fischer, 1964)

### 해설

**개념 설명**

Union-Find는 동등성 관계(equivalence relations)를 효율적으로 관리하는 자료구조이다. 타입 분석에서 "두 타입이 같아야 한다"는 제약조건을 처리하기 위해 사용된다.

**핵심 개념**

1. **동등성 관계**: "같다"는 관계
   - 반사성(reflexive): a = a
   - 대칭성(symmetric): a = b ⟹ b = a
   - 추이성(transitive): a = b ∧ b = c ⟹ a = c

2. **구조**: 각 노드는 정확히 하나의 부모 엣지를 가짐
   - 루트 노드: 자신을 부모로 함 (자기 자신으로의 엣지)
   - 일반 노드: 다른 노드를 부모로 함

3. **동등성 판정**: 두 노드가 공통 조상을 가지면 동등

**배경 지식**

Union-Find는 크루스칼의 최소 신장 트리 알고리즘(Kruskal's algorithm)에서도 사용되는 기본적인 자료구조이다. 대학원 알고리즘 과정에서 배운다.

**배경 지식 (학부 2학년 수준)**

등가 클래스(equivalence class)는 동등한 원소들을 모아놓은 집합이다. 예를 들어, 모듈로 3에서:
- 클래스 1: {1, 4, 7, 10, ...}
- 클래스 2: {2, 5, 8, 11, ...}
- 클래스 0: {0, 3, 6, 9, ...}

Union-Find에서 각 루트 노드는 이런 등가 클래스의 대표원(representative)이다.

---

## 슬라이드 22: Union-Find — Example

### 원문 내용

> **Union-Find — Example**
>
> [Graph showing three separate trees:]
> - Tree 1: A → B
> - Tree 2: A → B, C (A is root with children B and C)
> - Tree 3: D → E

### 해설

**개념 설명**

간단한 그래프 예제로 Union-Find 구조를 시각화한다. 각 노드의 연결 관계가 동등성을 결정한다.

**예제 분석**

첫 번째 그래프:
- A, B: 같은 트리에 속함 → 동등
- D, E: 다른 트리에 속함 → 동등하지 않음

두 번째 그래프:
- A: 루트
- B, C: A의 자식들
- A, B, C는 모두 같은 등가 클래스에 속함

세 번째 그래프:
- D: 루트
- E: D의 자식
- D, E는 동등

**배경 지식**

이 구조는 숲(forest)이라고 불리는데, 여러 개의 분리된 트리를 모아놓은 것이다. 각 트리는 하나의 등가 클래스를 나타낸다.

---

## 슬라이드 23: Union-Find — MakeSet

### 원문 내용

> **Union-Find — MakeSet**
>
> - **MakeSet(x):** adds a new node x that initially is its own parent
>
> ```
> MakeSet(x):
>   x.parent ← x
> ```
>
> **MakeSet(A)**
>
> [Shows a single node A with a self-loop]

### 해설

**개념 설명**

새로운 노드를 추가하는 가장 기본적인 연산이다. 초기화 단계에서 각 타입 변수마다 MakeSet을 호출한다.

**연산 설명**

```
MakeSet(x):
    x.parent ← x
```

- 새 노드 x를 생성
- x의 부모를 x 자신으로 설정
- 이는 x가 자신만의 등가 클래스를 형성함을 의미

**초기 상태**

MakeSet(A) 후:
```
A (루트, 자신을 가리킴)
```

**배경 지식**

프로그램의 모든 타입 변수 `[x1]`, `[x2]`, ..., `[xn]`에 대해 MakeSet을 호출하여 초기화한다. 이후 Union 연산으로 동등한 타입들을 같은 클래스로 병합한다.

---

## 슬라이드 24: Union-Find — Find

### 원문 내용

> **Union-Find — Find**
>
> - **Find(x):** finds the canonical representative of x by traversing the path to the root
>
> ```
> Find(x):
>   while x.parent ≠ x :
>     x ← x.parent
>   return x
> ```

### 해설

**개념 설명**

두 노드가 동등한지 판정하기 위해 각각의 루트를 찾는 연산이다. 같은 루트를 가지면 동등하다.

**알고리즘 설명**

```
Find(x):
    while x.parent ≠ x:
        x ← x.parent
    return x
```

- x가 루트가 될 때까지 부모로 계속 이동
- 루트(자신을 부모로 하는 노드)에 도달하면 반환

**시간 복잡도**

- 최악의 경우: O(n) (체인 형태의 트리)
- 최적의 경우: O(1) (x가 이미 루트)

**배경 지식**

Find 연산의 성능을 개선하기 위해 "경로 압축(path compression)" 기법을 사용한다 (다음 슬라이드 참조).

---

## 슬라이드 25: Union-Find — Find (Example)

### 원문 내용

> **Union-Find — Find (Example)**
>
> [Linear tree structure: A ← B ← C ← D]
>
> Find(A), Find(B), Find(C), Find(D) all return A

### 해설

**개념 설명**

일렬로 연결된 트리 구조에서 모든 노드의 대표원이 같음을 보여준다.

**예제 분석**

구조:
```
A
↑
B
↑
C
↑
D (루트)
```

실제로는 각 노드가 부모를 가지는 방식이므로:
- A.parent = B
- B.parent = C
- C.parent = D
- D.parent = D

Find 연산:
- Find(A): A → B → C → D, 반환 D
- Find(B): B → C → D, 반환 D
- Find(C): C → D, 반환 D
- Find(D): D (이미 루트), 반환 D

**배경 지식**

이 구조는 최악의 경우를 보여주는데, 트리가 사슬 형태이면 Find 비용이 높다. 따라서 Union 연산을 어떻게 수행하는지가 중요하다 (다음 슬라이드 참조).

---

## 슬라이드 26: Union-Find — Union

### 원문 내용

> **Union-Find — Union**
>
> - **Union(x, y):** finds the canonical representatives of x and y, and makes one the parent of the other unless they are already equivalent
>
> ```
> Union(x, y):
>   x_r ← Find(x)
>   y_r ← Find(y)
>   if x_r ≠ y_r :
>     x_r.parent ← y_r
> ```

### 해설

**개념 설명**

두 개의 분리된 등가 클래스를 병합하는 연산이다. 타입 분석에서 "이 두 타입은 같다"는 제약조건을 처리할 때 사용된다.

**알고리즘 설명**

```
Union(x, y):
    x_r ← Find(x)
    y_r ← Find(y)
    if x_r ≠ y_r :
        x_r.parent ← y_r
```

1. x와 y의 루트를 각각 찾음 (Find 연산)
2. 루트가 다르면 한 루트의 부모를 다른 루트로 설정
3. 이제 x와 y는 같은 등가 클래스에 속함

**예제**

초기: 분리된 두 클래스
- 클래스 1: {A, B}
- 클래스 2: {C}

Union(B, C) 수행:
- Find(B) = A, Find(C) = C
- A.parent ← C
- 결과: 클래스 {A, B, C}

**배경 지식**

Union-Find의 성능은 두 가지 최적화로 크게 향상된다: (1) 경로 압축, (2) Union by Rank (다음 슬라이드 참조).

---

## 슬라이드 27: Union-Find — Union (Example)

### 원문 내용

> **Union-Find — Union (Example)**
>
> Union(B, D)
>
> Before:
> [Shows two trees: A→B and C→D]
>
> After:
> [Shows merged tree with C as root: C → {A, B, D}]

### 해설

**개념 설명**

두 개의 분리된 트리를 병합하는 과정을 시각화한다.

**병합 과정**

초기 상태:
```
Tree 1: A → B (A가 루트)
Tree 2: C → D (C가 루트)
```

Union(B, D) 실행:
1. Find(B) = A (B의 루트)
2. Find(D) = C (D의 루트)
3. A ≠ C이므로 A.parent ← C

결과:
```
     C (새로운 루트)
    / \
   A   D
   |
   B
```

또는 더 정확히:
```
A → B (변경 없음)
B → C (C가 새로운 공통 부모)
D → C
```

**배경 지식**

Union-Find의 연산 순서에 따라 결과 트리의 모양이 달라질 수 있다. Union by Rank 최적화는 이 모양을 더 균형잡히게 유지하여 성능을 개선한다.

---

## 슬라이드 28: Union-Find — Complexity

### 원문 내용

> **Union-Find — Complexity**
>
> - We can express equivalence between type variables using union-find
> - Using union-find allows us to solve constraints in O(n²)
> - With path compression and union-by-rank optimizations, the time complexity becomes O(n · α(n)), where α is the inverse Ackermann function

### 해설

**개념 설명**

Union-Find를 사용하여 제약조건을 푸는 전체 알고리즘의 시간 복잡도를 분석한다.

**복잡도 분석**

1. **기본 구현**: O(n²)
   - n개의 제약조건
   - 각 제약조건마다 O(n) 시간의 Find 연산
   - 총 O(n²)

2. **경로 압축(Path Compression)**
   - Find 연산 중에 경로의 모든 노드의 부모를 루트로 직접 설정
   - 향후 Find 연산 비용 감소

3. **Union by Rank 최적화**
   - 작은 트리를 큰 트리 아래에 붙임
   - 트리의 높이를 로그 시간에 유지

4. **최종 복잡도**: O(n · α(n))
   - α(n): 역 아커만 함수 (inverse Ackermann function)
   - 매우 느리게 증가하는 함수
   - 모든 실용적인 n에 대해 α(n) ≤ 4

**배경 지식**

역 아커만 함수는 이론적으로 흥미로운 함수이다. 아커만 함수는 재귀를 이용한 매우 빠르게 증가하는 함수이고, 그 역함수는 거의 상수에 가깝다.

실제로:
- α(2) = 1
- α(4) = 2
- α(16) = 3
- α(65,536) = 4
- α(매우 큰 수) = 5

따라서 O(n · α(n))은 실무에서 거의 O(n)과 같다.

---

## 슬라이드 29: Path Compression

### 원문 내용

> **Path Compression**
>
> - During Find, make every node on the path point directly to the root
> - Flattens the tree structure, speeding up future Find operations
>
> ```
> Find(x):
>   if x.parent ≠ x :
>     x.parent ← Find(x.parent)
>   return x.parent
> ```

### 해설

**개념 설명**

경로 압축은 Find 연산 중에 트리를 평탄화하여 향후 연산의 비용을 줄이는 최적화 기법이다.

**알고리즘 설명**

```
Find(x):
    if x.parent ≠ x :
        x.parent ← Find(x.parent)
    return x.parent
```

기본 아이디어:
1. x가 루트가 아니면
2. x의 부모의 루트를 재귀적으로 찾음
3. x의 부모를 그 루트로 직접 설정 (경로 압축)
4. x의 부모(=루트)를 반환

**효과**

Before:
```
A → B → C → D → E (E가 루트)
```

Find(A) 호출 후 (경로 압축):
```
A → E (직접 연결)
B → E
C → E
D → E
```

모든 노드가 루트로 직접 연결되므로 다음 Find 연산은 O(1)이다.

**배경 지식**

경로 압축은 "경로를 밟을 때마다 그 경로를 개선한다"는 "자기 조정" 자료구조의 예이다. 다른 예로는 Splay Tree가 있다.

---

## 슬라이드 30: Path Compression (Example)

### 원문 내용

> **Path Compression (Example)**
>
> Before Find(D):
> [Linear chain: A → B → C → D (D is root)]
>
> After Find(D):
> [D is now root, with A, B, C directly connected to it]

### 해설

**개념 설명**

경로 압축의 효과를 시각적으로 보여준다.

**실행 과정**

Find(D) 호출:
1. D.parent ≠ D? 아니오 (D는 이미 루트)
2. return D

하지만 Find 과정에서 A, B, C의 부모들도 업데이트된다:
- Find(C): C.parent ← E (또는 최종 루트)
- Find(B): B.parent ← 결과 루트
- Find(A): A.parent ← 결과 루트

결과:
```
Before:  A → B → C → D
After:   A, B, C 모두 D를 직접 가리킴
         A → D
         B → D
         C → D
         D → D
```

**효과**

- Before: 최악 경로 길이 = 4
- After: 모든 경로 길이 = 1

---

## 슬라이드 31: Union by Rank

### 원문 내용

> **Union by Rank**
>
> - Each node has a rank (an upper bound on its height)
> - When unioning, attach the smaller-rank tree under the root of the larger-rank tree
> - If ranks are equal, choose one as the new root and increment its rank
>
> ```
> MakeSet(x):
>   x.parent ← x
>   x.rank ← 0
>
> Union(x, y):
>   x_r ← Find(x)
>   y_r ← Find(y)
>   if x_r ≠ y_r :
>     return
>   if x_r.rank < y_r.rank :
>     y_r.parent ← x_r
>   else if x_r.rank > y_r.rank :
>     y_r.parent ← x_r
>   else :
>     y_r.parent ← x_r
>     x_r.rank ← x_r.rank + 1
> ```

### 해설

**개념 설명**

Union by Rank는 두 트리를 병합할 때 항상 작은 트리를 큰 트리 아래에 붙여서, 결과 트리가 균형잡힌 형태를 유지하도록 한다.

**Rank의 정의**

Rank: 노드 아래 서브트리의 높이의 상한(upper bound)

- 새 노드: rank = 0
- 두 노드를 연결할 때: rank만 업데이트 (실제 높이는 경로 압축으로 변함)

**Union 규칙**

```
if x_r.rank < y_r.rank:
    y_r.parent ← x_r (큰 트리를 root로)
else if x_r.rank > y_r.rank:
    y_r.parent ← x_r (큰 트리를 root로)
else:
    y_r.parent ← x_r (동등하면 선택, rank 증가)
```

주요 포인트: 항상 작은 rank의 트리를 큰 rank의 트리 아래에 놓는다.

**배경 지식**

Union by Rank와 경로 압축의 조합이 O(n · α(n)) 복잡도를 달성한다. 이것이 Union-Find 알고리즘이 현실에서 거의 O(n) 성능을 내는 이유이다.

---

## 슬라이드 32: Union by Rank (Example)

### 원문 내용

> **Union by Rank (Example)**
>
> Union(B, C): Find(B) = A (rank 1), Find(C) = C (rank 0), so C goes under A
>
> Before:
> [Shows: A with rank 1, B as child; C with rank 0]
>
> After:
> [Shows: A (rank 1) as root, B and C as children]

### 해설

**개념 설명**

Union by Rank를 실제로 적용하는 예제이다.

**병합 전**

상태:
- A (rank 1): 루트, B를 자식으로 가짐
- C (rank 0): 독립된 노드

**병합 과정**

Union(B, C):
1. Find(B) = A (rank 1)
2. Find(C) = C (rank 0)
3. A.rank (1) > C.rank (0)이므로 C.parent ← A

**병합 후**

```
     A (rank 1, 루트)
    / \
   B   C
```

결과: rank가 큰 A 아래에 rank가 작은 C가 붙여진다.

**배경 지식**

이 전략으로 인해 Union 이후에도 트리의 높이가 로그에 비례하도록 유지된다. 따라서 Find 연산의 비용도 로그에 유지된다.

---

## 슬라이드 33: Summary

### 원문 내용

> **Summary**
>
> - Type analysis determines whether a program may produce type errors at runtime; a sound analysis guarantees no false negatives
> - Constraint-based type analysis introduces type variables for each identifier and expression, then collects equality constraints from the program's syntax
> - These constraints can be solved using a union-find data structure in almost linear time

### 해설

**개념 설명**

전체 강의의 핵심을 세 가지로 요약한다.

**요약 1: 타입 분석의 목표**

타입 분석은:
- 프로그램이 런타임 타입 에러를 일으킬 가능성을 판단
- Sound 분석은 거짓 음성(False Negative)이 없음을 보장
  - 분석이 "OK"라고 하면 실제로도 OK
  - 분석이 "NOT OK"라고 해도 실제로는 OK일 수 있음 (보수적)

**요약 2: 제약조건 기반 타입 분석의 방법**

단계:
1. 각 변수와 식에 타입 변수 도입
2. 프로그램 문법에서 동등 제약조건 수집
3. 제약조건 풀이를 통해 타입 결정

장점:
- 직관적이고 이해하기 쉬움
- Hindley-Milner 알고리즘의 단순화 버전
- 다양한 정적 분석 기법의 기초

**요약 3: Union-Find를 이용한 효율적 풀이**

- Union-Find로 동등성 관계 관리
- 경로 압축 + Union by Rank 최적화
- 시간 복잡도: O(n · α(n)) ≈ O(n)

**전체적인 맥락**

이 강의에서 배운 기법들:
- 타입 분석의 기초 이론
- 정적 분석 일반의 기초 알고리즘
- 제약조건 풀이의 기본 패턴

이후 강의에서:
- 더 복잡한 타입 시스템 (generic, trait 등)
- 다른 정적 분석 (포인터 분석, 데이터 흐름 분석)
- 이들 분석의 조합

**추가 설명**

"거의 선형 시간(almost linear time)"이라는 표현이 중요하다. O(n · α(n))은 모든 실용적인 크기의 n에 대해 O(n)에 매우 가까우므로, 대규모 프로그램의 타입 분석도 현실적인 시간에 완료될 수 있다. 이것이 컴파일러에서 타입 분석을 기본적으로 수행할 수 있는 이유이다.

**왜 효과적인가**: rank가 k인 트리는 최소 2^k개의 노드를 가집니다(귀납법으로 증명 가능). 따라서 n개 노드가 있으면 트리 높이는 최대 log₂(n)입니다. 이것만으로도 Find가 O(log n)이 보장되고, 여기에 경로 압축까지 더하면 거의 상수 시간인 O(α(n))까지 내려가는 것입니다. rank 없이 그냥 합치면 최악의 경우 일직선 체인(높이 n)이 만들어져서 Find가 O(n)이 되어버립니다.
