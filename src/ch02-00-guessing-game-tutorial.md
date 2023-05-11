# Programando un juego de predicciones

¡Vamos a sumergirnos en Rust trabajando en un proyecto práctico juntos! Este
capítulo te introducirá algunos conceptos de Rust enseñándote cómo usarlos en
un programa real. Aprenderás sobre `let`, `match`, métodos, funciones asociadas,
utilizar cajas externas y ¡mucho más! Los siguientes capítulos explicarán estas
ideas en más detalle. En este capítulo, practicarás las bases.

Implementaremos un problema de programación para principiantes clásico. Funciona
así: el programa generará un entero aleatorio entre 1 y 100. Luego preguntará al
usuario para introducir una predicción. Tras introducirla, el programa indicará
si la predicción fue muy alta o muy baja. Si la predicción fue correcta, el 
juego imprimirá un mensaje de felicitación y terminará.

## Configurando un nuevo proyecto

Para configurar un nuevo proyecto, ve a la carpeta de *proyectos* que creaste 
en el capítulo 1 y haz un nuevo proyecto utilizando Cargo, por ejemplo:

```console
$ cargo new juego_predicciones
$ cd juego_predicciones
```

El primer comando, `cargo new`, utiliza el nombre del proyecto (`juego_predicciones`)
cómo el primer argumento. El segundo comando cambia la ubicación de la 
consola al directorio del proyecto

Veamos el archivo *Cargo.toml* que se ha generado:

<span class="filename">Nombre del archivo: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```

Si la información del autor que Cargo ha obtenido de tu entorno no es 
correcta, arréglalo en el archivo y guárdalo

Cómo viste en el capítulo 1, `cargo new` genera un “!Hola, mundo!” para tí.
Comprueba el archivo *src/main.rs*:

<span class="filename">Nombre del archivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

Ahora vamos a compilar este programa de “¡Hola, mundo!” y ejecutémoslo en el
mismo paso utilizando `cargo run`:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```

El comando `run` es muy útil cuando necesitas iterar rápidamente en un proyecto,
tal y cómo vamos a hacer en este juego, testeando rápidamente cada iteración
antes de ir a la siguiente.

Vuelve a abrir el archivo *src/main.rs*. Vas a escribir todo el código en este
archivo.

## Procesando una predicción

La primera parte del juego de predicciones será preguntar al usuario por ella,
procesarla y comprobar que la respuesta tiene el formato adecuado. Para empezar,
permitiremos al jugador introducir una predicción. Escribe el código del listado
2-1 en *src/main.rs*.

<span class="filename">Nombre del archivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```

<span class="caption">Listado 2-1: Código que obtiene una predicción del usuario
y la imprime</span>

Este código tiene mucha información, así que vamos línea por línea. Para obtener
la entrada del usuario y luego imprimir el resultado, necesitamos importar la
librería `io` (input/output, en español, entrada/salida) en este ámbito. La 
libería `io` viene de la librería estándar (que es cónocida cómo `std`):

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```

Por defecto, Rust trae solo unos pocos tipos al ámbito en [el *preludio*][prelude] <!--ignore-->.
Si un tipo que quieres no está en el preludio, tienes que traer ese tipo al
ámbito de manera explícita con la declaración `use`. Utilizar la librería
`std::io` te da numerosas características útiles, incluyendo la habilidad
para aceptar entrada del usuario.

[prelude]: ../std/prelude/index.html

Cómo viste en el capítulo 1, la función `main` es el punto de entrada al
programa:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

La sintaxis `fn` declara una nueva función, los parentesis `()`, indican que no
hay parámetros, y la llave `{`, inicia el cuerpo de la función.

Cómo también aprendiste en el capítulo 1, `println!` es una macro que imprime una
cadena de texto en pantalla:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```

Este código está imprimiendo una solicitud diciendo que es el juego y solicitando
entrada por parte del usuario.

### Almacenando valores con variables

Ahora, crearemos un lugar dónde almacenar la entrada del usuario, cómo este:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```

¡Ahora el programa se está poniendo interesante! Hay mucho ocurriendo en está
pequeña línea. Nota cómo esto esto es una declaración `let`, que se utiliza para
crear una *variable*. Aquí otro ejemplo:

```rust,ignore
let foo = bar;
```

Esta línea crea una nueva variable llamada `foo` y la une al valor de la variable
`bar`. En Rust, la variables son inmutables por defecto. Trataremos esto en más
detalle en la sección del capítulo 3 ["Variables y mutabilidad"][variables-and-mutability]
<!--ignore-->. El siguiente ejemplo muestra cómo utilizar `mut` antes del nombre 
de la variable para hacer una variable mutable:

```rust,ignore
let foo = 5; // inmutable
let mut bar = 5; // mutable
```
> Nota: La sintaxis `//` comienza un comentario hasta el final de la línea.
> Rust ignora todo lo que haya en comentario, que se discutirán en más detalle
> en el capítulo 3.

Ahora volvamos al programa del juego de predicciones. Ahora ya sabes que
`let mut prediccion` introducirá en el programa una variable mutable llamada
`guess`. En el otro lado de la igualdad (`=`) está el valor al que `guess` está
atado, que es el resultado de llamar a `String::new`, una función que devuelve
una nueva instancia de `String` (cadena de texto, en español). [`String`][string]<!--ignore-->
es un tipo de cadena de texto que da la librería estándar que es un trocito de
texto expandible y codificado en UTF-8

[string]: ../std/string/struct.String.html

La sintaxis `::` en `::new` indica que `new` es una *función asociada* al tipo
`String`. Una función asociada es implementada en un tipo, en este caso `String`,
en vez de en una instancia en particular de `String`. Algunos lenguajes llaman a
esto un *método estático*.

Esta función `new` crea una cadena de texto nueva y vacía. Encontrarás una 
función `new` en muchos tipos, porque es un nombre común para una función que
crea un nuevo valor de algún tipo. 

Pra resumir, la línea `let mut guess = String::new();` a creado una variable
mutable que está asignada a una nueva instancia vacía de `String`. ¡Guau!

Recuerda que hemos incluido la funcionalidad de la entrada/salida de la librería
estándar con `use std::io;` en la primera línea del programa. Ahora llamaremos a
la función `stdin` desde el módulo `io`:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

Si no hubieramos puesto la línea `use std::io` en el principio del programa,
podríamos haber escrito la llamada a la función cómo `std::io::stdin`. La 
función `stdin` devuelve una instancia de [`std::io::Stdin`][iostdin] <!--ignore-->,
que es un tipo que representa una manilla a la entrada estándar para tu terminal.

[iostdin]: ../std/io/struct.Stdin.html

La siguiente parte del código, `.read_line(&mut guess)`, llama
al método ['read_line`][read_line]<!--ignore--> en la manilla de la entrada
estándar para conseguir entrada por parte del usuario. También estamos pasando
un argumento a `read_line`: `&mut guess`.

[read_line]: ../std/io/struct.Stdin.html#method.read_line

El trabajo de `read_line` es capturar lo que el usuario escriba en la entrada
estándar y añadirlo en una cadena de texto (sin sobreescribir sus contenidos),
así que toma una cadena de texto cómo un argumento. El argumento de la cadena
de texto necesita ser mutable para el que método pueda cambiar el contenido de
la cadena añadiendo la entrada del usuario.

El `&` indica que este argumento es una *referencia*, lo que permite que varias
partes de tu código accedan a la misma información sin necesitar copiar esta 
información en memoria múltiples veces. Las refencias son una característica
compleja, y una de las mayores ventajas de Rust en cómo de seguro y sencillo es
utilizar referencias. No necesitar saber muchos detalles para terminar este programa.
Por ahora, todo lo que necesitas saber es que cómo las variables, las referencias
son inmutables por defecto. Por lo tanto, tienes que escribir `&mut guess` en vez
de `&guess` para hacerlo mutable. (En el capítulo 4 se explicarán las referencias
con mayor profundidad).

### Manejar posibles fallos con el tipo `Result`

Todavía estamos trabajando en la misma línea de código. Aunque estemos discutiendo
una tercera línea de texto, todavía es parte de la misma línea lógica de código. La
siguiente parte este mmétodo:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```

Cuándo llamas al método con la sinxtaxis `.foo()`, puede ser una decisión sabia
introducir una nueva línea y un espacio en blanco para poder romper líneas largas.
Podríamos haber escrito este código de la siguiente manera:

```rust,ignore
io::stdin().read_line(&mut prediccion).expect("Lectura de la línea fallida");
```

Sin embargo, una línea larga es difícil de leer, así que es mejor dividirla. Ahora,
hablemos de lo que hace esta línea.

Cómo mencionamos antes, `read_line` pone lo que haya escrito el usuario en
la cadena de texto que le hayamos pasado, pero también retorna un valor en
este caso, un [`io::Result`][ioresult]<!--ignore-->. Rust tiene un número
de tipos llamados `Result` en su librería estándar: un [`Result`][result]<!--ignore-->
así cómo versiones específicas para submódulos, cómo `io::Result`.

[ioresult]: ../std/io/type.Result.html
[result]: ../std/result/enum.Result.html

Los tipos `Result` son [*enumeraciones*][enums]<!--ignore-->, comúnmente llamadas
cómo *enums*. Una enumeración es un tipo que puede tener un conjunto fijo de 
valores, y todos esos valores se llaman las *variantes* de la enum. El capítulo 6
cubrirá las enums en más detalle.

[enums]: ch06-00-enums.html

Para `Result`, las variantes son `Ok` o `Err`. La variante `Ok` indica que la
operación fue exitosa, y dentro de `Ok` está el valor generado. La variante
`Err` significa que la operación ha fallado, y `Err` contiene la información
sobre cómo o por qué la operación falló.

El propósito de estos tipos `Result` es codificar la información para el manejo
de los errores. Los valores del tipo `Result`, cómo los valores de cualquier tipo,
tienen métodos definidos en ellos. Una instancia de `io::Result` tiene un 
[método `expect`][expect]<!--ignore--> que puedes llamar. Si esta instancia de 
`io::Result` es un valor `Err`, `expect` hará que el programa se estrelle y
mostrará el mensaje que pasaste cómo argumento. Si el método `read_line`
devuelve un valor `Err`, es probable que el error venga del sistema operativo
subyacente. Si esta instancia de `io::Result` es un valor `Ok`, `expect` tomará el
valora que `Ok` está almacenando y devolverá ese valor para que puedas utilizarlo.
En este caso, el valor es el número de bytes que en los que el usuario introdujo en
la entrada estándar. 

[expect]: ../std/result/enum.Result.html#method.expect

Si no llamases a `expect`, el programa compilaría, pero tendrías una advertencia:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```

Rust advierto que no has usado el valor `Result` devuelto desde `read_line`,
indicando que el programa no ha manejado un posible error.

La manera correcta de quitar esta advertencia es manejar el error de verdad, pero
cómo simplemente quieres que tu programa se estrelle cuando ocurra un problema, 
puedes usar `expect`. Aprenderás sobre recuperación de errores en el capítulo 9.

### Printing Values with `println!` Placeholders

Aside from the closing curly bracket, there’s only one more line to discuss in
the code added so far, which is the following:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

This line prints the string we saved the user’s input in. The set of curly
brackets, `{}`, is a placeholder: think of `{}` as little crab pincers that
hold a value in place. You can print more than one value using curly brackets:
the first set of curly brackets holds the first value listed after the format
string, the second set holds the second value, and so on. Printing multiple
values in one call to `println!` would look like this:

```rust
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```

This code would print `x = 5 and y = 10`.

### Testing the First Part

Let’s test the first part of the guessing game. Run it using `cargo run`:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-01/
cargo clean
cargo run
input 6 -->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```

At this point, the first part of the game is done: we’re getting input from the
keyboard and then printing it.

## Generating a Secret Number

Next, we need to generate a secret number that the user will try to guess. The
secret number should be different every time so the game is fun to play more
than once. Let’s use a random number between 1 and 100 so the game isn’t too
difficult. Rust doesn’t yet include random number functionality in its standard
library. However, the Rust team does provide a [`rand` crate][randcrate].

[randcrate]: https://crates.io/crates/rand

### Using a Crate to Get More Functionality

Remember that a crate is a collection of Rust source code files.
The project we’ve been building is a *binary crate*, which is an executable.
The `rand` crate is a *library crate*, which contains code intended to be
used in other programs.

Cargo’s use of external crates is where it really shines. Before we can write
code that uses `rand`, we need to modify the *Cargo.toml* file to include the
`rand` crate as a dependency. Open that file now and add the following line to
the bottom beneath the `[dependencies]` section header that Cargo created for
you. Be sure to specify `rand` exactly as we have here, or the code examples in
this tutorial may not work.

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

In the *Cargo.toml* file, everything that follows a header is part of a section
that continues until another section starts. The `[dependencies]` section is
where you tell Cargo which external crates your project depends on and which
versions of those crates you require. In this case, we’ll specify the `rand`
crate with the semantic version specifier `0.8.3`. Cargo understands [Semantic
Versioning][semver]<!-- ignore --> (sometimes called *SemVer*), which is a
standard for writing version numbers. The number `0.8.3` is actually shorthand
for `^0.8.3`, which means any version that is at least `0.8.3` but below
`0.9.0`. Cargo considers these versions to have public APIs compatible with
version `0.8.3`, and this specification ensures you'll get the latest patch
release that will still compile with the code in this chapter. Any version
`0.9.0` or greater is not guaranteed to have the same API as what the following
examples use.

[semver]: http://semver.org

Now, without changing any of the code, let’s build the project, as shown in
Listing 2-2.

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo clean
cargo build -->

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.3
  Downloaded libc v0.2.86
  Downloaded getrandom v0.2.2
  Downloaded cfg-if v1.0.0
  Downloaded ppv-lite86 v0.2.10
  Downloaded rand_chacha v0.3.0
  Downloaded rand_core v0.6.2
   Compiling rand_core v0.6.2
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.3
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
```

<span class="caption">Listing 2-2: The output from running `cargo build` after
adding the rand crate as a dependency</span>

You may see different version numbers (but they will all be compatible with
the code, thanks to SemVer!), different lines (depending on the operating
system), and the lines may be in a different order.

Now that we have an external dependency, Cargo fetches the latest versions of
everything from the *registry*, which is a copy of data from
[Crates.io][cratesio]. Crates.io is where people in the Rust ecosystem post
their open source Rust projects for others to use.

[cratesio]: https://crates.io/

After updating the registry, Cargo checks the `[dependencies]` section and
downloads any crates you don’t have yet. In this case, although we only listed
`rand` as a dependency, Cargo also grabbed other crates that `rand` depends on
to work. After downloading the crates, Rust compiles them and then compiles the
project with the dependencies available.

If you immediately run `cargo build` again without making any changes, you
won’t get any output aside from the `Finished` line. Cargo knows it has already
downloaded and compiled the dependencies, and you haven’t changed anything
about them in your *Cargo.toml* file. Cargo also knows that you haven’t changed
anything about your code, so it doesn’t recompile that either. With nothing to
do, it simply exits.

If you open up the *src/main.rs* file, make a trivial change, and then save it
and build again, you’ll only see two lines of output:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
touch src/main.rs
cargo build -->

```console
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
```

These lines show Cargo only updates the build with your tiny change to the
*src/main.rs* file. Your dependencies haven’t changed, so Cargo knows it can
reuse what it has already downloaded and compiled for those. It just rebuilds
your part of the code.

#### Ensuring Reproducible Builds with the *Cargo.lock* File

Cargo has a mechanism that ensures you can rebuild the same artifact every time
you or anyone else builds your code: Cargo will use only the versions of the
dependencies you specified until you indicate otherwise. For example, what
happens if next week version 0.8.4 of the `rand` crate comes out and
contains an important bug fix but also contains a regression that will break
your code?

The answer to this problem is the *Cargo.lock* file, which was created the
first time you ran `cargo build` and is now in your *guessing_game* directory.
When you build a project for the first time, Cargo figures out all the
versions of the dependencies that fit the criteria and then writes them to
the *Cargo.lock* file. When you build your project in the future, Cargo will
see that the *Cargo.lock* file exists and use the versions specified there
rather than doing all the work of figuring out versions again. This lets you
have a reproducible build automatically. In other words, your project will
remain at `0.8.3` until you explicitly upgrade, thanks to the *Cargo.lock*
file.

#### Updating a Crate to Get a New Version

When you *do* want to update a crate, Cargo provides another command, `update`,
which will ignore the *Cargo.lock* file and figure out all the latest versions
that fit your specifications in *Cargo.toml*. If that works, Cargo will write
those versions to the *Cargo.lock* file.

But by default, Cargo will only look for versions greater than `0.8.3` and less
than `0.9.0`. If the `rand` crate has released two new versions, `0.8.4` and
`0.9.0`, you would see the following if you ran `cargo update`:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo update
assuming there is a new 0.8.x version of rand; otherwise use another update
as a guide to creating the hypothetical output shown here -->

```console
$ cargo update
    Updating crates.io index
    Updating rand v0.8.3 -> v0.8.4
```

At this point, you would also notice a change in your *Cargo.lock* file noting
that the version of the `rand` crate you are now using is `0.8.4`.

If you wanted to use `rand` version `0.9.0` or any version in the `0.9.x`
series, you’d have to update the *Cargo.toml* file to look like this instead:

```toml
[dependencies]
rand = "0.9.0"
```

The next time you run `cargo build`, Cargo will update the registry of crates
available and reevaluate your `rand` requirements according to the new version
you have specified.

There’s a lot more to say about [Cargo][doccargo]<!-- ignore --> and [its
ecosystem][doccratesio]<!-- ignore --> which we’ll discuss in Chapter 14, but
for now, that’s all you need to know. Cargo makes it very easy to reuse
libraries, so Rustaceans are able to write smaller projects that are assembled
from a number of packages.

[doccargo]: http://doc.crates.io
[doccratesio]: http://doc.crates.io/crates-io.html

### Generating a Random Number

Now that you’ve added the `rand` crate to *Cargo.toml*, let’s start using
`rand`. The next step is to update *src/main.rs*, as shown in Listing 2-3.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

<span class="caption">Listing 2-3: Adding code to generate a random
number</span>

First, we add a `use` line: `use rand::Rng`. The `Rng` trait defines
methods that random number generators implement, and this trait must be in
scope for us to use those methods. Chapter 10 will cover traits in detail.

Next, we’re adding two lines in the middle. The `rand::thread_rng` function
will give us the particular random number generator that we’re going to use:
one that is local to the current thread of execution and seeded by the
operating system. Then we call the `gen_range` method on the random number
generator. This method is defined by the `Rng` trait that we brought into scope
with the `use rand::Rng` statement. The `gen_range` method takes a range
expression as an argument and generates a random number in the range. The kind
of range expression we’re using here takes the form `start..end`. It’s
inclusive on the lower bound but exclusive on the upper bound, so we need to
specify `1..101` to request a number between 1 and 100. Alternatively, we could
pass the range `1..=100`, which is equivalent.

> Note: You won’t just know which traits to use and which methods and functions
> to call from a crate. Instructions for using a crate are in each crate’s
> documentation. Another neat feature of Cargo is that you can run the `cargo
> doc --open` command, which will build documentation provided by all of your
> dependencies locally and open it in your browser. If you’re interested in
> other functionality in the `rand` crate, for example, run `cargo doc --open`
> and click `rand` in the sidebar on the left.

The second line that we added to the middle of the code prints the secret
number. This is useful while we’re developing the program to be able to test
it, but we’ll delete it from the final version. It’s not much of a game if the
program prints the answer as soon as it starts!

Try running the program a few times:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-03/
cargo run
4
cargo run
5
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
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

Now that we have user input and a random number, we can compare them. That step
is shown in Listing 2-4. Note that this code won’t compile quite yet, as we
will explain.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

<span class="caption">Listing 2-4: Handling the possible return values of
comparing two numbers</span>

The first new bit here is another `use` statement, bringing a type called
`std::cmp::Ordering` into scope from the standard library. Like `Result`,
`Ordering` is another enum, but the variants for `Ordering` are `Less`,
`Greater`, and `Equal`. These are the three outcomes that are possible when you
compare two values.

Then we add five new lines at the bottom that use the `Ordering` type. The
`cmp` method compares two values and can be called on anything that can be
compared. It takes a reference to whatever you want to compare with: here it’s
comparing the `guess` to the `secret_number`. Then it returns a variant of the
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
code might encounter and make sure that you handle them all. These features
will be covered in detail in Chapter 6 and Chapter 18, respectively.

Let’s walk through an example of what would happen with the `match` expression
used here. Say that the user has guessed 50 and the randomly generated secret
number this time is 38. When the code compares 50 to 38, the `cmp` method will
return `Ordering::Greater`, because 50 is greater than 38. The `match`
expression gets the `Ordering::Greater` value and starts checking each arm’s
pattern. It looks at the first arm’s pattern, `Ordering::Less`, and sees that
the value `Ordering::Greater` does not match `Ordering::Less`, so it ignores
the code in that arm and moves to the next arm. The next arm’s pattern,
`Ordering::Greater`, *does* match `Ordering::Greater`! The associated code in
that arm will execute and print `Too big!` to the screen. The `match`
expression ends because it has no need to look at the last arm in this scenario.

However, the code in Listing 2-4 won’t compile yet. Let’s try it:

```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

The core of the error states that there are *mismatched types*. Rust has a
strong, static type system. However, it also has type inference. When we wrote
`let mut guess = String::new()`, Rust was able to infer that `guess` should be
a `String` and didn’t make us write the type. The `secret_number`, on the other
hand, is a number type. A few number types can have a value between 1 and 100:
`i32`, a 32-bit number; `u32`, an unsigned 32-bit number; `i64`, a 64-bit
number; as well as others. Rust defaults to an `i32`, which is the type of
`secret_number` unless you add type information elsewhere that would cause Rust
to infer a different numerical type. The reason for the error is that Rust
cannot compare a string and a number type.

Ultimately, we want to convert the `String` the program reads as input into a
real number type so we can compare it numerically to the secret number. We can
do that by adding another line to the `main` function body:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

The line is:

```rust,ignore
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

We create a variable named `guess`. But wait, doesn’t the program already have
a variable named `guess`? It does, but Rust allows us to *shadow* the previous
value of `guess` with a new one. This feature is often used in situations in
which you want to convert a value from one type to another type. Shadowing lets
us reuse the `guess` variable name rather than forcing us to create two unique
variables, such as `guess_str` and `guess` for example. (Chapter 3 covers
shadowing in more detail.)

We bind `guess` to the expression `guess.trim().parse()`. The `guess` in the
expression refers to the original `guess` that was a `String` with the input in
it. The `trim` method on a `String` instance will eliminate any whitespace at
the beginning and end. Although `u32` can contain only numerical characters,
the user must press <span class="keystroke">enter</span> to satisfy
`read_line`. When the user presses <span class="keystroke">enter</span>, a
newline character is added to the string. For example, if the user types <span
class="keystroke">5</span> and presses <span class="keystroke">enter</span>,
`guess` looks like this: `5\n`. The `\n` represents “newline,” the result of
pressing <span class="keystroke">enter</span> (On Windows, pressing <span
class="keystroke">enter</span> results in a carriage return and a newline,
`\r\n`). The `trim` method eliminates `\n` or `\r\n`, resulting in just `5`.

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

[parse]: ../std/primitive.str.html#method.parse

The call to `parse` could easily cause an error. If, for example, the string
contained `A👍%`, there would be no way to convert that to a number. Because it
might fail, the `parse` method returns a `Result` type, much as the `read_line`
method does (discussed earlier in [“Handling Potential Failure with the
`Result` Type”](#handling-potential-failure-with-the-result-type)<!-- ignore
-->). We’ll treat this `Result` the same way by using the `expect` method
again. If `parse` returns an `Err` `Result` variant because it couldn’t create
a number from the string, the `expect` call will crash the game and print the
message we give it. If `parse` can successfully convert the string to a number,
it will return the `Ok` variant of `Result`, and `expect` will return the
number that we want from the `Ok` value.

Let’s run the program now!

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/
cargo run
  76
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/guessing_game`
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

The `loop` keyword creates an infinite loop. We’ll add that now to give users
more chances at guessing the number:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

As you can see, we’ve moved everything into a loop from the guess input prompt
onward. Be sure to indent the lines inside the loop another four spaces each
and run the program again. Notice that there is a new problem because the
program is doing exactly what we told it to do: ask for another guess forever!
It doesn’t seem like the user can quit!

The user could always interrupt the program by using the keyboard shortcut <span
class="keystroke">ctrl-c</span>. But there’s another way to escape this
insatiable monster, as mentioned in the `parse` discussion in [“Comparing the
Guess to the Secret Number”](#comparing-the-guess-to-the-secret-number)<!--
ignore -->: if the user enters a non-number answer, the program will crash. The
user can take advantage of that in order to quit, as shown here:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-04-looping/
cargo run
(too small guess)
(too big guess)
(correct guess)
quit
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/guessing_game`
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
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/main.rs:28:47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Typing `quit` actually quits the game, but so will any other non-number input.
However, this is suboptimal to say the least. We want the game to automatically
stop when the correct number is guessed.

### Quitting After a Correct Guess

Let’s program the game to quit when the user wins by adding a `break` statement:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

Adding the `break` line after `You win!` makes the program exit the loop when
the user guesses the secret number correctly. Exiting the loop also means
exiting the program, because the loop is the last part of `main`.

### Handling Invalid Input

To further refine the game’s behavior, rather than crashing the program when
the user inputs a non-number, let’s make the game ignore a non-number so the
user can continue guessing. We can do that by altering the line where `guess`
is converted from a `String` to a `u32`, as shown in Listing 2-5.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

<span class="caption">Listing 2-5: Ignoring a non-number guess and asking for
another guess instead of crashing the program</span>

Switching from an `expect` call to a `match` expression is how you generally
move from crashing on an error to handling the error. Remember that `parse`
returns a `Result` type and `Result` is an enum that has the variants `Ok` or
`Err`. We’re using a `match` expression here, as we did with the `Ordering`
result of the `cmp` method.

If `parse` is able to successfully turn the string into a number, it will
return an `Ok` value that contains the resulting number. That `Ok` value will
match the first arm’s pattern, and the `match` expression will just return the
`num` value that `parse` produced and put inside the `Ok` value. That number
will end up right where we want it in the new `guess` variable we’re creating.

If `parse` is *not* able to turn the string into a number, it will return an
`Err` value that contains more information about the error. The `Err` value
does not match the `Ok(num)` pattern in the first `match` arm, but it does
match the `Err(_)` pattern in the second arm. The underscore, `_`, is a
catchall value; in this example, we’re saying we want to match all `Err`
values, no matter what information they have inside them. So the program will
execute the second arm’s code, `continue`, which tells the program to go to the
next iteration of the `loop` and ask for another guess. So, effectively, the
program ignores all errors that `parse` might encounter!

Now everything in the program should work as expected. Let’s try it:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-05/
cargo run
(too small guess)
(too big guess)
foo
(correct guess)
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 4.45s
     Running `target/debug/guessing_game`
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

Awesome! With one tiny final tweak, we will finish the guessing game. Recall
that the program is still printing the secret number. That worked well for
testing, but it ruins the game. Let’s delete the `println!` that outputs the
secret number. Listing 2-6 shows the final code.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

<span class="caption">Listing 2-6: Complete guessing game code</span>

## Summary

At this point, you’ve successfully built the guessing game. Congratulations!

This project was a hands-on way to introduce you to many new Rust concepts:
`let`, `match`, methods, associated functions, the use of external crates, and
more. In the next few chapters, you’ll learn about these concepts in more
detail. Chapter 3 covers concepts that most programming languages have, such as
variables, data types, and functions, and shows how to use them in Rust.
Chapter 4 explores ownership, a feature that makes Rust different from other
languages. Chapter 5 discusses structs and method syntax, and Chapter 6
explains how enums work.

[variables-and-mutability]:
ch03-01-variables-and-mutability.html#variables-and-mutability
