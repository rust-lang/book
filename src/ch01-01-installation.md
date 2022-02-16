## Instalación

El primer paso es instalar Rust. Vamos a descargar Rust a través de `rustup`, una
herramienta de línea de comandos para gestionar las versiones de Rust y las herramientas asociadas. Necesitarás
una conexión a Internet para la descarga.

> Nota: Si prefiere no usar `rustup` por alguna razón, por favor vea [la página de instalación de Rust
> página de instalación](https://www.rust-lang.org/tools/install) para conocer otras opciones.

Los siguientes pasos instalan la última versión estable del compilador de Rust.
Las garantías de estabilidad de Rust aseguran que todos los ejemplos del libro que
compilan seguirán compilando con las nuevas versiones de Rust. La salida puede
diferir ligeramente entre versiones, porque Rust a menudo mejora los mensajes de error
y advertencias. En otras palabras, cualquier versión más nueva y estable de Rust que instales
usando estos pasos debería funcionar como se espera con el contenido de este libro.

> ### Notación de la línea de comandos
>
> En este capítulo y a lo largo del libro, mostraremos algunos comandos utilizados en el
> terminal. Las líneas que debes introducir en una terminal comienzan todas con `$`. Usted
> No es necesario que escriba el carácter `$`; indica el comienzo de cada comando.
> comando. Las líneas que no comienzan con `$` normalmente muestran la salida del
> comando anterior. Además, los ejemplos específicos de PowerShell utilizarán `>`
> en lugar de `$`.

### Instalación de `rustup` en Linux o macOS

Si utilizas Linux o macOS, abre un terminal e introduce el siguiente comando:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

El comando descarga un script e inicia la instalación de la herramienta `rustup
que instala la última versión estable de Rust. Es posible que se le pida
su contraseña. Si la instalación tiene éxito, aparecerá la siguiente línea:

```text
Rust is installed now. Great!
```

Además, necesitarás un enlazador de algún tipo. Es probable que uno ya esté
instalado, pero cuando intentas compilar un programa de Rust y obtienes errores indicando
que un enlazador no pudo ejecutarse, eso significa que un enlazador no está instalado en tu
sistema y tendrás que instalar uno manualmente. Los compiladores de C suelen venir con
el enlazador correcto. Consulta la documentación de tu plataforma para saber cómo instalar un compilador de C
para instalar un compilador de C. Además, algunos paquetes comunes de Rust dependen del código C y necesitarán un compilador C
C. Por lo tanto, podría valer la pena instalar uno ahora.

### Instalación de `rustup` en Windows

En Windows, vaya a [https://www.rust-lang.org/tools/install][install] y siga
las instrucciones para instalar Rust. En algún momento de la instalación, recibirás un mensaje
recibirás un mensaje explicando que también necesitarás las herramientas de compilación de C++ para
Visual Studio 2013 o posterior. La forma más sencilla de adquirir las herramientas de compilación es
instalar [Build Tools for Visual Studio 2019][visualstudio]. Cuando se le pregunte qué
cargas de trabajo instalar asegúrese de que se selecciona "C++ build tools" y que el
SDK de Windows 10 y los componentes del paquete de idioma inglés estén incluidos.

[install]: https://www.rust-lang.org/tools/install
[visualstudio]: https://visualstudio.microsoft.com/visual-cpp-build-tools/

El resto de este libro utiliza comandos que funcionan tanto en *cmd.exe* como en PowerShell.
Si hay diferencias específicas, explicaremos cuál usar.

### Actualización y desinstalación

Después de haber instalado Rust a través de `rustup`, la actualización a la última versión es
fácil. Desde tu shell, ejecuta el siguiente script de actualización:

```console
$ rustup update
```

Para desinstalar Rust y `rustup`, ejecute el siguiente script de desinstalación desde su
shell:

```console
$ rustup self uninstall
```

### Solución de problemas

Para comprobar si tienes Rust instalado correctamente, abre un shell e introduce esta
línea:

```console
$ rustc --version
```

Debería ver el número de versión, el hash de confirmación y la fecha de confirmación de la última
versión estable que se ha publicado en el siguiente formato:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

Si ves esta información, ¡has instalado Rust con éxito! Si no ve
esta información y estás en Windows, comprueba que Rust está en tu variable de sistema `%PATH%`.
en tu variable de sistema. Si todo esto es correcto y Rust sigue sin funcionar, hay
varios lugares donde puedes obtener ayuda. El más fácil es el canal #beginners en
[el Discord oficial de Rust][discord]. Allí, puedes chatear con otros Rustaceans
(un tonto apodo con el que nos llamamos a nosotros mismos) que pueden ayudarte. Otros grandes
recursos incluyen [el foro de usuarios][usuarios] y [Stack Overflow][stackoverflow].

[discord]: https://discord.gg/rust-lang
[users]: https://users.rust-lang.org/
[stackoverflow]: https://stackoverflow.com/questions/tagged/rust

### Documentación local

La instalación de Rust también incluye una copia de la documentación localmente, para que
puedes leerla fuera de línea. Ejecute `rustup doc` para abrir la documentación local en
tu navegador.

Cada vez que un tipo o función es proporcionada por la biblioteca estándar y no estás
no está seguro de lo que hace o de cómo utilizarlo, utilice la documentación de la interfaz de
(API) para averiguarlo.
