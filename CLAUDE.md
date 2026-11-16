# CLAUDE.md

Tradução PT-BR de **The Rust Programming Language** (o "livro do Rust"), fork de `rust-lang/book`, parte da org [`documentacao-dev`](https://github.com/documentacao-dev). Publicado em https://rust.documentacao.dev.

- Conteúdo traduzível: `src/*.md`, traduzido no lugar. Primeiro os capítulos 1 a 6; `listings/` (código) não se traduz.
- Site: mdBook (`mdbook build`, saída em `book/`), com os pré-processadores de `packages/mdbook-trpl` (`cargo install --locked --path packages/mdbook-trpl`).
- Sync: branch `sync/upstream-AAAA-MM` com merge de `upstream/main`, retradução só do que mudou, atualização de `tool/traducao/ultimo-sync-upstream.txt`.

## Modelo da tradução: sempre Haiku

Toda tradução e retradução de página roda no **Haiku** (`model: haiku`), local ou na nuvem. O modelo principal da sessão só orquestra (git, scripts, commits, PR). A revisão é de um subagente Sonnet de contexto limpo, com 100% da prosa lida lado a lado e `COBERTURA` declarada; lotes de no máximo ~1.500 linhas por revisor.

## Convenções do livro

- Marcador: primeira linha do arquivo `<!-- ia-translate: true -->`.
- `{{#include ...}}`, `{{#rustdoc_include ...}}`, tags `<Listing ...>` / `</Listing>` (o atributo `caption` se traduz; `number` e `file-name` não), blocos de código e saídas de terminal ficam idênticos.
- Links de referência `[texto][chave]` e as definições `[chave]: url` no fim do arquivo: a chave e a URL ficam em inglês; traduza só o texto visível.
- Títulos traduzidos levam a âncora original: depois de traduzir, rode `python3 tool/traducao/ancorar_titulos.py <arquivos>` (acrescenta `{#slug-do-inglês}`, que o mdBook aceita). Links `#...` continuam apontando para o id em inglês.
- `python3 tool/traducao/verificar.py` confere tudo isso contra o inglês do commit em `ultimo-sync-upstream.txt`.

## Aviso de página não traduzida

`tool/aviso/aviso.py` é um pré-processador do mdBook (registrado no `book.toml`): todo capítulo sem `<!-- ia-translate: true -->` ganha no topo um aviso de que ainda está em inglês, com link para o arquivo no GitHub. Ao traduzir um capítulo, o aviso some sozinho no próximo build. Estilo em `theme/aviso.css`.
