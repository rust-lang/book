# Desenler ve Eşleme

_Desenler_ (patterns), Rust'ta hem karmaşık hem de basit türlerin yapısına göre eşleme yapmak için kullanılan özel bir söz dizimidir. Desenleri `match` ifadeleri ve diğer yapılarla birlikte kullanmak, bir programın kontrol akışı üzerinde daha fazla kontrol sağlar. Bir desen, aşağıdakilerin bir kombinasyonundan oluşabilir:

- Sabitler (literals)
- Yapısı açılmış diziler, enum'lar, struct'lar veya tuple'lar
- Değişkenler
- Joker karakterler (wildcards)
- Yer tutucular (placeholders)

Bazı örnek desenler: `x`, `(a, 3)` ve `Some(Color::Red)`. Desenlerin geçerli olduğu bağlamlarda, bu bileşenler verinin şeklini tanımlar. Programımız, değerleri desenlerle eşleştirerek, belirli bir kod parçasını çalıştırmaya devam etmek için doğru şekle sahip olup olmadığını belirler.

Bir deseni kullanmak için, onu bir değerle karşılaştırırız. Eğer desen değere uyuyorsa, değerin parçalarını kodumuzda kullanırız. 6. bölümdeki desen kullanan `match` ifadelerini hatırlayın; örneğin, para ayırıcı makine örneği. Eğer değer, desenin şekline uyuyorsa, adlandırılmış parçaları kullanabiliriz. Uymuyorsa, desene bağlı kod çalışmaz.

Bu bölüm, desenlerle ilgili her şey için bir başvuru niteliğindedir. Desenlerin geçerli olduğu yerleri, refutable (çürütülebilir) ve irrefutable (çürütülemez) desenler arasındaki farkı ve görebileceğiniz farklı desen söz dizimlerini ele alacağız. Bölümün sonunda, desenleri birçok kavramı açık bir şekilde ifade etmek için nasıl kullanacağınızı biliyor olacaksınız.
