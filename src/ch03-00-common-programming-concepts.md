# Yaygın Programlama Kavramları

Bu bölüm hemen hemen her programlama dilinde yer alan kavramları
ve bunların Rust'ta nasıl çalıştığını ele almaktadır. Pek çok programlama dilinin
özünde pek çok ortak nokta vardır. Bu bölümde sunulan kavramların hiçbiri Rust'a özgü değildir
ancak bunları Rust bağlamında tartışacağız ve bu kavramların kullanımıyla ilgili kuralları
açıklayacağız.

Özellikle, değişkenler, temel tipler, fonksiyonlar, yorumlar,
ve kontrol akışı hakkında bilgi edineceksiniz. Bu temeller her Rust programında olacaktır ve
bunları erken öğrenmek size başlamak için güçlü bir çekirdek sağlayacaktır.

> #### Anahtar Kelimeler
>
> Rust dilinde, diğer dillerde olduğu gibi yalnızca
> dili tarafından kullanılmak üzere ayrılmış bir dizi _anahtar kelime_ vardır. Bu kelimeleri
> değişken veya fonksiyon adı olarak kullanamayacağınızı unutmayın. Anahtar sözcüklerin çoğunun
> özel anlamları vardır ve bunları Rust
> programlarınızda çeşitli görevleri yerine getirmek için kullanacaksınız; birkaçının şu anda onlarla ilişkili bir işlevi yoktur, ancak
> gelecekte Rust'a eklenebilecek işlevler için ayrılmıştır. Anahtar kelimelerin bir listesini
> [Ek A][appendix_a]<!-- ignore --> adresinde bulabilirsiniz.

[appendix_a]: appendix-01-keywords.md
