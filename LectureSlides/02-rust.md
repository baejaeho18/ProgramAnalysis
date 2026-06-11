# Introduction to Rust - 강의 슬라이드 해설

CSE552 Program Analysis — Lecture 2
강사: Jaemin Hong

> **이 파일을 읽는 법**: 각 슬라이드는 `원문 내용` → `번역` → `해설`(개념·배경·맥락) → 필요 시 `각주`/`슬라이드 연결` 순서입니다. 누락·왜곡 없이 원문을 모두 담되 처음 보는 사람도 따라올 수 있게 풀어 썼습니다.

---

## 강의 2 전체 조감도 (먼저 큰 그림)

이 강의는 **Rust 언어와 그 컴파일러(rustc)**를 소개합니다. 왜 프로그램 분석 과목에서 Rust를 배울까요? 두 가지 이유입니다:
1. **Rust는 이 과목의 구현 언어이자 분석 대상**입니다. 과제에서 Rust로 정적 분석기를 만들고, **Rust 프로그램(의 MIR)을 분석**합니다(예: Assignment 4의 interprocedural interval analysis).
2. **Rust의 안전성 메커니즘(소유권·빌림·수명) 자체가 정적 분석의 산물**입니다. Rust 컴파일러는 빌림 검사(borrow checking)라는 정적 분석으로 메모리 안전성을 보장합니다.

강의는 두 부분입니다:
- **A. Rust 언어 기능** (슬라이드 2~27): 변수·함수·타입·표현식·루프 같은 기초부터, **소유권(ownership)·빌림(borrowing)·앨리어싱 XOR 가변성** 같은 Rust 고유의 안전성 개념, 그리고 enum·`Option`/`Result`·제네릭·트레이트·수명 등.
- **B. rustc 컴파일러 파이프라인** (슬라이드 28~40): 소스 → AST → HIR → THIR → **MIR** → 기계어. 특히 **MIR(제어 흐름 그래프)**가 정적 분석에 가장 적합하며, 분석에 쓰이는 핵심 타입들(`Body`, `BasicBlock`, `Local`, `Place`, `Rvalue` 등).

이 강의의 개념들은 이후 곳곳에서 재등장합니다: 소유권·`Drop`은 강의 12(락 가드)·13(I/O 스트림), enum·`Option`/`Result`는 강의 13(출력 매개변수→ADT), 트레이트는 강의 13(I/O 능력), MIR 구조는 모든 과제와 강의 18~20(의미론)의 기반입니다.

---

## 슬라이드 1: 제목 슬라이드

### 원문 내용
> Introduction to Rust
> CSE552 Program Analysis — Lecture 2
> Jaemin Hong

### 번역
> Rust 입문 / CSE552 프로그램 분석 — 강의 2 / 홍재민

### 해설
이 과목의 도구이자 분석 대상인 **Rust**를 소개하는 강의입니다.

---

## 슬라이드 2: What is Rust?

### 원문 내용
> - A systems programming language to replace C and C++
> - Performance comparable to C and C++
> - Memory safety guaranteed by type checking at compile time
> - Language features adopted from functional languages: closures, algebraic data types (enums), generics, traits

### 번역
> - C·C++를 대체하기 위한 **시스템 프로그래밍 언어**
> - C·C++에 **필적하는 성능**
> - **컴파일 타임 타입 검사로 메모리 안전성을 보장**
> - 함수형 언어에서 채택한 기능들: 클로저, 대수적 자료형(enum), 제네릭, 트레이트

### 해설

**개념 설명 — Rust의 세 기둥**

Rust는 세 가지를 동시에 추구합니다:
1. **성능**: C·C++처럼 빠름(가비지 컬렉터 없음, 제로코스트 추상화).
2. **안전성**: C·C++의 고질병(메모리 버그)을 **컴파일 타임에** 막음 — 런타임 오버헤드 없이 타입 시스템으로.
3. **표현력**: 함수형 언어의 좋은 기능들(클로저·ADT·제네릭·트레이트)을 도입.

핵심은 "**안전성을 런타임이 아니라 타입 검사(정적 분석)로**" 보장한다는 점 — 이것이 정적 분석 과목과 맞닿는 부분입니다. 강의 12~13에서 C의 위험한 코드를 Rust의 안전한 형태로 변환하는 것이 바로 이 안전성을 얻기 위함입니다.

---

## 슬라이드 3: Rust in the Real World

### 원문 내용
> - The White House recommended using safe languages, including Rust, instead of C and C++¹
> - Many new systems, including operating systems and web browsers, have been developed in Rust
> - Linux has officially added support for Rust in kernel development
> - Coreutils have been rewritten in Rust and installed by default in some recent operating systems, such as Ubuntu 25.10
>
> ¹ Back to the building blocks: A path toward secure and measurable software (White House, 2024)

### 번역
> - 백악관이 C·C++ 대신 Rust 같은 **안전한 언어** 사용을 권고(2024)
> - 운영체제·웹 브라우저 등 많은 새 시스템이 Rust로 개발됨
> - 리눅스가 커널 개발에 Rust를 공식 지원
> - Coreutils가 Rust로 재작성되어 일부 최신 OS(Ubuntu 25.10 등)에 기본 설치

### 해설

**배경 지식 — Rust의 부상**

Rust는 학술적 호기심을 넘어 **산업·정부 표준**이 되고 있습니다. 미국 백악관이 메모리 안전 언어를 권고하고, 리눅스 커널이 Rust를 받아들였다는 것은 C/C++의 메모리 버그(강의 1의 하트블리드 등)가 얼마나 심각한 문제인지를 보여 줍니다. 강의 12~13의 C-to-Rust 변환 연구가 시의적절한 이유입니다.

---

## 슬라이드 4: Rust in This Course

### 원문 내용
> - Rust is both an implementation language and a target language for static analysis in this course
>   - In assignments, you will implement a static analyzer for Rust programs in Rust
>   - For the term project, you can use any language and analyze any language
> - Functional languages (e.g., Scala and OCaml) have typically been used for implementing static analyzers
> - Rust provides useful features for implementing static analyzers, while also allowing performant implementations

### 번역
> - 이 과목에서 Rust는 **구현 언어이자 분석 대상 언어**다
>   - 과제: Rust로 **Rust 프로그램용 정적 분석기**를 구현
>   - 텀 프로젝트: 어떤 언어든 사용·분석 가능
> - 전통적으로 정적 분석기는 함수형 언어(Scala·OCaml)로 구현되어 왔다
> - Rust는 정적 분석기 구현에 유용한 기능을 제공하면서 고성능 구현도 가능케 한다

### 해설

**개념 설명**

과제에서 **Rust로 Rust 분석기**를 만듭니다(Assignment 4가 그 예 — rustc 내부 API를 써서 Rust MIR을 분석). 전통적으로 분석기는 OCaml·Scala 같은 함수형 언어로 만들었지만(패턴 매칭·ADT가 편해서), Rust는 그런 함수형 기능(enum·match)에 더해 고성능까지 줍니다. 그래서 이 과목이 Rust를 택했습니다.

---

## 슬라이드 5: Variables

### 원문 내용
> - `let [id] = [expr];` — immutable variable
> - `let mut [id] = [expr];` — mutable variable
> ```rust
> // Immutable:
> let x = 1;
> x = 2; // error
> // Mutable:
> let mut x = 1;
> x = 2; // ok
> ```

### 번역
> - `let x = ...;` — **불변(immutable)** 변수 (재대입 불가)
> - `let mut x = ...;` — **가변(mutable)** 변수 (재대입 가능)
> - 기본은 불변. `x=2` 재대입은 `mut` 없으면 에러.

### 해설

**개념 설명 — 기본이 불변**

Rust는 **변수가 기본적으로 불변**입니다(`let`). 바꾸려면 명시적으로 `mut`를 붙여야 합니다. 이는 "의도하지 않은 변경"을 막는 안전 설계 — 변경 가능성을 타입에 드러내야 합니다. 이 "가변성을 명시"하는 철학은 빌림(`&` vs `&mut`, 슬13)과 앨리어싱 규칙(슬14)으로 이어집니다.

---

## 슬라이드 6: Functions

### 원문 내용
> - `fn [id]([id]: [type], ...) -> [type] { ... }`
> - All parameter types and the return type must be specified
> - The last expression in the function body is the return value
> ```rust
> fn add(x: i32, y: i32) -> i32 {
>   x + y
> }
> ```

### 번역
> - 함수 정의: `fn 이름(매개변수: 타입, ...) -> 반환타입 { ... }`
> - **모든 매개변수 타입과 반환 타입을 명시**해야 함
> - 함수 본문의 **마지막 식이 반환값**(return 키워드 불필요)

### 해설

**개념 설명**

함수는 타입을 모두 명시합니다(타입 추론은 함수 내부 지역 변수에만). `add`에서 `x + y`(세미콜론 없음)가 곧 반환값 — Rust는 표현식 지향 언어(슬8)라 마지막 식이 값이 됩니다. 강의 3~4의 타입 분석은 이 타입 정보를 다룹니다.

---

## 슬라이드 7: Types

### 원문 내용
> - Signed integers: i8, i16, i32, i64, i128, isize
> - Unsigned integers: u8, u16, u32, u64, u128, usize
> - Floating-point numbers: f32, f64
> - Boolean: bool; Unit: (); Character: char
> - String: String; Tuples: (T1, T2, ...); Arrays: [T; N]; Vectors: Vec<T>; Hash maps: HashMap<K, V>

### 번역
> Rust의 기본 타입들: 부호 정수(i8~i128, isize), 부호 없는 정수(u8~u128, usize), 부동소수(f32, f64), 불리언, 유닛 `()`, 문자, 문자열, 튜플, 배열 `[T; N]`, 벡터 `Vec<T>`, 해시맵.

### 해설

**개념 설명 — 풍부한 타입 시스템**

정수도 비트 크기와 부호 유무로 세분(i32, u64 등) — 메모리·오버플로를 명시적으로 다룹니다(강의 1의 아리안 5 오버플로 방지와 통함). **튜플 `(T1,T2)`(곱 타입)와 이후 enum(합 타입)**이 대수적 자료형(ADT)을 이루며, 강의 13의 출력 매개변수→ADT 변환에서 핵심이 됩니다. `Vec`·`HashMap`은 힙 할당 자료구조로, 소유권(슬10)의 대상입니다. 과제에서 다루는 MIR은 주로 `i32`만 씁니다(Assignment 4 가정).

---

## 슬라이드 8: Expression-oriented Programming

### 원문 내용
> - Every expression evaluates to a value
> ```rust
> let x = {
>   let y = 10;
>   y * y
> };
> let x = if y >= 0 {
>   println!("y is non-negative");
>   y
> } else {
>   println!("y is negative");
>   -y
> };
> ```

### 번역
> - **모든 표현식이 값을 가진다**(표현식 지향)
> - 블록 `{...}`도 값을 가짐(마지막 식). `if-else`도 값을 가짐(각 가지의 마지막 식).

### 해설

**개념 설명 — 문장이 아니라 표현식**

Rust에서는 `if`, 블록 `{}`, `match` 등이 모두 **값을 가지는 표현식**입니다(C의 문장과 대조). `let x = if ... {a} else {b}`처럼 조건식 결과를 변수에 바로 담을 수 있습니다. 이 표현식 지향성은 함수형 언어 유산이며, 코드를 간결하게 합니다.

---

## 슬라이드 9: Loops

### 원문 내용
> - `loop { ... }`, `while [expr] { ... }`, `for [id] in [expr] { ... }`
> ```rust
> let mut x = 0;
> let y = loop {
>   x += 1;
>   if x == 10 { break x * 2; }
> };
> let v = vec![1, 2, 3];
> for x in v { println!("{x}"); }
> ```

### 번역
> 세 가지 루프: `loop`(무한, `break`로 탈출하며 값 반환 가능), `while`(조건 반복), `for`(컬렉션 순회). `loop`도 `break 값`으로 값을 낼 수 있다.

### 해설

**개념 설명**

`loop`/`while`/`for` 세 종류. 흥미롭게 `loop`도 `break x*2`로 값을 반환하는 표현식입니다(슬8 연장). 루프는 정적 분석에서 **고정점 반복**(강의 7~9)이 필요한 핵심 구조 — 루프가 있으면 "몇 번 도는지" 모르므로 위드닝(강의 9)이 필요해집니다. MIR에서는 루프가 기본 블록 간 순환 간선이 됩니다(슬33).

---

## 슬라이드 10: Ownership

### 원문 내용
> - Each value is owned by at most one variable
> - When the value is assigned to another variable or passed to a function, the ownership is transferred (moved)
> - The compiler prevents using a variable after its ownership has been moved
> ```rust
> let s = "hi".to_string();
> println!("{s}"); // ok
> let t = s;
> println!("{s}"); // error
> println!("{t}"); // ok
> ```

### 번역
> - 각 값은 **최대 한 변수가 소유**한다
> - 다른 변수에 대입하거나 함수에 넘기면 **소유권이 이동(move)**된다
> - 컴파일러는 소유권이 이동된 변수의 **이후 사용을 막는다**
> - `let t = s` 후 `s`는 무효 → `s` 사용은 에러, `t`는 OK.

### 해설

**개념 설명 — 소유권(ownership) ★ (Rust의 핵심)**

Rust 안전성의 심장입니다. **각 값은 단 하나의 소유자**만 가지며, 대입·전달 시 소유권이 **이동(move)**합니다. 이동 후 원래 변수는 무효가 되어 컴파일러가 사용을 막습니다.

왜 이렇게 하나? **이중 해제(double free)·use-after-free를 원천 차단**하기 위해서입니다. 값이 한 소유자만 가지면, 그 소유자가 사라질 때 메모리를 정확히 한 번만 해제하면 됩니다(RAII). 이 소유권 개념이 강의 12의 `MutexGuard`(가드 소유권이 락 보유를 표현), 강의 13의 스트림(닫힌 스트림 사용 방지)의 기반입니다. 또 강의 14~15의 포인터 분석이 다루는 앨리어싱 문제를, Rust는 소유권·빌림으로 정적으로 통제합니다.

---

## 슬라이드 11: Copy and Clone

### 원문 내용
> - Some types (e.g., primitive types) implement Copy, which allows copying instead of moving
> - For types not implementing Copy (typically heap-allocated data), use clone to create a deep copy
> ```rust
> let x = 1;
> let y = x;
> println!("{x}"); // ok (copied)
> let s = "hi".to_string();
> let t = s.clone();
> println!("{s}"); // ok (deep copied)
> ```

### 번역
> - 일부 타입(원시 타입 등)은 **Copy**를 구현해, 이동 대신 **복사**된다(원본 유지)
> - Copy 안 하는 타입(보통 힙 데이터)은 `clone()`으로 **깊은 복사**
> - 정수 `x`는 Copy라 `y=x` 후에도 `x` 사용 OK. 문자열은 `clone()`해야 둘 다 유효.

### 해설

**개념 설명 — Copy vs Move**

소유권 이동의 예외: **Copy 타입**(정수·불리언 등 스택에 있는 작은 값)은 대입 시 **복사**되어 원본도 유효합니다(슬10의 move와 대조). 힙 데이터(`String`, `Vec`)는 Copy가 아니라 move되며, 양쪽을 다 쓰려면 명시적 `clone()`(깊은 복사)이 필요합니다. 이 구분은 "비싼 복사를 실수로 하지 않게" 하는 설계 — 성능과 안전성의 균형.

---

## 슬라이드 12: Ownership and Use-After-Free

### 원문 내용
> - The notion of ownership prevents use-after-free
> ```rust
> let s = "hi".to_string();
> let t = s;
> drop(t); // deallocate the heap memory
> println!("{s}"); // error
> ```

### 번역
> - 소유권 개념이 **use-after-free를 방지**한다
> - `t = s`로 소유권 이동 후 `drop(t)`로 메모리 해제. 이후 `s` 사용은 에러(이미 이동·해제됨).

### 해설

**개념 설명 — use-after-free 차단**

`s`의 소유권이 `t`로 이동했고, `drop(t)`로 메모리가 해제됩니다. 이제 `s`를 쓰면 use-after-free인데, **컴파일러가 `s`는 이미 이동됐다며 막습니다**. C였다면 런타임 크래시·보안 취약점이 될 버그가 컴파일 에러가 됩니다. 강의 13의 "닫힌 스트림 사용 방지"(`drop(f)` 후 `f.write()` 에러)가 정확히 이 메커니즘입니다.

---

## 슬라이드 13: Borrowing

### 원문 내용
> - Borrowing allows temporarily using a value without taking ownership
> - `&[expr]` creates a shared reference (read-only) of type &T
> - `&mut [expr]` creates a mutable reference (read-write) of type &mut T
> ```rust
> fn show(s: &String) { println!("{s}"); }
> let s = "hi".to_string();
> show(&s);
> println!("{s}"); // ok
>
> fn push_dot(s: &mut String) { s.push('.'); }
> let mut s = "hi".to_string();
> push_dot(&mut s);
> println!("{s}"); // ok
> ```

### 번역
> - **빌림(borrowing)**: 소유권을 가져가지 않고 값을 **잠시 사용**
> - `&값` → **공유 참조(shared reference, 읽기 전용)** `&T`
> - `&mut 값` → **가변 참조(mutable reference, 읽기·쓰기)** `&mut T`
> - `show(&s)`는 s를 빌려 읽기만 하므로 이후 `s`도 유효. `push_dot(&mut s)`는 빌려 수정.

### 해설

**개념 설명 — 빌림(borrowing) ★**

매번 소유권을 넘기면 불편하므로, **참조(빌림)**로 잠시 빌려 씁니다(소유권은 그대로). 두 종류:
- **공유 참조 `&T`**: 읽기만. 여러 개 동시 존재 가능.
- **가변 참조 `&mut T`**: 읽기·쓰기. 단 하나만 존재 가능.

이 구분이 슬라이드 14의 **앨리어싱 XOR 가변성** 규칙으로 이어집니다 — Rust 안전성의 두 번째 핵심. 빌림은 C의 포인터에 해당하지만, 컴파일러가 그 사용을 엄격히 검사(borrow checker)합니다. 강의 14~15의 포인터 분석이 다루는 앨리어싱을, Rust는 이 규칙으로 정적 통제합니다.

---

## 슬라이드 14: Aliasing XOR Mutability

### 원문 내용
> - Multiple shared references to the same value can coexist
> - A mutable reference to a value cannot coexist with any other reference (shared or mutable) to the same value
> - This discipline also prevents use-after-free
> ```rust
> let mut v = vec![1, 2, 3];
> let r = &v[0];
> v.push(4); // may reallocate the vector
> println!("{r}"); // error
> ```

### 번역
> - 같은 값에 대한 **공유 참조는 여러 개 공존 가능**
> - 같은 값에 대한 **가변 참조는 다른 어떤 참조와도(공유든 가변이든) 공존 불가**
> - 이 규율이 use-after-free도 방지
> - 예: `r`이 `v[0]`을 빌린 상태에서 `v.push(4)`(벡터 재할당 가능) → `r`이 무효가 될 수 있어 에러.

### 해설

**개념 설명 — 앨리어싱 XOR 가변성 (Rust 안전성의 황금률) ★**

Rust의 빌림 검사 핵심 규칙: **"앨리어싱(공유)과 가변성(쓰기)은 동시에 안 된다"**(XOR).
- 읽기만 한다면(공유 참조) 여러 개 OK — 아무도 안 바꾸니 안전.
- 쓴다면(가변 참조) 반드시 **독점** — 그 동안 다른 누구도 접근 못 함.

왜? 예시가 보여 줍니다: `r`이 `v[0]`을 가리키는데 `v.push(4)`가 벡터를 재할당하면, `r`은 해제된 메모리를 가리키게 됩니다(use-after-free). 이를 막으려고, `r`이 살아 있는 동안 `v`를 변경(가변 접근)하는 것을 컴파일러가 금지합니다.

**왜 정적 분석 과목에 중요한가**: 강의 12의 락 분석이 "포인터 앨리어싱을 무시해 unsound"라 했던 것(강의 12 슬14), 강의 14~15의 포인터 분석이 통째로 앨리어싱을 다루는 것 — 모두 이 앨리어싱 문제 때문입니다. Rust는 이 어려운 문제를 "XOR 규칙"으로 컴파일 타임에 푸는 셈입니다. 단, 이 규칙이 너무 엄격할 때를 위한 탈출구가 슬라이드 15.

---

## 슬라이드 15: Interior Mutability

### 원문 내용
> - Some types allow mutating the value even through a shared reference
> - Cell<T>: provides get and set methods
> - RefCell<T>: provides borrow and borrow_mut methods that enforce the aliasing XOR mutability rule at runtime
> ```rust
> let x = Cell::new(1);
> let r1 = &x;
> let r2 = &x;
> r1.set(2);
> println!("{}", r2.get());
> ```
> - Useful for certain purposes, e.g., caching and cyclic structures

### 번역
> - 일부 타입은 **공유 참조를 통해서도 값을 변경**할 수 있게 한다(내부 가변성)
> - `Cell<T>`: get/set 제공
> - `RefCell<T>`: borrow/borrow_mut 제공, 앨리어싱 XOR 가변성 규칙을 **런타임에** 강제
> - 캐싱·순환 구조 등에 유용

### 해설

**개념 설명 — 내부 가변성(interior mutability)**

슬라이드 14의 XOR 규칙은 강력하지만 때로 너무 엄격합니다(예: 순환 자료구조, 캐시). **`Cell`/`RefCell`**은 공유 참조를 통해서도 내부를 바꿀 수 있게 하는 탈출구입니다. 단 공짜는 아닙니다 — `RefCell`은 XOR 규칙을 **컴파일 타임 대신 런타임에** 검사해, 위반 시 패닉합니다. 즉 정적 검사를 동적 검사로 미룬 것. 정적 분석의 관점에서, 이런 내부 가변성은 분석을 어렵게 만드는 요인입니다(컴파일러가 보장하던 불변식이 약해짐).

---

## 슬라이드 16: Function Pointers

### 원문 내용
> - Type: `fn([type], ...) -> [type]`
> - Can be called or passed to higher-order functions
> ```rust
> fn is_neg(x: &i32) -> bool { *x < 0 }
> let f: fn(&i32) -> bool = is_neg;
> println!("{}", f(&-1));
> let mut v = vec![1, -2, 3];
> v.retain(f); // equivalent to v.retain(is_neg);
> ```

### 번역
> - **함수 포인터** 타입: `fn(타입,...) -> 타입`
> - 호출하거나 **고차 함수에 전달** 가능
> - `f`에 함수 `is_neg`를 담아 호출하거나 `v.retain(f)`로 넘김.

### 해설

**개념 설명 — 함수가 값이 되다**

함수를 변수에 담고 인자로 넘길 수 있습니다(일급 함수). 이것이 강의 11의 **제어 흐름 분석(CFA)**이 필요한 바로 그 상황 — "이 호출 지점에서 어떤 함수가 불릴까?"가 자명하지 않게 됩니다(`f`에 어떤 함수가 들었는지 추적해야 함). 강의 14의 함수 포인터 points-to 분석으로 이어집니다.

---

## 슬라이드 17: Anonymous Functions and Closures

### 원문 내용
> Anonymous Functions:
> - Functions without a name: `|[id], ...| [expr]`
> ```rust
> let mut v = vec![1, -2];
> v.retain(|x| *x < 0);
> ```
> Closures:
> - Anonymous functions can capture variables from the surrounding scope
> ```rust
> let n = 10;
> let mut v = vec![5, 15];
> v.retain(|x| *x > n);
> ```

### 번역
> - **익명 함수(anonymous function)**: 이름 없는 함수 `|인자| 식`
> - **클로저(closure)**: 익명 함수가 **주변 스코프의 변수를 포획(capture)**한 것 (예: `n`을 포획해 `*x > n`)

### 해설

**개념 설명 — 클로저**

`|x| *x < 0`은 이름 없는 함수(람다)입니다. 주변 변수(`n`)를 **포획**하면 클로저가 됩니다. 클로저는 "코드 + 포획한 환경"이라 함수 포인터보다 분석이 복잡합니다(포획 변수의 값까지 추적해야 함). 함수형 언어 유산이며, 강의 11의 "함수가 값" 상황을 더 풍부하게 만듭니다.

---

## 슬라이드 18: Structs

### 원문 내용
> - Custom data types with named fields
> ```rust
> struct Point { x: f64, y: f64 }
> let p = Point { x: 1.0, y: 2.0 };
> impl Point {
>   fn distance_from_origin(&self) -> f64 {
>     (self.x * self.x + self.y * self.y).sqrt()
>   }
> }
> ```

### 번역
> - **구조체(struct)**: 이름 붙은 필드를 가진 사용자 정의 타입(곱 타입)
> - `impl` 블록으로 **메서드** 정의 가능(`&self`는 자기 자신을 빌림)

### 해설

**개념 설명**

구조체는 여러 값을 묶은 **곱 타입**입니다(튜플의 이름 붙은 버전). `impl`로 메서드를 답니다. `&self`는 "이 구조체를 빌려서" 메서드가 동작함을 뜻합니다(슬13의 빌림). 객체지향의 클래스와 비슷하지만, 데이터(struct)와 동작(impl)이 분리됩니다.

---

## 슬라이드 19: Enums

### 원문 내용
> - Custom data types that can be one of several variants
> - Used with pattern matching
> ```rust
> enum Color { Red, Green, Blue }
> let c = Color::Green;
> let name = match c {
>   Color::Red => "red",
>   Color::Green => "green",
>   Color::Blue => "blue",
> };
> ```

### 번역
> - **열거형(enum)**: 여러 변형(variant) 중 하나일 수 있는 타입(**합 타입**)
> - **패턴 매칭(match)**과 함께 사용

### 해설

**개념 설명 — enum = 합 타입 ★**

enum은 "여럿 중 하나"를 표현하는 **합 타입(sum type)**입니다. `Color`는 Red/Green/Blue 중 하나. `match`로 각 경우를 다룹니다. struct(곱, "이것 *그리고* 저것")와 enum(합, "이것 *또는* 저것)이 합쳐져 **대수적 자료형(ADT)**을 이룹니다. 이 ADT가 강의 13의 핵심 — C의 출력 매개변수를 Rust의 enum(`Option`/`Result`)으로 변환하는 것이 강의 13 전반부입니다. `match`의 **완전성 검사**(모든 경우 처리 강제)가 안전성의 한 축입니다(슬20~21).

---

## 슬라이드 20: Enums (cont.)

### 원문 내용
> - Each variant can have associated data
> ```rust
> enum Shape { Circle(f64), Rectangle(f64, f64) }
> let s = Shape::Circle(1.0);
> let area = match s {
>   Shape::Circle(r) => PI * r * r,
>   Shape::Rectangle(w, h) => w * h,
> };
> ```
> - `if let` is useful when we only care about one variant
> ```rust
> if let Shape::Circle(r) = s {
>   println!("circle with radius {r}");
> }
> ```

### 번역
> - 각 변형이 **연관 데이터**를 가질 수 있음 (`Circle(f64)`는 반지름을 품음)
> - 한 변형만 관심 있으면 **`if let`**이 편리

### 해설

**개념 설명**

enum 변형이 데이터를 담습니다(`Circle(반지름)`). `match`로 변형을 구분하며 데이터를 꺼냅니다(패턴 매칭). `if let`은 한 변형만 처리할 때 간결한 형태. 이 "데이터를 품은 합 타입"이 `Option<T>`(Some(T)/None)·`Result<T,E>`(Ok(T)/Err(E))의 토대입니다(슬21~22).

---

## 슬라이드 21: Options

### 원문 내용
> - Option<T> is an enum that can be either Some(T) or None
> - Represents optional existence of a value or the possibility of failure
> ```rust
> let map = HashMap::from([("a", 1), ("b", 2)]);
> match map.get("a") {
>   Some(x) => println!("value: {x}"),
>   None => println!("key not found"),
> }
> ```

### 번역
> - **`Option<T>`**: `Some(T)`(값 있음) 또는 `None`(없음)인 enum
> - 값의 **선택적 존재** 또는 **실패 가능성**을 표현

### 해설

**개념 설명 — Option = 널 안전성**

`Option<T>`는 "값이 있을 수도 없을 수도"를 타입으로 표현합니다. C/Java의 **null 포인터** 문제(널을 역참조해 크래시)를 해결합니다 — 값을 꺼내려면 반드시 `None` 경우를 `match`로 다뤄야 컴파일됩니다. "10억 달러짜리 실수"라 불리는 null을 타입 시스템으로 제거. 강의 13의 출력 매개변수→`Option` 변환의 핵심.

---

## 슬라이드 22: Results

### 원문 내용
> - Similar to Option<T>, but provides information about errors
> - Result<T, E> can be either Ok(T) or Err(E)
> ```rust
> fn divide(x: i32, y: i32) -> Result<i32, String> {
>   if y == 0 {
>     Err("division by zero".to_string())
>   } else {
>     Ok(x / y)
>   }
> }
> ```

### 번역
> - **`Result<T, E>`**: `Ok(T)`(성공값) 또는 `Err(E)`(에러값)인 enum. Option과 비슷하나 **에러 정보**를 담음.

### 해설

**개념 설명 — Result = 에러 처리**

`Result<T,E>`는 "성공(T) 또는 실패(이유 E)"를 표현합니다. `Option`(실패 이유 없음)과 달리 에러 정보 E를 담습니다. 예외(exception) 대신 **반환값으로 에러를 명시**하는 방식 — 에러 처리를 빠뜨릴 수 없게 합니다(타입에 드러나니까). 강의 13에서 C의 에러 코드(0=성공, -1=실패)를 Rust의 `Result`로 변환하는 것이 정확히 이 개념. 에러 처리 메커니즘이 슬라이드 23.

---

## 슬라이드 23: Error Handling

### 원문 내용
> - Rust does not have exceptions; errors should be explicitly expressed using Option or Result
>   - panic: unrecoverable error, terminates the program (not intended to be handled)
> - Error propagation can be done using the ? operator
> ```rust
> fn add_values_associated_with_keys(key1: i32, key2: i32, map: &HashMap<i32, i32>) -> Option<i32> {
>   let v1 = map.get(&key1)?;
>   let v2 = map.get(&key2)?;
>   Some(v1 + v2)
> }
> ```

### 번역
> - Rust엔 **예외(exception)가 없다**; 에러는 `Option`/`Result`로 명시
>   - `panic`: 복구 불가 에러, 프로그램 종료(처리 대상 아님)
> - **`?` 연산자**로 에러 전파: `map.get(&key1)?`는 `None`이면 즉시 `None` 반환, `Some(v)`면 `v`를 꺼냄

### 해설

**개념 설명 — 명시적 에러 처리와 `?`**

Rust는 예외 대신 `Option`/`Result`로 에러를 **값으로** 다룹니다. `?` 연산자는 "실패면 즉시 반환, 성공이면 값 추출"을 간결하게 해 줍니다(에러 전파). `panic`은 복구 불가능한 상황(배열 범위 초과 등)에서 프로그램을 멈춥니다. 이 명시적 에러 모델이 "에러를 무시할 수 없게" 만드는 안전 설계 — 강의 13의 출력 매개변수 변환이 이를 활용합니다(패턴 매칭 강제로 실패 처리 누락 방지).

---

## 슬라이드 24: Generics

### 원문 내용
> - Functions can be parameterized over types
> ```rust
> fn pop_until<T>(v: &mut Vec<T>, n: usize) {
>   while v.len() > n { v.pop(); }
> }
> ```
> - User-defined types can also be generic
> ```rust
> struct Point<T> { x: T, y: T }
> let p1: Point<f64> = ...;
> let p2: Point<i32> = ...;
> ```

### 번역
> - **제네릭(generics)**: 함수를 타입에 대해 매개변수화 (`<T>`)
> - 사용자 타입도 제네릭 가능 (`Point<T>` — f64 점, i32 점 등)

### 해설

**개념 설명 — 제네릭(다형성)**

`<T>`로 "어떤 타입이든" 동작하는 함수·타입을 만듭니다. `pop_until<T>`는 어떤 벡터에도 작동. 코드 중복 없이 여러 타입을 다루는 **매개변수 다형성(parametric polymorphism)**입니다. 강의 3~4의 타입 분석/추론은 이런 다형 타입을 다룹니다. C++의 템플릿, Java의 제네릭과 유사.

---

## 슬라이드 25: Traits

### 원문 내용
> - A trait represents a set of types that share common behavior
>   - Resembles type classes in Haskell
>   - Similar to interfaces or abstract classes in object-oriented languages
> ```rust
> trait HasArea { fn area(&self) -> f64; }
> struct Circle { radius: f64 }
> impl HasArea for Circle { fn area(&self) -> f64 { PI * self.radius * self.radius } }
> struct Rectangle { width: f64, height: f64 }
> impl HasArea for Rectangle { fn area(&self) -> f64 { self.width * self.height } }
> fn larger<T: HasArea>(a: T, b: T) -> T { if a.area() > b.area() { a } else { b } }
> ```

### 번역
> - **트레이트(trait)**: 공통 동작을 공유하는 타입들의 집합 (Haskell의 타입클래스, OOP의 인터페이스와 유사)
> - `HasArea`를 구현한 타입은 `area()` 메서드를 가짐. `larger<T: HasArea>`는 "넓이를 가진 어떤 타입이든" 받음(트레이트 바운드).

### 해설

**개념 설명 — 트레이트 = 능력의 인터페이스 ★**

트레이트는 "이 타입은 무엇을 할 수 있는가"를 정의합니다(인터페이스). `HasArea`를 구현한 타입은 `area()`를 가집니다. `<T: HasArea>`(트레이트 바운드)는 "HasArea를 구현한 타입만"이라는 제약. 이는 **능력 기반 추상화** — 강의 13의 I/O에서 `Read`/`Write`/`Seek` 트레이트로 스트림의 능력을 표현하고, 다중 출처 스트림을 `Box<dyn Read>`로 추상화하는 것이 정확히 이 트레이트입니다. 객체지향의 가상 메서드(강의 11 CHA/RTA)와도 연결됩니다.

---

## 슬라이드 26: Lifetimes

### 원문 내용
> - A lifetime represents how long a reference is valid
> - &'a T or &'a mut T indicates that the reference is valid for the lifetime 'a
> - Lifetime annotations can be omitted in many cases, but sometimes they are required to disambiguate lifetimes
> ```rust
> fn foo<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 { x }
> struct Point<'a> { x: &'a f64, y: &'a f64 }
> ```

### 번역
> - **수명(lifetime)**: 참조가 **유효한 기간**을 나타냄
> - `&'a T`는 "수명 `'a` 동안 유효한 참조". 많은 경우 생략 가능하나 때로 명시 필요.
> - 참조를 담는 타입은 수명에 대해 매개변수화됨(`Point<'a>`).

### 해설

**개념 설명 — 수명(lifetime)**

빌림(참조)은 **원본보다 오래 살면 안 됩니다**(원본이 사라진 뒤 참조하면 use-after-free). **수명**은 "이 참조가 얼마나 유효한가"를 컴파일러가 추적하는 장치입니다. `&'a T`는 "수명 `'a` 동안만 유효". 컴파일러의 **빌림 검사(borrow checker)**가 수명을 분석해, 참조가 원본보다 오래 사는 것을 막습니다 — 이것이 Rust 컴파일러 안의 정적 분석입니다. 강의 12의 가드 수명(보호 구간)도 이 개념의 응용. 수명 분석 자체가 일종의 데이터플로우/영역 분석입니다.

---

## 슬라이드 27: References (참고 자료)

### 원문 내용
> - The Rust Programming Language: https://doc.rust-lang.org/book
> - Rust By Example: https://doc.rust-lang.org/rust-by-example/
> - The Rust Standard Library: https://doc.rust-lang.org/std
> - The Rust Reference: https://doc.rust-lang.org/reference
> - Rust Playground: https://play.rust-lang.org/

### 번역
> Rust 학습 자료: 공식 책(The Book), Rust By Example, 표준 라이브러리 문서, 레퍼런스, 온라인 플레이그라운드.

### 해설
Rust를 더 배우고 싶을 때의 공식 자료들. 특히 Playground는 설치 없이 브라우저에서 Rust를 실행해 볼 수 있어 학습에 좋습니다. 여기까지가 **언어 기능** 파트. 슬라이드 28부터는 **컴파일러(rustc) 내부 구조** — 정적 분석기 구현의 토대.

---

## 슬라이드 28: The Rust Compiler (rustc)

### 원문 내용
> - The compiler consists of several passes that transform the source code into machine code:
>   Source →(parsing)→ AST →(desugaring & symbol resolution)→ HIR →(type checking)→ THIR →(lowering)→ MIR →(borrow checking & optimization)→ MIR →(code generation)→ machine code
> - It exposes its internal data structures and passes as APIs, which can be used for implementing static analyzers
> - Choosing the right code representation is important for implementing a static analyzer

### 번역
> - rustc 컴파일러는 소스→기계어 변환을 여러 **패스(pass)**로 수행:
>   소스 →(파싱)→ **AST** →(디슈가링·심볼 해석)→ **HIR** →(타입 검사)→ **THIR** →(낮추기)→ **MIR** →(빌림 검사·최적화)→ MIR →(코드 생성)→ 기계어
> - 내부 자료구조·패스를 **API로 노출**해 정적 분석기 구현에 활용 가능
> - **올바른 코드 표현(representation) 선택이 분석기 구현에 중요**

### 해설

**개념 설명 — 컴파일러 파이프라인 ★**

컴파일러는 소스를 여러 **중간 표현(IR)**을 거쳐 기계어로 바꿉니다. 각 단계는 점점 더 "낮은 수준"(기계에 가까운):
- **AST**: 소스의 트리 구조(슬29).
- **HIR**: 디슈가링·심볼 해석 후(슬30).
- **THIR**: 타입 정보 추가(슬31).
- **MIR**: 제어 흐름 그래프(슬32) — **분석에 가장 적합**.

핵심: rustc는 이 IR들을 **API로 공개**해, 우리가 분석기를 만들 때 가져다 쓸 수 있습니다(Assignment 4가 `rustc_middle`의 MIR을 사용). **어떤 IR을 분석 대상으로 고르느냐**가 중요한데(슬34), 보통 MIR을 씁니다(제어 흐름이 명시적이고 구조가 단순). 각 IR을 차례로 봅니다(슬29~33).

---

## 슬라이드 29: Abstract Syntax Tree (AST)

### 원문 내용
> - Represents code as a tree, not text
> - Similar to source code, but: Macros are expanded; Submodules implemented in other files are loaded (a single AST for the whole crate)
> - Example: (1 + 2) * 3 → tree with Mul(Add(Lit 1, Lit 2), Lit 3)
> - You can see the AST using `rustc -Z unpretty=ast-tree,expanded [file.rs]`

### 번역
> - **AST(추상 구문 트리)**: 코드를 텍스트가 아닌 **트리**로 표현
> - 소스와 비슷하나 매크로가 전개되고, 다른 파일의 서브모듈이 로드됨(크레이트 전체가 하나의 AST)
> - 예: `(1+2)*3` → `Mul(Add(1,2), 3)` 트리

### 해설

**개념 설명 — AST**

소스 코드를 파싱하면 **트리**가 됩니다 — `(1+2)*3`은 곱셈 노드 아래 덧셈 노드... 식으로. 텍스트보다 다루기 쉽지만, 아직 심볼(변수가 무엇을 가리키는지)이 해석 안 된 가장 높은 수준의 IR입니다. 매크로 전개·모듈 통합은 된 상태. 분석엔 정보가 부족해(슬34) 보통 HIR/MIR을 씁니다.

---

## 슬라이드 30: High-level Intermediate Representation (HIR)

### 원문 내용
> - Similar to AST, but: Desugared (e.g., for becomes loop); Symbols are resolved
> ```rust
> let x = 1;
> { let x = 2; println!("{x}"); }
> ```
> - What does this x refer to? (심볼 해석으로 명확해짐)
> - You can see the HIR using `-Z unpretty=hir` or `-Z unpretty=hir-tree`

### 번역
> - **HIR**: AST와 비슷하나 **디슈가링**(예: `for`가 `loop`로 변환)되고 **심볼이 해석**됨
> - 위 코드에서 안쪽 `x`가 어느 변수인지(스코프) 명확히 결정됨

### 해설

**개념 설명 — HIR**

HIR은 AST를 정리한 것입니다:
- **디슈가링**: 편의 문법(`for`, `?` 등)을 기본 형태(`loop`, match 등)로 풀어 — 다룰 경우의 수가 줄어듦.
- **심볼 해석**: "이 `x`는 어느 선언의 x인가"가 확정(섀도잉 해소).

타입 검사 전 단계라 타입은 아직 완전치 않습니다. 강의 34에서 밝히듯, **타입 분석(강의 3~4)에는 HIR을 사용**합니다(소스에 가깝고 부모 노드 순회가 편함). 핵심 타입(`HirId`, `Item`, `Expr`, `Visitor`)이 슬35.

---

## 슬라이드 31: Typed High-level Intermediate Representation (THIR)

### 원문 내용
> - Similar to HIR, but: Overloading is resolved; Implicit coercions are made explicit
> ```
> HIR:  fn foo(x: &mut i32) -> &i32 { x }
> THIR: fn foo(x: &mut i32) -> &i32 { &*x }   // coercion explicit
> ```
> - You can see the THIR using `-Z unpretty=thir-tree`

### 번역
> - **THIR**: HIR과 비슷하나 **오버로딩 해소**, **암묵적 강제 변환(coercion)을 명시화**
> - 예: HIR의 `x`(타입 강제 변환 암묵)가 THIR에선 `&*x`로 명시됨

### 해설

**개념 설명 — THIR**

THIR은 **타입 검사 후** 표현입니다. 오버로딩(같은 연산자가 타입마다 다른 의미)이 해소되고, 암묵적 타입 변환(coercion)이 코드에 드러납니다(`x` → `&*x`). 타입 정보가 완전한 표현. MIR로 낮추기 전 단계로, 일반 분석엔 잘 안 쓰이고 주로 컴파일러 내부에서 MIR 생성에 쓰입니다.

---

## 슬라이드 32: Mid-level Intermediate Representation (MIR)

### 원문 내용
> - A control-flow graph (CFG) representation
>   - A function consists of basic blocks and edges between them
>   - Each basic block consists of a sequence of statements and a terminator
>   - Statement: assignment
>   - Terminator: jump, switch, call, return

### 번역
> - **MIR(중간 수준 IR)**: **제어 흐름 그래프(CFG)** 표현
>   - 함수 = **기본 블록(basic block)**들과 그 사이 간선
>   - 각 기본 블록 = **문장(statement)들의 수열 + 종결자(terminator)**
>   - 문장: 대입
>   - 종결자: 점프(jump)·스위치(switch)·호출(call)·반환(return)

### 해설

**개념 설명 — MIR = 분석의 무대 ★**

MIR은 **제어 흐름 그래프(CFG)**로, 정적 분석에 가장 적합한 표현입니다(강의 7~9의 데이터플로우, Assignment 4가 모두 MIR 기반):
- **기본 블록**: 분기 없이 쭉 실행되는 문장 묶음.
- **문장**: 대입(`x = ...`)만.
- **종결자**: 블록 끝에서 어디로 갈지 — 점프(Goto), 스위치(분기), 호출(Call), 반환(Return).

이 구조는 강의 7의 CFG, 강의 11의 SwitchInt 처리, Assignment 4의 MIR 분석과 정확히 일치합니다. **실행 순서가 명시적**(블록 간 간선)이라 흐름 감각 분석(강의 7~9, 15)에 이상적. 예가 슬33.

---

## 슬라이드 33: MIR — Example

### 원문 내용
> ```rust
> let mut x = 0;
> let mut y = 0;
> while x < 10 { x += 1; y += x; }
> return y;
> ```
> MIR (CFG):
> - bb0: x=0; y=0; jump bb1
> - bb1: t = x < 10; switch t (T: bb2, F: bb3)
> - bb2: x = x + 1; y = y + x; jump bb1
> - bb3: return y

### 번역
> while 루프가 MIR에서 4개 기본 블록으로: bb0(초기화)→bb1(조건 검사·스위치)→bb2(본문, bb1로 복귀)/bb3(반환). 루프가 **bb2→bb1 순환 간선**으로 표현됨.

### 해설

**개념 설명 — 루프가 CFG가 되다**

소스의 `while` 루프가 MIR에서는 **기본 블록 + 간선**으로 풀립니다: 조건 블록(bb1)이 스위치로 본문(bb2)·탈출(bb3)을 가르고, 본문은 다시 조건으로 돌아갑니다(순환). 이 형태가 Assignment 4의 테스트 MIR, 강의 7~9의 데이터플로우 분석 대상과 똑같습니다. 순환 간선이 있어 고정점 반복·위드닝(강의 9)이 필요해집니다. `switch`는 강의 11의 SwitchInt와 동일.

---

## 슬라이드 34: Comparison of Code Representations

### 원문 내용
> - AST, HIR, THIR vs. MIR: MIR is suitable for most analyses because its CFG structure makes execution order explicit and it has fewer language constructs. If the execution order does not matter (flow-insensitive analysis), other representations may be used, especially when results should be close to the source code.
> - AST vs. HIR, THIR: Symbols are not resolved in AST, so HIR and THIR are more convenient
> - HIR vs. THIR: HIR is useful when traversing parent nodes is required or even ill-typed code should be analyzed; THIR is useful if type information is important
> - In this course, MIR is used in most cases, but HIR is used for type analysis

### 번역
> - **MIR vs 나머지**: MIR은 CFG라 **실행 순서가 명시적**이고 구문이 단순해 **대부분 분석에 적합**. 흐름 무감각 분석이거나 결과가 소스에 가까워야 하면 다른 표현 사용.
> - **AST vs HIR/THIR**: AST는 심볼 미해석이라 HIR/THIR이 편함.
> - **HIR vs THIR**: HIR은 부모 노드 순회·잘못된 타입 코드 분석에, THIR은 타입 정보가 중요할 때.
> - **이 과목**: 대부분 MIR, 단 **타입 분석은 HIR** 사용.

### 해설

**개념 설명 — 어떤 IR을 고를까 (분석 설계의 첫 결정)**

분석 대상 IR 선택의 지침입니다. 핵심: **MIR은 흐름 감각·제어 흐름 분석에 최적**(실행 순서 명시, 구문 단순)이라 대부분의 분석(데이터플로우·포인터·구간)에 씁니다. 단 **타입 분석(강의 3~4)은 HIR**을 쓰는데, 타입 분석은 소스 구조(부모-자식 관계)와 가까워야 하고 흐름 순서가 덜 중요하기 때문. 이 "IR 선택"이 곧 강의 1의 "올바른 추상화 고르기"(슬31)의 실무 버전입니다. 분석에 쓰는 핵심 타입들이 슬35(HIR)·36~37(MIR).

---

## 슬라이드 35: Important Types in HIR

### 원문 내용
> - LocalDefId: a unique identifier for a top-level item in the crate
> - DefId: a unique identifier for a top-level item in any crate, including dependencies (DefId = LocalDefId + crate ID)
> - HirId: a unique identifier for any node in the HIR; each local variable has a unique HirId as well
> - Item: a top-level item; Stmt: a statement; Expr: an expression
> - Visitor: a trait for traversing the HIR; you can implement your own logic by defining a struct that implements this trait

### 번역
> HIR의 핵심 타입: `LocalDefId`(크레이트 내 최상위 아이템 ID), `DefId`(의존성 포함 전역 아이템 ID = LocalDefId+크레이트ID), `HirId`(HIR의 모든 노드·지역 변수 ID), `Item`(최상위 아이템), `Stmt`(문장), `Expr`(식), `Visitor`(HIR 순회 트레이트 — 직접 구현해 분석 로직 작성).

### 해설

**개념 설명 — HIR 분석 도구상자**

타입 분석(강의 3~4)을 HIR로 구현할 때 쓰는 타입들입니다. 핵심은 **`Visitor` 트레이트** — 이걸 구현한 구조체를 만들면 HIR 트리를 순회하며 원하는 정보를 모을 수 있습니다(방문자 패턴). `DefId`/`HirId`는 노드·정의를 가리키는 고유 ID. Assignment의 타입 분석 과제에서 직접 다루게 됩니다.

---

## 슬라이드 36: Important Types in MIR

### 원문 내용
> - Body: a function in MIR; BasicBlock: a unique identifier for a basic block; BasicBlockData: a basic block; Statement: a statement; Terminator: a terminator
> - Place: a memory location; can be used as the LHS of an assignment
> - Rvalue: the RHS of an assignment
> - Operand: an operand used in an Rvalue (a place or a constant)
> - Const: a constant value

### 번역
> MIR의 핵심 타입: `Body`(MIR의 함수), `BasicBlock`(기본 블록 ID), `BasicBlockData`(기본 블록 내용), `Statement`(문장), `Terminator`(종결자), `Place`(메모리 위치, 대입 좌변), `Rvalue`(대입 우변), `Operand`(Rvalue의 피연산자 — place 또는 상수), `Const`(상수값).

### 해설

**개념 설명 — MIR 분석 도구상자 ★**

이 타입들은 **Assignment 4와 강의 7~9·14~15 분석의 직접적 어휘**입니다:
- `Body`: 한 함수의 MIR 전체.
- `BasicBlockData`: 기본 블록(문장들 + 종결자).
- **`Place`(좌변, 메모리 위치) vs `Rvalue`(우변, 계산식) vs `Operand`(피연산자)**: 대입 `Place = Rvalue` 구조. 강의 14의 포인터 문장(`x = &y`, `*x = y` 등)이 이 Place/Rvalue로 표현됩니다.

Assignment 4의 `analysis.rs`가 정확히 이 타입들(`Body`, `BasicBlock`, `Place`, `Rvalue`, `Operand`)을 import해서 씁니다. `Local`이 슬37.

---

## 슬라이드 37: Important Types in MIR (cont.)

### 원문 내용
> - Local: a unique identifier for a local variable
>   - Place is a Local with projections (e.g., field access)
>   - Each Local is represented as an integer
>   - _0 is the return value; _1, _2, ... are parameters
> ```
> fn add(x: i32, y: i32) -> i32 { x + y }
> → _0 = Add(_1, _2); return;
> ```
> - Visitor: a trait for traversing the MIR

### 번역
> - **`Local`**: 지역 변수 ID(정수로 표현). `Place`는 `Local` + 프로젝션(필드 접근 등).
>   - **`_0`은 반환값, `_1, _2, ...`는 매개변수**, 이후는 지역 변수
> - 예: `add(x,y)`의 MIR은 `_0 = _1 + _2; return;`
> - `Visitor`: MIR 순회 트레이트

### 해설

**개념 설명 — Local 번호 규칙 ★**

MIR에서 변수는 **정수 Local**로 표현됩니다. 규칙이 중요합니다:
- **`_0` = 반환값(RET)**, **`_1, _2, ... = 매개변수**(선언 순), 이후 = 지역 변수.

이 규칙은 Assignment 4 명세(`_0=RET, _1=첫 파라미터...`)와 강의 10의 절차간 분석에서 그대로 쓰입니다. `Place`는 Local에 프로젝션(`.field`, `[i]`)을 붙인 것이지만, Assignment 4는 "프로젝션 없는 순수 Local"만 다룬다고 가정했습니다. `Visitor`로 MIR을 순회해 분석을 구현합니다(Assignment 4의 CmpVisitor가 그 예). 컴파일러의 중심 자료구조가 슬38.

---

## 슬라이드 38: TyCtxt

### 원문 내용
> - The central data structure of the compiler²
> - When invoking the compiler, a TyCtxt value is given, and many APIs are provided as methods of TyCtxt
> - Examples: hir_visit_all_item_likes_in_crate (visits all item-likes in the crate in some deterministic order); parent_hir_id (returns the HirId of the parent HIR node); optimized_mir (MIR after the optimization passes have run)

### 번역
> - **`TyCtxt`**: 컴파일러의 **중심 자료구조**
> - 컴파일러 호출 시 `TyCtxt` 값이 주어지고, 많은 API가 그 메서드로 제공됨
> - 예: `hir_visit_all_item_likes_in_crate`(크레이트의 모든 아이템 순회), `parent_hir_id`(부모 HIR 노드), `optimized_mir`(최적화 후 MIR 획득)

### 해설

**개념 설명 — TyCtxt = 분석의 진입점**

`TyCtxt`(type context)는 컴파일러의 모든 정보를 담은 **중심 객체**입니다. 분석기는 이걸 받아 시작합니다 — Assignment 4의 `analyze(tcx: TyCtxt)`가 바로 이것. `tcx.optimized_mir(def_id)`로 함수의 MIR을 얻고, `tcx.hir_body_owners()`로 모든 함수를 순회합니다(Assignment 4의 `analyze`가 정확히 이렇게 함). 즉 **`TyCtxt`가 rustc 기반 분석기의 출발점**입니다.

---

## 슬라이드 39: References (참고 자료)

### 원문 내용
> - HIR: https://doc.rust-lang.org/stable/nightly-rustc/rustc_hir/hir
> - MIR: https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir
> - Rust Compiler Development Guide: https://rustc-dev-guide.rust-lang.org/

### 번역
> rustc 내부 API 문서: HIR(`rustc_hir`), MIR(`rustc_middle::mir`), 그리고 컴파일러 개발 가이드.

### 해설
정적 분석기 구현(과제)에 필요한 rustc 내부 API 문서. Assignment 4가 `rustc_middle::mir`의 타입들을 직접 씁니다. 컴파일러 개발 가이드는 rustc 내부 구조를 깊이 다룹니다.

---

## 슬라이드 40: Summary

### 원문 내용
> - Rust is a systems programming language with memory safety guaranteed by the type system at compile time
> - Key language features: variables, functions, types, ownership, borrowing, enums, traits, lifetimes
> - The rustc compiler pipeline: AST → HIR → THIR → MIR → machine code
> - MIR (control-flow graph) is most suitable for static analysis due to explicit execution order and fewer language constructs
> - Key types for analysis: HIR (HirId, Item, Expr); MIR (Body, BasicBlockData, Statement, Terminator, Place, Rvalue, Operand, Const, Local); LocalDefId, DefId, TyCtxt, Visitor

### 번역
> - Rust = 컴파일 타임 타입 시스템으로 메모리 안전성을 보장하는 시스템 언어
> - 핵심 기능: 변수·함수·타입·소유권·빌림·enum·트레이트·수명
> - rustc 파이프라인: AST → HIR → THIR → MIR → 기계어
> - **MIR(CFG)**이 실행 순서 명시·단순 구문 덕에 정적 분석에 가장 적합
> - 분석용 핵심 타입: HIR(HirId/Item/Expr), MIR(Body/Statement/Terminator/Place/Rvalue/Operand/Const/Local), TyCtxt/Visitor

### 해설

**전체 정리 — 강의 2의 한 장 요약**

1. **언어**: Rust는 소유권·빌림·앨리어싱 XOR 가변성으로 **컴파일 타임에** 메모리 안전성을 보장. enum(합)·struct(곱)=ADT, 트레이트, 수명 등 풍부한 기능.
2. **컴파일러**: 소스 → AST → HIR → THIR → MIR → 기계어. 각 IR은 점점 낮은 수준.
3. **분석 대상**: 대부분 **MIR**(CFG, 실행 순서 명시), 타입 분석만 HIR. `TyCtxt`가 진입점.

**다른 강의와의 연결 (파일 간 연결성)**

- → **Assignment 4 / 강의 7~9 (데이터플로우·구간)**: MIR의 Body·BasicBlock·Place·Rvalue·Local(_0=RET, _1=param)을 직접 분석. CFG 구조가 그대로.
- → **강의 3~4 (타입 분석)**: HIR과 그 Visitor를 사용.
- → **강의 11 (제어 흐름)**: 함수 포인터·클로저(슬16~17)가 "함수가 값" 문제를, MIR의 SwitchInt(슬33)가 분기를 제공.
- → **강의 12 (락)**: 소유권·`Drop`·RAII가 `MutexGuard`(가드 수명=보호 구간)의 기반.
- → **강의 13 (출력 매개변수·I/O)**: enum·`Option`/`Result`(ADT)와 트레이트(`Read`/`Write`)가 변환의 목표.
- → **강의 14~15 (포인터)**: Rust의 앨리어싱 XOR 가변성이, C 포인터 분석이 씨름하는 앨리어싱을 정적으로 통제하는 모델.

**가장 큰 교훈**: Rust의 안전성은 **공짜가 아니라 정적 분석(빌림 검사·수명 분석)의 산물**입니다. 그리고 이 과목의 분석은 Rust 컴파일러가 노출한 **MIR(CFG)**을 무대로 삼습니다 — 소유권·빌림 같은 언어 개념과 MIR 같은 컴파일러 구조를 함께 이해해야 이후의 분석들(특히 과제)을 따라갈 수 있습니다.

---

## 마치며

강의 2는 이 과목의 **도구(Rust)이자 무대(MIR)**를 소개합니다. 전반부의 **소유권·빌림·앨리어싱 XOR 가변성**은 Rust가 메모리 안전성을 *정적 분석으로* 달성하는 방식이며, 강의 12~15의 응용·포인터 분석으로 직접 이어집니다. 후반부의 **컴파일러 파이프라인(특히 MIR)**과 핵심 타입들은 모든 프로그래밍 과제(특히 Assignment 4)와 강의 7~9의 데이터플로우 분석의 토대입니다. 시험에서는 (a) 소유권/이동/Copy·Clone과 use-after-free 방지(슬10~12), (b) 앨리어싱 XOR 가변성 규칙과 그 안전성 근거(슬14), (c) enum/Option/Result(ADT)와 트레이트(슬19~25, 강의 13 예고), (d) 다섯 IR(AST/HIR/THIR/MIR)의 차이와 분석에 MIR이 적합한 이유(슬32~34), (e) MIR의 Local 규칙(_0=RET, _1=param)과 Place/Rvalue/Operand(슬36~37)가 단골입니다.
