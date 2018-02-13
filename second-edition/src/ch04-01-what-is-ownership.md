¿Qué es Propiedad?

La característica central de Rust es la *propiedad*. Aunque la característica es fácil de 
explicar, tiene profundas implicaciones para el resto del lenguaje.

Todos los programas tienen que administrar la forma en que utilizan la memoria del ordenador mientras se está ejecutando.
Algunos lenguajes tienen una colección de basura que busca constantemente la memoria que ya no se usa
mientras se ejecuta el programa; en otros lenguajes, el programador debe explícitamente 
asignar y liberar la memoria. Rust utiliza un tercer enfoque: la memoria se gestiona
a través de un sistema de propiedad con un conjunto de reglas que el compilador comprueba en
tiempo de compilación. No se producen costes de tiempo de ejecución para ninguna de las características de propiedad.

Debido a que la propiedad es un concepto nuevo para muchos programadores, toma algún tiempo 
acostumbrarse a ella. La buena noticia es que cuanto más experimentado seas con Rust
y las reglas del sistema de propiedad, serás más capaz de desarrollar código naturalmente
seguro y eficiente. ¡Sigue con ello!

Cuando entiendas la propiedad, tendrás una base sólida para comprender
las características que hacen que Rust sea único. En este capítulo, aprenderás la propiedad 
trabajando con algunos ejemplos que se centran en una estructura de datos muy común:
las cadenas.

<!-- PROD: START BOX -->

### La Pila y el Montón
>
>En muchos lenguajes de programación, no tenemos que pensar en la pila y
>el montón muy a menudo. Pero en un lenguaje de programación de sistemas como Rust, si un 
>valor está en la pila o en el montón tiene más efecto sobre cómo se comporta el lenguaje
>y por qué tenemos que tomar ciertas decisiones. Vamos a describir partes de 
>la propiedad en relación con la pila y el montón más adelante en este capítulo, así que aquí 
>hay una breve explicación en preparación.
>
>Tanto la pila como el montón son partes de la memoria que están disponibles para que tu código
>las utilice en tiempo de ejecución, pero están estructuradas de diferentes maneras. La pila almacena
>los valores en el orden en que se obtienen y elimina los valores en el orden opuesto.
>Esto se conoce como *última entrada, primera salida*.  Piensa en un montón de platos: cuando 
>añades más platos, los pones encima del montón, y cuando necesitas un
>plato, lo sacas de la parte superior. ¡Añadir o quitar platos del medio o
>del fondo no funcionaría tan bien! La adición de datos se llama *pushing en la pila*,
>y la eliminación de datos se llama *popping de la pila*.
>
>La pila es rápida debido a la forma en que accede a los datos: nunca tiene que
>buscar un lugar para poner nuevos datos o un lugar de donde obtener datos, porque ese
>lugar siempre es la cima. Otra propiedad que hace que la pila sea rápida es que todos los
>datos de la pila deben tener un tamaño conocido y fijo.
>
>Para los datos con un tamaño desconocido para nosotros al momento de compilar o un tamaño que podría cambiar,
>podemos almacenar los datos en el montón en su lugar. El montón es menos organizado: cuando ponemos 
>datos sobre el montón, pedimos algo de espacio. El sistema operativo encuentra
>un punto vacío en algún lugar del montón que es lo suficientemente grande, lo marca como en
>uso, y nos devuelve un puntero a esa ubicación. Este proceso se llama
>*asignar en el montón*, y a veces abreviamos la frase simplemente como
>"asignar". Los valores de empuje sobre la pila no se consideran asignados.
>Debido a que el puntero es un tamaño conocido y fijo, podemos almacenar el puntero en la 
>pila, pero cuando queremos los datos reales, tenemos que seguir el puntero.
>
>Piensa en sentarte en un restaurante. Cuando ingresas, indicas el número de
>personas en tu grupo y el personal encuentra una mesa vacía que se ajuste a todos y 
>te lleve hasta allí. Si alguien en su grupo llega tarde, puede preguntar dónde te
>has sentado para encontrarte.
>
>Acceder a los datos en el montón es más lento que acceder a los datos de la pila porque
>tenemos que seguir un puntero para llegar allí. Los procesadores contemporáneos son más rápidos si 
>saltan menos en la memoria. Siguiendo la analogía, considera un servidor en un 
>restaurante tomando órdenes de muchas mesas. Es más eficiente obtener todas las 
>órdenes en una mesa antes de pasar a la siguiente. Tomar un pedido de
>la mesa A, luego un pedido de la mesa B, luego uno de A otra vez, y luego uno de B 
>otra vez sería un proceso mucho más lento. De la misma manera, un procesador puede hacer su 
>trabajo mejor si trabaja con datos que están cerca de otros datos (como lo está en la
>pila) y no más lejos (como puede estar en el montón). La asignación de una gran cantidad de 
>espacio en el montón también puede tomar tiempo.
>
>Cuando nuestro código llama a una función, los valores pasados a la función (incluyendo,
>potencialmente, punteros a los datos sobre el montón) y las variables locales de la función
>se empujan a la pila. Cuando la función termina, esos valores se 
>eliminan de la pila.
>
>Mantener un seguimiento de qué partes del código están usando qué datos en el monton, minimizando
>la cantidad de datos duplicados en el monton, y limpiando los datos no utilizados en el
>monton para que no nos quedemos sin espacio son todos los problemas que la propiedad trata.
>Una vez que entiendas la propiedad, no tendrás que pensar en la pila y 
>el montón muy a menudo, pero saber que la gestión de datos del monton es la razón por la que existe la propiedad
>puede ayudar a explicar por qué funciona de la forma en que funciona.
>
<!-- PROD: END BOX -->

### Reglas de Propiedad

Primero, echemos un vistazo a las reglas de propiedad. Ten en cuenta estas reglas mientras
trabajamos a través de los ejemplos que ilustran las reglas:

> 1. Cada valor en Rust tiene una variable que se llama *owner*.
> 2. Sólo puede haber un dueño a la vez.
> 3. Cuando el propietario salga del alcance, el valor será eliminado.

### Variable Scope

Hemos caminado a través de un ejemplo de un programa en Rust ya en el capítulo 2. Ahora 
que ya hemos superado la sintaxis básica, no incluiremos todo el código `fn main () {` 
en ejemplos, así que si estás siguiendo el ejemplo, tendrás que poner los siguientes 
ejemplos dentro de una función `main` manualmente. Como resultado, nuestros ejemplos serán 
un poco más concisos, permitiéndonos centrarnos en los detalles reales en lugar de los
códigos repetitivos.

Como primer ejemplo de propiedad, veremos el *scope* de algunas variables. Un 
scope es el rango dentro de un programa para el cual una posición es válida. Digamos que 
tenemos una variable que se parece a esto:

```rust
let s = "hello";
```

La variable `s` se refiere a una cadena literal, donde el valor de la cadena es
hardcodeada en el texto de nuestro programa. La variable es válida desde el momento
en que se declara hasta el final del actual *scope*. Listado 4-1 tiene
comentarios anotando donde la variable `s` es válida:

```rust
{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
}                      // this scope is now over, and s is no longer valid
```

<span class="caption">Listado 4-1: Una variable y el alcance en el que es 
    válida</span>

En otras palabras, hay dos puntos importantes en el tiempo:

1. Cuando `s` viene *into scopee*, es válido.
1. Permanece así hasta que sale *fuera of scope*.

En este punto, la relación entre scopes y cuando las variables son válidas es 
similar a la de otros lenguajes de programación. Ahora vamos a construir encima de este 
entendimiento introduciendo el tipo de `String`.

### El Tipo De `String`

Para ilustrar las reglas de propiedad, necesitamos un tipo de datos que sea más complejo 
que los que hemos tratado en el Capítulo 3. Todos los tipos de datos que hemos visto 
anteriormente se almacenan en la pila y salen de la pila cuando termina su 
alcance, pero queremos ver los datos que se almacenan en la pila y explorar cómo
Rust sabe cuándo limpiar esos datos.

Usaremos `String` como ejemplo aquí y nos concentraremos en las partes de `String`
que se relacionan con la propiedad. Estos aspectos también se aplican a otros tipos de datos complejos
proporcionados por la biblioteca estándar y que tu creas. Discutiremos `String` con
más profundidad en el Capítulo 8.

Ya hemos visto cadenas literales, donde el valor de la cadena está harcodeado en 
nuestro programa. Las cadenas literales son convenientes, pero no siempre son adecuadas para
todas las situaciones en las que se quiera utilizar texto. Una razón es que son 
inmutables. Otro es que no todos los valores de las cadenas pueden ser conocidos cuando escribimos
nuestro código: por ejemplo, ¿qué pasa si queremos tomar la entrada del usuario y guardarlo? Para
estas situaciones, Rust tiene un segundo tipo de cadena, `String`. Este tipo se 
asigna en el monton y como tal es capaz de almacenar una cantidad de texto que es
desconocido para nosotros en el tiempo de compilación. Puedes crear una `String` desde una cadena literal
usando la función `from`, así:

```rust
let s = String::from("hello");
```

El doble dos puntos (`::`) es un operador que nos permite ponerle un espacio de nombres 
a esta función particular `from` bajo el tipo `String` en lugar de usar algún tipo de
nombre como `string_from`. Discutiremos más esta sintaxis en la sección "Métodos
de Sintaxis" del Capítulo 5 y cuando hablemos de cómo se abre el espacio entre nombres con los módulos
en el Capítulo 7.

Este tipo de cadena *puede* ser mutada:

```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print `hello, world!`
```

Entonces, ¿cuál es la diferencia aquí? ¿Por qué `String` puede mutar pero los literales
no? La diferencia es cómo estos dos tipos tratan con la memoria.

### Memoria y Asignación

En el caso de una cadena literal, conocemos los contenidos en tiempo de compilación para que el
texto sea codificado directamente en el ejecutable final, haciendo que las cadenas literales
sean rápidas y eficientes. Pero estas propiedades sólo provienen de su inmutabilidad.
Desafortunadamente, no podemos poner una mancha de memoria en el binario para cada pieza de 
texto cuyo tamaño es desconocido al momento de compilar y cuyo tamaño podría cambiar mientras
que se ejecuta el programa.

Con el tipo `String`, para soportar un fragmento de texto mutable y cultivable, 
necesitamos asignar una cantidad de memoria en el monton, desconocida a la hora de compilar,
para retener el contenido. Esto significa:

1. La memoria debe solicitarse al sistema operativo en tiempo de ejecución.
2. Necesitamos una forma de devolver esta memoria al sistema operativo cuando
terminemos con nuestra `String`.

Esa primera parte la hacemos nosotros: cuando llamamos `String::from`, su implementación 
solicita la memoria que necesita. Esto es bastante universal en lenguajes de 
programación.

Sin embargo, la segunda parte es diferente. En los lenguajes con un *garbage collector
(GC)*, el GC mantiene un registro y limpia la memoria que ya no se usa, 
y nosotros, como programador, no necesitamos pensar en ello. Sin un GC, es el
programador el responsable de identificar cuando la memoria ya no está siendo utilizada y 
llamar al código para devolverla explícitamente, tal como lo hicimos para solicitarla. Hacer esto
correctamente ha sido históricamente un problema de programación difícil. Si lo olvidamos,
perderemos la memoria. Si lo hacemos demasiado pronto, tendremos una variable inválida. Si
lo hacemos dos veces, eso también es un error. Necesitamos emparejar exactamente una `allocate` con 
exactamente un `free`.

Rust toma un camino diferente: la memoria se devuelve automáticamente una vez que
la variable que posee se sale del scope. Aquí está una versión de nuestro ejemplo 
de scope de Listado 4-1 usando una `String` en lugar de una cadena literal:

```rust
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                                  // this scope is now over, and s is no
                                   // longer valid
```

Hay un punto natural en el que podemos devolver la memoria que nuestra `String` necesita
al sistema operativo: cuando la `s` se sale del scope. Cuando una variable se sale 
del scope, Rust nos llama a una función especial. Esta función se llama `drop`,
y es donde el autor de `String` puede poner el código para devolver la memoria.
Rust llama `drop` automáticamente al cierre `}`.

> Nota: En C++, este patrón de distribución de recursos al final de la vida de un artículo 
> a veces se llama *Resource Acquisition Is Initialization (RAII)*.
La función `drop` en Rust te resultará familiar si has utilizado patrones 
RAII.

Este patrón tiene un profundo impacto en la forma en que se escribe el código de Rust. Puede parecer
sencillo ahora mismo, pero el comportamiento del código puede ser inesperado en situaciones más
complicadas cuando queremos que múltiples variables utilicen los datos 
que hemos asignado en el montón. Exploremos algunas de esas situaciones.

#### Las Variables y los Datos Interactúan: Desplazar

Múltiples variables pueden interactuar con los mismos datos de diferentes maneras en Rust. 
Veamos un ejemplo usando un número entero en Listado 4-2:

```rust
let x = 5;
let y = x;
```

<span class="caption">Listado 4-2: Asignación del valor entero de la variable `x` 
a `y`.</span>

Probablemente podemos adivinar lo que esto está haciendo basándonos en nuestra experiencia con otros 
lenguajes: "Vincular el valor `5` a `x`; luego hacer una copia del valor en `x` y
atarlo a `y`". Ahora tenemos dos variables, `x` y `y`, y ambas iguales `5`.
Esto es precisamente lo que está ocurriendo porque los números enteros son valores simples con un 
tamaño fijo conocido, y estos dos valores `5` se empujan sobre la pila.

Ahora veamos la versión `String`:

```rust
let s1 = String::from("hello");
let s2 = s1;
```

Esto se parece mucho al código anterior, así que podríamos suponer que la forma 
en que funciona sería la misma: es decir, la segunda línea haría una copia del 
valor en `s1` y lo uniría a `s2`. Pero esto no es exactamente lo que pasa.

Para explicar esto más a fondo, veamos cómo es `String` bajo las 
cubiertas en la Figura 4-3. Una `String` se compone de tres partes, mostradas a la
izquierda: un puntero a la memoria que contiene el contenido de la cadena, una longitud
y una capacidad. Este grupo de datos se almacena en la pila. A la derecha está la
memoria del montón que contiene el contenido.

<img alt="String in memory" src="img/trpl04-01.svg" class="center" style="width: 50%;" />

<span class="caption">Figura 4-3: Representación en memoria de una `String` 
con el valor `"hello"`vinculado a `s1`.</span>

La longitud es cuánta memoria, en bytes, está utilizando el contenido
de la `String`. La capacidad es la cantidad total de memoria, en bytes, que la 
`String` ha recibido del sistema operativo. La diferencia entre la longitud
y la capacidad es importante, pero no en este contexto, así que por ahora está bien ignorar
la capacidad.

Cuando asignamos `s1` a `s2`, se copian los datos de `String`, lo que significa que copiamos el 
puntero, la longitud y la capacidad que hay en la pila. No copiamos los
datos del montón al que se refiere el puntero. En otras palabras, la representación de
datos en la memoria se parece a la Figura 4-4.

<img alt="s1 and s2 pointing to the same value" src="img/trpl04-02.svg" class="center" style="width: 50%;" />

<span class="caption">Figura 4-4: Representación en memoria de la variable `s2`
que tiene una copia del puntero, longitud y capacidad de `s1`.</span>

La representación *no* se parece a la Figura 4-5, que es cómo sería la memoria
si Rust copiara en su lugar también los datos del montón. Si Rust hizo esto, la 
operación `s2 = s1` podría ser potencialmente muy costosa en términos de rendimiento
en tiempo de ejecución si los datos del montón fueran grandes.

<img alt="s1 and s2 to two places" src="img/trpl04-03.svg" class="center" style="width: 50%;" />

<span class="caption">Figura 4-5: Otra posibilidad de lo que `s2 = s1` podría
hacer si Rust también copiara los datos del montón.</span>

Anteriormente, dijimos que cuando una variable se sale del alcance, Rust llama automáticamente 
a la función `drop` y limpia la memoria del montón para esa variable. Pero 
la Figura 4-4 muestra ambos indicadores de datos apuntando a la misma ubicación. Esto es un 
problema: cuando `s2` y `s1` se salen del alcance, ambos intentarán liberar la
misma memoria. Esto se conoce como un error *double free* y es uno de los errores de seguridad de memoria 
que mencionamos anteriormente. Liberar la memoria dos veces puede conducir a la corrupción 
de la memoria, lo que puede conducir potencialmente a vulnerabilidades de seguridad.

Para garantizar la seguridad de la memoria, hay un detalle más de lo que sucede en esta
situación en Rust. En vez de intentar copiar la memoria asignada, Rust 
considera que `s1` ya no es válido y por lo tanto, Rust no necesita liberar 
nada cuando `s1` se sale del alcance. Comprueba lo que sucede cuando intentas
usar `s1` después de crear `s2`:

```rust,ignore
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

You’ll get an error like this because Rust prevents you from using the
invalidated reference:

```text
error[E0382]: use of moved value: `s1`
 --> src/main.rs:4:27
  |
3 |     let s2 = s1;
  |         -- value moved here
4 |     println!("{}, world!", s1);
  |                            ^^ value used here after move
  |
  = note: move occurs because `s1` has type `std::string::String`,
which does not implement the `Copy` trait
```

If you’ve heard the terms “shallow copy” and “deep copy” while working with
other languages, the concept of copying the pointer, length, and capacity
without copying the data probably sounds like a shallow copy. But because Rust
also invalidates the first variable, instead of calling this a shallow copy,
it’s known as a *move*. Here we would read this by saying that `s1` was *moved*
into `s2`. So what actually happens is shown in Figure 4-6.

<img alt="s1 moved to s2" src="img/trpl04-04.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 4-6: Representation in memory after `s1` has been
invalidated</span>

That solves our problem! With only `s2` valid, when it goes out of scope, it
alone will free the memory, and we’re done.

In addition, there’s a design choice that’s implied by this: Rust will never
automatically create “deep” copies of your data. Therefore, any *automatic*
copying can be assumed to be inexpensive in terms of runtime performance.

#### Ways Variables and Data Interact: Clone

If we *do* want to deeply copy the heap data of the `String`, not just the
stack data, we can use a common method called `clone`. We’ll discuss method
syntax in Chapter 5, but because methods are a common feature in many
programming languages, you’ve probably seen them before.

Here’s an example of the `clone` method in action:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

This works just fine and is how you can explicitly produce the behavior shown
in Figure 4-5, where the heap data *does* get copied.

When you see a call to `clone`, you know that some arbitrary code is being
executed and that code may be expensive. It’s a visual indicator that something
different is going on.

#### Stack-Only Data: Copy

There’s another wrinkle we haven’t talked about yet. This code using integers,
part of which was shown earlier in Listing 4-2, works and is valid:

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

But this code seems to contradict what we just learned: we don’t have a call to
`clone`, but `x` is still valid and wasn’t moved into `y`.

The reason is that types like integers that have a known size at compile time
are stored entirely on the stack, so copies of the actual values are quick to
make. That means there’s no reason we would want to prevent `x` from being
valid after we create the variable `y`. In other words, there’s no difference
between deep and shallow copying here, so calling `clone` wouldn’t do anything
differently from the usual shallow copying and we can leave it out.

Rust has a special annotation called the `Copy` trait that we can place on
types like integers that are stored on the stack (we’ll talk more about traits
in Chapter 10). If a type has the `Copy` trait, an older variable is still
usable after assignment. Rust won’t let us annotate a type with the `Copy`
trait if the type, or any of its parts, has implemented the `Drop` trait. If
the type needs something special to happen when the value goes out of scope and
we add the `Copy` annotation to that type, we’ll get a compile time error. To
learn about how to add the `Copy` annotation to your type, see Appendix C on
Derivable Traits.

So what types are `Copy`? You can check the documentation for the given type to
be sure, but as a general rule, any group of simple scalar values can be
`Copy`, and nothing that requires allocation or is some form of resource is
`Copy`. Here are some of the types that are `Copy`:

* All the integer types, like `u32`.
* The boolean type, `bool`, with values `true` and `false`.
* All the floating point types, like `f64`.
* Tuples, but only if they contain types that are also `Copy`. `(i32, i32)` is
`Copy`, but `(i32, String)` is not.

### Ownership and Functions

The semantics for passing a value to a function are similar to assigning a
value to a variable. Passing a variable to a function will move or copy, just
like assignment. Listing 4-7 has an example with some annotations showing where
variables go into and out of scope:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope.

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here.
    let x = 5;                      // x comes into scope.

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward.

} // Here, x goes out of scope, then s. But since s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope.
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

<span class="caption">Listing 4-7: Functions with ownership and scope
annotated</span>

If we tried to use `s` after the call to `takes_ownership`, Rust would throw a
compile time error. These static checks protect us from mistakes. Try adding
code to `main` that uses `s` and `x` to see where you can use them and where
the ownership rules prevent you from doing so.

### Return Values and Scope

Returning values can also transfer ownership. Here’s an example with similar
annotations to those in Listing 4-7:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1.

    let s2 = String::from("hello");     // s2 comes into scope.

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3.
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it.

    let some_string = String::from("hello"); // some_string comes into scope.

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function.
}

// takes_and_gives_back will take a String and return one.
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope.

    a_string  // a_string is returned and moves out to the calling function.
}
```

The ownership of a variable follows the same pattern every time: assigning a
value to another variable moves it. When a variable that includes data on the
heap goes out of scope, the value will be cleaned up by `drop` unless the data
has been moved to be owned by another variable.

Taking ownership and then returning ownership with every function is a bit
tedious. What if we want to let a function use a value but not take ownership?
It’s quite annoying that anything we pass in also needs to be passed back if we
want to use it again, in addition to any data resulting from the body of the
function that we might want to return as well.

It’s possible to return multiple values using a tuple, like this:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String.

    (s, length)
}
```

But this is too much ceremony and a lot of work for a concept that should be
common. Luckily for us, Rust has a feature for this concept, and it’s called
*references*.
