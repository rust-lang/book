% Trait Objects

<small>There is a new edition of the book and this is an old link.</small>

> Trait objects combine the data made up of the pointer to a concrete object with the behavior of the methods defined in the trait.
> A trait defines behavior that we need in a given situation.
> We can then use a trait as a trait object in places where we would use a concrete type or a generic type.

```rust,ignore
pub struct InputBox {
    pub label: String,
}

impl Draw for InputBox {
    fn draw(&self) {
        // Code to actually draw an input box
    }
}

pub struct Button {
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // Code to actually draw a button
    }
}

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
    where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(InputBox {
                label: String::from("OK"),
            }),
            Box::new(Button {
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```

---

Here are the relevant sections in the new and old books:

* **[In the second edition: Ch 17.02 — Trait Objects][2]**
* <small>[In the first edition: Ch 3.22 — Trait Objects][1]</small>


[1]: first-edition/trait-objects.html
[2]: second-edition/ch17-02-trait-objects.html
