<!-- ia-translate: true -->
## Um programa de exemplo usando structs {#an-example-program-using-structs}

Para entender quando podemos querer usar structs, vamos escrever um programa que calcula a área de um retângulo. Começaremos usando variáveis simples e depois refatoraremos o programa até usarmos structs.

Vamos criar um novo projeto binário com Cargo chamado _rectangles_, que vai receber a largura e a altura de um retângulo especificadas em pixels e calcular a área do retângulo. A Listagem 5-8 mostra um programa curto com uma forma de fazer exatamente isso, no arquivo _src/main.rs_ do nosso projeto.

<Listing number="5-8" file-name="src/main.rs" caption="Calculando a área de um retângulo especificado por variáveis separadas de largura e altura">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:all}}
```

</Listing>

Agora, execute este programa usando `cargo run`:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/output.txt}}
```

Este código consegue calcular a área do retângulo chamando a função `area` com cada dimensão, mas podemos fazer mais para deixá-lo claro e legível.

O problema deste código fica evidente na assinatura de `area`:

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:here}}
```

A função `area` deveria calcular a área de um retângulo, mas a função que escrevemos tem dois parâmetros, e não fica claro em nenhum lugar do programa que os parâmetros estão relacionados. Seria mais legível e mais fácil de manter agrupar largura e altura. Já discutimos uma maneira de fazer isso na seção [“O tipo tupla”][the-tuple-type]<!-- ignore --> do Capítulo 3: usando tuplas.

### Refatorando com tuplas {#refactoring-with-tuples}

A Listagem 5-9 mostra outra versão do nosso programa, que usa tuplas.

<Listing number="5-9" file-name="src/main.rs" caption="Especificando a largura e a altura do retângulo com uma tupla">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-09/src/main.rs}}
```

</Listing>

De um jeito, este programa é melhor. Tuplas nos permitem adicionar um pouco de estrutura, e agora passamos apenas um argumento. Mas, de outro jeito, esta versão é menos clara: tuplas não nomeiam os seus elementos, então precisamos acessar as partes da tupla por índice, o que deixa o cálculo menos óbvio.

Trocar largura e altura não faria diferença no cálculo da área, mas, se quisermos desenhar o retângulo na tela, faria! Teríamos que lembrar que `width` é o índice `0` da tupla e `height` é o índice `1`. Isso seria ainda mais difícil de entender e de lembrar para outra pessoa que usasse o nosso código. Como não deixamos explícito o significado dos dados no código, fica mais fácil introduzir erros.

<!-- Old headings. Do not remove or links may break. -->

<a id="refactoring-with-structs-adding-more-meaning"></a>

### Refatorando com structs {#refactoring-with-structs}

Usamos structs para adicionar significado, rotulando os dados. Podemos transformar a tupla que estamos usando em um struct com um nome para o conjunto e nomes para as partes, como mostra a Listagem 5-10.

<Listing number="5-10" file-name="src/main.rs" caption="Definindo um struct `Rectangle`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-10/src/main.rs}}
```

</Listing>

Aqui, definimos um struct e o chamamos de `Rectangle`. Dentro das chaves, definimos os campos `width` e `height`, ambos do tipo `u32`. Depois, em `main`, criamos uma instância específica de `Rectangle` com largura `30` e altura `50`.

Nossa função `area` agora é definida com um parâmetro, que chamamos de `rectangle`, cujo tipo é um immutable borrow de uma instância do struct `Rectangle`. Como mencionamos no Capítulo 4, queremos tomar o struct emprestado em vez de tomar o seu ownership. Assim, `main` mantém o ownership e pode continuar usando `rect1`, que é o motivo de usarmos o `&` na assinatura da função e na chamada.

A função `area` acessa os campos `width` e `height` da instância de `Rectangle` (observe que acessar campos de uma instância de struct emprestada não move os valores dos campos, e por isso é comum ver empréstimos de structs). A assinatura da nossa função `area` agora diz exatamente o que queremos: calcular a área de um `Rectangle`, usando os seus campos `width` e `height`. Isso deixa claro que largura e altura estão relacionadas entre si, e dá nomes descritivos aos valores em vez de usar os índices `0` e `1` da tupla. É um ganho de clareza.

<!-- Old headings. Do not remove or links may break. -->

<a id="adding-useful-functionality-with-derived-traits"></a>

### Adicionando funcionalidade com traits derivados {#adding-functionality-with-derived-traits}

Seria útil poder imprimir uma instância de `Rectangle` enquanto depuramos o programa e ver os valores de todos os seus campos. A Listagem 5-11 tenta usar a [macro `println!`][println]<!-- ignore -->, como fizemos nos capítulos anteriores. Isso não vai funcionar, porém.

<Listing number="5-11" file-name="src/main.rs" caption="Tentando imprimir uma instância de `Rectangle`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/src/main.rs}}
```

</Listing>

Ao compilar este código, recebemos um erro com esta mensagem principal:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:3}}
```

A macro `println!` pode fazer vários tipos de formatação e, por padrão, as chaves dizem ao `println!` para usar a formatação conhecida como `Display`: saída destinada ao consumo direto do usuário final. Os tipos primitivos que vimos até agora implementam `Display` por padrão, porque só há uma forma de querer mostrar um `1` ou qualquer outro tipo primitivo para o usuário. Mas, com structs, a forma como `println!` deve formatar a saída é menos clara, porque há mais possibilidades de exibição: você quer vírgulas ou não? Quer imprimir as chaves? Todos os campos devem aparecer? Por causa dessa ambiguidade, o Rust não tenta adivinhar o que queremos, e structs não têm uma implementação de `Display` pronta para usar com `println!` e o marcador `{}`.

Se continuarmos lendo os erros, encontraremos esta nota útil:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:9:10}}
```

Vamos tentar! A chamada da macro `println!` agora vai ficar assim: `println!("rect1 is {rect1:?}");`. Colocar o especificador `:?` dentro das chaves diz ao `println!` que queremos usar um formato de saída chamado `Debug`. O trait `Debug` nos permite imprimir nosso struct de um jeito útil para desenvolvedores, para que possamos ver o valor dela enquanto depuramos o código.

Compile o código com essa mudança. Droga! Ainda recebemos um erro:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:3}}
```

Mas, de novo, o compilador nos dá uma nota útil:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:9:10}}
```

O Rust _tem_ funcionalidade para imprimir informações de depuração, mas precisamos optar explicitamente por disponibilizá-la para o nosso struct. Para isso, adicionamos o atributo externo `#[derive(Debug)]` logo antes da definição do struct, como mostra a Listagem 5-12.

<Listing number="5-12" file-name="src/main.rs" caption="Adicionando o atributo para derivar o trait `Debug` e imprimindo a instância de `Rectangle` com formatação de depuração">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/src/main.rs}}
```

</Listing>

Agora, quando executamos o programa, não recebemos mais erros e vemos a seguinte saída:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/output.txt}}
```

Legal! Não é a saída mais bonita, mas mostra os valores de todos os campos desta instância, o que certamente ajudaria durante a depuração. Quando temos structs maiores, é útil ter uma saída um pouco mais fácil de ler; nesses casos, podemos usar `{:#?}` em vez de `{:?}` na string do `println!`. Neste exemplo, usar o estilo `{:#?}` vai produzir a seguinte saída:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-02-pretty-debug/output.txt}}
```

Outra forma de imprimir um valor usando o formato `Debug` é usar a macro [`dbg!`][dbg]<!-- ignore -->, que toma o ownership de uma expressão (ao contrário do `println!`, que recebe uma referência), imprime o arquivo e o número da linha onde a chamada da macro `dbg!` acontece no seu código junto com o valor resultante dessa expressão, e devolve o ownership do valor.

> Nota: Chamar a macro `dbg!` imprime no fluxo de erro padrão do console (`stderr`), diferente do `println!`, que imprime no fluxo de saída padrão do console (`stdout`). Vamos falar mais sobre `stderr` e `stdout` na seção [“Redirecionando erros para a saída de erro padrão” do Capítulo 12][err]<!-- ignore -->.

Aqui está um exemplo em que nos interessa o valor atribuído ao campo `width`, bem como o valor do struct inteiro em `rect1`:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/src/main.rs}}
```

Podemos colocar `dbg!` em volta da expressão `30 * scale` e, como `dbg!` devolve o ownership do valor da expressão, o campo `width` vai receber o mesmo valor que receberia se não tivéssemos a chamada a `dbg!`. Não queremos que `dbg!` tome o ownership de `rect1`, então usamos uma referência a `rect1` na chamada seguinte. Eis a saída deste exemplo:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/output.txt}}
```

O primeiro trecho da saída veio da linha 10 de _src/main.rs_, onde depuramos a expressão `30 * scale`, cujo valor resultante é `60` (a formatação `Debug` implementada para inteiros imprime apenas o valor). A chamada `dbg!` na linha 14 de _src/main.rs_ imprime o valor de `&rect1`, que é o struct `Rectangle`. Essa saída usa a formatação `Debug` “bonita” do tipo `Rectangle`. A macro `dbg!` pode ser muito útil quando você está tentando descobrir o que o seu código está fazendo!

Além do trait `Debug`, o Rust oferece vários traits que podemos usar com o atributo `derive`, que adicionam comportamentos úteis aos nossos tipos personalizados. Esses traits e seus comportamentos estão listados no [Apêndice C][app-c]<!-- ignore -->. Veremos como implementar esses traits com comportamento próprio, bem como criar os seus próprios traits, no Capítulo 10. Também existem muitos atributos além de `derive`; para mais informações, consulte [a seção “Attributes” da Referência do Rust][attributes].

Nossa função `area` é muito específica: ela só calcula a área de retângulos. Seria útil associar esse comportamento mais de perto ao nosso struct `Rectangle`, porque ela não funcionaria com nenhum outro tipo. Vamos ver como podemos continuar refatorando este código transformando a função `area` em um método `area` definido no nosso tipo `Rectangle`.

[the-tuple-type]: ch03-02-data-types.html#the-tuple-type
[app-c]: appendix-03-derivable-traits.md
[println]: ../std/macro.println.html
[dbg]: ../std/macro.dbg.html
[err]: ch12-06-writing-to-stderr-instead-of-stdout.html
[attributes]: ../reference/attributes.html
