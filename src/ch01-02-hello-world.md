<!-- ia-translate: true -->
## Olá, mundo! {#hello-world}

Agora que você instalou o Rust, é hora de escrever seu primeiro programa em Rust. É tradicional, ao aprender uma nova linguagem, escrever um pequeno programa que imprime o texto `Hello, world!` na tela, então faremos o mesmo aqui!

> Nota: Este livro pressupõe familiaridade básica com a linha de comando. O Rust não impõe exigências específicas sobre seu editor, suas ferramentas ou o lugar onde seu código fica, então, se preferir usar uma IDE em vez da linha de comando, fique à vontade para usar a IDE de sua preferência. Muitas IDEs já oferecem algum grau de suporte a Rust; consulte a documentação da sua IDE para detalhes. A equipe do Rust tem se concentrado em oferecer um ótimo suporte a IDEs por meio do `rust-analyzer`. Veja o [Apêndice D][devtools]<!-- ignore --> para mais detalhes.

<!-- Old headings. Do not remove or links may break. -->
<a id="creating-a-project-directory"></a>

### Configuração do diretório do projeto {#project-directory-setup}

Você começará criando um diretório para guardar seu código em Rust. Para o Rust não importa onde seu código fica, mas, para os exercícios e projetos deste livro, sugerimos criar um diretório _projects_ na sua pasta pessoal e guardar todos os seus projetos ali.

Abra um terminal e digite os seguintes comandos para criar o diretório _projects_ e um diretório para o projeto “Hello, world!” dentro de _projects_.

Para Linux, macOS e PowerShell no Windows, digite:

```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

Para o CMD do Windows, digite:

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

<!-- Old headings. Do not remove or links may break. -->
<a id="writing-and-running-a-rust-program"></a>

### Básico de programas em Rust {#rust-program-basics}

Em seguida, crie um novo arquivo de código-fonte e chame-o de _main.rs_. Arquivos em Rust sempre terminam com a extensão _.rs_. Se o nome do arquivo tiver mais de uma palavra, a convenção é separá-las com sublinhado. Por exemplo, use _hello_world.rs_ em vez de _helloworld.rs_.

Agora abra o arquivo _main.rs_ que você acabou de criar e digite o código da Listagem 1-1.

<Listing number="1-1" file-name="main.rs" caption="Um programa que imprime `Hello, world!`">

```rust
fn main() {
    println!("Hello, world!");
}
```

</Listing>

Salve o arquivo e volte à janela do terminal, no diretório _~/projects/hello_world_. No Linux ou macOS, digite os seguintes comandos para compilar e executar o arquivo:

```console
$ rustc main.rs
$ ./main
Hello, world!
```

No Windows, digite o comando `.\main` em vez de `./main`:

```powershell
> rustc main.rs
> .\main
Hello, world!
```

Independentemente do seu sistema operacional, a string `Hello, world!` deve ser impressa no terminal. Se você não vir essa saída, volte à parte [“Solução de problemas”][troubleshooting]<!-- ignore --> da seção de Instalação para encontrar formas de obter ajuda.

Se `Hello, world!` foi impresso, parabéns! Você acaba de escrever oficialmente um programa em Rust. Isso faz de você um programador Rust. Bem-vindo!

<!-- Old headings. Do not remove or links may break. -->

<a id="anatomy-of-a-rust-program"></a>

### A anatomia de um programa em Rust {#the-anatomy-of-a-rust-program}

Vamos revisar em detalhes este programa “Hello, world!”. Aqui está a primeira peça do quebra-cabeça:

```rust
fn main() {

}
```

Essas linhas definem uma função chamada `main`. A função `main` é especial: ela é sempre o primeiro código executado em todo programa executável em Rust. Aqui, a primeira linha declara uma função chamada `main` que não tem parâmetros e não retorna nada. Se houvesse parâmetros, eles iriam dentro dos parênteses (`()`).

O corpo da função fica envolto em `{}`. O Rust exige chaves ao redor de todos os corpos de função. É uma boa prática de estilo colocar a chave de abertura na mesma linha da declaração da função, deixando um espaço entre as duas.

> Nota: Se você quiser seguir um estilo padrão entre projetos em Rust, pode usar uma ferramenta de formatação automática chamada `rustfmt` para formatar seu código em um estilo específico (mais sobre o `rustfmt` no [Apêndice D][devtools]<!-- ignore -->). A equipe do Rust incluiu essa ferramenta na distribuição padrão, assim como o `rustc`, então ela já deve estar instalada no seu computador!

O corpo da função `main` contém o seguinte código:

```rust
println!("Hello, world!");
```

Esta linha faz todo o trabalho neste pequeno programa: ela imprime texto na tela. Há três detalhes importantes a notar aqui.

Primeiro, `println!` chama uma macro do Rust. Se ela chamasse uma função, seria escrita como `println` (sem o `!`). Macros do Rust são uma forma de escrever código que gera código para estender a sintaxe da linguagem, e vamos discuti-las em mais detalhes no [Capítulo 20][ch20-macros]<!-- ignore -->. Por enquanto, basta saber que usar `!` significa que você está chamando uma macro em vez de uma função comum, e que macros nem sempre seguem as mesmas regras das funções.

Segundo, você vê a string `"Hello, world!"`. Passamos essa string como argumento para `println!`, e ela é impressa na tela.

Terceiro, terminamos a linha com um ponto e vírgula (`;`), que indica que essa expressão terminou e que a próxima está pronta para começar. A maioria das linhas de código em Rust termina com ponto e vírgula.

<!-- Old headings. Do not remove or links may break. -->
<a id="compiling-and-running-are-separate-steps"></a>

### Compilação e execução {#compilation-and-execution}

Você acabou de executar um programa recém-criado, então vamos examinar cada etapa do processo.

Antes de executar um programa em Rust, você precisa compilá-lo usando o compilador do Rust, digitando o comando `rustc` e passando o nome do arquivo de código-fonte, assim:

```console
$ rustc main.rs
```

Se você tem experiência com C ou C++, vai notar que isso é parecido com `gcc` ou `clang`. Depois de compilar com sucesso, o Rust gera um executável binário.

No Linux, macOS e PowerShell no Windows, você pode ver o executável digitando o comando `ls` no shell:

```console
$ ls
main  main.rs
```

No Linux e no macOS, você verá dois arquivos. No PowerShell do Windows, verá os mesmos três arquivos que veria usando o CMD. No CMD do Windows, digitaria o seguinte:

```cmd
> dir /B %= the /B option says to only show the file names =%
main.exe
main.pdb
main.rs
```

Isso mostra o arquivo de código-fonte com a extensão _.rs_, o arquivo executável (_main.exe_ no Windows, mas _main_ em todas as outras plataformas) e, ao usar Windows, um arquivo com informações de depuração com a extensão _.pdb_. A partir daí, você executa o arquivo _main_ ou _main.exe_, assim:

```console
$ ./main # or .\main on Windows
```

Se o seu _main.rs_ for o programa “Hello, world!”, esta linha imprimirá `Hello, world!` no seu terminal.

Se você está mais acostumado com uma linguagem dinâmica, como Ruby, Python ou JavaScript, pode não estar habituado a compilar e executar um programa como etapas separadas. Rust é uma linguagem _compilada antecipadamente_ (_ahead-of-time_), o que significa que você pode compilar um programa e entregar o executável para outra pessoa, e ela poderá executá-lo mesmo sem ter o Rust instalado. Se você entregar a alguém um arquivo _.rb_, _.py_ ou _.js_, essa pessoa precisa ter, respectivamente, uma implementação de Ruby, Python ou JavaScript instalada. Mas, nessas linguagens, você só precisa de um comando para compilar e executar o programa. Em design de linguagens, tudo é uma questão de compromisso (trade-off).

Usar apenas o `rustc` para compilar é suficiente para programas simples, mas, conforme seu projeto cresce, você vai querer gerenciar todas as opções e facilitar o compartilhamento do seu código. A seguir, vamos apresentar a ferramenta Cargo, que vai ajudar você a escrever programas em Rust do mundo real.

[troubleshooting]: ch01-01-installation.html#troubleshooting
[devtools]: appendix-04-useful-development-tools.html
[ch20-macros]: ch20-05-macros.html
