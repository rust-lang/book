## Validating References with Lifetimes

When we talked about references in Chapter 4, we left out an important detail:
every reference in Rust has a *lifetime*, which is the scope for which that
reference is valid. Most of the time lifetimes are implicit and inferred, just
like most of the time types are inferred. Similarly to when we have to annotate
types because multiple types are possible, there are cases where the lifetimes
of references could be related in a few different ways, so Rust needs us to
annotate the relationships using generic lifetime parameters so that it can
make sure the actual references used at runtime will definitely be valid.

Yes, it’s a bit unusual, and will be different to tools you’ve used in other
programming languages. Lifetimes are, in some ways, Rust’s most distinctive
feature.

Lifetimes are a big topic that can’t be covered in entirety in this chapter, so
we’ll cover common ways you might encounter lifetime syntax in this chapter to
get you familiar with the concepts. Chapter 19 will contain more advanced
information about everything lifetimes can do.

### Lifetimes Prevent Dangling References

The main aim of lifetimes is to prevent dangling references, which will cause a
program to reference data other than the data we’re intending to reference.
Consider the program in Listing 10-18, with an outer scope and an inner scope.
The outer scope declares a variable named `r` with no initial value, and the
inner scope declares a variable named `x` with the initial value of 5. Inside
the inner scope, we attempt to set the value of `r` as a reference to `x`. Then
the inner scope ends, and we attempt to print out the value in `r`:

```rust,ignore
{
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

<span class="caption">Listing 10-18: An attempt to use a reference whose value
has gone out of scope</span>

> #### Uninitialized Variables Cannot Be Used
>
> The next few examples declare variables without giving them an initial value,
> so that the variable name exists in the outer scope. This might appear to be
> in conflict with Rust not having null. However, if we try to use a variable
> before giving it a value, we’ll get a compile-time error. Try it out!

When we compile this code, we’ll get an error:

```text
error: `x` does not live long enough
   |
6  |         r = &x;
   |              - borrow occurs here
7  |     }
   |     ^ `x` dropped here while still borrowed
...
10 | }
   | - borrowed value needs to live until here
```

The variable `x` doesn’t “live long enough.” Why not? Well, `x` is going to go
out of scope when we hit the closing curly bracket on line 7, ending the inner
scope. But `r` is valid for the outer scope; its scope is larger and we say
that it “lives longer.” If Rust allowed this code to work, `r` would be
referencing memory that was deallocated when `x` went out of scope, and
anything we tried to do with `r` wouldn’t work correctly. So how does Rust
determine that this code should not be allowed?

#### The Borrow Checker

The part of the compiler called the *borrow checker* compares scopes to
determine that all borrows are valid. Listing 10-19 shows the same example from
Listing 10-18 with annotations showing the lifetimes of the variables:

```rust,ignore
{
    let r;                // -------+-- 'a
                          //        |
    {                     //        |
        let x = 5;        // -+-----+-- 'b
        r = &x;           //  |     |
    }                     // -+     |
                          //        |
    println!("r: {}", r); //        |
}                         // -------+
```

<span class="caption">Listing 10-19: Annotations of the lifetimes of `r` and
`x`, named `'a` and `'b` respectively</span>

<!-- Just checking I'm reading this right: the inside block is the b lifetime,
correct? I want to leave a note for production, make sure we can make that
clear -->
<!-- Yes, the inside block for the `'b` lifetime starts with the `let x = 5;`
line and ends with the first closing curly bracket on the 7th line. Do you
think the text art comments work or should we make an SVG diagram that has
nicer looking arrows and labels? /Carol -->

We’ve annotated the lifetime of `r` with `'a` and the lifetime of `x` with
`'b`. As you can see, the inner `'b` block is much smaller than the outer `'a`
lifetime block. At compile time, Rust compares the size of the two lifetimes
and sees that `r` has a lifetime of `'a`, but that it refers to an object with
a lifetime of `'b`. The program is rejected because the lifetime `'b` is
shorter than the lifetime of `'a`: the subject of the reference does not live
as long as the reference.

Let’s look at an example in Listing 10-20 that doesn’t try to make a dangling
reference and compiles without any errors:

```rust
{
    let x = 5;            // -----+-- 'b
                          //      |
    let r = &x;           // --+--+-- 'a
                          //   |  |
    println!("r: {}", r); //   |  |
                          // --+  |
}                         // -----+
```

<span class="caption">Listing 10-20: A valid reference because the data has a
longer lifetime than the reference</span>

Here, `x` has the lifetime `'b`, which in this case is larger than `'a`. This
means `r` can reference `x`: Rust knows that the reference in `r` will always
be valid while `x` is valid.

Now that we’ve shown where the lifetimes of references are in a concrete
example and discussed how Rust analyzes lifetimes to ensure references will
always be valid, let’s talk about generic lifetimes of parameters and return
values in the context of functions.

### Generic Lifetimes in Functions

Let’s write a function that will return the longest of two string slices. We
want to be able to call this function by passing it two string slices, and we
want to get back a string slice. The code in Listing 10-21 should print `The
longest string is abcd` once we’ve implemented the `longest` function:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

<span class="caption">Listing 10-21: A `main` function that calls the `longest`
function to find the longest of two string slices</span>

Note that we want the function to take string slices (which are references, as
we talked about in Chapter 4) since we don’t want the `longest` function to
take ownership of its arguments. We want the function to be able to accept
slices of a `String` (which is the type of the variable `string1`) as well as
string literals (which is what variable `string2` contains).

<!-- why is `a` a slice and `b` a literal? You mean "a" from the string "abcd"? -->
<!-- I've changed the variable names to remove ambiguity between the variable
name `a` and the "a" from the string "abcd". `string1` is not a slice, it's a
`String`, but we're going to pass a slice that refers to that `String` to the
`longest` function (`string1.as_str()` creates a slice that references the
`String` stored in `string1`). We chose to have `string2` be a literal since
the reader might have code with both `String`s and string literals, and the way
most readers first get into problems with lifetimes is involving string slices,
so we wanted to demonstrate the flexibility of taking string slices as
arguments but the issues you might run into because string slices are
references.
All of the `String`/string slice/string literal concepts here are covered
thoroughly in Chapter 4, which is why we put two back references here (above
and below). If these topics are confusing you in this context, I'd be
interested to know if rereading Chapter 4 clears up that confusion.
/Carol -->

Refer back to the “String Slices as Parameters” section of Chapter 4 for more
discussion about why these are the arguments we want.

If we try to implement the `longest` function as shown in Listing 10-22, it
won’t compile:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

<span class="caption">Listing 10-22: An implementation of the `longest`
function that returns the longest of two string slices, but does not yet
compile</span>

Instead we get the following error that talks about lifetimes:

```text
error[E0106]: missing lifetime specifier
   |
1  | fn longest(x: &str, y: &str) -> &str {
   |                                 ^ expected lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the
   signature does not say whether it is borrowed from `x` or `y`
```

The help text is telling us that the return type needs a generic lifetime
parameter on it because Rust can’t tell if the reference being returned refers
to `x` or `y`. Actually, we don’t know either, since the `if` block in the body
of this function returns a reference to `x` and the `else` block returns a
reference to `y`!

As we’re defining this function, we don’t know the concrete values that will be
passed into this function, so we don’t know whether the `if` case or the `else`
case will execute. We also don’t know the concrete lifetimes of the references
that will be passed in, so we can’t look at the scopes like we did in Listings
10-19 and 10-20 in order to determine that the reference we return will always
be valid. The borrow checker can’t determine this either, because it doesn’t
know how the lifetimes of `x` and `y` relate to the lifetime of the return
value. We’re going to add generic lifetime parameters that will define the
relationship between the references so that the borrow checker can perform its
analysis.

### Lifetime Annotation Syntax

Lifetime annotations don’t change how long any of the references involved live.
In the same way that functions can accept any type when the signature specifies
a generic type parameter, functions can accept references with any lifetime
when the signature specifies a generic lifetime parameter. What lifetime
annotations do is relate the lifetimes of multiple references to each other.

Lifetime annotations have a slightly unusual syntax: the names of lifetime
parameters must start with an apostrophe `'`. The names of lifetime parameters
are usually all lowercase, and like generic types, their names are usually very
short. `'a` is the name most people use as a default. Lifetime parameter
annotations go after the `&` of a reference, and a space separates the lifetime
annotation from the reference’s type.

Here’s some examples: we’ve got a reference to an `i32` without a lifetime
parameter, a reference to an `i32` that has a lifetime parameter named `'a`,
and a mutable reference to an `i32` that also has the lifetime `'a`:

```rust,ignore
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

One lifetime annotation by itself doesn’t have much meaning: lifetime
annotations tell Rust how the generic lifetime parameters of multiple
references relate to each other. If we have a function with the parameter
`first` that is a reference to an `i32` that has the lifetime `'a`, and the
function has another parameter named `second` that is another reference to an
`i32` that also has the lifetime `'a`, these two lifetime annotations that have
the same name indicate that the references `first` and `second` must both live
as long as the same generic lifetime.

### Lifetime Annotations in Function Signatures

Let’s look at lifetime annotations in the context of the `longest` function
we’re working on. Just like generic type parameters, generic lifetime
parameters need to be declared within angle brackets between the function name
and the parameter list. The constraint we want to tell Rust about for the
references in the parameters and the return value is that they all must have
the same lifetime, which we’ll name `'a` and add to each reference as shown in
Listing 10-23:

<span class="filename">Filename: src/main.rs</span>

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

<span class="caption">Listing 10-23: The `longest` function definition that
specifies all the references in the signature must have the same lifetime,
`'a`</span>

This will compile and will produce the result we want when used with the `main`
function in Listing 10-21.

The function signature now says that for some lifetime `'a`, the function will
get two parameters, both of which are string slices that live at least as long
as the lifetime `'a`. The function will return a string slice that also will
last at least as long as the lifetime `'a`. This is the contract we are telling
Rust we want it to enforce.

By specifying the lifetime parameters in this function signature, we are not
changing the lifetimes of any values passed in or returned, but we are saying
that any values that do not adhere to this contract should be rejected by the
borrow checker. This function does not know (or need to know) exactly how long
`x` and `y` will live, but only needs to know that there is some scope that
can be substituted for `'a` that will satisfy this signature.

When annotating lifetimes in functions, the annotations go on the function
signature, and not in any of the code in the function body. This is because
Rust is able to analyze the code within the function without any help, but when
a function has references to or from code outside that function, the lifetimes
of the arguments or return values will potentially be different each time the
function is called. This would be incredibly costly and often impossible for
Rust to figure out. In this case, we need to annotate the lifetimes ourselves.

When concrete references are passed to `longest`, the concrete lifetime that
gets substituted for `'a` is the part of the scope of `x` that overlaps with
the scope of `y`. Since scopes always nest, another way to say this is that the
generic lifetime `'a` will get the concrete lifetime equal to the smaller of
the lifetimes of `x` and `y`. Because we’ve annotated the returned reference
with the same lifetime parameter `'a`, the returned reference will therefore be
guaranteed to be valid as long as the shorter of the lifetimes of `x` and `y`.

Let’s see how this restricts the usage of the `longest` function by passing in
references that have different concrete lifetimes. Listing 10-24 is a
straightforward example that should match your intuition from any language:
`string1` is valid until the end of the outer scope, `string2` is valid until
the end of the inner scope, and `result` references something that is valid
until the end of the inner scope. The borrow checker approves of this code; it
will compile and print `The longest string is long string is long` when run:

<span class="filename">Filename: src/main.rs</span>

```rust
# fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
#     if x.len() > y.len() {
#         x
#     } else {
#         y
#     }
# }
#
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```

<span class="caption">Listing 10-24: Using the `longest` function with
references to `String` values that have different concrete lifetimes</span>

Next, let’s try an example that will show that the lifetime of the reference in
`result` must be the smaller lifetime of the two arguments. We’ll move the
declaration of the `result` variable outside the inner scope, but leave the
assignment of the value to the `result` variable inside the scope with
`string2`. Next, we’ll move the `println!` that uses `result` outside of the
inner scope, after it has ended. The code in Listing 10-25 will not compile:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

<span class="caption">Listing 10-25: Attempting to use `result` after `string2`
has gone out of scope won’t compile</span>

If we try to compile this, we’ll get this error:

```text
error: `string2` does not live long enough
   |
6  |         result = longest(string1.as_str(), string2.as_str());
   |                                            ------- borrow occurs here
7  |     }
   |     ^ `string2` dropped here while still borrowed
8  |     println!("The longest string is {}", result);
9  | }
   | - borrowed value needs to live until here
```

The error is saying that in order for `result` to be valid for the `println!`,
`string2` would need to be valid until the end of the outer scope. Rust knows
this because we annotated the lifetimes of the function parameters and return
values with the same lifetime parameter, `'a`.

Podemos ver este código como personas y ver que `string1` es más largo, y
por lo tanto `result` contendrá una referencia a` string1`. Porque `string1` aún
no salido de su alcance, una referencia a `string1` seguirá siendo válida para el
`println!`. Sin embargo, lo que le hemos dicho a Rust con los parámetros de vida es que
la duración de la referencia devuelta por la función `longest` sea la misma que
la menor de las vidas de las referencias pasadas. Por lo tanto, el comprobador
no permite que el código del listado 10-25 tenga posiblemente un código de referencia
inválido.

Intente diseñar algunos experimentos más que varíen los valores y tiempos de vida de las
referencias pasadas a la función `longest` y cómo se usa la referencia
devuelta. Haga hipótesis sobre si sus experimentos pasarán el comprobador
o no, antes de compilar, ¡luego verifique si tiene razón!

### Pensando en Términos de Vidas

La forma exacta de especificar los parámetros de vida depende de cuál es la función 
que aplicas. Por ejemplo, si cambiamos la implementación de la función `longest`
para devolver siempre el primer argumento en lugar del segmento de cadena más largo,
no necesitaríamos especificar un tiempo de vida en el parámetro `y`. Este código compila:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

En este ejemplo, hemos especificado un parámetro de vida en `'a` para el parámetro
`x` y el tipo de retorno, pero no para el parámetro `y`, ya que la vida útil de
`y` no tiene ninguna relación con la vida útil de `x` o su valor de retorno.

Al devolver una referencia desde una función, el parámetro de duración para el
tipo de devolución debe coincidir con el parámetro de duración de uno de los argumentos. Si
la referencia devuelta *no* se refiere a uno de los argumentos, la única 
posibilidad es que se refiere a un valor creado dentro de esta función, que
sería una referencia pendiente ya que el valor saldrá del alcance al final
de la función. Considere esta implementación intentada de la función `longest`
que no se compilará:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust,ignore
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

A pesar de que hemos especificado un parámetro de vida de `'a` para el tipo de retorno, esta
implementación falla al compilar porque la vida útil del valor de retorno no está
relacionada con la vida de los parámetros en absoluto. Aquí está el mensaje de error que
obtenemos:

```text
error: `result` does not live long enough
  |
3 |     result.as_str()
  |     ^^^^^^ does not live long enough
4 | }
  | - borrowed value only lives until here
  |
note: borrowed value must be valid for the lifetime 'a as defined on the block
at 1:44...
  |
1 | fn longest<'a>(x: &str, y: &str) -> &'a str {
  |                                             ^
```

El problema es que `result` saldrá del alcance y se limpiará al final
de la función `longest`, y estamos tratando de devolver una referencia al `result`
de la función. No hay manera de que podamos especificar parámetros de vida que
cambien la función pendiente, y Rust no nos dejará crear una referencia
pendiente. En este caso, la mejor solución sería devolver un tipo de datos de propiedad
en lugar de una referencia para que la función de llamada sea la responsable de
arreglar el valor expuesto.

En última instancia, la sintaxis de la vida útil se trata de conectar las vidas de varios
argumentos y devolver valores de funciones. Una vez que están conectados, Rust tiene
suficiente información para permitir operaciones de seguridad de memoria y no permitir operaciones que
creen punteros pendientes de otra manera esto violaría la seguridad de la memoria.

### Anotaciones de por Vida en las Definiciones de la Estructura

Hasta ahora, solo hemos definido estructuras para mantener los tipos de propiedad. Es posible
para las estructuras contener referencias, pero tenemos que agregar una anotación de por vida en
cada referencia en la definición de la estructura. El listado 10-26 tiene una estructura llamada
`ImportantExcerpt` que contiene un segmento de cadena:

<span class="filename">Nombre de archivo: src/main.rs</span>

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}
```

<span class="caption">Listado 10-26: Una estructura que contiene una referencia, por lo que
su definición necesita una anotación de vida</span>

Esta estructura tiene un campo, `part`, que contiene una parte de cuerda, que es una
referencia. Al igual que con los tipos de datos genéricos, tenemos que declarar el nombre de
el parámetro de vida genérico dentro de los corchetes angulares después del nombre de la
estructura para que podamos usar el parámetro de duración en el cuerpo de la definición 
de la estructura.

La función `main` aquí crea una instancia de estructura tipo `ImportantExcerpt` 
que contiene una referencia a la primera frase de la `String` propiedad de la
variable `novel`.

### Lifetime Elision

En esta sección, hemos aprendido que cada referencia tiene una vida, y necesitamos
especificar parámetros de vida para funciones o estructuras que usan referencias.
Sin embargo, en el Capítulo 4 teníamos una función en la sección "String slices", que se muestra
de nuevo en el listado 10-27, compilado sin anotaciones de vida:

<span class="filename">Nombre del archivo src/lib.rs</span>

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

<span class="caption">Listado 10-27: Una función que definimos en el Capítulo 4 que
compilamos sin anotaciones de vida, aunque el parámetro y el tipo de 
retorno son referencias</span>

La razón por la que esta función se compila sin anotaciones de vida es histórica:
en versiones anteriores de Rust pre-1.0, esto de hecho no se habría compilado. Cada
referencia necesitaba una vida explícita. En ese momento, la firma de la función
huvbiera estado escrita algo así:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

Después de escribir mucho código de Rust, el equipo de Rust descubrió que los programadores de Rust
estaban escribiendo las mismas anotaciones de vida una y otra vez en situaciones
particulares. Estas situaciones fueron predecibles y siguieron algunos patrones
deterministas. El equipo de Rust luego programó estos patrones en el compilador de códigos
de Rust para que el comprobador pueda inferir las vidas en estas situaciones
sin forzar al programador a agregar explícitamente las anotaciones.

Mencionamos esta parte de la historia de Rust porque es muy posible que más
patrones deterministas surgirán y se agregarán al compilador. En el futuro,
incluso se necesitarán menos anotaciones de vida.

Los patrones programados en el análisis de referencias de Rust se llaman
*lifetime elision rules*. Estas no son reglas que los programadores deben seguir; las
reglas son un conjunto de casos particulares que el compilador considerará, y si
su código se ajusta a estos casos, no necesita escribir las vidas explícitamente.

Las reglas de elisión no proporcionan una inferencia completa: si Rust deterministamente
aplica las reglas, pero todavía hay ambigüedad en cuanto a qué vidas de
referencias tiene, no adivinará en algunos casos cuales son las referencias
que deberían ser. En este caso, el compilador le dará un error que puede ser
resuelto agregando las anotaciones de por vida que corresponden a tus intenciones
de cómo las referencias se relacionan entre sí.

Primero, algunas definiciones: se llaman periodos de vida en función o parámetros del método
*input lifetimes*, y las vidas en los valores de retorno se llaman *output lifetimes*.

Ahora, a las reglas que usa el compilador para averiguar qué referencias
de vidas tiene cuando no hay anotaciones explícitas. La primera regla aplica
para ingresar tiempos de vida, y las segundas dos reglas se aplican a las vidas útiles de salida. Si el
compilador llega al final de las tres reglas y todavía hay referencias que
no se puede calcular sus duraciones, el compilador se detendrá con un error.

1. Cada parámetro que es una referencia obtiene su propio parámetro de por vida. En otras
   palabras, una función con un parámetro obtiene un parámetro de vida: `fn
   foo<'a>(x: &'a i32)`,una función con dos argumentos obtiene dos por parámetros
   de vida por separado: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`, y así.

2. Si hay exactamente un parámetro de vida útil de entrada, se asigna esa duración
   a todos los parámetros de vida útil de salida: `fn foo<'a>(x: &'a i32) -> &'a i32`.

3.Si hay múltiples parámetros de duración de entrada, pero uno de ellos es `&self`
   o `&mut self` porque este es un método, entonces la vida de `self` es
   asignada a todos los parámetros de vida útil de salida. Esto hace que los métodos de escritura sean
   mucho mejores.

Let’s pretend we’re the compiler and apply these rules to figure out what the
lifetimes of the references in the signature of the `first_word` function in
Listing 10-27 are. The signature starts without any lifetimes associated with
the references:

```rust,ignore
fn first_word(s: &str) -> &str {
```

Then we (as the compiler) apply the first rule, which says each parameter gets
its own lifetime. We’re going to call it `'a` as usual, so now the signature is:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &str {
```

On to the second rule, which applies because there is exactly one input
lifetime. The second rule says the lifetime of the one input parameter gets
assigned to the output lifetime, so now the signature is:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

Now all the references in this function signature have lifetimes, and the
compiler can continue its analysis without needing the programmer to annotate
the lifetimes in this function signature.

Let’s do another example, this time with the `longest` function that had no
lifetime parameters when we started working with in Listing 10-22:

```rust,ignore
fn longest(x: &str, y: &str) -> &str {
```

Pretending we’re the compiler again, let’s apply the first rule: each parameter
gets its own lifetime. This time we have two parameters, so we have two
lifetimes:

```rust,ignore
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

Looking at the second rule, it doesn’t apply since there is more than one input
lifetime. Looking at the third rule, this also does not apply because this is a
function rather than a method, so none of the parameters are `self`. So we’re
out of rules, but we haven’t figured out what the return type’s lifetime is.
This is why we got an error trying to compile the code from Listing 10-22: the
compiler worked through the lifetime elision rules it knows, but still can’t
figure out all the lifetimes of the references in the signature.

Because the third rule only really applies in method signatures, let’s look at
lifetimes in that context now, and see why the third rule means we don’t have
to annotate lifetimes in method signatures very often.

### Lifetime Annotations in Method Definitions

<!-- Is this different to the reference lifetime annotations, or just a
finalized explanation? -->
<!-- This is about lifetimes on references in method signatures, which is where
the 3rd lifetime elision rule kicks in. It can also be confusing where lifetime
parameters need to be declared and used since the lifetime parameters could go
with the struct's fields or with references passed into or returned from
methods. /Carol -->

When we implement methods on a struct with lifetimes, the syntax is again the
same as that of generic type parameters that we showed in Listing 10-11: the
place that lifetime parameters are declared and used depends on whether the
lifetime parameter is related to the struct fields or the method arguments and
return values.

Lifetime names for struct fields always need to be declared after the `impl`
keyword and then used after the struct’s name, since those lifetimes are part
of the struct’s type.

In method signatures inside the `impl` block, references might be tied to the
lifetime of references in the struct’s fields, or they might be independent. In
addition, the lifetime elision rules often make it so that lifetime annotations
aren’t necessary in method signatures. Let’s look at some examples using the
struct named `ImportantExcerpt` that we defined in Listing 10-26.

First, here’s a method named `level`. The only parameter is a reference to
`self`, and the return value is just an `i32`, not a reference to anything:

```rust
# struct ImportantExcerpt<'a> {
#     part: &'a str,
# }
#
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

The lifetime parameter declaration after `impl` and use after the type name is
required, but we’re not required to annotate the lifetime of the reference to
`self` because of the first elision rule.

Here’s an example where the third lifetime elision rule applies:

```rust
# struct ImportantExcerpt<'a> {
#     part: &'a str,
# }
#
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

There are two input lifetimes, so Rust applies the first lifetime elision rule
and gives both `&self` and `announcement` their own lifetimes. Then, because
one of the parameters is `&self`, the return type gets the lifetime of `&self`,
and all lifetimes have been accounted for.

### The Static Lifetime

There is *one* special lifetime we need to discuss: `'static`. The `'static`
lifetime is the entire duration of the program. All string literals have the
`'static` lifetime, which we can choose to annotate as follows:

```rust
let s: &'static str = "I have a static lifetime.";
```

The text of this string is stored directly in the binary of your program and
the binary of your program is always available. Therefore, the lifetime of all
string literals is `'static`.

<!-- How would you add a static lifetime (below)? -->
<!-- Just like you'd specify any lifetime, see above where it shows `&'static str`. /Carol -->

You may see suggestions to use the `'static` lifetime in error message help
text, but before specifying `'static` as the lifetime for a reference, think
about whether the reference you have is one that actually lives the entire
lifetime of your program or not (or even if you want it to live that long, if
it could). Most of the time, the problem in the code is an attempt to create a
dangling reference or a mismatch of the available lifetimes, and the solution
is fixing those problems, not specifying the `'static` lifetime.

### Generic Type Parameters, Trait Bounds, and Lifetimes Together

Let’s briefly look at the syntax of specifying generic type parameters, trait
bounds, and lifetimes all in one function!

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

This is the `longest` function from Listing 10-23 that returns the longest of
two string slices, but with an extra argument named `ann`. The type of `ann` is
the generic type `T`, which may be filled in by any type that implements the
`Display` trait as specified by the `where` clause. This extra argument will be
printed out before the function compares the lengths of the string slices,
which is why the `Display` trait bound is necessary. Because lifetimes are a
type of generic, the declarations of both the lifetime parameter `'a` and the
generic type parameter `T` go in the same list within the angle brackets after
the function name.

## Summary

We covered a lot in this chapter! Now that you know about generic type
parameters, traits and trait bounds, and generic lifetime parameters, you’re
ready to write code that isn’t duplicated but can be used in many different
situations. Generic type parameters mean the code can be applied to different
types. Traits and trait bounds ensure that even though the types are generic,
those types will have the behavior the code needs. Relationships between the
lifetimes of references specified by lifetime annotations ensure that this
flexible code won’t have any dangling references. And all of this happens at
compile time so that run-time performance isn’t affected!

Believe it or not, there’s even more to learn in these areas: Chapter 17 will
discuss trait objects, which are another way to use traits. Chapter 19 will be
covering more complex scenarios involving lifetime annotations. Chapter 20 will
get to some advanced type system features. Up next, though, let’s talk about
how to write tests in Rust so that we can make sure our code using all these
features is working the way we want it to!
