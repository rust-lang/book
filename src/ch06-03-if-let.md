<!-- ia-translate: true -->
## Controle de fluxo conciso com `if let` e `let...else` {#concise-control-flow-with-if-let-and-letelse}

A sintaxe `if let` permite combinar `if` e `let` em uma forma menos verbosa de tratar valores que correspondem a um padrão, ignorando o restante. Considere o programa da Listagem 6-6, que faz `match` em um valor `Option<u8>` na variável `config_max`, mas só quer executar código se o valor for a variante `Some`.

<Listing number="6-6" caption="Um `match` que só se importa em executar código quando o valor é `Some`">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-06/src/main.rs:here}}
```

</Listing>

Se o valor for `Some`, imprimimos o valor dentro da variante `Some` vinculando-o à variável `max` no padrão. Não queremos fazer nada com o valor `None`. Para satisfazer a expressão `match`, precisamos adicionar `_ => ()` depois de processar apenas uma variante, o que é código repetitivo e chato de escrever.

Em vez disso, podemos escrever isso de forma mais curta usando `if let`. O código a seguir se comporta da mesma forma que o `match` da Listagem 6-6:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-12-if-let/src/main.rs:here}}
```

A sintaxe `if let` recebe um padrão e uma expressão separados por um sinal de igual. Ela funciona da mesma forma que um `match`, em que a expressão é passada para o `match` e o padrão é o primeiro braço. Neste caso, o padrão é `Some(max)`, e `max` se vincula ao valor dentro do `Some`. Podemos então usar `max` no corpo do bloco `if let` do mesmo jeito que usamos `max` no braço `match` correspondente. O código dentro do bloco `if let` só é executado se o valor corresponder ao padrão.

Usar `if let` significa menos digitação, menos indentação e menos código repetitivo. No entanto, você perde a verificação exaustiva que o `match` impõe, que garante que você não esqueça de tratar nenhum caso. Escolher entre `match` e `if let` depende do que você está fazendo na sua situação específica e de a concisão valer a pena em troca da verificação exaustiva.

Em outras palavras, você pode pensar em `if let` como açúcar sintático para um `match` que executa código quando o valor corresponde a um padrão e depois ignora todos os outros valores.

Podemos incluir um `else` com um `if let`. O bloco de código que acompanha o `else` é o mesmo que acompanharia o caso `_` na expressão `match` equivalente ao `if let` com `else`. Lembre da definição do enum `Coin` da Listagem 6-4, em que a variante `Quarter` também guardava um valor `UsState`. Se quiséssemos contar todas as moedas que não são de 25 centavos (_quarters_) e, ao mesmo tempo, anunciar o estado das moedas de 25 centavos, poderíamos fazer isso com uma expressão `match`, assim:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-13-count-and-announce-match/src/main.rs:here}}
```

Ou poderíamos usar uma expressão `if let` e `else`, assim:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-14-count-and-announce-if-let-else/src/main.rs:here}}
```

## Seguindo pelo “caminho feliz” com `let...else` {#staying-on-the-happy-path-with-letelse}

O padrão comum é executar algum cálculo quando um valor está presente e retornar um valor padrão caso contrário. Continuando com nosso exemplo de moedas com um valor `UsState`, se quiséssemos dizer algo engraçado dependendo de quão antigo era o estado da moeda de 25 centavos, poderíamos introduzir um método em `UsState` para verificar a idade de um estado, assim:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-07/src/main.rs:state}}
```

Depois, poderíamos usar `if let` para fazer `match` no tipo da moeda, introduzindo uma variável `state` dentro do corpo da condição, como na Listagem 6-7.

<Listing number="6-7" caption="Verificando se um estado existia em 1900 usando condicionais aninhados dentro de um `if let`">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-07/src/main.rs:describe}}
```

</Listing>

Isso resolve o problema, mas empurrou o trabalho para dentro do corpo da instrução `if let`, e, se o trabalho a ser feito fosse mais complicado, poderia ser difícil acompanhar exatamente como os ramos de nível superior se relacionam. Também poderíamos aproveitar o fato de que expressões produzem um valor para gerar o `state` a partir do `if let` ou para retornar antecipadamente, como na Listagem 6-8. (Você poderia fazer algo parecido com um `match`, também.)

<Listing number="6-8" caption="Usando `if let` para produzir um valor ou retornar antecipadamente">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-08/src/main.rs:describe}}
```

</Listing>

Mas isso também é um pouco chato de acompanhar, do seu jeito! Um dos ramos do `if let` produz um valor, e o outro retorna da função inteira.

Para tornar esse padrão comum mais fácil de expressar, o Rust tem `let...else`. A sintaxe `let...else` recebe um padrão do lado esquerdo e uma expressão do lado direito, muito parecida com `if let`, mas não tem um ramo `if`, apenas um ramo `else`. Se o padrão corresponder, ele vincula o valor do padrão no escopo externo. Se o padrão _não_ corresponder, o programa segue para o braço `else`, que precisa retornar da função.

Na Listagem 6-9, você pode ver como a Listagem 6-8 fica usando `let...else` no lugar de `if let`.

<Listing number="6-9" caption="Usando `let...else` para deixar mais claro o fluxo pela função">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-09/src/main.rs:describe}}
```

</Listing>

Perceba que, dessa forma, o código permanece no “caminho feliz” no corpo principal da função, sem ter um fluxo de controle significativamente diferente para dois ramos, como acontecia com o `if let`.

Se você tiver uma situação em que a lógica do programa fica verbosa demais para expressar com um `match`, lembre-se de que `if let` e `let...else` também estão na sua caixa de ferramentas do Rust.

## Resumo {#summary}

Vimos como usar enums para criar tipos personalizados que podem ser um de um conjunto de valores enumerados. Mostramos como o tipo `Option<T>` da biblioteca padrão ajuda você a usar o sistema de tipos para evitar erros. Quando valores de enum carregam dados internamente, você pode usar `match` ou `if let` para extrair e usar esses valores, dependendo de quantos casos precisa tratar.

Seus programas em Rust agora podem expressar conceitos do seu domínio usando structs e enums. Criar tipos personalizados para usar na sua API garante a segurança de tipos: o compilador vai garantir que suas funções só recebam valores do tipo que cada função espera.

Para oferecer uma API bem organizada aos seus usuários, simples de usar e que exponha exatamente o que eles vão precisar, vamos agora falar sobre os módulos do Rust.
