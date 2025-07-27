## Derlemeleri Yayın Profilleriyle Özelleştirmek

Rust'ta _yayın profilleri_ (release profiles), farklı yapılandırmalara sahip, önceden tanımlanmış ve özelleştirilebilir profillerdir. Bu profiller, bir programcının kodu derlerken çeşitli seçenekler üzerinde daha fazla kontrol sahibi olmasını sağlar. Her profil, diğerlerinden bağımsız olarak yapılandırılır.

Cargo'nun iki ana profili vardır: `cargo build` komutunu çalıştırdığınızda kullanılan `dev` profili ve `cargo build --release` komutunu çalıştırdığınızda kullanılan `release` profili. `dev` profili, geliştirme için iyi varsayılanlarla, `release` profili ise yayın derlemeleri için iyi varsayılanlarla tanımlanmıştır.

Bu profil adları, derleme çıktılarınızdan tanıdık gelebilir:

<!-- manuel-yenileme
herhangi bir yerde şunu çalıştırın:
cargo build
cargo build --release
ve aşağıdaki çıktının doğru olduğundan emin olun
-->

```console
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
$ cargo build --release
    Finished `release` profile [optimized] target(s) in 0.32s
```

`dev` ve `release`, derleyici tarafından kullanılan farklı profillerdir.

Cargo, projede _Cargo.toml_ dosyanızda herhangi bir `[profile.*]` bölümü eklemediğinizde, her profil için varsayılan ayarlara sahiptir. Özelleştirmek istediğiniz herhangi bir profil için `[profile.*]` bölümleri ekleyerek, varsayılan ayarların herhangi bir alt kümesini geçersiz kılabilirsiniz. Örneğin, `dev` ve `release` profilleri için `opt-level` ayarının varsayılan değerleri şunlardır:

<span class="filename">Dosya Adı: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

`opt-level` ayarı, Rust'ın kodunuza uygulayacağı optimizasyon sayısını kontrol eder ve 0 ile 3 arasında bir değere sahiptir. Daha fazla optimizasyon uygulamak derleme süresini uzatır, bu nedenle geliştirme aşamasında kodunuzu sık sık derliyorsanız, daha hızlı derleme için daha az optimizasyon istersiniz; bu durumda ortaya çıkan kod daha yavaş çalışsa bile. Bu yüzden `dev` için varsayılan `opt-level` değeri `0`'dır. Kodunuzu yayımlamaya hazır olduğunuzda ise, daha fazla derleme süresi harcamak en iyisidir. Yayın modunda yalnızca bir kez derleme yaparsınız, ancak derlenmiş programı birçok kez çalıştırırsınız; bu nedenle yayın modu, daha hızlı çalışan kod için daha uzun derleme süresiyle takas yapar. Bu yüzden `release` profili için varsayılan `opt-level` değeri `3`'tür.

Varsayılan bir ayarı, _Cargo.toml_ dosyanıza farklı bir değer ekleyerek geçersiz kılabilirsiniz. Örneğin, geliştirme profilinde optimizasyon seviyesini 1 yapmak istersek, proje _Cargo.toml_ dosyamıza şu iki satırı ekleyebiliriz:

<span class="filename">Dosya Adı: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 1
```

Bu kod, varsayılan `0` ayarını geçersiz kılar. Artık `cargo build` komutunu çalıştırdığımızda, Cargo `dev` profili için varsayılanların yanı sıra bizim `opt-level` özelleştirmemizi de kullanacaktır. `opt-level`'ı `1` olarak ayarladığımız için, Cargo varsayılandan daha fazla, ancak yayın derlemesinden daha az optimizasyon uygulayacaktır.

Her profil için tüm yapılandırma seçeneklerinin ve varsayılanların tam listesi için [Cargo’nun belgelerine](https://doc.rust-lang.org/cargo/reference/profiles.html) bakabilirsiniz.
