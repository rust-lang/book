## ¡Hola, Mundo!

Ahora que tiene Rust instalado, comencemos escribiendo su primer programa Rust.
Es una tradición que al aprender un nuevo lenguaje se escriba un pequeño programa
para imprimir el texto "Hello, world!" (¡Hola, mundo!) por pantalla, y en esta
sección, seguiremos esa tradición.

> Nota: Este libro asume familiarización básica con la línea de comandos. Rust
> no hace demandas sobre su edición, herramientas usadas, o en donde usted guarde su
> código, así que si prefiere un IDE en vez de la línea de comandos, siéntase libre de
> usar su IDE favorito. Muchos IDEs ahora tienen cierto grado de soporte para Rust;
> revise la documentación del IDE para más detalles. Permitiendo gran soporte de
> IDE ha sido de gran atención recientemente para el equipo de Rust, y el progreso
> se ha llevado a cabo rápidamente!

### Creando un Directorio de Proyecto

Primero, crée un directorio donde colocar su código Rust. A Rust no le importa
donde usted guarde su código, pero para este libro, sugerimos crear un directorio
*projects* (proyectos) en su directorio hogar y mantener sus proyectos ahí. Abra
un terminal e introduzca los siguientes comandos para crear un directorio y un
directorio dentro de este para el proyecto “Hello, world!”:

Linux y Mac:

```text
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

CMD de Windows:

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

PowerShell de Windows:

```powershell
> mkdir $env:USERPROFILE\projects
> cd $env:USERPROFILE\projects
> mkdir hello_world
> cd hello_world
```

### Escribiendo y Ejecutando un Programa de Rust

Luego, cree un nuevo archivo fuente y llámelo *main.rs*. Los archivos Rust siempre
terminan con la extensión *.rs*. Si está usando más de una palabra para el nombre
del archivo, use un piso para separarlas. Por ejemplo, usaría *hello_world.rs* en
vez de *helloworlds.rs*.

Ahora abra el archivo *main.rs* que acaba de crear, y escriba el código mostrado en
Listado 1-1:


<span class="filename">Filename: main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

<span class="caption">Listing 1-1: A program that prints “Hello, world!”</span>

Guarde el archivo, y vuelva a la ventana del terminal. En Linux o en OSX, introduzca
los siguientes comandos:


```text
$ rustc main.rs
$ ./main
Hello, world!
```

En Windows, ejecute `.\main.exe` en vez de `./main`.

```powershell
> rustc main.rs
> .\main.exe
Hello, world!
```

Sin importar su sistema
operativo, verá la cadena `Hello, world!` impresa en el terminal. Si eso ve,
¡Felicidades! Usted ha oficialmente escrito un programa en Rust. ¡Eso lo hace
un programador Rust! ¡Bienvenido!

### Anatomía de un Programa en Rust

Ahora, vamos a revisar lo que acaba de pasar en su programa "Hello, world!"
en detalle. Esta es la primera pieza del rompecabezas:


```rust
fn main() {

}
```

Estas líneas definen una *function* (función) en Rust. La función `main` es
especial: es la primera cosa que se ejecuta en cada programa Rust ejecutable.
La primera línea dice, "Estoy declarando una función llamada `main` que no
tiene parámetros y no retorna nada." Si hay parámetros, sus nombres irían
dentro de los paréntesis, `(` y `)`.

También note que el cuerpo de la función está dentro de llaves, `{` y `}`.
Rust requiere estas alrededor de todos los cuerpos de las funciones. Es
considerado un buen estilo poner la abertura de las llaves en la misma línea
de la declaración de la función, con un espacio de por medio


> Durante la escritura, un formateador automático, `rustfmt`, está en
> desarrollo. Si usted quiere apegarse a un estilo estándar a lo largo
> de los proyectos Rust, `rustfmt` es una herramienta que formateará
> su código con un estilo particular. El plan es eventualmente incluirlo
> con la distribución estándar de Rust, así como `rustc`, entonces dependiendo
> de cuándo usted lea este libro, !Podría ya haberlo instalado! Revise la
> documentación en línea para más detalles.

Dentro de la función `main`:, tenemos este código:

```rust
    println!("Hello, world!");
```

Esta línea hace todo el trabajo en este pequeño programa: ella imprime
por pantalla. Hay algunos detalles a notar aquí. El primero es que el
estilo de Rust es para indentar con cuatro espacios, no con un tab
(tabulador).

La segunda parte importante es `println!`. Esto llama a un *macro* de Rust,
que es cómo se hace la metaprogramación en Rust. Si llamara a una función,
se vería así: `println` (sin el `!`). Discutiremos los macros de Rust con
más detalle en Appendix E, pero por ahora sólo debe saber que cuando vea
un `!` significa que usted está llamando a un macro en vez de a una función
normal.


> ### Porqué `println!` es un Macro
>
> Hay múltiples razones por las cuales `println!` es un macro en vez de
> una función, y aún no hemos explicado Rust, así que no es exactamente
> obvio. Aquí están las razones:
>
> * La cadena pasada a `println!` puede tener especificadores de formateo
>   en ella, y esos se revisan en tiempo de compilación.
> * Las funciones Rust sólo pueden tener un número predeterminado de argumentos
>   pero `println!` (y los macros en general) pueden tomar un número variable.
> * Los especificadores de formateo pueden tener argumentos nombrados, a
>   diferencia de la funciones de Rust que no pueden tenerlos.
> * Toma implícitamente sus argumentos por referencia incluso cuando son pasados
>   por valor.
>
> Si nada de esto tiene sentido, no se preocupe. Luego cubriremos estos conceptos con
> más detalle.

Luego viene `"Hello, world!"` el cual es un *string* (cadena). Pasamos esta cadena
como argumento a `println!`, el cual imprime la cadena por pantalla. ¡Bastante
facil!

La línea termina con un punto y coma (`;`). El `;` indica que esta expresión
acabó, y que la próxima está lista para comenzar. La mayoría de las líneas del
código de Rust terminan con `;`.

### Compilar y Ejecutar Son Pasos Diferentes

En "Escribiendo y Ejecutando un Programa de Rust", mostramos cómo ejecutar
un programa recién creado. Ahora vamos a descomponer este proceso para examinar
cada paso.

Antes de ejecutar el programa de Rust, usted tiene que compilarlo. Puede usar
el compilador de Rust introduciendo el comando `rustc` y pasando el nombre de
su archivo fuente, así:


```text
$ rustc main.rs
```

Si usted viene de usar C o C++, notará que esto es similar a `gcc` o `clang`.
Luego de compilar con éxito, Rust debería de producir un ejecutable en binario.


En Linux, Mac, y PowerShell en Windows, puede ver el ejecutable introduciendo
el comando `ls` en su shell así:

```text
$ ls
main  main.rs
```

En CMD de Windows, usted introduciría:

```cmd
> dir /B %= the /B option says to only show the file names =%
main.exe
main.pdb
main.rs
```

Esto muestra que tenemos dos archivos: el código fuente, con la extensión *.rs*,
y el ejecutable (*main.exe* en Windows, *main* en los demás). Lo que queda por
hacer aquí es ejectuar el archivo *main* o *main.exe*, así:


```text
$ ./main  # or .\main.exe on Windows
```

Si su programa "Hello, wordl!" fuese *main.rs*, imprimiría `Hello, world!` en
su terminal.

Si usted viene de lenguajes dinámicos como Ruby, Python, o JavaScript, podría
no estar acostumbrado a que compilar y ejecutar sean dos pasos diferentes. Rust
es un lenguaje *compilado desde-antes*, lo que significa que usted puede compilar
un programa, dárselo a alguien más, y esa persona puede ejecutarlo aunque no
tenga Rust instalado. Si usted le da un archivo `.rb`, `.py`, o `.js` a otra
persona, por otro lado, necesitará tener una implementación de Ruby, Python, o
JavaScript instalada (respectivamente), pero usted sólo necesita un comando para
compilar y ejecutar su programa. En el diseño de lenguajes, todo es un balance.

Sólo con compilar con `rustc` está bien para programas simples, pero a medida que
su programa crece, usted querrá poder manejar todas las opciones que su proyecto
tenga y tener facilidad de compartir su código con otras personas y proyectos.
A continuación, le presentaremos una herramienta llamada Cargo, la cual le ayudará
a escribir programas en Rust del mundo real.

## ¡Hola, Cargo!


Cargo es el sistema de construcción de Rust y administrador de paquetes, y los Rustaceanos
usan Cargo para administrar sus proyectos de Rust ya que hace muchas tareas más
fáciles. Por ejemplo, Cargo se encarga de construir su código, descargar las librerías
de las cuales depende su código, y de construir esas librerías. Llamamos *dependencies*
(dependencias) a las librerías que su código necesita.

Los programas en Rust más simples, como el que hemos escrito hasta ahora, no tiene
ninguna dependencia, así que ahora, usted sólo usaría la parte de Cargo que se
encarga de construir su código. A medida que escribe códigos más complejos en
Rust, usted querrá añadir dependencias, y si empieza usando Cargo, eso será mucho
más fácil.

Como la gran, gran mayoría de proyectos en Rust usan Cargo, asumiremos que usted lo
estará usando por el resto del libro. Cargo viene instalado con Rust, si usted usó
los instaladores oficiales que fueron cubiertos en el capítulo de Instalación.
Si instaló Rust de alguna otra forma, puede comprobar si tiene Cargo instalado
escribiendo lo siguiente en su terminal:


```text
$ cargo --version
```

Si ve un número de versión, ¡Entonces perfecto! Si ve un error como `command not found`
(comando no encontrado), entonces tendrá que revisar la documentación para su
método de instalación para determinar cómo instalar Cargo por separado.

### Creando un Proyecto con Cargo

Vamos a crear un nuevo proyecto usando Cargo y ver cómo difiere con nuestro proyecto
en `hello_world`. Vuelva a su directorio de proyectos (o donde sea que decidió
almacenar su código):


Linux, Mac, and PowerShell:

```text
$ cd ~/projects
```

CMD for Windows:

```cmd
> cd \d "%USERPROFILE%\projects"
```

And then on any operating system run:

```text
$ cargo new hello_cargo --bin
$ cd hello_cargo
```

We passed the `--bin` argument to `cargo new` because our goal is to make an
executable application, as opposed to a library. Executables are binary
executable files often called just *binaries*. We’ve given `hello_cargo` as the
name for our project, and Cargo creates its files in a directory of the same
name that we can then go into.

If we list the files in the *hello_cargo* directory, we can see that Cargo has
generated two files and one directory for us: a *Cargo.toml* and a *src*
directory with a *main.rs* file inside. It has also initialized a new git
repository in the *hello_cargo* directory for us, along with a *.gitignore*
file. Git is a common version control system. You can change `cargo new` to use
a different version control system, or no version control system, by using the
`--vcs` flag. Run `cargo new --help` to see the available options.

Open up *Cargo.toml* in your text editor of choice. It should look similar to
the code in Listing 1-2:

<span class="filename">Filename: Cargo.toml</span>

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
```

<span class="caption">Listing 1-2: Contents of *Cargo.toml* generated by `cargo
new`</span>

This file is in the [*TOML*][toml]<!-- ignore --> (Tom’s Obvious, Minimal
Language) format. TOML is used as Cargo’s configuration format.

[toml]: https://github.com/toml-lang/toml

The first line, `[package]`, is a section heading that indicates that the
following statements are configuring a package. As we add more information to
this file, we’ll add other sections.

The next three lines set the three bits of configuration that Cargo needs to
see in order to know that it should compile your program: its name, what
version it is, and who wrote it. Cargo gets your name and email information
from your environment. If it’s not correct, go ahead and fix that and save the
file.

The last line, `[dependencies]`, is the start of a section for you to list any
*crates* (which is what we call packages of Rust code) that your project will
depend on so that Cargo knows to download and compile those too. We won’t need
any other crates for this project, but we will in the guessing game tutorial in
Chapter 2.

Now let’s look at *src/main.rs*:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo has generated a “Hello World!” for you, just like the one we wrote in
Listing 1-1! So that part is the same. The differences between our previous
project and the project generated by Cargo that we’ve seen so far are:

- Our code goes in the *src* directory
- The top level contains a *Cargo.toml* configuration file

Cargo expects your source files to live inside the *src* directory so that the
top-level project directory is just for READMEs, license information,
configuration files, and anything else not related to your code. In this way,
using Cargo helps you keep your projects nice and tidy. There’s a place for
everything, and everything is in its place.

If you started a project that doesn’t use Cargo, as we did with our project in
the *hello_world* directory, you can convert it to a project that does use
Cargo by moving your code into the *src* directory and creating an appropriate
*Cargo.toml*.

### Building and Running a Cargo Project

Now let’s look at what’s different about building and running your Hello World
program through Cargo! To do so, enter the following commands:

```text
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

This creates an executable file in *target/debug/hello_cargo* (or
*target\\debug\\hello_cargo.exe* on Windows), which you can run with this
command:

```text
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
```

Bam! If all goes well, `Hello, world!` should print to the terminal once more.

Running `cargo build` for the first time also causes Cargo to create a new file
at the top level called *Cargo.lock*. Cargo uses *Cargo.lock* to keep track of
the exact versions of dependencies used to build your project. This project
doesn’t have dependencies, so the file is a bit sparse. You won’t ever need to
touch this file yourself; Cargo will manage its contents for you.

We just built a project with `cargo build` and ran it with
`./target/debug/hello_cargo`, but we can also use `cargo run` to compile and
then run:

```text
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Notice that this time, we didn’t see the output telling us that Cargo was
compiling `hello_cargo`. Cargo figured out that the files haven’t changed, so
it just ran the binary. If you had modified your source code, Cargo would have
rebuilt the project before running it, and you would have seen output like
this:

```text
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Finally, there’s `cargo check`. This will quickly check your code to make sure
that it compiles, but not bother producing an executable:

```text
$ cargo check
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

Why would you not want an executable? `cargo check` is often much faster than
`cargo build`, because Cargo can skip the entire step of producing the
executable. If we’re checking our work throughout the process of writing the
code, this will speed things up! As such, many Rustaceans run `cargo check` as
they write their program to make sure that it compiles, and then run `cargo
build` once they’re ready to give it a spin themselves.

So a few more differences we’ve now seen:

- Instead of using `rustc`, build a project using `cargo build` or
  `cargo check` (or build and run it in one step with `cargo run`).
- Instead of the result of the build being put in the same directory as our
  code, Cargo will put it in the *target/debug* directory.

The other advantage of using Cargo is that the commands are the same no matter
what operating system you’re on, so at this point we will no longer be
providing specific instructions for Linux and Mac versus Windows.

### Building for Release

When your project is finally ready for release, you can use `cargo build
--release` to compile your project with optimizations. This will create an
executable in *target/release* instead of *target/debug*. These optimizations
make your Rust code run faster, but turning them on makes your program take
longer to compile. This is why there are two different profiles: one for
development when you want to be able to rebuild quickly and often, and one for
building the final program you’ll give to a user that won’t be rebuilt and that
we want to run as fast as possible. If you’re benchmarking the running time of
your code, be sure to run `cargo build --release` and benchmark with the
executable in *target/release*.

### Cargo as Convention

With simple projects, Cargo doesn’t provide a whole lot of value over just
using `rustc`, but it will prove its worth as you continue. With complex
projects composed of multiple crates, it’s much easier to let Cargo coordinate
the build. With Cargo, you can just run `cargo build`, and it should work the
right way.

Even though the `hello_cargo` project is simple, it now uses much of the real
tooling you’ll use for the rest of your Rust career. In fact, you can get
started with virtually all Rust projects you want to work on with the following
commands to check out the code using Git, change into the project directory,
and build:

```text
$ git clone someurl.com/someproject
$ cd someproject
$ cargo build
```

If you want to look at Cargo in more detail, check out [its documentation],
which covers all of its features.

[its documentation]: https://doc.rust-lang.org/cargo/

