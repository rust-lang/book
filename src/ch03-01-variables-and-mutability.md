<!-- ia-translate: true -->
## Variáveis e mutabilidade {#variables-and-mutability}

Como mencionamos na seção [“Armazenando valores com variáveis”][storing-values-with-variables]<!-- ignore -->, por padrão as variáveis são imutáveis. Esse é um dos muitos incentivos que o Rust dá para que você escreva código de um jeito que aproveite a segurança e a facilidade de concorrência que a linguagem oferece. No entanto, você ainda tem a opção de tornar suas variáveis mutáveis. Vamos explorar como e por que o Rust incentiva você a preferir a imutabilidade, e por que às vezes você pode querer desativá-la.

Quando uma variável é imutável, depois que um valor é vinculado a um nome, você não pode mudar esse valor. Para ilustrar isso, crie um novo projeto chamado _variables_ no seu diretório _projects_ usando `cargo new variables`.

Depois, dentro do novo diretório _variables_, abra _src/main.rs_ e substitua o código por este, que ainda não vai compilar:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/src/main.rs}}
```

Salve e execute o programa usando `cargo run`. Você deve receber uma mensagem de erro sobre imutabilidade, como mostra esta saída:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/output.txt}}
```

Este exemplo mostra como o compilador ajuda você a encontrar erros nos programas. Mensagens de erro do compilador podem ser frustrantes, mas elas só significam que seu programa ainda não faz com segurança o que você quer; elas _não_ significam que você não é um bom programador! Rustaceans experientes também recebem erros do compilador.

Você recebeu a mensagem de erro `` cannot assign twice to immutable variable `x` `` porque tentou atribuir um segundo valor à variável imutável `x`.

É importante que recebamos erros em tempo de compilação quando tentamos alterar um valor que foi designado como imutável, porque justamente essa situação pode levar a bugs. Se uma parte do código opera com a suposição de que um valor nunca vai mudar, e outra parte do código muda esse valor, é possível que a primeira parte não faça o que foi projetada para fazer. A causa desse tipo de bug pode ser difícil de rastrear depois, especialmente quando o segundo trecho de código muda o valor apenas _às vezes_. O compilador do Rust garante que, quando você declara que um valor não vai mudar, ele realmente não vai mudar, então você não precisa acompanhar isso por conta própria. Assim, fica mais fácil raciocinar sobre o seu código.

Mas a mutabilidade pode ser muito útil e tornar o código mais conveniente de escrever. Embora as variáveis sejam imutáveis por padrão, você pode torná-las mutáveis adicionando `mut` na frente do nome da variável, como fez no [Capítulo 2][storing-values-with-variables]<!-- ignore -->. Adicionar `mut` também comunica a intenção a futuros leitores do código, indicando que outras partes do código vão alterar o valor dessa variável.

Por exemplo, vamos alterar _src/main.rs_ para o seguinte:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/src/main.rs}}
```

Quando executamos o programa agora, obtemos isto:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/output.txt}}
```

Podemos alterar o valor vinculado a `x` de `5` para `6` quando `mut` é usado. No fim das contas, decidir se usa ou não mutabilidade depende de você e de qual opção considera mais clara em cada situação.

<!-- Old headings. Do not remove or links may break. -->
<a id="constants"></a>

### Declarando constantes {#declaring-constants}

Assim como as variáveis imutáveis, _constantes_ são valores vinculados a um nome que não têm permissão para mudar, mas há algumas diferenças entre constantes e variáveis.

A primeira: você não pode usar `mut` com constantes. Constantes não são apenas imutáveis por padrão — elas são sempre imutáveis. Você declara constantes usando a palavra-chave `const` em vez de `let`, e o tipo do valor _precisa_ ser anotado. Vamos falar sobre tipos e anotações de tipo na próxima seção, [“Tipos de dados”][data-types]<!-- ignore -->, então não se preocupe com os detalhes agora. Basta saber que você sempre deve anotar o tipo.

Constantes podem ser declaradas em qualquer escopo, inclusive no escopo global, o que as torna úteis para valores que muitas partes do código precisam conhecer.

A última diferença é que constantes só podem ser definidas com uma expressão constante, e não com o resultado de um valor que só pode ser calculado em tempo de execução.

Aqui está um exemplo de declaração de constante:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

O nome da constante é `THREE_HOURS_IN_SECONDS`, e seu valor é o resultado da multiplicação de 60 (o número de segundos em um minuto) por 60 (o número de minutos em uma hora) por 3 (o número de horas que queremos contar neste programa). A convenção de nomes do Rust para constantes é usar tudo em maiúsculas, com sublinhados entre as palavras. O compilador consegue avaliar um conjunto limitado de operações em tempo de compilação, o que nos permite escrever esse valor de um jeito mais fácil de entender e verificar, em vez de definir a constante com o valor 10.800. Veja a [seção sobre avaliação de constantes da Referência do Rust][const-eval] para mais informações sobre quais operações podem ser usadas ao declarar constantes.

Constantes são válidas durante todo o tempo de execução do programa, dentro do escopo em que foram declaradas. Essa propriedade torna as constantes úteis para valores do domínio da sua aplicação que várias partes do programa podem precisar conhecer, como o número máximo de pontos que um jogador de um jogo pode ganhar, ou a velocidade da luz.

Dar nome a valores fixos usados em todo o programa, na forma de constantes, ajuda a transmitir o significado desse valor para futuros mantenedores do código. Também ajuda a ter um único lugar no código onde seria preciso fazer a mudança caso o valor fixo precise ser atualizado no futuro.

### Sombreamento {#shadowing}

Como você viu no tutorial do jogo de adivinhação no [Capítulo 2][comparing-the-guess-to-the-secret-number]<!-- ignore -->, você pode declarar uma nova variável com o mesmo nome de uma variável anterior. Os Rustaceans dizem que a primeira variável é _sombreada_ (_shadowed_) pela segunda, o que significa que a segunda variável é a que o compilador enxerga quando você usa o nome da variável. Na prática, a segunda variável ofusca a primeira, assumindo todos os usos do nome até que ela mesma seja sombreada ou o escopo termine. Podemos sombrear uma variável usando o mesmo nome e repetindo o uso da palavra-chave `let`, assim:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```

Este programa primeiro vincula `x` ao valor `5`. Depois, cria uma nova variável `x` repetindo `let x =`, pegando o valor original e somando `1`, de modo que o valor de `x` seja `6`. Em seguida, dentro de um escopo interno criado com chaves, a terceira instrução `let` também sombreia `x` e cria uma nova variável, multiplicando o valor anterior por `2` para dar a `x` o valor `12`. Quando esse escopo termina, o sombreamento interno acaba e `x` volta a valer `6`. Ao executar este programa, a saída será a seguinte:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```

Sombreamento é diferente de marcar uma variável como `mut`, porque teremos um erro em tempo de compilação se, por acidente, tentarmos reatribuir essa variável sem usar a palavra-chave `let`. Usando `let`, podemos fazer algumas transformações em um valor, mas deixar a variável imutável depois que essas transformações terminam.

A outra diferença entre `mut` e sombreamento é que, como estamos efetivamente criando uma nova variável quando usamos `let` de novo, podemos mudar o tipo do valor e ainda reutilizar o mesmo nome. Por exemplo, suponha que nosso programa peça ao usuário que mostre quantos espaços ele quer entre algum texto, digitando caracteres de espaço, e depois queremos armazenar essa entrada como um número:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```

A primeira variável `spaces` é do tipo string, e a segunda variável `spaces` é do tipo número. Assim, o sombreamento nos poupa de inventar nomes diferentes, como `spaces_str` e `spaces_num`; em vez disso, podemos reutilizar o nome mais simples `spaces`. No entanto, se tentarmos usar `mut` para isso, como mostrado aqui, teremos um erro em tempo de compilação:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```

O erro diz que não podemos mudar o tipo de uma variável:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/output.txt}}
```

Agora que exploramos como as variáveis funcionam, vamos ver mais tipos de dados que elas podem ter.

[comparing-the-guess-to-the-secret-number]: ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[data-types]: ch03-02-data-types.html#data-types
[storing-values-with-variables]: ch02-00-guessing-game-tutorial.html#storing-values-with-variables
[const-eval]: ../reference/const_eval.html
