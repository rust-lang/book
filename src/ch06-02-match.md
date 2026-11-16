<!-- ia-translate: true -->
<!-- Old headings. Do not remove or links may break. -->

<a id="the-match-control-flow-operator"></a>

## A construção de fluxo de controle `match` {#the-match-control-flow-construct}

Rust tem uma construção de fluxo de controle extremamente poderosa, chamada `match`, que permite comparar um valor com uma série de padrões e então executar código de acordo com o padrão que casar. Padrões podem ser formados por valores literais, nomes de variáveis, curingas e muitas outras coisas; o [Capítulo
19][ch19-00-patterns]<!-- ignore --> cobre todos os tipos diferentes de padrões e o que cada um faz. O poder do `match` vem da expressividade dos padrões e do fato de que o compilador confirma que todos os casos possíveis são tratados.

Pense em uma expressão `match` como uma máquina de separar moedas: as moedas deslizam por uma trilha com furos de tamanhos variados ao longo dela, e cada moeda cai pelo primeiro furo em que cabe. Do mesmo modo, os valores passam por cada padrão de um `match`, e no primeiro padrão em que o valor “encaixa”, o valor cai no bloco de código associado para ser usado durante a execução.

Já que falamos em moedas, vamos usá-las como exemplo com `match`! Podemos escrever uma função que recebe uma moeda de dólar americano desconhecida e, de maneira semelhante à máquina de contagem, determina qual é a moeda e retorna o valor dela em centavos, como mostrado na Listagem 6-3.

<Listing number="6-3" caption="Um enum e uma expressão `match` cujos padrões são as variantes do enum">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-03/src/main.rs:here}}
```

</Listing>

Vamos detalhar o `match` na função `value_in_cents`. Primeiro, escrevemos a palavra-chave `match` seguida de uma expressão, que neste caso é o valor `coin`. Isso parece muito semelhante a uma expressão condicional usada com `if`, mas há uma grande diferença: com `if`, a condição precisa resultar em um valor booleano, mas aqui ela pode ser de qualquer tipo. O tipo de `coin` neste exemplo é o enum `Coin` que definimos na primeira linha.

Em seguida, vêm os braços do `match`. Um braço tem duas partes: um padrão e algum código. O primeiro braço aqui tem um padrão que é o valor `Coin::Penny`, depois o operador `=>`, que separa o padrão do código a ser executado. O código, neste caso, é apenas o valor `1`. Cada braço é separado do próximo por uma vírgula.

Quando a expressão `match` é executada, ela compara o valor resultante com o padrão de cada braço, em ordem. Se um padrão casar com o valor, o código associado a esse padrão é executado. Se esse padrão não casar com o valor, a execução continua para o próximo braço, assim como na máquina de separar moedas. Podemos ter quantos braços forem necessários: na Listagem 6-3, nosso `match` tem quatro braços.

O código associado a cada braço é uma expressão, e o valor resultante da expressão do braço que casou é o valor retornado por toda a expressão `match`.

Normalmente não usamos chaves quando o código do braço do `match` é curto, como na Listagem 6-3, em que cada braço apenas retorna um valor. Se você quiser executar várias linhas de código em um braço do `match`, precisa usar chaves, e a vírgula depois do braço passa a ser opcional. Por exemplo, o código a seguir imprime “Lucky penny!” toda vez que o método é chamado com um `Coin::Penny`, mas ainda retorna o último valor do bloco, `1`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```

### Padrões que vinculam valores {#patterns-that-bind-to-values}

Outro recurso útil dos braços do `match` é que eles podem vincular partes dos valores que casam com o padrão. É assim que conseguimos extrair valores de dentro das variantes de um enum.

Como exemplo, vamos mudar uma das variantes do nosso enum para que ela guarde dados dentro dela. De 1999 a 2008, os Estados Unidos cunharam moedas de 25 centavos (quarters) com designs diferentes para cada um dos 50 estados, em um dos lados. Nenhuma outra moeda recebeu designs de estados, então só as moedas de 25 centavos têm esse valor extra. Podemos adicionar essa informação ao nosso `enum` alterando a variante `Quarter` para incluir um valor `UsState` armazenado dentro dela, o que fizemos na Listagem 6-4.

<Listing number="6-4" caption="Um enum `Coin` no qual a variante `Quarter` também armazena um valor `UsState`">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```

</Listing>

Imagine que um amigo esteja tentando colecionar as 50 moedas de 25 centavos dos estados. Enquanto separamos o troco por tipo de moeda, também vamos anunciar o nome do estado associado a cada moeda de 25 centavos, para que, se for um que o nosso amigo ainda não tem, ele possa acrescentá-lo à coleção.

Na expressão `match` deste código, adicionamos uma variável chamada `state` ao padrão que casa com valores da variante `Coin::Quarter`. Quando um `Coin::Quarter` casa, a variável `state` vai vincular o valor do estado daquela moeda. Então, podemos usar `state` no código desse braço, assim:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```

Se chamássemos `value_in_cents(Coin::Quarter(UsState::Alaska))`, `coin` seria `Coin::Quarter(UsState::Alaska)`. Quando comparamos esse valor com cada um dos braços do `match`, nenhum deles casa até chegarmos a `Coin::Quarter(state)`. Nesse ponto, a vinculação de `state` será o valor `UsState::Alaska`. Podemos então usar essa vinculação na expressão `println!`, obtendo assim o valor do estado de dentro da variante `Quarter` do enum `Coin`.

<!-- Old headings. Do not remove or links may break. -->

<a id="matching-with-optiont"></a>

### O padrão `match` com `Option<T>` {#the-optiont-match-pattern}


Na seção anterior, queríamos obter o valor interno `T` do caso `Some` ao usar `Option<T>`; também podemos tratar `Option<T>` com `match`, da mesma forma que fizemos com o enum `Coin`! Em vez de comparar moedas, vamos comparar as variantes de `Option<T>`, mas a maneira como a expressão `match` funciona continua a mesma.

Digamos que queremos escrever uma função que recebe um `Option<i32>` e, se houver um valor dentro dele, soma 1 a esse valor. Se não houver valor dentro dele, a função deve retornar o valor `None` e não tentar realizar nenhuma operação.

Essa função é muito fácil de escrever, graças ao `match`, e ficará parecida com a Listagem 6-5.

<Listing number="6-5" caption="Uma função que usa uma expressão `match` sobre um `Option<i32>`">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

</Listing>

Vamos examinar com mais detalhes a primeira execução de `plus_one`. Quando chamamos `plus_one(five)`, a variável `x` no corpo de `plus_one` terá o valor `Some(5)`. Então, comparamos esse valor com cada braço do `match`:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

O valor `Some(5)` não casa com o padrão `None`, então seguimos para o próximo braço:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```

`Some(5)` casa com `Some(i)`? Casa! Temos a mesma variante. A variável `i` vincula o valor contido em `Some`, então `i` assume o valor `5`. O código do braço do `match` é então executado: somamos 1 ao valor de `i` e criamos um novo valor `Some` com o total `6` dentro.

Agora vamos considerar a segunda chamada de `plus_one` na Listagem 6-5, em que `x` é `None`. Entramos no `match` e comparamos com o primeiro braço:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

Casa! Não há valor ao qual somar, então o programa para e retorna o valor `None` do lado direito do `=>`. Como o primeiro braço casou, nenhum dos outros braços é comparado.

Combinar `match` e enums é útil em muitas situações. Você vai ver esse padrão com frequência no código Rust: fazer `match` sobre um enum, vincular uma variável aos dados que estão dentro dele e então executar código com base nisso. No começo é um pouco complicado, mas, quando você se acostumar, vai desejar tê-lo em todas as linguagens. Ele é consistentemente um dos favoritos dos usuários.

### As expressões `match` são exaustivas {#matches-are-exhaustive}

Há mais um aspecto do `match` que precisamos discutir: os padrões dos braços devem cobrir todas as possibilidades. Considere esta versão da nossa função `plus_one`, que tem um bug e não compila:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```

Não tratamos o caso `None`, então este código vai causar um bug. Felizmente, é um bug que o Rust sabe identificar. Se tentarmos compilar este código, receberemos este erro:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```

O Rust sabe que não cobrimos todos os casos possíveis e até sabe qual padrão esquecemos! Os matches no Rust são _exaustivos_: precisamos esgotar cada última possibilidade para que o código seja válido. Especialmente no caso de `Option<T>`, quando o Rust nos impede de esquecer de tratar explicitamente o caso `None`, ele nos protege de presumir que temos um valor quando, na verdade, poderíamos ter null, o que torna impossível o erro de um bilhão de dólares discutido anteriormente.

### Padrões de captura geral e o marcador `_` {#catch-all-patterns-and-the-_-placeholder}

Usando enums, também podemos tomar ações especiais para alguns valores específicos, mas, para todos os outros valores, executar uma ação padrão. Imagine que estamos implementando um jogo em que, se você tirar 3 no dado, seu jogador não anda, mas ganha um chapéu novo e chamativo. Se tirar 7, seu jogador perde um chapéu chamativo. Para todos os outros valores, seu jogador anda essa quantidade de casas no tabuleiro. Aqui está um `match` que implementa essa lógica, com o resultado do dado fixado no código em vez de ser um valor aleatório, e toda a outra lógica representada por funções sem corpo, porque implementá-las de fato está fora do escopo deste exemplo:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-15-binding-catchall/src/main.rs:here}}
```

Para os dois primeiros braços, os padrões são os valores literais `3` e `7`. Para o último braço, que cobre todos os outros valores possíveis, o padrão é a variável que escolhemos chamar de `other`. O código que roda no braço `other` usa a variável passando-a para a função `move_player`.

Este código compila, mesmo que não tenhamos listado todos os valores possíveis que um `u8` pode ter, porque o último padrão vai casar com todos os valores que não foram listados especificamente. Esse padrão de captura geral atende ao requisito de que o `match` deve ser exaustivo. Repare que precisamos colocar o braço de captura geral por último, porque os padrões são avaliados em ordem. Se colocássemos o braço de captura geral antes, os outros braços nunca seriam executados, então o Rust vai nos avisar se adicionarmos braços depois de um braço de captura geral!

O Rust também tem um padrão que podemos usar quando queremos uma captura geral, mas não queremos _usar_ o valor capturado: `_` é um padrão especial que casa com qualquer valor e não vincula esse valor. Isso diz ao Rust que não vamos usar o valor, então ele não vai nos avisar sobre uma variável não utilizada.

Vamos mudar as regras do jogo: agora, se você tirar qualquer coisa diferente de 3 ou 7, precisa jogar o dado de novo. Não precisamos mais usar o valor da captura geral, então podemos mudar nosso código para usar `_` em vez da variável chamada `other`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-16-underscore-catchall/src/main.rs:here}}
```

Este exemplo também atende ao requisito de exaustividade, porque estamos ignorando explicitamente todos os outros valores no último braço; não esquecemos nada.

Por fim, vamos mudar as regras do jogo mais uma vez, para que nada mais aconteça na sua vez se você tirar qualquer coisa diferente de 3 ou 7. Podemos expressar isso usando o valor unitário (o tipo tupla vazia que mencionamos na seção [“O tipo tupla”][tuples]<!-- ignore -->) como o código que acompanha o braço `_`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-17-underscore-unit/src/main.rs:here}}
```

Aqui, estamos dizendo explicitamente ao Rust que não vamos usar nenhum outro valor que não case com um padrão de um braço anterior, e que não queremos executar nenhum código nesse caso.

Há mais sobre padrões e matching que vamos abordar no [Capítulo
19][ch19-00-patterns]<!-- ignore -->. Por enquanto, vamos seguir para a sintaxe `if let`, que pode ser útil em situações em que a expressão `match` fica um pouco verbosa.

[tuples]: ch03-02-data-types.html#the-tuple-type
[ch19-00-patterns]: ch19-00-patterns.html
