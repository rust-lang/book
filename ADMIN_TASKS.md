# İdari Görevler

Bu belge, depoyu yöneten herkesin ara sıra yapılması gereken bakım görevlerini hatırlaması için hazırlanmıştır.

## `rustc` sürümünü güncelleyin

- `target` dizinini silin, zaten her şeyi yeniden derleyeceksiniz
- `.github/workflows/main.yml` dosyasındaki sürüm numarasını değiştirin
- `rust-toolchain` içindeki sürüm numarasını değiştirin, bu da `rustup` ile yerel olarak kullandığınız sürümü değiştirmelidir.

- `src/title-page.md` içindeki sürüm numarasını değiştirin.

- `./tools/update-rustc.sh` komutunu çalıştırın (ne yaptığına dair ayrıntılar için yorumlu koduna bakın).

- Değişiklikleri inceleyin (git'e göre değişen dosyalara bakarak) ve
- Değişiklikleri (git'e göre değiştirilen dosyalara bakarak) ve bunların etkilerini (`tmp/book-before` ve `tmp/book-after` içindeki dosyalara bakarak) inceleyin ve uygun görünüyorsa bunları kaydedin

- `manual-regeneration` komutunu çalıştırın ve bu komutun verdiği talimatları izleyerek komut dosyası tarafından oluşturulamayan çıktıları güncelleyin

## Tüm listelerde `edition` bilgisini güncelleyin

Tüm listelerin `Cargo.toml` dosyalarındaki `edition = “[yıl]”` meta verisini güncellemek için,
`./tools/update-editions.sh` komut dosyasını çalıştırın. Farklılıkları kontrol ederek makul göründüğünden emin olun ve özellikle güncellemelerin metinde herhangi bir değişiklik gerektirip gerektirmediğini kontrol edin.
Ardından değişiklikleri kaydedin.

## mdBook yapılandırmasında `edition`'ı güncelleyin


`book.toml` ve `nostarch/book.toml` dosyalarını açın ve `[rust]` tablosundaki `edition` değerini yeni sürüme ayarlayın.

## Listelerin yeni sürümünü yayınlayın

Artık, mevcut tüm listeleri içeren tam projelerin `.tar` dosyalarını
[GitHub Sürümleri](https://github.com/rust-lang/book/releases) olarak yayınlıyoruz. Örneğin, düzenlemeler veya Rust ve `rustfmt` güncellemeleri nedeniyle kod değişiklikleri olduysa, yeni bir sürüm artefaktı oluşturmak için aşağıdakileri yapın:

- Sürüm için bir git etiketi oluşturun ve GitHub'a gönderin veya GitHub kullanıcı arayüzüne giderek [yeni bir sürüm taslağı oluşturun](https://github.com/rust-lang/book).

- Sürüm için bir git etiketi oluşturun ve GitHub'a gönderin veya GitHub kullanıcı arayüzüne gidip [yeni bir sürüm taslağı oluşturun](https://github.com/rust-lang/book/releases/new) ve mevcut bir etiketi seçmek yerine yeni bir etiket girerek yeni bir etiket oluşturun.
  - `cargo run --bin release_listings` komutunu çalıştırın, bu komut
  `tmp/listings.tar.gz` dosyasını oluşturacaktır.
- `cargo run --bin release_listings` komutunu çalıştırın, bu komut `tmp/listings.tar.gz` dosyasını oluşturacaktır.
  `tmp/listings.tar.gz`
- Taslak sürüm için GitHub kullanıcı arayüzüne `tmp/listings.tar.gz` dosyasını yükleyin.
- Sürümü yayınlayın.

## Yeni bir liste ekleyin

Tüm listelerde `rustfmt` komutunu çalıştıran komut dosyalarını kolaylaştırmak, derleyici güncellendiğinde çıktıyı güncellemek ve listeler için tam projeleri içeren sürüm artefaktları üretmek için, en basit olanlar dışındaki tüm listeler bir dosyaya çıkarılmalıdır. Bunu yapmak için:
- Yeni dinlemenin `listelerinin` dizininde kaydedileceği yer bulun.
  - Her bölüm için bir alt dizin vardır.
  -Numaralı listeler, dizin adları için `listeleme-[bölüm numarası]-[liste numarası]`
    formatını kullanmalıdır.
  - Numaraz listeleyici, `no-listing-` ile başlamalı ve ardından
   bölü mleri nin diğerlerini listelere göre ayırarak bir numara
    ve sonrasında aradıkları kodu bulmak için okunabilecekler kısa bir açıklama
    gelmelidir.
  - Yalnızca kodun özelliklerini görüntülemek için kullanılan listeler ( örneğin,
    "y yerine x yazılmış olsaydı, şu derleyici hata alırdı
    hata alı : " ama aslında x kodunu göstermiyoruz) sadece çıktıyı göstermek için kullanılan listeleyici,
    ``output-only-`' ile başlamalı, sonraki bölüm diğer çıktı için kullanılan listelere göre yazan bir sayı
    ve daha sonra yazarların veya katkı bulunanların aradıkları kodu bulmak için okunabilecek kısa bir
    açıklama gelmelidir.
    -
- **Çevreleyen liste numaralarını uygun şekilde ayarlamayı unutmayın!**
- Bu dizinde, `cargo new` komutunu kullanarak veya
  başlangıç noktası olarak başka bir listeyi kopyalayarak tam bir Cargo projesi oluşturun.
- Tamamen çalışan bir örnek oluşturmak için gerekli olan kodu ve çevresindeki tüm kodları ekleyin.
- Dosyadaki kodun sadece bir kısmını göstermek istiyorsanız, gösterilmek istenen kısımları işaretlemek için bağlantı yorumları
  (`// ANCHOR: some_tag` ve `// ANCHOR_END: some_tag`) kullanın.
  .
- Rust kodu için, metindeki kod blokları içinde `{{#rustdoc_include [filename:some_tag]}}` yönergesini kullanın.
  `rustdoc_include` yönergesi, `mdbook test` amacıyla `rustdoc`'a gösterilmeyen
  kodu verir.
- Diğer her şey için `{{#include [filename:some_tag]}}` yönergesini kullanın.
- Bir komutun çıktısını da metinde görüntülemek istiyorsanız,
  listelemenin dizininde aşağıdaki gibi bir
  `output.txt` dosyası oluşturun:
- `cargo run` veya `cargo test` gibi komutu çalıştırın ve tüm
  çıktıyı kopyalayın.
- İlk satırı `$ [çalıştırdığınız komut]` olan yeni bir `output.txt` dosyası oluşturun.
-
  - Az önce kopyaladığınız çıktıyı yapıştırın.
  - `./tools/update-rustc.sh` komutunu çalıştırın, bu komut derleyici çıktısında bazı normalizasyonlar yapmalıdır
    .
  - Çıktıyı `{{#include [dosya adı]}}` yönergesi ile metne ekleyin.
  - output.txt dosyasını ekleyin ve kaydedin.
- Çıktıyı görüntülemek istiyorsanız ancak bir nedenden dolayı bir
  komut dosyası tarafından üretilemiyorsa (örneğin, kullanıcı girişi veya web
  isteği gibi harici olaylar nedeniyle), çıktıyı satır içi olarak tutun ancak
  `manual-regeneration` ve satır içi çıktıyı manuel olarak güncelleme talimatlarını
  içeren bir yorum ekleyin.
- Bu örneğin
  `rustfmt` tarafından biçimlendirilmesini istemiyorsanız (örneğin, örnek kasıtlı olarak ayrıştırılmıyorsa),
  listelemenin dizinine bir
  `rustfmt-ignore` dosyası ekleyin ve bu dosyanın içeriğine biçimlendirilememe nedenini yazın (bir gün düzeltilebilecek bir rustfmt hatası olması durumunda).

## Render edilmiş kitapta bazı değişikliklerin etkisini görün

Örneğin, `mdbook` güncellemesini veya dosyaların dahil edilme şeklini kontrol etmek için:

- Test etmek istediğiniz değişiklikten önce `mdbook
  build -d tmp/book-before` komutunu çalıştırarak oluşturulmuş bir kitap oluşturun
- Test etmek istediğiniz değişiklikleri uygulayın ve `mdbook build -d tmp/book-after` komutunu çalıştırın
- `./tools/megadiff.sh` komutunu çalıştırın
- `tmp/book-before` ve `tmp/book-after` dizinlerinde kalan dosyalar arasında farklar vardır.
  Bu farkları, tercih ettiğiniz fark görüntüleme mekanizmasıyla manuel olarak inceleyebilirsiniz

## No Starch için yeni markdown dosyaları oluşturun

- `./tools/nostarch.sh` komutunu çalıştırın
- Komut dosyasının `nostarch` dizininde oluşturduğu dosyaları kontrol edin
- Bir dizi düzenleme başlatıyorsanız, bunları git'e kaydedin

## Farklılıkları karşılaştırmak için docx dosyasından markdown oluşturun

- docx dosyasını `tmp/chapterXX.docx` dosyasına kaydedin.
- Word'de, gözden geçirme sekmesine gidin ve “Tüm değişiklikleri kabul et ve izlemeyi durdur” seçeneğini seçin
- docx dosyasını tekrar kaydedin ve Word'ü kapatın
- `./tools/doc-to-md.sh` komutunu çalıştırın
- Bu, `nostarch/chapterXX.md` dosyasını yazmalıdır.
  `tools/doc-to-md.xsl` içindeki XSL'yi ayarlayın ve gerekirse `./tools/doc-to-md.sh` komutunu tekrar çalıştırın.

## Graphviz dot oluşturma

Kitaptaki bazı diyagramlar için [Graphviz](http://graphviz.org/) kullanıyoruz.
Bu Bu dosyaların kaynağı `dot` dizininde bulunur. Bir `dot`
dosyasını, örneğin `dot/trpl04-01.dot` dosyasını `svg` dosyasına dönüştürmek için şunu çalıştırın:

```bash
$ dot dot/trpl04-01.dot -Tsvg > src/img/trpl04-01.svg
```

Oluşturulan SVG'de, `svg`
öğesinden genişlik ve yükseklik özniteliklerini kaldırın ve `viewBox` özniteliğini `0.00 0.00 1000.00 1000.00` veya görüntüyü kesmeyen diğer
değerlere ayarlayın.

## GitHub Pages'a önizleme yayınlayın

Bazen devam eden önizlemeler için GitHub Pages'a yayın yapıyoruz. Yayınlama için önerilen
akış şöyledir:

- `pip install ghp-import` (veya [pipx][pipx] kullanarak `pipx install ghp-import`) komutunu çalıştırarak `ghp-import` aracını yükleyin.
- Kök dizininde `tools/generate-preview.sh` komutunu çalıştırın.
[pipx]: https://pipx.pypa.io/stable/#install-pipx
