## What Is Ownership?

Rust’s central feature is *ownership*. Although the feature is straightforward
to explain, it has deep implications for the rest of the language.

All programs have to manage the way they use a computer’s memory while running.
Some languages have garbage collection that constantly looks for no longer used
memory as the program runs; in other languages, the programmer must explicitly
allocate and free the memory. Rust uses a third approach: memory is managed
through a system of ownership with a set of rules that the compiler checks at
compile time. No run-time costs are incurred for any of the ownership features.

Because ownership is a new concept for many programmers, it does take some time
to get used to. The good news is that the more experienced you become with Rust
and the rules of the ownership system, the more you’ll be able to naturally
develop code that is safe and efficient. Keep at it!

When you understand ownership, you’ll have a solid foundation for understanding
the features that make Rust unique. In this chapter, you’ll learn ownership by
working through some examples that focus on a very common data structure:
strings.

<!-- PROD: START BOX -->

> ### The Stack and the Heap
>
> In many programming languages, we don’t have to think about the stack and the
> heap very often. But in a systems programming language like Rust, whether a
> value is on the stack or the heap has more of an effect on how the language
> behaves and why we have to make certain decisions. We’ll describe parts of
> ownership in relation to the stack and the heap later in this chapter, so here
> is a brief explanation in preparation.
>
> Both the stack and the heap are parts of memory that is available to your code
> to use at runtime, but they are structured in different ways. The stack stores
> values in the order it gets them and removes the values in the opposite order.
> This is referred to as *last in, first out*. Think of a stack of plates: when
> you add more plates, you put them on top of the pile, and when you need a
> plate, you take one off the top. Adding or removing plates from the middle or
> bottom wouldn’t work as well! Adding data is called *pushing onto the stack*,
> and removing data is called *popping off the stack*.
>
> The stack is fast because of the way it accesses the data: it never has to
> search for a place to put new data or a place to get data from because that
> place is always the top. Another property that makes the stack fast is that all
> data on the stack must take up a known, fixed size.
>
> For data with a size unknown to us at compile time or a size that might change,
> we can store data on the heap instead. The heap is less organized: when we put
> data on the heap, we ask for some amount of space. The operating system finds
> an empty spot somewhere in the heap that is big enough, marks it as being in
> use, and returns to us a pointer to that location. This process is called
> *allocating on the heap*, and sometimes we abbreviate the phrase as just
> “allocating.” Pushing values onto the stack is not considered allocating.
> Because the pointer is a known, fixed size, we can store the pointer on the
> stack, but when we want the actual data, we have to follow the pointer.
>
> Think of being seated at a restaurant. When you enter, you state the number of
> people in your group, and the staff finds an empty table that fits everyone and
> leads you there. If someone in your group comes late, they can ask where you’ve
> been seated to find you.
>
> Accessing data in the heap is slower than accessing data on the stack because
> we have to follow a pointer to get there. Contemporary processors are faster if
> they jump around less in memory. Continuing the analogy, consider a server at a
> restaurant taking orders from many tables. It’s most efficient to get all the
> orders at one table before moving on to the next table. Taking an order from
> table A, then an order from table B, then one from A again, and then one from B
> again would be a much slower process. By the same token, a processor can do its
> job better if it works on data that’s close to other data (as it is on the
> stack) rather than farther away (as it can be on the heap). Allocating a large
> amount of space on the heap can also take time.
>
> When our code calls a function, the values passed into the function (including,
> potentially, pointers to data on the heap) and the function’s local variables
> get pushed onto the stack. When the function is over, those values get popped
> off the stack.
>
> Keeping track of what parts of code are using what data on the heap, minimizing
> the amount of duplicate data on the heap, and cleaning up unused data on the
> heap so we don’t run out of space are all problems that ownership addresses.
> Once you understand ownership, you won’t need to think about the stack and the
> heap very often, but knowing that managing heap data is why ownership exists
> can help explain why it works the way it does.
>
<!-- PROD: END BOX -->

### Ownership Rules

First, let’s take a look at the ownership rules. Keep these rules in mind as we
work through the examples that illustrate the rules:

> 1. Each value in Rust has a variable that’s called its *owner*.
> 2. There can only be one owner at a time.
> 3. When the owner goes out of scope, the value will be dropped.

### Variable Scope

We’ve walked through an example of a Rust program already in Chapter 2. Now
that we’re past basic syntax, we won’t include all the `fn main() {` code in
examples, so if you’re following along, you’ll have to put the following
examples inside a `main` function manually. As a result, our examples will be a
bit more concise, letting us focus on the actual details rather than
boilerplate code.

As a first example of ownership, we’ll look at the *scope* of some variables. A
scope is the range within a program for which an item is valid. Let’s say we
have a variable that looks like this:

```rust
let s = "hello";
```

The variable `s` refers to a string literal, where the value of the string is
hardcoded into the text of our program. The variable is valid from the point at
which it’s declared until the end of the current *scope*. Listing 4-1 has
comments annotating where the variable `s` is valid:

```rust
{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
}                      // this scope is now over, and s is no longer valid
```

<span class="caption">Listado 4-1: Una variable y el alcance en el que es 
    válida</span>

In other words, there are two important points in time here:

1. When `s` comes *into scope*, it is valid.
1. It remains so until it goes *out of scope*.

At this point, the relationship between scopes and when variables are valid is
similar to other programming languages. Now we’ll build on top of this
understanding by introducing the `String` type.

### The `String` Type

To illustrate the rules of ownership, we need a data type that is more complex
than the ones we covered in Chapter 3. All the data types we’ve looked at
previously are stored on the stack and popped off the stack when their scope is
over, but we want to look at data that is stored on the heap and explore how
Rust knows when to clean up that data.

We’ll use `String` as the example here and concentrate on the parts of `String`
that relate to ownership. These aspects also apply to other complex data types
provided by the standard library and that you create. We’ll discuss `String` in
more depth in Chapter 8.

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
2. Necesitamos una forma de devolver esta memoria al sistema operativo cuando terminemos con nuestra `String`.

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
