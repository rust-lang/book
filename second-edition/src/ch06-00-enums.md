# Enumeraciones y patrones de coincidencia

En este capítulo veremos las *enumeraciones*, que también son conocidas como *enums*.
Las enumeraciones te permiten definir un tipo enumerando sus posibles valores. Primero,
definiremos y usaremos un enumerador para mostrar como puede codificar el significado con datos. A continuación, exploraremos una enumeración particularmente útil, llamada `Option`, que
expresa que un valor puede tener algo o nada. Luego veremos como la coincidencia de patrones
en la expresión `match`  facilita ejecutar código para diferentes valores en una enumeración. Finalmente, cubriremos como el constructor `if let`
es otro conveniente y conciso idioma disponible para que puedas manejar
enumeraciones en tu código.

Las enumeraciones son una característica en muchos languajes, pero sus capacidades son distintas en cada lenguaje. Las enumeraciones de Rust son más similares a *tipos de datos algebraicos* en lenguajes funcionales como F#, OCaml y Haskell.
