## Ek E - Sürümler

1. Bölümde, `cargo new` komutunun _Cargo.toml_ dosyanıza bir sürümle ilgili
biraz meta veri eklediğini gördünüz. Bu ek, bunun ne anlama geldiğini anlatıyor!

Rust dili ve derleyicisi altı haftalık bir sürüm döngüsüne sahiptir, yani kullanıcılar
sürekli yeni özellikler alırlar. Diğer programlama dilleri daha büyük değişiklikleri
daha seyrek yayınlar; Rust ise daha küçük güncellemeleri daha sık yayınlar. Bir süre sonra,
tüm bu küçük değişiklikler birikir. Ancak sürümden sürüme, geriye dönüp "Vay canına, Rust 1.10 ile Rust 1.31 arasında Rust
çok değişmiş!" demek zor olabilir.
Her üç yılda bir, Rust ekibi yeni bir Rust _sürümü_ üretir. Her

Rust ekibi, yaklaşık üç yılda bir yeni bir Rust sürümü yayınlar. Her
sürüm, tamamen güncellenmiş belgeler ve araçlarla birlikte, net bir pakette yer alan
özellikleri bir araya getirir. Yeni sürümler, olağan altı haftalık sürüm sürecinin bir parçası olarak
yayınlanır.

Sürümler farklı kişiler için farklı amaçlara hizmet eder:

- Aktif Rust kullanıcıları için yeni bir sürüm, artımlı değişiklikleri
  anlaşılması kolay bir pakette bir araya getirir.
- Kullanıcı olmayanlar için yeni bir sürüm, bazı önemli gelişmelerin
  gerçekleştiğini gösterir ve bu da Rust'u tekrar gözden geçirmeye değer hale getirebilir.
- Rust geliştirenler için yeni bir sürüm, projeyi bir bütün olarak bir araya getiren
  bir odak noktası sağlar.

Bu yazının yazıldığı tarihte, dört Rust sürümü mevcuttur: Rust 2015, Rust
2018, Rust 2021 ve Rust 2024. Bu kitap, Rust 2024 sürümünün
deyimleri kullanılarak yazılmıştır.

_Cargo.toml_ içindeki `edition` anahtarı, derleyicinin kodunuz için hangi sürümü
kullanması gerektiğini belirtir. Anahtar mevcut değilse, Rust geriye dönük uyumluluk
nedenleriyle sürüm değeri olarak `2015` kullanır.

Her proje, varsayılan 2015 sürümü dışında bir sürümü seçebilir.
Sürümler, koddaki tanımlayıcılarla çakışan yeni bir anahtar kelime eklemek gibi
uyumsuz değişiklikler içerebilir. Ancak, bu değişiklikleri seçmediğiniz sürece,
kullandığınız Rust derleyici sürümünü yükseltirken bile kodunuz derlenmeye devam
edecektir.

Tüm Rust derleyici sürümleri, o derleyicinin yayınlanmasından önce var olan tüm sürümleri destekler
ve desteklenen tüm sürümlerin kütüphanelerini birbirine bağlayabilir.
Sürüm değişiklikleri yalnızca derleyicinin kodu ilk olarak nasıl ayrıştırdığına etki eder.
Bu nedenle, Rust 2015 kullanıyorsanız ve bağımlılıklarınızdan biri Rust 2018 kullanıyorsa,
projeniz derlenecek ve o bağımlılığı kullanabilecektir. Ters
durumda, projeniz Rust 2018 kullanırken bir bağımlılık Rust 2015 kullanıyorsa,
bu da sorunsuz çalışır.

Açıkça belirtmek gerekirse: çoğu özellik tüm sürümlerde kullanılabilir olacaktır. Herhangi bir Rust sürümünü kullanan geliştiriciler, yeni kararlı sürümler
yayınlandıkça iyileştirmeleri görmeye devam edeceklerdir. Ancak, bazı durumlarda,
özellikle yeni anahtar kelimeler eklendiğinde, bazı yeni özellikler yalnızca daha
sonraki sürümlerde kullanılabilir olabilir. Bu tür özelliklerden yararlanmak istiyorsanız sürümleri değiştirmeniz
gerekecektir.

Daha fazla ayrıntı için, [_Sürüm Kılavuzu_](https://doc.rust-lang.org/stable/edition-guide/) sürümler hakkında
sürümler arasındaki farkları sıralayan ve `cargo fix` ile kodunuzu yeni bir
sürüme otomatik olarak nasıl yükselteceğinizi açıklayan eksiksiz bir kitaptır.
