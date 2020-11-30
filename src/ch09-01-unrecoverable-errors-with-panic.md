## 복구 불가능한 에러에는 `panic!`!

가끔 여러분의 코드에서 나쁜 일이 일어나고, 이에 여러분이 할 수 있는 것이 없을 수도 있습니다.
이러한 경우를 위하여 러스트는 `panic!` 매크로를 가지고 있습니다.
이 매크로가 실행되면, 여러분의 프로그램은 실패 메세지를 출력하고,
스택을 되감고 정리하고, 종료됩니다.
이런 상황은 주로 프로그래머가 해당 에러를 처리할 방법이 마땅치 않을 때입니다.

> ### `panic!`에 응하여 스택을 되감거나 그만두기
>
> 기본적으로, `panic!`이 발생하면, 프로그램은 *되감기(unwinding)* 를 시작하는데,
> 이는 러스트가 패닉을 마주친 각 함수로부터 스택을 거꾸로 훑어가면서 데이터를
> 제거한다는 뜻입니다. 하지만, 이는 간단한 작업이 아닙니다. 다른 대안으로는
> 즉시 *그만두기(abort)* 가 있는데, 이는 데이터 제거 없이 프로그램을 끝내는 것입니다.
> 이때 프로그램이 사용하고 있던 메모리는 운영체제가 청소해 주어야 합니다.
> 여러분의 프로젝트 내에서 결과 바이너리를 가능한 한 작게 만들고 싶다면,
> 여러분의 *Cargo.toml* 내에서 적합한 `[profile]` 섹션에 `panic = 'abort'`를
> 추가함으로써 unwinding을 abort로 바꿀 수 있습니다.
> 예를 들어, 여러분이 릴리즈 모드에서는 패닉시 abort 방식을
> 쓰고 싶다면, 다음을 추가하세요:
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

단순한 프로그램 내에서 `panic!`을 호출해 봅시다:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic,panics
fn main() {
    panic!("crash and burn");
}
```

프로그램을 실행하면, 다음과 같은 내용이 나타납니다:

```text
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

`panic!`의 호출이 마지막 두 줄의 에러 메세지를 일으킵니다.
첫 번째 줄은 우리가 작성한 패닉 메세지와 소스 코드에서 패닉이
발생한 지점을 보여줍니다. *src/main.rs:2:5*는 *src/main.rs* 파일의
두 번째 줄 다섯 번째 문자를 나타냅니다.

이 예제에서는, 표시된 줄이 우리가 작성한 코드 부분이고, 해당 줄에서
`panic!` 매크로 호출을 눈으로 직접 볼 수 있습니다. 그 외의 경우들에서는,
`panic!` 호출이 우리가 호출한 외부 코드에 있을 수도 있습니다. 에러 메세지에 의해
보고되는 파일 이름과 라인 번호는 `panic!` 매크로가 호출된 다른 누군가의 코드일 것이며,
궁극적으로 `panic!`을 이끌어낸 것이 우리 코드 라인이 아닐 것입니다.
문제를 일으킨 코드 부분을 발견하기 위해서 `panic!` 호출이 발생한 함수에 대한
백트레이스(backtrace)를 사용할 수 있습니다. 백트레이스가 무엇인가에 대해서는
뒤에 더 자세히 다를 것입니다.

### `panic!` 백트레이스 이용하기

다른 예제를 통해서, 우리 코드가 직접 매크로를 호출하는 대신 우리 코드의
버그 때문에 `panic!` 호출이 라이브러리로부터 발생할 때는 어떻게 되는지 살펴봅시다.
Listing 9-1은 벡터 내의 요소를 인덱스로
접근 시도하는 코드입니다:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic,panics
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

<span class="caption">Listing 9-1: `panic!`을 일으키는 벡터의 끝을 넘어선
요소에 대한 접근 시도</span>

여기서 우리는 벡터의 100번째 요소(0부터 시작하므로 99입니다)에 접근하기를
시도하고 있지만, 벡터는 단 3개의 요소만 가지고 있습니다. 이 경우 러스트는
패닉을 일으킬 것입니다. `[]`를 사용하는 것은 어떤 요소를 반환하기를 가정하지만,
유효하지 않은 인덱스를 넘기게 되면 러스트가 반환할 올바른 요소는
없습니다.

이러한 상황에서 C와 같은 다른 언어들은 여러분이 원하는 것이 아닐지라도,
여러분이 요청한 것을 정확히 주려고 시도할 것입니다. 여러분은 벡터 내에
해당 요소와 상응하는 위치의 메모리에 들어 있는 무언가를 얻을 것입니다.
설령 그 메모리 영역이 벡터 소유가 아닐지라도 말이죠. 이러한 것을
*버퍼 오버리드(buffer overread)* 라 하며, 만일 어떤 공격자가 읽도록
허용되어선 안 되지만 배열 뒤에 저장된 데이터를 읽어낼 방법으로서
인덱스를 다룰 수 있게 된다면, 이는 보안 취약점을 발생시킬 수 있습니다.

여러분의 프로그램을 이러한 취약점으로부터 보호하기 위해서, 존재하지 않는
인덱스 상의 요소를 읽으려 시도한다면, 러스트는 실행을 멈추고 계속하기를
거부할 것입니다. 한번 시도해 봅시다:

```text
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', libcore/slice/mod.rs:2448:10
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

위 에러는 우리가 작성하지 않은 파일인 *libcore/slice/mod.rs* 를 가리키고 있습니다.
이는 러스트 자체 코드 내에 있는 `slice` 의 구현 부분입니다. 우리가 벡터 `v`에
`[]`를 사용할 때 실행되는 코드는 *libcore/slice/mod.rs* 안에 있으며,
그곳이 바로 `panic!`이 실제 발생한 곳입니다.

그다음 줄은 `RUST_BACKTRACE` 환경 변수를 설정하여 에러의 원인이 된 것이 무엇인지
정확하게 백트레이스할 수 있다고 말해주고 있습니다. *백트레이스 (backtrace)* 란
어떤 지점에 도달하기까지 호출한 모든 함수의 목록을 말합니다.
러스트의 백트레이스는 다른 언어들에서와 마찬가지로 동작합니다. 백트레이스를
읽는 요령은 위에서부터 시작하여 여러분이 작성한 파일이 보일 때까지 읽는 것입니다.
그곳이 바로 문제를 읽으킨 지점입니다. 여러분의 파일이 나타난 줄보다 위에 있는 줄은
여러분의 코드가 호출한 코드이고, 아래의 코드는 여러분의 코드를 호출한 코드입니다.
이 목록에는 핵심(core) 러스트 코드, 표준 라이브러리, 여러분이 이용하고 있는
크레이트가 포함될 수 있습니다. 한번 `RUST_BACKTRACE` 환경변수를 0이 아닌 값으로
설정하여 백트레이스를 얻어봅시다. Listing 9-2는 여러분이 보게 될 것과
유사한 출력을 나타냅니다.

```text
$ RUST_BACKTRACE=1 cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', libcore/slice/mod.rs:2448:10
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: <std::panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::get
             at libstd/panicking.rs:476
   5: std::panicking::continue_panic_fmt
             at libstd/panicking.rs:390
   6: std::panicking::try::do_call
             at libstd/panicking.rs:325
   7: core::ptr::drop_in_place
             at libcore/panicking.rs:77
   8: core::ptr::drop_in_place
             at libcore/panicking.rs:59
   9: <usize as core::slice::SliceIndex<[T]>>::index
             at libcore/slice/mod.rs:2448
  10: core::slice::<impl core::ops::index::Index<I> for [T]>::index
             at libcore/slice/mod.rs:2316
  11: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
             at liballoc/vec.rs:1653
  12: panic::main
             at src/main.rs:4
  13: std::rt::lang_start::{{closure}}
             at libstd/rt.rs:74
  14: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  15: macho_symbol_search
             at libpanic_unwind/lib.rs:102
  16: std::alloc::default_alloc_error_hook
             at libstd/panicking.rs:289
             at libstd/panic.rs:392
             at libstd/rt.rs:58
  17: std::rt::lang_start
             at libstd/rt.rs:74
  18: panic::main
```

<span class="caption">Listing 9-2: 환경 변수 `RUST_BACKTRACE`가 설정되었을 때 `panic!`의
호출에 의해 발생하는 백트레이스 출력</span>

출력이 엄청 많군요! 여러분이 보는 실제 출력값은 여러분의 운영 체제 및
러스트 버전에 따라 다를 수 있습니다. 이러한 정보들과 함께 백트레이스를
얻기 위해서는 디버그 심볼이 활성화되어 있어야 합니다. 디버그 심볼은 여기서처럼
여러분이 `cargo build`나 `cargo run`을 `--release` 플래그 없이 실행했을 때
기본적으로 활성화됩니다.

Listing 9-2 출력 내용은, 백트레이스의 12번 줄이 문제를 일으킨
우리 프로젝트의 *src/main.rs*, 4번 줄을 가리키고 있습니다.
만일 프로그램이 패닉에 빠지지 않도록 하고 싶다면, 우리가 작성한 파일이
언급된 첫 줄부터 조사해봅시다.
백트레이스 사용법 시범을 보이기 위해 고의로 패닉을 일으키도록 코드를 작성한
Listing 9-1에서 패닉을 고칠 방법은, 단 3개의 아이템을 가진 벡터로부터
인덱스 100에서의 요소를 요청하지 않도록 하는 것입니다.
추후 여러분의 코드에서 패닉이 발생할 때, 여러분은 어떤 코드가 패닉을 일으키는지,
코드를 어떻게 고쳐야 할지를 알아야 합니다.

다음은 에러가 발생했을 때 `Result`를 이용하여 복구하는 방법을 살펴보겠습니다.
언제 `panic!`을 써야 하는지, 혹은 쓰지 말아야 하는지에 대해서는
그다음에 나올 ["panic!이냐, panic!이 아니냐, 그것이 문제로다"][to-panic-or-not-to-panic]<!-- ignore -->
에서 알아볼 예정입니다.

[to-panic-or-not-to-panic]:
ch09-03-to-panic-or-not-to-panic.html%23panic%21%EC%9D%B4%EB%83%90%2C%20panic%21%EC%9D%B4%20%EC%95%84%EB%8B%88%EB%83%90%2C%20%EA%B7%B8%EA%B2%83%EC%9D%B4%20%EB%AC%B8%EC%A0%9C%EB%A1%9C%EB%8B%A4
