<!-- ia-translate: true -->

## Definindo um enum {#defining-an-enum}

Onde structs oferecem uma forma de agrupar campos e dados relacionados, como um
`Rectangle` com seus `width` e `height`, enums oferecem uma forma de dizer que um
valor é um de um conjunto possível de valores. Por exemplo, podemos querer dizer
que um `Rectangle` é uma de um conjunto de formas possíveis que também inclui
`Circle` e `Triangle`. Para isso, o Rust permite codificar essas possibilidades
como um enum.

Vamos ver uma situação que podemos querer expressar em código e entender por que
enums são úteis e mais apropriados que structs neste caso. Suponha que precisamos
trabalhar com endereços IP. Atualmente, dois padrões principais são usados para
endereços IP: a versão quatro e a versão seis. Como essas são as únicas
possibilidades para um endereço IP que nosso programa vai encontrar, podemos
_enumerar_ todas as variantes possíveis, e é daí que vem o nome “enumeração”.

Qualquer endereço IP pode ser ou um endereço versão quatro ou um endereço versão
seis, mas não os dois ao mesmo tempo. Essa propriedade dos endereços IP torna a
estrutura de dados enum apropriada, porque um valor de enum só pode ser uma das
suas variantes. Tanto endereços versão quatro quanto versão seis ainda são
fundamentalmente endereços IP, então devem ser tratados como o mesmo tipo quando o
código lida com situações que se aplicam a qualquer tipo de endereço IP.

Podemos expressar esse conceito em código definindo uma enumeração `IpAddrKind` e
listando os tipos possíveis que um endereço IP pode ter, `V4` e `V6`. Essas são as
variantes do enum:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:def}}
```

`IpAddrKind` agora é um tipo de dado personalizado que podemos usar em outras
partes do nosso código.

### Valores de enum {#enum-values}

Podemos criar instâncias de cada uma das duas variantes de `IpAddrKind` assim:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:instance}}
```

Observe que as variantes do enum ficam dentro do namespace do seu identificador, e
usamos dois-pontos duplos para separar os dois. Isso é útil porque agora os valores
`IpAddrKind::V4` e `IpAddrKind::V6` são do mesmo tipo: `IpAddrKind`. Podemos
então, por exemplo, definir uma função que recebe qualquer `IpAddrKind`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn}}
```

E podemos chamar essa função com qualquer uma das variantes:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn_call}}
```

Usar enums tem ainda mais vantagens. Pensando mais sobre nosso tipo de endereço
IP, no momento não temos como armazenar o _dado_ do endereço IP em si; sabemos
apenas o _tipo_ dele. Como você acabou de aprender sobre structs no Capítulo 5,
pode ser tentador resolver esse problema com structs, como mostrado na Listagem 6-1.

<Listing number="6-1" caption="Armazenando os dados e a variante `IpAddrKind` de um endereço IP usando um `struct`">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-01/src/main.rs:here}}
```

</Listing>

Aqui, definimos um struct `IpAddr` que tem dois campos: um campo `kind`, do tipo
`IpAddrKind` (o enum que definimos antes), e um campo `address`, do tipo `String`.
Temos duas instâncias desse struct. A primeira é `home`, e ela tem o valor
`IpAddrKind::V4` como seu `kind`, com os dados de endereço associados `127.0.0.1`.
A segunda instância é `loopback`. Ela tem a outra variante de `IpAddrKind` como
seu valor `kind`, `V6`, e tem o endereço `::1` associado a ele. Usamos um struct
para agrupar os valores `kind` e `address`, então agora a variante está associada
ao valor.

No entanto, representar o mesmo conceito usando apenas um enum é mais conciso: em
vez de um enum dentro de um struct, podemos colocar os dados diretamente em cada
variante do enum. Esta nova definição do enum `IpAddr` diz que as variantes `V4` e
`V6` terão valores `String` associados:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-02-enum-with-data/src/main.rs:here}}
```

Anexamos os dados a cada variante do enum diretamente, então não há necessidade de
um struct extra. Aqui, também é mais fácil ver outro detalhe de como enums
funcionam: o nome de cada variante do enum que definimos também se torna uma função
que constrói uma instância do enum. Ou seja, `IpAddr::V4()` é uma chamada de
função que recebe um argumento `String` e retorna uma instância do tipo `IpAddr`.
Essa função construtora é definida automaticamente como resultado da definição do
enum.

Há outra vantagem em usar um enum em vez de um struct: cada variante pode ter
tipos e quantidades diferentes de dados associados. Endereços IP versão quatro
sempre terão quatro componentes numéricos, com valores entre 0 e 255. Se
quiséssemos armazenar endereços `V4` como quatro valores `u8`, mas ainda assim
representar endereços `V6` como um único valor `String`, não conseguiríamos fazer
isso com um struct. Enums resolvem esse caso com facilidade:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-03-variants-with-different-data/src/main.rs:here}}
```

Mostramos várias formas diferentes de definir estruturas de dados para armazenar
endereços IP versão quatro e versão seis. No entanto, acontece que guardar
endereços IP e codificar qual é o tipo deles é tão comum que [a biblioteca padrão
já tem uma definição que podemos usar!][IpAddr]<!-- ignore --> Vamos ver como a
biblioteca padrão define `IpAddr`. Ela tem exatamente o enum e as variantes que
definimos e usamos, mas incorpora os dados de endereço dentro das variantes, na
forma de dois structs diferentes, que são definidos de forma diferente para cada
variante:

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

Este código ilustra que você pode colocar qualquer tipo de dado dentro de uma
variante de enum: strings, tipos numéricos ou structs, por exemplo. Você pode até
incluir outro enum! Além disso, tipos da biblioteca padrão muitas vezes não são
muito mais complicados do que o que você poderia criar.

Observe que, mesmo que a biblioteca padrão contenha uma definição para `IpAddr`,
podemos criar e usar nossa própria definição sem conflito, porque não trouxemos a
definição da biblioteca padrão para o nosso escopo. Vamos falar mais sobre trazer
tipos para o escopo no Capítulo 7.

Vamos ver outro exemplo de enum na Listagem 6-2: este tem uma grande variedade de
tipos embutidos em suas variantes.

<Listing number="6-2" caption="Um enum `Message` cujas variantes armazenam diferentes quantidades e tipos de valores">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

</Listing>

Este enum tem quatro variantes com tipos diferentes:

- `Quit`: não tem nenhum dado associado a ele
- `Move`: tem campos nomeados, como um struct
- `Write`: inclui uma única `String`
- `ChangeColor`: inclui três valores `i32`

Definir um enum com variantes como as da Listagem 6-2 é parecido com definir
diferentes tipos de struct, exceto que o enum não usa a palavra-chave `struct` e
todas as variantes ficam agrupadas sob o tipo `Message`. Os structs a seguir
poderiam guardar os mesmos dados que as variantes do enum anterior guardam:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-04-structs-similar-to-message-enum/src/main.rs:here}}
```

Mas, se usássemos os structs diferentes, cada uma com seu próprio tipo, não
conseguiríamos definir com a mesma facilidade uma função que recebe qualquer um
desses tipos de mensagem, como conseguimos com o enum `Message` da Listagem 6-2,
que é um único tipo.

Há mais uma semelhança entre enums e structs: assim como podemos definir métodos
em structs usando `impl`, também podemos definir métodos em enums. Aqui está um
método chamado `call` que poderíamos definir no nosso enum `Message`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-05-methods-on-enums/src/main.rs:here}}
```

O corpo do método usaria `self` para obter o valor no qual chamamos o método. Neste
exemplo, criamos uma variável `m` que tem o valor
`Message::Write(String::from("hello"))`, e é isso que `self` será no corpo do
método `call` quando `m.call()` rodar.

Vamos ver outro enum da biblioteca padrão que é muito comum e útil: `Option`.

<!-- Old headings. Do not remove or links may break. -->
<a id="the-option-enum-and-its-advantages-over-null-values"></a>

### O enum `Option` {#the-option-enum}

Esta seção explora um estudo de caso de `Option`, outro enum definido pela
biblioteca padrão. O tipo `Option` codifica o cenário muito comum em que um valor
pode ser algo, ou pode ser nada.

Por exemplo, se você pedir o primeiro item de uma lista não vazia, vai obter um
valor. Se pedir o primeiro item de uma lista vazia, não vai obter nada. Expressar
esse conceito em termos do sistema de tipos significa que o compilador consegue
verificar se você tratou todos os casos que deveria tratar; essa funcionalidade
pode evitar bugs que são extremamente comuns em outras linguagens de programação.

O design de linguagens de programação costuma ser pensado em termos de quais
recursos incluir, mas os recursos que você exclui também são importantes. Rust não
tem o recurso null que muitas outras linguagens têm. _Null_ é um valor que significa
que não há valor ali. Em linguagens com null, variáveis podem sempre estar em um de
dois estados: null ou não-null.

Na sua apresentação de 2009 “Null References: The Billion Dollar Mistake”, Tony
Hoare, o inventor do null, disse o seguinte:

> Eu chamo isso de meu erro de um bilhão de dólares. Naquela época, eu estava
> projetando o primeiro sistema de tipos abrangente para referências em uma
> linguagem orientada a objetos. Meu objetivo era garantir que todo uso de
> referências fosse absolutamente seguro, com verificações feitas automaticamente
> pelo compilador. Mas eu não resisti à tentação de incluir uma referência nula,
> simplesmente porque era muito fácil de implementar. Isso levou a inúmeros
> erros, vulnerabilidades e falhas de sistema, que provavelmente causaram um
> bilhão de dólares em prejuízos e danos nos últimos quarenta anos.

O problema com valores null é que, se você tentar usar um valor null como se
fosse não-null, vai obter algum tipo de erro. Como essa propriedade de null ou
não-null está em todo lugar, é extremamente fácil cometer esse tipo de erro.

No entanto, o conceito que null tenta expressar ainda é útil: um null é um valor
que está atualmente inválido ou ausente por algum motivo.

O problema não está realmente no conceito, mas na implementação específica. Por
isso, o Rust não tem null, mas tem um enum que consegue codificar o conceito de um
valor estar presente ou ausente. Esse enum é `Option<T>`, e ele é [definido pela
biblioteca padrão][option]<!-- ignore --> da seguinte forma:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

O enum `Option<T>` é tão útil que ele até está incluído no prelúdio; você não
precisa trazê-lo para o escopo explicitamente. Suas variantes também estão incluídas
no prelúdio: você pode usar `Some` e `None` diretamente, sem o prefixo `Option::`.
O enum `Option<T>` continua sendo um enum comum, e `Some(T)` e `None` continuam
sendo variantes do tipo `Option<T>`.

A sintaxe `<T>` é um recurso do Rust que ainda não abordamos. É um parâmetro de
tipo genérico, e vamos ver genéricos com mais detalhes no Capítulo 10. Por ora, tudo
que você precisa saber é que `<T>` significa que a variante `Some` do enum `Option`
pode guardar um dado de qualquer tipo, e que cada tipo concreto usado no lugar de
`T` torna o tipo `Option<T>` geral um tipo diferente. Aqui estão alguns exemplos de
uso de valores `Option` para guardar tipos numéricos e tipos `char`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-06-option-examples/src/main.rs:here}}
```

O tipo de `some_number` é `Option<i32>`. O tipo de `some_char` é `Option<char>`,
que é um tipo diferente. O Rust consegue inferir esses tipos porque especificamos
um valor dentro da variante `Some`. Para `absent_number`, o Rust exige que
anotemos o tipo geral `Option`: o compilador não consegue inferir o tipo que a
variante `Some` correspondente vai guardar olhando apenas para um valor `None`. Aqui,
dizemos ao Rust que queremos que `absent_number` seja do tipo `Option<i32>`.

Quando temos um valor `Some`, sabemos que um valor está presente, e ele está
guardado dentro do `Some`. Quando temos um valor `None`, de certo modo ele significa
a mesma coisa que null: não temos um valor válido. Então, por que ter `Option<T>` é
melhor do que ter null?

Em resumo, como `Option<T>` e `T` (em que `T` pode ser qualquer tipo) são tipos
diferentes, o compilador não nos deixa usar um valor `Option<T>` como se ele fosse
definitivamente um valor válido. Por exemplo, este código não compila, porque tenta
somar um `i8` com um `Option<i8>`:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/src/main.rs:here}}
```

Se rodarmos este código, recebemos uma mensagem de erro como esta:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/output.txt}}
```

Caramba! Na prática, essa mensagem de erro significa que o Rust não entende como
somar um `i8` e um `Option<i8>`, porque eles são tipos diferentes. Quando temos um
valor de um tipo como `i8` em Rust, o compilador garante que sempre temos um valor
válido. Podemos seguir com confiança sem precisar verificar se há null antes de usar
esse valor. Só quando temos um `Option<i8>` (ou qualquer que seja o tipo de valor
com que estamos trabalhando) é que precisamos nos preocupar com a possibilidade de
não ter um valor, e o compilador vai garantir que tratemos esse caso antes de usar
o valor.

Em outras palavras, você precisa converter um `Option<T>` em um `T` antes de poder
realizar operações de `T` com ele. Em geral, isso ajuda a evitar um dos problemas
mais comuns com null: presumir que algo não é null quando na verdade é.

Eliminar o risco de presumir incorretamente um valor não-null ajuda você a ter mais
confiança no seu código. Para ter um valor que pode ser null, você precisa optar
explicitamente por isso, tornando o tipo desse valor `Option<T>`. Depois, quando
usar esse valor, você é obrigado a tratar explicitamente o caso em que ele é null.
Em todo lugar em que um valor tem um tipo que não é `Option<T>`, você _pode_ presumir
com segurança que o valor não é null. Essa foi uma decisão de design deliberada do
Rust para limitar a presença de null e aumentar a segurança do código Rust.

Então, como você obtém o valor `T` de dentro de uma variante `Some` quando tem um
valor do tipo `Option<T>` e quer usar esse valor? O enum `Option<T>` tem um grande
número de métodos úteis em várias situações; você pode conferi-los na [documentação
dele][docs]<!-- ignore -->. Se familiarizar com os métodos de `Option<T>` vai ser
extremamente útil na sua jornada com Rust.

Em geral, para usar um valor `Option<T>`, você quer ter código que trate cada
variante. Você quer um código que rode só quando tiver um valor `Some(T)`, e esse
código pode usar o `T` interno. Você quer outro código que rode só se tiver um valor
`None`, e esse código não tem um valor `T` disponível. A expressão `match` é uma
construção de controle de fluxo que faz exatamente isso quando usada com enums: ela
roda código diferente dependendo de qual variante do enum ela tem, e esse código
pode usar os dados dentro do valor correspondente.

[IpAddr]: ../std/net/enum.IpAddr.html
[option]: ../std/option/enum.Option.html
[docs]: ../std/option/enum.Option.html
