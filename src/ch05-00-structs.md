<!-- ia-translate: true -->
# Usando Structs para Estruturar Dados Relacionados {#using-structs-to-structure-related-data}

Um _struct_, ou _estrutura_, é um tipo de dado personalizado que permite agrupar
e nomear vários valores relacionados que formam um grupo com significado. Se
você conhece uma linguagem orientada a objetos, um struct é como os atributos de
dados de um objeto. Neste capítulo, vamos comparar e contrastar tuplas com
structs para construir sobre o que você já sabe e demonstrar quando structs são
uma forma melhor de agrupar dados.

Vamos mostrar como definir e instanciar structs. Vamos discutir como definir
funções associadas, especialmente o tipo de função associada chamada _método_,
para especificar o comportamento associado a um tipo struct. Structs e enums
(discutidos no Capítulo 6) são os blocos de construção para criar novos tipos no
domínio do seu programa, para aproveitar ao máximo a verificação de tipos em
tempo de compilação do Rust.
