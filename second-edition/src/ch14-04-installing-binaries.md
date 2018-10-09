## `cargo install` 을 이용해 Crates.io 에서 바이너리 설치하기

`cargo install` 명령어는 여러분이 로컬에서 바이너리 크레이트를 설치하고 사용할 수 있도록 해줍니다.
이는 시스템 패키지를 대체하기 위한 것이 아닌, 러스트 개발자들이
[crates.io](https://crates.io)<!-- ignore --> 에서 공유하고
있는 툴을 편리하게 설치할 수 있도록 하기 위함입니다.
여러분은 *바이너리 타겟(binary target)* 을 가진 패키지만 설치할 수 있다는 걸
알아두셔야 하는데, 이 *바이너리 타겟* 이란 혼자서 실행될 수 없고 다른
프로그램에 포함되는 용도인 라이브러리 타겟과는 반대되는 의미로, *src/main.rs*
파일 혹은 따로 바이너리로 지정된 파일을 가진 크레이트가 생성해낸 실행 가능한
프로그램을 말합니다. 보통 해당 크레이트가 라이브러리인지, 바이너리 타겟을 갖는지,
혹은 둘 다인지에 대한 정보를 *README* 파일에 작성해둡니다.

`cargo install` 을 이용해 설치한 모든 바이너리들은 Cargo가 설치된 폴더의 *bin*
폴더에 저장됩니다. 만약 여러분이 *rustup.rs* 를 이용해 러스트를 설치하셨고,
따로 설정을 건들지 않으셨다면 *$HOME/.cargo/bin* 폴더입니다.
`cargo install` 로 설치한 프로그램을 실행하시려면 여러분의 `$PATH` 환경변수에
해당 디렉토리가 등록되어 있는지 확인하세요.

12 장에서 언급한 `grep` 을 러스트로 구현한
파일 검색 툴인 `ripgrep` 을 예로 들어봅시다.
`ripgrep` 을 설치하려면 다음과 같이 하면 됩니다:

```text
$ cargo install ripgrep
Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading ripgrep v0.3.2
 --snip--
   Compiling ripgrep v0.3.2
    Finished release [optimized + debuginfo] target(s) in 97.91 secs
  Installing ~/.cargo/bin/rg
```

출력의 마지막 줄은 설치된 바이너리의 경로와 이름을 보여줍니다. `ripgrep` 의
이름은 `rg` 네요. 방금 앞에서 말했던 것처럼 여러분의 `$PATH` 환경변수에 설치된
폴더가 등록되어 있는 한, 여러분은 명령창에서 `rg --help` 를 실행할 수 있고,
앞으로 파일을 찾을때 더 빠르고 러스트다운 툴을 사용할 수 있습니다!
