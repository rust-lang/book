## Apéndice D - Herramientas útiles para el desarrollo

In this appendix, we talk about some useful development tools that the Rust
project provides. We’ll look at automatic formatting, quick ways to apply
warning fixes, a linter, and integrating with IDEs.

### Formateo automático con `rustfmt`

La herramienta `rustfmt` reformatea su código según el estilo de código de la comunidad.
Muchos proyectos colaborativos utilizan `rustfmt` para evitar discusiones sobre qué
estilo a utilizar cuando se escribe Rust: todo el mundo formatea su código utilizando la herramienta.

Para instalar `rustfmt`, introduzca lo siguiente:

```console
$ rustup component add rustfmt
```

Este comando te da `rustfmt` y `cargo-fmt`, de forma similar a como Rust te da
tanto `rustc` como `cargo`. Para formatear cualquier proyecto Cargo, introduzca lo siguiente:

```console
$ cargo fmt
```

La ejecución de este comando reformatea todo el código Rust en el crate actual. Este
sólo debería cambiar el estilo del código, no la semántica del mismo. Para más información
de `rustfmt`, ver [its documentation][rustfmt].

[rustfmt]: https://github.com/rust-lang/rustfmt

### Arregle su código con `rustfix`.

La herramienta rustfix se incluye con las instalaciones de Rust y puede corregir automáticamente
algunas advertencias del compilador. Si ha escrito código en Rust, probablemente haya visto
advertencias del compilador. Por ejemplo, considere este código:

<span class="filename">nombre de fichero: src/main.rs</span>

```rust
fn do_something() {}

fn main() {
    for i in 0..100 {
        do_something();
    }
}
```

Aquí, estamos llamando a la función `do_something` 100 veces, pero nunca usamos la variable
variable `i` en el cuerpo del bucle `for`. Rust nos advierte de ello:

```console
$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: unused variable: `i`
 --> src/main.rs:4:9
  |
4 |     for i in 1..100 {
  |         ^ help: consider using `_i` instead
  |
  = note: #[warn(unused_variables)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
```

La advertencia sugiere que utilicemos `_i` como nombre: el guión bajo
indica que pretendemos que esta variable no se utilice. Podemos aplicar automáticamente
aplicar esta sugerencia con la herramienta `rustfix` ejecutando el comando command `cargo
fix`:

```console
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

Cuando miramos *src/main.rs* de nuevo, veremos que `cargo fix` ha cambiado el
code:

<span class="filename">nombre de fichero: src/main.rs</span>

```rust
fn do_something() {}

fn main() {
    for _i in 0..100 {
        do_something();
    }
}
```

La variable del bucle `for` se llama ahora `_i`, y ya no aparece la advertencia.

También puedes usar el comando `cargo fix` para hacer la transición de tu código entre
diferentes ediciones de Rust. Las ediciones están cubiertas en el Apéndice E.

### Más Lints con Clippy

La herramienta Clippy es una colección de lints para analizar tu código y así poder detectar
errores comunes y mejorar tu código Rust.

Para instalar Clippy, introduzca lo siguiente:

```console
$ rustup component add clippy
```

Para ejecutar las lints de Clippy en cualquier proyecto de Cargo, introduzca lo siguiente:

```console
$ cargo clippy
```

Por ejemplo, digamos que escribes un programa que utiliza una aproximación de una
constante matemática, como pi, como hace este programa:

<span class="filename">nombre de fichero: src/main.rs</span>

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("el área del círculo es {}", x * r * r);
}
```

La ejecución de `cargo clippy` en este proyecto da lugar a este error:

```text
error: approximate value of `f{32, 64}::consts::PI` found. Consider using it directly
 --> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = note: #[deny(clippy::approx_constant)] on by default
  = help: for further information visit https://rust-lang-nursery.github.io/rust-clippy/master/index.html#approx_constant
```

Este error le permite saber que Rust tiene esta constante definida con mayor precisión y
que tu programa sería más correcto si usaras la constante en su lugar. Usted
cambiaría su código para utilizar la constante `PI`. El siguiente código
no da lugar a ningún error o advertencia de Clippy:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("el área del círculo es {}", x * r * r);
}
```

Para más información sobre Clippy, consulte [la documentación][clippy].

[clippy]: https://github.com/rust-lang/rust-clippy

### Integración del IDE mediante el servidor de lenguaje Rust

Para ayudar a la integración del IDE, el proyecto Rust distribuye el *Rust Language
Server* (`rls`). Esta herramienta habla el [Language Server
Protocolo][lsp], 
para comunicarse entre sí. Diferentes clientes pueden utilizar el `rls`,
como por ejemplo [el plug-in de Rust para Visual Studio Code][vscode].

[lsp]: http://langserver.org/
[vscode]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust

Para instalar el `rls`, introduzca lo siguiente:

```console
$ rustup component add rls
```

A continuación, instale el soporte para el servidor de lenguaje en su IDE particular; obtendrá
habilidades como el autocompletado, el salto a la definición y los errores en línea.

Para más información sobre el `rls`, véase [su documentación][rls].

[rls]: https://github.com/rust-lang/rls
