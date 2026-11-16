<!-- ia-translate: true -->
## Definindo e instanciando structs {#defining-and-instantiating-structs}

Structs são parecidos com as tuplas, discutidas na seção [“O tipo tupla”][tuples]<!--
ignore -->, pois ambas guardam vários valores relacionados. Assim como nas tuplas, as partes de um struct podem ser de tipos diferentes. Diferentemente das tuplas, em um struct você dá nome a cada parte dos dados, para deixar claro o que cada valor significa. Esses nomes tornam os structs mais flexíveis que as tuplas: você não precisa depender da ordem dos dados para especificar ou acessar os valores de uma instância.

Para definir um struct, escrevemos a palavra-chave `struct` e damos nome ao struct inteiro. O nome de um struct deve descrever o significado dos dados agrupados. Depois, dentro de chaves, definimos os nomes e os tipos das partes dos dados, que chamamos de _campos_. Por exemplo, a Listagem 5-1 mostra um struct que armazena informações sobre uma conta de usuário.

<Listing number="5-1" file-name="src/main.rs" caption="Uma definição do struct `User`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-01/src/main.rs:here}}
```

</Listing>

Para usar um struct depois de defini-la, criamos uma _instância_ dela especificando valores concretos para cada um dos campos. Criamos uma instância informando o nome do struct e então adicionando chaves com pares _`key:
value`_, em que as chaves são os nomes dos campos e os valores são os dados que queremos armazenar neles. Não precisamos especificar os campos na mesma ordem em que os declaramos no struct. Em outras palavras, a definição do struct é como um modelo geral para o tipo, e as instâncias preenchem esse modelo com dados específicos para criar valores do tipo. Por exemplo, podemos declarar um usuário específico como mostrado na Listagem 5-2.

<Listing number="5-2" file-name="src/main.rs" caption="Criando uma instância do struct `User`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-02/src/main.rs:here}}
```

</Listing>

Para obter um valor específico de um struct, usamos a notação de ponto. Por exemplo, para acessar o endereço de e-mail desse usuário, usamos `user1.email`. Se a instância for mutável, podemos alterar um valor usando a notação de ponto e atribuindo a um campo específico. A Listagem 5-3 mostra como alterar o valor do campo `email` de uma instância mutável de `User`.

<Listing number="5-3" file-name="src/main.rs" caption="Alterando o valor do campo `email` de uma instância de `User`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-03/src/main.rs:here}}
```

</Listing>

Repare que a instância inteira precisa ser mutável; o Rust não permite marcar apenas alguns campos como mutáveis. Assim como em qualquer expressão, podemos construir uma nova instância do struct como a última expressão do corpo de uma função, para retorná-la implicitamente.

A Listagem 5-4 mostra uma função `build_user` que retorna uma instância de `User` com o e-mail e o nome de usuário informados. O campo `active` recebe o valor `true`, e `sign_in_count` recebe o valor `1`.

<Listing number="5-4" file-name="src/main.rs" caption="Uma função `build_user` que recebe um e-mail e um nome de usuário e retorna uma instância de `User`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-04/src/main.rs:here}}
```

</Listing>

Faz sentido dar aos parâmetros da função o mesmo nome dos campos do struct, mas ter que repetir os nomes dos campos `email` e `username` e das variáveis é um pouco cansativo. Se o struct tivesse mais campos, repetir cada nome ficaria ainda mais incômodo. Felizmente, existe um atalho prático!

<!-- Old headings. Do not remove or links may break. -->

<a id="using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name"></a>

### Usando o atalho de inicialização de campos {#using-the-field-init-shorthand}

Como os nomes dos parâmetros e os nomes dos campos do struct são exatamente iguais na Listagem 5-4, podemos usar a sintaxe de _atalho de inicialização de campos_ (_field init shorthand_) para reescrever `build_user` de modo que ela se comporte exatamente igual, mas sem a repetição de `username` e `email`, como mostrado na Listagem 5-5.

<Listing number="5-5" file-name="src/main.rs" caption="Uma função `build_user` que usa o atalho de inicialização de campos, porque os parâmetros `username` e `email` têm o mesmo nome dos campos do struct">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-05/src/main.rs:here}}
```

</Listing>

Aqui, estamos criando uma nova instância do struct `User`, que tem um campo chamado `email`. Queremos definir o valor do campo `email` como o valor do parâmetro `email` da função `build_user`. Como o campo `email` e o parâmetro `email` têm o mesmo nome, basta escrever `email` em vez de `email: email`.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-instances-from-other-instances-with-struct-update-syntax"></a>

### Criando instâncias com a sintaxe de atualização de struct {#creating-instances-with-struct-update-syntax}

Costuma ser útil criar uma nova instância de um struct que inclua a maior parte dos valores de outra instância do mesmo tipo, mas altere alguns deles. Você pode fazer isso usando a sintaxe de atualização de struct.

Primeiro, a Listagem 5-6 mostra como criar uma nova instância de `User` em `user2` do jeito normal, sem a sintaxe de atualização. Definimos um novo valor para `email`, mas mantemos os demais valores de `user1`, que criamos na Listagem 5-2.

<Listing number="5-6" file-name="src/main.rs" caption="Criando uma nova instância de `User` usando todos os valores de `user1`, exceto um">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-06/src/main.rs:here}}
```

</Listing>

Usando a sintaxe de atualização de struct, podemos obter o mesmo efeito com menos código, como mostrado na Listagem 5-7. A sintaxe `..` especifica que os campos restantes que não foram definidos explicitamente devem ter o mesmo valor dos campos da instância informada.

<Listing number="5-7" file-name="src/main.rs" caption="Usando a sintaxe de atualização de struct para definir um novo valor de `email` para uma instância de `User`, mas usando os demais valores de `user1`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-07/src/main.rs:here}}
```

</Listing>

O código da Listagem 5-7 também cria uma instância em `user2` que tem um valor diferente para `email`, mas os mesmos valores dos campos `username`, `active` e `sign_in_count` de `user1`. O `..user1` precisa vir por último para especificar que quaisquer campos restantes devem receber seus valores dos campos correspondentes de `user1`, mas podemos escolher definir valores para quantos campos quisermos, em qualquer ordem, independentemente da ordem dos campos na definição do struct.

Repare que a sintaxe de atualização de struct usa `=`, como uma atribuição; isso acontece porque ela move os dados, assim como vimos na seção [“Variáveis e dados interagindo com move”][move]<!-- ignore -->. Neste exemplo, não podemos mais usar `user1` depois de criar `user2`, porque a `String` do campo `username` de `user1` foi movida para `user2`. Se tivéssemos dado a `user2` novos valores de `String` para `email` e `username`, usando apenas os valores `active` e `sign_in_count` de `user1`, então `user1` ainda seria válida depois de criar `user2`. Tanto `active` quanto `sign_in_count` são tipos que implementam o trait `Copy`, então o comportamento que discutimos na seção [“Dados somente na stack: Copy”][copy]<!-- ignore --> se aplicaria. Também podemos continuar usando `user1.email` neste exemplo, porque o valor dele não foi movido para fora de `user1`.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-tuple-structs-without-named-fields-to-create-different-types"></a>

### Criando tipos diferentes com tuple structs {#creating-different-types-with-tuple-structs}

O Rust também oferece suporte a structs que se parecem com tuplas, chamadas de _tuple structs_. Os tuple structs têm o significado adicional que o nome do struct fornece, mas não têm nomes associados aos seus campos; têm apenas os tipos dos campos. Tuple structs são úteis quando você quer dar um nome à tupla inteira e fazer dela um tipo diferente de outras tuplas, e quando nomear cada campo, como em um struct comum, seria verboso ou redundante.

Para definir um tuple struct, comece com a palavra-chave `struct` e o nome do struct, seguidos pelos tipos da tupla. Por exemplo, aqui definimos e usamos dois tuple structs chamados `Color` e `Point`:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-01-tuple-structs/src/main.rs}}
```

</Listing>

Repare que os valores `black` e `origin` são de tipos diferentes, porque são instâncias de tuple structs diferentes. Cada struct que você define é seu próprio tipo, mesmo que os campos dentro do struct tenham os mesmos tipos. Por exemplo, uma função que recebe um parâmetro do tipo `Color` não pode receber um `Point` como argumento, mesmo que ambos os tipos sejam formados por três valores `i32`. Fora isso, instâncias de tuple structs são parecidos com tuplas: você pode desestruturá-las em suas partes individuais e pode usar um `.` seguido do índice para acessar um valor individual. Diferentemente das tuplas, os tuple structs exigem que você informe o nome do tipo do struct ao desestruturá-las. Por exemplo, escreveríamos `let Point(x, y, z) = origin;` para desestruturar os valores do ponto `origin` em variáveis chamadas `x`, `y` e `z`.

<!-- Old headings. Do not remove or links may break. -->

<a id="unit-like-structs-without-any-fields"></a>

### Definindo structs do tipo unidade {#defining-unit-like-structs}

Você também pode definir structs que não têm nenhum campo! Eles são chamados de _structs do tipo unidade_ (_unit-like structs_), porque se comportam de modo parecido com `()`, o tipo unidade que mencionamos na seção [“O tipo tupla”][tuples]<!-- ignore -->. Structs do tipo unidade podem ser úteis quando você precisa implementar um trait para algum tipo, mas não tem dados que queira armazenar no próprio tipo. Vamos discutir traits no Capítulo 10. Aqui está um exemplo de declaração e instanciação de um struct do tipo unidade chamado `AlwaysEqual`:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-04-unit-like-structs/src/main.rs}}
```

</Listing>

Para definir `AlwaysEqual`, usamos a palavra-chave `struct`, o nome que queremos e, então, um ponto e vírgula. Não há necessidade de chaves nem de parênteses! Depois, podemos obter uma instância de `AlwaysEqual` na variável `subject` de um jeito parecido: usando o nome que definimos, sem chaves nem parênteses. Imagine que, mais tarde, vamos implementar um comportamento para este tipo tal que cada instância de `AlwaysEqual` seja sempre igual a qualquer instância de qualquer outro tipo, talvez para ter um resultado conhecido em testes. Não precisaríamos de nenhum dado para implementar esse comportamento! Você verá no Capítulo 10 como definir traits e implementá-los em qualquer tipo, incluindo structs do tipo unidade.

> ### Propriedade dos dados do struct
>
> Na definição do struct `User` da Listagem 5-1, usamos o tipo `String` dono dos dados, em vez do tipo fatia de string `&str`. Essa é uma escolha deliberada, porque queremos que cada instância deste struct seja dona de todos os seus dados e que esses dados sejam válidos enquanto o struct inteiro for válido.
>
> Também é possível que structs armazenem referências a dados que pertencem a outra coisa, mas isso exige o uso de _lifetimes_, um recurso do Rust que vamos discutir no Capítulo 10. Lifetimes garantem que os dados referenciados por um struct sejam válidos enquanto o struct for válido. Suponha que você tente armazenar uma referência em um struct sem especificar lifetimes, como o seguinte em
> *src/main.rs*; isso não vai funcionar:
>
> <Listing file-name="src/main.rs">
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore,does_not_compile
> struct User {
>     active: bool,
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
> }
>
> fn main() {
>     let user1 = User {
>         active: true,
>         username: "someusername123",
>         email: "someone@example.com",
>         sign_in_count: 1,
>     };
> }
> ```
>
> </Listing>
>
> O compilador vai reclamar que precisa de especificadores de lifetime:
>
> ```console
> $ cargo run
>    Compiling structs v0.1.0 (file:///projects/structs)
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:3:15
>   |
> 3 |     username: &str,
>   |               ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 ~     username: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:4:12
>   |
> 4 |     email: &str,
>   |            ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 |     username: &str,
> 4 ~     email: &'a str,
>   |
>
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `structs` (bin "structs") due to 2 previous errors
> ```
>
> No Capítulo 10, vamos discutir como corrigir esses erros para que você possa armazenar referências em structs, mas, por enquanto, vamos corrigir erros como esses usando tipos donos, como `String`, em vez de referências, como `&str`.

<!-- manual-regeneration
for the error above
after running update-rustc.sh:
pbcopy < listings/ch05-using-structs-to-structure-related-data/no-listing-02-reference-in-struct/output.txt
paste above
add `> ` before every line -->

[tuples]: ch03-02-data-types.html#the-tuple-type
[move]: ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move
[copy]: ch04-01-what-is-ownership.html#stack-only-data-copy
