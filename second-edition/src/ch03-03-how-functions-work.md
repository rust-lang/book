## Cómo Trabajan Las Funciones

Las funciones son generalizadas en el código Rust. Ya ha visto una de las funciones más 
importantes en el lenguaje: la función `main`, que es el punto de entrada 
de muchos programas. También has visto la palabra clave `fn`, que te permite declarar 
nuevas funciones.

El código de Rust usa *snake case* como el estilo convencional para los nombres de función y
variables. En snake case, todas las letras son minúsculas y se subrayan palabras separadas.
Aquí hay un programa que contiene una definición de la función como ejemplo:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

Las definiciones de funciones en Rust comienzan con "fn" y tienen un paréntesis
después del nombre de la función. Las llaves le dicen al compilador dónde comienza
y termina el cuerpo de la función.

Podemos llamar a cualquier función que hayamos definido introduciendo su nombre seguido de 
un paréntesis. Dado que en el programa se define la función `another_function`, se puede
llamar desde dentro de la función `main`. Ten en cuenta que hemos definido la función `another_function`
*después* de la función `main` en el código fuente; podríamos haberla definido antes 
también. A Rust no le importa dónde definas tus funciones, sólo que estén 
definidas en alguna parte.

Empecemos un nuevo proyecto binario llamado *funciones* para explorar
más a fondo las funciones. Coloca el ejemplo de la función `another_function` en *src/main. rs* y ejecútalo.
Deberías ver la siguiente salida:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
Hello, world!
Another function.
```

Las líneas se ejecutan en el orden en que aparecen en la función "main".
Primero, en el "¡Hola, mundo!" el mensaje se imprime y, luego se llama a `another_function` 
y se imprime su mensaje.

### Parámetros de Función

Las funciones también se pueden definir para tener *parámetros*, que son variables especiales
que forman parte de la firma de una función. Cuando una función tiene parámetros, podemos 
proporcionarle valores concretos para esos parámetros. Técnicamente, los valores concretos
se llaman *argumentos*, pero en la conversación casual la gente tiende a utilizar 
las palabras "parámetro" y "argumento" indistintamente para las variables 
en la definición de una función o los valores concretos transferidos cuando se llama a una
función.

La siguiente versión reescrita de `another_function` muestra cómo se ven los 
parámetros en Rust:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

Intenta ejecutar este programa; deberías obtener la siguiente salida:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
```

La declaración de `another_function` tiene un parámetro llamado "x". El tipo de 
`x` se especifica como `i32`. Cuando `5` se pasa a `another_function`, el macro
`println!` pone `5` donde el par de llaves estaban en la cadena 
de formato.

En las firmas de función, *debes* declarar el tipo de cada parámetro. Esta es 
una decisión deliberada en el diseño de Rust: requerir anotaciones de tipo en función
de las definiciones significa que el compilador casi nunca necesita que las uses en 
otra parte del código para averiguar lo que quieres decir.

Cuando desees que una función tenga varios parámetros, separa las declaraciones de parámetros
con comas, de este modo:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

Este ejemplo crea una función con dos parámetros, ambos del tipo `i32`. 
A continuación, la función imprime los valores de ambos parámetros. Ten en cuenta 
que los parámetros de función no tienen que ser del mismo tipo, simplemente están
en este ejemplo.

Intentemos ejecutar este código. Reemplaza el programa que se encuentra actualmente en el archivo *src/main. rs*
de tu proyecto *function* con el ejemplo anterior, y ejecútalo utilizando
`cargo run`:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
The value of y is: 6
```

Debido a que llamamos a la función con `5` como el valor para `x` y `6` se pasa 
como el valor para `y`, las dos cadenas se imprimen con estos valores.

### Cuerpos de las Funciones

Los cuerpos de las funciones están formados por una serie de enunciados que terminan opcionalmente en una
expresión. Hasta ahora, sólo hemos cubierto funciones sin una expresión final,
pero hemos visto las expresiones como partes de las declaraciones. Debido a que Rust es un 
lenguaje basado en expresiones, esta es una distinción importante a entender.
Otros lenguajes no tienen las mismas distinciones, así que veamos qué son las
declaraciones y expresiones y cómo sus diferencias afectan a los cuerpos de funciones.

### Declaraciones y expresiones

En realidad ya hemos usado declaraciones y expresiones. Las *declaraciones* son 
instrucciones que realizan alguna acción y no devuelven un valor. Las *expresiones* 
se evalúan a un valor resultante. Veamos algunos ejemplos.

Crear una variable y asignarle un valor con la palabra clave `let` es una declaración.
En el Listado 3-3, `let y = 6;` es una declaración:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let y = 6;
}
```

<span class="caption">Listing 3-3: A `main` function declaration containing one statement.</span>

Las definiciones de función son también declaraciones; todo el ejemplo anterior es una
expresión en sí misma.

Las declaraciones no devuelven valores. Por lo tanto, no puedes asignar una declaración `let` 
a otra variable, como intenta hacer el siguiente código:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let x = (let y = 6);
}
```

Cuando ejecutes este programa, obtendrás un error como éste:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: expected expression, found statement (`let`)
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^
  |
  = note: variable declaration using `let` is a statement
```

La declaración `let y = 6` no devuelve un valor, por lo que no hay nada 
a lo que obligar a `x`. Esto es diferente que en otros lenguajes, como C y Ruby,
donde la asignación devuelve el valor de la asignación. En esos lenguajes,
puedes escribir `x = y = 6` y tener tanto `x` como `y` tienen el valor `6`; ese no 
es el caso en Rust.

Las expresiones evalúan a algo y componen la mayor parte del resto del código que 
escribirás en Rust. Considera una simple operación matemática, como `5 + 6`, que 
es una expresión que evalúa al valor `11`. Las expresiones pueden ser parte de 
las declaraciones: en el Listado 3-3 que tenía la expresión `let y = 6; `, `6` es una 
expresión que evalúa al valor `6`. Llamar a una función es una 
expresión. Llamar a una macro es una expresión. El bloque que utilizamos para crear 
nuevos ámbitos, `{}`, es una expresión, por ejemplo:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

Esta expresión:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

es un bloque que, en este caso, evalúa a `4`. Ese valor se vincula a "y"
como parte de la declaración `let`. Observa la línea sin punto y coma al final,
a diferencia de la mayoría de las líneas que has visto hasta ahora. Las expresiones no incluyen
punto y coma final. Si agregas un punto y coma al final de una expresión, la conviertes en 
una sentencia, que no devuelve un valor. Ten esto en cuenta al 
explorar la función de retorno de valores y expresiones a continuación.

### Funciones con Valores de Retorno

Las funciones pueden devolver valores al código que los llama. No nombramos valores
de retorno, pero declaramos su tipo después de una flecha (`->`). En Rust, el valor de 
retorno de la función es sinónimo del valor de la expresión final en 
el bloque del cuerpo de una función. Aquí hay un ejemplo de una función que 
devuelve un valor:

<span class="filename">Filename: src/main.rs</span>

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

No hay llamadas de función, macros, o incluso expresiones `let` en la función `five`
sólo el número `5` por sí mismo. Esa es una función perfectamente válida en 
Rust. Ten en cuenta que el tipo de retorno de la función también se especifica como `-> i32`. Intenta
ejecutar este código; la salida debería verse así:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
```

El `5` en `five` es el valor de retorno de la función, por lo que el tipo de retorno
es `i32`. Examinemos esto con más detalle. Hay dos bits importantes:
primero, la línea `let x = five();` muestra que estamos usando el valor de retorno de 
una función para inicializar una variable. Debido a que la función `five` devuelve un `5`, 
esa línea es la misma que la siguiente:


```rust
let x = 5;
```

En segundo lugar, la función `five` no tiene parámetros y define el tipo de 
valor de retorno, pero el cuerpo de la función es un `5` solitario, sin punto y coma 
porque es una expresión cuyo valor queremos devolver. Veamos otro
ejemplo:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

Al ejecutar este código se imprimirá `The value of x is: 6`. ¿Qué sucede si colocamos un 
punto y coma al final de la línea que contiene `x + 1`, cambiándolo de una 
expresión a una declaración?

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

Al ejecutar este código se produce un error, como este:

```text
error[E0308]: mismatched types
 --> src/main.rs:7:28
  |
7 |   fn plus_one(x: i32) -> i32 {
  |  ____________________________^
8 | |     x + 1;
9 | | }
  | |_^ expected i32, found ()
  |
  = note: expected type `i32`
             found type `()`
help: consider removing this semicolon:
 --> src/main.rs:8:10
  |
8 |     x + 1;
  |          ^
```

El mensaje de error principal,"mismatched types", revela el problema principal con este 
código. La definición de la función `plus_one` dice que devolverá un 
`i32`, pero las declaraciones no evalúan a un valor, que se expresa por `()`,
la tupla vacía. Por lo tanto, no se devuelve nada que contradiga la definición 
de la función y provoque un error. En esta salida, Rust proporciona un mensaje para 
posiblemente ayudar a rectificar este problema: sugiere eliminar el punto y coma,
lo que solucionaría el error.
