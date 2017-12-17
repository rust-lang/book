## Instalacion

El primer paso para usar Rust es instalarlo. Necesitarás tener conexión a internet para ejecutar los comandos de este capítulo ya que nos vamos a descargar Rust de internet

Vamos a ir mostrando una serie de comandos usando el terminal; todas estas líneas comienzan por `$`. No necesitas teclear el caracter `$`; esto se usa para indicar el inicio de cada comando. Habrás visto muchos tutoriales y ejemplos en internet que siguen esta convención: `$` para comandos ejecutados como un usuario normal y `#` para los comandos que debes ejecutar como administrador o root. Las líneas que no comienzan con `$` muestran, normalmente, la salida por terminal del comando anterior.

### Instalación en Linux o Mac

Si utilizas Linux o Mac todo lo que tienes que hacer es abrir un terminal y escribir:

```text
$ curl https://sh.rustup.rs -sSf | sh
```

Esto descargará un script y comenzará la instalación. El proceso de instalación te pedirá tu contraseña. Si todo va bien verás aparecer lo siguiente:

```text
Rust is installed now. Great!
```

Por supuesto, si no estás de acuerdo con el uso de `curl | sh` puedes descargar, inspeccionar y ejecutar el script de instalación del modo que prefieras.

El script de instalación añade automáticamente Rust a tu PATH de sistema después del siguiente login.
Si quieres comenzar a usar Rust ya mismo, ejecuta el siguiente comando en tu terminal:

```text
$ source $HOME/.cargo/env
```

Como alternativa, añade la siguiente línea a tu fichero `~/.bash_profile`:

```text
$ export PATH="$HOME/.cargo/bin:$PATH"
```

### Instalación en Windows

En Windows ve a [https://rustup.rs](https://rustup.rs/)<!-- ignore --> y sigue las instrucciones para descargar rustup-init.exe. Ejecútalo y sigue las instrucciones.

Para el resto de comandos específicos para Windows de este libro asumiremos que estás usando `cmd` como shell. Si usas un intérprete de comandos diferente, deberías poder ejecutar los mismos comandos que los usuarios de Linux y Mac. Si no funciona, consulta la documentación del intérprete de comandos que estás usando.

### Instalaciones personalizadas

Si por alguna razón prefieres no usar rustup.rs, mira la [página de instalación de Rust](https://www.rust-lang.org/install.html) para ver otras opciones.

### Actualización

Una vez tienes Rust instalado, actualizar a la última versión es muy sencillo.
En tu terminal o intérprete de comandos ejecuta el script de actualización:

```text
$ rustup update
```

### Desinstalación

Desinstalar Rust es tan sencillo como instalarlo. En tu terminal ejecula el script de desinstalación:

```text
$ rustup self uninstall
```

### Solución de problemas

Si tienes Rust instalado, puedes abrir un terminal y escribir lo siguiente:

```text
$ rustc --version
```

Esto te mostrará el número de versión, el hash del commit y la fecha de la versión que tienes instalada en un formato similar a este:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

¡Si ves esto significa que Rust ha sido instalado correctamente! ¡Felicitaciones!

Si no lo ves y estás usando Windows, comprueba que Rust está en la variable `%PATH%` de sistema.

Si aún así no funciona hay numerosos lugares en los que buscar ayuda.
Lo más sencillo es [el canal #rust del IRC en irc.mozilla.org][irc]<!-- ignore-->, al que puedes acceder mediante [Mibbit][mibbit]. Ve a esa dirección y estarás chateando con otros Rustaceans (un apodo que usamos para referirnos a nosotros mismos) que te podrán ayudar. Otros recursos importantes son el [forum de Usuarios][users] y [Stack Overflow][stackoverflow].

[irc]: irc://irc.mozilla.org/#rust
[mibbit]: http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust
[users]: https://users.rust-lang.org/
[stackoverflow]: http://stackoverflow.com/questions/tagged/rust

### Documentación local

La instalación incluye también una copia local de la documentación que puedes leer offline. Ejecuta el comando `rustup doc` para abrir la documentación local en tu navegador.

¡Cada vez que te encuentres con un tipo o función provista por las bibliotecas standard y no estés seguro de qué hace, consulta la documentación del API para encontrar la respuesta!
