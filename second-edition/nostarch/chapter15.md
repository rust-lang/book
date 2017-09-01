
[TOC]

# Smart Pointers

*Pointer* is a generic programming term for a reference to a location that
stores some data. In Chapter 4 we learned that a plain pointer, in Rust
indicated by the `&` symbol, borrows the value it points to. *Smart pointers*
on the other hand are data structures that act like pointers, but that have
additional metadata and capabilities, such as reference counting. Unlike
regular pointers, smart pointers also implement the `Deref` and `Drop` traits,
and in this chapter we'll be discussing both of those traits and why they're
important to smart pointers.

<!-- maybe a brief explanation what deref and drop? I'm not really sure what
reference counting is here too, can you outline that in brief?-->

The pattern for smart pointers originated in C++.

<!--if this is relevant here, can you expand? Are we saying they will be
familiar to C++ people? -->

In Rust, smart pointers can only borrow data, whereas in many other langauges,
smart pointers *own* the data they point to.

<!-- Is this to help keep Rust memory safe? Let's sell Rust a bit here, if so!
-->

We've actually already encountered a few smart pointers in this book, such as
`String` and `Vec<T>` from Chapter 8, though we didn't call them smart pointers
at the time. Both these types count as smart pointers because they own some
memory and allow you to manipulate it, and have metadata (like data on their
capacity) and extra capabilities or guarantees; for example, `String` data will
always be valid UTF-8.

<!-- Above: we said smart pointers don't own values earlier but in the
paragraph above we're saying String and Vec own memory, is that a
contradiction? -->

Given that the smart pointer pattern is a general design pattern used
frequently in Rust, this chapter won't cover every smart pointer that exists.
Many libraries have their own smart pointers and you can even write some
yourself. We'll just cover the most common smart pointers and associated
features from the standard library, including:

<!-- Would it make sense to hyphenate reference-counted (and its derivations)
here? I think that would be more clear, but I don't want to do that if that's
not the Rust convention -->

* `Box<T>` for allocating values on the heap
* `Rc<T>`, a reference counted type that allows multiple owners for data
* `RefCell<T>`, a type that manages access to the
  smart pointers `Ref` and `RefMut` to enforce the borrowing rules at runtime
  instead of compile time, but is not actually a smart pointer itself.

<!-- Should we add Ref and RefMut to this list, too? -->

Along the way, we'll cover the *interior mutability* pattern where an immutable
type exposes an API for mutating an interior value. We'll also cover the
borrowing rules specific to smart pointers, which apply at runtime instead of
compile time, and *reference* cycles, how they can leak memory, and how to
prevent them.

Let's dive in!

## `Box<T>` Points to Data on the Heap and Has a Known Size

The most straightforward smart pointer is a *box*, whose type is written
`Box<T>`. Boxes allow you to place a single value on the heap while the box
itself is stored on the stack (we talked about the stack vs. the heap in
Chapter 4).

<!-- do we mean, allows you to place a value on the heap rather than the
default behavior of placing it on the stack? Can you quickly recap on what the
advantage to this can be, help them know when they'd use this? -->

Listing 15-1 shows how to use a box to store an `i32` on the heap:

Filename: src/main.rs

```
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

Listing 15-1: Storing an `i32` value on the heap using a box

We define a box `b` that will contain the value `5`; this program will print `b
= 5`. We can access the data in the box the same as if this data was on the
stack. Just like any value that has ownership of data, when a box goes out of
scope like `b` does at the end of `main`, both the box, stored on the stack,
and the data it points to stored on the heap will be deallocated.

However, putting a single value on the heap isn't very useful, and you won't
often use boxes by themselves like in Listing 15-1 since we can do the same
thing more simply with variables.

<!-- is this what we mean, we wouldn't bother with a box for something that can
be done more simply with a variable? -->

Boxes are most useful when you want to ensure that your type has a known size.
For an example we'll create an enum definition for a cons list, a type of data
structure that comes from functional programming.

### Using Box to Fix Recursive Types

<!-- (or something that encompasses everything we do with this example) -->

<!-- below: I'm unfamiliar with the cons concept, are we saying each value
except the first is repeated? does an item contain both its own value and the
next **item**, or the next **value**? Is it a continually nesting list? I'm
finding it hard to visualize -->

<!-- can you also say why we're discussing cons lists in such depth? It seems
like a detour from the smart pointers conversation, is it just another concept
we're covering or is it imperative for learning about smart pointers? Either
way, can you lay that out up front, I thnk this could throw readers -->

A *cons list* is a list whose each item contains two thing: a value and the
next item until. The end of the list is signified by a value called `Nil`. Note
that we aren't introducing the idea of "nil" or "null" that we discussed in
Chapter 6, this is just a regular enum variant name we're using because it's
the canonical name to use when describing the cons list data structure. Cons
lists aren't used very often in Rust, `Vec<T>` is a better choice most of the
time, but implementing this data structure is useful as an example for <box>.

<!-- If there isn't a better example for introducing box, I think we need more
justification for using cons lists here. This is supposed to be showing why box
is useful, but we're saying the thing we use box for isn't useful either. What
is it useful for, then? -->

Listing 15-2 shows a first try at defining a cons list as an enum; note that
this won't compile quite yet:

<!-- why won't it compile? Are we just defining it to use in the next example?
Can you make it clear to the reader why they are doing this?-->

Filename: src/main.rs

```
enum List {
    Cons(i32, List),
    Nil,
}
```

Listing 15-2: The first attempt of defining an enum to represent a cons list of
`i32` values

<!-- any reason, in that case, that we use i32s here? Does it just provide a
more stable example? -->

> Note: We're choosing to implement a cons list that only holds `i32` values as
> our example. However we could have also implemented it using generics, as we
> discussed in Chapter 10, to define a cons list concept independent of the
> type of value stored in the cons list.

We can use a cons list to store the list `1, 2, 3` using our `List` enum like
so:

```
use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

The first `Cons` value holds `1` and another `List` value. This `List` value is
another `Cons` value that holds `2` and another `List` value. This is one more
`Cons` value that holds `3` and a `List` value, which is finally `Nil`, the
non-recursive variant that signals the end of the list.

If we try to compile the above code, we get the error shown in Listing 15-3:

```
error[E0072]: recursive type `List` has infinite size
 -->
  |
1 |   enum List {
  |  _^ starting here...
2 | |     Cons(i32, List),
3 | |     Nil,
4 | | }
  | |_^ ...ending here: recursive type has infinite size
  |
  = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to
  make `List` representable
```

Listing 15-3: The error we get when attempting to define a recursive enum

The error says this type 'has infinite size'. The reason is that we've defined
`List` to have a variant that is recursive: it holds another value of itself.

<!-- above-- but isn't that the definition of a cons list that we gave earlier,
that is must hold a value of itself? As you can see, I'm struggling with the
cons definition at the moment! -->

This means Rust can't figure out how much space it needs to store a `List`
value. Let's break this down a bit: first let's look at how Rust decides how
much space it needs to store a value of a non-recursive type.

#### How Rust Determines the Required Storage Space

Recall the `Message` enum we defined in Listing 6-2 when we discussed enum
definitions in Chapter 6:

```
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

To determine how much space to allocate for a `Message` value, Rust goes
through each of the variants to see how much maximum space is needed. Rust sees
that `Message::Quit` does not need any space, `Message::Move` needs enough
space to store two `i32` values, and so forth. Since only one variant will end
up being used, the most space a `Message` value will need is the space it would
take to store the largest of its variants.

Contrast this to how Rust determines how much space a recursive type like
`List` in Listing 15-2 needs. The compiler starts by looking at the `Cons`
variant, which holds a value of type `i32` and a value of type `List`, so
`Cons` needs an amount of space equal to the size of an `i32` plus the size of
a `List`. To figure out how much memory a `List` needs, it looks at its
variants, starting with the `Cons` variant. The `Cons` variant holds a value of
type `i32` and a value of type `List`, and this continues infinitely, as shown
in Figure 15-4.

<img alt="An infinite Cons list" src="img/trpl15-01.svg" class="center" style="width: 50%;" />

Figure 15-4: An infinite `List` consisting of infinite `Cons` variants

Rust can't figure out how much space to allocate for recursively defined types,
so the compiler gives the error in Listing 15-3, which includes this helpful
suggestion:

```
= help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to
        make `List` representable
```

Because a `Box<T>` is a pointer, Rust always know how much space it needs: a
pointer takes up a `usize` amount of space. The value of the `usize` will be
the address of the heap data. The heap data can be any size, but the address to
the start of that heap data will always fit in a `usize`. So if we change our
definition from Listing 15-2 to include a `Box`, it will compile.

#### Defining Storage Space in a Cons List with Box

Listing 15-5 shows a working version of Listing 15-X. We change `main` to use
`Box::new` for the values inside the `Cons` variants like so:

Filename: src/main.rs

```
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}
```

Listing 15-5: Definition of `List` that uses `Box<T>` in order to have a known
size

The `Cons` variant will need the size of an `i32` plus the space to store a
`usize`, since a box always has the size of a `usize`, no matter what it's
pointing to. The `Nil` variant stores no values and doesn't need any space.
With a box, we've broken the infinite, recursive chain so the compiler is able
to figure out the size it needs to store a `List` value. Figure 15-6 shows what
the `Cons` variant looks like now:

<img alt="A finite Cons list" src="img/trpl15-02.svg" class="center" />

Figure 15-6: A `List` that is not infinitely sized since `Cons` holds a `Box`

This is the main way that boxes are useful in Rust: in breaking up an infinite
data structure so that the compiler can determine its size. We'll look more
use-cases for boxes in Chapter 17, too.

<!-- Below: why use boxes for this example, then, and not a more common smart
pointer? -->

You likely won't use boxes very often, but we'll used them in our examples to
illustrate how the smart pointer pattern works. Two aspects of `Box<T>`
commonly used with smart pointers are its implementations of the `Deref` trait
and the `Drop` trait. Let's investigate how these traits work and how smart
pointers use them.

<!-- so deref and drop are features of Box and not of smart pointers? Or of
both? I'm not sure it's clear -->

## Treating Smart Pointers like Regular References with the `Deref` Trait

The `Deref` trait allows us to override the *dereference operator* `*`(as
opposed to the multiplication or glob operator), which makes accessing the data
behind the smart pointer convenient. The dereference operator, mentioned
briefly in Chapter 8, allows us to change which value a reference is pointing
to by first dereferencing the existing reference, meaning we cut the connection
between the reference and the value it points to. In this context, convenient
means

<!-- Why would we want to override the dereference operator? Can you lay that
out? -->

<!-- I'd suggest introducing what you mean by "convenient" here, if we are
using it as the reason we want to use Deref -->

<!-- We briefly mentioned the dereference operator in Chapter 8, in the section
"Update a Value Based on the Old Value" on hash maps. There we had a mutable
reference, and we wanted to change the value the reference was pointing to. In
order to do that, we had to first dereference the existing reference, meaning
we cut the connection between the reference and the value it pointed to.-->

<!-- I want to avoid too much cross referencing, I think it can be distracting,
make the reader feel they need to flip back but they don't really, here -->

<!--Oh! I see, de-reference, meaning we cut the tie betweewn the data and the
reference? I've assumed so above, please correct if not! -->

We'll start with an example of dereferencing and re-allocating references to
`i32` values:

<!-- Is this what this is an example of? -->

```
let mut x = 5;
{
    let y = &mut x;

    *y += 1
}

assert_eq!(6, x);
```

With the `*y` syntax we can access the *data* that the mutable reference in `y`
refers to, rather than merely accessing the reference itself. We can then
modify that data, in this case by adding 1.

The `Deref` trait is important to the smart pointer pattern because it allows
smart pointers to be treated like regular references and used in places that
expect regular references. This means we don't have to redefine methods and
functions to take smart pointers explicitly.

<!-- Below: what does the metadata have to do with whether it can be treated
like a regular reference>? This paragraph didn't tie together for me, can you
help clear that up? -->

Regular references (that aren't smart pointers) only point to one value, so the
dereference operation is straightforward. Smart pointers, however, can also
store metadata about the pointer or the data. In order to treat a smart pointer
as a regular reference, we only want access to the data when we dereference,
and not the metadata,

<!-- below: do we mean we want to dereference the metadata so we are only left
with the data, or we only want to dereference the data so we can alter the
data, but leave the metadata intact? Opposite meanings! But I wasn't sure -->

since dereferencing a regular reference only gives us data and not metadata.
That will allow us to use smart pointers in the same places that we can use
regular references. To enable that, we can override the behavior of the `*`
operator by implementing the `Deref` trait.

<!-- Above: Are we saying that using *, rather than deref, would dereference
both the data and the metadata, so we have to override it? -->

### Overriding * with Deref to Access Data

In Listing 15-7 we'll override `*` using `Deref` on a struct that holds mp3
data and metadata. The `Mp3` struct is, in a sense, a smart pointer: it owns
the `Vec<u8>` data containing the audio. It also holds some optional metadata,
in this case the artist and title of the song in the audio data. We want to be
able to conveniently access the audio data, bypassing the metadata in order to
leave it intact. We implement the `Deref` trait to return only the audio data.
This requires that we implement the method `deref`, which borrows `self` and
returns the inner data:

Filename: src/main.rs

```
use std::ops::Deref;

struct Mp3 {
    audio: Vec<u8>,
    artist: Option<String>,
    title: Option<String>,
}

impl Deref for Mp3 {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
        &self.audio
    }
}

fn main() {
    let my_favorite_song = Mp3 {
        // we would read the actual audio data from an mp3 file
        audio: vec![1, 2, 3],
        artist: Some(String::from("Nirvana")),
        title: Some(String::from("Smells Like Teen Spirit")),
    };

    assert_eq!(vec![1, 2, 3], *my_favorite_song);
}
```

Listing 15-7: Implementing the `Deref` trait on a struct that holds mp3 file
data and metadata

Most of this should look familiar: we define a struct, add a trait
implementation, and in our main function create an instance of the struct.

The one part we haven't explained thoroughly yet is the `type Target = T;`
syntax, which defines an associated type. This is just a slightly different way
of declaring a generic parameter that you don't need to worry about too much
for now, we'll cover it in more detail in Chapter 19.

<!-- Is it possible to just use a method for declaring a generic parameter we
have seen before, so we can focus on the deref trait here? -->

The `assert_eq!` function verifies that, when we dereference the `Mp3` instance
with `*my_favorite_song` by implementing the `deref` method, our result is
`vec![1, 2, 3]`. If we hadn't implemented the `Deref` trait for `Mp3`, Rust
wouldn't compile the code `*my_favorite_song`: we'd get an error saying type
`Mp3` cannot be dereferenced.

<!-- why wouldn't it work without Deref? I don't think that's clear yet. It
isn't possible to dereference metadata? That seems to conflict with what we've
said so far-->

The reason this code works is that when we call `*my_favorite_song`, behind the
scenes, the `*` operator is calling:

<!-- why is that happening behind the scenes, rather than us just calling this
up front? -->

```
*(my_favorite_song.deref())
```

<!-- I really struggled to follow the discussion here and have tried to
reorganize more logically, can you check that I've interpreted this correctly?
-->

This calls the `deref` method on `my_favorite_song`. We defined the `deref`
method to return a reference to itself plus `.audio`, so this borrows
`my_favorite_song` and returns a reference to `my_favorite_song.audio`, giving
us access to the data but ignoring the metadata. Using `*` on references is
defined to just follow the reference and return the data, so the expansion of
`*` doesn't recurse for the outer `*`. We end up with data of type `Vec<u8>`,
which matches the `vec![1, 2, 3]` in the `assert_eq!` in Listing 15-5.

<!-- Do you mean 15-7? Also, I'm not sure what the "outer `*`" refers to? -->

If the `deref` method just returned the value and not a reference to the value,
using `*` would always take ownership, so it's necessary to dereference the
result of the method.

### Implicit Deref Coercions with Functions and Methods

<!--Below -- "A deref coercion happens when..." So this isn't something the
reader is making happen, but something that just happens behind the scene? If
not, can you change this to an active tone? -->

Rust tends to favor explicitness over implicitness, but one exception case is
in deref coercions of arguments to functions and methods. A *deref coercion* is
an implicit ==function== that automatically converts a reference to a pointer,
or a smart pointer to a reference to that pointer's contents.

<!--above -- I'm not sure "function" is the right term here, what would we call
it? Also, what is the different between a reference and a pointer? I was under
the impression a pointer *was* a reference -->

A deref coercion only happens when it is needed to make the type of the value
passed in match the type of the parameter defined in the signature. In the case
that it is necessary, the deref coercion occurs when a value is passed to a
function or method.

<!-- Above: so would a deref coercion only occur when we have a pointer as a
parameter but a reference passed in as an argument? (If there is indeed a
difference) -->

Deref coercions mean programmer don't need as many explicit references and
dereferences, with `&` and `*`, in order to call functions and methods.

As an example of deref coercions in action, using our `Mp3` struct from Listing
15-5, here's the signature of a working function that compresses mp3 audio data
and takes a slice of `u8`:

```
fn compress_mp3(audio: &[u8]) -> Vec<u8> {
    // the actual implementation would go here
}
```

If Rust didn't have deref coercion capabilities, this same function would look
like this:

<!--Would we need fn here, below? If so, maybe add it in to make it more
comparable with the one above -->

```
compress_mp3(my_favorite_song.audio.as_slice())
```

<!-- So above we said the coercions mean we dont need as many & and *, but in
this example, the one without coercions has fewer &s -- that seems confusing,
do we need to use a different example, or clear that up somehow? -->

Here we have to explicitly state that we want the data in the `audio` field of
`my_favorite_song` and that we want a slice referring to the whole `Vec<u8>`.
If we wanted to process the `audio` data in a similar manner in lots of places,
`.audio.as_slice()` would be wordy and repetitive.

With deref coercion and our implementation of the `Deref` trait on `Mp3`, we
can call this function with the data in `my_favorite_song` just using this code:

```
let result = compress_mp3(&my_favorite_song);
```

We just need an `&` and the instance, much cleaner! We can treat our smart
pointer as if it was a regular reference.

<!--This next paragraph felt dense, especially with so many code references.
I've tried to pull it out a bit, slow it down, but I think it could use more
care, can you take a look?-->

By using deref coercion, Rust knows that the `Mp3` struct implements the
`Deref` trait and returns `&Vec<u8>` from the `deref` method. Rust also knows
the standard library implements the `Deref` trait on `Vec<T>` to return `&[T]`
from the `deref` method. That means that, at compile time, Rust will see that
it can use `Deref::deref` twice to turn `&Mp3` into `&Vec<u8>` and then into
`&[T]` to match the signature of `compress_mp3`. This allows us to do less
typing!

When the `Deref` trait is defined for the types in a program, Rust will analyze
types using `Deref::deref` as many times as it needs in order to get a
reference to match the parameter's type.

<!--What do you mean by "indirection" below? -->

The indirection is resolved at compile time, so there is no run-time penalty
for taking advantage of deref coercion.

### Deref Coercions on Mutable References

<!-- below: are we talking about any mutable references, or are we talking
about mutable generic types, below? Can you make sure it's clear throughout, I
wasn't 100% -->

Rust also provides the `DerefMut` trait for overriding `*` on mutable
references in the same way. Deref coercions are applied to types and trait
implementations in three cases:

<!-- Would it mkse sense to move this list to the start of the deref setion? -->

* From `&T` to `&U` when `T: Deref<Target=U>`.
* From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`.
* From `&mut T` to `&U` when `T: Deref<Target=U>`.

The first two are the same, except for mutability: if you have a `&T`, and `T`
implements `Deref` to some type `U`, you can get a `&U` transparently. The same
is true for mutable references. The last point is more tricky: on a mutable
reference, deref coercion will also coerce the reference to an immutable one.
The other case is _not_ possible though: immutable references will never coerce
to mutable ones.

<!-- Why does it coerce to an immutable reference, and why cant it go the other
way?-->

<!-- I've moved this paragraph to the start of the deref section to help the
reader understand why we are discusing this.

The `Deref` trait is important to the smart pointer pattern because it allows
smart pointers to be treated like regular references and used in places that
expect regular references. This means we don't have to redefine methods and
functions to take smart pointers explicitly. -->

## The `Drop` Trait Runs Code on Cleanup

The second trait important to the smart pointer pattern is `Drop`, which lets
us run a chunk of code when a value is about to go out of scope.

<!-- Why? You mean some code that is relevant to the value about to go out of
scope? -->

When they're dropped, smart pointers perform important cleanup tasks, like
deallocating memory or decrementing a reference count.

<!-- using Drop, you mean? Or they do this anyway? -->

Data types in general have the abilitiy to manage resources like files or
network connections and use `Drop` to release those resources when our code is
done with them. We'll only discuss `Drop` in the context of smart pointers,
however, because the functionality of the `Drop` trait is almost always used
when implementing smart pointers.

In some languages, the programmer must call code to free the memory or resource
every time they finish using an instance of a smart pointer. If they forget,
the system might become overloaded and crash. In Rust, we can specify that a
particular bit of code should be run whenever a value goes out of scope, and
the compiler will insert this code automatically.

<!-- Are we saying that any code can be run, and that we can use that to clean
up, or that this code that can be run is specifically always for clean up? -->

This means we don't need be careful about placing clean up code everywhere in a
program that an instance of a particular type is finished with, but we still
won't leak resources!

We specify the code to run when a value goes out of scope by implementing the
`Drop` trait. The `Drop` trait requires us to implement one method named `drop`
that takes a mutable reference to `self`.

<!-- Why are we showing this as an example and not an example of it being used
for clean up? -->

Listing 15-8 shows a `CustomSmartPointer` struct that does nothing, but in the
program we print out `CustomSmartPointer created.` right after we create an
instance of the struct. Using the Drop trait, we tell it to print `Dropping
CustomSmartPointer!` when the instance goes out of scope. This will show you
when each piece of code is run:

<!-- Is this below just telling us how to adapt it for cleaning up instead?
Maybe save it for when we have context for it? Instead of a `println!`
statement, you'd fill in `drop` with whatever cleanup code your smart pointer
needs to run: -->

Filename: src/main.rs

```
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer!");
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    println!("Wait for it...");
}
```

Listing 15-8: A `CustomSmartPointer` struct that implements the `Drop` trait,
where we would put our clean up code.

The `Drop` trait is included in the prelude, so we don't need to import it. We
implement the `drop` method and tell it to call the `println!` function.
Everywhere you see a `println!` function is where you'd put the actual code
needed to close the socket.

<!-- Where you'd put this code, or where this code would be called? It seems
laborious to write this clean up code wherever there's a print call? -->

In `main`, we create a new instance of `CustomSmartPointer` then print out
`CustomSmartPointer created.` and then a second message so we can see in our
output where our code is at any point. At the end of `main`, our instance of
`CustomSmartPointer` will go out of scope, and will call the code we put in the
`drop` method, printing our final message. Note that we didn't need to call the
`drop` method explicitly.

When we run this program, we'll see the following output:

```
CustomSmartPointer created.
Wait for it...
Dropping CustomSmartPointer!
```

Rust automatically called `drop` for us when our instance went out of scope,
calling on the code we specified. This is just to give you a visual guide to
how the drop method works, but usually you would give `drop` code for XXXXX
rather than a print message.

<!-- Can you wrap this example up by saying what you would actually put in a
drop method and why?-->

#### Dropping a Value Early with drop

<!-- is this a new method from Drop or the same method? -->

We can use the `std::mem::drop` function to drop a value earlier than when it
goes out of scope. This isn't usually necessary; the whole point of the `Drop`
trait is that it's taken care of automatically for us. We'll see an example of
a case when we'll need to drop a value earlier than when it goes out of scope
in Chapter 16 when we're talking about concurrency. For now, let's just see
that it's possible, and `std::mem::drop` is in the prelude so we can just call
`drop` as shown in Listing 15-9:

<!-- Above: I'm not following why we are doing this, if it's not necesary and
we aren't going to cover it now anyway -- can you lay out why we're discussing
this here? -->

Filename: src/main.rs

```
fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("Wait for it...");
}
```

Listing 15-9: Calling `std::mem::drop` to explicitly drop a value before it
goes out of scope

Running this code will print the following:

```
CustomSmartPointer created.
Dropping CustomSmartPointer!
Wait for it...
```

<!-- What's the destructor code, here? We haven't mentioned that before, not in
this chapter in any case -->

The `Dropping CustomSmartPointer!` is printed between `CustomSmartPointer
created.` and `Wait for it...`, showing that the destructor code is called to
drop `c`, calling the `println!` code we gave to the drop method earlier.

<!-- How does this show that the destructor code (is that drop?) is called? Is
this correct, above?-->

Note that if we try to call the `drop` method that we defined directly, for
example by replacing `drop(c)` in Listing 15-9 with `c.drop()`, we'll get a
compiler error that says `explicit destructor calls not allowed`. This is
because Rust inserts its call to `Drop::drop` automatically when the value goes
out of scope, so if we also called it explicitly the value would get dropped
twice, which could cause an error or corrupt memory.

<!-- Below: use `std::mem::drop` to do what, exactly? Do we need a drop method
we can call directly if Rust does it for us anyway? I'm still lost on why we
are looking at this-->

Instead if calling `.drop()` explicitly, therefore, we use `std::mem::drop`,
whose definition is:

```
pub mod std {
    pub mod mem {
        pub fn drop<T>(x: T) { }
    }
}
```

<!--can you pick out the important bits here? I'm not sure what we're looking at -->

This function is generic over any type `T`, so we can pass any value to it. The
function doesn't actually have anything in its body so doesn't use its
parameter, but this empty function is still useful because `drop` takes
ownership of its parameter, which means the value in `x` gets dropped at the
end of this function when `x` goes out of scope.

<!--Above: why does that make it useful? You mean it's useful to see? -->

Code specified in a `Drop` trait implementation can be used in many ways to
make cleanup convenient and safe: we could use it to create our own memory
allocator, for instance! With the `Drop` trait and Rust's ownership system, you
don't have to remember to clean up after yourself, Rust takes care of it
automatically.

We also don't have to worry about accidentally cleaning up values still in use
because that would cause a compiler error: the ownership system that makes sure
references are always valid will also make sure that `drop` only gets called
once when the value is no longer being used.

Now that we've gone over `Box<T>` and some of the characteristics of smart
pointers, let's talk about a few other smart pointers defined in the standard
library.

## `Rc<T>`, the Reference Counted Smart Pointer

In the majority of cases, ownership is clear: you know exactly which variable
owns a given value. However, there are cases when a single variable may need
multiple owners.

<!-- Can you give an example or two for when a variable needs multiple owners?
-->

For this, Rust has a type called `Rc<T>`, an abbreviation for reference
counting. *Reference counting* means keeping track of the number of references
to a value in order to know if a value is still in use or not. If there are
zero references to a value, the value can be cleaned up without any references
becoming invalid.

Imagine it like a TV in a family room. When one person enters to watch TV, they
turn it on. Others can come into the room and watch the TV. When the last
person leaves the room, they turn the TV off because it's no longer being used.
If someone turns the TV off while others are still watching it, there's be
uproar from the remaining TV watchers!

`Rc<T>` is used when we want to allocate some data on the heap for multiple
parts of our program to read, and we can't determine at compile time which part
will finish using the data last. If we did know which part would finish last,
we could just make that the owner of the data and the normal ownership rules
enforced at compile time would kick in.

Note that `Rc<T>` is only for use in single-threaded scenarios; Chapter 16 on
concurrency will cover how to do reference counting in multithreaded programs.

### Using `Rc<T>` to Share Data

Let's return to our cons list example from Listing 15-5 and try to use `List`
as we defined it using `Box<T>`. We want two lists that both share ownership of
the third list, which conceptually will be something like Figure 15-10:

<img alt="Two lists that share ownership of a third list" src="img/trpl15-03.svg" class="center" />

Figure 15-10: Two lists, `b` and `c`, sharing ownership of a third list, `a`

We'll create list `a` that contains 5 and then 10, then make two more lists:
`b` that starts with 3 and `c` that starts with 4. Both `b` and `c` lists will
then continue on to the first `a` list containing 5 and 10. In other words,
both lists will try to share the first 5 and 10 list.

Trying to implement this using our definition of `List` with `Box<T>` won't
work, as shown in Listing 15-11:

Filename: src/main.rs

```
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Cons(5,
        Box::new(Cons(10,
            Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}
```

Listing 15-11: Failing at Having two lists using `Box<T>` that try to share
ownership of a third list

If we compile this, we get this error:

```
error[E0382]: use of moved value: `a`
  --> src/main.rs:13:30
   |
12 |     let b = Cons(3, Box::new(a));
   |                              - value moved here
13 |     let c = Cons(4, Box::new(a));
   |                              ^ value used here after move
   |
   = note: move occurs because `a` has type `List`, which does not
   implement the `Copy` trait
```

The `Cons` variants own the data they hold, so when we create the `b` list, `a`
is moved into the ownership of `b`. Then when we try to use `a` again when
creating `c`, we're not allowed to because `a` has been moved.

We could change the definition of `Cons` to hold references instead, but then
we'd have to specify lifetime parameters: we'd have to construct a list whose
every element lives at least as long as the list itself or the borrow checker
won't even let us compile the code.

Instead, we'll change our definition of `List` to use `Rc<T>` in place of
`Box<T>` as shown here in Listing 15-12:

<!-- And what will Rc do that's different here, how will the ownerships of a b
c change? Could you write a paragraph equivalent to the one describing the cons
variants above? That was really useful -->

Filename: src/main.rs

```
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, a.clone());
    let c = Cons(4, a.clone());
}
```

Listing 15-12: A definition of `List` that uses `Rc<T>`

We need to add a `use` statement to bring `Rc` into scope because it's not in
the prelude. In `main`, we create the list holding 5 and 10 and store it in a
new `Rc` in `a`. Then when we create `b` and `c`, we call the `clone` method on
`a`.

<!-- so we clone a? Did we do that before? I think this could use some more
explanation -->

### Cloning an `Rc<T>` Increases the Reference Count

We've used the `clone` method previously for making a complete copy of some
data. With `Rc<T>`, though, it doesn't make a full copy, but instead

<!-- So what is clone doing, if not making a complete copy? We seem to only
discuss what Rc is doing here-->

`Rc<T>` holds a *reference count*; that is, a count of how many clones exist.

<!-- Below -- can you let the reader know why we are doing this? What does it
show us/improve? Is this our working version of the code, or just illustrating
reference count? -->

To get our code working, we'll change `main` so that it has an inner scope
around list `c`, and we'll make it print out the results of a new
`Rc::strong_count` function, which will return the reference count of the `Rc`
value we pass to it. We'll talk about why this function is named `strong_count`
in the section later in this chapter about preventing reference cycles.

<!-- If we need to talk about this later, that might indicate that this chapter
is out of order --- should the section on referenec cycles come first? -->

Filename: src/main.rs

```
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("rc = {}", Rc::strong_count(&a));
    let b = Cons(3, a.clone());
    println!("rc after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, a.clone());
        println!("rc after creating c = {}", Rc::strong_count(&a));
    }
    println!("rc after c goes out of scope = {}", Rc::strong_count(&a));
}
```

Listing 15-13: Printing out the reference count

This will print out:

```
rc = 1
rc after creating b = 2
rc after creating c = 3
rc after c goes out of scope = 2
```

<!-- is there a reason we call `a` rc here, and not just `a`? -->

We're able to see that `a` has an initial reference count of one, then each
time we call `clone`, the count goes up by one. When `c` goes out of scope at
the implementation of the `Drop` trait for `Rc<T>` the count is decreased by
one. What we can't see in this example is that when `b` and then `a` go out of
scope at the end of `main`, the count is then 0, and the list is dropped. This
method allows a single variable to have multiple owners, and the count will
ensure that the value remains valid as long as any of the owners still exist.

In the beginning of this section, we said `Rc<T>` allows you to share data for
multiple parts of your program to read only through immutable references to the
`T` value. If `Rc<T>` allowed us to have a mutable reference, the progrma would
conflict with one of the the borrowing rules that we discussed in Chapter 4:
two mutable borrows to the same place can cause data races and inconsistencies.
But being able to mutate data is very useful! In the next section, we'll
discuss the interior mutability pattern and the `RefCell<T>` type that we can
use in conjunction with an `Rc<T>` to work with this restriction on
immutability.

## `RefCell<T>` and the Interior Mutability Pattern

<!-- I'm concerned here about referencing forward too much, do we need that
information from Ch 19 to understand this? Should we look at rearranging a few
things here? -->

*Interior mutability* is a design pattern in Rust for allowing you to mutate
data even when that data has immutable references, normally disallowed by the
borrowing rules. To do so, the pattern uses `unsafe` code inside a data
structure to bend Rust's usual rules around mutation and borrowing. We haven't
yet covered unsafe code; we will in Chapter 19.

<!--below: as in, we use the pattern, or it's used automatically? I'm not clear
on what's the user's responsibility with this pattern -->

The interior mutability pattern is used when you can ensure that the borrowing
rules will be followed at runtime, even though the compiler can't ensure that.
The `unsafe` code involved is then wrapped in a safe API, and the outer type is
still immutable.

Let's explore this by looking at the `RefCell<T>` type that follows the
interior mutability pattern.

### Borrowing Immutable References with `RefCell<T>`

Unlike `Rc<T>`, the `RefCell<T>` type represents single ownership over the data
it holds. So, what makes `RefCell<T>` different than a type like `Box<T>`?
Let's recall the borrowing rules we learned in Chapter 4:

1. At any given time, you can have *either* but not both:
  * One mutable reference.
  * Any number of immutable references.
2. References must always be valid.

With references and `Box<T>`, the borrowing rules' invariants are enforced at
compile time. With `RefCell<T>`, these invariants are enforced *at runtime*.
With references, if you break these rules, you'll get a compiler error. With
`RefCell<T>`, if you break these rules, you'll get a `panic!`.

<!-- Is there an advantage to having these rules enforced at different times?
-->

Static analysis, like the Rust compiler, is inherently conservative. Some
properties of code are impossible to detect by analyzing the code: the most
famous exampled is the Halting Problem, which is out of scope of this book but
an interesting topic to research if you're interested.

<!--below: can't be sure of what, exactly? Sure that the code complies with the
ownership rules? -->

Because some analysis is impossible, if the Rust compiler ==can't be sure== the
code complies with the ownership rules, it may reject a correct program; in
this way, it is conservative. If Rust were to accept an incorrect program,
users would not be able to trust in the guarantees Rust makes, but if Rust
rejects a correct program, the programmer will be inconvenienced, but nothing
catastrophic can occur. `RefCell<T>` is useful when you yourself are sure that
the borrowing rules have been followed, but the compiler is not able to
guarantee as much.

Similarly to `Rc<T>`, `RefCell<T>` is only for use in single-threaded scenarios
and will give you a compile time error if you try in a multithreaded context.
We'll talk about how to get the functionality of `RefCell<T>` in a
multithreaded program in Chapter 16.

<!-- I'm not really clear at this point what the different to RcT and RefCellT
is, perhaps a succinct round up would help? -->

When creating immutable and mutable references we use the `&` and `&mut`
syntax, respectively. With `RefCell<T>`, we use the `borrow` and `borrow_mut`
methods, which are part of the safe API that belongs to `RefCell<T>`. The
`borrow` method returns the smart pointer type `Ref`, and `borrow_mut` returns
the smart pointer type `RefMut`. Both types implement `Deref` so we can treat
them like regular references. `Ref` and `RefMut` track borrows dynamically, and
their implementation of `Drop` releases the borrow dynamically.

<!-- can you clarify what you mean, practically, by "track borrows
dynamically"?-->

Listing 15-14 shows `RefCell<T>` in use with functions that borrow parameters
both immutably and mutably. Note that the `data` variable is declared as
immutable, with `let data` rather than `let mut data`, yet
`a_fn_that_mutably_borrows` is allowed to borrow the data mutably and make
changes to the data!

Filename: src/main.rs

```
use std::cell::RefCell;

fn a_fn_that_immutably_borrows(a: &i32) {
    println!("a is {}", a);
}

fn a_fn_that_mutably_borrows(b: &mut i32) {
    *b += 1;
}

fn demo(r: &RefCell<i32>) {
    a_fn_that_immutably_borrows(&r.borrow());
    a_fn_that_mutably_borrows(&mut r.borrow_mut());
    a_fn_that_immutably_borrows(&r.borrow());
}

fn main() {
    let data = RefCell::new(5);
    demo(&data);
}
```

Listing 15-14: Using `RefCell<T>`, `borrow`, and `borrow_mut`

This example prints:

```
a is 5
a is 6
```

In `main`, we've created a new `RefCell<T>` containing the value 5 and stored
it in the variable `data`, which we declared *without* the `mut` keyword. We
then call the `demo` function with an immutable reference to `data`: as far as
`main` is concerned, `data` is immutable!

In the `demo` function definition, we get an immutable reference to the value
inside the `RefCell<T>` by calling the `borrow` method, and we call
`a_fn_that_immutably_borrows` with that immutable reference. More
interestingly, we can get a *mutable* reference to the value inside the
`RefCell<T>` with the `borrow_mut` method, and the function
`a_fn_that_mutably_borrows` is then allowed to change the value. The next time
we call the `a_fn_that_immutably_borrows` function that prints out the value,
it's 6 instead of 5. We've just borrowed an immutable reference!

### Borrowing Rules are Checked at Runtime on `RefCell<T>`

<!-- Can you make it clear what we are looking at here, simply just
illustrating that refcellt checks are at runtime? -->

We know from Chapter 4 that, because of the borrowing rules, we cannot create
two mutable borrows in the same scope. Therefore, the following code won't
compile:

```
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;
```

We'll get this compiler error:

```
error[E0499]: cannot borrow `s` as mutable more than once at a time
 -->
  |
5 |     let r1 = &mut s;
  |                   - first mutable borrow occurs here
6 |     let r2 = &mut s;
  |                   ^ second mutable borrow occurs here
7 | }
  | - first borrow ends here
```

In contrast, using `RefCell<T>` and calling `borrow_mut` twice in the same
scope *will* compile, but it'll panic at runtime instead. Run this code with
`cargo run`:

```
use std::cell::RefCell;

fn main() {
    let s = RefCell::new(String::from("hello"));

    let r1 = s.borrow_mut();
    let r2 = s.borrow_mut();
}
```

You should see that it compiles, but panics, with the following error:

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.83 secs
     Running `target/debug/refcell`
thread 'main' panicked at 'already borrowed: BorrowMutError',
/stable-dist-rustc/build/src/libcore/result.rs:868
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

This runtime `BorrowMutError` is similar to our compiler error: it says we've
already borrowed `s` mutably once, so we're not allowed to borrow it again. We
aren't getting around the borrowing rules, we're just choosing to have Rust
enforce them at runtime instead of compile time.

<!--Why would we choose to enforce the rules at runtime instead? -->

You could choose to use `RefCell<T>` everywhere all the time in order to allow
yourself mutability and immutability, but in addition to having to type
`RefCell` a lot, you'd likely find problems later, possibly in production
rather than during development. Doing so would also have a performance penalty.

### Having Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`

So why would we choose to make the tradeoffs that using `RefCell<T>` involves?
On its own, it might not be worth it, but remember that `Rc<T>` only lets you
have an immutable reference to `T`; because `RefCell<T>` is immutable, but has
interior mutability, if we combine `Rc<T>` and `RefCell<T>` we can get a type
that's both reference counted and mutable. This is a huge advantage when....

<!-- maybe just recap on why we'd want that? -->

Listing 15-15 shows an example of how to combine `RefCell<T>` and `Rc<T>`,
again using our cons list from Listing 15-5. Instead of storing `i32` values,
we'll store `Rc<RefCell<i32>>` values so that we can create an owner of the
value that's not part of the list, meaning we won't get the XXX problem we had
with `i32` values. We'll use the multiple owners functionality that `Rc<T>`
provides. This method also allows us to mutate the inner `i32` value using the
interior mutability functionality that `RefCell<T>` provides:

Filename: src/main.rs

```
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Cons(value.clone(), Rc::new(Nil));
    let shared_list = Rc::new(a);

    let b = Cons(Rc::new(RefCell::new(6)), shared_list.clone());
    let c = Cons(Rc::new(RefCell::new(10)), shared_list.clone());

    *value.borrow_mut() += 10;

    println!("shared_list after = {:?}", shared_list);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```

Listing 15-15: Using `Rc<RefCell<i32>>` to create a `List` that we can mutate

We create a value that's an instance of `Rc<RefCell<i32>` and store it in a
variable named `value` so we can access it directly later. Then we create a
`List` in `a` with a `Cons` variant that holds `value`. We need to clone
`value`so that it has ownership in addition to `a`.

<!-- above: so that `value` has ownership of what, in addition to a? I didn't
follow the final sentence above -->

Then we wrap the list `a` in an `Rc<T>` so that, when we create lists `b` and
`c`, they can both refer to `a`, the same as we did in Listing 15-12.

Once we have the lists in `shared_list`, `b`, and `c` created, we add 10 to the
value in `value` by dereferencing the `Rc<T>` and calling `borrow_mut` on the
`RefCell`.

When we print out `shared_list`, `b`, and `c`, we can see that they all have
the modified value of 15:

```
shared_list after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
c after = Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
```

This is pretty neat! By using `RefCell<T>`, we have an outwardly immutable
`List`, but we can use the methods on `RefCell<T>` that provide access to its
interior mutability so we can modify our data when we need to. The runtime
checks of the borrowing rules protect us from data races, and it's sometimes
worth trading a bit of speed for this flexibility in our data structures.

The standard library has other types that provides interior mutability, too,
like `Cell<T>`, which is similar except that instead of giving references to
the inner value, the value is copied in and out of the `Cell<T>`. There is also
`Mutex<T>`, which offers interior mutability that's safe to use across threads,
and we'll be discussing its use in the next chapter on concurrency. Check out
the standard library docs for more details on the differences between these
types.

## Creating Reference Cycles and Preventing Memory Leaks

Rust's memory safety guarantees make it *difficult* to accidentally create
memory that's never cleaned up, known as a *memory leak*, but not impossible.
Entirely preventing memory leaks is not one of Rust's guarantess in the same
way that disallowing data races at compile time is, meaning memory leaks are
memory safe in Rust. We can see this with `Rc<T>` and `RefCell<T>`: it's
possible to create cycles of references where items refer to each other in a
cycle. This creates memory leaks because the reference count of each item in
the cycle will never reach 0, and the values will never be dropped.

### Creating a Reference Cycle Example

Let's take a look at how that might happen and how to prevent it, using Listing
15-16 as our example.

Filename: src/main.rs

```
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}
```

Listing 15-16: A cons list definition that holds a `RefCell` so that we can
modify what a `Cons` variant is referring to

We're using another variation of the `List` definition from Listing 15-5, with
an `i32` value as the first element in the `Cons` variant again. The second
element is now `RefCell<Rc<List>>`, meaning that instead of adding the ability
to modify the `i32` value, we're trying to modify which `List` a `Cons` variant
is pointing to. We've also added a `tail` method to make it convenient for us
to access the second item, if we have a `Cons` variant.

<!-- Can you link this more clearly, what do we have at this point? This change
to a new listing feels unexpected. What are we going to do with this cons list?
Why are we making this next listing, what is it's overall purpose? -->

In listing 15-17 we add functionality that will create a memory leaking cycle.

<!-- so are we adding this to the end of the previous listing? It's in the same
file -->

Filename: src/main.rs

```
use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(a.clone())));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(ref link) = a.tail() {
        *link.borrow_mut() = b.clone();
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle; it will
    // overflow the stack
    // println!("a next item = {:?}", a.tail());
}
```

Listing 15-17: Creating a reference cycle of two `List` values pointing to
each other

We create a `List` value in the variable `a` with an initial list of `5, Nil`.
We then create a `List` value in the variable `b` that contains the value 10,
then points to the list in `a`. Finally, we modify `a` so that it points to `b`
instead of `Nil`, which creates a cycle.

We use the `tail` method to get a reference to the `RefCell` in `a`, which we
put in the variable `link`. Then we use the `borrow_mut` method on the
`RefCell` to change the value inside from an `Rc` that holds a `Nil` value to
the `Rc` in `b`. If you uncomment the last `println!` and run the program, Rust
will try and print this cycle out with `a` pointing to `b` pointing to `a` and
so forth until it overflows the stack.

<!-- Can you show us the output? Also, why are we commenting out the print
statement in the first place?-->

We've created a reference cycle that looks like Figure 15-18:

<img alt="Reference cycle of lists" src="img/trpl15-04.svg" class="center" />

Figure 15-18: A reference cycle of lists `a` and `b` pointing to each other

Looking at the results of the `println!` calls excepting the last one, you
should see that the reference count of both `a` and `b` are 2 after we change
`a` to point to `b`. At the end of `main`, Rust will try and drop `b` first,
which will decrease the count of the `a` and `b` by one.

<!-- Above -- previously `a` and `b` said `Rc`, I wanted to clarify that by Rc
we mean a and b, is that right? -->

<!-- Below--"that Rc" - what are we referring to, a is still referencing b? Can
you clarify that? -->

However, because `a` is still referencing that `Rc`, it has a count of 1 rather
than 0, so the memory the `Rc` has on the heap won't be dropped, creating a
cycle. The memory will just sit there with a count of one, forever.

In this specific case, the program ends right away, so it's not a problem, but
if a more complex program allocates lots of memory in a cycle and holds onto it
for a long time, the program would be using more memory than it needs, and
might overwhelm the system and cause it to run out of available memory.

Creating reference cycles is not easily done, but it's not impossible either.
If you have `RefCell<T>` values that contain `Rc<T>` values or similar nested
combinations of types with interior mutability and reference counting, be aware
that you have to ensure you don't create cycles yourself; you can't rely on
Rust to catch them. In the example in Listing 15-14, the solution would
probably be to not write code that could create cycles like this, since we do
want `Cons` variants to own the list they point to.

<!-- Above-- this seems like a vague solution, just not writing the code that
creates cycles, can you be more specific about which part they should
exclude/change? -->

With data structures like graphs, it's sometimes necessary to have references
that create cycles in order to have parent nodes point to their children and
children nodes point back to their parents, for example. If one of the
directions is expressing ownership and the other isn't, one way to model the
relationship of the data without creating reference cycles and memory leaks is
to use the smart pointer `Weak<T>`. Let's explore that next!

### Preventing Reference Cycles: Turn an `Rc<T>` into a `Weak<T>`

In situations where you have cycles of reference but only one direction
expresses ownership, the Rust standard library provides the smart pointer
`Weak<T>`. `Weak<T>` is a way to reference an `Rc<T>` that does not increment
the `strong_count`, unlike the method we've been using that clones an `Rc<T>`.
With `Weak<T>`, we instead increment the `weak_count` of references to an `Rc`.

<!-- What is a weak_count? I don't think we've defined that, or strong_count,
really. Are we just giving another variable to store the count that has no
input on whether memory is dropped? When is a count stored in strong_count and
when is it stored in weak_count? -->

When an `Rc` goes out of scope, the inner value will be dropped when the
`strong_count` is 0, even if the `weak_count` is not 0.

<!-- Below: I'm struggling to follow here, why do we want to get a value from
Weak<T>? This section is losing me somewhat, can you slow this down, make sure
you define anything new up front and give it's purpose, what we intend it to
do? -->

To be able to get the value from a `Weak<T>`, we first have to upgrade it to an
`Option<Rc<T>>` using the `upgrade` method. From the upgraded `Weak<T>` we'll
get a result of `Some` if the `Rc` value has not been dropped yet, and `None`
if the `Rc` value has been dropped. Because `upgrade` returns an `Option`, we
can be sure that Rust will handle both the `Some` case and the `None` case, and
there won't be an invalid pointer.

As an example, rather than using a list whose items know only about the next
item, we'll create a tree whose items know about their children items *and*
their parent items.

#### HEADING

To build this tree, we'll start with a struct named `Node` that holds its own
`i32` value as well as references to its children `Node` values:

Filename: src/main.rs

```
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}
```

We want a `Node` to own its children, and we want to each node owned by
variables so we can access them directly. To do this, we make the `Vec` items
into `Rc<Node>` values. We also want to be able to modify which nodes are
children to another node, so we have a `RefCell` in `children` around the
`Vec`. Using our struct, create an instance `Node` named `leaf` which will have
the value 3 and no children, and another instance named `branch` with the value
5 and `leaf` as one of its children, as shown in Listing 15-19:

Filename: src/main.rs

```
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![leaf.clone()]),
    });
}
```

Listing 15-19: Creating a `leaf` node with no children and a `branch` node with
`leaf` as one of its children

We clone the `Rc` in `leaf` and store that in `branch`, meaning the `Node` in
`leaf` now has two owners: `leaf` and `branch`. In `branch` we gave a reference
to `leaf` in `branch.children`, but `leaf` has no reference to `branch` and so
doesn't know they are related. We'd like `leaf` to know that `branch` is its
parent.

#### HEADING

To make the child node aware of its parent, we need to add a `parent` field to
our `Node` struct definition. The trouble is in deciding what the type of
`parent` should be. We know it can't contain an `Rc<T>` because that would
create a reference cycle, with `leaf.parent` pointing to `branch` and
`branch.children` pointing, meaning their reference counts would never be zero.

So instead of `Rc`, we'll make the type of `parent` use `Weak<T>`, specifically
a `RefCell<Weak<Node>>`.

<!-- I think because I still don't understand what Weak<T> is, I'm not really
sure what it means for the parent to use Weak<T>, can you make sure that's
clear at this point -->

Filename: src/main.rs

```
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
```

We replace `Rc` with `{Rc, Weak}`, meaning it

<!-- Can you fill out this line, above; talk through the syntax, too? Also,
below, how does this mean a node can refer to a parent without owning it?
What's is actually doing here?-->

This way, a node will be able to refer to its parent node, but does not own its
parent. A parent node will be dropped even if it has child nodes referring to
it, as long as it doesn't have a parent node as well. Now let's update `main`
to look like Listing 15-20:

<!-- Why are we updating it, what are we doing here? Can you make that clear?
-->

Filename: src/main.rs

```
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![leaf.clone()]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```

Listing 15-20: A `leaf` node with a `Weak` reference to its parent node,
`branch`

<!-- Below: looks similar to what? What are we doing with this listing, can you
talk it through -->

Creating the `leaf` node== looks similar to XXXXX==; `leaf` starts out without
a parent, so we create a new `Weak` reference instance. When we try to get a
reference to the parent of `leaf` by using the `upgrade` method, we'll get a
`None` value, as shown by the first `println!` that outputs:

```
leaf parent = None
```

<!-- Is this the explanation of the previous program? If so, can you change the
tone to an active tone, make it clear that it's connected? I'm struggling to
connect things up -->

Our `branch` node will also have a new `Weak` reference, since `branch` does
not have a parent node, but it does still have `leaf` as one of its children.
Once we have a new `Node` instance in `branch`, we can modify `leaf` to give it
a `Weak` reference to its parent. We use the `borrow_mut` method on the
`RefCell` in the `parent` field of `leaf`, then we use the `Rc::downgrade`
function to create a `Weak` reference to `branch` from the `Rc` in `branch.`

<!-- Below: What does this mean for our program, that now leaf recognizes its
parent? -->

When we print out the parent of `leaf` again, this time we'll get a `Some`
variant holding `branch`. We also avoid a cycle that would eventually end in a
stack overflow like we did in Listing 15-14: the `Weak` references are just
printed as `(Weak)`:

```
leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) },
children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
children: RefCell { value: [] } }] } })
```

The lack of an infinite output indicates that this code did note create a
reference cycle. We can also tell this by looking at the values we get from
calling `Rc::strong_count` and `Rc::weak_count`.

#### Heading somewhere around here, I'm not sure this is the best place?

Let's check for reference cycles in the `Rc` values by creating a new inner
scope and moving the creation of `branch` in there---this will let us see what
happens when `branch` is created and then dropped when it goes out of scope:

Filename: src/main.rs

```
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![leaf.clone()]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
```

Listing 15-21: Creating `branch` in an inner scope and examining strong and
weak reference counts

Once `leaf` is created it has a strong count of 1 and a weak count of 0. In the
inner scope we create `branch` and associate it with `leaf`, at which point
`branch` will have a strong count of 1 and a weak count of 1 (for `leaf.parent`
pointing to `branch` with a `Weak<T>`). Here `leaf` will have a strong count of
2, because `branch` now has a clone of the `Rc` of `leaf` stored in
`branch.children`, but will still have a weak count of 0.

When the inner scope ends, `branch` goes out of scope and its strong count
decreases to 0, so its `Node` gets dropped. The weak count of 1 from
`leaf.parent` has no bearing on whether `Node` is dropped or not, so we don't
get any memory leaks!

If we try to access the parent of `leaf` after the end of the scope, we'll get
`None` again. At the end of the program, `leaf` has a strong count of 1 and a
weak count of 0, because`leaf` is now the only thing pointing to it again.

<!-- Just to clarify, leaf is pointing to itself? -->

All of the logic that manages the counts and value dropping is being managed by
`Rc` and `Weak` and their implementations of the `Drop` trait. By specifying
that the relationship from a child to its parent should be a `Weak<T>`
reference in the definition of `Node`, we're able to have parent nodes point to
child nodes and vice versa without creating a reference cycle and memory leaks.

<!-- Ah! This actually cleared up a lot, we specify in the definition that a
reference should be weak and therefore ignored by the Drop trait, is that
right? It would really help to specify that up front, can you add something
like that to the start of the Weak section? -->

## Summary

This chapter covered how you can use smart pointers to make different
guarantees and tradeoffs than those Rust makes by default with regular
references. `Box<T>` has a known size and points to data allocated on the heap.
`Rc<T>` keeps track of the number of references to data on the heap so that
data can have multiple owners. `RefCell<T>` with its interior mutability gives
us a type that can be used when we need an immutable type but may need to
change the value of that type, and enforces the borrowing rules at runtime
instead of at compile time.

We also discussed the `Deref` and `Drop` traits that enable a lot of the
functionality of smart pointers. We explored reference cycles that cause memory
leaks can occue, and how to prevent them using `Weak<T>`.

If this chapter has piqued your interest and you want to implement your own
smart pointers, check out The Nomicon at
*https://doc.rust-lang.org/stable/nomicon/vec.html* for even more useful
information.

Next, let's talk about concurrency in Rust. We'll even learn about a few new
smart pointers.
