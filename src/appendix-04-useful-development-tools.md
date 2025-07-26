## Appendix D - Yararlı Geliştirme Araçları
Bu ekte, Rust projesinin sağladığı bazı yararlı geliştirme araçlarından bahsedeceğiz.
Otomatik biçimlendirme, uyarı düzeltmelerini hızlı bir şekilde uygulama yolları,
linter ve IDE'lerle entegrasyon konularını ele alacağız.

### `rustfmt` ile Otomatik Biçimlendirme

`rustfmt` aracı, kodunuzu topluluk kod stiline göre yeniden biçimlendirir.
Birçok ortak proje, Rust yazarken hangi stilin kullanılacağı konusunda tartışmaları önlemek için
`rustfmt` kullanır: herkes kodunu bu araçla biçimlendirir.

Rust kurulumları varsayılan olarak rustfmt'yi içerir, bu nedenle sisteminizde
`rustfmt` ve `cargo-fmt` programları zaten bulunmalıdır. Bu iki komut,
`rustc` ve `cargo` ile benzerdir; `rustfmt` daha ayrıntılı kontrol sağlar ve
`cargo-fmt`, Cargo kullanan bir projenin kurallarını anlar. Herhangi bir Cargo projesini biçimlendirmek için
aşağıdakini girin:

```sh
$ cargo fmt
```

Bu komutu çalıştırmak, mevcut crate içindeki tüm Rust kodunu yeniden biçimlendirir. Bu
sadece kod stilini değiştirir, kod semantiğini değiştirmez.

Bu komut size `rustfmt` ve `cargo-fmt` sağlar, tıpkı Rust'un size
hem `rustc` hem de `cargo` sağladığı gibi. Herhangi bir Cargo projesini biçimlendirmek için şunu girin:

```console
$ cargo fmt
```

Bu komutu çalıştırmak, mevcut crate içindeki tüm Rust kodunu yeniden biçimlendirir. Bu
sadece kod stilini değiştirir, kod semantiğini değiştirmez. `rustfmt` hakkında daha fazla bilgi
için [belgelerine][rustfmt] bakın.

[rustfmt]: https://github.com/rust-lang/rustfmt

### `rustfix` ile Kodunuzu Düzeltin

`rustfix` aracı Rust kurulumlarına dahildir ve sorunu düzeltmenin açık bir yolu olan
derleyici uyarılarını otomatik olarak düzeltebilir. Bu, muhtemelen sizin istediğiniz
sonuçtur. Muhtemelen daha önce derleyici uyarıları görmüşsünüzdür. Örneğin,
şu kodu ele alalım:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let mut x = 42;
    println!("{x}");
}
```

Burada, `x` değişkenini değiştirilebilir olarak tanımlıyoruz, ancak aslında onu asla değiştirmiyoruz.
Rust bizi bu konuda uyarıyor:

```console
$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: variable does not need to be mutable
 --> src/main.rs:2:9
  |
2 |     let mut x = 0;
  |         ----^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default
```

Uyarı, `mut` anahtar kelimesini kaldırmamızı öneriyor. Bu öneriyi, `rustfix` aracını kullanarak
`cargo fix` komutunu çalıştırarak otomatik olarak uygulayabiliriz:

```console
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

_src/main.rs_ dosyasına tekrar baktığımızda, `cargo fix` komutunun kodu değiştirdiğini göreceğiz:
:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 42;
    println!("{x}");
}
```

`x` değişkeni artık değiştirilemez ve uyarı artık görünmüyor.

`cargo fix` komutunu kullanarak kodunuzu farklı Rust sürümleri arasında geçiş yapabilirsiniz.
Sürümler [Ek E][editions] bölümünde ele alınmaktadır.

### Clippy ile Daha FazlHata Kontrolü

Clippy aracı, kodunuzu analiz etmek için bir dizi hata kontrolü içerir, böylece
yaygın hataları yakalayabilir ve Rust kodunuzu iyileştirebilirsiniz. Clippy, standart
Rust kurulumlarına dahildir.

Clippy'nin hata kontrollerini herhangi bir Cargo projesinde çalıştırmak için aşağıdakileri girin:

```console
$ cargo clippy
```

Örneğin, bu programda olduğu gibi pi gibi bir matematik sabitinin yaklaşık değerini kullanan bir program yazdığınızı varsayalım:

<Listing file-name="src/main.rs">

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

</Listing>

Bu projede `cargo clippy` komutunu çalıştırdığınızda şu hata mesajı görüntülenir:

```text
error: approximate value of `f{32, 64}::consts::PI` found
 --> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = note: `#[deny(clippy::approx_constant)]` on by default
  = help: consider using the constant directly
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant
```

Bu hata, Rust'ta daha kesin bir `PI` sabiti
tanımlandığını ve programınızda bu sabiti kullanmanızın daha doğru olacağını
belirtir. Bu durumda, kodunuzu `PI` sabitini kullanacak şekilde değiştirirsiniz.
Aşağıdaki kod, Clippy'den herhangi bir hata veya uyarı almaz:

<Listing file-name="src/main.rs">

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

</Listing>

Clippy hakkında daha fazla bilgi için [belgelerine][clippy] bakın.

[clippy]: https://github.com/rust-lang/rust-clippy

### `rust-analyzer` Kullanarak IDE Entegrasyonu

IDE entegrasyonuna yardımcı olmak için Rust topluluğu
[`rust-analyzer`][rust-analyzer]<!-- ignore -->. Bu araç, IDE'ler ve programlama dilleri arasında
iletişim kurmak için kullanılan bir spesifikasyon olan [Language Server Protocol][lsp]<!--
ignore --> ile iletişim kuran, derleyici merkezli bir dizi yardımcı programdır.
Farklı istemciler `rust-analyzer`'ı kullanabilir, örneğin Farklı istemciler `rust-analyzer` kullanabilir, örneğin
[Visual Studio Code için Rust analizörü eklentisi][vscode].

[lsp]: http://langserver.org/
[vscode]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer

`rust-analyzer` projesinin [ana sayfası][rust-analyzer]<!-- ignore -->
adresini ziyaret ederek kurulum talimatlarını inceleyin ve ardından dil sunucusu desteğini kendi
IDE'nize yükleyin. IDE'niz otomatik tamamlama, tanıma atlama ve satır içi hata
gibi özelliklere kavuşacaktır.

[rust-analyzer]: https://rust-analyzer.github.io
[editions]: appendix-05-editions.md
