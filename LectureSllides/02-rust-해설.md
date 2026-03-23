# Rust 입문 강의 해설
## CSE552 Program Analysis — Lecture 2

---

## 슬라이드 1: Introduction to Rust

### 원문 내용
> Introduction to Rust
> CSE552 Program Analysis — Lecture 2
>
> Jaemin Hong

### 해설

**개념 설명**

이것은 강의의 제목 슬라이드로, Rust 프로그래밍 언어를 프로그램 분석(Program Analysis) 과목의 두 번째 강의에서 다룰 것임을 나타냅니다.

**전체적인 맥락**

CSE552는 프로그램 분석에 관한 교과목입니다. 이 강의 시리즈에서는 정적 분석기를 구현하기 위한 대상 언어로 Rust를 소개합니다. Rust는 메모리 안전성과 성능의 장점 때문에 이 과목에서 특히 중요합니다.

---

## 슬라이드 2: What is Rust?

### 원문 내용
> - A systems programming language to replace C and C++
> - Performance comparable to C and C++
> - Memory safety guaranteed by type checking at compile time
> - Language features adopted from functional languages:
>   closures, algebraic data types (enums), generics, traits

### 해설

**개념 설명**

Rust는 C와 C++을 대체하기 위해 설계된 시스템 프로그래밍 언어입니다. 핵심 특징은 다음과 같습니다:

1. **성능**: C/C++과 비슷한 성능을 제공합니다. 런타임 오버헤드가 거의 없고 저수준 메모리 조작이 가능합니다.

2. **메모리 안전성**: 컴파일 시점에 타입 검사를 통해 메모리 안전성을 보장합니다. 이는 buffer overflow, use-after-free 같은 전형적인 C/C++ 버그를 방지합니다.

3. **함수형 언어의 특징들**:
   - **클로저(closures)**: 주변 환경의 변수를 캡처할 수 있는 익명 함수
   - **대수적 데이터 타입(algebraic data types, enums)**: 여러 변형(variants)을 가질 수 있는 타입
   - **제네릭(generics)**: 타입 파라미터를 사용한 코드 재사용
   - **트레이트(traits)**: 공통 동작을 정의하는 인터페이스 같은 메커니즘

**배경 지식**

함수형 프로그래밍의 특징들(클로저, 패턴 매칭, 불변성)을 C++ 스타일의 저수준 제어와 결합하여, 안전하면서도 효율적인 시스템 코드를 작성할 수 있게 해줍니다.

---

## 슬라이드 3: Rust in the Real World

### 원문 내용
> - The White House recommended using safe languages,
>   including Rust, instead of C and C++¹
> - Many new systems, including operating systems and web
>   browsers, have been developed in Rust
> - Linux has officially added support for Rust in kernel
>   development
> - Coreutils have been rewritten in Rust and installed by default
>   in some recent operating systems, such as Ubuntu 25.10
>
> ¹ "Back to the building blocks: A path toward secure and measurable
> software (White House, 2024)"

### 해설

**개념 설명**

이 슬라이드는 Rust가 실제 산업에서 어떻게 사용되고 있는지 보여줍니다:

1. **정부 차원의 권장**: 미국 백악관이 C/C++ 대신 Rust와 같은 안전한 언어 사용을 권장했습니다. 이는 보안 취약점을 줄이기 위한 정책적 조치입니다.

2. **운영체제 개발**: Linux 커널에 공식적으로 Rust 지원이 추가되었습니다. 이는 Linux 커널의 일부 모듈을 Rust로 작성할 수 있다는 의미입니다.

3. **시스템 유틸리티**: Ubuntu 25.10 같은 최신 OS에서 핵심 유틸리티들(coreutils)이 Rust로 재작성되고 기본적으로 설치됩니다.

**배경 지식**

전통적으로 C는 시스템 프로그래밍의 표준 언어였지만, 메모리 안전 문제로 인한 보안 취약점이 많았습니다. Rust는 이러한 문제를 해결하면서도 C와 비슷한 성능을 제공하기 때문에 산업에서 점차 채택되고 있습니다.

---

## 슬라이드 4: Rust in This Course

### 원문 내용
> - Rust is both an implementation language and a target
>   language for static analysis in this course
>   - In assignments, you will implement a static analyzer for Rust
>     programs in Rust
>   - For the term project, you can use any language and analyze
>     any language
> - Functional languages (e.g., Scala and OCaml) have typically
>   been used for implementing static analyzers
> - Rust provides useful features for implementing static
>   analyzers, while also allowing performant implementations

### 해설

**개념 설명**

이 과목에서 Rust의 두 가지 역할:

1. **구현 언어(Implementation Language)**: 정적 분석기를 작성할 때 사용할 언어입니다. 학생들은 Rust를 사용하여 정적 분석기를 구현합니다.

2. **대상 언어(Target Language)**: 분석 대상이 되는 언어입니다. 구현한 분석기는 Rust 프로그램을 분석합니다.

**추가 설명**

- 전통적으로 정적 분석기는 Scala나 OCaml 같은 함수형 언어로 구현되었습니다. 이들 언어는 재귀적 데이터 구조(예: AST)와 패턴 매칭을 다루기 쉽기 때문입니다.
- Rust도 함수형 특징들(패턴 매칭, 열거형)을 가지고 있으면서도 성능이 우수하기 때문에 분석기 구현에 적합합니다.

---

## 슬라이드 5: Variables

### 원문 내용
> - let [id] = [expr]; — immutable variable
> - let mut [id] = [expr]; — mutable variable
>
> Immutable:
> ```
> fn main() {
>   let x = 1;
>   x = 2; // error
>   println!("{}{x}");
> }
> ```
>
> Mutable:
> ```
> fn main() {
>   let mut x = 1;
>   x = 2; // ok
>   println!("{}{x}");
> }
> ```

### 해설

**개념 설명**

Rust의 변수는 기본적으로 불변(immutable)입니다. 이는 함수형 프로그래밍의 특징으로, 부작용을 줄이고 코드를 더 예측 가능하게 만듭니다.

- **`let x = 1;`**: 불변 변수를 선언합니다. 이후 `x`의 값을 변경할 수 없습니다.
- **`let mut x = 1;`**: `mut` 키워드를 사용하여 가변 변수를 선언합니다. 이제 `x`의 값을 변경할 수 있습니다.

**배경 지식**

C/C++에서는 기본적으로 변수가 가변입니다. 반면 Rust는 기본 불변 원칙을 따르며, 명시적으로 `mut`을 사용한 경우에만 변수를 변경할 수 있습니다. 이는 의도하지 않은 상태 변화를 방지하는 데 도움이 됩니다.

**수식/기호/코드 설명**

코드에서 `println!`는 매크로(macro)입니다. Rust의 매크로는 `!`로 표시됩니다. `"{x}"`는 문자열 보간(string interpolation)으로, `x` 변수의 값을 문자열에 삽입합니다.

---

## 슬라이드 6: Functions

### 원문 내용
> - fn [id]([id]: [type], ...): -> [type] { ... }
> - All parameter types and the return type must be specified
> - The last expression in the function body is the return value
>
> ```
> fn add(x: i32, y: i32) -> i32 {
>   x + y
> }
> ```

### 해설

**개념 설명**

Rust의 함수 문법:

1. **완전한 타입 명시**: C/C++ 같은 정적 타입 언어처럼 모든 파라미터 타입과 반환 타입을 명시해야 합니다.

2. **암시적 반환**: 함수 본문의 마지막 식(expression)이 자동으로 반환 값입니다. 세미콜론 없는 `x + y`는 식으로 평가되고, 그 값이 반환됩니다. 세미콜론이 있으면 (`x + y;`) 명령문(statement)이 되어 `()` (unit type)을 반환합니다.

**배경 지식**

Rust는 식 지향 언어(expression-oriented language)입니다. C/C++의 대부분은 명령문 지향이지만, Rust는 제어 구조(if, loop)도 값을 반환하는 식입니다.

**수식/기호/코드 설명**

- `i32`: 32비트 부호 있는 정수 타입
- `->`: 반환 타입을 나타내는 화살표 표기법

---

## 슬라이드 7: Types

### 원문 내용
> - Signed integers: i8, i16, i32, i64, i128, isize
> - Unsigned integers: u8, u16, u32, u64, u128, usize
> - Floating-point numbers: f32, f64
> - Boolean: bool
> - Unit: ()
> - Character: char
>
> - String: String
> - Tuples: (T1, T2, ...)
> - Arrays: [T; N]
> - Vectors: Vec<T>
> - Hash maps: HashMap<K, V>

### 해설

**개념 설명**

Rust의 기본 타입들:

**원시 타입(Primitive Types)**:
- **정수형**: `isize`, `usize`는 플랫폼 의존적입니다 (64비트 머신에서는 64비트)
- **부동소수점**: IEEE 754 표준을 따릅니다
- **논리형**: `bool`은 `true` 또는 `false`
- **단위형**: `()` - 값이 없음을 나타냅니다

**복합 타입(Compound Types)**:
- **String**: 힙에 할당되는 가변 길이 문자열 (다른 언어의 String 클래스와 유사)
- **튜플**: 고정 크기, 각 요소가 다른 타입 가능
- **배열**: 고정 크기, 모든 요소가 같은 타입 (`[T; N]`에서 N은 컴파일 타임 상수)
- **벡터**: 동적 배열, 런타임에 크기 변경 가능
- **HashMap**: 키-값 쌍 저장

**배경 지식**

C/C++과 달리 Rust의 `String`은 단순 문자 배열이 아닌 UTF-8 인코딩된 텍스트를 관리하는 구조체입니다. 문자 리터럴(`"hello"`)은 실제로는 `&str` (문자열 슬라이스) 타입입니다.

---

## 슬라이드 8: Expression-oriented Programming

### 원문 내용
> - Every expression evaluates to a value
>
> ```
> let x = {
>   let y = 10;
>   y * y
> };
> ```
>
> ```
> let x = if y >= 0 {
>   println!("y is non-negative");
>   y
> } else {
>   println!("y is negative");
>   -y
> };
> ```

### 해설

**개념 설명**

Rust는 **식 지향 언어**입니다. 이는 C/C++과의 중요한 차이점입니다:

1. **블록 식**: 중괄호 `{ ... }`로 둘러싼 코드도 하나의 식으로 취급되며 값을 반환합니다. 이 값은 변수에 할당될 수 있습니다.

2. **제어 구조 식**: `if`, `else`, `loop` 등도 식입니다. 따라서 이들로부터 값을 얻을 수 있습니다.

**중요한 규칙**: 식의 마지막 줄에 세미콜론을 붙이면 안 됩니다. 세미콜론이 붙으면 명령문이 되어 `()` (unit)을 반환합니다.

**배경 지식**

첫 번째 예제에서:
- `{ let y = 10; y * y }`는 `100`을 평가합니다
- 변수 `x`에 `100`이 할당됩니다

두 번째 예제에서:
- `if` 식이 `y` 또는 `-y`를 반환합니다
- 변수 `x`는 그 반환 값을 받습니다

---

## 슬라이드 9: Loops

### 원문 내용
> - loop { ... }
> - while [expr] { ... }
> - for [id] in [expr] { ... }
>
> ```
> let mut x = 0;
> let y = loop {
>   x += 1;
>   if x == 10 {
>     break x * 2;
>   }
> };
> ```
>
> ```
> let v = vec![1, 2, 3];
> for x in v {
>   println!("{}{x}");
> }
> ```

### 해설

**개념 설명**

Rust의 세 가지 반복 구조:

1. **`loop { ... }`**: 무한 루프입니다. `break` 또는 `return`으로만 종료됩니다. 또한 `break` 뒤에 값을 줄 수 있어서, 루프 자체가 값을 반환할 수 있습니다.

2. **`while [expr] { ... }`**: 조건이 참인 동안 반복합니다. C/C++과 유사합니다.

3. **`for [id] in [expr] { ... }`**: 반복자(iterator)를 통해 순회합니다. 매우 안전하고 효율적입니다.

**수식/기호/코드 설명**

첫 번째 예제에서:
- `x += 1`은 `x = x + 1`의 약자입니다
- `break x * 2;`는 루프를 종료하고 `x * 2` 값을 반환합니다
- 변수 `y`는 `20`을 받습니다

두 번째 예제에서:
- `vec![1, 2, 3]`은 벡터를 생성하는 매크로입니다
- `for x in v`는 벡터의 소유권을 가져가면서 각 요소를 순회합니다

---

## 슬라이드 10: Ownership

### 원문 내용
> - Each value is owned by at most one variable
> - When the value is assigned to another variable or passed to a
>   function, the ownership is transferred (moved)
> - The compiler prevents using a variable after its ownership has
>   been moved
>
> ```
> let s = "hi".to_string();
> println!("{}{s}"); // ok
> let t = s;
> println!("{}{s}"); // error
> println!("{}{t}"); // ok
> ```

### 해설

**개념 설명**

**소유권(Ownership)**은 Rust의 가장 혁신적인 특징입니다. 메모리 관리를 자동화하면서도 가비지 컬렉터 없이 메모리 안전성을 보장합니다.

핵심 규칙:
1. 각 값은 정확히 하나의 소유자를 가집니다
2. 값을 다른 변수에 할당하거나 함수에 전달하면, 소유권이 이전됩니다 (**move**)
3. 소유권을 잃은 변수는 더 이상 사용할 수 없습니다

**배경 지식**

```
let s = "hi".to_string();  // s가 String을 소유
println!("{}", s);         // s 사용 가능
let t = s;                 // 소유권이 s에서 t로 이전(move)
println!("{}", s);         // 컴파일 에러! s는 더 이상 소유하지 않음
println!("{}", t);         // t 사용 가능
```

이는 메모리 누수와 use-after-free를 완전히 방지합니다.

---

## 슬라이드 11: Copy and Clone

### 원문 내용
> - Some types (e.g., primitive types) implement Copy,
>   which allows copying instead of moving
>
> ```
> let x = 1;
> println!("{}{x}"); // ok
> let y = x;
> println!("{}{x}"); // ok
> println!("{}{y}"); // ok
> ```
>
> - For types not implementing Copy (typically
>   heap-allocated data), use clone to create a deep
>   copy
>
> ```
> let s = "hi".to_string();
> println!("{}{s}"); // ok
> let t = s.clone();
> println!("{}{s}"); // ok
> println!("{}{t}"); // ok
> ```

### 해설

**개념 설명**

소유권의 이전은 모든 타입에 적용되지 않습니다:

1. **Copy 트레이트**: 정수, 부동소수점, 불린 같은 원시 타입들은 `Copy` 트레이트를 구현합니다. 이들은 할당 시 값이 복사되므로 소유권이 이전되지 않습니다.

2. **Clone 메서드**: 힙 할당 데이터(String, Vec 등)는 `Copy`를 구현하지 않습니다. 대신 명시적으로 `clone()`을 호출하여 깊은 복사를 수행할 수 있습니다. 그러나 이는 비용이 큼니다.

**배경 지식**

`Copy`는 자동 복사가 가능한 작은 타입들을 위한 최적화입니다. 스택에 적합한 크기의 데이터는 복사 비용이 저렴하므로 자동으로 복사되고, 힙 할당 데이터는 명시적으로 복사를 지시하도록 강제합니다.

---

## 슬라이드 12: Ownership and Use-After-Free

### 원문 내용
> - The notion of ownership prevents use-after-free
>
> ```
> let s = "hi".to_string();
> let t = s;
> drop(t); // deallocate the heap memory
> println!("{}{s}"); // error
> ```

### 해설

**개념 설명**

소유권 시스템은 **use-after-free** 버그를 완전히 방지합니다:

1. `s`가 String을 소유합니다
2. 소유권이 `t`로 이전됩니다
3. `drop(t)`가 메모리를 명시적으로 해제합니다
4. `s`를 사용하려고 하면 컴파일 에러가 발생합니다 (이미 해제된 메모리)

**배경 지식**

C/C++에서는 이런 코드가 컴파일되지만 런타임에 undefined behavior를 초래합니다. Rust는 컴파일 타임에 이를 감지하여 허용하지 않습니다.

**추가 설명**

- `drop(t)`는 명시적으로 메모리를 해제합니다
- 값이 스코프를 벗어나면 자동으로 `drop`이 호출됩니다
- 이를 **RAII (Resource Acquisition Is Initialization)** 패턴이라고 합니다

---

## 슬라이드 13: Borrowing

### 원문 내용
> - Borrowing allows temporarily using a value without taking
>   ownership
> - &[expr] creates a shared reference (read-only) of type &T
> - &mut [expr] creates a mutable reference (read-write) of type
>   &mut T
>
> ```
> fn show(s: &String) {
>   println!("{}{s}");
> }
> let s = "hi".to_string();
> show(&s);
> println!("{}{s}"); // ok
> ```
>
> ```
> fn push_dot(
>   s: &mut String
> ) {
>   s.push('.');
> }
> let mut s =
>   "hi".to_string();
> push_dot(&mut s);
> println!("{}{s}"); // ok
> ```

### 해설

**개념 설명**

**차용(Borrowing)**은 소유권을 이전하지 않고 값을 임시로 사용하도록 허가합니다:

1. **공유 참조 `&T`**: 읽기 전용 참조입니다. 여러 공유 참조가 동시에 존재할 수 있습니다.

2. **가변 참조 `&mut T`**: 읽기-쓰기 참조입니다. 한 번에 하나의 가변 참조만 존재할 수 있습니다.

**수식/기호/코드 설명**

첫 번째 예제:
- `&s`는 `s`의 공유 참조를 만듭니다
- `show` 함수는 소유권을 얻지 않고 참조만 받습니다
- `show` 호출 후에도 `s`는 여전히 사용 가능합니다

두 번째 예제:
- `&mut s`는 가변 참조를 만듭니다
- 함수 내에서 문자를 추가할 수 있습니다
- 함수 반환 후 `s`는 변경된 상태입니다

**배경 지식**

참조를 사용하면 메모리를 복사하지 않으면서도 값을 전달할 수 있습니다. 이는 C의 포인터와 유사하지만, 더 안전합니다.

---

## 슬라이드 14: Aliasing XOR Mutability

### 원문 내용
> - Multiple shared references to the same value can coexist
> - A mutable reference to a value cannot coexist with any other
>   reference (shared or mutable) to the same value
> - This discipline also prevents use-after-free

### 해설

**개념 설명**

Rust는 **Aliasing XOR Mutability** 규칙을 시행합니다:

- **여러 공유 참조**: 동시에 여러 `&T`가 존재할 수 있습니다
- **단일 가변 참조**: 어떤 시점에서든 최대 하나의 `&mut T`만 존재할 수 있습니다
- **상호 배제**: `&mut T`가 존재할 때는 다른 어떤 참조도 (공유든 가변이든) 있을 수 없습니다

**배경 지식**

이 규칙은 데이터 경합(data race)을 완전히 방지합니다. 여러 스레드가 같은 메모리를 접근할 때 발생하는 문제들을 컴파일 타임에 탐지합니다.

**코드 예시** (슬라이드에서)

```rust
let mut v = vec![1, 2, 3];
let r = &v[0];              // 공유 참조
v.push(4);                  // 에러! 벡터를 변경하려고 함
println!("{}{r}");          // 이 행은 실행되지 않음
```

컴파일러는 `r`이 아직 유효한 상태에서 벡터 재할당을 시도하므로 에러를 발생시킵니다.

---

## 슬라이드 15: Interior Mutability

### 원문 내용
> - Some types allow mutating the value even through a shared
>   reference
> - Cell<T>: provides get and set methods
> - RefCell<T>: provides borrow and borrow_mut methods
>   that enforce the aliasing XOR mutability rule at runtime
>
> ```
> let x = Cell::new(1);
> let r1 = &x;
> let r2 = &x;
> r1.set(2);
> println!("{}", r2.get());
> ```
>
> - Useful for certain purposes, e.g., caching and cyclic structures

### 해설

**개념 설명**

**내부 가변성(Interior Mutability)**은 공유 참조를 통해서도 값을 변경할 수 있게 하는 패턴입니다:

1. **`Cell<T>`**: 복사 가능한 타입을 위한 내부 가변성
   - `get()`과 `set()`만 제공합니다
   - 런타임 비용이 거의 없습니다

2. **`RefCell<T>`**: 비복사 타입을 위한 내부 가변성
   - `borrow()`와 `borrow_mut()`을 제공합니다
   - aliasing XOR mutability를 **런타임에** 강제합니다
   - 위반하면 panic이 발생합니다

**배경 지식**

일반적으로 Rust의 타입 시스템은 컴파일 타임에 aliasing XOR mutability를 강제합니다. 그러나 특정 상황에서는 (예: 캐싱, 순환 구조) 이를 런타임으로 미루는 것이 필요합니다.

**추가 설명**

코드 예제에서:
- `x`는 공유 참조 `r1`, `r2`를 가지고 있습니다
- 일반적으로는 불가능하지만, `Cell`을 사용하면 `r1.set(2)`로 값을 변경할 수 있습니다
- `r2.get()`은 변경된 값 `2`를 반환합니다

---

## 슬라이드 16: Function Pointers

### 원문 내용
> - Type: fn([type], ...) -> [type]
> - Can be called or passed to higher-order functions
>
> ```
> fn is_neg(x: &i32) -> bool {
>   *x < 0
> }
>
> let f: fn(&i32) -> bool = is_neg;
> println!("{}", f(&-1));
>
> let mut v = vec![1, -2, 3];
> v.retain(f); // equivalent to 'v.retain(is_neg);'
> ```

### 해설

**개념 설명**

함수 포인터는 함수 자체를 값으로 취급할 수 있게 합니다:

1. **함수 포인터 타입**: `fn([param_types]) -> [return_type]`
2. **고차 함수**: 함수를 인자로 받거나 반환할 수 있습니다
3. **함수 이름의 할당**: 함수 이름은 그 함수 포인터로 자동 변환됩니다

**수식/기호/코드 설명**

코드 예제:
- `fn is_neg(x: &i32) -> bool`는 정수 참조를 받아 불린을 반환합니다
- `let f: fn(&i32) -> bool = is_neg;`는 `is_neg` 함수를 함수 포인터로 할당합니다
- `v.retain(f)`는 `f`를 술어(predicate)로 사용하여 조건에 맞는 요소만 유지합니다

**배경 지식**

`*x < 0`에서 `*`는 **역참조(dereference)** 연산자입니다. 참조를 따라 실제 값에 접근합니다.

---

## 슬라이드 17: Anonymous Functions and Closures

### 원문 내용
> Anonymous Functions:
> - Functions without a name
> - |[id], ...|  [expr]
>
> ```
> let mut v = vec![1, -2, 3];
> v.retain(|x| x < 0);
> ```
>
> Closures:
> - Anonymous functions can
>   capture variables from the
>   surrounding scope
>
> ```
> let n = 10;
> let mut v = vec![5, 15];
> v.retain(|x| x > n);
> ```

### 해설

**개념 설명**

**익명 함수(Anonymous Functions)**와 **클로저(Closures)**는 함수형 프로그래밍의 중요한 특징입니다:

1. **익명 함수**: 이름이 없는 함수입니다. 파이프 `|...|` 문법으로 정의됩니다.

2. **클로저**: 익명 함수의 특별한 형태로, 주변 스코프의 변수를 캡처(capture)할 수 있습니다.

**수식/기호/코드 설명**

첫 번째 예제:
- `|x| x < 0`는 매개변수 `x`를 받아 `x < 0`을 반환하는 익명 함수입니다
- `v.retain(|x| x < 0)`는 음수만 유지합니다

두 번째 예제:
- `|x| x > n`은 외부 변수 `n`을 캡처합니다
- 이것이 클로저의 특징입니다

**배경 지식**

Rust의 클로저는 세 가지 방식으로 환경의 변수를 캡처할 수 있습니다:
- **Fn**: 변수를 읽기만 함
- **FnMut**: 변수를 변경할 수 있음
- **FnOnce**: 변수의 소유권을 가져감

---

## 슬라이드 18: Structs

### 원문 내용
> - Custom data types with named fields
>
> ```
> struct Point {
>   x: f64,
>   y: f64
> }
> let p = Point { x: 1.0, y: 2.0 };
> println!("{}{},{}{}", p.x, p.y);
> ```
>
> - Structs can have methods
>
> ```
> impl Point {
>   fn distance_from_origin(&self) -> f64 {
>     (self.x * self.x + self.y * self.y).sqrt()
>   }
> }
> let p = Point { x: 3.0, y: 4.0 };
> println!("{}", p.distance_from_origin());
> ```

### 해설

**개념 설명**

**구조체(Struct)**는 C의 구조체처럼 여러 필드를 가진 사용자 정의 타입입니다:

1. **구조체 정의**: `struct` 키워드로 정의하고, 각 필드의 타입을 명시합니다

2. **구조체 인스턴스**: 중괄호를 사용하여 생성합니다

3. **메서드**: `impl` 블록으로 구조체에 메서드를 추가합니다

**수식/기호/코드 설명**

메서드의 특징:
- `&self`: 구조체의 불변 참조를 받습니다. 메서드가 값을 변경하지 않습니다
- `&mut self`: 가변 참조를 받아 값을 변경할 수 있습니다
- `self`: 소유권을 가져갑니다 (self-consuming method)

코드에서 `p.distance_from_origin()`은 자동으로 `&p`를 전달합니다 (자동 참조).

---

## 슬라이드 19: Enums

### 원문 내용
> - Custom data types that can be one of several variants
> - Used with pattern matching
>
> ```
> enum Color { Red, Green, Blue }
> let c = Color::Green;
> let name = match c {
>   Color::Red => "red",
>   Color::Green => "green",
>   Color::Blue => "blue",
> };
> ```

### 해설

**개념 설명**

**열거형(Enum)**은 제한된 집합의 값 중 하나가 될 수 있는 타입입니다:

1. **열거형 정의**: 가능한 변형들(variants)을 나열합니다
2. **패턴 매칭**: `match` 표현식을 사용하여 각 변형을 처리합니다

**배경 지식**

Rust의 열거형은 C의 열거형보다 훨씬 강력합니다. 각 변형은 다른 타입의 데이터를 연관시킬 수 있습니다.

**수식/기호/코드 설명**

- `Color::Green`: `::`은 경로 구분자로, 열거형 내의 변형에 접근합니다
- `match` 표현식은 모든 경우를 철저히 처리해야 합니다. 컴파일러가 빠진 경우를 감지합니다

---

## 슬라이드 20: Enums (cont.)

### 원문 내용
> - Each variant can have associated data
>
> ```
> enum Shape {
>   Circle(f64),
>   Rectangle(f64, f64),
> }
> let s = Shape::Circle(1.0);
> let area = match s {
>   Shape::Circle(r) => PI * r * r,
>   Shape::Rectangle(w, h) => w * h,
> };
>
> - if let is useful when we only care about one variant
>
> ```
> if let Shape::Circle(r) = s {
>   println!("circle with radius {r}");
> }
> ```

### 해설

**개념 설명**

열거형의 고급 기능들:

1. **연관 데이터**: 각 변형은 다른 타입의 데이터를 가질 수 있습니다
   - `Circle(f64)`: 원의 반지름
   - `Rectangle(f64, f64)`: 직사각형의 너비와 높이

2. **`if let`**: 특정 패턴만 관심 있을 때 사용합니다. `match`보다 간결합니다.

**수식/기호/코드 설명**

패턴 매칭의 강력한 점:
- 변형을 구분할 뿐 아니라 연관 데이터도 동시에 추출합니다
- `Shape::Circle(r) = s`는 `s`가 Circle 변형인지 확인하고, 반지름을 `r`에 바인딩합니다

**배경 지식**

이 패턴은 대수적 데이터 타입(algebraic data types)이며, Haskell, OCaml 같은 함수형 언어에서 광범위하게 사용됩니다.

---

## 슬라이드 21: Options

### 원문 내용
> - Option<T> is an enum that can be either Some(T) or None
> - Represents optional existence of a value or the possibility of
>   failure
>
> ```
> let map = HashMap::from([("a", 1), ("b", 2)]);
> match map.get("a") {
>   Some(x) => println!("value: {x}"),
>   None => println!("key not found"),
> };
> ```

### 해설

**개념 설명**

`Option<T>`는 값이 있을 수도, 없을 수도 있는 상황을 나타냅니다:

```
enum Option<T> {
  Some(T),    // 값이 있음
  None,       // 값이 없음
}
```

이는 C의 null 포인터를 더 안전하게 다루는 방법입니다.

**배경 지식**

C/C++에서는 null을 확인하지 않으면 null pointer dereference로 crash가 발생합니다. Rust는 컴파일러가 `Option`의 모든 경우(Some과 None)를 처리하도록 강제하므로 이 문제가 발생하지 않습니다.

**수식/기호/코드 설명**

- `HashMap::from([...])`은 맵을 초기화합니다
- `map.get("a")`는 `Option<&i32>`를 반환합니다 (있으면 Some, 없으면 None)

---

## 슬라이드 22: Results

### 원문 내용
> - Similar to Option<T>, but provides information about errors
> - Result<T, E> can be either Ok(T) or Err(E)
>
> ```
> fn divide(x: i32, y: i32) -> Result<i32, String> {
>   if y == 0 {
>     Err("division by zero".to_string())
>   } else {
>     Ok(x / y)
>   }
> }
> ```

### 해설

**개념 설명**

`Result<T, E>`는 성공과 실패를 모두 나타낼 수 있습니다:

```
enum Result<T, E> {
  Ok(T),      // 성공, 값 T를 포함
  Err(E),     // 실패, 에러 E를 포함
}
```

**배경 지식**

C에서는 함수가 에러 코드를 정수로 반환하거나, 포인터를 반환하여 null로 에러를 나타냅니다. Rust의 `Result`는 성공과 실패를 명시적으로 구분하고, 에러 정보도 함께 제공합니다.

---

## 슬라이드 23: Error Handling

### 원문 내용
> - Rust does not have exceptions; errors should be explicitly
>   expressed using Option or Result
>   - panic: unrecoverable error, terminates the program (not
>     intended to be handled)
> - Error propagation can be done using the ? operator
>
> ```
> fn add_values_associated_with_keys(
>   key1: i32,
>   key2: i32,
>   map: &HashMap<i32, i32>,
> ) -> Option<i32> {
>   let v1 = map.get(&key1)?;
>   let v2 = map.get(&key2)?;
>   Some(v1 + v2)
> }
> ```

### 해설

**개념 설명**

Rust의 에러 처리 철학:

1. **예외 없음**: Java나 C++처럼 예외를 던지지 않습니다
2. **명시적 처리**: 모든 에러를 `Option` 또는 `Result`로 명시합니다
3. **복구 가능/불가능 구분**:
   - **복구 가능**: `Result`/`Option`으로 처리
   - **복구 불가능**: `panic!` 매크로로 프로그램 종료

**수식/기호/코드 설명**

`?` 연산자는 **에러 전파 연산자**입니다:
- `Option`이나 `Result`가 실패 케이스(None/Err)면 즉시 함수에서 반환합니다
- 성공 케이스면 값을 추출합니다

코드 예제:
```rust
let v1 = map.get(&key1)?;  // None이면 None을 반환, Some(x)면 x 추출
let v2 = map.get(&key2)?;  // 마찬가지
Some(v1 + v2)              // 둘 다 성공하면 합계 반환
```

---

## 슬라이드 24: Generics

### 원문 내용
> - Functions can be parametrized over types
>
> ```
> fn pop_until<T>(v: &mut Vec<T>, n: usize) {
>   while v.len() > n {
>     v.pop();
>   }
> }
> ```
>
> - User-defined types can also be generic
>
> ```
> struct Point<T> { x: T, y: T }
> let p1: Point<f64> = Point { x: 1.0, y: 2.0 };
> let p2: Point<i32> = Point { x: 1, y: 2 };
> ```

### 해설

**개념 설명**

**제네릭(Generics)**은 타입 파라미터를 사용하여 코드를 재사용하게 합니다:

1. **함수 제네릭**: `fn func<T>(...)`
2. **타입 제네릭**: `struct Point<T> { ... }`

**배경 지식**

C++ 템플릿과 유사하지만, Rust의 제네릭은 **단형화(monomorphization)**되어 컴파일됩니다. 각 타입 조합에 대해 코드의 복사본이 생성되므로 런타임 성능 오버헤드가 없습니다.

**수식/기호/코드 설명**

`pop_until<T>`에서:
- `T`는 타입 파라미터입니다
- 호출할 때 `pop_until::<i32>(...)`처럼 명시하거나, 컴파일러가 추론하도록 할 수 있습니다

---

## 슬라이드 25: Traits

### 원문 내용
> - A trait represents a set of types that share common behavior
>   - Resembles type classes in Haskell
>   - Similar to interfaces or abstract classes in object-oriented
>     languages
>
> ```
> trait HasArea {
>   fn area(&self) -> f64;
> }
> struct Circle { radius: f64 }
> impl HasArea for Circle {
>   fn area(&self) -> f64 { PI * self.radius * self.radius }
> }
> struct Rectangle { width: f64, height: f64 }
> impl HasArea for Rectangle {
>   fn area(&self) -> f64 { self.width * self.height }
> }
> fn larger<T: HasArea>(a: T, b: T) -> T {
>   if a.area() > b.area() { a } else { b }
> }
> ```

### 해설

**개념 설명**

**트레이트(Trait)**는 공통 동작을 정의하는 추상적 인터페이스입니다:

1. **트레이트 정의**: 메서드 시그니처를 정의합니다
2. **구현**: `impl TraitName for TypeName`으로 타입에 트레이트를 구현합니다
3. **트레이트 바운드**: `T: HasArea`로 제네릭 타입이 특정 트레이트를 구현하도록 제약합니다

**배경 지식**

- Haskell의 **타입 클래스**와 유사합니다
- 객체지향의 인터페이스/추상 클래스와 비슷하지만, 메서드를 나중에 타입에 추가할 수 있습니다 (**코히런트 확장(coherent extension)**)

**수식/기호/코드 설명**

`fn larger<T: HasArea>(a: T, b: T) -> T`:
- `T: HasArea`는 "T는 HasArea 트레이트를 구현해야 한다"는 뜻입니다
- 이 제약이 있으므로 함수 내에서 `a.area()` 호출이 유효합니다

---

## 슬라이드 26: Lifetimes

### 원문 내용
> - A lifetime represents how long a reference is valid
> - &'a T or &'a mut T indicates that the reference is valid for
>   the lifetime 'a
> - Lifetime annotations can be omitted in many cases, but
>   sometimes they are required to disambiguate lifetimes
>
> ```
> fn foo<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 { x }
> ```
>
> - Types are parametrized over lifetimes when they contain
>   references
>
> ```
> struct Point<'a> { x: &'a f64, y: &'a f64 }
> ```

### 해설

**개념 설명**

**라이프타임(Lifetime)**은 참조의 유효 기간을 명시합니다:

1. **라이프타임 파라미터**: `'a`, `'b` 같은 라벨로 서로 다른 참조의 유효 기간을 추적합니다

2. **라이프타임 주석**: `&'a T`는 "T에 대한 참조로, 라이프타임 'a 동안 유효"를 의미합니다

3. **타입의 라이프타임 파라미터**: 참조를 포함하는 구조체는 라이프타임 파라미터를 가집니다

**배경 지식**

Rust의 가장 복잡한 기능 중 하나입니다. 컴파일러가 많은 경우 자동으로 라이프타임을 추론하므로 명시적으로 써야 할 경우는 드뭅니다 (**라이프타임 엘리전(elision)**).

**수식/기호/코드 설명**

`fn foo<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32`:
- 함수가 두 개의 다른 라이프타임을 가진 참조를 받습니다
- 반환 참조는 `x`와 같은 라이프타임 `'a`를 가집니다
- 이는 반환값이 `x`의 유효 기간만큼 유효하다는 뜻입니다

---

## 슬라이드 27: References

### 원문 내용
> - The Rust Programming Language:
>   https://doc.rust-lang.org/book
> - Rust By Example:
>   https://doc.rust-lang.org/rust-by-example/
> - The Rust Standard Library:
>   https://doc.rust-lang.org/std
> - The Rust Reference:
>   https://doc.rust-lang.org/reference
> - Rust Playground: https://play.rust-lang.org/

### 해설

**개념 설명**

이 슬라이드는 Rust 학습과 참고를 위한 공식 자료들을 제시합니다:

1. **The Rust Programming Language** (일명 "The Book"): 가장 권장되는 입문 자료입니다
2. **Rust By Example**: 예제 중심의 학습 자료
3. **Standard Library 문서**: API 레퍼런스
4. **The Rust Reference**: 언어 명세서
5. **Rust Playground**: 브라우저에서 직접 코드를 작성하고 실행할 수 있습니다

---

## 슬라이드 28: The Rust Compiler (rustc)

### 원문 내용
> - The compiler consists of several passes that transform the
>   source code into machine code:
>
>   Source -> parsing -> AST -> desugaring & symbol resolution -> HIR
>   -> type checking -> THIR -> lowering -> MIR -> borrow checking & optimization -> MIR -> code generation -> machine code
>
> - It exposes its internal data structures and passes as APIs,
>   which can be used for implementing static analyzers
> - Choosing the right code representation is important for
>   implementing a static analyzer

### 해설

**개념 설명**

Rust 컴파일러의 구조와 파이프라인:

**컴파일 단계들**:
1. **Parsing**: 소스 코드를 구문 분석
2. **AST (Abstract Syntax Tree)**: 구문 트리 생성
3. **Desugaring & Symbol Resolution**: 문법적 설탕 제거, 심볼 해석
4. **HIR (High-level Intermediate Representation)**: 고수준 중간 표현
5. **Type Checking**: 타입 검사
6. **THIR (Typed HIR)**: 타입이 명시된 중간 표현
7. **Lowering**: 더 낮은 수준으로 변환
8. **MIR (Mid-level Intermediate Representation)**: 중간 수준 표현
9. **Borrow Checking & Optimization**: 차용 검사 및 최적화
10. **Code Generation**: 기계 코드 생성

**배경 지식**

정적 분석기는 이 중 적절한 단계의 표현을 사용하여 분석을 수행합니다. HIR은 원본 소스에 가깝고, MIR은 제어 흐름이 명시적이며, THIR은 타입 정보가 풍부합니다.

---

## 슬라이드 29: Abstract Syntax Tree (AST)

### 원문 내용
> - Represents code as a tree, not text
> - Similar to source code, but:
>   - Macros are expanded
>   - Submodules implemented in other files are loaded (a single
>     AST for the whole crate)
>
> - Example: (1 + 2) * 3
>
>   ```
>        Mul
>       /   \
>     Add   Lit
>     / \    |
>   Lit Lit  3
>    |   |
>    1   2
>   ```
>   (Conceptual example, not the actual AST used in the compiler)
>
> - You can see the AST using rustc -Z unpretty=ast-tree,expanded [file.rs]

### 해설

**개념 설명**

**AST (Abstract Syntax Tree)**는 소스 코드를 트리 구조로 표현합니다:

특징:
- 텍스트가 아닌 구조화된 표현입니다
- 매크로가 전개됩니다 (예: `vec![1,2,3]`은 실제 함수 호출로 확장됨)
- 모듈 포함(submodule inclusion)이 처리되어 전체 crate에 대한 단일 AST를 생성합니다

**배경 지식**

AST는 대부분의 컴파일러와 언어 분석 도구의 첫 번째 중간 표현입니다. 원본 텍스트의 문법적 구조를 정확히 반영합니다.

**수식/기호/코드 설명**

예제 트리:
- 루트는 `Mul` (곱셈 연산)
- 왼쪽 자식은 `Add` (더하기), 오른쪽 자식은 `Lit` (3)
- `Add`는 두 개의 `Lit` (1, 2)을 가집니다

컴파일러 명령: `rustc -Z unpretty=ast-tree,expanded [file.rs]`로 실제 AST를 볼 수 있습니다.

---

## 슬라이드 30: High-level Intermediate Representation (HIR)

### 원문 내용
> - Similar to AST, but:
>   - Desugared (e.g., for becomes loop)
>   - Symbols are resolved
>
> ```
> let x = 1;
> {
>   let x = 2;
>   println!("{x}");
> }
> ```
>
> - What does this x refer to?
> - You can see the HIR using -Z unpretty=hir (text) or -Z
>   unpretty=hir-tree (tree)

### 해설

**개념 설명**

**HIR (High-level Intermediate Representation)**은 AST보다 정규화되고 해석된 표현입니다:

**AST와의 차이**:
1. **Desugaring (문법적 설탕 제거)**:
   - `for x in v { ... }`는 `loop`와 `Iterator` 호출로 변환됨
   - `for`가 AST에는 있지만 HIR에는 없습니다

2. **심볼 해석**: 이름들이 정의된 위치와 연결됩니다

**배경 지식**

코드 예제에서:
- 외부 스코프의 `let x = 1;`
- 내부 블록에서 `let x = 2;` (같은 이름이지만 다른 변수)
- HIR에서는 각 사용처(use)가 어느 정의를 참조하는지 명확합니다

컴파일러 명령: `-Z unpretty=hir` (텍스트) 또는 `-Z unpretty=hir-tree` (트리)

---

## 슬라이드 31: Typed High-level Intermediate Representation (THIR)

### 원문 내용
> - Similar to HIR, but:
>   - Overloading is resolved
>   - Implicit coercions are made explicit
>
> HIR:
> ```
> fn foo(
>   x: &mut i32
> ) -> &i32 {
>   x
> }
> ```
>
> THIR:
> ```
> fn foo(
>   x: &mut i32
> ) -> &i32 {
>   &*x
> }
> ```
>
> - You can see the THIR using -Z unpretty=thir-tree

### 해설

**개념 설명**

**THIR (Typed High-level Intermediate Representation)**은 타입 정보가 풍부한 표현입니다:

**HIR과의 차이**:
1. **오버로딩 해석**: 어떤 함수/연산을 호출하는지 명확합니다
2. **암시적 강제 변환 명시화**: 타입 시스템이 자동으로 수행하는 변환을 명시적으로 표현합니다

**수식/기호/코드 설명**

예제에서:
- HIR: `x`를 반환 (겉으로는 단순)
- THIR: `&*x`를 반환 (역참조 후 다시 참조)

왜 이 변환이 일어나나?
- `x`의 타입: `&mut i32`
- 반환 타입: `&i32`
- 컴파일러가 자동으로 `&mut i32`에서 `&i32`로 강제 변환합니다
- 이는 역참조(`*`)로 값을 얻은 후 다시 참조(`&`)함으로써 수행됩니다

컴파일러 명령: `-Z unpretty=thir-tree`

---

## 슬라이드 32: Mid-level Intermediate Representation (MIR)

### 원문 내용
> - A control-flow graph (CFG) representation
>   - A function consists of basic blocks and edges between them
>   - Each basic block consists of a sequence of statements and a
>     terminator
>   - Statement: assignment
>   - Terminator: jump, switch, call, return

### 해설

**개념 설명**

**MIR (Mid-level Intermediate Representation)**은 제어 흐름 그래프(CFG) 형식입니다:

**구성 요소**:
1. **기본 블록(Basic Block)**: 분기 없이 순차적으로 실행되는 명령의 블록
2. **간선(Edge)**: 블록 간의 연결 (제어 흐름)
3. **문(Statement)**: 대입(assignment) 같은 기본 명령
4. **종료자(Terminator)**: 블록의 마지막에 올 수 있는 제어 이동 명령
   - `jump`: 무조건 점프
   - `switch`: 조건부 분기
   - `call`: 함수 호출
   - `return`: 함수 반환

**배경 지식**

MIR은 **제어 흐름이 명시적**이므로, 정적 분석에 매우 유용합니다. 데이터 흐름 분석, 경로 분석 등을 수행하기 쉽습니다.

---

## 슬라이드 33: Mid-level Intermediate Representation (MIR) — Example

### 원문 내용
> ```
> let mut x = 0;
> let mut y = 0;
> while x < 10 {
>   x += 1;
>   y += x;
> }
> return y;
> ```
>
> CFG:
> ```
>      bb0:
>      x = 0
>      y = 0
>      jump bb1
>           |
>           v
>      bb1:
>      x < 10 (T: bb2, F: bb3)
>           |     \
>           v      v
>      bb2:       bb3:
>      x = x + 1  return y
>      y = y + x
>      jump bb1
>      ^
>      |
>      +-----------+
> ```
>
> (Conceptual example, not the actual MIR used in the compiler)
>
> - You can see the MIR using -Z unpretty=mir

### 해설

**개념 설명**

MIR CFG 예제 분석:

**블록 분석**:
- **bb0**: 초기화 (x=0, y=0) → bb1로 점프
- **bb1**: 루프 조건 (x < 10) 검사
  - 참(T): bb2로 이동
  - 거짓(F): bb3으로 이동
- **bb2**: 루프 본문 (x+=1, y+=x) → bb1로 점프 (루프)
- **bb3**: 함수 반환

**CFG의 특징**:
- 제어 흐름이 완전히 명시됩니다
- 루프가 점프와 분기로 표현됩니다
- 고수준 for/while 구문은 사라지고 저수준 점프로 표현됩니다

**배경 지식**

이 표현은 전통적인 컴파일러 중간 코드와 유사합니다 (예: LLVM IR). 제어 흐름 분석을 위해 설계되었습니다.

컴파일러 명령: `-Z unpretty=mir`

---

## 슬라이드 34: Comparison of Code Representations

### 원문 내용
> - AST, HIR, THIR vs. MIR:
>   - MIR is suitable for most analyses because its CFG structure
>     makes execution order explicit and it has fewer language
>     constructs
>   - If the execution order does not matter (flow-insensitive
>     analysis), other representations may be used, especially when
>     results should be close to the source code
> - AST vs. HIR, THIR:
>   - Symbols are not resolved in AST, so HIR and THIR are more
>     convenient
> - HIR vs. THIR:
>   - HIR is useful when traversing parent nodes is required or even
>     ill-typed code should be analyzed
>   - THIR is useful if type information is important
> - In this course, MIR is used in most cases, but HIR is used for
>   type analysis

### 해설

**개념 설명**

각 표현의 사용 시나리오:

**MIR 사용 이유**:
- CFG 구조로 실행 순서가 명시적입니다
- 언어 특성들(for, match 등)이 저수준으로 정규화됩니다
- 데이터 흐름, 제어 흐름 분석에 최적입니다

**다른 표현의 선택**:
- **실행 순서가 무관한 분석** (flow-insensitive): 소스 코드에 가까운 표현 사용
  - 예: 타입 분석, 심볼 해석
- **AST vs. HIR/THIR**: HIR/THIR은 심볼이 해석되어 있으므로 편리합니다
- **HIR vs. THIR**:
  - HIR: 부모 노드 탐색 필요, 타입 오류가 있는 코드 분석
  - THIR: 타입 정보 중요할 때

**배경 지식**

이 과목에서는:
- 주로 MIR을 사용합니다 (명시적 제어 흐름)
- 타입 분석에는 HIR을 사용합니다 (타입 정보)

---

## 슬라이드 35: Important Types in HIR

### 원문 내용
> - LocalDefId: a unique identifier for a top-level item in the
>   crate
> - DefId: a unique identifier for a top-level item in any crate,
>   including dependencies. (DefId = LocalDefId + crate ID)
> - HirId: a unique identifier for any node in the HIR; each local
>   variable has a unique HirId as well
> - Item: a top-level item
> - Stmt: a statement
> - Expr: an expression
> - Visitor: a trait for traversing the HIR; you can implement
>   your own logic by defining a struct that implements this trait

### 해설

**개념 설명**

HIR 분석을 위한 핵심 타입들:

1. **LocalDefId**: crate 내 최상위 항목의 고유 식별자 (함수, 구조체, 모듈 등)

2. **DefId**: 모든 crate (의존성 포함)의 항목 식별자
   - `DefId = LocalDefId + crate ID`
   - 외부 라이브러리의 항목도 추적 가능합니다

3. **HirId**: HIR의 모든 노드 (최상위 항목, 지역 변수 포함)의 고유 식별자
   - 가장 세밀한 식별자입니다

4. **Item, Stmt, Expr**: HIR 노드의 다양한 종류
   - Item: 함수, 구조체 정의
   - Stmt: 명령문
   - Expr: 식

5. **Visitor 패턴**: HIR을 순회하는 트레이트
   - 이를 구현하여 커스텀 분석 로직을 작성합니다
   - 모든 노드를 방문하기 위한 일반적인 방법입니다

**배경 지식**

Visitor 패턴은 트리 구조 순회의 표준 패턴입니다. Rust 컴파일러는 HIR 순회를 위해 이 패턴을 광범위하게 사용합니다.

---

## 슬라이드 36: Important Types in MIR

### 원문 내용
> - Body: a function in MIR
> - BasicBlock: a unique identifier for a basic block
> - BasicBlockData: a basic block
> - Statement: a statement
> - Terminator: a terminator
>
> - Place: a memory location;
>   can be used as the LHS of
>   an assignment
> - Rvalue: the RHS of an
>   assignment
> - Operand: an operand used
>   in an Rvalue (a place or a
>   constant)
> - Const: a constant value

### 해설

**개념 설명**

MIR 분석을 위한 핵심 타입들:

**제어 흐름**:
1. **Body**: MIR의 함수 전체를 나타냅니다
2. **BasicBlock**: 기본 블록의 고유 식별자
3. **BasicBlockData**: 실제 기본 블록 데이터 (문과 종료자)
4. **Statement**: 기본 명령 (대부분 대입)
5. **Terminator**: 블록을 종료하는 제어 이동 명령

**데이터 흐름**:
6. **Place**: 메모리 위치
   - 변수, 필드, 배열 원소 등을 나타냅니다
   - 대입의 LHS(좌변)에 올 수 있습니다

7. **Rvalue**: 대입의 RHS(우변)
   - 값을 계산하는 연산 (덧셈, 함수 호출 등)

8. **Operand**: Rvalue에서 사용되는 피연산자
   - Place 또는 상수

9. **Const**: 상수 값

**배경 지식**

MIR의 대입: `Place = Rvalue`
- Place는 변수나 필드
- Rvalue는 값을 계산하는 식

---

## 슬라이드 37: Important Types in MIR (cont.)

### 원문 내용
> - Local: a unique identifier for a local variable
>   - Place is a Local with projections (e.g., field access)
>   - Each variable is represented as an integer
>   - _0 is the return value; _1, _2, ... are parameters
>
> ```
> fn add(x: i32, y: i32) -> i32 {
>   x + y
> }
> ```
>
> ```
> _0 = Add(_1, _2);
> return;
> ```
>
> - Visitor: a trait for traversing the MIR; you can implement
>   your own logic by defining a struct that implements this trait

### 해설

**개념 설명**

**Local**: MIR의 지역 변수 고유 식별자

**명명 규칙**:
- `_0`: 반환값 (return value)
- `_1, _2, ...`: 파라미터들 (순서대로)

**Place와의 관계**:
- Place는 Local에 projection을 적용한 것입니다
- 예: `_1.field`, `_2[5]` (필드 접근, 배열 인덱싱)

**수식/기호/코드 설명**

예제 함수:
```rust
fn add(x: i32, y: i32) -> i32 {
  x + y
}
```

MIR 표현:
```
_0 = Add(_1, _2);
return;
```

- `_0`: 반환값이 될 임시 변수
- `_1`: 첫 번째 파라미터 `x`
- `_2`: 두 번째 파라미터 `y`
- `Add(_1, _2)`: 두 개 피연산자의 덧셈
- 반환값 `_0`이 자동으로 반환됩니다

**Visitor 패턴**: MIR을 순회하는 트레이트로, 커스텀 분석 로직을 구현할 수 있습니다.

---

## 슬라이드 38: TyCtxt

### 원문 내용
> - The central data structure of the compiler²
> - When invoking the compiler, a TyCtxt value is given, and
>   many APIs are provided as methods of TyCtxt
> - Examples:
>   - hir_visit_all_item_likes_in_crate: visits all item-likes
>     in the crate in some deterministic order
>   - parent_hir_id: returns the HirId of the parent HIR node of
>     node with the given hir_id
>   - optimized_mir: MIR after the optimization passes have run
>
> ² https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/ty/context/struct.TyCtxt.html

### 해설

**개념 설명**

**TyCtxt** (Type Context)는 Rust 컴파일러의 중앙 데이터 구조입니다:

역할:
1. **컴파일 컨텍스트**: 현재 컴파일 작업의 모든 정보를 담고 있습니다
2. **API 제공**: 다양한 메서드로 HIR, MIR, 타입 정보에 접근합니다

**주요 메서드들**:
- `hir_visit_all_item_likes_in_crate`: crate의 모든 항목을 순회합니다
- `parent_hir_id`: 주어진 HIR 노드의 부모를 찾습니다
- `optimized_mir`: 최적화된 MIR을 얻습니다

**배경 지식**

정적 분석기를 작성할 때, TyCtxt를 입력으로 받아 컴파일러의 모든 기능을 활용할 수 있습니다.

---

## 슬라이드 39: References

### 원문 내용
> - HIR: https://doc.rust-lang.org/stable/
>   nightly-rustc/rustc_hir/hir
> - MIR: https://doc.rust-lang.org/stable/
>   nightly-rustc/rustc_middle/mir
> - Rust Compiler Development Guide:
>   https://rustc-dev-guide.rust-lang.org/

### 해설

**개념 설명**

Rust 컴파일러 개발을 위한 공식 문서 자료:

1. **HIR 문서**: HIR의 데이터 구조와 API에 대한 완전한 레퍼런스

2. **MIR 문서**: MIR의 모든 타입과 인터페이스 설명

3. **Rust Compiler Development Guide**: 컴파일러 구조, 분석 패스, 개발 방법론 등을 설명합니다

**배경 지식**

이 강의에서 정적 분석기를 구현할 때 이 문서들이 주요 참고 자료가 됩니다.

---

## 슬라이드 40: Summary

### 원문 내용
> - Rust is a systems programming language with memory safety
>   guaranteed by the type system at compile time
> - Key language features: variables, functions, types, ownership,
>   borrowing, enums, traits, lifetimes
> - The rustc compile pipeline: AST → HIR → THIR → MIR
>   → machine code
> - MIR (control-flow graph) is most suitable for static analysis
>   due to explicit execution order and fewer language constructs
> - Key types for analysis:
>   - HIR: HirId, Item, Expr
>   - MIR: Body, BasicBlockData, Statement, Terminator,
>     Place, Rvalue, Operand, Const, Local
>   - LocalDefId, DefId, TyCtxt, Visitor

### 해설

**개념 설명**

강의의 핵심 요점 정리:

**Rust의 특징**:
- 시스템 프로그래밍 언어
- 컴파일 타임 타입 검사로 메모리 안전성 보장

**주요 언어 기능**:
- 변수 (기본 불변), 함수, 다양한 타입
- 소유권과 차용 (메모리 관리)
- 열거형 (패턴 매칭)
- 트레이트 (인터페이스)
- 라이프타임 (참조 유효 기간)

**컴파일 파이프라인**:
```
Source → Parsing → AST → Desugaring & Symbol Resolution
→ HIR → Type Checking → THIR → Lowering → MIR
→ Borrow Checking & Optimization → MIR → Code Generation
→ Machine Code
```

**정적 분석을 위한 표현**:
- **MIR**: 가장 적합 (제어 흐름 명시적, 언어 요소 적음)
- **HIR**: 타입 분석에 사용

**분석 구현의 핵심 타입**:
- HIR 분석: HirId, Item, Expr, Visitor
- MIR 분석: Body, BasicBlockData, Statement, Terminator, Place, Rvalue, Operand, Const, Local
- 전역: LocalDefId, DefId, TyCtxt, Visitor

**전체적인 맥락**

이 강의는 Rust 언어의 기본 개념부터 컴파일러 내부 표현까지를 망라합니다. 다음 강의에서는 이 지식을 바탕으로 실제 정적 분석기를 구현할 때 이 표현들을 어떻게 활용하는지 배우게 됩니다.

---

**문서 작성 완료**

이 해설 문서는 02-rust.pdf의 모든 40페이지 슬라이드에 대한 상세한 설명을 한국어로 제공합니다. 각 슬라이드의 원문 내용을 정확히 인용한 후, 개념 설명, 배경 지식, 코드 설명 등을 추가했습니다.
