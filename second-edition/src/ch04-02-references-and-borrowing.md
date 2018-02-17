## Referencias y Préstamos

El problema con el código tuple al final de la sección anterior es que tenemos que
devolver la `String` a la función de llamada para que podamos seguir usando la
`String` después de la llamada a `calculate_length`, porque la `String` se ha movido
a `calculate_length`.

A continuación se explica cómo definiría y utilizaría una función `calculate_length` que tiene
una *referencia* a un objeto como parámetro en lugar de tomar posesión del
valor:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

Primero, nota que todo el código tuple en la declaración variable y el
valor de retorno de función ha desaparecido. Segundo, nota que pasamos `&s1` a
`calculate_length`, y en su definición, tomamos `&String` en lugar de
`String`.

Estos ampersands(&) son *referencias*, y te permiten referirte a algún valor
sin apropiarte de él. La Figura 4-5 muestra un diagrama.

<img alt="&String s pointing at String s1" src="img/trpl04-05.svg" class="center" />

<span class="caption">Figura 4-5: `&String s` señalando `String s1`.</span>

> Note: The opposite of referencing by using `&` is *dereferencing*, which is
> accomplished with the dereference operator, `*`. We’ll see some uses of the
> dereference operator in Chapter 8 and discuss details of dereferencing in
> Chapter 15.

Echemos un vistazo más de cerca de la llamada a la función:

```rust
# fn calculate_length(s: &String) -> usize {
#     s.len()
# }
let s1 = String::from("hello");

let len = calculate_length(&s1);
```

La sintaxis `&s1` nos permite crear una referencia que *refiere* al valor de `s1`
pero que no lo posee. Debido a que no es de su propiedad, el valor al que apunta no se
dejará caer cuando la referencia salga del alcance.

Del mismo modo, la firma de la función utiliza `&` para indicar que el tipo de
parámetro `s` es una referencia. Añadamos algunas anotaciones explicativas:

```rust
fn calculate_length(s: &String) -> usize { // s es una referencia a un String
    s.len()
} // Aquí, s sale del alcance. Pero debido a que no tiene la propiedad
  // a la que se refiere, no pasa nada.
```

El alcance en el que la variable `s` es válida es el mismo que cualquier parámetro
de la función de scope, pero no dejamos caer lo que la referencia indica cuando se sale
del alcance porque no tenemos propiedad. Las funciones que tienen referencias como parámetros en
lugar de los valores reales significan que no necesitaremos devolver los valores
para devolver la propiedad, ya que nunca tuvimos la propiedad.

Llamamos tener referencias como parámetros de la función *préstamo*. Como en la vida real,
si una persona es dueña de algo, puedes tomarlo prestado. Cuando termines, tienes
que devolverlo.

¿Qué pasa si tratamos de modificar algo que pedimos prestado? Pruebe el código en
Listado 4-4. Alerta de spoiler: ¡no funciona!

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

<span class="caption">Listado 4-4: Intentar modificar un valor prestado</span>

Aquí está el error:

```text
error[E0596]: cannot borrow immutable borrowed content `*some_string` as mutable
 --> error.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- use `&mut String` here to make mutable
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^ cannot borrow as mutable
```

Así como las variables son inmutables por defecto, también lo son las referencias. No se nos
permite modificar algo a lo que tenemos una referencia.

### Referencias Mutables

Podemos corregir el error en el código de Listing 4-4 con sólo un pequeño ajuste:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

Primero, tuvimos que cambiar los `s`para ser `mut`. Luego tuvimos que crear una referencia
mutable con `&mut s` y aceptar una referencia mutable con `some_string: &mut
String`.

Pero las referencias mutables tienen una gran restricción: sólo puedes tener una referencia
mutable en un dato concreto en un scope particular. Este código
fallará:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;
```

Aquí está el error:

```text
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> borrow_twice.rs:5:19
  |
4 |     let r1 = &mut s;
  |                   - first mutable borrow occurs here
5 |     let r2 = &mut s;
  |                   ^ second mutable borrow occurs here
6 | }
  | - first borrow ends here
```

Esta restricción permite la mutación pero de una manera muy controlada. Es
algo con lo que los nuevos Rustaceos luchan, porque la mayoría de los lenguajes te
permiten mutar cuando quieras. El beneficio de tener esta restricción es que Rust
puede prevenir carreras de datos en el momento de compilar.

Una *carrera de datos* es similar a una condición de carrera, que ocurre cuando
suceden estos tres comportamientos:

1. Dos o más punteros acceden a los mismos datos al mismo tiempo.
1. Al menos uno de los punteros se está usando para escribir los datos.
1. No se utiliza ningún mecanismo para sincronizar el acceso a los datos.

Las carreras de datos causan un comportamiento indefinido y pueden ser difíciles de diagnosticar y arreglar
cuando se está tratando de localizarlas en tiempo de ejecución; Rust previene que este problema
suceda porque ni siquiera compila código con las carreras de datos!

Como siempre, podemos utilizar llaves para crear un nuevo scope, permitiendo
múltiples referencias mutables, pero no *simultáneas*:

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;

} // r1 se sale del ámbito de aplicación, por lo que podemos hacer una nueva referencia sin problemas.

let r2 = &mut s;
```

Una regla similar existe para combinar referencias mutables e inmutables. Este código
provoca un error:

```rust,ignore
let mut s = String::from("hello");

let r1 = &s; // sin problema
let r2 = &s; // sin problema
let r3 = &mut s; // GRAN PROBLEMA
```

Aquí el error:

```text
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as
immutable
 --> borrow_thrice.rs:6:19
  |
4 |     let r1 = &s; // sin problema
  |               - immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // GRAN PROBLEMA
  |                   ^ mutable borrow occurs here
7 | }
  | - immutable borrow ends here
```

¡Uf! Nosotros *también* no podemos tener una referencia mutable mientras que tenemos una inmutable.
Los usuarios de una referencia inmutable no esperan que los valores cambien repentinamente
de debajo de ellos! Sin embargo, múltiples referencias inmutables están bien porque nadie que
esté leyendo los datos tiene la habilidad de afectar la lectura de los datos de cualquier
otra persona.

A pesar de que a veces estos errores pueden ser frustrantes, recuerda que es el
compilador de Rust señalando un potencial error temprano (en tiempo de compilación en lugar de
en tiempo de ejecución) y mostrarte exactamente dónde está el problema en lugar de tener que
buscar por qué a veces tus datos no son lo que pensabas que deberían ser.

### Referencias Colgantes

En los lenguajes con punteros, es fácil crear erróneamente un *puntero
colgante*, un puntero que hace referencia a una ubicación en la memoria que puede haber
sido dada a alguien más, liberando algo de memoria mientras se conserva un puntero a
esa memoria. En Rust, por el contrario, el compilador garantiza que las referencias nunca
serán referencias colgantes: si tenemos una referencia a algunos datos, el compilador
se asegurará de que los datos no salgan del scope antes que la referencia a los
datos.

Tratemos de crear una referencia colgante, which Rust will prevent with a
compile-time error:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

Aquí el error:

```text
error[E0106]: missing lifetime specifier
 --> dangle.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is
  no value for it to be borrowed from
  = help: consider giving it a 'static lifetime
```

Este mensaje de error se refiere a una característica que aún no hemos cubierto: *vida útil*.
Discutiremos la vida útil en detalle en el capítulo 10. Pero, si tu no haces caso de
las partes sobre vida útil, el mensaje contiene la llave de por qué este código es
un problema:

```text
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from.
```

Echemos un vistazo más de cerca a lo que está sucediendo exactamente en cada etapa de nuestro
código `dangle`:

```rust,ignore
fn dangle() -> &String { // dangle regresa una referencia a String

    let s = String::from("hello"); // s es una nueva String

    &s // devolvemos una referencia a String, s
} // Aquí, s sale fuera del alcance, y se deja caer. Su memoria desaparece.
  // peligro!
```

Debido a que la `s`se crea dentro del `dangle`, cuando el código de `dángle` está terminado,
`s`se deslocalizará. Pero intentamos devolverle una referencia. Eso significa
que esta referencia estaría apuntando a una "`String` inválida! Eso no es bueno. Rust
no nos deja hacer esto.

La solución aquí es devolver la `String` directamente:

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

Esto funciona sin problemas. La propiedad se retira, y nada se
deslocaliza.

### El Reglamento de Referencias

Recapitulemos lo que hemos discutido sobre las referencias:

1. En cualquier momento dado, puedes tener *cualquiera* pero no ambos:
  * Una referencia mutable.
  * Cualquier número de referencias inmutables.
2. Las referencias deben ser siempre válidas.

A continuación, veremos un tipo de referencia diferente: porciones.
