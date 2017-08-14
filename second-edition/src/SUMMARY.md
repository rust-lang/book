# A linguagem de programação Rust

## Começando

- [Introdução](ch01-00-introduction.md)
    - [Instalação](ch01-01-installation.md)
    - [Olá, Mundo!](ch01-02-hello-world.md)

- [Tutorial: Jogo de Adivinhação](ch02-00-guessing-game-tutorial.md)

- [Conceitos Comuns de Programação](ch03-00-common-programming-concepts.md)
    - [Variáveis e Mutabilidade](ch03-01-variables-and-mutability.md)
    - [Tipos de Dados](ch03-02-data-types.md)
    - [Como Funcionam as Funções](ch03-03-how-functions-work.md)
    - [Comentários](ch03-04-comments.md)
    - [Controle de Fluxo](ch03-05-control-flow.md)

- [Compreendendo o Ownership](ch04-00-understanding-ownership.md)
    - [O que é Ownership?](ch04-01-what-is-ownership.md)
    - [Referências & Borrowing](ch04-02-references-and-borrowing.md)
    - [Slices](ch04-03-slices.md)

- [Usando Struct para Estruturar Dados Relacionados](ch05-00-structs.md)
    - [Definindo e Instanciando Structs](ch05-01-defining-structs.md)
    - [Um Programa Exemplo Usando Structs](ch05-02-example-structs.md)
    - [Sintaxe de Método](ch05-03-method-syntax.md)

- [Enums e Pattern Matching](ch06-00-enums.md)
    - [Definindo um Enum](ch06-01-defining-an-enum.md)
    - [Operador de Controle de Fluxo `match`](ch06-02-match.md)
    - [Fluxo de controle Conciso com `if let`](ch06-03-if-let.md)

## Alfabetização Básica de Rust

- [Módulos](ch07-00-modules.md)
    - [`mod` e o Sistema de Arquivos](ch07-01-mod-and-the-filesystem.md)
    - [Controlando Visibilidade com `pub`](ch07-02-controlling-visibility-with-pub.md)
    - [Importando Nomes com `use`](ch07-03-importing-names-with-use.md)

- [Coleções Comuns](ch08-00-common-collections.md)
    - [Vetores](ch08-01-vectors.md)
    - [Strings](ch08-02-strings.md)
    - [Hash Maps](ch08-03-hash-maps.md)

- [Tratando Erros](ch09-00-error-handling.md)
    - [Erros Irrecuperáveis com `panic!`](ch09-01-unrecoverable-errors-with-panic.md)
    - [Erros Recuperáveis com `Result`](ch09-02-recoverable-errors-with-result.md)
    - [Entrar em `panic!` ou Não Entrar em `panic!`](ch09-03-to-panic-or-not-to-panic.md)

- [Tipos de Dados Genéricos, Traits, e Lifetimes](ch10-00-generics.md)
    - [Dados de Tipos Genéricos](ch10-01-syntax.md)
    - [Traits: Definindo Comportamento Compartilhado](ch10-02-traits.md)
    - [Validando Referências com  Lifetimes](ch10-03-lifetime-syntax.md)

- [Testes](ch11-00-testing.md)
    - [Escrevendo Testes](ch11-01-writing-tests.md)
    - [Executando Testes](ch11-02-running-tests.md)
    - [Organização de Teste](ch11-03-test-organization.md)

- [Projeto de I/O: Criando um Programa de Linha de Comando](ch12-00-an-io-project.md)
    - [Aceitando Argumentos da Linha de Comando](ch12-01-accepting-command-line-arguments.md)
    - [Lendo um arquivo](ch12-02-reading-a-file.md)
    - [Refatorando para Melhorar a Modularidade e o Tratamento de Erros](ch12-03-improving-error-handling-and-modularity.md)
    - [Desenvolvendo a Funcionalidade da Biblioteca com `Test Driven Development`](ch12-04-testing-the-librarys-functionality.md)
    - [Trabalhando com Variáveis de Ambiente](ch12-05-working-with-environment-variables.md)
    - [Escrevendo Mensagens de Erro Para `stderr` em Vez de `stdout`](ch12-06-writing-to-stderr-instead-of-stdout.md)

## Pensando em Rust

- [Funcionalidades de Linguagem Funcional em Rust](ch13-00-functional-features.md)
    - [Closures](ch13-01-closures.md)
    - [Iterators](ch13-02-iterators.md)
    - [Melhorando o Projeto de I/O](ch13-03-improving-our-io-project.md)
    - [Desempenho](ch13-04-performance.md)

- [Mais Sobre Cargo e Crates.io](ch14-00-more-about-cargo.md)
    - [Perfis de Lançamento](ch14-01-release-profiles.md)
    - [Publicando um Crate para Crates.io](ch14-02-publishing-to-crates-io.md)
    - [Cargo Workspaces](ch14-03-cargo-workspaces.md)
    - [Instalando Binários de Crates.io com `cargo install`](ch14-04-installing-binaries.md)
    - [Extendendo Cargo com Comandos Personalizados](ch14-05-extending-cargo.md)

- [Ponteiros Inteligentes](ch15-00-smart-pointers.md)
    - [`Box<T>` Aponta para dados na Heap e Possui Tamanho Conhecido](ch15-01-box.md)
    - [O `Deref` Trait Permite Acesso a Dados por Referência](ch15-02-deref.md)
    - [O `Drop` Trait Executa Código no Cleanup](ch15-03-drop.md)
    - [`Rc<T>`, o Contador de Referência para Ponteiros Inteligentes](ch15-04-rc.md)
    - [`RefCell<T>` e o Padrão de Mutação Interior](ch15-05-interior-mutability.md)
    - [Criar Ciclos de Referência e `Leaking Memory` é Seguro](ch15-06-reference-cycles.md)

- [Concorrência sem Medo](ch16-00-concurrency.md)
    - [Threads](ch16-01-threads.md)
    - [Passagem de Mensagem](ch16-02-message-passing.md)
    - [Estado Compartilhado](ch16-03-shared-state.md)
    - [Concorrência Extensível: `Sync` e `Send`](ch16-04-extensible-concurrency-sync-and-send.md)

- [Rust é uma Linguagem de Programação Orientada a Objetos?](ch17-00-oop.md)
    - [O que Significa Orientado a Objeto?](ch17-01-what-is-oo.md)
    - [Objetos `Trait` para Usar Valores de Diferentes Tipos](ch17-02-trait-objects.md)
    - [Implementações de Padrões de Design Orientados a Objetos](ch17-03-oo-design-patterns.md)

## Tópicos Avançados

- [Padrões Correspondentes à Estrutura dos Valores](ch18-00-patterns.md)
    - [Em Todos os Locais os Padrões Podem ser Usados](ch18-01-all-the-places-for-patterns.md)
    - [Refutabilidade: Se um Padrão Falha no Match](ch18-02-refutability.md)
    - [Toda a Sintaxe do Padrão](ch18-03-pattern-syntax.md)

- [Características Avançadas](ch19-00-advanced-features.md)
    - [Rust Inseguro](ch19-01-unsafe-rust.md)
    - [Lifetimes Avançados](ch19-02-advanced-lifetimes.md)
    - [Traits Avançados](ch19-03-advanced-traits.md)
    - [Tipos Avançados](ch19-04-advanced-types.md)
    - [Funções Avançadas & Closures](ch19-05-advanced-functions-and-closures.md)

- [Projeto Final: Criando um Servidor Web Multithreaded](ch20-00-final-project-a-web-server.md)
    - [Servidor Web de Uma Thread](ch20-01-single-threaded.md)
    - [Como Requisições Lentas Afetam a Taxa de Transferência](ch20-02-slow-requests.md)
    - [Projetando a Interface de Threads](ch20-03-designing-the-interface.md)
    - [Criando um Pool de Threads e as Guardando](ch20-04-storing-threads.md)
    - [Enviando Requisições Para Threads via Canais](ch20-05-sending-requests-via-channels.md)
    - [Shutdown e Cleanup de Modo Elegante](ch20-06-graceful-shutdown-and-cleanup.md)

- [Apêndice](appendix-00.md)
    - [A - Palavras-chave](appendix-01-keywords.md)
    - [B - Operadores](appendix-02-operators.md)
    - [C - Traits Deriváveis]()
    - [D - Nightly Rust]()
    - [E - Macros]()
    - [F - Traduções]()
    - [G - Novos recursos](appendix-07-newest-features.md)
