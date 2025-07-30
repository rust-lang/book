## Test Organizasyonu

Bu bölümün başında da belirtildiği gibi, testler karmaşık bir disiplindir ve
farklı kişiler farklı terminoloji ve organizasyonlar kullanır. Rust topluluğu,
testleri iki ana kategori altında ele alır: birim testleri ve entegrasyon
testleri. _Birim testleri_ küçük ve daha odaklıdır, tek bir modülü tek başına
test eder ve özel arayüzleri test edebilir. _Entegrasyon testleri_ tamamen
kütüphanenizin dışındadır ve kodunuzu diğer harici
kodlar gibi kullanır, yalnızca genel arayüzü kullanır ve test başına birden fazla
modülü çalıştırabilir.

Her iki tür testi de yazmak, kütüphanenizin parçalarının
ayrı ayrı ve birlikte beklediğiniz gibi çalıştığından emin olmak için önemlidir.

### Birim Testleri

Birim testlerinin amacı, kodun geri kalanından ayrı olarak her birim kodunu test etmek ve
kodun nerede beklendiği gibi çalıştığını ve çalışmadığını hızlı bir şekilde belirlemektir.
Birim testlerini, test ettikleri kodun bulunduğu her dosyanın _src_ dizinine
yerleştireceksiniz. Geleneksel olarak, her dosyada test işlevlerini içeren `tests`
adlı bir modül oluşturulur ve modüle
`cfg(test)`

#### Test Modülü ve `#[cfg(test)]`

`tests` modülündeki `#[cfg(test)]` ek açıklaması, Rust'a test kodunu yalnızca `cargo test` komutunu çalıştırdığınızda derlemesini ve
çalıştırmasını söyler, `cargo
build` komutunu çalıştırdığınızda değil. Bu, yalnızca kütüphaneyi derlemek istediğinizde derleme süresinden tasarruf sağlar ve
testler dahil edilmediği için sonuçta ortaya çıkan derlenmiş yapıda yer tasarrufu sağlar.
Entegrasyon testleri farklı bir
dizine yerleştirildiği için `#[cfg(test)]` ek açıklamasına ihtiyaç duymadıklarını göreceksiniz. Ancak, birim
testleri kodla aynı dosyalara yerleştirildiğinden, derlenen sonuca dahil edilmemeleri gerektiğini belirtmek için `#[cfg(test)]` kullanacaksınız.

Bu bölümün ilk bölümünde yeni `adder` projesini oluşturduğumuzda, Cargo bizim için şu kodu oluşturduğunu hatırlayın:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

Otomatik olarak oluşturulan `tests` modülünde, `cfg` özniteliği
_configuration_ anlamına gelir ve Rust'a, aşağıdaki öğenin yalnızca belirli bir yapılandırma seçeneği verildiğinde dahil edilmesi gerektiğini
söyler. Bu durumda, yapılandırma seçeneği
`test`'tir ve Rust tarafından testleri derlemek ve çalıştırmak için sağlanır.
`cfg` özniteliğini kullanarak, Cargo test kodumuzu yalnızca `cargo test` ile testleri
aktif olarak çalıştırırsak derler. Bu, `#[test]` ile işaretlenmiş işlevlerin yanı sıra,
bu modül içinde bulunabilecek tüm yardımcı işlevleri de içerir.

#### Özel İşlevleri Test Etme

Test topluluğu içinde, özel işlevlerin doğrudan test edilip edilmeyeceği konusunda tartışmalar vardır
ve diğer dillerde özel işlevleri test etmek zor veya
imkansızdır. Hangi test ideolojisini benimserseniz benimseyin,
 Rust'un gizlilik kuralları özel işlevleri test etmenize izin verir.
Listing 11-12'deki özel işlev `internal_adder` içeren kodu ele alalım.

<Listing number="11-12" file-name="src/lib.rs" caption="Testing a private function">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-12/src/lib.rs}}
```

</Listing>

`internal_adder` işlevinin `pub` olarak işaretlenmediğine dikkat edin. Testler sadece
Rust kodudur ve `tests` modülü sadece başka bir modüldür.
[“Modül Ağacındaki Bir Öğeye Başvurma Yolları”][paths]<!-- ignore -->,
alt modüllerdeki öğeler, üst modüllerindeki öğeleri kullanabilir. Bu
testte, `use
super::*` ile `tests` modülünün üst modülündeki tüm öğeleri kapsam içine alıyoruz ve ardından test `internal_adder` işlevini çağırabiliyor. Özel işlevlerin test edilmesi gerektiğini düşünmüyorsanız, Rust'ta sizi
bunu yapmaya zorlayacak hiçbir şey yoktur.

### Entegrasyon Testleri

Rust'ta entegrasyon testleri, kütüphanenizin tamamen dışındadır. Kütüphanenizi
diğer kodlar gibi kullanırlar, yani yalnızca kütüphanenizin genel API'sinin bir parçası olan
fonksiyonları çağırabilirler. Amaçları, kütüphanenizin birçok parçasının birlikte doğru bir şekilde çalışıp çalışmadığını
test etmektir. Kendi başlarına doğru çalışan kod birimleri
entegre edildiğinde sorunlar yaşayabilir, bu nedenle entegre kodun test
kapsamı da önemlidir. Entegrasyon
testleri oluşturmak için önce bir _tests_ dizini oluşturmanız gerekir.

#### _tests_ Dizini

Proje dizininizin en üst düzeyinde, _src_ dizininin yanında bir _tests_ dizini
oluşturuyoruz. Cargo, bu dizinde entegrasyon test dosyalarını arar.
Ardından istediğimiz kadar test dosyası oluşturabiliriz ve Cargo her bir dosyayı
ayrı bir crate olarak derler.

Bir entegrasyon testi oluşturalım. Listing 11-12'deki kod hala
_src/lib.rs_ dosyasında iken, bir _tests_ dizini oluşturun ve
_tests/integration_test.rs_ adında yeni bir dosya oluşturun. Dizin yapınız şöyle görünmelidir:

```text
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

Listing 11-13'teki kodu _tests/integration_test.rs_ dosyasına girin.

<Listing number="11-13" file-name="tests/integration_test.rs" caption="An integration test of a function in the `adder` crate">

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-13/tests/integration_test.rs}}
```

</Listing>

_tests_ dizinindeki her dosya ayrı bir crate olduğundan, kütüphanemizi
her test crate'inin kapsamına dahil etmemiz gerekir. Bu nedenle, birim testlerinde gerekmeyen `use
adder::add_two;` kodunu kodun en üstüne ekliyoruz.

_tests/integration_test.rs_ içindeki hiçbir koda
`#[cfg(test)]` eklememiz gerekmez. Cargo, _tests_ dizinini özel olarak ele alır ve bu dizindeki dosyaları
sadece `cargo test` komutunu çalıştırdığımızda derler. Şimdi `cargo test` komutunu çalıştırın:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-13/output.txt}}
```

Çıktının üç bölümü, birim testleri, entegrasyon testi ve
doküman testlerini içerir. Bir bölümdeki herhangi bir test başarısız olursa, sonraki bölümler
çalıştırılmayacaktır. Örneğin, bir birim testi başarısız olursa, entegrasyon ve doküman testleri için herhangi bir çıktı
olmayacaktır, çünkü bu testler yalnızca tüm birim
testleri başarılı olursa çalıştırılır.

Birim testleri için ilk bölüm, daha önce gördüğümüzle aynıdır: her birim testi için bir satır
(Listing 11-12'de eklediğimiz `internal` adlı birim testi) ve
ardından birim testleri için bir özet satırı.

Entegrasyon testleri bölümü `Running
tests/integration_test.rs` satırıyla başlar. Ardından, o entegrasyon testindeki her test işlevi için bir satır ve
`Doc-tests adder` bölümü başlamadan hemen önce entegrasyon
testinin sonuçları için bir özet satırı vardır.

Her entegrasyon test dosyasının kendi bölümü vardır, bu nedenle
_tests_ dizinine daha fazla dosya eklersek, daha fazla entegrasyon testi bölümü olacaktır.

Hala, test fonksiyonunun adını `cargo test` komutuna argüman olarak
belirterek belirli bir entegrasyon testi fonksiyonunu çalıştırabiliriz. Belirli bir
entegrasyon testi dosyasındaki tüm testleri çalıştırmak için, `cargo test` komutunun
`--test` argümanını kullanın ve ardından dosyanın adını yazın:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-05-single-integration/output.txt}}
```

Bu komut yalnızca _tests/integration_test.rs_ dosyasındaki testleri çalıştırır.

#### Entegrasyon Testlerinde Alt Modüller

Daha fazla entegrasyon testi ekledikçe, bunları düzenlemek için
_tests_ dizininde daha fazla dosya oluşturmak isteyebilirsiniz; örneğin, test
işlevlerini test ettikleri işlevselliğe göre gruplandırabilirsiniz. Daha önce de belirtildiği gibi, _tests_ dizinindeki her dosya
kendi ayrı crate'i olarak derlenir, bu da son kullanıcıların crate'inizi kullanma
şekline daha yakından benzemek için ayrı kapsamlar oluşturmak için kullanışlıdır.
Ancak bu, _tests_ dizinindeki dosyaların, 7. Bölümde kodun modüllere ve dosyalara
ayrılmasıyla ilgili öğrendiğiniz gibi, _src_ dizinindeki dosyalarla aynı
davranışı paylaşmadığı anlamına gelir.

_tests_ dizinindeki dosyaların farklı davranışı, birden fazla entegrasyon test dosyasında kullanmak üzere bir dizi yardımcı işleviniz olduğunda ve
Bölüm 7'deki [“Modülleri Farklı Dosyalara Ayırma”][separating-modules-into-files]<!-- ignore --> bölümündeki adımları izleyerek
bunları ortak bir modüle çıkarmaya çalıştığınızda
en belirgin şekilde görülür.
Örneğin, _tests/common.rs_ dosyası oluşturup Örneğin, _tests/common.rs_ dosyasını oluşturup
içine `setup` adlı bir işlev yerleştirirsek, birden fazla test dosyasında birden fazla test işlevinden çağırmak istediğimiz bazı kodları `setup` işlevine ekleyebiliriz:

<span class="filename">Filename: tests/common.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/tests/common.rs}}
```

Testleri tekrar çalıştırdığımızda,
_common.rs_ dosyası için test çıktısında yeni bir bölüm göreceğiz, ancak bu dosya herhangi bir test işlevi içermiyor ve
`setup` işlevini hiçbir yerden çağırmadık:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/output.txt}}
```

Test sonuçlarında `common` görünmesi ve bunun için `0 test çalıştırılıyor` mesajının görüntülenmesi
istediğimiz bir durum değil. Biz sadece diğer entegrasyon test dosyalarıyla bazı kodları paylaşmak
istiyorduk. Test çıktısında `common` görünmesini önlemek için,
_tests/common.rs_ dosyası oluşturmak yerine, _tests/common/mod.rs_ dosyası oluşturacağız.
Proje dizini şimdi şöyle görünüyor:

```text
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

Bu, Rust'un da anladığı, Bölüm 7'deki [“Alternatif Dosya Yolları”][alt-paths]<!-- ignore --> bölümünde bahsettiğimiz eski adlandırma kuralıdır.
Dosyayı bu şekilde adlandırmak, Rust'a `common` modülünü entegrasyon testi dosyası olarak
değerlendirmesini söylemez.
`setup` işlevinin kodunu _tests/common/mod.rs_ dosyasına taşıyıp `setup` işlev kodunu _tests/common/mod.rs_ dosyasına taşıyıp
_tests/common.rs_ dosyasını sildiğimizde, test çıktısındaki bölüm artık
görünmeyecektir. _tests_ dizinindeki alt dizinlerdeki dosyalar ayrı
krate olarak derlenmez veya test çıktısında bölümleri olmaz.

_tests/common/mod.rs_ dosyasını oluşturduktan sonra, bunu herhangi bir
entegrasyon test dosyasında modül olarak kullanabiliriz. Aşağıda, _tests/integration_test.rs_ dosyasındaki `it_adds_two` testinden `setup`
işlevini çağırmanın bir örneği verilmiştir:

<span class="filename">Filename: tests/integration_test.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-13-fix-shared-test-code-problem/tests/integration_test.rs}}
```

`mod common;` bildiriminin, Listing 7-21'de gösterdiğimiz modül bildirimi ile aynı olduğunu unutmayın.
Ardından, test işlevinde
`common::setup()` işlevini çağırabiliriz.

#### İkili Kratlar için Entegrasyon Testleri

Projemiz yalnızca _src/main.rs_ dosyası içeren ve
_src/lib.rs_ dosyası içermeyen bir ikili krat ise,
_tests_ dizininde entegrasyon testleri oluşturamayız ve _src/main.rs_ dosyasında tanımlanan işlevleri
`use` deyimi ile kapsam içine alamayız. Yalnızca kütüphane krate'leri diğer
krate'lerin kullanabileceği işlevleri ortaya çıkarır; ikili krate'ler kendi başlarına çalışmak üzere tasarlanmıştır.

Bu, ikili dosya sağlayan Rust projelerinin,
_src/lib.rs_ dosyasında bulunan mantığı çağıran basit bir _src/main.rs_ dosyasına sahip olmasının nedenlerinden biridir.
Bu yapıyı kullanarak, entegrasyon testleri önemli işlevselliği kullanılabilir hale getirmek için
`use` ile kütüphane krate'ini test edebilir. Önemli işlevsellik çalışıyorsa, _src/main.rs_
dosyasındaki küçük miktardaki kod da çalışacaktır ve bu küçük miktardaki kodun test edilmesine gerek yoktur.
Önemli işlevsellik çalışmıyorsa, küçük miktardaki kod da çalışmayacaktır ve bu durumda küçük miktardaki kodun test edilmesine gerek yoktur.

## Özet

Rust'un test özellikleri, kodun nasıl çalışması gerektiğini belirlemenin bir yolunu sunar.
Böylece, değişiklikler yaparken bile kodun beklediğiniz gibi çalışmaya devam etmesini sağlar. Birim testleri,
kütüphanenin farklı bölümlerini ayrı ayrı çalıştırır ve özel
uygulama ayrıntılarını test edebilir. Entegrasyon testleri, kütüphanenin birçok bölümünün
birlikte doğru şekilde çalıştığını kontrol eder ve kütüphanenin genel API'sini kullanarak kodu,
harici kodun kullanacağı şekilde test eder. Rust'un tür sistemi ve
sahiplik kuralları bazı türdeki hataları önlemeye yardımcı olsa da, kodunuzun nasıl çalışması gerektiği ile ilgili mantık hatalarını azaltmak için testler hala önemlidir.

Bu bölümde ve önceki bölümlerde öğrendiğiniz bilgileri birleştirerek bir proje üzerinde çalışalım!


[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md
[separating-modules-into-files]: ch07-05-separating-modules-into-different-files.md
[alt-paths]: ch07-05-separating-modules-into-different-files.md#alternatif-dosya-yolları
