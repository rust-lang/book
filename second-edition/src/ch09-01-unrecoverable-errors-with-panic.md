## Необрабатываемые ошибки с помощь макроса `panic!`

Бывает, что ошибки случаются и ничего с этим нельзя поделать. В таких случаях Rust
предлагает использовать макрос `panic!`. Когда этот макрос выполняется программа
печатает сообщене об ошибке, очищается стеки данных и затем программа завершает свою
работу. Весьма часто бывает, что нельзя предугадать появление ошибки.


> ### Unwinding the Stack Versus Aborting on Panic
> По умолчанию, когда срабатывает макрос `panic!`, программа входит в определенное
> состояние, при котором очищаются стеки и данные каждой функции. Происходит много
> служебных действий, гарантирующих удаление устаревших данных, очисти буферов и пр.
> Есть также возможность просто мгновенно прервать работу программы без очистки
> буферов, данных. При этом очистка буферов, данных ложиться на плечи операционной
> системы. Самый простой вариант работы программы - это простое прерывание. При этом
> программа будет иметь минимальных код.  Для этого просто добавьте текст `panic = 'abort'`
> в соответствующую секцию `[profile]` файла конфигурации *Cargo.toml*.
> Например, если вы хотите прерывания в релизных версиях вашей программы:
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

Вызов макроса `panic!` в программном коде:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
fn main() {
    panic!("crash and burn");
}
```

Строка вывода:

```text
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25 secs
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/panic` (exit code: 101)
```

### Использование информационных сообщений макроса `panic!`

Рассмотрим пример, где макрос `panic!` вызывается из библиотечных функций. В данном
примере ошибка в коде программы:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
fn main() {
    let v = vec![1, 2, 3];

    v[100];
}
```
Попытка доступа к несуществующему элементу привела к ошибке.

В таких языках, как C подобная ошибка приводит к переполнению буфера.

Для защиты от подобного рода ошибок в Rust останавливается работа программы.

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
Здесь приводится ссылка на файл из стандартной библиотеки *libcollections/vec.rs*.
Это реализация `Vec<T>`.
`RUST_BACKTRACE` - это переменная системы. Если она установлена - происходит оповещение
о ошибке.

```text
$ set RUST_BACKTRACE=1 &&cargo run
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

<span class="caption">Listing 9-1: Подробное сообщение об ошибке, когда переменная
`RUST_BACKTRACE` установлена</span>

Здесь мы видим описание всех функций, которые связана с данной проблемой.
