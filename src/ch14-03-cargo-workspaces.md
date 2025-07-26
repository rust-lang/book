## Kargo Çalışma Alanları

Bölüm 12'de, bir ikili sandık ve bir kütüphane
sandığı içeren bir paket oluşturduk. Projeniz geliştikçe, kütüphane sandığının
büyümeye devam ettiğini ve paketinizi
birden fazla kütüphane sandığına bölmek istediğinizi fark edebilirsiniz. Cargo,
birlikte geliştirilen birden fazla ilgili paketin yönetilmesine yardımcı olabilecek _workspaces_ adlı bir özellik sunar.

### Çalışma Alanı Oluşturma

Bir _workspace_, aynı
dizinini ve çıktısını paylaşan bir dizi pakettir. Bir çalışma alanı kullanarak bir proje yapalım-önemsiz bir kod kullanacağız, böylece
çalışma alanının yapısına konsantre olabiliriz. Bir çalışma alanını
yapılandırmanın birden fazla yolu vardır, bu yüzden sadece yaygın bir yolu göstereceğiz. Bir ikili ve iki kütüphane içeren bir
çalışma alanımız olacak. Ana işlevselliği
sağlayacak olan ikili, iki kütüphaneye bağlı olacaktır. Bir kütüphane
bir `add_one` fonksiyonu ve diğer kütüphane bir `add_two` fonksiyonu sağlayacaktır.
Bu üç sandık aynı çalışma alanının parçası olacaktır. Çalışma alanı için
adresinde yeni bir dizin oluşturarak başlayacağız:

```console
$ mkdir add
$ cd add
```

Daha sonra, _add_ dizininde,
tüm çalışma alanını yapılandıracak olan_Cargo.toml_ dosyasını oluşturuyoruz. Bu dosyanın bir `[package]` bölümü olmayacak.
Bunun yerine, çalışma alanına
üyelerini eklememizi sağlayacak bir `[workspace]` bölümü ile başlayacaktır. Ayrıca,
`resolver` değerini `"3"` olarak ayarlayarak çalışma alanımızda Cargo'nun çözümleyici algoritmasının en son ve en büyük
sürümünü kullanmaya özen gösteriyoruz.

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-01-workspace/add/Cargo.toml}}
```

Daha sonra,
_add_ dizini içinde `cargo new` çalıştırarak `adder` ikili sandığını oluşturacağız:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-01-adder-crate/add
remove `members = ["adder"]` from Cargo.toml
rm -rf adder
cargo new adder
copy output below
-->

```console
$ cargo new adder
     Created binary (application) `adder` package
      Adding `adder` as member of workspace at `file:///projects/add`
```

Bir çalışma alanı içinde `cargo new` çalıştırıldığında, yeni oluşturulan
paketi otomatik olarak
_Cargo.toml_ çalışma alanındaki `[workspace]` tanımında yer alan `members` anahtarına eklenir:

```toml
{{#include ../listings/ch14-more-about-cargo/output-only-01-adder-crate/add/Cargo.toml}}
```
Bu noktada, `cargo build` komutunu çalıştırarak çalışma alanını oluşturabiliriz. _add_ dizininizdeki
dosyaları aşağıdaki gibi görünmelidir:

```text
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

Çalışma alanında, derlenen
eserlerinin içine yerleştirileceği en üst düzeyde bir _target_ dizini vardır; `adder` paketinin kendi
_target_ dizini yoktur. Cargo build`i
_adder_ dizini içinden çalıştırsak bile, derlenen eserler yine de _add/adder/target_ yerine _add/target_
dizinine yerleştirilecektir. Cargo, bir
çalışma alanındaki _target_ dizinini bu şekilde yapılandırır çünkü bir çalışma alanındaki sandıkların birbirlerine
bağlı olması amaçlanmıştır. Eğer her crate kendi _target_ dizinine sahip olsaydı, her crate
çalışma alanındaki diğer crate'lerin her birini yeniden derleyerek
artifact'ları kendi _target_ dizinine yerleştirirdi. Bir _target_ dizinini paylaşarak,
crates gereksiz yeniden oluşturmayı önleyebilir.

### Çalışma Alanında İkinci Paketi Oluşturma

Ardından, çalışma alanında başka bir üye paket oluşturalım ve buna
`add_one` adını verelim. add_one` adında yeni bir kütüphane sandığı oluşturun:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-02-add-one/add
remove `"add_one"` from `members` list in Cargo.toml
rm -rf add_one
cargo new add_one --lib
copy output below
-->

```console
$ cargo new add_one --lib
     Created library `add_one` package
      Adding `add_one` as member of workspace at `file:///projects/add`
```

En Üst düzey _Cargo.toml_ artık `members`
listesinde _add_one_ yolunu içerecektir:

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/Cargo.toml}}
```

Your _add_ directory should now have these directories and files:

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

In the _add_one/src/lib.rs_ file, let’s add an `add_one` function:

<span class="filename">Filename: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/add_one/src/lib.rs}}
```

artık ikili dosyamızın bulunduğu `adder` paketinin, kütüphanemizin bulunduğu `add_one`
paketine bağımlı olmasını sağlayabiliriz. Öncelikle, _adder/Cargo.toml_ dosyasına
`add_one` için bir yol bağımlılığı eklememiz gerekecek.

<span class="filename">Filename: adder/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/adder/Cargo.toml:6:7}}
```

Cargo doesn’t assume that crates in a workspace will depend on each other, so
we need to be explicit about the dependency relationships.

Next, let’s use the `add_one` function (from the `add_one` crate) in the
`adder` crate. Open the _adder/src/main.rs_ file and change the `main`
function to call the `add_one` function, as in Listing 14-7.

<Listing number="14-7" file-name="adder/src/main.rs" caption="Using the `add_one` library crate from the `adder` crate">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-07/add/adder/src/main.rs}}
```

</Listing>

Let’s build the workspace by running `cargo build` in the top-level _add_
directory!

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
```

To run the binary crate from the _add_ directory, we can specify which
package in the workspace we want to run by using the `-p` argument and the
package name with `cargo run`:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo run -p adder
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo run -p adder
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

This runs the code in _adder/src/main.rs_, which depends on the `add_one` crate.

#### Depending on an External Package in a Workspace

Notice that the workspace has only one _Cargo.lock_ file at the top level,
rather than having a _Cargo.lock_ in each crate’s directory. This ensures that
all crates are using the same version of all dependencies. If we add the `rand`
package to the _adder/Cargo.toml_ and _add_one/Cargo.toml_ files, Cargo will
resolve both of those to one version of `rand` and record that in the one
_Cargo.lock_. Making all crates in the workspace use the same dependencies
means the crates will always be compatible with each other. Let’s add the
`rand` crate to the `[dependencies]` section in the _add_one/Cargo.toml_ file
so we can use the `rand` crate in the `add_one` crate:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
-->

<span class="filename">Filename: add_one/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add/add_one/Cargo.toml:6:7}}
```

We can now add `use rand;` to the _add_one/src/lib.rs_ file, and building the
whole workspace by running `cargo build` in the _add_ directory will bring in
and compile the `rand` crate. We will get one warning because we aren’t
referring to the `rand` we brought into scope:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
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

The top-level _Cargo.lock_ now contains information about the dependency of
`add_one` on `rand`. However, even though `rand` is used somewhere in the
workspace, we can’t use it in other crates in the workspace unless we add
`rand` to their _Cargo.toml_ files as well. For example, if we add `use rand;`
to the _adder/src/main.rs_ file for the `adder` package, we’ll get an error:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-03-use-rand/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
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

To fix this, edit the _Cargo.toml_ file for the `adder` package and indicate
that `rand` is a dependency for it as well. Building the `adder` package will
add `rand` to the list of dependencies for `adder` in _Cargo.lock_, but no
additional copies of `rand` will be downloaded. Cargo will ensure that every
crate in every package in the workspace using the `rand` package will use the
same version as long as they specify compatible versions of `rand`, saving us
space and ensuring that the crates in the workspace will be compatible with each
other.

If crates in the workspace specify incompatible versions of the same dependency,
Cargo will resolve each of them, but will still try to resolve as few versions
as possible.

#### Adding a Test to a Workspace

For another enhancement, let’s add a test of the `add_one::add_one` function
within the `add_one` crate:

<span class="filename">Filename: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add/add_one/src/lib.rs}}
```

Now run `cargo test` in the top-level _add_ directory. Running `cargo test` in
a workspace structured like this one will run the tests for all the crates in
the workspace:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test
copy output below; the output updating script doesn't handle subdirectories in
paths properly
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

The first section of the output shows that the `it_works` test in the `add_one`
crate passed. The next section shows that zero tests were found in the `adder`
crate, and then the last section shows zero documentation tests were found in
the `add_one` crate.

We can also run tests for one particular crate in a workspace from the
top-level directory by using the `-p` flag and specifying the name of the crate
we want to test:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test -p add_one
copy output below; the output updating script doesn't handle subdirectories in paths properly
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

This output shows `cargo test` only ran the tests for the `add_one` crate and
didn’t run the `adder` crate tests.

If you publish the crates in the workspace to
[crates.io](https://crates.io/)<!-- ignore -->, each crate in the workspace
will need to be published separately. Like `cargo test`, we can publish a
particular crate in our workspace by using the `-p` flag and specifying the
name of the crate we want to publish.

For additional practice, add an `add_two` crate to this workspace in a similar
way as the `add_one` crate!

As your project grows, consider using a workspace: it enables you to work with
smaller, easier-to-understand components than one big blob of code. Furthermore,
keeping the crates in a workspace can make coordination between crates easier if
they are often changed at the same time.
