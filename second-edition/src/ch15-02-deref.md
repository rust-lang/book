## The `Deref` Trait Allows Access to the Data Through a Reference

The first important smart pointer-related trait is `Deref`, which allows us to
override `*`, the dereference operator (as opposed to the multiplication
operator or the glob operator). Overriding `*` for smart pointers makes
accessing the data behind the smart pointer convenient, and we’ll talk about
what we mean by convenient when we get to deref coercions later in this section.

We briefly mentioned the dereference operator in Chapter 8, in the hash map
section titled “Update a Value Based on the Old Value”. We had a mutable
reference, and we wanted to change the value that the reference was pointing
to. In order to do that, first we had to dereference the reference. Here’s
another example using references to `i32` values:

```rust
let mut x = 5;
{
    let y = &mut x;

    *y += 1
}

assert_eq!(6, x);
```

We use `*y` to access the data that the mutable reference in `y` refers to,
rather than the mutable reference itself. We can then modify that data, in this
case by adding 1.

With references that aren’t smart pointers, there’s only one value that the
reference is pointing to, so the dereference operation is straightforward.
Smart pointers can also store metadata about the pointer or the data. When
dereferencing a smart pointer, we only want the data, not the metadata, since
dereferencing a regular reference only gives us data and not metadata. We want
to be able to use smart pointers in the same places that we can use regular
references. To enable that, we can override the behavior of the `*` operator by
implementing the `Deref` trait.

Listing 15-7 has an example of overriding `*` using `Deref` on a struct we’ve
defined to hold mp3 data and metadata. `Mp3` is, in a sense, a smart pointer:
it owns the `Vec<u8>` data containing the audio. In addition, it holds some
optional metadata, in this case the artist and title of the song in the audio
data. We want to be able to conveniently access the audio data, not the
metadata, so we implement the `Deref` trait to return the audio data.
Implementing the `Deref` trait requires implementing one method named `deref`
that borrows `self` and returns the inner data:

<span class="filename">Filename: src/main.rs</span>

```rust
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

<span class="caption">Listing 15-7: An implementation of the `Deref` trait on a
struct that holds mp3 file data and metadata</span>

Most of this should look familiar: a struct, a trait implementation, and a
main function that creates an instance of the struct. There is one part we
haven’t explained thoroughly yet: similarly to Chapter 13 when we looked at the
Iterator trait with the `type Item`, the `type Target = T;` syntax is defining
an associated type, which is covered in more detail in Chapter 19. Don’t worry
about that part of the example too much; it is a slightly different way of
declaring a generic parameter.

In the `assert_eq!`, we’re verifying that `vec![1, 2, 3]` is the result we get
when dereferencing the `Mp3` instance with `*my_favorite_song`, which is what
happens since we implemented the `deref` method to return the audio data. If
we hadn’t implemented the `Deref` trait for `Mp3`, Rust wouldn’t compile the
code `*my_favorite_song`: we’d get an error saying type `Mp3` cannot be
dereferenced.

Without the `Deref` trait, the compiler can only dereference `&` references,
which `my_favorite_song` is not (it is an `Mp3` struct). With the `Deref`
trait, the compiler knows that types implementing the `Deref` trait have a
`deref` method that returns a reference (in this case, `&self.audio` because of
our definition of `deref` in Listing 15-7). So in order to get a `&` reference
that `*` can dereference, the compiler expands `*my_favorite_song` to this:

```rust,ignore
*(my_favorite_song.deref())
```

The result is the value in `self.audio`. The reason `deref` returns a reference
that we then have to dereference, rather than just returning a value directly,
is because of ownership: if the `deref` method directly returned the value
instead of a reference to it, the value would be moved out of `self`. We don’t
want to take ownership of `my_favorite_song.audio` in this case and most cases
where we use the dereference operator.

Note that replacing `*` with a call to the `deref` method and then a call to
`*` happens once, each time the `*` is used. The substitution of `*` does not
recurse infinitely. That’s how we end up with data of type `Vec<u8>`, which
matches the `vec![1, 2, 3]` in the `assert_eq!` in Listing 15-7.

### Implicit Deref Coercions with Functions and Methods

Rust tends to favor explicitness over implicitness, but one case where this
does not hold true is *deref coercions* of arguments to functions and methods.
A deref coercion will automatically convert a reference to any pointer into a
reference to that pointer’s contents. A deref coercion happens when the
reference type of the argument passed into the function differs from the
reference type of the parameter defined in that function’s signature. Deref
coercion was added to Rust to make calling functions and methods not need as
many explicit references and dereferences with `&` and `*`.

Using our `Mp3` struct from Listing 15-7, here’s the signature of a function to
compress mp3 audio data that takes a slice of `u8`:

```rust,ignore
fn compress_mp3(audio: &[u8]) -> Vec<u8> {
    // the actual implementation would go here
}
```

If Rust didn’t have deref coercion, in order to call this function with the
audio data in `my_favorite_song`, we’d have to write:

```rust,ignore
compress_mp3(my_favorite_song.audio.as_slice())
```

That is, we’d have to explicitly say that we want the data in the `audio` field
of `my_favorite_song` and that we want a slice referring to the whole
`Vec<u8>`. If there were a lot of places where we’d want to process the `audio`
data in a similar manner, `.audio.as_slice()` would be wordy and repetitive.

However, because of deref coercion and our implementation of the `Deref` trait
on `Mp3`, we can call this function with the data in `my_favorite_song` by
using this code:

```rust,ignore
let result = compress_mp3(&my_favorite_song);
```

Just an `&` and the instance, nice! We can treat our smart pointer as if it was
a regular reference. Deref coercion means that Rust can use its knowledge of
our `Deref` implementation, namely: Rust knows that `Mp3` implements the
`Deref` trait and returns `&Vec<u8>` from the `deref` method. Rust also knows
the standard library implements the `Deref` trait on `Vec<T>` to return `&[T]`
from the `deref` method (and we can find that out too by looking at the API
documentation for `Vec<T>`). So, at compile time, Rust will see that it can use
`Deref::deref` twice to turn `&Mp3` into `&Vec<u8>` and then into `&[T]` to
match the signature of `compress_mp3`. That means we get to do less typing!
Rust will analyze types through `Deref::deref` as many times as it needs to in
order to get a reference to match the parameter’s type, when the `Deref` trait
is defined for the types involved. This indirection is resolved at compile time,
so there is no run-time penalty for taking advantage of deref coercion!

Similar to how we use the `Deref` trait to override `*` on `&T`s, there is also
a `DerefMut` trait for overriding `*` on `&mut T`.

Rust does deref coercion when it finds types and trait implementations in three
cases:

* From `&T` to `&U` when `T: Deref<Target=U>`.
* From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`.
* From `&mut T` to `&U` when `T: Deref<Target=U>`.

The first two are the same, except for mutability: if you have a `&T`, and
`T` implements `Deref` to some type `U`, you can get a `&U` transparently. Same
for mutable references. The last one is more tricky: if you have a mutable
reference, it will also coerce to an immutable one. The other case is _not_
possible though: immutable references will never coerce to mutable ones.

The reason that the `Deref` trait is important to the smart pointer pattern is
that smart pointers can then be treated like regular references and used in
places that expect regular references. We don’t have to redefine methods and
functions to take smart pointers explicitly, for example.
