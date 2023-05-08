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

### Características inestable

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

Rustup makes it easy to change between different release channels of Rust, on a
global or per-project basis. By default, you’ll have stable Rust installed. To
install nightly, for example:

```console
$ rustup toolchain install nightly
```

You can see all of the *toolchains* (releases of Rust and associated
components) you have installed with `rustup` as well. Here’s an example on one
of your authors’ Windows computer:

```powershell
> rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc
```

As you can see, the stable toolchain is the default. Most Rust users use stable
most of the time. You might want to use stable most of the time, but use
nightly on a specific project, because you care about a cutting-edge feature.
To do so, you can use `rustup override` in that project’s directory to set the
nightly toolchain as the one `rustup` should use when you’re in that directory:

```console
$ cd ~/projects/needs-nightly
$ rustup override set nightly
```

Now, every time you call `rustc` or `cargo` inside of
*~/projects/needs-nightly*, `rustup` will make sure that you are using nightly
Rust, rather than your default of stable Rust. This comes in handy when you
have a lot of Rust projects!

### The RFC Process and Teams

So how do you learn about these new features? Rust’s development model follows
a *Request For Comments (RFC) process*. If you’d like an improvement in Rust,
you can write up a proposal, called an RFC.

Anyone can write RFCs to improve Rust, and the proposals are reviewed and
discussed by the Rust team, which is comprised of many topic subteams. There’s
a full list of the teams [on Rust’s
website](https://www.rust-lang.org/governance), which includes teams for
each area of the project: language design, compiler implementation,
infrastructure, documentation, and more. The appropriate team reads the
proposal and the comments, writes some comments of their own, and eventually,
there’s consensus to accept or reject the feature.

If the feature is accepted, an issue is opened on the Rust repository, and
someone can implement it. The person who implements it very well may not be the
person who proposed the feature in the first place! When the implementation is
ready, it lands on the `master` branch behind a feature gate, as we discussed
in the [“Unstable Features”](#unstable-features)<!-- ignore --> section.

After some time, once Rust developers who use nightly releases have been able
to try out the new feature, team members will discuss the feature, how it’s
worked out on nightly, and decide if it should make it into stable Rust or not.
If the decision is to move forward, the feature gate is removed, and the
feature is now considered stable! It rides the trains into a new stable release
of Rust.
