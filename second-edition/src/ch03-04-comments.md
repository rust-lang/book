## Комментарии

Все хорошие программисты, создавая программный код, стремятся сделать его простым
 и понятным. Бывают всё же случаи, когда дополнительное описание просто необходимо.
 В этих случаях программисты пишут заметки (или как их ещё называют, комментарии).
 Комментарии игнорируются компилятором, но для тех кто код читает - это очень важная
 часть документации.

Пример:

```rust
// Hello, world.
```

Самый простой вид комментария - это комментарий строки `//`. Он ставится там, где
вся последующая строка текста должна быть игнорирована компилятором:

```rust
// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.
```

Ещё пример:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let lucky_number = 7; // I’m feeling lucky today.
}
```
Ещё пример (комментарий-аннотация):

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    // I’m feeling lucky today.
    let lucky_number = 7;
}
```
