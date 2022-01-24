## Apéndice E - Ediciones

En el Capítulo 1, viste que `cargo new` añade un poco de metadatos a tu archivo
*Cargo.toml* sobre una edición. Este apéndice habla de lo que eso significa.

El lenguaje y el compilador de Rust tienen un ciclo de lanzamiento de seis semanas, lo que significa que los usuarios reciben
un flujo constante de nuevas características. Otros lenguajes de programación lanzan cambios más grandes
cambios más grandes con menos frecuencia; Rust lanza actualizaciones más pequeñas con más frecuencia. Después de un tiempo, todos estos pequeños cambios se suman. Pero de una versión a otra, puede ser
puede ser difícil mirar atrás y decir: "¡Vaya, entre Rust 1.10 y Rust 1.31, Rust ha cambiado mucho!

Cada dos o tres años, el equipo de Rust produce una nueva *edición* de Rust. Cada
edición reúne las características que han aterrizado en un paquete claro con
documentación y herramientas totalmente actualizadas. Las nuevas ediciones se envían como parte del habitual
proceso de lanzamiento de seis semanas.

Las ediciones sirven a diferentes propósitos para diferentes personas:

* Para los usuarios activos de Rust, una nueva edición reúne los cambios incrementales en
  un paquete fácil de entender.
* Para los no usuarios, una nueva edición indica que han llegado algunos avances importantes
  que puede hacer que merezca la pena volver a mirar a Rust.
* Para quienes desarrollan Rust, una nueva edición proporciona un punto de encuentro para el
  proyecto en su conjunto.

En el momento de escribir este artículo, hay dos ediciones de Rust disponibles: Rust 2015 y
Rust 2018. Este libro está escrito utilizando los modismos de la edición Rust 2018.

La clave `edición` en *Cargo.toml* indica la edición que el compilador debe
utilizar para su código. Si la clave no existe, Rust utiliza `2015` como valor de edición
por razones de compatibilidad.

Cada proyecto puede optar por una edición distinta de la edición por defecto de 2015.
Las ediciones pueden contener cambios incompatibles, como la inclusión de una nueva palabra clave que
entra en conflicto con los identificadores en el código. Sin embargo, a menos que opte por esos
cambios, su código continuará compilando aunque actualice la versión del compilador de Rust
que utilices.

Todas las versiones del compilador de Rust soportan cualquier edición que exista antes de ese
compilador, y pueden enlazar crates de cualquier edición soportada
soportadas. Los cambios de edición sólo afectan a la forma en que el compilador analiza inicialmente el código.
código. Por lo tanto, si estás usando Rust 2015 y una de tus dependencias usa
Rust 2018, tu proyecto compilará y podrá utilizar esa dependencia. La situación
situación opuesta, en la que tu proyecto utiliza Rust 2018 y una dependencia utiliza
Rust 2015, también funciona.

Para ser claros: la mayoría de las funciones estarán disponibles en todas las ediciones. Los desarrolladores que utilicen
cualquier edición de Rust seguirán viendo mejoras a medida que se realicen nuevas versiones estables
se realicen. Sin embargo, en algunos casos, principalmente cuando se añaden nuevas palabras clave, algunas nuevas
nuevas características podrían estar disponibles sólo en ediciones posteriores. Tendrá que cambiar de
ediciones si quieres aprovechar estas características.

Para más detalles, la [*Edición
Guide*](https://doc.rust-lang.org/stable/edition-guide/) es un libro completo
sobre ediciones que enumera las diferencias entre ediciones y explica
cómo actualizar automáticamente tu código a una nueva edición mediante `cargo fix`.
