## Apéndice G - Cómo está hecho Rust y "Rust nocturno"

Este apéndice trata sobre cómo Rust está hecho y cómo esto te afecta como
un desarrollador de Rust

### Estabilidad sin estancamiento

Cómo lenguaje, a Rust le importa *mucho* la estabilidad de tu código. Queremos
que Rust sea cual cimientos sólidos cómo la roca sobre los que tu puedas construir,
y si las cosas estuvieran cambiando constantemente, sería imposible.
Al mismo tiempo, si no podemos experimentar con características nuevas, no 
podríamos encontrar deficiencias importantes hasta su lanzamiento, cuando
ya no podemos arreglar nada. 

Nuestra solución a este problema es lo que llamamos "estabilidad sin estancamiento",
y el principio que nos guía es este: nunca deberías tener miedo a actualizar a una
nueva versión estable de Rust. Cada actualización debería venir sin dolores
de cabeza, pero al mismo tiempo traerte nuevas características, menos errores
y tiempos de compilación más rápidos.

### ¡Chu, Chu! Canales de lanzamiento y montando los trenes

El desarrollo de Rust opera en un *horario ferroviario*. Esto siendo, que todo
el desarrollo se lleva cabo en la rama `master` del repositorio de Rust. Los
lanzamientos siguen un modelo ferroviario para lanzamiento de software, que ha
sido utilizado para Cisco IOS y otros proyectos de software. Hay tres *canales de lanzamiento*
para Rust:

* Nocturno
* Beta
* Estable

La mayoría de desarrolladores de Rust usan principalmente el canal estable, pero
los que quieran probar nuevas características experimentales pueden usar
nocturno o beta.

Aquí hay un ejemplo de que cómo funciona el proceso de desarrollo y lanzamiento:
vamos a asumir que el equipo de Rust está trabajando en el lanzamiento de Rust 1.5.
Este lanzamiento ocurrió en Diciembre de 2015, pero no dará con números de
versión realistas. Una nueva caractarística se añade a Rust: un nuevo commit
cae en la rama `master`. Cada noche, una nueva versión de Rust nocturno es
producida. Cada día es una día de lanzamiento, y estos lanzamientos son
creados por nuestra infraestructura automáticamente. Así que según pasa el
tiempo, nuestros lanzamientos tienen esta pinta, una vez por noche:
automatically. So as time passes, our releases look like this, once a night:

```text
nocturno: * - - * - - *
```

Cada seis semanas, ¡es hora de preparar un nuevo lanzamiento! La rama `beta` 
del repositorio de Rust se escinde de la rama `master` utilizada para nocturno.
Ahora hay dos lanzamientos:

```text
nocturno: * - - * - - *
                      |
beta:                 *
```

La mayoría de los usuarios de Rust no utilizan las versiones beta activamente,
pero prueban con estas sus sistemas de CI (Continous Integration, en español,
Integración Continua) para ayudar a Rust a descubrir posibles deficiencias.
Mientras tanto, sigue habiendo un nuevo lanzamiento de Rust nocturno cada noche:

```text
nocturno: * - - * - - * - - * - - *
                      |
beta:                 *
```

Digamos que se ha encontrado una deficiencia. ¡Que bien que hallamos tenido 
algo de tiempo antes de que se haya colado en el lanzamiento estable! El parche
se aplica a `master`, para que la versión nocturna quede arreglada, y el parche 
respalde a la rama `beta`, y una nueva versión beta se produce:

```text
nocturno: * - - * - - * - - * - - * - - *
                      |
beta:                 * - - - - - - - - *
```

Seis semanas tras la creación de la primera beta, ¡es hora de un lanzamiento
estable! La rama `stable` se crea desde la rama `beta`:

```text
nocturno: * - - * - - * - - * - - * - - * - * - *
                      |
beta:                 * - - - - - - - - *
                                        |
estable:                                *
```

¡Hurra! ¡Rust 1.5 está creado! Sin embargo, nos hemos olvidado una cosa: cómo
han pasado seis semanas, necesitamos una nueva beta para la *siguiente* versión
de Rust, 1.6. Así que tras la escisión de `stable` desde `beta`, la siguiente 
versión de `beta` se escinde de `master`:

```text
nocturno: * - - * - - * - - * - - * - - * - * - *
                      |                         |
beta:                 * - - - - - - - - *       *
                                        |
estable:                                *
```


Esto se llama "modelo ferroviario" porque cada seis semanas, una versión "parte
de la estación", pero todavía tiene que hacer un viaje a través del canal beta
antes de llegar al lanzamiento estable.

Rust lanza cada seis semanas, cómo un reloj. Si sabes la fecha de lanzamiento de
uno de los lanzamientos de Rust, puedes conocer el siguiente: seis semanas después.
Un aspecto chulo de tener un lanzamiento cada seis semanas es que el siguiente tren
va a venir pronto. Si una característica se pierde un lanzamiento en concreto, no hay
necesidad de preocuparse: ¡va a haber una nueva en poco tiempo! Esto ayuda a
reducir la presión de introducir una característica sin pulir cerca de la fecha
de lanzamiento.

Gracias a este proceso, siempre puedes comprobar la siguiente versión de Rust
y comprobar por tí mismo lo fácil que es actualizar: ¡si una beta no funciona
cómo se espera de ella, puedes avisar al equipo de Rust y que quede reparada
para el siguiente lanzamiento estable! Una rotura en una versión beta es 
bastante extraño, pero `rustc` sigue siendo software, y los bugs seguirán
existiendo.

### Características inestables

Hay una cosa más con este tiempo de modelo de lanzamiento: características 
inestables. Rust usa una técnica llamada "banderas de características" para
determinar que características estarán activadas en un lanzamiento en concreto.
Si una nueva característica está bajo desarrollo activo, cae en `master`, y por
lo tanto, en nocturno, pero detrás de una *bandera de característica*. Si tú, 
cómo usuario, quieres probar una de las características en desarrollo, puedes,
pero deber usar la versión nocturna correspondiente e indicar tu código fuente
con la bandera apropiada para poder optar a utilizarla.

Si estas utilizando una versión beta o estable de Rust, no puedes utilizar
característica bandera. Esta es la clave que nos permite conseguir uso práctico
antes de declararlas estables para siempre. Aquellos que quieran optar en 
la vanguardia pueden hacerlo, y aquellos que quieran una experiencia sólida
cómo la roca pueden quedarse cón la versión estable y saber que su código no
se romperá. Estabilidad sin estancamiento.

Este libro solo contiene información sobre las características estables, 
cómo las características en progreso todavía están cambiando, y seguramente
serán distintas desde que están escritas en este libro y son habilitadas
en las versiones estables. Puedes encontrar documentación para las 
características exclusivas de las versiones nocturnas online.

### Rustup y el papel de Rust nocturno

Rustup hace que sea sencillo cambiar entre diferentes canales de lanzamiento de,
Rust, de manera global o por proyecto. Por defecto, tendrás Rust estable instalado.
Para instalar nocturno, por ejemplo:

```console
$ rustup toolchain install nightly
```

También puedes ver todas las *cadena de herramientas* (lanzamientos de Rust y 
componenetes asociados) que tienes instaladas con `rustup`. Aquí hay un ejemplo
en uno de los PC con Windows de uno de los autores:

```powershell
> rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc
```

Cómo puedes ver, la cadena de herramientas estable es la opción por defecto. La
mayoría de los usuarios de Rust usan la versión estable la mayoría del tiempo. 
Puede que quieras utilizar la versión estable la mayor parte del tiempo, pero
quieres utilizar la versión nocturna para un proyecto en específico porque 
necesitas una de las características pioneras de dicha versión. Para ello, puedes
usar `rustup override` en el directorio del proyecto para establecer la cadena
de herramientas nocturna como la opción que `rustup` debe utilizar cuando estés
en ese directorio en concreto:

```console
$ cd ~/proyectos/necesita-nocturno
$ rustup override set nightly
```

Ahora, cada vez que invoques `rustc` o `cargo` dentro de *~/proyectos/necesita-nocturno*,
`rustup` se asegurará de que estés utilizando Rust nocturno, en vez de la
versión estable por defecto de Rust. ¡Esto es muy útil cuándo tienes muchos proyectos
en Rust!

### El proceso RFC y equipos 

¿Cómo puedes aprender sobre estas nuevas características? El modelo de desarrollo
de Rust sigue un 
*proceso de Solicitud De Comentarios (Requesto for Comments, en ingles, RFC)*.
Si te gustaría una mejora en Rust, puedes escribir una proposición, llamada
RFC.

Cualquiera puede escribir RFCs para mejorar Rust, y estas proposiciones
son revisadas y discutidas por el equipo de Rusts, el cuál está dividido
por tópicos en subequipos. Hay una lista completa de los equipos
[en la página web de Rust](https://www.rust-lang.org/es/governance),
el cuál incluye equipos para cada área del proyecto: diseño de lenguaje,
implementación del compilador, infraestructura, documentación y más. El
equipo correspondiente lee la proposición y los comentarios, escribe sus
propios comentarios, y al final, hay un consenso sobre si aceptar o rechazar
la característica.

Si la característica es aceptada, se abre un asunto en el repositorio de Rust,
y alguien puede implementarlo. ¡La persona que mejor lo implemente, es probable que
no sea la misma que propuso la característica en primer lugar! Cuando la 
implementación esté lista, cae en la rama `master` detrás de una bandera de
característica, cómo ya comentamos en la sección <!-- ignorar -->
["Características inestables"](#características-inestables). 

Después de algún tiempo, cuándo los desarrolladores de Rust que utilizan los 
lanzamientos nocturnos pueden probar la nueva característica, los miembros del 
equipo discutirán sobre la característica, y cómo ha funcionado en la versión
nocturna, y decidir si debería pasar a Rust estable o no. Si la decisión es 
seguir adelante, se quita la bandera de característica, ¡y la característica
ya se considera estable! Se monta en el tren hacia un nuevo lanzamiento de 
Rust estable.