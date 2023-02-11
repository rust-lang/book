## Apéndice C: Rasgos Derivables

Apéndice C: Rasgos Derivables

En varios lugares del libro, hemos hablado del atributo derive, que
puede aplicarse a una definición de estructura o enumeración. El atributo derive genera
código que implementará un rasgo con su propia implementación por defecto en el
tipo que se ha anotado con la sintaxis `derive`.

En este apéndice, proporcionamos una referencia de todos los rasgos en la biblioteca estándar
que puede usar con `derive`. Cada sección cubre:

* Qué operadores y métodos derivan este rasgo permitirán
* Qué hace la implementación del rasgo proporcionada por `derive`
* Qué significa implementar el rasgo sobre el tipo
* Las condiciones en las que estás permitido o no permitido implementar el rasgo
* Ejemplos de operaciones que requieren el rasgo

Si deseas un comportamiento diferente al proporcionado por el atributo `derive`,
consulta la documentación de la [biblioteca estándar](../std/index.html)<!-- ignore -->
para cada rasgo para obtener detalles de cómo implementarlos manualmente.

El resto de los rasgos definidos en la biblioteca estándar no se pueden implementar en
tus tipos usando `derive`. Estos rasgos no tienen un comportamiento por defecto sensato,
así que depende de ti implementarlos de la manera que sea adecuada para lo que estás
tratando de lograr.

Un ejemplo de un rasgo que no se puede derivar es `Display`, que maneja la
formateación para los usuarios finales. Siempre debes considerar la forma adecuada de
mostrar un tipo a un usuario final. ¿Qué partes del tipo debería un usuario final ver?
¿Qué partes les resultarían relevantes? ¿Qué formato de los datos sería más relevante
para ellos? El compilador de Rust no tiene esta visión, por lo que
no puede proporcionar un comportamiento por defecto adecuado para ti.

La lista de rasgos derivables proporcionados en este apéndice no es exhaustiva:
las bibliotecas pueden implementar `derive` para sus propios rasgos, haciendo que la lista de
rasgos que puedes usar con `derive` sea verdaderamente abierta. Implementar `derive`
implica usar una macro procedural, que se cubre en la
[sección "Macros"][macros]<!-- ignore -->  del Capítulo 19.

### `Debug` para salida de programador

El parámetro `Debug` permite el formato de depuración en cadenas de formato, que indica añadiendo `:?` dentro de los marcadores `{}`.

El parámetro `Debug` le permite imprimir instancias de un tipo para propósitos de depuración, de modo que usted y otros programadores que utilicen su tipo puedan inspeccionar una instancia en un punto específico de la ejecución de un programa.

El parámetro `Debug` es necesario, por ejemplo, en el uso de la macro `assert_eq!`. Esta macro imprime los valores de las instancias dadas como argumentos si la aserción de igualdad falla, para que los programadores puedan ver por qué las dos instancias no eran iguales.


### `PartialEq` y `Eq` para comparaciones de igualdad

El parámetro `PartialEq` le permite comparar instancias de un tipo para comprobar la igualdad y permite el uso de los operadores `==` y `!=`.

Derivando `PartialEq` implementa el método `eq`. Cuando `PartialEq` es derivado en estructuras, dos instancias son iguales solo si todos los campos son iguales y las instancias no son iguales si algún campo no es igual. Cuando se deriva en enums, cada variante es igual a sí misma y no es igual a las otras variantes.

El parámetro `PartialEq` es requerido, por ejemplo, con el uso de la macro `assert_eq!`, que necesita ser capaz de comparar dos instancias de un tipo para la igualdad.

El parámetro `Eq` no tiene métodos. Su propósito es señalar que para cada valor del tipo anotado, el valor es igual a sí mismo. El trait `Eq` solo se puede aplicar a tipos que también implementan `PartialEq`, aunque no todos los tipos que implementan `PartialEq` pueden implementar `Eq`. Un ejemplo de esto son los tipos de números con punto flotante: la implementación de los números con punto flotante establece que dos instancias del valor no es un número (`NaN`) no son iguales entre sí.

Un ejemplo de cuando se requiere `Eq` es para las claves en un `HashMap<K, V>` para que el `HashMap<K, V>` pueda decir si dos claves son iguales.

### `PartialOrd` y `Ord` para comparaciones de ordenación

El parámetro `PartialOrd` le permite comparar instancias de un tipo con propósitos de ordenación.
fines de ordenación. Un tipo que implemente `PartialOrd` puede usarse con los operadores `<`,`>`.
operadores `<`,`>`, `<=` y `>=`. Sólo puedes aplicar el parámetro `PartialOrd` a tipos que también implementen `PartialOrd`.
a tipos que también implementen `PartialEq`.

La función `PartialOrd` implementa el método `partial_cmp`, que devuelve una `Option<Ordering`.
que será `None` cuando los valores no produzcan un `Option<Ordering>`.
ordenación. Un ejemplo de un valor que no produce un ordenamiento, aunque la mayoría de los valores de ese tipo puedan
la mayoría de los valores de ese tipo pueden compararse, es el valor de coma flotante
no-un-número (`NaN`). Llame a `partial_cmp` con cualquier número flotante y
el valor flotante `NaN` devolverá `None`.

Cuando se han utilizado structs, `PartialOrd` compara dos instancias comparando
el valor en cada campo en el orden en que los campos aparecen en la definición del struct.
de la estructura. Cuando se derivan en enums, las variantes del enum declaradas anteriormente en la definición del enum son
anteriores en la definición del enum se consideran menores que las variantes declaradas posteriormente.
variantes enumeradas más tarde.

El parámetro `PartialOrd` es necesario, por ejemplo, para el método `gen_range` de la crate `rand` que es necesario para el método `gen_range` de la crate `rand` de la caja `rand`,  que genera un valor aleatorio en el rango especificado por una expresión `range`.

El parámetro `Order` permite saber que para dos valores cualesquiera de tipo
anotados, habrá un orden válido. El parámetro `Order` implementa el método `cmp`, que devuelve el método `cmp`.
método `cmp`, que devuelve un `Ordering` en lugar de una `Option<Ordering>` porque
siempre habrá una ordenación válida. Sólo se puede aplicar el parámetro `Ordering` a los tipos
que también implementen `PartialOrd` y `Eq` (y `Eq` requiere `PartialEq`).
Cuando se deriva en structs y enums, `cmp` se comporta de la misma manera que la implementación `partialOrder` de `partialEq`.
la implementación derivada de `partial_cmp` con `PartialOrd`.

Un ejemplo de cuando se requiere un `Ord` es cuando se almacenan valores en un
`BTreeSet<T>`, una estructura de datos que almacena datos basados en el orden de los valores.
orden de clasificación de los valores.

### `Clone` y `Copy` para Duplicar Valores.

El parámetro Clone te permite crear explícitamente una copia profunda de un valor, y
el proceso de duplicación puede involucrar ejecutar código arbitrario y copiar datos en la heap. Consulta la sección  [“Formas en que las variables y los datos interactúan:
Clonar”][ways-variables-and-data-interact-clone]<!-- ignore --> section in
en el Capítulo 4 para obtener más información sobre `Clone`.

Derivar `Clone` implementa el método `clone`, que al implementarse para el
tipo completo, llama a `clone` en cada una de las partes del tipo. Esto significa que todos los
campos o valores en el tipo también deben implementar `Clone` para derivar `Clone`.

Un ejemplo de cuándo se requiere `Clone` es al llamar al método `to_vec` en una
porción. La porción no posee las instancias de tipo que contiene, pero el vector
devuelto desde `to_vec` necesitará poseer sus instancias, por lo que `to_vec` llama
`clone` en cada elemento. Por lo tanto, el tipo almacenado en la porción debe implementar `Clone`.

El parámetro `Copy` te permite duplicar un valor copiando únicamente bits almacenados en
la pila; no se requiere código arbitrario. Consulta la sección [“Datos solo en la pila:
Copiar”][stack-only-data-copy]<!-- ignore --> en el Capítulo 4 para obtener más
información sobre `Copy`.

Puedes usar el parámetro `Copy` en cualquier tipo cuyas partes implementen todas `Copy`. Un tipo que
implementa `Copy` también debe implementar Clone, porque un tipo que implementa
`Copy` tiene una implementación trivial de `Clone` que realiza la misma tarea que
`Copy`.

El parámetro `Copy` se requiere raramente; los tipos que implementan `Copy` tienen
optimizaciones disponibles, lo que significa que no tienes que llamar a `clone`, lo que hace
que el código sea más conciso.

Todo lo posible con `Copy` también se puede lograr con `Clone`, pero el código puede ser más lento o tener que usar `clone` en algunos lugares.

### `Hash` para asignar un valor a un valor de tamaño fijo

El parámetro `Hash` permite tomar una instancia de un tipo de tamaño arbitrario y mapear esa instancia a un valor de tamaño fijo usando una función hash. La derivación de `Hash` implementa el método `hash`. La implementación derivada del método `hash` combina el resultado de llamar a hash en cada una de las partes del tipo, lo que significa que todos los campos o valores también deben implementar `Hash` para usar `Hash`.

Un ejemplo de cuándo se requiere `Hash` es al almacenar claves en un `HashMap<K, V>` para almacenar datos de manera eficiente.

### `Default` para valores por defecto

El parametro `Default` permite crear un valor por defecto para un tipo. Derivando
`Default` implementa la función `default`. La implementación derivada de la función
llama a la función `default` en cada parte del tipo,
es decir, todos los campos o valores del tipo deben implementar también la función `Default` para poder
derivar `Default`.

La función `Default::default` se utiliza habitualmente en combinación con la sintaxis struct
de la que se habla en la sección ["Creación de instancias a partir de otras instancias con
actualización de estructuras
Syntax"][creating-instances-from-other-instances-with-struct-update-syntax]<!-- ignore -->
en el Capítulo 5. Puede personalizar algunos campos de una estructura y luego
establecer y utilizar un valor por defecto para el resto de los campos utilizando
Default::default()`.

El parametro `Default` es necesario cuando utilizas el método `unwrap_or_default` en instancias de `Option<T>`.
en instancias de `Option<T>`, por ejemplo. Si la `Option<T>` es `None`, el método
`unwrap_or_default` devolverá el resultado de `Default::default` para el tipo
`T` almacenado en la `Opción<T>`.

[creating-instances-from-other-instances-with-struct-update-syntax]:
ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax
[stack-only-data-copy]:
ch04-01-what-is-ownership.html#stack-only-data-copy
[ways-variables-and-data-interact-clone]:
ch04-01-what-is-ownership.html#ways-variables-and-data-interact-clone
[macros]: ch19-06-macros.html#macros
