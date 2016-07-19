## Variable Bindings in Detail

So far, we’ve created the simplest kind of variable binding, but the `let`
statement has some more tricks up its sleeve. Now we'll look at doing more
complex things: creating multiple bindings at once, adding type annotations,
creating mutating bindings, understanding shadowing, and more.

### Creating Multiple Bindings

The previous example program just bound one variable, but it's also possible to
create multiple variable bindings in one go. Let’s try a more complex example,
creating two variable bindings at once. Change your example program to this:

```rust
fn main() {
    let (x, y) = (5, 6);

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

And enter `cargo run` to run it:

```bash
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
     Running `target/debug/bindings`
The value of x is: 5
The value of y is: 6
```

We’ve created two bindings with one `let` statement! The `let` statement binds
the values in `(5, 6)` to the corresponding patterns of `(x, y)`. The first
value `5` binds to the first part of the pattern, `x`, and the second value `6`
binds to `y`. We could alternatively have used two `let` statements to the same
effect, as follows:

```rust
fn main() {
    let x = 5;
    let y = 6;
}
```

In simple cases like this where we are only binding two variables, two `let`
statements may be clearer in the code, but when you're creating many multiple
bindings, it's useful to be able to do so all at once. Deciding which technique
to use is mostly a judgement call, and as you become more proficient in Rust,
you’ll be able to figure out which style is better in each case.

### Delayed Initialization

The examples so far have all provided bindings with an initial value, but that
isn't always necessary. Rather, we can assign a value for the binding later,
after the `let` statement. To try this out, write the following program:

```rust
fn main() {
    let x;
    x = 5;
    println!("The value of x is: {}", x);
}
```

And enter `cargo run` to run it:

```bash
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
     Running `target/debug/bindings`
The value of x is: 5
```

As you can see, this works just like the previous program in which we assigned
an initial value.

This raises an interesting question: what happens if we try to print out a
binding before we declare a value? Let's find out. Modify your code to look
like the following:

```rust,ignore
fn main() {
    let x;
    println!("The value of x is: {}", x);
    x = 5;
}
```

When you enter `cargo run` to run this code, you should see output like this
after the command:

```bash
   Compiling bindings v0.1.0 (file:///projects/bindings)
src/main.rs:4:39: 4:40 error: use of possibly uninitialized variable: `x` [E0381]
src/main.rs:4     println!("The value of x is: {}", x);
                                                    ^
<std macros>:2:25: 2:56 note: in this expansion of format_args!
<std macros>:3:1: 3:54 note: in this expansion of print! (defined in <std macros>)
src/main.rs:4:5: 4:42 note: in this expansion of println! (defined in <std macros>)
src/main.rs:4:39: 4:40 help: run `rustc --explain E0381` to see a detailed explanation
error: aborting due to previous error
Could not compile `bindings`.

To learn more, run the command again with --verbose.
```

There's been an error! The compiler won’t let us write a program like this, and
instead it requests that you assign a value to the variable `x`. This is our
first example of the compiler helping us find an error in our program.
Different programming languages have different ways of approaching this
problem. Some languages will always initialize values with some sort of
default. Other languages leave the value uninitialized and make no promises
about what happens if you try to use something before initialization. Rust
responds with an error to prod the programmer to declare the value they want.
We must initialize any variable before we can use it.

PROD: START BOX
######Extended Error Explanations

Now that you've seen an example of a Rust error, I want to point out one
particularly useful aspect of errors. Rust encourages you to seek further
information on the kind of error you've received with output like this:

```bash
src/main.rs:4:39: 4:40 help: run `rustc --explain E0381` to see a detailed explanation
```

This tells us that if we pass the `--explain` flag to `rustc` with the provided
error code, we can see an extended explanation which will try to explain common
causes of and solutions to that kind of error. Not every error has a longer
explanation, but many do. Here’s the explanation for the `E0381` error we
received previously:

```bash
$ rustc --explain E0381
It is not allowed to use or capture an uninitialized variable. For example:

fn main() {
    let x: i32;

    let y = x; // error, use of possibly uninitialized variable

To fix this, ensure that any declared variables are initialized before being
used.
```

These explanations can really help if you’re stuck on an error, so don't
hesitate to look up the error code. The compiler is your friend, and it's there
to help.

PROD: END BOX

### Mutable bindings

By default, variable bindings are *immutable*, meaning that once a value is
bound, you can't change that value. Try writing the following sample program to
illustrate this:

```rust,ignore
fn main() {
    let x = 5;
    x = 6;
    println!("The value of x is: {}", x);
}
```

Save and run the program, and you should receive another error message, as in
this output:

```bash
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
src/main.rs:4:5: 4:10 error: re-assignment of immutable variable `x` [E0384]
src/main.rs:4     x = 6;
                  ^~~~~
src/main.rs:4:5: 4:10 help: run `rustc --explain E0384` to see a detailed explanation
src/main.rs:2:9: 2:10 note: prior assignment occurs here
src/main.rs:2     let x = 5;
                      ^
```

The error includes the message `re-assigment of immutable variable` because the
program tried to assign a second value to the `x` variable. But bindings are
immutable only by default; you can make them mutable by adding `mut` in front
of the variable name. For example, change the program you just wrote to the
following:

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

Running this, we get:

```bash
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
     Running `target/debug/bindings`
The value of x is: 5
The value of x is: 6
```

Using `mut`, we change the value that `x` binds to from `5` to `6`.

### Variable Binding Scope

As we discussed in the ownership section of Chapter XX, variable bindings are
only valid as long as they are *in scope*. That scope begins at the point where
the binding is declared and ends with the curly brace that closes the block of
code containing that binding. We cannot access bindings "before they come into
scope" or "after they go out of scope." Here’s an example to illustrate this:

```rust
fn main() {
    println!("x is not yet in scope. Try to print x in this statement to see the compiler error!");

    let x = 5;

    println!("x is now in scope and its value is {}.", x);

    println!("In real code, we’d now do a bunch of work.");

    println!("x will go out of scope now! The next curly brace is ending the main function.");
}
```

The variable binding for `x` goes out of scope with the last curly brace in the
`main()` function.

This example only has one scope, though. In Rust, it's possible to create
arbitrary scopes within a scope by placing code within another pair of curly
braces (we'll look at this more in the next chapter). For example:

```rust
fn main() {
    println!("x is not yet in scope");

    let x = 5;

    println!("x is now in scope and its value is {}.", x);
    println!("y is not yet in scope. Try to print y in this statement to see the compiler error!");
    println!("Let’s start a new scope!");

    {
        println!("y is still not yet in scope...");
        let y = 8;

        println!("NOW y is in scope and its value is {}", y);
        println!("x is also still in scope with value {}", x);

        println!("y will go out of scope now!");
        println!("The next curly brace is ending the scope we started.");
    }

    println!("x is still in scope: {}", x);
    println!("y is now out of scope and is not usable. Try using it here!");

    println!("x will go out of scope now! The next curly brace is ending the main function.");
}
```

The `y` variable is only in scope in the section of the code that's between the
nested pair of curly braces, whereas `x` is in scope from the `let` statement
that binds it until the final curly brace. The scope of bindings will become
much more important later as you learn about references in Chapter XX.

### Shadowing Earlier Bindings

One final thing about bindings: they can *shadow* previous bindings. Shadowing
is what happens when you declare two bindings with the same name. We say that
the first binding is ‘shadowed’ by the second, which means that the second
binding's value is what you will see when you use the variable after the second
binding. This can be useful if you’d like to perform a few transformations on a
value, but still leave the binding immutable. For example:

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}
```

This program first binds `x` to a value of `5`. Then, it shadows `x` by saying
`let x =` again, taking the original value and adding `1` so that the value of
`x` is then `6`. The third `let` statement also shadows `x`, taking the
previous value and multiplying it by `2` to give `x` a final value of `12`. If
you run this, it will output:

```bash
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
     Running `target/debug/bindings`
The value of x is: 12
```

Shadowing is useful because it lets us modify `x` without having to make the
variable mutable. This means the compiler will still keep us from accidentally
trying to mutate `x` directly later. For example, say after calculating `12` we
don’t want `x` to be modified again; if we write the program in a mutable
style, like this:

```rust
fn main() {
    let mut x = 5;

    x = x + 1;
    x = x * 2;

    println!("The value of x is: {}", x);

    x = 15;

    println!("The value of x is: {}", x);
}
```

Rust is happy to let us mutate `x` again, to `15`. A similar program using the
default immutable style, however, will prevent us from mutating `x`. Here's an
example:

```rust,ignore
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    x = 15;

    println!("The value of x is: {}", x);
}
```

If we try to compile this, we get an error:

```bash
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
src/main.rs:8:5: 8:11 error: re-assignment of immutable variable `x` [E0384]
src/main.rs:8     x = 15;
                  ^~~~~~
src/main.rs:8:5: 8:11 help: run `rustc --explain E0384` to see a detailed explanation
src/main.rs:4:9: 4:10 note: prior assignment occurs here
src/main.rs:4     let x = x * 2;
                      ^
error: aborting due to previous error
Could not compile `bindings`.
```

Since we don't want the binding to be mutable, this is exactly what should
happen.

#### Shadowing Over Bindings

You can also shadow bindings over one another, without re-using the initial
binding in the value. Here's how that looks:

```rust
fn main() {
    let x = 5;
    let x = 6;

    println!("The value of x is: {}", x);
}
```

Running this sample program, we can see the shadowing in action:

```bash
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
src/main.rs:2:9: 2:10 warning: unused variable: `x`, #[warn(unused_variables)] on by default
src/main.rs:2     let x = 5;
                      ^
     Running `target/debug/bindings`
The value of x is: 6
```

Rust gives the value of `x` as `6`, which is the value from the *second* `let`
statement. There are a few interesting things in this output. First, Rust
will compile and run the program without issue. This is because we haven't
mutated the value; instead, we declared a _new_ binding that is _also_ named
`x`, and gave it a new value.

The other interesting thing in this output is this warning line:

```bash
src/main.rs:2:9: 2:10 warning: unused variable: `x`, #[warn(unused_variables)] on by default
```

Rust is pointing out that we shadowed `x`, but never used the initial value.
Doing so isn’t _wrong_, but Rust is checking whether this is intentional and
not just a mistake. In this case, the compiler issues a warning, but still
compiles our program. A warning like this is called a *lint*, which is an old
term for the bits of fluff and fibers in sheep’s wool that you wouldn't want to
put in cloth.

Similarly, this lint is telling us that we may have an extra bit of code (the
statement `let x = 5`) that we don’t need. Even though our program works just
fine, listening to these warnings and fixing the problems they point out is
worthwhile, as they can be signs of a larger problem. In this case, we may not
have realized that we were shadowing `x`, when we meant to, say, define a new
variable with a different name.

Shadowing can take some time to get used to, but it’s very powerful and works
well with immutability.

#### Shadowing and Scopes

Like any binding, a binding that shadows another binding becomes invalid at the
end of a scope. Here’s an example program to illustrate this:

```rust
fn main() {
    let x = 5;
    println!("Before shadowing, x is: {}", x);

    {
        let x = 6;
        println!("Now that x is shadowed, x is: {}", x);
    }

    println!("After shadowing, x is: {}", x);
}
```

This code first creates the `x` variable and prints `x` to the terminal. Then,
inside a new scope, it creates a new binding for `x` with a new value and
prints that value. When the arbitrary scope ends, `x` is printed once more. If
we run this example, we can see the shadow appear and disappear in the output:

```bash
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
     Running `target/debug/bindings`
Before shadowing, x is: 5
Now that x is shadowed, x is: 6
After shadowing, x is: 5
```

In this case, the binding value reverts to the original value once the shadow
binding goes out of scope.
