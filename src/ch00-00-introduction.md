# Introdução

> Aviso: Esta edição do livro é a mesmo disponível na forma impressa em [The Rust Programming Language][nsprust].

[nsprust]: https://nostarch.com/rust-programming-language-3rd-edition
[nsp]: https://nostarch.com/

Bem-vindo (a) ao **O Livro da Linguagem de Programação Rust**, um livro introdutório sobre a linguagem Rust.
A linguagem de programação Rust ajuda a escrever softwares de forma mais rápida e legível. Une a facilidade de uma linguagem de alto nível e controle de linguagem de baixo nível, são coisas que entram em conflito em linguagens de programação. Rust desafia esse conflito com uma poderosa capacidade técnica e uma boa experiência de desenvolvimento, você tem a opção de controle de baixo nível (como uso de memória) sem toda a dificuldade associada a esse controle (uso de memória).

## Rust é para quem?

Rust é ideal para muitas pessoas por uma grande variedade de razões. Confira alguns grupos.

### Time de Desenvolvedores

Rust é uma ótima ferramenta para grandes times de desenvolvedores que colaboram entre si e possuí uma grande variedade de conhecimento em programação. Código de baixo nível é sensível a erros que na maioria das linguagens só é percebido com uma abundância de teste e revisão cuidadosa por desenvolvedores experientes. Em Rust, o compilador tem um grande e importante papel em não compilar códigos que possuem esses erros, desse modo a equipe pode dedicar tempo na lógica do programa e não em procurar _‘bugs’_.

Rust também traz ferramentas de desenvolvimentos atuais para programação de sistemas:

- Cargo é um gerenciador de dependências e ferramenta de compilação que torna o processo de adição, compilação e o gerenciamento de dependências tranquila e consistente através do ecosistema Rust.
- O `rustfmt` ferramenta de formatação de código (_linter_) que mantém o estilo de código padronizado entre os desenvolvedores.
- O _Rust Language Server_ um poderoso completator de código (_code completion_) e mostrador de menssagem de erro em tempo de escrita (_inline_) para ambientes de desenvolvimento integrado (em inglês: _IDE_).

Ao usar essas e outras ferramentas do ecosistema Rust, desenvolvedores podem ser mais produtivos enquanto escrevem códigos de baixo nível.

### Estudantes


Rust é para estudantes e quem está interessado em aprender sobre conceito de sistemas. Ao usar Rust, muitas pessoas aprendem sobre tópicos como desenvolvimento de sistemas operacionais, a comunidade é engajada em responder estudantes. 

Por meio de atitudes como esse livro, a equipe do Rust deseja tornar conceitos de sistema mais acessível a outras pessoas, especialmente os iniciantes em programação.

### Empresas

Centenas de empresas, grandes e pequenas usam Rust em produção para uma grande variedade de coisas como ferramentas de linha de comando, serviços _web_, _DevOps_, dispositivos embarcados, análise e transcrição de áudio e vídeo, criptomoedas, bioinformática, sistemas de pesquisas, aplicações de _internet_ das coisas, aprendizado de máquina e maior parte do navegador Firefox.

### Desenvolvedores de Código Aberto

Rust é para pessoas que querem ajudar construir a linguagem Rust, ferramentas para comunidade e bibliotecas. Contribua!

### Pessoas que valorizam velocidade e estabilidade

Rust é para pessoas que gostam de uma linguagem de programação que ofereça velocidade e estabilidade. Quando falo sobre velocidade quero dizer o quão rápido códigos Rust podem ser escritos e executados. O compilador do Rust verifica e garante estabilidade durante a adição e refatoração de funcionalidades. Isso é um diferencial em relação aos códigos legados que não realizam esse tipo de verificação e que os desenvolvedores geralmente têm medo de modificar. Rust faz com que um código seguro também seja rápido, ao fazer que recursos de alto nível compile para baixo nível como se fossem escritos manualmente.

A Linguagem Rust espera atender muitos outros usuários. Os mencionados, são apenas uma parte de grandes interessados. Sobretudo, Rust tem a ambição de eliminar as dificuldades que programadores tem aceitado há anos para fornecer segurança **e** produtividade, velocidade **e** ergonomia. Veja as opções que o Rust fornece para você.

## Para quem é esse livro

Este livro assume que você já escreveu códigos em outra linguagem de programação, não importa qual. Temos tentado fazer um material amigável e acessível para varios níveis de experiência em programação. Não queremos gastar muito tempo falando sobre o que é um programa, se você é totalmente novo em programação, você deveria dedicar um tempo lendo livros para ter uma introdução sobre programação.

## Orientações sobre como usar esse livro

De forma geral, esse livro espera que você leia em sequência do **capítulo 1** ao **apêndice**. Os capítulos posteriores desenvolvem assuntos dos capítulos anteriores, e os capítulos anteriores pode não desenvolver um tema específico, mas será reabordado o tema posteriormente.

Existem dois tipos de capítulos nesse livro: capítulos de conceito e capítulos de projeto.
Em conceitos, você irá aprender sobre características do Rust. Em projetos, nós iremos construir pequenos programas juntos para aplicar o que você aprendeu. O capítulo 2, capítulo 12 e capítulo 21 são projetos os restantes são conceitos.

**Capítulo 1** explica como instalar o Rust, como escrever o "Olá, mundo!", como usar o cargo (gerenciador de pacotes em Rust e ferramenta de compilação).

**Capítulo 2** é um "mão na massa" em que escrevemos um jogo de adivinhação em Rust. Assim, nós abordamos conceitos de alto nível e nos capítulos seguintes abordaremos detalhes. Se você quer diretamente "sujar as suas mãos" o capítulo é o lugar certo. Se você é mais detalhista e gosta de aprender todos os detalhes antes de seguir você deve pular o capítulo 2 e ir para o **Capítulo 3**, em que aborda funcionalidades do Rust que são similares a outras linguagens de programação e depois você retorna ao capítulo 2 para aplicar o que aprendeu.

**Capítulo4** você aprenderá sobre `ownership`.

**Capítulo 5** abordaremos `structs` e `methods`.

**Capítulo 6** abordaremos `enums`, `match` expressions e os controles de fluxo `if let` e `let...else`. Você irá usar `structs` e `enums` para construir tipos personalizados.

**Capítulo 7** você irá aprender sobre módulos em Rust e sobre regras de privacidade para organizar o seu código e API pública.

**Capítulo 8** discutiremos sobre estrutura de dados `collection`e bibliotecas padrões que fornecem: vetores, _strings_ e _hasp map_.

**Capítulo 9** exploraremos a filosofia e técnicas para tratamento de erro.

**Capítulo 10** começaremos a abordar `generics`,`traits` e `lifetime`(ciclo de vida), como isso te da poder de definir códigos que se aplica há vários tipos.

**Capítulo 11** é sobre testes e como Rust de forma segura garante que a lógica do seu programa esteja correta.

**Capítulo 12** vamos implementar uma funcionalidade da linha de comando, o `grep`, que busca texto em arquivos. Assim, usaremos muitos conceitos que discutímos nos capítulos anteriores.

**Capítulo 13** exploraremos `closures` e `interators`funcionalidades do Rust para programação funcional.

**Capítulo 14** iremos aprofundar no Cargo e falar sobre as melhores práticas para compartilhar as suas bibliotecas com outros desenvolvedores.

**Capítulo 15** conversaremos bibliotecas padrão que fornecem _smart pointers_.

**Capítulo 16** abordaremos diferentes modelos de processos concorrentes e falar como Rust te ajuda a construir multiplos processos.

**Capítulo 17** aqui abordaremos a sintaxe Rust para `async` e `await`, tarefas, funcionalidades, _streams_ e como ativar modelo de concorrência.

**Capítulo 18** iremos fazer comparações entre padrões Rust e programação orientada a objetos.

**Capítulo 19** é onde abordamos sobre _patterns_ (padrões) e _pattern matching_ (combinação de padrões) e como são poderosos em programas Rust.

**Capítulo 20** contém tópicos avançados como _unsafe_ (inseguro) Rust, macros e mais sobre ciclo de vida, traits, tipos, funções e closures.

**Capítulo 21** iremos completar um projeto de servidor web multithreaded!

Finalmente, alguns apêndices com informações importantes e mais referências.
**Apêndice A** aborda palavras-chave em Rust.

**Apêndice B** aborda operadores e símbolos.

**Apêndice C** aborda variações de traits fornecidas pela biblioteca padrão.

**Apêndice D** aborda ferramentas úteis de desenvolvimento.

**Apêndice E** explica as edições do Rust.

**Apêndice F** você pode encontrar traduções do livro.

**Apêndice G** aborda como faz e o que é uma versão _nightly_ do Rust.

Não tem um jeito errado de ler esse livro. Se você quer pular adiante, faça isso.
Futuramente, talvez, você tenha que voltar em capítulos anteriores. Enfim, faça o que for melhor para você.

<span id="ferris"></span>

Uma parte importante do processo de aprender Rust é aprender como ler mensagens de erro do compilador. Elas irão te ajudar a alcançar um código funcional. Forneceremos muitos exemplos que não compilam e o compilador irá mostrar  o erro para cada situação. Sabemos que se você digitar e executar um exemplo aleatório ele não irar compilar. Leia o erro. Na maioria das situações nos te mostraremos a versão correta de qualquer código que não compila. Ferris também pode te ajudar a entender o que são está funcionando.


| Ferris                                                                                                           | Meaning                                          |
| ---------------------------------------------------------------------------------------------------------------- | ------------------------------------------------ |
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain" alt="Ferris with a question mark"/>            | This code does not compile!                      |
| <img src="img/ferris/panics.svg" class="ferris-explain" alt="Ferris throwing up their hands"/>                   | This code panics!                                |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain" alt="Ferris with one claw up, shrugging"/> | This code does not produce the desired behavior. |

## Código fonte

Os arquivos desse livro podem ser encontrados no [GitHub][book].

[book]: https://github.com/rust-lang/book/tree/main/src
