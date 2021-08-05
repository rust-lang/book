# El lenguaje de programación Rust

![Status de compilación](https://github.com/rust-lang/book/workflows/CI/badge.svg)

Este repositorio contiene el código fuente del libro "The Rust Programming Language".

[El libro está disponible en formato de archivo fijo en No Starch Press.][nostarch].

[nostarch]: https://nostarch.com/rust

También puede leer el libro gratuitamente en línea. Por favor, vea el libro tal y como se envía con
las últimas versiones [estable], [beta], o [nightly] Ten en cuenta que los problemas 
 de esas versiones pueden haber sido ya corregidos en este repositorio, ya que esas 
 versiones se actualizan con menos frecuencia.

[estable]: https://doc.rust-lang.org/stable/book/
[beta]: https://doc.rust-lang.org/beta/book/
[nightly]: https://doc.rust-lang.org/nightly/book/

Vea las [versiones] para descargar sólo el código de todos los listados de código que aparecen en el libro.

[versiones]: https://github.com/rust-lang/book/releases

## Requisitos

La construcción del libro requiere  [mdBook], preferiblemente la misma versión que 
utiliza rust-lang/rust en [este archivo][rust-mdbook]. Para conseguirlo: 


[mdBook]: https://github.com/rust-lang-nursery/mdBook
[rust-mdbook]: https://github.com/rust-lang/rust/blob/master/src/tools/rustbook/Cargo.toml

```bash
$ cargo install mdbook --vers [version-num]
```

## crear

Para crear el libro, escriba:

```bash
$ mdbook build
```

El resultado estará en el subdirectorio del `book`. Para comprobarlo, ábralo en 
su navegador web.

_Firefox:_
```bash
$ firefox book/index.html                       # Linux
$ open -a "Firefox" book/index.html             # OS X
$ Start-Process "firefox.exe" .\book\index.html # Windows (PowerShell)
$ start firefox.exe .\book\index.html           # Windows (Cmd)
```

_Chrome:_
```bash
$ google-chrome book/index.html                 # Linux
$ open -a "Google Chrome" book/index.html       # OS X
$ Start-Process "chrome.exe" .\book\index.html  # Windows (PowerShell)
$ start chrome.exe .\book\index.html            # Windows (Cmd)
```

Para ejecutar las pruebas: 
```bash
$ mdbook test
```

## Forma de colaborar

Nos encantaría que nos ayudaras.[CONTRIBUTING.md][contrib] ara conocer el tipo de 
contribuciones que buscamos.

[contrib]: https://github.com/rust-lang/book/blob/master/CONTRIBUTING.md

### Traducciones

Nos encantaría que nos ayudaras a traducir el libro. Consulte la etiqueta [Traducciones] 
para unirse a los esfuerzos que se están llevando a cabo actualmente. Abra una nueva edición
para empezar a trabajar en un nuevo idioma. Estamos esperando a que el [mdbook] sea 
compatible con varios idiomas antes de fusionar alguno, pero ¡no dudes en empezar!
Revisión ortográfica

[Traducciones]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook]: https://github.com/rust-lang-nursery/mdBook/issues/5

## Revisión ortográfica

Para analizar los archivos fuente en busca de errores ortográficos, 
puede utilizar el script `spellcheck.sh` disponible en el directorio `ci`.
Necesita un diccionario de palabras válidas, que se proporciona en `ci/dictionary.txt.` 
Si el script produce un falso positivo (por ejemplo, usted utilizó la palabra `
BTreeMap`, que el script considera inválida), debe añadiresta 
palabra a `ci/dictionary.txt`(mantenga el orden ordenado para mantener la coherencia).

