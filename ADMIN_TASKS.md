# Atividades Administrativas

Essa documentação é para qualquer um que gerencia o repositório para lembrar de como deve ser feita a manutenção das tarefas.

## Atualize a versão `rustc`

- Delete a sua pasta `target`, você irá recompilar tudo.
- Mude o número da versão em`.github/workflows/main.yml`.
- Mude o número da versão em `rust-toolchain`, o que deve mudar o que está usando localmente com `rustup`.
- Mude o número da versão em `src/title-page.md`
- Execute `./tools/update-rustc.sh` (verifique o código comentado para mais detalhes do que isso faz).
- Verifique as mudanças (olhe os arquivos modificados conforme o indicado pelo git que podem ser vistas em `tmp/book-before` e `tmp/book-after`) e fazer _commit_ é algo bom.
- Busque por `manual-regeneration` e siga as instruções encontradas para atualizar o _output_ que não pode ser gerado por um script.

## Atualize a `edition` em todas as referências

Para atualizar os metadados de `edition = "[year]"` em todas as referências `Cargo.toml` execute script`./tools/update-editions.sh`. Veja as modificações para ter certeza que está correto. Então, faça o _commit_.

## Atualize a `edition` em mdBook config

Abra `book.toml` e `nostarch/book.toml` e defina o valor em `edition` no tabela`[rust]` para uma nova edição.

## Publicar uma nova versão

Agora temos arquivos`.tar` que contém todas as versões disponiveis em [GitHub Releases](https://github.com/rust-lang/book/releases). Para uma nova versão caso haja mudanças no Rust pelo `rustfmt`, siga esses passos:

- Crie uma _tag_ no _GitHub_ para _releases_ e envie ou crie usando _GitHub UI_, [novas releases](https://github.com/rust-lang/book/releases/new), e selecione a nova tag ao invés da pré existentes.
- Execute `cargo run --bin release_listings`, que gerará
  `tmp/listings.tar.gz`
- Envie `tmp/listings.tar.gz` para o _GitHub UI_ para criar um rascunho da _release_
- Publique a _release_

## Adicionando uma nova referência

Para facilitar execute `rustfmt` para atualizar a saídas de todos os arquivos quando o compilador estiver terminado e gera uma _release_ que contém todas as referências do projeto. Como fazer:

- Encontre onde a nova referência na pasta `listings`.
  - Encontrará um subdiretório para cada capítulo.
  - As referências numerada devem usar `listing-[capitulo numero]-[listing numero]` para os nomes das pastas.
  - Referências sem um número deverá começar com `no-listing-` seguido por um número que indica a posição do capítulo quando comparado à outra referência sem número no capítulo e uma breve descrição que alguém poderia encontrar. Por exemplo:
    - Nós dizemos "se tivéssemos escrito X ou invés de Y teríamos esse erro no compilador". "Erro: não encontramos códigos para X". Deveria ser nomeado com `output-only-` seguido por um número que indica a posição relativa ao capítulo usado apenas para saída, com uma pequena descrição que os autores ou contribuidores possa ler e encontrar o código que procuram.
    - **Lembre de ajustar os números de forma apropriada!**
  - Crie um projeto Cargo na pasta, usando `cargo new` ou copiando outra referência como ponto de partida.
- Adicione o código e para criar um exemplo de como funciona. Exemplo:
  Se você apenas quiser mostrar parte de um código dentro do arquivo, use um _anchor comment_ (`// ANCHOR: some_tag` and `// ANCHOR_END: some_tag`) para destacar portes do arquivo que quer mostrar.
- Para código Rust, utilize o `{{#rustdoc_include [filename:some_tag]}}`
  para incluir o bloco no texto. A diretiva `rustdoc_include` fornece código que não será mostrado para o `rustdoc` e `mdbook test`.
- Para qualquer outra coisa use `{{#include [filename:some_tag]}}`.
- Se você deseja mostrar a saída de um comando no texto crie um `output.txt` na lista de diretórios.
  - Execute código como `cargo run` ou `cargo test`, e copie as saídas geradas.
  - Crie `output.txt` com a primeira linha `$ [comando que executou]`.
  - Cole a saída copiada
  - Execute `./tools/update-rustc.sh` que deve realizar uma normalização da saída do compilador.
  - Inclua a saída no formato de texto com o comando `{{#include [filename]}}`.
  - Adicione e faça um _commit_ de `output.txt`.
  - Se deseja ver a saída e por algum motivo não pode ser gerado por um _script_ (informe, que não pode ser gerado por alguns motivos, como requisições _web_), mantenha a saída da mesma linha, mas adicione um comentário que contenha `manual-regeneration` instruções para atualizações manuais de saída.
- Se você não quer esse exemplo seja formatado pelo `rustfmt` (por exemplo, porque o exemplo não pode ser convertido) adicione o arquivo `rustfmt-ignore` no diretório e o arquivo não será formatado. Erros serão corrigidos algum di).

## Veja o efeito das mudanças no livro

Para verificar a atualização do `mdbook` ou mudanças de arquivos inclusos:

- Gere um livro antes das mudanças que você quer testar usando `mdbook
  build -d tmp/book-before`
- Aplique as mudanças que você quer testar e execute `mdbook build -d tmp/book-after`
- Execute `./tools/megadiff.sh`
- Os arquivos gerados com `tmp/book-before` e `tmp/book-after` contém as diferenças que você pode verificar manualmente.

## Gere novos arquivos Markdown para No Starch

- Execute `./tools/nostarch.sh`
- Verifique os arquivos criado na pasta `nostarch`
- Verifique o git se quiser começar novas edições.

## Gere markdown de arquivos docx

- Salve o arquivo `.docx` em `tmp/chapterXX.docx`.
- No editor de texto vá para revisão e "Aceite todas as mudanças e parar o rastreamento".
- salve o arquivo `.docx` e feche o editor de texto.
- Execute `./tools/doc-to-md.sh`
- Deverá ter a saída `nostarch/chapterXX.md`. Ajustada como .`.XSL` em
  `tools/doc-to-md.xsl` e execute `./tools/doc-to-md.sh` novamente se precisar.

## Gerar visualização gráfica (Graphviz)

Usamos [Graphviz](http://graphviz.org/) para alguns diagramas no livro. O código-fonte desses arquivos está na pasta `dot` directory. Para transformar o `dot` em arquivo, por exemplo, `dot/trpl04-01.dot` em um `svg`, execute:

```bash
$ dot dot/trpl04-01.dot -Tsvg > src/img/trpl04-01.svg
```

Remova os atributos de tamanho e altura do `svg` gerado e defina `viewBox` como `0.00 0.00 1000.00 1000.00` ou qualquer outro valor que não corte a imagem.

## Publique uma prévia no Github Pages

As vezes, publicamos no GitHub Pages o progresso. Para publicação:

- Instale `ghp-import` com `pip install ghp-import` (ou `pipx install ghp-import`, usando [pipx][pipx]).
- No pasta raiz, execute `tools/generate-preview.sh`

[pipx]: https://pipx.pypa.io/stable/#install-pipx
