# 러스트 프로그래밍 언어 - 2판 한글 번역 작업을 위한 저장소

[빌드된 페이지 바로가기](https://rinthel.github.io/rust-lang-book-ko)

안녕하세요? 한국어를 쓰시는 러스트 유저 여러분들을 환영합니다. 이 저장소는 러스트 프로그래밍 언어
문서(소위 the book이라고 일컫는)의 2번째 판에 대한 번역 작업을 위해 만들어졌습니다.
1번째 판에 대한 번역 작업은 [sarojaba님께서 운영하시는 penflip 페이지](
https://www.penflip.com/sarojaba/rust-doc-korean/blob/master/About.txt)에
거의 완성되어 있으므로, 혹시 교본을 보러 오신 분이라면 이쪽을 추천하고 싶습니다. :)

이 저장소는 앞서 말씀드렸듯이 2번째 판에 대한 번역 작업을 위해 만들어졌습니다. 아래에 있는 설명에서
보시는 것과 같이, 러스트 문서는 현재 [mdbook](https://github.com/azerupi/mdBook)이라고
부르는 마크다운 기반의 웹문서 작성 툴을 이용해 빌드되는데, 이 툴에 다국어 지원 기능이 완성되면
번역본들도 함께 합쳐질 예정이라고 합니다. 한편, 2번째 판의 내용은 1번째 판과 많은 부분이 달라졌기에
새로 번역해볼 필요 겸 공부할 차원에서 만들어진 저장소라고 보시면 되겠습니다.

현재 2번째 판의 문서는 수시로 고쳐지고 있는 상태지만, [원본 저장소](https://github.com/rust-lang/book)에
프로젝트란을 보시면 frozen column이라고 되어 있는 부분들은 거의 완성되어 많은 수정이 이루어지지 않을 것이라고
언급되고 있기에, 이런 부분들을 위주로 먼저 번역해보고자 합니다.

혹시 함께 번역에 참여하고 싶으신 분들, 환영합니다! :)
저에게 메세지를 주시거나 issue에 요청을 남겨주시면, collaborator로 등록해 드리겠습니다!

### 번역 기조

#### 친절한 구어체

1번째 판도 그렇지만 2번째 판을 보면서 느낀 점은, 기초 C 프로그래밍 정도의 수준을 익힌 분들
혹은 스크립트 언어만 공부해본 분들 또한 읽기 쉽게끔 기초 개념에 충실한 설명을 하고 있다는 점입니다.
우리가 러스트의 저변을 더 넓히고자 한다면 보다 친절한 어투가 좋겠다고 생각하고 있습니다.

#### 번역하기 애매한 용어는 가급적 원어로

번역하기 애매한 용어를 억지로 한글화 하는 것 보다는 원어를 그대로 사용하는 편이 오해를 줄이는데
더 도움이 될거라고 생각하고 있습니다.

### 작업방식

각 장(chapter) 혹은 각 절(section) 별로 이슈를 만들고, 브랜치를 나누어 작업한 다음,
PR 리뷰를 통해 통합하고, 그 외에 오탈자 수정 등 자잘한 수정 사항도 PR을 통해 검수해나가볼 생각입니다.

### 현재까지 번역 용어 정리 (abc순)

- annotation: 어노테이션
- arm: (match 문에서의 arm) 갈래
- assertion: 어서션, 단언
- assign: 대입하다
- associated function: 연관함수
- associated type: 연관 타입
- binary: 바이너리
- bind: 묶다
- boilerplate code: 보일러플레이트 코드
- borrowing: 빌림
- box: 박스
- CamelCase: 낙타 표기법
- cargo: 카고
- clone: 클론
- collection: 컬렉션
- crate: 크레이트
- copy: 복사
- crate: 크래이트
- dangling reference: 댕글링 참조자
- data race: 데이터 레이스
- deep copy: 깊은 복사
- dependency: 디펜던시
- deref coercion: 역참조 강제
- derived trait: 파생 트레잇
- documentation comments: 문서화 주석
- double free: 중복 해제
- `drop`: `drop`한다, 버리다
- feature: 특성
- format string: 형식 문자열
- generic: 제네릭
- handle: 핸들
- heap: 힙
- identifier: 식별자
- immutable: 불변
- interior mutability: 내부 가변성
- instance: 인스턴스
- integration test: 통합 테스트
- irrefutable pattern: 반증 불가 패턴
- iterator: 반복자
- lifetime: 라이프타임
- library: 라이브러리
- module: 모듈
- monomorphization: 단형성화
- move: 이동
- mutable: 가변
- namespace: 이름공간
- orphan rule: 고아 규칙
- ownership: 소유권
- placeholder: 변경자
- pointer: 포인터
- prelude: 프렐루드
- private: 비공개
- public: 공개
- recover: 복구
- recursive type: 재귀적 타입
- reference: 참조자
- reference counting: 참조 카운팅
- reference cycle: 참조 순환
- refutable pattern: 반증 가능 패턴
- return: 반환
- rust: 러스트
- scope: 스코프
- slice: 슬라이스
- stack: 스택
- signature: 시그니처
- string literal: 스트링 리터럴
- test harness: 테스트 도구
- trait bound: 트레잇 바운드
- trait object: 트레잇 객체
- type annotation: 타입 명시
- unit test: 단위 테스트
- variable: 변수
- visibility: 가시성

더 좋은 번역 용어가 있으신 분들 의견 주시면 반영하겠습니다!

---

# The Rust Programming Language

[![Build Status](https://travis-ci.org/rust-lang/book.svg?branch=master)](https://travis-ci.org/rust-lang/book)

This repo contains two editions of “The Rust Programming Language”.

The second edition is a rewrite that will be printed by NoStarch Press,
available around October 2017.

[You can read it online][html]; the last few chapters aren't completed yet, but
the first half of the book is much improved from the first edition. We recommend
starting with the second edition.

[html]: http://rust-lang.github.io/book/

[The first edition is still available to read online][first].

[first]: https://doc.rust-lang.org/book/

## Requirements

Building the book requires [mdBook] >= v0.0.13. To get it:

[mdBook]: https://github.com/azerupi/mdBook

```bash
$ cargo install mdbook
```

## Building

To build the book, first `cd` into either the `first-edition` or
`second-edition` directory depending on which edition of the book you would
like to build. Then type:

```bash
$ mdbook build
```

The output will be in the `book` subdirectory. To check it out, open it in
your web browser.

_Firefox:_
```bash
$ firefox book/index.html                       # Linux
$ open -a "Firefox" book/index.html             # OS X
$ Start-Process "firefox.exe" .\book\index.html # Windows (PowerShell)
$ start firefox.exe .\book\index.html           # Windows (Cmd)
```

_Chrome:_
```bash
$ google-chrome book/index.html                 # Linux
$ open -a "Google Chrome" book/index.html       # OS X
$ Start-Process "chrome.exe" .\book\index.html  # Windows (PowerShell)
$ start chrome.exe .\book\index.html            # Windows (Cmd)
```

To run the tests:

```bash
$ mdbook test
```

## Contributing

We'd love your help! Please see [CONTRIBUTING.md][contrib] to learn about the
kinds of contributions we're looking for.

[contrib]: https://github.com/rust-lang/book/blob/master/CONTRIBUTING.md

### Translations

We'd especially love help translating the second edition of the book! See the
[Translations] label to join in efforts that are currently in progress. Open
a new issue to start working on a new language! We're waiting on [mdbook
support] for multiple languages before we merge any in, but feel free to
start! The chapters in [the frozen column] of the project won't see major
changes, so if you start with those, you won't have to redo work :)

[Translations]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook support]: https://github.com/azerupi/mdBook/issues/5
[the frozen column]: https://github.com/rust-lang/book/projects/1

## No Starch

As the second edition of the book will be published by No Starch, we first
iterate here, then ship the text off to No Starch. Then they do editing, and we
fold it back in.

As such, there’s a directory, *nostarch*, which corresponds to the text in No
Starch’s system.

When we've started working with No Starch in a word doc, we will also check
those into the repo in the *nostarch/odt* directory. To extract the text from
the word doc as markdown in order to backport changes to the online book:

1. Open the doc file in LibreOffice
1. Accept all tracked changes
1. Save as Microsoft Word 2007-2013 XML (.docx) in the *tmp* directory
1. Run `./doc-to-md.sh`
1. Inspect changes made to the markdown file in the *nostarch* directory and
   copy the changes to the *src* directory as appropriate.

## Graphviz dot

This is mostly for Carol's reference because she keeps having to look it up.

We're using [Graphviz](http://graphviz.org/) for some of the diagrams in the
book. The source for those files live in the `dot` directory. To turn a `dot`
file, for example, `dot/trpl04-01.dot` into an `svg`, run:

```bash
$ dot dot/trpl04-01.dot -Tsvg > src/img/trpl04-01.svg
```

In the generated SVG, remove the width and the height attributes from the `svg`
element and set the `viewBox` attribute to `0.00 0.00 1000.00 1000.00` or other
values that don't cut off the image.

## Spellchecking

To scan source files for spelling errors, you can use the `spellcheck.sh`
script. It needs a dictionary of valid words, which is provided in
`dictionary.txt`. If the script produces a false positive (say, you used word
`BTreeMap` which the script considers invalid), you need to add this word to
`dictionary.txt` (keep the sorted order for consistency).
