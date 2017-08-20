# Guessing Game
# Jogo de Adivinhação

Let’s learn some Rust! For our first project, we’ll implement a classic
beginner programming problem: the guessing game. Here’s how it works: Our
program will generate a random integer between one and a hundred. It will then
prompt us to enter a guess. Upon entering our guess, it will tell us if we’re
too low or too high. Once we guess correctly, it will congratulate us. Sounds
good?
Vamos aprender um pouco de Rust! Para nosso primeiro projeto, vamos
implementar um clássico problema de programação para iniciantes: o
jogo de adivinhação. Eis como ele funciona: nosso programa gerará um
inteiro aleatório entre um e cem. Ele então aguardará entrarmos um
palpite. Entrando nosso palpite, ele nos informará se estamos muito
baixos ou muito altos. Uma vez que palpitemos corretamente, ele nos
congratulará. Parece uma boa?

Along the way, we’ll learn a little bit about Rust. The next chapter, ‘Syntax
and Semantics’, will dive deeper into each part.
Ao longo do caminho, aprenderemos um pouco sobre Rust. No capítulo
seguinte, 'Sintaxe e Semântica', mergulharemos mais fundo em cada
parte.

# Set up
# Configurando

Let’s set up a new project. Go to your projects directory. Remember how we had
to create our directory structure and a `Cargo.toml` for `hello_world`? Cargo
has a command that does that for us. Let’s give it a shot:
Vamos configurar um novo projeto, Vá ao seu diretório de
projetos. Lembra como tínhamos que criar nossa estrutura de diretórios
e um `Cargo.toml` para o `hello_world`? Cargo tem um comando que faz
isto por nós. Vamos dar uma olhada:

```bash
$ cd ~/projects
$ cargo new guessing_game --bin
     Created binary (application) `guessing_game` project
$ cd guessing_game
```

We pass the name of our project to `cargo new`, and then the `--bin` flag,
since we’re making a binary, rather than a library.
Passamos o nome de nosso projeto para `cargo new`, e então a opção de
comando `--bin`, já que estamos fazendo um binário em vez de uma
biblioteca.

Check out the generated `Cargo.toml`:
Confira o arquivo gerado `Cargo.toml`:

```toml
[package]

name = "guessing_game"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
```

Cargo gets this information from your environment. If it’s not correct, go ahead
and fix that.
Cargo obtém essa informação a partir de seu ambiente. Se ela não
estiver correta, vá em frente e faça os reparos necessários.

Finally, Cargo generated a ‘Hello, world!’ for us. Check out `src/main.rs`:
Finalmente, Cargo gerou um ‘Hello, world!’ para nós. Confira em `src/main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

Let’s try compiling what Cargo gave us:
Vamos tentar compilar o que o Cargo nos forneceu:

```{bash}
$ cargo build
   Compiling guessing_game v0.1.0 (file:///home/you/projects/guessing_game)
    Finished debug [unoptimized + debuginfo] target(s) in 0.53 secs
```

Excellent! Open up your `src/main.rs` again. We’ll be writing all of
our code in this file.
Excelente! Abra novamente o arquivo `src/main.rs`. Escreveremos todo
nosso código nele.

Remember the `run` command from last chapter? Try it out again here:
Lembram do comando `run` do capítulo anterior? Teste-o novamente aqui:

```bash
$ cargo run
   Compiling guessing_game v0.1.0 (file:///home/you/projects/guessing_game)
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/guessing_game`
Hello, world!
```

Great! Our game is just the kind of project `run` is good for: we need
to quickly test each iteration before moving on to the next one.
Muito bom! Nosso jogo é exatamente o tipo de projeto para o qual `run`
vem a calhar: nós precisamos testar rapidamente cada iteração antes de
movermos para a próxima.

# Processing a Guess
# Processando o Palpite

Let’s get to it! The first thing we need to do for our guessing game is
allow our player to input a guess. Put this in your `src/main.rs`:
Vamos lá! A primeira coisa que precisamos fazer no nosso jogo de
adivinhação é permitir que o jogador entre com um palpite. Escreva
isso no arquivo `src/main.rs`:

```rust,no_run
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

There’s a lot here! Let’s go over it, bit by bit.
Tem bastante coisa aqui! Vamos passar por este código, uma parte de
cada vez:

```rust,ignore
use std::io;
```

We’ll need to take user input, and then print the result as output. As such, we
need the `io` library from the standard library. Rust only imports a few things
by default into every program, [the ‘prelude’][prelude]. If it’s not in the
prelude, you’ll have to `use` it directly. There is also a second ‘prelude’, the
[`io` prelude][ioprelude], which serves a similar function: you import it, and it
imports a number of useful, `io`-related things.
Precisaremos obter entrada do usuário, e então imprimir o resultado
como saída. Para tal, precisaremos da biblioteca padrão `io`. Rust
importa apenas algumas poucas coisas por padrão em cada programa, [o
'prelúdio'][prelude]. Se algo não está no prelúdio, você tem que
importá-lo diretamente via `use`. Existe também um segundo 'prelúdio',
o [prelúdio `io`][ioprelude], que serve a uma função similar: você o
importa e ele importa algumas coisas importantes relacionadas a E/S.

[prelude]: ../../std/prelude/index.html
[ioprelude]: ../../std/io/prelude/index.html

```rust,ignore
fn main() {
```

As you’ve seen before, the `main()` function is the entry point into your
program. The `fn` syntax declares a new function, the `()`s indicate that
there are no arguments, and `{` starts the body of the function. Because
we didn’t include a return type, it’s assumed to be `()`, an empty
[tuple][tuples].
Como você já deve ter visto antes, a função `main()` é o ponto de
entrada de seu programa. A sintaxe `fn` declara uma nova função, o
`()` indica que não há argumentos, e o `{` inicia o corpo da
função. Como não incluímos um tipo de retorno, este tipo é assumido
como `()`, uma [tupla][tuples] vazia.

[tuples]: primitive-types.html#tuples

```rust,ignore
   println!("Guess the number!");

   println!("Please input your guess.");
```

We previously learned that `println!()` is a [macro][macros] that
prints a [string][strings] to the screen.
Já aprendemos antes que `println!()` é uma [macro][macros] que imprime
uma [string][strings] na tela.

[macros]: macros.html
[strings]: strings.html

```rust,ignore
    let mut guess = String::new();
```

Now we’re getting interesting! There’s a lot going on in this little line.
The first thing to notice is that this is a [let statement][let], which is
used to create ‘variable bindings’. They take this form:
Agora está ficando interessante! Tem muita coisa acontecendo nesta
pequena linha. A primeira coisa é notar que este é uma [declaração
let][let], usada para criar 'vinculação de variáveis'. Declarações
`let` tomam este formato:

```rust,ignore
let foo = bar;
```

[let]: variable-bindings.html

This will create a new binding named `foo`, and bind it to the value `bar`. In
many languages, this is called a ‘variable’, but Rust’s variable bindings have
a few tricks up their sleeves.
Isto cria um novo vínculo de nome `foo`, e o vincula ao valor
`bar`. Em muitas linguagens isto é chamado de 'variável', mas as
vinculações de variáveis em Rust têm alguns truques nas mangas.

For example, they’re [immutable][immutable] by default. That’s why our example
uses `mut`: it makes a binding mutable, rather than immutable. `let` doesn’t
take a name on the left hand side of the assignment, it actually accepts a
‘[pattern][patterns]’. We’ll use patterns later. It’s easy enough
to use for now:
Por exemplo, elas são [imutáveis][immutable] por padrão. É por isso
que nosso exemplo usa `mut`: ele torna o vínculo mutável, em vez de
imutável. `let`não toma um nome do lado esquerdo da atribuição, ele na
realidade aceita um '[padrão][patterns]'. Nós usaremos padrões mais
tarde. Isto é fácil o suficiente para usar por enquanto:

```rust
let foo = 5; // `foo` is immutable.
let mut bar = 5; // `bar` is mutable.
let foo = 5; // `foo` é imutável
let mut bar = 5; // `bar` é mutável
```

[immutable]: mutability.html
[patterns]: patterns.html

Oh, and `//` will start a comment, until the end of the line. Rust ignores
everything in [comments][comments].
Ah! E o `//` inicia um comentário, até o fim da linha. Rust ignora
tudo nos [comentários][comments].

[comments]: comments.html

So now we know that `let mut guess` will introduce a mutable binding named
`guess`, but we have to look at the other side of the `=` for what it’s
bound to: `String::new()`.
Agora então sabemos que `let mut guess` introduzirá um vínculo mutável
de nome `guess`, mas temos que observar o outro lado do `=` pelo que
ele está vinculado: `String::new()`.

`String` is a string type, provided by the standard library. A
[`String`][string] is a growable, UTF-8 encoded bit of text.
`String` é um tipo string (grosso modo, uma cadeia de caracteres),
fornecido pela biblioteca padrão. Uma [`String`][string] é uma parcela
de texto encodada em UTF-8, que pode modificar de tamanho.

[string]: ../../std/string/struct.String.html

The `::new()` syntax uses `::` because this is an ‘associated function’ of
a particular type. That is to say, it’s associated with `String` itself,
rather than a particular instance of a `String`. Some languages call this a
‘static method’.
A sintaxe `::new()` usa `::` porque esta é uma 'função associada' de
um tipo particular. Quer dizer, é associada ao próprio tipo `String`
em vez de uma instância particular de  `String`. Algumas linguagens
dão a isso o nome 'método estático'.

This function is named `new()`, because it creates a new, empty `String`.
You’ll find a `new()` function on many types, as it’s a common name for making
a new value of some kind.
Esta função é chamada `new()` porque ela cria uma nova `String`
vazia. Você encontrará uma função `new()` em muitos tipos, já que ela
é um nome comum para produzir um novo valor de algum tipo.

Let’s move forward:
Prossigamos:

```rust,ignore
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
```

That’s a lot more! Let’s go bit-by-bit. The first line has two parts. Here’s
the first:
Aqui tem bem mais coisa! Vamos prosseguir uma parte por vez. A
primeira linha tem duas partes; eis a primeira:

```rust,ignore
io::stdin()
```

Remember how we `use`d `std::io` on the first line of the program? We’re now
calling an associated function on it. If we didn’t `use std::io`, we could
have written this line as `std::io::stdin()`.
Lembra como usamos o `std::io` na primeira linha do programa? Agora
estamos chamando uma função associada nele. Se não usássemos o `use
std::io`, teríamos que escrever esta linha como `std::io::stdin()`.

This particular function returns a handle to the standard input for your
terminal. More specifically, a [std::io::Stdin][iostdin].
Esta função em particular retorna um manipulador (*handle*) para a
entrada padrão do seu terminal. Mais especificamente, um
[std::io::Stdin][iostdin].

[iostdin]: ../../std/io/struct.Stdin.html

The next part will use this handle to get input from the user:
A próxima parte usará esse *handle* para obter entrada do usuário:

```rust,ignore
.read_line(&mut guess)
```

Here, we call the [`read_line`] method on our handle.
[Methods][method] are like associated functions, but are only available on a
particular instance of a type, rather than the type itself. We’re also passing
one argument to `read_line()`: `&mut guess`.
Aqui, nós chamamos o método [`read_line`] do nosso
*handle*. [Métodos][method] são como funções associadas, mas só estão
disponíveis em uma instância particular de um tipo, em vez de no
próprio tipo. Também estamos passando um argumento para `read_line()`:
`&mut guess`.

[`read_line`]: ../../std/io/struct.Stdin.html#method.read_line
[method]: method-syntax.html

Remember how we bound `guess` above? We said it was mutable. However,
`read_line` doesn’t take a `String` as an argument: it takes a `&mut String`.
Rust has a feature called ‘[references][references]’, which allows you to have
multiple references to one piece of data, which can reduce copying. References
are a complex feature, as one of Rust’s major selling points is how safe and
easy it is to use references. We don’t need to know a lot of those details to
finish our program right now, though. For now, all we need to know is that
like `let` bindings, references are immutable by default. Hence, we need to
write `&mut guess`, rather than `&guess`.

Why does `read_line()` take a mutable reference to a string? Its job is
to take what the user types into standard input, and place that into a
string. So it takes that string as an argument, and in order to add
the input, it needs to be mutable.
Lembra como vinculamos `guess` acima? Dissemos que ele era
mutável. Porém, `read_line` não toma um `String` como argumento; ele
toma um `&mut String`.

Rust tem uma característica chamada  ‘[referência][references]’, que
permite que tenhamos múltiplas referências a uma peça de dado, o que
pode reduzir as cópias. Referências são uma característica complexa,
já que uma das maiores vantagens de Rust é o quão seguro e fácil é
usar referências. Porém, não precisamos conhecer muitos desses detalhes para
finalizar nosso programa agora. Por enquanto, tudo que precisamos
saber é que como as vinculações `let`, referências são imutáveis por
padrão. Por isso, precisamos escrever `&mut guess`, em vez de
`&guess`.

Por que `read_line()` toma uma referência mutável a uma string? O
trabalho de `read_line()` é tomar o que o usuário escreve na entrada
padrão, e colocar isso numa string. Então ele toma uma string como
argumento, e para adicionar a entrada, ela precisa ser mutável.

[references]: references-and-borrowing.html

But we’re not quite done with this line of code, though. While it’s
a single line of text, it’s only the first part of the single logical line of
code:
Mas não finalizamos completamente com esta linha de código,
porém. Enquanto esta é uma simples linha de texto, é apenas a primeira
parte de uma linha lógica de código:

```rust,ignore
        .expect("Failed to read line");
```

When you call a method with the `.foo()` syntax, you may introduce a newline
and other whitespace. This helps you split up long lines. We _could_ have
done:
Quando você chama um método com a sintaxe `.foo()`, você pode
introduzir uma nova linha e outro espaço em branco. Isto te ajuda a
dividir linhas compridas. Poderíamos ter feito assim:

```rust,ignore
    io::stdin().read_line(&mut guess).expect("Failed to read line");
```

But that gets hard to read. So we’ve split it up, two lines for two method
calls. We already talked about `read_line()`, but what about `expect()`? Well,
we already mentioned that `read_line()` puts what the user types into the `&mut
String` we pass it. But it also returns a value: in this case, an
[`io::Result`][ioresult]. Rust has a number of types named `Result` in its
standard library: a generic [`Result`][result], and then specific versions for
sub-libraries, like `io::Result`.
Mas isto fica difícil de ler. Então dividimos a linha, duas linhas
para duas chamadas de métodos. Já comentamos sobre `read_line()`, mas
e quanto ao `expect()`? Bem, nós já mencionamos que `read_line()`
coloca o que o usuário escreve no `&mut String` que passamos. Mas ela
também retorna um valor; neste caso, um [`io::Result`][ioresult]. Rust
tem uma quantidade de tipos com o nome `Result` em sua biblioteca
padrão: um [`Result`][result] genérico, e daí as versões específicas
de sub-bibliotecas, como `io::Result`.

[ioresult]: ../../std/io/type.Result.html
[result]: ../../std/result/enum.Result.html

The purpose of these `Result` types is to encode error handling information.
Values of the `Result` type, like any type, have methods defined on them. In
this case, `io::Result` has an [`expect()` method][expect] that takes a value
it’s called on, and if it isn’t a successful one, [`panic!`][panic]s with a
message you passed it. A `panic!` like this will cause our program to crash,
displaying the message.
O propósito destes tipos `Result` é encodar informação de manipulação
de erros. Valores do tipo `Result`, como qualquer tipo, têm métodos
definidos para eles. Neste caso, `io::Result` tem um método
[`expect()`][expect] que toma o valor que foi chamado, e se ele não é
um valor bem-sucedido, cai em pânico ([`panic!`][panic]) com a
mensagem que você passou. Um `panic!` deste fará o programa 'quebrar',
exibindo a mensagem.

[expect]: ../../std/result/enum.Result.html#method.expect
[panic]: error-handling.html

If we do not call `expect()`, our program will compile, but
we’ll get a warning:
Se não chamarmos `expect()`, nosso programa compilará, mas obteremos
um alerta:

```bash
$ cargo build
   Compiling guessing_game v0.1.0 (file:///home/you/projects/guessing_game)
warning: unused result which must be used, #[warn(unused_must_use)] on by default
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^

    Finished debug [unoptimized + debuginfo] target(s) in 0.42 secs
```

Rust warns us that we haven’t used the `Result` value. This warning comes from
a special annotation that `io::Result` has. Rust is trying to tell you that
you haven’t handled a possible error. The right way to suppress the error is
to actually write error handling. Luckily, if we want to crash if there’s
a problem, we can use `expect()`. If we can recover from the
error somehow, we’d do something else, but we’ll save that for a future
project.
Rust alerta que não usamos o valor `Result`. Este alerta vem de uma
anotação especial que `io::Result` tem. Rust está tentando te avisar
que você não lidou com um possível erro. A maneira correta de suprimir
o erro é realmente escrevendo o manipulador de erro. Por sorte, se
queremos quebrar o programa acaso ocorra um problema, podemos usar
`expect()`. Se podemos de alguma forma recuperar do erro, podemos
fazer outra coisa, mas guardaremos isso para um projeto futuro.

There’s only one line of this first example left:
Há apenas uma linha sobrando neste primeiro exemplo:

```rust,ignore
    println!("You guessed: {}", guess);
}
```

This prints out the string we saved our input in. The `{}`s are a placeholder,
and so we pass it `guess` as an argument. If we had multiple `{}`s, we would
pass multiple arguments:
Esta imprime a string na qual salvamos nossa entrada. O `{}` é um
gabarito, e assim podemos passar `guess` como argumento. Se tivermos
múltiplos `{}`, poderíamos passar múltiplos argumentos:

```rust
let x = 5;
let y = 10;

println!("x and y: {} and {}", x, y);
```

Easy.
Fácil.

Anyway, that’s the tour. We can run what we have with `cargo run`:
De qualquer forma, este é um *tour*. Podemos rodar o que temos com `cargo run`:

```bash
$ cargo run
   Compiling guessing_game v0.1.0 (file:///home/you/projects/guessing_game)
    Finished debug [unoptimized + debuginfo] target(s) in 0.44 secs
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```

All right! Our first part is done: we can get input from the keyboard,
and then print it back out.
Tudo certo! Nossa primeira parte está feita: podemos coletar entrada
do teclado, e daí imprimi-lo de volta.

# Generating a secret number
# Gerando um número secreto

Next, we need to generate a secret number. Rust does not yet include random
number functionality in its standard library. The Rust team does, however,
provide a [`rand` crate][randcrate]. A ‘crate’ is a package of Rust code.
We’ve been building a ‘binary crate’, which is an executable. `rand` is a
‘library crate’, which contains code that’s intended to be used with other
programs.
A seguir, precisamos gerar um número secreto. Rust ainda não inclui
uma funcionalidade de geração de números aleatórios em sua biblioteca
padrão. Porém, o time Rust fornece uma [*crate* `rand`][randcrate]. Um
*crate* (caixote) é um pacote de código Rust. Nós estamos construindo
um *crate* binário, que é um executável. Já `rand` é um *crate*
biblioteca, que contém código cujo objetivo é ser usado por outros
programas.

[randcrate]: https://crates.io/crates/rand

Using external crates is where Cargo really shines. Before we can write
the code using `rand`, we need to modify our `Cargo.toml`. Open it up, and
add these few lines at the bottom:
É no uso de *crates* externas que Cargo realmente brilha. Antes que
possamos escrever o código usando `rand`, precisamos modificar nosso
`Cargo.toml`. Abra-o, e adicione estas linhas no final:

```toml
[dependencies]

rand = "0.3.0"
```

The `[dependencies]` section of `Cargo.toml` is like the `[package]` section:
everything that follows it is part of it, until the next section starts.
Cargo uses the dependencies section to know what dependencies on external
crates you have, and what versions you require. In this case, we’ve specified version `0.3.0`,
which Cargo understands to be any release that’s compatible with this specific version.
Cargo understands [Semantic Versioning][semver], which is a standard for writing version
numbers. A bare number like above is actually shorthand for `^0.3.0`,
meaning "anything compatible with 0.3.0".
If we wanted to use only `0.3.0` exactly, we could say `rand = "=0.3.0"`
(note the two equal signs).
We could also use a range of versions.
[Cargo’s documentation][cargodoc] contains more details.
A seção `[dependencies]` do arquivo `Cargo.toml` é como a seção
`[package]`: tudo que a segue é parte dela, até a próxima seção
iniciar. Cargo usa a seção de dependências para saber que dependências
em *crates* externas você tem, e que versões você exige. Neste caso,
especificamos a versão `0.3.0`, a qual Cargo entende como qualquer
versão que seja compatível com esta em específico. Cargo compreende
[versões semânticas][semver], um padrão para escrever números de
versão. Um número limpo como este é na verdade uma forma curta de
escrever `^0.3.0`, o que significa "qualquer uma compatível com
0.3.0". Se quiséssemos usar somente a versão `0.3.0` exatamente,
poderíamos ter escrito  `rand = "=0.3.0"` (note os dois sinais de
igual). Poderíamos também usar um intervalo de versões. A [documentação
do Cargo ][cargodoc] contém mais detalhes.

[semver]: http://semver.org
[cargodoc]: http://doc.crates.io/specifying-dependencies.html

Now, without changing any of our code, let’s build our project:
Agora, sem mudar código algum, vamos montar nosso projeto:

```bash
$ cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading rand v0.3.14
 Downloading libc v0.2.17
   Compiling libc v0.2.17
   Compiling rand v0.3.14
   Compiling guessing_game v0.1.0 (file:///home/you/projects/guessing_game)
    Finished debug [unoptimized + debuginfo] target(s) in 5.88 secs
```

(You may see different versions, of course.)
(É claro, você pode acabar vendo versões diferentes.)

Lots of new output! Now that we have an external dependency, Cargo fetches the
latest versions of everything from the registry, which is a copy of data from
[Crates.io][cratesio]. Crates.io is where people in the Rust ecosystem
post their open source Rust projects for others to use.
Muita saída nova! Agora que temos uma dependência externa nova, Cargo
busca as versões mais recentes de tudo no registro, o qual é uma cópia
de dados do [Crates.io][cratesio]. Crates.io é onde as pessoas do
ecossistema Rust postam seus projetos *opensource* para que os outros
usem.

[cratesio]: https://crates.io

After updating the registry, Cargo checks our `[dependencies]` and downloads
any we don’t have yet. In this case, while we only said we wanted to depend on
`rand`, we’ve also grabbed a copy of `libc`. This is because `rand` depends on
`libc` to work. After downloading them, it compiles them, and then compiles
our project.
Após atualizar o registro, Cargo confere as dependências em
`[dependencies]` e baixa as que ainda não foram baixadas. Neste caso,
enquanto nós apenas dizemos que queremos `rand` como dependência, nós
também pegamos uma cópia da `libc`. Isto é porque `rand` depende de
`libc` para funcionar. Depois de baixá-las, o Cargo as compila e
então compila nosso projeto.

If we run `cargo build` again, we’ll get different output:
Se executarmos `cargo build` novamente, obteremos uma saída diferente:

```bash
$ cargo build
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
```

That’s right, nothing was done! Cargo knows that our project has been built, and that
all of its dependencies are built, and so there’s no reason to do all that
stuff. With nothing to do, it simply exits. If we open up `src/main.rs` again,
make a trivial change, and then save it again, we’ll only see two lines:
Isso mesmo, nada foi feito! Cargo sabe que nosso projeto foi
construído, e que todas as dependências foram montadas, e portanto não
há razão para refazer tudo. Sem nada a fazer, ele simplesmente sai. Se
abríssemos `src/main.rs` novamente, e fizéssemos uma modificação
trivial, veríamos apenas duas linhas:

```bash
$ cargo build
   Compiling guessing_game v0.1.0 (file:///home/you/projects/guessing_game)
    Finished debug [unoptimized + debuginfo] target(s) in 0.45 secs
```

So, we told Cargo we wanted any `0.3.x` version of `rand`, and so it fetched the latest
version at the time this was written, `v0.3.14`. But what happens when next
week, version `v0.3.15` comes out, with an important bugfix? While getting
bugfixes is important, what if `0.3.15` contains a regression that breaks our
code?

The answer to this problem is the `Cargo.lock` file you’ll now find in your
project directory. When you build your project for the first time, Cargo
figures out all of the versions that fit your criteria, and then writes them
to the `Cargo.lock` file. When you build your project in the future, Cargo
will see that the `Cargo.lock` file exists, and then use that specific version
rather than do all the work of figuring out versions again. This lets you
have a repeatable build automatically. In other words, we’ll stay at `0.3.14`
until we explicitly upgrade, and so will anyone who we share our code with,
thanks to the lock file.

What about when we _do_ want to use `v0.3.15`? Cargo has another command,
`update`, which says ‘ignore the lock, figure out all the latest versions that
fit what we’ve specified. If that works, write those versions out to the lock
file’. But, by default, Cargo will only look for versions larger than `0.3.0`
and smaller than `0.4.0`. If we want to move to `0.4.x`, we’d have to update
the `Cargo.toml` directly. When we do, the next time we `cargo build`, Cargo
will update the index and re-evaluate our `rand` requirements.

There’s a lot more to say about [Cargo][doccargo] and [its
ecosystem][doccratesio], but for now, that’s all we need to know. Cargo makes
it really easy to re-use libraries, and so Rustaceans tend to write smaller
projects which are assembled out of a number of sub-packages.
Então, nós instruímos o Cargo que queremos qualquer versão `0.3.x` de
`rand`, e assim ele busca a versão mais recente desde quando este
texto foi escrito, `0.3.14`. Mas, o que acontece quando semana que
vem chegar a versão `0.3.15` com um *bugfix* importante? Enquanto
obter *bugfixes* é importante, e se `0.3.15` contiver uma regressão
que quebra nosso código?

A resposta para isso está no arquivo `Cargo.lock` que você agora verá
no diretório do seu projeto. Quando você monta seu projeto pela
primeira vez, Cargo anota todas as versões que preenchem os critérios
e então as escreve no arquivo `Cargo.lock`. Quando você montar seu
projeto futuramente, Cargo verá que o arquivo `Cargo.lock` existe, e
então usará aquela versão específica em vez de refazer todo o trabalho
de anotar as versões. Isto te permite ter uma montagem repetível
automaticamente. Em outras palavras, continuaremos com a versão
`0.3.14` até que façamos uma atualização explícita, e assim com todos
com quem dividirmos nosso código, graças ao arquivo de trava (*lock file*)
`Cargo.lock`.

E quando quisermos usar a versão `0.3.15`? Cargo tem outro comando,
`update`, que diz 'ignore o *lock file*, anote quais as novas versões
que se encaixam no que foi especificado; se isto funcionar, escreva
estas novas versões no *lock file*'. Mas, por padrão, Cargo procurará
pelas versões maiores ou iguais a `0.3.0` e estritamente menores que
`0.4.0`. Se nós quisermos mover para a `0.4.x`, teremos então que
atualizar o arquivo `Cargo.toml` diretamente. Quando fizermos isso, na
próxima ver que executarmos `cargo build`, Cargo atualizará o índice e
re-evaluará nossos requerimentos sobre `rand`;

Há muito mais o que ser dito sobre [Cargo][doccargo] e [seu
ecossistema][doccratesio], mas por ora isto é tudo que precisamos
saber. Cargo torna realmente fácil reutilizar bibliotecas, e portanto
*rustáceos* tendem a escrever projetos menores que são montados em um
número de sub-pacotes.

[doccargo]: http://doc.crates.io
[doccratesio]: http://doc.crates.io/crates-io.html

Let’s get on to actually _using_ `rand`. Here’s our next step:
Agora vamos de fato _usar_ `rand`. Eis nosso próximo passo:

```rust,ignore
extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

The first thing we’ve done is change the first line. It now says
`extern crate rand`. Because we declared `rand` in our `[dependencies]`, we
can use `extern crate` to let Rust know we’ll be making use of it. This also
does the equivalent of a `use rand;` as well, so we can make use of anything
in the `rand` crate by prefixing it with `rand::`.
A primeira coisa que fizemos foi modificar a primeira linha. Agora ela
diz `extern crate rand`. Como declaramos `rand` no `[dependencies]`,
nós podemos usar `extern crate` para deixar Rust saber que faremos uso
dela. Isto também faz o equivalente de um `use rand;` também, assim
podemos fazer uso de qualquer coisa na *crate* `rand` prefixando-a com
`rand::`.

Next, we added another `use` line: `use rand::Rng`. We’re going to use a
method in a moment, and it requires that `Rng` be in scope to work. The basic
idea is this: methods are defined on something called ‘traits’, and for the
method to work, it needs the trait to be in scope. For more about the
details, read the [traits][traits] section.
A seguir, adicionamos outra linha `use` : `use rand::Rng`. Usaremos um
método em um momento, e ele requer que `Rng` esteja no escopo para
funcionar. A ideia básica é: métodos são definidos em alguma coisa
chamada 'tratos', e para o método funcionar, ele precisa que o trato
esteja no escopo. Para mais sobre os detalhes, leia a seção de
[tratos][traits].

[traits]: traits.html

There are two other lines we added, in the middle:
Tem outras duas linhas que adicionamos no meio:

```rust,ignore
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);
```

We use the `rand::thread_rng()` function to get a copy of the random number
generator, which is local to the particular [thread][concurrency] of execution
we’re in. Because we `use rand::Rng`’d above, it has a `gen_range()` method
available. This method takes two arguments, and generates a number between
them. It’s inclusive on the lower bound, but exclusive on the upper bound,
so we need `1` and `101` to get a number ranging from one to a hundred.
Nós usamos a função `rand::thread_rng()` para obter uma cópia do
gerador de números aleatórios, que é local à [*thread*][concurrency]
particular de execução na qual estamos. Como usamos o `use rand::Rng`
acima, ele tem um método `gen_range()` disponível. Este método toma
dois argumentos, e gera um número aleatório entre eles. Este método
inclui o limitante inferior mas exclui o superior, então precisamos de
`1` e `101` para obter um número de um a cem.

[concurrency]: concurrency.html

The second line prints out the secret number. This is useful while
we’re developing our program, so we can easily test it out. But we’ll be
deleting it for the final version. It’s not much of a game if it prints out
the answer when you start it up!
A segunda linha imprime o número secreto. Isto é útil enquanto estamos
desenvolvendo o programa, de forma que possamos facilmente
testá-lo. Mas o retiraremos na versão final. Um jogo não é muito
interessante se ele imprime a resposta quando o executamos!

Try running our new program a few times:
Tente rodar nosso novo programa algumas vezes:

```bash
$ cargo run
   Compiling guessing_game v0.1.0 (file:///home/you/projects/guessing_game)
    Finished debug [unoptimized + debuginfo] target(s) in 0.55 secs
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4
$ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```

Great! Next up: comparing our guess to the secret number.
Muito bom! A seguir: comparar nosso palpite com o número secreto.

# Comparing guesses
# Comparando os Palpites

Now that we’ve got user input, let’s compare our guess to the secret number.
Here’s our next step, though it doesn’t quite compile yet:
Agora que obtivemos a entrada do usuário, vamos comparar nosso palpite
com o número secreto. Este é nosso próximo passo, apesar de ainda não
compilar:

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   => println!("You win!"),
    }
}
```

A few new bits here. The first is another `use`. We bring a type called
`std::cmp::Ordering` into scope. Then, five new lines at the bottom that use
it:
Algumas coisinhas novas aqui. A primeira é outro `use`. Nós trouxemos
um tipo chamado `std::cmp::Ordering` para o escopo. Então, cinco novas
linhas no topo que o usam:

```rust,ignore
match guess.cmp(&secret_number) {
    Ordering::Less    => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal   => println!("You win!"),
}
```

The `cmp()` method can be called on anything that can be compared, and it
takes a reference to the thing you want to compare it to. It returns the
`Ordering` type we `use`d earlier. We use a [`match`][match] statement to
determine exactly what kind of `Ordering` it is. `Ordering` is an
[`enum`][enum], short for ‘enumeration’, which looks like this:
O método `cmp()` pode ser chamado em qualquer coisa que possa ser
comparada, e ele toma uma referência à coisa com a qual você quer
comparar. Ele retorna o tipo `Ordering` que usamos antes. Nós usamos
uma declaração [`match`][match] para determinar exatamente o tipo de
`Ordering` que ele é. `Ordering` é uma [`enum`][enum], contração de
'enumeração', que se parece com isso:

```rust
enum Foo {
    Bar,
    Baz,
}
```

[match]: match.html
[enum]: enums.html

With this definition, anything of type `Foo` can be either a
`Foo::Bar` or a `Foo::Baz`. We use the `::` to indicate the
namespace for a particular `enum` variant.
Com esta definição, qualquer coisa do tipo `Foo` pode ser ou um
`Foo::Bar` ou um `Foo::Baz`. Usamos um `::` para indicar um
*namespace* (espaço de nomes) para uma variante `enum` particular.

The [`Ordering`][ordering] `enum` has three possible variants: `Less`, `Equal`,
and `Greater`. The `match` statement takes a value of a type, and lets you
create an ‘arm’ for each possible value. Since we have three types of
`Ordering`, we have three arms:
A `enum` [`Ordering`][ordering] tem três variantes possíveis: `Less`,
`Equal` e `Greater` (respectivamente, 'menor', 'igual' e 'maior'). A
declaração `match` toma um valor de um tipo, e te permite criar um
'braço', um ramo, para cada valor possível. Desde que temos três tipos
de `Ordering`, teremos três ramos:

```rust,ignore
match guess.cmp(&secret_number) {
    Ordering::Less    => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal   => println!("You win!"),
}
```

[ordering]: ../../std/cmp/enum.Ordering.html

If it’s `Less`, we print `Too small!`, if it’s `Greater`, `Too big!`, and if
`Equal`, `You win!`. `match` is really useful, and is used often in Rust.
Se for o `Less`, imprimimos `Too small!`; se for `Greater`, `Too
big!`.; e se for `Equal`, `You win!`. `match` é realmente útil, e
usado com frequência em Rust.

I did mention that this won’t quite compile yet, though. Let’s try it:
Eu mencionei que isto não compilaria ainda, porém. Vamos tentar:

```bash
$ cargo build
   Compiling guessing_game v0.1.0 (file:///home/you/projects/guessing_game)
error[E0308]: mismatched types
  --> src/main.rs:23:21
   |
23 |     match guess.cmp(&secret_number) {
   |                     ^^^^^^^^^^^^^^ expected struct `std::string::String`, found integral variable
   |
   = note: expected type `&std::string::String`
   = note:    found type `&{integer}`

error: aborting due to previous error

error: Could not compile `guessing_game`.

To learn more, run the command again with --verbose.
```

Whew! This is a big error. The core of it is that we have ‘mismatched types’.
Rust has a strong, static type system. However, it also has type inference.
When we wrote `let guess = String::new()`, Rust was able to infer that `guess`
should be a `String`, and so it doesn’t make us write out the type. And with
our `secret_number`, there are a number of types which can have a value
between one and a hundred: `i32`, a thirty-two-bit number, or `u32`, an
unsigned thirty-two-bit number, or `i64`, a sixty-four-bit number or others.
So far, that hasn’t mattered, and so Rust defaults to an `i32`. However, here,
Rust doesn’t know how to compare the `guess` and the `secret_number`. They
need to be the same type. Ultimately, we want to convert the `String` we
read as input into a real number type, for comparison. We can do that
with two more lines. Here’s our new program:
Nossa! Este é um erro grande. O núcleo dele é que temos 'tipos
incompatíveis'. Rust tem um sistema de tipos forte e estático. Porém,
Rust também tem inferência de tipo. Quando escrevemos `let guess =
String::new()`, Rust foi capaz de inferir que `guess` deveria ser um
`String`, então ele não nos faz escrever o tipo. e com nossos
`secret_number`, existe uma quantidade de tipos que podem ter valor
de um a cem: `i32`, um número de 32 bits, ou `u32`, um número de 32
bits sem sinal (*unsigned*), ou `i64`, um número de 64 bits, ou
outros. Até aqui, isto não teve importância, e portanto Rust
estabelece como padrão o `i32`. Porém, agora, Rust não sabe como
comparar o `guess` e o `secret_number`. Eles precisam ser do mesmo
tipo. Em última análise, queremos converter o `String` que lemos como
entrada em um tipo real de número, para comparação. Podemos fazer isso
com mais duas linhas. Eis nosso novo programa:

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   => println!("You win!"),
    }
}
```

The new two lines:
As duas linhas novas:

```rust,ignore
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");
```

Wait a minute, I thought we already had a `guess`? We do, but Rust allows us
to ‘shadow’ the previous `guess` with a new one. This is often used in this
exact situation, where `guess` starts as a `String`, but we want to convert it
to an `u32`. Shadowing lets us re-use the `guess` name, rather than forcing us
to come up with two unique names like `guess_str` and `guess`, or something
else.
Espera um minuto, eu pensei que já tínhamos um `guess`, não? Sim,
tínhamos, porém Rust nos permite "sombrear" o `guess` anterior com um
novo. Isto é usado comumente nesta exata situação, onde `guess` começa
como `String` mas queremos convertê-lo para `u32`. O sombreamento nos
permite reutilizar o nome `guess`, em vez de forçar-nos a ficar com
dois nomes únicos como `guess_str` e `guess`, ou algo mais.

We bind `guess` to an expression that looks like something we wrote earlier:
Nós vinculamos `guess` a uma expressão que parece com algo que
escrevemos anteriormente:

```rust,ignore
guess.trim().parse()
```

Here, `guess` refers to the old `guess`, the one that was a `String` with our
input in it. The `trim()` method on `String`s will eliminate any white space at
the beginning and end of our string. This is important, as we had to press the
‘return’ key to satisfy `read_line()`. This means that if we type `5` and hit
return, `guess` looks like this: `5\n`. The `\n` represents ‘newline’, the
enter key. `trim()` gets rid of this, leaving our string with only the `5`. The
[`parse()` method on strings][parse] parses a string into some kind of number.
Since it can parse a variety of numbers, we need to give Rust a hint as to the
exact type of number we want. Hence, `let guess: u32`. The colon (`:`) after
`guess` tells Rust we’re going to annotate its type. `u32` is an unsigned,
thirty-two bit integer. Rust has [a number of built-in number types][number],
but we’ve chosen `u32`. It’s a good default choice for a small positive number.
Aqui, `guess` refere-se à antiga `guess`, aquela que era uma `string`
com nossa entrada nela. O método `trim()` em `String`s eliminará
quaisquer espaços em branco no início ou no fim da nossa string. Isto
é importante, dado que temos que pressionar a tecla 'return' (mais
conhecida como 'Enter') para satisfazer `read_line()`. Isto significa
que se escrevermos `5` e apertarmos 'return', `guess` se parecerá com
isso: `5\n`. O `\n` representa 'nova linha' (*newline*), a tecla
Enter. 'trim()' elimina isso, deixando em nossa string apenas o '5'. O
método [`parse()` em strings][parse] analisa uma string para algum
tipo de número. Dado que ele pode interpretar uma variedade de
números, precisamos dar ao Rust uma dica de qual exato número nós
queremos. Daí, `let guess: u32`. O dois-pontos (`:`) depois de `guess`
informa o Rust que estamos anotando seu tipo. `u32` é um inteiro de 32
bits sem sinal. Rust tem [uma variedade de tipos de números já
embutida][number], mas escolhemos `u32`. É uma boa escolha padrão para
um número positivo pequeno.

[parse]: ../../std/primitive.str.html#method.parse
[number]: primitive-types.html#numeric-types

Just like `read_line()`, our call to `parse()` could cause an error. What if
our string contained `A👍%`? There’d be no way to convert that to a number. As
such, we’ll do the same thing we did with `read_line()`: use the `expect()`
method to crash if there’s an error.
A exemplo de `read_line()`, nossa chamada para `parse()` pode causar
um erro. E se nossa string contiver  `A👍%`? Não tem como converter
isto em um número. Como tal, faremos o mesmo que fizemos com
`read_line()`: usar o método `expect()` para quebrar se tivermos um
erro.

Let’s try our program out!
Vamos tentar executar nosso programa!

```bash
$ cargo run
   Compiling guessing_game v0.1.0 (file:///home/you/projects/guessing_game)
    Finished debug [unoptimized + debuginfo] target(s) in 0.57 secs
     Running `target/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```

Nice! You can see I even added spaces before my guess, and it still figured
out that I guessed 76. Run the program a few times, and verify that guessing
the number works, as well as guessing a number too small.
Boa! Você pode ver que eu até acrescentei espaços antes do meu
palpite, e o programa mesmo assim concluiu que eu palpitei 76. Execute
o programa mais algumas vezes, e verifique que palpitar o número
funciona, bem como palpitar um número muito pequeno.

Now we’ve got most of the game working, but we can only make one guess. Let’s
change that by adding loops!
Agora nós conseguimos a maior parte do jogo funcionando, mas só
podemos oferecer um palpite. Vamos mudar isso adicionando laços!

# Looping
# *Looping*

The `loop` keyword gives us an infinite loop. Let’s add that in:
A palavra-chave `loop` nos fornece um laço (*loop*) infinito. Vamos
adicioná-la ao programa:

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => println!("You win!"),
        }
    }
}
```

And try it out. But wait, didn’t we just add an infinite loop? Yup. Remember
our discussion about `parse()`? If we give a non-number answer, we’ll `panic!`
and quit. Observe:
E tente usar. Mas espere, não acabamos de criar um laço infinito?
Sim. Lembra nossa discussão sobre `parse()`? Se fornecermos uma
resposta não-numérica, ele vai entrar em `panic!` e sair. Observe:

```bash
$ cargo run
   Compiling guessing_game v0.1.0 (file:///home/you/projects/guessing_game)
    Finished debug [unoptimized + debuginfo] target(s) in 0.58 secs
     Running `target/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit
thread 'main' panicked at 'Please type a number!'
```

Ha! `quit` actually quits. As does any other non-number input. Well, this is
suboptimal to say the least. First, let’s actually quit when you win the game:
Ha! `quit` de fato sai. Assim como qualquer outra entrada
não-numérica. Bem, isto é sub-ótimo, para falar o mínimo. Primeiro,
vamos realmente sair quando vencermos o jogo:

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
```

By adding the `break` line after the `You win!`, we’ll exit the loop when we
win. Exiting the loop also means exiting the program, since it’s the last
thing in `main()`. We have only one more tweak to make: when someone inputs a
non-number, we don’t want to quit, we want to ignore it. We can do that
like this:
Adicionando a linha `break` após o `You win!`, podemos sair do laço
quando vencermos. Nós temos apenas mais um ajuste para fazer: quando
alguém insere um não-número, não queremos sair, mas ignorá-lo. Podemos
fazer isso dessa forma:

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
```

These are the lines that changed:
Estas são as linhas modificadas:

```rust,ignore
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```
This is how you generally move from ‘crash on error’ to ‘actually handle the
error’, by switching from `expect()` to a `match` statement. A `Result` is
returned by `parse()`, this is an `enum`  like `Ordering`, but in this case,
each variant has some data associated with it: `Ok` is a success, and `Err` is a
failure. Each contains more information: the successfully parsed integer, or an
error type. In this case, we `match` on `Ok(num)`, which sets the name `num` to
the unwrapped `Ok` value (the integer), and then we  return it on the
right-hand side. In the `Err` case, we don’t care what kind of error it is, so
we just use the catch all `_` instead of a name. This catches everything that
isn't `Ok`, and `continue` lets us move to the next iteration of the loop; in
effect, this enables us to ignore all errors and continue with our program.

Now we should be good! Let’s try:
É desta forma que geralmente mudamos de 'quebrar em caso de erro' para
'realmente lidar com os erros', mudando de `expect()` para uma
declaração `match`. Um `Result` é retornado por `parse()`, ele é um
`enum` como `Ordering`, mas neste caso cada variante tem algum dado
associado a ela: `Ok` é sucesso, e `Err` é falha. Cada um contém mais
informação: o número analisado com sucesso ou um tipo erro. Neste
caso, fazemos o `match` em `Ok(num)`, que estabelece o nome `num` para
o valor não-envelopado (*unwrapped*) em `Ok` (o inteiro), e então o
retornamos para o lado direito. No caso de `Err`, não estamos
interessados no tipo de erro que seja, então apenas usamos o símbolo
`_` para capturar todas as ocorrências em vez de um nome. Isto captura
tudo que não seja `Ok`, e o `continue` nos permite mover para a
próxima iteração do laço; com efeito, permite que ignoremos todos os
erros e continuemos a execução do nosso programa.

Agora devemos estar bem! Vamos tentar:

```bash
$ cargo run
   Compiling guessing_game v0.1.0 (file:///home/you/projects/guessing_game)
    Finished debug [unoptimized + debuginfo] target(s) in 0.57 secs
     Running `target/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

Awesome! With one tiny last tweak, we have finished the guessing game. Can you
think of what it is? That’s right, we don’t want to print out the secret
number. It was good for testing, but it kind of ruins the game. Here’s our
final source:
Demais! Com apenas um último ajuste, terminaremos o jogo de
adivinhação. Consegue imaginar qual seja? Isso mesmo, não precisamos
imprimir o número secreto. Isto foi bom para testar, mas meio que
estraga o jogo. Eis nosso código final:

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
```

# Complete!
# Completo!

This project showed you a lot: `let`, `match`, methods, associated
functions, using external crates, and more.
Este projeto te mostrou muita coisa: `let`, `match`, métodos, funções
associadas, uso de *crates* externos etc.;

At this point, you have successfully built the Guessing Game! Congratulations!
Neste ponto, você construiu com sucesso um Jogo de Adivinhação!
Parabéns!
