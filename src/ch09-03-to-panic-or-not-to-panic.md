## `panic!` yapmak ya da `panic!` yapmamak

Peki ne zaman `panic!` çağırmanız ve ne zaman
`Result` döndürmeniz gerektiğine nasıl karar verirsiniz? Kod paniklediğinde, kurtarmanın bir yolu yoktur. Kurtarmanın olası bir yolu olsun ya da olmasın, herhangi bir hata durumu için `panic!`
diyebilirsiniz, ancak
o zaman bir durumun kurtarılamaz olduğuna
çağıran kod adına karar vermiş olursunuz. Bir `Result` değeri döndürmeyi seçtiğinizde,
çağıran koda seçenekler sunmuş olursunuz. Çağıran kod, kendi durumuna uygun bir
şekilde kurtarmayı deneyebilir veya bu durumda bir `Err`
değerinin kurtarılamaz olduğuna karar verebilir, böylece `panic!` çağırabilir ve
kurtarılabilir hatanızı kurtarılamaz bir hataya dönüştürebilir. Bu nedenle, başarısız olabilecek bir fonksiyon tanımlarken `Result` değerini döndürmek
iyi bir varsayılan seçimdir.

Örnekler, prototip kodu ve testler gibi durumlarda, bir `Result` döndürmek yerine panikleyen kod yazmak daha
uygundur. Nedenini
inceleyelim, ardından derleyicinin
başarısızlığının imkansız olduğunu söyleyemediği, ancak bir insan olarak sizin söyleyebileceğiniz durumları tartışalım. Bölüm, kütüphane kodunda panik yapıp yapmamaya nasıl karar verileceğine ilişkin bazı genel yönergeler
ile sona erecektir.

### Örnekler, Prototip Kod ve Testler

Bir kavramı açıklamak için bir örnek yazarken,
sağlam hata işleme kodunu da dahil etmek örneği daha az anlaşılır hale getirebilir. Örneklerde,
`unwrap` gibi panik yaratabilecek bir yönteme yapılan çağrının, uygulamanızın hataları nasıl ele almasını istediğinize dair bir
yer tutucu olduğu anlaşılmaktadır; bu da
kodunuzun geri kalanının ne yaptığına bağlı olarak farklılık gösterebilir.

Benzer şekilde, `unwrap` ve `expect` yöntemleri, hataları nasıl ele alacağınıza karar vermeye hazır olmadan önce
prototip oluştururken çok kullanışlıdır. Programınızı daha sağlam hale getirmeye hazır olduğunuzda kodunuz için
adresinde net işaretler bırakırlar.

Bir testte bir yöntem çağrısı başarısız olursa,
bu yöntem test edilen işlevsellik olmasa bile tüm testin başarısız olmasını istersiniz. Çünkü `panic!` bir testin
nasıl başarısız olarak işaretlendiğini gösterir, `unwrap` veya `expect` çağrısı tam olarak
olması gereken şeydir.

### Derleyiciden Daha Fazla Bilgiye Sahip Olduğunuz Durumlar

Ayrıca, `Result` değerinin `Ok` değerine sahip olmasını sağlayan
başka bir mantığınız olduğunda da `expect` çağrısı yapmak uygun olacaktır, ancak mantık
derleyicinin anlayacağı bir şey değildir. Hala
işlemeniz gereken bir `Result` değerine sahip olacaksınız: çağırdığınız işlem her ne olursa olsun, sizin özel
durumunuzda mantıksal olarak imkansız olsa bile, genel olarak
başarısız olma olasılığı vardır. Kodu manuel olarak inceleyerek
asla bir `Err` varyantına sahip olmayacağınızdan emin olabiliyorsanız, `expect` çağrısı yapmak ve argüman metninde asla bir `Err` varyantına sahip olmayacağınızı düşünmenizin nedenini
belgelemek tamamen kabul edilebilir.
İşte bir örnek:

```rust
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-08-unwrap-that-cant-fail/src/main.rs:here}}
```

Kodlanmış bir dizeyi ayrıştırarak bir `IpAddr` örneği oluşturuyoruz. adresinde `127.0.0.1`in geçerli bir IP adresi olduğunu görebiliyoruz, bu nedenle burada `expect`
kullanmak kabul edilebilir. Ancak, sabit kodlanmış geçerli bir dizeye sahip olmak `parse` yönteminin dönüş türünü
değiştirmez: hala bir `Result` değeri alırız ve derleyici
hala `Result` değerini `Err` varyantı bir olasılıkmış gibi ele almamızı sağlar
çünkü derleyici bu dizenin her zaman
geçerli bir IP adresi olduğunu görecek kadar akıllı değildir. Eğer IP adresi dizesi
programa kodlanmak yerine bir kullanıcıdan gelseydi ve bu nedenle başarısızlık olasılığı olsaydı
`Sonuç`u kesinlikle daha sağlam bir şekilde ele almak isterdik.
Bu IP adresinin sabit kodlu olduğu varsayımından bahsetmek, gelecekte IP adresini
yerine başka bir kaynaktan almamız gerekirse
`expect` kodunu daha iyi bir hata işleme koduyla değiştirmemizi sağlayacaktır.

### Hata İşleme Yönergeleri

Kodunuzun
kötü bir duruma düşmesi mümkün olduğunda kodunuzun paniklemesini sağlamanız tavsiye edilir. Bu bağlamda, bir _kötü durum_ geçersiz değerler,
çelişkili değerler veya eksik değerlerin kodunuza aktarılması gibi bazı varsayımların,
garantilerin, sözleşmelerin veya değişmezlerin ihlal edilmesi ve ayrıca aşağıdakilerden biri veya
daha fazlasıdır:

- Kötü durum, kullanıcının verileri yanlış
 biçiminde girmesi gibi
 ara sıra gerçekleşebilecek bir şeyin aksine beklenmedik bir şeydir.
- Bu noktadan sonra kodunuzun her adımda sorunu kontrol etmek yerine
 bu kötü durumda olmamaya güvenmesi gerekir.
- Kullandığınız türlerde bu bilgiyi kodlamanın iyi bir yolu yoktur. Ne demek istediğimizi
 Bölüm 18'de [“Durumları ve Davranışı
 Türleri Olarak Kodlama”][encoding]<!-- ignore --> kısmında bir örnekle açıklayacağız.

Eğer birisi kodunuzu çağırır ve mantıklı olmayan değerler girerse,
en iyisi hata döndürmektir, böylece kütüphane kullanıcısı bu durumda ne
yapmak istediğine karar verebilir. Bununla birlikte, devam etmenin
güvensiz veya zararlı olabileceği durumlarda, en iyi seçim `panic!` çağrısı yapmak ve kütüphanenizi kullanan
kişisini kodlarındaki hata konusunda uyarmak olabilir, böylece
geliştirme sırasında düzeltebilirler. Benzer şekilde, kontrolünüz dışında olan
harici kodu çağırıyorsanız ve
düzeltme imkanınız olmayan geçersiz bir durum döndürüyorsa, `panic!` genellikle uygundur.

Ancak, başarısızlık beklendiğinde, bir `panic!` çağrısı yapmaktansa bir `Result`
döndürmek daha uygundur. Örnekler arasında bir ayrıştırıcıya hatalı biçimlendirilmiş
verisi verilmesi veya bir HTTP isteğinin
hız sınırına ulaştığınızı gösteren bir durum döndürmesi yer alır. Bu durumlarda, bir `Result` döndürmek, başarısızlığın
beklenen bir olasılık olduğunu ve çağıran kodun nasıl ele alınacağına karar vermesi gerektiğini gösterir.

Kodunuz
geçersiz değerler kullanılarak çağrıldığında kullanıcıyı riske atabilecek bir işlem gerçekleştirdiğinde
kodunuz önce değerlerin geçerli olduğunu doğrulamalı değerler geçerli değilse paniklemelidir. Bu çoğunlukla güvenlik nedenleriyle yapılır:
geçersiz veriler üzerinde işlem yapmaya çalışmak kodunuzu güvenlik açıklarına maruz bırakabilir.
Standart kütüphanenin
sınır dışı bir bellek erişimi denediğinizde `panic!` çağrısı yapmasının ana nedeni budur:
mevcut veri yapısına ait olmayan belleğe erişmeye çalışmak yaygın bir güvenlik sorunudur. Fonksiyonların genellikle
_sözleşmeleri_ vardır: davranışları yalnızca girdilerin belirli
gereksinimlerini karşılaması durumunda garanti edilir. Sözleşme ihlal edildiğinde paniklemek mantıklıdır çünkü
sözleşme ihlali her zaman çağıran tarafında bir hata olduğunu gösterir ve çağıran kodun açıkça ele almasını istediğiniz bir tür
hatası değildir. Aslında
çağıran kodun bunu düzeltmesi için makul bir yol yoktur; çağıran _programcıların_ kodu düzeltmek için
adresine ihtiyacı vardır. Bir işlev için sözleşmeler, özellikle de bir ihlalin
paniğe neden olacağı durumlarda, işlevin API belgelerinde açıklanmalıdır.

Bununla birlikte, tüm fonksiyonlarınızda çok sayıda hata kontrolü yapmak ayrıntılı
ve can sıkıcı olacaktır. Neyse ki, Rust'ın tip sistemini (ve dolayısıyla derleyici tarafından yapılan
tip kontrolünü) kontrollerin çoğunu sizin yerinize yapmak için kullanabilirsiniz. Eğer
fonksiyonunuz parametre olarak belirli bir tipe sahipse, derleyicinin zaten geçerli bir değere sahip olduğunuzdan emin olduğunu bilerek kodunuzun
mantığı ile devam edebilirsiniz. Örneğin
için, bir `Option` yerine bir türünüz varsa, programınız
_nothing_ yerine _something_ olmasını bekler. Bu durumda kodunuz
`Some` ve `None` varyantları için iki durumla uğraşmak zorunda kalmaz:
için kesinlikle bir değere sahip olan tek bir durum olacaktır. İşlevinize hiçbir şey iletmemeye çalışan kod
derlenmeyecektir, bu nedenle işlevinizin çalışma zamanında bu durumu kontrol etmesi gerekmez.
Başka bir örnek de
parametrenin asla negatif olmamasını sağlayan `u32` gibi işaretsiz bir tamsayı türü kullanmaktır.

### Doğrulama için Özel Tipler Oluşturma

Geçerli bir değere sahip olduğumuzdan emin olmak için Rust'ın tip sistemini kullanma fikrini
bir adım daha ileri götürelim ve doğrulama için özel bir tip oluşturmaya bakalım. Bölüm 2'deki
tahmin oyununu hatırlayın; kodumuz kullanıcıdan
1 ile 100 arasında bir sayı tahmin etmesini istemişti. Gizli sayımızla karşılaştırmadan önce kullanıcının tahmininin bu
sayılar arasında olduğunu doğrulamadık; yalnızca
tahminin olumlu olduğunu doğruladık. Bu durumda sonuçlar çok vahim değildi:
adresimizin “Çok yüksek” veya “Çok düşük” çıktısı yine de doğru olacaktı. Ancak, kullanıcıyı geçerli tahminlere yönlendirmek ve kullanıcı aralık dışında bir sayı tahmin ettiğinde farklı
davranışına sahip olmak ve
kullanıcısının bunun yerine örneğin harfler yazması
yararlı bir geliştirme olacaktır.

Bunu yapmanın bir yolu, potansiyel olarak negatif sayılara izin vermek için tahmini yalnızca
`u32` yerine bir `i32` olarak ayrıştırmak ve ardından
sayısının aralıkta olup olmadığına dair bir kontrol eklemek olabilir:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-09-guess-out-of-range/src/main.rs:here}}
```

</Listing>

`if` ifadesi değerimizin aralık dışında olup olmadığını kontrol eder, kullanıcıya
sorun hakkında bilgi verir ve
döngüsünün bir sonraki iterasyonunu başlatmak ve başka bir tahmin istemek için `continue` ifadesini çağırır. `if` ifadesinden sonra, `guess` değerinin
1 ile 100 arasında olduğunu bilerek `guess` ile gizli sayı arasındaki
karşılaştırmalarına devam edebiliriz.

Ancak, bu ideal bir çözüm değildir:
programının yalnızca 1 ile 100 arasındaki değerler üzerinde çalışması kesinlikle kritik olsaydı ve bu gereksinime sahip birçok işlevi
olsaydı, her işlevde bunun gibi bir kontrol yapmak
sıkıcı olurdu (ve performansı etkileyebilir).

Bunun yerine, özel bir modülde yeni bir tür oluşturabilir ve doğrulamaları
doğrulamalarını her yerde tekrarlamak yerine türün bir örneğini oluşturmak için
işlevine koyabiliriz. Bu şekilde, işlevlerin yeni türü
imzalarında kullanmaları ve aldıkları değerleri güvenle kullanmaları güvenli olur. Liste 9-13,
yalnızca
`new` fonksiyonu 1 ile 100 arasında bir değer alırsa bir `Guess` örneği oluşturacak bir `Guess` türü tanımlamanın bir yolunu göstermektedir.

<Listing number="9-13" caption="A `Guess` type that will only continue with values between 1 and 100" file-name="src/guessing_game.rs">

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-13/src/guessing_game.rs}}
```

</Listing>

src/guessing_game.rs* dosyasındaki bu kodun, burada göstermediğimiz *src/lib.rs* dosyasındaki `mod guessing_game;` modül
bildiriminin eklenmesine bağlı olduğunu unutmayın.
Bu yeni modülün dosyası içinde `Guess`
adında, `i32` tutan `value` adında bir alana sahip bir yapı tanımlıyoruz. Burası
sayısının saklanacağı yerdir.

Daha sonra, `Guess` değerlerinin
örneklerini oluşturan `Guess` üzerinde `new` adlı ilişkili bir işlev uygularız. `new` fonksiyonu, `i32` tipinde `value` adında bir
parametresine sahip olacak ve bir `Guess` döndürecek şekilde tanımlanmıştır. `new` fonksiyonunun
gövdesindeki kod, 1 ile 100 arasında olduğundan emin olmak için `value` değerini test eder.
Eğer `value` bu testi geçemezse, bir `panic!` çağrısı yaparız, bu da çağıran kodu yazan programcıyı
düzeltmesi için
adresine ihtiyaç duydukları bir hata olduğu konusunda uyarır, çünkü bu aralığın dışında bir `value` ile bir `Guess` oluşturmak
`Guess::new` fonksiyonunun dayandığı sözleşmeyi ihlal edecektir. `Guess::new` uygulamasının panik yapabileceği koşullar halka açık API
dokümantasyonunda tartışılmalıdır; Bölüm 14'te oluşturacağınız API dokümantasyonunda bir `panik!` olasılığını
belirten dokümantasyon kurallarını ele alacağız. Eğer
`value` testi geçerse, `value` alanı
tarafından `value` parametresine ayarlanmış yeni bir `Guess` yaratırız ve `Guess`i geri döndürürüz.

Daha sonra, `self`i ödünç alan,
başka parametresi olmayan ve bir `i32` döndüren `value` adlı bir yöntem uyguluyoruz. Bu tür yöntemlere bazen
a _getter_ denir, çünkü amacı alanlarından bazı verileri almak ve
döndürmektir. Bu genel yöntem gereklidir çünkü `Guess`
yapısının `value` alanı özeldir. Değer alanının özel olması önemlidir, böylece `Guess` yapısını kullanan
kodunun değeri doğrudan ayarlamasına izin verilmez:
dışındaki kod `guessing_game` modülü _must_ bir
`Guess` örneği oluşturmak için `Guess::new` işlevini kullanmalıdır, böylece bir `Guess`in `Guess::new` işlevindeki koşullar tarafından kontrol edilmemiş bir
`değer`e sahip olmasının hiçbir yolu yoktur.

Parametresi olan ya da yalnızca 1 ile 100 arasındaki sayıları döndüren bir fonksiyon
imzasında
`i32` yerine bir `Guess` aldığını ya da döndürdüğünü bildirebilir ve gövdesinde herhangi bir ek kontrol yapması gerekmez.

## Özet

Rust'ın hata işleme özellikleri daha sağlam kod yazmanıza yardımcı olmak için tasarlanmıştır.
`panic!` makrosu, programınızın üstesinden gelemeyeceği bir durumda olduğunu belirtir ve
geçersiz veya
yanlış değerlerle devam etmeye çalışmak yerine işleme durmasını söylemenizi sağlar. `Result` enumu,
işlemlerinin kodunuzun kurtarabileceği bir şekilde başarısız olabileceğini belirtmek için Rust'ın tür sistemini kullanır. Kodunuzu çağıran koda, olası
başarı veya başarısızlığını da ele alması gerektiğini söylemek için
`Result` kullanabilirsiniz. Uygun
durumlarda `panic!` ve `Result` kullanmak, kaçınılmaz sorunlar karşısında kodunuzu daha güvenilir hale getirecektir.

Artık standart kütüphanenin
ile `Option` ve `Result` enumlarını kullanarak jenerikleri nasıl kullandığını gördüğünüze göre, jeneriklerin nasıl çalıştığından ve
bunları kodunuzda nasıl kullanabileceğinizden bahsedeceğiz.

[encoding]: ch18-03-oo-design-patterns.md#durumları-ve-davranışları-türler-olarak-kodlama
