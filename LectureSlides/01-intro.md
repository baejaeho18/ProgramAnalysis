# Introduction to Program Analysis - 강의 슬라이드 해설

CSE552 Program Analysis — Lecture 1
강사: Jaemin Hong

> **이 파일을 읽는 법**: 각 슬라이드는 `원문 내용`(영어 슬라이드의 충실한 인용) → `번역`(한국어 직역) → `해설`(개념·배경·맥락) → 필요 시 `각주`/`슬라이드 연결` 순서입니다. 각주(¹²³)는 신입생이 모를 수 있는 용어나 더 깊이 들어가고 싶은 사람을 위한 보충입니다. 누락·왜곡 없이 원문의 모든 정보를 담되, 처음 보는 사람도 따라올 수 있게 풀어 썼습니다.

---

## 강의 1 전체 조감도 (먼저 큰 그림부터)

이 강의는 과목 전체의 **출발점이자 지도**입니다. 한 문장으로: **"프로그램 분석이란 무엇이고, 왜 필요하며, 무엇이 근본적으로 가능하고 불가능한가"**를 정합니다.

뼈대는 네 부분입니다:
1. **왜 필요한가 (동기 3가지)** — 프로그램 변환(최적화), 버그 찾기, 프로그램 이해(IDE) (슬라이드 5~14)
2. **정적 vs 동적 분석** — 실행하지 않고 분석하는 정적 분석의 장단점 (슬라이드 15~17)
3. **근본적 한계** — 어떤 분석도 **건전(sound)하면서 완전(complete)할 수 없다**(Rice 정리), 그래서 둘 중 하나를 골라야 함 (슬라이드 18~26)
4. **설계 목표** — 종료·건전성(필수) + 효율·정밀도(유용성), 그리고 그 사이의 trade-off (슬라이드 27~33)

이 강의에서 세운 두 핵심 개념 — **건전성(soundness)**과 **정밀도(precision)**, 그리고 그 사이의 **trade-off** — 는 이후 모든 강의(부호·구간·포인터·관계형·추상 해석)를 관통하는 나침반이 됩니다. 특히 "건전하지만 정밀한 분석을 어떻게 설계하는가"가 과목 전체의 목표입니다(강의 18~20의 추상 해석이 이 "건전성"을 수학적으로 완성).

---

## 슬라이드 1: 제목 슬라이드

### 원문 내용
> Introduction to Program Analysis
> CSE552 Program Analysis — Lecture 1
> Jaemin Hong

### 번역
> 프로그램 분석 입문 / CSE552 프로그램 분석 — 강의 1 / 홍재민

### 해설
과목의 첫 강의. **프로그램 분석(Program Analysis)** — 프로그램의 성질을 자동으로 알아내는 기법들 — 을 소개합니다.

---

## 슬라이드 2: Instructor Information

### 원문 내용
> - Name: Jaemin Hong
> - Email: jaemin.hong@unist.ac.kr
> - Website: https://hjaem.info/
> - Research Areas: Programming Languages, Static Analysis, Program Transformation
> - Office: Room 710-1, Building 106
> - Office Hours: Wednesday 10:00–11:00 AM (or by appointment)

### 번역
> 강사 정보: 홍재민, 이메일·웹사이트·연구분야(프로그래밍 언어·정적 분석·프로그램 변환)·연구실·오피스아워.

### 해설
강사의 연구 분야(정적 분석·프로그램 변환)가 곧 이 과목의 주제이며, 강의 12~13의 C-to-Rust 변환 응용이 강사 본인의 연구입니다.

---

## 슬라이드 3: Course Information

### 원문 내용
> - Lecture slides will be provided
> - Textbook: "Static Program Analysis" by Anders Møller and Michael I. Schwartzbach (https://cs.au.dk/~amoeller/spa/)
> - Reference: "Introduction to Static Analysis: An Abstract Interpretation Perspective" by Xavier Rival and Kwangkeun Yi

### 번역
> 강의 슬라이드 제공. 교재: Møller & Schwartzbach "Static Program Analysis"(무료 공개). 참고서: Rival & Yi "Introduction to Static Analysis"(추상 해석 관점).

### 해설

**배경 지식 — 두 교재**

- **Møller & Schwartzbach (SPA)**: 무료 공개 교재로, 타입 분석·포인터 분석·데이터플로우의 표준 입문서. 이 과목의 주 흐름.
- **Rival & Yi**: 추상 해석(강의 18~20)을 깊이 다루는 책. 한국 정적 분석 연구의 대가 이광근(Kwangkeun Yi) 교수 공저.

두 책 모두 이 과목의 후반(특히 추상 해석)에서 배경이 됩니다.

---

## 슬라이드 4: Grading

### 원문 내용
> - Midterm Exam: 25%
> - Final Exam: 25%
> - Assignments: 20% (4 programming assignments, 5% each)
> - Presentations: 30% (Term project and end-of-term presentation; Define your own software problem and solve it by designing and implementing a static analysis)

### 번역
> 평가: 중간 25%, 기말 25%, 과제 20%(프로그래밍 4개, 각 5%), 발표 30%(텀 프로젝트 — 직접 소프트웨어 문제를 정의하고 정적 분석을 설계·구현해 해결).

### 해설

**개념 설명**

시험이 50%로 큰 비중입니다(이 해설집의 시험 대비가 중요한 이유). 텀 프로젝트는 "직접 정적 분석을 설계·구현"하는 것으로, 과목의 최종 목표(새 분석을 만들 수 있게 되기)와 직결됩니다. 프로그래밍 과제 4개는 실제 분석 구현(예: Assignment 4의 interprocedural interval analysis).

---

## 슬라이드 5: Motivation 1: Program Transformation

### 원문 내용
> - Compiler optimization significantly affects software performance
> - GCC -O0 vs -O3: 1.2x–15x speedup¹
> - How can we optimize programs automatically?
>
> ¹ Impact of GCC optimization levels in energy consumption during C/C++ program execution (Branco and Henriques, 2015)

### 번역
> - 컴파일러 최적화는 소프트웨어 성능에 큰 영향
> - GCC `-O0`(최적화 끔) vs `-O3`(최대 최적화): 1.2~15배 속도 향상
> - 어떻게 프로그램을 **자동으로** 최적화할까?

### 해설

**개념 설명 — 동기 1: 변환/최적화**

프로그램 분석의 첫 용도는 **컴파일러 최적화**입니다. 같은 코드라도 최적화 수준에 따라 1.2~15배 빨라집니다. 그런데 최적화(코드 변환)를 **안전하게**(의미를 바꾸지 않고) 하려면, "이 변환이 정말 결과를 안 바꾸나?"를 알아야 합니다 — 그 판단이 곧 프로그램 분석입니다. 구체 예가 슬라이드 6.

---

## 슬라이드 6: Example: Loop-Invariant Code Motion

### 원문 내용
> Original:
> ```c
> for (i = 0; i < n; i++) {
>   y = x * x;
>   print(y);
>   *p += 1;
> }
> ```
> Optimized:
> ```c
> y = x * x;
> for (i = 0; i < n; i++) {
>   print(y);
>   *p += 1;
> }
> ```
> Does x * x evaluate to the same value in every iteration? If so, we can move it outside the loop

### 번역
> 루프 안의 `y = x*x`를 루프 밖으로 빼는 최적화(루프 불변 코드 이동). **단, `x*x`가 매 반복마다 같은 값일 때만** 안전하다.

### 해설

**개념 설명 — 변환의 안전 조건은 분석으로 검증된다**

`y = x*x`를 루프 밖으로 빼면 매번 다시 계산 안 해 빨라집니다. 하지만 이게 안전하려면 **루프 안에서 x가 바뀌지 않아야** 합니다. 그런데 `*p += 1`이 있습니다 — 만약 `p`가 `x`를 가리킨다면(앨리어싱!) x가 바뀌어 `x*x`도 매번 달라지므로 이 변환은 **틀립니다**.

"p가 x를 가리킬 수 있나?"는 **포인터 분석**(강의 14~15)이 답하고, "x*x가 루프 불변인가?"는 **데이터플로우 분석**(강의 7~8)이 답합니다. 즉 안전한 최적화의 전제 조건을 프로그램 분석이 검증합니다. 핵심 통찰이 슬라이드 7.

**슬라이드 연결**: 이 예의 앨리어싱 문제는 강의 14의 첫 동기(`*x=42; *y=-87; z=*x`)와 정확히 같은 주제입니다.

---

## 슬라이드 7: Program Transformation: Key Insight

### 원문 내용
> - Semantics-preserving transformation for optimization or refactoring requires understanding program behavior
> - We need to know what the program does
> - This is where program analysis comes in

### 번역
> - 최적화·리팩터링을 위한 **의미 보존(semantics-preserving) 변환**은 프로그램 동작의 이해를 요구한다
> - 프로그램이 *무엇을 하는지* 알아야 한다
> - 바로 여기서 프로그램 분석이 등장한다

### 해설

**개념 설명**

핵심 통찰: **변환을 안전하게 하려면 프로그램의 동작을 알아야 하고, 그 "앎"이 프로그램 분석이다.** "의미 보존"은 변환 전후 프로그램이 같은 결과를 낸다는 뜻. 분석이 동작을 (안전하게) 파악해 주면, 컴파일러는 그 정보를 믿고 최적화할 수 있습니다. 두 번째 동기가 슬라이드 8.

---

## 슬라이드 8: Motivation 2: Bug Finding

### 원문 내용
> Software bugs can lead to:
> - Significant financial losses
> - Security breaches
> - Loss of life
> We need automated techniques to find bugs before they cause damage

### 번역
> 소프트웨어 버그는 막대한 금전 손실·보안 침해·인명 피해로 이어질 수 있다. 피해가 나기 전에 버그를 찾는 **자동화 기법**이 필요하다.

### 해설

**개념 설명 — 동기 2: 버그 찾기**

두 번째 용도는 **버그 검출**입니다. 버그는 단순한 불편이 아니라 돈·보안·생명의 문제입니다(슬라이드 9~11의 실제 사례). 사람이 일일이 검사하기엔 코드가 너무 크니, 자동 분석이 필요합니다. 역사적 참사 세 가지가 슬라이드 9~11.

---

## 슬라이드 9: Historical Bug Example: Ariane 5

### 원문 내용
> - Ariane 5 rocket failure (1996)
> - Bug type: Integer overflow
> - Impact: Loss of more than $370 million²

### 번역
> 아리안 5 로켓 폭발(1996). 버그: **정수 오버플로**. 피해: 3억 7천만 달러 이상 손실.

### 해설

**배경 지식**

아리안 5 로켓은 발사 37초 후 폭발했습니다. 원인은 64비트 부동소수점을 16비트 정수로 변환하다 **오버플로**가 난 것. 정수 범위 분석(강의 9의 구간 분석)이 잡을 수 있는 종류의 버그입니다. 정적 분석이 막을 수 있었던 대표적 참사.

---

## 슬라이드 10: Historical Bug Example: Therac-25

### 원문 내용
> - Therac-25 radiation therapy machine (1985–1987)
> - Bug type: Data race
> - Impact: At least 6 accidents, resulting in death or serious injury³

### 번역
> 테락-25 방사선 치료기(1985~1987). 버그: **데이터 레이스**. 피해: 최소 6건 사고, 사망·중상.

### 해설

**배경 지식**

테락-25는 동시성 버그(**데이터 레이스**)로 환자에게 치명적 과다 방사선을 조사했습니다. 데이터 레이스는 강의 12(락 분석)에서 다루는 바로 그 문제 — 공유 데이터를 락 없이 동시 접근. 정적 분석으로 락 사용을 검증하면 막을 수 있는 종류입니다.

---

## 슬라이드 11: Historical Bug Example: Heartbleed

### 원문 내용
> - Heartbleed bug (2014)
> - Bug type: Buffer overflow
> - Impact: Estimated cost of $500 million⁴

### 번역
> 하트블리드 버그(2014). 버그: **버퍼 오버플로**. 피해: 추정 5억 달러.

### 해설

**배경 지식**

하트블리드는 OpenSSL의 **버퍼 오버플로**로, 서버 메모리를 외부에서 읽어낼 수 있게 해 전 세계 보안을 위협했습니다. 배열 경계 검사(강의 16~17의 관계형 분석에서 `인덱스 < 크기` 추적)가 잡을 수 있는 종류. C의 메모리 비안전성(강의 12~13의 C-to-Rust 동기)이 낳은 참사입니다.

---

## 슬라이드 12: Bug Finding Example

### 원문 내용
> - Can this divisor expression evaluate to zero? If so, we have a division by zero error
> - This is a question that program analysis can answer

### 번역
> "이 나눗셈의 분모가 0이 될 수 있나? 그렇다면 0으로 나누기 오류다." — 이것이 프로그램 분석이 답할 수 있는 질문이다.

### 해설

**개념 설명**

"분모가 0이 될 수 있나?"는 변수의 가능한 값 범위를 추적하면 답할 수 있습니다 — **구간 분석**(강의 9)이나 **부호 분석**(강의 5)의 전형적 응용. 분석이 "분모는 항상 양수"라고 증명하면 0으로 나누기가 없음을 보장합니다(버그 부재 증명). 세 번째 동기가 슬라이드 13.

---

## 슬라이드 13: Motivation 3: Program Comprehension

### 원문 내용
> - Modern development process heavily utilizes IDEs or editor plugins
> - Example: "mouse hover" to show the type of an expression
> (코드 예: 변수에 마우스를 올리면 타입 `float`, `dict[str, float]` 등을 표시)

### 번역
> 현대 개발은 IDE·에디터 플러그인을 많이 쓴다. 예: 식 위에 마우스를 올리면 그 식의 **타입을 표시**(타입 추론).

### 해설

**개념 설명 — 동기 3: 프로그램 이해**

세 번째 용도는 **개발자 도구**입니다. IDE가 "이 변수의 타입은?", "이 함수는 어디서 호출되나?"를 즉시 보여 주는 것 — 그 뒤엔 **타입 추론**(강의 3~4)과 **호출 그래프 분석**(강의 11)이 돌아갑니다. 분석이 개발자의 코드 이해를 돕습니다. 세 동기를 종합한 정의가 슬라이드 14.

---

## 슬라이드 14: Program Analysis

### 원문 내용
> A collection of techniques to automatically figure out the properties of given programs
> Essential for:
> - Program transformation
> - Bug finding
> - Program comprehension

### 번역
> **프로그램 분석** = 주어진 프로그램의 성질을 **자동으로 알아내는** 기법들의 모음. 변환·버그찾기·이해에 필수.

### 해설

**개념 설명 — 정의**

세 동기를 묶은 정의입니다. 핵심 단어는 **"자동으로(automatically)"** — 사람이 아니라 알고리즘이 프로그램의 성질을 알아냅니다. "성질(property)"은 "x는 항상 양수", "이 식은 0이 될 수 없다", "이 두 포인터는 같은 곳을 안 가리킨다" 같은 것. 어떻게 알아내는가(실행 여부)에 따라 정적/동적으로 갈립니다(슬라이드 15).

---

## 슬라이드 15: Static vs Dynamic Analysis

### 원문 내용
> - Static: without executing the program
> - Dynamic: by executing the program

### 번역
> - **정적(static)** 분석: 프로그램을 **실행하지 않고** 분석
> - **동적(dynamic)** 분석: 프로그램을 **실행하여** 분석

### 해설

**개념 설명 — 두 갈래**

- **정적 분석**: 코드를 *읽어서* 성질을 추론(컴파일러 경고, 타입 검사 등). 실행 안 함.
- **동적 분석**: 코드를 *돌려보며* 관찰(테스트, 프로파일링, 디버거 등).

비유: 정적은 "요리법을 읽고 맛을 예측", 동적은 "직접 만들어 맛보기". 이 과목은 **정적 분석**에 집중합니다(슬라이드 16). 둘의 장단점이 슬라이드 17~26의 주제.

---

## 슬라이드 16: Focus of This Course: Static Analysis

### 원문 내용
> - We will focus on static analysis techniques
> - Why static analysis?

### 번역
> 이 과목은 **정적 분석** 기법에 집중한다. 왜 정적 분석인가?

### 해설
과목의 초점을 정적 분석으로 명시. "왜 정적인가?"의 답이 슬라이드 17(장점)과 18~26(한계까지 솔직히).

---

## 슬라이드 17: Static Analysis: Advantages

### 원문 내용
> - Ensures termination (Execution may not terminate)
> - Does not require concrete inputs (Execution requires concrete inputs)
> - Can deal with partial programs (Execution requires complete programs)

### 번역
> 정적 분석의 장점 (vs 실행):
> - **종료 보장**: 실행은 안 끝날 수 있지만(무한 루프), 정적 분석은 항상 끝나게 설계 가능
> - **구체 입력 불필요**: 실행은 실제 입력이 있어야 하지만, 정적 분석은 입력 없이 가능
> - **부분 프로그램 처리 가능**: 실행은 완성된 프로그램이 필요하지만, 정적 분석은 일부 코드만으로도 가능

### 해설

**개념 설명 — 정적 분석이 동적보다 나은 점**

1. **종료 보장**: 동적 분석은 무한 루프에 빠질 수 있지만, 정적 분석은 (위드닝 등으로, 강의 9) 항상 유한 시간에 끝나게 만들 수 있습니다.
2. **입력 불필요**: 동적은 "이 입력에 대해서만" 알지만, 정적은 **모든 입력에 대해** 한 번에 추론(예: "어떤 입력이든 x는 양수").
3. **부분 프로그램**: 라이브러리 함수 하나만 떼어 분석 가능(전체 프로그램 없이도). IDE의 실시간 분석이 가능한 이유.

이 세 장점이 정적 분석을 강력하게 만들지만, 공짜는 아닙니다 — 한계가 슬라이드 18.

---

## 슬라이드 18: Static Analysis: Limitations

### 원문 내용
> - Cannot be both sound and complete for non-trivial properties
> - We must choose: soundness or completeness?

### 번역
> - 비자명한(non-trivial) 성질에 대해 **건전(sound)하면서 동시에 완전(complete)할 수는 없다**
> - 우리는 선택해야 한다: 건전성인가, 완전성인가?

### 해설

**개념 설명 — 정적 분석의 근본 한계 ★**

정적 분석의 가장 깊은 진실입니다. 의미 있는(비자명한) 성질에 대해서는 **완벽한 분석이 불가능**합니다 — 건전성과 완전성 둘 다 가질 수 없습니다(슬라이드 22의 Rice 정리가 증명). 따라서 **반드시 하나를 포기**해야 합니다. 두 개념의 정의가 슬라이드 19. 이 한계를 받아들이고 "어느 쪽을 포기할지" 정하는 것이 정적 분석 설계의 출발점입니다.

---

## 슬라이드 19: Soundness and Completeness

### 원문 내용
> - Soundness: If the analysis says a certain behavior is impossible, then it is indeed impossible
>   - Overapproximates possible behaviors
>   - No false negatives
> - Completeness: If the analysis says a certain behavior is possible, then it is indeed possible
>   - Underapproximates possible behaviors
>   - No false positives

### 번역
> - **건전성(soundness)**: 분석이 "어떤 동작이 불가능하다"고 하면, 실제로도 불가능하다
>   - 가능한 동작을 **과근사(overapproximate)** — 실제 가능한 것보다 더 많이(또는 같게) 봄
>   - **거짓 음성(false negative) 없음** — 실제 가능한 걸 "불가능"이라 잘못 말하지 않음
> - **완전성(completeness)**: 분석이 "어떤 동작이 가능하다"고 하면, 실제로도 가능하다
>   - 가능한 동작을 **과소근사(underapproximate)** — 실제보다 적게(또는 같게) 봄
>   - **거짓 양성(false positive) 없음** — 불가능한 걸 "가능"이라 잘못 말하지 않음

### 해설

**개념 설명 — 건전성 vs 완전성 (이 강의의 핵심 정의) ★**

헷갈리기 쉬운 두 개념을 정확히 잡읍시다. 분석이 "가능한 동작의 집합"을 추정한다고 봅시다.

- **건전한(sound) 분석**: 실제 가능한 동작을 **하나도 빠뜨리지 않음**(과근사 = 실제 ⊆ 분석). "불가능하다"는 말이 **항상 믿을 만함**. 대신 실제론 불가능한 것도 "가능"이라 할 수 있음(거짓 경보 가능). **거짓 음성 없음**.¹
- **완전한(complete) 분석**: 분석이 "가능"이라 한 건 **실제로 다 가능**(과소근사 = 분석 ⊆ 실제). "가능하다"는 말이 **항상 믿을 만함**. 대신 실제 가능한 걸 놓칠 수 있음. **거짓 양성 없음**.

**기억법**: 건전 = "안전 쪽으로 넘침(over)", "불가능 판정을 신뢰", 빠뜨림 없음. 완전 = "확실한 것만(under)", "가능 판정을 신뢰", 헛소리 없음. 예가 슬라이드 20.

**각주**

¹ "거짓 음성/양성"의 기준은 "버그가 있다(positive)"를 탐지하는 맥락에서 정의됩니다. 건전한 버그 검출기는 버그를 놓치지 않음(거짓 음성 없음)이지만 헛경보(거짓 양성)는 낼 수 있습니다. 맥락(무엇을 positive로 보나)에 따라 용어가 뒤집힐 수 있으니, **과근사=건전, 과소근사=완전**으로 기억하는 게 안전합니다.

---

## 슬라이드 20: Example: Soundness vs Completeness

### 원문 내용
> ```c
> if (...) { x = 1; } else { x = 2; }
> return x;
> ```
> Sound analysis may say:
> - "x is 1 or 2" ✓
> - "x is a positive integer" ✓
> - "x is any integer" ✓
> - but NOT "x is 1" ✗
> Complete analysis may say:
> - "x is 1" ✓
> - "x is 2" ✓
> - "x is 1 or 2" ✓
> - but NOT "x is a positive integer" ✗

### 번역
> 실제로 x는 {1, 2}.
> - **건전한 분석**은: "x는 1 또는 2"(정확), "x는 양의 정수"(과근사), "x는 임의 정수"(과근사) 모두 OK. 하지만 "x는 1"은 **안 됨**(실제 2도 가능한데 빠뜨림 = unsound).
> - **완전한 분석**은: "x는 1", "x는 2", "x는 1 또는 2" 모두 OK. 하지만 "x는 양의 정수"는 **안 됨**(양의 정수 중 3,4도 가능하다고 암시하는데 실제론 불가 = incomplete).

### 해설

**개념 설명 — 같은 코드, 다른 허용 답**

실제 x의 값 집합은 {1,2}.
- **건전(과근사)**: {1,2}를 **포함하는** 집합이면 다 OK — {1,2}, {양의 정수}, {모든 정수}. 빠뜨림(실제를 누락)만 금지 → "x는 1"({1}은 2를 빠뜨림)은 불가.
- **완전(과소근사)**: {1,2}에 **포함되는** 집합이면 다 OK — {1}, {2}, {1,2}. 군더더기(불가능을 포함)만 금지 → "x는 양의 정수"({양의 정수}는 3,4 등 불가능을 포함)는 불가.

가장 정밀한 답 "x는 1 또는 2"는 **건전이면서 완전**(정확히 {1,2})입니다. 이 코드처럼 분기만 있으면 둘 다 가능하지만, 루프·재귀가 끼면 불가능해집니다(Rice 정리, 슬22). 이상적 목표가 슬라이드 21.

---

## 슬라이드 21: The Ideal: Sound and Complete

### 원문 내용
> Ideally, we want a sound AND complete analysis:
> - If it says a behavior is impossible ⇒ it is indeed impossible
> - If it says a behavior is possible ⇒ it is indeed possible
> - No false negatives AND no false positives
> But is this possible?

### 번역
> 이상적으로는 **건전하면서 완전한** 분석을 원한다(거짓 음성도 양성도 없는 완벽한 분석). 그런데 이게 가능한가?

### 해설

**개념 설명**

건전+완전 = "분석이 정확히 실제 가능한 동작만, 빠짐없이" = **완벽한 분석**. 누구나 원하지만, 슬라이드 22의 Rice 정리가 **"비자명한 성질엔 불가능"**임을 증명합니다. 이 불가능성이 정적 분석의 모든 trade-off의 뿌리입니다.

---

## 슬라이드 22: Rice's Theorem

### 원문 내용
> Rice's Theorem: Any non-trivial property of the behavior of programs in a Turing-complete language is undecidable⁵
>
> ⁵ Classes of recursively enumerable sets and their decision problems (Rice, 1953)

### 번역
> **Rice 정리**: 튜링 완전(Turing-complete) 언어로 쓰인 프로그램의 동작에 관한 **비자명한 성질은 모두 결정 불가능(undecidable)**하다.

### 해설

**개념 설명 — 왜 완벽한 분석이 불가능한가 ★**

**Rice 정리**는 계산 이론의 근본 결과입니다. "비자명한 성질"이란 "어떤 프로그램은 가지고 어떤 프로그램은 안 가지는" 의미 있는 성질(예: "이 프로그램은 0으로 나눈다", "이 변수는 항상 양수"). **튜링 완전**(일반적인 프로그래밍 언어는 모두 그러함) 언어에서는 그런 성질을 **항상 정확히 판정하는 알고리즘이 존재하지 않습니다**(결정 불가능).²

직관: 그런 분석이 있다면 **정지 문제(halting problem)**도 풀 수 있는데, 정지 문제는 풀 수 없음이 증명됐습니다(슬라이드 23). 따라서 건전+완전한(완벽한) 분석은 원리적으로 불가능 → 둘 중 하나를 포기해야 합니다. 예가 슬라이드 23.

**각주**

² "자명한(trivial) 성질"은 모든 프로그램이 갖거나(예: "이 프로그램은 프로그램이다") 아무 프로그램도 안 갖는 성질로, 이건 판정 가능(항상 yes/no). Rice 정리는 그 외 **모든 의미 있는 성질**이 결정 불가능이라는 강력한 주장입니다.

---

## 슬라이드 23: Rice's Theorem: Example

### 원문 내용
> - Solving an analysis problem is at least as hard as solving the halting problem, which is undecidable
> ```c
> if (...) { f(); x = 1; } else { x = 2; }
> return x;
> ```
> - x can be 1 only when f() terminates
> - Determining this requires solving the halting problem
> - We must choose either soundness or completeness

### 번역
> 분석 문제는 **정지 문제(halting problem)만큼 어렵다**(정지 문제는 결정 불가능).
> 코드: `f()` 호출 후 x=1, else x=2. **x가 1이 되는 건 `f()`가 종료할 때뿐**. 이를 판정하려면 "f()가 멈추나?"(정지 문제)를 풀어야 하는데 그건 불가능 → 건전이나 완전 중 하나를 골라야.

### 해설

**개념 설명 — 정지 문제로의 환원**

"x가 1이 될 수 있나?"를 정확히 답하려면 "`f()`가 멈추나?"를 알아야 합니다(멈춰야 x=1 줄에 도달). 그런데 **정지 문제**(임의 프로그램이 멈추는지 판정)는 결정 불가능하다고 튜링이 증명했습니다. 따라서 이 분석 질문도 정확히는 못 풉니다.

**결론**: 정확히 못 푸니 **근사**해야 하고, 근사 방향(과대 vs 과소)이 곧 건전성 vs 완전성. 어느 쪽을 택할지는 용도에 달렸습니다(슬라이드 24~26).

---

## 슬라이드 24: Soundness vs Completeness: Which to Choose?

### 원문 내용
> The choice depends on the application
> Let's consider two applications:
> 1. Semantics-preserving transformation
> 2. Bug finding

### 번역
> 선택은 **용도에 달렸다**. 두 응용으로 살펴보자: (1) 의미 보존 변환, (2) 버그 찾기.

### 해설
건전/완전 선택은 절대적 정답이 없고 **용도 의존적**입니다. 변환(슬25)과 버그찾기(슬26)에서 각각 어느 쪽이 맞는지 봅니다.

---

## 슬라이드 25: For Semantics-Preserving Transformation

### 원문 내용
> ```c
> if (x == 1) { ... } else { ... }
> ```
> - If a complete analysis says "x is 1": Only with this information, we CANNOT remove the else branch (It might miss the case where x is not 1)
> - If a sound analysis says "x is 1": x can have no other value, We CAN safely remove the else branch
> Conclusion: We need sound analysis

### 번역
> - **완전한** 분석이 "x는 1"이라 해도 → else 가지를 **제거할 수 없다**(완전 분석은 가능한 걸 놓칠 수 있어, x가 1이 아닌 경우를 빠뜨렸을 수 있으니까).
> - **건전한** 분석이 "x는 1"이라 하면 → x는 **다른 값일 수 없으므로** else 가지를 안전히 제거 가능.
> 결론: **변환에는 건전한 분석이 필요**하다.

### 해설

**개념 설명 — 변환엔 건전성**

최적화(변환)는 "이 경우는 절대 안 일어난다"는 **확신**이 있어야 코드를 지울 수 있습니다. 건전한 분석의 "x는 1"은 "x는 1 외엔 절대 안 됨"을 보장(과근사라 빠뜨림 없음) → else를 안전히 제거. 완전한 분석의 "x는 1"은 "x가 1일 수 있다"일 뿐(다른 값도 가능할 수 있음) → 제거 위험. **변환·검증에는 건전성**이 맞습니다. 버그 찾기는 다를 수 있습니다(슬26).

---

## 슬라이드 26: For Bug Finding

### 원문 내용
> - A sound analysis: Allows proving the absence of bugs; But often suffers from false alarms
> - A complete analysis: Never produces false alarms; But may miss some bugs
> - Dynamic analyses are complete: "Program testing can be used to show the presence of bugs, but never to show their absence."⁶
> - To complement dynamic analyses, people often design sound static analyses
>
> ⁶ The humble programmer (Dijkstra, 1972)

### 번역
> - **건전한 분석**: 버그 **부재를 증명**할 수 있지만, **헛경보(false alarm)가 잦다**.
> - **완전한 분석**: 헛경보가 전혀 없지만, **일부 버그를 놓칠 수 있다**.
> - **동적 분석(테스트)은 완전**하다: 다익스트라 — "테스트는 버그의 *존재*는 보일 수 있어도 *부재*는 결코 보일 수 없다."
> - 동적 분석을 보완하려고, 사람들은 흔히 **건전한 정적 분석**을 설계한다.

### 해설

**개념 설명 — 버그 찾기에서의 선택**

- **건전한 분석**: "버그 없음"을 증명할 수 있음(빠뜨림 없으니). 대신 실제론 멀쩡한 코드도 의심해 **헛경보**가 많음.
- **완전한 분석**: 경보하면 진짜 버그(헛경보 없음). 대신 일부 버그를 **놓침**.

**테스트(동적 분석)는 완전**합니다 — 실제로 돌려서 버그를 보니 헛경보가 없지만, 안 돌려본 경우의 버그는 못 봄(부재 증명 불가). 다익스트라의 유명한 격언이 이 점입니다. 그래서 **테스트(완전)를 보완하려고 건전한 정적 분석**을 만듭니다 — 둘이 상보적. 이 과목은 **건전한 분석**에 집중합니다(슬33). 하지만 건전성만으론 부족(슬27).

---

## 슬라이드 27: Designing Sound Analyses: Challenge

### 원문 내용
> - A sound analysis is easy to implement if soundness is the only goal
>   - An analysis that says "every behavior is possible" is trivially sound
>   - But it is useless
> - To be useful, a sound analysis should be precise as well

### 번역
> - 건전성만이 목표라면 건전한 분석은 **쉽다**: "모든 동작이 가능하다"고 말하는 분석은 **자명하게 건전**하다(아무것도 빠뜨리지 않으니).
> - 하지만 그건 **쓸모없다**.
> - 유용하려면 건전한 분석은 **정밀(precise)해야** 한다.

### 해설

**개념 설명 — 건전성만으론 부족, 정밀도가 필요 ★**

함정: "모든 게 가능하다"고 답하는 분석은 **완벽하게 건전**합니다(실제 가능한 걸 절대 안 빠뜨림 — 다 포함하니까). 하지만 아무 정보도 안 주니 **쓸모 제로**. 예: "이 변수는 어떤 정수든 될 수 있다"는 항상 맞지만 무의미.

따라서 진짜 목표는 **"건전하면서도 정밀한"** 분석입니다 — 안전하되(과근사) 가능한 한 작은(타이트한) 집합을 주는 것. 정밀도의 정의가 슬라이드 28. 이 "건전성+정밀도" 추구가 과목 전체의 과제입니다.

---

## 슬라이드 28: Precision

### 원문 내용
> - An analysis is more precise if the set of computed possible behaviors is smaller
> - Example (from more to less precise): "x is 1 or 2" (most precise), "x is a positive integer", "x is any integer" (least precise)
> - This coincides with the usual use of the term "precision": precision = true positives / all positives

### 번역
> - 분석이 계산한 **가능한 동작의 집합이 작을수록 더 정밀**하다
> - 예(정밀→부정밀): "x는 1 또는 2"(가장 정밀) > "x는 양의 정수" > "x는 임의 정수"(가장 부정밀)
> - 이는 통상의 "정밀도(precision)" 개념과 일치: precision = 참 양성 / 전체 양성

### 해설

**개념 설명 — 정밀도 = 집합이 작을수록 좋다**

건전성을 지키는 한(실제를 포함), **추정 집합이 작을수록 정밀**합니다. 실제가 {1,2}일 때 "{1,2}"가 가장 정밀하고, "{양의 정수}", "{모든 정수}"로 갈수록 부정밀(군더더기가 많음). 머신러닝의 precision(참 양성 비율)과도 통합니다 — 헛것을 적게 포함할수록 정밀. **건전성은 "실제를 덮어라"(하한), 정밀도는 "가능한 작게"(상한 최소화)**. 이 둘의 긴장이 설계 목표를 만듭니다(슬29).

---

## 슬라이드 29: Goals in Static Analysis Design

### 원문 내용
> - Minimal goals: Termination, Soundness
> - Additional goals (to make it useful): Efficiency, Precision
> - Trade-off: Often exists between efficiency and precision

### 번역
> - **필수 목표**: 종료(termination), 건전성(soundness)
> - **추가 목표(유용성을 위해)**: 효율(efficiency), 정밀도(precision)
> - **trade-off**: 효율과 정밀도 사이에 흔히 존재

### 해설

**개념 설명 — 정적 분석 설계의 네 목표 ★**

이 슬라이드가 과목 전체의 **나침반**입니다:
- **필수(반드시)**: ① 종료 — 분석이 끝나야 함(위드닝, 강의 9). ② 건전성 — 빠뜨리지 않아야 함(추상 해석, 강의 18~20).
- **추가(유용하려면)**: ③ 효율 — 빨라야 함. ④ 정밀도 — 타이트해야 함.

핵심은 **효율 ↔ 정밀도 trade-off**: 더 정밀하면 보통 더 느립니다. 이 trade-off가 이후 모든 강의에서 반복됩니다 — Andersen(정밀·느림) vs Steensgaard(거침·빠름, 강의 14), 구간 vs 다면체 vs 팔각형(강의 16~17), 문맥 민감도(강의 10). **"종료·건전성은 지키되, 효율과 정밀도의 균형을 어떻게 잡을까"**가 정적 분석 설계의 영원한 질문입니다.

---

## 슬라이드 30: Course Objectives

### 원문 내용
> - Understand existing static analysis techniques (Type analysis, interval analysis, Andersen-style pointer analysis, etc.)
> - Understand frameworks for designing new static analyses (Unification, monotone framework, cubic solver)
> - Reason about the soundness of static analyses (Abstract interpretation)

### 번역
> 과목 목표 세 가지:
> - 기존 정적 분석 기법 이해 (타입·구간·Andersen 포인터 분석 등)
> - 새 분석 설계 **프레임워크** 이해 (단일화, 단조 프레임워크, cubic 솔버)
> - 분석의 **건전성을 추론**하기 (추상 해석)

### 해설

**개념 설명 — 과목 로드맵**

이 세 목표가 곧 과목 전체의 구조이자 이 해설집의 강의 배치입니다:
- **기존 기법**: 타입(강3~4), 구간(강9), Andersen 포인터(강14) 등.
- **설계 프레임워크**: 단일화(강14 Steensgaard), 단조 프레임워크(강7~8 데이터플로우), cubic 솔버(강11).
- **건전성 추론**: 추상 해석(강18~20).

즉 "쓸 줄 알고 → 만들 줄 알고 → 옳음을 증명할 줄 안다". 이 강의(1)에서 세운 건전성·정밀도 개념이 마지막 목표(추상 해석)에서 수학적으로 완성됩니다.

---

## 슬라이드 31: Learning in the LLM Era

### 원문 내용
> Quote from Chris Lattner (creator of LLVM, Clang, Swift, MLIR):⁷
> "The scarce skills become choosing the right abstractions, defining meaningful problems, and designing systems that humans and AI can evolve together."
> "Engineers should clarify intent with rigor, validate outcomes with tests, and improve their design."

### 번역
> 크리스 래트너(LLVM·Clang·Swift·MLIR 창시자) 인용:
> "희소해지는 능력은 **올바른 추상을 고르고, 의미 있는 문제를 정의하고, 인간과 AI가 함께 발전시킬 시스템을 설계하는 것**이다."
> "엔지니어는 의도를 엄밀히 명확히 하고, 결과를 테스트로 검증하며, 설계를 개선해야 한다."

### 해설

**개념 설명 — AI 시대에 더 중요해진 능력**

흥미롭게도 LLM 시대에 **추상화를 고르고 문제를 정의하는 능력**이 더 귀해진다는 메시지입니다. 이는 정적 분석의 핵심 — "어떤 추상 도메인(부호·구간·다면체)을 고를까"(강의 5~17), "무엇을 분석할까" — 과 정확히 일치합니다. 구현은 AI가 도와도, **올바른 추상화 선택과 문제 정의**는 사람의 몫. 이 과목이 기르려는 능력이기도 합니다.

---

## 슬라이드 32: LLMs and This Course

### 원문 내용
> - LLMs are already good at implementing PL techniques, including static analysis
> - You are allowed and encouraged to use LLMs for assignments and project
> - Focus on: Understanding important ideas in static analysis; Designing high-level structures; Finding effective ways to manage LLMs; Validating the results of LLMs

### 번역
> - LLM은 이미 정적 분석을 포함한 PL 기법 구현을 잘한다
> - 과제·프로젝트에 **LLM 사용을 허용·권장**한다
> - 집중할 것: 정적 분석의 핵심 아이디어 이해, 고수준 구조 설계, LLM을 효과적으로 다루는 법, **LLM 결과 검증**

### 해설

**개념 설명**

과목 정책: LLM 사용 권장. 단 초점은 **구현 자체가 아니라 아이디어 이해·설계·검증**에 둡니다. LLM이 코드를 짜 줘도, "이 분석이 건전한가", "이 추상화가 맞나"를 판단·검증하는 건 사람 — 그게 이 과목이 기르는 능력. (실제로 이 해설집과 Assignment 4 검증도 같은 철학.)

---

## 슬라이드 33: Summary

### 원문 내용
> - Program analysis automatically figures out program properties
> - Static analysis (without execution) vs Dynamic analysis (with execution)
> - Static analysis advantages: termination, no inputs needed, partial programs
> - Static analysis limitations: cannot be both sound and complete (Rice's theorem)
> - We focus on sound analyses with high precision

### 번역
> - 프로그램 분석은 프로그램 성질을 자동으로 알아낸다
> - 정적(실행 안 함) vs 동적(실행) 분석
> - 정적 분석의 장점: 종료, 입력 불필요, 부분 프로그램 처리
> - 정적 분석의 한계: 건전+완전을 동시에 만족 불가(Rice 정리)
> - 우리는 **높은 정밀도의 건전한 분석**에 집중한다

### 해설

**전체 정리 — 강의 1의 한 장 요약**

1. **정의**: 프로그램 분석 = 프로그램 성질을 자동으로 알아내는 기법. 동기 3가지(변환·버그찾기·이해).
2. **정적 vs 동적**: 정적은 실행 없이(종료 보장·입력 불필요·부분 프로그램), 동적은 실행하며.
3. **근본 한계**: Rice 정리 — 비자명한 성질엔 건전+완전 불가. 둘 중 하나를 포기.
4. **선택**: 변환·검증엔 건전성, 버그찾기엔 둘 다 쓰임(테스트=완전을 정적=건전으로 보완).
5. **목표**: 종료·건전성(필수) + 효율·정밀도(유용성), 효율↔정밀도 trade-off. 이 과목은 **건전하고 정밀한** 분석을 추구.

**다른 강의와의 연결 (파일 간 연결성)**

- → **강의 5~9 (부호·격자·데이터플로우·구간·위드닝)**: 건전한 분석을 실제로 설계. 종료(위드닝)·건전성·정밀도가 구체화.
- → **강의 14~15 (포인터)**: 슬라이드 6의 앨리어싱 문제를 정면으로. Andersen vs Steensgaard = 정밀·비용 trade-off(슬29).
- → **강의 16~17 (관계형)**: 정밀도 위계(구간<팔각형<다면체)가 슬라이드 28~29의 trade-off를 도메인으로 구현.
- → **강의 18~20 (추상 해석)**: 슬라이드 19의 "건전성"을 수학적으로 정의·증명. 이 강의의 직관이 정리로 완성.
- → **강의 12~13 (응용)**: 슬라이드 8~11의 버그(데이터레이스·메모리)가 C-to-Rust 변환 동기로 재등장.

**가장 큰 교훈**: 정적 분석은 **"완벽할 수 없음(Rice)을 인정하고, 건전성을 지키며 정밀도를 최대화하는 기술"**입니다. 종료·건전성은 타협 불가, 효율·정밀도는 균형의 대상 — 이 네 목표와 그 trade-off가 이후 모든 강의를 읽는 렌즈입니다.

---

## 마치며

강의 1은 과목 전체의 **개념적 토대**를 놓습니다. 핵심 한 줄: **"비자명한 성질은 정확히 분석할 수 없으므로(Rice 정리), 우리는 건전성(빠뜨리지 않음)을 지키면서 정밀도(군더더기 줄이기)를 최대화하는 분석을 설계한다."** 건전성 vs 완전성, 과근사 vs 과소근사, 효율 vs 정밀도라는 세 쌍의 긴장이 이후 17개 강의를 관통합니다. 시험에서는 (a) 건전성·완전성의 정의와 과근사/과소근사 대응(슬19~20), (b) Rice 정리와 정지 문제로의 환원(슬22~23), (c) 변환엔 왜 건전성이 필요한가(슬25), (d) "모든 게 가능"이 자명하게 건전하지만 쓸모없는 이유와 정밀도의 필요(슬27~28), (e) 정적 분석의 네 설계 목표와 trade-off(슬29)가 단골입니다.
