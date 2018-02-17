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
