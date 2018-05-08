# I/O 프로젝트: 커맨드 라인 프로그램 만들기

<!-- We might need a more descriptive title, something that captures the new
elements we're introducing -- are we going to cover things like environment
variables more in later chapters, or is this the only place we explain how to
use them? -->

<!-- This is the only place we were planning on explaining both environment
variables and printing to standard error. These are things that people commonly
want to know how to do in Rust, but there's not much more than what we've said
here about them, people just want to know how to do them in Rust. We realize
that those sections make this chapter long, but think it's worth it to include
information that people want. We've gotten really positive feedback from people
who have read this chapter online; people who like learning through projects
have really enjoyed this chapter. /Carol-->

이 장에서 우리는 지금까지 배운 많은 내용을 요약 정리하고 몇 가지 표준 라이브러리 기능을 탐색하고자 합니다. 현재 
우리가 보유한 러스트 실력을 연습하기 위한 커맨드 라인 툴을 만들고 파일, 커맨드 라인 입출력 작업을 해보게 될 것 
입니다.

러스트는 성능, 안전성, '단일 바이너리'로 출력, 그리고 교차 플랫폼 지원으로 커맨드 라인 툴을 제작하기 좋은 언어입니다.
그러니 우리는 고전적인 커맨드 라인 툴 `grep`을 우리 자체 버전으로 만들어 볼 것입니다. Grep은 "정규 표현식 검색
및 인쇄"의 약어 입니다. `grep`의 간단한 사용 예로 다음의 단계를 거쳐 지정된 파일에서 지정된 문자를 검색합니다. 

- 인자로 파일 이름과 문자를 취합니다. 
- 파일을 읽어들입니다.
- 문자 인자를 포함하는 파일의 행들을 찾습니다. 
- 해당 라인들을 표시합니다. 

우리는 또한 환경 변수를 사용하는 방법과 표준 출력 대신 표준 에러로 표시하는 방법을 다루고자 합니다. 이러한 기법들은 
일반적으로 커맨드 라인 도구들에서 사용됩니다. 

한 러스트 커뮤니티 멤버인 Andrew Gallant가 이미 `grep`의 전체 기능이 구현됐으면서도 월등히 빠른 
`ripgrep`을 만들었습니다. 이에 비해 우리의 `grep`은 훨씬 간단하게 만들 것 입니다, 이번 장에서 
`ripgrep`과 같은 실제 프로젝트를 이해하는데 필요한 배경지식을 제공합니다. 

이 프로젝트는 우리가 지금까지 학습한 다양한 개념을 종합하게 될 겁니다:

- 구조척 코드 (7장 모듈 편에서 배운 내용)
- 벡터와 문자열의 사용 (8장 콜렉션)
- 에러 처리 (9장)
- 특성과 생명주기를 적절히 사용하기 (10장)
- 테스트 작성 (11장)

또한 우리는 클로저, 반복자, 특성 개체를 간단히 소개하고자 합니다. 이는 13장과 17장에서 상세히 다룰 겁니다.

언제나처럼 `cargo new`를 통해 새로운 프로젝트를 생성합시다. 새 프로젝트의 이름을 `greprs`로 이름 지어서
시스템에 이미 존재하는 `grep`와 구분짓도록 하겠습니다:


```text
$ cargo new --bin greprs
     Created binary (application) `greprs` project
$ cd greprs
```

<!-- 업데이트된 원본:
# An I/O Project: Building a Command Line Program

This chapter is a recap of the many skills you’ve learned so far and an
exploration of a few more standard library features. We’ll build a command line
tool that interacts with file and command line input/output to practice some of
the Rust concepts you now have under your belt.

Rust’s speed, safety, single binary output, and cross-platform support make it
an ideal language for creating command line tools, so for our project, we’ll
make our own version of the classic command line tool `grep` (**g**lobally
search a **r**egular **e**xpression and **p**rint). In the simplest use case,
`grep` searches a specified file for a specified string. To do so, `grep` takes
as its arguments a filename and a string. Then it reads the file, finds lines
in that file that contain the string argument, and prints those lines.

Along the way, we’ll show how to make our command line tool use features of the
terminal that many command line tools use. We’ll read the value of an
environment variable to allow the user to configure the behavior of our tool.
We’ll also print to the standard error console stream (`stderr`) instead of
standard output (`stdout`), so, for example, the user can redirect successful
output to a file while still seeing error messages onscreen.

One Rust community member, Andrew Gallant, has already created a fully
featured, very fast version of `grep`, called `ripgrep`. By comparison, our
version of `grep` will be fairly simple, but this chapter will give you some of
the background knowledge you need to understand a real-world project such as
`ripgrep`.

Our `grep` project will combine a number of concepts you’ve learned so far:

* Organizing code (using what you learned in modules, Chapter 7)
* Using vectors and strings (collections, Chapter 8)
* Handling errors (Chapter 9)
* Using traits and lifetimes where appropriate (Chapter 10)
* Writing tests (Chapter 11)

We’ll also briefly introduce closures, iterators, and trait objects, which
Chapters 13 and 17 will cover in detail.
-->

