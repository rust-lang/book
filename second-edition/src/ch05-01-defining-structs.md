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
