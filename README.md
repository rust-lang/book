# Rust Programlama Dili

![Build Status](https://github.com/rust-lang/book/workflows/CI/badge.svg)

Bu depo, `Rust Proglamlama Dili` kitabının kaynak kodunu içerir.

[Kitap, No Starch Press tarafından basılı olarak satışa sunulmuştur.][nostarch].

[nostarch]: https://nostarch.com/rust-programming-language-2nd-edition

Kitabı çevrimiçi olarak ücretsiz olarak da okuyabilirsiniz. Lütfen kitabı, en son [kararlı], [beta] veya [gece] Rust sürümleriyle birlikte gönderilen haliyle inceleyin. Bu sürümlerdeki sorunların, bu depoda zaten düzeltilmiş olabileceğini unutmayın, çünkü bu sürümler daha az sıklıkta güncellenmektedir.

[kararlı]: https://doc.rust-lang.org/stable/book/
[beta]: https://doc.rust-lang.org/beta/book/
[gece]: https://doc.rust-lang.org/nightly/book/

Kitapta yer alan tüm kod listelerinin yalnızca kodunu indirmek için [sürümler] bölümüne bakın.

[sürümler]: https://github.com/rust-lang/book/releases

## Gereksinimler

Kitabı oluşturmak için [mdBook][mdBook] gerekir, ideal olarak
rust-lang/rust'un [bu dosyada][rust-mdbook] kullandığı sürüm. Bunu edinmek için:

[mdBook]: https://github.com/rust-lang/mdBook
[rust-mdbook]: https://github.com/rust-lang/rust/blob/master/src/tools/rustbook/Cargo.toml

```bash
$ cargo install mdbook --locked --version <version_num>
```

Kitap ayrıca bu deponun bir parçası olan iki mdbook eklentisini kullanır. Bunları yüklemezseniz, derleme sırasında uyarılar görürsünüz ve çıktı doğru görünmez, ancak
kitabı yine de derleyebilirsiniz. Eklentileri kullanmak için
şunu çalıştırmalısınız:

```bash
$ cargo install --locked --path packages/mdbook-trpl --force
```

## Oluşturma

Kitabı oluşturmak için şunu yazın:

```bash
$ mdbook build
```

Çıktı, `book` alt dizininde olacaktır. Kontrol etmek için, web tarayıcınızda açın.

_Firefox:_

```bash
$ firefox book/index.html                       # Linux
$ open -a "Firefox" book/index.html             # OS X
$ Start-Process "firefox.exe" .\book\index.html # Windows (PowerShell)
$ start firefox.exe .\book\index.html           # Windows (Cmd)
```

_Chrome:_

```bash
$ google-chrome book/index.html                 # Linux
$ open -a "Google Chrome" book/index.html       # OS X
$ Start-Process "chrome.exe" .\book\index.html  # Windows (PowerShell)
$ start chrome.exe .\book\index.html            # Windows (Cmd)
```

Testleri çalıştırmak için:

```bash
$ cd packages/trpl
$ mdbook test --library-path packages/trpl/target/debug/deps
```

## Katkıda bulunmak

Yardımınıza ihtiyacımız var! Lütfen [CONTRIBUTING.md][contrib] dosyasına bakarak
aradığımız katkı türleri hakkında bilgi edinin.

[contrib]: https://github.com/rust-lang/book/blob/main/CONTRIBUTING.md

Kitap [basılı][nostarch] olduğundan ve kitabın çevrimiçi sürümünü mümkün olduğunca basılı sürümüne yakın tutmak istediğimizden,
 sorununuzu veya çekme isteğinizi ele almamız
alıştığınızdan daha uzun sürebilir.
Bu nedenle, lütfen sabırlı olun ve sorunlarınızı veya çekme isteklerinizi

Şimdiye kadar, [Rust Editions](https://doc.rust-lang.org/edition-guide/) ile eşzamanlı olarak daha büyük bir revizyon yapıyoruz. Bu büyük
revizyonlar arasında, sadece hataları düzelteceğiz. Sorununuz veya çekme isteğiniz
kesinlikle bir hatayı düzeltmiyorsa, bir sonraki büyük revizyon çalışmamıza kadar
bekleyebilir: bu, aylar veya yıllar sürebilir. Sabrınız için
teşekkür ederiz!

### Translations

Kitabı çevirmeye yardım etmek isteriz! [Çeviriler] etiketine bakarak şu anda devam eden çalışmalara katılabilirsiniz.
Yeni bir dil üzerinde çalışmaya başlamak için yeni bir konu açın!
Herhangi bir dili birleştirmadan önce [mdbook desteği] için birden fazla dil bekliyoruz,
ama siz çalışmaya başlayabilirsiniz!

[Çeviriler]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook desteği]: https://github.com/rust-lang/mdBook/issues/5

## Yazım Denetimi

Kaynak dosyalarında yazım hatalarını taramak için, `ci` dizininde bulunan `spellcheck.sh`
komut dosyasını kullanabilirsiniz. Bu komut dosyası, geçerli kelimelerin bulunduğu bir sözlüğe ihtiyaç duyar.
Bu sözlük, `ci/dictionary.txt` dosyasında bulunur. Komut dosyası yanlış bir
pozitif sonuç verirse (örneğin, komut dosyası tarafından geçersiz kabul edilen `BTreeMap` kelimesini kullandıysanız),
bu kelimeyi `ci/dictionary.txt` dosyasına eklemeniz gerekir (tutarlılık için sıralı sırayı koruyun).