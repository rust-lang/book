<!-- ia-translate: true -->
## Métodos {#methods}

Métodos são parecidos com funções: nós os declaramos com a palavra-chave `fn` e um nome, eles podem ter parâmetros e um valor de retorno, e contêm algum código que é executado quando o método é chamado a partir de outro lugar. Diferente das funções, métodos são definidos no contexto de um struct (ou de um enum ou de um trait object, que cobrimos no [Capítulo 6][enums]<!-- ignore --> e no [Capítulo 18][trait-objects]<!-- ignore -->, respectivamente), e o primeiro parâmetro deles é sempre `self`, que representa a instância do struct no qual o método está sendo chamado.

<!-- Old headings. Do not remove or links may break. -->

<a id="defining-methods"></a>

### Sintaxe de método {#method-syntax}

Vamos trocar a função `area`, que recebe uma instância de `Rectangle` como parâmetro, por um método `area` definido no struct `Rectangle`, como mostra a Listagem 5-13.

<Listing number="5-13" file-name="src/main.rs" caption="Definindo um método `area` no struct `Rectangle`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

</Listing>

Para definir a função no contexto de `Rectangle`, começamos um bloco `impl` (implementação) para `Rectangle`. Tudo o que estiver dentro desse bloco `impl` ficará associado ao tipo `Rectangle`. Depois, movemos a função `area` para dentro das chaves do `impl` e trocamos o primeiro (e, neste caso, único) parâmetro por `self`, tanto na assinatura quanto em todo o corpo. Em `main`, onde chamávamos a função `area` passando `rect1` como argumento, podemos usar a _sintaxe de método_ para chamar o método `area` na nossa instância de `Rectangle`. A sintaxe de método vem depois de uma instância: adicionamos um ponto, seguido do nome do método, parênteses e os argumentos, se houver.

Na assinatura de `area`, usamos `&self` no lugar de `rectangle: &Rectangle`. O `&self` é, na verdade, uma abreviação de `self: &Self`. Dentro de um bloco `impl`, o tipo `Self` é um alias para o tipo ao qual o bloco `impl` se refere. Métodos precisam ter como primeiro parâmetro um parâmetro chamado `self` do tipo `Self`, então o Rust permite abreviar isso apenas com o nome `self` na primeira posição. Observe que ainda precisamos usar o `&` na frente do `self` abreviado para indicar que o método pega a instância de `Self` emprestada (borrow), do mesmo jeito que fizemos com `rectangle: &Rectangle`. Métodos podem tomar o ownership de `self`, pegar `self` emprestado de forma imutável, como fizemos aqui, ou pegar `self` emprestado de forma mutável, assim como fazem com qualquer outro parâmetro.

Escolhemos `&self` aqui pelo mesmo motivo que usamos `&Rectangle` na versão com função: não queremos tomar o ownership, só queremos ler os dados do struct, não escrevê-los. Se quiséssemos mudar a instância na qual o método foi chamado como parte do que ele faz, usaríamos `&mut self` como primeiro parâmetro. Ter um método que toma o ownership da instância, usando apenas `self` como primeiro parâmetro, é raro; essa técnica costuma ser usada quando o método transforma `self` em outra coisa e você quer impedir quem chamou de usar a instância original depois da transformação.

O principal motivo para usar métodos em vez de funções, além de oferecer a sintaxe de método e de não precisar repetir o tipo de `self` na assinatura de cada método, é a organização. Colocamos tudo o que podemos fazer com uma instância de um tipo em um único bloco `impl`, em vez de obrigar quem usa nosso código a procurar as capacidades de `Rectangle` em vários lugares da biblioteca que fornecemos.

Observe que podemos dar a um método o mesmo nome de um dos campos do struct. Por exemplo, podemos definir um método em `Rectangle` que também se chama `width`:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-06-method-field-interaction/src/main.rs:here}}
```

</Listing>

Aqui, escolhemos fazer o método `width` retornar `true` se o valor do campo `width` da instância for maior que `0`, e `false` se o valor for `0`: podemos usar um campo dentro de um método com o mesmo nome para qualquer finalidade. Em `main`, quando colocamos parênteses depois de `rect1.width`, o Rust entende que nos referimos ao método `width`. Quando não usamos parênteses, o Rust entende que nos referimos ao campo `width`.

Muitas vezes, mas nem sempre, quando damos a um método o mesmo nome de um campo, queremos que ele apenas retorne o valor do campo e não faça mais nada. Métodos assim são chamados de _getters_, e o Rust não os implementa automaticamente para campos de struct, como algumas outras linguagens fazem. Getters são úteis porque você pode deixar o campo privado, mas o método público, e assim habilitar acesso somente leitura a esse campo como parte da API pública do tipo. Vamos discutir o que são público e privado, e como marcar um campo ou método como público ou privado, no [Capítulo 7][public]<!-- ignore -->.

> ### Cadê o operador `->`?
>
> Em C e C++, dois operadores diferentes são usados para chamar métodos: você usa `.` se estiver chamando um método diretamente no objeto, e `->` se estiver chamando o método em um ponteiro para o objeto e precisar desreferenciar o ponteiro primeiro. Em outras palavras, se `object` for um ponteiro, `object->something()` é parecido com `(*object).something()`.
>
> O Rust não tem um equivalente ao operador `->`; em vez disso, ele tem um recurso chamado _referenciamento e desreferenciamento automáticos_. Chamar métodos é um dos poucos lugares em Rust onde esse comportamento existe.
>
> Veja como funciona: quando você chama um método com `object.something()`, o Rust adiciona automaticamente `&`, `&mut` ou `*` para que `object` corresponda à assinatura do método. Em outras palavras, os dois códigos a seguir são iguais:
>
> <!-- CAN'T EXTRACT SEE BUG https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Point {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Point {
> #    fn distance(&self, other: &Point) -> f64 {
> #        let x_squared = f64::powi(other.x - self.x, 2);
> #        let y_squared = f64::powi(other.y - self.y, 2);
> #
> #        f64::sqrt(x_squared + y_squared)
> #    }
> # }
> # let p1 = Point { x: 0.0, y: 0.0 };
> # let p2 = Point { x: 5.0, y: 6.5 };
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
>
> O primeiro é bem mais limpo. Esse comportamento de referenciamento automático funciona porque métodos têm um receptor bem definido: o tipo de `self`. Dados o receptor e o nome de um método, o Rust consegue determinar com certeza se o método está lendo (`&self`), modificando (`&mut self`) ou consumindo (`self`) a instância. O fato de o Rust tornar o empréstimo implícito nos receptores de métodos é uma grande parte do que torna o ownership ergonômico na prática.

### Métodos com mais parâmetros {#methods-with-more-parameters}

Vamos praticar o uso de métodos implementando um segundo método no struct `Rectangle`. Desta vez, queremos que uma instância de `Rectangle` receba outra instância de `Rectangle` e retorne `true` se o segundo `Rectangle` couber completamente dentro de `self` (o primeiro `Rectangle`); caso contrário, deve retornar `false`. Ou seja, depois de definirmos o método `can_hold`, queremos poder escrever o programa mostrado na Listagem 5-14.

<Listing number="5-14" file-name="src/main.rs" caption="Usando o método `can_hold`, que ainda não foi escrito">

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```

</Listing>

A saída esperada seria a seguinte, porque as duas dimensões de `rect2` são menores que as de `rect1`, mas `rect3` é mais largo que `rect1`:

```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

Sabemos que queremos definir um método, então ele ficará dentro do bloco `impl Rectangle`. O nome do método será `can_hold`, e ele vai receber como parâmetro um empréstimo imutável de outro `Rectangle`. Podemos saber o tipo do parâmetro olhando o código que chama o método: `rect1.can_hold(&rect2)` passa `&rect2`, que é um empréstimo imutável de `rect2`, uma instância de `Rectangle`. Isso faz sentido porque só precisamos ler `rect2` (e não escrever, o que exigiria um empréstimo mutável), e queremos que `main` continue com o ownership de `rect2` para que possamos usá-lo de novo depois de chamar o método `can_hold`. O valor retornado por `can_hold` será um booleano, e a implementação vai verificar se a largura e a altura de `self` são maiores que a largura e a altura do outro `Rectangle`, respectivamente. Vamos adicionar o novo método `can_hold` ao bloco `impl` da Listagem 5-13, mostrado na Listagem 5-15.

<Listing number="5-15" file-name="src/main.rs" caption="Implementando o método `can_hold` em `Rectangle`, que recebe outra instância de `Rectangle` como parâmetro">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

</Listing>

Quando executamos este código com a função `main` da Listagem 5-14, obtemos a saída desejada. Métodos podem receber vários parâmetros, que adicionamos à assinatura depois do parâmetro `self`, e esses parâmetros funcionam exatamente como os parâmetros de funções.

### Funções associadas {#associated-functions}

Todas as funções definidas dentro de um bloco `impl` são chamadas de _funções associadas_, porque estão associadas ao tipo cujo nome vem depois do `impl`. Podemos definir funções associadas que não têm `self` como primeiro parâmetro (e, portanto, não são métodos), porque elas não precisam de uma instância do tipo para funcionar. Já usamos uma função assim: a função `String::from`, definida no tipo `String`.

Funções associadas que não são métodos costumam ser usadas como construtores que retornam uma nova instância do struct. Elas geralmente se chamam `new`, mas `new` não é um nome especial e não faz parte da linguagem. Por exemplo, poderíamos escrever uma função associada chamada `square` que receberia apenas um parâmetro de dimensão e o usaria como largura e altura ao mesmo tempo, facilitando a criação de um `Rectangle` quadrado sem precisar informar o mesmo valor duas vezes:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

As palavras-chave `Self` no tipo de retorno e no corpo da função são aliases para o tipo que aparece depois da palavra-chave `impl`, que neste caso é `Rectangle`.

Para chamar essa função associada, usamos a sintaxe `::` com o nome do struct; `let sq = Rectangle::square(3);` é um exemplo. Essa função fica dentro do namespace do struct: a sintaxe `::` é usada tanto para funções associadas quanto para namespaces criados por módulos. Vamos falar sobre módulos no [Capítulo 7][modules]<!-- ignore -->.

### Múltiplos blocos `impl` {#multiple-impl-blocks}

Cada struct pode ter vários blocos `impl`. Por exemplo, a Listagem 5-15 é equivalente ao código mostrado na Listagem 5-16, que coloca cada método em seu próprio bloco `impl`.

<Listing number="5-16" caption="Reescrevendo a Listagem 5-15 com múltiplos blocos `impl`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

</Listing>

Não há motivo para separar esses métodos em vários blocos `impl` aqui, mas essa sintaxe é válida. Veremos um caso em que vários blocos `impl` são úteis no Capítulo 10, onde discutimos tipos genéricos e traits.

## Resumo {#summary}

Structs permitem criar tipos personalizados que fazem sentido para o seu domínio. Usando structs, você mantém os dados relacionados conectados entre si e dá um nome a cada parte, para deixar o código claro. Nos blocos `impl`, você pode definir funções associadas ao seu tipo, e métodos são um tipo de função associada que permite especificar o comportamento das instâncias das suas structs.

Mas structs não são a única forma de criar tipos personalizados: vamos ver o recurso de enums do Rust para adicionar mais uma ferramenta à sua caixa de ferramentas.

[enums]: ch06-00-enums.html
[trait-objects]: ch18-02-trait-objects.md
[public]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
