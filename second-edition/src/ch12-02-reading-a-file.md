## 파일 읽기

다음으로, 우리는 커맨드 라인 인자 파일이름으로 지정된 파일을 읽어볼 겁니다. 먼저, 함께 테스트 할 샘플 파일이 필요합니다.
'greprs'가 동작하는 것을 확신할 수 있기 위해 가장 좋은 종류의 파일은 몇 개의 반복되는 단어의 다수의 줄에 걸쳐 존재하는 
작은 양의 텍스트입니다. 항목 12-3의 에밀리 딕킨스 시는 잘 작동할 겁니다. `poem.txt`로 명명된 파일을 당신의 프로젝트 최상위에
생성하고 시를 입력합시다 "I'm nobody! Who are you?":

<span class="filename">Filename: poem.txt</span>

```text
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us — don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

<span class="caption">항목 12-3: 테스트 용으로 적합한 에밀리 딕킨슨의 시 "I'm nobody! Who are you?"</span>

언급된 위치에 위의 파일을 생성한 후, *src/main.rs* 파일을 아래 항목 12-4의 내용을 참고하여 편집합니다.  

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
```

항목 12-4: 두 번째 인자로 특정된 파일의 내용 읽어들이기

먼저, 우리는 `use`문 몇 개를 추가하여 표준 라이브러리에서 관련 있는 부분을 가져옵니다: 우리는 파일 관련하여 
`std::fs::File`과, 파일 I/O를 포함한 I/O 작업을 위해 유용한 다양한 특성이 있는 `std::io::prelude::*`이 
필요합니다.

Rust가 가진 지정된 것들을 영역 내로 가져오는 일반적인 도입부와 동일하게, `std::io` 모듈은 당신이 I/O 작업을 할 때 
필요할만한 일반적인 것들에 대한 그 자신만의 도입부를 갖습니다. 기본적인 도입부와는 다르게, 우리는 반드시 `std::io`의 
도입부를 명시적으로 `use`해야 합니다. 

`main`에서, 우리는 다음 세 가지를 추가했습니다:  첫 째, `File::open`함수를 호출하고 `filename`값을 전달하여 
파일을 변경할 수 있는 핸들을 얻습니다. 두 번째로, `contents`라는 이름의 빈 `String` 가변 변수를 만들었습니다. 이 
변수는 우리가 읽어들인 내용을 보관하기 위한 용도로 사용될 겁니다. 셋 째, 우리가 만들어 둔 파일 핸들에 
`read_to_string`을 호출하여 가변 참조를 `contents`의 인자로 전달합니다.

이후, 임시로 `println!`을 추가하여 `contents`의 값을 출력함으로서 파일을 읽어들인 이후 우리 프로그램이 제대로 
동작했는지 확인할 수 있습니다. 

아무 문자나 첫 번째 커맨드라인 인자로 입력하고(우리가 아직 검색 부분을 구현하지 않았기 때문에) 두 번째는 우리가 만들어 둔 
*poem.txt* 파일로 입력하여 이 코드를 실행해봅시다.

```text
$ cargo run the poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/greprs the poem.txt`
Searching for the
In file poem.txt
With text:
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us — don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

좋군요! 우리의 코드가 파일 내용을 읽고 출력했습니다. 우리 프로그램은 몇 가지 결점이 있습니다: `main` 함수는 많은 책임을 
지고(역주: [단일 책임 원칙](https://ko.wikipedia.org/wiki/%EB%8B%A8%EC%9D%BC_%EC%B1%85%EC%9E%84_%EC%9B%90%EC%B9%99) 참고), 우리가 할 수 있는 에러처리를 하지 않았습니다. 아직 우리의 프로그램이 작기 때문에, 이 결점들은 
큰 문제가 아닐 수도 있습니다. 하지만 우리 프로그램 커져가면, 점점 이를 깔끔하게 수정하기 어렵게 됩니다. 프로그램의 개발 초기 
단계에 리팩토링을 하면 코드의 양이 적은만큼 리팩토링을 하기 훨씬 쉬워지기 때문에 훌륭한 단련법 입니다. 그러니 지금 해봅시다.

<!-- 업데이트된 원본:
## Reading a File

Now we’ll add functionality to read the file that is specified in the
`filename` command line argument. First, we need a sample file to test it with:
the best kind of file to use to make sure `minigrep` is working is one with a
small amount of text over multiple lines with some repeated words. Listing 12-3
has an Emily Dickinson poem that will work well! Create a file called
*poem.txt* at the root level of your project, and enter the poem “I’m Nobody!
Who are you?”

<span class="filename">Filename: poem.txt</span>

```text
I’m nobody! Who are you?
Are you nobody, too?
Then there’s a pair of us — don’t tell!
They’d banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

<span class="caption">Listing 12-3: A poem by Emily Dickinson makes a good test
case</span>

With the text in place, edit *src/main.rs* and add code to open the file, as
shown in Listing 12-4:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
#     let args: Vec<String> = env::args().collect();
#
#     let query = &args[1];
#     let filename = &args[2];
#
#     println!("Searching for {}", query);
    // --snip--
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}
```

<span class="caption">Listing 12-4: Reading the contents of the file specified
by the second argument</span>

First, we add some more `use` statements to bring in relevant parts of the
standard library: we need `std::fs::File` to handle files, and
`std::io::prelude::*` contains various useful traits for doing I/O, including
file I/O. In the same way that Rust has a general prelude that brings certain
types and functions into scope automatically, the `std::io` module has its own
prelude of common types and functions you’ll need when working with I/O. Unlike
with the default prelude, we must explicitly add a `use` statement for the
prelude from `std::io`.

In `main`, we’ve added three statements: first, we get a mutable handle to the
file by calling the `File::open` function and passing it the value of the
`filename` variable. Second, we create a variable called `contents` and set it
to a mutable, empty `String`. This will hold the content of the file after we
read it in. Third, we call `read_to_string` on our file handle and pass a
mutable reference to `contents` as an argument.

After those lines, we’ve again added a temporary `println!` statement that
prints the value of `contents` after the file is read, so we can check that the
program is working so far.

Let’s run this code with any string as the first command line argument (because
we haven’t implemented the searching part yet) and the *poem.txt* file as the
second argument:

```text
$ cargo run the poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep the poem.txt`
Searching for the
In file poem.txt
With text:
I’m nobody! Who are you?
Are you nobody, too?
Then there’s a pair of us — don’t tell!
They’d banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

Great! The code read and then printed the contents of the file. But the code
has a few flaws. The `main` function has multiple responsibilities: generally,
functions are clearer and easier to maintain if each function is responsible
for only one idea. The other problem is that we’re not handling errors as well
as we could. The program is still small, so these flaws aren’t a big problem,
but as the program grows, it will be harder to fix them cleanly. It’s good
practice to begin refactoring early on when developing a program, because it’s
much easier to refactor smaller amounts of code. We’ll do that next. -->
