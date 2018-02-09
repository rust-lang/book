## Variables y Mutabilidad

Como se menciona en el Capítulo 2, las variables por defecto son *inmutables*. Este es uno de los
muchos empujones en Rust que te anima a escribir tu código de una manera que toma
ventaja de la seguridad y fácil concurrencia que ofrece Rust. Sin embargo, 
todavía tienes la opción de hacer mutables tus variables. Exploremos cómo y por qué
Rust te anima a favorecer la inmutabilidad, y por qué es posible que desees optar por abandonar.

Cuando una variable es inmutable, significa que una vez que un valor está ligado a un nombre, tu
no puedes cambiar ese valor. Para ilustrar, vamos a generar un nuevo proyecto llamado
*variables* en tu directorio *projects* usando `cargo new --bin variables`.

Entonces, en tu nuevo directorio *variables*, abre *src/main. rs* y reemplaza tu directorio *variables*
con el siguiente código:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

Guarda y ejecuta el programa usando `Cargo run`. Deberías recibir un error
como se muestra en esta salida:

```text
error[E0384]: re-assignment of immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     println!("The value of x is: {}", x);
4 |     x = 6;
  |     ^^^^^ re-assignment of immutable variable
```

Este ejemplo muestra cómo el compilador te ayuda a encontrar errores en tus programas.
Aunque los errores del compilador pueden ser frustrantes, sólo significan que tu programa
todavía no está haciendo lo que quieres que haga; Eso *no* significa que no estás
siendo un buen programador! Los Rustaceos experimentados siguen teniendo errores en el compilador. El
error indica que la causa del error es `re-assignment of immutable
variable`, porque intentamos asignar un segundo valor a la variable inmutable 
`x`.

Es importante que obtengamos errores de tiempo de compilación cuando intentemos cambiar un 
valor que anteriormente habíamos designado como inmutable porque esta misma situación
puede llevar a los errores. Si una parte de nuestro código opera en el supuesto de que 
un valor nunca cambiará y otra parte de nuestro código cambia ese valor, es
posible que la primera parte del código no haga lo que fue diseñado para hacer.
Esta causa de errores puede ser difícil de rastrear después del hecho, especialmente
cuando la segunda pieza de código cambia el valor sólo *algunas veces*.

En Rust el compilador garantiza que cuando afirmamos que un valor no cambiará,
realmente no cambiará. Esto significa que cuando estás leyendo y escribiendo código, 
no tienes que hacer un seguimiento de cómo y dónde puede cambiar un valor, lo que puede 
hacer que el código sea más fácil de razonar.

Pero la mutabilidad puede ser muy útil. Las variables son inmutables sólo por defecto; 
podemos hacerlas mutables añadiendo `mut` delante del nombre de la variable. Además 
de permitir que este valor cambie, transmite la intención a futuros lectores 
del código indicando que otras partes del código cambiarán este valor variable.

Por ejemploe, cambia *src/main.rs* a lo siguiente:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

Cuando corremos el programa, tenemos lo siguiente:

```text
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
     Running `target/debug/variables`
The value of x is: 5
The value of x is: 6
```

Usando `mut`, se nos permite cambiar el valor al que `x` se une de `5` a 
`6`. En algunos casos, querrás hacer una variable mutable porque hace que el 
código sea más conveniente de escribir que una implementación que sólo usa variables 
inmutables.

Hay múltiples compensaciones a considerar, además de la prevención de 
errores. Por ejemplo, en los casos en los que estas utilizando grandes estructuras de datos, la mutación 
de una instancia puede ser más rápida que la copia y la devolución de nuevas instancias 
asignadas. Con estructuras de datos más pequeñas, la creación de nuevas instancias y la escritura en 
un estilo de programación más funcional puede ser más fácil de razonar, por lo que el menor 
rendimiento podría ser una penalización útil para obtener esa claridad.

### Diferencias Entre Variables y Constantes

No poder cambiar el valor de una variable podría haberte recordado 
otro concepto de programación que la mayoría de los otros lenguajes tienen: *constantes*. Como 
las variables inmutables, las constantes son también valores que están ligados a un nombre y 
no se les permite cambiar, pero hay algunas diferencias entre las constantes y 
las variables.

En primer lugar, no podemos usar `mut` con constantes: las constantes no sólo 
son inmutables de forma predeterminada, sino que siempre son inmutables.

Declaramos constantes usando la palabra clave `const` en lugar de la palabra clave `let`, 
y el tipo de valor *debe* ser anotado. Estamos a punto de cubrir tipos y 
escribir anotaciones en la siguiente sección, "Data Types", así que no te preocupe por los 
detalles ahora mismo, sólo ten en cuenta que siempre debemos anotar el tipo.

Las constantes pueden ser declaradas en cualquier ámbito, incluyendo el ámbito global, lo que las hace 
útiles para valores que muchas partes del código necesitan conocer.

La última diferencia es que las constantes sólo se pueden establecer en una expresión constante,
no el resultado de una llamada de función o cualquier otro valor que sólo se pueda 
calcular en tiempo de ejecución.

Aquí hay un ejemplo de una declaración constante donde el nombre de la constante es 
`MAX_POINTS` y su valor se fija en 100.000. (La convención de nomenclatura constante de Rust 
consiste en utilizar todas las mayúsculas con subrayados entre palabras):

```rust
const MAX_POINTS: u32 = 100_000;
```

Las constantes son válidas durante todo el tiempo que un programa se ejecuta, dentro del ámbito en el que 
fueron declaradas, lo que las convierte en una opción útil para los valores del dominio de tu aplicación 
que múltiples partes del programa podrían necesitar conocer, como el 
número máximo de puntos que cualquier jugador de un juego puede ganar o la velocidad de la luz.

Es útil nombrar valores codificados duros usados a lo largo de tu programa como constantes 
para transmitir el significado de ese valor a los futuros mantenedores del código. También 
ayuda tener solamente un lugar en tu código que necesites cambiar si el valor del 
hardcodeo necesitara ser actualizado en el futuro.

### Sombreado

Como vimos en el tutorial del juego de adivinanzas en el Capítulo 2, podemos declarar una nueva 
variable con el mismo nombre que una variable anterior, y la nueva variable 
*shadows* la variable anterior. Los Rustaceanos dicen que la primera variable es 
*sombreado* por la segunda, lo que significa que el valor de la segunda variable es lo que 
veremos cuando utilicemos la variable. Podemos sombrear una variable usando el mismo 
nombre de la variable y repitiendo el uso de la palabra clave `let` como se indica a continuación:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}
```

Este programa une primero `x` a un valor de `5`. Entonces, se sombrea `x` 
repitiendo `let x =`, tomando el valor original y agregando `1` para que el valor 
de `x` sea entonces `6`. La tercera declaración `let` también sombrea `x`, tomando el 
valor anterior y multiplicándolo por `2` para dar a `x` un valor final de `12`.
Cuando ejecutes este programa, saldrá lo siguiente:

```text
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
     Running `target/debug/variables`
The value of x is: 12
```

Esto es diferente a marcar una variable como `mut`, porque a menos que usemos la 
palabra clave `let` de nuevo, tendremos un error de tiempo de compilación si accidentalmente tratamos de 
reasignar a esta variable. Podemos realizar algunas transformaciones sobre un valor, pero 
la variable debe ser inmutable una vez completadas esas transformaciones.

La otra diferencia entre `mut` y sombreado es que, como estamos creando 
efectivamente una nueva variable cuando usamos de nuevo la palabra clave `let`, podemos 
cambiar el tipo de valor, pero reutilizar el mismo nombre. Por ejemplo, digamos que 
nuestro programa le pide a un usuario que muestre cuántos espacios desea entre un texto 
introduciendo caracteres de espacio, pero realmente queremos almacenar esa entrada como un número:

```rust
let spaces = "   ";
let spaces = spaces.len();
```

Esta construcción está permitida porque la primera variable `spaces` es de tipo cadena,
y la segunda variable `spaces`, que es una variable nueva que resulta 
tener el mismo nombre que la primera, es de tipo número. Sombreando así nos 
ahorra tener que inventar diferentes nombres, como `spaces_str` y 
`spaces_num`; en su lugar, podemos reutilizar el nombre más simple de `spaces`. Sin embargo, si 
nosotros intentamos usar `mut` para esto, como se muestra aquí:

```rust,ignore
let mut spaces = "   ";
spaces = spaces.len();
```

tendremos un error de tiempo de compilación porque no podemos mutar el 
tipo de una variable:

```text
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected &str, found usize
  |
  = note: expected type `&str`
             found type `usize`
```

Ahora que hemos explorado cómo funcionan las variables, veamos que más tipos de datos que 
pueden tener.
