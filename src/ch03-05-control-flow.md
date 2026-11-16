<!-- ia-translate: true -->

## Controle de fluxo {#control-flow}

A capacidade de executar um trecho de código dependendo de uma condição ser
`true` e a capacidade de executar um trecho de código repetidamente enquanto uma
condição for `true` são blocos básicos de construção na maioria das linguagens
de programação. As construções mais comuns que permitem controlar o fluxo de
execução do código Rust são as expressões `if` e os loops.

### Expressões `if` {#if-expressions}

Uma expressão `if` permite que você ramifique seu código dependendo de condições.
Você fornece uma condição e, então, afirma: "Se esta condição for atendida,
execute este bloco de código. Se a condição não for atendida, não execute este
bloco de código."

Crie um novo projeto chamado _branches_ na sua pasta _projects_ para explorar a
expressão `if`. No arquivo _src/main.rs_, digite o seguinte:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

Todas as expressões `if` começam com a palavra-chave `if`, seguida de uma
condição. Neste caso, a condição verifica se a variável `number` tem um valor
menor que 5. Colocamos o bloco de código a ser executado se a condição for `true`
logo após a condição, entre chaves. Os blocos de código associados às condições
das expressões `if` às vezes são chamados de _braços_, assim como os braços das
expressões `match` que discutimos na seção [“Comparando o palpite com o número
secreto”][comparing-the-guess-to-the-secret-number]<!-- ignore --> do Capítulo 2.

Opcionalmente, também podemos incluir uma expressão `else`, como fizemos aqui,
para dar ao programa um bloco de código alternativo a ser executado caso a
condição seja avaliada como `false`. Se você não fornecer uma expressão `else` e
a condição for `false`, o programa simplesmente pula o bloco `if` e segue para o
próximo trecho de código.

Experimente executar este código; você deve ver a seguinte saída:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```

Vamos tentar mudar o valor de `number` para um valor que torne a condição
`false`, para ver o que acontece:

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```

Execute o programa novamente e observe a saída:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```

Vale notar também que a condição neste código _precisa_ ser um `bool`. Se a
condição não for um `bool`, vamos ter um erro. Por exemplo, experimente executar
o código a seguir:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```

Desta vez, a condição do `if` é avaliada como o valor `3`, e o Rust gera um
erro:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```

O erro indica que o Rust esperava um `bool`, mas recebeu um inteiro. Diferentemente
de linguagens como Ruby e JavaScript, o Rust não tenta converter automaticamente
tipos que não são booleanos para booleano. Você precisa ser explícito e sempre
fornecer um booleano como condição do `if`. Se quisermos que o bloco `if` seja
executado somente quando um número for diferente de `0`, por exemplo, podemos
alterar a expressão `if` para a seguinte:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```

Executar este código vai imprimir `number was something other than zero`.

#### Tratando múltiplas condições com `else if` {#handling-multiple-conditions-with-else-if}

Você pode usar múltiplas condições combinando `if` e `else` em uma expressão
`else if`. Por exemplo:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

Este programa tem quatro caminhos possíveis a seguir. Depois de executá-lo, você
deve ver a seguinte saída:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```

Quando este programa é executado, ele verifica cada expressão `if` em sequência e
executa o primeiro bloco cuja condição é avaliada como `true`. Repare que, embora
6 seja divisível por 2, não vemos a saída `number is divisible by 2`, nem o texto
`number is not divisible by 4, 3, or 2` do bloco `else`. Isso acontece porque o
Rust só executa o bloco da primeira condição `true` e, assim que encontra uma,
nem chega a verificar as demais.

Usar muitas expressões `else if` pode deixar seu código poluído, então, se você tiver
mais de uma, talvez queira refatorar o código. O Capítulo 6 descreve uma
construção de ramificação poderosa do Rust chamada `match`, feita para esses casos.

#### Usando `if` em uma instrução `let` {#using-if-in-a-let-statement}

Como `if` é uma expressão, podemos usá-lo do lado direito de uma instrução `let`
para atribuir o resultado a uma variável, como na Listagem 3-2.

<Listing number="3-2" file-name="src/main.rs" caption="Atribuindo o resultado de uma expressão `if` a uma variável">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

</Listing>

A variável `number` vai ser vinculada a um valor com base no resultado da
expressão `if`. Execute este código para ver o que acontece:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-02/output.txt}}
```

Lembre-se de que blocos de código são avaliados para a última expressão que
contêm, e que números sozinhos também são expressões. Neste caso, o valor da
expressão `if` inteira depende de qual bloco de código é executado. Isso significa
que os valores que podem resultar de cada braço do `if` precisam ser do mesmo
tipo; na Listagem 3-2, os resultados tanto do braço `if` quanto do braço `else`
eram inteiros `i32`. Se os tipos forem incompatíveis, como no exemplo a seguir,
teremos um erro:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```

Quando tentamos compilar este código, recebemos um erro. Os braços `if` e `else`
têm tipos de valor incompatíveis, e o Rust indica exatamente onde está o problema
no programa:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```

A expressão no bloco `if` é avaliada como um inteiro, e a expressão no bloco
`else` é avaliada como uma string. Isso não vai funcionar, porque variáveis precisam
ter um único tipo, e o Rust precisa saber com certeza, em tempo de compilação,
qual é o tipo da variável `number`. Conhecer o tipo de `number` permite que o
compilador verifique se o tipo é válido em todos os lugares onde usamos `number`.
O Rust não conseguiria fazer isso se o tipo de `number` fosse determinado somente
em tempo de execução; o compilador ficaria mais complexo e ofereceria menos
garantias sobre o código se precisasse acompanhar múltiplos tipos hipotéticos
para qualquer variável.

### Repetição com loops {#repetition-with-loops}

Com frequência, é útil executar um bloco de código mais de uma vez. Para essa
tarefa, o Rust oferece vários _loops_, que percorrem o código dentro do corpo do
loop até o fim e, então, recomeçam imediatamente do início. Para experimentar
com loops, vamos criar um novo projeto chamado _loops_.

O Rust tem três tipos de loops: `loop`, `while` e `for`. Vamos testar cada um.

#### Repetindo código com `loop` {#repeating-code-with-loop}

A palavra-chave `loop` diz ao Rust para executar um bloco de código várias vezes
seguidas, para sempre ou até que você mande explicitamente parar.

Como exemplo, altere o arquivo _src/main.rs_ no seu diretório _loops_ para ficar
assim:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

Quando executarmos este programa, veremos `again!` impresso repetidamente, sem
parar, até que interrompamos o programa manualmente. A maioria dos terminais
oferece o atalho de teclado <kbd>ctrl</kbd>-<kbd>C</kbd> para interromper um
programa preso em um loop contínuo. Experimente:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-32-loop
cargo run
CTRL-C
-->

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

O símbolo `^C` representa o ponto em que você pressionou <kbd>ctrl</kbd>-<kbd>C</kbd>.

Você pode ou não ver a palavra `again!` impressa depois do `^C`, dependendo de
onde o código estava no loop quando recebeu o sinal de interrupção.

Felizmente, o Rust também oferece uma forma de sair de um loop por meio do código.
Você pode colocar a palavra-chave `break` dentro do loop para dizer ao programa
quando parar de executar o loop. Lembre-se de que fizemos isso no jogo de adivinhação,
na seção [“Saindo após um palpite correto”][quitting-after-a-correct-guess]<!-- ignore
--> do Capítulo 2, para encerrar o programa quando o usuário ganhou o jogo ao
adivinhar o número correto.

Também usamos `continue` no jogo de adivinhação, que, em um loop, diz ao programa
para pular o restante do código nesta iteração do loop e ir para a próxima iteração.

#### Retornando valores de loops {#returning-values-from-loops}

Um dos usos de um `loop` é tentar novamente uma operação que você sabe que pode
falhar, como verificar se uma thread terminou seu trabalho. Você também pode
precisar passar o resultado dessa operação para fora do loop, para o restante do
seu código. Para isso, você pode adicionar o valor que deseja retornar depois da
expressão `break` que usa para parar o loop; esse valor será retornado para fora
do loop, para que você possa usá-lo, como mostrado aqui:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

Antes do loop, declaramos uma variável chamada `counter` e a inicializamos com
`0`. Em seguida, declaramos uma variável chamada `result` para guardar o valor
retornado pelo loop. A cada iteração do loop, somamos `1` à variável `counter` e
depois verificamos se `counter` é igual a `10`. Quando for, usamos a palavra-chave
`break` com o valor `counter * 2`. Depois do loop, usamos um ponto e vírgula para
encerrar a instrução que atribui o valor a `result`. Por fim, imprimimos o valor de
`result`, que neste caso é `20`.

Você também pode usar `return` dentro de um loop. Enquanto `break` sai apenas do
loop atual, `return` sempre sai da função atual.

<!-- Old headings. Do not remove or links may break. -->
<a id="loop-labels-to-disambiguate-between-multiple-loops"></a>

#### Desambiguando com rótulos de loop {#disambiguating-with-loop-labels}

Se você tiver loops dentro de loops, `break` e `continue` se aplicam ao loop mais
interno naquele ponto. Opcionalmente, você pode especificar um _rótulo de loop_
em um loop, que depois pode ser usado com `break` ou `continue` para indicar que
essas palavras-chave se aplicam ao loop rotulado em vez do loop mais interno.
Rótulos de loop devem começar com uma aspa simples. Aqui está um exemplo com dois
loops aninhados:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/src/main.rs}}
```

O loop externo tem o rótulo `'counting_up` e vai contar de 0 até 2. O loop interno,
sem rótulo, conta de 10 até 9. O primeiro `break` que não especifica um rótulo vai
sair apenas do loop interno. A instrução `break
'counting_up;` vai sair do loop externo. Este código imprime:

```console
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/output.txt}}
```

<!-- Old headings. Do not remove or links may break. -->
<a id="conditional-loops-with-while"></a>

#### Simplificando loops condicionais com `while` {#streamlining-conditional-loops-with-while}

Um programa frequentemente precisa avaliar uma condição dentro de um loop. Enquanto
a condição for `true`, o loop roda. Quando a condição deixa de ser `true`, o
programa chama `break`, parando o loop. É possível implementar um comportamento
desses combinando `loop`, `if`, `else` e `break`; se quiser, você pode tentar
isso agora em um programa. Porém, esse padrão é tão comum que o Rust tem uma
construção de linguagem pronta para ele, chamada loop `while`. No Listagem 3-3,
usamos `while` para fazer o programa dar três voltas, contando regressivamente
a cada vez, e, depois do loop, imprimir uma mensagem e encerrar.

<Listing number="3-3" file-name="src/main.rs" caption="Usando um loop `while` para executar código enquanto uma condição é avaliada como `true`">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

</Listing>

Essa construção elimina muito do aninhamento que seria necessário se você usasse
`loop`, `if`, `else` e `break`, e ela é mais clara. Enquanto uma condição for
avaliada como `true`, o código roda; caso contrário, o programa sai do loop.

#### Percorrendo uma coleção com `for` {#looping-through-a-collection-with-for}

Você pode optar por usar a construção `while` para percorrer os elementos de uma
coleção, como um array. Por exemplo, o loop na Listagem 3-4 imprime cada elemento
do array `a`.

<Listing number="3-4" file-name="src/main.rs" caption="Percorrendo cada elemento de uma coleção usando um loop `while`">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

</Listing>

Aqui, o código percorre os elementos do array contando para cima. Ele começa no
índice `0` e então repete até alcançar o último índice do array (isto é, quando
`index < 5` não for mais `true`). Executar este código vai imprimir cada elemento
do array:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

Todos os cinco valores do array aparecem no terminal, como esperado. Mesmo que
`index` chegue a um valor de `5` em algum momento, o loop para de executar antes de
tentar buscar um sexto valor no array.

No entanto, essa abordagem é propensa a erros; podemos fazer o programa entrar em
pânico se o valor do índice ou a condição de teste estiverem incorretos. Por exemplo,
se você mudasse a definição do array `a` para ter quatro elementos, mas esquecesse
de atualizar a condição para `while index < 4`, o código entraria em pânico. Também
é lento, porque o compilador adiciona código em tempo de execução para fazer, a cada
iteração do loop, a verificação condicional de se o índice está dentro dos limites
do array.

Como alternativa mais concisa, você pode usar um loop `for` e executar algum código
para cada item de uma coleção. Um loop `for` se parece com o código da Listagem 3-5.

<Listing number="3-5" file-name="src/main.rs" caption="Percorrendo cada elemento de uma coleção usando um loop `for`">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

</Listing>

Quando executamos este código, vemos a mesma saída da Listagem 3-4. Mais importante
ainda, aumentamos a segurança do código e eliminamos a chance de bugs que poderiam
surgir por ir além do fim do array ou por não ir longe o suficiente e deixar de
fora alguns itens. O código de máquina gerado por loops `for` também pode ser mais
eficiente, porque o índice não precisa ser comparado com o tamanho do array a cada
iteração.

Usando o loop `for`, você não precisaria se lembrar de alterar nenhum outro código
se mudasse a quantidade de valores no array, como seria necessário com o método
usado na Listagem 3-4.

A segurança e a concisão dos loops `for` fazem deles a construção de loop mais usada
em Rust. Mesmo em situações em que você quer executar algum código um certo número
de vezes, como no exemplo de contagem regressiva que usou um loop `while` no
Listagem 3-3, a maioria dos Rustaceans usaria um loop `for`. A forma de fazer isso
seria usar um `Range`, fornecido pela biblioteca padrão, que gera todos os números
em sequência, começando em um número e terminando antes de outro.

Veja como ficaria a contagem regressiva usando um loop `for` e outro método que
ainda não discutimos, `rev`, para inverter o intervalo:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

Este código ficou um pouco mais bonito, não ficou?

## Resumo {#summary}

Você chegou ao fim! Foi um capítulo extenso: você aprendeu sobre variáveis, tipos
de dados escalares e compostos, funções, comentários, expressões `if` e loops! Para
praticar os conceitos discutidos neste capítulo, tente criar programas que façam o
seguinte:

- Converter temperaturas entre Fahrenheit e Celsius.
- Gerar o *n*-ésimo número de Fibonacci.
- Imprimir a letra da canção de Natal “The Twelve Days of Christmas”,
  aproveitando a repetição da música.

Quando você estiver pronto para seguir em frente, vamos falar sobre um conceito
do Rust que normalmente _não_ existe em outras linguagens de programação: ownership.

[comparing-the-guess-to-the-secret-number]: ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[quitting-after-a-correct-guess]: ch02-00-guessing-game-tutorial.html#quitting-after-a-correct-guess
