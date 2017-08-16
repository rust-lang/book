## `panic!`과 함께하는 회복 불가능한 에러

가끔씩 나쁜 일은 일어나고, 이에 대해 여러분이 할 수 있는 것이 없을 수도 있습니다. 이러한 경우를 위하여
러스트는 `panic!` 매크로를 가지고 있습니다. 이 매크로가 실행되면, 여러분의 프로그램은 실패 메세지를
출력하고, 스택을 되감고 청소하고, 그후 종료됩니다. 이런 일이 발생하는 가장 흔한 상황은 어떤 종류의
버그가 발견되었고 프로그래머가 이 에러를 어떻게 처리할지가 명확하지 않을 때 입니다. 

> ### 패닉 상에서 스택 되감기 v.s. 그만두기
>
> 기본적으로, `panic!`이 발생하면, 프로그램은 *되감기(unwinding)* 를 시작하는데, 이는 러스트가
> 패닉을 마주친 각 함수로부터 스택을 거꾸로 훑어가면서 데이터를 제거한다는 뜻이지만, 이 훑어가기 및
> 제거는 일이 많습니다. 다른 대안으로는 즉시 *그만두기(about)* 가 있는데, 이는 데이터 제거 없이
> 프로그램을 끝내는 것입니다. 프로그램이 사용하고 있던 메모리는 운영체제에 의해 청소될 필요가 있을
> 것입니다. 여러분의 프로젝트 내에서 결과 바이너리가 가능한 작아지기를 원한다면, 여러분의
> *Cargo.toml* 내에서 적합한 `[profile]` 섹션에 `panic = 'abort'`를 추가함으로써 되감기를
> 그만두기로 바꿀 수 있습니다. 예를 들면, 여러분이 릴리즈 모드 내에서는 패닉 상에서 그만두기를
> 쓰고 싶다면 아래와 같이 합니다:
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

단순한 프로그램으로 `panic!` 호출을 시도해 봅시다:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
fn main() {
    panic!("crash and burn");
}
```

이걸 실행하면 다음과 같은 것을 보게 될 것입니다:

```text
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25 secs
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/panic` (exit code: 101)
```

마지막 세 줄이 `panic!`을 호출함으로 인해 생긴 에러 메세지를 담고 있습니다. 첫번째 줄은 우리의 패닉
메세지와 소스 코드에서 패닉이 발생한 지점을 보여줍니다: *src/main.rs:2*는 *src/main.rs* 파일의
두번째 줄을 가리킵니다.

위 예제의 경우, 가리키고 있는 줄은 우리 코드 부분이고, 해당 줄로 가면 `panic!` 매크로 호출을 보게 됩니다.
그 외의 경우들에서는, `panic!` 호출이 우리 코드가 호출한 코드 내에 있을 수도 있습니다. 에러 메세지에
의해 보고되는 파일 이름과 라인 번호는 `panic!` 매크로가 호출된 다른 누군가의 코드일 것이며, 궁극적으로
`panic!`을 이끌어낸 것이 우리 코드 라인이 아닐 것입니다. 이를 발견하기 위해서 `panic!` 호출이
발생된 함수에 대한 백트레이스(backtrace)를 사용할 수 있습니다.

### `panic!` 백트레이스 사용하기

다른 예를 통해서, 우리 코드가 직접 매크로를 호출하는 대신 우리 코드의 버그 때문에 `panic!` 호출이
라이브러리로부터 발생될 때는 어떻게 되는지 살펴봅시다:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
fn main() {
    let v = vec![1, 2, 3];

    v[100];
}
```

우리는 벡터의 100번째 요소에 접근하기를 시도하고 있지만, 벡터는 오직 3개의 요소만 가지고 있습니다.
이러한 상황이면 러스트는 패닉을 일으킬 것입니다. `[]`를 사용하는 것은 어떤 요소를 반환하기를 가정하지만,
유효하지 않은 인덱스를 넘기게 되면 러스트가 반환할 올바른 요소는 없습니다.

이러한 상황에서 C 같은 다른 언어들은 여러분이 원하는 것이 아닐지라도, 여러분이 요청한 것을 정확히 주려고
시도할 것입니다: 여러분은 벡터 내에 해당 요소와 상응하는 위치의 메모리에 들어있는 무언가를 얻을 것입니다.
설령 그 메모리 영역이 벡터 소유가 아닐지라도 말이죠. 이러한 것을 *버퍼 오버리드(buffer overread)*
라고 부르며, 만일 어떤 공격자가 읽도록 허용되어선 안되지만 배열 뒤에 저장되어 있는 데이터를 읽어낼
방법으로서 인덱스를 다룰 수 있게 된다면, 이는 보안 취약점을 발생시킬 수 있습니다.

여러분의 프로그램을 이러한 종류의 취약점으로부터 보호하기 위해서, 여러분이 존재하지 않는 인덱스 상의
요소를 읽으려 시도한다면, 려스트는 실행을 멈추고 계속하기를 거부할 것입니다. 한번 시도해 봅시다:

```text
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is
100', /stable-dist-rustc/build/src/libcollections/vec.rs:1362
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/panic` (exit code: 101)
```

위 에러는 우리가 작성하지 않은 파일인 *libcollections/vec.rs*를 가리키고 있습니다. 이는
표준 라이브러리 내에 있는 `Vec<T>`의 구현 부분입니다. 우리가 벡터 `v`에 `[]`를 사용할 때 실행되는
코드는 *libcollections/vec.rs* 안에 있으며, 그곳이 바로 `panic!`이 실제 발생한 곳입니다.

그 다음 노트는 `RUST_BACKTRACE` 환경 변수를 설정하여 에러의 원인이 된 것이 무엇인지 정확하게
백트레이스할 수 있다고 말해주고 있습니다. 이를 시도해봅시다. Listing 9-1은 결과를 보여줍니다:

```text
$ RUST_BACKTRACE=1 cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 100', /stable-dist-rustc/build/src/libcollections/vec.rs:1392
stack backtrace:
   1:     0x560ed90ec04c - std::sys::imp::backtrace::tracing::imp::write::hf33ae72d0baa11ed
                        at /stable-dist-rustc/build/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x560ed90ee03e - std::panicking::default_hook::{{closure}}::h59672b733cc6a455
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:351
   3:     0x560ed90edc44 - std::panicking::default_hook::h1670459d2f3f8843
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:367
   4:     0x560ed90ee41b - std::panicking::rust_panic_with_hook::hcf0ddb069e7abcd7
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:555
   5:     0x560ed90ee2b4 - std::panicking::begin_panic::hd6eb68e27bdf6140
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:517
   6:     0x560ed90ee1d9 - std::panicking::begin_panic_fmt::abcd5965948b877f8
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:501
   7:     0x560ed90ee167 - rust_begin_unwind
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:477
   8:     0x560ed911401d - core::panicking::panic_fmt::hc0f6d7b2c300cdd9
                        at /stable-dist-rustc/build/src/libcore/panicking.rs:69
   9:     0x560ed9113fc8 - core::panicking::panic_bounds_check::h02a4af86d01b3e96
                        at /stable-dist-rustc/build/src/libcore/panicking.rs:56
  10:     0x560ed90e71c5 - <collections::vec::Vec<T> as core::ops::Index<usize>>::index::h98abcd4e2a74c41
                        at /stable-dist-rustc/build/src/libcollections/vec.rs:1392
  11:     0x560ed90e727a - panic::main::h5d6b77c20526bc35
                        at /home/you/projects/panic/src/main.rs:4
  12:     0x560ed90f5d6a - __rust_maybe_catch_panic
                        at /stable-dist-rustc/build/src/libpanic_unwind/lib.rs:98
  13:     0x560ed90ee926 - std::rt::lang_start::hd7c880a37a646e81
                        at /stable-dist-rustc/build/src/libstd/panicking.rs:436
                        at /stable-dist-rustc/build/src/libstd/panic.rs:361
                        at /stable-dist-rustc/build/src/libstd/rt.rs:57
  14:     0x560ed90e7302 - main
  15:     0x7f0d53f16400 - __libc_start_main
  16:     0x560ed90e6659 - _start
  17:                0x0 - <unknown>
```

<span class="caption">Listing 9-1: 환경 변수 `RUST_BACKTRACE`가 설정되었을 때 `panic!`의
호출에 의해 발생되는 백트레이스 출력</span>

출력이 엄청 많군요! 백트레이스의 11번 라인이 문제를 일으킨 우리 프로젝트의 라인을 가리키고 있습니다:
*src/main.rs*, 4번 라인입니다. 백트레이스는 이 지점에서 호출되었던 모든 함수들의 리스트입니다.
러스트의 백트레이스는 다른 언어 내에서의 백트레이스와 비슷하게 동작합니다: 백트레이스를 읽는 열쇠는
여러분이 작성한 파일을 볼때까지 위에서부터 읽어내려가기 시작하는 것입니다. 그곳이 바로 문제가 시작된
지점입니다. 여러분의 파일이 언급된 라인의 윗 라인들은 여러분의 코드가 호출한 코드입니다; 밑의 라인들은
여러분의 코드를 호출한 코드입니다. 이 라인들은 핵심 러스트 코드, 표준 라이브러리 코드, 혹은 여러분이
사용중인 크레이트를 포함할 수도 있습니다.

만일 프로그램이 패닉에 빠지지 않도록 하고 싶다면, 우리가 작성한 파일이 언급된 첫 라인으로 지적된 위치가
바로 패닉을 일으킨 값을 가지고 있는 위치를 찾아내기 위해 수하기 시작할 지점입니다. 백트레이스를 어떻게
사용하는지 시범을 보이기 위해 고의로 패닉을 일으키는 코드를 작성한 우리의 예제에서, 패닉을 고칠 방법은
고작 3개의 아이템을 가진 벡터로부터 인덱스 100에서의 요소를 요청하지 않도록 하는 것입니다. 여러분의
코드가 추후 패닉에 빠졌을 때, 여러분의 특정한 경우에 대하여 어떤 코드가 패닉을 일으키는 값을 만드는지와
코드는 대신 어떻게 되어야 할지를 알아낼 필요가 있을 것입니다.

우리는 `panic!`으로 다시 돌아올 것이며 언제 이 메소드를 써야하는지, 혹은 쓰지 말아야 하는지에 대해
이 장의 뒷 부분에서 알아보겠습니다. 다음으로 `Result`를 이용한 에러로부터 어떻게 회복하는지를 보겠습니다.
