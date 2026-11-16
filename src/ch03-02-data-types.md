<!-- ia-translate: true -->

## Tipos de dados {#data-types}

Todo valor em Rust é de um certo _tipo de dado_, que diz ao Rust que tipo de
dado está sendo especificado para que ele saiba como trabalhar com esse dado.
Vamos ver dois subconjuntos de tipos de dados: escalares e compostos.

Tenha em mente que Rust é uma linguagem de _tipagem estática_, o que significa
que ele precisa conhecer os tipos de todas as variáveis em tempo de compilação.
O compilador geralmente consegue inferir o tipo que queremos usar com base no
valor e em como o usamos. Em casos em que muitos tipos são possíveis, como
quando convertemos uma `String` para um tipo numérico usando `parse` na seção
[“Comparando o palpite com o número secreto”][comparing-the-guess-to-the-secret-number]<!-- ignore -->
do Capítulo 2, precisamos adicionar uma anotação de tipo, assim:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

Se não adicionarmos a anotação de tipo `: u32` mostrada no código anterior, o
Rust vai exibir o seguinte erro, que significa que o compilador precisa de mais
informações nossas para saber qual tipo queremos usar:

```console
{{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
```

Você vai ver anotações de tipo diferentes para outros tipos de dados.

### Tipos escalares {#scalar-types}

Um tipo _escalar_ representa um único valor. Rust tem quatro tipos escalares
principais: inteiros, números de ponto flutuante, booleanos e caracteres. Você
deve reconhecê-los de outras linguagens de programação. Vamos ver como eles
funcionam em Rust.

#### Tipos inteiros {#integer-types}

Um _inteiro_ é um número sem componente fracionário. Usamos um tipo inteiro no
Capítulo 2, o tipo `u32`. Essa declaração de tipo indica que o valor associado
a ele deve ser um inteiro sem sinal (tipos inteiros com sinal começam com `i` em
vez de `u`) que ocupa 32 bits de espaço. A Tabela 3-1 mostra os tipos inteiros
embutidos em Rust. Podemos usar qualquer uma dessas variantes para declarar o
tipo de um valor inteiro.

<span class="caption">Tabela 3-1: Tipos inteiros em Rust</span>

| Tamanho                | Com sinal | Sem sinal |
| ---------------------- | --------- | --------- |
| 8 bits                 | `i8`      | `u8`      |
| 16 bits                | `i16`     | `u16`     |
| 32 bits                | `i32`     | `u32`     |
| 64 bits                | `i64`     | `u64`     |
| 128 bits               | `i128`    | `u128`    |
| Depende da arquitetura | `isize`   | `usize`   |

Cada variante pode ser com sinal ou sem sinal e tem um tamanho explícito.
_Com sinal_ e _sem sinal_ indicam se o número pode ser negativo — em
outras palavras, se o número precisa ter um sinal junto (com sinal) ou se ele
será sempre positivo e pode, portanto, ser representado sem sinal (sem sinal).
É como escrever números no papel: quando o sinal importa, um número é mostrado
com um sinal de mais ou de menos; já quando é seguro presumir que o número é
positivo, ele é mostrado sem sinal. Números com sinal são armazenados usando a
representação em [complemento de dois][twos-complement]<!-- ignore -->.

Cada variante com sinal pode armazenar números de −(2<sup>n − 1</sup>) até
2<sup>n − 1</sup> − 1, inclusive, onde _n_ é o número de bits que a variante
usa. Assim, um `i8` pode armazenar números de −(2<sup>7</sup>) até
2<sup>7</sup> − 1, que dá −128 a 127. Variantes sem sinal podem armazenar
números de 0 até 2<sup>n</sup> − 1, então um `u8` pode armazenar números de 0
até 2<sup>8</sup> − 1, que dá 0 a 255.

Além disso, os tipos `isize` e `usize` dependem da arquitetura do computador em
que seu programa está rodando: 64 bits se você estiver em uma arquitetura de 64
bits e 32 bits se estiver em uma de 32 bits.

Você pode escrever literais inteiros em qualquer uma das formas mostradas na
Tabela 3-2. Observe que literais numéricos que podem ser de vários tipos
numéricos aceitam um sufixo de tipo, como `57u8`, para indicar o tipo. Literais
numéricos também podem usar `_` como separador visual para facilitar a leitura,
como `1_000`, que terá o mesmo valor que se você tivesse escrito `1000`.

<span class="caption">Tabela 3-2: Literais inteiros em Rust</span>

| Literais numéricos | Exemplo       |
| ------------------ | ------------- |
| Decimal            | `98_222`      |
| Hexadecimal        | `0xff`        |
| Octal              | `0o77`        |
| Binário            | `0b1111_0000` |
| Byte (somente `u8`) | `b'A'`       |

Então, como saber qual tipo de inteiro usar? Se você não tiver certeza, os
padrões do Rust geralmente são bons pontos de partida: tipos inteiros usam `i32`
por padrão. A principal situação em que você usaria `isize` ou `usize` é ao
indexar algum tipo de coleção.

> ##### Estouro de inteiro
>
> Suponha que você tenha uma variável do tipo `u8`, que pode guardar valores
> entre 0 e 255. Se você tentar mudar a variável para um valor fora dessa faixa,
> como 256, ocorrerá um _estouro de inteiro_ (_integer overflow_), que pode
> resultar em um de dois comportamentos. Quando você compila em modo de
> depuração, o Rust inclui verificações de estouro de inteiro que fazem seu
> programa entrar em _panic_ em tempo de execução se esse comportamento
> acontecer. O Rust usa o termo _panicking_ quando um programa termina com um
> erro; vamos discutir panics com mais profundidade na seção [“Erros
> irrecuperáveis com `panic!`”][unrecoverable-errors-with-panic]<!-- ignore -->
> do Capítulo 9.
>
> Quando você compila em modo de release com a flag `--release`, o Rust _não_
> inclui verificações de estouro de inteiro que causem panics. Em vez disso, se
> ocorrer estouro, o Rust realiza _wrapping em complemento de dois_. Em resumo,
> valores maiores que o valor máximo que o tipo pode guardar “dão a volta” até o
> valor mínimo que o tipo pode guardar. No caso de um `u8`, o valor 256 vira 0,
> o valor 257 vira 1, e assim por diante. O programa não entra em panic, mas a
> variável terá um valor que provavelmente não é o que você esperava. Depender
> do comportamento de wrapping do estouro de inteiro é considerado um erro.
>
> Para tratar explicitamente a possibilidade de estouro, você pode usar estas
> famílias de métodos fornecidas pela biblioteca padrão para tipos numéricos
> primitivos:
>
> - Dar a volta (wrapping) em todos os modos de compilação com os métodos
>   `wrapping_*`, como `wrapping_add`.
> - Retornar o valor `None` se houver estouro com os métodos `checked_*`.
> - Retornar o valor e um booleano indicando se houve estouro com os métodos
>   `overflowing_*`.
> - Saturar nos valores mínimo ou máximo do tipo com os métodos `saturating_*`.

#### Tipos de ponto flutuante {#floating-point-types}

Rust também tem dois tipos primitivos para _números de ponto flutuante_, que são
números com casas decimais. Os tipos de ponto flutuante do Rust são `f32` e
`f64`, que têm 32 bits e 64 bits de tamanho, respectivamente. O tipo padrão é
`f64` porque, em CPUs modernas, ele tem praticamente a mesma velocidade de `f32`,
mas é capaz de mais precisão. Todos os tipos de ponto flutuante são com sinal.

Aqui está um exemplo que mostra números de ponto flutuante em ação:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

Números de ponto flutuante são representados de acordo com o padrão IEEE-754.

#### Operações numéricas {#numeric-operations}

Rust oferece suporte às operações matemáticas básicas que você espera para todos
os tipos numéricos: adição, subtração, multiplicação, divisão e resto. A divisão
de inteiros trunca em direção ao zero, até o inteiro mais próximo. O código a
seguir mostra como você usaria cada operação numérica em uma instrução `let`:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

Cada expressão nessas instruções usa um operador matemático e é avaliada para um
único valor, que então é associado a uma variável. O [Apêndice B][appendix_b]<!-- ignore -->
contém uma lista de todos os operadores que Rust oferece.

#### O tipo booleano {#the-boolean-type}

Como na maioria das outras linguagens de programação, um tipo booleano em Rust
tem dois valores possíveis: `true` e `false`. Booleanos têm um byte de tamanho. O
tipo booleano em Rust é especificado usando `bool`. Por exemplo:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

A principal forma de usar valores booleanos é por meio de condicionais, como uma
expressão `if`. Vamos ver como expressões `if` funcionam em Rust na seção
[“Controle de fluxo”][control-flow]<!-- ignore -->.

#### O tipo caractere {#the-character-type}

O tipo `char` do Rust é o tipo alfabético mais primitivo da linguagem. Aqui
estão alguns exemplos de declaração de valores `char`:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

Observe que especificamos literais `char` com aspas simples, diferentemente de
literais de string, que usam aspas duplas. O tipo `char` do Rust tem 4 bytes de
tamanho e representa um valor escalar Unicode, o que significa que ele pode
representar muito mais do que apenas ASCII. Letras acentuadas; caracteres
chineses, japoneses e coreanos; emojis; e espaços de largura zero são todos
valores `char` válidos em Rust. Valores escalares Unicode vão de `U+0000` até
`U+D7FF` e de `U+E000` até `U+10FFFF`, inclusive. No entanto, “caractere” não é
realmente um conceito no Unicode, então a sua intuição sobre o que é um
“caractere” pode não bater com o que é um `char` em Rust. Vamos discutir esse
tema em detalhes na seção [“Armazenando texto codificado em UTF-8 com
strings”][strings]<!-- ignore --> do Capítulo 8.

### Tipos compostos {#compound-types}

_Tipos compostos_ podem agrupar vários valores em um só tipo. Rust tem dois tipos
compostos primitivos: tuplas e arrays.

#### O tipo tupla {#the-tuple-type}

Uma _tupla_ é uma forma geral de agrupar vários valores de tipos variados em um
único tipo composto. Tuplas têm tamanho fixo: depois de declaradas, não podem
crescer nem diminuir.

Criamos uma tupla escrevendo uma lista de valores separados por vírgula entre
parênteses. Cada posição na tupla tem um tipo, e os tipos dos diferentes valores
na tupla não precisam ser iguais. Adicionamos anotações de tipo opcionais neste
exemplo:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

A variável `tup` está associada à tupla inteira, porque uma tupla é considerada
um único elemento composto. Para obter os valores individuais de uma tupla,
podemos usar _pattern matching_ para desestruturar o valor da tupla, assim:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

Este programa primeiro cria uma tupla e a associa à variável `tup`. Depois, usa
um padrão com `let` para pegar `tup` e transformá-la em três variáveis separadas,
`x`, `y` e `z`. Isso é chamado de _desestruturação_ porque quebra a tupla única
em três partes. Por fim, o programa imprime o valor de `y`, que é `6.4`.

Também podemos acessar um elemento de tupla diretamente usando um ponto (`.`)
seguido do índice do valor que queremos acessar. Por exemplo:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

Este programa cria a tupla `x` e depois acessa cada elemento da tupla usando
seus respectivos índices. Como na maioria das linguagens de programação, o
primeiro índice em uma tupla é 0.

A tupla sem nenhum valor tem um nome especial, _unit_ (unidade). Esse valor e o
seu tipo correspondente são escritos ambos como `()` e representam um valor vazio
ou um tipo de retorno vazio. Expressões retornam implicitamente o valor unit se
não retornarem nenhum outro valor.

#### O tipo array {#the-array-type}

Outra forma de ter uma coleção de vários valores é com um _array_. Diferentemente
de uma tupla, todo elemento de um array precisa ter o mesmo tipo. Diferentemente
de arrays em algumas outras linguagens, arrays em Rust têm tamanho fixo.

Escrevemos os valores de um array como uma lista separada por vírgulas entre
colchetes:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

Arrays são úteis quando você quer que seus dados sejam alocados na stack, como os
outros tipos que vimos até aqui, em vez de na heap (vamos discutir mais sobre
stack e heap no [Capítulo 4][stack-and-heap]<!-- ignore -->), ou quando você quer
garantir que sempre terá um número fixo de elementos. Um array, porém, não é tão
flexível quanto o tipo vetor. Um vetor é um tipo de coleção semelhante, fornecido
pela biblioteca padrão, que _pode_ crescer ou diminuir de tamanho porque seu
conteúdo fica na heap. Se você não tiver certeza se deve usar um array ou um
vetor, as chances são de que você deva usar um vetor. O [Capítulo 8][vectors]<!-- ignore -->
discute vetores em mais detalhes.

No entanto, arrays são mais úteis quando você sabe que o número de elementos não
precisará mudar. Por exemplo, se você estivesse usando os nomes dos meses em um
programa, provavelmente usaria um array em vez de um vetor, porque sabe que ele
sempre terá 12 elementos:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

Você escreve o tipo de um array usando colchetes com o tipo de cada elemento, um
ponto e vírgula e depois o número de elementos do array, assim:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Aqui, `i32` é o tipo de cada elemento. Depois do ponto e vírgula, o número `5`
indica que o array contém cinco elementos.

Você também pode inicializar um array para conter o mesmo valor em cada elemento,
especificando o valor inicial, seguido de um ponto e vírgula e, depois, o
tamanho do array entre colchetes, como mostrado aqui:

```rust
let a = [3; 5];
```

O array chamado `a` vai conter `5` elementos, todos inicialmente definidos com o
valor `3`. Isso é o mesmo que escrever `let a = [3, 3, 3, 3, 3];`, mas de forma
mais concisa.

<!-- Old headings. Do not remove or links may break. -->
<a id="accessing-array-elements"></a>

#### Acesso a elementos de array {#array-element-access}

Um array é um único bloco de memória de tamanho conhecido e fixo que pode ser
alocado na stack. Você pode acessar elementos de um array usando indexação, assim:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

Neste exemplo, a variável chamada `first` vai receber o valor `1`, porque esse é
o valor no índice `[0]` do array. A variável chamada `second` vai receber o valor
`2` do índice `[1]` do array.

#### Acesso inválido a elementos de array {#invalid-array-element-access}

Vamos ver o que acontece se você tentar acessar um elemento de array além do fim
dele. Suponha que você rode este código, parecido com o jogo de adivinhação do
Capítulo 2, para obter um índice de array do usuário:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

Este código compila com sucesso. Se você rodar este código com `cargo run` e
digitar `0`, `1`, `2`, `3` ou `4`, o programa vai imprimir o valor correspondente
naquele índice do array. Se, em vez disso, você digitar um número além do fim do
array, como `10`, verá uma saída como esta:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access
cargo run
10
-->

```console
thread 'main' panicked at src/main.rs:19:19:
index out of bounds: the len is 5 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

O programa resultou em um erro de execução no ponto em que um valor inválido foi
usado na operação de indexação. O programa terminou com uma mensagem de erro e
não executou a instrução `println!` final. Quando você tenta acessar um elemento
usando indexação, o Rust verifica se o índice especificado é menor que o tamanho
do array. Se o índice for maior ou igual ao tamanho, o Rust entra em panic. Essa
verificação precisa acontecer em tempo de execução, especialmente neste caso,
porque o compilador não tem como saber qual valor um usuário vai digitar quando
rodar o código depois.

Este é um exemplo dos princípios de segurança de memória do Rust em ação. Em
muitas linguagens de baixo nível, esse tipo de verificação não é feita e, ao
fornecer um índice incorreto, memória inválida pode ser acessada. O Rust protege
você contra esse tipo de erro encerrando imediatamente, em vez de permitir o
acesso à memória e continuar. O Capítulo 9 discute mais sobre o tratamento de
erros do Rust e como você pode escrever código legível e seguro que não entra em
panic nem permite acesso a memória inválida.

[comparing-the-guess-to-the-secret-number]: ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[twos-complement]: https://en.wikipedia.org/wiki/Two%27s_complement
[control-flow]: ch03-05-control-flow.html#control-flow
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[stack-and-heap]: ch04-01-what-is-ownership.html#the-stack-and-the-heap
[vectors]: ch08-01-vectors.html
[unrecoverable-errors-with-panic]: ch09-01-unrecoverable-errors-with-panic.html
[appendix_b]: appendix-02-operators.md
