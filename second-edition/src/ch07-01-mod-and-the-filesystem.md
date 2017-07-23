## `mod`와 파일 시스템

먼저 카고를 이용해서 새로운 프로젝트를 만드는 것으로 모듈 예제를 시작하려고 하는데,
바이너리 크레잇(crate)을 만드는 대신에 라이브러리 크레잇을 만들 것입니다. 여기서
라이브러리 크레잇이란 다른 사람들이 자신들의 프로젝트에 디펜던시(dependency)로 추가할
수 있는 프로젝트를 말합니다. 예를 들어, 2장의 `rand` 크레잇은 우리가 추측 게임
프로젝트에서 디펜던시로 사용했던 라이브러리 크레잇입니다.

우리는 몇가지 일반적인 네트워크 기능을 제공하는 라이브러리의 뼈대를 만들 것입니다;
여기서는 모듈들과 함수들의 조직화에 집중할 것이고, 함수의 본체에 어떤 코드가 들어가야
하는지는 신경쓰지 않겠습니다. 이 라이브러리를 `communicator`라고 부르겠습니다.
기본적으로, 카고는 다른 타입의 프로젝트로 특정하지 않는 이상 라이브러리를 만들
것입니다: 이전의 모든 장들에서 사용해왔던 `--bin` 옵션을 제거하면, 프로젝트는
라이브러리가 될 것입니다:

```text
$ cargo new communicator
$ cd communicator
```

카고가 *src/main.rs* 대신 *src/lib.rs*을 생성했음을 주목하세요. *src/lib.rs*
내부를 보면 다음과 같은 코드를 찾을 수 있습니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

카고는 우리가 만든 라이브러리의 작성 시작을 돕기 위해 빈 테스트를 만드는데,
이는 `--bin` 옵션을 사용했을때 “Hello, world!” 바이너리를 만들어준 것과 사뭇
다릅니다. `#[]`와 `mod tests` 문법은 이 장의 “`super`를 이용하여 부모 모듈에
접근하기”절에서 더 자세히 다룰 것이지만, 당장은 *src/lib.rs*의 아래쪽에 이 코드를
남겨두겠습니다.

*src/main.rs* 파일이 없기 떄문에, `cargo run` 커맨드로 카고가 실행할 것이 없습니다.
따라서, 여기서는 라이브러리 크레잇의 코드를 컴파일하기 위해 `cargo build`를 사용할
것입니다.

이제 여러분이 작성하는 코드의 의도에 따라 만들어지는 다양한 상황에 알맞도록 라이브러리
코드를 조직화하는 다양한 옵션들을 살펴보겠습니다.

### 모듈 정의

우리의 `communicator` 네트워크 라이브러리를 위해서, 먼저 `connect`라는 이름의 함수가
정의되어 있는 `network`라는 이름의 모듈을 정의하겠습니다. 러스트 내 모듈 정의는 모두
`mod`로 시작됩니다. 이 코드를 *src/lib.rs*의 시작 부분, 즉 테스트 코드의 윗 쪽에
추가해봅시다:

<span class="filename">Filename: src/lib.rs</span>

```rust
mod network {
    fn connect() {
    }
}
```

`mod` 키워드 뒤에, 모듈의 이름 `network`가 쓰여지고 중괄호 안에 코드 블록이 옵니다.
이 블록 안의 모든 것은 이름공간 `network` 안에 있습니다. 위의 경우 `connect`라는
이름의 함수 하나가 있습니다. 이 함수를 `network` 모듈 바깥의 스크립트에서 호출하고자
한다면, 우리는 모듈을 특정할 필요가 있으므로 이름공간 문법 `::`를 이용해야 합니다:
`connect()` 이렇게만 하지 않고 `network::connect()` 이런 식으로요.

또한 같은 *src/lib.rs* 파일 내에 여러 개의 모듈을 나란히 정의할 수도 있습니다.
예를 들어, `connect`라는 이름의 함수를 갖고 있는 `client` 모듈을 정의하려면,
Listing 7-1에 보시는 바와 같이 이를 추가할 수 있습니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
mod network {
    fn connect() {
    }
}

mod client {
    fn connect() {
    }
}
```

<span class="caption">Listing 7-1: *src/lib.rs* 내에 나란히 정의된 `network`
모듈과 `client` 모듈</span>

이제 우리는 `network::connect` 함수와 `client::connect` 함수를 갖게 되었습니다.
이들은 완전히 다른 기능을 갖고 있을 수 있고, 서로 다른 모듈에 정의되어 있기 때문에
함수 이름이 서로 부딪힐 일은 없습니다.

이 경우, 우리가 라이브러리를 만드는 중이기 때문에, 라이브러리의 시작 지점으로서
제공되는 파일은 *src/lib.rs* 입니다. 하지만 모듈을 만드는 것에 관하여
*src/lib.rs*는 특별할 것이 없습니다. 우리는 라이브러리 크레잇의 *src/lib.rs* 내에
모듈을 만드는 것과 똑같은 방식으로 바이너리 크레잇의 *src/main.rs* 내에도 모듈을
만들 수 있습니다. 사실 모듈 안에 다른 모듈을 집어넣는 것도 가능한데, 이는 여러분의
모듈이 커짐에 따라 관련된 기능이 잘 조직화 되도록 하는 한편 각각의 기능을 잘 나누도록
하는데 유용할 수 있습니다. 여러분의 코드를 어떻게 조직화 할 것인가에 대한 선택은
여러분이 코드의 각 부분 간의 관계에 대해 어떻게 생각하고 있는지에 따라 달린 문제입니다.
예를 들어, Listing 7-2와 같이 `client` 모듈과 `connect` 함수가 `network` 이름공간
내에 있다면 우리의 라이브러리 사용자가 더 쉽게 이해할지도 모릅니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
mod network {
    fn connect() {
    }

    mod client {
        fn connect() {
        }
    }
}
```

<span class="caption">Listing 7-2: `client` 모듈을 `network` 모듈 안으로 이동</span>

*src/lib.rs* 파일에서 Listing 7-2와 같이 `client` 모듈이 `network` 모듈의
내부 모듈이 되도록 `mod network`와 `mod client`의 위치를 바꿔 봅시다. 이제
우리는 `network::connect`와 `network::client::connect` 함수를 갖게 되었습니다:
다시 말하지만, `connect`라는 이름의 두 함수는 서로 다른 이름공간에 있으므로
부딪힐 일이 없습니다.

In this way, modules form a hierarchy. The contents of *src/lib.rs* are at the
topmost level, and the submodules are at lower levels. Here’s what the
organization of our example in Listing 7-1 looks like when thought of as a
hierarchy:
이런 식으로 모듈들은 계층을 구성하게 됩니다. *src/lib.rs*의 내용은 가장 위의 층을
이루고, 서브 모듈들은 그보다 낮은 층에 있습니다. Listing 7-1 예제에서의 조직화는
계층 구조로 생각하면 어떻게 생겼을지를 봅시다:

```text
communicator
 ├── network
 └── client
```

그리고 Listing 7-2 예제에 대응되는 계층 구조는 이렇습니다:

```text
communicator
 └── network
     └── client
```

Listing 7-2에서 계층 구조는 `client`가 `network`의 형제이기 보다는 자식임을
보여줍니다. 더 복잡한 프로젝트는 많은 수의 모듈을 갖고 있을 수 있고, 이들은 지속적인
트래킹을 위해 논리적으로 잘 조직화될 필요가 있을 것입니다. 여러분의 프로젝트 내에서
“논리적으로”가 의미하는 것은 여러분에게 달려 있는 것이며, 여러분과 여러분의 라이브러리
사용자들이 프로젝트 도메인에 대해 어떻게 생각하는지에 따라 달라집니다. 여러분이 선호하는
어떤 형태의 구조이건 간에 여기서 보여준 나란한 모듈 및 중첩된(nested) 모듈을 만드는
테크닉을 이용해 보세요.

### Moving Modules to Other Files

Modules form a hierarchical structure, much like another structure in computing
that you’re used to: filesystems! We can use Rust’s module system along with
multiple files to split up Rust projects so not everything lives in
*src/lib.rs* or *src/main.rs*. For this example, let’s start with the code in
Listing 7-3:

<span class="filename">Filename: src/lib.rs</span>

```rust
mod client {
    fn connect() {
    }
}

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```

<span class="caption">Listing 7-3: Three modules, `client`, `network`, and
`network::server`, all defined in *src/lib.rs*</span>

The file *src/lib.rs* has this module hierarchy:

```text
communicator
 ├── client
 └── network
     └── server
```

If these modules had many functions, and those functions were becoming lengthy,
it would be difficult to scroll through this file to find the code we wanted to
work with. Because the functions are nested inside one or more mod blocks, the
lines of code inside the functions will start getting lengthy as well. These
would be good reasons to separate the `client`, `network`, and `server` modules
from *src/lib.rs* and place them into their own files.

First, replace the `client` module code with only the declaration of the `client`
module, so that your *src/lib.rs* looks like the following:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
mod client;

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```

We’re still *declaring* the `client` module here, but by replacing the block
with a semicolon, we’re telling Rust to look in another location for the code
defined within the scope of the `client` module. In other words, the line `mod
client;` means:

```rust,ignore
mod client {
    // contents of client.rs
}
```

Now we need to create the external file with that module name. Create a
*client.rs* file in your *src/* directory and open it. Then enter the
following, which is the `connect` function in the `client` module that we
removed in the previous step:

<span class="filename">Filename: src/client.rs</span>

```rust
fn connect() {
}
```

Note that we don’t need a `mod` declaration in this file because we already
declared the `client` module with `mod` in *src/lib.rs*. This file just
provides the *contents* of the `client` module. If we put a `mod client` here,
we’d be giving the `client` module its own submodule named `client`!

Rust only knows to look in *src/lib.rs* by default. If we want to add more
files to our project, we need to tell Rust in *src/lib.rs* to look in other
files; this is why `mod client` needs to be defined in *src/lib.rs* and can’t
be defined in *src/client.rs*.

Now the project should compile successfully, although you’ll get a few
warnings. Remember to use `cargo build` instead of `cargo run` because we have
a library crate rather than a binary crate:

```text
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/client.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/lib.rs:4:5
  |
4 |     fn connect() {
  |     ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/lib.rs:8:9
  |
8 |         fn connect() {
  |         ^
```

These warnings tell us that we have functions that are never used. Don’t worry
about these warnings for now; we’ll address them in the “Controlling Visibility
with `pub`” section later in this chapter. The good news is that they’re just
warnings; our project built successfully!

Next, let’s extract the `network` module into its own file using the same
pattern. In *src/lib.rs*, delete the body of the `network` module and add a
semicolon to the declaration, like so:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
mod client;

mod network;
```

Then create a new *src/network.rs* file and enter the following:

<span class="filename">Filename: src/network.rs</span>

```rust
fn connect() {
}

mod server {
    fn connect() {
    }
}
```

Notice that we still have a `mod` declaration within this module file; this is
because we still want `server` to be a submodule of `network`.

Run `cargo build` again. Success! We have one more module to extract: `server`.
Because it’s a submodule—that is, a module within a module—our current tactic
of extracting a module into a file named after that module won’t work. We’ll
try anyway so you can see the error. First, change *src/network.rs* to have
`mod server;` instead of the `server` module’s contents:

<span class="filename">Filename: src/network.rs</span>

```rust,ignore
fn connect() {
}

mod server;
```

Then create a *src/server.rs* file and enter the contents of the `server`
module that we extracted:

<span class="filename">Filename: src/server.rs</span>

```rust
fn connect() {
}
```

When we try to `cargo build`, we’ll get the error shown in Listing 7-4:

```text
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)
error: cannot declare a new module at this location
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
  |
note: maybe move this module `network` to its own directory via `network/mod.rs`
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
note: ... or maybe `use` the module `server` instead of possibly redeclaring it
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
```

<span class="caption">Listing 7-4: Error when trying to extract the `server`
submodule into *src/server.rs*</span>

The error says we `cannot declare a new module at this location` and is
pointing to the `mod server;` line in *src/network.rs*. So *src/network.rs* is
different than *src/lib.rs* somehow: keep reading to understand why.

The note in the middle of Listing 7-4 is actually very helpful because it
points out something we haven’t yet talked about doing:

```text
note: maybe move this module `network` to its own directory via
`network/mod.rs`
```

Instead of continuing to follow the same file naming pattern we used
previously, we can do what the note suggests:

1. Make a new *directory* named *network*, the parent module’s name.
2. Move the *src/network.rs* file into the new *network* directory, and
   rename *src/network/mod.rs*.
3. Move the submodule file *src/server.rs* into the *network* directory.

Here are commands to carry out these steps:

```text
$ mkdir src/network
$ mv src/network.rs src/network/mod.rs
$ mv src/server.rs src/network
```

Now when we try to run `cargo build`, compilation will work (we’ll still have
warnings though). Our module layout still looks like this, which is exactly the
same as it did when we had all the code in *src/lib.rs* in Listing 7-3:

```text
communicator
 ├── client
 └── network
     └── server
```

The corresponding file layout now looks like this:

```text
├── src
│   ├── client.rs
│   ├── lib.rs
│   └── network
│       ├── mod.rs
│       └── server.rs
```

So when we wanted to extract the `network::server` module, why did we have to
also change the *src/network.rs* file to the *src/network/mod.rs* file and put
the code for `network::server` in the *network* directory in
*src/network/server.rs* instead of just being able to extract the
`network::server` module into *src/server.rs*? The reason is that Rust wouldn’t
be able to recognize that `server` was supposed to be a submodule of `network`
if the *server.rs* file was in the *src* directory. To clarify Rust’s behavior
here, let’s consider a different example with the following module hierarchy,
where all the definitions are in *src/lib.rs*:

```text
communicator
 ├── client
 └── network
     └── client
```

In this example, we have three modules again: `client`, `network`, and
`network::client`. Following the same steps we did earlier for extracting
modules into files, we would create *src/client.rs* for the `client` module.
For the `network` module, we would create *src/network.rs*. But we wouldn’t be
able to extract the `network::client` module into a *src/client.rs* file
because that already exists for the top-level `client` module! If we could put
the code for *both* the `client` and `network::client` modules in the
*src/client.rs* file, Rust wouldn’t have any way to know whether the code was
for `client` or for `network::client`.

Therefore, in order to extract a file for the `network::client` submodule of
the `network` module, we needed to create a directory for the `network` module
instead of a *src/network.rs* file. The code that is in the `network` module
then goes into the *src/network/mod.rs* file, and the submodule
`network::client` can have its own *src/network/client.rs* file. Now the
top-level *src/client.rs* is unambiguously the code that belongs to the
`client` module.

### Rules of Module Filesystems

Let’s summarize the rules of modules with regard to files:

* If a module named `foo` has no submodules, you should put the declarations
  for `foo` in a file named *foo.rs*.
* If a module named `foo` does have submodules, you should put the declarations
  for `foo` in a file named *foo/mod.rs*.

These rules apply recursively, so if a module named `foo` has a submodule named
`bar` and `bar` does not have submodules, you should have the following files
in your *src* directory:

```text
├── foo
│   ├── bar.rs (contains the declarations in `foo::bar`)
│   └── mod.rs (contains the declarations in `foo`, including `mod bar`)
```

The modules should be declared in their parent module’s file using the `mod`
keyword.

Next, we’ll talk about the `pub` keyword and get rid of those warnings!
