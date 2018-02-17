## Flujo de Control

Decidir si ejecutar o no algún código dependiendo de si una condición es verdadera o 
decidir ejecutar un código repetidamente mientras una condición es verdadera son elementos
básicos en la mayoría de los lenguajes de programación. Las construcciones más comunes que 
permiten controlar el flujo de ejecución del código Rust son las expresiones `if` y los
loops.

### Expresiones `if`

Una expresión `if` nos permite ramificar nuestro código dependiendo de las condiciones. 
Proporcionamos una condición y luego declaramos: "Si se cumple esta condición, se ejecute este bloque
de código. Si la condición no se cumple, no se ejecute este bloque de código."

Crea un nuevo proyecto llamado *branches* en tu directorio *projects* para explorar
la expresión `if`. En el archivo *src/main. rs*, escribe lo siguiente:

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

Todas las expresiones `if` comienzan con la palabra clave `if`, que es seguida por una
condición. En este caso, la condición verifica si la variable `number` 
tiene un valor inferior a 5. El bloque de código que queremos ejecutar si la 
condición es verdadera se coloca inmediatamente después de la condición dentro de las
llaves. Los bloques de código asociados con las condiciones en las expresiones `if`
a veces se llaman *arms*, al igual que las arms en las expresiones `match` que 
discutimos en la sección "Comparación de Adivinanzas con el Número Secreto" del
Capítulo 2. Opcionalmente, también podemos incluir una expresión `else`, que optamos por
hacer aquí, para dar al programa un bloque de código alternativo a ejecutar en caso
de que la condición se evalúe como falsa. Si no proporcionas una expresión `else` y
la condición es falsa, el programa saltará el bloque `if` y pasará al 
siguiente bit de código.

Intenta ejecutar este código; deberías ver la siguiente salida:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/branches`
condition was true
```

Intentemos cambiar el valor de `number` a un valor que haga que la condición
sea `false` para ver qué sucede:

```rust,ignore
let number = 7;
```

Vuelve a ejecutar el programa y mira la salida:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/branches`
condition was false
```

También vale la pena notar que la condición en este código *debe* ser un `bool`. Para
ver qué sucede si la condición no es un `bool`, intenta ejecutar el siguiente
código:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```

La condición `if` evalúa a un valor de `3` esta vez, y Rust lanza un 
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

El error indica que Rust esperaba un `bool` pero obtuvo un entero. Rust no 
intentará convertir automáticamente tipos no booleanos a booleano, a diferencia
de lenguajes como Ruby y JavaScript. Tu debes ser explícito y proporcionar siempre
`if` con un `boolean` como su condición. Si queremos que el bloque de código `if` se ejecute
sólo cuando un número no es igual a `0`, por ejemplo, podemos cambiar `if` 
a la siguiente expresión:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}
```

Al ejecutar este código se imprimirá `number was something other than zero`.

#### Multiple Condiciones con `else if`

Podemos tener múltiples condiciones combinando `if` y `else` en una expresión 
`else if`. Por ejemplo:

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

Este programa tiene cuatro caminos posibles que puede tomar. Después de ejecutarlo, deberías 
ver la siguiente salida:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/branches`
number is divisible by 3
```

Cuando este programa se ejecuta, comprueba cada expresión `if` y ejecuta 
el primer cuerpo para el cual la condición es verdadera. Nota que aunque 6 es 
divisible por 2, no vemos la salida `number is divisible by 2`, ni vemos
el `number is not divisible by 4, 3, or 2` texto del bloque `else`. La 
razón es que Rust sólo ejecutará el bloque para la primera condición verdadera,
y una vez que encuentre una, este no comprobará el resto.

Usar demasiadas expresiones `else if` puede desordenar tu código, así que si tienes más
de una, quizás quieras refactorizar tu código. El capítulo 6 describe una poderosa
construcción de ramificación de Rust llamada `match` para estos casos.

#### Usando `if` en una Declaración `let`

Debido a que `if` es una expresión, podemos usarla en el lado derecho de una declaración 
`let`, por ejemplo en Listado 3-2:

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

<span class="caption">Listado 3-2: Asignar el resultado de una expresión `if`
 a una variable</span>

La variable `number` estará ligada a un valor basado en el resultado de la expresión
`if`. Corre este código para ver qué pasa:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30 secs
     Running `target/debug/branches`
The value of number is: 5
```

Recuerda que los bloques de código evalúan hasta la última expresión en ellos, y 
los números por sí mismos son también expresiones. En este caso, el valor completo de la 
expresión `if` depende del bloque de código que se ejecute. Esto significa que 
los valores que tienen el potencial de ser resultados de cada arm del `if` deben ser
del mismo tipo; en el Listado 3-4, los resultados tanto del arm `if` como del arm
`else` fueron `i32` enteros. Pero, ¿qué sucede si los tipos no coinciden, 
como en el siguiente ejemplo:

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

Cuando intentemos ejecutar este código, tendremos un error. Los arms `if` y `else` tienen 
tipos de valores incompatibles y Rust indica exactamente dónde encontrar el 
problema en el programa:

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

The expression in the `if` block evaluates to an integer, and the expression in
the `else` block evaluates to a string. This won’t work because variables must
have a single type. Rust needs to know at compile time what type the `number`
variable is, definitively, so it can verify at compile time that its type is
valid everywhere we use `number`. Rust wouldn’t be able to do that if the type
of `number` was only determined at runtime; the compiler would be more complex
and would make fewer guarantees about the code if it had to keep track of
multiple hypothetical types for any variable.

### Repetition with Loops

It’s often useful to execute a block of code more than once. For this task,
Rust provides several *loops*. A loop runs through the code inside the loop
body to the end and then starts immediately back at the beginning. To
experiment with loops, let’s make a new project called *loops*.

Rust has three kinds of loops: `loop`, `while`, and `for`. Let’s try each one.

#### Repeating Code with `loop`

The `loop` keyword tells Rust to execute a block of code over and over again
forever or until you explicitly tell it to stop.

As an example, change the *src/main.rs* file in your *loops* directory to look
like this:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    loop {
        println!("again!");
    }
}
```

When we run this program, we’ll see `again!` printed over and over continuously
until we stop the program manually. Most terminals support a keyboard shortcut,
<span class="keystroke">ctrl-c</span>, to halt a program that is stuck in a
continual loop. Give it a try:

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

The symbol `^C` represents where you pressed <span class="keystroke">ctrl-c
</span>. You may or may not see the word `again!` printed after the `^C`,
depending on where the code was in the loop when it received the halt signal.

Fortunately, Rust provides another, more reliable way to break out of a loop.
You can place the `break` keyword within the loop to tell the program when to
stop executing the loop. Recall that we did this in the guessing game in the
“Quitting After a Correct Guess” section of Chapter 2 to exit the
program when the user won the game by guessing the correct number.

#### Conditional Loops with `while`

It’s often useful for a program to evaluate a condition within a loop. While
the condition is true, the loop runs. When the condition ceases to be true, you
call `break`, stopping the loop. This loop type could be implemented using a
combination of `loop`, `if`, `else`, and `break`; you could try that now in a
program, if you’d like.

However, this pattern is so common that Rust has a built-in language construct
for it, and it’s called a `while` loop. The following example uses `while`: the
program loops three times, counting down each time. Then, after the loop, it
prints another message and exits:

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

This construct eliminates a lot of nesting that would be necessary if you used
`loop`, `if`, `else`, and `break`, and it’s clearer. While a condition holds
true, the code runs; otherwise, it exits the loop.

#### Looping Through a Collection with `for`

You could use the `while` construct to loop over the elements of a collection,
such as an array. For example, let’s look at Listing 3-3:

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

<span class="caption">Listing 3-3: Looping through each element of a collection
using a `while` loop</span>

Here, the code counts up through the elements in the array. It starts at index
`0`, and then loops until it reaches the final index in the array (that is,
when `index < 5` is no longer true). Running this code will print out every
element in the array:

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

All five array values appear in the terminal, as expected. Even though `index`
will reach a value of `5` at some point, the loop stops executing before trying
to fetch a sixth value from the array.

But this approach is error prone; we could cause the program to panic if the
index length is incorrect. It’s also slow, because the compiler adds runtime
code to perform the conditional check on every element on every iteration
through the loop.

As a more concise alternative, you can use a `for` loop and execute some code
for each item in a collection. A `for` loop looks like this code in Listing 3-4:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

<span class="caption">Listing 3-4: Looping through each element of a collection
using a `for` loop</span>

When we run this code, we’ll see the same output as in Listing 3-3. More
importantly, we’ve now increased the safety of the code and eliminated the
chance of bugs that might result from going beyond the end of the array or not
going far enough and missing some items.

For example, in the code in Listing 3-3, if you removed an item from the `a`
array but forgot to update the condition to `while index < 4`, the code would
panic. Using the `for` loop, you don’t need to remember to change any other
code if you changed the number of values in the array.

The safety and conciseness of `for` loops make them the most commonly used loop
construct in Rust. Even in situations in which you want to run some code a
certain number of times, as in the countdown example that used a `while` loop
in Listing 3-3, most Rustaceans would use a `for` loop. The way to do that
would be to use a `Range`, which is a type provided by the standard library
that generates all numbers in sequence starting from one number and ending
before another number.

Here’s what the countdown would look like using a `for` loop and another method
we’ve not yet talked about, `rev`, to reverse the range:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

This code is a bit nicer, isn’t it?

## Summary

You made it! That was a sizable chapter: you learned about variables, scalar
and `if` expressions, and loops! If you want to practice with the concepts
discussed in this chapter, try building programs to do the following:

* Convert temperatures between Fahrenheit and Celsius.
* Generate the nth Fibonacci number.
* Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”
taking advantage of the repetition in the song.

When you’re ready to move on, we’ll talk about a concept in Rust that *doesn’t*
commonly exist in other programming languages: ownership.
