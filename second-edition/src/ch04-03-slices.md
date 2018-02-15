## Slices

Another data type that does not have ownership is the *slice*. Slices let you
reference a contiguous sequence of elements in a collection rather than the
whole collection.

Here’s a small programming problem: write a function that takes a string and
returns the first word it finds in that string. If the function doesn’t find a
space in the string, it means the whole string is one word, so the entire
string should be returned.

Let’s think about the signature of this function:

```rust,ignore
fn first_word(s: &String) -> ?
```

This function, `first_word`, has a `&String` as a parameter. We don’t want
ownership, so this is fine. But what should we return? We don’t really have a
way to talk about *part* of a string. However, we could return the index of the
end of the word. Let’s try that as shown in Listing 4-10:

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

<span class="caption">Listing 4-10: The `first_word` function that returns a
byte index value into the `String` parameter</span>

Let’s break down this code a bit. Because we need to go through the `String`
element by element and check whether a value is a space, we’ll convert our
`String` to an array of bytes using the `as_bytes` method:

```rust,ignore
let bytes = s.as_bytes();
```

Next, we create an iterator over the array of bytes using the `iter` method :

```rust,ignore
for (i, &item) in bytes.iter().enumerate() {
```

We’ll discuss iterators in more detail in Chapter 13. For now, know that `iter`
is a method that returns each element in a collection, and `enumerate` wraps
the result of `iter` and returns each element as part of a tuple instead. The
first element of the returned tuple is the index, and the second element is a
reference to the element. This is a bit more convenient than calculating the
index ourselves.

Because the `enumerate` method returns a tuple, we can use patterns to
destructure that tuple, just like everywhere else in Rust. So in the `for`
loop, we specify a pattern that has `i` for the index in the tuple and `&item`
for the single byte in the tuple. Because we get a reference to the element
from `.iter().enumerate()`, we use `&` in the pattern.

We search for the byte that represents the space by using the byte literal
syntax. If we find a space, we return the position. Otherwise, we return the
length of the string by using `s.len()`:

```rust,ignore
    if item == b' ' {
        return i;
    }
}
s.len()
```

We now have a way to find out the index of the end of the first word in the
string, but there’s a problem. We’re returning a `usize` on its own, but it’s
only a meaningful number in the context of the `&String`. In other words,
because it’s a separate value from the `String`, there’s no guarantee that it
will still be valid in the future. Consider the program in Listing 4-11 that
uses the `first_word` function from Listing 4-10:

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

    let word = first_word(&s); // word will get the value 5.

    s.clear(); // This empties the String, making it equal to "".

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```

<span class="caption">Listing 4-11: Storing the result from calling the
`first_word` function then changing the `String` contents</span>

This program compiles without any errors and also would if we used `word` after
calling `s.clear()`. `word` isn’t connected to the state of `s` at all, so
`word` still contains the value `5`. We could use that value `5` with the
variable `s` to try to extract the first word out, but this would be a bug
because the contents of `s` have changed since we saved `5` in `word`.

Having to worry about the index in `word` getting out of sync with the data in
`s` is tedious and error prone! Managing these indices is even more brittle if
we write a `second_word` function. Its signature would have to look like this:

```rust,ignore
fn second_word(s: &String) -> (usize, usize) {
```

Now we’re tracking a start *and* an ending index, and we have even more values
that were calculated from data in a particular state but aren’t tied to that
state at all. We now have three unrelated variables floating around that need
to be kept in sync.

Luckily, Rust has a solution to this problem: string slices.

### String Slices

A *string slice* is a reference to part of a `String`, and looks like this:

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

This is similar to taking a reference to the whole `String` but with the extra
`[0..5]` bit. Rather than a reference to the entire `String`, it’s a reference
to a portion of the `String`. The `start..end` syntax is a range that begins at
`start` and continues up to, but not including, `end`.

We can create slices using a range within brackets by specifying
`[starting_index..ending_index]`, where `starting_index` is the first position
included in the slice and `ending_index` is one more than the last position
included in the slice. Internally, the slice data structure stores the starting
position and the length of the slice, which corresponds to `ending_index` minus
`starting_index`. So in the case of `let world = &s[6..11];`, `world` would be
a slice that contains a pointer to the 6th byte of `s` and a length value of 5.

Figure 4-12 shows this in a diagram.

<img alt="world containing a pointer to the 6th byte of String s and a length 5" src="img/trpl04-06.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 4-12: String slice referring to part of a
`String`</span>

With Rust’s `..` range syntax, if you want to start at the first index (zero),
you can drop the value before the two periods. In other words, these are equal:

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

By the same token, if your slice includes the last byte of the `String`, you
can drop the trailing number. That means these are equal:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

You can also drop both values to take a slice of the entire string. So these
are equal:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

With all this information in mind, let’s rewrite `first_word` to return a
slice. The type that signifies “string slice” is written as `&str`:

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

We get the index for the end of the word in the same way as we did in Listing
4-10, by looking for the first occurrence of a space. When we find a space, we
return a string slice using the start of the string and the index of the space
as the starting and ending indices.

Now when we call `first_word`, we get back a single value that is tied to the
underlying data. The value is made up of a reference to the starting point of
the slice and the number of elements in the slice.

Returning a slice would also work for a `second_word` function:

```rust,ignore
fn second_word(s: &String) -> &str {
```

Ahora tenemos una API sencilla que es mucho más difícil de desordenar, ya que el
compilador garantizará que las referencias a la `String` siguen siendo válidas. ¿Recuerdas
el fallo en el programa de Listado 4-11, cuando obtuvimos el índice al final de la
primera palabra, pero luego se borró la cadena y que nuestro índice fue inválido? Ese código era
lógicamente incorrecto pero no mostraba ningún error inmediato. Los problemas aparecerían
más tarde si continuamos intentando usar el índice de la primera palabra con una cadena 
vacía. Slices hace este error imposible y nos hace saber que tenemos un problema con 
nuestro código mucho antes. Usando la versión slice de `first_word` lanzará un
error de compilación de tiempo:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // Error!
}
```

Aquí está el error del compilador:

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

Recordemos las normas de préstamo que si tenemos una referencia inmutable
a algo, no podemos también tomar una referencia mutable.  Como `clear` necesita 
truncar la `String`, intenta tomar una referencia mutable, que falla. Rust no
sólo ha hecho que nuestra API sea más fácil de usar, sino que también ha eliminado toda 
una serie de errores en el momento de la compilación!.

#### Las Cadenas Literales son Slices

Recordemos que hablamos de que las cadenas literales se guardan dentro del binario. Ahora
que sabemos de las slices, podemos entender bien los las cadenas literales:

```rust
let s = "Hello, world!";
```

El tipo de `s`aquí es `&str`: es una slice que apunta a ese punto específico del
binario. Por eso también las cadenas literales son inmutables; `&str` es una 
referencia inmutable.

#### Cadenas Slices como Parámetros

Sabiendo que puedes tomar slices de literales y `String`s nos lleva a una 
mejora más en `first_word`, y esa es su firma:

```rust,ignore
fn first_word(s: &String) -> &str {
```

Un Rustacean con más experiencia escribiría la siguiente línea en su lugar porque
nos permite usar la misma función tanto en `String`s como en `&str`s:

```rust,ignore
fn first_word(s: &str) -> &str {
```

Si tenemos una cadena slice, podemos pasar eso directamente. Si tenemos una `String`, podemos 
pasar una slice de toda la `String`. Definir una función para tomar una cadena
slice en lugar de una referencia a una String hace que nuestra API sea más general y útil
sin perder ninguna funcionalidad:

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

    // first_word funciona en slices de `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word funciona en slices de cadenas literales
    let word = first_word(&my_string_literal[..]);

    // ya que las cadenas literales ya *son* slices de cadena,
    // esto también funciona, ¡sin la sintaxis de slice!
    let word = first_word(my_string_literal);
}
```

### Otras Slices

Las Cadenas slice, como podrías imaginarte, son específicas de las cadenas. Pero también 
hay un tipo de slice más general. Considera este arreglo:

```rust
let a = [1, 2, 3, 4, 5];
```

Al igual que podríamos querer referirnos a una parte de una cadena, podríamos querer referirnos
a una parte de una array y hacerlo así:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

Esta slice tiene el tipo `&[i32]`. Funciona del mismo modo que las slices de cadena lo hacen, 
almacenando una referencia al primer elemento y una longitud. Usarás este tipo de 
slices para todo tipo de colecciones. Discutiremos estas colecciones en 
detalle cuando hablemos de vectores en el Capítulo 8.

## Resumen

Los conceptos de propiedad, préstamo y slices son los que aseguran la seguridad de la memoria 
en los programas de Rust al momento de la compilación. El lenguaje Rust te da control sobre el 
uso de la memoria como otros lenguajes de programación de sistemas, pero tener la propiedad de 
los datos automáticamente limpia los datos cuando el propietario sale fuera de alcance significa 
que tu no tienes que escribir y depurar código extra para obtener este control.

La propiedad afecta la forma en que muchas otras partes de Rust trabajan, por lo que hablaremos sobre 
estos conceptos durante el resto del libro. Pasemos al
siguiente capítulo y veamos la agrupación de datos en una `struct`.
