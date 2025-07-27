<!-- Eski bağlantı, silmeyin -->

<a id="installing-binaries-from-cratesio-with-cargo-install"></a>

## `cargo install` ile İkili Dosyalar Yüklemek

`cargo install` komutu, ikili crate'leri yerel olarak yüklemenizi ve kullanmanızı sağlar. Bu, sistem paketlerinin yerini almak için değil; Rust geliştiricilerinin [crates.io](https://crates.io/)<!-- ignore -->'da başkalarıyla paylaştığı araçları kolayca yükleyebilmesi için tasarlanmıştır. Yalnızca ikili hedefi (binary target) olan paketleri yükleyebileceğinizi unutmayın. Bir _ikili hedef_, crate'in bir _src/main.rs_ dosyası veya ikili olarak belirtilmiş başka bir dosyası varsa oluşturulan çalıştırılabilir programdır; buna karşılık, kütüphane hedefi kendi başına çalıştırılamaz, ancak başka programlara dahil edilmeye uygundur. Genellikle, crate'lerin _README_ dosyasında, crate'in bir kütüphane mi, ikili hedefi mi, yoksa her ikisi mi olduğu belirtilir.

`cargo install` ile yüklenen tüm ikili dosyalar, yükleme kök dizininin _bin_ klasöründe saklanır. Rust'ı _rustup.rs_ ile yüklediyseniz ve özel bir yapılandırmanız yoksa, bu dizin *$HOME/.cargo/bin* olur. Bu dizinin `$PATH` ortam değişkeninizde olduğundan emin olun ki, `cargo install` ile yüklediğiniz programları çalıştırabilesiniz.

oùrnek olarak, 12. bölümde dosya aramak için kullanılan `grep` aracının Rust implementasyonu olan `ripgrep`'den bahsetmiştik. `ripgrep`'i yüklemek için şu komutu çalıştırabiliriz:

<!-- manuel-yenileme
cargo install ile sahip olmadığınız bir şey yükleyin, ilgili çıktıyı kopyalayın
-->

```console
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v14.1.1
  Downloaded 1 crate (213.6 KB) in 0.40s
  Installing ripgrep v14.1.1
--snip--
   Compiling grep v0.3.2
    Finished `release` profile [optimized + debuginfo] target(s) in 6.73s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v14.1.1` (executable `rg`)
```

Çıktının sondan bir önceki satırı, yüklenen ikilinin konumunu ve adını gösterir; `ripgrep` için bu, `rg`'dir. Daha önce belirtildiği gibi yükleme dizini `$PATH`'inizde olduğu sürece, artık `rg --help` komutunu çalıştırabilir ve dosya aramak için daha hızlı, Rust tabanlı bir aracı kullanmaya başlayabilirsiniz!
