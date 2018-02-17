¿Qué es Propiedad?

La característica central de Rust es la *propiedad*. Aunque la característica es fácil de
explicar, tiene profundas implicaciones para el resto del lenguaje.

Todos los programas tienen que administrar la forma en que utilizan la memoria del ordenador mientras se está ejecutando.
Algunos lenguajes tienen una colección de basura que busca constantemente la memoria que ya no se usa
mientras se ejecuta el programa; en otros lenguajes, el programador debe explícitamente
asignar y liberar la memoria. Rust utiliza un tercer enfoque: la memoria se gestiona
a través de un sistema de propiedad con un conjunto de reglas que el compilador comprueba en
tiempo de compilación. No se producen costes de tiempo de ejecución para ninguna de las características de propiedad.

Debido a que la propiedad es un concepto nuevo para muchos programadores, toma algún tiempo
acostumbrarse a ella. La buena noticia es que cuanto más experimentado seas con Rust
y las reglas del sistema de propiedad, serás más capaz de desarrollar código naturalmente
seguro y eficiente. ¡Sigue con ello!

Cuando entiendas la propiedad, tendrás una base sólida para comprender
las características que hacen que Rust sea único. En este capítulo, aprenderás la propiedad
trabajando con algunos ejemplos que se centran en una estructura de datos muy común:
las cadenas.

<!-- PROD: START BOX -->

### La Pila y el Montón
>
>En muchos lenguajes de programación, no tenemos que pensar en la pila y
>el montón muy a menudo. Pero en un lenguaje de programación de sistemas como Rust, si un
>valor está en la pila o en el montón tiene más efecto sobre cómo se comporta el lenguaje
>y por qué tenemos que tomar ciertas decisiones. Vamos a describir partes de
>la propiedad en relación con la pila y el montón más adelante en este capítulo, así que aquí
>hay una breve explicación en preparación.
>
>Tanto la pila como el montón son partes de la memoria que están disponibles para que tu código
>las utilice en tiempo de ejecución, pero están estructuradas de diferentes maneras. La pila almacena
>los valores en el orden en que se obtienen y elimina los valores en el orden opuesto.
>Esto se conoce como *última entrada, primera salida*.  Piensa en un montón de platos: cuando
>añades más platos, los pones encima del montón, y cuando necesitas un
>plato, lo sacas de la parte superior. ¡Añadir o quitar platos del medio o
>del fondo no funcionaría tan bien! La adición de datos se llama *pushing en la pila*,
>y la eliminación de datos se llama *popping de la pila*.
>
>La pila es rápida debido a la forma en que accede a los datos: nunca tiene que
>buscar un lugar para poner nuevos datos o un lugar de donde obtener datos, porque ese
>lugar siempre es la cima. Otra propiedad que hace que la pila sea rápida es que todos los
>datos de la pila deben tener un tamaño conocido y fijo.
>
>Para los datos con un tamaño desconocido para nosotros al momento de compilar o un tamaño que podría cambiar,
>podemos almacenar los datos en el montón en su lugar. El montón es menos organizado: cuando ponemos
>datos sobre el montón, pedimos algo de espacio. El sistema operativo encuentra
>un punto vacío en algún lugar del montón que es lo suficientemente grande, lo marca como en
>uso, y nos devuelve un *puntero* que es la referencia a esa ubicación. Este proceso se llama
>*asignar en el montón*, y a veces abreviamos la frase simplemente como
>"asignar". Los valores de empuje sobre la pila no se consideran asignados.
>Debido a que el puntero es un tamaño conocido y fijo, podemos almacenar el puntero en la
>pila, pero cuando queremos los datos reales, tenemos que seguir el puntero.
>
>Piensa en sentarte en un restaurante. Cuando ingresas, indicas el número de
>personas en tu grupo y el personal encuentra una mesa vacía que se ajuste a todos y
>te lleve hasta allí. Si alguien en su grupo llega tarde, puede preguntar dónde te
>has sentado para encontrarte.
>
>Acceder a los datos en el montón es más lento que acceder a los datos de la pila porque
>tenemos que seguir un puntero para llegar allí. Los procesadores contemporáneos son más rápidos si
>saltan menos en la memoria. Siguiendo la analogía, considera un servidor en un
>restaurante tomando órdenes de muchas mesas. Es más eficiente obtener todas las
>órdenes en una mesa antes de pasar a la siguiente. Tomar un pedido de
>la mesa A, luego un pedido de la mesa B, luego uno de A otra vez, y luego uno de B
>otra vez sería un proceso mucho más lento. De la misma manera, un procesador puede hacer su
>trabajo mejor si trabaja con datos que están cerca de otros datos (como lo está en la
>pila) y no más lejos (como puede estar en el montón). La asignación de una gran cantidad de
>espacio en el montón también puede tomar tiempo.
>
>Cuando nuestro código llama a una función, los valores pasados a la función (incluyendo,
>potencialmente, punteros a los datos sobre el montón) y las variables locales de la función
>se empujan a la pila. Cuando la función termina, esos valores se
>eliminan de la pila.
>
>Mantener un seguimiento de qué partes del código están usando qué datos en el monton, minimizando
>la cantidad de datos duplicados en el monton, y limpiando los datos no utilizados en el
>monton para que no nos quedemos sin espacio son todos los problemas que la propiedad trata.
>Una vez que entiendas la propiedad, no tendrás que pensar en la pila y
>el montón muy a menudo, pero saber que la gestión de datos del monton es la razón por la que existe la propiedad
>puede ayudar a explicar por qué funciona de la forma en que funciona.
>
<!-- PROD: END BOX -->

### Reglas de Propiedad

Primero, echemos un vistazo a las reglas de propiedad. Ten en cuenta estas reglas mientras
trabajamos a través de los ejemplos que ilustran las reglas:

> 1. Cada valor en Rust tiene una variable que se llama *owner*.
> 2. Sólo puede haber un dueño a la vez.
> 3. Cuando el propietario salga del alcance, el valor será eliminado.

### Variable Scope

Hemos caminado a través de un ejemplo de un programa en Rust ya en el capítulo 2. Ahora
que ya hemos superado la sintaxis básica, no incluiremos todo el código `fn main () {`
en ejemplos, así que si estás siguiendo el ejemplo, tendrás que poner los siguientes
ejemplos dentro de una función `main` manualmente. Como resultado, nuestros ejemplos serán
un poco más concisos, permitiéndonos centrarnos en los detalles reales en lugar de los
códigos repetitivos.

Como primer ejemplo de propiedad, veremos el *scope* de algunas variables. Un
scope es el rango dentro de un programa para el cual una posición es válida. Digamos que
tenemos una variable que se parece a esto:

```rust
let s = "hello";
```

La variable `s` se refiere a una cadena literal, donde el valor de la cadena es
hardcodeada en el texto de nuestro programa. La variable es válida desde el momento
en que se declara hasta el final del actual *scope*. Listado 4-1 tiene
comentarios anotando donde la variable `s` es válida:

```rust
{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
}                      // this scope is now over, and s is no longer valid
```

<span class="caption">Listado 4-1: Una variable y el alcance en el que es
    válida</span>

En otras palabras, hay dos puntos importantes en el tiempo:

1. Cuando `s` viene *into scopee*, es válido.
1. Permanece así hasta que sale *fuera of scope*.

En este punto, la relación entre scopes y cuando las variables son válidas es
similar a la de otros lenguajes de programación. Ahora vamos a construir encima de este
entendimiento introduciendo el tipo de `String`.

### El Tipo De `String`

Para ilustrar las reglas de propiedad, necesitamos un tipo de datos que sea más complejo
que los que hemos tratado en el Capítulo 3. Todos los tipos de datos que hemos visto
anteriormente se almacenan en la pila y salen de la pila cuando termina su
alcance, pero queremos ver los datos que se almacenan en la pila y explorar cómo
Rust sabe cuándo limpiar esos datos.

Usaremos `String` como ejemplo aquí y nos concentraremos en las partes de `String`
que se relacionan con la propiedad. Estos aspectos también se aplican a otros tipos de datos complejos
proporcionados por la biblioteca estándar y que tu creas. Discutiremos `String` con
más profundidad en el Capítulo 8.

Ya hemos visto cadenas literales, donde el valor de la cadena está harcodeado en
nuestro programa. Las cadenas literales son convenientes, pero no siempre son adecuadas para
todas las situaciones en las que se quiera utilizar texto. Una razón es que son
inmutables. Otro es que no todos los valores de las cadenas pueden ser conocidos cuando escribimos
nuestro código: por ejemplo, ¿qué pasa si queremos tomar la entrada del usuario y guardarlo? Para
estas situaciones, Rust tiene un segundo tipo de cadena, `String`. Este tipo se
asigna en el monton y como tal es capaz de almacenar una cantidad de texto que es
desconocido para nosotros en el tiempo de compilación. Puedes crear una `String` desde una cadena literal
usando la función `from`, así:

```rust
let s = String::from("hello");
```

El doble dos puntos (`::`) es un operador que nos permite ponerle un espacio de nombres
a esta función particular `from` bajo el tipo `String` en lugar de usar algún tipo de
nombre como `string_from`. Discutiremos más esta sintaxis en la sección "Métodos
de Sintaxis" del Capítulo 5 y cuando hablemos de cómo se abre el espacio entre nombres con los módulos
en el Capítulo 7.

Este tipo de cadena *puede* ser mutada:

```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print `hello, world!`
```

Entonces, ¿cuál es la diferencia aquí? ¿Por qué `String` puede mutar pero los literales
no? La diferencia es cómo estos dos tipos tratan con la memoria.

### Memoria y Asignación

En el caso de una cadena literal, conocemos los contenidos en tiempo de compilación para que el
texto sea codificado directamente en el ejecutable final, haciendo que las cadenas literales
sean rápidas y eficientes. Pero estas propiedades sólo provienen de su inmutabilidad.
Desafortunadamente, no podemos poner una mancha de memoria en el binario para cada pieza de
texto cuyo tamaño es desconocido al momento de compilar y cuyo tamaño podría cambiar mientras
que se ejecuta el programa.

Con el tipo `String`, para soportar un fragmento de texto mutable y cultivable,
necesitamos asignar una cantidad de memoria en el monton, desconocida a la hora de compilar,
para retener el contenido. Esto significa:

1. La memoria debe solicitarse al sistema operativo en tiempo de ejecución.
2. Necesitamos una forma de devolver esta memoria al sistema operativo cuando
terminemos con nuestra `String`.

Esa primera parte la hacemos nosotros: cuando llamamos `String::from`, su implementación
solicita la memoria que necesita. Esto es bastante universal en lenguajes de
programación.

Sin embargo, la segunda parte es diferente. En los lenguajes con un *garbage collector
(GC)*, el GC mantiene un registro y limpia la memoria que ya no se usa,
y nosotros, como programador, no necesitamos pensar en ello. Sin un GC, es el
programador el responsable de identificar cuando la memoria ya no está siendo utilizada y
llamar al código para devolverla explícitamente, tal como lo hicimos para solicitarla. Hacer esto
correctamente ha sido históricamente un problema de programación difícil. Si lo olvidamos,
perderemos la memoria. Si lo hacemos demasiado pronto, tendremos una variable inválida. Si
lo hacemos dos veces, eso también es un error. Necesitamos emparejar exactamente una `allocate` con
exactamente un `free`.

Rust toma un camino diferente: la memoria se devuelve automáticamente una vez que
la variable que posee se sale del scope. Aquí está una versión de nuestro ejemplo
de scope de Listado 4-1 usando una `String` en lugar de una cadena literal:

```rust
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                                  // this scope is now over, and s is no
                                   // longer valid
```

Hay un punto natural en el que podemos devolver la memoria que nuestra `String` necesita
al sistema operativo: cuando la `s` se sale del scope. Cuando una variable se sale
del scope, Rust nos llama a una función especial. Esta función se llama `drop`,
y es donde el autor de `String` puede poner el código para devolver la memoria.
Rust llama `drop` automáticamente al cierre `}`.

> Nota: En C++, este patrón de distribución de recursos al final de la vida de un artículo
> a veces se llama *Resource Acquisition Is Initialization (RAII)*.
La función `drop` en Rust te resultará familiar si has utilizado patrones
RAII.

Este patrón tiene un profundo impacto en la forma en que se escribe el código de Rust. Puede parecer
sencillo ahora mismo, pero el comportamiento del código puede ser inesperado en situaciones más
complicadas cuando queremos que múltiples variables utilicen los datos
que hemos asignado en el montón. Exploremos algunas de esas situaciones.

#### Las Variables y los Datos Interactúan: Desplazar

Múltiples variables pueden interactuar con los mismos datos de diferentes maneras en Rust.
Veamos un ejemplo usando un número entero en Listado 4-2:

```rust
let x = 5;
let y = x;
```

<span class="caption">Listado 4-2: Asignación del valor entero de la variable `x`
a `y`.</span>

Probablemente podemos adivinar lo que esto está haciendo basándonos en nuestra experiencia con otros
lenguajes: "Vincular el valor `5` a `x`; luego hacer una copia del valor en `x` y
atarlo a `y`". Ahora tenemos dos variables, `x` y `y`, y ambas iguales `5`.
Esto es precisamente lo que está ocurriendo porque los números enteros son valores simples con un
tamaño fijo conocido, y estos dos valores `5` se empujan sobre la pila.

Ahora veamos la versión `String`:

```rust
let s1 = String::from("hello");
let s2 = s1;
```

Esto se parece mucho al código anterior, así que podríamos suponer que la forma
en que funciona sería la misma: es decir, la segunda línea haría una copia del
valor en `s1` y lo uniría a `s2`. Pero esto no es exactamente lo que pasa.

Para explicar esto más a fondo, veamos cómo es `String` bajo las
cubiertas en la Figura 4-1. Una `String` se compone de tres partes, mostradas a la
izquierda: un puntero a la memoria que contiene el contenido de la cadena, una longitud
y una capacidad. Este grupo de datos se almacena en la pila. A la derecha está la
memoria del montón que contiene el contenido.

<img alt="String in memory" src="img/trpl04-01.svg" class="center" style="width: 50%;" />

<span class="caption">Figura 4-1: Representación en memoria de una `String`
con el valor `"hello"`vinculado a `s1`.</span>

La longitud es cuánta memoria, en bytes, está utilizando el contenido
de la `String`. La capacidad es la cantidad total de memoria, en bytes, que la
`String` ha recibido del sistema operativo. La diferencia entre la longitud
y la capacidad es importante, pero no en este contexto, así que por ahora está bien ignorar
la capacidad.

Cuando asignamos `s1` a `s2`, se copian los datos de `String`, lo que significa que copiamos el
puntero, la longitud y la capacidad que hay en la pila. No copiamos los
datos del montón al que se refiere el puntero. En otras palabras, la representación de
datos en la memoria se parece a la Figura 4-2.

<img alt="s1 and s2 pointing to the same value" src="img/trpl04-02.svg" class="center" style="width: 50%;" />

<span class="caption">Figura 4-2: Representación en memoria de la variable `s2`
que tiene una copia del puntero, longitud y capacidad de `s1`.</span>

La representación *no* se parece a la Figura 4-3, que es cómo sería la memoria
si Rust copiara en su lugar también los datos del montón. Si Rust hizo esto, la
operación `s2 = s1` podría ser potencialmente muy costosa en términos de rendimiento
en tiempo de ejecución si los datos del montón fueran grandes.

<img alt="s1 and s2 to two places" src="img/trpl04-03.svg" class="center" style="width: 50%;" />

<span class="caption">Figura 4-3: Otra posibilidad de lo que `s2 = s1` podría
hacer si Rust también copiara los datos del montón.</span>

Anteriormente, dijimos que cuando una variable se sale del alcance, Rust llama automáticamente
a la función `drop` y limpia la memoria del montón para esa variable. Pero
la Figura 4-2 muestra ambos indicadores de datos apuntando a la misma ubicación. Esto es un
problema: cuando `s2` y `s1` se salen del alcance, ambos intentarán liberar la
misma memoria. Esto se conoce como un error *double free* y es uno de los errores de seguridad de memoria
que mencionamos anteriormente. Liberar la memoria dos veces puede conducir a la corrupción
de la memoria, lo que puede conducir potencialmente a vulnerabilidades de seguridad.

Para garantizar la seguridad de la memoria, hay un detalle más de lo que sucede en esta
situación en Rust. En vez de intentar copiar la memoria asignada, Rust
considera que `s1` ya no es válido y por lo tanto, Rust no necesita liberar
nada cuando `s1` se sale del alcance. Comprueba lo que sucede cuando intentas
usar `s1` después de crear `s2`, simplemente no funciona:

```rust,ignore
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

Recibirás un error como este porque Rust te impide usar la
referencia invalidada:

```text
error[E0382]: use of moved value: `s1`
 --> src/main.rs:5:28
  |
3 |     let s2 = s1;
  |         -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value used here after move
  |
  = note: move occurs because `s1` has type `std::string::String`, which does
  not implement the `Copy` trait
```

Si has escuchado los términos "copia superficial" y "copia profunda" mientras trabajas
con otros lenguajes, el concepto de copiar el puntero, la longitud y la capacidad
sin copiar los datos probablemente suena como una copia superficial. Pero debido a que Rust
también invalida la primera variable, en lugar de llamarla copia superficial,
se conoce como *move*. Aquí leíamos esto diciendo que `s1` era *movido*
en `s2`. Así que lo que realmente sucede se muestra en la Figura 4-4.

<img alt="s1 moved to s2" src="img/trpl04-04.svg" class="center" style="width: 50%;" />

<span class="caption">Figura 4-4: Representación en la memoria después de que se haya
invalidado `s1`</span>

Eso resuelve nuestro problema! Con sólo `s2` válido, cuando salga del alcance, solo
liberará la memoria, y ya está.

Además, hay una elección de diseño que implica esto: Rust nunca
creará automáticamente copias "profundas" de sus datos. Por lo tanto, se puede suponer
que cualquier copia *automática* es barata en términos de rendimiento en tiempo de ejecución.

### Variables de Vías e Interacción de Datos: Clonación

Si queremos copiar  *do* con profundidad de datos del monton del `String`, no sólo los
datos de pila, podemos usar un método común llamado `clone`. Discutiremos la sintaxis
del método en el Capítulo 5, pero como los métodos son una característica común en muchos
lenguajes de programación, probablemente los hayas visto antes.

He aquí un ejemplo del método `clone` en acción:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

Esto funciona perfectamente y es cómo se puede producir explícitamente el comportamiento que se muestra
en la Figura 4-3, donde los datos de montones *se copian*.

Cuando ves una llamada a `clone`, sabes que se está ejecutando algún código arbitrario
y ese código puede ser caro. Es un indicador visual de que algo
diferente está pasando.

#### Datos de Sólo-Pila: Copiar

Hay otra arruga de la que aún no hemos hablado. Este código utilizando números enteros,
parte de los cuales se mostraron anteriormente en el Listado 4-2, funciona y es válido:

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

Pero este código parece contradecir lo que acabamos de aprender: no tenemos una llamada a
`clone`, pero `x` sigue siendo válido y no fue movido a `y`.

La razón es que los tipos como enteros que tienen un tamaño conocido en el momento de compilar
se almacenan enteramente en la pila, por lo que las copias de los valores reales se pueden hacer
rápidamente. Eso significa que no hay ninguna razón por la que queramos evitar que `x` sea
válido después de crear la variable `y`. En otras palabras, no hay diferencia
entre la copia profunda y superficial aquí, por lo que llamar `clone` no haría nada diferente
de la copia superficial habitual y podemos dejarla fuera.

Rust tiene una anotación especial llamada el rasgo de "Copy" que podemos colocar en
tipos como números enteros que se almacenan en la pila (hablaremos más sobre rasgos
en el Capítulo 10). Si un tipo tiene el rasgo `Copy`, una variable antigua sigue
siendo utilizable después de la asignación. Rust no nos permite anotar un tipo con el rasgo `Copy`
si el tipo, o cualquiera de sus partes, ha implementado el rasgo `Drop`. Si
el tipo necesita algo especial para que suceda cuando el valor salga del alcance y
agreguemos la anotación `Copy` a ese tipo, tendremos un error de tiempo de compilación. Para
obtener más información sobre cómo agregar la anotación `Copy` a tu tipo, consulta el Apéndice C sobre
Rasgos derivables.

Entonces, ¿qué tipos son `Copy`? Puede comprobar la documentación del tipo dado para
estar seguro, pero como regla general, cualquier grupo de valores escalares simples puede ser
`Copy`, y nada que requiera asignación o sea alguna forma de recurso es
`Copy`. Éstos son algunos de los tipos que son `Copy`:

* Todos los tipos enteros, como `u32`.
* El tipo booleano,`bool`, con valores `true` y `false`.
* El tipo caracter, `char`.
* Todos los tipos de coma flotante, como `f64`.
* Tuples, pero sólo si contienen tipos que también son `Copy`. ` (i32, i32)` es
`Copy`, pero `(i32, String)` no lo es.

### Propiedad y Funciones

La semántica para pasar un valor a una función que es similar a la asignación de
un valor a una variable. Pasar una variable a una función se moverá o copiará, al igual
que la asignación. El Listado 4-3 tiene un ejemplo con algunas anotaciones que muestran donde
las variables entran y salen del alcance:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let s = String::from("hello");  // s entra a alcance.

    takes_ownership(s);             // el valor de s se mueve dentro de la función...
                                    // ... y por lo tanto ya no es válido aquí.

    let x = 5;                      // x entra a alcance.

    makes_copy(x);                  // x pasaría a la función,
                                    // pero i32 es Copy, así que está bien seguir
                                    // uso x después.

} // Aquí, X se sale del alcance, luego s. Pero como el valor de s fue movido, nada
  // especial pasa.

fn takes_ownership(some_string: String) { // some_string entra a alcance.
    println!("{}", some_string);
} // Aquí, some_string se sale del alcance y se llama `drop`. El respaldo
  // de la memoria se libera.

fn makes_copy(some_integer: i32) { // some_integer entra a alcance.
    println!("{}", some_integer);
} // Aquí, some_integer entero queda fuera del alcance. No pasa nada especial.
```

<span class="caption">Listado 4-3: Funciones con propiedad y alcance
anotados</span>

Si intentáramos usar `s` después de la llamada a `takes_ownership`, Rust arrojaría un
error de compilación de tiempo. Estas comprobaciones estáticas nos protegen de los errores. Intenta agregar
código a `main` que usa `s` y `x` para ver dónde puedes usarlas y dónde
las reglas de propiedad te impiden hacerlo.

### Valores de Retorno y Alcance

Los valores devueltos también pueden transferir la propiedad. Aquí hay un ejemplo con anotaciones similares
a las del Listado 4-3:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership mueve su retorno
                                        // valor en s1.

    let s2 = String::from("hello");     // s2 entra a alcance.

    let s3 = takes_and_gives_back(s2);  // s2 se traslada a
                                        // takes_and_gives_back, que también
                                        // mueve su valor de retorno a s3.
} // Aquí, s3 se sale del alcance y es eliminado. s2 se sale del ámbito de aplicación pero fue
  // movido, así que no pasa nada. s1 se sale del alcance y se abandona.

fn gives_ownership() -> String {             // gives_ownership moverá su
                                             // valor de retorno a la función
                                             // que lo llama.

    let some_string = String::from("hello"); // some_string entra en alcance.

    some_string                              // some_string es devuelto y
                                             // se traslada a la función de
                                             // llamado.
}

// takes_and_gives_back tomará una cadena y la devolverá
fn takes_and_gives_back(a_string: String) -> String { // a_string entra en
                                                      // alcance.

    a_string  // a_string se devuelve y pasa a la función de llamado.
}
```

La propiedad de una variable sigue el mismo patrón cada vez: asignar un
valor a otra variable la mueve. Cuando una variable que incluye datos en el
monton se sale del alcance, el valor será limpiado por `drop` a menos que los datos
hayan sido movidos para ser propiedad de otra variable.

Tomar posesión y luego devolver la propiedad con cada función es un poco
tedioso. ¿Qué pasa si queremos que una función utilice un valor pero no tome posesión?
Es bastante molesto que cualquier cosa que pasemos también tenga que ser devuelta si
queremos volver a usarla, además de cualquier dato que resulte del cuerpo de la
función que queramos devolver también.

Es posible devolver múltiples valores usando un tuple, así:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() devuelve la longitud de una cadena.

    (s, length)
}
```

Pero esto es demasiada ceremonia y mucho trabajo para un concepto que debería ser
común. Afortunadamente para nosotros, Rust tiene una característica para este concepto, y se llama
*referencias*.
