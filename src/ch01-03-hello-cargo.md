<!-- ia-translate: true -->
## Olá, Cargo! {#hello-cargo}

O Cargo é o sistema de build e o gerenciador de pacotes do Rust. A maioria dos Rustaceans usa essa ferramenta para gerenciar seus projetos Rust, porque o Cargo faz muitas tarefas por você, como compilar o seu código, baixar as bibliotecas das quais o seu código depende e compilar essas bibliotecas. (Chamamos de _dependências_ as bibliotecas de que o seu código precisa.)

Os programas Rust mais simples, como o que escrevemos até agora, não têm nenhuma dependência. Se tivéssemos criado o projeto “Hello, world!” com o Cargo, ele usaria apenas a parte do Cargo responsável por compilar o código. À medida que você escrever programas Rust mais complexos, vai adicionar dependências, e, se começar um projeto usando o Cargo, adicionar dependências será muito mais fácil.

Como a grande maioria dos projetos Rust usa o Cargo, o restante deste livro presume que você também esteja usando. O Cargo vem instalado junto com o Rust se você usou os instaladores oficiais discutidos na seção [“Instalação”][installation]<!-- ignore -->. Se você instalou o Rust de outra forma, verifique se o Cargo está instalado digitando o seguinte no seu terminal:

```console
$ cargo --version
```

Se aparecer um número de versão, você já o tem! Se aparecer um erro, como `command
not found`, consulte a documentação do seu método de instalação para descobrir como instalar o Cargo separadamente.

### Criando um projeto com o Cargo {#creating-a-project-with-cargo}

Vamos criar um novo projeto usando o Cargo e ver em que ele se diferencia do nosso projeto original “Hello, world!”. Volte para o seu diretório _projects_ (ou para onde você decidiu guardar o seu código). Depois, em qualquer sistema operacional, execute o seguinte:

```console
$ cargo new hello_cargo
$ cd hello_cargo
```

O primeiro comando cria um novo diretório e um projeto chamado _hello_cargo_. Nomeamos o projeto _hello_cargo_, e o Cargo cria os arquivos dele em um diretório com o mesmo nome.

Entre no diretório _hello_cargo_ e liste os arquivos. Você verá que o Cargo gerou para nós dois arquivos e um diretório: um arquivo _Cargo.toml_ e um diretório _src_ com um arquivo _main.rs_ dentro.

Ele também inicializou um novo repositório Git, junto com um arquivo _.gitignore_. Os arquivos do Git não serão gerados se você executar `cargo new` dentro de um repositório Git já existente; você pode alterar esse comportamento usando `cargo new --vcs=git`.

> Nota: Git é um sistema de controle de versão bastante comum. Você pode alterar o `cargo new` para usar outro sistema de controle de versão ou nenhum, usando a flag `--vcs`. Execute `cargo new --help` para ver as opções disponíveis.

Abra o _Cargo.toml_ no seu editor de texto preferido. Ele deve ser parecido com o código da Listagem 1-2.

<Listing number="1-2" file-name="Cargo.toml" caption="Conteúdo do *Cargo.toml* gerado pelo `cargo new`">

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

</Listing>

Este arquivo está no formato [_TOML_][toml]<!-- ignore --> (_Tom’s Obvious, Minimal
Language_), que é o formato de configuração do Cargo.

A primeira linha, `[package]`, é um cabeçalho de seção que indica que as instruções seguintes estão configurando um pacote. À medida que adicionarmos mais informações a este arquivo, incluiremos outras seções.

As três linhas seguintes definem as informações de configuração de que o Cargo precisa para compilar o seu programa: o nome, a versão e a edição da linguagem Rust a ser usada. Vamos falar sobre a chave `edition` no [Apêndice E][appendix-e]<!-- ignore -->.

A última linha, `[dependencies]`, é o início de uma seção em que você pode listar as dependências do seu projeto. Em Rust, os pacotes de código são chamados de _crates_. Não precisaremos de outros crates para este projeto, mas vamos precisar deles no primeiro projeto do Capítulo 2, então usaremos essa seção de dependências por lá.

Agora abra _src/main.rs_ e dê uma olhada:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

O Cargo gerou um programa “Hello, world!” para você, igualzinho ao que escrevemos na Listagem 1-1! Até aqui, as diferenças entre o nosso projeto e o projeto gerado pelo Cargo são que o Cargo colocou o código no diretório _src_ e que temos um arquivo de configuração _Cargo.toml_ no diretório de nível superior.

O Cargo espera que os arquivos-fonte fiquem dentro do diretório _src_. O diretório de nível superior do projeto serve apenas para arquivos README, informações de licença, arquivos de configuração e qualquer outra coisa que não esteja relacionada ao seu código. Usar o Cargo ajuda a organizar os seus projetos. Há um lugar para cada coisa, e cada coisa está no seu lugar.

Se você começou um projeto que não usa o Cargo, como fizemos com o projeto “Hello, world!”, pode convertê-lo para um projeto que usa o Cargo. Mova o código do projeto para o diretório _src_ e crie um arquivo _Cargo.toml_ apropriado. Uma maneira fácil de obter esse arquivo _Cargo.toml_ é executar `cargo init`, que o cria automaticamente para você.

### Compilando e executando um projeto Cargo {#building-and-running-a-cargo-project}

Agora vamos ver o que muda ao compilar e executar o programa “Hello, world!” com o Cargo! A partir do diretório _hello_cargo_, compile o seu projeto digitando o seguinte comando:

```console
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

Esse comando cria um arquivo executável em _target/debug/hello_cargo_ (ou _target\debug\hello_cargo.exe_ no Windows), em vez de no diretório atual. Como o build padrão é um build de debug, o Cargo coloca o binário em um diretório chamado _debug_. Você pode executar o executável com este comando:

```console
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
```

Se tudo correr bem, `Hello, world!` deve ser impresso no terminal. Executar `cargo
build` pela primeira vez também faz o Cargo criar um novo arquivo no nível superior: _Cargo.lock_. Esse arquivo registra as versões exatas das dependências do projeto. Este projeto não tem dependências, então o arquivo é bem simples. Você nunca precisará alterar esse arquivo manualmente; o Cargo gerencia o conteúdo dele para você.

Acabamos de compilar um projeto com `cargo build` e o executamos com `./target/debug/hello_cargo`, mas também podemos usar `cargo run` para compilar o código e executar o executável resultante em um único comando:

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Usar `cargo run` é mais prático do que lembrar de executar `cargo
build` e depois digitar o caminho completo até o binário, por isso a maioria dos desenvolvedores usa `cargo
run`.

Repare que, desta vez, não vimos nenhuma saída indicando que o Cargo estava compilando `hello_cargo`. O Cargo percebeu que os arquivos não tinham mudado, então não recompilou e apenas executou o binário. Se você tivesse modificado o código-fonte, o Cargo teria recompilado o projeto antes de executá-lo, e você veria esta saída:

```console
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

O Cargo também oferece um comando chamado `cargo check`. Esse comando verifica rapidamente se o seu código compila, mas não produz um executável:

```console
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

Por que você não gostaria de ter um executável? Muitas vezes, `cargo check` é bem mais rápido que `cargo build`, porque pula a etapa de produzir o executável. Se você estiver verificando continuamente o seu trabalho enquanto escreve o código, usar `cargo check` acelera o processo de descobrir se o projeto ainda compila! Por isso, muitos Rustaceans executam `cargo check` periodicamente enquanto escrevem o programa, para garantir que ele compila. Depois, executam `cargo build` quando estão prontos para usar o executável.

Vamos recapitular o que aprendemos até agora sobre o Cargo:

- Podemos criar um projeto usando `cargo new`.
- Podemos compilar um projeto usando `cargo build`.
- Podemos compilar e executar um projeto em um único passo usando `cargo run`.
- Podemos compilar um projeto sem produzir um binário para verificar erros usando `cargo check`.
- Em vez de salvar o resultado da compilação no mesmo diretório do nosso código, o Cargo o guarda no diretório _target/debug_.

Outra vantagem de usar o Cargo é que os comandos são os mesmos independentemente do sistema operacional em que você estiver trabalhando. Por isso, a partir deste ponto, não daremos mais instruções específicas para Linux e macOS em comparação com Windows.

### Compilando para release {#building-for-release}

Quando o seu projeto finalmente estiver pronto para ser lançado, você pode usar `cargo build
--release` para compilá-lo com otimizações. Esse comando cria um executável em _target/release_ em vez de _target/debug_. As otimizações deixam o seu código Rust mais rápido, mas ativá-las aumenta o tempo que o programa leva para compilar. Por isso existem dois perfis diferentes: um para desenvolvimento, quando você quer recompilar rápido e com frequência, e outro para gerar o programa final que você vai entregar a um usuário, que não será recompilado repetidamente e deve rodar o mais rápido possível. Se você estiver medindo o tempo de execução do seu código, certifique-se de executar `cargo build --release` e fazer o benchmark com o executável em _target/release_.

<!-- Old headings. Do not remove or links may break. -->
<a id="cargo-as-convention"></a>

### Aproveitando as convenções do Cargo {#leveraging-cargos-conventions}

Em projetos simples, o Cargo não traz muito valor em relação a usar apenas o `rustc`, mas ele se prova útil conforme os seus programas ficam mais complexos. Quando os programas passam a ter vários arquivos ou precisam de uma dependência, é muito mais fácil deixar o Cargo coordenar a compilação.

Mesmo o projeto `hello_cargo` sendo simples, ele já usa boa parte das ferramentas reais que você vai usar no restante da sua carreira em Rust. De fato, para trabalhar em qualquer projeto existente, você pode usar os comandos a seguir para baixar o código com o Git, entrar no diretório desse projeto e compilá-lo:

```console
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

Para mais informações sobre o Cargo, veja [a documentação dele][cargo].

## Resumo {#summary}

Você já está muito bem encaminhado na sua jornada com Rust! Neste capítulo, você aprendeu a:

- Instalar a versão estável mais recente do Rust usando `rustup`.
- Atualizar para uma versão mais nova do Rust.
- Abrir a documentação instalada localmente.
- Escrever e executar um programa “Hello, world!” usando o `rustc` diretamente.
- Criar e executar um novo projeto seguindo as convenções do Cargo.

Este é um ótimo momento para construir um programa mais substancial e se acostumar a ler e escrever código Rust. Então, no Capítulo 2, vamos construir um programa de jogo de adivinhação. Se preferir começar aprendendo como os conceitos comuns de programação funcionam em Rust, veja o Capítulo 3 e depois volte ao Capítulo 2.

[installation]: ch01-01-installation.html#installation
[toml]: https://toml.io
[appendix-e]: appendix-05-editions.html
[cargo]: https://doc.rust-lang.org/cargo/
