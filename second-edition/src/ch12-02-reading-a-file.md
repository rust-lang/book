## Reading a File

Next, we're going to read the file that we specify in the filename command line
argument. First, we need a sample file to test it with---the best kind of file
to use to make sure that `minigrep` is working is one with a small amount of text
over multiple lines with some repeated words. Listing 12-3 has an Emily
Dickinson poem that will work well! Create a file called `poem.txt` at the root
level of your project, and enter the poem "I'm nobody! Who are you?":

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

<span class="caption">Listing 12-3: The poem "I'm nobody! Who are you?" by
Emily Dickinson that will make a good test case</span>

With that in place, edit *src/main.rs* and add code to open the file as shown
in Listing 12-4:

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
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}
```

Listing 12-4: Reading the contents of the file specified by the second argument

First, we add some more `use` statements to bring in relevant parts of the
standard library: we need `std::fs::File` for dealing with files, and
`std::io::prelude::*` contains various traits that are useful when doing I/O,
including file I/O. In the same way that Rust has a general prelude that brings
certain things into scope automatically, the `std::io` module has its own
prelude of common things you'll need when working with I/O. Unlike the default
prelude, we must explicitly `use` the prelude in `std::io`.

In `main`, we've added three things: first, we get a mutable handle to the file
by calling the `File::open` function and passing it the value of the `filename`
variable. Second, we create a variable called `contents` and set it to a
mutable, empty `String`. This will hold the content of the file after we read
it in. Third, we call `read_to_string` on our file handle and pass a mutable
reference to `contents` as an argument.

After those lines, we've again added temporary `println!` that prints out the
value in `contents` after we've read the file so we can check that our program
is working so far.

Let's try running this code with any string as the first command line argument
(since we haven't implemented the searching part yet) and our *poem.txt* file
as the second argument:

```text
$ cargo run the poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep the poem.txt`
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

Great! Our code read in and printed out the content of the file. We've got a
few flaws though: the `main` function has multiple responsibilities, and we're
not handling errors as well as we could be. While our program is still small,
these flaws aren't a big problem, but as our program grows, it will be harder
to fix them cleanly. It's good practice to begin refactoring early on when
developing a program, as it's much easier to refactor smaller amounts of code,
so we'll do that now.
