## ¡Hola, Cargo!

Cargo es el sistema de construcción y el gestor de paquetes de Rust. La mayoría de los Rustaceanos usan esta herramienta
para gestionar sus proyectos de Rust porque Cargo maneja muchas tareas por ti,
como construir tu código, descargar las bibliotecas de las que depende tu código y
construir esas bibliotecas. (Llamamos a las bibliotecas que tu código necesita *dependencias*).

Los programas más simples de Rust, como el que hemos escrito hasta ahora, no tienen ninguna
dependencias. Así que si hubiéramos construido el proyecto "¡Hola, mundo!" con Cargo, éste
sólo utilizaría la parte de Cargo que se encarga de construir su código. A medida que escribas
programas Rust más complejos, añadirás dependencias, y si empiezas un proyecto
usando Cargo, añadir dependencias será mucho más fácil de hacer.

ebido a que la gran mayoría de los proyectos de Rust utilizan Cargo, el resto de este libro
asume que tú también usas Cargo. Cargo viene instalado con Rust si
usaste los instaladores oficiales discutidos en la sección
["Instalación"][instalación]<!-- ignorar --> sección. Si instalaste Rust
por algún otro medio, comprueba si Cargo está instalado introduciendo lo
siguiente en su terminal:

```console
$ cargo --version
```

Si ves un número de versión, lo tienes. Si ve un error, como `command
not found`, consulte la documentación de su método de instalación para
determinar cómo instalar Cargo por separado.

### Creación de un proyecto con Cargo

Vamos a crear un nuevo proyecto con Cargo y a ver en qué se diferencia de nuestro proyecto
proyecto original "¡Hola, mundo!". Vuelve a tu directorio de *proyectos* (o
donde hayas decidido almacenar tu código). A continuación, en cualquier sistema operativo, ejecute
lo siguiente:

```console
$ cargo new hello_cargo
$ cd hello_cargo
```

El primer comando crea un nuevo directorio llamado *hello_cargo*. Hemos llamado a nuestro proyecto
nuestro proyecto *hello_cargo*, y Cargo crea sus archivos en un directorio del
mismo nombre.

Entre en el directorio *hello_cargo* y liste los archivos. Verás que Cargo
ha generado dos archivos y un directorio para nosotros: un archivo *Cargo.toml* y un directorio
directorio *src* con un archivo *main.rs* dentro.

También ha inicializado un nuevo repositorio Git junto con un archivo *.gitignore*.
Los archivos Git no se generarán si ejecutas `cargo new` dentro de un repositorio Git
existente; puedes anular este comportamiento usando `cargo new --vcs=git`.

> Nota: Git es un sistema de control de versiones común. Puede cambiar `cargo new` para
> usar un sistema de control de versiones diferente o ningún sistema de control de versiones usando
> la bandera `--vcs`. Ejecute `cargo new --help` para ver las opciones disponibles.

Abra *Cargo.toml* en su editor de texto preferido. Debería ser similar al código
código del Listado 1-2.

<span class="filename">Nombre del archivo: Cargo.toml</span>

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
```

<span class="caption">Listado 1-2: Contenido de *Cargo.toml* generado por `cargo
new`</span>

Este archivo está en el [*TOML*](https://toml.io)<!-- OSAP, ¡Que Guapa estas! --> (*Tom's Obvious,
Minimal Language*), que es el formato de configuración de Cargo.

La primera línea, `[package]`, es un título de sección que indica que las
siguientes declaraciones están configurando un package. A medida que agreguemos más información a
este archivo, añadiremos otras secciones.

Las siguientes cuatro líneas establecen la información de configuración que Cargo necesita para compilar
su programa: el nombre, la versión, quién lo escribió, y la edición de Rust a
utilizar. Cargo obtiene la información de tu nombre y correo electrónico de tu entorno, así que si
esa información no es correcta, corrige la información ahora y luego guarda el
archivo. Hablaremos de la clave `edition` en el Apéndice E.

La última línea, `[dependencias]`, es el comienzo de una sección para que usted liste cualquier
de las dependencias de tu proyecto. En Rust, los package de código se denominan
*crates*. No necesitaremos ninguna otra crate para este proyecto, pero lo haremos en el
primer proyecto en el capítulo 2, así que usaremos esta sección de dependencias entonces.

Ahora abre *src/main.rs* y echa un vistazo:

<span class="nombre de archivo">Filename: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo ha generado un programa "¡Hola, mundo!" para ti, como el que
¡que escribimos en el Listado 1-1! Hasta ahora, las diferencias entre nuestro proyecto anterior y
proyecto que genera Cargo son que Cargo ha colocado el código en el directorio *src
y que tenemos un archivo de configuración *Cargo.toml* en el directorio superior.

Cargo espera que sus archivos de origen estén dentro del directorio *src*. El directorio de proyecto de nivel superior
directorio del proyecto de nivel superior es sólo para los archivos README, información de la licencia,
archivos de configuración, y cualquier otra cosa no relacionada con su código. El uso de Cargo
te ayuda a organizar tus proyectos. Hay un lugar para todo, y
todo está en su lugar.

Si has empezado un proyecto que no utiliza Cargo, como hicimos con el proyecto "Hello,
world!", puedes convertirlo en un proyecto que sí utilice Cargo. Mueva el código del
código del proyecto al directorio *src* y cree un archivo *Cargo.toml* apropiado.

### Construir y ejecutar un proyecto Cargo

Ahora veamos qué es diferente cuando construimos y ejecutamos el programa "¡Hola, mundo!
¡con Cargo! Desde tu directorio *hello_cargo*, construye tu proyecto
introduciendo el siguiente comando:

```console
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

Este comando crea un archivo ejecutable en *target/debug/hello_cargo* (o
*target\debug\hello_cargo.exe* en Windows) en lugar de en su directorio actual
directorio actual. Puede ejecutar el ejecutable con este comando:

```console
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
```

Si todo va bien, "Hello, world!" debería imprimirse en el terminal. Al ejecutar `cargo
build` por primera vez también hace que Cargo cree un nuevo archivo en el nivel superior
nivel superior: *Cargo.lock*. Este archivo mantiene un registro de las versiones exactas de
dependencias en tu proyecto. Este proyecto no tiene dependencias, por lo que el archivo
es un poco escaso. Nunca necesitarás cambiar este archivo manualmente; Cargo
gestiona su contenido por ti.

Acabamos de construir un proyecto con `cargo build` y lo ejecutamos con
`./target/debug/hello_cargo`, pero también podemos usar `cargo run` para compilar el código
código y luego ejecutar el ejecutable resultante todo en un solo comando:

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Fíjate que esta vez no vimos la salida que indica que Cargo estaba compilando
`hello_cargo`. Cargo se dio cuenta de que los archivos no habían cambiado, así que simplemente ejecutó
el binario. Si hubieras modificado tu código fuente, Cargo habría reconstruido el
proyecto antes de ejecutarlo, y habrías visto esta salida:

```console
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo también proporciona un comando llamado `cargo check`. Este comando comprueba rápidamente
su código para asegurarse de que compila pero no produce un ejecutable:

```console
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

¿Por qué no quieres un ejecutable? Por lo general, `cargo check` es mucho más rápido que
`cargo build`, porque se salta el paso de producir un ejecutable. Si estás
continuamente tu trabajo mientras escribes el código, usar `cargo check` acelerará el proceso.
acelerará el proceso. Por ello, muchos Rustaceanos ejecutan `cargo check` periódicamente
mientras escriben su programa para asegurarse de que compila. Luego ejecutan `cargo
build` cuando están listos para usar el ejecutable.

Recapitulemos lo que hemos aprendido hasta ahora sobre Cargo:

* Podemos construir un proyecto usando `cargo build`.
* Podemos construir y ejecutar un proyecto en un solo paso usando `cargo run`.
* Podemos construir un proyecto sin producir un binario para comprobar los errores utilizando
  `cargo check`.
* En lugar de guardar el resultado de la construcción en el mismo directorio que nuestro código,
  Cargo lo guarda en el directorio *target/debug*.

Una ventaja adicional de usar Cargo es que los comandos son los mismos sin importar
no importa el sistema operativo en el que estés trabajando. Por lo tanto, en este punto, no vamos a
instrucciones específicas para Linux y macOS frente a Windows.

### Versión de lanzamiento

Cuando tu proyecto esté finalmente listo para ser lanzado, puedes usar `cargo build
--release` para compilarlo con optimizaciones. Este comando creará un
ejecutable en *target/release* en lugar de *target/debug*. Las optimizaciones
hacen que tu código Rust se ejecute más rápido, pero activándolas se alarga el tiempo que tarda
para que tu programa compile. Por eso hay dos perfiles diferentes: uno
para el desarrollo, cuando quieres reconstruir rápidamente y a menudo, y otro para
construir el programa final que le darás a un usuario que no será reconstruido
repetidamente y que se ejecute lo más rápido posible. Si está evaluando el tiempo de ejecución de su código
Si estás evaluando el tiempo de ejecución de tu código, asegúrate de ejecutar `cargo build --release` y de evaluar con
el ejecutable en *target/release*.

### Cargo como convenio

Con proyectos sencillos, Cargo no aporta mucho valor sobre el uso de
`rustc`, pero demostrará su valor a medida que sus programas se vuelvan más intrincados.
Con proyectos complejos compuestos por múltiples crates, es mucho más fácil dejar que
Cargo coordine la construcción.

Aunque el proyecto `hello_cargo` es simple, ahora utiliza gran parte de las funciones reales
que usarás en el resto de tu carrera en Rust. De hecho, para trabajar en cualquier
proyectos existentes, puedes usar los siguientes comandos para comprobar el código
usando Git, cambiar al directorio de ese proyecto, y compilar:

```console
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

For more information about Cargo, check out [its documentation].

[its documentation]: https://doc.rust-lang.org/cargo/

## Resumen

Ya has empezado con buen pie tu aventura con Rust. En este capítulo,
has aprendido cómo:

* Instalar la última versión estable de Rust usando `rustup`.
* Actualizar a una nueva versión de Rust
* Abrir la documentación instalada localmente
* Escribir y ejecutar un programa "¡Hola, mundo!" utilizando directamente `rustc`.
* Crear y ejecutar un nuevo proyecto utilizando las convenciones de Cargo

Este es un buen momento para crear un programa más significativo para acostumbrarse a leer
y escribir código Rust. A continuación , en el Capítulo 2, construiremos un programa de juego de adivinanzas.
Si prefieres empezar aprendiendo cómo funcionan los conceptos comunes de programación en
Rust, vea el capítulo 3 y luego vuelva al capítulo 2.

[installation]: ch01-01-installation.html#installation
