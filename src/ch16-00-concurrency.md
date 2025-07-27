# Korkusuz Eşzamanlılık (Fearless Concurrency)

Eşzamanlı programlamayı güvenli ve verimli bir şekilde yönetmek, Rust'ın başlıca hedeflerinden bir diğeridir. Bir programın farklı bölümlerinin bağımsız olarak çalıştığı _eşzamanlı programlama_ (concurrent programming) ve farklı bölümlerin aynı anda çalıştığı _paralel programlama_ (parallel programming), daha fazla bilgisayar çoklu işlemcilerden yararlandıkça giderek daha önemli hale geliyor. Tarihsel olarak, bu bağlamlarda programlama zor ve hataya açık olmuştur. Rust bunu değiştirmeyi hedefliyor.

Başlangıçta, Rust ekibi bellek güvenliğini sağlamak ve eşzamanlılık problemlerini önlemenin iki ayrı zorluk olduğunu ve farklı yöntemlerle çözülmesi gerektiğini düşünüyordu. Zamanla, sahiplik (ownership) ve tip sistemlerinin, hem bellek güvenliğini _hem de_ eşzamanlılık problemlerini yönetmek için güçlü araçlar olduğu keşfedildi! Sahiplik ve tip denetimini kullanarak, birçok eşzamanlılık hatası Rust'ta çalışma zamanı hatası yerine derleme zamanı hatası olur. Böylece, bir eşzamanlılık hatasının çalışma zamanında hangi koşullarda ortaya çıktığını tekrar tekrar bulmaya çalışmak yerine, hatalı kod derlenmez ve sorunu açıklayan bir hata mesajı gösterir. Sonuç olarak, kodunuzu üzerinde çalışırken düzeltebilirsiniz; potansiyel olarak üretime gönderdikten sonra değil. Rust'ta bu özelliğe _korkusuz eşzamanlılık_ (fearless concurrency) diyoruz. Korkusuz eşzamanlılık, ince hatalardan arınmış ve yeniden düzenlemesi kolay kodlar yazmanızı sağlar.

> Not: Basitlik adına, birçok problemi _eşzamanlı_ olarak adlandıracağız; aslında _eşzamanlı ve/veya paralel_ demek daha doğru olurdu. Bu bölümde, _eşzamanlı_ dediğimizde lütfen zihninizde _eşzamanlı ve/veya paralel_ olarak düşünün. Sonraki bölümde, bu ayrım daha önemli olduğunda daha spesifik olacağız.

Birçok dil, eşzamanlılık problemlerini çözmek için sunduğu çözümler konusunda katıdır. Örneğin, Erlang mesajlaşma tabanlı eşzamanlılık için zarif işlevsellik sunar, ancak thread'ler arasında durumu paylaşmak için yalnızca dolaylı yollar sağlar. Yalnızca olası çözümlerin bir alt kümesini desteklemek, üst düzey diller için makul bir stratejidir; çünkü üst düzey bir dil, bazı kontrollerden vazgeçerek soyutlamalar sunma sözü verir. Ancak, alt düzey dillerin herhangi bir durumda en iyi performansı sağlayacak çözümü sunması ve donanım üzerinde daha az soyutlama yapması beklenir. Bu nedenle, Rust durumunuza ve gereksinimlerinize uygun şekilde problemleri modellemek için çeşitli araçlar sunar.

Bu bölümde ele alacağımız konular şunlardır:

- Aynı anda birden fazla kod parçası çalıştırmak için thread'ler oluşturmak
- Thread'ler arasında mesaj gönderen _mesajlaşma tabanlı_ eşzamanlılık
- Birden fazla thread'in aynı veriye erişebildiği _paylaşılan durumlu_ eşzamanlılık
- Rust'ın eşzamanlılık garantilerini hem standart kütüphanedeki hem de kullanıcı tanımlı türlere genişleten `Sync` ve `Send` trait'leri
