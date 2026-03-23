# CSE552 프로그램 분석 - Lecture 1: Introduction to Program Analysis

## 슬라이드 1: Introduction to Program Analysis

### 원문 내용
> Introduction to Program Analysis
> CSE552 Program Analysis — Lecture 1
> Jaemin Hong

### 해설

**개념 설명**
이 강의는 프로그램 분석(Program Analysis)이라는 주제를 소개하는 첫 번째 강의입니다. 프로그램 분석은 주어진 프로그램의 성질과 동작을 자동으로 파악하는 기법들의 모음입니다.

**배경 지식** (학부 2학년 수준)
- 이 강의는 학부생들이 이미 기본 프로그래밍과 자료구조에 대해 알고 있다고 가정합니다
- 컴파일러의 기본 개념과 프로그램 실행의 기초를 이해하고 있어야 합니다

**전체적인 맥락**
이것이 첫 번째 슬라이드로서 강의의 제목과 강의자 정보를 제시합니다. 강의자는 Jaemin Hong이며, 이 강의는 CSE552 프로그램 분석 과정의 첫 번째 강의입니다.

---

## 슬라이드 2: Instructor Information

### 원문 내용
> **Instructor Information**
>
> - Name: Jaemin Hong
> - Email: jaemin.hong@unist.ac.kr
> - Website: https://hjaem.info/
> - Research Areas: Programming Languages, Static Analysis, Program Transformation
> - Office: Room 710-1, Building 106
> - Office Hours: Wednesday 10:00–11:00 AM (or by appointment)

### 해설

**개념 설명**
강의자의 기본 정보와 연락처 및 오피스 시간을 제시합니다. 학생들이 강의자에게 질문이나 상담이 필요할 때 이 정보를 사용합니다.

**배경 지식**
- 강의자의 연구 분야는 프로그래밍 언어, 정적 분석, 프로그램 변환입니다
- 이는 이 강의에서 다룰 주요 주제들과 직접적으로 관련이 있습니다

**전체적인 맥락**
이 강의는 관계형 정보로서, 학생들이 강의자와 소통하기 위한 방법을 제공합니다.

---

## 슬라이드 3: Course Information

### 원문 내용
> **Course Information**
>
> - Lecture slides will be provided
> - Textbook: "Static Program Analysis" by Anders Møller and Michael I. Schwartzbach
>   - https://cs.au.dk/~amoeller/spa/
> - Reference: "Introduction to Static Analysis: An Abstract Interpretation Perspective" by Xavier Rival and Kwangeun Yi

### 해설

**개념 설명**
강의에서 사용할 주요 학습 자료와 참고 자료를 소개합니다.

**배경 지식**
- 주 교재는 "Static Program Analysis"로, 프로그램 분석의 기초를 다루는 표준 교재입니다
- 참고 자료는 추상 해석(abstract interpretation) 관점에서 정적 분석을 설명합니다
- 이 두 자료는 모두 국제적으로 인정받은 전문 자료입니다

**전체적인 맥락**
학생들이 강의 외 시간에 추가로 학습할 때 참고할 수 있는 자료들입니다.

---

## 슬라이드 4: Grading

### 원문 내용
> **Grading**
>
> - Midterm Exam: 25%
> - Final Exam: 25%
> - Assignments: 20%
>   - 4 programming assignments (5% each)
> - Presentations: 30%
>   - Term project and end-of-term presentation
>   - Define your own software problem and solve it by designing and implementing a static analysis
>   - Details will be provided later

### 해설

**개념 설명**
강의의 평가 방식은 시험(50%), 과제(20%), 발표(30%)로 구성됩니다.

**배경 지식**
- 중간고사와 기말고사가 각각 25%씩 배정되어 있습니다
- 4개의 프로그래밍 과제는 각 5%씩이며, 정적 분석 기법을 직접 구현하는 것입니다
- 학기말 프로젝트는 학생이 자신이 정의한 소프트웨어 문제를 정적 분석으로 해결하는 과정입니다

**전체적인 맥락**
이 강의는 이론 학습(시험)과 실제 구현(과제 및 프로젝트)의 균형을 맞추고 있습니다. 특히 30%의 발표는 학생들이 자신의 작업을 설명하고 방어할 수 있는 능력을 키우는 데 중점을 두고 있습니다.

---

## 슬라이드 5: Motivation 1: Program Transformation

### 원문 내용
> **Motivation 1: Program Transformation**
>
> - Compiler optimization significantly affects software performance
> - GCC -O0 vs -O3: 1.2x–15x speedup¹
> - How can we optimize programs automatically?
>
> ¹Impact of GCC optimization levels in energy consumption during C/C++ program execution (Branco and Henriques, 2015)

### 해설

**개념 설명**
프로그램 분석의 첫 번째 실제 응용 분야는 프로그램 변환(최적화)입니다. 컴파일러 최적화 수준에 따라 프로그램의 속도가 대폭 달라질 수 있습니다.

**배경 지식**
- GCC는 GNU C Compiler로, 널리 사용되는 오픈소스 컴파일러입니다
- -O0 플래그는 최적화 없음, -O3은 가장 높은 수준의 최적화를 의미합니다
- 1.2x~15x의 속도 향상은 최적화의 영향이 얼마나 큰지를 보여줍니다

**전체적인 맥락**
왜 프로그램 분석이 필요한지 보여주는 첫 번째 동기입니다. 프로그램을 더 잘 이해할수록, 더 효과적으로 최적화할 수 있습니다.

**추가 설명**
최적화 예시: 루프 불변식(loop invariant) 코드를 루프 밖으로 이동하면 반복 계산을 피할 수 있고, 이는 프로그램의 동작을 바꾸지 않으면서도 성능을 향상시킵니다.

---

## 슬라이드 6: Example: Loop-Invariant Code Motion

### 원문 내용
> **Example: Loop-Invariant Code Motion**
>
> Original:
> ```
> for (i = 0; i < n; i++) {
>   y = x * x;
>   print(y);
>   *p += i;
> }
> ```
>
> Optimized:
> ```
> y = x * x;
> for (i = 0; i < n; i++) {
>   print(y);
>   *p += i;
> }
> ```
>
> Does x * x evaluate to the same value in every iteration? If so, we can move it outside the loop

### 해설

**개념 설명**
루프 불변식 코드 이동(Loop-Invariant Code Motion, LICM)은 루프 내에서 반복해서 계산되지만 매번 같은 결과를 내는 코드를 루프 밖으로 옮기는 최적화 기법입니다.

**수식/기호/코드 설명**
- 원본 코드: `x * x`는 루프 반복마다 계산되지만, x의 값이 변하지 않으므로 매번 같은 값을 계산합니다
- 최적화된 코드: `y = x * x;`를 루프 전에 한 번만 계산하므로, 루프 반복이 많을수록 더 많은 성능 향상을 얻습니다
- 이 변환이 안전하려면 x가 루프 내에서 변경되지 않아야 합니다

**배경 지식**
이 최적화를 안전하게 수행하려면 프로그램 분석이 필요합니다:
- x가 루프 내에서 수정되는가?
- 포인터 *p를 통한 간접적 수정이 x에 영향을 주는가?

**전체적인 맥락**
이 예시는 프로그램을 분석해야 올바른 최적화를 할 수 있다는 것을 보여줍니다. 잘못된 분석은 의도하지 않은 의미 변경(semantic change)을 초래할 수 있습니다.

---

## 슬라이드 7: Program Transformation: Key Insight

### 원문 내용
> **Program Transformation: Key Insight**
>
> - Semantics-preserving transformation for optimization or refactoring requires understanding program behavior
> - We need to know what the program does
> - This is where program analysis comes in

### 해설

**개념 설명**
의미 보존 변환(semantics-preserving transformation)을 위해서는 프로그램의 동작을 이해해야 합니다. 이것이 프로그램 분석이 필요한 핵심 이유입니다.

**배경 지식**
- 의미 보존이란: 변환 전후의 프로그램이 같은 입력에 대해 같은 결과를 내야 한다는 의미입니다
- 리팩토링(refactoring)은 코드의 구조를 개선하되 동작은 유지하려는 작업입니다
- 최적화는 성능을 향상시키면서 동작을 유지해야 합니다

**전체적인 맥락**
이 슬라이드는 프로그램 분석이 왜 필요한지에 대한 철학적 기초를 제공합니다. 프로그램 변환 시 실수를 피하려면 반드시 프로그램 분석이 필요합니다.

---

## 슬라이드 8: Motivation 2: Bug Finding

### 원문 내용
> **Motivation 2: Bug Finding**
>
> Software bugs can lead to:
> - Significant financial losses
> - Security breaches
> - Loss of life
>
> We need automated techniques to find bugs before they cause damage

### 해설

**개념 설명**
프로그램 분석의 두 번째 주요 응용 분야는 버그 찾기입니다. 소프트웨어 버그는 매우 심각한 결과를 초래할 수 있습니다.

**배경 지식**
- 소프트웨어 버그로 인한 손실은 경제적 비용뿐만 아니라 생명과 직결될 수 있습니다
- 수동 테스트만으로는 모든 버그를 찾기 어렵습니다
- 자동화된 분석 기법이 필요합니다

**전체적인 맥락**
이것이 프로그램 분석의 두 번째 동기로서, 프로그램을 자동으로 분석하면 버그를 미리 발견할 수 있습니다.

---

## 슬라이드 9: Historical Bug Example: Ariane 5

### 원문 내용
> **Historical Bug Example: Ariane 5**
>
> - Ariane 5 rocket failure (1996)
> - Bug type: Integer overflow
> - Impact: Loss of more than $370 million²
>
> ²The Ariane 5 software failure (Dowson, 1997)

### 해설

**개념 설명**
Ariane 5는 유럽 우주국(ESA)의 발사 로켓으로, 1996년에 비행 중 발생한 정수 오버플로우 버그로 인해 폭발했습니다.

**배경 지식**
- 정수 오버플로우(integer overflow)는 계산 결과가 정수 자료형의 범위를 초과할 때 발생합니다
- 이 경우, 64비트 부동소수점 수를 16비트 정수로 변환하려다 발생했습니다
- 약 37억 달러의 손실을 초래했습니다

**전체적인 맥락**
이 사례는 작은 프로그래밍 오류가 얼마나 큰 결과를 초래할 수 있는지를 보여줍니다. 정적 분석이 이런 타입 오류를 사전에 발견할 수 있었을 것입니다.

---

## 슬라이드 10: Historical Bug Example: Therac-25

### 원문 내용
> **Historical Bug Example: Therac-25**
>
> - Therac-25 radiation therapy machine (1985–1987)
> - Bug type: Data race
> - Impact: At least 6 accidents, resulting in death or serious injury³
>
> ³An investigation of the Therac-25 accidents (Leveson and Turner, 1992)

### 해설

**개념 설명**
Therac-25는 방사선 치료기로, 데이터 경합(data race) 버그로 인해 환자들에게 과도한 방사선을 조사했습니다.

**배경 지식**
- 데이터 경합(data race)은 여러 스레드가 동기화 없이 같은 메모리에 접근할 때 발생합니다
- 이 경우, 여러 프로세스가 동시에 같은 자료를 수정하여 예측 불가능한 동작이 발생했습니다
- 최소 6명의 환자가 심각한 피해를 입었고 일부 사망했습니다

**전체적인 맥락**
이 사례는 동시성 버그의 위험성을 보여주며, 정적 분석이 이런 경합 조건(race condition)을 사전에 탐지할 수 있음을 시사합니다.

---

## 슬라이드 11: Historical Bug Example: Heartbleed

### 원문 내용
> **Historical Bug Example: Heartbleed**
>
> - Heartbleed bug (2014)
> - Bug type: Buffer overflow
> - Impact: Estimated cost of $500 million⁴
>
> ⁴Ten most expensive bugs in history (part 2) (Sixsentix, 2024)

### 해설

**개념 설명**
Heartbleed는 OpenSSL 라이브러리의 버퍼 오버플로우 취약점으로, 암호화 키와 민감한 데이터를 노출시켰습니다.

**배경 지식**
- 버퍼 오버플로우(buffer overflow)는 할당된 메모리 영역을 초과하여 데이터를 쓸 때 발생합니다
- 이 버그로 인해 암호화 연결의 개인 키가 유출되었습니다
- 경제적 손실은 약 5억 달러로 추정됩니다

**전체적인 맥락**
보안 관련 버그는 특히 심각한 결과를 초래합니다. 정적 분석은 메모리 접근 오류를 탐지하여 이런 취약점을 예방할 수 있습니다.

---

## 슬라이드 12: Bug Finding Example

### 원문 내용
> **Bug Finding Example**
>
> - Can this divisor expression evaluate to zero? If so, we have a division by zero error
> - This is a question that program analysis can answer

### 해설

**개념 설명**
프로그램 분석의 구체적인 예로, 주어진 식이 0으로 나누기 오류를 초래할 수 있는지를 판단하는 문제입니다.

**배경 지식**
- 0으로 나누기는 런타임 오류를 발생시키며, 프로그램을 중단시킵니다
- 프로그램 분석은 이 오류 조건을 정적으로 검사할 수 있습니다
- 분석기는 분모가 0이 될 수 있는 모든 경로를 추적해야 합니다

**전체적인 맥락**
이 예시는 프로그램 분석이 구체적으로 어떤 문제를 해결할 수 있는지를 보여줍니다.

---

## 슬라이드 13: Motivation 3: Program Comprehension

### 원문 내용
> **Motivation 3: Program Comprehension**
>
> - Modern development process heavily utilizes IDEs or editor plugins
> - Example: "mouse hover" to show the type of an expression

### 해설

**개념 설명**
프로그램 분석의 세 번째 응용 분야는 프로그램 이해입니다. 현대의 IDE들은 프로그램 분석을 사용하여 개발자를 지원합니다.

**배경 지식**
- IDE(Integrated Development Environment)는 코드 편집기와 분석 도구를 통합한 개발 환경입니다
- 마우스 호버 기능은 타입 추론(type inference) 분석을 사용하여 표현식의 타입을 표시합니다
- 자동 완성(auto-completion), 리팩토링 도구 등도 프로그램 분석에 기반합니다

**전체적인 맥락**
학생들이 매일 사용하는 IDE의 많은 기능이 프로그램 분석에 기반하고 있습니다.

---

## 슬라이드 14: Program Analysis

### 원문 내용
> **Program Analysis**
>
> A collection of techniques to automatically figure out the properties of given programs
>
> Essential for:
> - Program transformation
> - Bug finding
> - Program comprehension

### 해설

**개념 설명**
프로그램 분석은 주어진 프로그램의 성질을 자동으로 파악하는 기법들의 모음입니다. 이는 앞에서 언급한 세 가지 주요 응용 분야를 모두 지원합니다.

**배경 지식**
- 프로그램 분석은 추상화(abstraction)를 통해 프로그램의 본질적 특성을 추출합니다
- 이는 프로그램을 실행하지 않고도 그 성질을 파악할 수 있게 합니다

**전체적인 맥락**
이 슬라이드는 프로그램 분석이란 무엇인지에 대한 정의를 제공하며, 앞서 제시한 세 가지 동기를 통합합니다.

---

## 슬라이드 15: Static vs Dynamic Analysis

### 원문 내용
> **Static vs Dynamic Analysis**
>
> - Static: without executing the program
> - Dynamic: by executing the program

### 해설

**개념 설명**
프로그램 분석은 크게 정적 분석(static analysis)과 동적 분석(dynamic analysis)으로 나뉩니다.

**배경 지식**
- 정적 분석(Static Analysis): 프로그램을 실행하지 않고 소스 코드나 바이트코드를 분석합니다
  - 장점: 모든 가능한 실행 경로를 고려할 수 있습니다
  - 단점: 보수적이므로 false positive가 발생할 수 있습니다

- 동적 분석(Dynamic Analysis): 프로그램을 실제로 실행하면서 런타임 정보를 수집합니다
  - 장점: 구체적인 실행 데이터를 얻을 수 있습니다
  - 단점: 실행된 경로만 분석하므로 모든 버그를 찾을 수 없습니다

**전체적인 맥락**
이 강의는 정적 분석에 초점을 맞추고 있습니다.

---

## 슬라이드 16: Focus of This Course: Static Analysis

### 원문 내용
> **Focus of This Course: Static Analysis**
>
> - We will focus on static analysis techniques
> - Why static analysis?

### 해설

**개념 설명**
이 강의는 정적 분석(static analysis)에 집중합니다.

**배경 지식**
이 강의에서 정적 분석을 선택하는 이유는 다음 슬라이드에서 설명됩니다.

**전체적인 맥락**
이 슬라이드는 강의의 초점을 명확히 하고, 다음 슬라이드로 연결되는 전환점입니다.

---

## 슬라이드 17: Static Analysis: Advantages

### 원문 내용
> **Static Analysis: Advantages**
>
> - Ensures termination
>   - Execution may not terminate
> - Does not require concrete inputs
>   - Execution requires concrete inputs
> - Can deal with partial programs
>   - Execution requires complete programs

### 해설

**개념 설명**
정적 분석은 동적 분석에 비해 여러 장점을 가집니다.

**배경 지식**

1. **종료 보장 (Ensures termination)**
   - 동적 분석은 무한 루프에 빠질 수 있습니다
   - 정적 분석은 프로그램을 실행하지 않으므로 항상 종료됩니다

2. **구체적 입력 불필요 (Does not require concrete inputs)**
   - 동적 분석은 테스트 입력이 필요합니다
   - 정적 분석은 입력 값을 가정하지 않고 모든 가능한 입력을 고려합니다

3. **부분 프로그램 처리 가능 (Can deal with partial programs)**
   - 동적 분석은 완전한 프로그램이 필요합니다
   - 정적 분석은 라이브러리 함수의 구현이 없어도 분석할 수 있습니다

**전체적인 맥락**
이들 장점은 정적 분석이 실무에서 실용적이라는 것을 보여줍니다.

---

## 슬라이드 18: Static Analysis: Limitations

### 원문 내용
> **Static Analysis: Limitations**
>
> - Cannot be both sound and complete for non-trivial properties
> - We must choose: soundness or completeness?

### 해설

**개념 설명**
정적 분석은 근본적인 한계가 있습니다. 비자명한 성질에 대해서는 정확성(soundness)과 완전성(completeness)을 동시에 만족할 수 없습니다.

**배경 지식**
- 이것은 Rice의 정리(Rice's theorem)로부터 따르는 이론적 한계입니다
- 비자명한 성질(non-trivial property)이란 일부 프로그램은 만족하고 일부는 만족하지 않는 성질입니다

**전체적인 맥락**
정적 분석의 설계는 이 기본적 한계에 직면하게 되며, 어느 쪽을 포기할지 선택해야 합니다.

---

## 슬라이드 19: Soundness and Completeness

### 원문 내용
> **Soundness and Completeness**
>
> Soundness: If the analysis says a certain behavior is impossible, then it is indeed impossible
> - Overapproximates possible behaviors
> - No false negatives
>
> Completeness: If the analysis says a certain behavior is possible, then it is indeed possible
> - Underapproximates possible behaviors
> - No false positives

### 해설

**개념 설명**
정확성과 완전성은 정적 분석의 두 가지 중요한 성질입니다.

**배경 지식**

**정확성 (Soundness)**
- 정의: 분석이 "불가능"이라고 하면, 그것은 정말 불가능합니다
- 의미: 과도 근사(overapproximation) - 실제보다 더 많은 가능성을 포함합니다
- 거짓 음성(false negative) 없음: 실제 버그를 놓치지 않습니다
- 수식: 분석된 동작 ⊇ 실제 동작

**완전성 (Completeness)**
- 정의: 분석이 "가능"이라고 하면, 그것은 정말 가능합니다
- 의미: 과소 근사(underapproximation) - 실제보다 더 적은 가능성만 포함합니다
- 거짓 양성(false positive) 없음: 거짓 경보가 없습니다
- 수식: 분석된 동작 ⊆ 실제 동작

**전체적인 맥락**
이 두 성질은 상충관계(trade-off)에 있습니다. 다음 슬라이드에서 구체적인 예를 봅니다.

---

## 슬라이드 20: Example: Soundness vs Completeness

### 원문 내용
> **Example: Soundness vs Completeness**
>
> ```c
> if (...) {
>   x = 1;
> } else {
>   x = 2;
> }
> return x;
> ```
>
> Sound analysis may say:
> - "x is 1 or 2" ✓
> - "x is a positive integer" ✓
> - "x is any integer" ✓
> - but NOT "x is 1" ✗
>
> Complete analysis may say:
> - "x is 1" ✓
> - "x is 2" ✓
> - "x is 1 or 2" ✓
> - but NOT "x is a positive integer" ✗

### 해설

**개념 설명**
간단한 코드 예시를 통해 정확성과 완전성의 차이를 보여줍니다.

**수식/기호/코드 설명**
```c
if (...) {
  x = 1;
} else {
  x = 2;
}
return x;
```

이 코드에서 x는 실제로 정확히 {1, 2}의 값을 가질 수 있습니다.

**정확한 분석 (Sound Analysis)**
- "x는 1 또는 2" ✓ (실제 가능성을 포함)
- "x는 양의 정수" ✓ (실제 가능성을 포함하고 더 포함)
- "x는 어떤 정수" ✓ (실제 가능성을 포함하고 훨씬 더 포함)
- "x는 1" ✗ (실제로는 2도 가능하므로, 정확한 분석은 이렇게 말할 수 없음)

**완전한 분석 (Complete Analysis)**
- "x는 1" ✓ (실제로 가능)
- "x는 2" ✓ (실제로 가능)
- "x는 1 또는 2" ✓ (정확)
- "x는 양의 정수" ✗ (완전한 분석은 이렇게 과장할 수 없음, x가 다른 양의 정수가 될 가능성이 없기 때문)

**배경 지식**
- 정확한 분석은 실제 가능성의 상위집합(overapproximation)을 계산합니다
- 완전한 분석은 실제 가능성의 부분집합(underapproximation)을 계산합니다

**전체적인 맥락**
이 예시는 정확성과 완전성 사이의 기본적인 트레이드오프를 명확히 합니다.

---

## 슬라이드 21: The Ideal: Sound and Complete

### 원문 내용
> **The Ideal: Sound and Complete**
>
> Ideally, we want a sound AND complete analysis:
> - If it says a behavior is impossible ⟹ it is indeed impossible
> - If it says a behavior is possible ⟹ it is indeed possible
> - No false negatives AND no false positives
>
> But is this possible?

### 해설

**개념 설명**
이상적으로는 정확하면서도 완전한 분석을 원하지만, 이것이 가능한가라는 질문입니다.

**배경 지식**
- 정확성과 완전성을 동시에 만족하려면 분석 결과가 정확히 실제 동작과 일치해야 합니다
- 이는 다음 슬라이드에서 설명되는 Rice의 정리로 인해 불가능함을 알 수 있습니다

**전체적인 맥락**
이 슬라이드는 다음 슬라이드로의 전개를 준비하는 수사적 질문입니다.

---

## 슬라이드 22: Rice's Theorem

### 원문 내용
> **Rice's Theorem**
>
> Rice's Theorem: Any non-trivial property of the behavior of programs in a Turing-complete language is undecidable⁵
>
> ⁵Classes of recursively enumerable sets and their decision problems (Rice, 1953)

### 해설

**개념 설명**
Rice의 정리는 계산 가능성 이론의 기초가 되는 중요한 정리입니다. 튜링-완전 언어에서는 프로그램 동작의 비자명한 성질을 자동으로 판정할 수 없다는 것입니다.

**배경 지식**
- **튜링-완전 (Turing-complete)**: 현대의 대부분 프로그래밍 언어는 튜링-완전입니다 (C, Python, Java 등)
- **비자명한 성질 (non-trivial property)**: 모든 프로그램이 만족하거나 모든 프로그램이 만족하지 않는 것이 아닌 성질입니다
  - 예: "프로그램이 무한 루프에 빠지는가?" → 비자명
  - 반례: "프로그램이 코드를 포함하는가?" → 자명 (항상 참)

- **판정 불가능 (undecidable)**: 어떤 프로그램도 이 문제를 정확하게 풀 수 없다는 의미입니다

**전체적인 맥락**
Rice의 정리는 정적 분석이 완벽할 수 없다는 이론적 증명을 제공합니다. 따라서 정확성과 완전성 사이의 선택을 피할 수 없습니다.

---

## 슬라이드 23: Rice's Theorem: Example

### 원문 내용
> **Rice's Theorem: Example**
>
> - Solving an analysis problem is at least as hard as solving the halting problem, which is undecidable
>
> ```c
> if (...) {
>   f();
>   x = 1;
> } else {
>   x = 2;
> }
> return x;
> ```
>
> - x can be 1 only when f() terminates
> - Determining this requires solving the halting problem
> - We must choose either soundness or completeness

### 해설

**개념 설명**
Rice의 정리를 구체적인 예로 설명합니다. 어떤 변수가 특정 값만 가질 수 있는지 판정하는 것은 멈춤 문제(halting problem) 만큼 어렵습니다.

**수식/기호/코드 설명**
```c
if (...) {
  f();
  x = 1;
} else {
  x = 2;
}
return x;
```

- x의 값은 f()가 반환되는지 여부에 따라 달라집니다
- "x는 정확히 {1, 2}인가?"를 판정하려면 f()가 종료되는지 알아야 합니다
- 하지만 일반적인 경우 함수의 종료 여부를 결정할 수 없습니다

**배경 지식**
- **멈춤 문제 (Halting problem)**: 주어진 프로그램이 종료되는지 판정하는 문제
- Turing이 1936년에 증명했듯이, 이 문제는 판정 불가능합니다
- 정적 분석 문제를 멈춤 문제로 축약(reduction)할 수 있으므로, 정적 분석도 일반적으로 판정 불가능합니다

**전체적인 맥락**
이 예시는 단순해 보이는 분석 문제도 근본적으로 불가능할 수 있음을 보여줍니다.

---

## 슬라이드 24: Soundness vs Completeness: Which to Choose?

### 원문 내용
> **Soundness vs Completeness: Which to Choose?**
>
> The choice depends on the application
>
> Let's consider two applications:
> 1. Semantics-preserving transformation
> 2. Bug finding

### 해설

**개념 설명**
정확성과 완전성 중 어느 것을 선택할지는 응용 분야에 따라 달라집니다. 두 가지 주요 응용을 비교합니다.

**배경 지식**
같은 정적 분석이라도 사용 목적에 따라 필요한 성질이 다릅니다.

**전체적인 맥락**
다음 두 슬라이드에서 각 응용에 대해 자세히 설명합니다.

---

## 슬라이드 25: For Semantics-Preserving Transformation

### 원문 내용
> **For Semantics-Preserving Transformation**
>
> ```
> if (x == 1) { ... } else { ... }
> ```
>
> - If a complete analysis says "x is 1":
>   - Only with this information, we CANNOT remove the else branch
>   - It might miss the case where x is not 1
>
> - If a sound analysis says "x is 1":
>   - x can have no other value
>   - We CAN safely remove the else branch
>
> Conclusion: We need sound analysis

### 해설

**개념 설명**
의미 보존 변환(프로그램 최적화)을 위해서는 정확한(sound) 분석이 필요합니다.

**수식/기호/코드 설명**
```c
if (x == 1) { ... } else { ... }
```

**완전한 분석의 문제**
- "x는 1이다"라고 하면, 실제로 x는 1일 수도 있고 아닐 수도 있습니다
- else 분기를 제거하면 x가 1이 아닌 경우를 놓칠 수 있습니다
- 프로그램의 의미가 변경됩니다 (잘못된 최적화)

**정확한 분석의 이점**
- "x는 1이다"라고 하면, x는 정말로 1이고 다른 값일 수 없습니다
- else 분기를 안전하게 제거할 수 있습니다
- 프로그램의 의미가 보존됩니다 (올바른 최적화)

**배경 지식**
정확성(soundness)은 보수적(conservative) 분석이라고도 합니다. 더 많은 가능성을 고려하여 잘못된 최적화를 피합니다.

**전체적인 맥락**
최적화에서는 정확성이 필수입니다. 틀린 최적화는 프로그램을 깨뜨립니다.

---

## 슬라이드 26: For Bug Finding

### 원문 내용
> **For Bug Finding**
>
> - A sound analysis:
>   - Allows proving the absence of bugs
>   - But often suffers from false alarms
>
> - A complete analysis:
>   - Never produces false alarms
>   - But may miss some bugs
>
> - Dynamic analyses are complete:
>   - "Program testing can be used to show the presence of bugs, but never to show their absence."⁶
>
> - To complement dynamic analyses, people often design sound static analyses

### 해설

**개념 설명**
버그 찾기를 위해서는 정확한(sound) 분석이 더 유용합니다. 정확한 분석은 버그가 없음을 증명할 수 있지만, 거짓 경보(false alarm)가 많을 수 있습니다.

**배경 지식**

**정확한 분석 (Sound Analysis)**
- 장점: 버그의 부재를 증명할 수 있습니다 ("버그가 없다"는 확신)
- 단점: 거짓 경보가 많음 (실제 버그가 아닌데 버그라고 말함)
- 거짓 경보는 개발자 피로로 이어집니다

**완전한 분석 (Complete Analysis)**
- 장점: 거짓 경보가 없음 (보고된 것은 모두 실제 버그)
- 단점: 일부 버그를 놓칠 수 있음

**동적 분석 (Dynamic Analysis)**
- 테스트를 통해 버그의 존재를 보일 수 있지만, 부재를 증명할 수는 없습니다
- 따라서 동적 분석을 보완하기 위해 정확한 정적 분석을 많이 사용합니다

**수식/공식**
- 정확한 분석: 버그가 없음을 증명 가능 (sound)
- 완전한 분석: 모든 버그를 찾음 (complete)
- 둘 다 불가능하므로, 버그 찾기에는 정확한 분석을 선택합니다

**전체적인 맥락**
버그 찾기와 최적화는 서로 다른 요구사항이 있으며, 이에 따라 정확성과 완전성의 트레이드오프를 다르게 해결합니다.

---

## 슬라이드 27: Designing Sound Analyses: Challenge

### 원문 내용
> **Designing Sound Analyses: Challenge**
>
> - A sound analysis is easy to implement if soundness is the only goal
>   - An analysis that says "every behavior is possible" is trivially sound
>   - But it is useless
>
> - To be useful, a sound analysis should be precise as well

### 해설

**개념 설명**
정확한 분석을 설계하는 것은 쉽지 않습니다. 정확성만을 고려하면, 모든 가능한 동작을 말하는 분석은 정확하지만 쓸모없습니다.

**배경 지식**
- 자명한 정확한 분석: "모든 동작이 가능하다" - 정확하지만 실용성이 없음
- 유용한 정확한 분석: 정확하면서도 좁은 범위의 가능성만 포함

**전체적인 맥락**
정확한 분석을 설계할 때는 정확성과 정밀도(precision)의 새로운 트레이드오프가 생깁니다.

---

## 슬라이드 28: Precision

### 원문 내용
> **Precision**
>
> - An analysis is more precise if the set of computed possible behaviors is smaller
>
> - Example (from more to less precise):
>   - "x is 1 or 2" (most precise)
>   - "x is a positive integer"
>   - "x is any integer" (least precise)
>
> - This coincides with the usual use of the term "precision":
>   - precision = true positives / all positives

### 해설

**개념 설명**
정밀도(precision)는 정적 분석에서 또 다른 중요한 개념입니다. 분석 결과가 실제 가능성에 얼마나 가까운지를 나타냅니다.

**배경 지식**

**정밀도의 정의**
- 계산된 가능한 동작의 집합이 작을수록 정밀합니다
- 이상적으로는 실제 가능한 동작과 정확히 일치해야 합니다

**예시 (정밀도 높음 → 낮음)**
1. "x는 1 또는 2" (가장 정밀함)
2. "x는 양의 정수"
3. "x는 어떤 정수" (가장 부정밀함)

**정밀도의 수식**
- precision = true positives / all positives
- precision = |실제 동작 ∩ 분석된 동작| / |분석된 동작|
- 정밀도가 높을수록 거짓 경보가 적습니다

**배경 지식**
정밀도는 정확성(soundness)과는 독립적인 개념입니다:
- 정확한 분석은 높은 정밀도를 위해 노력해야 함
- 완전한 분석은 정밀도를 희생할 수 있음

**전체적인 맥락**
실용적인 정적 분석은 정확하면서도 정밀해야 합니다.

---

## 슬라이드 29: Goals in Static Analysis Design

### 원문 내용
> **Goals in Static Analysis Design**
>
> Minimal goals:
> - Termination
> - Soundness
>
> Additional goals (to make it useful):
> - Efficiency
> - Precision
>
> Trade-off: Often exists between efficiency and precision

### 해설

**개념 설명**
정적 분석을 설계할 때는 여러 목표를 고려해야 합니다. 이들 목표 중 일부는 상충합니다.

**배경 지식**

**최소 목표 (Minimal Goals)**
1. **종료 (Termination)**: 분석은 반드시 종료되어야 합니다
   - 무한 루프에 빠지는 분석은 불가능합니다

2. **정확성 (Soundness)**: 분석이 불가능이라고 하면 정말 불가능해야 합니다
   - 최적화나 버그 찾기 모두 정확성이 필요합니다

**추가 목표 (Additional Goals)**
1. **효율성 (Efficiency)**: 분석이 빨라야 합니다
   - 실제 사용을 위해서는 빠른 분석이 필요합니다

2. **정밀도 (Precision)**: 분석 결과가 정확해야 합니다
   - 거짓 경보를 줄이기 위해 필요합니다

**트레이드오프 (Trade-off)**
- 효율성과 정밀도 사이: 더 정밀한 분석은 보통 더 느립니다
- 정확성과 효율성 사이: 완전히 정확한 분석은 보통 불가능하므로, 정확하면서도 효율적인 근사를 찾습니다

**전체적인 맥락**
좋은 정적 분석 도구는 이 네 가지 목표 사이에서 현명한 균형을 맞춰야 합니다.

---

## 슬라이드 30: Course Objectives

### 원문 내용
> **Course Objectives**
>
> - Understand existing static analysis techniques
>   - Type analysis, interval analysis, Andersen-style pointer analysis, etc.
>
> - Understand frameworks for designing new static analyses
>   - Unification, monotone framework, cubic solver
>
> - Reason about the soundness of static analyses
>   - Abstract interpretation

### 해설

**개념 설명**
이 강의의 세 가지 주요 목표를 제시합니다.

**배경 지식**

**1. 기존 정적 분석 기법 이해**
- **타입 분석 (Type Analysis)**: 변수의 타입을 자동으로 추론합니다
- **구간 분석 (Interval Analysis)**: 변수가 가질 수 있는 값의 범위를 분석합니다
- **Andersen 스타일 포인터 분석**: 포인터 별칭 관계를 분석합니다

**2. 새로운 정적 분석 설계 프레임워크 이해**
- **Unification (합일)**: 타입 변수를 풀기 위한 기법
- **Monotone Framework (단조 프레임워크)**: 데이터 흐름 분석의 일반적 틀
- **Cubic Solver (3차 솔버)**: 효율적인 포인터 분석을 위한 알고리즘

**3. 정적 분석의 정확성에 대한 추론**
- **추상 해석 (Abstract Interpretation)**: 정적 분석의 수학적 기초
  - 구체적 의미(concrete semantics)와 추상 의미(abstract semantics) 사이의 관계
  - 분석이 정확한 이유를 이론적으로 증명할 수 있습니다

**전체적인 맥락**
이 강의는 세 가지 수준의 지식을 제공합니다:
1. 이미 존재하는 기법들 배우기
2. 새로운 기법 설계하기
3. 자신의 설계가 올바른지 증명하기

---

## 슬라이드 31: Learning in the LLM Era

### 원문 내용
> **Learning in the LLM Era**
>
> Quote from Chris Lattner (creator of LLVM, Clang, Swift, MLIR):⁷
>
> "The scarce skills become choosing the right abstractions, defining meaningful problems, and designing systems that humans and AI can evolve together."
>
> "Engineers should clarify intent with rigor, validate outcomes with tests, and improve their design."
>
> ⁷https://www.modular.com/blog/the-claude-c-compiler-what-it-reveals-about-the-future-of-software

### 해설

**개념 설명**
LLM(Large Language Model) 시대에 프로그래머가 개발해야 할 기술이 변하고 있습니다. Chris Lattner의 인용문은 이를 명확히 합니다.

**배경 지식**
- **Chris Lattner**: LLVM, Clang, Swift, MLIR의 창시자
  - LLVM은 현대 컴파일러 개발의 표준 기반입니다
  - Swift는 Apple의 현대 프로그래밍 언어입니다
  - 그의 의견은 프로그래밍 언어 커뮤니티에서 매우 영향력이 있습니다

**인용문의 의미**
1. **선택할 수 있는 기술 (scarce skills)**
   - 올바른 추상화 선택
   - 의미 있는 문제 정의
   - 인간과 AI가 함께 진화할 수 있는 시스템 설계

2. **엔지니어가 해야 할 일**
   - 의도를 명확하게 설명 (clarify intent with rigor)
   - 테스트로 결과 검증 (validate outcomes with tests)
   - 설계 개선 (improve their design)

**전체적인 맥락**
이 강의에서 배우는 프로그램 분석 기술은 이러한 "선택할 수 있는 기술"의 핵심입니다. AI 시대에도 인간의 창의적 설계와 문제 정의가 중요하며, 프로그램 분석은 설계를 검증하는 도구입니다.

---

## 슬라이드 32: LLMs and This Course

### 원문 내용
> **LLMs and This Course**
>
> - LLMs are already good at implementing PL techniques, including static analysis
> - You are allowed and encouraged to use LLMs for assignments and project
>
> - Focus on:
>   - Understanding important ideas in static analysis
>   - Designing high-level structures
>   - Finding effective ways to manage LLMs
>   - Validating the results of LLMs

### 해설

**개념 설명**
이 강의는 LLM 시대의 학습 환경을 반영하고 있습니다. LLM 사용을 권장하지만, 비판적 사고와 검증에 초점을 맞춥니다.

**배경 지식**
- LLM은 이미 프로그래밍 언어 기법(including 정적 분석)을 잘 구현할 수 있습니다
- 따라서 "구현 자체"보다는 "이해"와 "검증"이 더 중요합니다

**강의의 초점**
1. **정적 분석의 중요한 아이디어 이해 (Understanding important ideas)**
   - 이론적 기초 (Rice의 정리, 추상 해석 등)
   - 정확성과 완전성의 트레이드오프
   - 다양한 분석 기법

2. **고수준 구조 설계 (Designing high-level structures)**
   - 어떤 종류의 분석이 필요한가?
   - 분석의 구조를 어떻게 설계할 것인가?
   - 추상화의 선택

3. **LLM 관리 (Managing LLMs)**
   - 효과적인 프롬프트 작성
   - LLM의 출력을 조정하고 개선하기
   - 여러 LLM의 결과를 비교

4. **결과 검증 (Validating results)**
   - LLM의 구현이 올바른가?
   - 생성된 분석이 정확한가?
   - 테스트를 통한 검증

**전체적인 맥락**
현대의 소프트웨어 엔지니어는 LLM을 효과적으로 사용할 수 있어야 하지만, 더 중요한 것은 무엇을 하려는지 이해하고, 결과가 올바른지 검증할 수 있는 능력입니다.

---

## 슬라이드 33: Summary

### 원문 내용
> **Summary**
>
> - Program analysis automatically figures out program properties
> - Static analysis (without execution) vs Dynamic analysis (with execution)
> - Static analysis advantages: termination, no inputs needed, partial programs
> - Static analysis limitations: cannot be both sound and complete (Rice's theorem)
> - We focus on sound analyses with high precision

### 해설

**개념 설명**
첫 번째 강의의 핵심 내용을 요약합니다.

**배경 지식**

**프로그램 분석 (Program Analysis)**
- 주어진 프로그램의 성질을 자동으로 파악합니다
- 세 가지 주요 응용: 프로그램 변환, 버그 찾기, 프로그램 이해

**정적 분석 vs 동적 분석**
- 정적 분석: 프로그램을 실행하지 않고 분석 (빠르고 철저함)
- 동적 분석: 프로그램을 실행하면서 분석 (정확하지만 불완전함)

**정적 분석의 장점**
1. 종료 보장: 프로그램을 실행하지 않으므로 무한 루프 문제 없음
2. 구체적 입력 불필요: 모든 가능한 입력을 고려
3. 부분 프로그램 처리 가능: 라이브러리 구현이 없어도 분석 가능

**정적 분석의 한계**
- Rice의 정리: 튜링-완전 언어에서 비자명한 성질은 판정 불가능
- 따라서 정확성(soundness)과 완전성(completeness)을 동시에 만족할 수 없음

**이 강의의 초점**
- 정확한 분석 (sound analysis) 설계
- 높은 정밀도 (high precision)를 목표로 함
- 정밀도를 통해 거짓 경보를 최소화

**전체적인 맥락**
이 요약은 강의의 첫 번째 부분이 다룬 핵심 개념들을 정리합니다. 다음 강의부터는 이러한 기초 위에 구체적인 정적 분석 기법들을 배웁니다.

---

## 종합 해설: Lecture 1의 흐름

이 첫 번째 강의는 다음과 같은 흐름으로 진행됩니다:

### 1. 동기 제시 (슬라이드 5-13)
- 프로그램 분석이 왜 필요한가?
- 세 가지 실제 응용: 최적화, 버그 찾기, IDE 지원

### 2. 기본 개념 정의 (슬라이드 14-18)
- 프로그램 분석이란 무엇인가?
- 정적 분석 vs 동적 분석

### 3. 이론적 기초 (슬라이드 19-23)
- 정확성과 완전성의 정의
- Rice의 정리를 통한 이론적 한계 증명

### 4. 실제 선택 (슬라이드 24-29)
- 정확성과 완전성 사이의 선택
- 정밀도의 중요성

### 5. 강의의 방향성 (슬라이드 30-33)
- 강의의 목표와 초점
- 현대 (LLM 시대)에서의 학습 방법
