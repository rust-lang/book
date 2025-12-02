## Instalação

O primeiro passo é instalar Rust. Nós iremos usar o `rustup`, uma ferramenta de linha de comando para gerenciar versões do Rust e as suas ferramentas e para isso precisamos de uma conexão com a _internet_.
>[!NOTE]
> Se por algum motivo você prefere não utilizar o `rustup`, por favor veja as opções em 
> [Outras maneiras de instalar o Rust][otherinstall].

Siga os passos para instalar a versão estável do compilador Rust. Todos os exemplos desse livro irá compilar mesmo com as versões maIs recentes (retrocompatibilidade).

### Como escrever linha de comando

>[!WARNING]
> Neste capítulo e em todo o livro nós iremos usar alguns comandos usados no terminal. Você não precisa adicionar o caracter`$` na linha de comando. Cada instrução deve ser passada uma linha de cade vez. 


> [!CAUTION]
>Normalmente PowerShell usa, por exemplo, `>` ao invés de `$`.

### Instalando `rustup` no linux ou MacOS


Se você estiver usando Linux ou macOs, abra o terminal e execute:

```terminal
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | shxs
```
O comando realiza o _download_ do _script_ e inicia a instalação do `rustup` responsável por instalar a última versão estável do Rust. Talvez tenha que digitar a sua senha. Se tudo der certo, você deverá ver a mensagem:

```text
Rust is installed now. Great!
```
Você também precisará de um _linker_ que o Rust utiliza para compilar as saídas em único arquivo. Talvez você já tenha um. Se quiser verificar os erros, precisa um compilador C que tem um _linker_. Um compilador C também pode ser útil para alguns pacotes que precisa de um compilador C.

Para instalar um compilador C no macOS execute:


```terminal
xcode-select --install
```

>[!WARNING]
Usuários Linux devem ter instalado o GCC ou Clang compatível com a sua distribuição. Por exemplo, para distro Ubuntu pode instalar o pacote `build-essential`.

### Instalar `rustup` no Windows

No Windows, visite [https://www.rust-lang.org/tools/install][install]<!-- ignore
--> e siga as instruções de instalação do Rust. Se precisar de mais ajuda, acesse: [https://rust-lang.github.io/rustup/installation/windows-msvc.html][msvc]<!--
ignore -->.

Os demais comandos usados nesse livro funciona no **CMD** e **PowerShell**. Se houver uma excessão, será avisado e explicado como utilizar.

### Solução de problemas

Para verificar a versão instalada do Rust, execute o comando:

```console
rustc --version
```

>[!NOTE]
Com esse comando você deve visualizar a versão instalada do Rust. A saída será semelhante ao exemplo abaixo.

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```
Se você encontrou essa informação, significa que você tem o Rust instalado. Se você não ver, verifique se o Rust está nas **variáveis de sistema** do Windows

No CMD, execute:

```console
> echo %PATH%
```
ou no PowerShell, execute:

```powershell
> echo $env:Path
```

No Linux ou macOs, execute:

```console
echo $PATH
```
Se tiver executado os comandos e o Rust ainda não funcionar, você pode pedir ajuda para outros Rustaceans na [página da comunidade][community].

### Atualizando e Desinstalando

Se você usou instalou Rust usando o `rustup`, atualizar para uma versão mais atualizada fica fácil. No seu terminal/CMD execute:

```console
rustup update
```

Para desinstalar Rust e `rustup`, execute:

```console
rustup self uninstall
```

<!-- Old headings. Do not remove or links may break. -->
<a id="local-documentation"></a>

### Lendo a documentação de forma local

A instalação do Rust também inclui a cópia da documentação, então, você pode ler _offline_. Execute `rustup doc` para abrir a documentação no seu navegador.

A qualquer momento você pode não entender como um tipo ou função fornecida pela biblioteca padrão funciona, use a documentação API para entender melhor.

<!-- Old headings. Do not remove or links may break. -->
<a id="text-editors-and-integrated-development-environments"></a>

### Editores de texto e IDEs

Esse livro não espera que você tenha conhecimento prévio sobre ferramentas usadas para escrever códigos em Rust. Qualquer editor pode dar conta do recado. Mas, muitos editores de texto e IDE tem suporte nativo para Rust. Você pode consultar uma lista de IDEs em [página de ferramentas][tools].

### Estudando offline

Em muitos exemplos, nós iremos usar pacotes disponibilizados pela biblioteca padrão. Para trabalharmos com esses exemplos você já precisa estar conectado a _internet_ ou já ter feito _download_ dessas dependências anteriormente. Para baixar as dependências previamente você pode, por exemplo, executar:

>[!NOTE]
> Nos próximos capítulos serão abordados assuntos relacionados ao `cargo`

```console
cargo new get-dependencies
cd get-dependencies
cargo add rand@0.8.5 trpl@0.2.0
```
Isso irá armazenar o _download_ desses pacotes em forma de _cache_. Uma vez executado o comando você não precisa manter a pasta `get-dependencies`. Se preferir, você pode usar os comandos com a _flag_ `--offline`.

[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install]: https://www.rust-lang.org/tools/install
[msvc]: https://rust-lang.github.io/rustup/installation/windows-msvc.html
[community]: https://www.rust-lang.org/community
[tools]: https://www.rust-lang.org/tools
