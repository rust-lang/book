<!-- ia-translate: true -->
## Funções {#functions}

Funções são muito comuns no código Rust. Você já viu uma das funções mais importantes da linguagem: a função `main`, que é o ponto de entrada de muitos programas. Você também já viu a palavra-chave `fn`, que permite declarar novas funções.

O código Rust usa _snake case_ como estilo convencional para nomes de funções e variáveis, em que todas as letras são minúsculas e sublinhados separam as palavras. Aqui está um programa que contém um exemplo de definição de função:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```

Definimos uma função em Rust escrevendo `fn` seguido de um nome de função e um par de parênteses. As chaves indicam ao compilador onde o corpo da função começa e onde termina.

Podemos chamar qualquer função que definimos escrevendo seu nome seguido de um par de parênteses. Como `another_function` está definida no programa, ela pode ser chamada de dentro da função `main`. Observe que definimos `another_function` _depois_ da função `main` no código-fonte; poderíamos tê-la definido antes também. O Rust não se importa onde você define suas funções, só que elas estejam definidas em algum lugar do escopo visível para quem chama.

Vamos começar um novo projeto binário chamado _functions_ para explorar funções com mais detalhes. Coloque o exemplo `another_function` em _src/main.rs_ e execute-o. Você deve ver a seguinte saída:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```

As linhas são executadas na ordem em que aparecem na função `main`. Primeiro é impressa a mensagem "Hello, world!", e depois `another_function` é chamada e a sua mensagem é impressa.

### Parâmetros {#parameters}

Podemos definir funções com _parâmetros_, que são variáveis especiais que fazem parte da assinatura da função. Quando uma função tem parâmetros, você pode fornecer a ela valores concretos para esses parâmetros. Tecnicamente, os valores concretos são chamados de _argumentos_, mas na conversa do dia a dia as pessoas costumam usar as palavras _parâmetro_ e _argumento_ como sinônimos, tanto para as variáveis da definição da função quanto para os valores concretos passados na chamada.

Nesta versão de `another_function`, adicionamos um parâmetro:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```

Experimente executar este programa; você deve obter a seguinte saída:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```

A declaração de `another_function` tem um parâmetro chamado `x`. O tipo de `x` é especificado como `i32`. Quando passamos `5` para `another_function`, a macro `println!` coloca o `5` no lugar onde estava o par de chaves contendo `x` na string de formatação.

Nas assinaturas de funções, você _precisa_ declarar o tipo de cada parâmetro. Essa é uma decisão deliberada no design do Rust: exigir anotações de tipo nas definições de função significa que o compilador quase nunca precisa que você as repita em outros pontos do código para descobrir qual tipo você quer dizer. Além disso, o compilador consegue dar mensagens de erro mais úteis quando sabe quais tipos a função espera.

Ao definir vários parâmetros, separe as declarações com vírgulas, assim:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```

Este exemplo cria uma função chamada `print_labeled_measurement` com dois parâmetros. O primeiro parâmetro se chama `value` e é um `i32`. O segundo se chama `unit_label` e é do tipo `char`. A função então imprime um texto que contém `value` e `unit_label`.

Vamos executar este código. Substitua o programa atual do arquivo _src/main.rs_ do seu projeto _functions_ pelo exemplo anterior e execute-o com `cargo run`:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/output.txt}}
```

Como chamamos a função com `5` como valor de `value` e `'h'` como valor de `unit_label`, a saída do programa contém esses valores.

### Instruções e expressões {#statements-and-expressions}

O corpo de uma função é formado por uma sequência de instruções, opcionalmente terminada por uma expressão. Até agora, as funções que vimos não tinham uma expressão final, mas você já viu uma expressão como parte de uma instrução. Como o Rust é uma linguagem baseada em expressões, essa é uma distinção importante de entender. Outras linguagens não têm as mesmas distinções, então vamos ver o que são instruções e expressões e como as diferenças afetam os corpos das funções.

- _Instruções_ (_statements_) são ações que executam alguma coisa e não retornam um valor.
- _Expressões_ são avaliadas para um valor resultante.

Vamos ver alguns exemplos.

Na verdade, já usamos instruções e expressões. Criar uma variável e atribuir um valor a ela com a palavra-chave `let` é uma instrução. Na Listagem 3-1, `let y = 6;` é uma instrução.

<Listing number="3-1" file-name="src/main.rs" caption="Uma declaração de função `main` contendo uma instrução">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```

</Listing>

Definições de função também são instruções; todo o exemplo anterior é uma instrução por si só. (Como veremos em breve, chamar uma função não é uma instrução, no entanto.)

Instruções não retornam valores. Por isso, você não pode atribuir uma instrução `let` a outra variável, como o código a seguir tenta fazer; você receberá um erro:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```

Ao executar este programa, o erro que você verá é este:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/output.txt}}
```

A instrução `let y = 6` não retorna um valor, então não há nada para `x` se vincular. Isso é diferente do que acontece em outras linguagens, como C e Ruby, nas quais a atribuição retorna o valor atribuído. Nessas linguagens, você pode escrever `x = y = 6` e fazer com que tanto `x` quanto `y` tenham o valor `6`; isso não funciona em Rust.

Expressões são avaliadas para um valor e formam a maior parte do restante do código que você vai escrever em Rust. Considere uma operação matemática, como `5 + 6`, que é uma expressão avaliada para o valor `11`. Expressões podem fazer parte de instruções: na Listagem 3-1, o `6` da instrução `let y = 6;` é uma expressão avaliada para o valor `6`. Chamar uma função é uma expressão. Chamar uma macro é uma expressão. Um novo bloco de escopo criado com chaves também é uma expressão, por exemplo:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-20-blocks-are-expressions/src/main.rs}}
```

Esta expressão:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

é um bloco que, neste caso, é avaliado para `4`. Esse valor é vinculado a `y` como parte da instrução `let`. Observe a linha `x + 1` sem ponto e vírgula no final, o que é diferente da maioria das linhas que você viu até agora. Expressões não incluem ponto e vírgula no final. Se você adicionar um ponto e vírgula ao final de uma expressão, ela se torna uma instrução e, então, deixa de retornar um valor. Tenha isso em mente enquanto exploramos a seguir os valores de retorno de funções e expressões.

### Funções com valores de retorno {#functions-with-return-values}

Funções podem retornar valores para o código que as chama. Não damos nome aos valores de retorno, mas devemos declarar o tipo deles depois de uma seta (`->`). Em Rust, o valor de retorno da função é sinônimo do valor da última expressão no bloco do corpo da função. Você pode retornar antecipadamente de uma função usando a palavra-chave `return` e especificando um valor, mas a maioria das funções retorna a última expressão implicitamente. Aqui está um exemplo de uma função que retorna um valor:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```

Não há chamadas de função, macros, nem mesmo instruções `let` na função `five`: apenas o número `5` por si só. Essa é uma função perfeitamente válida em Rust. Observe que o tipo de retorno da função também é especificado, como `-> i32`. Execute este código; a saída deve ser assim:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/output.txt}}
```

O `5` em `five` é o valor de retorno da função, e é por isso que o tipo de retorno é `i32`. Vamos examinar isso com mais detalhes. Há dois pontos importantes: primeiro, a linha `let x = five();` mostra que estamos usando o valor de retorno de uma função para inicializar uma variável. Como a função `five` retorna um `5`, essa linha é igual à seguinte:

```rust
let x = 5;
```

Segundo, a função `five` não tem parâmetros e define o tipo do valor de retorno, mas o corpo da função é um `5` sozinho, sem ponto e vírgula, porque é uma expressão cujo valor queremos retornar.

Vejamos outro exemplo:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```

Executar este código vai imprimir `The value of x is: 6`. Mas o que acontece se colocarmos um ponto e vírgula no fim da linha que contém `x + 1`, mudando-a de expressão para instrução?

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

Compilar este código vai produzir um erro, assim:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/output.txt}}
```

A mensagem de erro principal, `mismatched types`, revela o problema central deste código. A definição da função `plus_one` diz que ela retornará um `i32`, mas instruções não são avaliadas para um valor, o que é expresso por `()`, o tipo unitário. Portanto, nada é retornado, o que contradiz a definição da função e resulta em erro. Nessa saída, o Rust fornece uma mensagem que pode ajudar a corrigir o problema: ele sugere remover o ponto e vírgula, o que resolveria o erro.
