## Anexo A: Palabras clave

The following list contains keywords that are reserved for current or future
use by the Rust language. As such, they cannot be used as identifiers (except
as raw identifiers as we’ll discuss in the “[Raw
Identifiers][raw-identifiers]<!-- ignore -->” section), including names of
functions, variables, parameters, struct fields, modules, crates, constants,
macros, static values, attributes, types, traits, or lifetimes.

[raw-identifiers]: #raw-identifiers

### Palabras clave actualmente en uso

Las siguientes palabras clave tienen actualmente la funcionalidad descrita.

* `as` - realizar un casting primitivo, desambiguar el rasgo específico que contiene
  un elemento, o cambiar el nombre de los elementos en las declaraciones `use` y `extern crate`.
* `async` -  retornar un `Future` en lugar de bloquear el hilo actual
* `await` - suspender la ejecución hasta que el resultado de un `Future` esté listo
* `break` - salir inmediatamente de un bucle
* `const` - definir elementos constantes o punteros crudos constantes
* `continue` - continuar con la siguiente iteración del bucle
* `crate` - vincular un crate externo o una variable de macro que represente el crate en
  en el que está definida la macro
* `dyn` - envío dinámico a un objeto trait
* `else` - la respuesta a las construcciones de flujo de control "if" y "if let".
* `enum` - definir una enumeración
* `extern` - vincular una caja, función o variable externa
* `false` - Boolean false literal
* `fn` - definir una función o el tipo de puntero de función
* `for` - bucle sobre los elementos de un iterador, implementar un rasgo, o especificar un
  tiempo de duración mayor
* `if` - rama basada en el resultado de una expresión condicional
* `impl` - implementar la funcionalidad inherente o de rasgo
* `in` - parte de la sintaxis del bucle `for`.
* `let` - enlazar una variable
* `loop` - bucle de forma incondicional
* `match` - hacer coincidir un valor con los patrones
* `mod` - definir un módulo
* `move` - hacer que las capturas de un cierre sean de su propiedad
* `mut` - denotan la mutabilidad en las referencias, los punteros en bruto o los enlaces de patrones
* `pub` - denotan la visibilidad pública en campos struct, bloques `impl` o módulos
* `ref` - enlazar por referencia
* `return` - retorno de la función
* `Self` - un nombre de tipo para el tipo que estamos definiendo o implementando
* `self` - método tema o módulo actual
* `static` - variable global o el tiempo de vida que dura toda la ejecución del programa
* `struct` - definir una estructura
* `super` - módulo padre del módulo actual
* `trait` - definir una trait
* `true` - Booleano verdadero
* `type` - definir un nombre de tipo o un tipo asociado
* `union` - definen una [union] y sólo es una palabra clave cuando se utiliza en una declaración de unión
* `unsafe` - denotan código, funciones, rasgos o implementaciones no seguras
* `use` - introducir los símbolos en el ámbito de aplicación
* `where` - denotan declaraciones que restringen un tipo
* `while` - bucle condicional basado en el resultado de una expresión

[union]: ../reference/items/unions.html

### Palabras clave reservadas para uso futuro

Las siguientes palabras clave no tienen ninguna funcionalidad pero están reservadas por Rust
para un posible uso futuro.

* `abstract`
* `become`
* `box`
* `do`
* `final`
* `macro`
* `override`
* `priv`
* `try`
* `typeof`
* `unsized`
* `virtual`
* `yield`

### Identificadores en bruto

*Los identificadores brutos* son la sintaxis que permite utilizar palabras clave donde normalmente no se
normalmente se permiten. Para utilizar un identificador sin procesar, hay que anteponer a la palabra clave `r#`

Por ejemplo, `match` es una palabra clave. Si intenta compilar la siguiente función
que utiliza `match` como nombre:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
fn match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}
```

obtendrá este error:

```text
error: expected identifier, found keyword `match`
 --> src/main.rs:4:4
  |
4 | fn match(needle: &str, haystack: &str) -> bool {
  |    ^^^^^ expected identifier, found keyword
```

El error muestra que no se puede utilizar la palabra clave `match` como identificador de la función
como identificador de la función. Para utilizar `match` como nombre de función, es necesario utilizar la sintaxis de identificador
sintaxis del identificador, así:

<span class="filename">Filename: src/main.rs</span>

```rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}
```

Este código se compilará sin ningún error. Observe el prefijo `r#` en el nombre de la función
en su definición, así como el lugar donde se llama a la función en `main`.

Los identificadores en bruto le permiten utilizar cualquier palabra que elija como identificador, incluso si esa palabra sea 
una palabra clave reservada. Además, los identificadores sin procesar le permiten
utilizar bibliotecas escritas en una edición de Rust diferente a la que utiliza tu crate.
Por ejemplo, " try " no es una palabra clave en la edición de 2015 pero sí en la de 2018
2018. Si depende de una biblioteca escrita con la edición de 2015 y
tiene una función `try`, tendrá que utilizar la sintaxis del identificador bruto, `r#try` en
en este caso, para llamar a esa función desde tu código de la edición 2018. Véase en [Appendix
E][appendix-e]<!-- ignore ecma--> para más información sobre las ediciones.

[appendix-e]: appendix-05-editions.html
