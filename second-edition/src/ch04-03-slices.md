## Rebanadas (Slice)

Otro tipo de datos que no tiene propiedad es el *slice*. Las rebanadas te permiten
hacer referencia a una secuencia continua de elementos en una colección en lugar de a 
toda la colección.

Aquí hay un pequeño problema de programación: escribir una función que toma una cadena y
devuelve la primera palabra que encuentra en esa cadena. Si la función no encuentra un 
espacio en la cadena, significa que toda la cadena es una palabra, por lo que se debe devolver
la cadena completa.

Pensemos en la firma de esta función:

```rust,ignore
fn first_word(s: &String) -> ?
```

Esta función, `first_word`, tiene un parámetro `&String` como parámetro. No queremos la
propiedad, así que está bien. Pero, ¿qué debemos devolver? No tenemos realmente una 
manera de hablar de *parte* de una cadena. Sin embargo, podríamos devolver el índice del
final de la palabra. Intentemos eso como se muestra en Listado 4-10:

<span class="filename">Filename: src/main.rs</span>

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

<span class="caption">Listing 4-10: Listado 4-10: La función `first_word` que devuelve un 
valor de índice de byte al parámetro `String`.</span>

Vamos a descifrar un poco este código. Debido a que necesitamos pasar por el elemento `String` 
elemento por elemento y comprobar si un valor es un espacio, convertiremos nuestra
`String` a un forma de bytes usando el método `as_bytes`:

```rust,ignore
let bytes = s.as_bytes();
```

A continuación, creamos un iterador sobre la matriz de bytes usando el método `iter`:

```rust,ignore
for (i, &item) in bytes.iter().enumerate() {
```

Discutiremos los iteradores con más detalle en el Capítulo 13. Por ahora, debes saber que `iter`
es un método que devuelve cada elemento de una colección, y `enumerate` envuelve 
el resultado de `iter` y devuelve cada elemento como parte de una tupla. El 
primer elemento de la tupla devuelta es el índice, y el segundo elemento es una
referencia al elemento. Esto es un poco más conveniente que calcular el 
índice nosotros mismos.

Debido a que el método `enumerate` devuelve una tupla, podemos usar patrones para
desestructurar esa tupla, igual que en todas partes en Rust. Así que en el bucle 
`for`, especificamos un patrón que tiene `i` para el índice en la tupla y `&item`
para el byte simple en la tupla. Debido a que obtenemos una referencia al elemento de
`.iter().enumerate()`, usamos `&` en el patrón.

Buscamos el byte que representa el espacio utilizando la sintaxis literal del
byte. Si encontramos un espacio, devolvemos la posición. De lo contrario, devolvemos la 
longitud de la cadena utilizando `s.len()`:

```rust,ignore
    if item == b' ' {
        return i;
    }
}
s.len()
```

Ahora tenemos una manera de averiguar el índice del final de la primera palabra en la
cadena, pero hay un problema. Estamos devolviendo un `usize` por su cuenta, pero es
sólo un número significativo en el contexto de la `&String`. En otras palabras,
debido a que es un valor separado de la `String`, no hay garantía de que 
siga siendo válido en el futuro. Considera el programa en Listado 4-11 que
usa la función `first_word` del Listado 4-10:

<span class="filename">Filename: src/main.rs</span>

```rust
# fn first_word(s: &String) -> usize {
#     let bytes = s.as_bytes();
#
#     for (i, &item) in bytes.iter().enumerate() {
#         if item == b' ' {
#             return i;
#         }
#     }
#
#     s.len()
# }
#
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // la palabra recibirá el valor 5.

    s.clear(); // Esto vacía la cadena, haciéndola igual a "".

    // la palabra aquí todavía tiene el valor 5 pero no hay más cadenas que
    // podamos usar significativamente con el valor de 5. la palabra es ahora totalmente inválida!
}
```

<span class="caption">Listing 4-11: Listado 4-11: Almacenar el resultado de la llamada a la función
`first_word` y luego cambiar el contenido de la 'String`</span>

Este programa compila sin errores y también lo haría si utilizamos `word` después
de llamar a `s.clear()`. `word` no está conectado al estado de `s` en absoluto, así
que `word` todavía contiene el valor `5`. Podríamos usar ese valor `5` con la 
variable `s` para tratar de extraer la primera palabra, pero esto sería un error
porque el contenido de `s` ha cambiado desde que guardamos `5` en `word`.

Tener que preocuparse por el índice en `word` que se sale de la sincronización con los datos en 
`s` es tedioso y propenso al error! La gestión de estos índices es aún más frágil si
escribimos una función `second_word`. La firma tendría que verse así:

```rust,ignore
fn second_word(s: &String) -> (usize, usize) {
```

Ahora estamos rastreando un índice inicial *y* un índice final, y tenemos aún más valores 
que fueron calculados a partir de datos en un estado particular pero no están vinculados a 
ese estado en absoluto. Ahora tenemos tres variables no relacionadas que flotan alrededor y que necesitan 
mantenerse sincronizadas.

Afortunadamente, Rust tiene una solución a este problema: las rebanadas de cadenas.

### Rebanadas de Cadenas (String Slice)

Una *rebanada de cadena* (String Slice) es una referencia a una parte de una `String`, y se ve así:

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

Esto es similar a tomar una referencia al conjunto de la `String` pero con el bit
extra `[0..5]`. En lugar de una referencia a toda la `String`, es una referencia
a una parte de la `String`. La sintaxis `start..end` es un rango que comienza en
`start` y continúa, pero no incluye, `end`.

Podemos crear slices usando un rango dentro de los corchetes especificando
`[starting_index...ending_index]`, donde `starting_index` es la primera posición
incluida en el slice y `ending_index` es una posición más que la última posición
incluida en el slice. Internamente, la estructura de datos slice almacena la posición inicial
y la longitud del slice, que corresponde a `end_index` menos 
`starting_index`. Así que en el caso de `let world = &s[6..11]; `, `world` sería un
slice que contendría un puntero al 6º byte de `s` y un valor de longitud de 5. 

La Figura 4-12 muestra esto en un diagrama.

<img alt="world containing a pointer to the 6th byte of String s and a length 5" src="img/trpl04-06.svg" class="center" style="width: 50%;" />

<span class="caption">Figura 4-12: String slice que se refiere a una parte de una
`String`</span>

Con la sintaxis de rango `..` de Rust, si quieres empezar en el primer índice (cero),
puedes reducir el valor antes de los dos periodos. En otras palabras, son iguales:

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

De la misma manera, si tu slice incluye el último byte de la `String`, puedes
dejar caer(drop) el número de seguimiento. Eso significa que son iguales:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

También puede soltar ambos valores para tomar un slice de la cadena entera. Así que
estos son iguales:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

Con toda esta información en mente, reescribamos `first_word` para devolver un 
slice. El tipo que significa "string slice" se escribe como `&str`:

<span class="filename">Filename: src/main.rs</span>

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

Obtenemos el índice para el final de la palabra de la misma manera que lo hicimos en Listing 
4-10, buscando la primera aparición de un espacio. Cuando encontramos un espacio, devolvemos
un string slice utilizando el inicio de la cadena y el índice del espacio 
como índices inicial y final.

Ahora, cuando llamamos `first_word`, recuperamos un único valor que está ligado a los
datos subyacentes. El valor está formado por una referencia al punto de partida del
slice y el número de elementos del mismo.

Devolver un slice también funcionaría para una función `second_word`:

```rust,ignore
fn second_word(s: &String) -> &str {
```

We now have a straightforward API that’s much harder to mess up, since the
compiler will ensure the references into the `String` remain valid. Remember
the bug in the program in Listing 4-11, when we got the index to the end of the
first word but then cleared the string so our index was invalid? That code was
logically incorrect but didn’t show any immediate errors. The problems would
show up later if we kept trying to use the first word index with an emptied
string. Slices make this bug impossible and let us know we have a problem with
our code much sooner. Using the slice version of `first_word` will throw a
compile time error:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // Error!
}
```

Here’s the compiler error:

```text
17:6 error: cannot borrow `s` as mutable because it is also borrowed as
            immutable [E0502]
    s.clear(); // Error!
    ^
15:29 note: previous borrow of `s` occurs here; the immutable borrow prevents
            subsequent moves or mutable borrows of `s` until the borrow ends
    let word = first_word(&s);
                           ^
18:2 note: previous borrow ends here
fn main() {

}
^
```

Recall from the borrowing rules that if we have an immutable reference to
something, we cannot also take a mutable reference. Because `clear` needs to
truncate the `String`, it tries to take a mutable reference, which fails. Not
only has Rust made our API easier to use, but it has also eliminated an entire
class of errors at compile time!

#### String Literals Are Slices

Recall that we talked about string literals being stored inside the binary. Now
that we know about slices, we can properly understand string literals:

```rust
let s = "Hello, world!";
```

The type of `s` here is `&str`: it’s a slice pointing to that specific point of
the binary. This is also why string literals are immutable; `&str` is an
immutable reference.

#### String Slices as Parameters

Knowing that you can take slices of literals and `String`s leads us to one more
improvement on `first_word`, and that’s its signature:

```rust,ignore
fn first_word(s: &String) -> &str {
```

A more experienced Rustacean would write the following line instead because it
allows us to use the same function on both `String`s and `&str`s:

```rust,ignore
fn first_word(s: &str) -> &str {
```

If we have a string slice, we can pass that directly. If we have a `String`, we
can pass a slice of the entire `String`. Defining a function to take a string
slice instead of a reference to a String makes our API more general and useful
without losing any functionality:

<span class="filename">Filename: src/main.rs</span>

```rust
# fn first_word(s: &str) -> &str {
#     let bytes = s.as_bytes();
#
#     for (i, &item) in bytes.iter().enumerate() {
#         if item == b' ' {
#             return &s[0..i];
#         }
#     }
#
#     &s[..]
# }
fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // since string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

### Other Slices

String slices, as you might imagine, are specific to strings. But there’s a
more general slice type, too. Consider this array:

```rust
let a = [1, 2, 3, 4, 5];
```

Just like we might want to refer to a part of a string, we might want to refer
to part of an array and would do so like this:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

This slice has the type `&[i32]`. It works the same way as string slices do, by
storing a reference to the first element and a length. You’ll use this kind of
slice for all sorts of other collections. We’ll discuss these collections in
detail when we talk about vectors in Chapter 8.

## Summary

The concepts of ownership, borrowing, and slices are what ensure memory safety
in Rust programs at compile time. The Rust language gives you control over your
memory usage like other systems programming languages, but having the owner of
data automatically clean up that data when the owner goes out of scope means
you don’t have to write and debug extra code to get this control.

Ownership affects how lots of other parts of Rust work, so we’ll talk about
these concepts further throughout the rest of the book. Let’s move on to the
next chapter and look at grouping pieces of data together in a `struct`.
