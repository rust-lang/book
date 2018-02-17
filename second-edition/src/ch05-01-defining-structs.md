## Defining and Instantiating Structs

Structs are similar to tuples, which were discussed in Chapter 3. Like tuples,
the pieces of a struct can be different types. Unlike tuples, we name each
piece of data so it’s clear what the values mean. As a result of these names,
structs are more flexible than tuples: we don’t have to rely on the order of
the data to specify or access the values of an instance.

To define a struct, we enter the keyword `struct` and name the entire struct. A
struct’s name should describe the significance of the pieces of data being
grouped together. Then, inside curly brackets, we define the names and types of
the pieces of data, which we call *fields*. For example, Listing 5-1 shows a
struct to store information about a user account:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

<span class="caption">Listing 5-1: A `User` struct definition</span>

To use a struct after we’ve defined it, we create an *instance* of that struct
by specifying concrete values for each of the fields. We create an instance by
stating the name of the struct, and then add curly brackets containing `key:
value` pairs where the keys are the names of the fields and the values are the
data we want to store in those fields. We don’t have to specify the fields in
the same order in which we declared them in the struct. In other words, the
struct definition is like a general template for the type, and instances fill
in that template with particular data to create values of the type. For
example, we can declare a particular user as shown in Listing 5-2:

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

<span class="caption">Listing 5-2: Creating an instance of the `User`
struct</span>

To get a specific value from a struct, we can use dot notation. If we wanted
just this user’s email address, we can use `user1.email` wherever we want to
use this value. If the instance is mutable, we can change a value by using the
dot notation and assigning into a particular field. Listing 5-3 shows how to
change the value in the `email` field of a mutable `User` instance:

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```

<span class="caption">Listing 5-3: Changing the value in the `email` field of a
`User` instance</span>

Note that the entire instance must be mutable; Rust doesn’t allow us to mark
only certain fields as mutable. Also note that as with any expression, we can
construct a new instance of the struct as the last expression in the function
body to implicitly return that new instance.

Listing 5-4 shows a `build_user` function that returns a `User` instance with
the given email and username. The `active` field gets the value of `true`, and
the `sign_in_count` gets a value of `1`.

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

<span class="caption">Listing 5-4: A `build_user` function that takes an email
and username and returns a `User` instance</span>

It makes sense to name the function arguments with the same name as the struct
fields, but having to repeat the `email` and `username` field names and
variables is a bit tedious. If the struct had more fields, repeating each name
would get even more annoying. Luckily, there’s a convenient shorthand!

### Using the Field Init Shorthand when Variables and Fields Have the Same Name

Because the parameter names and the struct field names are exactly the same in
Listing 5-4, we can use the *field init shorthand* syntax to rewrite
`build_user` so that it behaves exactly the same but doesn’t have the
repetition of `email` and `username` in the way shown in Listing 5-5.

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

<span class="caption">Listing 5-5: A `build_user` function that uses field init
shorthand since the `email` and `username` parameters have the same name as
struct fields</span>

Here, we’re creating a new instance of the `User` struct, which has a field
named `email`. We want to set the `email` field’s value to the value in the
`email` parameter of the `build_user` function. Because the `email` field and
the `email` parameter have the same name, we only need to write `email` rather
than `email: email`.

### Creating Instances From Other Instances With Struct Update Syntax

It’s often useful to create a new instance of a struct that uses most of an old
instance’s values, but changes some. We do this using *struct update syntax*.

First, Listing 5-6 shows how we create a new `User` instance in `user2` without
the update syntax. We set new values for `email` and `username`, but otherwise
use the same values from `user1` that we created in Listing 5-2:

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
# let user1 = User {
#     email: String::from("someone@example.com"),
#     username: String::from("someusername123"),
#     active: true,
#     sign_in_count: 1,
# };
#
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```

<span class="caption">Listing 5-6: Creating a new `User` instance using some of
the values from `user1`</span>

Using struct update syntax, we can achieve the same effect with less code, as
shown in Listing 5-7. The syntax `..` specifies that the remaining fields not
explicitly set should have the same value as the fields in the given instance.

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
# let user1 = User {
#     email: String::from("someone@example.com"),
#     username: String::from("someusername123"),
#     active: true,
#     sign_in_count: 1,
# };
#
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

<span class="caption">Listing 5-7: Using struct update syntax to set new
`email` and `username` values for a `User` instance but use the rest of the
values from the fields of the instance in the `user1` variable</span>

The code in Listing 5-7 also creates an instance in `user2` that has a
different value for `email` and `username` but has the same values for the
`active` and `sign_in_count` fields from `user1`.

### Estructuras de Tupla sin Campos Especificados Para Crear Tipos Diferentes

También podemos definir estructuras que parecen similares a tuplas (que se discutieron en 
el Capítulo 3), llamadas *tuple estructs*, que tienen el significado agregado que el nombre
de la estructura proporciona, pero no tienen nombres asociados con sus campos; más bien, sólo tienen 
los tipos de los campos. Las estructuras de tupla son útiles cuando se quiere dar un 
nombre a la tupla entera y hacer que la tupla sea un tipo diferente a otras 
tuplas, pero nombrar cada campo como en una estructura regular sería verboso o 
redundante.

Para definir una estructura doble se empieza con la palabra clave `struct` y el nombre
de la estructura seguido por los tipos en la tupla. Por ejemplo, aquí están las definiciones
y usos de dos estructuras dobles llamadas `Color` y `Point`:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

Ten en cuenta que los valores `black` y `originn` son diferentes tipos, ya que son
instancias de diferentes estructuras de tuplas. Cada estructura que definimos es su propio tipo,
aunque los campos dentro de la estructura tienen los mismos tipos. Por ejemplo, una
función que toma un parámetro del tipo `Color` no puede tomar un `Point` como 
argumento, aunque ambos tipos estén compuestos de tres valores `i32`. De otro modo,
las instancias de estructura tuplas se comportan como tuplas: puedes desestructurarlas en sus piezas
individuales y puedes usar un `.` seguido por el índice para acceder a un
valor individual, y así sucesivamente.

### Estructuras Unitarias sin Ningún Campo

También podemos definir estructuras que no tienen campos! Éstas se denominan
"unit-like structs" ya que se comportan de forma similar a `()`, el tipo de unidad.
Las estructuras unitarias pueden ser útiles en situaciones tales como cuando necesites
implementar un rasgo en algún tipo, pero no se tiene ningún dato que quieras
almacenar en el mismo tipo. Discutiremos los rasgos en el capítulo 10.

> ### Posesión de los Datos de una Estructura
>
> En la definición de la estructura de `User` en el Listado 5-1, usamos el propio tipo
> de `String` en lugar del tipo de slice de cadena `&str`. Esta es una elección deliberada
> porque queremos que las instancias de esta estructura posean todos sus datos y que estos
> datos sean válidos mientras la estructura entera sea válida.
>
> Es posible que las estructuras almacenen referencias a datos que pertenecen a otra cosa,
> pero para ello se requiere el uso de *lifetimes*, una función de Rust que 
> discutiremos en el Capítulo 10. La vida útil asegura que los datos referenciados por una estructura
> son válidos mientras la estructura sea válida. Digamos que intentas guardar una referencia 
> en una estructura sin especificar la vida útil, como ésta:
>
> <span class="filename">Filename: src/main.rs</span>
>
> ```rust,ignore
> struct User {
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
>     active: bool,
> }
>
> fn main() {
>     let user1 = User {
>         email: "someone@example.com",
>         username: "someusername123",
>         active: true,
>         sign_in_count: 1,
>     };
> }
> ```
>
> El compilador reclamará de que necesita especificadores de vida útil:
>
> ```text
> error[E0106]: missing lifetime specifier
>  -->
>   |
> 2 |     username: &str,
>   |               ^ expected lifetime parameter
>
> error[E0106]: missing lifetime specifier
>  -->
>   |
> 3 |     email: &str,
>   |            ^ expected lifetime parameter
> ```
>
> En el Capítulo 10, discutiremos cómo corregir estos errores para que puedas almacenar
> referencias en estructuras, pero por ahora, arreglaremos errores como estos usando tipos
> propios como `String` en lugar de referencias como `&str`.
