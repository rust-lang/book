## Hello, Cargo!

Cargo é o compilador e gerenciador de pacotes do Rust. Muitos Rustaceans usa essa ferramenta para gerenciar seus projetos porque o Cargo executa muitas tarefas difíceis para você compilar o código, baixar bibliotecas e fazer a compilação (nós chamamos essas bibliotecas que o seu código precisa de dependências).

Pequenos programas em Rust como escrevemos anteriormente não tem nenhuma dependência. Nós construímos "Hello, world!" com Cargo e a única tarefa executada pelo Cargo foi a compilação. A medida que a complexidade do projeto aumenta, será muito mais fácil adicionar dependências com Cargo.

Como as maiorias dos programas escritos em Rust utilizam Cargo, assumiremos que para executar os projetos você também irá utilizar, veja o tópico de [“instalação”][installation]<!-- ignore -->. Se você instalou Rust através de outros meios, você deve verificar se o cargo está instalado. Execute o código:

```console
cargo --version
```
>[!WARNING]
Se foi mostrado o número da versão significa que ele está instalado. Se houver um erro como <span style="color: red">**command not found**</span>, consulte a documentação do método de instalação que você utilizou.

### Criando um projeto com Cargo

Agora, vamos criar um projeto usando Cargo e entender como ele se difere do seu "Hello, world!". Volte a pasta _projects_ (ou onde você decidiu salvar o seu arquivo _hello_world.rs_). Então, execute o comando:

```console
cargo new hello_cargo
cd hello_cargo
```
O primeiro comando `cargo new hello_cargo` cria uma pasta chamada _hello_cargo_, inicia um projeto Cargo e cria um projeto Git com um arquivo _.gitignore_. O segundo comando `cd hello_cargo` entra da pasta do projeto criado com o primeiro comando. Veja os arquivos que contém dentro da pasta do projeto.

Você verá que o Cargo gerou dois arquivos, _Cargo.toml_ e dentro da pasta _src_ haverá um arquivo chamado _main.rs_. 
>[!NOTE]
Se você utilizar o comando `cargo new` dentro de uma pasta com repositório Git você só poderá subscrever os arquivos Git usando o comando `cargo new --vcs=git`.

Abra _Cargo.toml_ no seu editor de texto e você vera algo parecido com isso.

<Listing number="1-2" file-name="Cargo.toml" caption="Arquivo *Cargo.toml* gerados com `cargo new`">

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

</Listing>

Esse arquivo no formato [_TOML_][toml]<!-- ignore --> (_Tom’s Obvious, Minimal
Language_) é a configuração do Cargo.

A primeira linha `[package]` indica que as declarações que estão "dentro" da secção são configurações do pacote. A maneira de adicionar mais informações a esse arquivo veremos em outra sessão.

As próximas três linhas definem as informações que o Cargo precisa para compilar o seu programa: nome, versão e edição do Rust. Voltaremos falar sobre `edition` no [Apêndice E][appendix-e]<!-- ignore -->.

A última linha, `[dependencies]`, inicia a sessão que lista todas as dependências do seu projeto. Em Rust, pacotes de códigos chamamos _crates_. Não iremos utilizar nenhum outro _crates_ para esse projeto, mas iremos utilizar no nosso projeto do Capítulo 2.

Abra `_src/main.rs_` e dê uma olhada:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```
Cargo criou um programa "Hello, World!" para você com apenas uma linha de comando `cargo new hello_world` que executamos anteriormente. Note que a diferença entre o seu projeto e o gerado automaticamente pelo Cargo são o código fonte dentro de `src` e um arquivo de configuração`Cargo.toml` no diretório principal.

Cargo precisa que o seu código-fonte esteja dentro da pasta `src`. A pasta raiz do seu projeto é somente para _README_.md, informações de licensa, arquivos de configuração e nada relacionado ao seu código. Cargo te ajuda a organizar os seus projetos. Esse lugar é para tudo e tudo é para esse lugar.

Se você iniciou o projeto sem usar o Cargo, você pode converter o seu projeto, mova o código para um pasta _src_ e crie um _Cargo_.toml apropriado ou execute o comando  `cargo init`, para que ele crie para você

### Compilando e Executando um Projeto com Cargo

Agora, vamos ver o que há de diferente quando fazemos a compilação do programa "_Hello, World!_" com Cargo! 

Na sua pasta _hello_cargo_, compile o projeto digitando os comandos:

```console
 cargo build
```
>[!WARNING]
O comando cria um arquivo executável em `target/debug/hello_cargo` (ou
_target\debug\hello_cargo.exe_ no Windows). O modo padrão de compilação é o _debug_ então o Cargo coloca o arquivo binário na pasta _debug_. Para executar o binário utilize o comando:

```console
./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
```

>[!TIP]
> Você verá algo como:
> 
> Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs

Se tudo estiver certo você deverá ver `Hello, world!` no seu terminal. Executando `cargo
build` pela primeira vez também faz com que o Cargo crie um novo arquivo na pasta raiz chamado _Cargo.lock_. Esse arquivo mantém um "registro" que demonstra exatamente a versão das dependências do seu projeto. Até agora nosso projeto não tem nenhuma dependência, então o arquivo estará um pouco vazio. 

>[!CAUTION]
Você NUNCA deverá alterar esse arquivo manualmente. O cargo faz isso para você.

Nós construimos o nosso projeto com comando `cargo build` e isso gerou um arquivo que ficou armazenado em `./target/debug/hello_cargo`, mas também podemos usar o `cargo run` para compilar e executar o código com um único comando.

```console
cargo run
```

>[!TIP]
> Você vera algo como:
> 
> Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
Running `target/debug/hello_cargo`
Hello, world!

Usar `cargo run` é menos trabalhoso que usar `cargo build` e depois todo o caminho para o arquivo binário. Então, muitos desenvolvedores usam `cargo run`.

Repare que dessa vez não vimos a saída indicando que o Cargo está compilando `hello_cargo`. O compilador Cargo percebe que nada mudou, então não refaz a construção, mas continua executando o binário. Se você modificou o código fonte, o Cargo irá reconstruir seu binário e você vera algo parecido com:

```console
cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```
Cargo também um comando chamado `cargo check`. Esse comando verifica rápidamente se o seu código poderá compilar.

```console
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```
Se você não querer um executável pode utilizar o `cargo check` que é muito mais rápido que `cargo build` porque ele pula passos para produção do executável. 

>[!TIP]
Se você periodicamnte fica checando o seu código enquanto escreve, usar o `cargo check` tornará o processo mais rápido e você continuara sabendo se o seu projeto ainda pode ser compilado!

Muitos _Rustaceans_ utilizam o `cargo check` periodicamente para saber se o seu programa pode ser compilado e utilizam `cargo build` quando precisam do executável.

Vamos relembrar o que aprendemos sobre o Cargo:

- Podemos criar um projeto usando `cargo new`.
- Podemos compilar um projeto usando `cargo build`.
- Podemos compilar e executar um projeto usando apenas `cargo run`.
- Podemos construir um projeto sem o binário de produção, mas verificando erros usando `cargo check`.
- Cargo salva o binário em na pasta `target/debug`.

Outra vantagem em se utilizar o Cargo é que os comandos são os mesmos independentemente do sistema. Então a partir daqui não vamos especificar instruções para Linux, macOS e Windows.

### Construindo Releases

Quando o seu projeto estiver finalmente pronto para lançamento, use `cargo build --release` para compilar o código com otimizações. Esse comando irá criar um executável em `target/release` ao invés de `target/debug`. As otimizações feitas fazem o seu código Rust executar mais rápido, mas isso levará mais tempo para o seu programa compilar. 

Existem dois motivos diferentes para existir mais de um perfil:

 1-) Em desenvolvimento você quer as coisas de uma forma mais rápida relativamente à construção do programa completo.

2-) Na versão final do programa você dara ao usuário um arquivo que será executado da forma mais rápida possível.

Se quiser o tempo para compilação ao executar o comando adicione a "marcação" `--release` e a comparação também ficará na pasta `target/release` com o binário.

<!-- Old headings. Do not remove or links may break. -->
<a id="cargo-as-convention"></a>

### Convenções Cargo

Em projetos simples, usar Cargo não agrega muito valor quando comparado ao `rustc`, mas é muito útil conforme o seu programa se torna complexo.

O programa `hello_cargo` foi um simples exemplo, até agora não usamos muitas ferramentas que você irá utilizar na sua carreira em Rust. Para trabalhar em projetos existentes, você pode usar o Git para te ajudar. Por exemplo:

```console
git clone example.org/someproject
cd someproject
cargo build
```
Para mais informações sobre o Cargo, veja a [documentação][cargo].

## Resumo

Agora você está pronto para iniciar uma pela carreira em Rust. Nesse capítulo você aprendeu como:

- Instalar a última versão do Rust usando `rustup`.
- Atualizar para uma nova versão do Rust.
- Abrir a documentação de forma local e _offline_.
- Escrever e executar um “_Hello, world!_” usando `rustc`.
- Criar um projeto em Rust usando as convenções do Cargo.

Foi um bom tempo que passamos construindo um programa em Rust. No Capítulo 2 vamos construir um jogo de adivinhação. Se você prefere aprender mais conceitos sobre programação em Rust, veja o Capítulo 3 e depois retorne ao Capítulo 2.

[installation]: ch01-01-installation.html#installation
[toml]: https://toml.io
[appendix-e]: appendix-05-editions.html
[cargo]: https://doc.rust-lang.org/cargo/
