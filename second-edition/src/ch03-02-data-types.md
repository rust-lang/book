## Data Types

Every value in Rust is of a certain *type*, which tells Rust what kind of data
is being specified so it knows how to work with that data. In this section,
we’ll look at a number of types that are built into the language. We split the
types into two subsets: scalar and compound.

Throughout this section, keep in mind that Rust is a *statically typed*
language, which means that it must know the types of all variables at compile
time. The compiler can usually infer what type we want to use based on the
value and how we use it. In cases when many types are possible, such as when we
converted a `String` to a numeric type using `parse` in Chapter 2, we must add
a type annotation, like this:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

If we don’t add the type annotation here, Rust will display the following
error, which means the compiler needs more information from us to know which
possible type we want to use:

```text
error[E0282]: unable to infer enough type information about `_`
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^ cannot infer type for `_`
  |
  = note: type annotations or generic parameter binding required
```

You’ll see different type annotations as we discuss the various data types.

### Scalar Types

A *scalar* type represents a single value. Rust has four primary scalar types:
integers, floating-point numbers, booleans, and characters. You’ll likely
recognize these from other programming languages, but let’s jump into how they
work in Rust.

#### Integer Types

An *integer* is a number without a fractional component. We used one integer
type earlier in this chapter, the `i32` type. This type declaration indicates
that the value it’s associated with should be a signed integer (hence the `i`,
as opposed to a `u` for unsigned) that takes up 32 bits of space. Table 3-1
shows the built-in integer types in Rust. Each variant in the Signed and
Unsigned columns (for example, *i32*) can be used to declare the type of an
integer value.

<span class="caption">Table 3-1: Integer Types in Rust</span>

| Length | Signed | Unsigned |
|--------|--------|----------|
| 8-bit  | i8     | u8       |
| 16-bit | i16    | u16      |
| 32-bit | i32    | u32      |
| 64-bit | i64    | u64      |
| arch   | isize  | usize    |

Each variant can be either signed or unsigned and has an explicit size.
Signed and unsigned refers to whether it’s possible for the number to be
negative or positive; in other words, whether the number needs to have a sign
with it (signed) or whether it will only ever be positive and can therefore be
represented without a sign (unsigned). It’s like writing numbers on paper: when
the sign matters, a number is shown with a plus sign or a minus sign; however,
when it’s safe to assume the number is positive, it’s shown with no sign.
Signed numbers are stored using two’s complement representation (if you’re
unsure what this is, you can search for it online; an explanation is outside
the scope of this book).

Each signed variant can store numbers from -(2<sup>n - 1</sup>) to 2<sup>n -
1</sup> - 1 inclusive, where `n` is the number of bits that variant uses. So an
`i8` can store numbers from -(2<sup>7</sup>) to 2<sup>7</sup> - 1, which equals
-128 to 127. Unsigned variants can store numbers from 0 to 2<sup>n</sup> - 1,
so a `u8` can store numbers from 0 to 2<sup>8</sup> - 1, which equals 0 to 255.

Additionally, the `isize` and `usize` types depend on the kind of computer your
program is running on: 64-bits if you’re on a 64-bit architecture and 32-bits
if you’re on a 32-bit architecture.

You can write integer literals in any of the forms shown in Table 3-2. Note
that all number literals except the byte literal allow a type suffix, such as
`57u8`, and `_` as a visual separator, such as `1_000`.

<span class="caption">Table 3-2: Integer Literals in Rust</span>

| Number literals  | Example       |
|------------------|---------------|
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` only) | `b'A'`        |

So how do you know which type of integer to use? If you’re unsure, Rust’s
defaults are generally good choices, and integer types default to `i32`: it’s
generally the fastest, even on 64-bit systems. The primary situation in which
you’d use `isize` or `usize` is when indexing some sort of collection.

#### Floating-Point Types

Rust also has two primitive types for *floating-point numbers*, which are
numbers with decimal points. Rust’s floating-point types are `f32` and `f64`,
which are 32 bits and 64 bits in size, respectively. The default type is `f64`
because it’s roughly the same speed as `f32` but is capable of more precision.
It’s possible to use an `f64` type on 32-bit systems, but it will be slower
than using an `f32` type on those systems. Most of the time, trading potential
worse performance for better precision is a reasonable initial choice, and you
should benchmark your code if you suspect floating-point size is a problem in
your situation.

Here’s an example that shows floating-point numbers in action:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

Floating-point numbers are represented according to the IEEE-754 standard. The
`f32` type is a single-precision float, and `f64` has double precision.

#### Numeric Operations

Rust supports the usual basic mathematical operations you’d expect for all of the
number types: addition, subtraction, multiplication, division, and remainder.
The following code shows how you’d use each one in a `let` statement:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
}
```

Each expression in these statements uses a mathematical operator and evaluates
to a single value, which is then bound to a variable. Appendix B contains a
list of all operators that Rust provides.

#### The Boolean Type

As in most other programming languages, a boolean type in Rust has two possible
values: `true` and `false`. The boolean type in Rust is specified using `bool`.
For example:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

The main way to consume boolean values is through conditionals, such as an `if`
expression. We’ll cover how `if` expressions work in Rust in the “Control Flow”
section.

#### El Tipo de Carácter

Hasta ahora sólo hemos trabajado con números, pero Rust también soporta letras. El tipo de 
`char` de Rust es el tipo alfabético más primitivo del lenguaje, y el siguiente
código muestra una forma de usarlo:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
   let c = 'z';
   let z = 'ℤ';
   let heart_eyed_cat = '😻';
}
```

El tipo `char` de Rust representa un valor escalar Unicode, lo que significa que puede
representar mucho más que ASCII. Letras acentuadas, ideogramas 
chinos/japoneses/coreanos, emojis y espacios de anchura cero son todos tipos válidos de `char` en Rust.
Los valores escalares Unicode van desde `U+000000` hasta `U+D7FF` e incluso de `U+E000` hasta
`U+10FFFF`. Sin embargo, un "caracter" no es realmente un concepto en Unicode, 
así que tu intuición humana para lo que es un "caracter" puede que no coincida con lo que 
es un `char` en Rust. Discutiremos este tema en detalle en la sección "Cadenas" 
en el Capítulo 8.

### Tipos de Compuestos

Los *tipos de compuestos* pueden agrupar múltiples valores de otros tipos a un tipo. Rust
tiene dos tipos de compuestos primitivos: tuplas y matrices.

#### Clasificación de Valores en Tuplas

Una tupla es una forma general de agrupar un cierto número de otros valores con 
una variedad de tipos en un tipo compuesto.

Creamos una tupla escribiendo una lista de valores separados por comas dentro
de paréntesis. Cada posición en la tupla tiene un tipo, y los tipos de los
diferentes valores en la tupla no tienen que ser los mismos. Hemos añadido anotaciones
de tipo opcionales en este ejemplo:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

La variable `tup` se une a la tupla entera, ya que una tupla se considera un
solo elemento compuesto. Para obtener los valores individuales de una tupla, podemos
usar la coincidencia de patrones para desestructurar el valor de una tupla, como este:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

Este programa crea primero una tupla y lo une a la variable `tup`. Luego
usa un patrón con `lets` para tomar `tup` y convertirlo en tres variables
separadas, `x`, `y` y `z`. Esto se llama *desestructuración*, porque rompe 
la tupla simple en tres partes. Finalmente, el programa imprime el valor de
`y`, que es `6.4`.

Además de la desestructuración por coincidencia de patrones, también podemos acceder 
directamente a un elemento de tupla utilizando un periodo (`.`) seguido del índice del
valor al que queremos acceder. Por ejemplo:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

Este programa crea una tupla, `x`, y luego crea nuevas variables para cada
elemento usando su índice. Como en la mayoría de los lenguajes de programación, el primer 
índice en una tupla es 0.

#### Arrays (Matrices)

Otra forma de tener una colección de múltiples valores es con una *array*. A diferencia
de una tupla, cada elemento de un array debe tener el mismo tipo. Las arrays en Rust son 
diferentes a las arrays en otros lenguajes porque las arrays en Rust tienen una 
longitud fija: una vez declaradas, no pueden crecer o reducirse de tamaño.

En Rust, los valores que entran en una array se escriben como una lista separada por comas
dentro de corchetes:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

Las matrices son útiles cuando deseas que tus datos se asignen a la pila en lugar de
al montón (hablaremos más sobre la pila y el montón en el Capítulo 4), o cuando 
quieres asegurarte de que siempre tienes un número fijo de elementos. No son tan
flexibles como el tipo vector. El tipo vector es un tipo de colección similar
proporcionado por la biblioteca estándar que *se* permite que crezca o se reduzca 
de tamaño. Si no estás seguro de si debes usar una array o un vector, debes probablemente
usar un vector: el Capítulo 8 discute los vectores con más detalle.

Un ejemplo de cuándo es posible que desees utilizar una matriz en lugar de un vector es en
un programa que necesita saber los nombres de los meses del año. Es muy poco
probable que un programa de este tipo necesite añadir o eliminar meses, por lo que puedes utilizar
una array porque sabes que siempre contendrá 12 elementos:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

##### Acceso a Los Elementos de la Array

Una array es un único trozo de memoria asignado en la pila. Podemos acceder 
a elementos de una array usando indexación, así:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

En este ejemplo, la variable llamada `first` obtendrá el valor `1`, porque
ese es el valor en el índice `[0]` de la array. La variable llamada `second` obtendrá 
el valor `2` del índice `[1]` en la array.

##### Acceso a Elementos de Array no Válidos

¿Qué sucede si intentamos acceder a un elemento de una array que está más allá del final
de la array? Digamos que cambiamos el ejemplo a lo siguiente:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}
```

Ejecutar este código usando `Cargo Run` produce el siguiente resultado:

```text
$ cargo run
   Compiling arrays v0.1.0 (file:///projects/arrays)
     Running `target/debug/arrays`
thread '<main>' panicked at 'index out of bounds: the len is 5 but the index is
 10', src/main.rs:6
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

La compilación no produjo ningún error, pero el programa da como resultado un 
error de *runtime* y no salió con éxito. Cuando intentes acceder a un 
elemento utilizando la indexación, Rust comprobará que el índice especificado es inferior
a la longitud de la array. Si el índice es mayor que la longitud, Rust entrará en 
*panic*, que es el término que usa Rust cuando un programa sale con un error.

Este es el primer ejemplo de los principios de seguridad de Rust en acción. En muchos
lenguajes de bajo nivel, este tipo de comprobación no se realiza y cuando proporcionas un 
índice incorrecto, se puede acceder a la memoria no válida. Rust te protege contra este 
tipo de error al salir inmediatamente en lugar de permitir el acceso a la memoria y
continuar. El Capítulo 9 trata más sobre el manejo de los errores de Rust.
