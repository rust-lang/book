<!-- ia-translate: true -->
# Enums e Pattern Matching {#enums-and-pattern-matching}

Neste capítulo, vamos analisar as enumerações, também chamadas de _enums_.
Enums permitem definir um tipo listando as suas variantes possíveis. Primeiro,
vamos definir e usar um enum para mostrar como um enum pode codificar um
significado junto com dados. Em seguida, vamos explorar um enum particularmente
útil, chamado `Option`, que expressa que um valor pode ser algo ou nada. Depois,
vamos ver como o pattern matching na expressão `match` facilita executar códigos
diferentes para valores diferentes de um enum. Por fim, vamos cobrir como a
construção `if let` é outra construção idiomática prática e concisa disponível para lidar com
enums no seu código.
