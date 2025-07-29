## `panic!` ile Kurtarılamayan Hatalar

Bazen kodunuzda kötü şeyler olur ve
bu konuda yapabileceğiniz hiçbir şey yoktur. Bu durumlarda, Rust `panic!` makrosuna sahiptir. Pratikte
paniğine neden olmanın iki yolu vardır: kodumuzun paniklemesine neden olan bir eylemde bulunarak (örneğin
bir diziye sondan erişerek) veya açıkça `panic!` makrosunu çağırarak.
Her iki durumda da programımızda bir paniğe neden oluruz. Varsayılan olarak, bu panikler
bir hata mesajı yazdıracak, gevşeyecek, yığını temizleyecek ve çıkacaktır. Bir
ortam değişkeni aracılığıyla, paniğin kaynağını bulmayı kolaylaştırmak için bir
paniği oluştuğunda Rust'ın çağrı yığınını görüntülemesini de sağlayabilirsiniz.

> ### Panik Durumunda Yığını Geri Sarma veya İptal Etme
>
> Varsayılan olarak, bir panik oluştuğunda program _unwinding_ başlatır, yani
> Rust yığını geri yürür ve
> karşılaştığı her işlevdeki verileri temizler. Ancak, geri yürümek ve temizlemek çok fazla iş gerektirir. Rust,
> bu nedenle, hemen _aborting_ alternatifini seçmenize izin verir,
> bu da programı temizlemeden sonlandırır.
>
> Programın kullandığı belleğin
> işletim sistemi tarafından temizlenmesi gerekecektir. Projenizde ortaya çıkan ikiliyi mümkün olduğunca
> küçük yapmanız gerekiyorsa,
>
> _Cargo.toml_ dosyanızdaki uygun `[profile]` bölümlerine `panic = 'abort'` ekleyerek bir panik üzerine çözme işleminden iptal etme işlemine geçebilirsiniz. Örneğin, serbest bırakma modunda panik durumunda iptal etmek istiyorsanız,
> bunu ekleyin:
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

Basit bir programda `panic!

<Listing file-name="src/main.rs">

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-01-panic/src/main.rs}}
```

</Listing>

Programı çalıştırdığınızda şuna benzer bir şey göreceksiniz:

```console
{{#include ../listings/ch09-error-handling/no-listing-01-panic/output.txt}}
```

panic!` çağrısı son iki satırda yer alan hata mesajına neden olur.
İlk satır panik mesajımızı ve kaynak kodumuzda
paniğin meydana geldiği yeri gösterir: _src/main.rs:2:5_, bunun _src/main.rs_ dosyamızın ikinci satırı,
beşinci karakteri olduğunu gösterir.

Bu durumda, belirtilen satır kodumuzun bir parçasıdır ve
satırına gidersek, `panic!` makro çağrısını görürüz. Diğer durumlarda, `panic!` çağrısı
bizim kodumuzun çağırdığı kodda olabilir ve
tarafından bildirilen dosya adı ve satır numarası, hata iletisi, sonunda `panic!` çağrısına yol açan kodumuzun satırı değil, `panic!` makrosunun
çağrıldığı başka birinin kodu olacaktır.

<!-- Eski başlık. Kaldırmayın yoksa bağlantılar kopabilir. -->

<a id="using-a-panic-backtrace"></a>

Kodumuzun soruna neden olan kısmını
bulmak için `panic!` çağrısının geldiği fonksiyonların backtrace'ini kullanabiliriz. a `panic!` backtrace'in nasıl kullanılacağını anlamak için, başka bir örneğe bakalım ve
a `panic!` çağrısının
doğrudan makroyu çağıran kodumuzdan değil de kodumuzdaki bir hata nedeniyle bir kütüphaneden gelmesinin nasıl bir şey olduğunu görelim. Liste 9-1,
adresinin geçerli indeks aralığının ötesinde bir vektördeki bir indekse erişmeye çalıştığı bazı kodlara sahiptir.

<Listing number="9-1" file-name="src/main.rs" caption="Attempting to access an element beyond the end of a vector, which will cause a call to `panic!`">

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-01/src/main.rs}}
```

</Listing>

Burada, vektörümüzün 100. elemanına erişmeye çalışıyoruz (indeksleme sıfırdan başladığı için
indeksi 99'da), ancak vektörün yalnızca üç
elemanı var. Bu durumda, Rust panikleyecektir. []` kullanımının
bir eleman döndürmesi beklenir, ancak geçersiz bir indeks geçerseniz, Rust
'un burada doğru olarak döndürebileceği hiçbir eleman yoktur.

C'de, bir veri yapısının sonundan ötesini okumaya çalışmak tanımlanmamış
davranıştır. Bellekte
veri yapısındaki o öğeye karşılık gelen konumda ne varsa onu alabilirsiniz, ancak
belleği o yapıya ait değildir. Buna _buffer overread_ denir ve bir saldırgan
dizinini veri yapısının
adresinden sonra depolanan ve izin verilmemesi gereken verileri okuyacak şekilde manipüle edebilirse
güvenlik açıklarına yol açabilir.

Programınızı bu tür bir güvenlik açığından korumak için, mevcut olmayan bir dizinde
öğesini okumaya çalışırsanız, Rust yürütmeyi durduracak ve
devam etmeyi reddedecektir. Deneyelim ve görelim:

```console
{{#include ../listings/ch09-error-handling/listing-09-01/output.txt}}
```

Bu hata, _main.rs_ dosyamızın 4. satırında, `v` içindeki vektörün
`99` indeksine erişmeye çalıştığımız yeri işaret etmektedir.

Not: satırı bize `RUST_BACKTRACE` ortam
değişkenini ayarlayarak hataya neden olan şeyin tam olarak ne olduğuna dair bir geri izleme alabileceğimizi söylüyor. Bir
_backtrace_, bu
noktasına ulaşmak için çağrılan tüm fonksiyonların bir listesidir. Rust'ta geri izleme diğer dillerde olduğu gibi çalışır:
geri izlemeyi okumanın anahtarı en baştan başlamak ve
yazdığınız dosyaları görene kadar okumaktır. Bu, sorunun ortaya çıktığı noktadır. Bu noktanın üzerindeki satırlar
kodunuzun çağırdığı kodlardır; aşağıdaki satırlar ise
kodunuzu çağıran kodlardır. Bu önceki ve sonraki satırlar çekirdek Rust kodunu, standart
kütüphane kodunu veya kullandığınız crate'leri içerebilir. adresinden `RUST_BACKTRACE` ortam değişkenini `0` dışında herhangi bir değere ayarlayarak bir geri izleme almayı deneyelim.
Liste 9-2, göreceğinize benzer bir çıktı gösterir.

<!-- manual-regeneration
cd listings/ch09-error-handling/listing-09-01
RUST_BACKTRACE=1 cargo run
copy the backtrace output below
check the backtrace number mentioned in the text below the listing
-->

<Listing number="9-2" caption="The backtrace generated by a call to `panic!` displayed when the environment variable `RUST_BACKTRACE` is set">

```console
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0: rust_begin_unwind
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/std/src/panicking.rs:692:5
   1: core::panicking::panic_fmt
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/core/src/panicking.rs:75:14
   2: core::panicking::panic_bounds_check
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/core/src/panicking.rs:273:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/slice/index.rs:274:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/slice/index.rs:16:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3361:9
   6: panic::main
             at ./src/main.rs:4:6
   7: core::ops::function::FnOnce::call_once
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

</Listing>

Bu çok fazla çıktı! Gördüğünüz tam çıktı
işletim sisteminize ve Rust sürümünüze bağlı olarak farklı olabilir. Bu
bilgisini içeren backtrace'leri alabilmek için hata ayıklama sembollerinin etkinleştirilmesi gerekir. Hata ayıklama sembolleri, burada olduğu gibi
, `--release` bayrağı olmadan `cargo build` veya `cargo run` kullanıldığında
varsayılanı tarafından etkinleştirilir.

Listing 9-2'deki çıktıda, backtrace'in 6. satırı
projemizde soruna neden olan satırı işaret etmektedir: _src/main.rs_ dosyasının 4. satırı. Eğer
programımızın paniklemesini istemiyorsak, araştırmamıza
yazdığımız bir dosyadan bahseden ilk satırın işaret ettiği yerden başlamalıyız. kasıtlı olarak panik yaratacak kod yazdığımız Liste 9-1'de, paniği düzeltmenin yolu
vektör indeksleri aralığının ötesinde bir eleman talep etmemektir. Kodunuz
gelecekte panik yaptığında, kodun
hangi değerlerle paniğe neden olduğunu ve bunun yerine kodun ne yapması gerektiğini bulmanız gerekecektir.

Bu
bölümünün ilerleyen kısımlarında [“`panic!` yapmak ya da
`panic!` yapmamak”][to-panic-or-not-to-panic]<!-- ignore --> bölümünde hata koşullarını ele almak için `panic!` ve ne zaman `panic!` kullanıp kullanmamamız gerektiği konusuna geri döneceğiz. Daha sonra, `Result` kullanarak bir hatadan nasıl kurtulacağımıza bakacağız.

[to-panic-or-not-to-panic]: ch09-03-to-panic-or-not-to-panic.md#panic-yapmak-ya-da-panic-yapmamak