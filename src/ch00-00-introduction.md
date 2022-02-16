# Introducción

>Nota: Esta edición del libro es la misma que [The Rust Programming
> Languaje][nsprust] disponible en formato impreso y ebook de [No Starch
> Press][nsp].

[nsprust]: https://nostarch.com/rust
[nsp]: https://nostarch.com/

Bienvenido a *El lenguaje de programación Rust*, un libro de introducción a Rust.
El lenguaje de programación Rust le ayuda a escribir un software más rápido y fiable.
La flexibilidad de alto nivel y el control de bajo nivel están a menudo en conflicto en el
diseño de lenguajes de programación; Rust desafía ese conflicto. Mediante el equilibrio entre una potente
capacidad técnica y una gran experiencia para el desarrollador, Rust te da la opción
de controlar los detalles de bajo nivel (como el uso de la memoria) sin todas las molestias
tradicionalmente asociados a dicho control.

## Para quién es Rust

Rust es ideal para muchas personas por diversas razones. Veamos algunos de
los grupos más importantes.

### Equipos de desarrollo

Rust está demostrando ser una herramienta productiva para la colaboración entre grandes equipos de
desarrolladores con distintos niveles de conocimiento de programación de sistemas. El código de bajo nivel
es propenso a una variedad de errores sutiles, que en la mayoría de los otros lenguajes pueden ser
en la mayoría de los otros lenguajes sólo se pueden detectar mediante pruebas exhaustivas y una cuidadosa revisión del código por parte de desarrolladores experimentados.
experimentados. En Rust, el compilador desempeña un papel de guardián al negarse a
compilar código con estos errores escurridizos, incluidos los de concurrencia. Al trabajar
trabajando junto al compilador, el equipo puede dedicar su tiempo a centrarse en la lógica del programa
lógica del programa en lugar de perseguir errores.

Rust también aporta herramientas de desarrollo contemporáneas al mundo de la programación de sistemas:

* Cargo, el gestor de dependencias y la herramienta de compilación incluidos, hace que añadir,
  compilar y gestionar las dependencias de forma sencilla y coherente en todo el ecosistema de Rust.
* Rustfmt garantiza un estilo de codificación coherente entre los desarrolladores.
* El Servidor de Lenguaje Rust permite la integración del Entorno de Desarrollo Integrado (IDE)
  para la finalización del código y los mensajes de error en línea.

Utilizando estas y otras herramientas del ecosistema Rust, los desarrolladores pueden ser
productivos mientras escriben código a nivel de sistemas.

### Estudiantes

Rust se dirige a los estudiantes y a los interesados en aprender los conceptos de los sistemas
conceptos. Usando Rust, mucha gente ha aprendido sobre temas como el desarrollo de
desarrollo de sistemas operativos. La comunidad es muy acogedora y está dispuesta a responder
preguntas de los estudiantes. A través de esfuerzos como este libro, los equipos de Rust quieren
hacer que los conceptos de sistemas sean más accesibles para más gente, especialmente para los nuevos en
programación.

### Empresas

Cientos de empresas, grandes y pequeñas, utilizan Rust en la producción para una variedad de
tareas. Esas tareas incluyen herramientas de línea de comandos, servicios web, herramientas DevOps
dispositivos integrados, análisis y transcodificación de audio y vídeo, criptomonedas,
bioinformática, motores de búsqueda, aplicaciones del Internet de las Cosas, aprendizaje
aprendizaje automático, e incluso partes importantes del navegador web Firefox.

### Desarrolladores de código abierto

Rust es para la gente que quier construir en el lenguaje de programación Rust, la comunidad
herramientas para desarrolladores y bibliotecas. Nos encantaría que contribuyeras al lenguaje Rust

### Personas que valoran la rapidez y la estabilidad

Rust es para la gente que anhela velocidad y estabilidad en un lenguaje. Por velocidad, nos referimos a
nos referimos a la velocidad de los programas que puedes crear con Rust y a la velocidad a la que
Rust te permite escribirlos. Los controles del compilador de Rust aseguran la estabilidad
a través de la adición de características y la refactorización. Esto contrasta con el frágil
código heredado en lenguajes sin estas comprobaciones, que los desarrolladores a menudo
los desarrolladores suelen tener miedo de modificar. Al esforzarse por conseguir abstracciones de coste cero, las características de nivel superior
de alto nivel que compilan en código de bajo nivel tan rápido como el código escrito manualmente, Rust
se esfuerza por hacer que el código seguro sea también código rápido.

El lenguaje Rust también espera dar soporte a muchos otros usuarios; los mencionados
aquí son sólo algunos de los principales interesados. En general, la mayor ambición de Rust
de Rust es eliminar las compensaciones que los programadores han aceptado durante
décadas proporcionando seguridad *y* productividad, velocidad *y* ergonomía. Pruebe
Rust y comprueba si sus opciones te sirven.

## A quién va dirigido este libro

Este libro asume que usted ha escrito código en otro lenguaje de programación pero
no hace ninguna suposición sobre cuál. Hemos tratado de hacer el material
ampliamente accesible para aquellos con una amplia variedad de antecedentes de programación. En
No dedicamos mucho tiempo a hablar de lo que es la programación o de cómo pensar en ella.
sobre ella. Si es completamente nuevo en la programación, le convendría
leer un libro que proporcione específicamente una introducción a la programación.

## Cómo usar este libro

En general, este libro supone que se lee en secuencia de adelante hacia 
atrás. Los capítulos posteriores se basan en los conceptos de los anteriores, y los primeros
capítulos anteriores, y es posible que no se profundice en los detalles de un tema; normalmente
tema en un capítulo posterior.

En este libro encontrará dos tipos de capítulos: capítulos de conceptos y capítulos de proyectos. En los capítulos de concepto, aprenderás sobre un aspecto de Rust. En los capítulos de proyectos
En los capítulos de proyectos, construiremos juntos pequeños programas, aplicando lo que has aprendido hasta ahora.
hasta ahora. Los capítulos 2, 12 y 20 son capítulos de proyectos; el resto son capítulos de conceptos.

El capítulo 1 explica cómo instalar Rust, cómo escribir un programa "¡Hola, mundo!
y cómo utilizar Cargo, el gestor de paquetes y la herramienta de construcción de Rust. El capítulo 2 es una
introducción práctica al lenguaje Rust. Aquí cubrimos los conceptos a un nivel alto, y los capítulos posteriores proporcionarán detalles adicionales. Si quieres ensuciarte las manos de inmediato, el capítulo 2 es el lugar adecuado para ello. Al principio, es posible que
Incluso puede que quieras saltarte el capítulo 3, que cubre características de Rust similares a las de otros lenguajes de programación, y pasar directamente al capítulo 3.
de otros lenguajes de programación, y pasar directamente al capítulo 4 para aprender sobre
sistema de propiedad de Rust. Sin embargo, si eres un aprendiz particularmente meticuloso
que prefiere aprender cada detalle antes de pasar al siguiente, puede que quiera
saltarse el Capítulo 2 e ir directamente al Capítulo 3, volviendo al Capítulo 2 cuando
cuando quieras trabajar en un proyecto aplicando los detalles que has aprendido.

El capítulo 5 trata de los structs y los métodos, y el capítulo 6 cubre los enums, las expresiones `match
y la construcción de flujo de control `if let`. Utilizarás structs y
enums para crear tipos personalizados en Rust.

En el Capítulo 7, aprenderás sobre el sistema de módulos de Rust y sobre las reglas de privacidad
para organizar tu código y su Interfaz de Programación de Aplicaciones pública
(API). El capítulo 8 discute algunas estructuras de datos de colección comunes que la
biblioteca estándar proporciona, como los vectores, las cadenas y los mapas hash. El capítulo 9
explora la filosofía y las técnicas de gestión de errores de Rust.

El capítulo 10 profundiza en los genéricos, los rasgos y los tiempos de vida, que te dan el poder
para definir código que se aplica a múltiples tipos. El capítulo 11 trata de las pruebas,
que incluso con las garantías de seguridad de Rust es necesario para asegurar que la lógica de tu programa es correcta.
lógica de tu programa es correcta. En el Capítulo 12, construiremos nuestra propia implementación de un subconjunto
de funcionalidad de la herramienta de línea de comandos `grep` que busca texto
dentro de los archivos. Para esto, usaremos muchos de los conceptos que discutimos en los
capítulos anteriores.

El capítulo 13 explora los cierres e iteradores: características de Rust que provienen de
de los lenguajes de programación funcionales. En el capítulo 14, examinaremos Cargo con más
profundidad y hablaremos de las mejores prácticas para compartir tus bibliotecas con otros.
El capítulo 15 trata de los punteros inteligentes que proporciona la biblioteca estándar y los
rasgos que permiten su funcionalidad.

En el capítulo 16, recorreremos diferentes modelos de programación concurrente
y hablaremos de cómo Rust te ayuda a programar en múltiples hilos sin miedo.
El capítulo 17 examina cómo el lenguaje de Rust se compara con los principios de la programación orientada a objetos
con los que puede estar familiarizado.

El capítulo 18 es una referencia a los patrones y a la concordancia de patrones, que son poderosas
formas de expresar ideas en los programas de Rust. 
El capítulo 19 contiene temas avanzados de gran interes, incluyendo Rust inseguro, macros, y
más sobre tiempos de vida, rasgos, tipos, funciones y cierres.

En el capítulo 20, completaremos un proyecto en el que implementaremos un servidor web de bajo nivel
multihilo de bajo nivel.

Por último, algunos apéndices contienen información útil sobre el lenguaje a través de un formato de referencia.
El Apéndice A cubre las palabras clave de Rust, el Apéndice B
cubre los operadores y símbolos de Rust, el Apéndice C cubre los rasgos derivables
de la biblioteca estándar, el Apéndice D cubre algunas herramientas de desarrollo útiles, y el Apéndice E
de desarrollo, y el Apéndice E explica las ediciones de Rust.

No hay una forma incorrecta de leer este libro: si quieres saltarlo, hazlo.
Puede que tenga que volver a los capítulos anteriores si experimenta alguna
confusión. Pero haz lo que te funcione.

<span id="ferris"></span>

Una parte importante del proceso de aprendizaje de Rust es aprender a leer los
mensajes de error que muestra el compilador: estos te guiarán hacia un código que funcione.
Como tal, proporcionaremos muchos ejemplos que no compilan junto con el mensaje de error que el compilador mostrará en cada situación.
que el compilador le mostrará en cada situación. Sepa que si introduce
y ejecuta un ejemplo al azar, ¡puede que no compile! Asegúrese de leer el
texto circundante para ver si el ejemplo que está tratando de ejecutar está destinado a
error. Ferris también te ayudará a distinguir el código que no está destinado a funcionar:

| Ferris                                                                 | Meaning                                                                  |
|------------------------------------------------------------------------|--------------------------------------------------|
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain"/>    | ¡Este código no compila!                                                 |
| <img src="img/ferris/panics.svg" class="ferris-explain"/>              | Este código se bloquea!                                                  |
| <img src="img/ferris/unsafe.svg" class="ferris-explain"/>              | Este bloque de código contiene código inseguro.                          |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain"/>| Este código no produce el comportamiento deseado.                        |

En la mayoría de las situaciones, le llevaremos a la versión correcta de cualquier código que
no compila.

## Código fuente

Los archivos fuente a partir de los cuales se ha generado este libro pueden encontrarse en
[GitHub][book].

[book]: https://github.com/rust-lang/book/tree/master/src
