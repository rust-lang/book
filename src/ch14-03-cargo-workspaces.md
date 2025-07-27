## Cargo Çalışma Alanları (Workspaces)

12. bölümde, bir ikili crate ve bir kütüphane crate'i içeren bir paket oluşturmuştuk. Projeniz geliştikçe, kütüphane crate'inizin büyümeye devam ettiğini ve paketinizi birden fazla kütüphane crate'ine bölmek isteyebileceğinizi fark edebilirsiniz. Cargo, birlikte geliştirilen birden fazla ilişkili paketi yönetmeye yardımcı olan _çalışma alanları_ (workspaces) adlı bir özellik sunar.

### Bir Çalışma Alanı Oluşturmak

Bir _çalışma alanı_, aynı _Cargo.lock_ ve çıktı dizinini paylaşan paketler kümesidir. Şimdi bir çalışma alanı kullanarak bir proje oluşturalım—odak noktamız çalışma alanının yapısı olacağı için basit kodlar kullanacağız. Bir çalışma alanını yapılandırmanın birden fazla yolu vardır; biz yaygın bir yolu göstereceğiz. Bir ikili ve iki kütüphane içeren bir çalışma alanımız olacak. Ana işlevselliği sağlayacak ikili, iki kütüphaneye bağımlı olacak. Kütüphanelerden biri `add_one` fonksiyonunu, diğeri ise `add_two` fonksiyonunu sağlayacak. Bu üç crate aynı çalışma alanının parçası olacak. Önce çalışma alanı için yeni bir dizin oluşturarak başlayalım:

```console
$ mkdir add
$ cd add
```

Sonra, _add_ dizininde, tüm çalışma alanını yapılandıracak _Cargo.toml_ dosyasını oluşturuyoruz. Bu dosyada bir `[package]` bölümü olmayacak. Bunun yerine, üyeleri çalışma alanına eklememizi sağlayan bir `[workspace]` bölümüyle başlayacak. Ayrıca, çalışma alanımızda Cargo'nun en güncel çözümleyici algoritmasını kullanmak için `resolver` değerini "3" olarak ayarlıyoruz.

<span class="filename">Dosya Adı: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-01-workspace/add/Cargo.toml}}
```

Şimdi, _add_ dizininde `cargo new` komutunu çalıştırarak `adder` adlı ikili crate'i oluşturalım:

<!-- manuel-yenileme
cd listings/ch14-more-about-cargo/output-only-01-adder-crate/add
members = ["adder"]'ı Cargo.toml'dan kaldırın
rm -rf adder
cargo new adder
aşağıdaki çıktıyı kopyalayın
-->

```console
$ cargo new adder
     Created binary (application) `adder` package
      Adding `adder` as member of workspace at `file:///projects/add`
```

Bir çalışma alanında `cargo new` çalıştırmak, oluşturulan paketi otomatik olarak çalışma alanı _Cargo.toml_'undaki `[workspace]` tanımındaki `members` anahtarına ekler:

```toml
{{#include ../listings/ch14-more-about-cargo/output-only-01-adder-crate/add/Cargo.toml}}
```

Bu noktada, `cargo build` komutunu çalıştırarak çalışma alanını derleyebiliriz. _add_ dizininizdeki dosyalar şöyle görünmelidir:

```text
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

Çalışma alanında derlenmiş çıktılar, en üst düzeydeki _target_ dizinine yerleştirilir; `adder` paketinin kendi _target_ dizini yoktur. Hatta _adder_ dizininde `cargo build` çalıştırsak bile, derlenmiş çıktılar yine _add/target_ dizininde olur. Cargo, çalışma alanındaki crate'ler birbirine bağımlı olacağı için _target_ dizinini bu şekilde yapılandırır. Her crate'in kendi _target_ dizini olsaydı, her crate diğerlerini kendi _target_ dizinine derlemek zorunda kalırdı. Ortak bir _target_ dizini paylaşarak, gereksiz yeniden derlemelerden kaçınılır.

### Çalışma Alanında İkinci Paketi Oluşturmak

Şimdi, çalışma alanında bir üye paket daha oluşturalım ve adını `add_one` koyalım. Yeni bir kütüphane crate'i oluşturun:

<!-- manuel-yenileme
cd listings/ch14-more-about-cargo/output-only-02-add-one/add
"add_one"'ı Cargo.toml'daki members listesinden kaldırın
rm -rf add_one
cargo new add_one --lib
aşağıdaki çıktıyı kopyalayın
-->

```console
$ cargo new add_one --lib
     Created library `add_one` package
      Adding `add_one` as member of workspace at `file:///projects/add`
```

Artık üst düzey _Cargo.toml_ dosyası, `members` listesinde _add_one_ yolunu da içerir:

<span class="filename">Dosya Adı: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/Cargo.toml}}
```

_add_ dizininizdeki dosya ve dizinler artık şöyle olmalı:

```text
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

_add_one/src/lib.rs_ dosyasına bir `add_one` fonksiyonu ekleyelim:

<span class="filename">Dosya Adı: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/add_one/src/lib.rs}}
```

Artık ikili crate'imiz olan `adder` paketinin, kütüphanemiz olan `add_one` paketine bağımlı olmasını sağlayabiliriz. Öncelikle, _adder/Cargo.toml_ dosyasına `add_one` için bir yol bağımlılığı eklememiz gerekecek.

<span class="filename">Dosya Adı: adder/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/adder/Cargo.toml:6:7}}
```

Cargo, çalışma alanındaki crate'lerin birbirine bağımlı olacağını varsaymaz; bu yüzden bağımlılık ilişkilerini açıkça belirtmemiz gerekir.

Şimdi, `adder` crate'inde, `add_one` fonksiyonunu kullanalım. _adder/src/main.rs_ dosyasını açıp, `main` fonksiyonunu, Liste 14-7'deki gibi `add_one` fonksiyonunu çağıracak şekilde değiştirin.

<Listing number="14-7" file-name="adder/src/main.rs" caption="`adder` crate'inden `add_one` kütüphanesini kullanmak">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-07/add/adder/src/main.rs}}
```

</Listing>

Şimdi, üst düzey _add_ dizininde `cargo build` komutunu çalıştırarak çalışma alanını derleyelim!

<!-- manuel-yenileme
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo build
aşağıdaki çıktıyı kopyalayın
-->

```console
$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
```

İkili crate'i _add_ dizininden çalıştırmak için, `cargo run` komutunda `-p` argümanı ve paket adını kullanarak hangi paketi çalıştırmak istediğimizi belirtebiliriz:

<!-- manuel-yenileme
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo run -p adder
aşağıdaki çıktıyı kopyalayın
-->

```console
$ cargo run -p adder
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

Bu, _adder/src/main.rs_ dosyasındaki kodu çalıştırır; bu kod, `add_one` crate'ine bağımlıdır.

#### Çalışma Alanında Harici Bir Pakete Bağımlı Olmak

Dikkat edin, çalışma alanında her crate'in dizininde ayrı bir _Cargo.lock_ dosyası yerine, yalnızca üst düzeyde bir _Cargo.lock_ dosyası vardır. Bu, tüm crate'lerin aynı bağımlılık sürümünü kullandığından emin olmayı sağlar. Eğer _adder/Cargo.toml_ ve _add_one/Cargo.toml_ dosyalarına `rand` paketini eklersek, Cargo her ikisini de tek bir `rand` sürümüne çözümler ve bunu tek _Cargo.lock_ dosyasına kaydeder. Tüm crate'lerin aynı bağımlılıkları kullanması, crate'lerin her zaman birbiriyle uyumlu olmasını sağlar. Şimdi, `add_one` crate'inde `rand` paketini kullanabilmek için _add_one/Cargo.toml_ dosyasındaki `[dependencies]` bölümüne `rand` crate'ini ekleyelim:

<!-- `rand` sürümünü güncellerken, aşağıdaki dosyalarda da aynı sürümü kullandığınızdan emin olun:
* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
-->

<span class="filename">Dosya Adı: add_one/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add/add_one/Cargo.toml:6:7}}
```

Artık _add_one/src/lib.rs_ dosyasına `use rand;` ekleyebiliriz ve _add_ dizininde `cargo build` komutunu çalıştırmak, `rand` crate'ini indirip derleyecektir. Ancak, `rand`'ı scope'a aldığımız halde kullanmadığımız için bir uyarı alırız:

<!-- manuel-yenileme
cd listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add
cargo build
aşağıdaki çıktıyı kopyalayın
-->

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
   --snip--
   Compiling rand v0.8.5
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
warning: unused import: `rand`
 --> add_one/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `add_one` (lib) generated 1 warning (run `cargo fix --lib -p add_one` to apply 1 suggestion)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.95s
```

Artık üst düzey _Cargo.lock_ dosyası, `add_one`'ın `rand` bağımlılığına dair bilgileri içeriyor. Ancak, çalışma alanında bir yerde `rand` kullanılsa bile, diğer crate'lerde kullanmak için onların _Cargo.toml_ dosyalarına da `rand` eklememiz gerekir. Örneğin, `adder` paketinin _adder/src/main.rs_ dosyasına `use rand;` eklersek, bir hata alırız:

<!-- manuel-yenileme
cd listings/ch14-more-about-cargo/output-only-03-use-rand/add
cargo build
aşağıdaki çıktıyı kopyalayın
-->

```console
$ cargo build
  --snip--
   Compiling adder v0.1.0 (file:///projects/add/adder)
error[E0432]: unresolved import `rand`
 --> adder/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
```

Bunu düzeltmek için, `adder` paketinin _Cargo.toml_ dosyasını düzenleyip, onun için de `rand`'ı bağımlılık olarak belirtmeliyiz. `adder` paketini derlemek, _Cargo.lock_ dosyasına `adder` için de `rand` bağımlılığını ekler, ancak ek bir `rand` kopyası indirilmez. Cargo, çalışma alanındaki tüm crate'lerin aynı `rand` sürümünü kullandığından emin olur (uyumlu sürümler belirtildiği sürece), böylece hem yerden tasarruf edilir hem de crate'ler birbiriyle uyumlu olur.

Çalışma alanındaki crate'ler aynı bağımlılığın uyumsuz sürümlerini belirtirse, Cargo her biri için çözümleme yapar, ancak yine de mümkün olduğunca az sürüm kullanmaya çalışır.

#### Çalışma Alanına Test Eklemek

Bir başka geliştirme olarak, `add_one` crate'inde `add_one::add_one` fonksiyonunun testini ekleyelim:

<span class="filename">Dosya Adı: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add/add_one/src/lib.rs}}
```

Şimdi, üst düzey _add_ dizininde `cargo test` komutunu çalıştırın. Böyle yapılandırılmış bir çalışma alanında `cargo test` çalıştırmak, çalışma alanındaki tüm crate'lerin testlerini çalıştırır:

<!-- manuel-yenileme
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test
aşağıdaki çıktıyı kopyalayın
-->

```console
$ cargo test
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running unittests src/lib.rs (target/debug/deps/add_one-93c49ee75dc46543)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-3a47283c568d2b6a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Çıktının ilk bölümünde, `add_one` crate'indeki `it_works` testinin geçtiği görülüyor. Sonraki bölümde, `adder` crate'inde sıfır test bulunduğu, son bölümde ise `add_one` crate'inde sıfır dokümantasyon testi bulunduğu görülüyor.

Ayrıca, üst düzey dizinden `-p` bayrağı ve test etmek istediğimiz crate'in adını belirterek, çalışma alanındaki belirli bir crate'in testlerini çalıştırabiliriz:

<!-- manuel-yenileme
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test -p add_one
aşağıdaki çıktıyı kopyalayın
-->

```console
$ cargo test -p add_one
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/add_one-93c49ee75dc46543)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Bu çıktı, `cargo test` komutunun yalnızca `add_one` crate'indeki testleri çalıştırdığını, `adder` crate'indeki testleri çalıştırmadığını gösteriyor.

Çalışma alanındaki crate'leri [crates.io](https://crates.io/)<!-- ignore -->'ya yayımlarsanız, çalışma alanındaki her crate'i ayrı ayrı yayımlamanız gerekir. `cargo test` gibi, yayımlamak istediğiniz crate'in adını `-p` bayrağı ile belirterek yalnızca o crate'i yayımlayabilirsiniz.

Ek alıştırma olarak, bu çalışma alanına `add_one` crate'iyle benzer şekilde bir `add_two` crate'i ekleyin!

Projeniz büyüdükçe, bir çalışma alanı kullanmayı düşünün: bu, tek bir büyük kod yığını yerine daha küçük ve anlaşılır bileşenlerle çalışmanızı sağlar. Ayrıca, çalışma alanındaki crate'ler sık sık aynı anda değişiyorsa, aralarındaki koordinasyonu kolaylaştırır.
