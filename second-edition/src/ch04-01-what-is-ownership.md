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

<span class="caption">Listing 4-1: A variable and the scope in which it is
valid</span>

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

We’ve already seen string literals, where a string value is hardcoded into our
program. String literals are convenient, but they aren’t always suitable for
every situation in which you want to use text. One reason is that they’re
immutable. Another is that not every string value can be known when we write
our code: for example, what if we want to take user input and store it? For
these situations, Rust has a second string type, `String`. This type is
allocated on the heap and as such is able to store an amount of text that is
unknown to us at compile time. You can create a `String` from a string literal
using the `from` function, like so:

```rust
let s = String::from("hello");
```

The double colon (`::`) is an operator that allows us to namespace this
particular `from` function under the `String` type rather than using some sort
of name like `string_from`. We’ll discuss this syntax more in the “Method
Syntax” section of Chapter 5 and when we talk about namespacing with modules in
Chapter 7.

This kind of string *can* be mutated:

```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print `hello, world!`
```

So, what’s the difference here? Why can `String` be mutated but literals
cannot? The difference is how these two types deal with memory.

### Memory and Allocation

In the case of a string literal, we know the contents at compile time so the
text is hardcoded directly into the final executable, making string literals
fast and efficient. But these properties only come from its immutability.
Unfortunately, we can’t put a blob of memory into the binary for each piece of
text whose size is unknown at compile time and whose size might change while
running the program.

With the `String` type, in order to support a mutable, growable piece of text,
we need to allocate an amount of memory on the heap, unknown at compile time,
to hold the contents. This means:

1. The memory must be requested from the operating system at runtime.
2. We need a way of returning this memory to the operating system when we’re
done with our `String`.

That first part is done by us: when we call `String::from`, its implementation
requests the memory it needs. This is pretty much universal in programming
languages.

However, the second part is different. In languages with a *garbage collector
(GC)*, the GC keeps track and cleans up memory that isn’t being used anymore,
and we, as the programmer, don’t need to think about it. Without a GC, it’s the
programmer’s responsibility to identify when memory is no longer being used and
call code to explicitly return it, just as we did to request it. Doing this
correctly has historically been a difficult programming problem. If we forget,
we’ll waste memory. If we do it too early, we’ll have an invalid variable. If
we do it twice, that’s a bug too. We need to pair exactly one `allocate` with
exactly one `free`.

Rust takes a different path: the memory is automatically returned once the
variable that owns it goes out of scope. Here’s a version of our scope example
from Listing 4-1 using a `String` instead of a string literal:

```rust
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                                  // this scope is now over, and s is no
                                   // longer valid
```

There is a natural point at which we can return the memory our `String` needs
to the operating system: when `s` goes out of scope. When a variable goes out
of scope, Rust calls a special function for us. This function is called `drop`,
and it’s where the author of `String` can put the code to return the memory.
Rust calls `drop` automatically at the closing `}`.

> Note: In C++, this pattern of deallocating resources at the end of an item's
> lifetime is sometimes called *Resource Acquisition Is Initialization (RAII)*.
> The `drop` function in Rust will be familiar to you if you’ve used RAII
> patterns.

This pattern has a profound impact on the way Rust code is written. It may seem
simple right now, but the behavior of code can be unexpected in more
complicated situations when we want to have multiple variables use the data
we’ve allocated on the heap. Let’s explore some of those situations now.

#### Ways Variables and Data Interact: Move

Multiple variables can interact with the same data in different ways in Rust.
Let’s look at an example using an integer in Listing 4-2:

```rust
let x = 5;
let y = x;
```

<span class="caption">Listing 4-2: Assigning the integer value of variable `x`
to `y`</span>

We can probably guess what this is doing based on our experience with other
languages: “Bind the value `5` to `x`; then make a copy of the value in `x` and
bind it to `y`.” We now have two variables, `x` and `y`, and both equal `5`.
This is indeed what is happening because integers are simple values with a
known, fixed size, and these two `5` values are pushed onto the stack.

Now let’s look at the `String` version:

```rust
let s1 = String::from("hello");
let s2 = s1;
```

This looks very similar to the previous code, so we might assume that the way
it works would be the same: that is, the second line would make a copy of the
value in `s1` and bind it to `s2`. But this isn’t quite what happens.

To explain this more thoroughly, let’s look at what `String` looks like under
the covers in Figure 4-3. A `String` is made up of three parts, shown on the
left: a pointer to the memory that holds the contents of the string, a length,
and a capacity. This group of data is stored on the stack. On the right is the
memory on the heap that holds the contents.

<img alt="String in memory" src="img/trpl04-01.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 4-3: Representation in memory of a `String`
holding the value `"hello"` bound to `s1`</span>

The length is how much memory, in bytes, the contents of the `String` is
currently using. The capacity is the total amount of memory, in bytes, that the
`String` has received from the operating system. The difference between length
and capacity matters, but not in this context, so for now, it’s fine to ignore
the capacity.

When we assign `s1` to `s2`, the `String` data is copied, meaning we copy the
pointer, the length, and the capacity that are on the stack. We do not copy the
data on the heap that the pointer refers to. In other words, the data
representation in memory looks like Figure 4-4.

<img alt="s1 and s2 pointing to the same value" src="img/trpl04-02.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 4-4: Representation in memory of the variable `s2`
that has a copy of the pointer, length, and capacity of `s1`</span>

The representation does *not* look like Figure 4-5, which is what memory would
look like if Rust instead copied the heap data as well. If Rust did this, the
operation `s2 = s1` could potentially be very expensive in terms of runtime
performance if the data on the heap was large.

<img alt="s1 and s2 to two places" src="img/trpl04-03.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 4-5: Another possibility of what `s2 = s1` might
do if Rust copied the heap data as well</span>

Earlier, we said that when a variable goes out of scope, Rust automatically
calls the `drop` function and cleans up the heap memory for that variable. But
Figure 4-4 shows both data pointers pointing to the same location. This is a
problem: when `s2` and `s1` go out of scope, they will both try to free the
same memory. This is known as a *double free* error and is one of the memory
safety bugs we mentioned previously. Freeing memory twice can lead to memory
corruption, which can potentially lead to security vulnerabilities.

To ensure memory safety, there’s one more detail to what happens in this
situation in Rust. Instead of trying to copy the allocated memory, Rust
considers `s1` to no longer be valid and therefore, Rust doesn’t need to free
anything when `s1` goes out of scope. Check out what happens when you try to
use `s1` after `s2` is created:

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
