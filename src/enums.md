# Enums

Rust has a second kind of type definition that is similar to structures:
enumerations. ‘Enums’, as they’re more commonly known, a data type whose values
are one of several variants, each of which carries different compound data
inside.

Here’s the simplest way to use enums:

```rust
enum StopLight {
    Red,
    Yellow,
    Green,
}
```

A value of type `StopLight` can be one of three variants: `Red`, `Yellow`, or
`Green`. It cannot be multiple types at once. This is what makes enums
different from structs: a struct must have a value for all of its members. An
enum is a single value from the list of options.

Let’s implement a function on `StopLight` that cycles between these options:

```rust
# #[derive(PartialEq,Debug)]
# enum StopLight {
#     Red,
#     Yellow,
#     Green,
# }
impl StopLight {
    fn next_color(self) -> StopLight {
        if self == StopLight::Red {
            StopLight::Green
        } else if self == StopLight::Yellow {
            StopLight::Red
        } else {
            StopLight::Yellow
        }
    }
}

let light = StopLight::Red;

let light = light.next_color();
assert_eq!(light, StopLight::Green);

let light = light.next_color();
assert_eq!(light, StopLight::Yellow);

let light = light.next_color();
assert_eq!(light, StopLight::Red);
```

## Enums with values

## Enums with discriminants
