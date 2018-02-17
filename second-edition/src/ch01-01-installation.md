## Instalacion

El primer paso para usar Rust es instalarlo. Necesitarás tener conexión a internet
 para ejecutar los comandos de este capítulo ya que nos vamos a descargar Rust de internet
We’ll actually be installing Rust using `rustup`, a
command-line tool for managing Rust versions and associated tools.

The following steps will install the latest stable version of the Rust
compiler. The examples and output shown in this book used stable Rust 1.21.0.
Due to Rust’s stability guarantees, which we’ll discuss further in the “How
Rust is Made” section later in this chapter, all of the examples that compile
will continue to compile with newer versions of Rust. The output may differ
slightly as error messages and warnings are often improved. In other words, the
newer, stable version of Rust you will install with these steps should work as
expected with the content of this book.

> #### Command Line Notation
>
> Vamos a ir mostrando una serie de comandos usando el terminal; todas estas líneas
> comienzan por `$`. No necesitas teclear el caracter `$`; esto se usa para indicar
> el inicio de cada comando. Habrás visto muchos tutoriales y ejemplos en internet
> que siguen esta convención: `$` para comandos ejecutados como un usuario normal
> y `#` para los comandos que debes ejecutar como administrador o root.
> Las líneas que no comienzan con `$` muestran, normalmente, la salida por terminal
> del comando anterior.
> Additionally, PowerShell specific examples will use `>` rather than `$`.

### Instalación en Linux o Mac

Si utilizas Linux o Mac, el 99% de lo que tienes que hacer es abrir un terminal
y escribir lo siguiente:

```text
$ curl https://sh.rustup.rs -sSf | sh
```

Esto descargará un script y comenzará la instalación de la herramienta `rustup`,
que instala la ultima version estable de Rust. El proceso de instalación seguramente
te pedirá tu contraseña. Si todo va bien verás aparecer este mensaje:

```text
Rust is installed now. Great!
```

Por supuesto, si no confias en usar  `curl | sh` puedes descargar, inspeccionar y
ejecutar el script de instalación del modo que prefieras.

El script de instalación añade automáticamente Rust a tu PATH de sistema después del siguiente login.
Si quieres comenzar a usar Rust ya mismo, ejecuta el siguiente comando en tu terminal:

```text
$ source $HOME/.cargo/env
```

Como alternativa, añade la siguiente línea a tu fichero `~/.bash_profile`:

```text
$ export PATH="$HOME/.cargo/bin:$PATH"
```

Finally, you’ll need a linker of some kind. You likely have one installed. If
not, when you compile a Rust program, you’ll get errors that a linker could not
be executed. Check your platform’s documentation for how to install a C
compiler; they usually come with the correct linker as well, given that C needs
one. You may want to install a C compiler regardless of your need for only a
linker; some common Rust packages depend on C code and will need a C compiler
too.

### Instalación en Windows

En Windows ve a [https://rustup.rs](https://rustup.rs/)<!-- ignore --> y sigue
las instrucciones.
You’ll also need the C++ build tools for Visual Studio
2013 or later. The easiest way to acquire the build tools is by installing
[Build Tools for Visual Studio 2017][visualstudio] which provides only the
Visual C++ build tools. Alternately, you can [install][visualstudio] Visual
Studio 2017, Visual Studio 2015, or Visual Studio 2013 and during installation
select the desktop development with C++ workload.

[install]: https://www.rust-lang.org/en-US/install.html
[visualstudio]: https://www.visualstudio.com/downloads/

The rest of this book will use commands that work in both `cmd.exe` and
PowerShell. If there are specific differences, we’ll explain which to use.

### Custom Installations Without Rustup

Si por alguna razón prefieres no usar `rustup`, mira la
[página de instalación de Rust](https://www.rust-lang.org/install.html)
para ver otras opciones.

### Actualización

Una vez tienes Rust instalado desde `rustup`, actualizar a la última versión
es muy sencillo. Desde el terminal, ejecuta el script de actualización:

```text
$ rustup update
```

### Desinstalación

Desinstalar Rust y Rustup es tan sencillo como instalarlos. En tu terminal,
ejecuta el script de desinstalación:

```text
$ rustup self uninstall
```

### Solución de problemas

Para comprobar que tienes Rust instalado, puedes abrir un terminal y escribir
lo siguiente:

```text
$ rustc --version
```

Esto te mostrará el número de versión, el hash del commit y la fecha
de la versión que tienes instalada en un formato similar a este:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

¡Si ves esto significa que Rust ha sido instalado correctamente! ¡Felicitaciones!

Si no lo ves y estás usando Windows, comprueba que Rust está en la variable `%PATH%`
 del sistema.

Si aún así no funciona hay numerosos lugares en los que buscar ayuda.
Lo más sencillo es [el canal #rust del IRC en irc.mozilla.org][irc]<!-- ignore-->,
al que puedes acceder mediante [Mibbit][mibbit]. Ve a esa dirección para chatear
con otros Rustaceans (un apodo que usamos para referirnos a nosotros mismos) que
 te podrán ayudar. Otros recursos importantes son el [foro de Usuarios][users] y
 [Stack Overflow][stackoverflow].

[irc]: irc://irc.mozilla.org/#rust
[mibbit]: http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust
[users]: https://users.rust-lang.org/
[stackoverflow]: http://stackoverflow.com/questions/tagged/rust

### Documentación local

La instalación incluye también una copia local de la documentación que puedes
leer offline. Ejecuta el comando `rustup doc` para abrir la documentación local
en tu navegador.

Cada vez que te encuentres con un tipo o función provista por las bibliotecas
 standard y no estés seguro de qué hace o como usarlo, ¡consulta la documentación
 de la API para encontrar la respuesta!
