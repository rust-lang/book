## Tipos de datos

Cada valor en Rust es de un cierto *tipo*, que le dice a Rust qué tipo de datos 
se está especificando para que sepa cómo trabajar con esos datos. En esta sección,
veremos una serie de tipos que se construyen en el lenguaje. Dividimos los
tipos en dos subconjuntos: escalar y compuesto.

A lo largo de esta sección, ten en cuenta que Rust es un lenguaje *estáticamente mecanografiado*,
lo que significa que debes conocer los tipos de todas las variables en tiempo de
compilación. El compilador generalmente puede inferir qué tipo de información queremos usar en base al 
valor y cómo la usamos. En los casos en que muchos tipos son posibles, como cuando 
convertimos una `String` a un tipo numérico usando `parse` en el Capítulo 2, debemos añadir
una anotación de tipo, como esta:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

Si no añadimos el tipo de anotación aquí, Rust mostrará el siguiente
error, lo que significa que el compilador necesita más información de nosotros para saber 
qué tipo posible queremos usar:

```text
error[E0282]: unable to infer enough type information about `_`
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^ cannot infer type for `_`
  |
  = note: type annotations or generic parameter binding required
```

Verás diferentes tipos de anotaciones a medida que discutimos los distintos tipos de datos.

### Tipos Escalares

Un tipo *escalar* representa un solo valor. Rust tiene cuatro tipos escalares principales:
enteros, números de coma flotante, booleanos y caracteres. Probablemente los 
reconocerás de otros lenguajes de programación, pero vamos a saltar a cómo
funcionan en Rust.

#### tipo Entero

Un *integer* es un número sin un componente fraccionario. Usamos un tipo 
entero antes en este capítulo, el tipo `i32`. Esta tipo de declaración indica 
que el valor con el que está asociado debe ser un entero firmado (de ahí el `i`, 
a diferencia de una `u` para no firmado) que ocupa 32 bits de espacio. La Tabla 3-1 
muestra los tipos enteros incorporados en Rust. Cada variante de las columnas 
Firmada y No Firmada (por ejemplo, *i32*) puede utilizarse para declarar el tipo de 
un valor entero.

<span class="caption">Tabla 3-1: Tipos enteros en Rust</span>

| Longitud | Firmada | No firmada |
|----------|---------|------------|
| 8-bit    | i8      | u8         |
| 16-bit   | i16     | u16        |
| 32-bit   | i32     | u32        |
| 64-bit   | i64     | u64        |
| arch     | isize   | usize      |

Cada variante puede estar firmada o sin firmar y tiene un tamaño explícito. 
Firmada y no firmada se refiere a si es posible que el número sea 
negativo o positivo; en otras palabras, si el número necesita tener un signo
con él (firmada) o si sólo será positivo y por lo tanto puede
ser representado sin un signo (no firmada). Es como escribir números en papel: cuando
el signo importa, un número se muestra con un signo más o un signo menos; sin embargo,
cuando es seguro suponer que el número es positivo, se muestra sin signo.
Los números firmados se almacenan usando la representación del complemento de dos (si no estás 
seguro de lo que es esto, puedes buscarlo en línea; una explicación está fuera
del alcance de este libro).

Cada variante firmada puede almacenar números de - (2<sup>n - 1</sup>) a 2<sup>n - 
1</sup> - 1 inclusive, donde `n` es el número de bits que usa la variante. Por lo tanto,
un `i8` puede almacenar números de - (2<sup>7</sup>) a 2<sup>7</sup> - 1, lo que equivale 
a -128 a 127. Las variantes no firmadas pueden almacenar números de 0 a 2<sup>n</sup> - 1, 
por lo que un `u8` puede almacenar números de 0 a 2<sup>8</sup> - 1, lo que equivale a 0 a 255.
 
 Además, los tipos `isize` y `usize` dependen del tipo de equipo en el que estes
 ejecutando el programa: 64 bits si está en una arquitectura de 64 bits y 32 bits
 si está en una arquitectura de 32 bits.
 
Puede escribir enteros literales en cualquiera de las formas mostradas en la Tabla 3-2. Nota
que todos los números literales excepto el byte literal permiten un tipo de sufijo, tal como
`57u8`, y `_` como un separador visual, así como `1_000`.

<span class="caption">Tabla 3-2: Literales enteros en Rust</span>

| Números Literales  | Ejemplo       |
|--------------------|---------------|
| Decimal            | `98_222`      |
| Hex                | `0xff`        |
| Octal              | `0o77`        |
| Binario            | `0b1111_0000` |
| Byte (solo `u8` )  | `b'A'`        |

Entonces, ¿cómo sabes qué tipo de entero usar? Si no estás seguro, los valores
predeterminados de Rust son generalmente buenas opciones, y los tipos enteros por defecto son `i32`: 
generalmente es el más rápido, incluso en sistemas de 64 bits. La situación principal en la que
se utiliza `isize` o `usize` es cuando se indiza algún tipo de colección.

#### Tipos de Punto Flotante

Rust también tiene dos tipos primitivos para *floating-point numbers*, que son 
números con decimales. Los tipos de punto flotante de Rust son `f32` y `f64`, 
que tienen 32 bits y 64 bits de tamaño, respectivamente. El tipo por defecto es `f64`
porque es más o menos la misma velocidad que `f32` pero es capaz de mayor precisión.
Es posible usar un tipo `f64` en sistemas de 32 bits, pero será más lento 
que usar un tipo `f32` en esos sistemas. La mayor parte del tiempo, el potencial de 
negociación bajar el rendimiento para una mayor precisión es una elección inicial razonable, y tu 
debes poner a prueba tu código si sospechas que el tamaño del punto flotante es un problema en
tu situación.

Aquí hay un ejemplo que muestra números de punto flotante en acción:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

Los números de punto flotante se representan según la norma IEEE-754. El 
tipo `f32` es un flotador de precisión simple, y el `f64` tiene doble precisión.

#### Operaciones Numéricas

Rust soporta las operaciones matemáticas básicas usuales que tu esperarías para todos los
tipos de números: suma, resta, multiplicación, división y porcentaje. 
El siguiente código muestra cómo se usa cada uno de ellos en una declaración `let`:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    // Suma
    let sum = 5 + 10;

    // Resta
    let difference = 95.5 - 4.3;

    // Multiplicación
    let product = 4 * 30;

    // División
    let quotient = 56.7 / 32.2;

    // Porcentaje
    let remainder = 43 % 5;
}
```

Cada expresión de estas declaraciones utiliza un operador matemático y evalúa 
a un único valor, que luego se vincula a una variable. El Apéndice B contiene una
lista de todos los operadores que Rust proporciona.

#### El tipo Booleano

Como en la mayoría de los otros lenguajes de programación, un tipo booleano en Rust tiene dos valores
posibles: `true` y `false`. El tipo booleano en Rust se especifica con `bool`. 
Por ejemplo:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let t = true;

    let f: bool = false; // con anotación de tipo explícita
}
```

La forma principal de consumir valores booleanos es a través de condicionales, como una expresión
`if`. Cubriremos cómo funcionan las expresiones `if` en Rust en la sección 
"Flujo de Control".

#### The Character Type

So far we’ve only worked with numbers, but Rust supports letters too. Rust’s
`char` type is the language’s most primitive alphabetic type, and the following
code shows one way to use it:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
   let c = 'z';
   let z = 'ℤ';
   let heart_eyed_cat = '😻';
}
```

Rust’s `char` type represents a Unicode Scalar Value, which means it can
represent a lot more than just ASCII. Accented letters, Chinese/Japanese/Korean
ideographs, emoji, and zero width spaces are all valid `char` types in Rust.
Unicode Scalar Values range from `U+0000` to `U+D7FF` and `U+E000` to
`U+10FFFF` inclusive. However, a “character” isn’t really a concept in Unicode,
so your human intuition for what a “character” is may not match up with what a
`char` is in Rust. We’ll discuss this topic in detail in the “Strings” section
in Chapter 8.

### Compound Types

*Compound types* can group multiple values of other types into one type. Rust
has two primitive compound types: tuples and arrays.

#### Grouping Values into Tuples

A tuple is a general way of grouping together some number of other values with
a variety of types into one compound type.

We create a tuple by writing a comma-separated list of values inside
parentheses. Each position in the tuple has a type, and the types of the
different values in the tuple don’t have to be the same. We’ve added optional
type annotations in this example:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

The variable `tup` binds to the entire tuple, since a tuple is considered a
single compound element. To get the individual values out of a tuple, we can
use pattern matching to destructure a tuple value, like this:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

This program first creates a tuple and binds it to the variable `tup`. It then
uses a pattern with `let` to take `tup` and turn it into three separate
variables, `x`, `y`, and `z`. This is called *destructuring*, because it breaks
the single tuple into three parts. Finally, the program prints the value of
`y`, which is `6.4`.

In addition to destructuring through pattern matching, we can also access a
tuple element directly by using a period (`.`) followed by the index of the
value we want to access. For example:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

This program creates a tuple, `x`, and then makes new variables for each
element by using their index. As with most programming languages, the first
index in a tuple is 0.

#### Arrays

Another way to have a collection of multiple values is with an *array*. Unlike
a tuple, every element of an array must have the same type. Arrays in Rust are
different than arrays in some other languages because arrays in Rust have a
fixed length: once declared, they cannot grow or shrink in size.

In Rust, the values going into an array are written as a comma-separated list
inside square brackets:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

Arrays are useful when you want your data allocated on the stack rather than
the heap (we will discuss the stack and the heap more in Chapter 4), or when
you want to ensure you always have a fixed number of elements. They aren’t as
flexible as the vector type, though. The vector type is a similar collection
type provided by the standard library that *is* allowed to grow or shrink in
size. If you’re unsure whether to use an array or a vector, you should probably
use a vector: Chapter 8 discusses vectors in more detail.

An example of when you might want to use an array rather than a vector is in a
program that needs to know the names of the months of the year. It’s very
unlikely that such a program will need to add or remove months, so you can use
an array because you know it will always contain 12 items:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

##### Accessing Array Elements

An array is a single chunk of memory allocated on the stack. We can access
elements of an array using indexing, like this:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

In this example, the variable named `first` will get the value `1`, because
that is the value at index `[0]` in the array. The variable named `second` will
get the value `2` from index `[1]` in the array.

##### Invalid Array Element Access

What happens if we try to access an element of an array that is past the end of
the array? Say we change the example to the following:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}
```

Running this code using `cargo run` produces the following result:

```text
$ cargo run
   Compiling arrays v0.1.0 (file:///projects/arrays)
     Running `target/debug/arrays`
thread '<main>' panicked at 'index out of bounds: the len is 5 but the index is
 10', src/main.rs:6
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

The compilation didn’t produce any errors, but the program results in a
*runtime* error and didn’t exit successfully. When you attempt to access an
element using indexing, Rust will check that the index you’ve specified is less
than the array length. If the index is greater than the length, Rust will
*panic*, which is the term Rust uses when a program exits with an error.

This is the first example of Rust’s safety principles in action. In many
low-level languages, this kind of check is not done, and when you provide an
incorrect index, invalid memory can be accessed. Rust protects you against this
kind of error by immediately exiting instead of allowing the memory access and
continuing. Chapter 9 discusses more of Rust’s error handling.
