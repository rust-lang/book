# Otomatik Testler Yazma

Edsger W. Dijkstra 1972 yılında yazdığı “The Humble Programmer” adlı makalesinde "program
testleri hataların varlığını göstermek için çok etkili bir yol olabilir, ancak
yokluklarını göstermek için umutsuzca yetersizdir." Bu demek değil ki
eli̇mi̇zden geldi̇ği̇nce test etmeye çalişalim!

Programlarımızdaki doğruluk, kodumuzun amaçladığımız şeyi ne ölçüde yaptığıdır
yapmak için. Rust, doğruluk konusunda yüksek derecede endişe ile tasarlanmıştır
ancak doğruluk karmaşıktır ve kanıtlanması kolay değildir. Rust'ın tipi
sistemi bu yükün büyük bir kısmını üstlenmektedir, ancak tip sistemi
her şey. Bu nedenle Rust, otomatik yazılım testleri yazmak için destek içerir.

Diyelim ki `add_two` fonksiyonunu yazdık ve bu fonksiyona aktarılan sayıya 2 ekledik
it. Bu fonksiyonun imzası parametre olarak bir tamsayı kabul eder ve bir
sonuç olarak tamsayı. Bu fonksiyonu uyguladığımızda ve derlediğimizde, Rust tüm
sağlamak için şimdiye kadar öğrendiğiniz tür denetimi ve ödünç denetimi
Örneğin, bir `String` değeri veya geçersiz bir referans aktarmadığımızı
bu işleve. Ancak Rust, bu işlevin tam olarak aşağıdakileri yapacağını kontrol edemez
amaçladığımız şey, parametre artı 2'yi döndürmek yerine, diyelim ki
parametre artı 10 veya parametre eksi 50! İşte burada testler devreye girer.

Örneğin, `3` değerini
'add_two' fonksiyonunun döndürdüğü değer `5`tir. Bu testleri her zaman çalıştırabiliriz
mevcut doğru davranışın değişmediğinden emin olmak için kodumuzda değişiklikler yaparız
değişti.

Test karmaşık bir beceridir: bir bölümde her ayrıntıyı ele alamasak da
İyi testlerin nasıl yazılacağı hakkında, bu bölümde testlerin mekaniğini tartışacağız.
Rust'ın test olanakları. Ek açıklamalar ve makrolar hakkında konuşacağız
testlerinizi yazarken kullanabileceğiniz varsayılan davranış ve seçenekler
testlerinizi çalıştırmak için sağlanan ve testleri birim testleri ve
entegrasyon testleri.