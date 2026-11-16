#!/usr/bin/env python3
"""Confere os capítulos traduzidos do livro do Rust contra o inglês.

Para cada src/*.md com `<!-- ia-translate: true -->`:

- blocos de código, includes do mdBook ({{#include}}, {{#rustdoc_include}}) e
  tags <Listing> (menos o caption) idênticos e na mesma ordem;
- URLs de links e definições de link de referência idênticas;
- mesmo número de títulos; título traduzido com {#id} em inglês
  (rode ancorar_titulos.py) e aviso para título idêntico ao inglês;
- pouca prosa sobrando em inglês.

O inglês vem do commit em tool/traducao/ultimo-sync-upstream.txt.
Uso: python3 tool/traducao/verificar.py [caminhos...]   (código 1 se houver erro)
"""
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
BASE = (ROOT / 'tool' / 'traducao' / 'ultimo-sync-upstream.txt').read_text().strip()
MARCADOR = '<!-- ia-translate: true -->'
CERCA = re.compile(r'^```.*?^```', re.S | re.M)
INCLUDE = re.compile(r'\{\{#[a-z_]+ [^}]*\}\}')
LISTING = re.compile(r'<Listing\b(?:[^>"]|"[^"]*")*>|</Listing>')
CAPTION = re.compile(r'\s*caption="[^"]*"')
LINK = re.compile(r'\]\(([^)\s]+)')
DEFS = re.compile(r'^\[([^\]]+)\]:\s*(\S+)', re.M)
TITULO = re.compile(r'^#{1,6}\s+(.*)$', re.M)
EN = set('the and of to is are you your this that with for in it be can when '
         'will from have which an by on as at or'.split())


def sem_codigo(t):
    return CERCA.sub('', t)


def conferir(caminho):
    erros, avisos = [], []
    pt = (ROOT / caminho).read_text()
    r = subprocess.run(['git', 'show', f'{BASE}:{caminho}'], cwd=ROOT, capture_output=True, text=True)
    if r.returncode:
        return [f'{caminho}: não existe no upstream ({BASE[:9]})']
    en = r.stdout
    if not pt.startswith(MARCADOR):
        erros.append('marcador ia-translate não está na primeira linha')
    pt = pt[len(MARCADOR):].lstrip('\n') if pt.startswith(MARCADOR) else pt

    for nome, rx in (('blocos de código', CERCA), ('includes do mdBook', INCLUDE)):
        a, b = rx.findall(en), rx.findall(pt)
        if a != b:
            erros.append(f'{nome}: {len(a)} no inglês, {len(b)} na tradução'
                         + (' (conteúdo diferente)' if len(a) == len(b) else ''))
    def inline(t):
        t = re.sub(r'\{#[^}]*\}', '', sem_codigo(t))
        return [m.group(2).strip() for m in re.finditer(r'(`+)(.+?)\1', t, re.S)]
    novos = sorted(set(inline(pt)) - set(inline(en)))
    for c in novos[:5]:
        avisos.append(f'código inline que não existe no inglês (traduzido?): `{c}`')
    norm = lambda t: [CAPTION.sub('', m.group(0)) for m in LISTING.finditer(t)]
    if norm(en) != norm(pt):
        erros.append('tags <Listing> diferentes do inglês (só o caption pode mudar)')
    if LINK.findall(sem_codigo(en)) != LINK.findall(sem_codigo(pt)):
        erros.append('URLs de links inline diferentes do inglês')
    if DEFS.findall(en) != DEFS.findall(pt):
        erros.append('definições [chave]: url diferentes do inglês')

    t_en, t_pt = TITULO.findall(sem_codigo(en)), TITULO.findall(sem_codigo(pt))
    if len(t_en) != len(t_pt):
        erros.append(f'títulos: {len(t_en)} no inglês, {len(t_pt)} na tradução')
    else:
        sem = [t for a, t in zip(t_en, t_pt) if a != t and '{#' not in t]
        if sem:
            erros.append(f'{len(sem)} título(s) traduzido(s) sem {{#id}} (rode ancorar_titulos.py)')
        for a, t in zip(t_en, t_pt):
            if a == t and re.search(r'[A-Za-z]{3,}', re.sub(r'`[^`]*`', '', t)):
                avisos.append(f'título igual ao inglês: "{t}"')

    prosa = re.sub(r'`[^`]*`|<[^>]+>|\]\([^)]*\)|\{\{[^}]*\}\}', ' ', sem_codigo(pt))
    prosa = DEFS.sub('', prosa).lower()
    palavras = re.findall(r'[a-zà-ú]+', prosa)
    taxa = sum(p in EN for p in palavras) / max(len(palavras), 1)
    if taxa > 0.08:
        erros.append(f'prosa ainda parece inglês ({taxa:.0%} de palavras comuns em inglês)')
    for a in avisos:
        print(f'AVISO {caminho}: {a}')
    return [f'{caminho}: {e}' for e in erros]


def main():
    arquivos = sys.argv[1:] or sorted(
        str(p.relative_to(ROOT)) for p in (ROOT / 'src').glob('*.md') if p.read_text().startswith(MARCADOR))
    erros = [e for a in arquivos for e in conferir(a)]
    for e in erros:
        print('ERRO', e)
    print(f'{len(arquivos)} arquivo(s) traduzido(s), {len(erros)} erro(s)')
    sys.exit(1 if erros else 0)


if __name__ == '__main__':
    main()
