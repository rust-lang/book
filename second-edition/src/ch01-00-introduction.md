# Introdução

Seja bem vindo(a) à “Linguagem de Programação Rust”, um livro introdutório sobre Rust.
Rust é uma linguagem de programação focada em segurança, eficiência e
concorrência. Seu *design* lhe permite criar programas com o desempenho e
controle de uma linguagem de baixo nível, mas com as abstrações poderosas de uma
linguagem de alto nível. Estas propriedades tornam Rust adequado para programadores que
tem experiência em linguagens como C e procuram por uma alternativa mais segura, bem
como para aqueles que vem de linguagens como o Python e que procuram por maneiras de escrever código
com melhor desempenho sem sacrificar a expressividade.

Rust executa a maioria das suas verificações de segurança e decisões de gerenciamento de memória
em tempo de compilação, para que o desempenho de execução do seu programa não seja impactado. Isso
torna a linguagem útil em um número de casos para os quais outras linguagens não são adequadas:
programas com requisitos de tempo e espaço previsíveis, incorporação de código em outras
linguagens e a escrita de código de baixo nível, como *drivers* de dispositivo e sistemas
operacionais. A linguagem Rust também é fantástica para aplicações web: ela está por trás do site
do registro de pacotes do Rust, [crates.io]! Estamos curiosos para saber o que *você* fará com Rust.

[crates.io]: https://crates.io/

Este livro tem como público alvo um leitor que já sabe como programar em pelo menos
uma linguagem de programação. Após ler este livro, você deve se sentir confiante para
escrever programas em Rust. Ensinaremos Rust através de exemplos focados e pequenos,
que se complementam gradualmente para demonstrar o uso de vários da linguagem Rust, bem
como como eles funcionam “nos bastidores”.

## Sobre a Tradução

Esta é uma tradução *não oficial* da [nova versão](https://rust-lang.github.io/book) do livro  “The Rust Programming Language”.
Várias porções desta tradução (assim como do original) ainda estão incompletas. A
[versão antiga](https://doc.rust-lang.org/book) do livro (em inglês) ainda é a leitura
de referência recomendada da linguagem.

Sempre que possível, nos exemplos de código, optamos por usar nomes de variáveis, funções
e arquivos em português. Essa escolha foi feita para fins didáticos, e se limita às porções
de código de exemplo apresentadas. Os nomes provenientes da biblioteca padrão da linguagem,
bem como de *crates* já existentes são mantidos no original em inglês, para que o código
funcione corretamente. (Ex: `Usuario` ao invés de `User`, mas manteremos
nomes como `Box` em inglês) Para fins de compatibilidade, utilizaremos
somente caracteres sem acento e cedilha.

Apesar da escolha didática do livro, recomendamos que ao escrever código “de verdade” em Rust,
utilize sempre que possível nomes em inglês, especialmente para projetos
*open source* a fim de tornar seu código acessível para uma maior audiência.

## Contribuindo com o Livro

Este livro é *open source*. Se encontrar um erro, por favor não hesite em abrir uma *issue*
ou enviar um *pull request* [no GitHub].

[no GitHub]: https://github.com/rust-br/rust-book-pt-br
