<!-- ia-translate: true -->
# Introdução {#introduction}

> Nota: Esta edição do livro é a mesma que [_The Rust Programming
> Language_][nsprust], disponível em formato impresso e e-book pela [No Starch
> Press][nsp].

[nsprust]: https://nostarch.com/rust-programming-language-3rd-edition
[nsp]: https://nostarch.com/

Bem-vindo a _The Rust Programming Language_, um livro introdutório sobre Rust.
A linguagem de programação Rust ajuda você a escrever software mais rápido e
mais confiável. Ergonomia de alto nível e controle de baixo nível costumam estar
em conflito no design de linguagens de programação; o Rust enfrenta esse
conflito. Equilibrando uma grande capacidade técnica e uma excelente experiência
para o desenvolvedor, o Rust dá a você a opção de controlar detalhes de baixo
nível (como o uso de memória) sem todo o trabalho tradicionalmente associado a
esse tipo de controle.

## Para Quem É o Rust {#who-rust-is-for}

O Rust é ideal para muitas pessoas, por uma variedade de motivos. Vamos ver
alguns dos grupos mais importantes.

### Equipes de Desenvolvedores {#teams-of-developers}

O Rust se mostra uma ferramenta produtiva para colaborar em equipes grandes de
desenvolvedores, com diferentes níveis de conhecimento em programação de
sistemas. Código de baixo nível está sujeito a vários bugs sutis, que na maioria
das outras linguagens só podem ser encontrados com muitos testes e revisões de
código cuidadosas feitas por desenvolvedores experientes. No Rust, o compilador
faz o papel de guardião, recusando-se a compilar código com esses bugs
difíceis de achar, inclusive bugs de concorrência. Trabalhando junto com o
compilador, a equipe pode concentrar seu tempo na lógica do programa em vez de
caçar bugs.

O Rust também traz ferramentas de desenvolvimento contemporâneas para o mundo da
programação de sistemas:

- O Cargo, o gerenciador de dependências e a ferramenta de build que vêm
  incluídos, tornam a adição, a compilação e o gerenciamento de dependências
  simples e consistentes em todo o ecossistema Rust.
- A ferramenta de formatação `rustfmt` garante um estilo de código consistente
  entre os desenvolvedores.
- O Rust Language Server oferece a integração com ambientes de desenvolvimento
  integrados (IDEs) para completar código e exibir mensagens de erro na própria
  linha.

Usando essas e outras ferramentas do ecossistema Rust, os desenvolvedores podem
ser produtivos ao escrever código em nível de sistema.

### Estudantes {#students}

O Rust é para estudantes e para quem tem interesse em aprender conceitos de
sistemas. Usando Rust, muitas pessoas aprenderam sobre temas como o
desenvolvimento de sistemas operacionais. A comunidade é muito acolhedora e
gosta de responder às perguntas dos estudantes. Por meio de iniciativas como
este livro, as equipes do Rust querem tornar os conceitos de sistemas mais
acessíveis a mais pessoas, especialmente a quem está começando a programar.

### Empresas {#companies}

Centenas de empresas, grandes e pequenas, usam Rust em produção para uma
variedade de tarefas, incluindo ferramentas de linha de comando, serviços web,
ferramentas de DevOps, dispositivos embarcados, análise e transcodificação de
áudio e vídeo, criptomoedas, bioinformática, mecanismos de busca, aplicações da
Internet das Coisas, aprendizado de máquina e até partes importantes do
navegador Firefox.

### Desenvolvedores de Código Aberto {#open-source-developers}

O Rust é para quem quer construir a própria linguagem de programação Rust, a
comunidade, as ferramentas de desenvolvimento e as bibliotecas. Adoraríamos
contar com você para contribuir com a linguagem Rust.

### Para Quem Valoriza Velocidade e Estabilidade {#people-who-value-speed-and-stability}

O Rust é para quem busca velocidade e estabilidade em uma linguagem. Por
velocidade, queremos dizer tanto a rapidez com que o código Rust pode ser
executado quanto a velocidade com que o Rust permite que você escreva programas.
As verificações do compilador do Rust garantem estabilidade ao longo da
adição de funcionalidades e de refatorações. Isso contrasta com o código legado
frágil de linguagens sem essas verificações, que os desenvolvedores muitas vezes
têm medo de modificar. Ao buscar abstrações de custo zero (funcionalidades de
alto nível que compilam para código de baixo nível tão rápido quanto o código
escrito manualmente), o Rust se esforça para que código seguro também seja
código rápido.

A linguagem Rust espera apoiar muitos outros usuários; os mencionados aqui são
apenas os maiores interessados. No geral, a maior ambição do Rust é eliminar as
concessões que os programadores aceitaram por décadas, oferecendo segurança
_e_ produtividade, velocidade _e_ ergonomia. Experimente o Rust e veja se as
escolhas dele funcionam para você.

## Para Quem É Este Livro {#who-this-book-is-for}

Este livro pressupõe que você já escreveu código em outra linguagem de
programação, mas não faz suposições sobre qual seja. Procuramos tornar o
conteúdo amplamente acessível a pessoas com as mais variadas formações em
programação. Não gastamos muito tempo falando sobre o que _é_ programar ou
sobre como pensar a respeito disso. Se você é totalmente novo em programação,
seria melhor ler um livro que apresente especificamente a programação.

## Como Usar Este Livro {#how-to-use-this-book}

Em geral, este livro pressupõe que você o lê em sequência, do começo ao fim.
Os capítulos posteriores se apoiam em conceitos dos capítulos anteriores, e os
capítulos anteriores podem não se aprofundar em um tema específico, mas
voltarão a ele em um capítulo posterior.

Você vai encontrar dois tipos de capítulos neste livro: capítulos de conceitos e
capítulos de projeto. Nos capítulos de conceitos, você vai aprender sobre um
aspecto do Rust. Nos capítulos de projeto, vamos construir pequenos programas
juntos, aplicando o que você aprendeu até ali. O Capítulo 2, o Capítulo 12 e o
Capítulo 21 são capítulos de projeto; os demais são capítulos de conceitos.

O **Capítulo 1** explica como instalar o Rust, como escrever um programa
“Hello, world!” e como usar o Cargo, o gerenciador de pacotes e ferramenta de
build do Rust. O **Capítulo 2** é uma introdução prática à escrita de um
programa em Rust, em que você constrói um jogo de adivinhação de números. Aqui,
abordamos os conceitos em alto nível, e os capítulos seguintes trazem mais
detalhes. Se você quer colocar a mão na massa logo de cara, o Capítulo 2 é o
lugar certo. Se você é um aprendiz especialmente meticuloso, que prefere
entender todos os detalhes antes de seguir adiante, pode querer pular o Capítulo
2 e ir direto para o **Capítulo 3**, que cobre funcionalidades do Rust
parecidas com as de outras linguagens de programação; depois, você pode voltar
ao Capítulo 2 quando quiser trabalhar em um projeto aplicando os detalhes que
aprendeu.

No **Capítulo 4**, você vai aprender sobre o sistema de ownership do Rust. O
**Capítulo 5** trata de structs e métodos. O **Capítulo 6** cobre enums,
expressões `match` e as construções de controle de fluxo `if let` e
`let...else`. Você vai usar structs e enums para criar tipos personalizados.

No **Capítulo 7**, você vai aprender sobre o sistema de módulos do Rust e sobre
as regras de privacidade para organizar seu código e a sua interface de
programação de aplicações (API) pública. O **Capítulo 8** discute algumas
estruturas de dados de coleção comuns que a biblioteca padrão oferece: vetores,
strings e mapas de hash. O **Capítulo 9** explora a filosofia e as técnicas
de tratamento de erros do Rust.

O **Capítulo 10** aprofunda genéricos, traits e lifetimes, que dão a você o poder
de definir código que se aplica a vários tipos. O **Capítulo 11** trata de testes,
que são necessários, mesmo com as garantias de segurança do Rust, para
assegurar que a lógica do seu programa está correta. No **Capítulo 12**,
vamos construir nossa própria implementação de um subconjunto das funcionalidades
da ferramenta de linha de comando `grep`, que procura texto dentro de arquivos.
Para isso, vamos usar muitos dos conceitos que discutimos nos capítulos
anteriores.

O **Capítulo 13** explora closures e iteradores: funcionalidades do Rust
que vêm de linguagens de programação funcional. No **Capítulo 14**, vamos
examinar o Cargo com mais profundidade e falar sobre boas práticas para
compartilhar suas bibliotecas com outras pessoas. O **Capítulo 15** trata dos
smart pointers que a biblioteca padrão oferece e dos traits que possibilitam
o funcionamento deles.

No **Capítulo 16**, vamos percorrer diferentes modelos de programação concorrente
e falar sobre como o Rust ajuda você a programar com várias threads sem medo.
No **Capítulo 17**, avançamos explorando a sintaxe `async` e `await` do Rust,
junto com tarefas, futures e streams, e o modelo de concorrência leve que eles
possibilitam.

O **Capítulo 18** analisa como as construções idiomáticas do Rust se comparam aos princípios da
programação orientada a objetos que você talvez já conheça. O **Capítulo 19** é
uma referência sobre padrões e pattern matching, que são formas poderosas de
expressar ideias em programas Rust. O **Capítulo 20** reúne um apanhado de
tópicos avançados de interesse, incluindo Rust `unsafe`, macros e mais sobre
lifetimes, traits, tipos, funções e closures.

No **Capítulo 21**, vamos concluir um projeto em que implementaremos um servidor
web multithread de baixo nível!

Por fim, alguns apêndices reúnem informações úteis sobre a linguagem em um
formato mais parecido com o de uma referência. O **Apêndice A** trata das
palavras-chave do Rust, o **Apêndice B** trata dos operadores e símbolos do
Rust, o **Apêndice C** trata dos traits deriváveis fornecidos pela biblioteca
padrão, o **Apêndice D** trata de algumas ferramentas de desenvolvimento úteis,
e o **Apêndice E** explica as edições do Rust. No **Apêndice F**, você
encontra traduções do livro, e no **Apêndice G** vamos falar sobre como o Rust
é feito e o que é o Rust nightly.

Não existe um jeito errado de ler este livro: se quiser pular adiante, vá em
frente! Você pode precisar voltar a capítulos anteriores se sentir alguma
confusão. Mas faça do seu jeito.

<span id="ferris"></span>

Uma parte importante do processo de aprender Rust é aprender a ler as mensagens
de erro que o compilador exibe: elas vão guiar você até um código que funciona.
Por isso, vamos fornecer muitos exemplos que não compilam, junto com a mensagem
de erro que o compilador vai mostrar em cada situação. Saiba que, se você
digitar e executar um exemplo qualquer, ele pode não compilar! Leia o texto ao
redor para ver se o exemplo que você está tentando executar deve mesmo dar erro.
Na maioria das situações, vamos levar você à versão correta de qualquer código
que não compile. O Ferris também vai ajudar você a distinguir os códigos que não
devem funcionar:

| Ferris                                                                                                           | Significado                                      |
| ---------------------------------------------------------------------------------------------------------------- | ------------------------------------------------ |
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain" alt="Ferris com um ponto de interrogação"/>    | Este código não compila!                         |
| <img src="img/ferris/panics.svg" class="ferris-explain" alt="Ferris levantando as duas garras"/>                  | Este código entra em pânico!                     |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain" alt="Ferris com uma garra levantada, dando de ombros"/> | Este código não produz o comportamento desejado. |

Na maioria das situações, vamos levar você à versão correta de qualquer código
que não compile.

## Código-fonte {#source-code}

Os arquivos-fonte a partir dos quais este livro é gerado podem ser encontrados no
[GitHub][book].

[book]: https://github.com/rust-lang/book/tree/main/src
