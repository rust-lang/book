<!-- Are you open to addressing the reader as "you" and the writer as "I" (or we, since it's "the Rust community")?
     That would make things feel more casual in a good way, like a conversation between you and the reader. -->
<!-- I generally prefer the collective ‘we’ for teaching materials, since it’s something that ‘we’ are doing together. I’m kind of open to it, I guess... -->
     
<!-- Possible to use fewer carriage returns in the code, or will Rust need that whitespace? -->
<!-- It’s possible, but one of the themes of the book is showing off good Rust style; I don’t want people to be writing super-cramped code. That said, it seems like you’ve inserted a _lot_ of newlines, or at least, something did. I’ve trimmed them down again. How’s it look now? -->

<!-- Throughout, I've suggested some more detailed heading names to help readers navigate the chapter, but please do 
     change any of those that don't make sense to you. In general, where we can, let's avoid single-word headings, 
     as they can leave the section contents a bit mysterious. -->
<!-- Great! Headings are a _big_ weakness of mine as an author. I always need more of them and better-named ones -->

[TOC]

# Up and Running

We’ll start our Rust journey by talking about the absolute basics, concepts that appear in almost every programming language. Many programming languages have much in common at their core. None of the concepts presented in this chapter are unique to Rust, but we’ll discuss Rust’s particular syntax and conventions concerning these common concepts.


<!--- Can you give a small summary of what we're going to cover? it helps readers navigate - and they can tell early on if it's something they need to read or can skip. --> 
<!-- added it below -->

<!-- If you want to skip this section, you can, but you may end up coming back later

to find out small details.

    I'm unsure about advising readers to skip at this point - if they want to skip, they probably will anyway, and it can tempt merely lazy readers to skip and miss something important. -->
<!-- I’m fine with dropping it. -->    

Specifically, we’ll be talking about variable bindings, functions, basic types, comments, `if` statements, and looping. These foundations will be in every Rust program, and learning them early will give you a strong core to start from.


## Anatomy of a Rust Program

<!-- I suggest giving this section a name that reflects the fact that we're dissecting a program (mine may not be quite
     right, of course), so readers have a good idea of what's in store. I just worry they'll expect to dive straight
     into bindings in the first subsection, and get confused. -->

The foundation of virtually every program is the ability to store and modify data, but to create this data, you first have to create a program. Here, we'll write some code that demonstrates how to begin a Rust program, how to bind a variable, and how to print text to the terminal.

<!-- Let's give a brief summary of the section above, so readers know what's ahead. I've suggested something, but do tweak that if it doesn't make sense. -->
<!-- tweaked! -->


#### A Simple Program that Binds a Variable

<!-- I've addded a short description of the program coming up, below. In general, readers will find even brief descriptions of any code helpful, even when the code itself is short. -->
<!-- seems good, I’m assuming there will be lots more of this later; I’m bad at remembering these, too -->

Let’s start with a short example that binds a value to a variable, and then uses that binding in a sentence that we'll print to the screen. First, we’ll generate a new project with Cargo. Open a terminal, and navigate to the directory you want to store your projects in. From there, generate a new project:

```bash
$ cargo new --bin bindings
$ cd bindings
```

<!-- We typically italicize filenames, as below, would you be happy to follow that convention throughout? If you prefer to keep
     to your current styles for the online version, just let us know. --> 
<!-- happy to italicize; our own conventions here are very weak. Let’s do it. -->

This creates a new project called `bindings` and sets up our *Cargo.toml* and *src/main.rs* files. As we saw in Chapter XX, Cargo will generate these files and create a little "hello world" program like this:

```rust
fn main() {
    println!("Hello, world!");
}
```

Open that program and replace its code with the following: 

```rust
fn main() {
    let x = 5;

    println!("The value of x is: {}", x);
}
```

This is the full program for our example. Enter the `run` command now to to see it working:

```bash
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
     Running `target/debug/bindings`
The value of x is: 5
```

<!-- Is it important to show Rust compiling in each example of output? I wonder if it would make
     sense to skip those messages after this first instance, to expedite things, but I'd like to hear your thoughts. --> 
<!-- So my thought on this was to include it here, in the first chapter, but drop it later. If you think it’s too much, we can cut it out. -->

If you get an error instead of this output, double check that you've copied the program
exactly as written, and then try again. Now let’s break this program down, line by line.

#### Starting a Program with the main() Function

Most Rust programs open with the same first line as this one from our example program:

```rust,ignore
fn main() {
```

The `main()` function is the entry point of every Rust program. It doesn’t have to be at the very beginning of our source code, but it will be the first bit of code that runs when we execute our program. We’ll talk more about functions in the next section, but for now, just know is that ```main()``` is where our program begins. The opening curly brace (`{`) indicates the start of the function’s body.

<!-- Could you clarify "entry point," above? It seems like we're saying that `fn main()` needs to be at the start of 
      every Rust program, but I wasn't quite certain. -->
<!-- done -->


#### Binding a Variable with `let`

Inside the function body, we added the following: 

```rust,ignore
    let x = 5;
```

This is a `let` statement, and it binds the value `5` to the variable `x`. Basic `let` statements take the following form:

```text
let NAME = EXPRESSION;
```

<!-- I suggest mentioning something now (or perhaps earlier) about semi-colons. They come up later, but as they 
      recurr throughout the code examples, readers will find touching on the subject sooner helpful. -->
<!-- sounds good, I like the modification below. -->



A `let` statement first evaluates the `EXPRESSION`, and then binds the resulting value to `NAME` to give us a variable to use later in the program. Notice the semicolon at the end of the statement, too. As in many other programming languages, statements in Rust must end with a semicolon.

In this simple example, the expression already is a value, but we could achieve the same result like this:

```rust
let x = 2 + 3;
```

<!-- If a reader isn't familiar with patterns in a Rust context, I think they'd find a brief explanation
     helpful here. Could you add that, if you think it would be useful to some readers? -->

The expression `2 + 3` would evaluate to `5`, which would in turn be stored in the `x` variable binding. In general, `let` statements work with *patterns*. Patterns are part of a feature of Rust called ‘pattern matching’. We can compare an expression against a pattern, and then make a choice based on how the two compare. A name like `x` is a particularly humble form of pattern; it will always match. Patterns are a big part of Rust, and we’ll see more complex and powerful patterns as we go along.

<!-- Is an expression also considered a pattern? -->
<!-- I tried to answer this in the text, but I’m not 100% happy with it. -->

#### Printing to the Screen with a Macro

The next line of our program is:

```rust,ignore
    println!("The value of x is: {}", x);
```

<!-- Rather than "much later in the book," could we specify a chapter below? It would also be helpful to flesh this out 
     a bit, and talk about what macros are. Would it be accurate to say, "Macros are an abstracting mechanism that brings 
     metaprogramming to Rust. Macros actually expand into source code that gets compiled in place with the rest of the 
     program."?  --> 
<!-- that sentence is accurate, but it’s also not the only way of doing metaprogramming. -->

The `println!` command is a *macro* that prints the text passed to it to the screen. Macros are indicated with the `!` character. In Chapter <???>, you'll learn how to write macros your, but for now we'll use macros provided by the standard Rust library. Macros can add new syntax to the language, and the `!` is a reminder that things may look slightly unusual.

<!-- Can you briefly describe what may look slightly unusual? Perhaps also discuss the difference between a macro and a function, as many programmers coming to Rust from elsewhere would expect println to be a function/method. -->
<!-- I’m not sure of the best way to do that without getting too much into macros and what they are. Maybe something like the function above? Also, it’s tough to talk about why println! is a macro, because it inolves a lot of things we haven’t talked about yet. -->

The `println!` macro only requires one argument: a format string. You can add optional arguments inside this format string by using the special text `{}`. Each instance of `{}` corresponds to an additional argument. Here’s an example:

```rust
let x = 2 + 3;
let y = x + 5;

println!("The value of x is {}, and the value of y is {}", x, y);
```

If you were to run a program containing these statements, it would print the following:

```rust
The value of x is 5, and the value of y is 10
```

Think of `{}` as little crab pincers, holding a value in place. The first `{}` holds the first value after the format string, the second set holds the second value, and so on.


<!-- Would it be correct to say that in Rust, the values after the format string are placed into the
     curly brace placeholders in order, as I've suggested above? Showing the output may help to cement that. --> 
<!-- yes, this is exactly right. I like it. -->


The `{}` placeholder has a number of more advanced formatting options that we’ll discuss
later.

After the `println` macro, we match the opening curly brace that declared the `main()` function with a closing curly brace to declare the end of the function:

```rust,ignore
}
```

And of course, when we run the program, our output is:

```text

The value of x is: 5

```

<!-- I think it could be fine not to repeat what the program does here, if we explain that earlier. A little signoff
     to close out the section would give helpful context for the next section, though. Maybe something to this effect?--> 
<!-- I added a little flourish that people seemed to love from the previous iteration of the book. :) -->

With this simple program, you've created your first variable and used your first Rust macro. That makes you a Rust programmer. Welcome! Now that you've seen the basics, let's explore variable bindings further.

## Variable Bindings in Detail

<!-- If we want to stick with calling the previous section "Anatomy of a Rust Program," then we could use a heading here
     to set off the more detailed dive into variable bindings. If you're happy with this structure, could you add 
     a paragraph here to introduce the coming subsections? -->

So far, we’ve created the simplest kind of variable binding, but the `let` statement has some tricks up its sleeve.
Let’s do some more complex things: create multiple bindings at once, how to add type annotations, mutating bindings,
shadowing, and more.

### Creating Multiple Bindings

The previous example program just bound one variable, but it's also possible to create multiple variable bindings in one go. Let’s try a more complex example, creating two variable bindings at once. Change your example program to this:

```rust
fn main() {
    let (x, y) = (5, 6);

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

And enter `cargo run` to run it:

```text
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
     Running `target/debug/bindings`
The value of x is: 5
The value of y is: 6
```



We’ve created two bindings with one `let` statement! 

<!-- After the first line-by-line example, I would suggest just dissecting the programs by quoting the code
     in the body text rather than on individual lines - it can help keep the chapter flowing, make the learning feel quicker; I've commented out the two listings below, to show what I mean. -->

<!--- ```text

(x, y)

```



And here’s the value:



```text

(5, 6)

```

---> 



The `let` statement binds the values in `(5, 6)` to the corresponding patterns of `(x, y)`. The first value `5` binds to the first part of the pattern, `x`, and the second value `6` binds to `y`. We could alternatively have used two `let` statements to the same effect, as follows:


```rust
fn main() {
    let x = 5;
    let y = 6;
}
```

In simple cases like this, where we are only binding two variables, two `let` statements may be clearer in the code, but when you're creating many multiple bindings, it's useful to be able to do so all at once. Deciding which technique to use is mostly a judgement call, and as you become more proficient in Rust, you’ll be able to figure out which style is better in each case.

### Delayed Initialization

The examples so far have all provided bindings with an initial value, but that isn't always necessary. Rather, we can assign a value for the binding later, after the `let` statement. To try this out, write the following program:

```rust
fn main() {
    let x;

    x = 5;

    println!("The value of x is: {}", x);
}
```

And enter `cargo run` to run it:

```text
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
     Running `target/debug/bindings`
The value of x is: 5
```

As you can see, this works just like the previous program, in which we assigned an initial value.

This raises an interesting question: what happens if we try to print out a binding before we declare a value? Let's find out. Modify your code to look like the following:

<!-- Our convention is to reserve "enter" for entering commands. Should readers replace their old code with the 
     following, or perhaps create a new program file? -->
<!-- sounds good, I’ve tweaked it -->


```rust,ignore
fn main() {
    let x;

    println!("The value of x is: {}", x);

    x = 5;
}
```

When you enter `cargo run` to run this code, you should see output like this after the command:

```text
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



There's been an error! The compiler won’t let us write a program like this, and instead it requests that you declare a value. This is our first example of the compiler helping us find an error in our program. Different programming languages have different ways of approaching this problem. Some languages will always initialize values with some sort of default. Other languages leave the value uninitialized, and make no promises about what happens if you try to use something before initialization. Rust responds with an error to prod the programmer to declare the value they want. We must initialize any variable before we can use it.


<!-- Possible to box this next section? That's what I would suggest for the print book, as it feels a little
     like an aside, though is still important. -->
<!-- Sure, I’d be okay with that. Reading error messages is really important, but you’re right that this feels more like an aside. -->

### Extended Error Explanations

Now that you've seen an example of a Rust error, I want to point out one particularly useful aspect of errors. Rust encourages you to seek further information on the kind of error you've received with output like this:

```text
src/main.rs:4:39: 4:40 help: run `rustc --explain E0381` to see a detailed explanation
```

This tells us that if we pass the `--explain` flag to `rustc` with the provided error code, we can see an extended explanation, which will try to explain common causes of and solutions to that kind of error. Not every error has a longer explanation, but many do. Here’s the explanation for the `E0381` error we received previously:

```bash
$ rustc --explain E0381
It is not allowed to use or capture an uninitialized variable. For example:

fn main() {
    let x: i32;

    let y = x; // error, use of possibly uninitialized variable

To fix this, ensure that any declared variables are initialized before being
used.
```

These explanations can really help if you’re stuck on an error, so don't hesitate to look up the error code. The compiler is your friend, and it's there to help.

### Mutable bindings

By default, variable bindings are *immutable*, meaning that once a value is bound, you can't change that value. Try writing the following sample program to illustrate this:

```rust,ignore
fn main() {
    let x = 5;

    x = 6;

    println!("The value of x is: {}", x);
}
```

Save and run the program, and you should receive another error message, as in this output:

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

The error includes the message `re-assigment of immutable variable` because the program tried to assign a second value to the `x` variable. But bindings are immutable only by default; you can make them mutable by adding `mut` in front of the variable name. For example, change the program you just wrote to the following:

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

Using `mut`, we change the value that `x` binds to from `5` to `6`. Note, however, that `mut` is part of the pattern in the `let` statement. This becomes more obvious if we add mutability to a pattern that binds multiple variables, like this:

```rust,ignore
fn main() {
    let (mut x, y) = (5, 6);

    x = 7;
    y = 8;
}
```

If you run this code, the compiler will output an error:

```bash
$ cargo build
   Compiling bindings v0.1.0 (file:///projects/bindings)
src/main.rs:5:5: 5:10 error: re-assignment of immutable variable `y` [E0384]
src/main.rs:5     y = 8;
                  ^~~~~
src/main.rs:5:5: 5:10 help: run `rustc --explain E0384` to see a detailed explanation
src/main.rs:2:17: 2:18 note: prior assignment occurs here
src/main.rs:2     let (mut x, y) = (5, 6);
                              ^
```

The way `mut` is used here, the compiler is fine with reassigning the `x` variable, but not the `y` variable. That's because `mut` only applies to the name that directly follows it, not the whole pattern. For the compiler to allow you to reassign the `y` variable, you'd need to write the pattern as `(mut x, mut y)` instead.

<!-- In general, I'd shy away from "negative" examples, examples that demonstrate the opposite of the concept being
      conveyed. Perhaps show it being successful, instead? If you prefer to keep as-is, I would suggest giving the 
      correct code in the body text; would we have to use (mut x, mut y)? -->
<!-- I agree generally, but a big part of getting comfortable with Rust, specifically, is being okay with seeing errors, knowing how to read them, and how to take the compiler’s advice to fix them. Maybe that should be more focused on the kinds of errors where that’s important, though... -->

<!-- #### Reassignment, not Mutation

 I'd just make this part of the above section, or perhaps a note?  -->
<!-- Seems good :) -->

One thing to know about mutating bindings: `mut` allows you to mutate _the binding_, but not _what the name binds to_. In other words, the value is not what changes, but rather the path between the value and the name. For example:

<!-- Would it be accurate the say 'what the name binds to', rather than `what the binding binds to`?  Is the binding the connection between name and value, rather than either the name or the value itself? That might be useful to say explicitly. -->
<!-- Yes, you’re right. The binding is the connection. -->


```rust
fn main() {
    let mut x = 5;

    x = 6;
}
```

This does not change the value that `x` is bound to, but creates a new value (`6`) and changes the binding so that it binds the name `x` to this new value instead. This subtle but important difference will become more important as your Rust programs get more complex. 

<!-- Specifically, passing arguments to functions will illustrate the difference. We’ll talk more about this in the next section, when we discuss functions. -->

<!--- Perhaps save the specific point about functions above for the Function section, for flow. --->
<!-- seems good -->

### Variable Binding Scope

<!--- Is it worth mentioning global variables here, if applicable, too? I've added a bit, but do delete if that
      doesn't make sense for Rust. -->
<!-- Rust has a weird relationship with global variables. You can’t use `let` to make them, and they require lots of extra annotations. They’ll be covered later, and are used _very_ rarely in Rust -->

Another important thing to know about variable bindings is that they are only valid as long as they are *in scope*. That scope begins at the point where the binding is declared, and ends at with the curley brace that closes the block of code containing it. We cannot access bindings "before they come into scope" or "after they go out of scope." Here’s an example to illustrate this:


```rust
fn main() {
    println!("x is not yet in scope");

    let x = 5;

    println!("x is now in scope");

    println!("In real code, we’d now do a bunch of work."); 

    println!("x will go out of scope now! The next curly brace is ending the main function.");
}
```

The variable binding for `x` goes out of scope with the last curly brace in the `main()` function.

<!-- I added this sentence above, to sum up the code example; is this accurate? Could you please edit/add to it, if not? -->
<!-- it is :) -->

This example only has one scope, though. In Rust, it's possible to create arbitrary scopes within a scope by placing code within another pair of curly braces. For example:

<!--- Perhaps say when this would be useful, to give the reader some context. --> 
<!-- we don’t have the features yet that makes this useful, unfortunately: that’s the next chapter -->

```rust

fn main() {

    println!("x is not yet in scope");

    let x = 5;

    println!("x is now in scope");

    println!("Let’s start a new scope!");

    {
        let y = 5;

        println!("y is now in scope");
        println!("x is also still in scope");

        println!("y will go out of scope now!");
        println!("The next curly brace is ending the scope we started.");
    }

    println!("x is still in scope, but y is now out of scope and is not usable");

    println!("x will go out of scope now! The next curly brace is ending the main function.");
}

```

<!--- Readers will find it helpful if we talk through the code a little. I've offered some text here, but fine to
      replace it, of course. ---> 
<!-- seems good :) -->

The `y` variable is only in scope in the section of the code that's between the nested pair of curly braces, whereas `x` is in scope from the `let` statement that binds it until the final curly brace. The scope of bindings will become much more important later, as you learn about references in Chapter XX.

### Shadowing Earlier Bindings

<!--- Can you add a definition of shadowing, to help the reader know what to look out for here? --> 

One final thing about bindings: they can *shadow* previous bindings with the same name. Shadowing is what happens when you declare two bindings with the same name, we say that the second binding ‘shadows’ the first.

<!-- Hm, I'd suggest switching these two examples - I think the second, adding values to the variable x and redefining the binding of x, explains the concept very clearly, as well as showing how it is useful. I've moved them around to show you what I mean, but if you think this doesn't work do move it back. -->
<!-- I like it! -->

This can be useful if you’d like to perform a few transformations on a value, but still leave the binding immutable. For example:

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}
```

<!--- Readers will appreciate it if we talk through what's going on in the code a bit. I've made some suggestions, but please do check through and revise as necessary. ---> 

This program first binds `x` to a value of `5`. Then, it shadows `x`, taking the original value and adding `1` so that the value of `x` is then `6`. The third `let` statement shadows `x` again, taking the previous value and multiplying it by `2` to give `x` a final value of `12`. If you run this, it will output:

<!-- If we continue showing the compiling messages, I think it would be good to show the cargo run command
     and the $ consistently as well. Could you add that below, if needed? -->
<!-- ah yes, that’s just a mistake; I agree they should be there every time -->

```bash
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
     Running `target/debug/bindings`
The value of x is: 12
```

Shadowing is useful because it lets us modify `x` without having to make the variable mutable. This means the compiler will still warn us if we accidentally try to mutate `x` directly later. For example, say after calculating `12` we don’t want `x` to be modified again; if we write the program in a mutable style, like this:

<!-- I would flip the next two examples, to show the immutable version first, and then show how mutability comes into 
     play. Concepts are usually more clear to readers when we show "how to do" something first, rather than "how not to 
     do" it. -->

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

Rust is happy to let us mutate `x` again, to `15`. A similar program using the default immutable style, however, will let us know about that accidental mutation. Here's an example:

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
$ cargo build
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

Since we don't want the binding to be mutable, this exactly what should happen.

#### Shadowing Over Bindings

<!-- Here is the first example from the section, I've moved it down. -->

You can also shadow bindings over one another, without re-using the initial binding. Here's how that looks: 

```rust

fn main() {
    let x = 5;
    let x = 6;

    println!("The value of x is: {}", x);
}  
```

Running this sample program, we can see the shadowing in action:

```text
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
src/main.rs:2:9: 2:10 warning: unused variable: `x`, #[warn(unused_variables)] on by default
src/main.rs:2     let x = 5;
                      ^
     Running `target/debug/bindings`
The value of x is: 6

```

Rust gives the value of `x` as `6`, which is the value from the *second* `let` statement. There are a few interesting things in this output. First, that Rust will compile and run the program without issue. This is because we haven't mutated the value; instead, we declared a _new_ binding that is _also_ named `x`, and gave it a new value.

<!--- Is it fair to say that the new x overwrites the old x? --->
<!-- it doesn’t actually. it’s not accessible, but it’s still there. This is shown off in the next example, actually. -->

The other interesting thing in this output is this error line:

```text
src/main.rs:2:9: 2:10 warning: unused variable: `x`, #[warn(unused_variables)] on by default
```

Rust is pointing out that we shadowed `x`, but never used the initial value. Doing so isn’t _wrong_, but Rust is checking whether this is intentional and not just a mistake. In this case, the compiler issues a warning, but still compiles our program. <!---The `#[warn(unused_variables)]` syntax is called an ‘attribute’, which we’ll discuss in Section XX.---> A warning like this is called a *lint*, which is an old term for the bits of fluff and fibers in sheep’s wool that you wouldn't want to put in cloth.

Similarly, this lint is telling us that we may have an extra bit of code (the statement `let x = 5`) that we don’t need. Even though our program works just fine, listening to these warnings and fixing the problems they point out is worthwhile, as they can be signs of a larger problem. In this case, we may not have realized that we were shadowing `x`, when we meant to, say, define a new variable with a different name.

Shadowing can take some time to get used to, but it’s very powerful, and works well with immutability. 

#### Shadowing and Scopes

Like any binding, a binding that shadows another binding becomes invalid at the end of a scope. Here’s an example program to illustrate this:

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

This code first creates the `x` variable and prints `x` to the terminal. Then, inside a new scope, it creates a new binding for `x` with a new value, and prints that value. When the arbitrary scope ends, `x` is printed once more. If we run this example, we can see the shadow appear and disappear in the output:

```bash
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
     Running `target/debug/bindings`
Before shadowing, x is: 5
Now that x is shadowed, x is: 6
After shadowing, x is: 5
```

In this case, the binding value reverts to the original value once the shadow binding goes out of scope.

<!-- Since we have readers create a new project for Functions, this might be a good place to split the chapter. --> 

## How Functions Work in Rust

<!-- Does Rust define functions any differently than other languages? If so, it may be helpful to include
     a brief definition, before talking about casing. I also suggest contrasting functions with macros like println
     somewhere here. -->
<!-- it’s pretty much the same as in any language. -->

Functions are pervasive in Rust code. We’ve already seen one of the most important functions in the language: the `main()` function that’s the start of every program. We've also seen the `fn` keyword, which allows us to declare new functions.

<!--- I think we can get away without showing these examples again, as the reader should have the hang of them now. --->

<!---:

```rust

fn main() {

    println!("Hello, world!");

}

```

---> 

<!-- 

```rust

fn another_function() {

    println!("Another function.");

}

```

--->  

Rust code uses *snake case* as the conventional style for function names. In snake case, all letters are lower case, and there are underscores separating words. (Rust also uses snake case for the names of variable bindings; we just haven't used any variable bindings long enough to need underscores yet.) Here's a program containing an example function definition:

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

<!-- In the text, it sounds like the function should come before its use in the program, but the code makes it
     seem that's not the case. Could you clarify the text? -->
<!-- yes, that’s not required. I’ll tweak it. -->

Function definitions in Rust always start with `fn` and have a set of parentheses after the function name. The curly braces tell the compiler where the function begins and ends.

We can call any function we’ve defined by entering its name followed by a pair of parentheses. Since `another_function()` is defined in the program, it can be called from inside the `main()` function. Note that we defined `another_function()` _after_ the `main()` function in our source code; we could have defined it before as well. Rust doesn’t care where you define your functions, only that they are defined somewhere.

Let’s start a new project to explore functions further. Open a terminal, and navigate to the directory you're keeping your projects in. From there, use Cargo to generate a new project, as follows:

```bash
$ cargo new --bin functions
$ cd functions
```

<!-- Is this generating a new project, or converting the previous another_function program to cargo? -->
<!-- I was generating a new one per section: one for bindings, another for functions. We could skip it, I guess, and just stick with one. -->



Place the `another_function()` example in a file named *src/main.rs* and run it. You should see the following output:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
Hello, world!
Another function.
```

The lines execute in the order they appear in the `main()` function. First, our “Hello, world!” message prints, and then `another_function()` is called and its message is printed.

### Function Arguments

<!-- I found this section quite hard to follow, and I worry readers will too. Could you add a bit more text explanation
     that talks through the examples, and explicitly discuss what they do? The code examples are good, but in general,
     readers find code even more helpful with just a brief description. -->
<!-- hmm, I’m having a tough time here. Is there any way you could elborate on what parts got you stuck? or how it’s difficult? -->


Functions can also take arguments. The following rewritten version of `another_function()` shows what arguments look like in Rust:


```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

<!-- Can you talk through what's happening in this code? Perhaps highlight the fact that the arguments
     go inside the parentheses, if that's what readers should be noticing here. -->


Try running this program, and you should get this output:



```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
```

Since `main()` passed `5` to `another_function()`, the `println` macro put `5` where the pair of curly braces were in the format string.

Let’s take a closer look at the signature of a function which takes a single argument:

<!-- Perhaps show this signature before we show the argument version of another_function, to keep a flow from
     general to specific. -->

```text
fn NAME(PATTERN: TYPE) {
```

The parameter declaration in a single-argument function signature looks like the `let` bindings we used earlier. Just look at both together, and compare them:

```rust,ignore
let x: i32;
fn another_function(x: i32) {
```

<!-- What should the reader notice when looking at the signature of the function? ---> 
<!-- the two things are: that the ‘pattern: type’ situation is repeated, and that the type is required here -->

The one difference is that in function signatures, we _must_ declare the type. This is a deliberate decision in the design of Rust; requiring type annotations in function definitions means you almost never need to use them elsewhere in the code.

<!-- Accurate to call this a deliberate decision "in the design of Rust"? And why do we almost never need
     to use them elsewhere? Is it because Rust will automatically interpret the rest? --> 
<!-- yes, it is deliberate. And yeah, it’s because it will infer usage; we had previously talked about that in the ‘type annotations’ section of the variable bindings bit. -->

When you want a function to have multiple arguments, just separate them inside the function signature with commas, like this:

```text
fn NAME(PATTERN, PATTERN, PATTERN, PATTERN...) {
```

And just like a `let` declaration with multiple patterns, a type must be applied to each pattern separately. To demonstrate, here’s a full example of a function with multiple arguments:

```rust
fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

<!--- Readers will find it helpful if we talk through this example before showing output, even if it's only a sentence or two. Let's mention anything the reader should notice in particular. Do all arguments of the same function need to be the same type? --->
<!-- added -->

In this example, we make a function with two arguments. In this case, both are `i32`s, but if your function has multiple arguments, they don’t have to be the same time. They just happen to be in this example. Our function then prints out the values of both of its arguments.

Let’s try out this code. Replace the program currently in your `function` project's `main.rs` file with the example above, and run it as follows:



```bash

$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
The value of y is: 6
```

Since `5` is passed as the `x` argument and `6` is passed as the `y` argument, the two strings are printed with these values.

### Bindings as Arguments

<!-- Would it make sense to say "Variables as Arguments" here, instead? Or is it more common to refer to variables
     as bindings in Rust? Some readers just may think that "let a = 5" would be the argument. -->
<!-- yes, they’re properly called ‘bindings’ or ‘variable bindings’, but not ‘variables’. -->

It's also possible to create bindings and pass them in as arguments in Rust. For example:

```rust
fn main() {
    let a = 5;
    let b = 6;

    another_function(a, b);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

Instead of passing `5` and `6` directly, this first creates two bindings containing the values, and passes those bindings instead. When you run this, you'll find that it has the same effect as just using integers:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
The value of y is: 6
```

Note that our bindings are called `a` and `b`, yet inside the function, we refer to them by the names in the signature, `x` and `y`. Inside a function, its parameters are in scope but the names of the bindings we passed as parameters are not, so we need to use the parameter names within the function block. Bindings passed as parameters don’t need to have the same names as the arguments.

### Functions with Return Values

<!-- Can you put this in context? When would you want to return a value back to the function? I'm not sure readers will be entirely clear on how this differs much from previous examples. --->
<!-- so my struggle here is that I am assuming that the reader knows at least one programming language, so it’s tough, because it feels self-evident to me. hm. -->

Functions can return values back to functions that call them. The signature for a function that returns a value looks like this:

```TEXT
fn NAME(PATTERN, PATTERN, PATTERN, PATTERN...) -> TYPE {
```

In Rust, we don’t name return values, but we do declare their type, after the arrow (`->`). Here’s a sample program to illustrate this concept:

```rust
fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}
```

There are no function calls, macros, or even `let` statements in the `five()` function--just the number `5` by itself. That's a perfectly valid function in Rust. Note the function's return type, too. Try running this code, and the output should look like this:



```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
```

<!--- what makes this a return value rather than, say, just the value of the variable? Can you make that explicit in the text? --->
<!-- this gets circuitous: it’s the return value because it’s the value that’s being returned. :( -->


The `5` in `five()` is actually the function's return value, which is why the return type is `i32`. Let’s examine this in more detail. There are two important bits. First, the line `let x = five();` in `main()` shows that we can use the return value of a function to initialize a binding.

<!--
```rust,ignore

let x = five();

```
-->

Because the function `five()` returns a `5`, that line is the same as saying:

```rust
let x = 5;
```

The second interesting bit is the `five()` function itself. It requires no arguments and defines the type of the return, but the body of the function is a lonely `5` with no semicolon. So far, we’ve ended almost every line in our programs with a semicolon, so why not here?

<!--
```rust

fn five() -> i32 {

    5

}

```
-->

The answer is that the return value of a function is the value of its final expression. To explain this, we have to go over statements and expressions. 

### Statements and Expressions

<!-- We use statements and expressions much earlier in the chapter, so if possible, I suggest reworking this to be
     shown immediately before "Variable Bindings in Detail." Readers will find seeing this material earlier helpful, but 
     I do see the chicken-and-egg problem, that as-written it requires concepts covered previously. If you feel strongly 
     about showing it closer to this point, could we rework and show it right before "Functions"?-->
<!-- yeah, this is what makes this first bit SO tough; by fixing issues like this, you can sometimes introduce other problems. In this case, if we put it _before_ functions, we haven’t seen the motivations for expressions on their own; everything else we’ve seen up till this `5` is a statement, not really an expression. The right hand side of `let` is one, I guess... hmmm -->

Expressions are bits of code that evaluate to a value. Consider a simple math operation, like this:

```rust,ignore
5 + 6
```

Evaluating this expression results in the value: `11`. In Rust, most bits of code are expressions. For example, calling a function is an expression:

<!-- If you agree about moving this section to before "Functions," we should make this point somewhere else in the
     "Functions" section. -->

```rust,ignore
foo(5)
```

The value is equal to the return value of the `foo()` function.

Statements are instructions. While expressions _compute_ something, statements perform some action. For example, `let` statements bind variables, and `fn` declarations are statements that begin functions.

One practical difference between an expression and a statement is that you can bind an expression, but you can't bind a statement.

<!--- Is this because a statement doesn't evaluate to a value to bind to? I'd suggest saying why, explicitly --->
<!-- it’s because `let` is a statement which only accepts expressions on the right hand side, not expressions or statemetns. Is that too deep in the grammar to be a useful explanation? -->

For example, `let` is a statement, so you can’t assign it to another binding, as this code tries to do:

```rust,ignore
fn main() {
    let x = (let y = 6);
}
```

If we were to run this program, we’d get an error like this:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
src/main.rs:2:14: 2:17 error: expected identifier, found keyword `let`
src/main.rs:2     let x = (let y = 6);
                           ^~~
src/main.rs:2:18: 2:19 error: expected one of `!`, `)`, `,`, `.`, `::`, `{`, or an operator, found `y`
src/main.rs:2     let x = (let y = 6);
                               ^
Could not compile `functions`.
```

<!-- Perhaps talk this through a little, say briefly why this wouldn't work? --> 

In the same way, we can't assign a `fn` declaration to a binding, either.

#### Expressions as Return Values

<!-- I think we could use a heading here, but do change this one if it doesn't seem quite right. -->

So what does the way statements and expressions work have to do with return values? Well, the block that we use to create new scopes, `{}`, is an expression. Let’s take a closer look at `{}` with the following signature:

<!--- Is this something the reader can bring up? I'm not 100% sure what this is showing us, can you explain it a little more? If it's a bit of a sidenote, perhaps consider deleting the signature and explaining it in text - I do think the explanation works without it ---> 

```text
{
    STATEMENT*
    EXPRESSION
}
```

<!--Is the * a convention used in the Rust documentation? -->
<!-- it’s more of a general convention used in things like regular expressions -->

The `*` by `STATEMENT` indicates "zero or more," meaning we can have any number of statements inside a block, followed by an expression. Since blocks are expressions themselves, we can nest blocks inside of blocks.

<!---So we can also have any number of expressions inside a block too? Should there be a * after EXPRESSION? --->
<!-- no, you can’t, or at least, not without another block. -->

And since blocks return a value, we can use them in `let` statements. For example:

```rust
fn main() {

    let x = 5;

    let y = {
        let z = 1;

        x + z + 5
    };

    println!("The value of y is: {}", y);
}
```

<!-- I found this section a bit hard to follow, with the explanations appearing after the output, so I've suggested
     a bit of rearrangement. In general, I do suggest explaining what a chunk of code does in one place, if the code
     is short enough to show all at once like this. -->

Here, we're using a block to give us a value for the `y` variable. Inside that block, we create a new variable binding, `z`, with a `let` statement and give `z` a value. For the final expression of the block, we do some math, ultimately binding the result to `y`. Since `x` is `5` and `z` is `1`, the calculation is `5 + 1 + 5`, and so the value of the entire block is `11`. This gets substituted into our `let` statement for `y`, making that statement equivalent to:

```rust,ignore
let y = 11;
```

Try running the program, and you should see the following output:

```bash
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of y is: 11
```

<!--```rust,ignore

let y = {



};

```-->


<!--```rust,ignore
{

    let z = 1;

    x + z + 5

}
```-->

<!--- Let's bring this back round to return values and functions. Is the takeaway that we're using the return value from the block in a function? Perhaps worth pointing out that the final expression in the nested blocks lacks a semicolon, and why that works. --->
<!-- yes, that is. that’s what the next section is about -->

As expected, the output string says that `y` is `11`.

#### Functions Are Expressions (Or a different heading, if this one doesn't make sense.)
<!-- well, functions are _blocks_, which are a specific kind of expression -->

We also use blocks as the body of functions, for example:

<!-- Could you clarify the sentence above? Do you mean that functions are contained within blocks 
     by virtue of being wrapped in {}? How does that relate to expressions? I do think readers are going to have
     some trouble connecting the dots here; perhaps we could use a heading? I added one, but it's a shot in the dark. -->

<!-- hmm. I’m having another hard time understanding what specifically is confusing here :/ -->

```rust
fn main() {
    let x = 5;
    
    let y = {
        x + 1
    };

    println!("The value of y is: {}", y);

    let y = plus_one(x);

    println!("The value of y is: {}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

In both `let` statements that bind values to the `y` variable, we use a block to produce the value. In the first case, the block is an arbitrary scope nested within the `main()` function. In the second, the block is the body of the `plus_one()` function, which is passed `x` as a parameter. Running this gives:

<!--- I'm not sure what you meant by "it's the return value of the function", is my edit correct above? ---> 
<!-- “the return value of the function” is synonymous with “the value of the final expression in the block of the body of a function”. So your edit is correct, but sounds a bit awkward to my ear -->


```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of y is: 6
The value of y is: 6
```

The `x` variable doesn't change before the new `y` variable is created and bound to the return value of the `plus_one()` function, so both `println` macros tell us that `y` is `6`.

<!--

```rust,ignore

let y = {



``` -->


<!--```rust,ignore

fn plus_one(x: i32) -> i32 {

```-->





#### Expression Statements

Another impoortant thing to know about expressions and statements is that adding a semicolon to the end of an expression turns it into a statement. For example, look at this modified version of our `plus_one()` function from earlier:

```rust,ignore
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}

```

<!--- Is there ever a case where you *would* want to turn an expression into a statement? That might make a good example, if so, a positive rather than negative example ---> 
<!-- there are, but we don’t have the constructs for that yet. -->

Since `x + 1` is the only code in the function, it should be an expression, so that the value it evaluates to can be the function's return value. But the semicolon has turned it into a statement, so running this code would give an error, as follows:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
src/main.rs:7:1: 9:2 error: not all control paths return a value [E0269]
src/main.rs:7 fn plus_one(x: i32) -> i32 {
src/main.rs:8     x + 1;
src/main.rs:9 }
src/main.rs:7:1: 9:2 help: run `rustc --explain E0269` to see a detailed explanation
src/main.rs:8:10: 8:11 help: consider removing this semicolon:
src/main.rs:8     x + 1;
                       ^
error: aborting due to previous error
Could not compile `functions`.
```

The main error message, "not all control paths return a value," reveals the core of the issue with this code. Statements don’t evaluate to a value, but `plus_one()` tries to return an `i32` with only a statement in the function body. In this output, Rust gives an option to rectify this: it suggests removing the semicolon, which would fix the error.

In practice, Rust programmers don’t often think about these rules at this level. On a practical level, you should remember that you usually have a semicolon at the end of most lines, but ...

<!--- Hm, I'm not sure I follow you above; could you clarify? Earlier we say most Rust code is expressions, and now we say semicolons are usually placed at the end of most lines, but wouldn't that turn expressions into statements and cause errors? Are there times when expression statements are useful, or do they only result in errors? --> 
<-- thinking about this more, I did think of one: calling functions. foo() is an expression, and so foo() bar() is invalid. but adding ; to make them statements is okay, foo(); bar(); Maybe i can work up an example after all, what do you think? -->




### Returning Multiple Values

By default, functions can only return single values. There’s a trick, however to get them to return multiple values. Remember how we used `()`s to create complex bindings in the "Creating Multiple Bindings" section on page XX?

```rust
fn main() {
    let (x, y) = (5, 6);
}
```

Braces used in this way form a *tuple*, which is a collection of elements that isn't assigned a name. Tuples are also a basic data type in Rust, and we'll cover them in detail in the "Tuples" section later in this chapter. For our purposes now, we can use tuples to return multiple values from functions, as so:

```rust
fn main() {
    let (x, y) = two_numbers();

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn two_numbers() -> (i32, i32) {
    (5, 6)
}
```

<!-- Rather than break this explanation up, could you explain the code above in a paragraph here, rather than show the
     individual parts again? This program is short enough that I think readers will find seeing the explanation all in
     one spot easier to follow. -->
<!-- leaveing this for now until we determine if we want to change all of this; it’s gonna have to be reworked for every example -->


Running this will give us the values:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
The value of y is: 6
```

<!---There are two interesting changes here: assigning the return value of

`two_numbers()` to `x` and `y`, and the declaration of `two_numbers()` itself. ---> 

Let's look at this more closely. First, we're assigning the return value of `two_numbers()` to `x` and `y`: 

```rust
fn two_numbers() -> (i32, i32) {
    (5, 6)
}
```

<!---We use the `(i32, i32)` code that we also used in `let` bindings earlier: ```rust let (x, y): (i32, i32) = (5, 6); ```---> <!---I'd suggest cutting this, to avoid confusing our discussion of the code---> 

In plain English, the `(i32, i32)` syntax translates to, “a tuple with two `i32`s in it." These two types are then applied to the tuple to be returned by the function block. In this case, that tuple contains the values `5` and `6`.  This tuple is returned, and assigned to `x` and `y`:

```rust,ignore
let (x, y) = two_numbers();
```

See how all these bits fit together? We call this behavior of `let` ‘destructuring’, because it takes the structure of the expression that follows the `=` and takes it apart.

## Data Types in Rust

We’ve seen that every value in Rust is of a certain *type*, which tells Rust what kind of data is being given so it knows how to work with that data. As described in the "Type Inference and Annotation" section, you can rely on Rust's ability to infer types to figure out the type of a binding, or you can annotate it explicitly if needed. In this section, we'll look at a number of types built into the language itself. We'll look at two subsets of Rust data types: scalar and compound.

### Type Inference and Annotation

<!--- I think this section may fit better into the flow of the chapter as part of the Types section. If the information is important to the discussion of bindings, perhaps we should discuss types before going into detail on bindings? ---> 
<!-- I’ve moved it into the types section. -->
<!-- okay, so one problem with doing this is that we lose the symmetry explanation between bindings with type annotations and function signatures..... :( -->

Rust is a *statically typed* language, which means that we must know the types of all bindings at compile time. However, you may have noticed that we didn’t declare a type for `x` or `y` in our previous examples.

<!-- By 'we must know', do you mean that Rust must know? -->

This is because Rust can often tell the type of a binding without you having to declare it. Annotating every single binding with a type can take uneccesary time and make code noisy. To  avoid this, Rust uses *type inference*, meaning that it attempts to infer the types of your bindings from how the binding is used. Let’s look at the the first `let` statement you wrote again:

```rust
fn main() {
    let x = 5;
}
```

<!--- It seems that Rust determines x should be numeric from the fact that we provide a numeric value (specifically an integer). Is that correct? I've edited below to that effect, but please do check. --> 

When we bind `x` to `5`, the compiler determines that `x` should be a numeric type based on the value it is bound to. Without any other information, it sets the `x` variable's type to `i32` (a thirty-two bit integer type) by default. We’ll talk more about Rust’s basic types in Section 3.3.

<!-- Rather than a section number, could you give the name, above? That's how readers will be used to seeing it
     in print. --> 

If we were to declare the type with the variable binding, that would be called a *type annotation*. A `let` statement like that would look like this:

```text

let PATTERN: TYPE = VALUE;

```

<!-- Where it makes sense, I suggest showing the "template" before a "specific example," as I've edited here. -->

The `let` statement now has a colon after the `PATTERN`, followed by the `TYPE` name. Note that the colon and the `TYPE` go _after_ the `PATTERN`, not inside the pattern itself. Given this structure, here's how you'd rewrite `let x = 5` to use type annotation:

```rust

fn main() {

    let x: i32 = 5;

}

```

This does the same thing as `let x = 5` but explicitly states that `x` should be of the `i32` type. This is a simple case, but more complex patterns with multiple bindings can use type annotation, too. A binding with two variables would look like this:



```rust

fn main() {

    let (x, y): (i32, i32) = (5, 6);

}

```



In the same way as we place the `VALUE` and the `PATTERN` in corresponding positions, we also match up the position of the `TYPE` with the `PATTERN` it corresponds to.

<!--- Could you say briefly when you would need to declare, rather than let Rust infer, a type?-->



<!--Remember, you can rely on type inference to figure out the type of a binding, or you can annotate it explicitly:



```rust

fn main() {

    let x: i32 = 5;

}

``` -->

 

### Scalar Types

A *scalar* type is one that represents a single value. There are four key scalar types in Rust: integers, floating point numbers, booleans, and characters. You'll likely recognize these from other programming languages, but let's jump into how they work in Rust.



#### Integer Types

<!---can you give a direct definition of an integer here? It may seem obvious, but it's good to make it as easy for every reader as possible - I've made a suggestion do illustrate, do edit over it. ---> 
<!-- I did some tweaking, ‘whole number’ is actually a subset of integers. tricky! -->

An *integer* is a number without a fractional component. We've used one integer type already in this chapter, the `i32` type. This type declaration indicates that the value it's associated with should be a signed integer (hence the `i`) for a 32-bit system. There are a number of built-in integer types in Rust, shown in Table 3-1.

| Length | signed | unsigned |
|--------|--------|----------|
|  8-bit |  i8    |  u8      |
| 16-bit | i16    | u16      |
| 32-bit | i32    | u32      |
| 64-bit | i64    | u64      |
| arch   | isize  | usize    |

*Table 3-1: Integer types in Rust. The code, for example i32, is used to define a type in a function.*

<!--In the book we'll have tables and any figures numbered, with captions, and we'll call out each instance with a reference in the text. I've added an example here, feel free to edit---> 

Each variant can be either signed or unsigned, and has an explicit size. Signed and unsigned merely refers to whether the number can be negative or positive. An unsigned number can only be positive, while a signed number can be either positive or negative. It's like writing numbers on paper: when the sign matters, a number is shown with a plus sign or minus sign, but when it's safe to assume the number is positive, it's shown with no sign. Signed numbers are stored using two’s complement representation.

<!--- I'm not sure what two's complement is; does it need explaining, or will the reader know? -->
<!-- it’s something that can be googled; it’s a very common way to represent numbers in hardware, which seems a bit out of place to explain here -->

Finally, the `isize` and `usize` types depend on the kind of computer your program is running on: 64-bits if you're on a 64-bit architecture, and 32-bits if you’re on a 32-bit architecture.

<!-- On first read, I interpreted the above as "we can't have 32-bit or 16-bit ints on a 64-bit computer." Did you 
     mean that the maximum number size differs depending on your architecture, not the size in general? -->
<!-- your edit made it confusing, the _isize_ and _usize_ types change, not numbers generally. Try this version? -->


So how do you know which type of integer to use? If you're unsure, Rust's defaults are generally good choices, and integer types default to `i32`: it’s generally the fastest, even on 64-bit systems. The primary situation in which you'd need to specify `isize` or `usize` is when indexing some sort of collection, which we'll talk about in the "Arrays" section.

<!--- If you're on a 64 bit system, would you still use the i32 default? Can you say something on that here?--->
<!-- yes, edited -->

#### Floating-Point Types

Rust also has two primitive types for *floating-point numbers*, which are just numbers with decimal points, as usual. Rust's floating-point types are `f32` and `f64`, which are 32 bits and 64 bits in size, respectively. The default type is `f64`, as it’s roughly the same speed as `f32`, but has a larger precision. Here's an example showing floating-point numbers in action:

<!--- Perhaps mention why the default for floating numbers is 64 but for integers 32?---> 
<!-- done. We had months of arguments about what the defaults should be, so I had blocked that out of my memory ;) -->

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

Floating-point numbers are represented according to the IEEE-754 standard. The `f32` type is a single-precision float, while `f64` has double-precision.

<!--- What does this mean for the reader learning Rust?---> 
<!-- it’s about the guarantees about the way that these numbers are implemented. As a low-level language, these kinds of details can matter to you, depending on what you’re doing -->

#### Numeric Operations

Rust supports the usual basic mathematic operations you’d expect for all of these number types--addition, subtraction, multiplication, division, and modulo. This code shows how you'd use each one in a `let` statement:

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // modulus
    let remainder = 43 % 5;
}

```

Each expression in these statements uses a mathematical operator and evaluates to a single value, which is then bound to a variable.

<!--- Perhaps move this to the end of the section, to keep all the types together. Also, are there are operators specific to Rust that differ to what you might expect? If so, I'd list them here --->
<!-- there’s nothing specific, no -->

#### The Boolean Type

As in most other programming languages, a boolean type has two possible values: `true` and `false`. The boolean type in Rust is specified with `bool`. For example:

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explict type annotation
}
```

The main way to consume boolean values is through conditionals like an `if` statement. We’ll cover how `if` statements work in Rust in the "Control Flow" section of this chapter.

#### The Character Type

So far we’ve only worked with numbers, but Rust supports letters too. Rust’s `char` type is the language's most primitive alphabetic type, and this code shows one way to use it:

```rust
fn main() {
   let c = 'z';

   let z = 'ℤ';
}
```

Rust’s `char` represents a Unicode Scalar Value, which means that it can represent a lot more than just ASCII. (You can learn more about Unicode Scalar Values at *http://www.unicode.org/glossary/#unicode_scalar_value*) A "character" isn’t really a concept in Unicode, however, so your human intutition for what a "character" is may not match up with what a `char` is in Rust. It also means that `char`s are four bytes each.

<!--- Can you explain this a little? In what way does it represent more than just ASCII? How might it fall short of expectations? I'm also just a bit confused in general, as there are lists of "Unicode characters." --> 
<!-- I’ll give this some thought. Unicode is incredibly tough, you could write a book on just it alone -->

<!--- We won't be able to embed links in the text, so when you want to include a link can you introduce it in a sentece, something like the above? I checked this link out, and it didn't clarify much to me on its own, so maybe talk readers through it a little [Unicode Scalar Value]: http://www.unicode.org/glossary/#unicode_scalar_value---> 
<!-- ahh, this is gonna be a pain to convert back and forth. I can give it a try... sorry for the ones I miss in the future -->

### Compound Types

*Compound types* can group multiple values of other types into another type. Rust has two primitive compound types: tuples and arrays. You can put a compound type inside a compound type as well.

<!--- Could you fill in the blanks, above? If some readers may not be familiar with the concept of a compound type, then I suggest explaining that in a bit more detail, too. Can they also group other compound types, or just scalar? ---> 
<!-- edited! -->

#### Grouping Values into Tuples

We’ve seen tuples already, when binding or returning multiple values at once. A tuple is a general way of grouping together some number of other values with distinct types into one compound type. The number of values is called the *arity* of the tuple.

We create a tuple by writing a comma-separated list of values inside parentheses. Each position in the tuple has a distinct type, as in this example:

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

Note that, unlike the examples of multiple bindings, here we bind the single name `tup` to the entire tuple, emphasizing the fact that a tuple is considered a single compound element. We can then use pattern matching to destructure this tuple value, like this:

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

In this program, we first create a tuple, and bind it to the name `tup`. We then use a pattern with `let` to
take `tup` and turn it into three separate bindings, `x`, `y`, and `z`. This is called ‘destructuring’, because
it breaks the single tuple into three parts.

Finally, we print the value of `x`, which is `6.4`.

#### Tuple Indexing

In addition to destructuring through pattern matching, we can also access a tuple element directly by using a period (`.`) followed by the index of the value we want to access. For example:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

<!--- Let's say what this is doing: are we binding the values of the tuple to corresponding names by calling their index?--> 
<!-- added :) -->

This program creates a tuple, `x`, and then makes new bindings to each element by using their index.

As with most programming languages, the first index in a tuple is 0.



#### Single-Element Tuples

Not everything contained within parentheses is a tuple in Rust. For example, a `(5)` may be a tuple, or just a `5` in parentheses. To disambiguate, use a comma for single-element tuples, as in this example:

```rust
fn main() {
    let x = (5);  

    let x = (5,); 
}

```

<!--It's good to avoid putting long or important notes in code comments, people are more likely to skip or miss them ---> 

In the first `let` statement, because `(5)` has no comma, it's a simple i32 and not a tuple. In the second `let` example, `(5,)` is a tuple with only one element.

### Arrays

So far, we’ve only represented single values in a binding. Sometimes, though, it’s useful to bind a name to more than one value. Data structures that contain multiple values are called *collections*, and arrays are the first type of Rust collection we’ll learn about.

<!--- This seems a bit ambiguous, can you give a more direct definition of a collection? Perhaps something like, 'A data structure that contains multiple values is called a collection', (as an example, I'm sure that's incorrect!) -->
<!-- no that’s reasonable, edited :) -->


In Rust, arrays look like this:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

The values going into an array are written as a comma separated list inside square brackets. Unlike a tuple,
every element of an array must have the same type.

<!--- Can you say how this is different to a tuple, structurally? -->
<!-- I added a sentence, that’s a good call. They’re very, very similar -->

#### Type Annotation for Arrays

When you specify an array’s type, you'd do so as such:

```rust
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
}
```

Much like in a variable binding that uses type annotation, the array's type and length come after the pattern name and a colon. This array has `5` values, which are of the `i32` type. Unlike the values themselves, the type and array length are separated by a semicolon.

####Accessing and Modifying Array Elements

An array is a single chunk of memory, allocated on the stack. We can access elements of an array using indexing, like this:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

<!-- I've taken a stab at fleshing out/clarifying the next several paragraphs, as I had some trouble following. Please
     do tweak any suggestions that don't make sense, though. --> 
<!-- will do -->

In this example, the `first` variable will bind to `1` at index `[0]` in the array, and `second` will bind to `2` at index `[1]` in the array. Note that these values are copied out of the array and into `first` and `second` when the `let` statement is called. That means if the array changes after the `let` statements, these bindings will not, and the two variables should retain their values. For example, imagine you have the following code:



```rust
fn main() {
    let mut a = [1, 2, 3, 4, 5];

    let first = a[0];

    a[0] = 7;

    println!("The value of first is: {}", first);
}
```

<!--- Can an array contain elements of different types? Would be good to say so explicitly, we've only shown examples of tuples and arrays containing values of the same type. --> 
<!-- i already added it above, good catch! -->

First, notice the use of `mut` in the array declaration. We had to declare array `a` as `mut` to override Rust's default immutability. The line `a[0] = 7;` modifies the element at index 0 in the array, changing its value to `7`. This happens after `first` is bound to the original value at index 0, so `first` should still be equal `1`. Running the code will show this is true:

```rust
The value of first is: 1
```

Since `a[0]` didn't change until after `first` was assigned a value, the `println` macro replaced the `{}` with `1`, as expected.

<!---If we didn’t want a copy, but instead wanted to refer to the first element, whatever its value was, we need a new concept. We’ll talk about ‘references’ in Section 4.  -->             

<!-- Let's discuss references when we get there, as this won't mean much to the reader right now. -->  

## Macros and Data Structures <!-- Or something else descriptive. -->

Now that we've discussed data structures a little, there are a couple of relevant macro concepts we should cover: the `panic!` macro  and `Debug`, which is a new way of printing data to the terminal.

<!-- Could you say above why it's important to cover these now, or is this just the point at which we can understand
     these concepts? Why are they important? -->
<!-- so these sections are really awkward, but are important to fully understand some of the examples. Maybe we can cut this section for now and just handwave those bits? I dunno -->

### Rectifying Invalid Indexes with `Panic!`

Rust calls the `panic` macro when a program tries to access elements of an array (or any other data structure) and gives an invalid index. For an example, use the `functions` project we created on page XX and change your *src/main.rs* to look like this:

```rust,should_panic
fn main() {
    let a = [1, 2, 3, 4, 5];

    let invalid = a[10];

    println!("The value of invalid is: {}", invalid);
}
```

This program tries to access an element at index 10 in the `a` array. If we run it, we will get an error like this:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
thread ‘<main>’ panicked at ‘index out of bounds: the len is 5 but the index is 10’, src/main.rs:4
Process didn’t exit successfully: `target/debug/functions` (exit code: 101)
```

The output tells us that our thread panicked, and that our program didn’t exit successfully. It also gives the reason: we requested an index of 10 from an array with a length of 5.

So why did this cause Rust to panic? An array knows how many elements it holds. When we attempt to access an element using indexing, Rust will check that the index we've specified is less than the array length. If the index is greater than the length, it will panic. This is our first example of Rust’s safety principles in action. In many low-level languages, this kind of check is not done, and when you provide an incorrect index, invalid memory can be accessed. Rust protects us against this kind of error. We'll discuss more of Rust’s error handling in Chapter xx.

### Using Debug in the println Macro

So far, we’ve been printing values using `{}` in a `println` macro. If we try that with an array, however, we'll get an error. Say we have the following program:

```rust,ignore
fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("a is: {}", a);
}
```

This code tries to print the `a` array directly, which may seem innocuous. But running it produces the following output:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
src/main.rs:4:25: 4:26 error: the trait `core::fmt::Display` is not implemented for the type `[_; 5]` [E0277]
src/main.rs:4     println!(“a is {}”, a);
                                      ^
<std macros>:2:25: 2:56 note: in this expansion of format_args!
<std macros>:3:1: 3:54 note: in this expansion of print! (defined in <std macros>)
src/main.rs:4:5: 4:28 note: in this expansion of println! (defined in <std macros>)
src/main.rs:4:25: 4:26 help: run `rustc --explain E0277` to see a detailed explanation
src/main.rs:4:25: 4:26 note: `[_; 5]` cannot be formatted with the default formatter; try using `:?` instead if you are using a format string
src/main.rs:4:25: 4:26 note: required by `core::fmt::Display::fmt`
error: aborting due to previous error
```

<!--- Above, I've re-phrased this paragraph to exclude anything the reader won't know yet. Referring forward too much can just be confusing for readers and make them feel like they've missed something. -->
<!-- well done --> 

Whew! The core of the error is this part: the trait `core::fmt::Display` is not
implemented. We haven’t discussed traits yet, so this is bound to be confusing!
Here’s all we need to know for now: `println!` can do many kinds of formatting.
By default, `{}` implements a kind of formatting known as `Display`: output
intended for direct end-user consumption. The primitive types we’ve seen so far
implement `Display`, as there’s only one way you’d show a `1` to a user. But
with arrays, the output is less clear. Do you want commas or not? What about
the `[]`s?

More complex types in the standard library do not automatically implement `Display` formatting. Instead, Rust implements another kind of formatting, also intended for the programmer. This formatting type is called `Debug`. To ask `println!` to use `Debug` formatting, we include `:?` in the print string, like this:

<!--- By 'programmer consumption', do you just mean the user? ---> 
<!-- the author of the program, not the user of the program -->

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("a is {:?}", a);
}
```

If you run this, it should print the five values in the `a` array as desired:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
a is [1, 2, 3, 4, 5]
```

You’ll see this repeated later, with other types. We’ll cover traits fully in Chapter 9.

## Comments

All programmers strive to make their code easy to understand, but sometimes some extra explanation is warranted. In these cases, we leave notes in our source code that the compiler will ignore. These notes are called *comments*.

Here’s a simple comment:

```rust
// Hello, world.
```

In Rust, comments must start with two slashes, and will last until the end of the line. For comments that extend beyond a single line, you'll need to include `//` on each line, like this:

```rust
// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.
```

Comments can also be placed at the end of lines of code:

```rust
fn main() {
    let lucky_number = 7; // I’m feeling lucky today.
}
```

But you’ll more often see them above, like so:

```rust
fn main() {
    // I’m feeling lucky today.
    let lucky_number = 7;
}
```

That’s all there is to it. Comments are not particularly complicated.

### Documentation Comments

<!--- I'm not yet clear on how this differs from a normal comment. In what way will the compiler pay attention? Will it place the comment for you? Why would you need to use a normal comment, if this is better - are there disadvantages?  --->
<!-- isn’t that specifically what the following paragraph explains? it’s that the documentation generation tool can use them. Any suggestions on how to make it more clear? -->

Rust has another kind of comment: a *documentation comment*. These comments don’t affect the way that the code works, but they do work with Rust’s tools. More specifically, the `rustdoc` tool can read documentation comments and produce HTML documentation from them. Documentation comments use an extra slash, like this:

```rust
/// The foo function doesn’t really do much.
fn foo() {

}

/// Documentation comments can use
/// multiple line comments too,
/// like we did before.
fn bar() {

}
```

The `rustdoc` tool would interpret each comment in this example as documenting the thing
that follows it. The first comment would be used to document the `foo()` function, and the second comment would document the `bar()` function.

Because documentation comments have semantic meaning to `rustdoc`, the compiler will pay attention to the placement of your documentation comments. For example, a program containing only this:

```rust,ignore
/// What am I documenting?
```

Will give the following compiler error:

```text
src/main.rs:1:1: 1:27 error: expected item after doc comment
src/main.rs:1 /// What am I documenting?
              ^~~~~~~~~~~~~~~~~~~~~~~~~~
```

This happens because Rust expects a document comment to be associated with whatever code comes directly after it, so it sees that a document comment alone must be a mistake.

<!---Let's say why this produces an error, I've made an attempt above, but I'm not sure if this is what you mean. ---> 

## Control Flow with `if`

<!-- Control Flow may merit a chapter of its own, since we create new projects for both if statements and loops. 
     If not, I would consider having a "Control Flow" section, where "if Statements" and "Loops" are subheads. -->
<!-- I’d be okay with that -->

> Two roads diverged in a yellow wood,  
> And sorry I could not travel both  
> And be one traveler, long I stood  
> And looked down one as far as I could  
> To where it bent in the undergrowth;  
> 
> - Robert Frost, “The Road Not Taken”

<!---Has the copyright for this poem been checked? I think it's okay, if it was published prior to 1923, but the Frost estate has hunted people for rights before! -->
<!-- 1916 :) -->

<!--- In Rust, there are a few ways to cause our code to branch. The most fundamental

way is by using `if`. An `if` expression gives the path two paths forward, and asks the question, “Which one should I take?”

I'm not sure this is a very clear definition, but I've made a suggestion below for a more direct definition, can you please check?--->

<!-- seems good -->

In Rust, as in most programming languages, an `if` expression allows us to branch our code depending on conditions. We provide a condition, and then say, `if` this condition is met, then run this block of code; `if` the condition is not met, run a different block of code (or stop the program).

Let’s make a new project to explore `if`. Navigate to your projects directory,
and use Cargo to make a new project called `branches`:

```bash
$ cargo new --bin branches
$ cd branches
```

Write this sample program using `if` and save it in the *branches* directory:

```rust
fn main() {
    let condition = true;

    if condition {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}   
```

The `condition` variable is a boolean; here, it's set to true. All `if` statements start with `if`, which is followed by a condition. The block of code we want to execute if the condition is true goes immediately after the condition, inside curly braces. These blocks are sometimes called ‘arms’. We can also include an `else` statement, which gives the program a block of code to execute should `condition` evaluate to false.

Try running this code, and you should see output like this:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
condition was true
```

<!--- Let's talk through this code before moving on, even if it's only a sentence --> 
<!-- what do you think about this edit? since the explanation is to contrast the two -->

Before we talk about what’s happening here, let’s try changing the value of `condition` to `false` as follows:

```rust
    let condition = false;
```

Run the program again, and look at the output:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
condition was false
```

This time the second block of code is run, the `else` block is executed. This is the very basic structure of `if`: _if_ the condition is true, then execute some code. If it’s not true, then execute some other code. When an `else` block is included, that "other code" is the code in the `else` block. You could also not use an `else` expression, as in this example:

```rust
fn main() {
    let condition = false;

    if condition {
        println!("condition was true");
    }
}
```

In this case, nothing would be printed, because there is no code after the `if` block.

<!--- It might make more sense to switch these two examples, give the code using only the if statement, then give the alternative using an else statement--->
<!-- hmm -->

It’s also worth noting that `condition` here _must_ be a `bool`. To see what happens if the condition isn't a `bool`, try running this code:

<!--- So is there never a case where a non-bool value can be used? Or is it just that you need to specify? Would it be accurate to say, "the condition must evaluate to a boolean"? --> 
<!-- it must be exactly a boolean. something evaluating to a boolean works, as that’s a boolean, but rust doesn’t have ‘truthy’ and ‘falsy’ like many languages -->

```rust,ignore
fn main() {
    let condition = 5;

    if condition {
        println!("condition was five");
    }
}
```

The `condition` variable is assigned a value of `5` this time, and Rust will complain about it:

```bash
   Compiling branches v0.1.0 (file:///projects/branches)
src/main.rs:4:8: 4:17 error: mismatched types:
 expected `bool`,
    found `_`
(expected bool,
    found integral variable) [E0308]
src/main.rs:4     if condition {
                     ^~~~~~~~~
src/main.rs:4:8: 4:17 help: run `rustc --explain E0308` to see a detailed explanation
error: aborting due to previous error
Could not compile `branches`.
```

The error tells us that Rust expected a `bool`, but got an integer. Rust will not automatically try to convert non-boolean types to a boolean here. We must be explicit.

<!--  It seems like we're saying that there *is* a way to use other values, is that right? Is that what we need to be explicit about? If so, this is a good place to discuss it --> 
<!-- nope, there is not. -->

### Multiple Conditions with `else if`

We can set multiple coniditions by combining `if` and `else` in an `else if` expression. For example: 

```rust
fn main() {
    let number = 5;

    if number == 3 {
        println!("condition was 3");
    } else if number == 4 {
        println!("condition was 4");
    } else if number == 5 {
        println!("condition was 5");
    } else {
        println!("condition was something else");
    }
}
```

This program three possible paths it can take after checking the condition, and if you try running it, you should see output like this:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
condition was 5
```

When this program executes, it will check each `if` expression in turn, and execute the first body for which the condition holds true.

<!-- That's a really nice explanation. :)  This is the kind of thing that's really useful to have with each piece of example code - simple and straightforward --> 
<!-- thanks :) -->

Using too many `else if` expressions can clutter your code, so if you find yourself with more than one, you may want to look at refactoring your code. In Chapter XX, we'll talk about a powerful Rust branching construct called `match` for these cases.

### Using `if` in a Binding

The last detail you need to learn about `if` is that it’s an expression. That means that we can use it on the right hand side of a `let` binding, for instance:

```rust
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
``` 

The `number` variable will be bound to a value based on the outcome of the `if` expression. Let’s run this to see what happens:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
The value of number is: 5
```

Remember, blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions. In this case, the value of the whole `if` expression depends on which block of code executes. This means that the value in both arms of the `if` must be the same type; in the previous example, they were both `i32` integers. But what happens if the types are mismatched, as in the following example?

```rust,ignore
fn main() {
    let condition = true;

    let number = if condition {
        5
    } else {
        "six"
    };

    println!("The value of number is: {}", number);
}
```

The expression in one block of the `if` statement, is an integer and the expresion in the other block is a string. If we try to run this, we’ll get an error:

```bash
   Compiling branches v0.1.0 (file:///projects/branches)
src/main.rs:4:18: 8:6 error: if and else have incompatible types:
 expected `_`,
    found `&‘static str`
(expected integral variable,
    found &-ptr) [E0308]
src/main.rs:4     let number = if condition {
src/main.rs:5         5
src/main.rs:6     } else {
src/main.rs:7         "six"
src/main.rs:8     };
src/main.rs:4:18: 8:6 help: run `rustc --explain E0308` to see a detailed explanation
error: aborting due to previous error
Could not compile `branches`.
```

The `if` and `else` arms have value types that are incompatible, and Rust tells us exactly where to find the problem in our program. This can’t work, because variable bindings must have a single type.

<!-- Maybe mention briefly why this won't work? Because Rust can't make a direct comparison between two types? -->
<!-- added a sentence about it -->

## Control Flow with Loops

It’s often useful to be able to execute a block of code more than one time. For this, Rust has several constructs called *loops*. A loop runs through the code inside it to the end and then starts immediately back at the beginning. To try out loops, let’s make a new project. Navigate to your *projects* folder and use Cargo to make a new project:

```bash
$ cargo new --bin loops
$ cd loops
```

There are three kinds of loops in Rust: `loop`, `while`, and `for`. Let’s dig
in.

### Repeating Code with `loop`

<!-- I think we should cover the difference between keyword and statement before using the terms - I mentioned this in a note earlier in Functions. -->  

The `loop` keyword tells Rust to execute a block of code over and over again forever, or until we explicitly kill it.

For an example, change the *src/main.rs* file in your *loops* directory to look like this:

```rust,ignore
fn main() {
    loop {
        println!("again!");
    }
}
```

If we run this program, we’ll see `again!` printed over and over continuously until we kill the program manually. Most terminals support a keyboard shortcut, `control-c`, to kill a program stuck in a continual loop. Give it a try:

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
That `^C` there is where I hit `control-c`. Fortunately, Rust provides another, more reliable way to break out of a loop. We can place the `break` keyword within the loop to tell the program when to stop executing the loop. Try this version out of the program:

```rust
fn main() {
    loop {
        println!("once!");
        break;
    }
}
```

If you run this program, you’ll see that it only executes one
time:

```bash
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
     Running `target/debug/loops`
once!
```


When a Rust program hits a `break` statement, it will exit the current loop. This on its own is not very useful; if we wanted to print somtheing just once, we wouldn't put it in a loop. This is where conditions come in again.

<!-- It seems like if you wanted to print it once, a user probably wouldn't use a loop, so is it that it's only really useful when used in conjunction with while? I've added this above, please do edit if it's incorrect --> 
<!-- yes, and in fact it’s an error to use it outside of a loop for this reason -->

### Conditional Loops With `while`

To make `break` useful, we need to give our program a condition. While the condition is true, the loop runs. When the condition ceases to be true, the `break` code runs, stopping the loop.

<!-- I've added a little in, to give a more comprehensive idea early on - could you please check and edit where necessary? -->
<!-- done -->

Try this example:

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

If we run this, we’ll get:

```bash
   Compiling loops v0.1.0 (file:///projects/loops)
     Running `target/debug/loops`
3!
2!
1!
LIFTOFF!!!
```

<!--- Could you give a short explanation of what the program did here? --> 
<!-- below :) -->

This program loops three times, counting down each time. Finally, after the
loop, it prints another message, then exits.

The core of this example is in the combination of these three constructs:

```rust,ignore
    loop {
        if number != 0 {
            // do stuff
        } else {
            break;
        }
```

We want to `loop`, but only while some sort of condition is true. As soon as it isn't, we want to `break` out of the loop. This pattern is so common that Rust has a more efficient language construct for it, called a `while` loop. Here's the same example, but using `while` instead:


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

This gets rid of a lot of nesting, and it's more clear. While a condition holds,
run this code.

### Looping Though a Collection with `for`

We can use this `while` construct to loop over the elements of a collection, like an array. For example:

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

Here, we're counting up through the elements in the array. We start at index 0, then loop until we hit the final index of our array (that is, when `index < 5` is no longer true). Running this will print out every element of the array:

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


All five array values appear in the terminal, as expected. Even though `index` will reach a value of `6` at some point, the loop stops executing before trying to fetch a sixth value from the array. 

This approach is error-prone, though; we could trigger a `panic!` by getting the index length incorrect. It's also slow, as the compiler needs to perform the conditional check on every element on every iteration through the loop.

As a more efficient alternative, we can use a `for` loop. A `for` loop looks something like this:

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

<!--- Perhaos explain the difference a little more, and how this handles the same operations differently to while --> 

** I'm going to leave it at this for now until we decide how we want to do it**

