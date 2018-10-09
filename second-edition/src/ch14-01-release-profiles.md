# 릴리즈 프로필을 이용해 빌드 커스터마이징하기

러스트에서 *릴리즈 프로필(release profiles)* 은 프로그래머가
코드 컴파일에 관련된 여러가지 옵션을 제어할 수 있도록
다양한 구성으로 사전 정의되고 커스텀 가능한 프로필입니다.
각 프로필은 다른 프로필과 독립적으로 설정됩니다.

Cargo 는 두 메인 프로필을 가집니다: 여러분이 `cargo build` 를 실행할때
쓰는`dev` 프로필과 `cargo build --release` 를 실행할때 쓰는 `release` 프로필
입니다. `dev` 프로필은 개발에 적합한 설정을 기본값으로 갖고, `release`
프로필은 릴리즈 빌드용 설정을 기본값으로 가집니다.

여러분은 빌드 출력에서 이 프로필들의 이름을 몇 번 보셨을 수도 있습니다.

```text
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
$ cargo build --release
    Finished release [optimized] target(s) in 0.0 secs
```

위 출력의 `dev` 와 `release` 는 컴파일러가 다른 프로필을 사용한다는 것을
나타냅니다.

Cargo 는 프로젝트의 *Cargo.toml* 파일에 `[profile.*]` 구획이 따로 없을때
적용되는 각 프로필의 기본 설정을 가지고 있습니다. 이때 여러분은 원하는
프로필에 `[profile.*]` 구획을 추가하여 기본 설정을 덮어 씌울 수 있습니다.
여기 예시로 `dev` 와 `release` 프로필 각각의 `opt-level` 기본 설정 값을
보여드리겠습니다.

<span class="filename">Filename: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

`opt-level` 설정은 러스트가 여러분의 코드에 적용할 최적화 수치이며, 0 ~ 3
사이의 값을 가집니다. 여러분이 개발을 할 때와 같이 코드를 자주 컴파일 하는
상황에서는 코드의 실행 속도가 조금 느려지는 한이 있더라도 컴파일이 빨리 되길
원합니다. 하지만 높은 최적화 수치를 적용 할 수록 컴파일에 걸리는 시간은
증가합니다. 따라서 `dev` 의 기본 `opt-level` 값은 `0` 으로 되어 있습니다.
만약 여러분이 코드를 릴리즈 하려 한다면, 컴파일에 걸리는 시간이 늘어나도
상관이 없을 겁니다. 릴리즈 할 경우 컴파일은 한 번이지만, 실행 횟수는 여러번
이니까요. 따라서 릴리즈 모드에서는 컴파일 시간을 희생하는 대신 빠른 코드 실행 속도를
얻기 위해 `release` 프로필의 기본 `opt-level` 값이 `3` 으로 되어 있습니다.

이전에 말했듯, 여러분은 *Cargo.toml* 에 다른 값을 넣어서 기본 설정을 덮어
씌울 수 있습니다. 예를 들어 만약 우리가 개발용 프로필에 0 이 아닌 1 의 최적화
수치를 적용하고 싶다면 우리 프로젝트의 *Cargo.toml* 에 다음 두 줄을 추가하면
됩니다:

<span class="filename">Filename: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 1
```

이 코드는 기본 설정인 `0` 을 덮어 씌웁니다. 이후에 우리가 `cargo build` 를
실행하면 Cargo 는 `dev` 프로필의 기본값과 우리가 커스텀 한 `opt-level` 을
사용합니다. 우리가 `opt-level` 을 `1` 로 설정 했기 때문에 Cargo 는 릴리즈
빌드 만큼은 아니지만 기본 설정 보다 많은 최적화를 진행할 겁니다.

각 프로필의 설정 옵션 및 기본값의 전체 목록을 보시려면
[Cargo 공식 문서](https://doc.rust-lang.org/cargo/) 를 참고해 주시기 바랍니다.
