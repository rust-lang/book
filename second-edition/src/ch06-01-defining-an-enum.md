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
=======
We can represent the same concept in a more concise way using just an enum,
rather than an enum inside a struct, by putting data directly into each enum
variant. This new definition of the `IpAddr` enum says that both `V4` and `V6`
variants will have associated `String` values:


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
=======
Note that even though the standard library contains a definition for `IpAddr`,
we can still create and use our own definition without conflict because we
haven’t brought the standard library’s definition into our scope. We’ll talk
more about bringing types into scope in Chapter 7.


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
=======
* `Quit` has no data associated with it at all.
* `Move` includes an anonymous struct inside it.
* `Write` includes a single `String`.
* `ChangeColor` includes three `i32` values.


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

Pero si usamos diferentes estructuras, que cada una tiene su propio tipo, no seriamos
capaces de definir facilmente una función que podría tomar de estos tipos de
de mensajes como podriamos con la enumeración `Message` definida en el Listado 6-2,
que es un solo tipo.

Hay una similitud más entre enumeraciones y estructuras: de la misma forma que somos
capaces de definir en estructuras usando `impl`, también podemos definir métodos en
estructuras. Aquí hay un método llamado `call` que podriamos definir en nuestra 
enumeración `Message`:

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
        // el cuerpo del método se definiría aquí
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

El método del cuerpo usaría `self` para obtener el valor que llamamos el método
encendido. En este ejemplo, hemos creado una variable `m` que tiene el valor
`Message::Write(String::from("hello"))`, y eso es lo qué `self` tendrá en el cuerpo
del método `call` cuando `m.call()` se ejecute.

Veamos otra enumeración en la biblioteca estándar que es muy común y útil: `Option`

### La enumeración `Option` y sus ventajas sobre valores nulos

En la sección previa, hemos visto como la enumeración `IpAddr` nos dejo usar el
sistema de tipo Rust’s para codificar más información que solo los datos dentro de
nuestro programa. Esta sección explora un caso de estudio de `Option`, que es otra
enumeración definida por la biblioteca estándar. El tipo `Option`  es usado en muchos
lugares porque codifica un escenario muy común en que un valor podría tener algo o podría
no tener nada. Expresando este concepto en términos del tipo de sistema significa que
el compilador puede verificar que haya manejado todos los casos que debería manejar,
que puede prevenir errores que son extremadamente comunes en otros lenguajes de programación.

El diseño del lenguaje de programación a menudo se piensa en términos de que características
incluir, pero las características que excluyes también son importantes. Rust no tiene la 
característica nula que muchos otros lenguajes tienen. *Null* es un valor que significa que no
tiene valor allí. En lenguajes con nulo, las variables pueden estar siempre en uno o dos estados: 
nulo o no nulo.


En “Null References: The Billion Dollar Mistake,” Tony Hoare, el inventor del nulo, tiene
esto para decir::

> Yo llamo esto mi error billonario. En ese momento, estaba diseñando el 
> sistema de tipo completo para referencias en un lenguaje oriendato a objetos. Mi
> objetivo fue asegurar que todo el uso de referencias debería ser absolutamente
> seguro, con verificacion realizada automáticamente por el compilador. Pero no pude
> reistir la  de poner en una referencia nula, simplemente porque era más fácil de
> implementar. Esto ha ocasionado innumerables errores, vulnerabilidades y fallos del
> sistema, que probablemente hayan causado un billón de dolares de dolor y daño en
> los últimos 40 años.

El problema con los valores nulos es que si intentas usar un valor que es nulo como si
no fuera un valor nulo, obtendrás un error de algún tipo. Porque esta propiedad nula o no nula 
es penetrante, es extremadamente fácil hacer este tipo de error.


Sin embargo, el concepto que nulo está intentando intentando expresar sigue siendo útil: un
nulo es un valor que es actualmente inválido o está ausente por alguna razón.

El problema no es con el concepto actual sino con la particular implementación. Como tal, Rust
no tiene nulos, pero tiene una enumeración que puede codificar el concepto de un valor
estando presente o ausente. Esta enumeración es
`Option<T>`, y es [definida por la bibliotca estándar][option]<!-- ignorar -->
de la siguiente manera:

[option]: ../../std/option/enum.Option.html

```rust
enum Option<T> {
    Some(T),
    None,
}
```

La enumeración `Option<T>` es tan útil que está incluida incluso en el preludio; no necesitas
incluirla en el alcance explicitamente. Además, también lo son sus variantes:
puedes usar `Some` y `None` directamente sin prefijarlos con `Option::`.
`Option<T>` sigue siendo solo una enumeración regular, y `Some(T)` y `None` siguen siendo
variantes de tipo `Option<T>`.

La sintaxis `<T>` es una característica de Rust que aún no hemos hablado. Es un parametro
de tipo genérico, y cubriremos genericos en más detalles en el capítulo 10.
Por ahora, todo lo que necesitas saber es que `<T>` significa la variante `Some` de 
la enumeración `Option` que puede contener una pieza de datos de cualquier tipo. Aquí
hay algunos ejemplos de usando los valores `Option` para contener tipos de números y tipos de cadenas:

```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

Si usamos `None` en vez de `Some`, necesitamos decirle a Rust que tipo de
`Option<T>` tenemos, porque el compilador no puede deducir el tipo que la variante `Some`
se quedará mirando solo al valor `None`.

Cuando tenemos un valor `Some`, sabemos que un valor está presente, y el valor es
se produce dentro de `Some`. Cuando tenemos un valor `None`, en cierto sentido, esto significa
lo mismo que nulo: no tenemos un valor válido. Entonces por qué `Option<T>` no está
teniendo nada mejor que nulo?


En conclusión, porque `Option<T>` y `T` (donde `T` puede ser de cualquier tipo) son tipos
diferentes, el compilador no le dejará usar un valor `Option<T>` como si este fuera
definitivamente un valor válido. Por ejemplo, este código no se compilará porque está
tratando de agregar un `i8` a un `Option<i8>`:

```rust,ignore
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

Si ejecutamos este código, recibimos un mensaje de error como este:

```text
error[E0277]: the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is
not satisfied
 -->
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + std::option::Option<i8>`
  |
```

Intenso! En efecto, este mensaje de error significa que Rust no etiende como
agregar un `i8` y un `Option<i8>`, porque son diferentes tipos. Cuando tenemos
un valor de un tipo como `i8` en Rust, el compilador se asegurará que
siempre tengamos un valor válido. Podemos proceder con confianza sin tener que
verificar para nulo antes de usar ese valor. Solo cuando tenemos una `Option<i8>` (o
cualquier tipo de valor con el que estemos trabajando) tenemos que tener cuidado
con la posibilidad de no tener un valor, y el compilador se asegurará que manejamos
ese caso antes de usar el valor.

En otras palabras, debes convetir una `Option<T>` en una `T` antes de poder
realizar operaciones `T` con el. Generalmente, estas ayudas a atrapar uno de los 
errores más comunes con nulos: asumiendo que algo no es nulo cuando realmente lo es.

No se preocupe sobre por perder la suposición de que tener un valor no nulo
ayuda a tener más confianza en tu código. Para tener un valor que pueda ser posiblemente
nulo, debe optar explicitamente en seguir haciendo el tipo del valor
`Option<T>`. Luego, cuando uses ese valor, se requiere manejar explicitamente
el caso cuando el valor es nulo. En todo lugar que un valor tiene un tipo que
no es una `Option<T>`, usted *can* puede asumir con toda seguridad que el valor no es nulo.
Esta fue una decisión de diseño deliberada para Rust para limitar la penetrabilidad de nulos
e incrementar la seguridad del código Rust.

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
