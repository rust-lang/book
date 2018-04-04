## `RefCell<T>` and the Interior Mutability Pattern

<!-- NEXT PARAGRAPH WRAPPED WEIRD INTENTIONALLY SEE #199 -->

*Interior mutability* is a design pattern in Rust that allows you to mutate
data even when there are immutable references to that data; normally, this
action is disallowed by the borrowing rules. To mutate data, the pattern uses
`unsafe` code inside a data structure to bend Rust’s usual rules that govern
mutation and borrowing. We haven’t yet covered unsafe code; we will in
Chapter 19. We can use types that use the interior mutability pattern when we
can ensure that the borrowing rules will be followed at runtime, even though
the compiler can’t guarantee that. The `unsafe` code involved is then wrapped
in a safe API, and the outer type is still immutable.

Let’s explore this concept by looking at the `RefCell<T>` type that follows the
interior mutability pattern.

### Enforcing Borrowing Rules at Runtime with `RefCell<T>`

Unlike `Rc<T>`, the `RefCell<T>` type represents single ownership over the data
it holds. So, what makes `RefCell<T>` different from a type like `Box<T>`?
Recall the borrowing rules you learned in Chapter 4:

* At any given time, you can have *either* (but not both of) one mutable
  reference or any number of immutable references.
* References must always be valid.

With references and `Box<T>`, the borrowing rules’ invariants are enforced at
compile time. With `RefCell<T>`, these invariants are enforced *at runtime*.
With references, if you break these rules, you’ll get a compiler error. With
`RefCell<T>`, if you break these rules, your program will panic and exit.

The advantages of checking the borrowing rules at compile time are that errors
will be caught sooner in the development process, and there is no impact on
runtime performance because all the analysis is completed beforehand. For those
reasons, checking the borrowing rules at compile time is the best choice in the
majority of cases, which is why this is Rust’s default.

The advantage of checking the borrowing rules at runtime instead is that
certain memory-safe scenarios are then allowed, whereas they are disallowed by
the compile-time checks. Static analysis, like the Rust compiler, is inherently
conservative. Some properties of code are impossible to detect by analyzing the
code: the most famous example is the Halting Problem, which is beyond the scope
of this book but is an interesting topic to research.

Because some analysis is impossible, if the Rust compiler can’t be sure the
code complies with the ownership rules, it might reject a correct program; in
this way, it’s conservative. If Rust accepted an incorrect program, users
wouldn’t be able to trust in the guarantees Rust makes. However, if Rust
rejects a correct program, the programmer will be inconvenienced, but nothing
catastrophic can occur. The `RefCell<T>` type is useful when you’re sure your
code follows the borrowing rules but the compiler is unable to understand and
guarantee that.

Similar to `Rc<T>`, `RefCell<T>` is only for use in single-threaded scenarios
and will give you a compile-time error if you try using it in a multithreaded
context. We’ll talk about how to get the functionality of `RefCell<T>` in a
multithreaded program in Chapter 16.

Here is a recap of the reasons to choose `Box<T>`, `Rc<T>`, or `RefCell<T>`:

* `Rc<T>` enables multiple owners of the same data; `Box<T>` and `RefCell<T>`
  have single owners.
* `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>`
  allows only immutable borrows checked at compile time; `RefCell<T>` allows
  immutable or mutable borrows checked at runtime.
* Because `RefCell<T>` allows mutable borrows checked at runtime, you can
  mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is
  immutable.

Mutating the value inside an immutable value is the *interior mutability*
pattern. Let’s look at a situation in which interior mutability is useful and
examine how it’s possible.

### Interior Mutability: A Mutable Borrow to an Immutable Value

A consequence of the borrowing rules is that when you have an immutable value,
you can’t borrow it mutably. For example, this code won’t compile:

```rust,ignore
fn main() {
    let x = 5;
    let y = &mut x;
}
```

If you tried to compile this code, you’d get the following error:

```text
error[E0596]: cannot borrow immutable local variable `x` as mutable
 --> src/main.rs:3:18
  |
2 |     let x = 5;
  |         - consider changing this to `mut x`
3 |     let y = &mut x;
  |                  ^ cannot borrow mutably
```

However, there are situations in which it would be useful for a value to mutate
itself in its methods but appear immutable to other code. Code outside the
value’s methods would not be able to mutate the value. Using `RefCell<T>` is
one way to get the ability to have interior mutability. But `RefCell<T>`
doesn’t get around the borrowing rules completely: the borrow checker in the
compiler allows this interior mutability, and the borrowing rules are checked
at runtime instead. If you violate the rules, you’ll get a `panic!` instead of
a compiler error.

Let’s work through a practical example where we can use `RefCell<T>` to mutate
an immutable value and see why that is useful.

#### A Use Case for Interior Mutability: Mock Objects

A *test double* is the general programming concept for a type used in place of
another type during testing. *Mock objects* are specific types of test doubles
that record what happens during a test so you can assert that the correct
actions took place.

Rust doesn’t have objects in the same sense as other languages have objects,
and Rust doesn’t have mock object functionality built into the standard library
as some other languages do. However, you can definitely create a struct that
will serve the same purposes as a mock object.

Here’s the scenario we’ll test: we’ll create a library that tracks a value
against a maximum value and sends messages based on how close to the maximum
value the current value is. This library could be used to keep track of a
user’s quota for the number of API calls they’re allowed to make, for example.

Our library will only provide the functionality of tracking how close to the
maximum a value is and what the messages should be at what times. Applications
that use our library will be expected to provide the mechanism for sending the
messages: the application could put a message in the application, send an
email, send a text message, or something else. The library doesn’t need to know
that detail. All it needs is something that implements a trait we’ll provide
called `Messenger`. Listing 15-20 shows the library code:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}
```

<span class="caption">Listing 15-20: A library to keep track of how close a
value is to a maximum value and warn when the value is at certain levels</span>

One important part of this code is that the `Messenger` trait has one method
called `send` that takes an immutable reference to `self` and the text of the
message. This is the interface our mock object needs to have. The other
important part is that we want to test the behavior of the `set_value` method
on the `LimitTracker`. We can change what we pass in for the `value` parameter,
but `set_value` doesn’t return anything for us to make assertions on. We want
to be able to say that if we create a `LimitTracker` with something that
implements the `Messenger` trait and a particular value for `max`, when we pass
different numbers for `value`, the messenger is told to send the appropriate
messages.

We need a mock object that, instead of sending an email or text message when we
call `send`, will only keep track of the messages it’s told to send. We can
create a new instance of the mock object, create a `LimitTracker` that uses the
mock object, call the `set_value` method on `LimitTracker`, and then check that
the mock object has the messages we expect. Listing 15-21 shows an attempt to
implement a mock object to do just that, but the borrow checker won’t allow it:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: vec![] }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
```

<span class="caption">Listing 15-21: An attempt to implement a `MockMessenger`
that isn’t allowed by the borrow checker</span>

This test code defines a `MockMessenger` struct that has a `sent_messages`
field with a `Vec` of `String` values to keep track of the messages it’s told
to send. We also define an associated function `new` to make it convenient to
create new `MockMessenger` values that start with an empty list of messages. We
then implement the `Messenger` trait for `MockMessenger` so we can give a
`MockMessenger` to a `LimitTracker`. In the definition of the `send` method, we
take the message passed in as a parameter and store it in the `MockMessenger`
list of `sent_messages`.

In the test, we’re testing what happens when the `LimitTracker` is told to set
`value` to something that is more than 75 percent of the `max` value. First, we
create a new `MockMessenger`, which will start with an empty list of messages.
Then we create a new `LimitTracker` and give it a reference to the new
`MockMessenger` and a `max` value of 100. We call the `set_value` method on the
`LimitTracker` with a value of 80, which is more than 75 percent of 100. Then
we assert that the list of messages that the `MockMessenger` is keeping track
of should now have one message in it.

However, there’s one problem with this test, as shown here:

```text
error[E0596]: cannot borrow immutable field `self.sent_messages` as mutable
  --> src/lib.rs:52:13
   |
51 |         fn send(&self, message: &str) {
   |                 ----- use `&mut self` here to make mutable
52 |             self.sent_messages.push(String::from(message));
   |             ^^^^^^^^^^^^^^^^^^ cannot mutably borrow immutable field
```

We can’t modify the `MockMessenger` to keep track of the messages, because the
`send` method takes an immutable reference to `self`. We also can’t take the
suggestion from the error text to use `&mut self` instead, because then the
signature of `send` wouldn’t match the signature in the `Messenger` trait
definition (feel free to try and see what error message you get).

This is a situation in which interior mutability can help! We’ll store the
`sent_messages` within a `RefCell<T>`, and then the `send` message will be
able to modify `sent_messages` to store the messages we’ve seen. Listing 15-22
shows what that looks like:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--
#         let mock_messenger = MockMessenger::new();
#         let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
#         limit_tracker.set_value(75);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```

<span class="caption">Listing 15-22: Using `RefCell<T>` to mutate an inner
value while the outer value is considered immutable</span>

The `sent_messages` field is now of type `RefCell<Vec<String>>` instead of
`Vec<String>`. In the `new` function, we create a new `RefCell<Vec<String>>`
instance around the empty vector.

For the implementation of the `send` method, the first parameter is still an
immutable borrow of `self`, which matches the trait definition. We call
`borrow_mut` on the `RefCell<Vec<String>>` in `self.sent_messages` to get a
mutable reference to the value inside the `RefCell<Vec<String>>`, which is
the vector. Then we can call `push` on the mutable reference to the vector to
keep track of the messages sent during the test.

The last change we have to make is in the assertion: to see how many items are
in the inner vector, we call `borrow` on the `RefCell<Vec<String>>` to get an
immutable reference to the vector.

Now that you’ve seen how to use `RefCell<T>`, let’s dig into how it works!

#### Keeping Track of Borrows at Runtime with `RefCell<T>`

When creating immutable and mutable references, we use the `&` and `&mut`
syntax, respectively. With `RefCell<T>`, we use the `borrow` and `borrow_mut`
methods, which are part of the safe API that belongs to `RefCell<T>`. The
`borrow` method returns the smart pointer type `Ref<T>`, and `borrow_mut`
returns the smart pointer type `RefMut<T>`. Both types implement `Deref`, so we
can treat them like regular references.

The `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart
pointers are currently active. Every time we call `borrow`, the `RefCell<T>`
increases its count of how many immutable borrows are active. When a `Ref<T>`
value goes out of scope, the count of immutable borrows goes down by one. Just
like the compile-time borrowing rules, `RefCell<T>` lets us have many immutable
borrows or one mutable borrow at any point in time.

If we try to violate these rules, rather than getting a compiler error as we
would with references, the implementation of `RefCell<T>` will panic at
runtime. Listing 15-23 shows a modification of the implementation of `send` in
Listing 15-22. We’re deliberately trying to create two mutable borrows active
for the same scope to illustrate that `RefCell<T>` prevents us from doing this
at runtime.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        let mut one_borrow = self.sent_messages.borrow_mut();
        let mut two_borrow = self.sent_messages.borrow_mut();

        one_borrow.push(String::from(message));
        two_borrow.push(String::from(message));
    }
}
```

<span class="caption">Listing 15-23: Creating two mutable references in the
same scope to see that `RefCell<T>` will panic</span>

We create a variable `one_borrow` for the `RefMut<T>` smart pointer returned
from `borrow_mut`. Then we create another mutable borrow in the same way in the
variable `two_borrow`. This makes two mutable references in the same scope,
which isn’t allowed. When we run the tests for our library, the code in Listing
15-23 will compile without any errors, but the test will fail:

```text
---- tests::it_sends_an_over_75_percent_warning_message stdout ----
	thread 'tests::it_sends_an_over_75_percent_warning_message' panicked at
'already borrowed: BorrowMutError', src/libcore/result.rs:906:4
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

Notice that the code panicked with the message `already borrowed:
BorrowMutError`. This is how `RefCell<T>` handles violations of the borrowing
rules at runtime.

Catching borrowing errors at runtime rather than compile time means that you
would find a mistake in your code later in the development process and possibly
not until your code was deployed to production. Also, your code would incur a
small runtime performance penalty as a result of keeping track of the borrows
at runtime rather than compile time. However, using `RefCell<T>` makes it
possible to write a mock object that can modify itself to keep track of the
messages it has seen while you’re using it in a context where only immutable
values are allowed. You can use `RefCell<T>` despite its trade-offs to get more
functionality than regular references provide.

### Having Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`

A common way to use `RefCell<T>` is in combination with `Rc<T>`. Recall that
`Rc<T>` lets you have multiple owners of some data, but it only gives immutable
access to that data. If you have an `Rc<T>` that holds a `RefCell<T>`, you can
get a value that can have multiple owners *and* that you can mutate!

For example, recall the cons list example in Listing 15-18 where we used
`Rc<T>` to allow multiple lists to share ownership of another list. Because
`Rc<T>` holds only immutable values, we can’t change any of the values in the
list once we’ve created them. Let’s add in `RefCell<T>` to gain the ability to
change the values in the lists. Listing 15-24 shows that by using a
`RefCell<T>` in the `Cons` definition, we can modify the value stored in all
the lists:

<span class="filename">Filename: src/main.rs</span>

```rust
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

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```

<span class="caption">Listing 15-24: Using `Rc<RefCell<i32>>` to create a
`List` that we can mutate</span>

We create a value that is an instance of `Rc<RefCell<i32>>` and store it in a
variable named `value` so we can access it directly later. Then we create a
`List` in `a` with a `Cons` variant that holds `value`. We need to clone
`value` so both `a` and `value` have ownership of the inner `5` value rather
than transferring ownership from `value` to `a` or having `a` borrow from
`value`.

We wrap the list `a` in an `Rc<T>` so when we create lists `b` and `c`, they
can both refer to `a`, which is what we did in Listing 15-18.

After we’ve created the lists in `a`, `b`, and `c`, we add 10 to the value in
`value`. We do this by calling `borrow_mut` on `value`, which uses the
automatic dereferencing feature we discussed in Chapter 5 (see the section
“Where’s the `->` Operator?”) to dereference the `Rc<T>` to the inner
`RefCell<T>` value. The `borrow_mut` method returns a `RefMut<T>` smart
pointer, and we use the dereference operator on it and change the inner value.

When we print `a`, `b`, and `c`, we can see that they all have the modified
value of 15 rather than 5:

```text
a after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
c after = Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
```

This technique is pretty neat! By using `RefCell<T>`, we have an outwardly
immutable `List` value. But we can use the methods on `RefCell<T>` that provide
access to its interior mutability so we can modify our data when we need to.
The runtime checks of the borrowing rules protect us from data races, and it’s
sometimes worth trading a bit of speed for this flexibility in our data
structures.

The standard library has other types that provide interior mutability, such as
`Cell<T>`, which is similar except that instead of giving references to the
inner value, the value is copied in and out of the `Cell<T>`. There’s also
`Mutex<T>`, which offers interior mutability that’s safe to use across threads;
we’ll discuss its use in Chapter 16. Check out the standard library docs for
more details on the differences between these types.
