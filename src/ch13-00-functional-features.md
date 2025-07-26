# Fonksiyonel Dil Özellikleri: Yineleyiciler ve Kapanışlar

Rust'ın tasarımı, birçok mevcut dilden ve teknikten ilham almıştır ve önemli bir etkisi de _fonksiyonel programlama_ olmuştur. Fonksiyonel bir tarzda programlama genellikle fonksiyonları değer olarak kullanmayı, onları argüman olarak geçirmeyi, başka fonksiyonlardan döndürmeyi, daha sonra çalıştırmak üzere değişkenlere atamayı ve benzeri işlemleri içerir.

Bu bölümde, fonksiyonel programlamanın ne olup ne olmadığı tartışmasına girmeyeceğiz; bunun yerine, Rust'ta fonksiyonel olarak adlandırılan birçok dilde bulunan benzer özellikleri ele alacağız.

Daha spesifik olarak şunları ele alacağız:

- Bir değişkende saklayabileceğiniz fonksiyon benzeri bir yapı olan _kapanışlar_
- Bir dizi öğeyi işlemenin bir yolu olan _yineleyiciler_
- Kapanışları ve yineleyicileri 12. Bölümdeki G/Ç projesini geliştirmek için nasıl kullanacağımızı
- Kapanışların ve yineleyicilerin performansı (ipucu: düşündüğünüzden daha hızlılar!)

Daha önce, fonksiyonel tarzdan etkilenen desen eşleştirme ve enumlar gibi bazı Rust özelliklerini de ele aldık. Kapanışlar ve yineleyicilerde ustalaşmak, idiomatik ve hızlı Rust kodu yazmanın önemli bir parçası olduğundan, bu bölümün tamamını onlara ayıracağız.
