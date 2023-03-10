## Method Syntax

*Methods* are similar to functions: we declare them with the `fn` keyword and a
name, they can have parameters and a return value, and they contain some code
that’s run when the method is called from somewhere else. Unlike functions,
methods are defined within the context of a struct (or an enum or a trait
object, which we cover in Chapters 6 and 17, respectively), and their first
parameter is always `self`, which represents the instance of the struct the
method is being called on.

### Defining Methods

Let’s change the `area` function that has a `Rectangle` instance as a parameter
and instead make an `area` method defined on the `Rectangle` struct, as shown
in Listing 5-13.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

<span class="caption">Listing 5-13: Defining an `area` method on the
`Rectangle` struct</span>

To define the function within the context of `Rectangle`, we start an `impl`
(implementation) block for `Rectangle`. Everything within this `impl` block
will be associated with the `Rectangle` type. Then we move the `area` function
within the `impl` curly brackets and change the first (and in this case, only)
parameter to be `self` in the signature and everywhere within the body. In
`main`, where we called the `area` function and passed `rect1` as an argument,
we can instead use *method syntax* to call the `area` method on our `Rectangle`
instance. The method syntax goes after an instance: we add a dot followed by
the method name, parentheses, and any arguments.

In the signature for `area`, we use `&self` instead of `rectangle: &Rectangle`.
The `&self` is actually short for `self: &Self`. Within an `impl` block, the
type `Self` is an alias for the type that the `impl` block is for. Methods must
have a parameter named `self` of type `Self` for their first parameter, so Rust
lets you abbreviate this with only the name `self` in the first parameter spot.
Note that we still need to use the `&` in front of the `self` shorthand to
indicate this method borrows the `Self` instance, just as we did in `rectangle:
&Rectangle`. Methods can take ownership of `self`, borrow `self` immutably as
we’ve done here, or borrow `self` mutably, just as they can any other parameter.

We’ve chosen `&self` here for the same reason we used `&Rectangle` in the
function version: we don’t want to take ownership, and we just want to read the
data in the struct, not write to it. If we wanted to change the instance that
we’ve called the method on as part of what the method does, we’d use `&mut
self` as the first parameter. Having a method that takes ownership of the
instance by using just `self` as the first parameter is rare; this technique is
usually used when the method transforms `self` into something else and you want
to prevent the caller from using the original instance after the transformation.

The main reason for using methods instead of functions, in addition to providing
method syntax and not having to repeat the type of `self` in every method’s
signature, is for organization. We’ve put all the things we can do with an
instance of a type in one `impl` block rather than making future users of our
code search for capabilities of `Rectangle` in various places in the library we
provide.

Note that we can choose to give a method the same name as one of the struct’s
fields. For example, we can define a method on `Rectangle` also named `width`:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-06-method-field-interaction/src/main.rs:here}}
```

Here, we’re choosing to make the `width` method return `true` if the value in
the instance’s `width` field is greater than 0, and `false` if the value is 0:
we can use a field within a method of the same name for any purpose. In `main`,
when we follow `rect1.width` with parentheses, Rust knows we mean the method
`width`. When we don’t use parentheses, Rust knows we mean the field `width`.

Often, but not always, when we give methods with the same name as a field we
want it to only return the value in the field and do nothing else. Methods like
this are called *getters*, and Rust does not implement them automatically for
struct fields as some other languages do. Getters are useful because you can
make the field private but the method public and thus enable read-only access
to that field as part of the type’s public API. We will be discussing what
public and private are and how to designate a field or method as public or
private in Chapter 7.

### Methods with More Parameters

Let’s practice using methods by implementing a second method on the `Rectangle`
struct. This time, we want an instance of `Rectangle` to take another instance
of `Rectangle` and return `true` if the second `Rectangle` can fit completely
within `self` (the first `Rectangle`); otherwise it should return `false`. That
is, once we’ve defined the `can_hold` method, we want to be able to write the
program shown in Listing 5-14.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```

<span class="caption">Listing 5-14: Using the as-yet-unwritten `can_hold`
method</span>

And the expected output would look like the following, because both dimensions
of `rect2` are smaller than the dimensions of `rect1` but `rect3` is wider than
`rect1`:

```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

We know we want to define a method, so it will be within the `impl Rectangle`
block. The method name will be `can_hold`, and it will take an immutable borrow
of another `Rectangle` as a parameter. We can tell what the type of the
parameter will be by looking at the code that calls the method:
`rect1.can_hold(&rect2)` passes in `&rect2`, which is an immutable borrow to
`rect2`, an instance of `Rectangle`. This makes sense because we only need to
read `rect2` (rather than write, which would mean we’d need a mutable borrow),
and we want `main` to retain ownership of `rect2` so we can use it again after
calling the `can_hold` method. The return value of `can_hold` will be a
Boolean, and the implementation will check whether the width and height of
`self` are both greater than the width and height of the other `Rectangle`,
respectively. Let’s add the new `can_hold` method to the `impl` block from
Listing 5-13, shown in Listing 5-15.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

<span class="caption">Listing 5-15: Implementing the `can_hold` method on
`Rectangle` that takes another `Rectangle` instance as a parameter</span>

When we run this code with the `main` function in Listing 5-14, we’ll get our
desired output. Methods can take multiple parameters that we add to the
signature after the `self` parameter, and those parameters work just like
parameters in functions.


### Associated Functions

All functions defined within an `impl` block are called *associated functions*
because they’re associated with the type named after the `impl`. We can define
associated functions that don’t have `self` as their first parameter (and thus
are not methods) because they don’t need an instance of the type to work with.
We’ve already used one function like this: the `String::from` function that’s
defined on the `String` type.

Associated functions that aren’t methods are often used for constructors that
will return a new instance of the struct. These are often called `new`, but
`new` isn’t a special name and isn’t built into the language. For example, we
could choose to provide an associated function named `square` that would have
one dimension parameter and use that as both width and height, thus making it
easier to create a square `Rectangle` rather than having to specify the same
value twice:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

The `Self` keywords in the return type and in the body of the function are
aliases for the type that appears after the `impl` keyword, which in this case
is `Rectangle`.

To call this associated function, we use the `::` syntax with the struct name;
`let sq = Rectangle::square(3);` is an example. This function is namespaced by
the struct: the `::` syntax is used for both associated functions and
namespaces created by modules. We’ll discuss modules in Chapter 7.

### Multiple `impl` Blocks

Each struct is allowed to have multiple `impl` blocks. For example, Listing
5-15 is equivalent to the code shown in Listing 5-16, which has each method
in its own `impl` block.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

<span class="caption">Listing 5-16: Rewriting Listing 5-15 using multiple `impl`
blocks</span>

There’s no reason to separate these methods into multiple `impl` blocks here,
but this is valid syntax. We’ll see a case in which multiple `impl` blocks are
useful in Chapter 10, where we discuss generic types and traits.

### Method Calls are Syntactic Sugar for Function Calls

Using the concepts we've discussed so far, we can now see how method calls are syntactic sugar for function calls. For example, let's say we have a rectangle struct with an `area` method and a `set_width` method:

```rust,ignore
# struct Rectangle {
#     width: u32,
#     height: u32,
# }
# 
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}
```

And let's say we have a rectangle `r`. Then the method calls `r.area()` and `r.set_width(2)` are equivalent to this:

```rust
# struct Rectangle {
#     width: u32,
#     height: u32,
# }
# 
# impl Rectangle {
#     fn area(&self) -> u32 {
#        self.width * self.height
#      }
# 
#     fn set_width(&mut self, width: u32) {
#         self.width = width;
#     }
# }
# 
# fn main() {
    let mut r = Rectangle { 
        width: 1,
        height: 2
    };
    let area1 = r.area();
    let area2 = Rectangle::area(&r);
    assert_eq!(area1, area2);

    r.set_width(2);
    Rectangle::set_width(&mut r, 2);
# }
```

The method call `r.area()` becomes `Rectangle::area(&r)`. The function name is the associated function `Rectangle::area`. The function argument is the `&self` parameter. Rust automatically inserts the borrowing operator `&`.

> *Note:* if you are familiar with C or C++, you are used to two different syntaxes for method calls: `r.area()` and `r->area()`. Rust does not have an equivalent to the arrow operator `->`. Rust will automatically reference and dereference the method receiver when you use the dot operator.

The method call `r.set_width(2)` similarly becomes `Rectangle::set_width(&mut r, 2)`. This method expects `&mut self`, so the first argument is a mutable borrow `&mut r`. The second argument is exactly the same, the number 2.

As we described in Chapter 4.3 ["Dereferencing a Pointer Accesses Its Data"](ch04-02-references-and-borrowing.html#dereferencing-a-pointer-accesses-its-data), Rust will insert as many references and dereferences as needed to make the types match up for the `self` parameter. For example, here are two equivalent calls to `area` for a mutable reference to a boxed rectangle:

```rust
# struct Rectangle {
#     width: u32,
#     height: u32,
# }
# 
# impl Rectangle {
#     fn area(&self) -> u32 {
#        self.width * self.height
#      }
# 
#     fn set_width(&mut self, width: u32) {
#         self.width = width;
#     }
# }
# fn main() {
    let r = &mut Box::new(Rectangle { 
        width: 1,
        height: 2
    });
    let area1 = r.area();
    let area2 = Rectangle::area(&**r);
    assert_eq!(area1, area2);
# }
```

Rust will add two dereferences (once for the mutable reference, once for the box) and then one immutable borrow because `area` expects `&Rectangle`. Note that this is also a situation where a mutable reference is "downgraded" into a shared reference, like we discussed in [Chapter 4.2](ch04-02-references-and-borrowing.html#mutable-references-provide-unique-and-non-owning-access-to-data). Conversely, you would not be allowed to call `set_width` on a value of type `&Rectangle` or `&Box<Rectangle>`.

{{#quiz ../quizzes/ch05-03-method-syntax-sec1.toml}}


### Methods and Ownership

Like we discussed in Chapter 4.2 ["References and Borrowing"](ch04-02-references-and-borrowing.html), methods must be called on structs that have the necessary permissions. As a running example, we will use these three methods that take `&self`, `&mut self`, and `self`, respectively.

```rust,ignore
impl Rectangle {    
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle { 
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}
```

#### Reads and Writes with `&self` and `&mut self`

If we make an owned rectangle with `let rect = Rectangle { ... }`, then `rect` has @Perm{read} and @Perm{own} permissions. With those permissions, it is permissible to call the `area` and `max` methods:

```aquascope,permissions,boundaries,stepper
#struct Rectangle {
#    width: u32,
#    height: u32,
#}
#impl Rectangle {    
#  fn area(&self) -> u32 {
#    self.width * self.height
#  }
#
#  fn set_width(&mut self, width: u32) {
#    self.width = width;
#  }
#
#  fn max(self, other: Self) -> Self {
#    let w = self.width.max(other.width);
#    let h = self.height.max(other.height);
#    Rectangle { 
#      width: w,
#      height: h
#    }
#  }
#}
#fn main() {
let rect = Rectangle {
    width: 0,
    height: 0
};`(focus,rxpaths:^rect$)`
println!("{}", rect.area());`{}`

let other_rect = Rectangle { width: 1, height: 1 };
let max_rect = rect.max(other_rect);`{}`
#}
```

However, if we try to call `set_width`, we are missing the @Perm{write} permission:

```aquascope,permissions,boundaries,shouldFail
#struct Rectangle {
#    width: u32,
#    height: u32,
#}
#impl Rectangle {    
#  fn area(&self) -> u32 {
#    self.width * self.height
#  }
#
#  fn set_width(&mut self, width: u32) {
#    self.width = width;
#  }
#
#  fn max(self, other: Self) -> Self {
#    let w = self.width.max(other.width);
#    let h = self.height.max(other.height);
#    Rectangle { 
#      width: w,
#      height: h
#    }
#  }
#}
#fn main() {
let rect = Rectangle {
    width: 0,
    height: 0
};
rect.set_width(0);`{}`
#}
```

Rust will reject this program with the corresponding error:

```text
error[E0596]: cannot borrow `rect` as mutable, as it is not declared as mutable
  --> test.rs:28:1
   |
24 | let rect = Rectangle {
   |     ---- help: consider changing this to be mutable: `mut rect`
...
28 | rect.set_width(0);
   | ^^^^^^^^^^^^^^^^^ cannot borrow as mutable
```

We will get a similar error if we try to call `set_width` on an immutable reference to a `Rectangle`, even if the underlying rectangle is mutable:

```aquascope,permissions,boundaries,stepper,shouldFail
#struct Rectangle {
#    width: u32,
#    height: u32,
#}
#impl Rectangle {    
#  fn area(&self) -> u32 {
#    self.width * self.height
#  }
#
#  fn set_width(&mut self, width: u32) {
#    self.width = width;
#  }
#
#  fn max(self, other: Self) -> Self {
#    let w = self.width.max(other.width);
#    let h = self.height.max(other.height);
#    Rectangle { 
#      width: w,
#      height: h
#    }
#  }
#}
#fn main() {
// Added the mut keyword to the let-binding
let mut rect = Rectangle {
    width: 0,
    height: 0
};`(focus,rxpaths:^rect$)`
rect.set_width(1);`{}`     // this is now ok

let rect_ref = &rect;`(focus,rxpaths:^\*rect_ref$)`
rect_ref.set_width(2);`{}` // but this is still not ok
#}
```

#### Moves with `self`

Calling a method that expects `self` will move the input struct (unless the struct implements `Copy`). For example, we cannot use a `Rectangle` after passing it to `max`:

```aquascope,permissions,boundaries,stepper,shouldFail
#struct Rectangle {
#    width: u32,
#    height: u32,
#}
#impl Rectangle {    
#  fn area(&self) -> u32 {
#    self.width * self.height
#  }
#
#  fn set_width(&mut self, width: u32) {
#    self.width = width;
#  }
#
#  fn max(self, other: Self) -> Self {
#    let w = self.width.max(other.width);
#    let h = self.height.max(other.height);
#    Rectangle { 
#      width: w,
#      height: h
#    }
#  }
#}
#fn main() {
let rect = Rectangle {
    width: 0,
    height: 0
};`(focus,rxpaths:^rect$)`
let other_rect = Rectangle { 
    width: 1, 
    height: 1 
};
let max_rect = rect.max(other_rect);`(focus,rxpaths:^rect$)`
println!("{}", rect.area());`{}`
#}
```

Once we call `rect.max(..)`, we move `rect` and so lose all permissions on it. Trying to compile this program would give us the following error:

```text
error[E0382]: borrow of moved value: `rect`
  --> test.rs:33:16
   |
24 | let rect = Rectangle {
   |     ---- move occurs because `rect` has type `Rectangle`, which does not implement the `Copy` trait
...
32 | let max_rect = rect.max(other_rect);
   |                     --------------- `rect` moved due to this method call
33 | println!("{}", rect.area());
   |                ^^^^^^^^^^^ value borrowed here after move
```

A similar situation arises if we try to call a `self` method on a reference. For instance, say we tried to make a method `set_to_max` that assigns `self` to the output of `self.max(..)`:

```aquascope,permissions,boundaries,stepper,shouldFail
#struct Rectangle {
#    width: u32,
#    height: u32,
#}
impl Rectangle {    
#  fn area(&self) -> u32 {
#    self.width * self.height
#  }
#
#  fn set_width(&mut self, width: u32) {
#    self.width = width;
#  }
#
#  fn max(self, other: Self) -> Self {
#    let w = self.width.max(other.width);
#    let h = self.height.max(other.height);
#    Rectangle { 
#      width: w,
#      height: h
#    }
#  }
    fn set_to_max(&mut self, other: Rectangle) {`(focus,rxpaths:^\*self$)`
        *self = self.max(other);`{}`
    }
}
```

Then we can see that `self` is missing @Perm{own} permissions in the operation `self.max(..)`. Rust therefore rejects this program with the following error:

```text
error[E0507]: cannot move out of `*self` which is behind a mutable reference
  --> test.rs:23:17
   |
23 |         *self = self.max(other);
   |                 ^^^^^----------
   |                 |    |
   |                 |    `*self` moved due to this method call
   |                 move occurs because `*self` has type `Rectangle`, which does not implement the `Copy` trait
   |
```

This is the same kind of error we discussed in Chapter 4.3 ["Copying vs. Moving Out of a Collection"](ch04-03-fixing-ownership-errors.html#fixing-an-unsafe-program-copying-vs-moving-out-of-a-collection).

#### Good Moves and Bad Moves

You might wonder: why does it matter if we move out of `*self`? In fact, for the case of `Rectangle`, it actually is safe to move out of `*self`, even though Rust doesn't let you do it. For example, if we simulate a program that calls the rejected `set_to_max`, you can see how nothing unsafe occurs:

```aquascope,interpreter,shouldFail,horizontal
#struct Rectangle {
#    width: u32,
#    height: u32,
#}
impl Rectangle {    
#  fn max(self, other: Self) -> Self {
#    let w = self.width.max(other.width);
#    let h = self.height.max(other.height);
#    Rectangle { 
#      width: w,
#      height: h
#    }
#  }
    fn set_to_max(&mut self, other: Rectangle) {
        let max = self.max(other);`[]`
        *self = max;
    }
}

fn main() {
    let mut rect = Rectangle { width: 0, height: 1 };
    let other_rect = Rectangle { width: 1, height: 0 };`[]`
    rect.set_to_max(other_rect);`[]`
}
```

The reason it's safe to move out of `*self` is because `Rectangle` does not own any heap data.
In fact, we can actually get Rust to compile `set_to_max` by simply adding `#[derive(Copy, Clone)]` to the definition of `Rectangle`:

```aquascope,permissions,boundaries,stepper
\#[derive(Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {    
#  fn max(self, other: Self) -> Self {
#    let w = self.width.max(other.width);
#    let h = self.height.max(other.height);
#    Rectangle { 
#      width: w,
#      height: h
#    }
#  }
    fn set_to_max(&mut self, other: Rectangle) {`(focus,rxpaths:^\*self$)`
        *self = self.max(other);`{}`
    }
}
```

Notice that unlike before, `*self` now has the @Perm{own} permission. We are allowed to call an owned-self method like `max`.

You might wonder: why doesn't Rust automatically derive `Copy` for `Rectangle`? Rust does not auto-derive `Copy` for stability across API changes. Imagine that the author of the `Rectangle` type decided to add a `name: String` field. Then all client code that relies on `Rectangle` being `Copy` would suddenly get rejected by the compiler. To avoid that issue, API authors must explicitly add `#[derive(Copy)]` to indicate that they expect their struct to always be `Copy`.

To better understand the issue, let's run a simulation. Say we added `name: String` to `Rectangle`. What would happen if Rust allowed `set_to_max` to compile?

```aquascope,interpreter,shouldFail,horizontal
struct Rectangle {
    width: u32,
    height: u32,
    name: String,
}

impl Rectangle {    
#  fn max(self, other: Self) -> Self {
#    let w = self.width.max(other.width);
#    let h = self.height.max(other.height);
#    Rectangle { 
#      width: w,
#      height: h,
#      name: String::from("max")
#    }
#  }
    fn set_to_max(&mut self, other: Rectangle) {
        `[]`let max = self.max(other);`[]`
        drop(*self);`[]` // This is usually implicit,
                         // but added here for clarity.
        *self = max;
    }
}

fn main() {
    let mut r1 = Rectangle { 
        width: 9, 
        height: 9, 
        name: String::from("r1") 
    };
    let r2 = Rectangle {
        width: 16,
        height: 16,
        name: String::from("r2")
    };
    r1.set_to_max(r2);
}
```

In this program, we call `set_to_max` with two rectangles `r1` and `r2`. `self` is a mutable reference to `r1` and `other` is a move of `r2`. After calling `self.max(other)`, the `max` method consumes ownership of both rectangles. When `max` returns, Rust deallocates both strings "r1" and "r2" in the heap. Notice the problem: at the location L2, `*self` is supposed to be readable and writable. However, `(*self).name` (actually `r1.name`) has been deallocated.

Therefore when we do `*self = max`, we encounter undefined behavior. When we overwrite `*self`, Rust will implicitly drop the data previously in `*self`. To make that behavior explicit, we have added `drop(*self)`. After calling `drop(*self)`, Rust attempts to free `(*self).name` a second time. That action is a double-free, which is undefined behavior.

So remember: when you see an error like "cannot move out of `*self`", that's usually because you're trying to call a `self` method on a reference like `&self` or `&mut self`. Rust is protecting you from a double-free.


## Summary

Structs let you create custom types that are meaningful for your domain. By
using structs, you can keep associated pieces of data connected to each other
and name each piece to make your code clear. In `impl` blocks, you can define
functions that are associated with your type, and methods are a kind of
associated function that let you specify the behavior that instances of your
structs have.

But structs aren’t the only way you can create custom types: let’s turn to
Rust’s enum feature to add another tool to your toolbox.

{{#quiz ../quizzes/ch05-03-method-syntax-sec2.toml}}