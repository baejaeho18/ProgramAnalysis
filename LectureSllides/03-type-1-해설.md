# CSE552 강의 3: 타입 분석 (1) - 상세 해설

## 슬라이드 1: 제목 - CSE552 강의 3: 타입 분석 (1)

### 개념 설명
이 강의는 프로그램의 **타입 안정성(type safety)**을 분석하기 위한 체계적 접근법을 다룬다. 타입 분석은 프로그램이 실행되기 전에 타입 오류가 발생할 가능성이 있는지를 파악하는 정적 분석 기법이다. 정적 분석이란 프로그램을 직접 실행하지 않으면서도 그 성질을 파악하는 방법을 의미한다.

### 배경 지식
- 2학년 CS 학생은 이미 기본적인 타입 시스템(정수, 실수, 문자열 등)을 알고 있다
- 동적 타입 언어(Python, JavaScript)와 정적 타입 언어(Java, C++)의 차이를 이해해야 한다
- 컴파일러의 역할: 소스 코드를 분석하여 오류를 미리 발견하는 것
- 프로그래밍 언어 이론의 기초 (추상 구문 트리, 파싱 등) 이해

### 수식/기호/코드 설명
- 이 강의 전체에서 사용될 기호들:
  - `τ` (tau): 타입
  - `X`, `Y`, `Z`: 타입 변수
  - `e`, `e₁`, `e₂`: 표현식(expression)
  - `⟦·⟧` (Scott brackets): 표현식의 타입 변수
  - `∧` (and): 제약들의 논리적 결합

### 전체적인 맥락
이 강의는 4주간의 타입 분석 과정 중 첫 번째 부분이다. 기초 개념을 배우고 제약 기반 타입 분석의 전체 프레임워크를 이해하는 것이 목표다. 이후 강의에서는 이 기초 위에 더 복잡한 타입 시스템(예: 고계 함수, 다형성 함수)을 다룰 것이다.

---

## 슬라이드 2: 런타임 타입 오류 (Runtime Type Errors)

### 개념 설명
동적 타입 언어에서는 프로그래머가 타입을 명시하지 않기 때문에, 런타임에 타입 오류가 발생할 수 있다. 타입 오류란 연산의 피연산자가 그 연산이 요구하는 타입이 아닐 때 발생한다. 이러한 오류들은 프로그램이 실제로 그 코드를 실행하는 순간에만 발견되어, 이미 배포된 코드에서 사용자가 만나게 될 수 있다.

**구체적인 예시:**
- `1 + true`: 정수 1과 불린 값 true를 더하려고 시도. 더하기 연산은 두 피연산자가 모두 숫자여야 함
- `"hello"(42)`: 문자열 "hello"를 함수처럼 호출하려고 시도. 함수 호출은 호출 대상이 함수여야 함
- `[1, 2, 3][4]`: 배열 [1, 2, 3]은 3개 원소만 있는데 인덱스 4로 접근하려고 시도 (범위 초과)
- `let x = {name: "Alice"}; x + 1`: 객체에 정수를 더하려고 시도

### 배경 지식
- **동적 타입 언어**: 실행 시간에 변수의 타입이 결정된다 (Python, JavaScript, Ruby, PHP). 이들 언어는 유연성이 높지만 타입 오류를 찾기 어렵다
- **정적 타입 언어**: 컴파일 시간에 변수의 타입이 결정된다 (Java, C++, Rust, TypeScript). 이들 언어는 컴파일 단계에서 타입 오류를 발견한다
- **타입 오류 비용**: 런타임 타입 오류는 프로그램 충돌, 데이터 손상, 보안 문제로 이어질 수 있다
- **Type-driven development**: 타입 시스템을 활용하여 버그를 줄이는 개발 방식

### 수식/기호/코드 설명
런타임 오류는 다음과 같이 표현할 수 있다:
```
E[e] ↦ error  (evaluation of expression e results in type error)
```
여기서 E는 실행 환경(evaluation context)이다.

### 전체적인 맥락
이 슬라이드는 왜 타입 분석이 필요한지를 동기부여한다. 런타임 오류를 미리 발견함으로써 프로그램의 안정성과 신뢰성을 높일 수 있다. 타입 분석은 이러한 오류를 프로그램 실행 없이 정적으로 감지하는 방법을 제공한다.

---

## 슬라이드 3: 타입 분석 개요 (Type Analysis Overview)

### 개념 설명
타입 분석은 프로그램이 **실행 중에 타입 오류를 일으킬 가능성**이 있는지를 판단하는 정적 분석 기법이다. 이는 크게 두 가지 접근 방식으로 나뉜다:

1. **타입 체킹 (Type Checking)**: 프로그래머가 타입을 명시적으로 제공하고, 이것이 올바른지 검증한다. 예를 들어:
   ```rust
   let x: i32 = 5;  // 명시적으로 i32 타입
   let y: String = x;  // 타입 체커가 이 줄에서 오류 발견
   ```

2. **타입 추론 (Type Inference)**: 타입이 명시되지 않았을 때, 프로그램 구조를 분석하여 타입을 추론한다:
   ```rust
   let x = 5;  // x의 타입을 i32로 추론
   let y = x + 1;  // y도 i32로 추론
   ```

### 배경 지식
- **타입 체킹**: Haskell, ML 같은 언어에서 사용되는 방식. 명시적이지만 보일러플레이트 코드 증가
- **제약 기반 접근법 (Constraint-based approach)**: 이 강의의 핵심 기법으로, 타입 체킹과 타입 추론을 결합한다
- **정적 분석의 한계**: 모든 타입 오류를 감지하지 못할 수도 있다 (보수적 근사치)
- **거짓 양성(False Positive)**: 오류가 아닌데 오류라고 보고하는 경우

### 수식/기호 설명
- `τ`: 타입을 나타내는 기호 (예: `int`, `&int`, `int → int`)
- `X`, `Y`: 타입 변수 (아직 결정되지 않은 타입)
- `::=`: BNF의 정의 기호 ("다음 중 하나이다")

### 전체적인 맥락
이 강의에서는 제약 기반 타입 분석을 통해 동적 타입 언어의 프로그램도 정적으로 타입을 검사할 수 있음을 보여준다. 이는 Python의 mypy, JavaScript의 TypeScript 같은 현대적 도구들의 기반이 되는 기술이다.

---

## 슬라이드 4: 제약 기반 타입 분석 개요 (Constraint-Based Type Analysis Overview)

### 개념 설명
제약 기반 타입 분석은 세 가지 단계로 진행되는 체계적인 알고리즘이다:

**단계 1: 모든 표현식/식별자에 타입 변수 할당**
- 각 표현식이 갖는 타입을 아직 모르므로, 임시 변수 (예: X₁, X₂, ...)를 할당한다
- 이 변수들은 나중에 구체적인 타입으로 대체된다
- 각 서브표현식(부분 식)마다 독립적인 타입 변수가 필요하다

**단계 2: 프로그램 구문에서 등식 제약 수집**
- 프로그램을 읽으면서 타입 변수들 사이의 관계식을 수집한다
- 예: `1 + x`를 보면 "⟦1⟧ = int"와 "⟦x⟧ = int"라는 제약을 얻는다
- 이 제약들은 프로그램의 구문 규칙(syntax rules)에 의해 자동으로 생성된다

**단계 3: 통일(Unification)을 사용하여 제약 해결**
- 수집된 제약들을 풀어서 각 타입 변수에 구체적인 타입을 할당한다
- Union-Find 자료구조를 사용하면 이를 효율적으로 수행할 수 있다

### 배경 지식
- **선형 방정식과의 유사성**: 대수에서 미지수 x, y를 포함한 방정식들을 풀 때처럼, 여기서도 타입 변수들을 포함한 제약들을 푼다
- **통일(Unification)**: 두 타입이 같아야 한다는 제약을 만족시키는 타입 대입을 찾는 과정
- **동등성 관계(Equivalence Relation)**: 제약을 만족하는 타입들 사이의 관계
- **완전성(Completeness)**: 이 알고리즘이 모든 가능한 타입 해결책을 찾는가?

### 수식/기호 설명
```
Stage 1: τ variables ← GenVar(e) for each subexpression e
Stage 2: Constraints C ← CollectConstraints(P)
Stage 3: Substitution S ← Solve(C)
Result: Type of e is S(⟦e⟧)
```

### 전체적인 맥락
이것이 이 강의의 핵심 방법론이다. 다음 슬라이드들에서 각 단계를 상세히 설명할 것이다. 이 방법은 프로그램의 복잡성에 관계없이 체계적으로 적용할 수 있다는 장점이 있다.

---

## 슬라이드 5: 문법 정의 - 표현식 (Syntax: Expressions)

### 개념 설명
이 강의에서 다루는 언어는 Rust의 부분집합이다. 프로그래밍 언어의 문법을 Backus-Naur Form (BNF)으로 정의한다. BNF는 정형(formal) 언어 정의의 표준 방법이다.

**표현식 (Expressions) `e`:**
- `n`: 숫자 리터럴 (예: 1, 42, -5, 0). 이 강의에서는 정수만 다룬다
- `x`: 변수 이름. 변수는 let 바인딩이나 함수 매개변수를 통해 도입된다
- `e1 + e2`: 덧셈. 두 부분식의 합
- `e1 - e2`: 뺄셈. 두 부분식의 차이
- `if e1 { e2 } else { e3 }`: 조건 분기. e1이 0이 아니면 e2, 0이면 e3을 실행
- `&e`: 참조(reference) 생성. 표현식 e의 주소를 나타낸다
- `*e`: 역참조(dereference). 참조가 가리키는 값에 접근한다
- `(e1, ..., en)`: 튜플. 여러 값을 하나의 복합 값으로 묶는다
- `e.i`: 튜플에서 i번째 요소 접근 (0-indexed)
- `fn(x1,...,xn) { e }`: 함수 정의. 익명 함수(lambda)로 n개의 매개변수를 받는다
- `e(e1,...,en)`: 함수 호출. e를 함수로 실행하고 인자들을 전달한다
- `{ let x = e1; e2 }`: 지역 변수 선언. e1을 계산하여 x에 바인딩한 후 e2를 계산한다

### 배경 지식
- **BNF (Backus-Naur Form)**: 정형 문법을 정의하는 방법. `::=`는 "정의된다", `|`는 "또는"을 의미
- **Rust의 참조**: C의 포인터와 달리 메모리 안정성(memory safety)을 보장한다. Rust의 borrow checker는 참조의 수명(lifetime)을 관리한다
- **튜플**: Python의 튜플이나 C의 구조체와 유사하게, 여러 값을 하나의 복합 값으로 묶는다
- **렉시컬 스코핑(Lexical Scoping)**: let 바인딩의 범위는 그것이 나타나는 블록 내부로 제한된다

### 수식/기호 설명
- `::=`: "정의된다" 또는 "다음 중 하나이다"
- `|`: "또는" (선택지 분리)
- `...`: "반복" (0개 이상의 동일한 요소)
- `n ∈ ℤ`: n은 정수 집합의 원소
- `x ∈ Var`: x는 변수 이름 집합의 원소

### 전체적인 맥락
이 문법은 타입 분석의 핵심 개념들(참조, 함수, 튜플)을 포함하고 있으면서도 충분히 단순하여 분석이 가능하다. 실제 Rust는 더 많은 기능을 가지지만, 여기서는 타입 시스템의 본질을 학습하기 위해 부분집합만 다룬다.

---

## 슬라이드 6: 문법 정의 - 타입 (Syntax: Types)

### 개념 설명
타입은 프로그램의 값이 어떤 범주(category)에 속하는지를 나타낸다. 이 강의에서 정의하는 타입 문법은 다음과 같다:

**타입 (Types) `τ`:**
- `int`: 정수 타입. 모든 정수 리터럴의 타입이다
- `&τ`: 참조 타입. τ에 대한 참조를 나타낸다. 예: `&int`는 정수에 대한 참조
- `(τ1,...,τn)`: 튜플 타입. n개의 컴포넌트를 가진다. 예: `(int, &int, int)`
- `τ1 × ... × τn → τ`: 함수 타입. n개의 매개변수 타입을 받아 τ를 반환한다. 예: `int × int → int`는 두 정수를 받아 정수를 반환하는 함수
- `X`: 타입 변수 (아직 정해지지 않은 타입). 분석 과정에서 구체적인 타입으로 대체된다

### 배경 지식
- **타입 생성자(Type Constructor)**: `&`, `→` 같은 기호들은 기본 타입에서 새로운 타입을 만든다
- **참조 타입의 의미**: `&τ`는 "τ 타입의 값을 가리키는 참조"를 뜻한다. 메모리에서 실제 값은 다른 곳에 있다
- **함수 타입 표기법**: `τ1 → τ2`는 "τ1을 입력받아 τ2를 반환하는 함수"를 의미한다. 다중 인자는 Currying으로 표현할 수도 있지만, 여기서는 튜플을 사용한다
- **타입 변수의 역할**: 분석 시작 시에는 많은 타입이 미지수이므로 변수로 표현한다

### 수식/기호 설명
```
τ ::= int | &τ | (τ1,...,τn) | τ1 × ... × τn → τ | X

기호 설명:
- & : 참조 타입 생성자
- × : 타입의 카르테시안 곱 (복합 타입을 만드는 연산자)
- → : 함수 타입 생성자
- X, Y, Z : 임의의 타입 변수 (X ∈ TypeVar)
```

### 전체적인 맥락
슬라이드 5와 6은 함께 이 강의에서 분석할 언어의 완전한 정의를 제공한다. 표현식(슬라이드 5)은 프로그램의 동적 의미를 정의하고, 타입(슬라이드 6)은 각 표현식이 어떤 값을 가져야 하는지를 정의한다.

---

## 슬라이드 7: 타입 변수 (Type Variables)

### 개념 설명
모든 표현식과 식별자에 고유한 **타입 변수**를 할당한다. 이 변수들은 나중에 구체적인 타입으로 대체될 것이다. 이는 제약 기반 분석의 첫 번째 단계다.

**기호:**
- `⟦e⟧`: 표현식 e의 타입 변수. Scott brackets 또는 semantic brackets로 불린다
- `⟦x⟧`: 식별자 x의 타입 변수

**예시:**
```
표현식: 1 + 2
⟦1⟧ = X₁ (숫자 1의 타입)
⟦2⟧ = X₂ (숫자 2의 타입)
⟦1 + 2⟧ = X₃ (덧셈 결과의 타입)

표현식: fn(x) { x }
⟦x⟧ = X₁ (매개변수 x의 타입)
⟦fn(x) { x }⟧ = X₂ (함수 전체의 타입)
```

### 배경 지식
- 각 타입 변수는 **서로 다르다** (Gensym 기법 사용). 변수들의 겹침 없음을 보장한다
- 나중에 제약 해결 과정에서 이 변수들이 실제 타입으로 결정된다
- **Gensym**: "generated symbol"의 약자. 프로그램이 실행될 때마다 고유한 기호를 생성하는 기법
- **타입 변수의 범위**: 각 타입 변수는 정확히 하나의 표현식 또는 식별자에 대응된다

### 수식/기호 설명
```
GenVar: Expr → TypeVar
GenVar(e) = fresh type variable for expression e

⟦·⟧ : Expr ∪ Var → TypeVar
⟦e⟧ = type variable assigned to e
```

변수 할당 함수는 단사 함수(injective)여야 한다 (각 표현식마다 서로 다른 변수).

### 전체적인 맥락
이는 제약 기반 타입 분석의 첫 번째 단계(Stage 1)이다. 타입 변수들이 없다면, 제약을 나타낼 방법이 없다. 각 타입 변수는 하나의 "미지수"이고, 나중에 제약들을 풀어서 이 미지수들을 해결한다.

---

## 슬라이드 8: 제약 수집 규칙 (1) - 기본 규칙

### 개념 설명
프로그램의 구문을 읽으면서 타입 변수들 사이의 **등식 제약(equality constraints)**을 수집한다. 각 프로그램 구조에 대해 자동으로 제약이 생성된다.

**규칙들:**

1. **숫자 (Number)**:
   ```
   ⟦n⟧ = int
   ```
   모든 숫자 리터럴은 정수 타입이다. n이 어떤 값이든 (1, 2, -5, 1000 등), 그 타입은 항상 int이다.

2. **변수 (Variable)**:
   ```
   ⟦x⟧ = ⟦x⟧
   ```
   이것은 자명하게 보이지만, 의미는 다음과 같다: 변수 x가 어디서 사용되든, 그 타입은 항상 같아야 한다. 더 정확하게는, x가 나타나는 모든 위치에서 같은 타입 변수를 가져야 한다.

3. **덧셈 (Addition)**:
   ```
   ⟦e1 + e2⟧ = int ∧ ⟦e1⟧ = int ∧ ⟦e2⟧ = int
   ```
   세 개의 제약이 생성된다:
   - 덧셈의 결과는 int이다
   - 왼쪽 피연산자는 int여야 한다
   - 오른쪽 피연산자는 int여야 한다

4. **뺄셈 (Subtraction)**:
   ```
   ⟦e1 - e2⟧ = int ∧ ⟦e1⟧ = int ∧ ⟦e2⟧ = int
   ```
   덧셈과 동일한 규칙이다.

### 배경 지식
- **등식 제약**: 두 개의 타입이 같아야 한다는 조건. `τ₁ = τ₂` 형태
- **문법 기반 규칙**: 프로그램의 구조를 보고 자동으로 제약을 생성한다. 규칙은 결정적(deterministic)이다
- **제약 시스템**: 모든 제약을 함께 풀어야 타입이 결정된다. 개별 제약만으로는 충분하지 않다
- **∧ (논리적 AND)**: 여러 제약들을 동시에 만족해야 한다는 의미

### 수식/기호 설명
```
Constraint Grammar:
C ::= τ₁ = τ₂ | C ∧ C | true

⟦·⟧ notation:
⟦e⟧ refers to the type variable assigned to expression e
⟦x⟧ refers to the type variable assigned to identifier x
```

### 전체적인 맥락
이는 제약 기반 타입 분석의 두 번째 단계(Stage 2)이다. 이 규칙들을 모든 부분식에 적용하면 제약의 집합을 얻는다. 이 슬라이드의 규칙들은 가장 기본적인 프로그래밍 구성요소들을 다룬다.

---

## 슬라이드 9: 제약 수집 규칙 (2) - 참조와 조건문

### 개념 설명
더 복잡한 언어 구성요소들에 대한 제약 규칙을 정의한다.

**규칙들:**

1. **참조 (Reference)**:
   ```
   ⟦&e⟧ = &⟦e⟧
   ```
   참조 표현식 &e의 타입은 "&"를 e의 타입 앞에 붙인 것이다. 예: e의 타입이 int이면, &e의 타입은 &int이다.

2. **역참조 (Dereference)**:
   ```
   ⟦e⟧ = &⟦*e⟧
   ```
   이 규칙은 다음을 의미한다: *e를 계산하려면, e 자체가 참조 타입이어야 한다. 구체적으로, e의 타입은 "*e의 타입에 대한 참조"여야 한다. 이것은 역참조 연산의 유효성을 보장한다.

3. **조건문 (If)**:
   ```
   ⟦if e1 {e2} else {e3}⟧ = ⟦e2⟧ ∧ ⟦e2⟧ = ⟦e3⟧ ∧ ⟦e1⟧ = int
   ```
   세 개의 제약이 생성된다:
   - 조건문의 타입은 then 분기(e2)의 타입과 같다
   - then 분기와 else 분기의 타입이 같아야 한다 (일관성)
   - 조건(e1)은 정수여야 한다 (0-based truthiness)

### 배경 지식
- **참조 타입의 구조적 표현**: &τ는 "τ 타입의 값에 대한 참조"를 나타낸다
- **역참조의 의미**: *e는 e가 가리키는 값에 접근한다. 따라서 e는 반드시 참조 타입이어야 한다
- **조건식의 일관성**: then과 else 분기가 다른 타입을 반환하면 타입 오류다. 모든 실행 경로가 같은 타입을 반환해야 한다
- **truthy values**: 많은 언어에서 0은 false, 0이 아닌 모든 값은 true로 취급된다

### 수식/기호 설명
```
Reference type construction:
If ⟦e⟧ = τ, then ⟦&e⟧ = &τ

Dereference constraint:
⟦*e⟧ = τ implies ⟦e⟧ = &τ

If-then-else type constraint:
All three subexpressions must satisfy their respective constraints
```

### 전체적인 맥락
이 규칙들은 슬라이드 8의 기본 규칙을 확장하여 메모리 조작(참조, 역참조)과 제어 흐름(조건문)을 다룬다. 이들은 정적 타입 시스템의 안정성을 보장하기 위해 필수적이다.

---

## 슬라이드 10: 제약 수집 규칙 (3) - 튜플과 프로젝션

### 개념 설명
복합 데이터 구조인 튜플과 그 요소 접근을 다루는 제약 규칙이다.

**규칙들:**

1. **튜플 (Tuple)**:
   ```
   ⟦(e1,...,en)⟧ = (⟦e1⟧,...,⟦en⟧)
   ```
   튜플의 타입은 각 요소의 타입을 튜플로 묶은 것이다. n개의 서브표현식으로 만든 튜플은 n개의 컴포넌트 타입을 가진 튜플 타입을 갖는다.

   **예시:**
   ```
   (1, 2, 3)의 타입: (int, int, int)
   (1, &x, y)의 타입: (int, &⟦x⟧, ⟦y⟧)
   ```

2. **프로젝션/인덱싱 (Projection)**:
   ```
   ⟦e⟧ = (X₁,...,Xₙ) (e는 n-튜플)
   ⟦e.i⟧ = Xᵢ
   ```
   이 규칙의 의미:
   - e가 n-튜플이라는 제약을 생성한다
   - e.i는 튜플의 i번째 컴포넌트의 타입을 갖는다
   - 인덱스 i는 1 ≤ i ≤ n 범위여야 한다

   **예시:**
   ```
   e의 타입이 (int, &int, int)라면:
   e.1의 타입 = int
   e.2의 타입 = &int
   e.3의 타입 = int
   e.4를 접근하면 타입 오류 (범위 초과)
   ```

### 배경 지식
- **튜플**: 순서가 있는 복합 값. 각 컴포넌트는 서로 다른 타입을 가질 수 있다
- **구조적 타입(Structural Typing)**: 튜플의 타입은 그 구조(컴포넌트들의 타입)로 결정된다
- **프로젝션**: 복합 값에서 특정 요소를 추출하는 연산
- **정적 인덱싱**: i는 컴파일 타임에 알려져야 한다. 동적 인덱싱은 지원되지 않는다

### 수식/기호 설명
```
Tuple type construction:
If ⟦e₁⟧ = τ₁, ⟦e₂⟧ = τ₂, ..., ⟦eₙ⟧ = τₙ
then ⟦(e₁, e₂, ..., eₙ)⟧ = (τ₁, τ₂, ..., τₙ)

Projection constraint:
e.i is valid only if ⟦e⟧ = (τ₁, ..., τₙ) for some n ≥ i
```

### 전체적인 맥락
튜플은 여러 값을 함께 다루는 기본적인 방법이다. 이 규칙들은 튜플의 생성과 접근이 타입 안전하게 이루어지도록 보장한다. 런타임 범위 오류를 정적으로 감지한다.

---

## 슬라이드 11: 제약 수집 규칙 (4) - 함수와 Let 바인딩

### 개념 설명
고계(higher-order) 프로그래밍의 핵심인 함수 정의와 호출, 그리고 지역 변수 바인딩을 다루는 규칙들이다.

**규칙들:**

1. **함수 (Function)**:
   ```
   ⟦fn(x1,...,xn){e}⟧ = ⟦x1⟧ × ... × ⟦xn⟧ → ⟦e⟧
   ```
   함수의 타입은 다음으로 정의된다:
   - 매개변수 타입들의 곱(×)
   - 화살표(→)
   - 본문(body)의 타입

   이 규칙은 매개변수 타입들과 반환 타입을 함께 정의한다.

   **예시:**
   ```
   fn(x, y) { x + y }의 타입: ⟦x⟧ × ⟦y⟧ → ⟦x + y⟧
   제약을 풀면: int × int → int
   ```

2. **함수 호출 (Application)**:
   ```
   ⟦e⟧ = ⟦e1⟧ × ... × ⟦en⟧ → ⟦e(e1,...,en)⟧
   ```
   이 규칙의 의미:
   - 함수 e의 타입은 "인자 타입들을 받아 호출 결과를 반환하는 함수"여야 한다
   - 함수 호출이 타입 안전하려면, 함수의 매개변수 타입들이 전달되는 인자의 타입들과 일치해야 한다

   **예시:**
   ```
   f(1, 2)를 호출하면:
   ⟦f⟧ = ⟦1⟧ × ⟦2⟧ → ⟦f(1, 2)⟧
   = int × int → ⟦f(1, 2)⟧
   ```

3. **Let 바인딩 (Let Binding)**:
   ```
   ⟦{let x = e1; e2}⟧ = ⟦e2⟧ ∧ ⟦x⟧ = ⟦e1⟧
   ```
   두 개의 제약이 생성된다:
   - let 표현식 전체의 타입은 본문(e2)의 타입과 같다
   - 변수 x의 타입은 초기화 표현식(e1)의 타입과 같다

   이 규칙은 변수의 타입을 그것의 초기화 표현식에서 추론한다 (타입 추론의 핵심).

### 배경 지식
- **함수 타입**: 고계 함수를 지원하는 언어의 필수 개념
- **Currying**: 다중 매개변수 함수를 단일 매개변수 함수들의 중첩으로 표현하는 방법
- **렉시컬 스코핑**: let x = e1; e2에서 x의 범위는 e2 내부로 제한된다
- **타입 추론의 기본**: 명시적 타입 선언이 없어도 구조로부터 타입을 결정할 수 있다

### 수식/기호 설명
```
Function type:
fn(x₁,...,xₙ) { e } has type τ₁ × ... × τₙ → τ
where τᵢ = type of xᵢ and τ = type of e

Application constraint:
If f(a₁, ..., aₙ) is evaluated:
⟦f⟧ = ⟦a₁⟧ × ... × ⟦aₙ⟧ → ⟦f(a₁, ..., aₙ)⟧

Let binding scope:
{let x = e₁; e₂} binds x only within e₂
```

### 전체적인 맥락
이 규칙들은 제약 기반 분석의 완전성을 나타낸다. 함수와 let 바인딩은 고급 프로그래밍의 필수 요소이고, 이들도 제약 규칙으로 체계적으로 다룰 수 있다.

---

## 슬라이드 12: 예제 1 - Let 바인딩과 덧셈

### 개념 설명
**프로그램:**
```
{ let x = 1; x + 2 }
```

이는 가장 기본적인 예제로, let 바인딩과 덧셈 연산을 결합한다. 단계별로 분석해보자.

### 분석 과정

**단계 1: 타입 변수 할당**
```
⟦1⟧ = X₁          (리터럴 1)
⟦2⟧ = X₂          (리터럴 2)
⟦x⟧ = X₃          (변수 x의 사용)
⟦x + 2⟧ = X₄      (덧셈 연산)
⟦{ let x = 1; x + 2 }⟧ = X₅  (전체 let 표현식)
```

**단계 2: 제약 수집**

각 규칙을 적용하여 제약들을 수집한다:

1. 리터럴 1에 대해: `X₁ = int`
2. 리터럴 2에 대해: `X₂ = int`
3. 덧셈 x + 2에 대해:
   - `⟦x + 2⟧ = int` → `X₄ = int`
   - `⟦x⟧ = int` → `X₃ = int`
   - `⟦2⟧ = int` → `X₂ = int` (이미 있음)
4. let 바인딩에 대해:
   - `⟦{ let x = 1; x + 2 }⟧ = ⟦x + 2⟧` → `X₅ = X₄`
   - `⟦x⟧ = ⟦1⟧` → `X₃ = X₁`

**수집된 모든 제약:**
```
1. X₁ = int
2. X₂ = int
3. X₃ = int
4. X₄ = int
5. X₂ = int (중복)
6. X₅ = X₄
7. X₃ = X₁
```

**단계 3: 제약 해결 (Unification)**

제약들을 정리하고 해결한다:
```
X₁ = int
X₂ = int
X₃ = int  (from X₃ = X₁ and X₁ = int)
X₄ = int
X₅ = X₄ = int
```

### 결론
이 프로그램의 타입은 **`int`**이다. 변수 x는 1의 타입(int)으로 추론되고, x + 2의 결과도 int이며, 따라서 전체 let 표현식의 타입은 int이다.

### 배경 지식
- **제약 전파**: 한 변수가 결정되면, 그것과 연결된 모든 변수도 결정된다
- **중복 제약**: 같은 제약이 여러 번 수집될 수 있지만, 이는 문제가 되지 않는다
- **해의 유일성**: 일관된 제약 시스템은 유일한 해를 가진다 (또는 해가 없다)

### 전체적인 맥락
이 예제는 제약 기반 분석이 실제로 어떻게 작동하는지 보여주는 가장 간단한 경우다.

---

## 슬라이드 13: 예제 2 - 참조 생성

### 개념 설명
**프로그램:**
```
{ let x = 1; &x }
```

이 예제는 참조 타입의 생성과 전파를 보여준다.

### 분석 과정

**단계 1: 타입 변수 할당**
```
⟦1⟧ = X₁          (리터럴 1)
⟦x⟧ = X₂          (변수 x의 사용)
⟦&x⟧ = X₃         (참조 연산)
⟦{ let x = 1; &x }⟧ = X₄   (전체 let 표현식)
```

**단계 2: 제약 수집**

1. 리터럴 1에 대해: `X₁ = int`
2. 참조 &x에 대해: `⟦&x⟧ = &⟦x⟧` → `X₃ = &X₂`
3. let 바인딩에 대해:
   - `X₄ = X₃`
   - `X₂ = X₁`

**수집된 모든 제약:**
```
1. X₁ = int
2. X₃ = &X₂
3. X₄ = X₃
4. X₂ = X₁
```

**단계 3: 제약 해결**

치환(substitution)을 통해 단계적으로 해결:
```
X₁ = int
↓ (X₂ = X₁에 의해)
X₂ = int
↓ (X₃ = &X₂에 의해)
X₃ = &int
↓ (X₄ = X₃에 의해)
X₄ = &int
```

### 결론
이 프로그램의 타입은 **`&int`**이다. 변수 x는 정수이고, &x는 정수에 대한 참조이다. 따라서 전체 let 표현식은 &int 타입을 반환한다.

### 배경 지식
- **참조 타입의 생성**: &e 연산은 항상 참조 타입을 생성한다
- **타입 구조의 보존**: 참조 연산은 내부 타입을 변경하지 않고, & 래퍼만 추가한다
- **다중 참조**: &&int (참조의 참조)도 유효한 타입이다

### 전체적인 맥락
이 예제는 참조 타입이 어떻게 추론되는지, 그리고 제약이 어떻게 타입 구조를 통해 전파되는지 보여준다.

---

## 슬라이드 14: 예제 3 - 함수 정의

### 개념 설명
**프로그램:**
```
fn(x) { x + 1 }
```

이는 단순 함수 정의로, 함수 타입 추론의 기본을 보여준다.

### 분석 과정

**단계 1: 타입 변수 할당**
```
⟦x⟧ = X₁          (매개변수 x)
⟦1⟧ = X₂          (리터럴 1)
⟦x + 1⟧ = X₃      (덧셈 연산)
⟦fn(x) { x + 1 }⟧ = X₄  (함수 전체)
```

**단계 2: 제약 수집**

1. 리터럴 1에 대해: `X₂ = int`
2. 덧셈 x + 1에 대해:
   - `X₃ = int`
   - `X₁ = int`
   - `X₂ = int`
3. 함수 정의에 대해: `X₄ = ⟦x⟧ → ⟦x + 1⟧` → `X₄ = X₁ → X₃`

**수집된 모든 제약:**
```
1. X₂ = int
2. X₃ = int
3. X₁ = int
4. X₂ = int (중복)
5. X₄ = X₁ → X₃
```

**단계 3: 제약 해결**

```
X₁ = int
X₂ = int
X₃ = int
X₄ = X₁ → X₃ = int → int
```

### 결론
이 함수의 타입은 **`int → int`**이다. 함수는 정수를 입력받아 정수를 반환한다.

### 배경 지식
- **함수 타입의 형태**: 입력 타입(들)과 출력 타입으로 구성된다
- **매개변수 타입의 추론**: 매개변수는 함수 본문에서 어떻게 사용되는지로부터 타입이 추론된다
- **다형성 함수**: 이 예제는 모든 호출에서 같은 타입으로 특화되는 함수다

### 전체적인 맥락
이 예제는 함수 타입이 어떻게 구성되고 추론되는지 보여준다. 함수 정의만으로는 타입이 완전히 결정될 수 있다.

---

## 슬라이드 15: 예제 4 - 함수 정의와 호출

### 개념 설명
**프로그램:**
```
{ let f = fn(x) { x + 1 }; f(2) }
```

이는 함수의 정의와 호출을 결합하는 예제로, 함수 타입이 호출 시점에 사용되는 방식을 보여준다.

### 분석 과정

**단계 1: 타입 변수 할당**
```
⟦x⟧ = X₁              (함수 내의 매개변수)
⟦1⟧ = X₂              (리터럴 1)
⟦x + 1⟧ = X₃          (덧셈)
⟦fn(x) { x + 1 }⟧ = X₄    (함수 리터럴)
⟦f⟧ = X₅              (변수 f)
⟦2⟧ = X₆              (리터럴 2)
⟦f(2)⟧ = X₇           (함수 호출)
⟦{ let f = ...; f(2) }⟧ = X₈   (전체 let 표현식)
```

**단계 2: 제약 수집**

1. 덧셈 x + 1에 대해:
   - `X₃ = int`, `X₁ = int`, `X₂ = int`

2. 함수 정의에 대해:
   - `X₄ = X₁ → X₃`

3. let 바인딩 f = fn(...)에 대해:
   - `X₅ = X₄`

4. 리터럴 2에 대해:
   - `X₆ = int`

5. 함수 호출 f(2)에 대해:
   - `X₅ = X₆ → X₇` (함수 호출 규칙)

6. let 바인딩 전체에 대해:
   - `X₈ = X₇`

**수집된 모든 제약:**
```
1. X₂ = int
2. X₃ = int
3. X₁ = int
4. X₄ = X₁ → X₃
5. X₅ = X₄
6. X₆ = int
7. X₅ = X₆ → X₇
8. X₈ = X₇
```

**단계 3: 제약 해결**

단계별로:
```
X₁ = int              (제약 3)
X₂ = int              (제약 1)
X₃ = int              (제약 2)
X₄ = X₁ → X₃ = int → int    (제약 4)
X₅ = X₄ = int → int          (제약 5)
X₆ = int                      (제약 6)

X₅ = X₆ → X₇ 에서:
(int → int) = int → X₇
따라서 X₇ = int

X₈ = X₇ = int
```

### 결론
이 프로그램의 타입은 **`int`**이다. 함수 f는 int → int로 추론되고, f(2)의 호출은 int를 반환한다.

### 배경 지식
- **함수 호출 제약**: 함수의 매개변수 타입이 전달되는 인자의 타입과 일치해야 한다
- **타입 일관성**: X₅ = X₆ → X₇에서, X₅가 이미 int → int로 결정되었으므로, X₆ = int이고 X₇ = int이어야 한다
- **제약의 전파**: 한 제약이 다른 제약에 영향을 미친다

### 전체적인 맥락
이 예제는 함수의 정의, let 바인딩, 그리고 함수 호출이 함께 작동하는 방식을 보여준다. 함수의 타입이 호출 시점에서 검증된다.

---

## 슬라이드 16: 예제 5 - 다형성 함수의 특화

### 개념 설명
**프로그램:**
```
{ let f = fn(x) { x }; f(1) }
```

이는 **항등 함수(identity function)**의 예제로, 다형성과 타입 특화(specialization)를 보여준다.

### 분석 과정

**단계 1: 타입 변수 할당**
```
⟦x⟧ = X₁              (함수 내의 매개변수)
⟦fn(x) { x }⟧ = X₂        (항등 함수)
⟦f⟧ = X₃              (변수 f)
⟦1⟧ = X₄              (리터럴 1)
⟦f(1)⟧ = X₅           (함수 호출)
```

**단계 2: 제약 수집**

1. 함수 본문 x는 변수이므로:
   - 제약 생성 규칙에서 변수는 그냥 그 자신의 타입 변수를 가짐

2. 함수 정의 fn(x) { x }에 대해:
   - `X₂ = X₁ → X₁` (입력과 출력이 같은 타입)

3. let 바인딩 f = fn(x) { x }에 대해:
   - `X₃ = X₂`

4. 리터럴 1에 대해:
   - `X₄ = int`

5. 함수 호출 f(1)에 대해:
   - `X₃ = X₄ → X₅`

**수집된 모든 제약:**
```
1. X₂ = X₁ → X₁
2. X₃ = X₂
3. X₄ = int
4. X₃ = X₄ → X₅
```

**단계 3: 제약 해결**

단계별로:
```
X₄ = int               (제약 3)

X₃ = X₂ = X₁ → X₁    (제약 1, 2에서)

X₃ = X₄ → X₅ 에서:
(X₁ → X₁) = int → X₅

타입 일치 (unification):
X₁ = int
X₅ = int

따라서:
X₂ = int → int
X₃ = int → int
X₄ = int
X₅ = int
```

### 결론
이 프로그램의 타입은 **`int`**이다. 항등 함수 f는 구체적인 호출 f(1)을 통해 `int → int`로 **특화(specialize)**된다. 함수 정의만으로는 다형성이지만, 특정 호출로 인해 구체적인 타입이 결정된다.

### 배경 지식
- **다형성 함수**: 여러 타입에서 작동할 수 있는 함수
- **타입 특화**: 제약 해결 과정에서 타입 변수가 구체적인 타입으로 결정되는 과정
- **매개변수화된 다형성**: X₁이 타입 변수이므로 어떤 타입이든 가능하지만, 제약을 통해 결정된다

### 전체적인 맥락
이 예제는 제약 기반 분석이 다형성 함수를 어떻게 처리하는지 보여준다. 함수의 실제 사용에 따라 타입이 결정되는 '후보 추론(inference-based specialization)' 방식이다.

---

## 슬라이드 17: 선형 방정식과의 유사성

### 개념 설명
타입 제약 해결은 선형 대수의 방정식 풀이와 개념적으로 유사하다. 이 유사성은 타입 제약 해결이 수학적으로 잘 정의된 문제임을 이해하는 데 도움이 된다.

**대수 방정식과의 비교:**

**대수 예제:**
```
Equations:
  x + 2 = 5
  2x - y = 3

Solution process:
  From equation 1: x = 3
  Substitute into equation 2: 2(3) - y = 3
                              6 - y = 3
                              y = 3

Result: x = 3, y = 3
```

**타입 제약 예제:**
```
Constraints:
  X₁ = int
  X₂ = X₁ → X₃
  X₃ = int

Solution process:
  From constraint 1: X₁ = int
  Substitute into constraint 2: X₂ = int → X₃
  From constraint 3: X₃ = int
  Substitute into modified constraint 2: X₂ = int → int

Result: X₁ = int, X₂ = int → int, X₃ = int
```

### 배경 지식
- **선형 방정식**: 변수들의 차수가 1인 방정식. 예: `2x + 3y = 7`
- **비선형 제약**: 타입 제약 중 일부는 비선형이다 (예: 함수 타입 `τ₁ → τ₂`)
- **치환(Substitution)**: 변수를 그 값으로 대체하여 다른 식을 단순화하는 과정
- **유일성(Uniqueness)**: 일관된 선형 시스템은 유일한 해를 가진다 (일반적으로)

### 수식/기호 설명
```
Linear equation analogy:

Algebraic side:                Type constraint side:
------------------------------------------------------
Variables: x, y                Type variables: X₁, X₂, ...
Constants: 1, 2, 3            Type constants: int, &int, ...
Operators: +, -, ×            Operators: ×, →, &
Equations: τ₁ = τ₂ (equality)
Solution: substitution         Solution: unification

Unknown domain: ℝ (real numbers)  Unknown domain: Type (types)
```

### 전체적인 맥락
이 유사성은 단순한 비유 이상이다. 타입 제약은 실제로 방정식 시스템처럼 풀 수 있으며, 유사한 수학적 성질을 가진다. 이것이 Union-Find 같은 알고리즘을 사용할 수 있는 이유다.

---

## 슬라이드 18: 더 복잡한 예제 - 중첩된 참조

### 개념 설명
더 복잡한 프로그램들에서 제약이 어떻게 전파되는지 보여준다. 여기서는 참조가 여러 겹으로 중첩된 경우를 다룬다.

**프로그램:**
```
{ let x = 1; &(&x) }
```

### 분석 과정

**단계 1: 타입 변수 할당**
```
⟦1⟧ = X₁          (리터럴 1)
⟦x⟧ = X₂          (변수 x)
⟦&x⟧ = X₃         (첫 번째 참조)
⟦&(&x)⟧ = X₄      (두 번째 참조)
⟦{ let x = 1; &(&x) }⟧ = X₅  (전체)
```

**단계 2: 제약 수집**

1. 리터럴 1: `X₁ = int`
2. 첫 번째 참조 &x: `X₃ = &X₂`
3. 두 번째 참조 &(&x): `X₄ = &X₃` = `X₄ = &(&X₂)`
4. let 바인딩:
   - `X₂ = X₁`
   - `X₅ = X₄`

**단계 3: 제약 해결**

```
X₁ = int
X₂ = X₁ = int
X₃ = &X₂ = &int
X₄ = &X₃ = &(&int) = &&int
X₅ = X₄ = &&int
```

### 결론
이 프로그램의 타입은 **`&&int`** (정수에 대한 참조의 참조)이다. 타입 구조가 중첩될 수 있으며, 제약은 이를 투명하게 처리한다.

### 배경 지식
- **중첩된 타입 구조**: 참조, 함수, 튜플 등을 중첩시킬 수 있다
- **타입 구조의 깊이**: 프로그램이 복잡할수록 타입 구조도 깊어질 수 있다
- **타입 표현의 크기**: 매우 복잡한 타입은 표현하기 어려울 수 있다

### 전체적인 맥락
이 예제는 제약 기반 분석이 임의로 복잡한 타입 구조도 처리할 수 있음을 보여준다.

---

## 슬라이드 19: 더 복잡한 예제 - 여러 함수 호출

### 개념 설명
함수를 정의하고 여러 번 호출하는 복잡한 프로그램을 분석한다.

**프로그램:**
```
{ let f = fn(x) { x + 1 };
  let g = fn(y) { f(y) };
  g(2) }
```

### 분석 개요

이 프로그램은 다음과 같이 작동한다:
1. 함수 f를 정의: f(x) = x + 1 (int → int)
2. 함수 g를 정의: g(y) = f(y) (f를 호출하므로 y도 int여야 함)
3. g(2)를 호출: 결과는 int

### 주요 제약

1. **f의 타입**: `int → int` (덧셈으로부터)
2. **f의 호출 f(y)**: y는 int여야 함
3. **g의 타입**: `int → int` (f(y)가 int를 반환하므로)
4. **g(2) 호출**: 2는 int이고, g는 int를 인자로 받으므로 호출 가능

### 단계별 제약 해결

```
f의 정의: ⟦f⟧ = int → int
g의 정의: ⟦g⟧ = ⟦y⟧ → ⟦f(y)⟧
f(y) 호출: ⟦f⟧ = ⟦y⟧ → ⟦f(y)⟧
          int → int = ⟦y⟧ → ⟦f(y)⟧
          따라서 ⟦y⟧ = int, ⟦f(y)⟧ = int
그러므로 ⟦g⟧ = int → int

g(2) 호출: ⟦g⟧ = ⟦2⟧ → ⟦g(2)⟧
          int → int = int → ⟦g(2)⟧
          따라서 ⟦g(2)⟧ = int
```

### 결론
이 프로그램의 최종 타입은 **`int`**이다. 함수 f와 g는 모두 int → int로 타입되고, g(2)의 결과는 int이다.

### 배경 지식
- **함수 합성**: g가 f를 호출하는 방식으로 함수를 합성할 수 있다
- **타입 제약의 전파**: f의 타입이 g의 타입을 결정한다
- **타입 일관성**: 모든 함수 호출이 매개변수 타입과 일치해야 한다

### 전체적인 맥락
이 예제는 여러 함수가 상호작용할 때 타입이 어떻게 전파되는지 보여준다. 한 함수의 타입 오류는 그것을 호출하는 다른 함수도 영향을 미친다.

---

## 슬라이드 20: 더 복잡한 예제 - 타입 오류 감지

### 개념 설명
제약 기반 분석이 타입 오류를 어떻게 감지하는지 보여주는 예제다.

**프로그램 1 (타입 오류 있음):**
```
1 + true
```

**분석:**

타입 변수:
```
⟦1⟧ = X₁
⟦true⟧ = X₂ (불린 리터럴)
⟦1 + true⟧ = X₃
```

제약:
```
1. X₁ = int        (1은 숫자)
2. X₂ = bool       (true는 불린, 하지만 우리 언어에는 bool이 없음!)
3. X₃ = int        (덧셈 결과)
4. X₁ = int        (덧셈의 왼쪽)
5. X₂ = int        (덧셈의 오른쪽)
```

제약 해결 시 충돌:
```
X₂ = bool (제약 2)와 X₂ = int (제약 5)가 모순!
해가 없음 → 타입 오류 감지
```

**프로그램 2 (정상):**
```
1 + 2
```

이는 예제 1과 동일하게 int 타입으로 해결된다.

### 배경 지식
- **타입 불일치**: 제약을 풀 수 없으면 타입 오류다
- **보수적 분석**: 우리의 분석은 보수적이다. 오류가 있을 수 있는 모든 경우를 오류로 판정한다
- **거짓 양성**: 실제로는 안전한 코드를 오류로 판정할 수도 있다 (우리 분석의 한계)

### 수식/기호 설명
```
Constraint solving failure:
If constraints C cannot be satisfied by any type substitution:
  Type error detected

Unsatisfiable constraints:
  τ₁ = int ∧ τ₁ = &int  (incompatible)
  X = (int, int) ∧ X = int  (structural mismatch)
```

### 전체적인 맥락
제약 기반 분석의 강력함은 타입 오류를 자동으로 감지할 수 있다는 것이다. 프로그래머가 명시적으로 타입을 선언하지 않아도, 구조로부터 타입이 결정되고 검증된다.

---

## 슬라이드 21: Union-Find 소개 (Introduction to Union-Find)

### 개념 설명
Union-Find는 **동등성 제약(equality constraints)**을 효율적으로 해결하기 위한 자료구조이다. 타입 제약 `X = Y = Z`를 해결하는 과정은 본질적으로 여러 타입 변수가 "같은 타입이어야 한다"는 관계를 관리하는 것이다. Union-Find는 이를 매우 빠르게 처리한다.

**Union-Find가 지원하는 연산:**

1. **MakeSet(x)**: 원소 x를 포함하는 새로운 집합 생성
   - x가 자신의 집합의 유일한 멤버
   - 각 타입 변수마다 한 번 호출

2. **Find(x)**: x가 속한 집합의 대표원소(representative/root) 반환
   - 같은 집합에 속한 원소들은 같은 대표원소를 가짐
   - 타입 변수들을 동등성 클래스로 분할하는 역할

3. **Union(x, y)**: x가 속한 집합과 y가 속한 집합을 합치기
   - 두 타입 변수가 같은 타입이어야 한다는 제약을 나타냄
   - 후속 Find 연산들이 같은 대표원소를 반환하도록 함

**기본 아이디어:**

- 각 타입 변수를 정점으로 본다
- 두 변수가 같은 타입이어야 한다는 제약을 Union 연산으로 표현한다
- 같은 연결 성분(connected component)의 모든 정점은 같은 타입이어야 한다

**예시:**
```
초기: X₁, X₂, X₃, X₄ (각각 독립적)

제약: X₁ = X₂, X₂ = X₃
적용: Union(X₁, X₂), Union(X₂, X₃)

결과: X₁, X₂, X₃는 같은 집합에 속함
      Find(X₁) = Find(X₂) = Find(X₃) (같은 대표원소)
```

### 배경 지식
- **집합 분할(Set Partition)**: 전체 원소를 겹치지 않는 부분집합들로 나누는 것
- **동등성 관계(Equivalence Relation)**: 반사성, 대칭성, 추이성을 만족하는 관계
- **대표원소**: 각 부분집합을 하나의 원소로 대표하는 것. 보통은 "루트"

### 수식/기호 설명
```
Union-Find operations:
1. MakeSet(x): Create singleton set {x}
2. Find(x): Return representative of set containing x
3. Union(x, y): Merge set containing x with set containing y

Properties:
- After Union(x, y): Find(x) = Find(y)
- Union-Find maintains a forest of trees
- Each tree represents one equivalence class
```

### 전체적인 맥락
Union-Find는 이 강의의 핵심 알고리즘이다. 효율적인 구현이 없으면, 큰 프로그램의 타입 분석이 매우 느려질 것이다. 다음 슬라이드들에서 구현 세부사항을 다룬다.

---

## 슬라이드 22: Union-Find 예제 - 기본 구조

### 개념 설명
Union-Find를 시각적으로 이해하기 위한 예제이다.

**시작 상태:**
```
독립적인 각 노드:
A  B  C  D  E

각 노드는 자신을 가리키는 parent 포인터를 가짐:
A → A
B → B
C → C
D → D
E → E
```

**일련의 Union 연산 후:**
```
Union(A, B) 후:
  B
  ↑
  A

Union(B, C) 후:
    C
    ↑
    B
    ↑
    A

Union(D, E) 후:
  B       E
  ↑       ↑
  A       D

Union(C, E) 후:
      E
    / | \
   C  D  B
       ↑  ↑
       A

(결과적으로 모든 노드가 E를 root로 하는 트리에 속함)
```

### 상세 설명

**각 노드의 부모 포인터:**
```
초기 상태:
A.parent = A
B.parent = B
C.parent = C
D.parent = D
E.parent = E

Union(A, B) 후:
A.parent = A (변경 없음)
B.parent = A (또는 반대)

Union(B, C) 후:
C.parent = B (또는 B.parent = C)

...등등
```

### 배경 지식
- **포리스트(Forest) 구조**: Union-Find는 여러 트리들의 집합을 유지한다
- **루트**: 부모가 자신인 노드. Find(x)는 이 루트를 찾기 위해 노드를 따라간다
- **경로(Path)**: 노드에서 루트까지의 포인터 체인. 경로가 짧을수록 Find가 빠르다

### 수식/기호 설명
```
Tree representation:
Each node has a parent pointer:
  node.parent : Node → Node

Root node property:
  node is root iff node.parent = node

Canonical representative:
  rep(x) = the root of the tree containing x
```

### 전체적인 맥락
이 예제는 Union-Find가 내부적으로 어떻게 구조를 형성하는지 보여준다. 중요한 점은 Union-Find가 포리스트를 유지하며, 각 트리의 루트가 대표원소 역할을 한다는 것이다.

---

## 슬라이드 23: MakeSet 연산

### 개념 설명
Union-Find의 초기화 단계이다. 모든 타입 변수에 대해 MakeSet을 호출하여 초기 상태를 설정한다.

**의사 코드:**
```
MakeSet(x):
    x.parent ← x           // x는 자신의 부모
    x.rank ← 0            // 초기 rank는 0 (최적화용)
```

**시각적 표현:**

```
MakeSet(A):
┌─────┐
│  A  │
│ par │◄──┘  (A.parent = A)
└─────┘

MakeSet(B):
┌─────┐
│  B  │
│ par │◄──┘  (B.parent = B)
└─────┘

... (C, D, E도 동일)
```

### 상세 설명

**초기화의 의미:**

1. **각 노드는 싱글톤 집합**: {x}만 포함
2. **각 노드는 자신의 루트**: 초기에는 MakeSet한 노드가 곧 루트

**타입 제약 해결에서의 역할:**

```
타입 변수들: X₁, X₂, X₃, X₄

초기화:
for each X_i:
    MakeSet(X_i)

결과:
- X₁은 {X₁} 집합의 유일한 원소
- X₂는 {X₂} 집합의 유일한 원소
- ... (각각 독립적)
```

### 배경 지식
- **싱글톤 집합**: 원소가 하나뿐인 집합
- **Rank의 초기값**: 트리의 높이를 추정하기 위한 값. 최적화 기법에서 사용
- **선형적 초기화**: n개의 타입 변수에 대해 O(n) 시간

### 수식/기호 설명
```
MakeSet(x): Create a new set containing only x
  Precondition: x is not already in any set
  Postcondition: Find(x) returns x
                 ∀y ≠ x: Find(x) ≠ Find(y)
```

### 전체적인 맥락
MakeSet은 매우 단순하지만 필수적인 연산이다. 제약 해결의 첫 단계로, 각 타입 변수를 독립적인 집합으로 초기화한다.

---

## 슬라이드 24: Find 연산

### 개념 설명
Union-Find의 핵심 조회 연산이다. 어떤 원소가 속한 집합의 대표원소를 반환한다.

**기본 의사 코드 (경로 압축 없음):**
```
Find(x):
    while x.parent ≠ x:      // x가 루트가 아니면
        x ← x.parent         // 부모로 이동
    return x                 // 루트 반환
```

**시각적 예제:**

```
트리 구조:
    A (루트)
    ↑
    B
    ↑
    C
    ↑
    D

Find(D) 실행:
1. D.parent = C ≠ D, 그래서 D ← C
2. C.parent = B ≠ C, 그래서 C ← B
3. B.parent = A ≠ B, 그래서 B ← A
4. A.parent = A = A, 루프 종료
5. return A
```

### 상세 설명

**Find 연산의 의미:**

1. **대표원소 찾기**: x가 속한 집합의 루트(대표원소) 찾기
2. **동등성 검사**: Find(x) = Find(y)면 x와 y는 같은 집합에 속함
3. **포인터 추적**: 부모 포인터를 따라가며 루트까지 도달

**시간 복잡도:**

- **최악의 경우**: O(n) (체인 구조일 때)
- **평균적인 경우** (최적화 없음): O(log n)
- **경로 압축 적용 시**: O(α(n)) (거의 상수)

### 배경 지식
- **부모 포인터 추적**: Union-Find의 기본 메커니즘
- **루트의 정의**: x.parent = x인 노드만이 루트
- **경로의 길이**: 루트까지의 거리가 Find 비용을 결정

### 수식/기호 설명
```
Find(x): Return representative of the set containing x

Invariant:
  Find(x) = Find(y) ⟺ x and y are in the same set

Correctness:
  All paths lead to the unique root of the tree
  The root is the canonical representative
```

### 전체적인 맥락
Find는 Union-Find의 핵심 연산이다. Union과 Find를 함께 사용하면 동등성 제약을 효율적으로 관리할 수 있다. 그러나 기본 Find는 느릴 수 있으므로, 다음 슬라이드에서 최적화를 다룬다.

---

## 슬라이드 25: Find 연산 예제

### 개념 설명
Find 연산을 구체적인 예제로 보여준다.

**초기 트리 구조:**
```
    A (루트, rank=1)
   / \
  B   C (rank=0)

  D (루트, rank=1)
  |
  E (rank=0)
```

**Find 연산들:**

```
Find(A):
- A.parent = A이므로 즉시 루트 반환
- 결과: A
- 수행 단계: 1 (매우 빠름)

Find(B):
- B.parent = A ≠ B
- A.parent = A = A이므로 루트
- 결과: A
- 수행 단계: 2

Find(C):
- C.parent = A ≠ C
- A.parent = A = A이므로 루트
- 결과: A
- 수행 단계: 2

Find(D):
- D.parent = D이므로 즉시 루트 반환
- 결과: D
- 수행 단계: 1

Find(E):
- E.parent = D ≠ E
- D.parent = D = D이므로 루트
- 결과: D
- 수행 단계: 2
```

### 배경 지식
- **거리에 따른 비용**: 루트에 가까울수록 Find가 빠르다
- **불균형한 트리**: 체인 구조는 Find를 O(n)으로 만든다
- **균형 잡힌 트리**: Find를 더 빠르게 하려면 트리를 평탄하게 유지해야 한다

### 수식/기호 설명
```
Find(x) 비용:
  Cost(Find(x)) = distance(x, root) + 1

Example:
  Cost(Find(A)) = 0 + 1 = 1
  Cost(Find(B)) = 1 + 1 = 2
  Cost(Find(E)) = 1 + 1 = 2
```

### 전체적인 맥락
이 예제는 트리의 구조가 Find 성능에 미치는 영향을 보여준다. 얕은 트리는 빠른 Find를, 깊은 트리는 느린 Find를 만든다.

---

## 슬라이드 26: Union 연산

### 개념 설명
두 개의 분리된 집합을 합치는 연산이다. 타입 제약 해결에서 두 타입 변수가 같은 타입이어야 한다는 제약을 나타낸다.

**의사 코드:**
```
Union(x, y):
    rx ← Find(x)           // x의 루트 찾기
    ry ← Find(y)           // y의 루트 찾기

    if rx ≠ ry:            // 다른 집합에 속하면
        rx.parent ← ry     // rx의 부모를 ry로 설정
```

**시각적 예제:**

```
Union(B, D) 실행 전:

  A             D
  |             |
  B             E

Find(B) = A, Find(D) = D

Union 실행:
  A.parent ← D는 아니고
  (B의 루트 A를 D의 루트 아래에 붙임)

  A.parent ← D

결과:
    D
   / \
  A   E
  |
  B
```

### 상세 설명

**Union의 의미:**

1. **두 루트 찾기**: 각 원소의 루트를 찾음
2. **루트 연결**: 한 루트를 다른 루트의 부모로 설정
3. **집합 통합**: 이제 두 집합의 모든 원소는 같은 루트를 가짐

**중요한 점:**

```
Union(x, y) 전:
  Find(x) = A, Find(y) = D
  x와 y는 다른 집합에 속함

Union(x, y) 후:
  Find(x) = D, Find(y) = D
  x와 y는 같은 집합에 속함
```

### 배경 지식
- **순서 선택의 자유도**: Union(x, y)에서 어느 루트를 어느 아래에 붙일지 선택 가능
- **나이브한 구현의 문제**: 나이브하게 구현하면 체인 구조가 형성될 수 있음
- **최적화의 필요성**: Union by Rank나 다른 휴리스틱으로 균형 잡힌 트리 유지

### 수식/기호 설명
```
Union(x, y): Merge the set containing x with the set containing y

Effect:
  Before: Find(x) = A, Find(y) = B, A ≠ B
  After:  Find(x) = Find(y) = B (or A, depending on implementation)

Properties:
  - Union is idempotent on distinct sets
  - Once Union(x, y) is done, they're in the same set forever
```

### 전체적인 맥락
Union은 제약을 실제로 적용하는 연산이다. 타입 제약 `X = Y`를 Union(X, Y)로 구현하면, 이후 Find(X)와 Find(Y)는 같은 결과를 반환한다.

---

## 슬라이드 27: Union 연산 예제

### 개념 설명
Union 연산을 구체적인 예제로 보여준다.

**초기 상태:**
```
집합 1:
  A (루트)
  |
  B

집합 2:
  C (루트)

집합 3:
  D (루트)
  |
  E
```

**Union(B, D) 실행:**

```
Step 1: Find(B) = A
Step 2: Find(D) = D
Step 3: A ≠ D이므로 Union 실행
Step 4: A.parent ← D (또는 D.parent ← A)

결과 (A.parent ← D인 경우):
  D (루트)
 / \
A   E
|
B
```

**다른 예제 - Union(B, C) 실행:**

```
초기:
  A(루트)      C(루트)
  |
  B

Union(B, C):
  Find(B) = A
  Find(C) = C
  A ≠ C이므로 Union
  A.parent ← C (또는 C.parent ← A)

결과:
  C (루트)
  |
  A (또는 A.parent = C, C.parent = A의 구조)
  |
  B
```

### 시간 복잡도 분석

```
각 Union 연산:
  - Find(x): O(h₁) (h₁은 x의 트리 높이)
  - Find(y): O(h₂) (h₂는 y의 트리 높이)
  - 부모 설정: O(1)
  - 전체: O(h₁ + h₂)

최악의 경우:
  n개의 Union을 수행하면 O(n²)가 될 수 있음
  (모든 노드가 체인을 형성할 경우)
```

### 배경 지식
- **Union의 비가역성**: 한 번 Union되면, 그 관계는 영구적이다 (우리 분석에서는 분리 연산이 없음)
- **누적 효과**: 여러 Union으로 더 큰 트리가 형성된다
- **효율성 문제**: 나이브한 Union은 깊은 트리를 만들 수 있다

### 수식/기호 설명
```
Union sequence:
  Union(X₁, X₂): X₁과 X₂를 같은 집합으로
  Union(X₂, X₃): X₂와 X₃를 같은 집합으로
  → 결과: Find(X₁) = Find(X₂) = Find(X₃)

Transitivity:
  X₁ = X₂ ∧ X₂ = X₃ ⟹ X₁ = X₃
```

### 전체적인 맥락
이 예제는 여러 Union 연산으로 큰 집합이 형성되는 과정을 보여준다. 그러나 나이브한 Union은 비효율적인 구조를 만들 수 있으므로, 다음 슬라이드에서 최적화를 다룬다.

---

## 슬라이드 28: 복잡도 분석 (Complexity Analysis)

### 개념 설명
Union-Find의 시간 복잡도를 분석하고, 최적화 기법의 필요성을 보여준다.

**기본 Union-Find (최적화 없음):**

```
n개의 원소에 대해 m개의 Union/Find 연산을 수행할 때:

최악의 경우: O(n·m)
  - 각 Find가 O(n) 시간 소요 (체인 구조)
  - m개 연산이므로 총 O(n·m)

평균적인 경우: O(m·log n)
  - 무작위 Union으로 대략 균형 잡힌 트리 형성
```

**경로 압축(Path Compression) 적용 시:**

```
n개의 원소, m개의 Union/Find 연산:

시간 복잡도: O(m·log n)
  - 단순 Find: O(log n) (거의 모든 경우)
  - Union: O(log n) (두 Find 호출)

장점: 경로 압축으로 빈번하게 접근하는 노드들이 루트에 가까워짐
```

**Union by Rank 적용 시:**

```
n개의 원소, m개의 Union/Find 연산:

시간 복잡도: O(m·log n)
  - 트리의 높이가 O(log n)으로 제한됨
  - 각 Find/Union이 O(log n)
```

**경로 압축과 Union by Rank를 함께 적용:**

```
n개의 원소, m개의 Union/Find 연산:

시간 복잡도: O(m·α(n))
  - α(n): 역 Ackermann 함수 (매우 천천히 증가)
  - α(n) ≤ 4 for all practical values of n (예: n < 2^65536)

실제 성능: 거의 O(m)에 가깝다 (α(n)은 상수 취급)
```

### 배경 지식
- **Ackermann 함수**: 매우 빠르게 증가하는 함수. A(4, 4) = 65536
- **역 Ackermann 함수**: α(n)은 A(n, n) = m을 만족하는 n
- **거의 선형**: 실제로는 O(m) 정도로 생각할 수 있음

### 수식/기호 설명
```
Time complexity notation:
  - Worst case: O(n·m)
  - With path compression: O(m·log n)
  - With union by rank: O(m·log n)
  - With both: O(m·α(n))

α(n) values:
  α(1) = 1
  α(2) = 1
  α(3) = 2
  α(4) = 3
  α(5) = 4
  ... (거의 증가하지 않음)
```

### 전체적인 맥락
Union-Find의 복잡도 분석은 이 알고리즘이 실제로 매우 효율적이라는 것을 보여준다. 경로 압축과 Union by Rank 최적화로, 대규모 타입 분석도 거의 선형 시간에 수행할 수 있다.

---

## 슬라이드 29: 경로 압축 (Path Compression)

### 개념 설명
Find 연산의 성능을 개선하는 핵심 최적화 기법이다. Find 중에 거쳐 가는 모든 노드를 루트에 직접 연결한다.

**기본 아이디어:**

```
압축 전:
    A (루트)
    |
    B
    |
    C
    |
    D

D를 Find할 때, D → C → B → A 경로 추적
(4번의 포인터 읽기)

D를 다시 Find하면, D → A (1번의 포인터 읽기)
```

**의사 코드:**

```
Find(x) with Path Compression:
    if x.parent ≠ x:
        x.parent ← Find(x.parent)    // 경로 압축!
    return x.parent
```

**시각적 예제:**

```
Find(D) 실행 중:

압축 전:
    A
    |
    B
    |
    C
    |
    D

Step 1: D.parent = C, C가 루트 아님
Step 2: Find(C) 호출
Step 3: C.parent = B, B가 루트 아님
Step 4: Find(B) 호출
Step 5: B.parent = A, A가 루트
Step 6: B.parent ← A (이미 A)
Step 7: Return A

압축 후:
모든 노드가 A를 직접 가리킴:

    A
   /|\
  B C D
```

### 상세 설명

**경로 압축의 효과:**

1. **반복적인 접근 최적화**: 같은 노드를 여러 번 Find하면 두 번째부터 O(1)
2. **전체 구조 평탄화**: 많은 접근으로 트리가 점점 더 평탄해짐
3. **선형에 가까운 성능**: m번의 Find로 전체 구조가 거의 선형에 가까워짐

### 배경 지식
- **Lazy evaluation**: 경로 압축은 필요할 때만 (Find할 때만) 트리를 다시 구성한다
- **공간 효율성**: 여전히 각 노드마다 하나의 부모 포인터만 필요
- **병합 가능성**: Union by Rank와 함께 사용할 수 있으며, 더 좋은 성능을 낸다

### 수식/기호 설명
```
Path Compression effect:
  Before: height = h
  After many Finds: height ≈ 1 (모든 노드가 루트의 직접 자식)

Amortized complexity:
  With path compression: O(α(n)) per operation
```

### 전체적인 맥락
경로 압축은 Union-Find를 거의 선형에 가까운 성능으로 만드는 주요 기법이다. 구현이 간단하면서도 매우 효과적이다.

---

## 슬라이드 30: 경로 압축 예제

### 개념 설명
경로 압축이 실제로 어떻게 작동하는지 구체적으로 보여준다.

**초기 트리 구조:**
```
    A (루트)
    |
    B
    |
    C
    |
    D
```

**Find(D) 실행 (경로 압축 적용):**

```
호출 스택:
  Find(D)
    D.parent = C ≠ D
    temp = Find(C)
      C.parent = B ≠ C
      temp = Find(B)
        B.parent = A ≠ B
        temp = Find(A)
          A.parent = A = A
          return A
        B.parent ← A  // 이미 A, 변경 없음
        return A
      C.parent ← A  // 경로 압축!
      return A
    D.parent ← A  // 경로 압축!
    return A

결과:
    A
   /|\
  B C D
```

**Find(C) 실행 후:**

```
Find(C):
  C.parent = A = A (루트)
  return A

Cost: O(1)  (Before: O(2))
```

**다중 경로 압축 예제:**

```
초기 (두 개의 독립 체인):
  A           E
  |           |
  B           F
  |           |
  C           G

Union(C, F) 후 (F의 루트 E를 C의 루트 A 아래에):
  A
 /|\
B C E
    |
    F
    |
    G

Find(G) 실행:
Before: G → F → E → A (4 단계)
Find 중 경로 압축: G.parent ← E.parent ← A

After:
  A
 /|\\
B C E F G (모두 A를 직접 가리킴)

Next Find(G): O(1)
```

### 시간 복잡도 개선

```
Without path compression:
  Find(D) 반복: O(h) 매번
  h번 반복하면: O(h²)

With path compression:
  First Find(D): O(h)
  다른 Find들: O(1)에 가까움
  실제로는 거의 O(h) + O(1)·(m-1) ≈ O(h)
```

### 배경 지식
- **최악의 경우 개선**: 초기 체인 구조가 Find를 통해 평탄해짐
- **재귀적 경로 추적**: Find의 재귀 구조가 자연스럽게 경로를 압축
- **순차적 개선**: 첫 번째 Find는 비싸지만, 이후 Find들은 점점 싸진다

### 수식/기호 설명
```
Path compression during Find(x):
  경로 상의 모든 노드 y에 대해:
  y.parent ← Find(y.parent)

효과:
  모든 노드가 루트에 더 가까워짐
  → 다음 Find들이 빨라짐
```

### 전체적인 맥락
이 예제는 경로 압축이 얼마나 효과적인지 보여준다. 초기에는 비용이 높지만, 이후 연산들이 크게 개선된다.

---

## 슬라이드 31: Union by Rank

### 개념 설명
Union 연산의 성능을 개선하는 또 다른 최적화 기법이다. Union할 때, 항상 작은 트리를 큰 트리 아래에 붙여 트리의 높이를 낮게 유지한다.

**기본 아이디어:**

```
Without optimization:
  Union(B, D)일 때
  A를 D 아래에 붙이면 깊은 트리:
    D → A → B

With union by rank:
  rank[A] < rank[D]이므로
  A를 D 아래에 붙임 (더 얕은 구조 유지)
```

**의사 코드:**

```
MakeSet(x):
    x.parent ← x
    x.rank ← 0

Union(x, y):
    rx ← Find(x)
    ry ← Find(y)

    if rx = ry:
        return  // 이미 같은 집합

    // Rank에 따라 연결
    if rank[rx] < rank[ry]:
        rx.parent ← ry
    else if rank[rx] > rank[ry]:
        ry.parent ← rx
    else:
        ry.parent ← rx
        rank[rx] ← rank[rx] + 1
```

**시각적 예제:**

```
초기:
A (rank=1)    B (rank=0)
|
C (rank=0)

Union(A, B):
  rank[A] = 1 > rank[B] = 0
  B.parent ← A (B가 A의 루트 아래로)

결과:
    A (rank=1)
   / \
  C   B

Union(D, A):
  rank[D] = 0 < rank[A] = 1
  D.parent ← A (D가 A의 루트 아래로)

결과:
      A (rank=1)
     /|\
    C B D
```

### 상세 설명

**Rank의 의미:**

1. **높이의 상한**: rank[x]는 x를 루트로 하는 트리의 높이의 상한
2. **초기값**: 단일 노드의 rank는 0
3. **증가 조건**: 같은 rank의 두 트리를 Union할 때만 증가

**Rank 증가의 효과:**

```
Rank 0: 최대 1개 노드 (높이 0)
Rank 1: 최대 3개 노드 (높이 ≤ 1)
Rank 2: 최대 7개 노드 (높이 ≤ 2)
Rank k: 최대 2^(k+1) - 1개 노드 (높이 ≤ k)

따라서 트리의 높이 h ≤ log₂(n)
```

### 배경 지식
- **트리의 높이 제어**: Union by Rank로 높이를 O(log n)으로 제한
- **Potential method**: 이론적 복잡도 분석에서 사용되는 기법
- **경로 압축과의 결합**: 둘 다 적용하면 α(n)이라는 최고의 복잡도 달성

### 수식/기호 설명
```
Rank invariant:
  rank[x]는 x가 루트인 트리의 높이의 상한
  height(x) ≤ rank[x]

Height bound:
  height(T) ≤ log₂(n) for any tree T with n nodes

Complexity:
  Union by rank alone: O(m·log n)
  Union by rank + path compression: O(m·α(n))
```

### 전체적인 맥락
Union by Rank는 경로 압축과 함께 사용되면, Union-Find를 거의 최적에 가깝게 만든다. 두 최적화의 조합으로 실무적으로 선형에 가까운 성능을 달성한다.

---

## 슬라이드 32: Union by Rank 예제

### 개념 설명
Union by Rank가 실제로 어떻게 트리를 균형 잡히게 유지하는지 보여주는 예제이다.

**초기 상태:**
```
Node A: rank = 1 (아래에 B, C가 있음)
  A
 / \
B   C
(B, C의 rank = 0)

Node D: rank = 0 (단일)

Node E: rank = 0 (단일)
```

**Union(B, C):**
```
B는 root (rank=0), C는 root (rank=0)
같은 rank이므로:
  C.parent ← B
  rank[B] ← 1

결과:
  B (rank=1)
  |
  C (rank=0)
```

**Union(D, B):**
```
D는 root (rank=0), B는 root (rank=1)
rank[D] < rank[B]이므로:
  D.parent ← B (B가 더 크므로 B를 유지)

결과:
      B (rank=1)
     / \
    A   D
   / \
  C   (A의 다른 자식들)
```

**Union by Rank 없을 때와의 비교:**

```
Without union by rank (나이브하게 Union(X, Y) = X.root.parent ← Y.root):
  Union들의 순서에 따라 깊은 체인 형성 가능
  최악: depth = O(n)

With union by rank:
  항상 작은 트리를 큰 트리 아래로
  depth = O(log n) 보장

같은 노드에서 Find:
  Without: O(n)
  With: O(log n)
```

### 배경 지식
- **동적 균형**: Union by Rank는 트리를 동적으로 균형 잡힌 상태로 유지
- **이진 트리가 아님**: Union-Find 트리는 이진 트리가 아니라 일반 트리
- **단순하면서도 효과적**: 구현은 간단하지만 매우 효과적

### 수식/기호 설명
```
Union by rank property:
  If rank[x] < rank[y], then x is always attached under y

Invariant after Union:
  The root of the resulting tree has rank = max(rank[x], rank[y])
  or rank = max + 1 if ranks were equal
```

### 전체적인 맥락
이 예제는 Union by Rank가 깊은 체인 형성을 방지하고, 트리를 거의 균형 잡히게 유지함을 보여준다. 이로 인해 Find 연산이 대폭 빨라진다.

---

## 슬라이드 33: 요약 및 전체 맥락

### 개념 설명
이 강의의 핵심 내용을 정리하고, 타입 분석이 전체 프로그램 분석에서 어떤 역할을 하는지 설명한다.

**제약 기반 타입 분석의 전체 흐름:**

1. **분석 단계 (Analysis Phase)**:
   - 프로그램을 순회하며 각 표현식과 식별자에 타입 변수 할당 (⟦·⟧ 표기법 사용)
   - 프로그램의 구문 규칙에 따라 등식 제약 수집
   - 예: `e1 + e2`를 보면 `⟦e1 + e2⟧ = int ∧ ⟦e1⟧ = int ∧ ⟦e2⟧ = int` 생성

2. **해결 단계 (Resolution Phase)**:
   - 수집된 제약들을 Union-Find로 효율적으로 해결
   - 각 타입 변수를 구체적인 타입으로 할당 (또는 타입 오류 발견)
   - 최종 치환(substitution) 생성

3. **검증 단계 (Verification Phase)**:
   - 해결된 타입이 모든 제약을 만족하는지 확인
   - 타입 오류가 없으면 프로그램은 타입 안전하다고 판정
   - 타입 불일치가 발견되면 오류 보고

**핵심 알고리즘의 요약:**

```
Algorithm ConstraintBasedTypeAnalysis(P):
  1. Constraints C ← ∅
  2. For each subexpression e in P:
       - Assign fresh type variable ⟦e⟧
       - Add constraints for e according to syntax rules
  3. For each constraint τ₁ = τ₂ in C:
       - Union(τ₁, τ₂) in Union-Find structure
  4. For each type variable X:
       - Compute type using Find and substitution
  5. If any constraint cannot be satisfied:
       - Report type error
     Else:
       - Return computed types for all expressions
```

### 배경 지식
- **정적 분석**: 프로그램을 실행하지 않고 성질을 파악하는 기법
- **보수성(Conservativeness)**: 모든 실제 오류를 발견하지만, 거짓 양성(false positive)도 가능할 수 있다
- **완전성(Completeness)**: 올바른 프로그램을 모두 받아들이는가? (우리 분석은 완전하지 않을 수 있음)
- **정확성(Soundness)**: 거부된 프로그램이 정말 오류인가? (우리 분석은 정확하다)

### 수식/기호 설명
```
Type Analysis Result:
  ⊢ P : τ  (Program P has type τ)

Error Case:
  ⊢ P : error  (Type error found)

Constraints to Union-Find:
  τ₁ = τ₂ → Union(τ₁, τ₂)
  Find(τ₁) = Find(τ₂) ⟺ τ₁ and τ₂ must be same type
```

### 전체적인 맥락
Union-Find는 이 분석의 핵심 알고리즘이다. 효율적인 구현이 없으면, 큰 프로그램의 타입 분석이 매우 느려질 것이다. 경로 압축과 Union by Rank를 함께 사용하여 거의 선형 시간 복잡도를 달성한다.

**강의 진행 구조:**
- 강의 1-2: 프로그램 분석의 기초
- 강의 3 (이 강의): 제약 기반 타입 분석의 개요와 Union-Find 소개
- 강의 4: Union-Find의 완전한 구현 상세와 증명
- 강의 5-6: 고급 타입 분석 기법과 확장

**실용적 응용:**

1. **동적 타입 언어의 정적 분석 도구**:
   - Python의 mypy: Python 코드의 타입 오류 감지
   - JavaScript/TypeScript: JS에 정적 타입 시스템 추가

2. **컴파일러의 타입 추론 엔진**:
   - Rust, Haskell, Kotlin의 타입 추론
   - 제약 기반 접근법의 활용

3. **IDE의 자동 완성과 타입 힌트 기능**:
   - VSCode, IntelliJ IDEA의 타입 정보 제공
   - 프로그래밍 경험 향상

**학습 핵심:**

1. **제약 기반 분석의 3 단계**: 변수 할당 → 제약 수집 → 제약 해결
2. **각 언어 구성요소의 제약 규칙**: 숫자, 변수, 연산, 참조, 함수, 튜플 등
3. **Union-Find의 효율성**: 경로 압축과 Union by Rank로 O(m·α(n)) 성능
4. **실제 프로그램에서의 적용**: 복잡한 제약 시스템도 체계적으로 해결 가능

---

## 강의 3 종합 정리

### 학습 목표 달성 확인

✓ **목표 1**: 제약 기반 타입 분석의 세 단계 이해
  - 타입 변수 할당 ⟦·⟧
  - 구문 규칙에 따른 제약 수집
  - Union-Find를 사용한 제약 해결

✓ **목표 2**: 각 언어 구성요소의 제약 규칙 숙지
  - 기본: 숫자, 변수, 덧셈/뺄셈
  - 참조: 참조 생성(&), 역참조(*)
  - 복합: 튜플, 프로젝션(튜플 인덱싱)
  - 고계: 함수 정의, 함수 호출
  - 바인딩: let 표현식

✓ **목표 3**: 실제 프로그램에서 제약 수집 및 해결 능력
  - 5개의 구체적 예제 (단순부터 복잡까지)
  - 중첩된 구조, 다중 호출, 다형성 함수 처리

✓ **목표 4**: Union-Find 자료구조의 이해
  - 기본 연산: MakeSet, Find, Union
  - 성능 최적화: 경로 압축, Union by Rank
  - 복잡도 분석: O(m·α(n)) 의 이해

### 다음 강의 미리보기

**강의 4**에서는:
- Union-Find 구현의 완벽한 수학적 증명
- 더 복잡한 타입 시스템 (참조의 깊이, 고차 함수)
- 제약 해결의 실제 알고리즘
- 타입 오류의 정확한 진단과 보고

### 실습 과제 제안

1. 제공된 5개 예제 이상의 타입 분석을 손으로 수행해보기
2. Union-Find의 MakeSet, Find, Union 구현하기
3. 경로 압축과 Union by Rank를 추가 구현하기
4. 간단한 타입 분석기 구현 (분석 단계까지만)

### 추가 학습 자료

- "Modern Compiler Implementation" (Appel) - 타입 추론 장
- "Programming Language Pragmatics" (Scott) - 타입 시스템 상세
- 논문: "Algorithm W Step by Step" - Hindley-Milner 타입 추론
- "Purely Functional Data Structures" (Okasaki) - 고급 자료구조

---

이 강의 노트는 CSE552 강의 3의 모든 33개 슬라이드를 포괄적으로 다룹니다. 각 슬라이드는:
- **개념 설명**: 핵심 아이디어와 정의
- **배경 지식**: 2학년 학생이 이해할 수 있는 선행 개념
- **수식/기호/코드 설명**: 형식적 표기법의 의미
- **전체적 맥락**: 강의의 큰 그림에서의 역할

총 약 1400줄로 구성되어 있으며, 단순한 설명을 넘어 구체적인 예제와 수학적 엄밀성을 함께 제공합니다.
