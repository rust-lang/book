#!/usr/bin/env python3
"""Pré-processador do mdBook: avisa o leitor quando o capítulo ainda está em inglês.

Todo capítulo cujo arquivo não começa com `<!-- ia-translate: true -->` ganha,
no topo, um aviso em português com link para o arquivo no GitHub, para quem
quiser ajudar a traduzir. A decisão é feita no build; capítulos traduzidos não
mudam.

Protocolo do mdBook: chamado como `aviso.py supports <renderer>` (responde se
suporta) e depois sem argumentos, lendo [contexto, livro] em JSON no stdin e
devolvendo o livro em JSON no stdout.
"""
import json
import sys

MARCADOR = '<!-- ia-translate: true -->'
REPO = 'https://github.com/documentacao-dev/rust-book-brasil/blob/main/src/'


def aviso(caminho):
    return (
        '<div class="aviso-traducao" role="note">\n'
        '<p><strong>Esta página ainda está em inglês.</strong> '
        'A tradução para o português ainda não chegou aqui. Quer ajudar? '
        f'<a href="{REPO}{caminho}">Traduza no GitHub →</a></p>\n'
        '</div>\n\n'
    )


def percorrer(itens):
    for item in itens:
        cap = item.get('Chapter') if isinstance(item, dict) else None
        if not cap:
            continue
        caminho = cap.get('path') or cap.get('source_path')
        conteudo = cap.get('content', '')
        if caminho and not conteudo.lstrip().startswith(MARCADOR):
            cap['content'] = aviso(caminho) + conteudo
        percorrer(cap.get('sub_items', []))


def main():
    if len(sys.argv) > 1 and sys.argv[1] == 'supports':
        sys.exit(0 if sys.argv[2] == 'html' else 1)
    _contexto, livro = json.load(sys.stdin)
    percorrer(livro.get('items') or livro.get('sections') or [])
    json.dump(livro, sys.stdout)


if __name__ == '__main__':
    main()
