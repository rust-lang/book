## Tipos de datos

Cada valor en Rust es de un cierto *tipo*, que le dice a Rust qué tipo de datos 
se está especificando para que sepa cómo trabajar con esos datos. En esta sección,
veremos una serie de tipos que se construyen en el lenguaje. Dividimos los
tipos en dos subconjuntos: escalar y compuesto.

A lo largo de esta sección, ten en cuenta que Rust es un lenguaje *estáticamente mecanografiado*,
lo que significa que debes conocer los tipos de todas las variables en tiempo de
compilación. El compilador generalmente puede inferir qué tipo de información queremos usar en base al 
valor y cómo la usamos. En los casos en que muchos tipos son posibles, como cuando 
convertimos una `String` a un tipo numérico usando `parse` en el Capítulo 2, debemos añadir
una anotación de tipo, como esta:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

Si no añadimos el tipo de anotación aquí, Rust mostrará el siguiente
error, lo que significa que el compilador necesita más información de nosotros para saber 
qué tipo posible queremos usar:

```text
error[E0282]: unable to infer enough type information about `_`
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^ cannot infer type for `_`
  |
  = note: type annotations or generic parameter binding required
```

Verás diferentes tipos de anotaciones a medida que discutimos los distintos tipos de datos.

### Tipos Escalares

Un tipo *escalar* representa un solo valor. Rust tiene cuatro tipos escalares principales:
enteros, números de coma flotante, booleanos y caracteres. Probablemente los 
reconocerás de otros lenguajes de programación, pero vamos a saltar a cómo
funcionan en Rust.

#### tipo Entero

Un *integer* es un número sin un componente fraccionario. Usamos un tipo 
entero antes en este capítulo, el tipo `i32`. Esta tipo de declaración indica 
que el valor con el que está asociado debe ser un entero firmado (de ahí el `i`, 
a diferencia de una `u` para no firmado) que ocupa 32 bits de espacio. La Tabla 3-1 
muestra los tipos enteros incorporados en Rust. Cada variante de las columnas 
Firmada y No Firmada (por ejemplo, *i32*) puede utilizarse para declarar el tipo de 
un valor entero.

<span class="caption">Tabla 3-1: Tipos enteros en Rust</span>

| Longitud | Firmada | No firmada |
|----------|---------|------------|
| 8-bit    | i8      | u8         |
| 16-bit   | i16     | u16        |
| 32-bit   | i32     | u32        |
| 64-bit   | i64     | u64        |
| arch     | isize   | usize      |

Cada variante puede estar firmada o sin firmar y tiene un tamaño explícito. 
Firmada y no firmada se refiere a si es posible que el número sea 
negativo o positivo; en otras palabras, si el número necesita tener un signo
con él (firmada) o si sólo será positivo y por lo tanto puede
ser representado sin un signo (no firmada). Es como escribir números en papel: cuando
el signo importa, un número se muestra con un signo más o un signo menos; sin embargo,
cuando es seguro suponer que el número es positivo, se muestra sin signo.
Los números firmados se almacenan usando la representación del complemento de dos (si no estás 
seguro de lo que es esto, puedes buscarlo en línea; una explicación está fuera
del alcance de este libro).

Cada variante firmada puede almacenar números de - (2<sup>n - 1</sup>) a 2<sup>n - 
1</sup> - 1 inclusive, donde `n` es el número de bits que usa la variante. Por lo tanto,
un `i8` puede almacenar números de - (2<sup>7</sup>) a 2<sup>7</sup> - 1, lo que equivale 
a -128 a 127. Las variantes no firmadas pueden almacenar números de 0 a 2<sup>n</sup> - 1, 
por lo que un `u8` puede almacenar números de 0 a 2<sup>8</sup> - 1, lo que equivale a 0 a 255.
 
 Además, los tipos `isize` y `usize` dependen del tipo de equipo en el que estes
 ejecutando el programa: 64 bits si está en una arquitectura de 64 bits y 32 bits
 si está en una arquitectura de 32 bits.
 
Puede escribir enteros literales en cualquiera de las formas mostradas en la Tabla 3-2. Nota
que todos los números literales excepto el byte literal permiten un tipo de sufijo, tal como
`57u8`, y `_` como un separador visual, así como `1_000`.

<span class="caption">Tabla 3-2: Literales enteros en Rust</span>

| Números Literales  | Ejemplo       |
|--------------------|---------------|
| Decimal            | `98_222`      |
| Hex                | `0xff`        |
| Octal              | `0o77`        |
| Binario            | `0b1111_0000` |
| Byte (solo `u8` )  | `b'A'`        |

Entonces, ¿cómo sabes qué tipo de entero usar? Si no estás seguro, los valores
predeterminados de Rust son generalmente buenas opciones, y los tipos enteros por defecto son `i32`: 
generalmente es el más rápido, incluso en sistemas de 64 bits. La situación principal en la que
se utiliza `isize` o `usize` es cuando se indiza algún tipo de colección.

#### Tipos de Punto Flotante

Rust también tiene dos tipos primitivos para *floating-point numbers*, que son 
números con decimales. Los tipos de punto flotante de Rust son `f32` y `f64`, 
que tienen 32 bits y 64 bits de tamaño, respectivamente. El tipo por defecto es `f64`
porque es más o menos la misma velocidad que `f32` pero es capaz de mayor precisión.
Es posible usar un tipo `f64` en sistemas de 32 bits, pero será más lento 
que usar un tipo `f32` en esos sistemas. La mayor parte del tiempo, el potencial de 
negociación bajar el rendimiento para una mayor precisión es una elección inicial razonable, y tu 
debes poner a prueba tu código si sospechas que el tamaño del punto flotante es un problema en
tu situación.

Aquí hay un ejemplo que muestra números de punto flotante en acción:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

Los números de punto flotante se representan según la norma IEEE-754. El 
tipo `f32` es un flotador de precisión simple, y el `f64` tiene doble precisión.

#### Operaciones Numéricas

Rust soporta las operaciones matemáticas básicas usuales que tu esperarías para todos los
tipos de números: suma, resta, multiplicación, división y porcentaje. 
El siguiente código muestra cómo se usa cada uno de ellos en una declaración `let`:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    // Suma
    let sum = 5 + 10;

    // Resta
    let difference = 95.5 - 4.3;

    // Multiplicación
    let product = 4 * 30;

    // División
    let quotient = 56.7 / 32.2;

    // Porcentaje
    let remainder = 43 % 5;
}
```

Cada expresión de estas declaraciones utiliza un operador matemático y evalúa 
a un único valor, que luego se vincula a una variable. El Apéndice B contiene una
lista de todos los operadores que Rust proporciona.

#### El tipo Booleano

Como en la mayoría de los otros lenguajes de programación, un tipo booleano en Rust tiene dos valores
posibles: `true` y `false`. El tipo booleano en Rust se especifica con `bool`. 
Por ejemplo:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let t = true;

    let f: bool = false; // con anotación de tipo explícita
}
```

La forma principal de consumir valores booleanos es a través de condicionales, como una expresión
`if`. Cubriremos cómo funcionan las expresiones `if` en Rust en la sección 
"Flujo de Control".

#### El Tipo de Carácter

Hasta ahora sólo hemos trabajado con números, pero Rust también soporta letras. El tipo de 
`char` de Rust es el tipo alfabético más primitivo del lenguaje, y el siguiente
código muestra una forma de usarlo:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
   let c = 'z';
   let z = 'ℤ';
   let heart_eyed_cat = '😻';
}
```

El tipo `char` de Rust representa un valor escalar Unicode, lo que significa que puede
representar mucho más que ASCII. Letras acentuadas, ideogramas 
chinos/japoneses/coreanos, emojis y espacios de anchura cero son todos tipos válidos de `char` en Rust.
Los valores escalares Unicode van desde `U+000000` hasta `U+D7FF` e incluso de `U+E000` hasta
`U+10FFFF`. Sin embargo, un "caracter" no es realmente un concepto en Unicode, 
así que tu intuición humana para lo que es un "caracter" puede que no coincida con lo que 
es un `char` en Rust. Discutiremos este tema en detalle en la sección "Cadenas" 
en el Capítulo 8.

### Tipos de Compuestos

Los *tipos de compuestos* pueden agrupar múltiples valores de otros tipos a un tipo. Rust
tiene dos tipos de compuestos primitivos: tuplas y matrices.

#### Clasificación de Valores en Tuplas

Una tupla es una forma general de agrupar un cierto número de otros valores con 
una variedad de tipos en un tipo compuesto.

Creamos una tupla escribiendo una lista de valores separados por comas dentro
de paréntesis. Cada posición en la tupla tiene un tipo, y los tipos de los
diferentes valores en la tupla no tienen que ser los mismos. Hemos añadido anotaciones
de tipo opcionales en este ejemplo:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

La variable `tup` se une a la tupla entera, ya que una tupla se considera un
solo elemento compuesto. Para obtener los valores individuales de una tupla, podemos
usar la coincidencia de patrones para desestructurar el valor de una tupla, como este:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

Este programa crea primero una tupla y lo une a la variable `tup`. Luego
usa un patrón con `lets` para tomar `tup` y convertirlo en tres variables
separadas, `x`, `y` y `z`. Esto se llama *desestructuración*, porque rompe 
la tupla simple en tres partes. Finalmente, el programa imprime el valor de
`y`, que es `6.4`.

Además de la desestructuración por coincidencia de patrones, también podemos acceder 
directamente a un elemento de tupla utilizando un periodo (`.`) seguido del índice del
valor al que queremos acceder. Por ejemplo:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

Este programa crea una tupla, `x`, y luego crea nuevas variables para cada
elemento usando su índice. Como en la mayoría de los lenguajes de programación, el primer 
índice en una tupla es 0.

#### Arrays (Matrices)

Otra forma de tener una colección de múltiples valores es con una *array*. A diferencia
de una tupla, cada elemento de un array debe tener el mismo tipo. Las arrays en Rust son 
diferentes a las arrays en otros lenguajes porque las arrays en Rust tienen una 
longitud fija: una vez declaradas, no pueden crecer o reducirse de tamaño.

En Rust, los valores que entran en una array se escriben como una lista separada por comas
dentro de corchetes:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

Las matrices son útiles cuando deseas que tus datos se asignen a la pila en lugar de
al montón (hablaremos más sobre la pila y el montón en el Capítulo 4), o cuando 
quieres asegurarte de que siempre tienes un número fijo de elementos. No son tan
flexibles como el tipo vector. El tipo vector es un tipo de colección similar
proporcionado por la biblioteca estándar que *se* permite que crezca o se reduzca 
de tamaño. Si no estás seguro de si debes usar una array o un vector, debes probablemente
usar un vector: el Capítulo 8 discute los vectores con más detalle.

Un ejemplo de cuándo es posible que desees utilizar una matriz en lugar de un vector es en
un programa que necesita saber los nombres de los meses del año. Es muy poco
probable que un programa de este tipo necesite añadir o eliminar meses, por lo que puedes utilizar
una array porque sabes que siempre contendrá 12 elementos:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

##### Acceso a Los Elementos de la Array

Una array es un único trozo de memoria asignado en la pila. Podemos acceder 
a elementos de una array usando indexación, así:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

En este ejemplo, la variable llamada `first` obtendrá el valor `1`, porque
ese es el valor en el índice `[0]` de la array. La variable llamada `second` obtendrá 
el valor `2` del índice `[1]` en la array.

##### Acceso a Elementos de Array no Válidos

¿Qué sucede si intentamos acceder a un elemento de una array que está más allá del final
de la array? Digamos que cambiamos el ejemplo a lo siguiente:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}
```

Ejecutar este código usando `Cargo Run` produce el siguiente resultado:

```text
$ cargo run
   Compiling arrays v0.1.0 (file:///projects/arrays)
     Running `target/debug/arrays`
thread '<main>' panicked at 'index out of bounds: the len is 5 but the index is
 10', src/main.rs:6
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

La compilación no produjo ningún error, pero el programa da como resultado un 
error de *runtime* y no salió con éxito. Cuando intentes acceder a un 
elemento utilizando la indexación, Rust comprobará que el índice especificado es inferior
a la longitud de la array. Si el índice es mayor que la longitud, Rust entrará en 
*panic*, que es el término que usa Rust cuando un programa sale con un error.

Este es el primer ejemplo de los principios de seguridad de Rust en acción. En muchos
lenguajes de bajo nivel, este tipo de comprobación no se realiza y cuando proporcionas un 
índice incorrecto, se puede acceder a la memoria no válida. Rust te protege contra este 
tipo de error al salir inmediatamente en lugar de permitir el acceso a la memoria y
continuar. El Capítulo 9 trata más sobre el manejo de los errores de Rust.
