<!-- ia-translate: true -->
## Comentários {#comments}

Todo programador se esforça para deixar o código fácil de entender, mas às vezes
uma explicação extra é justificada. Nesses casos, os programadores deixam
_comentários_ no código-fonte, que o compilador ignora, mas que podem ser úteis
para quem lê o código.

Aqui está um comentário simples:

```rust
// hello, world
```

No Rust, o estilo idiomático de comentário começa com duas barras, e o
comentário continua até o fim da linha. Para comentários que ocupam mais de uma
linha, você precisa incluir `//` em cada uma delas, assim:

```rust
// So we're doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what's going on.
```

Comentários também podem ser colocados no fim de linhas que contêm código:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-24-comments-end-of-line/src/main.rs}}
```

Mas você verá com mais frequência o uso deste formato, com o comentário em uma
linha separada, acima do código que ele anota:

<span class="filename">Nome do arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-25-comments-above-line/src/main.rs}}
```

O Rust também tem outro tipo de comentário, os comentários de documentação, que
vamos discutir na seção [“Publicando um Crate no Crates.io”][publishing]<!--
ignore --> do Capítulo 14.

[publishing]: ch14-02-publishing-to-crates-io.html
