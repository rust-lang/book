## Reading a File

Now we’ll add functionality to read the file specified in the `file_path`
argument. First, we need a sample file to test it with: we’ll use a file with a
small amount of text over multiple lines with some repeated words. Listing 12-3
has an Emily Dickinson poem that will work well! Create a file called
*poem.txt* at the root level of your project, and enter the poem “I’m Nobody!
Who are you?”

<span class="filename">Filename: poem.txt</span>

```text
{{#include ../listings/ch12-an-io-project/listing-12-03/poem.txt}}
```

<span class="caption">Listing 12-3: A poem by Emily Dickinson makes a good test
case</span>

With the text in place, edit *src/main.rs* and add code to read the file, as
shown in Listing 12-4.

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/src/main.rs:here}}
```

<span class="caption">Listing 12-4: Reading the contents of the file specified
by the second argument</span>

First, we bring in a relevant part of the standard library with a `use`
statement: we need `std::fs` to handle files.

In `main`, the new statement `fs::read_to_string` takes the `file_path`, opens
that file, and returns a `std::io::Result<String>` of the file’s contents.

After that, we again add a temporary `println!` statement that prints the value
of `contents` after the file is read, so we can check that the program is
working so far.

Let’s run this code with any string as the first command line argument (because
we haven’t implemented the searching part yet) and the *poem.txt* file as the
second argument:

```console
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/output.txt}}
```

Great! The code read and then printed the contents of the file. But the code
has a few flaws. At the moment, the `main` function has multiple
responsibilities: generally, functions are clearer and easier to maintain if
each function is responsible for only one idea. The other problem is that we’re
not handling errors as well as we could. The program is still small, so these
flaws aren’t a big problem, but as the program grows, it will be harder to fix
them cleanly. It’s good practice to begin refactoring early on when developing
a program, because it’s much easier to refactor smaller amounts of code. We’ll
do that next.
