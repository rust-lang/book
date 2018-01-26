% Universal Function Call Syntax

<small>There is a new edition of the book and this is an old link.</small>

> Rust cannot prevent a trait from having a method with the same name as another trait’s method, nor can it prevent us from implementing both of these traits on one type.
> In order to be able to call each of the methods with the same name, then, we need to tell Rust which one we want to use.

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
#     fn fly(&self) {
#         println!("This is your captain speaking.");
#     }
}

impl Wizard for Human {
#     fn fly(&self) {
#         println!("Up!");
#     }
}

impl Human {
#     fn fly(&self) {
#         println!("*waving arms furiously*");
#     }
}

fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
```

---

Here are the relevant sections in the new and old books:

* **[In the second edition: Ch 19.03 — Advanced Traits, section Fully Qualified Syntax][2]**
* <small>[In the first edition: Ch 3.24 — Universal Function Call Syntax][1]</small>


[1]: first-edition/ufcs.html
[2]: second-edition/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name
