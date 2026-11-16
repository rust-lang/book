<!-- ia-translate: true -->
# Programando um Jogo de Adivinhação {#programming-a-guessing-game}

Vamos mergulhar no Rust trabalhando juntos em um projeto prático! Este capítulo
apresenta alguns conceitos comuns do Rust, mostrando como usá-los em um programa
real. Você vai aprender sobre `let`, `match`, métodos, funções associadas,
crates externos e muito mais! Nos capítulos seguintes, vamos explorar essas
ideias com mais detalhes. Neste capítulo, você vai apenas praticar os
fundamentos.

Vamos implementar um problema clássico de programação para iniciantes: um jogo de
adivinhação. Veja como ele funciona: o programa vai gerar um número inteiro
aleatório entre 1 e 100. Depois, vai pedir que o jogador digite um palpite. Após
um palpite ser digitado, o programa vai indicar se o palpite é baixo ou alto
demais. Se o palpite estiver certo, o jogo vai exibir uma mensagem de
parabéns e encerrar.

## Criando um Novo Projeto {#setting-up-a-new-project}

Para criar um novo projeto, vá até o diretório _projects_ que você criou no
Capítulo 1 e crie um novo projeto usando o Cargo, assim:

```console
$ cargo new guessing_game
$ cd guessing_game
```

O primeiro comando, `cargo new`, recebe o nome do projeto (`guessing_game`)
como primeiro argumento. O segundo comando muda para o diretório do novo
projeto.

Veja o arquivo _Cargo.toml_ gerado:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial
rm -rf no-listing-01-cargo-new
cargo new no-listing-01-cargo-new --name guessing_game
cd no-listing-01-cargo-new
cargo run > output.txt 2>&1
cd ../../..
-->

<span class="filename">Nome do arquivo: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```

Como você viu no Capítulo 1, o `cargo new` gera um programa “Hello, world!” para
você. Dê uma olhada no arquivo _src/main.rs_:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

Agora vamos compilar este programa “Hello, world!” e executá-lo na mesma etapa,
usando o comando `cargo run`:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```

O comando `run` é útil quando você precisa iterar rapidamente em um projeto, como
vamos fazer neste jogo, testando rapidamente cada iteração antes de passar para a
próxima.

Reabra o arquivo _src/main.rs_. Você vai escrever todo o código neste arquivo.

## Processando um Palpite {#processing-a-guess}

A primeira parte do programa do jogo de adivinhação vai pedir a entrada do
usuário, processar essa entrada e verificar se ela está na forma esperada. Para
começar, vamos permitir que o jogador digite um palpite. Digite o código da
Listagem 2-1 em _src/main.rs_.

<Listing number="2-1" file-name="src/main.rs" caption="Código que obtém um palpite do usuário e o imprime">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```

</Listing>

Este código contém muitas informações, então vamos examiná-lo linha por linha.
Para obter a entrada do usuário e depois imprimir o resultado como saída,
precisamos trazer para o escopo a biblioteca de entrada/saída `io`. A biblioteca
`io` vem da biblioteca padrão, conhecida como `std`:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```

Por padrão, o Rust tem um conjunto de itens definidos na biblioteca padrão que ele
traz para o escopo de todo programa. Esse conjunto é chamado de _prelúdio_
(em inglês, _prelude_), e você pode ver tudo o que ele contém [na documentação
da biblioteca padrão][prelude].

Se um tipo que você quer usar não estiver no prelúdio, você precisa trazê-lo para
o escopo explicitamente com uma instrução `use`. Usar a biblioteca `std::io`
oferece várias funcionalidades úteis, incluindo a capacidade de aceitar a
entrada do usuário.

Como você viu no Capítulo 1, a função `main` é o ponto de entrada do
programa:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

A sintaxe `fn` declara uma nova função; os parênteses, `()`, indicam que não há
parâmetros; e a chave, `{`, inicia o corpo da função.

Como você também aprendeu no Capítulo 1, `println!` é uma macro que imprime uma
string na tela:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```

Este código imprime uma mensagem que informa qual é o jogo e pede que o usuário
digite algo.

### Armazenando Valores com Variáveis {#storing-values-with-variables}

Em seguida, vamos criar uma _variável_ para armazenar a entrada do usuário, assim:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```

Agora o programa está ficando interessante! Acontece muita coisa nesta pequena
linha. Usamos a instrução `let` para criar a variável. Aqui está outro exemplo:

```rust,ignore
let apples = 5;
```

Esta linha cria uma nova variável chamada `apples` e a associa ao valor `5`.
No Rust, as variáveis são imutáveis por padrão, o que significa que, depois de
dar um valor à variável, esse valor não muda. Vamos discutir esse conceito em
detalhes na seção [“Variáveis e Mutabilidade”][variables-and-mutability]<!--
ignore --> do Capítulo 3. Para tornar uma variável mutável, adicionamos `mut`
antes do nome da variável:

```rust,ignore
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

> Nota: A sintaxe `//` inicia um comentário que continua até o fim da
> linha. O Rust ignora tudo o que está em comentários. Vamos discutir os
> comentários com mais detalhes no [Capítulo 3][comments]<!-- ignore -->.

Voltando ao programa do jogo de adivinhação, agora você sabe que `let mut guess`
vai introduzir uma variável mutável chamada `guess`. O sinal de igual (`=`) diz
ao Rust que queremos associar algo à variável agora. À direita do sinal de igual
está o valor ao qual `guess` será associada, que é o resultado da chamada de
`String::new`, uma função que retorna uma nova instância de uma `String`.
[`String`][string]<!-- ignore --> é um tipo de string fornecido pela biblioteca
padrão, um texto expansível, codificado em UTF-8.

A sintaxe `::` na linha `::new` indica que `new` é uma função associada ao tipo
`String`. Uma _função associada_ é uma função implementada em um tipo, neste caso
`String`. Esta função `new` cria uma nova string vazia. Você vai encontrar uma
função `new` em muitos tipos, porque é um nome comum para uma função que cria um
novo valor de algum tipo.

Ao todo, a linha `let mut guess = String::new();` criou uma variável mutável
que, no momento, está associada a uma nova instância vazia de `String`. Puxa!

### Recebendo a Entrada do Usuário {#receiving-user-input}

Lembre que incluímos a funcionalidade de entrada/saída da biblioteca padrão com
`use std::io;` na primeira linha do programa. Agora vamos chamar a função
`stdin` do módulo `io`, que vai nos permitir tratar a entrada do usuário:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

Se não tivéssemos importado o módulo `io` com `use std::io;` no início do
programa, ainda poderíamos usar a função escrevendo esta chamada como
`std::io::stdin`. A função `stdin` retorna uma instância de
[`std::io::Stdin`][iostdin]<!-- ignore -->, que é um tipo que representa uma
alça (handle) para a entrada padrão do seu terminal.

Em seguida, a linha `.read_line(&mut guess)` chama o método
[`read_line`][read_line]<!-- ignore --> da alça da entrada padrão para obter a
entrada do usuário. Também estamos passando `&mut guess` como argumento para
`read_line` para indicar em qual string a entrada do usuário deve ser armazenada.
O trabalho completo de `read_line` é pegar tudo o que o usuário digitar na
entrada padrão e acrescentar isso a uma string (sem sobrescrever o conteúdo
dela), por isso passamos essa string como argumento. O argumento string precisa
ser mutável para que o método possa alterar o conteúdo da string.

O `&` indica que este argumento é uma _referência_, que oferece uma forma de
permitir que várias partes do seu código acessem um mesmo dado sem precisar
copiar esse dado para a memória várias vezes. Referências são uma funcionalidade
complexa, e uma das grandes vantagens do Rust é quão seguro e fácil é usar
referências. Você não precisa conhecer muitos desses detalhes para terminar este
programa. Por enquanto, basta saber que, assim como as variáveis, as referências
são imutáveis por padrão. Por isso, você precisa escrever `&mut guess` em vez de
`&guess` para torná-la mutável. (O Capítulo 4 vai explicar as referências com
mais profundidade.)

<!-- Old headings. Do not remove or links may break. -->

<a id="handling-potential-failure-with-the-result-type"></a>

### Tratando Possíveis Falhas com `Result` {#handling-potential-failure-with-result}

Ainda estamos trabalhando nesta linha de código. Agora estamos discutindo uma
terceira linha de texto, mas observe que ela ainda faz parte de uma única linha
lógica de código. A próxima parte é este método:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```

Poderíamos ter escrito este código assim:

```rust,ignore
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

No entanto, uma linha longa é difícil de ler, então é melhor dividi-la. Muitas
vezes é prudente introduzir uma quebra de linha e outros espaços em branco para
ajudar a quebrar linhas longas quando você chama um método com a sintaxe
`.method_name()`. Agora vamos discutir o que esta linha faz.

Como mencionado antes, `read_line` coloca o que o usuário digitar na string que
passamos para ele, mas também retorna um valor `Result`. [`Result`][result]<!--
ignore --> é uma [_enumeração_][enums]<!-- ignore -->, muitas vezes chamada de
_enum_, que é um tipo que pode estar em um de vários estados possíveis. Chamamos
cada estado possível de _variante_.

O [Capítulo 6][enums]<!-- ignore --> vai tratar de enums com mais detalhes. A
finalidade desses tipos `Result` é codificar informações de tratamento de erros.

As variantes de `Result` são `Ok` e `Err`. A variante `Ok` indica que a operação
foi bem-sucedida, e contém o valor gerado com sucesso. A variante `Err` significa
que a operação falhou, e contém informações sobre como ou por que a operação
falhou.

Valores do tipo `Result`, assim como valores de qualquer tipo, têm métodos
definidos neles. Uma instância de `Result` tem um [método `expect`][expect]<!--
ignore --> que você pode chamar. Se esta instância de `Result` for um valor `Err`,
`expect` vai fazer o programa travar e exibir a mensagem que você passou como
argumento para `expect`. Se o método `read_line` retornar um `Err`, isso
provavelmente será resultado de um erro vindo do sistema operacional. Se esta
instância de `Result` for um valor `Ok`, `expect` vai pegar o valor que `Ok` está
guardando e retornar apenas esse valor para você, para que você possa usá-lo. Nesse
caso, esse valor é o número de bytes da entrada do usuário.

Se você não chamar `expect`, o programa vai compilar, mas você vai receber um aviso:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```

O Rust avisa que você não usou o valor `Result` retornado por `read_line`,
indicando que o programa não tratou um possível erro.

A forma correta de suprimir o aviso é de fato escrever código de tratamento de
erros, mas, no nosso caso, queremos apenas fazer este programa travar quando
ocorrer um problema, então podemos usar `expect`. Você vai aprender a se recuperar
de erros no [Capítulo 9][recover]<!-- ignore -->.

### Imprimindo Valores com Marcadores em `println!` {#printing-values-with-println-placeholders}

Além da chave de fechamento, há apenas mais uma linha para discutir no código até
aqui:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

Esta linha imprime a string que agora contém a entrada do usuário. O conjunto
`{}` de chaves é um marcador: pense em `{}` como pequenas pinças de caranguejo
que seguram um valor no lugar. Ao imprimir o valor de uma variável, o nome da
variável pode ir dentro das chaves. Ao imprimir o resultado da avaliação de uma
expressão, coloque chaves vazias na string de formato, depois siga a string de
formato com uma lista de expressões separadas por vírgula, para serem impressas
em cada marcador de chaves vazias, na mesma ordem. Imprimir uma variável e o
resultado de uma expressão em uma única chamada de `println!` ficaria assim:

```rust
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```

Este código imprimiria `x = 5 and y + 2 = 12`.

### Testando a Primeira Parte {#testing-the-first-part}

Vamos testar a primeira parte do jogo de adivinhação. Execute-o usando `cargo run`:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-01/
cargo clean
cargo run
input 6 -->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```

Neste ponto, a primeira parte do jogo está pronta: estamos recebendo a entrada do
teclado e depois imprimindo-a.

## Gerando um Número Secreto {#generating-a-secret-number}

Em seguida, precisamos gerar um número secreto que o usuário vai tentar adivinhar.
O número secreto deve ser diferente a cada vez, para que o jogo seja divertido de
jogar mais de uma vez. Vamos usar um número aleatório entre 1 e 100, para que o
jogo não fique difícil demais. O Rust ainda não inclui funcionalidades de números
aleatórios em sua biblioteca padrão. No entanto, a equipe do Rust fornece um
[crate `rand`][randcrate] com essa funcionalidade.

<!-- Old headings. Do not remove or links may break. -->
<a id="using-a-crate-to-get-more-functionality"></a>

### Ampliando as Funcionalidades com um Crate {#increasing-functionality-with-a-crate}

Lembre-se de que um crate é uma coleção de arquivos de código-fonte Rust. O
projeto que estamos construindo é um crate binário, ou seja, um executável. O
crate `rand` é um crate de biblioteca, que contém código feito para ser usado em
outros programas e que não pode ser executado sozinho.

A coordenação de crates externos pelo Cargo é onde o Cargo realmente brilha.
Antes de escrevermos código que usa `rand`, precisamos modificar o arquivo
_Cargo.toml_ para incluir o crate `rand` como dependência. Abra esse arquivo agora
e adicione a linha a seguir no final, abaixo do cabeçalho de seção
`[dependencies]` que o Cargo criou para você. Certifique-se de especificar `rand`
exatamente como fizemos aqui, com este número de versão, ou os exemplos de código
deste tutorial podem não funcionar:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:

* ch01-01-installation.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Nome do arquivo: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:8:}}
```

No arquivo _Cargo.toml_, tudo o que segue um cabeçalho faz parte dessa seção, que
continua até que outra seção comece. Em `[dependencies]`, você diz ao Cargo de quais
crates externos o seu projeto depende e quais versões desses crates você
requer. Neste caso, especificamos o crate `rand` com o especificador de versão
semântica `0.10.1`. O Cargo entende o [Versionamento Semântico][semver]<!--
ignore --> (chamado às vezes de _SemVer_), que é um padrão para escrever números
de versão. O especificador `0.10.1` é, na verdade, uma abreviação de `^0.10.1`,
o que significa qualquer versão que seja pelo menos 0.10.1 e abaixo de 0.11.0.

O Cargo considera que essas versões têm APIs públicas compatíveis com a versão
0.10.1, e essa especificação garante que você vai obter a versão de correção
(patch) mais recente que ainda compile com o código deste capítulo. Qualquer
versão 0.11.0 ou superior não tem garantia de ter a mesma API que os exemplos a
seguir usam.

Agora, sem alterar nenhum código, vamos compilar o projeto, como mostrado na
Listagem 2-2.

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
rm Cargo.lock
cargo clean
cargo build -->

<Listing number="2-2" caption="A saída da execução de `cargo build` depois de adicionar o crate `rand` como dependência">

```console
$ cargo build
    Updating crates.io index
     Locking 8 packages to latest Rust 1.96.0 compatible versions
  Downloaded rand_core v0.10.1
  Downloaded chacha20 v0.10.1
  Downloaded rand v0.10.1
  Downloaded 3 crates (162.9KiB) in 0.59s
   Compiling libc v0.2.186
   Compiling rand_core v0.10.1
   Compiling getrandom v0.4.3
   Compiling cfg-if v1.0.4
   Compiling chacha20 v0.10.1
   Compiling rand v0.10.1
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.03s
```

</Listing>

Você pode ver números de versão diferentes (mas todos serão compatíveis com o
código, graças ao SemVer) e linhas diferentes (dependendo do sistema
operacional), e as linhas podem aparecer em outra ordem.

Quando incluímos uma dependência externa, o Cargo busca as versões mais recentes
de tudo o que essa dependência precisa no _registro_, que é uma cópia dos dados
do [Crates.io][cratesio]. O Crates.io é onde as pessoas do ecossistema Rust
publicam seus projetos Rust de código aberto para que outros possam usá-los.

Depois de atualizar o registro, o Cargo verifica a seção `[dependencies]` e baixa
quaisquer crates listados que ainda não foram baixados. Neste caso, embora
tenhamos listado apenas `rand` como dependência, o Cargo também baixou outros
crates de que `rand` depende para funcionar. Depois de baixar os crates, o Rust
os compila e depois compila o projeto com as dependências disponíveis.

Se você executar `cargo build` de novo imediatamente, sem fazer nenhuma alteração,
não verá nenhuma saída além da linha `Finished`. O Cargo sabe que já baixou e
compilou as dependências, e você não alterou nada sobre elas no seu arquivo
_Cargo.toml_. O Cargo também sabe que você não alterou nada no seu código, então
também não o recompila. Sem nada a fazer, ele simplesmente encerra.

Se você abrir o arquivo _src/main.rs_, fizer uma alteração trivial, salvar e
compilar novamente, verá apenas duas linhas de saída:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
touch src/main.rs
cargo build -->

```console
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
```

Essas linhas mostram que o Cargo só atualiza a compilação com a sua pequena
alteração no arquivo _src/main.rs_. Suas dependências não mudaram, então o Cargo
sabe que pode reutilizar o que já baixou e compilou para elas.

<!-- Old headings. Do not remove or links may break. -->
<a id="ensuring-reproducible-builds-with-the-cargo-lock-file"></a>

#### Garantindo Builds Reprodutíveis {#ensuring-reproducible-builds}

O Cargo tem um mecanismo que garante que você possa recompilar o mesmo artefato
toda vez que você ou qualquer outra pessoa compilar o seu código: o Cargo vai usar
apenas as versões das dependências que você especificou, até que você indique o
contrário. Por exemplo, suponha que, na semana que vem, a versão 0.10.2 do crate
`rand` seja lançada, e que essa versão contenha uma correção de bug importante,
mas também uma regressão que quebre o seu código. Para lidar com isso, o Rust cria
o arquivo _Cargo.lock_ na primeira vez que você executa `cargo build`, então agora
temos esse arquivo no diretório _guessing_game_.

Quando você compila um projeto pela primeira vez, o Cargo descobre todas as
versões das dependências que atendem aos critérios e as grava no arquivo
_Cargo.lock_. Quando você compilar o seu projeto no futuro, o Cargo vai ver que o
arquivo _Cargo.lock_ existe e vai usar as versões especificadas ali, em vez de
refazer todo o trabalho de descobrir as versões de novo. Isso permite que você
tenha um build reprodutível automaticamente. Em outras palavras, o seu projeto
vai continuar na 0.10.1 até que você faça uma atualização explícita, graças ao
arquivo _Cargo.lock_. Como o arquivo _Cargo.lock_ é importante para builds
reprodutíveis, ele costuma ser versionado junto com o restante do código do
projeto.

#### Atualizando um Crate para Obter uma Nova Versão {#updating-a-crate-to-get-a-new-version}

Quando você _quer_ atualizar um crate, o Cargo oferece o comando `update`, que
ignora o arquivo _Cargo.lock_ e descobre todas as versões mais recentes que
atendem às suas especificações no _Cargo.toml_. Em seguida, o Cargo grava essas
versões no arquivo _Cargo.lock_. Caso contrário, por padrão, o Cargo só vai
procurar versões maiores que 0.10.1 e menores que 0.11.0. Se o crate `rand` tiver
lançado as duas novas versões 0.10.2 e 0.999.0, você veria o seguinte ao executar
`cargo update`:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo update
assuming there is a new version of rand; otherwise use another update
as a guide to creating the hypothetical output shown here -->

```console
$ cargo update
    Updating crates.io index
     Locking 1 package to latest Rust 1.96.0 compatible version
    Updating rand v0.10.1 -> v0.10.2 (available: v0.999.0)
```

O Cargo ignora a versão 0.999.0. Nesse momento, você também notaria uma mudança no
seu arquivo _Cargo.lock_ indicando que a versão do crate `rand` que você está
usando agora é a 0.10.2. Para usar a versão 0.999.0 do `rand`, ou qualquer versão
da série 0.999._x_, você precisaria atualizar o arquivo _Cargo.toml_ para ficar
assim (não faça essa alteração de fato, porque os exemplos a seguir pressupõem
que você está usando o `rand` 0.10):

```toml
[dependencies]
rand = "0.999.0"
```

Na próxima vez que você executar `cargo build`, o Cargo vai atualizar o registro
de crates disponíveis e reavaliar os seus requisitos de `rand` de acordo com a
nova versão que você especificou.

Há muito mais a dizer sobre o [Cargo][doccargo]<!-- ignore --> e [o seu
ecossistema][doccratesio]<!-- ignore -->, que vamos discutir no Capítulo 14, mas,
por enquanto, isso é tudo o que você precisa saber. O Cargo torna muito fácil
reutilizar bibliotecas, então os Rustaceans conseguem escrever projetos menores
montados a partir de vários pacotes.

### Gerando um Número Aleatório {#generating-a-random-number}

Vamos começar a usar o `rand` para gerar um número para adivinhar. O próximo passo
é atualizar o _src/main.rs_, como mostrado na Listagem 2-3.

<Listing number="2-3" file-name="src/main.rs" caption="Adicionando código para gerar um número aleatório">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

</Listing>

Primeiro, adicionamos a linha `use rand::prelude::*;`. O módulo `prelude` contém
as partes mais usadas do crate `rand`, e `use` deixa esses itens disponíveis no
escopo do nosso programa.

Em seguida, estamos adicionando duas linhas no meio. Na primeira linha, chamamos
a função `rand::rng`, que nos dá o gerador de números aleatórios que vamos usar:
um que é local à thread de execução atual e que é inicializado (seeded) pelo
sistema operacional. Depois, chamamos o método `random_range` no gerador de números
aleatórios. Esse método é definido pelo trait `RngExt`, que faz parte do módulo
`rand::prelude` que trouxemos para o escopo com a instrução `use
rand::prelude::*;`. O método `random_range` recebe uma expressão de intervalo como
argumento e gera um número aleatório dentro desse intervalo. O tipo de expressão de
intervalo que estamos usando aqui tem a forma `start..=end` e é inclusivo nos
limites inferior e superior, então precisamos especificar `1..=100` para pedir um
número entre 1 e 100.

> Nota: Você não vai saber, apenas por intuição, o que trazer para o escopo e quais
> métodos e funções chamar de um crate, por isso cada crate tem documentação com
> instruções para usá-lo. Outro recurso interessante do Cargo é que executar o
> comando `cargo doc --open` vai gerar localmente a documentação fornecida por
> todas as suas dependências e abri-la no navegador. Se você tiver interesse em
> outras funcionalidades do crate `rand`, por exemplo, execute `cargo doc --open`
> e clique em `rand` na barra lateral à esquerda.

A segunda linha nova imprime o número secreto. Isso é útil enquanto desenvolvemos
o programa, para podermos testá-lo, mas vamos apagá-la na versão final. Não seria
muito divertido se o programa mostrasse a resposta assim que começasse!

Experimente executar o programa algumas vezes:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-03/
cargo run
4
cargo run
5
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4

$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```

Você deve obter números aleatórios diferentes, e todos devem estar entre 1 e 100.
Se aparecerem avisos (warnings), pode ignorá-los com segurança. Se aparecerem
erros, verifique se você tem `rand = "0.10.1"` no seu *Cargo.toml*, já que
versões futuras do `rand` podem ter uma API diferente, mas qualquer versão da
série `0.10` deve funcionar com o código deste capítulo.

## Comparando o Palpite com o Número Secreto {#comparing-the-guess-to-the-secret-number}

Agora que temos a entrada do usuário e um número aleatório, podemos compará-los.
Esse passo é mostrado na Listagem 2-4. Observe que esse código ainda não vai
compilar, como vamos explicar.

<Listing number="2-4" file-name="src/main.rs" caption="Tratando os possíveis valores de retorno da comparação entre dois números">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

</Listing>

Primeiro, adicionamos mais uma instrução `use`, trazendo para o escopo um tipo
chamado `std::cmp::Ordering` da biblioteca padrão. O tipo `Ordering` é outro enum e
tem as variantes `Less`, `Greater` e `Equal`. Esses são os três resultados
possíveis ao comparar dois valores.

Depois, adicionamos cinco novas linhas no final que usam o tipo `Ordering`. O
método `cmp` compara dois valores e pode ser chamado em qualquer coisa que possa
ser comparada. Ele recebe uma referência ao que você quer comparar: aqui, está
comparando `guess` com `secret_number`. Depois, retorna uma variante do enum
`Ordering` que trouxemos para o escopo com a instrução `use`. Usamos uma expressão
[`match`][match]<!-- ignore --> para decidir o que fazer em seguida, com base em
qual variante de `Ordering` foi retornada pela chamada de `cmp` com os valores em
`guess` e `secret_number`.

Uma expressão `match` é formada por _braços_. Um braço consiste em um _padrão_ com
o qual comparar e no código que deve ser executado se o valor passado para `match`
corresponder ao padrão desse braço. O Rust pega o valor passado para `match` e
percorre o padrão de cada braço, um por vez. Padrões e a construção `match` são
recursos poderosos do Rust: eles permitem expressar uma variedade de situações que
o seu código pode encontrar e garantem que você trate todas elas. Esses recursos
serão cobertos em detalhes no Capítulo 6 e no Capítulo 19, respectivamente.

Vamos percorrer um exemplo com a expressão `match` que usamos aqui. Suponha que
o usuário tenha chutado 50 e que o número secreto gerado aleatoriamente desta vez
seja 38.

Quando o código compara 50 com 38, o método `cmp` vai retornar
`Ordering::Greater`, porque 50 é maior que 38. A expressão `match` recebe o valor
`Ordering::Greater` e começa a verificar o padrão de cada braço. Ela olha para o
padrão do primeiro braço, `Ordering::Less`, e vê que o valor `Ordering::Greater` não
corresponde a `Ordering::Less`, então ignora o código desse braço e passa para o
próximo. O padrão do próximo braço é `Ordering::Greater`, que _corresponde_ a
`Ordering::Greater`! O código associado a esse braço vai ser executado e imprimir
`Too big!` na tela. A expressão `match` termina após a primeira correspondência
bem-sucedida, então, neste cenário, ela não olha para o último braço.

No entanto, o código da Listagem 2-4 ainda não compila. Vamos tentar:

<!--
The error numbers in this output should be that of the code **WITHOUT** the
anchor or snip comments
-->

```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

O núcleo do erro afirma que há _tipos incompatíveis_ (mismatched types). O Rust
tem um sistema de tipos forte e estático. No entanto, ele também tem inferência de
tipos. Quando escrevemos `let mut guess = String::new()`, o Rust conseguiu inferir
que `guess` deveria ser uma `String` e não nos fez escrever o tipo. Já
`secret_number` é um tipo numérico. Alguns dos tipos numéricos do Rust podem ter
um valor entre 1 e 100: `i32`, um número de 32 bits; `u32`, um número sem sinal de
32 bits; `i64`, um número de 64 bits; além de outros. Salvo indicação em contrário,
o Rust usa `i32` por padrão, que é o tipo de `secret_number`, a menos que você
adicione informação de tipo em outro lugar que faça o Rust inferir um tipo numérico
diferente. O motivo do erro é que o Rust não consegue comparar um tipo string com um
tipo numérico.

No fim das contas, queremos converter a `String` que o programa lê como entrada em
um tipo numérico, para podermos compará-la numericamente com o número secreto.
Fazemos isso adicionando esta linha ao corpo da função `main`:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

A linha é:

```rust,ignore
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

Criamos uma variável chamada `guess`. Mas espere, o programa já não tem uma
variável chamada `guess`? Tem, mas, de forma útil, o Rust permite _sombrear_
(shadow) o valor anterior de `guess` com um novo. O _sombreamento_ (shadowing) nos
permite reutilizar o nome da variável `guess` em vez de forçar a criação de duas
variáveis únicas, como `guess_str` e `guess`, por exemplo. Vamos cobrir isso com
mais detalhes no [Capítulo 3][shadowing]<!-- ignore -->, mas, por enquanto, saiba
que esse recurso costuma ser usado quando você quer converter um valor de um tipo
para outro tipo.

Associamos essa nova variável à expressão `guess.trim().parse()`. O `guess` na
expressão se refere à variável `guess` original, que continha a entrada como string.
O método `trim` em uma instância de `String` elimina qualquer espaço em branco no
início e no fim, o que precisamos fazer antes de converter a string para um `u32`,
que só pode conter dados numéricos. O usuário precisa pressionar <kbd>enter</kbd>
para satisfazer o `read_line` e enviar o seu palpite, o que adiciona um caractere
de nova linha à string. Por exemplo, se o usuário digitar <kbd>5</kbd> e pressionar
<kbd>enter</kbd>, `guess` fica assim: `5\n`. O `\n` representa “nova linha”. (No
Windows, pressionar <kbd>enter</kbd> resulta em um retorno de carro e uma nova
linha, `\r\n`.) O método `trim` elimina `\n` ou `\r\n`, resultando apenas em `5`.

O [método `parse` em strings][parse]<!-- ignore --> converte uma string em outro
tipo. Aqui, usamos esse método para converter uma string em um número. Precisamos
dizer ao Rust qual é exatamente o tipo numérico que queremos, usando `let guess:
u32`. Os dois-pontos (`:`) depois de `guess` dizem ao Rust que vamos anotar o tipo
da variável. O Rust tem alguns tipos numéricos embutidos; o `u32` visto aqui é um
inteiro sem sinal de 32 bits. É uma boa escolha padrão para um número positivo
pequeno. Você vai aprender sobre outros tipos numéricos no
[Capítulo 3][integers]<!-- ignore -->.

Além disso, a anotação `u32` neste programa de exemplo e a comparação com
`secret_number` significam que o Rust vai inferir que `secret_number` também deve
ser um `u32`. Assim, a comparação será entre dois valores do mesmo tipo!

O método `parse` só funciona com caracteres que podem ser convertidos logicamente
em números e, por isso, pode facilmente causar erros. Se, por exemplo, a string
contiver `A👍%`, não haveria como converter isso em um número. Como pode falhar, o
método `parse` retorna um tipo `Result`, assim como o método `read_line` faz
(discutido anteriormente em [“Tratando Possíveis Falhas com
`Result`”](#handling-potential-failure-with-result)<!-- ignore -->). Vamos tratar
esse `Result` da mesma forma, usando o método `expect` novamente. Se `parse`
retornar a variante `Err` de `Result` por não conseguir criar um número a partir da
string, a chamada de `expect` vai travar o jogo e imprimir a mensagem que passamos a
ele. Se `parse` conseguir converter a string em um número com sucesso, ele vai
retornar a variante `Ok` de `Result`, e `expect` vai retornar o número que queremos
a partir do valor `Ok`.

Vamos executar o programa agora:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/
touch src/main.rs
cargo run
  76
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```

Muito bom! Mesmo com espaços adicionados antes do palpite, o programa conseguiu
descobrir que o usuário chutou 76. Execute o programa algumas vezes para verificar
o comportamento diferente com diferentes tipos de entrada: acertar o número,
chutar um número alto demais e chutar um número baixo demais.

Já temos a maior parte do jogo funcionando, mas o usuário só pode fazer um palpite.
Vamos mudar isso adicionando um laço (loop)!

## Permitindo Vários Palpites com Laços {#allowing-multiple-guesses-with-looping}

A palavra-chave `loop` cria um laço infinito. Vamos adicionar um laço para dar aos
usuários mais chances de adivinhar o número:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

Como você pode ver, movemos tudo, desde o pedido de palpite até o final, para
dentro de um laço. Certifique-se de indentar as linhas dentro do laço mais quatro
espaços cada uma e execute o programa novamente. Agora o programa vai pedir outro
palpite para sempre, o que na verdade introduz um novo problema. Parece que o
usuário não consegue sair!

O usuário sempre pode interromper o programa usando o atalho de teclado
<kbd>ctrl</kbd>-<kbd>C</kbd>. Mas há outra forma de escapar desse monstro
insaciável, como mencionado na discussão do `parse` em [“Comparando o Palpite com o
Número Secreto”](#comparing-the-guess-to-the-secret-number)<!-- ignore -->: se o
usuário digitar uma resposta que não seja um número, o programa vai travar. Podemos
aproveitar isso para permitir que o usuário saia, como mostrado aqui:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-04-looping/
touch src/main.rs
cargo run
(too small guess)
(too big guess)
(correct guess)
quit
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit

thread 'main' (6694925) panicked at src/main.rs:28:47:
Please type a number!: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Digitar `quit` vai encerrar o jogo, mas, como você vai notar, qualquer outra
entrada que não seja um número também vai. Isso é, no mínimo, subótimo; queremos
que o jogo também pare quando o número correto for adivinhado.

### Saindo Depois de um Palpite Correto {#quitting-after-a-correct-guess}

Vamos programar o jogo para encerrar quando o usuário vencer, adicionando uma
instrução `break`:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

Adicionar a linha `break` depois de `You win!` faz o programa sair do laço quando
o usuário acerta o número secreto. Sair do laço também significa sair do programa,
porque o laço é a última parte de `main`.

### Tratando Entradas Inválidas {#handling-invalid-input}

Para refinar ainda mais o comportamento do jogo, em vez de travar o programa quando
o usuário digita algo que não é um número, vamos fazer o jogo ignorar entradas que
não sejam números, para que o usuário possa continuar tentando. Podemos fazer isso
alterando a linha em que `guess` é convertida de `String` para `u32`, como mostrado
na Listagem 2-5.

<Listing number="2-5" file-name="src/main.rs" caption="Ignorando um palpite que não é número e pedindo outro palpite em vez de travar o programa">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

</Listing>

Trocamos uma chamada de `expect` por uma expressão `match` para passar de travar
diante de um erro para tratar o erro. Lembre-se de que `parse` retorna um tipo
`Result`, e `Result` é um enum que tem as variantes `Ok` e `Err`. Estamos usando
uma expressão `match` aqui, assim como fizemos com o resultado `Ordering` do
método `cmp`.

Se `parse` conseguir transformar a string em um número com sucesso, vai retornar um
valor `Ok` que contém o número resultante. Esse valor `Ok` vai corresponder ao
padrão do primeiro braço, e a expressão `match` simplesmente vai retornar o valor
`num` que `parse` produziu e colocou dentro do valor `Ok`. Esse número vai acabar
exatamente onde queremos, na nova variável `guess` que estamos criando.

Se `parse` _não_ conseguir transformar a string em um número, vai retornar um valor
`Err` que contém mais informações sobre o erro. O valor `Err` não corresponde ao
padrão `Ok(num)` do primeiro braço, mas corresponde ao padrão `Err(_)` do segundo
braço. O sublinhado, `_`, é um valor coringa; neste exemplo, estamos dizendo que
queremos corresponder a todos os valores `Err`, não importa qual informação eles
tenham dentro. Então, o programa vai executar o código do segundo braço, `continue`,
que diz ao programa para ir para a próxima iteração do `loop` e pedir outro palpite.
Ou seja, na prática, o programa ignora todos os erros que `parse` possa encontrar!

Agora tudo no programa deve funcionar como esperado. Vamos testar:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-05/
cargo run
(too small guess)
(too big guess)
foo
(correct guess)
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

Ótimo! Com mais um pequeno ajuste final, terminamos o jogo de adivinhação. Lembre
que o programa ainda imprime o número secreto. Isso funcionou bem para testar, mas
estraga o jogo. Vamos apagar o `println!` que imprime o número secreto. A
Listagem 2-6 mostra o código final.

<Listing number="2-6" file-name="src/main.rs" caption="Código completo do jogo de adivinhação">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

</Listing>

Neste ponto, você construiu com sucesso o jogo de adivinhação. Parabéns!

## Resumo {#summary}

Este projeto foi uma forma prática de apresentar a você muitos conceitos novos do
Rust: `let`, `match`, funções, o uso de crates externos e mais. Nos próximos
capítulos, você vai aprender sobre esses conceitos com mais detalhes. O Capítulo 3
trata de conceitos que a maioria das linguagens de programação tem, como variáveis,
tipos de dados e funções, e mostra como usá-los no Rust. O Capítulo 4 explora o
ownership, uma funcionalidade que torna o Rust diferente de outras linguagens. O
Capítulo 5 discute structs e a sintaxe de métodos, e o Capítulo 6 explica como os
enums funcionam.

[prelude]: ../std/prelude/index.html
[variables-and-mutability]: ch03-01-variables-and-mutability.html#variables-and-mutability
[comments]: ch03-04-comments.html
[string]: ../std/string/struct.String.html
[iostdin]: ../std/io/struct.Stdin.html
[read_line]: ../std/io/struct.Stdin.html#method.read_line
[result]: ../std/result/enum.Result.html
[enums]: ch06-00-enums.html
[expect]: ../std/result/enum.Result.html#method.expect
[recover]: ch09-02-recoverable-errors-with-result.html
[randcrate]: https://crates.io/crates/rand
[semver]: http://semver.org
[cratesio]: https://crates.io/
[doccargo]: https://doc.rust-lang.org/cargo/
[doccratesio]: https://doc.rust-lang.org/cargo/reference/publishing.html
[match]: ch06-02-match.html
[shadowing]: ch03-01-variables-and-mutability.html#shadowing
[parse]: ../std/primitive.str.html#method.parse
[integers]: ch03-02-data-types.html#integer-types
