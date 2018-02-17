## Control Flow

Deciding whether or not to run some code depending on if a condition is true or
deciding to run some code repeatedly while a condition is true are basic
building blocks in most programming languages. The most common constructs that
let you control the flow of execution of Rust code are `if` expressions and
loops.

### `if` Expressions

An `if` expression allows us to branch our code depending on conditions. We
provide a condition and then state, “If this condition is met, run this block
of code. If the condition is not met, do not run this block of code.”

Create a new project called *branches* in your *projects* directory to explore
the `if` expression. In the *src/main.rs* file, input the following:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

<!-- NEXT PARAGRAPH WRAPPED WEIRD INTENTIONALLY SEE #199 -->

All `if` expressions start with the keyword `if`, which is followed by a
condition. In this case, the condition checks whether or not the variable
`number` has a value less than 5. The block of code we want to execute if the
condition is true is placed immediately after the condition inside curly
brackets. Blocks of code associated with the conditions in `if` expressions are
sometimes called *arms*, just like the arms in `match` expressions that we
discussed in the “Comparing the Guess to the Secret Number” section of
Chapter 2. Optionally, we can also include an `else` expression, which we chose
to do here, to give the program an alternative block of code to execute should
the condition evaluate to false. If you don’t provide an `else` expression and
the condition is false, the program will just skip the `if` block and move on
to the next bit of code.

Try running this code; you should see the following output:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/branches`
condition was true
```

Let’s try changing the value of `number` to a value that makes the condition
`false` to see what happens:

```rust,ignore
let number = 7;
```

Run the program again, and look at the output:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/branches`
condition was false
```

It’s also worth noting that the condition in this code *must* be a `bool`. If
the condition isn’t a `bool`, we’ll get an error. For example, try running the
following code:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```

The `if` condition evaluates to a value of `3` this time, and Rust throws an
error:

```text
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected bool, found integral variable
  |
  = note: expected type `bool`
             found type `{integer}`
```

The error indicates that Rust expected a `bool` but got an integer. Rust will
not automatically try to convert non-Boolean types to a Boolean, unlike
languages such as Ruby and JavaScript. You must be explicit and always provide
`if` with a Boolean as its condition. If we want the `if` code block to run
only when a number is not equal to `0`, for example, we can change the `if`
expression to the following:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}
```

Running this code will print `number was something other than zero`.

#### Multiple Conditions with `else if`

We can have multiple conditions by combining `if` and `else` in an `else if`
expression. For example:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

This program has four possible paths it can take. After running it, you should
see the following output:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/branches`
number is divisible by 3
```

When this program executes, it checks each `if` expression in turn and executes
the first body for which the condition holds true. Note that even though 6 is
divisible by 2, we don’t see the output `number is divisible by 2`, nor do we
see the `number is not divisible by 4, 3, or 2` text from the `else` block. The
reason is that Rust will only execute the block for the first true condition,
and once it finds one, it won’t even check the rest.

Using too many `else if` expressions can clutter your code, so if you have more
than one, you might want to refactor your code. Chapter 6 describes a powerful
Rust branching construct called `match` for these cases.

#### Using `if` in a `let` statement

Because `if` is an expression, we can use it on the right side of a `let`
statement, for instance in Listing 3-2:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```

<span class="caption">Listing 3-2: Assigning the result of an `if` expression
to a variable</span>

The `number` variable will be bound to a value based on the outcome of the `if`
expression. Run this code to see what happens:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30 secs
     Running `target/debug/branches`
The value of number is: 5
```

Remember that blocks of code evaluate to the last expression in them, and
numbers by themselves are also expressions. In this case, the value of the
whole `if` expression depends on which block of code executes. This means the
values that have the potential to be results from each arm of the `if` must be
the same type; in Listing 3-2, the results of both the `if` arm and the `else`
arm were `i32` integers. If the types are mismatched, as in the following
example, we’ll get an error:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let condition = true;

    let number = if condition {
        5
    } else {
        "six"
    };

    println!("The value of number is: {}", number);
}
```

When we try to run this code, we’ll get an error. The `if` and `else` arms have
value types that are incompatible, and Rust indicates exactly where to find the
problem in the program:

```text
error[E0308]: if and else have incompatible types
 --> src/main.rs:4:18
  |
4 |       let number = if condition {
  |  __________________^
5 | |         5
6 | |     } else {
7 | |         "six"
8 | |     };
  | |_____^ expected integral variable, found reference
  |
  = note: expected type `{integer}`
             found type `&str`
```

La expresión en el bloque `if` evalúa a un entero, y la expresión en
el bloque `else` evalúa a una cadena. Esto no funcionará porque las variables deben 
tener un solo tipo. Rust necesita saber a la hora de compilar qué tipo de variable
`number` es, definitivamente, para que pueda verificar a la hora de compilar que su tipo
es válido en cualquier lugar donde usemos `number`. Rust no sería capaz de hacer eso si el
tipo de "number" sólo se determinara en tiempo de ejecución; el compilador sería más complejo
y daría menos garantías sobre el código si tuviera que hacer un seguimiento de 
varios tipos hipotéticos para cualquier variable.

### Repetición con Loops

A menudo es útil ejecutar un bloque de código más de una vez. Para esta tarea,
Rust proporciona varios *loops*. Un loop pasa el código dentro del cuerpo
del loop hasta el final y luego vuelve a empezar inmediatamente desde principio. Para
experimentar con loops, vamos a hacer un nuevo proyecto llamado *loops*.

Rust tiene tres tipos de bucles: `loop`, `while` y `for`. Probemos cada uno.

#### Repitiendo Código con `loop`

La palabra clave "loop" le dice a Rust que ejecute un bloque de código una y otra vez
por siempre o hasta que le diga explícitamente que se detenga.

Como ejemplo, cambia el archivo *src/main.rs* en tu directorio *loops* para que se
vea así:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    loop {
        println!("again!");
    }
}
```

Cuando ejecutamos este programa, veremos `again!` impreso una y otra vez 
hasta que paremos el programa manualmente. La mayoría de los terminales soportan un atajo de teclado,
<span class="keystroke">ctrl-c</span>, para detener un programa que está 
atascado en un loop continuo. Inténtalo:

```text
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29 secs
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

El símbolo `^C` representa el lugar donde pulsaste ctrl-C. Puede que veas o no 
la palabra `again!` impresa después de la palabra `^C`, dependiendo de dónde estaba el código en el
loop cuando recibió la señal de parada.

Afortunadamente, Rust proporciona otra forma más confiable de romper un loop.
Puedes colocar la palabra clave `break` dentro del loop para indicar al programa cuándo 
debe detenerse la ejecución del loop. Recuerda que hicimos esto en el juego de adivinanzas en la
sección "Abandonar Después de una Adivinación Correcta" del Capítulo 2 para salir del
programa cuando el usuario ganó el juego adivinando el número correcto.

#### Loops Condicionales con `while`

A menudo es útil para un programa evaluar una condición dentro de un loop. Mientras
la condición es verdadera, el loop se ejecuta. Cuando la condición deja de ser verdadera, tu
llamas a `break`, deteniendo el loop. Este tipo de loop puede ser implementado usando una
combinación de `loop`, `if`, `else`, y `break`; puedes intentarlo ahora en un
programa, si quieres.

Sin embargo, este patrón es tan común que Rust tiene una construcción de lenguaje
incorporada para él, y se llama un loop ` while`. El siguiente ejemplo usa ` while`:
el programa hace un loop tres veces, contando cada vez hacia atrás. Luego, después del loop, 
imprime otro mensaje y termina:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}
```

Este construcción elimina muchas anidaciones que serían necesarias si usas
`loop`, `if`, `else`, y `break`, y es más claro. Mientras una condición se mantiene
verdadera, el código se ejecuta; de lo contrario, sale del loop.

#### Looping a Través de una Colección con `for`

Podrías usar el constructor `while` para hacer un bucle sobre los elementos de una colección,
como una array. Por ejemplo:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}
```

<span class="caption">Listing 3-5: Listado 3-5: Looping a través de cada elemento de una colección
usando un loop `while`</span>

Aquí, el código cuenta a través de los elementos de la array. Comienza en el índice
`0`, y luego loops hasta que alcanza el índice final en la array (es decir,
cuando el `índice < 5` ya no es verdadero). Al ejecutar este código se imprimirán todos
los elementos de la array:

```text
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
     Running `target/debug/loops`
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50
```

Los cinco valores de la array aparecen en el terminal, como se esperaba. Aunque el `index`
alcanzará un valor de `5` en algún punto, el loop deja de ejecutarse antes de intentar
recuperar un sexto valor de la array.

Pero este enfoque es propenso a errores; podríamos causar pánico en el programa si la 
longitud del índice es incorrecta. También es lento, porque el compilador añade código 
de tiempo de ejecución para realizar la comprobación condicional de cada elemento en cada iteración
a través del loop.

Como alternativa más eficaz, puede utilizar un loop `for` y ejecutar algún código 
para cada elemento de una colección. Un loop `for` luce así:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

<span class="caption">Listado 3-4: Looping a través de cada elemento de una colección
usando un loop `for`</span>

Cuando ejecutemos este código, veremos la misma salida que en Listado 3-3. Y lo que es más
importante, ahora hemos incrementado la seguridad del código y eliminado la 
posibilidad de errores que podrían resultar de ir más allá del final de la array o no
ir lo suficientemente lejos y perder algunos elementos.

Por ejemplo, en el código del Listado 3-3, si eliminas un elemento de la array
`a` pero olvidaste actualizar la condición a `while index < 4`, el código entraría
en pánico. Usando el loop `for`, no necesitas recordar cambiar ningún otro
código si cambias el número de valores en la array.

La seguridad y concisión de los loops `for` los convierte en el loop más comúnmente utilizado 
en la construcción de Rust. Incluso en situaciones en las que se quiera ejecutar cierto código un 
cierto número de veces, como en el ejemplo de cuenta atrás que usó un loop `while` 
en el Listado 3-3, la mayoría de los rustaceanos usarían un loop `for`. La manera de hacerlo 
sería usar un `Range`, que es un tipo proporcionado por la biblioteca estándar
que genera todos los números en secuencia empezando por un número y terminando
antes de otro número.

Esto es como se vería la cuenta atrás usando un loop `for` y otro método
del que aún no hemos hablado, `rev`, para invertir el rango:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

Este código es un poco más bonito, ¿no?

## Resumen

¡Lo lograste! Ese fue un capítulo considerable: aprendiste sobre variables, expresiones
escalares e `if`, ¡y loops! Si deseas practicar con los conceptos 
discutidos en este capítulo, intenta crear programas para hacer lo siguiente:

* Convierte las temperaturas entre grados Fahrenheit y Celsius.
* Genera el número n. º Fibonacci.
* Imprime la letra del villancico navideño "Los Doce Días de Navidad,"
aprovechando la repetición de la canción.

Cuando estés listo para seguir adelante, hablaremos de un concepto en Rust que *no* 
existe comúnmente en otros lenguajes de programación: propiedad.
