## Performans Karşılaştırması: Döngüler ve Yineleyiciler

Döngüleri mi yoksa yineleyicileri mi kullanmanız gerektiğine karar vermek için, hangi uygulamanın daha hızlı olduğunu bilmeniz gerekir: `search` fonksiyonunun açık bir `for` döngüsüyle yazılan sürümü mü, yoksa yineleyicilerle yazılan sürümü mü?

Bir karşılaştırma yapmak için Sir Arthur Conan Doyle'un _The Adventures of Sherlock Holmes_ kitabının tüm içeriğini bir `String`'e yükleyip, içerikte _the_ kelimesini aradık. İşte `search` fonksiyonunun `for` döngüsü kullanan sürümü ile yineleyici kullanan sürümünün karşılaştırma sonuçları:

```text
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

İki uygulamanın da performansı oldukça benzer! Burada karşılaştırma kodunu açıklamıyoruz çünkü amacımız iki sürümün eşdeğer olduğunu kanıtlamak değil, bu iki uygulamanın performans açısından nasıl karşılaştırıldığını genel olarak görmek.

Daha kapsamlı bir karşılaştırma için, `contents` olarak farklı boyutlarda ve farklı metinler, `query` olarak farklı kelimeler ve farklı uzunlukta kelimeler ve başka pek çok varyasyonla test yapmalısınız. Buradaki önemli nokta şu: yineleyiciler yüksek seviyeli bir soyutlama olmasına rağmen, derleyici tarafından neredeyse sizin elle yazacağınız düşük seviyeli kodla aynı şekilde derlenir. Yineleyiciler, Rust'ın _sıfır maliyetli soyutlamalarından_ biridir; yani bu soyutlamayı kullanmak, çalışma zamanında ek bir maliyet getirmez. Bu, C++'ın orijinal tasarımcısı ve geliştiricisi Bjarne Stroustrup'un “Foundations of C++” (2012) kitabında _sıfır ek yük_ ilkesini tanımlamasına benzer:

> Genel olarak, C++ uygulamaları sıfır ek yük ilkesine uyar: Kullanmadığınız şeyin bedelini ödemezsiniz. Ve dahası: Kullandığınız şeyi, elle kodlasanız daha iyi yapamazsınız.

Çoğu durumda, yineleyiciler kullanan Rust kodu, elle yazacağınız assembly koduyla aynı şekilde derlenir. Döngü açma (loop unrolling) ve dizi erişiminde sınır kontrolünü kaldırma gibi optimizasyonlar uygulanır ve ortaya çıkan kod son derece verimli olur. Artık bunu bildiğinize göre, yineleyicileri ve kapanışları korkmadan kullanabilirsiniz! Kodunuzu daha yüksek seviyeli gösterirler, ancak bunu yaparken çalışma zamanında performans kaybı yaşatmazlar.

## Özet

Kapanışlar ve yineleyiciler, Rust'ın fonksiyonel programlama dillerinden esinlenen özellikleridir. Bu özellikler, Rust'ın yüksek seviyeli fikirleri düşük seviyeli performansla açıkça ifade etme yeteneğine katkıda bulunur. Kapanışların ve yineleyicilerin uygulamaları, çalışma zamanı performansını etkilemeyecek şekilde tasarlanmıştır. Bu, Rust'ın sıfır maliyetli soyutlamalar sağlama hedefinin bir parçasıdır.

Artık G/Ç projemizin ifade gücünü artırdığımıza göre, projemizi dünyayla paylaşmamıza yardımcı olacak bazı `cargo` özelliklerine göz atalım.
