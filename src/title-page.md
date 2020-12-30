# The Rust Programming Language

*이 책의 원문은 Steve Klabnik 와 Carol Nichols 가 집필했으며, 기여해주신 러스트 커뮤니티 여러분과 한국어 번역에 참여해주신 분들께 감사드립니다.*

책을 보시기에 앞서, 현재 버전의 책에서는 Rust(이하 러스트) 1.48 이상 버전을
사용하며 각 프로젝트는 *Cargo.toml* 에 `edition="2018"` 을 명시하여
러스트 2018 edition 문법을 사용하였음을 알려드립니다.
러스트 설치 및 업데이트 방법이나 러스트 에디션에 관한 설명은
각각 [1장 - "설치 방법"][install]<!-- ignore --> 과
[부록 E][editions]<!-- ignore --> 에서 찾아보실 수 있습니다.

러스트 2018 에디션은 기존 second 에디션에 비해 더 개발자 친화적이며,
진입장벽도 낮아졌습니다. 언어의 변화에 맞춰 이 책도 second 에디션에 맞춰
작성된 부분을 2018 에디션에 맞도록 수정했으며, 변경된 부분은 다음과 같습니다:

- 7장("거대해져가는 프로젝트를 패키지, 크레이트, 모듈로 나눠 관리하는 방법(Managing Growing Projects with Packages, Crates, and Modules)")은
  러스트 2018 에디션이 나오면서 경로 작동 방식 및 모듈 시스템이 보다
  일관적이게 변경되었기 때문에 거의 완전히 새로 작성했습니다.
- 새로 추가된 `impl Trait` 문법을 소개하는 "매개변수로서의 트레잇(Traits as Parameters)" 과
  "트레잇을 구현하는 타입 반환하기(Returning Types that Implement Traits)" 절을 10장에 추가했습니다.
- 테스트 내에서 `?` 연산자를 사용할 수 있도록 하는 방법을
  보여주는 "테스트에서 `Result<T, E>` 사용하기"라는 제목의 절을 11장에 추가했습니다.
- 기존 19장 "고급 라이프타임" 섹션은 그간 컴파일러 개선으로 인해
  해당 내용의 활용도가 줄어들어, 섹션을 통째로 삭제하기로 결정했습니다.
- 기존의 부록 D 에 위치하던 매크로 관련 내용은
  19장의 "매크로" 절로 이동되었습니다.
- 러스트 2015로 작성한 코드와 러스트 2018로 작성한 코드 간 호환을
  위해 새로이 추가된 raw 식별자를 부록 A "키워드" 에 추가했습니다.
- 부록 D가 "유용한 개발 도구들" 이라는 이름이 되어 여러분이 러스트 코드를 작성하는데
  도움될 최신 출시 도구를 다룹니다.
- 내용상 몇 가지 자잘한 오류나 애매한 표현을 수정했습니다.
  제보해주신 독자분들 정말 감사합니다!

여러분이 러스트 컴파일러 버전을 업데이트해도
기존 *The Rust Programming Language* 에 작성되어있던 코드는
해당 프로젝트 *Cargo.toml* 에 `edition="2018"` 을 명시하지
않는 이상 앞으로도 작동할 겁니다. 러스트의 버전 안정성 보장 덕분이죠.

이 책은 온라인, 오프라인 모두 제공됩니다.
온라인엔 원본(영문) 이외에도 번역본이 존재하며, 각각 [https://doc.rust-lang.org/stable/book/](https://doc.rust-lang.org/stable/book/)(영문)과
[https://rinthel.github.io/rust-lang-book-ko/](https://rinthel.github.io/rust-lang-book-ko/)(번역본)에서 읽어보실 수 있습니다.
오프라인 본(영문)은 설치되어있는 `rustup` 의 `rustup docs --book` 명령어로 열어보실 수 있습니다.

[No Starch Press][nsprust] 에서는 영문 원서가
종이책 및 ebook으로 제공됩니다.

[install]: ch01-01-installation.html
[editions]: appendix-05-editions.html
[nsprust]: https://nostarch.com/rust
