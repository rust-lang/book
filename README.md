# A Linguagem de Programação Rust

![Build Status](https://github.com/rust-lang/book/workflows/CI/badge.svg)

Esse repositório é uma tradução do livro "A linguagem de programação Rust". Você pode encontrar versão impressa (em inglês) em [No Starch Press][nostarch].

[nostarch]: https://nostarch.com/rust-programming-language-2nd-edition

Você pode ler o livro de forma grátis _online_. Atenção, o livro pode ser entregue com a última versão [estável], [beta], ou [instavel] das versões do Rust. Os erros dessa versão já pode ter sido corrigida no repositório _online_.

[estável]: https://doc.rust-lang.org/stable/book/
[beta]: https://doc.rust-lang.org/beta/book/
[instavel]: https://doc.rust-lang.org/nightly/book/

Veja os [lançamentos].

[lançamentos]: https://github.com/rust-lang/book/releases

## Requisitos

Esse livro também precisa do [mdBook], preferencialmente a mesma versão usada pela linguagem Rust [nesse arquivo][rust-mdbook]. Use esse comando:

[mdBook]: https://github.com/rust-lang/mdBook
[rust-mdbook]: https://github.com/rust-lang/rust/blob/HEAD/src/tools/rustbook/Cargo.toml

```bash
$ cargo install mdbook --locked --version <version_num>
```

Este livro também utiliza dois plugins _mdbook_ que faz parte desse repositório. Se você não instalar, você irá receber avisos durante o _build_ e o resultado do _build_ não estara formatado conforme o esperado, mas você ainda poderá fazer o _build_. Para usar os plugins, utilize o comando:

```bash
$ cargo install --locked --path packages/mdbook-trpl --force
```

## Inicializar e build

Para inicializar o mdbook execute

```bash
$ mdbook init
```

Para realizar o _build_, utilize o comando:

```bash
$ mdbook build
```

O arquivo será salvo na sub pasta `book`. Para conferir abra no seu navegador.

_Firefox:_

```bash
$ firefox book/index.html                       # Linux
$ open -a "Firefox" book/index.html             # OS X
$ Start-Process "firefox.exe" .\book\index.html # Windows (PowerShell)
$ start firefox.exe .\book\index.html           # Windows (Cmd)
```

_Chrome:_

```bash
$ google-chrome book/index.html                 # Linux
$ open -a "Google Chrome" book/index.html       # OS X
$ Start-Process "chrome.exe" .\book\index.html  # Windows (PowerShell)
$ start chrome.exe .\book\index.html            # Windows (Cmd)
```

Para realizar teste:

```bash
$ cd packages/trpl
$ mdbook test --library-path packages/trpl/target/debug/deps
```

## Contribuição

Adoraríamos receber a sua ajuda. Dê uma olhada em [CONTRIBUTING.md][contrib] para entender as formas de contribuição que estamos procurando.

[contrib]: https://github.com/rust-lang/book/blob/main/CONTRIBUTING.md

Por esse livro ser [impresso][nostarch], e por querermos manter a versão online do livro o mais próximo da versão do livro, possa ser que a sua solicitação de _issue_ ou _pull request_ demore para ser aceita.

Além disso, nós temos feito uma extensão revisão para correção de erros. Se a sua _issue_ ou _pull request_ não está relacionada à correção de erros, talvez ela fique para a próxima grande revisão esperada para meses ou anos. Obrigado pela paciência! Confira, [Edições do Rust](https://doc.rust-lang.org/edition-guide/).

### Traduções

Gostamos da ajuda que recebemos para a tradução desse livro. Veja a etiqueta [Translations] para participar de uma tradução que está ocorrendo. Abra uma _issue_ para começar uma nova tradução. Nós estamos aguardando [mdbook support] para multiplas linguagens antes de unir, mas sinta-se livre para começar!

[Translations]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook support]: https://github.com/rust-lang/mdBook/issues/5

## Gramática

Para procurar nos arquivos erros de gramática, você pode usar o comando `spellcheck.sh` disponível na pasta `ci`. É preciso um dicionário válido que pode ser encontrado em `ci/dictionary.txt`. Se o _script_ produzir um falso - positivo você precisa usar `BTreeMap` no _script_ considerado inválido e precisa adicionar a palavra ao `ci/dictionary.txt` (mantenha ordenado por consistencia).
