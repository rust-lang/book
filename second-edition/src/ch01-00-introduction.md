# Introducción

Bienvenido a "El Lenguaje de Programación Rust", un libro introductorio sobre Rust

Rust es un lenguaje de programación que le ayuda a escribir mejor software, de
forma más rápida. La ergonomía de alto-nivel y el control de bajo-nivel suelen
estar en conflicto entre ellos en el diseño de lenguajes de programación; Rust
se opone a eso, Rust le ofrece la opción de controlar detalles de bajo-nivel
(como el uso de memoria) sin la molestia que está tradicionalmente asociada
con ese control.

## Para Quién es Rust

Rust es perfecto para muchas personas por una variedad de razones. Discutamos
algunos de los grupos más importantes.

### Equipos de Desarrolladores

Rust está probando ser una herramienta de producción por colaborar con grandes
grupos de desarrolladores con variados niveles de conocimiento en programación
de sistemas. Los códigos de bajo-nivel son propensos a sufrir una variedad de
errores menores, los cuales en otros lenguajes de programación sólo pueden ser
descubiertos tras revisiones cuidadosas de códigos y exámenes extensos por
desarrolladores experimentados. En Rust, el compilador cumple un rol de portero
al evitar compilar códigos con esta clase de errores--incluyendo errores de
concurrencia. Trabajando junto al compilador, el equipo puede usar más tiempo
en la lógica del programa en vez de persiguiendo errores.

Rust también brinda herramientas de desarrollo contemporáneas al mundo de
programación de sistemas:

* Cargo, el administrador de dependencias y herramienta de contrucción incluída,
  hace añadir, compilar y manejar dependencias fácil y consistente dentro del
  ecosistema de Rust.
* Rustfmt asegura un estilo de código consistente entre desarrolladores.
* El Servidor de Lenguaje de Rust soporta la integración de IDE para completación
  de código y mensajes de error entre lineas.

Usando estas y otras herramientas en el ecosistema Rust, los desarrolladores
pueden ser productivos mientras escriben códigos al nivel de sistemas.

### Estudiantes

Rust es para estudiantes y personas que estén interesadas en aprender conceptos
sobre sistemas. Muchas personas han aprendido sobre temas como el desarrollo
de sistemas operativos a través de Rust. La comunidad está feliz de responder
preguntas de estudiantes. A través de esfuerzos como este libro, los equipos de
Rust quieren hacer a los conceptos de sistemas más accesibles para más personas,
especialmente para aquellos comenzando con programación.

### Compañías

Rust es usado en producción por cientas de compañías, grandes y pequeñas, para
una variedad de tareas, como herramientas de línea de comandos, servicios web,
herramientas DevOps, dispositivos embebidos, análisis de audio y video, y
transcodificación, criptomonedas, bioinformática, motores de búsqueda, aplicaciones
de cosas en internet, aprendizaje de máquina, e incluso partes mayores del buscador
web Firefox.

### Desarrolladores Open Source

Rust es para las personas que quieren construir el lenguaje de programación Rust,
la comunidad, las herramientas de desarrollo, y las librerias. Nos encantaría
que nos ayudaras con el lenguaje Rust.

### Personas Que Valoran La Velocidad y La Estabilidad

Por velocidad, nos referimos a la velocidad de los programas que Rust le permite
crear y la velocidad con la cual Rust le permite escribirlos. Las comprobaciones
del compilador de Rust aseguran estabilidad a través de la añadición de
características y la refactorización, a diferencia del legado de códigos
quebrantables en lenguajes sin estas comprobaciones que los desarrolladores
temen modificar. Al esforzarse por abstracciones de costo-cero, características
de niveles más altos que compilan códigos de niveles más bajos de códigos tan
rápidos como códigos escritos manualmente, Rust intenta hacer que hacer códigos
de forma segura también sea hacerlos de forma rápida.


## Para Quién Es Este Libro

Este libro asume que uste ha leído códigos en algún otro lenguaje de programación,
pero no asume cuál. Hemos intentado hacer el material extensamente accesible
para aquellos que vengan de diferentes pasados en programación. No usaremos mucho
tiempo explicando qué *es* la programación o cómo pensar al respecto; alguien
completamente nuevo a la programación estaría mejor leyendo un libro que ofrezca
específicamente una introducción a la programación.

## Cómo Usar Este Libro

Este libro generalmente asume que usted lo está leyendo de adelante-hacia-atrás,
eso significa, que los siguientes capítulos estarán construídos a partir de conceptos
de capítulos anteriores, y los capitulos anteriores no entrarán en mucho detalle en
un tema, revisando el tema luego en algún otro capítulo.

Hay dos tipos de capítulos en este libro: capítulos de concepto, y capítulos de proyectos.
En los capítulos de concepto, usted aprenderá sobre aspectos de Rust. En capítulos de
proyectos, construiremos programas pequeños juntos, aplicando lo que hemos aprendido
hasta entonces. Los apítulos 2, 12, y 20 son capítulos de proyectos; los demás son
capítulos de conceptos.

Adicionalmente, El Capítulo 2 es una introducción al Rust como lenguaje. Cubriremos
conceptos de alto nivel, y los capítulos que le siguen hablarán de estos conceptos
con más detalle. Si usted es la clase de persona que quiere ensuciarse las manos
de una vez, el Capítulo 2 es perfecto para eso. Si usted es *realmente* esa clase
de persona, podría preferir saltar al Capítulo 3, el cual cubre características
que son muy similares a otros lenguajes de programación, y puede continuar al
Capítulo 4 para aprender sobre el sistema de propiedad de Rust. En contraste,
si usted es un aprendiz particularmente meticuloso que prefiere aprender cada
detalle antes de continuar al siguiente, preferirá saltarse el Capítulo 2 e
irse directo al Capítulo 3.

Al final, no hay una forma errónea de leer un libro: si quiere saltarse partes,
¡Adelante! Tendrá que devolverse si encuentra cosas de forma confusa. Haga lo que
mejor le parezca.

Una parte importante en el proceso de aprendizaje de Rust es aprender cómo leer
los mensajes de error que el compilador le muestre. Como tal, mostraremos muchos
códigos que no compilará, y el mensaje de error que el compilador le mostrará
en esa situación. Como tal, si toma un ejemplo cualquiera, ¡Podría no compilar!
Por favor lea el texto alrededor para asegurarse de que no tomó uno de los ejemplos
en proceso.

Finalmente, hay algunos apéndices. Estos contienen información útil sobre los
lenguajes en un formato más referenciado.

## Contribuir a este libro

Este es un libro de código abierto. Si encuentras un error por favor no dudes en
crear un _issue_ o enviar una _pull request_ [a Github]. Mira el [CONTRIBUTING.md]
para mas detalles.

[En GitHub]: https://github.com/rust-lang/book
[CONTRIBUTING.md]: https://github.com/rust-lang/book/blob/master/CONTRIBUTING.md
