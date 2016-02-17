# Loops

It’s often quite useful to be able to execute a block of code more than one
time. For this, we have several constructs, called ‘loops’.

To try out loops, let’s make a new project. Navigate to your projects folder
and use Cargo to make a new one:

```bash
$ cargo new --bin loops
$ cd loops
```

There are three kinds of loops in Rust: `loop`, `while`, and `for`. Let’s dig
in.

## `loop`

The `loop` keyword is very straightforward: it executes a block of code over
and over and over and over and over and over forever. Change your `src/main.rs`
file to look like this:

```rust,ignore
fn main() {
    loop {
        println!("again!");
    }
}
```

If we run this program, we’ll see ‘`again!`’ printed over and over again. So
how does our program end? It doesn’t, until we kill it. Most terminals support
a keyboard shortcut, ‘control-c’, to stop a runaway program. Give it a try:

```bash
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

That `^C` there is where I hit control-c.

That’s a lot of trouble though! Luckily, there’s a way to break an infinite `loop`.

### Breaking out of a loop

The `break` keyword will allow us to quit looping. Try this version out:

```rust
fn main() {
    loop {
        println!("once!");
        break;
    }
}
```

If you run this program with `cargo run`, you’ll see that it only executes one
time:

```bash
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
     Running `target/debug/loops`
once!
```

When a Rust program hits a `break` statement, it will exit the current loop.

## `while`

What if we took `loop`, `break`, and `if`, and put them together? Something
like this:

```rust
fn main() {
    let mut number = 3;

    loop {
        if number != 0 {
            println!("{}!", number);

            number = number - 1;
        } else {
            break;
        }

    }

    println!("LIFTOFF!!!");
}
```

If we run this, we’ll get some output:

```bash
   Compiling loops v0.1.0 (file:///projects/loops)
     Running `target/debug/loops`
3!
2!
1!
LIFTOFF!!!
```

The core of this example is in the combination of these three constructs:

```rust,ignore
    loop {
        if number != 0 {
            // do stuff
        } else {
            break;
        }
```

We want to `loop`, but only while some sort of condition is true. As soon as it
isn't, we want to `break` out of the loop.

This pattern is so common that we have a language construct for it: `while`.
Here's the same example, but using `while` instead:

```rust
fn main() {
    let mut number = 3;
    while number != 0  {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}
```

This lets us get rid of a lot of nesting, and is more clear: while a condition holds,
run this code.

## `for`

We can use this `while` construct to loop over the elements of a collection, like an
array:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < 5 {
        println!("the value is is: {}", a[index]);

        index = index + 1;
    }
}
```

Running this will print out every element of the array:

```bash
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
     Running `target/debug/loops`
the value is: 1
the value is: 2
the value is: 3
the value is: 4
the value is: 5
```

Here, we're counting up instead of down: we start at zero, then loop until we
hit the final index of our array.

This approach is error-prone, though. If we get the index length incorrect, we
will end up causing a `panic!`. This is also slow, as the compiler needs to do
that check on every element on every iteration through the loop.

Instead, we can use our last kind of loop: the `for` loop. It looks like this:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let mut index = 0;

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

** NOTE: see [https://github.com/rust-lang/rust/issues/25725#issuecomment-166365658](https://github.com/rust-lang/rust/issues/25725#issuecomment-166365658), we may want to change this **

If we run this, we'll see the same output as the previous example.

** I'm going to leave it at this for now until we decide how we want to do it**
