# İdari Görevler

Bu dokümantasyon, depoyu yöneten herkesin
ara sıra bakım görevlerini nasıl yapacağını hatırlaması içindir.

## `rustc` sürümünü güncelleyin

- Hedef' dizininizi silin, zaten her şeyi yeniden derlemek üzeresiniz
- Sürüm numarasını `.github/workflows/main.yml` içinde değiştirin
- Sürüm numarasını `rust-toolchain` içinde değiştirin, bu da yerel olarak `rustup` ile kullandığınız
 sürümünü değiştirmelidir
- src/title-page.md` dosyasındaki sürüm numarasını değiştirin
- ./tools/update-rustc.sh` dosyasını çalıştırın (
 ne yaptığına ilişkin ayrıntılar için yorumlanmış koduna bakın)
- Değişiklikleri (git'e göre değiştirilen dosyalara bakarak) ve
 etkilerini (`tmp/book-before` ve
 `tmp/book-after` içindeki dosyalara bakarak) inceleyin ve iyi görünüyorlarsa işleyin
- Manual-regeneration` için arama yapın ve bir komut dosyası tarafından oluşturulamayan
 güncelleme çıktısı için bu yerlerdeki talimatları izleyin

## Tüm listelerdeki `edition`ı güncelleyin

Tüm listelerin `Cargo.toml`larındaki `edition = "[year]"` meta verilerini güncellemek için
`./tools/update-editions.sh` betiğini çalıştırın. makul göründüğünden emin olmak için farkı kontrol edin ve özellikle güncellemelerin metinde herhangi bir
değişikliği gerektirip gerektirmediğini kontrol edin. Ardından değişiklikleri işleyin.

## mdBook yapılandırmasında `edition`ı güncelleyin

`book.toml` ve `nostarch/book.toml` dosyalarını açın ve
`[rust]` tablosundaki `edition` değerini yeni baskıya ayarlayın.

## Listelerin yeni bir sürümünü yayınlayın

Artık
adresindeki her listeyi içeren eksiksiz projelerin `.tar` dosyalarını [GitHub Sürümleri olarak](https://github.com/rust-lang/book/releases) kullanılabilir hale getiriyoruz. Örneğin
düzenlemeler veya Rust ve `rustfmt` güncellemeleri nedeniyle kod değişiklikleri olduysa
yeni bir sürüm eseri oluşturmak için aşağıdakileri yapın:

- Sürüm için bir git etiketi oluşturun ve GitHub'a gönderin veya GitHub kullanıcı arayüzüne gidip [yeni bir sürüm taslağı](https://github.com/rust-lang/book/releases/new) ve mevcut bir etiketi seçmek yerine yeni bir
 etiketi girerek
 yeni bir etiket oluşturun
-
 `tmp/listings.tar.gz` dosyasını oluşturacak olan `cargo run --bin release_listings` dosyasını çalıştırın
- Taslak sürüm için GitHub kullanıcı arayüzünde `tmp/listings.tar.gz` dosyasını yükleyin
- Bülteni yayınlayın

## Yeni bir liste ekle

Tüm listeler üzerinde `rustfmt` çalıştıran, derleyici güncellendiğinde
çıktısını güncelleyen ve listeler için
tam projeler içeren sürüm eserleri üreten komut dosyalarını kolaylaştırmak için, en önemsiz olanın ötesindeki tüm listeler
bir dosyaya çıkarılmalıdır. Bunu yapmak için:

- Yeni listelemenin `listings` dizininde nereye gitmesi gerektiğini bulun.
  - Her bölüm için bir alt dizin vardır
  - Numaralandırılmış listeler
 dizin adları için `listing-[chapter num]-[listing num]` kullanmalıdır.
  - Numarasız listeler `no-listing-` ile başlamalı ve ardından bölümdeki numarasız diğer
 listelerine göre bölümdeki konumunu belirten bir
 numarası ve ardından
 birinin aradığı kodu bulmak için okuyabileceği kısa bir açıklama gelmelidir.
  - Yalnızca kodun çıktısını görüntülemek için kullanılan listeler (örneğin,
 adresinde "y yerine x yazsaydık, bu derleyici
 hatasını alırdık:" deriz, ancak aslında x kodunu göstermeyiz)
 `output-only-` ile adlandırılmalı ve ardından yalnızca çıktı için kullanılan diğer listelere göre
 bölümündeki konumunu belirten bir sayı, ardından yazarların veya katkıda bulunanların aradıkları kodu
 bulmak için okuyabilecekleri kısa bir
 açıklaması gelmelidir.
  - **Çevreleyen liste numaralarını uygun şekilde ayarlamayı unutmayın!**
- Bu dizinde `cargo new` kullanarak ya da
 adresinden başka bir listeyi başlangıç noktası olarak kopyalayarak tam bir Cargo projesi oluşturun.
- Tam çalışan bir örnek oluşturmak için gereken kodu ve çevresindeki kodları ekleyin.
- Dosyadaki kodun yalnızca bir kısmını göstermek istiyorsanız,
 dosyasının göstermek istediğiniz kısımlarını işaretlemek için
 (`// ANCHOR: some_tag` ve `// ANCHOR_END: some_tag`) çapa yorumlarını kullanın.
- Rust kodu için, metindeki kod blokları içinde `{#rustdoc_include [filename:some_tag]}}` yönergesini
 kullanın. `rustdoc_include` yönergesi
 adresine `mdbook test` amacıyla `rustdoc` için görüntülenmeyen kodu verir.
- Başka bir şey için `{#include [filename:some_tag]}}` yönergesini kullanın.
- Bir komutun çıktısını metin içinde de görüntülemek istiyorsanız, listenin dizininde aşağıdaki gibi bir
 `output.txt` dosyası oluşturun:
  - Komutu çalıştırın, örneğin `cargo run` veya `cargo test`, ve
 çıktısının tamamını kopyalayın.
  - İlk satırı `$ [çalıştırdığınız komut]` olan yeni bir `output.txt` dosyası oluşturun.
  - Az önce kopyaladığınız çıktıyı yapıştırın.
  - Derleyici çıktısını
 adresinde biraz normalleştirme yapması gereken `./tools/update-rustc.sh` komutunu çalıştırın.
  - Çıktıyı `{#include [filename]}}` yönergesi ile metne dahil edin.
  - output.txt dosyasını ekleyin ve işleyin.
- Çıktıyı görüntülemek istiyorsanız ancak herhangi bir nedenle
 betiği tarafından oluşturulamıyorsa (örneğin, kullanıcı girdisi veya web
 isteği yapmak gibi harici olaylar nedeniyle), çıktıyı satır içi tutun ancak
 `manual-regeneration` ve satır içi
 çıktısını manuel olarak güncellemek için talimatlar içeren bir yorum yapın.
- Bu örneğin
 `rustfmt` tarafından biçimlendirilmeye çalışılmasını bile istemiyorsanız (örneğin, örnek bilerek ayrıştırılmadığı için), listenin dizinine bir
 `rustfmt-ignore` dosyası ekleyin ve
 'nin bu dosyanın içeriği olarak biçimlendirilmemesinin nedenini (
 'un bir gün düzeltebileceği bir rustfmt hatası olması durumunda) ekleyin.

## Bazı değişikliklerin işlenen kitap üzerindeki etkisini görün

Örneğin, `mdbook` güncellemesini veya dosyaların dahil edilme şeklini değiştirmeyi kontrol etmek için:

- Test etmek istediğiniz değişiklikten önce `mdbook build -d tmp/book-before` komutunu çalıştırarak oluşturulmuş bir kitap oluşturun
- Test etmek istediğiniz değişiklikleri uygulayın ve `mdbook build -d tmp/book-after` komutunu çalıştırın
- `./tools/megadiff.sh` dosyasını çalıştırın
- `tmp/book-before` ve `tmp/book-after` içinde kalan dosyaların farkları vardır
 favori diff görüntüleme mekanizmanızla manuel olarak inceleyebilirsiniz

## No Starch için yeni markdown dosyaları üretin

- ./tools/nostarch.sh` dosyasını çalıştırın
- Komut dosyasının `nostarch` dizininde oluşturduğu dosyaları kontrol edin
- Bir düzenleme turuna başlıyorsanız bunları git'te kontrol edin

## Farklılaştırma için docx'ten markdown üretin

- Docx dosyasını `tmp/chapterXX.docx` olarak kaydedin.
- Word'de gözden geçir sekmesine gidin, "Tüm değişiklikleri kabul et ve izlemeyi durdur "u seçin
- Docx dosyasını tekrar kaydedin ve Word'ü kapatın
- `./tools/doc-to-md.sh` dosyasını çalıştırın
- Bu `nostarch/chapterXX.md` yazmalıdır. `XSL`i
 `tools/doc-to-md.xsl` içinde ayarlayın ve gerekirse `./tools/doc-to-md.sh` dosyasını tekrar çalıştırın.

## Graphviz noktası oluşturun

kitabındaki bazı diyagramlar için [Graphviz](http://graphviz.org/) kullanıyoruz. Bu dosyaların kaynağı `dot` dizininde bulunmaktadır. Bir `dot`
dosyasını, örneğin `dot/trpl04-01.dot` dosyasını bir `svg`ye dönüştürmek için, çalıştırın:

```bash
$ dot dot/trpl04-01.dot -Tsvg > src/img/trpl04-01.svg
```

Oluşturulan SVG'de, `svg`
öğesinden genişlik ve yükseklik niteliklerini kaldırın ve `viewBox` niteliğini `0.00 0.00 1000.00 1000.00` veya görüntüyü kesmeyen başka bir
değerine ayarlayın.

## GitHub Sayfalarında bir önizleme yayınlayın

Devam eden önizlemeler için bazen GitHub Sayfalarında yayınlarız. Yayınlama için önerilen
akışı şöyledir:

- `hyp install ghp-import` (veya [pipx][pipx] kullanarak `pipx install ghp-import`) çalıştırarak `ghp-import` aracını yükleyin.
- Kök dizinde `tools/generate-preview.sh` dosyasını çalıştırın

[pipx]: https://pipx.pypa.io/stable/#install-pipx
