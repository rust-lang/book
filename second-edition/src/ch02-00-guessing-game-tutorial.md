# Juego de adivinanzas

¡Vamos a adentrarnos en *Rust* trabajando en un proyecto juntos! Este capítulo
presenta algunos conceptos comunes en *Rust* mostrándonos cómo usarlos en un
programa real. Aprenderemos sobre las sentencias `let` y `match`, sobre métodos,
funciones asociadas, uso de _crates_ externos, ¡y mucho más! Los siguientes
capítulos se dedican a explorar estas ideas más en detalle. Por el momento,
en este capítulo se ponen en práctica las partes fundamentales.

Vamos a implementar un típico problema de programación para principiantes: Un
juego de adivinanzas. Funciona así: El programa generará un número aleatorio
entre 1 y 100 y luego le pedirá al jugador que trate de adivinarlo. Tras
introducir un número, indicará al jugador si su suposición fue muy alta o
muy baja. Si el jugador adivina el número, el juego imprimirá un mensaje de
felicitación y se cerrará.

## Configurando un nuevo proyecto

Para configurar un proyecto nuevo, ve al directorio de *proyectos* que
creamos en el capítulo 1, y crea un nuevo proyecto usando Cargo, de esta manera:

```text
$ cargo new adivinanza --bin
$ cd adivinanza
```

El primer comando, `cargo new`, recibe el nombre del proyecto (`adivinanza`)
como primer argumento. La opción `--bin` indica que Cargo debe crear un
proyecto binario, similar al del capítulo 1. El segundo comando cambia el
directorio actual al de nuestro proyecto.

Demos un vistazo al archivo *Cargo.toml* que se ha creado:

<span class="filename">Filename: Cargo.toml</span>

```toml
[package]
name = "adivinanza"
version = "0.1.0"
authors = ["Tu Nombre <tu@ejemplo.com>"]

[dependencies]
```

Si la información del autor que Cargo leyó del entorno de nuestra máquina no es
correcta, modifica el archivo con los cambios necesarios y guárdalo de nuevo.

Como vimos en el capítulo 1, `cargo new` genera un programa “Hola Mundo!”.
Abre ahora el archivo *src/main.rs*:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    println!("¡Hola, mundo!");
}
```

Ahora, vamos a compilar este programa y ejecutarlo en un solo paso usando el
comando `cargo run`:

```text
$ cargo run
   Compiling adivinanza v0.1.0 (file:///proyectos/adivinanza)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50 secs
     Running `target/debug/adivinanza`
Hello, world!
```
El comando `run` es bastante práctico cuando necesitamos iterar rápidamente en un
proyecto como este juego, donde queremos probar velozmente cada iteración antes
de movernos a la siguiente.

Abre de nuevo el archivo *src/main.rs*. En este archivo es donde vas a escribir todo
el código.

## Procesando un intento de adivinanza

La primera parte del programa pedirá datos al usuario, procesará los datos, y se
encargará de verificar que los datos tengan el formato esperado. Para comenzar,
vamos a dejar que el jugador introduzca una suposición. Escribe el código que se
encuentra en Código 2-1 dentro de nuestro archivo *src/main*.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::io;

fn main() {
    println!("¡Adivina el número!");

    println!("Por favor introduce tu suposición.");

    let mut suposicion = String::new();

    io::stdin().read_line(&mut suposicion)
        .expect("Error al leer la línea");

    println!("Tu suposición fue: {}", suposicion);
}
```

<span class="caption">Código 2-1: Código para obtener una suposición del usuario y mostrarla en pantalla</span>

Este código contiene bastante información, así que vamos paso a paso.
Para obtener la entrada del usuario y luego imprimir el resultado como salida,
necesitamos importar la librería `io` (entrada/salida o *input/output* en inglés)
a nuestro código. La librería `io` forma parte de la librería estándar (conocida
como `std`):

```rust,ignore
use std::io;
```

Por defecto, *Rust* solo importa unos cuantos tipos en el contexto de cada programa
([el *preludio*][prelude]<!-- ignore -->). Si queremos usar un tipo que no está en
el preludio, debemos importarlo explícitamente usando la sentencia `use`. Al
usar la librería `std::io`, le estamos dando al código nuevas posibilidades
para operar con entrada y salida de datos, incluyendo la funcionalidad de que
el usuario introduzca información.

[prelude]: ../../std/prelude/index.html

Como vimos en el Capítulo 1, la función `main` es el punto de entrada de nuestro
programa:

```rust,ignore
fn main() {
```

La sintaxis `fn` declara una nueva función, los paréntesis `()` indican que esta
función no recibe parámetros, y `{` indica el comienzo del cuerpo de la función.

Como también vimos en el Capítulo 1, `println!` es una macro que imprime una
cadena de caracters (string) en la pantalla:

```rust,ignore
println!("¡Adivina el número!");

println!("Por favor escribe una suposición.");
```

Este código muestra un mensaje con el propósito de juego, y pide
datos al usuario.

### Almacenando Valores en Variables

Lo siguiente será crear un lugar para almacenar la entrada del usuario:

```rust,ignore
let mut suposicion = String::new();
```

¡Ahora nuestro programa se vuelve más interesante! Varias cosas suceden en esta
breve línea de código. Fíjate que usamos la sentencia `let`, la cual nos permite
crear *variables*. Ahí va otro ejemplo:

```rust,ignore
let foo = bar;
```

Esta linea creará una variable nueva llamada `foo` y la vinculará al valor
`bar`. En *Rust*, las variables son inmutables por defecto. El siguiente ejemplo
nos muestra como usar `mut` antes del nombre de una variable para hacerla
mutable:

```rust
let foo = 5; // inmutable
let mut bar = 5; // mutable
```

> Nota: La sintaxis `//` indica el inicio de un comentario que termina al final
> de la línea. *Rust* ignora todo lo que encuentra dentro de un comentario.

Ahora sabemos que `let mut suposicion` creará una variable mutable llamada
`suposicion`. Al otro lado del signo igual (`=`) se encuentra el valor que vamos
a asignar a `suposicion`, el cual es el resultado de llamar a `String::new`, una
función que genera una nueva instancia de un `String`. [`String`][string]<!-- ignore -->
es un tipo de *String* de la librería estándar que puede contener texto de
tamaño variable en formato UTF-8.

[string]: ../../std/string/struct.String.html

La sintaxis `::` en `::new` indica que `new` es una *función asociada* al tipo
`String`. Una función asociada se implementa directamente en un tipo, `String`
en este caso, en vez de en una instancia particular de un `String`. Algunos
lenguajes llaman a este tipo de función *método estático*.

La función `new` crea un nuevo `String` vacío. Te encontrarás un montón de
funciones `new` en diferentes tipos, la razón es que `new` es el nombre que
suele darse a las funciones que crean un valor nuevo de algún tipo.

En resumen, la linea `let mut suposicion = String::new();` ha creado una
variable mutable que se encuentra vinculada a una instancia nueva y vacía de un
`String`. ¡Uff!

Recuerda que hemos importado la funcionalidad de entrada y salida de la
librería estandar con `use std::io;` en la primera linea de nuestro programa.
Ahora podemos llamar a la función asociada `stdin` de `io`:

```rust,ignore
io::stdin().read_line(&mut suposicion)
    .expect("Error al leer la línea");
```

Si no hubiéramos escrito la linea `use std::io` al comienzo del programa, aún
podríamos llamar a esta función escribiendo `std::io::stdin`. La función `stdin`
devuelve una instancia de [`std::io::Stdin`][iostdin]<!-- ignore -->, la cual
es un tipo que representa un identificador para la entrada estándar de nuestra
terminal.

[iostdin]: ../../std/io/struct.Stdin.html

La siguiente parte del código, `.read_line(&mut suposicion)`, llama al método
[`read_line`][read_line]<!-- ignore -->  del identificador para obtener la
entrada del usuario. También le estamos pasando un argumento a `read_line`:
`&mut suposicion`.

[read_line]: ../../std/io/struct.Stdin.html#method.read_line

El propósito de `read_line` es leer lo que el usuario escriba en la entrada
estándar y ponerlo en un *string*, por lo tanto toma un *string* como argumento.
Este *string* debe ser mutable para que el método pueda cambiar su contenido y
agregar la entrada del usuario.

El símbolo `&` indica que este argumento es una *referencia*, lo cual nos
permite que distintas partes de nuestro código accedan a una parte de los datos
sin que haga falta copiarla en memoria varias veces. Las referencias son un
asunto complejo, una de las grandes ventajas de *Rust* radica en lo sencillo y
seguro que es usar referencias. No necesitas saber demasiado para terminar este
programa: el Capítulo 4 trata el tema de las referencias de forma minuciosa.
Por ahora, todo lo que hay que saber es que las referencias son inmutables por
defecto, como las variables, y por tanto necesitamos escribir `&mut suposicion`
en vez de `&suposicion` para hacerla mutable.

Aun nos queda para terminar de entender esta línea. Siendo una sola línea, forma
parte de una única línea lógica de código. La segunda parte es este método:

```rust,ignore
.expect("Error al leer la línea");
```

A veces conviene llamar un método con la sintaxis `.foo()` en una línea nueva
para dividir líneas de texto muy largas. Aun así podríamos haber escrito:

```rust,ignore
io::stdin().read_line(&mut suposicion).expect("Error al leer la línea");
```

Sin embargo, una sola línea larga es difícil de leer, es mejor dividirla. Ahora
veamos en detalle que hace esta línea.

### Manejando posibles errores con el tipo `Result`

Como mencionamos antes, `read_line` pone recibe los datos del usuario y los coloca
en el *string* que le pasamos, pero también devuelve un valor, en este caso, un
[`io::Result`][ioresult]<!-- ignore -->. *Rust* tiene una serie de tipos
llamados `Result` en su librería estándar: Un [`Result`][result]<!-- ignore -->
genérico y varias versiones específicas para los sub-módulos, como `io::Result`.

[ioresult]: ../../std/io/type.Result.html
[result]: ../../std/result/enum.Result.html

Los tipos `Result` son [*enumeraciones*][enums]<!-- ignore --> o *enums*. Una
enumeración es un tipo que puede tener un número fijo de valores, estos valores
reciben el nombre de *variantes* de la enumeración (o *enum’s variants* en
inglés). El capítulo 6 explica las enumeraciones con mas detalle.

[enums]: ch06-00-enums.html

Las variantes de `Result` son `Ok` y `Err`. `Ok` indica que la operación tuvo
éxito, y que dentro de la variante `Ok` se encuentra el valor generado. `Err`
indica que la operación falló y contiene información del cómo y por qué de este
error.

El propósito de estos tipos `Result` es codificar información para tratar erorres
Los valores de tipo `Resultado`, como cualquier otro tipo, tienen métodos
definidos. Una instacia de `io::Result` tiene un
[método `expect`][expect]<!-- ignore --> que podemos llamar. Si esta instancia
de `io::Result` es un valor de tipo `Err`, `expect` hará que nuestro programa
falle y muestre el mensaje que le pasamos como argumento. Si el método
`read_line` devuelve `Err`, casi con total seguridad será por un error proveniente
del sistema operativo. Si esta instancia de `io::Result` es un valor `Ok`,
`expect` tomará el valor que se encuentra dentro del `Ok` y lo devolverá para
que podamos usarlo. En este caso, este valor es el número de bytes que el
usuario introdujo en la entrada estándar.

[expect]: ../../std/result/enum.Result.html#method.expect

Nuestro programa compilará si no utilizamos `expect`, pero recibiremos una
advertencia:

```text
$ cargo build
   Compiling adivinanza v0.1.0 (file:///proyectos/adivinanza)
warning: unused `std::result::Result` which must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut suposicion);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(unused_must_use)] on by default
```

Rust nos advierte que no hemos utilizado el valor de tipo `Result` que obtuvimos
de `read_line`, indicando que el programa no ha manejado un posible error. La
forma correcta de eliminar este error es escribir código para tratar con él,
pero de momento queremos que nuestro programa falle cuando haya un problema,
asi que podemos usar `expect`. Aprenderemos como recuperarnos de errores en el
capítulo 9.

### Imprimiendo Valores con marcadores de posición `println!`

Quitando la llave del final, sólo hay una línea mas por aclarar en nuestro
código, que es la siguiente:

```rust,ignore
println!("You guessed: {}", guess);
```

Esta línea imprime el string en el que hemos guardado los datos del usuario. El
grupo de `{}` es un marcador de posición que muestra un valor en un sitio determinado.
Se puede usar más de un valor con `{}`: el primer grupo de `{}` mostrara el primer
valor que aparezca después del string, el segundo grupo mostrará el segundo valor,
y así sucesivamente. Para imprimir varios valores en una llamada a `println!` se
haría de la siguiente manera:

```rust
let x = 5;
let y = 10;

println!("x = {} e y = {}", x, y);
```

El resultado de este código será `x = 5 e y = 10`.

### Probando la primera parte

Ahora vamos a probar la primera parte del juego de adivinanzas. Puedes ejecutarlo
usando `cargo run`.

```text
$ cargo run
   Compiling adivinanza v0.1.0 (file:///proyectos/adivinanza)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
     Running `target/debug/adivinanza`
¡Adivina el número!
Por favor escribe una suposición.
6
Tu suposición fue:  6
```

En este momento, podemos dar la primera parte del juego por terminada: estamos
leyendo datos del usuario y mostrándolos por pantalla.

## Generando un Número Secreto

A continuación, hay que generar un número secreto que el usuario deberá tratar
de adivinar. El número secreto debería ser diferente en cada nuevo juego, de
otra manera no sería muy divertido jugar más de una vez. Vamos a usar un número
aleatorio entre 1 y 100 para que el juego no sea muy difícil. De momento, Rust
no incluye funcionalidad para generar números aleatorios en la libería estándar.
Sin embargo, el equipo de Rust provee un [crate `rand`][randcrate]

[randcrate]: https://crates.io/crates/rand

### Usar un Crate para conseguir nuevas Funcionalidades

Recuerda que un *crate* no es más que un paquete de código en Rust. El proyecto
que estamos creando es un *crate binario*, el cual es ejecutable. El crate
`rand` es un *crate librería*, que continene código a reutilizar en otros
programas

Cargo es especialmente útil a la hora de utilizar crates externos. Antes de
escribir código que utilice `rand`, hay que modificar el archivo *Cargo.toml*,
donde vamos a incluid el crate `rand` como dependencia. Abre este archivo y
añade la siguiente línea justo debajo de la sección `[dependencies]`, que
Cargo ya include por defecto:

<span class="filename">Filename: Cargo.toml</span>

```toml
[dependencies]

rand = "0.3.14"
```

En el archivo , todo lo que viene después de una cabecera de sección, forma parte
de dicha sección hasta que otra nueva comience. La sección `[dependencies]` se
usa para indicar a Cargo qué crates externos - y qué versión específica de éstos -
necesita tu proyecto. En este ejemplo, vamos a añadir el crate `rand` en su
versión `0.3.14`. Cargo utiliza [Versionado Semántico][semver]<!-- ignore -->
(que a veces se llama *SemVer*), un estándar para escribir números de versión.
El número `0.3.14` es una abreviatura de `^0.3.14`, que quiere decir "cualquier
versión con API pública compatible con la version 0.3.14".

[semver]: http://semver.org

Ahora, sin tocar el código, vamos a compilar el proyecto, tal y como muestra en el
Código 2-2:


```text
$ cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading rand v0.3.14
 Downloading libc v0.2.14
   Compiling libc v0.2.14
   Compiling rand v0.3.14
   Compiling adivinanza v0.1.0 (file:///proyectos/adivinanza)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
```

<span class="caption">Código 2-2: El resultado de ejecutar `cargo build` tras
añadir el crate rand como dependencia</span>

En tu caso, pueden aparecer números de versión diferentes (pero de igual forma
serán compatible con el código, gracias a SemVer), y puede que el orden de las
líneas no sea exactamente el mismo.

Ahora que tenemos la dependencia externa, Cargo busca las últimas versiones
del *registro*, el cual es una copia del contenido de [Crates.io][cratesio].
Crates.io es el portal del ecosistema Rust en el que se añaden proyectos Rust
de software libre, para que otros desarrollares los reutilicen.

[cratesio]: https://crates.io

Tras actualizar el registro, Cargo comprueba la sección de `[dependencias]` y
descarga las que no estan aún en el proyecto. En este caso, aunque sólo hemos
puesto `rand` como dependencia, Cargo también descarga una copia de `libc`, ya
que `rand` necesita la librería `libc` para funcionar. Tras la descarga, Rust
compila las librerías y después compila el proyecto completo junto con las
dependencias disponibles.

Si ejecutas directamente `cargo build` de nuevo sin hacer ningún cambio, no
aparecerá nada en la consola. Cargo sabe que ha descargado y compilado las
dependencias, y que no has hecho cambios en el archivo *Cargo.toml*. Cargo
también sabe que no has tocado nada en tu código, por lo que no necesita
volver a compilarlo. Sin más por hacer, la ejecución simplemente acaba. Si por
ejemplo abres el archivo *src/main.rs* y realizar cualquier pequeño cambio, lo
guardas, y vuelves a compilar, verás solo dos líneas en la consola:


```text
$ cargo build
   Compiling adivinanza v0.1.0 (file:///proyectos/adivinanza)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
```

Estas líneas muestra que Cargo sólamente realiza una compilación tras tu ligero
cambio en el archivo *src/main.rs*. Las dependencias no han cambiado, así que
que Cargo puede reutilizar lo que ya ha descargado y compilado previamente.
Simplemente vuelve a compilar tus cambios en el código.


#### El archivo *Cargo.lock* asegura compilaciones reproducibles

Cargo tiene un mecanismo que permite asegurarte de que puedes crear de nuevo el
mismo artefacto, cada vez que tú o alguien más compila tu código: Cargo solo va
a utilizar las dependencias especificadas, a menos que indiques lo contrario.
Por ejemplo, ¿Qué pasa si sale una nueva versión de `rand` con número `v0.3.15`,
que continene un bug fix importante, pero que al mismo tiempo continene una
regresión que va a provocar errores en tu código?

La respuesta a este problema es el fichero *Cargo.lock*, que se crea al mismo
tiempo que utilizas `cargo build` por primera vez, y que se encuentra en el
directorio *guessing_game*. Cuando compilas un proyecto por primera vez, Cargo
busca automáticamente la versión más indicada de las dependencias de tu proyecto,
y las guarda en el fichero *Cargo.lock*. En posteriores compilaciones, Cargo
buscará primero si existe este fichero *Cargo.lock*, y usará las versiones de
las dependencias que estén ahí escritas, en lugar de buscar de nuevo cuál es
la versión más adecuada. Esto nos permite hacer un build reproducible de
manera automática. Dicho de otro modo, tu proyecto continuará con la versión
`0.3.14` de `rand` a menos que actualices a mano, gracias al fichero *Cargo.lock*.

#### Actualizar un Crate para obtener una versión nueva

Cuando *quieres* actualizar un crate, Cargo usa un comando diferente, `update`,
que hace lo siguiente:

1. Ignorar el fichero *Cargo.lock*, ya que va a buscar de nueva las últimas
versiones que encajas con nuestras especificaciones en *Cargo.toml*.
2. Si no hay problemas, Cargo escribe las nuevas versiones en el fichero
*Cargo.lock*.

Por defecto, Cargo sólo buscará versiones mayores a `0.3.0` y menores que `0.4.0`.
Si el crate de `rand` ha sacado dos nuevas versiones, `0.3.15` y `0.4.0`, cuando
hagas `cargo update` verás este mensaje en la consola:

```text
$ cargo update
    Updating registry `https://github.com/rust-lang/crates.io-index`
    Updating rand v0.3.14 -> v0.3.15
```

En este momento, verás que el fichero *Cargo.lock* también ha cambiado. Ahora
aparece la versión `0.3.15` del crate `rand` que estamos usando.

Si quieres usar la versión `0.4.0` de `rand`, o cualquier versión de la serie
`0.4.x`, tendrás que actualizar el fichero *Cargo.toml* y poner lo siguiente:

```toml
[dependencies]

rand = "0.4.0"
```

La siguiente vez que ejecutes `cargo build`, Cargo actalizará el registro de
crates disponibles, y eveluará de nuevo los requisitos para `rand` de acuerdo
a la versión nueva que hayas especificado.

Hay mucho más por descubrir sobre [Cargo][doccargo]<!-- ignore --> y [su
ecosistema][doccratesio]<!-- ignore --> que veremos en más profundidad
llegado el Capítulo 14. De momento, con lo que sabes hasta ahora es más que
suficiente. Reutilizar librerías es muy facil con Cargo, lo que permite a los
Rusteros usar diferentes paquetes para sus proyectos.

[doccargo]: http://doc.crates.io
[doccratesio]: http://doc.crates.io/crates-io.html

### Generating a Random Number

Let’s start *using* `rand`. The next step is to update *src/main.rs*, as shown
in Listing 2-3:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

<span class="caption">Listing 2-3: Code changes needed in order to generate a
random number</span>

We’re adding a `extern crate rand;` line to the top that lets Rust know we’ll be
using that external dependency. This also does the equivalent of calling `use
rand`, so now we can call anything in the `rand` crate by prefixing it with
`rand::`.

Next, we’re adding another `use` line: `use rand::Rng`. `Rng` is a trait that
defines methods that random number generators implement, and this trait must be
in scope for us to use those methods. Chapter 10 will cover traits in detail.

Also, we’re adding two more lines in the middle. The `rand::thread_rng` function
will give us the particular random number generator that we’re going to use:
one that is local to the current thread of execution and seeded by the
operating system. Next, we call the `gen_range` method on the random number
generator. This method is defined by the `Rng` trait that we brought into
scope with the `use rand::Rng` statement. The `gen_range` method takes two
numbers as arguments and generates a random number between them. It’s inclusive
on the lower bound but exclusive on the upper bound, so we need to specify `1`
and `101` to request a number between 1 and 100.

Knowing which traits to use and which functions and methods to call from a
crate isn’t something that you’ll just *know*. Instructions for using a crate
are in each crate’s documentation. Another neat feature of Cargo is that you
can run the `cargo doc --open` command that will build documentation provided
by all of your dependencies locally and open it in your browser. If you’re
interested in other functionality in the `rand` crate, for example, run `cargo
doc --open` and click `rand` in the sidebar on the left.

The second line that we added to the code prints the secret number. This is
useful while we’re developing the program to be able to test it, but we’ll
delete it from the final version. It’s not much of a game if the program prints
the answer as soon as it starts!

Try running the program a few times:

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4
$ cargo run
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```

You should get different random numbers, and they should all be numbers between
1 and 100. Great job!

## Comparing the Guess to the Secret Number

Now that we have user input and a random number, we can compare them. That
step is shown in Listing 2-4:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

<span class="caption">Listing 2-4: Handling the possible return values of
comparing two numbers</span>

The first new bit here is another `use`, bringing a type called
`std::cmp::Ordering` into scope from the standard library. `Ordering` is
another enum, like `Result`, but the variants for `Ordering` are `Less`,
`Greater`, and `Equal`. These are the three outcomes that are possible when you
compare two values.

Then we add five new lines at the bottom that use the `Ordering` type:

```rust,ignore
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

The `cmp` method compares two values and can be called on anything that can be
compared. It takes a reference to whatever you want to compare with: here it’s
comparing the `guess` to the `secret_number`. `cmp` returns a variant of the
`Ordering` enum we brought into scope with the `use` statement. We use a
[`match`][match]<!-- ignore --> expression to decide what to do next based on
which variant of `Ordering` was returned from the call to `cmp` with the values
in `guess` and `secret_number`.

[match]: ch06-02-match.html

A `match` expression is made up of *arms*. An arm consists of a *pattern* and
the code that should be run if the value given to the beginning of the `match`
expression fits that arm’s pattern. Rust takes the value given to `match` and
looks through each arm’s pattern in turn. The `match` construct and patterns
are powerful features in Rust that let you express a variety of situations your
code might encounter and helps ensure that you handle them all. These features
will be covered in detail in Chapter 6 and Chapter 18, respectively.

Let’s walk through an example of what would happen with the `match` expression
used here. Say that the user has guessed 50, and the randomly generated secret
number this time is 38. When the code compares 50 to 38, the `cmp` method will
return `Ordering::Greater`, because 50 is greater than 38. `Ordering::Greater`
is the value that the `match` expression gets. It looks at the first arm’s
pattern, `Ordering::Less`, but the value `Ordering::Greater` does not match
`Ordering::Less`, so it ignores the code in that arm and moves to the next arm.
The next arm’s pattern, `Ordering::Greater`, *does* match
`Ordering::Greater`! The associated code in that arm will execute and print
`Too big!` to the screen. The `match` expression ends because it has no need to
look at the last arm in this particular scenario.

However, the code in Listing 2-4 won’t compile yet. Let’s try it:

```text
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
error[E0308]: mismatched types
  --> src/main.rs:23:21
   |
23 |     match guess.cmp(&secret_number) {
   |                     ^^^^^^^^^^^^^^ expected struct `std::string::String`, found integral variable
   |
   = note: expected type `&std::string::String`
   = note:    found type `&{integer}`

error: aborting due to previous error
Could not compile `guessing_game`.
```

The core of the error states that there are *mismatched types*. Rust has a
strong, static type system. However, it also has type inference. When we wrote
`let guess = String::new()`, Rust was able to infer that `guess` should be a
`String` and didn’t make us write the type. The `secret_number`, on the other
hand, is a number type. A few number types can have a value between 1 and 100:
`i32`, a 32-bit number; `u32`, an unsigned 32-bit number; `i64`, a 64-bit
number; as well as others. Rust defaults to an `i32`, which is the type of
`secret_number` unless we add type information elsewhere that would cause Rust
to infer a different numerical type. The reason for the error is that Rust will
not compare a string and a number type.

Ultimately, we want to convert the `String` the program reads as input into a
real number type so we can compare it to the guess numerically. We can do
that by adding the following two lines to the `main` function body:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

The two new lines are:

```rust,ignore
let guess: u32 = guess.trim().parse()
    .expect("Please type a number!");
```

We create a variable named `guess`. But wait, doesn’t the program
already have a variable named `guess`? It does, but Rust allows us to
*shadow* the previous value of `guess` with a new one. This feature is often
used in similar situations in which you want to convert a value from one type
to another type. Shadowing lets us reuse the `guess` variable name rather than
forcing us to create two unique variables, like `guess_str` and `guess` for
example. (Chapter 3 covers shadowing in more detail.)

We bind `guess` to the expression `guess.trim().parse()`. The `guess` in the
expression refers to the original `guess` that was a `String` with the input in
it. The `trim` method on a `String` instance will eliminate any whitespace at
the beginning and end. `u32` can only contain numerical characters, but the
user must press the <span class="keystroke">enter</span> key to satisfy
`read_line`. When the user presses <span class="keystroke">enter</span>, a
newline character is added to the string. For example, if the user types <span
class="keystroke">5</span> and presses <span class="keystroke"> enter</span>,
`guess` looks like this: `5\n`. The `\n` represents “newline,” the
<span class="keystroke">enter</span>key. The `trim` method eliminates `\n`,
resulting in just `5`.

The [`parse` method on strings][parse]<!-- ignore --> parses a string into some
kind of number. Because this method can parse a variety of number types, we
need to tell Rust the exact number type we want by using `let guess: u32`. The
colon (`:`) after `guess` tells Rust we’ll annotate the variable’s type. Rust
has a few built-in number types; the `u32` seen here is an unsigned, 32-bit
integer. It’s a good default choice for a small positive number. You’ll learn
about other number types in Chapter 3. Additionally, the `u32` annotation in
this example program and the comparison with `secret_number` means that Rust
will infer that `secret_number` should be a `u32` as well. So now the
comparison will be between two values of the same type!

[parse]: ../../std/primitive.str.html#method.parse

The call to `parse` could easily cause an error. If, for example, the string
contained `A👍%`, there would be no way to convert that to a number. Because it
might fail, the `parse` method returns a `Result` type, much like the
`read_line` method does as discussed earlier in “Handling Potential Failure
with the Result Type”. We’ll treat this `Result` the same way by
using the `expect` method again. If `parse` returns an `Err` `Result` variant
because it couldn’t create a number from the string, the `expect` call will
crash the game and print the message we give it. If `parse` can successfully
convert the string to a number, it will return the `Ok` variant of `Result`,
and `expect` will return the number that we want from the `Ok` value.

Let’s run the program now!

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43 secs
     Running `target/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```

Nice! Even though spaces were added before the guess, the program still figured
out that the user guessed 76. Run the program a few times to verify the
different behavior with different kinds of input: guess the number correctly,
guess a number that is too high, and guess a number that is too low.

We have most of the game working now, but the user can make only one guess.
Let’s change that by adding a loop!

## Allowing Multiple Guesses with Looping

The `loop` keyword gives us an infinite loop. Add that now to give users more
chances at guessing the number:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
```

As you can see, we’ve moved everything into a loop from the guess input prompt
onward. Be sure to indent those lines another four spaces each, and run the
program again. Notice that there is a new problem because the program is doing
exactly what we told it to do: ask for another guess forever! It doesn’t seem
like the user can quit!

The user could always halt the program by using the keyboard shortcut
<span class="keystroke">ctrl-c</span>. But there’s another way to escape this
insatiable monster that we mentioned in the `parse` discussion in “Comparing the
Guess to the Secret Number”: if the user enters a non-number answer, the program
will crash. The user can take advantage of that in order to quit, as shown here:

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
     Running `target/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/libcore/result.rs:785
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/guess` (exit code: 101)
```

Typing `quit` actually quits the game, but so will any other non-number input.
However, this is suboptimal to say the least. We want the game to automatically
stop when the correct number is guessed.

### Quitting After a Correct Guess

Let’s program the game to quit when the user wins by adding a `break`:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

By adding the `break` line after `You win!`, the program will exit the loop
when the user guesses the secret number correctly. Exiting the loop also means
exiting the program, because the loop is the last part of `main`.

### Handling Invalid Input

To further refine the game’s behavior, rather than crashing the program when
the user inputs a non-number, let’s make the game ignore a non-number so the
user can continue guessing. We can do that by altering the line where `guess` is
converted from a `String` to a `u32`:

```rust,ignore
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

Switching from an `expect` call to a `match` expression is how you generally
move from crash on error to actually handling the error. Remember that `parse`
returns a `Result` type, and `Result` is an enum that has the variants `Ok` or
`Err`. We’re using a `match` expression here, like we did with the `Ordering`
result of the `cmp` method.

If `parse` is able to successfully turn the string into a number, it will return
an `Ok` value that contains the resulting number. That `Ok` value will match the
first arm’s pattern, and the `match` expression will just return the `num` value
that `parse` produced and put inside the `Ok` value. That number will end up
right where we want it in the new `guess` variable we’re creating.

If `parse` is *not* able to turn the string into a number, it will return an
`Err` value that contains more information about the error. The `Err` value
does not match the `Ok(num)` pattern in the first `match` arm, but it does match
the `Err(_)` pattern in the second arm. The `_` is a catchall value; in this
example, we’re saying we want to match all `Err` values, no matter what
information they have inside them. So the program will execute the second arm’s
code, `continue`, which means to go to the next iteration of the `loop` and ask
for another guess. So effectively, the program ignores all errors that `parse`
might encounter!

Now everything in the program should work as expected. Let’s try it by running
`cargo run`:

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
     Running `target/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

Awesome! With one tiny final tweak, we will finish the guessing game: recall
that the program is still printing out the secret number. That worked well for
testing, but it ruins the game. Let’s delete the `println!` that outputs the
secret number. Listing 2-5 shows the final code:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

<span class="caption">Listing 2-5: Complete code of the guessing game</span>

## Summary

At this point, you’ve successfully built the guessing game! Congratulations!

This project was a hands-on way to introduce you to many new Rust concepts:
`let`, `match`, methods, associated functions, using external crates, and more.
In the next few chapters, you’ll learn about these concepts in more detail.
Chapter 3 covers concepts that most programming languages have, such as
variables, data types, and functions, and shows how to use them in Rust.
Chapter 4 explores ownership, which is a Rust feature that is most different
from other languages. Chapter 5 discusses structs and method syntax, and
Chapter 6 endeavors to explain enums.
