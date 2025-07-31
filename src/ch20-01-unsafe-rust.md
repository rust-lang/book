## Güvensiz Rust

Şimdiye kadar ele aldığımız tüm kodlarda, Rust'ın bellek güvenliği garantileri derleme zamanında zorunlu tutuldu. Ancak, Rust'ın içinde bu bellek güvenliği garantilerini zorunlu kılmayan ikinci bir dil daha vardır: buna _güvensiz Rust_ (unsafe Rust) denir ve normal Rust gibi çalışır; fakat bize ekstra "süper güçler" verir.

Güvensiz Rust'ın var olma nedeni, statik analiz doğası gereği temkinli olmasıdır. Derleyici, kodun garantileri sağlayıp sağlamadığını belirlemeye çalışırken, bazı geçerli programları reddetmesi, bazı geçersiz programları kabul etmesinden daha iyidir. Kod _muhtemelen_ doğru olsa da, Rust derleyicisi emin olmak için yeterli bilgiye sahip değilse kodu reddeder. Bu gibi durumlarda, güvensiz kod kullanarak derleyiciye "Bana güven, ne yaptığımı biliyorum" diyebilirsiniz. Ancak dikkatli olun: güvensiz kodu yanlış kullanırsanız, null pointer dereference gibi bellek güvensizliğinden kaynaklanan sorunlar ortaya çıkabilir.

Rust'ın güvensiz bir alter egosunun olmasının bir diğer nedeni de, alttaki bilgisayar donanımının doğası gereği güvensiz olmasıdır. Rust size güvensiz işlemler yapma imkanı tanımasaydı, bazı görevleri gerçekleştiremezdiniz. Rust'ın hedeflerinden biri, işletim sistemiyle doğrudan etkileşim kurmak veya kendi işletim sisteminizi yazmak gibi düşük seviyeli sistem programlamasına olanak tanımaktır. Şimdi, güvensiz Rust ile neler yapabileceğimize ve bunu nasıl yapacağımıza bakalım.

### Güvensiz Süper Güçler

Güvensiz Rust'a geçmek için `unsafe` anahtar kelimesini kullanın ve ardından güvensiz kodu tutan yeni bir blok başlatın. Güvensiz Rust'ta, güvenli Rust'ta yapamayacağınız beş işlem yapabilirsiniz; bunlara _güvensiz süper güçler_ diyoruz. Bu süper güçler şunlardır:

- Ham bir işaretçiyi (raw pointer) dereference etmek
- Güvensiz bir fonksiyon veya metot çağırmak
- Değiştirilebilir (mutable) statik bir değişkene erişmek veya onu değiştirmek
- Güvensiz bir trait'i uygulamak
- Bir `union`'ın alanlarına erişmek

`unsafe`'ın, ödünç alma denetleyicisini (borrow checker) devre dışı bırakmadığını veya Rust'ın diğer güvenlik kontrollerini kapatmadığını anlamak önemlidir: Güvensiz kodda bir referans kullanırsanız, yine de kontrol edilir. `unsafe` anahtar kelimesi yalnızca bu beş özelliğe erişim sağlar; bu özellikler için derleyici bellek güvenliği kontrolü yapmaz. Yine de, bir `unsafe` bloğu içinde belirli bir güvenlik düzeyi elde edersiniz.

Ayrıca, `unsafe` bir bloğun içindeki kodun mutlaka tehlikeli olduğu veya kesinlikle bellek güvenliği sorunları içereceği anlamına gelmez: Amaç, programcı olarak, bir `unsafe` bloğun içindeki kodun belleğe geçerli şekilde erişmesini sağlamanızdır.

İnsanlar hata yapabilir ve hatalar olacaktır; ancak bu beş güvensiz işlemin `unsafe` ile işaretlenmiş bloklarda olmasını zorunlu kılarak, bellek güvenliğiyle ilgili hataların yalnızca `unsafe` bloklarda olacağını bilirsiniz. `unsafe` blokları küçük tutun; bellek hatalarını araştırırken buna minnettar olacaksınız.

Güvensiz kodu mümkün olduğunca izole etmek için, bu tür kodu güvenli bir soyutlama (abstraction) içine almak ve güvenli bir API sunmak en iyisidir; bunu, bu bölümün ilerleyen kısımlarında güvensiz fonksiyon ve metotları incelerken tartışacağız. Standart kütüphanenin bazı bölümleri, denetlenmiş güvensiz kodun üzerinde güvenli soyutlamalar olarak uygulanmıştır. Güvensiz kodu güvenli bir soyutlamaya sarmak, `unsafe` kullanımının, siz veya kullanıcılarınızın güvensiz kodla uygulanmış işlevselliği kullanmak isteyebileceği tüm yerlere sızmasını engeller; çünkü güvenli bir soyutlama kullanmak güvenlidir.

Şimdi, bu beş güvensiz süper gücün her birine tek tek bakalım. Ayrıca, güvensiz koda güvenli bir arayüz sağlayan bazı soyutlamalara da bakacağız.

### Ham İşaretçiyi Dereference Etmek

4. Bölümdeki ["Sarkan Referanslar"] [dangling-references]<!-- ignore --> başlığında, derleyicinin referansların her zaman geçerli olmasını sağladığından bahsetmiştik. Güvensiz Rust'ta, referanslara benzer iki yeni tür vardır: _ham işaretçiler_ (raw pointers). Referanslar gibi, ham işaretçiler de değiştirilemez (immutable) veya değiştirilebilir (mutable) olabilir ve sırasıyla `*const T` ve `*mut T` olarak yazılır. Yıldız işareti burada dereference operatörü değil, tip adının bir parçasıdır. Ham işaretçiler bağlamında, _değiştirilemez_ demek, işaretçi dereference edildikten sonra doğrudan atanamayacağı anlamına gelir.

Referanslar ve akıllı işaretçilerden farklı olarak, ham işaretçiler:

- Hem değiştirilemez hem de değiştirilebilir işaretçilerin aynı anda var olmasına izin verir (ödünç alma kurallarını yok sayabilir)
- Geçerli bir belleğe işaret etme garantisi yoktur
- Null olmasına izin verilir
- Herhangi bir otomatik temizleme uygulamaz

Rust'ın bu garantileri zorunlu kılmaktan vazgeçmesiyle, daha büyük bir performans veya Rust'ın garantilerinin geçerli olmadığı başka bir dil veya donanımla arayüz kurma yeteneği gibi avantajlardan feragat edersiniz.

Şimdi, geçerli bir bellek adresine işaret eden bir ham işaretçi oluşturmayı göstereceğiz. Liste 20-1, bir sabit ve bir değişken için nasıl ham işaretçi oluşturulacağını göstermektedir.

<Listing number="20-1" caption="Ham işaretçileri oluşturma">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-01/src/main.rs:here}}
```

</Listing>

Bu kodda `unsafe` anahtar kelimesini kullanmadığımıza dikkat edin. Ham işaretçileri güvenli kodda oluşturabiliriz; ancak, birazdan göreceğiniz gibi, ham işaretçileri dereference edemeyiz.

Ham işaretçileri, ham ödünç alma operatörlerini kullanarak oluşturduk: `&raw const num` bir `*const i32` değişmez ham işaretçisi oluşturur ve `&raw mut num` bir `*mut i32` değişken ham işaretçisi oluşturur. Onları doğrudan yerel bir değişkenden oluşturduğumuz için, bu belirli ham işaretçilerinin geçerli olduğunu biliyoruz, ancak herhangi bir ham işaretçisinin geçerli olduğu varsayımında bulunamayız.

Bunu göstermek için, bir değeri dönüştürmek yerine `as` kullanarak geçerliliğinden bu kadar emin olamayacağımız bir ham işaretçisi oluşturacağız. Liste 20-2, bellekteki keyfi bir konuma nasıl ham işaretçi oluşturulacağını göstermektedir. Keyfi belleği kullanmaya çalışmak tanımsızdır: o adreste veri olabilir veya olmayabilir, derleyici kodu optimize edebilir, böylece bellek erişimi olmaz veya program bir segmentasyon hatasıyla sonlanabilir. Genellikle, özellikle bunun yerine bir ham ödünç alma operatörü kullanabileceğiniz durumlarda, böyle bir kod yazmak için iyi bir neden yoktur, ancak mümkündür.

<Listing number="20-2" caption="Keyfi bir bellek adresine ham işaretçi oluşturma">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-02/src/main.rs:here}}
```

</Listing>

Ham işaretçileri güvenli kodda oluşturabiliriz, ancak geçerli bir değere işaret eden bir ham işaretçisini dereference edemeyiz. Liste 20-3'te, bir ham işaretçisini dereference etmek için dereference operatörü `*` kullanıyoruz; bu, bir `unsafe` bloğu gerektiriyor.

<Listing number="20-3" caption="Ham işaretçileri bir `unsafe` bloğu içinde dereference etme">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-03/src/main.rs:here}}
```

</Listing>

Bir işaretçi oluşturmak zarara yol açmaz; yalnızca işaret ettiği değere erişmeye çalıştığımızda geçersiz bir değerle karşılaşabiliriz.

Ayrıca, Liste 20-1 ve 20-3'te, her ikisi de `num`'un saklandığı aynı bellek konumuna işaret eden `*const i32` ve `*mut i32` ham işaretçileri oluşturduk. Eğer `num` için bir değişmez ve bir değişken referansı oluşturmaya çalışsaydık, kod derlenmezdi çünkü Rust'ın sahiplik kuralları, herhangi bir değişmez referansla aynı anda bir değişken referansına izin vermez. Ham işaretçilerle, aynı konuma bir değişken işaretçisi ve bir değişmez işaretçi oluşturabiliriz ve verileri değişken işaretçisi aracılığıyla değiştirebiliriz; bu da potansiyel olarak bir veri yarışı oluşturur. Dikkatli olun!

Tüm bu tehlikelere rağmen, ham işaretçileri neden kullanırsınız? Bir ana kullanım durumu, C kodu ile arayüz kurarken, bir diğeri ise ödünç alma denetleyicisinin anlamadığı güvenli soyutlamalar oluştururken ortaya çıkar. Şimdi, güvensiz fonksiyonlar tanımlayıp, ardından güvensiz kod kullanan bir güvenli soyutlama örneğine bakalım.

### Güvensiz Bir Fonksiyonu veya Metodu Çağırmak

Bir güvensiz blokta gerçekleştirebileceğiniz ikinci tür işlem, güvensiz fonksiyonları çağırmaktır. Güvensiz fonksiyonlar ve metotlar, normal fonksiyonlar ve metotlar gibi görünür, ancak tanımının önünde ekstra bir `unsafe` anahtar kelimesi vardır. Bu bağlamda `unsafe` anahtar kelimesi, fonksiyonun çağrıldığında yerine getirilmesi gereken bazı gereksinimlere sahip olduğunu belirtir; çünkü Rust bu gereksinimlerin karşılandığını garanti edemez. Bir `unsafe` bloğu içinde bir güvensiz fonksiyonu çağırarak, bu fonksiyonun belgelerini okuduğumuzu ve bu fonksiyonu doğru bir şekilde kullanmayı anladığımızı, ayrıca fonksiyonun sözleşmesini yerine getirdiğimizi beyan etmiş oluruz.

İşte, gövdesinde hiçbir şey yapmayan `dangerous` adlı bir güvensiz fonksiyon:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-01-unsafe-fn/src/main.rs:here}}
```

`dangerous` fonksiyonunu ayrı bir `unsafe` bloğu içinde çağırmalıyız. Eğer `unsafe` bloğu olmadan `dangerous`'ı çağırmaya çalışırsak, bir hata alırız:

```console
{{#include ../listings/ch20-advanced-features/output-only-01-missing-unsafe/output.txt}}
```

`unsafe` bloğuyla, Rust'a fonksiyonun belgelerini okuduğumuzu, doğru bir şekilde kullanmayı anladığımızı ve fonksiyonun sözleşmesini yerine getirdiğimizi beyan ediyoruz.

Güvensiz bir fonksiyonun gövdesinde güvensiz işlemler gerçekleştirmek için, normal bir fonksiyonda olduğu gibi bir `unsafe` bloğu kullanmanız gerekir ve derleyici, bunu unutursanız sizi uyarır. Bu, güvensiz işlemlerin, işlemin tüm gövdesinde gerekli olmayabileceği gibi, `unsafe` bloklarının mümkün olduğunca küçük tutulmasına yardımcı olur.

#### Güvensiz Kod Üzerine Güvenli Bir Soyutlama Oluşturma

Bir fonksiyon güvensiz kod içeriyorsa, bu, tüm fonksiyonu güvensiz olarak işaretlememiz gerektiği anlamına gelmez. Aslında, güvensiz kodu güvenli bir fonksiyon içinde sarmak yaygın bir soyutlamadır. Örneğin, standart kütüphaneden `split_at_mut` fonksiyonunu inceleyelim; bu fonksiyon bazı güvensiz kodlar gerektirir. Bunu nasıl uygulayabileceğimizi keşfedeceğiz. Bu güvenli yöntem, değiştirilebilir dilimlerin (mutable slices) üzerinde tanımlanmıştır: bir dilimi alır ve bir kesme işlemiyle ikiye ayırır. Liste 20-4, `split_at_mut` kullanımını göstermektedir.

<Listing number="20-4" caption="Güvenli `split_at_mut` fonksiyonunu kullanma">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-04/src/main.rs:here}}
```

</Listing>

Bu fonksiyonu yalnızca güvenli Rust kullanarak uygulayamayız. Liste 20-5'te olduğu gibi, derlenmeyecek bir deneme yapabiliriz. Basitlik açısından, `split_at_mut`'ı bir fonksiyon olarak ve yalnızca `i32` değerlerinin dilimleri için uygulayacağız; bu nedenle, genel bir tür `T` yerine yalnızca `i32` için uygulayacağız.

<Listing number="20-5" caption="Sadece güvenli Rust kullanarak `split_at_mut` uygulama denemesi">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-05/src/main.rs:here}}
```

</Listing>

Bu fonksiyon önce dilimin toplam uzunluğunu alır. Ardından, verilen indeksin dilim içinde olup olmadığını kontrol ederek, dilimi o indekste bölmek için güvenli bir şekilde kullanılabilir olduğunu doğrular. Bu doğrulama, dilimi böleceğimiz zaman, verilen indeksin dilimin uzunluğundan büyük olmaması gerektiğini garanti eder.

Daha sonra, bir demet içinde iki değiştirilebilir dilim döndürür: biri orijinal dilimin başlangıcından `mid` indeksine kadar, diğeri ise `mid`'den dilimin sonuna kadar.

Liste 20-5'teki kodu derlemeye çalıştığımızda bir hata alırız.

```console
{{#include ../listings/ch20-advanced-features/listing-20-05/output.txt}}
```

Rust'ın ödünç alma denetleyicisi, dilimin farklı parçalarını ödünç aldığımızı anlamaz; sadece aynı dilimden iki kez ödünç aldığımızı bilir. Bir dilimin farklı parçalarını ödünç almak temelde sorun değildir çünkü iki dilim örtüşmez, ancak Rust bunu bilmek için yeterince akıllı değildir. Kodun doğru olduğunu bildiğimizde ama Rust'ın bilmediği durumlarda güvensiz koda başvururuz.

Liste 20-6, bir `unsafe` bloğu, bir ham işaretçi ve bazı güvensiz fonksiyon çağrıları kullanarak `split_at_mut`'ın nasıl uygulanacağını göstermektedir.

<Listing number="20-6" caption="`split_at_mut` fonksiyonunun uygulanmasında güvensiz kod kullanma">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-06/src/main.rs:here}}
```

</Listing>

Daha önce 4. Bölümde [“Dilim Türü”][the-slice-type]<!-- ignore --> başlığında belirttiğimiz gibi, dilimler bazı verilere ve dilimin uzunluğuna işaret eden bir işaretçidir. Bir dilimin uzunluğunu almak için `len` yöntemini ve bir dilimin ham işaretçisine erişmek için `as_mut_ptr` yöntemini kullanırız. Bu durumda, çünkü `i32` değerlerine işaret eden bir değiştirilebilir dilimimiz var, `as_mut_ptr` bir `*mut i32` türünde bir ham işaretçi döndürür; bu işaretçiyi `ptr` değişkeninde sakladık.

`mid` indeksinin dilim içinde olup olmadığını kontrol eden doğrulamayı koruyoruz. Ardından, güvensiz kısma geliyoruz: `slice::from_raw_parts_mut` fonksiyonu bir ham işaretçi ve bir uzunluk alır ve bir dilim oluşturur. `ptr`'dan başlayıp `mid` öğe uzunluğunda bir dilim oluşturmak için kullanıyoruz. Ardından, `ptr` üzerinde `mid` argümanıyla `add` yöntemini çağırarak `mid`'den başlayan bir ham işaretçi alıyoruz ve bu işaretçiyi kullanarak `mid`'den sonraki kalan öğe sayısı kadar bir dilim oluşturuyoruz.

`slice::from_raw_parts_mut` fonksiyonu, bir ham işaretçi aldığı için güvensizdir ve bu işaretçinin geçerli olduğunu doğrulamak zorundadır. Ham işaretçiler üzerindeki `add` yöntemi de güvensizdir çünkü ofset konumunun da geçerli bir işaretçi olduğunu varsayar. Bu nedenle, bu çağrıları yapabilmemiz için `unsafe` bloğu etrafında sarmamız gerekiyordu. Kodun bu kısmını incelediğimizde ve `mid`'in `len`'den küçük veya eşit olması gerektiğini belirten doğrulamayı eklediğimizde, `unsafe` bloğu içindeki tüm ham işaretçilerinin geçerli veri işaretçileri olduğunu söyleyebiliriz. Bu, `unsafe` kullanımının kabul edilebilir ve uygun bir örneğidir.

Liste 20-7'deki `slice::from_raw_parts_mut` kullanımının, dilim kullanıldığında muhtemelen çökmesine neden olacağını unutmayın. Bu kod, keyfi bir bellek konumundan başlayarak 10.000 öğe uzunluğunda bir dilim oluşturur.

<Listing number="20-7" caption="Keyfi bir bellek konumundan dilim oluşturma">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-07/src/main.rs:here}}
```

</Listing>

Bu keyfi konumda belleği biz sahiplenmiyoruz ve bu kodun oluşturduğu dilimin geçerli `i32` değerlerini içereceğine dair hiçbir garanti yok. `values`'ı geçerli bir dilim gibi kullanmaya çalışmak tanımsız davranışla sonuçlanır.

#### `extern` Fonksiyonlarını Kullanarak Harici Kodu Çağırma

Bazen, Rust kodunuzun başka bir dilde yazılmış kodla etkileşimde bulunması gerekebilir. Bunun için, Rust, bir _Yabancı Fonksiyon Arayüzü (FFI)_ oluşturmayı ve kullanmayı kolaylaştıran `extern` anahtar kelimesini sağlar. Bir FFI, bir programlama dilinin fonksiyonları tanımlayıp, bu fonksiyonları çağırabilmesi için başka bir (yabancı) programlama diline olanak tanıyan bir yoldur.

Liste 20-8, C standart kütüphanesindeki `abs` fonksiyonu ile entegrasyonu nasıl ayarlayacağınızı göstermektedir. `extern` blokları içinde tanımlanan fonksiyonlar genellikle Rust kodundan çağrıldığında güvensizdir, bu nedenle `extern` blokları da `unsafe` ile işaretlenmelidir. Bunun nedeni, diğer dillerin Rust'ın kurallarını ve garantilerini zorunlu kılmaması ve Rust'ın bunları kontrol edememesidir; bu nedenle, güvenliği sağlamak programcının sorumluluğundadır.

<Listing number="20-8" file-name="src/main.rs" caption="Başka bir dilde tanımlanan bir `extern` fonksiyonunu bildirme ve çağırma">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-08/src/main.rs}}
```

</Listing>

`unsafe extern "C"` bloğu içinde, çağırmak istediğimiz diğer dilden gelen dışsal fonksiyonların adlarını ve imzalarını listeliyoruz. `"C"` kısmı, dışsal fonksiyonun hangi _uygulama ikili arayüzü (ABI)_ kullandığını tanımlar: ABI, fonksiyonu montaj seviyesinde çağırma şekliyle ilgilidir. `"C"` ABI, en yaygın olanıdır ve C programlama dilinin ABI'sini takip eder. Rust'ın desteklediği tüm ABI'ler hakkında bilgi, [Rust Referansı'nda][ABI] mevcuttur.

Her `extern` bloğu içinde tanımlanan öğe, dolaylı olarak `unsafe` olarak kabul edilir. Ancak, bazı FFI fonksiyonları çağrılması güvenli olabilir. Örneğin, C standart kütüphanesindeki `abs` fonksiyonu, herhangi bir `i32` ile çağrılabileceğini bildiğimiz için bellek güvenliği ile ilgili bir sorunu yoktur. Bu gibi durumlarda, bu belirli fonksiyonun `unsafe extern` bloğu içinde güvenli olduğunu belirtmek için `safe` anahtar kelimesini kullanabiliriz. Bu değişikliği yaptıktan sonra, Liste 20-9'da gösterildiği gibi, artık onu çağırmak için bir `unsafe` bloğuna ihtiyaç duyulmaz.

<Listing number="20-9" file-name="src/main.rs" caption="Bir `unsafe extern` bloğu içinde bir fonksiyonu `safe` olarak işaretleme ve güvenli bir şekilde çağırma">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-09/src/main.rs}}
```

</Listing>

Bir fonksiyonu `safe` olarak işaretlemek, onu güvenli hale getirmez! Bunun yerine, güvenli olduğunu Rust'a _söyleyen_ bir taahhüttür. Bu taahhüdün yerine getirileceğinden emin olmak yine sizin sorumluluğunuzdadır.

> #### Diğer Dillerden Rust Fonksiyonlarını Çağırma
>
> `extern` anahtar kelimesini, diğer dillerin Rust fonksiyonlarını çağırmasına olanak tanıyan bir arayüz oluşturmak için de kullanabiliriz. Bunu yapmak için, ilgili fonksiyonun `fn` anahtar kelimesinden önce `extern` anahtar kelimesini ekleyip, kullanılacak ABI'yi belirtmemiz yeterlidir. Ayrıca, Rust derleyicisine bu fonksiyonun adını değiştirmemesi için `#[unsafe(no_mangle)]` anotasyonunu eklememiz gerekir. _Ad değiştirme_ (Mangling), bir derleyicinin bir fonksiyona verdiğimiz ismi, diğer derleme süreci bölümlerinin tüketmesi için daha fazla bilgi içeren ama insan tarafından okunabilirliğini azaltan bir farklı isimle değiştirmesidir. Her programlama dili derleyicisi, isimleri biraz farklı şekilde değiştirir, bu nedenle, bir Rust fonksiyonunun diğer diller tarafından isimlendirilebilmesi için, Rust derleyicisinin isim değiştirmesini devre dışı bırakmalıyız. Bu, kütüphaneler arasında isim çakışmalarına yol açabileceğinden güvensizdir; bu nedenle, seçtiğimiz ismin güvenli olduğundan emin olmak bizim sorumluluğumuzdadır.
>
> Aşağıdaki örnekte, `call_from_c` fonksiyonunu, C kodundan erişilebilir hale getiriyoruz; bu, derlendikten sonra bir paylaşılan kütüphaneden ve C'den bağlandığında gerçekleşir:
>
> ```rust
> #[unsafe(no_mangle)]
> pub extern "C" fn call_from_c() {
>     println!("Just called a Rust function from C!");
> }
> ```
>
> `extern` kullanımının bu şekli, yalnızca atributta `unsafe` gerektirir, `extern` bloğunda değil.

### Değiştirilebilir Statik Bir Değişkene Erişme veya Onu Değiştirme

Bu kitapta, henüz küresel değişkenlerden bahsetmedik; ancak Rust, küresel değişkenleri destekler, ancak bunlar Rust'ın sahiplik kurallarıyla sorunlu olabilir. Eğer iki iş parçacığı aynı değiştirilebilir küresel değişkene erişiyorsa, bu bir veri yarışı oluşturabilir.

Rust'ta, küresel değişkenlere _statik_ değişkenler denir. Liste 20-10, bir dize dilimi içeren bir statik değişkenin nasıl tanımlanıp kullanılacağını gösterir.

<Listing number="20-10" file-name="src/main.rs" caption="Değişmez bir statik değişkeni tanımlama ve kullanma">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-10/src/main.rs}}
```

</Listing>

Statik değişkenler, 3. Bölümdeki [“Sabitler”][differences-between-variables-and-constants]<!-- ignore --> bölümünde tartıştığımız sabitlere benzer. Statik değişkenlerin isimleri, `SCREAMING_SNAKE_CASE` biçiminde yazılır. Statik değişkenler yalnızca `'static` ömrüne sahip referanslar saklayabilir; bu, Rust derleyicisinin ömrü belirleyebileceği ve bizim açıkça belirtmemizin gerekmediği anlamına gelir. Değişmez bir statik değişkene erişmek güvenlidir.

Değişmez statik değişkenler ile sabitler arasında ince bir fark vardır: statik değişkenlerdeki değerler bellekte sabit bir adrese sahiptir. Değerleri kullanmak her zaman aynı veriye erişir. Sabitler ise, verilerini kullandıklarında çoğaltmalarına izin verilir. Ayrıca, statik değişkenler değiştirilebilir olabilir. Değiştirilebilir statik değişkenlere erişmek ve bunları değiştirmek _güvensizdir_. Liste 20-11, `COUNTER` adlı bir değiştirilebilir statik değişkenin nasıl tanımlanıp erişileceğini ve değiştirileceğini göstermektedir.

<Listing number="20-11" file-name="src/main.rs" caption="Değiştirilebilir bir statik değişkene erişme veya onu değiştirme">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-11/src/main.rs}}
```

</Listing>

Normal değişkenlerde olduğu gibi, değiştirilebilirliği `mut` anahtar kelimesi ile belirtiriz. `COUNTER`'dan okuma veya yazma yapan herhangi bir kod, bir `unsafe` bloğu içinde olmalıdır. Bu kod, tek iş parçacıklı olduğu için beklediğimiz gibi `COUNTER: 3` yazdırır. Ancak, birden fazla iş parçacığı `COUNTER`'a erişirse, bu muhtemelen veri yarışlarına yol açar; bu nedenle, tanımsız bir davranıştır. Bu nedenle, tüm fonksiyonu `unsafe` olarak işaretlememiz ve güvenlik sınırlamalarını belgelemeniz gerekir; böylece, fonksiyonu çağıran herkes, güvenli bir şekilde ne yapıp ne yapamayacaklarını bilir.

Bir güvensiz fonksiyon yazdığımızda, genellikle `SAFETY` ile başlayan bir yorum yazmak alışılmadık bir durum değildir; bu yorum, çağıranın fonksiyonu güvenli bir şekilde çağırmak için ne yapması gerektiğini açıklar. Benzer şekilde, bir güvensiz işlem gerçekleştirdiğimizde, genellikle `SAFETY` ile başlayan bir yorum yazarak güvenlik kurallarının nasıl yerine getirildiğini açıklarız.

Ayrıca, derleyici, bir değiştirilebilir statik değişkene referans oluşturmanıza izin vermez. Sadece, ham ödünç alma operatörlerinden biriyle oluşturulmuş bir ham işaretçisi aracılığıyla ona erişebilirsiniz. Bu, referansın görünmez bir şekilde oluşturulduğu durumlarda bile geçerlidir; örneğin, bu kod parçasındaki `println!` makrosunda olduğu gibi. Değiştirilebilir statik değişkenlere referansların yalnızca ham işaretçiler aracılığıyla oluşturulabilmesi gerekliliği, bunları kullanmanın güvenlik gereksinimlerini daha belirgin hale getirmeye yardımcı olur.

Küresel olarak erişilebilen değişkenlerle, veri yarışlarının olmadığından emin olmak zordur; bu nedenle, Rust, değiştirilebilir statik değişkenleri güvensiz olarak kabul eder. Mümkün olduğunda, 16. Bölümde tartıştığımız eşzamanlılık tekniklerini ve thread-safe akıllı işaretçileri kullanmak tercih edilir; böylece, derleyici, farklı iş parçacıklarının veri erişiminin güvenli bir şekilde yapıldığını kontrol eder.

### Güvensiz Bir Trait Uygulamak

`unsafe` anahtar kelimesini, güvensiz bir trait'i uygulamak için de kullanabiliriz. Bir trait, en az bir yöntemi derleyicinin doğrulayamayacağı bir invarianta sahipse güvensizdır. Bir trait'in güvensiz olduğunu belirtmek için, `trait` anahtar kelimesinden önce `unsafe` anahtar kelimesini ekleriz ve trait'in uygulanmasını da `unsafe` olarak işaretleriz; bu, Liste 20-12'de gösterilmiştir.

<Listing number="20-12" caption="Güvensiz bir trait tanımlama ve uygulama">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-12/src/main.rs:here}}
```

</Listing>

`unsafe impl` kullanarak, derleyicinin doğrulayamayacağı invarianta sahip olduğumuz konusunda söz vermiş oluyoruz.

Örneğin, 16. Bölümde tartıştığımız `Sync` ve `Send` işaretçi trait'lerini hatırlayın: derleyici, bu trait'leri otomatik olarak, türlerimizin tamamen diğer `Send` ve `Sync`'yi uygulayan türlerden oluşması durumunda uygular. Eğer, `Send` veya `Sync`'yi uygulamayan bir türü içeren bir tür tanımlıyorsak, örneğin ham işaretçileri gibi, ve bu türü `Send` veya `Sync` olarak işaretlemek istiyorsak, `unsafe` kullanmalıyız. Rust, türümüzün, iş parçacıkları arasında güvenli bir şekilde gönderilebileceği veya birden fazla iş parçacığından erişilebileceği garantilerini yerine getirip getirmediğini doğrulayamaz; bu nedenle, bu kontrolleri manuel olarak yapmamız ve `unsafe` ile belirtmemiz gerekir.

### Bir Union'ın Alanlarına Erişmek

Sadece `unsafe` ile çalışan son eylem, bir union'ın alanlarına erişmektir. Bir `union`, bir `struct`'a benzer, ancak belirli bir örnekte yalnızca bir alan kullanılır. Union'lar esas olarak C kodundaki union'larla arayüz kurmak için kullanılır. Union alanlarına erişmek güvensizdir çünkü Rust, union örneğinde şu anda saklanan verinin türünü garanti edemez. Union'lar hakkında daha fazla bilgi için [Rust Referansı'na][unions] bakabilirsiniz.

### Güvensiz Kodu Kontrol Etmek İçin Miri Kullanma

Güvensiz kod yazarken, yazdığınız kodun gerçekten güvenli ve doğru olduğunu kontrol etmek isteyebilirsiniz. Bunu yapmanın en iyi yollarından biri, tanımsız davranışları tespit etmek için resmi bir Rust aracı olan Miri'yi kullanmaktır. Ödünç alma denetleyicisi _statik_ bir araçken, Miri _dinamik_ bir araçtır ve çalışma zamanında kodunuzu kontrol eder. Programınızı çalıştırarak veya test paketini çalıştırarak kodunuzu kontrol eder ve Rust'ın nasıl çalışması gerektiği konusunda anladığı kuralları ihlal ettiğinizde tespit eder.

Miri kullanmak için, Rust'ın gece sürümüne ihtiyacınız vardır (bunu daha fazla konuştuğumuz [Ek G: Rust Nasıl Yapılır ve "Gece Rust" ][nightly] bölümüne bakın). Hem Rust'ın gece sürümünü hem de Miri aracını yüklemek için `rustup +nightly component add miri` yazabilirsiniz. Bu, projenizin Rust sürümünü değiştirmez; yalnızca istediğinizde kullanabilmeniz için sisteminize aracı ekler. Miri'yi bir projede çalıştırmak için `cargo +nightly miri run` veya `cargo +nightly miri test` yazabilirsiniz.

Bunun ne kadar yardımcı olabileceğine bir örnek olarak, bunu Liste 20-11'e karşı çalıştırdığımızda ne olduğunu düşünün.

```console
{{#include ../listings/ch20-advanced-features/listing-20-11/output.txt}}
```

Miri, değiştirilebilir verilere karşı paylaşılan referanslarımız olduğunu doğru bir şekilde uyarır. Burada, Miri yalnızca bir uyarı verir çünkü bu durumda tanımsız davranış olup olmadığı garanti edilmez ve sorunu nasıl düzelteceğimize dair bir öneride bulunmaz. Ancak en azından, tanımsız davranış riski taşıdığımızı biliriz ve kodumuzu güvenli hale getirmek için ne yapacağımızı düşünebiliriz. Bazı durumlarda, Miri ayrıca kesinlikle yanlış olan kod desenlerini ve bu hataları nasıl düzelteceğinize dair önerilerde bulunabilir.

Miri, güvensiz kod yazarken yanlış yapabileceğiniz her şeyi yakalamaz. Miri, yalnızca çalıştırılan kodla ilgili sorunları yakalayan bir dinamik analiz aracıdır. Bu, güvenli kodlama teknikleriyle birleştirilmesi gerektiği anlamına gelir, böylece yazdığınız güvensiz kodun güvenli olduğundan daha fazla emin olabilirsiniz. Miri ayrıca, kodunuzun sağlıksız olabileceği her olasılığı kapsamaz.

Başka bir deyişle: Eğer Miri bir sorun tespit ederse, bir hatanız olduğunu bilirsiniz, ancak Miri bir hatayı tespit etmezse, bir sorun olmadığı anlamına gelmez. Ancak, birçok sorunu yakalayabilir. Geçmişteki diğer güvensiz kod örneklerinize karşı çalıştırmayı deneyin ve ne söylediğine bakın!

Miri hakkında daha fazla bilgi edinmek için [GitHub deposuna][miri] bakabilirsiniz.

### Güvensiz Kodu Ne Zaman Kullanmalıyız

Yukarıda tartışılan beş süper güçten birini kullanmak için `unsafe` kullanmak yanlış değildir veya hoş karşılanmaz; ancak, güvensiz kodu doğru bir şekilde kullanmak daha zordur çünkü derleyici bellek güvenliğini korumaya yardımcı olamaz. Güvensiz kod kullanmanız gerektiğinde, bunu yapabilirsiniz ve açık `unsafe` bildirimi, sorunlar ortaya çıktığında bunların kaynağını takip etmeyi kolaylaştırır. Herhangi bir güvensiz kod yazdığınızda, yazdığınız kodun Rust'ın kurallarını yerine getirip getirmediğinden daha fazla emin olmak için Miri'yi kullanabilirsiniz.

Güvensiz Rust ile etkili bir şekilde çalışmanın çok daha derin bir keşfi için, konuyla ilgili resmi Rust kılavuzunu, [Rustonomicon'u][nomicon] okuyun.

[dangling-references]: ch04-02-references-and-borrowing.md#sarkan-referanslar
[ABI]: ../reference/items/external-blocks.html#abi
[differences-between-variables-and-constants]: ch03-01-variables-and-mutability.md#sabitler
[extensible-concurrency-with-the-sync-and-send-traits]: ch16-04-extensible-concurrency-sync-and-send.md#send-ve-sync-traitleri-ile-genişletilebilir-eşzamanlılık
[the-slice-type]: ch04-03-slices.md#dilim-tipi
[unions]: ../reference/items/unions.html
[miri]: https://github.com/rust-lang/miri
[editions]: appendix-05-editions.md
[nightly]: appendix-07-nightly-rust.md
[nomicon]: https://doc.rust-lang.org/nomicon/
