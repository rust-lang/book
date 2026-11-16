#!/usr/bin/env python3
"""Mantém as âncoras em inglês nos títulos traduzidos.

O site gera o id de cada título com o github-slugger a partir do texto. Quando
o título é traduzido, o id muda e os links `#...` de outras páginas quebram.
Este script compara os títulos do arquivo traduzido com os do inglês (na mesma
ordem) e acrescenta `{#slug-do-ingles}` aos que mudaram. O gatsby-config.js do
fork liga `enableCustomId` no gatsby-remark-autolink-headers.

Uso: python3 tool/traducao/ancorar_titulos.py <arquivos...>
"""
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
BASE = (ROOT / 'tool' / 'traducao' / 'ultimo-sync-upstream.txt').read_text().strip()
TITULO = re.compile(r'^(#{1,6})\s+(.*?)\s*$')
CUSTOM = re.compile(r'\s*\{#[^}]+\}\s*$')


def titulos(linhas):
    """Índices e textos dos títulos fora de blocos de código e do front matter."""
    dentro = False
    fm = linhas[:1] == ['---']
    for i, linha in enumerate(linhas):
        if fm:
            if i > 0 and linha == '---':
                fm = False
            continue
        if linha.lstrip().startswith('```'):
            dentro = not dentro
            continue
        if not dentro and (m := TITULO.match(linha)):
            yield i, m.group(1), m.group(2)


def slug(texto, vistos):
    """github-slugger: minúsculas, sem pontuação, espaços viram hífens; repetidos ganham -1, -2."""
    s = re.sub(r'[^\w\- ]', '', texto.lower().strip().replace('`', ''))
    s = s.replace(' ', '-')
    base, n = s, vistos.get(s, 0)
    vistos[base] = n + 1
    return base if n == 0 else f'{base}-{n}'


def ancorar(caminho):
    pt = (ROOT / caminho).read_text().split('\n')
    r = subprocess.run(['git', 'show', f'{BASE}:{caminho}'], cwd=ROOT, capture_output=True, text=True)
    if r.returncode:
        print(f'{caminho}: não existe no upstream, pulado')
        return 0
    en = list(titulos(r.stdout.split('\n')))
    tp = list(titulos(pt))
    if len(en) != len(tp):
        print(f'{caminho}: {len(en)} títulos no inglês e {len(tp)} na tradução, nada feito')
        return 0
    vistos, mudou = {}, 0
    for (_, _, t_en), (i, nivel, t_pt) in zip(en, tp):
        s = slug(CUSTOM.sub('', t_en), vistos)
        if CUSTOM.search(t_pt) or t_pt == t_en:
            continue
        pt[i] = f'{nivel} {t_pt} {{#{s}}}'
        mudou += 1
    (ROOT / caminho).write_text('\n'.join(pt))
    print(f'{caminho}: {mudou} título(s) ancorado(s)')
    return mudou


if __name__ == '__main__':
    for a in sys.argv[1:]:
        ancorar(str(Path(a).resolve().relative_to(ROOT)) if Path(a).is_absolute() else a)
