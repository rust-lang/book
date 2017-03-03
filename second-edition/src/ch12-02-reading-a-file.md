## Reading a File

Now that we have some variables containing the information that we need, let's
try using them. The next step is to open the file that we want to search. To do
that, we need a file. Create one called `poem.txt` at the root level of your
project, and fill it up with some Emily Dickinson:

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

<!-- Public domain Emily Dickinson poem. This will work best with something
short, but that has multiple lines and some repetition. We could search through
code; that gets a bit meta and possibly confusing... Changes to this are most
welcome. /Carol -->

With that in place, let's edit *src/main.rs* and add code to open the file as
shown in Listing 12-3:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let search = &args[1];
    let filename = &args[2];

    println!("Searching for {}", search);
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}
```

<figcaption>

Listing 12-3: Read the contents of the file specified by the second argument

</figcaption>
</figure>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

We've added a few things. First of all, we need some more `use` statements to
bring in the relevant parts of the standard library: we need `std::fs::File`
for dealing with files, and `std::io::prelude::*` contains various traits that
are useful when doing I/O, including file I/O. In the same way that Rust has a
general prelude that brings certain things into scope automatically, the
`std::io` module has its own prelude of common things you'll need when working
with I/O. Unlike the default prelude, we must explicitly `use` the prelude in
`std::io`.

In `main`, we've added three things: first, we get a handle to the file and
open it by using the `File::open` function and passing it the name of the file
specified in the second argument. Second, we create a mutable, empty `String`
in the variable `contents`, then call `read_to_string` on our file handle with
our `contents` string as the argument; `contents` is where `read_to_string`
will place the data it reads. Finally, we print out the entire file contents,
which is a way for us to be sure our program is working so far.

Let's try running this code, specifying any string for the first argument (since
we haven't implemented the searching part yet) and our *poem.txt* file as the
second argument:

```text
$ cargo run the poem.txt
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target\debug\greprs.exe the poem.txt`
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

Great! Our code is working. However, it's got a few flaws. Because our program
is still small, these flaws aren't a huge deal, but as our program grows, it
will be harder and harder to fix them in a clean way. Let's do the refactoring
now, instead of waiting. The refactoring will be much easier to do with only
this small amount of code.
