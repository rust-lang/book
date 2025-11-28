# Contribuindo com o projeto

Adoraríamos receber a sua ajuda. Obrigado por se importar com esse livro.

## Onde Editar

Todas as edições devem ser feita na pasta `src`

A pasta `nostarch` contém trechos que serão enviadas aos editores e para a publicação da versão impressa. Esses trechos mostram o que devem ser enviados ou não, então quando editados eles serão para o No Starch. **Não envie _pull requests_ com modificações nos arquivos da pasta `nostarch`, os _pull requests_ serão fechados.**

Nós usamos [`rustfmt`][rustfmt] para usar uma formatação para código Rust em [`dprint`][dprint] aplicando a padronização para Markdown e códigos não Rust nesse projeto.

[rustfmt]: https://github.com/rust-lang/rustfmt
[dprint]: https://dprint.dev

Normalmente você tem `rustfmt` instalado se você tem um Rust. Se por algum motivo você não tem uma cópia de`rustfmt`, você pode adicionar usando o seguinte comando:

```sh
rustup component add rustfmt
```

Para instalar o `dprint`, usar o seguinte comando:

```sh
cargo install dprint
```

Ou seguir as [instruções][install-dprint] no _site_ do `dprint`.

[install-dprint]: https://dprint.dev/install/

Para formatar o código Rust, você pode utilizar o comando `rustfmt <caminho do arquivo>`, e para formatar outros arquivos, você pode utilizar `dprint fmt <path to file>`. Alguns editores de texto também suporte nativo para `rustfmt` e `dprint`.

## Correções

O livro acompanha o livro de treinamento do Rust. Portanto, se você um problema em https://doc.rust-lang.org/stable/book, deverá ser corrigido na branch `main` desse repositório, mas se
branch in this repo, mas se a correção não aconteceu entre as versões _nightly_ -> _beta_ -> _stable_. Por favor, verifique a branch `main` nesse repositório antes de informar uma _issue_.

Procurar pelo histórico de erro de um arquivo específico podera te ajudar a entender como um erro foi corrigido.

Por favor, também busque por _issues_ aberta e fechada antes de reportar ou abrir um novo _pull request_

## Licença

Este repositório esta sob as mesmas de esse da linguagem Rust, MIT/Apache2. O texto completo da licença você encontra no arquivo `LICENSE-*` neste repositório.

## Código de conduta

O projeto Rust tem [um código de conduta](http://rust-lang.org/policies/code-of-conduct) que governa todos os sub projetos do Rust, incluindo este. Por favor, respeite!

## Expectativas

Como esse livro é [impresso][nostarch], e como queremos manter a versão _online_ deste livro a mais próxima da versão impressa and talvez leve mais tempo que o normal para aprovarmos seu avaliarmos e aprovarmos o seu _pull request_.

[nostarch]: https://nostarch.com/rust-programming-language-2nd-edition

Estamos em uma longa revisão das [Versões do Rust](https://doc.rust-lang.org/edition-guide/). Entre essas revisões nós iremos corrigir apenas erros. Se a sua _issue_ ou _pull request_ não for somente a correção de um erro, talvez seja corrigida nas próximas vezes em que estamos fazendo uma grande revisão: expectativas de meses ou anos. Obrigado por sua paciência.

## Busque ajuda

Se você busca por maneiras que não envolva uma grande quantidade de leitura ou escrita, olhe a

If you're looking for ways to help that don't involve large amounts of
reading or writing, check out the [abertura de issues com etiqueta de ajuda online][help-wanted]. Talvez encontre pequenas correções, códigos Rust ou pequenos _scripts shell_ essas pequenas correções pode nos ajudar grandemente ou melhorar esse livro de alguma forma.

[help-wanted]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3AE-help-wanted

## Traduções

Nós adoraríamos receber ajuda para traduzir esse livro! Veja as etiquetas de [Translations] e junte-se ao progresso atual de tradução. Abra uma nova _issue_ para começar a trabalhar em uma nova linguagem. Aguardamos um suporte da [mdbook] para multiplas linguagens antes de realizarmos o _merge_, mas sinta-se livre para começar.

[Translations]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook]: https://github.com/rust-lang/mdBook/issues/5
