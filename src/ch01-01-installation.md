<!-- ia-translate: true -->

## Instalação {#installation}

O primeiro passo é instalar o Rust. Vamos baixar o Rust pelo `rustup`, uma
ferramenta de linha de comando para gerenciar versões do Rust e as ferramentas
associadas. Para o download, você vai precisar de conexão com a internet.

> Nota: Se, por algum motivo, você preferir não usar o `rustup`, veja a
> [página de outros métodos de instalação][otherinstall] para mais opções.

Os passos a seguir instalam a versão estável mais recente do compilador do Rust.
As garantias de estabilidade do Rust asseguram que todos os exemplos do livro que
compilam continuarão compilando em versões mais novas do Rust. A saída pode
variar um pouco entre versões, porque o Rust costuma melhorar as mensagens de
erro e os avisos. Em outras palavras, qualquer versão estável mais nova que você
instalar com estes passos deve funcionar como esperado com o conteúdo deste
livro.

> ### Notação de linha de comando
>
> Neste capítulo e em todo o livro, vamos mostrar alguns comandos usados no
> terminal. As linhas que você deve digitar em um terminal começam todas com `$`.
> Você não precisa digitar o caractere `$`; ele é o prompt da linha de comando,
> que indica o início de cada comando. As linhas que não começam com `$`
> normalmente mostram a saída do comando anterior. Além disso, os exemplos
> específicos do PowerShell vão usar `>` em vez de `$`.

### Instalando o `rustup` no Linux ou macOS {#installing-rustup-on-linux-or-macos}

Se você usa Linux ou macOS, abra um terminal e digite o seguinte comando:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

O comando baixa um script e inicia a instalação da ferramenta `rustup`, que
instala a versão estável mais recente do Rust. Talvez você precise informar sua
senha. Se a instalação for bem-sucedida, esta linha vai aparecer:

```text
Rust is installed now. Great!
```

Você também vai precisar de um _linker_, um programa que o Rust usa para juntar
as saídas compiladas em um único arquivo. É provável que você já tenha um. Se
aparecerem erros de linker, instale um compilador C, que normalmente inclui um
linker. Um compilador C também é útil porque alguns pacotes comuns do Rust
dependem de código C e precisam dele.

No macOS, você pode obter um compilador C executando:

```console
$ xcode-select --install
```

Usuários de Linux normalmente devem instalar o GCC ou o Clang, de acordo com a
documentação da sua distribuição. Por exemplo, se você usa o Ubuntu, pode
instalar o pacote `build-essential`.

### Instalando o `rustup` no Windows {#installing-rustup-on-windows}

No Windows, acesse [https://www.rust-lang.org/tools/install][install]<!-- ignore
--> e siga as instruções para instalar o Rust. Em algum momento da instalação,
você será solicitado a instalar o Visual Studio. Ele fornece um linker e as
bibliotecas nativas necessárias para compilar programas. Se precisar de mais
ajuda nesta etapa, veja
[https://rust-lang.github.io/rustup/installation/windows-msvc.html][msvc]<!--
ignore -->.

O restante deste livro usa comandos que funcionam tanto no _cmd.exe_ quanto no
PowerShell. Se houver diferenças específicas, vamos explicar qual usar.

### Solução de problemas {#troubleshooting}

Para verificar se o Rust foi instalado corretamente, abra um shell e digite
esta linha:

```console
$ rustc --version
```

Você deve ver o número da versão, o hash do commit e a data do commit da versão
estável mais recente lançada, no seguinte formato:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

Se você vir essas informações, o Rust foi instalado com sucesso! Se não vir,
confira se o Rust está na sua variável de sistema `%PATH%`, da seguinte forma.

No Windows CMD, use:

```console
> echo %PATH%
```

No PowerShell, use:

```powershell
> echo $env:Path
```

No Linux e no macOS, use:

```console
$ echo $PATH
```

Se está tudo certo e o Rust ainda não funciona, há vários lugares onde você pode
pedir ajuda. Veja como entrar em contato com outros Rustaceans (um apelido
engraçado que a comunidade usa para si mesma) na [página da comunidade][community].

### Atualizando e desinstalando {#updating-and-uninstalling}

Depois de instalar o Rust pelo `rustup`, atualizar para uma versão recém-lançada
é fácil. No seu shell, execute o seguinte script de atualização:

```console
$ rustup update
```

Para desinstalar o Rust e o `rustup`, execute o seguinte script de desinstalação
no seu shell:

```console
$ rustup self uninstall
```

<!-- Old headings. Do not remove or links may break. -->
<a id="local-documentation"></a>

### Lendo a documentação local {#reading-the-local-documentation}

A instalação do Rust também inclui uma cópia local da documentação, para que você
possa lê-la offline. Execute `rustup doc` para abrir a documentação local no seu
navegador.

Sempre que um tipo ou uma função vier da biblioteca padrão e você não tiver
certeza do que ele faz ou de como usá-lo, use a documentação da API (interface de
programação de aplicações) para descobrir!

<!-- Old headings. Do not remove or links may break. -->
<a id="text-editors-and-integrated-development-environments"></a>

### Usando editores de texto e IDEs {#using-text-editors-and-ides}

Este livro não faz suposições sobre quais ferramentas você usa para escrever código
Rust. Praticamente qualquer editor de texto resolve! Porém, muitos editores de
texto e ambientes de desenvolvimento integrados (IDEs) têm suporte nativo a Rust.
Você sempre pode encontrar uma lista razoavelmente atualizada de editores e IDEs
na [página de ferramentas][tools] do site do Rust.

### Trabalhando offline com este livro {#working-offline-with-this-book}

Em vários exemplos, vamos usar pacotes do Rust além da biblioteca padrão. Para
acompanhar esses exemplos, você vai precisar de conexão com a internet ou ter
baixado essas dependências antes. Para baixar as dependências com antecedência,
você pode executar os seguintes comandos. (Vamos explicar em detalhes mais
adiante o que é o `cargo` e o que cada um desses comandos faz.)

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:

* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

```console
$ cargo new get-dependencies
$ cd get-dependencies
$ cargo add rand@0.10.1 trpl@0.2.0
```

Isso vai guardar em cache os downloads desses pacotes, então você não precisará
baixá-los depois. Depois de executar esse comando, você não precisa mais manter a
pasta `get-dependencies`. Se você executou esse comando, pode usar a flag
`--offline` com todos os comandos do `cargo` no restante do livro, para usar essas
versões em cache em vez de tentar acessar a rede.

[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install]: https://www.rust-lang.org/tools/install
[msvc]: https://rust-lang.github.io/rustup/installation/windows-msvc.html
[community]: https://www.rust-lang.org/community
[tools]: https://www.rust-lang.org/tools
