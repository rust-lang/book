# Katkıda Bulunmak

Yardımınızı çok isteriz! Kitabı önemsediğiniz için teşekkürler.

## Nerede Düzenlenir

Tüm düzenlemeler `src` dizininde yapılmalıdır.

Nostarch` dizini, düzenlemeleri yayıncılara göndermek için anlık görüntüler içerir
basılı sürümün. Anlık görüntü dosyaları neyin gönderilip gönderilmediğini yansıtır, bu nedenle
yalnızca düzenlemeler No Starch'a gönderildiğinde güncellenir. **
adresinden `nostarch` dizinindeki dosyaları değiştirme istekleri göndermeyin, bunlar kapatılacaktır.**

reposundaki Rust koduna standart biçimlendirme uygulamak için [`rustfmt`][rustfmt] ve Markdown kaynağına
ve projedeki Rust olmayan koda ayakta biçimlendirme uygulamak için [`dprint`][dprint] kullanıyoruz.

[rustfmt]: https://github.com/rust-lang/rustfmt
[dprint]: https://dprint.dev

Rust araç zinciriniz varsa normalde `rustfmt` yüklü olacaktır
yüklü; herhangi bir nedenle `rustfmt` kopyasına sahip değilseniz, aşağıdaki komutu çalıştırarak
ekleyebilirsiniz:

```sh
rustup component add rustfmt
```

`dprint`i yüklemek için aşağıdaki komutu çalıştırabilirsiniz:

```sh
cargo install dprint
```

Ya da `dprint` web sitesindeki [talimatları][install-dprint] izleyin.

[install-dprint]: https://dprint.dev/install/

Rust kodunu biçimlendirmek için `rustfmt <path to file>` komutunu çalıştırabilir ve diğer
dosyalarını biçimlendirmek için `dprint fmt <path to file>` komutunu verebilirsiniz. Birçok metin editörünün hem `rustfmt` hem de `dprint` için yerel
desteği veya uzantıları vardır.

## Düzeltmeler için Kontrol

Kitap Rust sürüm trenlerine binmektedir. Bu nedenle,
https://doc.rust-lang.org/stable/book adresinde bir sorun görürseniz, bu depodaki `main`
dalında zaten düzeltilmiş olabilir, ancak düzeltme henüz nightly -> beta -> stable
üzerinden geçmemiştir. Lütfen bir sorunu bildirmeden önce bu depodaki `main` dalını kontrol edin.

Belirli bir dosyanın geçmişine bakmak,
adresini anlamaya çalışıyorsanız, bir sorunun nasıl düzeltildiği veya düzeltilip düzeltilmediği konusunda
adresinde daha fazla bilgi verebilir.

Lütfen
adresine yeni bir sorun bildirmeden veya yeni bir PR açmadan önce açık ve kapalı sorunları ve açık ve kapalı PR'leri de arayın.

## Lisanslama

Bu depo Rust'ın kendisi ile aynı lisans altındadır, MIT/Apache2.
her lisansın tam metnini bu
deposundaki `LICENSE-*` dosyalarında bulabilirsiniz.

## Davranış Kuralları

Rust projesinin, bu proje de dahil olmak üzere tüm alt projeleri yöneten [bir davranış kuralları](http://rust-lang.org/policies/code-of-conduct)
vardır. Lütfen buna saygı gösterin!

## Beklentiler

Kitap [basılı][nostarch] olduğundan ve
kitabın çevrimiçi sürümünü
mümkün olduğunda basılı sürüme yakın tutmak istediğimizden,
sorununuzu veya çekme isteğinizi ele almamız alıştığınızdan daha uzun sürebilir.
[nostarch]: https://nostarch.com/rust-programming-language-2nd-edition

Şimdiye kadar, [Rust Editions](https://doc.rust-lang.org/edition-guide/) ile aynı zamana denk gelecek şekilde daha büyük bir revizyon yaptık. Bu büyük
revizyonları arasında sadece hataları düzelteceğiz. Sorununuz veya çekme isteğiniz
kesinlikle bir hatayı düzeltmiyorsa, bir dahaki sefere
büyük bir revizyon üzerinde çalışana kadar bekleyebilir: aylar veya yıllar mertebesinde bekleyin. Sabrınız için teşekkür ederiz
!

## Yardım Aranıyor

Büyük miktarda
okuma veya yazmayı içermeyen yardım yolları arıyorsanız, [E-yardım aranıyor
etiketli açık konular][yardım aranıyor] bölümüne göz atın. Bunlar metinde küçük düzeltmeler, Rust kodu,
ön uç kodu veya daha verimli olmamıza veya
kitabı bir şekilde geliştirmemize yardımcı olacak kabuk komut dosyaları olabilir!

[help-wanted]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3AE-help-wanted

## Çeviriler

Kitabın çevirisine yardım etmek isteriz! Şu anda devam etmekte olan
çalışmalarına katılmak için [Çeviriler][Translations] etiketine bakın. adresinde yeni bir dil üzerinde çalışmaya başlamak için yeni bir konu açın! Herhangi birini birleştirmeden önce birden fazla dil için [mdbook support]
adresini bekliyoruz, ancak başlamaktan çekinmeyin!

[Translations]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook support]: https://github.com/rust-lang/mdBook/issues/5
