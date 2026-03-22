# CSE552 Lecture 2: Introduction to Rust - 상세 해설

## 목차
1. [슬라이드 1-2: 제목 및 Rust 소개](#슬라이드-1-2-제목-및-rust-소개)
2. [슬라이드 3-5: 변수와 타입](#슬라이드-3-5-변수와-타입)
3. [슬라이드 6-7: 함수](#슬라이드-6-7-함수)
4. [슬라이드 8-9: 제어 흐름](#슬라이드-8-9-제어-흐름)
5. [슬라이드 10-11: 소유권](#슬라이드-10-11-소유권)
6. [슬라이드 12-13: 차용(Borrowing)](#슬라이드-12-13-차용borrowing)
7. [슬라이드 14-15: 내부 가변성](#슬라이드-14-15-내부-가변성)
8. [슬라이드 16-17: 함수 포인터와 클로저](#슬라이드-16-17-함수-포인터와-클로저)
9. [슬라이드 18-19: 구조체와 열거형](#슬라이드-18-19-구조체와-열거형)
10. [슬라이드 20: 패턴 매칭](#슬라이드-20-패턴-매칭)
11. [슬라이드 21: Option 타입](#슬라이드-21-option-타입)
12. [슬라이드 22: Result 타입](#슬라이드-22-result-타입)
13. [슬라이드 23: 에러 처리](#슬라이드-23-에러-처리)
14. [슬라이드 24: 제네릭](#슬라이드-24-제네릭)
15. [슬라이드 25: 트레이트](#슬라이드-25-트레이트)
16. [슬라이드 26: 라이프타임](#슬라이드-26-라이프타임)
17. [슬라이드 27: 학습 자료](#슬라이드-27-학습-자료)
18. [슬라이드 28: Rust 컴파일러(rustc)](#슬라이드-28-rust-컴파일러rustc)
19. [슬라이드 29: 추상 구문 트리(AST)](#슬라이드-29-추상-구문-트리ast)
20. [슬라이드 30: 고수준 중간 표현(HIR)](#슬라이드-30-고수준-중간-표현hir)
21. [슬라이드 31: 타입 지정 고수준 IR(THIR)](#슬라이드-31-타입-지정-고수준-irthir)
22. [슬라이드 32: 중간 수준 표현(MIR)](#슬라이드-32-중간-수준-표현mir)
23. [슬라이드 33: MIR 예제](#슬라이드-33-mir-예제)
24. [슬라이드 34: 코드 표현 비교](#슬라이드-34-코드-표현-비교)
25. [슬라이드 35: HIR의 중요한 타입](#슬라이드-35-hir의-중요한-타입)
26. [슬라이드 36: MIR의 중요한 타입](#슬라이드-36-mir의-중요한-타입)
27. [슬라이드 37: MIR의 중요한 타입 계속](#슬라이드-37-mir의-중요한-타입-계속)
28. [슬라이드 38: TyCtxt](#슬라이드-38-tyctxt)
29. [슬라이드 39: 참고 자료](#슬라이드-39-참고-자료)
30. [슬라이드 40: 요약](#슬라이드-40-요약)

---

## 슬라이드 1-2: 제목 및 Rust 소개

### 개념 설명

이 강의는 **Rust 프로그래밍 언어**를 소개합니다. Rust는 C/C++과 같은 저수준 시스템 프로그래밍을 지원하면서도, **컴파일 타임에 메모리 안전성을 보장**하는 현대적인 프로그래밍 언어입니다.

핵심 특징:
- **시스템 프로그래밍 언어(Systems Programming Language)**: 운영체제, 임베디드 시스템, 성능이 중요한 애플리케이션 개발
- **메모리 안전성(Memory Safety)**: 컴파일 타임에 메모리 관련 버그(메모리 누수, 포인터 오류 등)를 자동으로 감지하고 방지
- **zero-cost abstraction**: 추상화로 인한 성능 저하가 거의 없음

### 배경 지식

C/C++ 경험자라면:
- C: 포인터 직접 관리, 메모리 안전성 개발자 책임
- C++: 객체 지향 + 스마트 포인터, 여전히 수동 메모리 관리

Rust는 이들의 **성능**은 유지하면서 **안전성**을 자동으로 보장합니다.

### 수식/기호/코드 설명

코드 예제는 이후 슬라이드에서 자세히 다루지만, 기본적으로:
```rust
fn main() {
    let x = 5;  // 불변 바인딩
    println!("Hello, Rust!");
}
```

### 전체적인 맥락

CSE552는 **프로그램 분석(Program Analysis)** 강좌입니다. 이 강좌에서:
- **Rust를 구현 언어로 사용**: 프로그램 분석 도구를 Rust로 개발
- Rust의 특징: 안전하고 빠른 분석 도구 작성 가능
- 컴파일러 구조 이해: Rust 컴파일러의 내부 구조를 분석하는 것이 핵심

---

## 슬라이드 3-5: 변수와 타입

### 개념 설명

#### 변수 선언 (Let Bindings)

Rust의 변수 선언은 **let** 키워드를 사용합니다:
- **`let x = 5;`**: 불변(immutable) 바인딩. 변수는 선언 후 변경 불가
- **`let mut y = 10;`**: 가변(mutable) 바인딩. `mut` 키워드로 명시적으로 변경 가능하게 선언

이것이 **Rust의 철학**: 기본적으로 불변이고, 변경이 필요할 때만 `mut`을 붙임

### 배경 지식

**불변(immutable) vs 가변(mutable)**:
- C/C++에서는 const 키워드로 상수를 선언하지만, 기본은 변수가 가변
- Rust는 정반대: 기본이 불변, 필요할 때만 가변

이 차이는 **동시성(concurrency) 안전성**에 중요합니다.

### 수식/기호/코드 설명

#### 기본 타입들

```rust
// 정수 타입
let a: i32 = 42;        // 32비트 부호있는 정수
let b: u64 = 1000;      // 64비트 부호없는 정수

// 실수 타입
let c: f64 = 3.14;      // 64비트 부동소수점

// 논리 타입
let d: bool = true;     // true 또는 false

// 문자 타입
let e: char = 'A';      // 단일 문자 (Unicode)

// 문자열 타입
let f: &str = "hello";           // 문자열 슬라이스 (고정 크기)
let g: String = String::from("world");  // 동적 문자열

// 튜플
let tuple: (i32, f64, bool) = (42, 3.14, true);

// 배열
let arr: [i32; 3] = [1, 2, 3];   // 길이 3의 배열
```

#### 타입 추론(Type Inference)

```rust
let x = 5;      // 컴파일러가 타입을 추론: i32
let y = 3.14;   // f64로 추론
```

컴파일러가 문맥을 보고 타입을 자동으로 결정합니다.

#### 표현식(Expression) vs 문(Statement)

```rust
// 표현식: 값을 반환
let x = {
    let y = 3;
    y + 1  // 세미콜론 없음! 이 값이 반환됨
};  // x = 4

// 문: 값을 반환하지 않음
let y = (let z = 3);  // 에러! let은 문(statement)

// 함수 본체의 마지막 표현식
fn add_one(x: i32) -> i32 {
    x + 1  // 세미콜론 없음! 반환값
}

// vs

fn add_one_statement(x: i32) -> i32 {
    x + 1;  // 세미콜론 있음! 아무것도 반환하지 않음 (컴파일 에러)
}
```

### 전체적인 맥락

변수와 타입은 Rust 프로그래밍의 **기초**입니다:
- 불변/가변 구분: 메모리 안전성의 첫 번째 계층
- 타입 시스템: 컴파일 타임에 많은 버그를 잡음
- 표현식: 함수형 프로그래밍 스타일 지원

---

## 슬라이드 6-7: 함수

### 개념 설명

함수는 **재사용 가능한 코드 블록**입니다. Rust 함수는 명시적인 타입 선언이 특징입니다.

### 배경 지식

- C/C++: 함수 선언, 정의 분리 가능
- Java: 메서드는 클래스 내에만 존재
- Rust: 최상위 함수와 메서드(타입에 속한) 모두 지원

### 수식/기호/코드 설명

#### 함수 선언

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y  // 반환값 (세미콜론 없음!)
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
    // 반환값 없음 (마지막 표현식이 없음)
}
```

**구조**:
- `fn`: 함수 정의 키워드
- `add`: 함수 이름
- `(x: i32, y: i32)`: 매개변수들 (타입 명시 필수)
- `-> i32`: 반환 타입
- 마지막 표현식 (세미콜론 없음)이 반환값

#### if/else as 표현식

```rust
fn absolute_value(x: i32) -> i32 {
    if x >= 0 {
        x
    } else {
        -x
    }  // 전체가 표현식, 값을 반환
}

// 더 간결하게
let max = if x > y { x } else { y };
```

### 전체적인 맥락

함수는 프로그래밍의 기본 단위이며:
- 재사용 가능한 로직 캡슐화
- 타입 안전성: 입출력 타입 명시
- 표현식 기반: 함수형 프로그래밍 스타일

---

## 슬라이드 8-9: 제어 흐름

### 개념 설명

프로그램의 실행 경로를 제어하는 구조들입니다.

### 배경 지식

모든 프로그래밍 언어의 기본 구조이지만, Rust는 이들을 **표현식**으로 취급합니다.

### 수식/기호/코드 설명

#### if/else

```rust
if x > 5 {
    println!("x is greater than 5");
} else if x == 5 {
    println!("x is 5");
} else {
    println!("x is less than 5");
}
```

#### while 루프

```rust
let mut counter = 0;
while counter < 5 {
    println!("{}", counter);
    counter += 1;
}
```

#### for 루프

```rust
// 범위 반복
for i in 0..5 {  // 0부터 4까지 (5 제외)
    println!("{}", i);
}

// 컬렉션 반복
let arr = [1, 2, 3];
for item in arr.iter() {
    println!("{}", item);
}
```

#### loop (무한 루프)

```rust
loop {
    println!("infinite loop!");
    break;  // 탈출 가능
}

// 루프에서 값 반환
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;  // 20을 반환
    }
};
```

#### match (패턴 매칭)

```rust
match x {
    0 => println!("zero"),
    1 | 2 => println!("one or two"),
    3..=5 => println!("three to five"),
    _ => println!("something else"),
}
```

### 전체적인 맥락

제어 흐름은 프로그램 로직의 핵심입니다:
- if/else: 조건부 실행
- 루프: 반복
- match: **매우 강력한 패턴 매칭** (이후 자세히)

---

## 슬라이드 10-11: 소유권

### 개념 설명

**Rust의 핵심 개념**: 메모리 안전성을 보장하는 메커니즘입니다.

**소유권 규칙**:
1. 각 값은 정확히 하나의 소유자(owner)를 가짐
2. 소유자가 스코프를 벗어나면, 값은 자동으로 해제됨 (drop)
3. 값의 소유권은 이동(move)할 수 있음

### 배경 지식

**메모리 관리 패러다임**:
- C/C++: 수동 메모리 관리 (malloc/new, free/delete)
- Java/Python: 자동 메모리 관리 (garbage collection)
- **Rust: 컴파일 타임 자동 메모리 관리 (소유권 시스템)**

Rust는 GC 없이도 안전한 메모리 관리가 가능합니다.

### 수식/기호/코드 설명

#### 기본 소유권

```rust
let s1 = String::from("hello");  // s1이 "hello" 소유
let s2 = s1;                      // 소유권이 s1에서 s2로 이동

// 이제 s1은 유효하지 않음!
println!("{}", s1);  // 컴파일 에러!
println!("{}", s2);  // OK: "hello"
```

메모리 상태:
```
s1 = String {              s2 = String {
    ptr: 0x100              ptr: 0x100
    len: 5                  len: 5
    capacity: 5             capacity: 5
}                          }
데이터: "hello" (0x100)
```

s1에서 s2로 이동하면, s1은 무효화됩니다.

#### 움직임(Move) vs 복사(Copy)

```rust
// 움직임 (이동)
let s1 = String::from("hello");
let s2 = s1;  // 소유권 이동

// 복사 (작은 타입들)
let x = 5;
let y = x;  // x의 값이 복사됨, x는 여전히 유효

// Copy 타입: i32, f64, bool, char 등 스택에 저장된 작은 타입들
// Non-Copy 타입: String, Vec 등 힙 메모리 사용
```

#### Clone (명시적 복사)

```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // 명시적으로 복사

println!("{}", s1);  // OK: "hello"
println!("{}", s2);  // OK: "hello"
```

clone()은 깊은 복사(deep copy)를 수행합니다.

### 전체적인 맥락

소유권은 Rust의 **혁신적인** 특징:
- **컴파일 타임에 메모리 안전성 보장**
- GC의 오버헤드 없음
- 명시적 소유권 추적으로 코드가 명확함
- 이후 "차용(Borrowing)"과 함께 작동

---

## 슬라이드 12-13: 차용(Borrowing)

### 개념 설명

**소유권이 아닌 임시 접근**을 제공합니다. 값의 소유권을 넘기지 않고도 사용할 수 있습니다.

**두 가지 종류의 참조(Reference)**:
1. **`&T`**: 공유 참조(shared reference), 불변, 여러 개 가능
2. **`&mut T`**: 배타적 참조(mutable reference), 가변, 최대 1개

### 배경 지식

C/C++ 포인터와 비슷하지만, **컴파일 타임에 안전성을 보장**합니다:
- C: 포인터는 언제든 null이거나 dangling 가능
- Rust: 참조는 항상 유효한 메모리를 가리킴

### 수식/기호/코드 설명

#### 공유 참조 (&T)

```rust
let s = String::from("hello");
let r1 = &s;  // s를 참조
let r2 = &s;  // s를 다시 참조

println!("{}, {}", r1, r2);  // OK

// s는 여전히 소유권을 가짐
println!("{}", s);  // OK
```

메모리 상태:
```
s (소유) → String { ptr: 0x100 } → "hello"
r1 (참조) ↗
r2 (참조) ↗
```

#### 배타적 참조 (&mut T)

```rust
let mut s = String::from("hello");
let r = &mut s;  // s를 가변 참조

r.push_str(" world");  // 참조를 통해 수정

println!("{}", s);  // OK: "hello world"
```

#### 별칭 XOR 가변성 규칙 (Aliasing XOR Mutability)

```rust
let mut x = 5;

let r1 = &x;      // 공유 참조
let r2 = &x;      // 다른 공유 참조 - OK

// let r3 = &mut x;  // 컴파일 에러! r1, r2가 활성화되어 있음

println!("{}, {}", r1, r2);  // r1, r2 마지막 사용

let r3 = &mut x;  // 이제 OK! r1, r2가 더 이상 사용되지 않음
r3 = 10;

println!("{}", x);  // OK: 10
```

**규칙**:
- 공유 참조 여러 개 + 배타적 참조 0개: OK
- 배타적 참조 1개 + 공유 참조 0개: OK
- 둘 다 있음: 컴파일 에러

### 전체적인 맥락

차용은 소유권 시스템의 **보완**입니다:
- 함수에 값을 전달할 때 소유권 이동 방지
- 임시 접근만 필요할 때 사용
- **데이터 경쟁(data race) 방지**: 동시성 문제를 컴파일 타임에 방지

---

## 슬라이드 14-15: 내부 가변성

### 개념 설명

**공유 참조를 통한 변경**을 가능하게 합니다.

"소유권 규칙은 너무 엄격하다"는 상황을 위한 솔루션입니다. 컴파일 타임에 증명할 수 없는 경우, 런타임에 borrow checking을 수행합니다.

### 배경 지식

**내부 가변성(Interior Mutability)** 패턴:
- 컴파일러가 항상 규칙을 검증할 수 없는 경우
- 런타임 검사로 안전성 보장
- 의도적으로 성능과 안전성 간 트레이드오프

### 수식/기호/코드 설명

#### Cell<T> (런타임 비용 없음)

```rust
use std::cell::Cell;

let x = Cell::new(5);

// Cell을 가변 참조 없이 내용 변경
x.set(10);

let value = x.get();
println!("{}", value);  // 10

// Cell: Copy 타입에만 사용 가능
```

**특징**:
- 런타임 오버헤드 없음
- Copy 타입만 사용 가능 (get() 반환값이 복사됨)
- 여러 &Cell 가능

#### RefCell<T> (런타임 borrow checking)

```rust
use std::cell::RefCell;

let s = RefCell::new(String::from("hello"));

{
    let mut borrowed = s.borrow_mut();  // 가변 빌림
    borrowed.push_str(" world");
}  // 빌림 반환

let borrowed = s.borrow();  // 불변 빌림
println!("{}", *borrowed);  // "hello world"

// 런타임 패닉 예제
let r1 = s.borrow();
// let r2 = s.borrow_mut();  // 런타임 패닉! r1이 여전히 빌려짐
```

**특징**:
- 런타임에 borrow 규칙 검사
- 규칙 위반 시 panic (프로그램 중단)
- 모든 타입 사용 가능

#### 사용 사례

```rust
// 캐시 예제
struct Cache {
    // 계산 결과를 저장하는 캐시 (내부 가변성 필요)
    data: RefCell<Vec<i32>>,
}

impl Cache {
    fn compute(&self, x: i32) -> i32 {
        // &self (불변 참조)이지만 내부 수정
        let mut data = self.data.borrow_mut();
        // 캐시 계산...
        x * 2
    }
}
```

### 전체적인 맥락

내부 가변성은:
- **Rust의 유연성** 제공
- 컴파일 타임 검사의 한계 극복
- 하지만 런타임 패닉 위험 (신중히 사용)
- 프로그램 분석 도구에서 상태 추적에 유용

---

## 슬라이드 16-17: 함수 포인터와 클로저

### 개념 설명

**일급 함수(First-class Function)**를 지원합니다. 함수를 값처럼 다룰 수 있습니다.

### 배경 지식

**고차 함수(Higher-order Function)**: 함수를 인자로 받거나 반환하는 함수
- C: 함수 포인터
- C++: std::function, 람다
- Java: 메서드 참조, 람다 (Java 8+)
- Python: 함수는 일급 객체

### 수식/기호/코드 설명

#### 함수 포인터

```rust
// 함수 포인터 타입
fn add(x: i32, y: i32) -> i32 {
    x + y
}

let f: fn(i32, i32) -> i32 = add;  // 함수 포인터
let result = f(3, 4);  // 7

// 고차 함수
fn apply_operation(x: i32, y: i32, op: fn(i32, i32) -> i32) -> i32 {
    op(x, y)
}

println!("{}", apply_operation(3, 4, add));  // 7
```

**구문**: `fn(T) -> U`
- T: 입력 타입
- U: 반환 타입

#### 클로저 (Closure)

```rust
// 기본 클로저
let add_five = |x| x + 5;
println!("{}", add_five(10));  // 15

// 타입 명시
let add_five: fn(i32) -> i32 = |x| x + 5;

// 환경 캡처
let y = 10;
let add_y = |x| x + y;  // y를 캡처
println!("{}", add_y(5));  // 15

// 가변 환경 캡처
let mut count = 0;
let mut increment = || {
    count += 1;
    count
};
println!("{}", increment());  // 1
println!("{}", increment());  // 2
```

#### Fn, FnMut, FnOnce 트레이트

```rust
// Fn(&self): 불변 캡처, 여러 번 호출 가능
let numbers = vec![1, 2, 3];
let print = || println!("{:?}", numbers);
print();  // OK
print();  // OK

// FnMut(&mut self): 가변 캡처, 여러 번 호출 가능
let mut count = 0;
let mut increment = || count += 1;
increment();  // OK
increment();  // OK

// FnOnce(self): 소유권 캡처, 한 번만 호출 가능
let x = String::from("hello");
let consume = || println!("{}", x);  // x의 소유권 캡처
consume();  // OK
// consume();  // 에러! 한 번만 호출 가능
```

#### map 예제

```rust
let numbers = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = numbers
    .iter()
    .map(|x| x * 2)  // 클로저를 인자로
    .collect();

println!("{:?}", doubled);  // [2, 4, 6, 8, 10]
```

### 전체적인 맥락

함수 포인터와 클로저는:
- **함수형 프로그래밍** 스타일 지원
- 콜백, 이벤트 핸들링 가능
- 고차 함수를 통한 추상화
- 컬렉션 처리 (map, filter, fold 등)

---

## 슬라이드 18-19: 구조체와 열거형

### 개념 설명

**사용자 정의 타입**을 만드는 방법입니다.

#### 구조체 (Struct)

여러 필드를 그룹화한 타입.

#### 열거형 (Enum)

여러 변형(variant) 중 하나의 값을 가지는 타입.

### 배경 지식

- C: struct, union (열거형 없음)
- Java: class (정렬 집합), enum
- Rust: struct, enum (둘 다 강력)

Rust의 enum은 각 variant가 **서로 다른 데이터**를 가질 수 있습니다.

### 수식/기호/코드 설명

#### 구조체

```rust
// 구조체 정의
struct Point {
    x: f64,
    y: f64,
}

// 인스턴스 생성
let p = Point { x: 3.0, y: 4.0 };

// 필드 접근
println!("x: {}, y: {}", p.x, p.y);

// 구조체 업데이트
let p2 = Point { x: 2.0, ..p };  // y는 p에서 복사

// 메서드 정의
impl Point {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

println!("{}", p.distance_from_origin());
```

#### 튜플 구조체

```rust
struct Color(u8, u8, u8);  // RGB 색상

let c = Color(255, 0, 0);
println!("{}", c.0);  // 255 (첫 필드)
```

#### 열거형

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

let dir = Direction::Up;
```

#### 데이터를 가진 열거형

```rust
enum Message {
    Quit,                       // 데이터 없음
    Move { x: i32, y: i32 },   // 구조체 형태
    Write(String),              // 튜플 형태
    ChangeColor(i32, i32, i32), // 여러 값
}

let msg1 = Message::Quit;
let msg2 = Message::Move { x: 10, y: 20 };
let msg3 = Message::Write(String::from("hello"));
```

#### 구조체와 열거형 모두에 메서드

```rust
impl Message {
    fn process(&self) {
        match self {
            Message::Quit => println!("Quit!"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(s) => println!("Write: {}", s),
            Message::ChangeColor(r, g, b) => println!("RGB({}, {}, {})", r, g, b),
        }
    }
}

let msg = Message::Move { x: 10, y: 20 };
msg.process();
```

### 전체적인 맥락

구조체와 열거형은:
- **객체 지향의 핵심** (메서드, 데이터 캡슐화)
- **함수형의 강점** (패턴 매칭)
- 타입 안전성: 데이터 구조 명확
- 프로그램 분석: AST 표현에 열거형 광범위 사용

---

## 슬라이드 20: 패턴 매칭

### 개념 설명

**구조를 분해하고 일치하는지 확인**하는 강력한 메커니즘입니다. Rust의 차별화 특징입니다.

### 배경 지식

**함수형 언어**의 핵심 기능:
- Haskell, OCaml: 패턴 매칭 기본
- C/C++: switch 문만 가능 (기본 값에만)
- Rust: 구조, 범위, 가드 등 지원

### 수식/기호/코드 설명

#### match 표현식

```rust
match x {
    0 => println!("zero"),
    1 => println!("one"),
    2 | 3 => println!("two or three"),  // OR 패턴
    4..=6 => println!("four to six"),   // 범위
    _ => println!("something else"),    // catch-all
}

// match는 표현식
let message = match x {
    0 => "zero",
    1 => "one",
    _ => "many",
};
```

#### 구조체 패턴

```rust
struct Point { x: i32, y: i32 }

let p = Point { x: 0, y: 7 };

match p {
    Point { x, y } => println!("({}, {})", x, y),
}

// 일부 필드만
match p {
    Point { x, .. } => println!("x = {}", x),
}
```

#### 열거형 패턴

```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

let color = Color::Rgb(255, 0, 0);

match color {
    Color::Rgb(r, g, b) => println!("RGB({}, {}, {})", r, g, b),
    Color::Hsv(h, s, v) => println!("HSV({}, {}, {})", h, s, v),
}
```

#### 가드 (Guard)

```rust
match x {
    1..=5 if x % 2 == 0 => println!("even number 2-4"),
    1..=5 => println!("odd number 1-5"),
    _ => println!("other"),
}
```

#### 변수 바인딩

```rust
match Point { x: 0, y: y } {
    Point { x, y } if y > 5 => println!("y > 5"),
    Point { x: 0, y } => println!("on y-axis: {}", y),
    _ => println!("other"),
}
```

#### 소유권과 match

```rust
let s = String::from("hello");

match s {
    value => println!("{}", value),  // 소유권 이동
}

// s는 이제 사용 불가

// ref 사용으로 참조 얻기
let s = String::from("hello");

match &s {
    value => println!("{}", value),
}

// s는 여전히 사용 가능
```

### 전체적인 맥락

패턴 매칭은:
- **표현력 높은 코드** 작성 가능
- **버그 방지**: 모든 경우 처리 강제 (exhaustiveness check)
- **구조 분해**: 데이터 추출 간편
- 프로그램 분석: **AST 방문(traversal) 시 필수**

---

## 슬라이드 21: Option 타입

### 개념 설명

**값이 있을 수도, 없을 수도 있는** 상황을 표현합니다.

```rust
enum Option<T> {
    Some(T),  // 값 있음
    None,     // 값 없음
}
```

### 배경 지식

**null 포인터 문제** ("십억 달러의 실수" - Tony Hoare):
- C/C++/Java: null이 모든 참조의 기본값
- null 처리 잊기 쉬움 → NullPointerException
- Rust: Option으로 명시적

### 수식/기호/코드 설명

#### Option 사용

```rust
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

let result = divide(10, 2);

// 패턴 매칭으로 처리
match result {
    Some(val) => println!("Result: {}", val),
    None => println!("Cannot divide by zero"),
}
```

#### HashMap::get은 Option 반환

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("key", 42);

let val = map.get("key");

match val {
    Some(&x) => println!("Value: {}", x),
    None => println!("Key not found"),
}
```

#### 편리한 메서드들

```rust
let opt: Option<i32> = Some(5);

// map: Option 내용 변환
let doubled = opt.map(|x| x * 2);  // Some(10)

// unwrap_or: 기본값 제공
let value = opt.unwrap_or(0);  // 5

// is_some, is_none
if opt.is_some() {
    println!("Has value");
}

// unwrap (위험! None이면 panic)
let val = opt.unwrap();  // 5
```

#### Option 체이닝

```rust
fn get_first_digit(s: &str) -> Option<u32> {
    s.chars()
        .next()  // Option<char>
        .and_then(|c| c.to_digit(10))  // Option<u32>
}

match get_first_digit("42") {
    Some(d) => println!("First digit: {}", d),
    None => println!("No digit found"),
}
```

### 전체적인 맥락

Option은:
- **안전한 null 처리**
- 컴파일러가 모든 경우 처리 강제
- 함수형 메서드들 (map, and_then, etc.)
- 프로그램 분석: 값의 가능성 추적

---

## 슬라이드 22: Result 타입

### 개념 설명

**성공 또는 실패**를 표현합니다. Option보다 더 정보 풍부합니다.

```rust
enum Result<T, E> {
    Ok(T),      // 성공, 값 포함
    Err(E),     // 실패, 에러 정보 포함
}
```

### 배경 지식

**에러 처리 패러다임**:
- C: int 반환값으로 에러 코드 전달 (문제: 무시하기 쉬움)
- C++: 예외 (exception)
- Rust: Result (명시적, 강제)

### 수식/기호/코드 설명

#### Result 사용

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

match divide(10, 2) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}
```

#### 파일 읽기 예제

```rust
use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;  // ?는 에러 전파
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

#### 편리한 메서드들

```rust
let result: Result<i32, String> = Ok(5);

// map: 성공값 변환
let doubled = result.map(|x| x * 2);  // Ok(10)

// map_err: 에러 변환
let result2 = result.map_err(|_| "Custom error");

// unwrap_or: 기본값
let val = result.unwrap_or(0);  // 5

// unwrap (위험!)
let val = result.unwrap();  // 5
```

#### ? 연산자 (Error Propagation)

```rust
fn complex_operation() -> Result<i32, String> {
    let a = some_operation()?;  // Err이면 반환
    let b = another_operation()?;
    Ok(a + b)
}

// 동등한 코드:
fn complex_operation_verbose() -> Result<i32, String> {
    let a = match some_operation() {
        Ok(val) => val,
        Err(e) => return Err(e),
    };
    let b = match another_operation() {
        Ok(val) => val,
        Err(e) => return Err(e),
    };
    Ok(a + b)
}
```

### 전체적인 맥락

Result는:
- **안전한 에러 처리**
- 에러를 무시하기 어렵게 설계
- 에러 정보를 전달 가능
- 함수형 조합 가능 (map, and_then)

---

## 슬라이드 23: 에러 처리

### 개념 설명

Rust에는 **예외(exception)가 없습니다**. 두 가지 에러 처리 방식:

1. **복구 가능한 에러**: Result<T, E>
2. **복구 불가능한 에러**: panic!

### 배경 지식

**예외 vs 에러 처리**:
- Java: 예외는 흐름 제어 (controversial)
- Go: 함수가 (result, error) 쌍 반환
- Rust: 추상화 선택 가능

### 수식/기호/코드 설명

#### panic! (복구 불가능)

```rust
fn main() {
    panic!("This is a fatal error!");
}

// 배열 경계 넘으면 자동 panic
let v = vec![1, 2, 3];
let _ = v[10];  // panic!
```

panic 시 동작:
- 스택 정리 (unwinding)
- 에러 메시지 출력
- 프로그램 종료

#### Result와 ? 연산자

```rust
fn add_values_associated_with_keys(
    map: &HashMap<String, i32>,
    key_a: &str,
    key_b: &str,
) -> Result<i32, String> {
    let a = map.get(key_a)
        .ok_or(format!("Key {} not found", key_a))?;

    let b = map.get(key_b)
        .ok_or(format!("Key {} not found", key_b))?;

    Ok(a + b)
}

// 사용
match add_values_associated_with_keys(&map, "x", "y") {
    Ok(sum) => println!("Sum: {}", sum),
    Err(e) => eprintln!("Error: {}", e),
}
```

**동작**:
- map.get()은 Option 반환
- ok_or()로 Option을 Result로 변환
- ? 연산자: Err이면 즉시 반환, Ok이면 값 추출

#### 사용자 정의 에러 타입

```rust
#[derive(Debug)]
enum CalculationError {
    DivisionByZero,
    InvalidInput(String),
}

fn safe_divide(a: i32, b: i32) -> Result<i32, CalculationError> {
    if b == 0 {
        Err(CalculationError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}
```

#### 에러 처리 전략

```rust
// 1. panic! (프로토타입, 테스트)
let port: u16 = "2049".parse().unwrap();

// 2. 기본값 제공
let port: u16 = "2049".parse().unwrap_or(3000);

// 3. 에러 전파
fn read_username() -> Result<String, std::io::Error> {
    let mut file = File::open("username.txt")?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

// 4. 사용자 정의 처리
match result {
    Ok(val) => process(val),
    Err(e) => {
        log_error(&e);
        use_default_value()
    }
}
```

### 전체적인 맥락

Rust의 에러 처리:
- **명시적**: 에러를 무시할 수 없음
- **유연함**: 각 상황에 맞는 처리 선택 가능
- **성능**: GC 필요 없음
- **타입 안전**: 어떤 에러 가능한지 명확

---

## 슬라이드 24: 제네릭

### 개념 설명

**하나의 코드로 여러 타입 처리**합니다. 타입 매개변수(type parameter)를 사용합니다.

### 배경 지식

**제네릭 프로그래밍**:
- C: 없음 (매크로로 흉내)
- C++: 템플릿 (매우 강력하지만 복잡)
- Java: 제네릭<T> (타입 안전성)
- Rust: 제네릭 (monomorphization로 zero-cost)

### 수식/기호/코드 설명

#### 제네릭 함수

```rust
// 타입 매개변수 T
fn pop_until<T>(v: &mut Vec<T>, n: usize) {
    while v.len() > n {
        v.pop();
    }
}

let mut v = vec![1, 2, 3, 4, 5];
pop_until(&mut v, 2);
println!("{:?}", v);  // [1, 2]

let mut s = vec!["a", "b", "c"];
pop_until(&mut s, 1);
println!("{:?}", s);  // ["a"]
```

#### 제네릭 구조체

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

let p = Point { x: 5, y: 10 };
let p2 = Point { x: 5.0, y: 10.0 };
```

#### 여러 타입 매개변수

```rust
struct Pair<T, U> {
    first: T,
    second: U,
}

let pair: Pair<i32, String> = Pair {
    first: 5,
    second: String::from("hello"),
};
```

#### 제네릭 열거형

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### 전체적인 맥락

제네릭은:
- **코드 재사용** (DRY 원칙)
- **타입 안전성** (컴파일 타임 검사)
- **성능**: 컴파일 시 각 타입별로 구체화됨
- 트레이트 바운드와 결합하면 매우 강력

---

## 슬라이드 25: 트레이트

### 개념 설명

**공유 동작(shared behavior)을 정의**합니다. 인터페이스와 유사하지만 더 강력합니다.

### 배경 지식

**다형성(Polymorphism)**:
- C: 없음
- C++: 가상 함수 (vtable)
- Java: 인터페이스/추상 클래스
- Rust: 트레이트

Rust는 구조적 다형성과 임시 다형성 모두 지원합니다.

### 수식/기호/코드 설명

#### 트레이트 정의 및 구현

```rust
trait HasArea {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

let circle = Circle { radius: 5.0 };
println!("Area: {}", circle.area());
```

#### 기본 메서드 구현

```rust
trait Animal {
    fn make_sound(&self);

    fn sleep(&self) {  // 기본 구현
        println!("Zzz...");
    }
}

struct Dog;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
    // sleep()은 기본 구현 사용
}

let dog = Dog;
dog.make_sound();  // "Woof!"
dog.sleep();       // "Zzz..."
```

#### 트레이트 바운드 (Trait Bounds)

```rust
// 함수에서 트레이트 바운드 사용
fn print_area<T: HasArea>(shape: T) {
    println!("Area: {}", shape.area());
}

print_area(Circle { radius: 5.0 });

// 여러 트레이트 바운드
fn draw<T: HasArea + Display>(shape: T) {
    println!("{}: area = {}", shape, shape.area());
}

// where 절 사용 (복잡한 경우)
fn compare<T, U>(a: T, b: U) -> i32
where
    T: PartialOrd + Display,
    U: PartialOrd + Display,
{
    if a < b { -1 } else { 1 }
}
```

#### 더 큰 예제

```rust
trait Drawable {
    fn draw(&self);
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing rectangle {}x{}", self.width, self.height);
    }
}

fn render_shapes<T: Drawable>(shapes: Vec<T>) {
    for shape in shapes {
        shape.draw();
    }
}

// 사용
let rect1 = Rectangle { width: 10.0, height: 20.0 };
let rect2 = Rectangle { width: 5.0, height: 15.0 };
render_shapes(vec![rect1, rect2]);
```

#### 트레이트 객체 (동적 디스패치)

```rust
// 다양한 타입을 하나의 컬렉션에
let shapes: Vec<Box<dyn Drawable>> = vec![
    Box::new(Rectangle { width: 10.0, height: 20.0 }),
    Box::new(Circle { radius: 5.0 }),
];

for shape in shapes {
    shape.draw();  // 동적 디스패치 (런타임 오버헤드)
}
```

### 전체적인 맥락

트레이트는:
- **추상화**: 공통 인터페이스 정의
- **확장성**: 기존 타입에 기능 추가 (orphan rule 준수 시)
- **코드 재사용**: 제네릭과 함께
- 프로그램 분석: Visitor 패턴에 광범위 사용

---

## 슬라이드 26: 라이프타임

### 개념 설명

**참조의 유효 범위를 명시**합니다. 참조가 유효한 기간을 컴파일러가 추적합니다.

이것이 **Rust의 가장 어려운 개념**입니다.

### 배경 지식

**dangling reference 문제**:
```c
// C 예제 (위험!)
int* get_ptr() {
    int x = 5;
    return &x;  // 스택의 주소 반환 (undefined behavior)
}
```

Rust는 컴파일 타임에 이를 방지합니다.

### 수식/기호/코드 설명

#### 라이프타임 기본

```rust
// 'a는 라이프타임 매개변수
fn get_string_length<'a>(s: &'a str) -> usize {
    s.len()
}

// 라이프타임 엘리전: 위와 동등
fn get_string_length(s: &str) -> usize {
    s.len()
}
```

**표기법**:
- `&'a T`: 라이프타임 'a로 유효한 T의 참조
- 'a는 임의의 이름 (관례: 'a, 'b, ...)

#### 함수 라이프타임

```rust
// 어느 입력 참조가 반환값?
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

let s1 = String::from("hello");
let s2 = String::from("hi");
let result = longest(&s1, &s2);  // 컴파일 에러!
// result는 &'a str이고, 'a는 s1의 라이프타임
// 하지만 루프 끝에서 s2가 먼저 drop됨
```

더 정확한 예제:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

let s1 = String::from("hello");
{
    let s2 = String::from("hi");
    let result = longest(&s1, &s2);  // s1, s2 모두 유효
    println!("{}", result);
}
// s2가 drop됨
// println!("{}", result);  // 컴파일 에러! s2가 없어짐
```

#### 구조체 라이프타임

```rust
struct Point<'a> {
    x: &'a f64,
    y: &'a f64,
}

let x = 5.0;
let y = 10.0;

let p = Point { x: &x, y: &y };
println!("{}", p.x);  // 5.0

// x, y 간의 라이프타임이 맞아야 함
```

#### 라이프타임 엘리전 규칙

컴파일러가 자동으로 라이프타임을 추론하는 경우:

```rust
// 규칙 1: 한 개 입력 참조 → 출력 라이프타임 같음
fn first(x: &str) -> &str { x }  // 엘리전됨

// 규칙 2: &self → 출력 라이프타임은 self
impl MyType {
    fn get_name(&self) -> &str { &self.name }  // 엘리전됨
}
```

### 전체적인 맥락

라이프타임은:
- **메모리 안전성의 핵심**
- dangling reference 방지
- 컴파일 타임 검사
- 처음엔 어렵지만, 익숙해지면 명확함

---

## 슬라이드 27: 학습 자료

### 개념 설명

Rust를 심화 학습하기 위한 공식 자료들입니다.

### 학습 자료

1. **The Rust Programming Language** (흔히 "The Book")
   - https://doc.rust-lang.org/book/
   - 공식 입문서

2. **Rust By Example**
   - https://doc.rust-lang.org/rust-by-example/
   - 예제 중심 학습

3. **Rust Standard Library Docs**
   - https://doc.rust-lang.org/std/
   - API 레퍼런스

4. **Rust Reference**
   - https://doc.rust-lang.org/reference/
   - 언어 명세

5. **Rust Playground**
   - https://play.rust-lang.org/
   - 웹 기반 Rust 컴파일러

### 배경 지식

공식 문서의 품질이 매우 높아, 학습 곡선이 가파르지만 자료는 충분합니다.

---

## 슬라이드 28: Rust 컴파일러(rustc)

### 개념 설명

**rustc** 컴파일러의 처리 파이프라인입니다. 프로그램 분석의 핵심입니다.

### 전체 파이프라인

```
Source Code
    ↓
Parsing → AST (추상 구문 트리)
    ↓
Desugaring & Symbol Resolution
    ↓
HIR (고수준 중간 표현)
    ↓
Type Checking
    ↓
THIR (타입 지정 고수준 IR)
    ↓
Lowering
    ↓
MIR (중간 수준 표현)
    ↓
Borrow Checking & Optimization
    ↓
MIR (최적화 후)
    ↓
Code Generation
    ↓
Machine Code
```

### 배경 지식

**컴파일러 구조**:
- **프런트엔드**: 소스 → 중간 표현 (parsing, type checking)
- **중간**: 최적화 (MIR)
- **백엔드**: 중간 → 기계어 (code generation)

Rust 컴파일러는 **각 단계를 API로 노출**하여 정적 분석 도구 개발 가능합니다.

### 수식/기호/코드 설명

#### 각 단계의 역할

1. **Parsing**: 텍스트를 트리 구조로
2. **Desugaring**: 편의 문법 제거 (for → while 등)
3. **Symbol Resolution**: 식별자가 뭔지 결정
4. **Type Checking**: 타입 오류 검사
5. **MIR**: 제어 흐름 명시화
6. **Borrow Checking**: 참조 안전성 검사
7. **Optimization**: 불필요한 코드 제거 등
8. **Code Generation**: LLVM IR 생성

### 전체적인 맥락

이 파이프라인을 이해하는 것이 **이 강좌의 목표**입니다:
- 각 단계에서 어떤 분석 가능한가
- 어떤 정보를 추출할 수 있는가
- 프로그램 분석 도구 구현

---

## 슬라이드 29: 추상 구문 트리(AST)

### 개념 설명

**소스 코드를 트리 구조로 표현**합니다. 텍스트가 아닌 구조를 분석합니다.

### 배경 지식

**파싱(Parsing)**:
- 정규 표현식: 패턴 매칭만 가능
- 파서: 문법(grammar) 기반 구조 추출
- AST: 파싱 결과를 트리로 표현

### 수식/기호/코드 설명

#### AST 예제

수식 `(1+2)*3`을 AST로 표현:

```
       Mul
      /   \
    Add   Lit(3)
   /  \
Lit(1) Lit(2)
```

노드 종류:
- `Lit(n)`: 리터럴 값
- `Add`: 덧셈
- `Mul`: 곱셈

#### 복잡한 코드 예제

```rust
let x = 42;
```

AST:
```
Let
├─ name: "x"
├─ init: Some(Lit(42))
└─ ...
```

#### 매크로 전개

```rust
println!("hello");
```

AST에서는 매크로가 전개됩니다:
```
Call
├─ function: std::println::...
└─ args: ["hello"]
```

#### 서브모듈 로딩

```rust
mod utils;
```

AST에서는 utils.rs 파일이 로드되어 트리에 포함됩니다.

#### AST 보기

```bash
rustc -Z unpretty=ast-tree,expanded [file.rs]
```

이 명령으로 컴파일러가 파싱한 AST를 볼 수 있습니다.

### 전체적인 맥락

AST는:
- **첫 번째 구조 표현**
- 텍스트 기반이 아닌 구조 분석 가능
- 매크로, 서브모듈 처리됨
- 하지만 여전히 **심볼 정보 불충분** (다음 슬라이드에서 해결)

---

## 슬라이드 30: 고수준 중간 표현(HIR)

### 개념 설명

**AST를 desugar하고 심볼을 해결**합니다. 더 정규화된 형태입니다.

### 배경 지식

**Desugaring**: 문법 설탕을 풀어서 쓰기
- `for x in v { ... }` → `let mut iter = v.into_iter(); loop { ... }`
- `a += b` → `a = a + b`

**심볼 해결**: 이름이 뭐를 가리키는지 결정
- `let x = 1; { let x = 2; ... }` → 어느 x?

### 수식/기호/코드 설명

#### Desugaring 예제

```rust
for i in 0..3 {
    println!("{}", i);
}
```

HIR에서는:
```rust
{
    let iter = (0..3).into_iter();
    loop {
        match iter.next() {
            Some(i) => {
                println!("{}", i);
            }
            None => break,
        }
    }
}
```

#### 심볼 해결

```rust
let x = 1;
{
    let x = 2;
    println!("{x}");  // 어느 x?
}
```

HIR에서는 각 x에 고유한 ID가 할당됩니다:
- `let x = 1;` → x₁
- `let x = 2;` → x₂
- `println!("{x}");` → x₂ (명확함)

#### HIR 보기

```bash
rustc -Z unpretty=hir [file.rs]
rustc -Z unpretty=hir-tree [file.rs]
```

### 전체적인 맥락

HIR은:
- **정규화된 표현**: 여러 문법 형태가 하나로
- **심볼 정보**: 이름 충돌 해결
- **부모 추적**: 각 노드가 누구의 자식인지
- 타입 정보는 아직 없음 (다음 THIR에서)

---

## 슬라이드 31: 타입 지정 고수준 IR(THIR)

### 개념 설명

**오버로딩을 해결하고 암시 변환을 명시화**합니다. 거의 모든 타입 정보를 포함합니다.

### 배경 지식

**오버로딩**: 같은 이름이 타입에 따라 다른 함수
- `+` 연산자: i32, f64, String 등에서 다름

**암시 변환(coercion)**: 컴파일러가 자동으로 타입 변환
- `&mut x` → `&x` (mutable → immutable)
- `[T; n]` → `&[T]` (배열 → 슬라이스)

### 수식/기호/코드 설명

#### 오버로딩 해결 예제

```rust
fn foo(x: &mut i32) -> &i32 {
    x
}
```

HIR에서:
```rust
fn foo(x: &mut i32) -> &i32 {
    x  // 아직 overburdened
}
```

THIR에서:
```rust
fn foo(x: &mut i32) -> &i32 {
    &*x  // 명시적 deref + 암시 변환
}
```

설명:
- `*x`: mutable reference 역참조
- `&`: immutable reference로 변환

#### 더 복잡한 예제

```rust
let x: i32 = "hello".parse();
```

THIR에서는:
```rust
let x: i32 = <str as std::str::FromStr>::from_str("hello")
    .expect("parse error");
```

#### THIR 보기

```bash
rustc -Z unpretty=thir-tree [file.rs]
```

### 전체적인 맥락

THIR은:
- **타입 정보 추가**: 각 표현식의 타입
- **오버로딩 해결**: 어느 메서드인지 결정
- **암시 변환 명시화**: 컴파일러가 뭘 하는지 명확
- 대부분의 타입 기반 분석 가능

---

## 슬라이드 32: 중간 수준 표현(MIR)

### 개념 설명

**제어 흐름을 명시적으로 표현**합니다. 정적 분석에 가장 적합한 형태입니다.

### 배경 지식

**제어 흐름 그래프(CFG, Control Flow Graph)**:
- 노드: 기본 블록(basic block)
- 엣지: 제어 흐름
- 복잡한 제어 구조를 단순한 그래프로

### 수식/기호/코드 설명

#### MIR 구조

```
Body
├─ fn_arg_types
├─ return_type
├─ local_decls  // 로컬 변수 선언
└─ basic_blocks  // 기본 블록들
    ├─ bb0
    ├─ bb1
    └─ ...
```

#### 기본 블록(Basic Block)

```
BasicBlockData {
    statements: [  // 순차 실행
        statement1,
        statement2,
    ],
    terminator: terminator,  // 분기 포인트
}
```

**Statement**:
- 대입: `_0 = 5;`
- 부작용: `Noop`

**Terminator**:
- `Goto(bb_target)`: 무조건 점프
- `SwitchInt(...)`: 조건 분기
- `Call(...)`: 함수 호출
- `Return`: 반환
- `Panic`: 패닉

#### 간단한 예제

```rust
if x > 5 {
    y = 10;
} else {
    y = 20;
}
```

MIR (개념적):
```
bb0:
    cond = x > 5
    SwitchInt(cond, [true→bb1, false→bb2])

bb1:
    _y = 10
    Goto(bb3)

bb2:
    _y = 20
    Goto(bb3)

bb3:
    Return
```

#### 변수 표기

```
_0       // 반환값
_1, _2   // 함수 매개변수
_3, _4   // 로컬 변수
```

### 전체적인 맥락

MIR은:
- **명시적 제어 흐름**: 복잡한 제어 구조 단순화
- **적은 표현력**: 분석이 쉬움
- **상세한 정보**: 메모리 접근, 함수 호출 등 명시
- **이상적인 분석 대상**: 대부분의 정적 분석이 MIR에서 수행

---

## 슬라이드 33: MIR 예제

### 개념 설명

실제 MIR 예제를 통해 이해를 돕습니다.

### 수식/기호/코드 설명

#### 전체 예제

```rust
fn example() -> i32 {
    let mut x = 0;
    let mut y = 0;
    while x < 10 {
        x += 1;
        y += x;
    }
    y
}
```

#### MIR 표현

```
fn example() -> i32 {
    let _0: i32;                     // 반환값
    let mut _1: i32;                 // x
    let mut _2: i32;                 // y

    bb0:
        _1 = 0                       // x = 0
        _2 = 0                       // y = 0
        goto bb1                     // 루프 시작

    bb1:
        _3 = _1 < 10                 // x < 10 계산
        switchint(_3) → [false: bb3, true: bb2]

    bb2:
        _1 = _1 + 1                  // x += 1
        _2 = _2 + _1                 // y += x
        goto bb1                     // 루프 계속

    bb3:
        _0 = _2                      // return y
        return
}
```

#### CFG 다이어그램

```
┌──────────┐
│   bb0    │  x = 0; y = 0;
└────┬─────┘
     │
     v
┌──────────┐
│   bb1    │◄──────┐
│ x < 10?  │       │
└─┬──────┬─┘       │
  │      └─────────┘ (true)
  │ (false)
  v
┌──────────┐
│   bb3    │
│ return y │
└──────────┘

  bb2 (true일 때)
  x += 1; y += x; → bb1로
```

#### MIR 보기

```bash
rustc -Z unpretty=mir [file.rs]
rustc -C overflow-checks=off -Z unpretty=mir [file.rs]
```

### 전체적인 맥락

MIR 예제를 통해:
- 루프가 조건부 점프로 표현됨
- 각 변수가 명시적으로 추적됨
- 복잡한 제어 흐름이 간단한 CFG로
- 정적 분석이 가능해짐 (loop detection, data flow, etc.)

---

## 슬라이드 34: 코드 표현 비교

### 개념 설명

AST, HIR, THIR, MIR의 차이를 비교합니다. 각자의 용도가 다릅니다.

### 배경 지식

**표현 선택의 트레이드오프**:
- 높은 수준: 원본 코드 유지, 분석 어려움
- 낮은 수준: 분석 쉬움, 원본 의도 불명확

### 수식/기호/코드 설명

#### 비교 테이블

| 특성 | AST | HIR | THIR | MIR |
|------|-----|-----|------|-----|
| 심볼 해결 | 아니오 | 예 | 예 | 예 |
| 타입 정보 | 아니오 | 일부 | 완전 | 완전 |
| 오버로딩 해결 | 아니오 | 아니오 | 예 | 예 |
| 제어 흐름 명시 | 아니오 | 아니오 | 아니오 | 예 |
| 부모 추적 가능 | 예 | 예 | 예 | 아니오 |

#### 용도별 추천

```
┌─────────────────────────────────────┐
│     AST                             │
│  - 구문 강조                         │
│  - 매크로 처리 분석                  │
│  - 문법 검증                         │
└────────┬────────────────────────────┘
         │
┌────────v────────────────────────────┐
│     HIR                             │
│  - 심볼 기반 분석                    │
│  - 스코프 분석                       │
│  - 부모-자식 관계 분석              │
└────────┬────────────────────────────┘
         │
┌────────v────────────────────────────┐
│     THIR                            │
│  - 타입 기반 분석                    │
│  - 타입 검사 정보                    │
└────────┬────────────────────────────┘
         │
┌────────v────────────────────────────┐
│     MIR                             │
│  - 데이터 흐름 분석                  │
│  - 제어 흐름 분석                    │
│  - Borrow checking                  │
│  - 루프 분석                         │
└─────────────────────────────────────┘
```

#### 구체적 비교

```rust
let x = (1 + 2) * 3;
```

**AST**:
```
Let { name: x, init: Mul(Add(1, 2), 3) }
```

**HIR**:
```
Let { id: x₁, init: Mul(Add(1, 2), 3) }
```

**THIR**:
```
Let { id: x₁, init: Mul(Add(1, 2), 3), ty: i32 }
```

**MIR**:
```
bb0:
  _1 = 1 + 2        // _1 = 3
  _2 = _1 * 3       // _2 = 9
  _0 = _2
  return
```

### 전체적인 맥락

이 강좌에서:
- **대부분 MIR 사용**: 명시적, 충분한 정보
- **타입 분석: HIR/THIR**: 타입 정보 필요

---

## 슬라이드 35: HIR의 중요한 타입

### 개념 설명

Rust 컴파일러 API에서 HIR을 다룰 때 사용하는 주요 데이터 구조들입니다.

### 배경 지식

**컴파일러 API**:
- Rust 컴파일러는 라이브러리로 사용 가능
- 내부 데이터 구조를 활용하는 정적 분석 도구 개발 가능
- rustc_driver, rustc_hir 등의 crate

### 수식/기호/코드 설명

#### LocalDefId

```rust
// 로컬 정의에 대한 고유 식별자
// 하나의 크레이트 내에서 유일

// 예: 함수, 상수, 타입 등 각 정의마다 하나
fn my_function() { }    // LocalDefId(123)
const CONST: i32 = 42;  // LocalDefId(124)
struct Point { x: i32 } // LocalDefId(125)
```

#### DefId

```rust
// 전역 정의 식별자
// 여러 크레이트에 걸쳐 유일

// DefId = (CrateNum, LocalDefId)
// 예: (std_crate, LocalDefId(10))
```

#### HirId

```rust
// HIR 노드의 고유 식별자
// 각 표현식, 문, 패턴 등이 HirId를 가짐

// 함수 내 모든 노드:
fn foo() {
    let x = 5;      // HirId(1)
    let y = x + 1;  // HirId(2), HirId(3), HirId(4)
    println!("{}", y);
}
```

#### Item, Stmt, Expr

```rust
// Item: 최상위 정의
enum Item {
    Function(...)   // fn
    Struct(...)     // struct
    Enum(...)       // enum
    Module(...)     // mod
    // ...
}

// Stmt: 문
enum Stmt {
    Local { ... }   // let x = ...;
    Item { ... }    // 내부 item
    Expr { ... }    // expression;
}

// Expr: 표현식
enum Expr {
    Literal(...)    // 5
    Path(...)       // x, foo::bar
    BinOp(...)      // x + y
    If(...)         // if condition { ... }
    // ...
}
```

#### Visitor 패턴

```rust
// HIR을 순회하기 위한 트레이트
trait Visitor<'v> {
    fn visit_item(&mut self, item: &'v Item) { ... }
    fn visit_fn(&mut self, fn_decl: &'v FnDecl, ...) { ... }
    fn visit_expr(&mut self, expr: &'v Expr) { ... }
    fn visit_stmt(&mut self, stmt: &'v Stmt) { ... }
    // ...
}

// 사용 예
struct MyAnalyzer;

impl Visitor<'_> for MyAnalyzer {
    fn visit_item(&mut self, item: &Item) {
        println!("Found item: {:?}", item);
        // 계속 순회
        walk_item(self, item);
    }
}

// 모든 아이템 방문
let mut analyzer = MyAnalyzer;
analyzer.visit_crate(hir_crate);
```

### 전체적인 맥락

HIR API는:
- **AST 분석**: 구문 기반 분석
- **심볼 추적**: 선언과 사용 연결
- **스코프 분석**: 변수의 가시성
- **심각한 타입 정보 제외**: 타입은 THIR 필요

---

## 슬라이드 36: MIR의 중요한 타입

### 개념 설명

MIR 레벨에서 프로그램 분석할 때 사용하는 주요 데이터 구조들입니다.

### 배경 지식

**MIR 분석의 목표**:
- 데이터 흐름 (어떤 값이 어디로 흘러가는가)
- 제어 흐름 (어떤 경로를 따르는가)
- 메모리 접근 (어떤 메모리에 접근하는가)

### 수식/기호/코드 설명

#### Body

```rust
// 함수의 MIR 표현

pub struct Body<'tcx> {
    pub basic_blocks: IndexVec<BasicBlock, BasicBlockData<'tcx>>,
    pub source_scopes: Vec<SourceScopeData<'tcx>>,
    pub local_decls: LocalDecls<'tcx>,
    pub arg_count: usize,
    pub var_debug_info: Vec<VarDebugInfo<'tcx>>,
    pub span: Span,
    pub generator: Option<GeneratorInfo>,
    pub return_ty: Ty<'tcx>,
    // ...
}
```

#### BasicBlock과 BasicBlockData

```rust
// BasicBlock: 기본 블록의 인덱스
pub struct BasicBlock {
    pub index: u32,
}

// BasicBlockData: 기본 블록의 내용
pub struct BasicBlockData<'tcx> {
    pub statements: Vec<Statement<'tcx>>,
    pub terminator: Option<Terminator<'tcx>>,
    pub is_cleanup: bool,
}
```

#### Statement와 Terminator

```rust
// Statement: 순차 실행 명령
pub enum StatementKind<'tcx> {
    Assign(Box<(Place<'tcx>, Rvalue<'tcx>)>),  // LHS = RHS
    FakeRead(...),
    SetDiscriminant { ... },  // enum 변형 설정
    Deinit(Box<Place<'tcx>>),
    Noop,
    // ...
}

// Terminator: 블록의 마지막 명령
pub enum TerminatorKind<'tcx> {
    Goto { target: BasicBlock },
    SwitchInt { discr: Operand<'tcx>, ... },
    Resume,
    Abort,
    Return,
    Unreachable,
    Drop { place: Place<'tcx>, ... },
    Call { func: Operand<'tcx>, args: Vec<Operand<'tcx>>, ... },
    Assert { ... },
    // ...
}
```

#### Place (메모리 위치)

```rust
// Place: 메모리의 위치 (좌측값)

pub struct Place<'tcx> {
    pub local: Local,
    pub projection: List<PlaceElem<'tcx>>,
}

pub enum PlaceElem<'tcx> {
    Deref,              // *p
    Field(u32, Ty),     // p.field
    Index(Local),       // p[index]
    ConstantIndex { .. },
    Subslice { .. },
    Downcast(VariantIdx),
}

// 예제
// _0 = 로컬 변수 0
// _1.f = 로컬 변수 1의 필드 f
// (*_2) = 로컬 변수 2 역참조
// _3[_4] = 로컬 변수 3의 인덱스 _4
```

#### Rvalue (우측값)

```rust
// Rvalue: 값을 생성하는 표현식 (우측값)

pub enum Rvalue<'tcx> {
    Use(Operand<'tcx>),                    // operand 사용
    Repeat(Operand<'tcx>, u64),            // [val; n]
    Ref(Region, BorrowKind, Place<'tcx>),  // &place
    AddressOf(Mutability, Place<'tcx>),    // &raw mut/const
    Len(Place<'tcx>),                      // len(place)
    Cast(CastKind, Operand<'tcx>, Ty<'tcx>), // (val as ty)
    BinOp(BinOp, Operand<'tcx>, Operand<'tcx>), // a op b
    UnaryOp(UnOp, Operand<'tcx>),          // !a, -a
    Discriminant(Place<'tcx>),             // enum 변형 추출
    Aggregate(AggregateKind, Vec<Operand<'tcx>>), // 구조 생성
    ShallowInitBox(Box<Operand<'tcx>>, Ty), // Box 초기화
    // ...
}
```

#### Operand (피연산자)

```rust
// Operand: 값의 원본

pub enum Operand<'tcx> {
    Copy(Place<'tcx>),      // 복사 (Copy 타입)
    Move(Place<'tcx>),      // 이동 (Non-Copy)
    Const(Box<Const<'tcx>>),  // 상수
}
```

#### Const (상수)

```rust
// Const: 컴파일 타임 상수

pub struct Const<'tcx> {
    pub ty: Ty<'tcx>,
    pub kind: ConstKind<'tcx>,
}

pub enum ConstKind<'tcx> {
    Param(ParamConst),
    Infer(InferConst),
    Bound(DebruijnIndex, BoundConst),
    Unevaluated(Unevaluated<'tcx, ()>),
    Value(ValTree),
    Error(...),
}
```

### 전체적인 맥락

MIR 타입들은:
- **명시적 메모리 모델**: 어떤 위치, 어떤 값
- **데이터 흐름 추적**: Place와 Rvalue로 명확
- **제어 흐름 추적**: Terminator로 분기 명확
- 대부분의 정적 분석 가능

---

## 슬라이드 37: MIR의 중요한 타입 계속

### 개념 설명

MIR의 추가 중요 타입들: Local, Place 구성, Visitor 패턴.

### 수식/기호/코드 설명

#### Local

```rust
// Local: 로컬 변수의 식별자

pub struct Local {
    pub index: u32,
}

// 관례
// _0 = 반환값
// _1, _2, ... = 함수 매개변수
// _n, ... = 로컬 변수

// 예제
fn foo(a: i32, b: String) {  // a = _1, b = _2
    let x = 5;               // x = _3
    let y = x + a;           // y = _4
}
```

#### Place = Local + Projections

```rust
// Place: 메모리 위치 = 기저 변수 + 경로

pub struct Place<'tcx> {
    pub local: Local,
    pub projection: &'tcx List<PlaceElem<'tcx>>,
}

// 예제
_0                          // 로컬 0 (반환값)
_1                          // 로컬 1 (매개변수)
_2.field0                   // 로컬 2의 field0
(*_3)                       // 로컬 3 역참조
_4[_5]                      // 로컬 4의 인덱스 _5
_6.field0.field1            // 중첩 필드 접근

// Projection 체인
_2 (base local)
  ↓
_2.field0 (field projection)
  ↓
_2.field0.field1 (another field)
```

#### 메모리 접근 추적

```rust
// 데이터 흐름: Place와 Rvalue

struct Point { x: i32, y: i32 }

let mut p = Point { x: 5, y: 10 };  // _1 = Point { ... }
let x = p.x;                        // _2 = _1.x (읽음)
p.x = 20;                           // _1.x = 20 (쓰기)
```

MIR:
```
bb0:
  _1 = Point { x: 5, y: 10 }
  _2 = (_1.0: i32)          // Place: _1.field(0)
  (_1.0: i32) = 20          // Assignment to _1.field(0)
  return
```

#### Visitor 패턴 (MIR)

```rust
// MIR 순회

trait Visitor<'tcx> {
    fn visit_basic_block_data(&mut self,
        block: BasicBlock,
        data: &BasicBlockData<'tcx>) { ... }

    fn visit_statement(&mut self,
        statement: &Statement<'tcx>,
        location: Location) { ... }

    fn visit_terminator(&mut self,
        terminator: &Terminator<'tcx>,
        location: Location) { ... }

    fn visit_place(&mut self,
        place: &Place<'tcx>,
        context: PlaceContext,
        location: Location) { ... }

    fn visit_rvalue(&mut self,
        rvalue: &Rvalue<'tcx>,
        location: Location) { ... }

    fn visit_operand(&mut self,
        operand: &Operand<'tcx>,
        location: Location) { ... }
}

// 구현 예
struct LoopFinder {
    loop_headers: HashSet<BasicBlock>,
}

impl<'tcx> Visitor<'tcx> for LoopFinder {
    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, ...) {
        if let TerminatorKind::Goto { target } = &terminator.kind {
            // 뒤로 가는 엣지 감지 → 루프
            if target < current_block {
                self.loop_headers.insert(*target);
            }
        }
    }
}
```

### 전체적인 맥락

MIR 타입들로:
- **메모리 접근 추적**: 변수의 읽고 쓰기
- **제어 흐름 그래프**: 실행 경로 분석
- **데이터 흐름 분석**: 값의 흐름
- 루프 감지, 도달 가능성 분석, 정의-사용 체인 등 가능

---

## 슬라이드 38: TyCtxt

### 개념 설명

**컴파일러의 중앙 데이터 구조**입니다. 모든 타입, 정의, 분석 정보를 보관합니다.

### 배경 지식

**타입 컨텍스트(Type Context)**:
- 컴파일 중인 프로그램의 모든 정보
- 쿼리 기반 설계 (lazy evaluation)
- 메모리 효율적

### 수식/기호/코드 설명

#### TyCtxt 개요

```rust
// TyCtxt<'tcx>: 타입 컨텍스트
// 'tcx: 타입 컨텍스트 라이프타임

pub struct TyCtxt<'tcx> {
    // 내부 구현 (공개되지 않음)
}

// TyCtxt는 불변 참조로 사용
// fn analysis<'tcx>(tcx: TyCtxt<'tcx>) { ... }
```

#### 주요 쿼리 메서드들

```rust
impl<'tcx> TyCtxt<'tcx> {
    // HIR 접근
    pub fn hir_visit_all_item_likes_in_crate(self, visitor: &mut dyn ...) {
        // 모든 아이템 방문
    }

    // 부모-자식 관계
    pub fn parent_hir_id(self, id: HirId) -> HirId {
        // 주어진 노드의 부모 반환
    }

    // MIR 접근
    pub fn optimized_mir(self, def_id: DefId) -> &'tcx Body<'tcx> {
        // 최적화된 MIR 반환
    }

    // 타입 정보
    pub fn type_of(self, def_id: DefId) -> Ty<'tcx> {
        // 정의의 타입 반환
    }

    // 제네릭 정보
    pub fn generics_of(self, def_id: DefId) -> &'tcx Generics {
        // 제네릭 매개변수 정보
    }
}
```

#### 사용 예제

```rust
fn analyze_crate<'tcx>(tcx: TyCtxt<'tcx>) {
    // 모든 함수 방문
    tcx.hir_visit_all_item_likes_in_crate(&mut |id| {
        if let ItemKind::Fn(...) = item.kind {
            let def_id = tcx.hir().local_def_id(id);

            // MIR 가져오기
            let mir = tcx.optimized_mir(def_id.to_def_id());

            // 분석...
            for (bb, data) in mir.basic_blocks.iter_enumerated() {
                // 각 기본 블록 분석
            }
        }
    });
}
```

### 전체적인 맥락

TyCtxt는:
- **중앙 진입점**: 모든 컴파일 정보
- **쿼리 기반**: 필요한 것만 계산 (lazy)
- **메모이제이션**: 반복 계산 방지
- **이 강좌의 핵심**: TyCtxt 쿼리로 프로그램 분석

---

## 슬라이드 39: 참고 자료

### 개념 설명

더 깊이 있는 학습을 위한 추가 자료들입니다.

### 참고 자료

#### 공식 문서

1. **Rust Compiler Development Guide**
   - https://rustc-dev-guide.rust-lang.org/
   - 컴파일러 내부 구조

2. **HIR Documentation**
   - https://rustc-dev-guide.rust-lang.org/hir.html
   - 고수준 중간 표현 상세

3. **MIR Documentation**
   - https://rustc-dev-guide.rust-lang.org/mir/
   - 중간 수준 표현 상세

4. **Rust API Documentation**
   - https://docs.rs/rustc_hir/
   - https://docs.rs/rustc_mir/
   - API 레퍼런스

#### 도움 되는 도구

```bash
# AST 확인
rustc -Z unpretty=ast-tree,expanded input.rs

# HIR 확인
rustc -Z unpretty=hir input.rs
rustc -Z unpretty=hir-tree input.rs

# THIR 확인
rustc -Z unpretty=thir-tree input.rs

# MIR 확인
rustc -Z unpretty=mir input.rs

# 최적화된 MIR
rustc -O -Z unpretty=mir input.rs
```

### 전체적인 맥락

이 강좌 진행:
1. Rust 언어 기본 (슬라이드 1-27)
2. 컴파일러 구조 (슬라이드 28-39)
3. 프로그램 분석 구현 (실습/프로젝트)

---

## 슬라이드 40: 요약

### 개념 설명

이 강좌의 핵심 내용을 정리합니다.

### 요약

#### Rust 언어 특징

**메모리 안전성**: 컴파일 타임에 보장
- 소유권: 각 값은 정확히 하나의 소유자
- 차용: 참조를 통한 임시 접근
- 라이프타임: 참조의 유효 기간

**타입 안전성**: 강력한 타입 시스템
- 변수와 타입: 명시적 선언
- 함수: 입출력 타입 명시
- 제네릭: 타입 재사용
- 트레이트: 공유 행동 정의

**표현력**: 함수형 + 객체지향
- 패턴 매칭: 구조 분해
- 클로저: 함수는 값
- 열거형/구조체: 사용자 정의 타입

#### 컴파일러 구조

**파이프라인**:
```
Source → AST → HIR → THIR → MIR → 기계어
```

**각 단계의 목적**:
- **AST**: 구문 구조
- **HIR**: 심볼 해결
- **THIR**: 타입 정보
- **MIR**: 제어/데이터 흐름

#### 주요 개념/용어

**언어**:
- let, mut, fn, impl, trait, match, Option, Result
- 소유권, 차용, 라이프타임, 제네릭

**컴파일러**:
- HirId, DefId, LocalDefId: 식별자
- Item, Stmt, Expr: 코드 요소
- Body, BasicBlock, Place, Rvalue: MIR 요소
- Statement, Terminator: 블록 내용
- Operand, Const: 값의 원본
- TyCtxt: 중앙 데이터 구조

#### 분석 도구 구현

프로그램 분석 도구는:
1. **TyCtxt를 통해 프로그램 접근**
2. **대부분 MIR에서 분석** (명시적, 충분한 정보)
3. **필요 시 HIR/THIR 참조** (타입, 심볼)
4. **Visitor 패턴으로 순회**

#### 이 강좌의 목표

1. **Rust 언어 이해**: 메모리 안전성, 타입 시스템
2. **컴파일러 파이프라인 이해**: 각 표현의 역할
3. **정적 분석 도구 개발**: rustc API 활용

### 전체적인 맥락

**학습 경로**:
```
Rust 기본 → 소유권/차용 → 컴파일러 구조
    ↓          ↓              ↓
간단한 프로  중급 프로   프로그램 분석
그래밍 작성  그래밍 작성   도구 개발
```

**이 강좌 후**:
- Rust로 안전한 시스템 프로그래밍 가능
- 컴파일러/인터프리터 개발 기초
- 정적 분석, 린팅 도구 개발 가능
- Rust 생태계 기여 가능

---

# 추가 학습 팁

## 실습 방법

### 1. Rust Playground 활용
```bash
# https://play.rust-lang.org/에서 직접 실행
```

### 2. 로컬 개발 환경

```bash
# Rust 설치
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 새 프로젝트
cargo new my_project
cd my_project
cargo run

# 컴파일러 내부 확인
rustc -Z unpretty=mir src/main.rs
```

### 3. 단계별 학습 프로젝트

**Level 1**: 기본 문법
- 변수, 타입, 함수, 루프 작성

**Level 2**: 소유권 이해
- 소유권 이동, 차용, 클론 실습

**Level 3**: 고급 기능
- 트레이트, 제네릭, 라이프타임

**Level 4**: 컴파일러 분석
- HIR/MIR 생성 및 분석

## 자주하는 실수

### 1. 소유권 혼동
```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1);  // 에러! s1의 소유권이 s2로 이동됨
```

**해결**: 참조(`&s1`) 또는 복제(`.clone()`) 사용

### 2. 차용 규칙 위반
```rust
let mut x = 5;
let r1 = &mut x;
let r2 = &mut x;  // 에러! 동시에 여러 가변 참조 불가
```

**해결**: 참조의 사용 범위 명확히 하기

### 3. 라이프타임 문제
```rust
fn bad() -> &String {
    let s = String::from("hello");
    &s  // 에러! 스택의 주소 반환
}
```

**해결**: 값을 반환하거나 매개변수 참조 반환

## 추가 도움말

- **컴파일 에러 읽기**: 매우 친절함, 제안까지 포함
- **rustlings**: 대화형 Rust 연습 (https://github.com/rust-lang/rustlings)
- **커뮤니티**: r/rust, Rust Forum, Discord

---

# 마치며

Rust는 **배우기 어렵지만, 배운 후에는 매우 강력합니다**.

이 강좌를 통해:
- Rust의 혁신적인 메모리 안전성 메커니즘 이해
- 현대 컴파일러 설계 학습
- 정적 분석 도구 개발 능력 습득

모두 화이팅!

