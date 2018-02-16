# Apéndice F - Características más Recientes

Este apéndice documenta características que han sido añadidas a Rust estable desde que
se completó la parte principal del libro.


## Campos en Abreviatura

Podemos inicializar una estructura de datos (estruct, enum, unión) con campos
nombrados, escribiendo `fieldname` como abreviatura de `fieldname: fieldname`.
Esto permite una sintaxis compacta para la inicialización, con menos duplicación:

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let name = String::from("Peter");
    let age = 27;

    // Usando la sintaxis completa:
    let peter = Person { name: name, age: age };

    let name = String::from("Portia");
    let age = 27;

    // Utilizando el campo abreviado:
    let portia = Person { name, age };

    println!("{:?}", portia);
}
```


## Retorno desde Loops

Uno de los usos de un `loop` es reintentar una operación que sabe que puede fallar, como
comprobar si un hilo ha completado su trabajo. Sin embargo, es posible que necesites pasar el
resultado de esa operación al resto de tu código. Si lo agregas a la expresión `break`
que usas para detener el loop, este será devuelto por el bucle roto:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
```
