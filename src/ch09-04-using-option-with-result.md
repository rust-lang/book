## `Option` and error handing

The `Option` type handles cases where a value may or may not be present. The absence of a value does not necessarily indicate an error case - `Option` and `Result` are distinct types.

As an example, `HashMap`, which we previously explored [REF], returns an `Option` when we `get` a value from the map. To revisit our example from [REF]:

```rust
use std::collections::HashMap;

fn main() {

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores
        .get(&team_name)          // returns an Option<u32>
        .map(|i| i.to_string())   // maps Option<u32> to Option<String>
        .unwrap_or("Unk".into()); // unwrap with a default for None case

    println!("{}",score);
}
```

However, we sometimes want to treat `None` as an error. In these cases, `Option` provides `ok_or` and `ok_or_else`, which map `Option`s to `Result`s. For example, if our league has a defined number of teams, we can ensure that our `HashMap` has a score for every team. In this case, failure to retrieve a value is an *error* indicating some problem in our program, not an expected value to handle. Accordingly, we use `ok_or` to map a `None` to a desired error type.

```rust
use std::collections::HashMap;
use std::error::Error;

#[derive(PartialEq,Eq,Hash)]
enum Teams {
    Blue,
    Yellow
}

fn init_scores() -> HashMap<Teams,u32> {

    let mut scores = HashMap::new();
    scores.insert(Teams::Blue, 10);
    scores.insert(Teams::Yellow, 50);
    scores
}

fn score_for<'a>(scores: &'a HashMap<Teams,u32>, team: Teams) -> Result<&'a u32,String> {
    scores.get(&team).ok_or(String::from("Missing team error"))
}

fn main() -> Result<(), Box<dyn Error>> {
    let scores = init_scores();
    println!("Blue team score: {}",score_for(&scores, Teams::Blue)?);
    Ok(())
}
```

## Option chaining with `?`

In [REF] we discussed using the `?` operator as a shortcut for handling `Result` values. Using `?` we can "pretend" that we only need to handle the happy path where all functions return the expected `Ok` value, while preserving the ability to handle errors elsewhere in the program.

Just as we naturally have chained or nested `Result` values, we often find situations where `Option` values are chained or deeply nested. As an example, consider our sports league above. Some teams have mascots, but not all. The mascots may have optional facts about them. We very frequently want to lookup  team mascot facts when we have a `League` in hand, so we want a convenient way to reference them.

We might model this situation as in the following:

```rust
use std::collections::HashMap;

#[derive(PartialEq,Eq,Hash)]
enum Teams {
    Blue,
    Red,
    Yellow
}

struct League {
    //... fields omitted ... //
    teams: HashMap<Teams,Team>,
}

struct Team {
    //... fields omitted ..//
    mascot: Option<Mascot>,
}

struct Mascot {
    // ... fields omitted ..//
    facts: Option<String>
}

impl League {
    fn team_mascot(&self,team: Teams) -> Option<&str> {
        match self.teams.get(&team) {
            Some(t) =>
                match t.mascot {
                    Some(ref m) =>
                        match m.facts {
                            Some(ref f) => Some(f.as_str()),
                            None => None
                        },
                    None => None
                },
            None => None
        }
    }
}
```

This does the job, but the chain of nested `match` statements is pretty hard to read. Unfortunately `if let` doesn't help us much here:

```rust
impl League {
    fn team_mascot(&self,team: Teams) -> Option<&str> {
        if let Some(t) = self.teams.get(&team) {
            if let Some(ref m) = t.mascot {
                if let Some(ref f) = m.facts {
                    Some(f.as_str())
                } else {
                     None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
```

Fortunately, the `?` is specialized to `Option` as well as `Result`, and is defined similarly. If `foo` is of type `Option<T>`, we can think of `?` as rewriting `foo?` to:

```rust
match foo {
    Some(v) => v,
    None => return None
}
```

There are a couple of things to note here:

1) `?` takes `f` as an owned value rather than by reference
2) If `foo` is `None`, `?` *returns* None, rather than evaluating to a `None` value, as in our examples above.

In our function example, an early return does not change how our function behaves, but the requirement to own the value contained in the `Option` causes Rust to *move* the value into the function. Because we took `&self` by reference, we do not own it, so this casues an error.

The naïve rewriting of our chain of `match` or `if let` statements in terms of `?` is:

```rust
impl League {
    fn team_mascot(&self,team: Teams) -> Option<&str> {
        let fact = self.teams.get(&team)?
            .mascot?
            .facts?
            .as_str();
        Some(fact)
    }
}
```

This generates the following error:

```text
error[E0507]: cannot move out of borrowed content
  --> main.rs:27:20
   |
27 |         let fact = self.teams.get(&team)?
   |                    ^^^^^^^^^^^^^^^^^^^^^^ cannot move out of borrowed content
```

It is the line `mascot?` that casues the move; if we look at our rewriting example above, we will see that `Some(v) => v` wants to own the `Mascot` struct. Rewriting our chain of `?` to take references rather that owned values solves the problem:

```rust
impl League {
    fn team_mascot(&self,team: Teams) -> Option<&str> {
        let fact = self.teams
            .get(&team)?      // A `&Team` or return if `None`
            .mascot.as_ref()? // A `&Mascot` or return
            .facts.as_ref()?  // A `&String` or return
            .as_str();        // Yields a `&str`
        Some(fact)
    }
}
```

As a reminder, the `as_ref` method maps `Option<T>` to `Option<&T>`, so when `?` pulls out the inner value, we are holding a reference, not an owned value.

### Avoiding early returns with `?` using closures

In our example above, we created a function that pulled a value out of a chain of `Option`s. Sometimes, we just want to pull the final `Option` out of a chain in the flow of some other logic. Unfortunately, if any value in our chain is `None` this will generate an early return of `None` which is not the flow we want, and probably conflicts with our return value.

In this situation, we can construct and evaluate a closure that behaves like our function above, returning an `Option` to use in our flow.

As an example:

```rust
let possible_fact = { ||        // empty argument list for closure
    Some(self.teams.get(&team)? // return `Some` if all values are `Some`
        .mascot.as_ref()?       // or `None` if any values are `None`
        .facts.as_ref()?
        .as_str())
}(); // evalutes the closure

if let Some(fact) = possible_fact {
   println!("I have a fact: {}", fact);
} else {
   println!("No facts available");
}
```

Now that you’ve seen useful ways that the standard library uses generics with
the `Option` and `Result` enums, we’ll talk about how generics work and how you
can use them in your code.