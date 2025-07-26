## Sürüm Profilleri ile Derlemeleri Özelleştirme

Rust'ta _release profiles_
programcının
kod derlemek için çeşitli seçenekler üzerinde daha fazla kontrol sahibi olmasını sağlayan farklı yapılandırmalara sahip önceden tanımlanmış ve özelleştirilebilir profillerdir. Her profil
diğerlerinden bağımsız olarak yapılandırılır.

Cargo'nun iki ana profili vardır: `cargo build` komutunu çalıştırdığınızda Cargo'nun kullandığı `dev` profili ve `cargo build` komutunu çalıştırdığınızda Cargo'nun kullandığı `release` profili
--release`. Dev` profili geliştirme için iyi varsayılanlarla tanımlanmıştır,
ve `release` profili sürüm derlemeleri için iyi varsayılanlara sahiptir.

Bu profil isimleri derlemelerinizin çıktılarından tanıdık gelebilir:

<!--
'u herhangi bir yerde manuel olarak yeniden oluşturun, çalıştırın:
cargo build
cargo build --release
ve aşağıdaki çıktının doğru olduğundan emin olun
-->

```console
$ cargo build
 0.00s içinde `dev` profili [optimize edilmemiş + debuginfo] hedef(ler)i tamamlandı
$ cargo build --release
 0.32s içinde `release` profili [optimize edilmiş] hedef(ler)i tamamlandı
```

`Dev` ve `release` derleyici tarafından kullanılan bu farklı profillerdir.

Cargo,
projenin _Cargo.toml_ dosyasına açıkça herhangi bir `[profile.*]` bölümü eklemediğinizde geçerli olan profillerin her biri için varsayılan ayarlara sahiptir.
Özelleştirmek istediğiniz herhangi bir profil için `[profile.*]` bölümleri ekleyerek
varsayılan ayarların herhangi bir alt kümesini geçersiz kılabilirsiniz. Örneğin, `dev` ve `release` profilleri için `opt-level` ayarı için varsayılan
değerleri aşağıda verilmiştir:

<span class="filename">Dosya adı: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

`Opt-level` ayarı
Rust'ın kodunuza uygulayacağı optimizasyon sayısını kontrol eder ve 0 ile 3 arasında değişir. Daha fazla optimizasyon uygulamak
derleme süresini uzatır; bu nedenle geliştirme aşamasındaysanız ve kodunuzu sık sık derliyorsanız
ortaya çıkan kod
daha yavaş çalışsa bile daha hızlı derlemek için daha az optimizasyon isteyeceksinizdir. Bu nedenle `dev` için varsayılan `opt-level` `0`dır. Kodunuzu yayınlamaya
hazır olduğunuzda, derlemeye daha fazla zaman harcamak en iyisidir. Yayınlama modunda yalnızca
bir kez derleme yapacaksınız, ancak derlenen programı birçok kez çalıştıracaksınız
bu nedenle yayınlama modu daha hızlı çalışan kod için daha uzun derleme süresini takas eder. Bu
yüzden `release` profili için varsayılan `opt-level` `3`tür.

Varsayılan ayarı,
 .toml_ dosyasına farklı bir değer ekleyerek geçersiz kılabilirsiniz. Örneğin,
geliştirme profilinde optimizasyon seviyesi 1'i kullanmak istiyorsak, bu iki satırı projemizin _Cargo.toml_
dosyasına ekleyebiliriz:

<span class="filename">Dosya adı: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 1
```

Bu kod varsayılan `0` ayarını geçersiz kılar. Şimdi `cargo build` çalıştırdığımızda,
Cargo, `dev` profili için varsayılanları ve
`opt-level` için özelleştirmemizi kullanacaktır. Opt-level` seçeneğini `1` olarak ayarladığımız için, Cargo varsayılandan daha fazla
optimizasyonu uygulayacaktır, ancak bir sürüm derlemesindeki kadar çok değildir.


Her profil için yapılandırma seçeneklerinin ve varsayılanların tam listesi için:
[Cargo’nun belgeleri](https://doc.rust-lang.org/cargo/reference/profiles.html).
