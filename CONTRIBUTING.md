# Katkı Sağlama

Yardımınıza çok seviniriz! Kitaba ilgi gösterdiğiniz için teşekkür ederiz.

## Düzenleme Yeri

Tüm düzenlemeler `src` dizininde yapılmalıdır.

`nostarch` dizini, basılı sürümün yayıncılarına düzenlemeleri göndermek için kullanılan anlık görüntüleri içerir. Anlık görüntü dosyaları, gönderilen ve gönderilmeyenleri yansıtır, bu nedenle

sadece düzenlemeler No Starch'a gönderildiğinde güncellenir. **nostarch dizinindeki dosyaları değiştiren pull istekleri göndermeyin, bunlar kapatılacaktır.**


Depodaki Rust koduna standart biçimlendirme uygulamak için [rustfmt][rustfmt] ve
projedeki Markdown kaynağına ve Rust dışı kodlara sabit biçimlendirme uygulamak için [dprint][dprint] kullanıyoruz.

[rustfmt]: https://github.com/rust-lang/rustfmt
[dprint]: https://dprint.dev

Rust araç zinciri yüklü ise normalde `rustfmt` de yüklü olacaktır;
herhangi bir nedenle `rustfmt` kopyası yoksa, aşağıdaki komutu çalıştırarak ekleyebilirsiniz:

```sh
rustup component add rustfmt
```

`dprint`'i yüklemek için aşağıdaki komutu çalıştırabilirsiniz:

```sh
cargo install dprint
```

Veya `dprint` web sitesindeki [talimatları][install-dprint] izleyin.

[install-dprint]: https://dprint.dev/install/

Rust kodunu biçimlendirmek için `rustfmt <dosya yolu>` komutunu çalıştırabilir, diğer dosyaları biçimlendirmek için ise `dprint fmt <dosya yolu>` komutunu kullanabilirsiniz. Birçok metin düzenleyici, hem `rustfmt` hem de `dprint` için yerel destek veya uzantılara sahiptir.

## Düzeltmelerin Kontrol Edilmesi

Kitap, Rust sürüm trenlerini takip etmektedir. Bu nedenle,
https://doc.rust-lang.org/stable/book adresinde bir sorun görürseniz, bu sorun bu depodaki `main`
dalında zaten düzeltilmiş olabilir, ancak düzeltme henüz nightly -> beta -> stable
aşamalarından geçmemiş olabilir. Bir sorun bildirmeden önce lütfen bu depodaki `main` dalını kontrol edin.

Belirli bir dosyanın geçmişine bakmak da, bir sorunun nasıl veya olup olmadığını anlamaya çalışıyorsanız, daha fazla bilgi verebilir.

Yeni bir sorun bildirmeden veya yeni bir PR açmadan önce lütfen açık ve kapalı sorunları ve açık ve kapalı PR'leri de arayın.


## Lisanslama

Bu depo, Rust'un kendisiyle aynı lisans altında, MIT/Apache2'dir. Her lisansın tam metnini bu depodaki `LICENSE-*` dosyalarında bulabilirsiniz.

## Davranış Kuralları

Rust projesi, bu proje dahil olmak üzere tüm alt projeleri yöneten [davranış kuralları](http://rust-lang.org/policies/code-of-conduct)

## Beklentiler

Kitap [basılı][nostarch] olduğundan ve kitabın çevrimiçi sürümünü mümkün olduğunca basılı sürüme yakın tutmak istediğimizden,
 sorununuzu veya çekme isteğinizi ele almamız normalde alıştığınızdan daha uzun sürebilir.

[nostarch]: https://nostarch.com/rust-programming-language-2nd-edition

Şimdiye kadar, [Rust Editions](https://doc.rust-lang.org/edition-guide/) ile eşzamanlı olarak daha büyük bir revizyon yapıyoruz. Bu büyük revizyonlar arasında
sadece hataları düzelteceğiz. Sorununuz veya çekme isteğiniz
kesinlikle bir hatayı düzeltmiyorsa, bir sonraki büyük revizyon çalışmamıza kadar beklemede kalabilir:
bu, aylar veya yıllar sürebilir. Sabrınız için teşekkür ederiz!


## Yardım aranıyor

Çok fazla okuma veya yazma gerektirmeyen yardım yolları arıyorsanız, [E-help-wanted etiketli açık sorunlara] bakın.
Bunlar, metin, Rust kodu, ön uç kodu veya kabuk betiklerinde küçük düzeltmeler olabilir ve bizim daha verimli olmamıza veya kitabı bir şekilde geliştirmemize yardımcı olabilir!

[help-wanted]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3AE-help-wanted

## Çeviriler

Kitabın çevirisine yardımcı olmak ister misiniz? [Çeviriler] etiketine bakarak şu anda devam eden çalışmalara katılabilirsiniz.
Yeni bir dil üzerinde çalışmaya başlamak için yeni bir konu açın!
Birden fazla dil için [mdbook desteği] bekliyoruz,
ancak siz çalışmaya başlayabilirsiniz!

[Translations]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook support]: https://github.com/rust-lang/mdBook/issues/5
