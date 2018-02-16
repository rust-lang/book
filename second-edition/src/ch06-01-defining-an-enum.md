## Definiendo una enumeración

Veamos una situación que podríamos querer expresar en código y veamos
por que las enumeraciones son útiles y más apropiadas que las estructuras
en este caso. Digamos que necesitamos trabajar con direcciones IP. 
Actualmente, se usan dos estándares mayores para las direcciones IP:
la versión cuatro y versión seis. Estas son las unicas posibilidades
para una dirección IP que nuestro programa encontrará: podemos *enumerar*
todos los valores posibles,que es donde la enumeración obtiene su nombre.

Cualquier dirección IP puede ser una versión cuatro o una versión seis
pero no ambas al mismo tiempo. Esa propiedad de las direcciones IP hace
la estructura de datos de enumeración apropiada para este caso, porque
los valores de la enumeración puede ser solo una de las variantes. Ambas
versiones, la cuatro como la seis continúan siendo fundamentalmente direcciones
IP, por lo tanto deben tratarse como del mismo tipo cuando el código está
manejando situaciones que se aplican a cualquier tipo de direcciones IP.

Podemos expresar este concepto en código redefiniendo una enumeración
`IpAddrKind` y listando los posibles tipos de direcciones IP que pueden
ser `V4` y `V6`. Estas son conocidas como *variants* de la enumeración:

```rust
enum IpAddrKind {
    V4,
    V6,
}
```


`IpAddrKind` ahora es un tipo de datos personalizado que podemos utilizar
en cualquier parte de nuestro código.

### Valores de enumeración

Podemos crear istancias de cada una de las dos variantes de `IpAddrKind` asi:

```rust
# enum IpAddrKind {
#     V4,
#     V6,
# }
#
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```
Tenga en cuenta que las variantes de la enumeración son espaciadas bajo
su identificador, y nosotros usamos dos puntos para separarlas. La razón
por la que esto es util es que ahora ambos valores `IpAddrKind::V4`
y `IpAddrKind::V6` son del mismo tipo: `IpAddrKind`. Podemos entonces,
por ejemplo, definir una función que tome cualquier `IpAddrKind`:

```rust
# enum IpAddrKind {
#     V4,
#     V6,
# }
#
fn route(ip_type: IpAddrKind) { }
```

Y podemos llamar a esta función con cualquiera de las variantes:


```rust
# enum IpAddrKind {
#     V4,
#     V6,
# }
#
# fn route(ip_type: IpAddrKind) { }
#
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

Usar enumeraciones tiene incluso más ventajas. Pensando más sobre
nuestro tipo de dirección IP, por el momento no tenemos forma de 
almacenar la dirección IP actual *datos*; solo sabemos que *tipo* es. 
Teniendo en cuenta que acabas de aprender sobre estructuras en el
Capítulo 5, puedes llevar a cabo este problema como se muestra en el Listado 6-1:

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

<span class="caption">Listado 6-1: Almacenamiento de los datos
    y la variante `IpAddrKind` de una dirección IP usando una `struct`</span>

Aquí, hemos definido una estructura `IpAddr` que tiene dos campos: un `kind` campo
que es del tipo `IpAddrKind` (la enumeración que definimos previamente) y un `address` campo
de tipo `String`. Tenemos dos instancias de esta estructura. La primera, `home`, tiene
el valor `IpAddrKind::V4` como su `kind` con datos de direcciones asociadas de
`127.0.0.1`. La segunda instancia, `loopback`, tiene la otra variante de
`IpAddrKind` como su `kind` de valor, `V6`, y tiene la dirección `::1` asociada
con eso. Hemos usado una estructura para juntar los valores `kind` y
`address`, por lo que la variante ahora está asociada con el valor.

Podemos representar el mismo concepto de una forma más concisa
usando solo una enumeración en vez de una enumeración como parte
de una estructura al poner datos directamente en cada variante de
enumeración. Esta nueva definición de enumeración `IpAddr` dice
que tanto las variantes `V4` y `V6` tendrán asociados valores `String`:

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```


Adjuntamos datos a cada variante de la enumeración directamente, por
lo tanto no hay necesidad de una estructura extra.

Existe otra ventaja de usar una enumeración en lugar de una estructura: cada variante
puede tener diferentes tipos y cantidades de datos asociados. Las direcciones IP
de versión cuatro siempre tendrán cuatro componentes numéricos que tendrán
valores entre 0 y 255. Si deseamos almacenar direcciones `V4` como cuatro
valores `u8` pero seguimos expresando las direcciones `V6` como un valor `String`,
no podríamos con una estructura. Las enumeraciones manejan este caso con facilidad :

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

Mostramos muchas posibilidades diferentes que podríamos definir
en nuestro código para almacenar direcciones IP de dos variedades diferentes
usando una enumeración. Sin embargo, como resultado,deseando almacenar
direcciones IP y codificar de que tipo son es tan común que
[la biblioteca estándar tiene una definición que podemos usar!][IpAddr]
<!-- ignorar --> Veamos como la biblioteca estándar define 
`IpAddr`: tiene la enumeración exacta y las variantes que hemos
definido y usado, pero esto inserta los datos de riección adentro de las variantes
en forma de dos estructuras diferentes, que son definidas diferentemente para cada variante:

[IpAddr]: ../../std/net/enum.IpAddr.html

```rust
struct Ipv4Addr {
    // details elided
}

struct Ipv6Addr {
    // details elided
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

Este código ilustra que puedes poner cualquier tipo de datos dentro de una variante de enumeración:
cadenas, tipos numéricos, o estructuras, por ejemplo. Incluso puedes incluir otra enumeración!
También, los tipos de biblioteca estándar son a menudo no mucho más complicadas de lo que podrías imaginar.

Tenga en cuenta que aunque la biblioteca estándar contiene una definición para `IpAddr`,
aún podemos crear y usar nuestra propia definición sin conflictos porque
no hemos traido la definición de bibliotecas estándar a nuestro alcance. Hablaremos más
sobre importar tipos en el Capítulo 7.

Veamos otro ejemplo de una enumeración en el Listado 6-2: este tiene
una amplia variedad de tipos embebidos en sus variantes:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

<span class="caption">Listado 6-2: Una enumeración de `Message` cuyas
    variantes almacene cada una diferentes cantidades y tipos de valores</span>

Esta enumeración tiene cuatro variantes con diferentes tipos:

* `Quit` no tiene datos asociados con el del todo.
* `Move` incluye una estructura anónima dentro de ella.
* `Write` incluye un solo `String`.
* `ChangeColor` incluye tres `i32`.

La definición de una enumeración con variantes como las del Listado 6-2 es similar a 
la definición de distintos tipos de definiciones de estructura, excepto que
la enumeración no usa la palabra clave `struct` y todas las variantes estan
agrupadas bajo el tipo `Message`. Las siguientes estructuras podrían contener
los mismos datos que las variantes de contención de la enumeración precedente: 

```rust
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```

But if we used the different structs, which each have their own type, we
wouldn’t be able to as easily define a function that could take any of these
kinds of messages as we could with the `Message` enum defined in Listing 6-2,
which is a single type.

There is one more similarity between enums and structs: just as we’re able to
define methods on structs using `impl`, we’re also able to define methods on
enums. Here’s a method named `call` that we could define on our `Message` enum:

```rust
# enum Message {
#     Quit,
#     Move { x: i32, y: i32 },
#     Write(String),
#     ChangeColor(i32, i32, i32),
# }
#
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

The body of the method would use `self` to get the value that we called the
method on. In this example, we’ve created a variable `m` that has the value
`Message::Write("hello")`, and that is what `self` will be in the body of the
`call` method when `m.call()` runs.

Let’s look at another enum in the standard library that is very common and
useful: `Option`.

### The `Option` Enum and Its Advantages Over Null Values

In the previous section, we looked at how the `IpAddr` enum let us use Rust’s
type system to encode more information than just the data into our program.
This section explores a case study of `Option`, which is another enum defined
by the standard library. The `Option` type is used in many places because it
encodes the very common scenario in which a value could be something or it
could be nothing. Expressing this concept in terms of the type system means the
compiler can check that you’ve handled all the cases you should be handling,
which can prevent bugs that are extremely common in other programming languages.

Programming language design is often thought of in terms of which features you
include, but the features you exclude are important too. Rust doesn’t have the
null feature that many other languages have. *Null* is a value that means there
is no value there. In languages with null, variables can always be in one of
two states: null or not-null.

In “Null References: The Billion Dollar Mistake,” Tony Hoare, the inventor of
null, has this to say:

> I call it my billion-dollar mistake. At that time, I was designing the first
> comprehensive type system for references in an object-oriented language. My
> goal was to ensure that all use of references should be absolutely safe, with
> checking performed automatically by the compiler. But I couldn't resist the
> temptation to put in a null reference, simply because it was so easy to
> implement. This has led to innumerable errors, vulnerabilities, and system
> crashes, which have probably caused a billion dollars of pain and damage in
> the last forty years.

The problem with null values is that if you try to actually use a value that’s
null as if it is a not-null value, you’ll get an error of some kind. Because
this null or not-null property is pervasive, it’s extremely easy to make this
kind of error.

However, the concept that null is trying to express is still a useful one: a
null is a value that is currently invalid or absent for some reason.

The problem isn’t with the actual concept but with the particular
implementation. As such, Rust does not have nulls, but it does have an enum
that can encode the concept of a value being present or absent. This enum is
`Option<T>`, and it is [defined by the standard library][option]<!-- ignore -->
as follows:

[option]: ../../std/option/enum.Option.html

```rust
enum Option<T> {
    Some(T),
    None,
}
```

The `Option<T>` enum is so useful that it’s even included in the prelude; you
don’t need to import it explicitly.  In addition, so are its variants: you can
use `Some` and `None` directly without prefixing them with `Option::`.
`Option<T>` is still just a regular enum, and `Some(T)` and `None` are still
variants of type `Option<T>`.

The `<T>` syntax is a feature of Rust we haven’t talked about yet. It’s a
generic type parameter, and we’ll cover generics in more detail in Chapter 10.
For now, all you need to know is that `<T>` means the `Some` variant of the
`Option` enum can hold one piece of data of any type. Here are some examples of
using `Option` values to hold number types and string types:

```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

If we use `None` rather than `Some`, we need to tell Rust what type of
`Option<T>` we have, because the compiler can't infer the type that the `Some`
variant will hold by looking only at a `None` value.

When we have a `Some` value, we know that a value is present, and the value is
held within the `Some`. When we have a `None` value, in some sense, it means
the same thing as null: we don’t have a valid value. So why is having
`Option<T>` any better than having null?

In short, because `Option<T>` and `T` (where `T` can be any type) are different
types, the compiler won’t let us use an `Option<T>` value as if it was
definitely a valid value. For example, this code won’t compile because it’s
trying to add an `i8` to an `Option<i8>`:

```rust,ignore
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

If we run this code, we get an error message like this:

```text
error[E0277]: the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is
not satisfied
 -->
  |
7 | let sum = x + y;
  |           ^^^^^
  |
```

Intense! In effect, this error message means that Rust doesn’t understand how
to add an `Option<i8>` and an `i8`, because they’re different types. When we
have a value of a type like `i8` in Rust, the compiler will ensure that we
always have a valid value. We can proceed confidently without having to check
for null before using that value. Only when we have an `Option<i8>` (or
whatever type of value we’re working with) do we have to worry about possibly
not having a value, and the compiler will make sure we handle that case before
using the value.

In other words, you have to convert an `Option<T>` to a `T` before you can
perform `T` operations with it. Generally, this helps catch one of the most
common issues with null: assuming that something isn’t null when it actually
is.

Not having to worry about missing an assumption of having a not-null value
helps you to be more confident in your code. In order to have a value that can
possibly be null, you must explicitly opt in by making the type of that value
`Option<T>`. Then, when you use that value, you are required to explicitly
handle the case when the value is null. Everywhere that a value has a type that
isn’t an `Option<T>`, you *can* safely assume that the value isn’t null. This
was a deliberate design decision for Rust to limit null’s pervasiveness and
increase the safety of Rust code.

So, how do you get the `T` value out of a `Some` variant when you have a value
of type `Option<T>` so you can use that value? The `Option<T>` enum has a large
number of methods that are useful in a variety of situations; you can check
them out in [its documentation][docs]<!-- ignore -->. Becoming familiar with
the methods on `Option<T>` will be extremely useful in your journey with Rust.

[docs]: ../../std/option/enum.Option.html

In general, in order to use an `Option<T>` value, we want to have code that
will handle each variant. We want some code that will run only when we have a
`Some(T)` value, and this code is allowed to use the inner `T`. We want some
other code to run if we have a `None` value, and that code doesn’t have a `T`
value available. The `match` expression is a control flow construct that does
just this when used with enums: it will run different code depending on which
variant of the enum it has, and that code can use the data inside the matching
value.
