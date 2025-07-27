## Kodunuzu Aynı Anda Çalıştırmak için Thread Kullanmak

Çoğu modern işletim sisteminde, çalıştırılan bir programın kodu bir _proses_ (process) içinde çalışır ve işletim sistemi aynı anda birden fazla prosesi yönetir. Bir programın içinde de, aynı anda bağımsız olarak çalışabilen bölümler olabilir. Bu bağımsız bölümleri çalıştıran özelliklere _thread_ (iş parçacığı) denir. Örneğin, bir web sunucusu, aynı anda birden fazla isteğe yanıt verebilmek için birden fazla thread'e sahip olabilir.

Programınızdaki hesaplamayı birden fazla thread'e bölerek aynı anda birden fazla görevi çalıştırmak performansı artırabilir, ancak bu aynı zamanda karmaşıklık da ekler. Thread'ler aynı anda çalışabildiği için, farklı thread'lerdeki kod bölümlerinin hangi sırayla çalışacağına dair doğal bir garanti yoktur. Bu da şu gibi sorunlara yol açabilir:

- Thread'lerin veri veya kaynaklara tutarsız bir sırayla eriştiği _yarış durumları_ (race conditions)
- İki thread'in birbirini bekleyip ikisinin de devam edemediği _deadlock_ (kilitlenme) durumları
- Sadece belirli durumlarda ortaya çıkan ve tekrarlaması, düzeltmesi zor olan hatalar

Rust, thread kullanmanın olumsuz etkilerini azaltmaya çalışır; ancak çoklu thread'li bir bağlamda programlama hâlâ dikkatli düşünmeyi ve tek thread'li programlardan farklı bir kod yapısı gerektirir.

Programlama dilleri thread'leri birkaç farklı şekilde uygular ve birçok işletim sistemi, programlama dilinin yeni thread'ler oluşturmak için çağırabileceği bir API sunar. Rust standart kütüphanesi, _1:1_ thread modeli kullanır; yani bir program, her bir dil thread'i için bir işletim sistemi thread'i kullanır. Farklı ödünleşimler sunan başka thread modelleri uygulayan crate'ler de vardır. (Rust'ın bir sonraki bölümde göreceğimiz async sistemi de eşzamanlılığa başka bir yaklaşım sunar.)

### `spawn` ile Yeni Bir Thread Oluşturmak

Yeni bir thread oluşturmak için, `thread::spawn` fonksiyonunu çağırır ve yeni thread'de çalıştırmak istediğimiz kodu içeren bir closure (13. bölümde closure'ları işlemiştik) veririz. 16-1 numaralı listede, ana thread'den bazı metinler ve yeni bir thread'den başka metinler yazdıran bir örnek gösteriliyor.

<Listing number="16-1" file-name="src/main.rs" caption="Ana thread başka bir şey yazdırırken yeni bir thread oluşturmak">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-01/src/main.rs}}
```

</Listing>

Dikkat edin, bir Rust programında ana thread tamamlandığında, başlatılan tüm thread'ler de, çalışmayı bitirip bitirmediklerine bakılmaksızın kapatılır. Bu programın çıktısı her seferinde biraz farklı olabilir, ancak aşağıdakine benzer olacaktır:

```text
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```

`thread::sleep` çağrıları, bir thread'in kısa bir süreliğine çalışmasını durdurur ve başka bir thread'in çalışmasına olanak tanır. Thread'ler muhtemelen sırayla çalışacaktır, ancak bu garanti değildir: işletim sisteminizin thread'leri nasıl zamanladığına bağlıdır. Bu çalıştırmada, ana thread ilk yazdırmayı yaptı, ancak kodda önce spawned thread'in print satırı olmasına rağmen. Ayrıca, spawned thread'e `i` 9 olana kadar yazdırmasını söylemiş olsak da, ana thread kapandığı için sadece 5'e kadar yazdırabildi.

Eğer bu kodu çalıştırdığınızda sadece ana thread'den çıktı görüyorsanız veya hiç örtüşme yoksa, thread'ler arasında geçiş için işletim sistemine daha fazla fırsat vermek adına aralardaki sayıları artırmayı deneyin.

### Tüm Thread'lerin Bitmesini `join` Handle ile Beklemek

16-1 numaralı listedeki kodda, ana thread sona erdiği için spawned thread çoğu zaman erken duruyor; ayrıca thread'lerin hangi sırayla çalışacağına dair bir garanti olmadığından, spawned thread'in hiç çalışmama ihtimali de var!

Spawned thread'in çalışmama veya erken bitme sorununu, `thread::spawn`'ın dönüş değerini bir değişkende saklayarak çözebiliriz. `thread::spawn`'ın dönüş tipi `JoinHandle<T>`'dir. Bir `JoinHandle<T>`, üzerinde `join` metodunu çağırdığımızda, ilgili thread'in bitmesini bekleyen sahipli bir değerdir. 16-2 numaralı listede, 16-1'de oluşturduğumuz thread'in `JoinHandle<T>`'ını nasıl kullandığımız ve `join` çağrısı ile spawned thread'in ana thread'den önce bitmesini nasıl sağladığımız gösteriliyor.

<Listing number="16-2" file-name="src/main.rs" caption="`thread::spawn`'dan dönen `JoinHandle<T>`'ı saklayarak thread'in tamamlanmasını garanti etmek">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-02/src/main.rs}}
```

</Listing>

Handle üzerinde `join` çağrısı, o anda çalışan thread'i, handle'ın temsil ettiği thread sona erene kadar _bloklar_. Bir thread'i bloklamak, o thread'in iş yapmasını veya çıkmasını engellemek demektir. `join` çağrısını ana thread'in `for` döngüsünden sonra koyduğumuz için, 16-2'yi çalıştırmak aşağıdakine benzer bir çıktı üretmelidir:

```text
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```

İki thread sırayla çalışmaya devam eder, ancak ana thread, `handle.join()` çağrısı nedeniyle spawned thread bitene kadar bekler ve sona ermez.

Ama `handle.join()`'ı ana thread'deki `for` döngüsünden önce koyarsak ne olur bakalım:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/no-listing-01-join-too-early/src/main.rs}}
```

</Listing>

Ana thread, spawned thread bitene kadar bekler ve ardından kendi `for` döngüsünü çalıştırır; bu nedenle çıktı artık iç içe geçmez, şöyle olur:

```text
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```

Küçük detaylar, örneğin `join`'ın nerede çağrıldığı, thread'lerinizin aynı anda çalışıp çalışmadığını etkileyebilir.

### Thread'lerle Birlikte `move` Closure Kullanmak

Genellikle, `thread::spawn`'a verilen closure'larda `move` anahtar kelimesini kullanırız; çünkü bu durumda closure, kullandığı değerlerin sahipliğini alır ve bu değerlerin sahipliğini bir thread'den diğerine aktarır. 13. bölümde ["Referansları Yakalamak veya Sahipliği Taşımak"](ch13-01-closures.html#capturing-references-or-moving-ownership) başlığında `move`'u closure'lar bağlamında tartışmıştık. Şimdi, `move` ile `thread::spawn` etkileşimine daha yakından bakacağız.

16-1 numaralı listede, `thread::spawn`'a verdiğimiz closure'ın argüman almadığına dikkat edin: spawned thread'in kodunda ana thread'den herhangi bir veri kullanmıyoruz. Ana thread'deki verileri spawned thread'de kullanmak için, spawned thread'in closure'ı ihtiyaç duyduğu değerleri yakalamalıdır. 16-3 numaralı listede, ana thread'de bir vektör oluşturup, onu spawned thread'de kullanmaya çalışıyoruz. Ancak, bu henüz çalışmayacak, birazdan göreceksiniz.

<Listing number="16-3" file-name="src/main.rs" caption="Ana thread'de oluşturulan bir vektörü başka bir thread'de kullanmaya çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-03/src/main.rs}}
```

</Listing>

Closure, `v`'yi kullandığı için onu yakalar ve closure'ın ortamının bir parçası yapar. `thread::spawn` bu closure'ı yeni bir thread'de çalıştırdığı için, yeni thread içinde `v`'ye erişebilmemiz gerekir. Ancak bu örneği derlediğimizde şu hatayı alırız:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-03/output.txt}}
```

Rust, `v`'yi nasıl yakalayacağını _tahmin eder_ ve `println!` yalnızca bir referansa ihtiyaç duyduğu için closure, `v`'yi ödünç almaya çalışır. Ancak burada bir sorun var: Rust, spawned thread'in ne kadar süreceğini bilemez, bu yüzden `v`'ye olan referansın her zaman geçerli olup olmayacağını da bilemez.

16-4 numaralı listede, `v`'ye olan referansın geçerli olmayacağı bir senaryo gösteriliyor.

<Listing number="16-4" file-name="src/main.rs" caption="Ana thread'den referans yakalamaya çalışan bir closure ile thread oluşturmak; ana thread 'v'yi düşürüyor">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-04/src/main.rs}}
```

</Listing>

Rust bu kodun çalışmasına izin verseydi, spawned thread hemen arka plana alınabilir ve hiç çalışmayabilirdi. Spawned thread'in içinde `v`'ye bir referans var, ancak ana thread hemen ardından `v`'yi `drop` fonksiyonu ile düşürüyor (15. bölümde `drop`'u işlemiştik). Sonra, spawned thread çalışmaya başladığında, `v` artık geçerli değildir ve referansı da geçersiz olurdu. Vay canına!

16-3 numaralı listedeki derleyici hatasını düzeltmek için, hata mesajının önerisini kullanabiliriz:

```text
help: closure'ın `v` (ve diğer referanslı değişkenlerin) sahipliğini almasını zorlamak için `move` anahtar kelimesini kullanın
  |
6 |     let handle = thread::spawn(move || {
  |                                ++++
```

Closure'ın başına `move` anahtar kelimesini ekleyerek, closure'ın kullandığı değerlerin sahipliğini almasını sağlarız; böylece Rust'ın bu değerleri ödünç alması yerine sahiplenmesini zorlamış oluruz. 16-3'teki değişikliğin gösterildiği 16-5 numaralı liste, istediğimiz gibi derlenir ve çalışır.

<Listing number="16-5" file-name="src/main.rs" caption="`move` anahtar kelimesiyle closure'ın kullandığı değerlerin sahipliğini almasını sağlamak">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-05/src/main.rs}}
```

</Listing>

Aynı şeyi, ana thread'de `drop` çağrısı yapılan 16-4 numaralı kodda da denemek isteyebilirsiniz. Ancak, bu çözüm burada işe yaramaz; çünkü 16-4'te yapılmak istenen şey farklı bir nedenle izin verilmez. Eğer closure'a `move` eklersek, `v`'nin sahipliğini closure ortamına taşımış oluruz ve artık ana thread'de `drop` çağrısı yapamayız. Bu sefer şu derleyici hatasını alırız:

```console
{{#include ../listings/ch16-fearless-concurrency/output-only-01-move-drop/output.txt}}
```

Rust'ın sahiplik kuralları bizi yine kurtardı! 16-3'teki kodda hata aldık çünkü Rust temkinli davranıp, thread için yalnızca `v`'yi ödünç alıyordu; bu da ana thread'in teorik olarak spawned thread'in referansını geçersiz kılabileceği anlamına geliyordu. Rust'a, `v`'nin sahipliğini spawned thread'e taşımasını söylediğimizde, ana thread'in artık `v`'yi kullanmayacağını garanti etmiş oluyoruz. 16-4'ü aynı şekilde değiştirirsek, bu sefer ana thread'de `v`'yi kullanmaya çalıştığımızda sahiplik kurallarını ihlal etmiş oluruz. `move` anahtar kelimesi, Rust'ın temkinli varsayılanı olan ödünç almayı geçersiz kılar; sahiplik kurallarını ihlal etmenize izin vermez.

Artık thread'lerin ne olduğunu ve thread API'sinin sağladığı yöntemleri gördüğümüze göre, thread'leri kullanabileceğimiz bazı durumlara bakalım.

[capture]: ch13-01-closures.html#capturing-references-or-moving-ownership
