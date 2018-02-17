## Definición e Instanciación de Estructuras

Las estructuras son similares a las tuplas, que fueron discutidas en el Capítulo 3. Al igual que las tuplas,
las piezas de una estructura pueden ser de diferentes tipos. A diferencia de las tuplas, nombramos cada
dato para que quede claro el significado de los valores. Como resultado de estos nombres, 
las estructuras son más flexibles que las tuplas: no tenemos que depender del orden de
los datos para especificar o acceder a los valores de una instancia.

Para definir una estructura, introducimos la palabra clave `struct` y nombramos toda la estructura. El
nombre de una estructura debe describir el significado de las piezas de datos que se están 
agrupando. Luego, dentro de las llaves, definimos los nombres y tipos de las piezas de
datos, que llamamos *campos*. Por ejemplo, el Listado 5-1 muestra una
estructura para almacenar información sobre una cuenta de usuario:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

<span class="caption">Listado 5-1: Una definición de estructura de `User`</span>

Para utilizar una estructura después de haberla definido, creamos una *instancia* de esa estructura 
especificando valores concretos para cada uno de los campos. Creamos una instancia 
indicando el nombre de la estructura, y luego añadimos llaves que contienen pares 
`key: value` donde las teclas son los nombres de los campos y los valores son los
datos que queremos almacenar en esos campos. No es necesario especificar los campos en 
el mismo orden en el que los declaramos en la estructura. En otras palabras, la definición 
de estructura es como una plantilla general para el tipo, y las instancias completan 
esa plantilla con datos particulares para crear valores del tipo. Por
ejemplo, podemos declarar un usuario particular como se muestra en la lista 5-2:

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

<span class="caption">Listado 5-2: Creación de una instancia de la estructura
`User`</span>

Para obtener un valor específico de una estructura, podemos usar la notación de puntos. Si sólo queríamos
la dirección de correo electrónico de este usuario, podemos usar `user1. email` donde queramos
usar este valor. Para cambiar un valor en una estructura, si la instancia es mutable, podemos
usar la notación de puntos y asignarla a un campo en particular. El listado 5-3 muestra
cómo cambiar el valor en el campo `email` de una instancia de `User` mutable:

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

<span class="caption">Listado 5-3: Cambiar el valor del campo `email` de una instancia
de `User`</span>

Ten en cuenta que toda la instancia debe ser mutable; Rust no nos permite marcar
sólo ciertos campos como mutables. También nota que como con cualquier expresión, podemos
construir una nueva instancia de la estructura como última expresión en el cuerpo 
funcional para devolver implícitamente esa nueva instancia.

El listado 5-4 muestra una función `build_user` que devuelve una instancia de
`User` con el correo electrónico y el nombre de usuario dados. El campo `true` obtiene el valor de `true`, 
y el `"sign_in_count` obtiene un valor de `1`.

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

<span class="caption">Listado 5-4: Una función `build_user` que toma un correo electrónico
 y nombre de usuario y devuelve una instancia de `User`</span>

Tiene sentido nombrar los argumentos de función con el mismo nombre que los campos de
estructura, pero tener que repetir los nombres de campo y las variables `email` y 
`username` es un poco tedioso. Si la estructura tuviera más campos, repetir cada nombre
sería aún más molesto. Afortunadamente, ¡hay una abreviatura conveniente!

### Abreviatura de Campo Inicial cuando las Variables Tienen el Mismo Nombre que los Campos

Debido a que los nombres de los parámetros y los nombres de los campos de estructura son exactamente los mismos en
el Listado 5-4, podemos usar la sintaxis *field init shorthand* para reescribir 
`build_user` para que se comporte exactamente igual pero no tenga la
repetición de `email` y `username` en la forma que se muestra en el Listado 5-5.

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

<span class="caption">Listado 5-5: Una función `build_user` que utiliza la abreviatura 
del campo ya que los parámetros `email` y `username` tienen el mismo nombre que
los campos de estructura.</span>

Aquí, estamos creando una nueva instancia de la estructura `User`, que tiene un campo 
llamado `email`. Queremos establecer el valor del campo `email` al valor del
parámetro `email` de la función `build_user`. Debido a que el campo `email` y el
parámetro `email` tienen el mismo nombre, sólo necesitamos escribir `email` en lugar 
de `email: email`.

### Creación de Instancias Desde otras Instancias con la Sintaxis de Actualización de Estructura

A menudo es útil crear una nueva instancia de una estructura que utiliza la mayoría de los valores de una
vieja instancia, pero cambia algunos. Lo hacemos usando *struct update syntax*.

En primer lugar, el Listado 5-6 muestra cómo creamos una nueva instancia de `User` en `user2` sin 
la sintaxis de actualización. Establecemos nuevos valores para `email` y `username`, pero por lo demás
usamos los mismos valores de `user1` que creamos en Listado 5-2:

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

<span class="caption">Listado 5-6: Creando una nueva instancia de `User` usando algunos de
los valores de `user1`</span>

Usando la sintaxis de actualización estructural, podemos lograr el mismo efecto con menos código, como
se muestra en Listado 5-7. La sintaxis `..` especifica que los campos restantes no 
definidos explícitamente deben tener el mismo valor que los campos de la instancia dada.

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

<span class="caption">Listado 5-7: Usando la sintaxis de actualización de estructura para establecer nuevos
valores de `email` y `username` para una instancia de `User` pero usar el resto de los
valores de los campos de la instancia en la variable `user1`</span>

El código del Listado 5-7 también crea una instancia en `user2` que tiene un
valor diferente para los campos `email` y `username` pero tiene los mismos valores para 
los campos `active` y `sign_in_count` de `user1`.

### Tuple Structs without Named Fields to Create Different Types

We can also define structs that look similar to tuples (which were discussed in
Chapter 3), called *tuple structs*, that have the added meaning the struct name
provides, but don’t have names associated with their fields; rather, they just
have the types of the fields. Tuple structs are useful when you want to give
the whole tuple a name and make the tuple be a different type than other
tuples, but naming each field as in a regular struct would be verbose or
redundant.

To define a tuple struct you start with the `struct` keyword and the struct
name followed by the types in the tuple. For example, here are definitions and
usages of two tuple structs named `Color` and `Point`:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

Note that the `black` and `origin` values are different types, since they’re
instances of different tuple structs. Each struct we define is its own type,
even though the fields within the struct have the same types. For example, a
function that takes a parameter of type `Color` cannot take a `Point` as an
argument, even though both types are made up of three `i32` values. Otherwise,
tuple struct instances behave like tuples: you can destructure them into their
individual pieces and you can use a `.` followed by the index to access an
individual value, and so on.

### Unit-Like Structs without Any Fields

We can also define structs that don’t have any fields! These are called
*unit-like structs* since they behave similarly to `()`, the unit type.
Unit-like structs can be useful in situations such as when you need to
implement a trait on some type, but you don’t have any data that you want to
store in the type itself. We’ll discuss traits in Chapter 10.

> ### Ownership of Struct Data
>
> In the `User` struct definition in Listing 5-1, we used the owned `String`
> type rather than the `&str` string slice type. This is a deliberate choice
> because we want instances of this struct to own all of its data and for that
> data to be valid for as long as the entire struct is valid.
>
> It’s possible for structs to store references to data owned by something else,
> but to do so requires the use of *lifetimes*, a Rust feature that we’ll
> discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct
> is valid for as long as the struct is. Let’s say you try to store a reference
> in a struct without specifying lifetimes, like this:
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
> The compiler will complain that it needs lifetime specifiers:
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
> In Chapter 10, we’ll discuss how to fix these errors so you can store
> references in structs, but for now, we’ll fix errors like these using owned
> types like `String` instead of references like `&str`.
