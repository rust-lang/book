## ¡Hola, mundo

Ahora que has instalado Rust, vamos a escribir tu primer programa en Rust.
Es tradición cuando se aprende un nuevo lenguaje escribir un pequeño programa que
imprima el texto `¡Hola, mundo!` en la pantalla, así que haremos lo mismo aquí.

> Nota: Este libro asume una familiaridad básica con la línea de comandos. Rust no hace
> demandas específicas sobre su edición o herramientas o donde vive su código, así
> que si usted prefiere usar un entorno de desarrollo integrado (IDE) en lugar de
> la línea de comandos, siéntase libre de usar su IDE favorito. Muchos IDEs tienen
> ahora algún grado de soporte para Rust; comprueba la documentación del IDE para más detalles.
> Recientemente, el equipo de Rust se ha centrado en habilitar un gran soporte de IDE,
> ¡y se ha progresado rápidamente en ese frente!

### Creando un directorio de proyecto

Empezarás por crear un directorio para almacenar tu código Rust. A Rust no le importa
dónde vive su código, pero para los ejercicios y proyectos de este libro,
sugerimos hacer un directorio de *proyectos* en su directorio personal y mantener
todos sus proyectos allí.

Abre un terminal e introduce los siguientes comandos para crear un directorio de *proyectos*
y un directorio para el proyecto "¡Hola, mundo!" dentro del directorio de *proyectos*.

Para Linux, macOS y PowerShell en Windows, introduzca esto:

```console
mkdir ~/proyecto
cd ~/proyecto
mkdir hola_mundo
cd hola_mundo
```

Para el CMD de Windows, introduzca lo siguiente

```cmd
> mkdir "%USERPROFILE%\projectos"
> cd /d "%USERPROFILE%\projectos"
> mkdir hola_mundo
> cd hola_mundo
```

### Escribir y ejecutar un programa de Rust

A continuación, crea un nuevo archivo fuente y llámalo *main.rs*. Los archivos
Rust siempre terminan con la extensión *.rs.* Si utilizas más de una palabra en tu nombre de archivo, utiliza
un guión bajo para separarlas. Por ejemplo, usa hola_mundo.rs en lugar de holamundo.rs.

Ahora abre el archivo *main.rs* que acabas de crear e introduce el código del Listado 1-1.

<span class="Nombre del archivo"> Nombre del archivo: main.rs</span>

```rust
fn main() {
    println!("¡Hola, mundo!");
}
```

<span class="caption">Listado 1-1 Un programa que imprime `¡Hola, mundo!`</span>

Guarda el archivo y vuelve a la ventana de tu terminal.
En Linux o macOS, introduce los siguientes
comandos para compilar y ejecutar el archivo:

```console
$ rustc main.rs
$ ./main
Hello, world!
```

En Windows, introduzca el comando `.\main.exe`en lugar de `./main`:

```powershell
> rustc main.rs
> .\main.exe
¡Hola, mundo!
```

Independientemente de su sistema operativo, la cadena `¡Hola, mundo!` debería imprimirse
en el terminal. Si no ve esta salida, consulte la parte de
[“Solución de problemas”][troubleshooting]<!-- ignore --> de la sección de Instalación
para obtener ayuda.

Si, `¡Hola, mundo!` se imprime, ¡felicidades! Has escrito oficialmente un programa en Rust.
Eso te convierte en un programador de Rust, ¡bienvenido!

### Anatomía de un programa Rust

Revisemos en detalle lo que acaba de suceder en tu programa "¡Hola, mundo!"
Aquí está la primera pieza del rompecabezas:

```rust
fn main() {

}
```

Estas líneas definen una función en Rust. La función `main` es especial:
siempre es el primer código que se ejecuta en todo programa ejecutable de Rust. La primera
línea declara una función llamada `main` que no tiene parámetros y no devuelve nada.
Si hubiera parámetros, irían dentro de los paréntesis, `()`.

Además, observe que el cuerpo de la función está encerrado entre llaves, `{}`. Rust
requiere estos alrededor de todos los cuerpos de las funciones. Es un buen estilo
colocar la llave de apertura en la misma línea que la declaración
de la función, añadiendo un espacio en medio.

Si quieres mantener un estilo estándar en todos los proyectos de Rust, puedes usar
una herramienta de formato automático llamada `rustfmt` para formatear
tu código en un estilo particular. El equipo de Rust ha incluido esta herramienta
con la distribución estándar de Rust, al igual que `rustc`, por lo que ya debería estar
instalada en tu ordenador. Consulta la documentación online para más detalles.

Dentro de la función `main` está el siguiente código:

```rust
    println!("¡Hola, mundo!");
```

Esta línea hace todo el trabajo en este pequeño programa:
imprime el texto en la pantalla. Hay cuatro detalles importantes a tener en cuenta aquí.

Primero, el estilo Rust es indentar con cuatro espacios, y no con un tabulador.

Segundo, `println!` llama a una macro de Rust. Si en cambio llamara a una función, se
introduciría como `println` (sin el `!`). Hablaremos de las macros de Rust con
más detalle en el capítulo 19. Por ahora, sólo necesitas saber que usar una `!` significa
que estás llamando a una macro en lugar de a una función normal.

En tercer lugar, ves la cadena `"¡Hola, mundo!"`. Pasamos esta cadena como argumento
a `println!`, y la cadena se imprime en la pantalla.

En cuarto lugar, terminamos la línea con un punto y coma (`;`), que indica
que esta expresión ha terminado y la siguiente está lista para comenzar. La mayoría de
las líneas de código de Rust terminan con un punto y coma.

### Compilar y ejecutar son pasos separados

Acabas de ejecutar un programa recién creado, así que vamos a
examinar cada paso del proceso.

Antes de ejecutar un programa Rust, debes compilarlo usando el compilador Rust
introduciendo el comando `rustc` y pasándole el nombre de tu archivo fuente, de esta manera:

```console
rustc main.rs
```

Si tienes experiencia en C o C++, notarás que esto es similar a `gcc` o `clang`.
Después de compilar con éxito, Rust produce un ejecutable binario.

En Linux, macOS y PowerShell en Windows, puedes ver el ejecutable introduciendo el
comando `ls` en tu shell. En Linux y macOS, verás dos archivos.
Con PowerShell en Windows, verás los mismos tres archivos que verías usando CMD.

```console
$ ls
main  main.rs
```

With CMD on Windows, you would enter the following:

```cmd
> dir /B %= the /B indica que sólo se muestren los nombres de los archivos =%
main.exe
main.pdb
main.rs
```

Esto muestra el archivo de código fuente con la
extensión *.rs,* el archivo ejecutable (main.exe en Windows, pero main en todas
las demás plataformas), y, cuando se utiliza Windows, un archivo que contiene información
de depuración con la extensión *.pdb.* Desde aquí, se ejecuta el
archivo *main* o *main.exe*, como se indica a continuación:

```console
./main # or .\main.exe en Windows
```

Si *main.rs* es tu programa "¡Hola, mundo!", esta línea imprimiría `¡Hola,
mundo!` en tu terminal.

Si estás más familiarizado con un lenguaje dinámico, como Ruby, Python o
JavaScript, puede que no estés acostumbrado a compilar y ejecutar un programa
como pasos separados. Rust es un *lenguaje compilado por adelantado*, lo que significa que puedes
compilar un programa y darle el ejecutable a otra persona, y ésta podrá ejecutarlo incluso sin
tener Rust instalado. Si le das a alguien un archivo *.rb*, *.py* o *.js,* necesita tener instalada una
implementación de Ruby, Python o JavaScript (respectivamente). Pero en esos lenguajes, sólo necesitas un comando
para compilar y ejecutar tu programa. Todo es una compensación en el diseño del lenguaje.

Sólo compilar con `rustc` está bien para programas simples, pero a medida
que tu proyecto crece, querrás manejar todas las opciones y facilitar el compartir
tu código. A continuación, te presentaremos la herramienta Cargo, que te
ayudará a escribir programas Rust del mundo real.

[troubleshooting]: ch01-01-installation.html#troubleshooting
