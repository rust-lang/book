## Hello, World!

Agora você tem o Rust instalado e é hora de escrever o seu primeiro programa em Rust. É muito tradicional quando aprendemos uma nova linguagem escrever um pequeno programa que mostra o texto `Hello, world!` na tela, então vamos fazer o mesmo com Rust

>[!TIP]
Assumimos que você tem uma certa familiaridade com terminal. Rust não especifíca qual editor de texto você deve usar, apenas recomendamos que você utilize uma IDE ao invés do terminal. Muitas IDEs oferece um certo grau de suporte para Rust. Consulte a documentação sobre IDE para mais detalhes. O time Rust tem focado para oferecer um bom suporte com `rust-analyzer`. Veja [Apêndice D][devtools]<!-- ignore --> <!-- Old headings. Do not remove or links may break. --> 
<a id="creating-a-project-directory"></a>

### Pasta do Projeto

Primeiro você deve criar uma pasta para salvar o seu código Rust. Não importa o nome da pasta onde estará o seu código, mas para uma melhor organização dos exercícios e projetos nós sugerimos a criação de uma pasta chamada _projects_ para salvar todos os projetos ali.

Crie a pasta _projects_ como a sugestão anterior e crie a pasta hello_word onde irá salvar o programa que mostra `Hello, world!`. Para usar o terminal para realizar a criação execute os seguintes comandos:

Para Linux, macOS, e PowerShell no Windows:

```console
mkdir ~/projects
cd ~/projects
mkdir hello_world
cd hello_world
```

Para Windows usando CMD:

```cmd
mkdir "%USERPROFILE%\projects"
cd /d "%USERPROFILE%\projects"
mkdir hello_world
cd hello_world
```

<!-- Old headings. Do not remove or links may break. -->
<a id="writing-and-running-a-rust-program"></a>

### Programa Básico Rust 

Agora, dentro da pasta `hello_world` crie um arquivo chamado _main.rs_. Arquivos Rust sempre terão extensão _.rs_ e se você tiver usando mais de uma palavra no nome do arquivo a convenção da comunidade é usar "_" para separar. Por exemplo, _hello_world.rs_ ao invés de _helloworld.rs_
>[!WARNING]
Padrão snake case.

Agora abra o arquivo _main.rs_ e digíte o código:

<Listing number="1-1" file-name="main.rs" caption="O programa mostra `Hello, world!`">

```rust
fn main() {
    println!("Hello, world!");
}
```

</Listing>

>[!CAUTION]
Salve o arquivo, volte ao terminal na pasta ~/projects/hello_world. 

No Linux ou macOS, execute o seguinte código para compilar e executar o arquivo:

```console
rustc main.rs
./main
Hello, world!
```

No Windows, execute o comando `.\main` ao invés de `./main`:

```powershell
> rustc main.rs
> .\main
Hello, world!
```

Independentemente do seu sistema operacional o texto `Hello, world!` deve aparecer no terminal. Se você não teve esse texto, volte para a parte de [Solução de problemas][troubleshooting]<!-- ignore --> para encontrar algo que possa te ajudar.

Se foi impresso `Hello, world!`, parabéns! Você oficialmente escreveu um programa em Rust e isso o torna um programador Rust.

<!-- Old headings. Do not remove or links may break. -->

<a id="anatomy-of-a-rust-program"></a>

### A anatomia de um programa Rust

Vamos rever o programa **_hello_world.rs_** em detalhes. A primeira parte do quebra cabeça:

```rust
fn main() {

}
```

Essas linhas definem uma função chamada `main`. A função `main` é sempre primeiro código que um programa Rust executa. 

A primeira linha declara uma função chamada `main` e ela não tem parâmetro e não retorna nada. Se tiver que passar algum parâmetro, ele deve estar dentro de parenteses `()`.

O corpo da função deve estar dentro `{}`. Rust necessita que o corpo da função esteja dentro `{}`. É uma boa prática abrir os colchetes (`{}`) na mesma linha em que a função é declarada adicionando um espaço entre elas.
>[!NOTE]
Se você quer manter um estilo padrão em todo o projetos Rust, você pode usar um formatador de código padrão do Rust executando `rustfmt`, se quiser cuidar criar um estilo de formação personalizado (mais sobre  `rustfmt` no [Apêndice D][devtools]<!-- ignore -->). A equipe do Rust incluiu essa ferramenta como padrão nas versões do Rust, então ele deve estar no seu computador!

O corpo da função `main` tem o seguinte código:

```rust
println!("Hello, world!");
```
Esse trecho é que faz todo o trabalho desse pequeno programa! Mostra o texto na tela e nós temos 3 detalhes importantes para anunciar aqui.

>[!WARNING]
**Primeiro**: `println` é uma macro em Rust. E isso significa que ela chama a função `println` (sem o `!`). Macros em Rust são uma maneira de usar código para gerar outros códigos e assim extender a sintaxe do Rust, nós iremos abordar esse assunto com mais detalhes no [Capítulo 20][ch20-macros]<!-- ignore -->. Por enquanto você só precisa saber que `!` significa que está usando uma macro ao invés de uma função normal e macros nem sempre vão seguir as mesmas regras de uma função. 
> 
>**Segundo**: Nós passamos a _string_ `"Hello, world!"` como um argumento para `println!`, e o texto aparece na tela. 
> 
>**Terceiro**: Nós terminamos a linha com ponto e vírgula (`;`) que indica que a expressão acabou. Há outras linhas de código em Rust termina com `;`

<!-- Old headings. Do not remove or links may break. -->
<a id="compiling-and-running-are-separate-steps"></a>

### Compilação e Execução

Agora você já tem um novo programa, então vamos examinar cada passo do processo.

Se você estiver usando o compilador do Rust deve executar o comando `rustc` seguido o nome do arquivo. Por exemplo

>[!IMPORTANT]
Certifique-se de estar dentro da pasta que contem o arquivo _main.rs_

```console
rustc main.rs
```

Se você já tem experiência com C ou C++, você irá notar que é semelhante ao `gcc` ou `clang`. Após concluir a compilação, Rust irá gerar um binário executável.

Para ver o executável no Linux, macOS, ou PowerShell no Windows, você pode executar o comando `ls` no terminal:

```console
ls 
main main.rs
```
No Linux e macOS, você verá dois arquivos. Com o PowerShell no Windows, você vera a mesmo arquivo, mas se você que visualizar usando CMD no Windows, você executa:

```cmd
dir /B %= the /B option says to only show the file names =%
main.exe
main.pdb
main.rs
```

Isso mostra o arquivo executável _main.exe_ do Windows e _main.rs_ para todas as outras plataformas. Quando estiver usando o Windows terá um arquivo com a extensão _.pdb_ com informações de _debug_. 

Se quiser executar o arquivo _main.rs_ ou _main.exe_ é só executar:

```console
$ ./main # or .\main on Windows
```
Em linguagens dinâmicas como Ruby, Python ou JavaScript ao construir um programa o processo de compilar e executar não são etapas separadas. Rust é uma linguagem compilada a frente do seu tempo, isso significa que após compilar o programa você pode entregar o executável a qualquer pessoa e ela poderá executar sem a necessidade de instalar o Rust, mas se você entrega para pessoa um arquivo _.rb_, _.py_ ou _.js_ eles precisam instalar _Ruby_, _Python_ e JavaScript (respectivamente). Nessas linguagens você precisa apenas de um comando para compilar e executar o programa. Ao criar uma linguagem tudo é uma questão perde e ganha.

É fácil e simples com `rustc` compilar programas simples, mas a medida que o seu programa cresce você precisará de algo para gerenciar todas as opções. Para fazer isso de uma forma fácil e compartilhar seu código em seguida introduziremos a ferramenta Cargo que irá te ajudar a escrever código de programas do mundo real em Rust.

[troubleshooting]: ch01-01-installation.html#troubleshooting
[devtools]: appendix-04-useful-development-tools.html
[ch20-macros]: ch20-05-macros.html
