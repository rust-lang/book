## Değişkenler ve Değişebilirlik

“Değerleri
[Değişkenlerle Saklama”][storing-values-with-variables]<!-- ignore --> bölümünde belirtildiği gibi, varsayılan olarak
değişkenleri değişmezdir. Bu, kodunuzu
Rust'ın sunduğu güvenlik ve kolay eşzamanlılıktan yararlanacak şekilde
yazmanız için Rust'ın size verdiği birçok dürtüden biridir. Ancak yine de değişkenlerinizi değiştirilebilir yapma seçeneğiniz vardır.
Rust'ın sizi nasıl ve neden değişmezliği tercih etmeye teşvik ettiğini ve neden
bazen vazgeçmek isteyebileceğinizi inceleyelim.

Bir değişken değişmez olduğunda, bir değer bir isme bağlandıktan sonra
bu değeri değiştiremezsiniz. Bunu göstermek için,
adresinde _projects_ dizininizde `cargo new variables` kullanarak _variables_ adında yeni bir proje oluşturun.

Ardından, yeni _variables_ dizininizde _src/main.rs_ dosyasını açın ve
kodunu henüz derlenmeyecek olan aşağıdaki kodla değiştirin:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/src/main.rs}}
```

Programı kaydedin ve `cargo run` kullanarak çalıştırın. Bu çıktıda gösterildiği gibi bir değişmezlik hatasıyla ilgili olarak
adresinde bir hata mesajı almalısınız:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/output.txt}}
```

Bu örnek, derleyicinin programlarınızdaki hataları bulmanıza nasıl yardımcı olduğunu gösterir.
Derleyici hataları sinir bozucu olabilir, ancak gerçekte bunlar yalnızca programınızın
yapmasını istediğiniz şeyi henüz güvenli bir şekilde yapmadığı anlamına gelir; sizin
iyi bir programcı olmadığınız anlamına gelmez! Deneyimli Rustaceanlar hala derleyici hataları alırlar.

Değişmez `x` değişkenine ikinci bir değer atamaya çalıştığınız için ``değişmez `x` değişkenine iki kez atama yapılamıyor`` hata mesajını aldınız.

Değişmez olarak belirlenmiş bir
değerini değiştirmeye çalıştığımızda derleme zamanı hataları almamız önemlidir, çünkü bu durum
hatalarına yol açabilir. Kodumuzun bir bölümü
bir değerin asla değişmeyeceği varsayımıyla çalışıyorsa ve kodumuzun başka bir bölümü bu değeri değiştiriyorsa
kodun ilk bölümünün yapmak için tasarlandığı şeyi yapmaması mümkündür. Bu tür bir hatanın nedenini
özellikle
ikinci kod parçası değeri yalnızca _bazen_ değiştirdiğinde olaydan sonra izlemek zor olabilir. Rust
derleyicisi, bir değerin değişmeyeceğini belirttiğinizde, gerçekten
değişmeyeceğini garanti eder, böylece bunu kendiniz takip etmek zorunda kalmazsınız. Böylece kodunuzda mantık yürütmek
daha kolay olur.

Bdeğişebilirlik çok yararlı olabilir ve kod yazmayı daha kolay hale getirebilir.
Değişkenler varsayılan olarak değişmez olsa da, [Bölüm
2][storing-values-with-variables]<!-- ignore -->'da yaptığınız gibi
değişken adının önüne `mut` ekleyerek onları değişebilir hale getirebilirsiniz. mut` eklemek ayrıca
kodunun diğer bölümlerinin bu değişkenin değerini değiştireceğini belirterek kodun gelecekteki okuyucularına
niyetini iletir.

Örneğin, _src/main.rs_ dosyasını aşağıdaki şekilde değiştirelim:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/src/main.rs}}
```

Programı şimdi çalıştırdığımızda şunu elde ediyoruz:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/output.txt}}
```

`mut`
kullanıldığında `x`e bağlı değeri `5`ten `6`ya değiştirmemize izin verilir. Nihayetinde, değişebilirliği kullanıp kullanmamaya karar vermek size bağlıdır ve
o özel durumda en açık olduğunu düşündüğünüz şeye bağlıdır.

### Sabitler

Değişmez değişkenler gibi, _sabitler_ de bir isme bağlı değerlerdir ve
değişmesine izin verilmez, ancak sabitler
ve değişkenler arasında birkaç fark vardır.

İlk olarak, sabitlerle `mut` kullanmanıza izin verilmez. Sabitler sadece
varsayılan olarak değişmez değildir, her zaman değişmezdir. Sabitleri `let` anahtar sözcüğü yerine
`const` anahtar sözcüğünü kullanarak bildirirsiniz ve değerin türü _must_
belirtilmelidir. Türleri ve tür ek açıklamalarını bir sonraki bölümde ele alacağız,
[“Veri Türleri”][data-types]<!-- ignore -->, bu nedenle şu anda
ayrıntıları hakkında endişelenmeyin. Sadece her zaman tipe açıklama eklemeniz gerektiğini bilin.

Sabitler, global kapsam da dahil olmak üzere herhangi bir kapsamda bildirilebilir, bu da
adresini kodun birçok bölümünün bilmesi gereken değerler için kullanışlı hale getirir.

Son fark ise sabitlerin yalnızca sabit bir ifadeye ayarlanabilmesidir
yalnızca çalışma zamanında hesaplanabilecek bir değerin sonucuna değil.

İşte bir sabit bildirimi örneği:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

Sabitin adı `THREE_HOURS_IN_SECONDS` ve değeri 60 (bir dakikadaki saniye sayısı) ile 60'ın (bir saatteki dakikaların
sayısı) 3 (bu
programında saymak istediğimiz saat sayısı) çarpımının
sonucuna ayarlanır. Rust'ın sabitler için adlandırma kuralı, sözcükler arasında
alt çizgi ile tüm büyük harfleri kullanmaktır. Derleyici, derleme zamanında sınırlı sayıda
işlemini değerlendirebilir, bu da bu değeri
sabitini 10.800 değerine ayarlamak yerine, anlaşılması ve doğrulanması daha kolay olan
bir şekilde yazmayı seçmemizi sağlar. Sabitleri bildirirken
hangi işlemlerin kullanılabileceği hakkında daha fazla bilgi için [Rust Reference'ın constant evaluation][const-eval] bölümüne bakın.

Sabitler, bildirildikleri
kapsamı dahilinde bir programın çalıştığı süre boyunca geçerlidir. Bu özellik sabitleri
uygulama alanınızda
programın birden fazla bölümünün bilmesi gerekebilecek değerler için kullanışlı hale getirir; örneğin
bir oyundaki herhangi bir oyuncunun kazanmasına izin verilen maksimum puan sayısı veya ışık hızı gibi.

Programınız boyunca kullanılan kodlanmış değerleri sabitler olarak adlandırmak,
adresinde bu değerin anlamını kodun gelecekteki bakımcılarına aktarmak açısından yararlıdır. Ayrıca

 kodlanmış değerin gelecekte güncellenmesi gerektiğinde kodunuzda değiştirmeniz gereken tek bir yer olmasına yardımcı olur.

### Gölgeleme

Bölüm
2][comparing-the-guess-to-the-secret-number]<!-- ignore -->'daki tahmin oyunu eğitiminde gördüğünüz gibi, önceki bir değişkenle aynı ada sahip
yeni bir değişken bildirebilirsiniz. Rustaceanlar
ilk değişkenin ikinci değişken tarafından _gölgelendiğini_ söylerler, bu da ikinci
değişkenin, değişkenin adını kullandığınızda derleyicinin göreceği şey olduğu anlamına gelir.
Gerçekte, ikinci değişken birinciyi gölgeler ve kendisi gölgelenene ya da kapsam sona erene kadar
değişken adının tüm kullanımlarını kendine alır.
Aynı değişkenin adını kullanarak ve
adresinde `let` anahtar sözcüğünü aşağıdaki gibi tekrarlayarak bir değişkeni gölgeleyebiliriz:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```

Bu program ilk olarak `x` değişkenini `5` değerine bağlar. Daha sonra `let x =` ifadesini tekrarlayarak, orijinal değeri alıp `1` ekleyerek yeni bir
`x` değişkeni yaratır, böylece `x`in
değeri `6` olur. Daha sonra, küme
parantezleriyle oluşturulan bir iç kapsam içinde, üçüncü `let` deyimi de `x` değerini gölgeler ve yeni bir
değişkeni oluşturur, önceki değeri `2` ile çarparak `x` değerini `12` olarak verir.
Bu kapsam sona erdiğinde, iç gölgeleme sona erer ve `x` değişkeni `6` değerine geri döner.
Bu programı çalıştırdığımızda, aşağıdaki çıktıyı verecektir:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```

Gölgeleme, bir değişkeni `mut` olarak işaretlemekten farklıdır, çünkü yanlışlıkla bu değişkene
olmadan `let` anahtar sözcüğünü kullanarak yeniden atama yapmaya çalışırsak
derleme zamanı hatası alırız. let` anahtar sözcüğünü kullanarak
bir değer üzerinde birkaç dönüşüm gerçekleştirebilir
ancak bu dönüşümler tamamlandıktan sonra değişkenin değişmez olmasını sağlayabiliriz.

`mut` ile gölgeleme arasındaki diğer fark ise, `let` anahtar sözcüğünü tekrar kullandığımızda
etkin bir şekilde yeni bir değişken oluşturduğumuz için
değerin türünü değiştirebilir ancak aynı ismi tekrar kullanabiliriz. Örneğin,
programımızın bir kullanıcıdan
boşluk karakterleri girerek bazı metinler arasında kaç boşluk istediğini göstermesini istediğini ve daha sonra bu girdiyi bir sayı olarak saklamak istediğimizi varsayalım:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```

İlk `spaces` değişkeni bir string tipidir ve ikinci `spaces` değişkeni
bir sayı tipidir. Böylece gölgeleme bizi `spaces_str` ve `spaces_num` gibi
farklı isimler bulmaktan kurtarır; bunun yerine
daha basit olan `spaces` ismini tekrar kullanabiliriz. Ancak, burada
gösterildiği gibi bunun için `mut` kullanmayı denersek, derleme zamanı hatası alırız:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```

Hata, bir değişkenin türünü değiştirmemize izin verilmediğini söylüyor:
```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/output.txt}}
```

Şimdi değişkenlerin nasıl çalıştığını keşfettiğimize göre,
sahip olabilecekleri daha fazla veri türüne bakalım.

[comparing-the-guess-to-the-secret-number]: ch02-00-guessing-game-tutorial.md#Tahmini-Gizli-Sayı-ile-Karşılaştırma
[data-types]: ch03-02-data-types.md#Veri-Türleri
[storing-values-with-variables]: ch02-00-guessing-game-tutorial.md#Değerleri-Değişkenlerle-Saklama
[const-eval]:https://doc.rust-lang.org/reference/const_eval.html
