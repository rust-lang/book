# Bir I/O Projesi: Komut Satırı Programı Oluşturma

Bu bölüm, şimdiye kadar öğrendiğiniz birçok becerinin bir özeti ve
birkaç standart kütüphane özelliğini daha keşfedeceğiz. Bir komut satırı oluşturacağız
bazılarını uygulamak için dosya ve komut satırı girişi / çıkışı ile etkileşime giren araç
Rust kavramları artık elinizin altında.

Rust'ın hızı, güvenliği, tek ikili çıktısı ve çapraz platform desteği onu
komut satırı araçları oluşturmak için ideal bir dildir, bu nedenle projemiz için
klasik komut satırı arama aracı `grep`in kendi versiyonunu yapmak
(**g**lobally search a **r**egular **e**xpression and **p**rint). İçinde
En basit kullanım durumunda, `grep` belirtilen bir dosyada belirtilen bir dizeyi arar. için
Bunu yapmak için, `grep` argüman olarak bir dosya yolu ve bir dize alır. Sonra şöyle yazıyor
dosyayı bulur, bu dosyada dize argümanını içeren satırları bulur ve şunları yazdırır
bu satırları.

Yol boyunca, komut satırı aracımızın terminali kullanmasını nasıl sağlayacağımızı göstereceğiz
diğer birçok komut satırı aracının kullandığı özellikler. Bir komut satırının değerini okuyacağız
çevre değişkenini kullanarak kullanıcının aracımızın davranışını yapılandırmasına izin vereceğiz.
Ayrıca hata mesajlarını standart hata konsol akışına (`stderr`) yazdıracağız
standart çıktı (`stdout`) yerine, örneğin kullanıcının
hata mesajlarını ekranda görmeye devam ederken başarılı çıktıyı bir dosyaya yönlendirir.

Bir Rust topluluğu üyesi, Andrew Gallant, zaten tamamen
özellikli, çok hızlı `grep` sürümü, `ripgrep` olarak adlandırılır. Karşılaştırmak gerekirse, bizim
versiyonu oldukça basit olacaktır, ancak bu bölüm size bazı
gibi gerçek dünyadaki bir projeyi anlamak için ihtiyaç duyduğunuz arka plan bilgisi
`ripgrep`.

grep` projemiz şimdiye kadar öğrendiğiniz bir dizi kavramı birleştirecek:

- Düzenleme kodu ([Bölüm 7][ch7]<!-- ignore -->)
- Vektörleri ve dizeleri kullanma ([Bölüm 8][ch8]<!-- ignore -->)
- Hataların işlenmesi ([Bölüm 9][ch9]<!-- ignore -->)
- Uygun olan yerlerde özellikleri ve yaşam sürelerini kullanma ([Bölüm 10][ch10]<!-- görmezden gel -->)
- Test yazma ([Bölüm 11][ch11]<!-- ignore -->)

Ayrıca kapanışları, yineleyicileri ve özellik nesnelerini de kısaca tanıtacağız.
[13. Bölüm] [ch13]<!-- görmezden gel --> ve [18. Bölüm] [ch18]<!-- görmezden gel --> olacaktır.
ayrıntılı olarak ele alacağız.

[ch7]: ch07-00-managing-growing-projects-with-packages-crates-and-modules.md
[ch8]: ch08-00-common-collections.md
[ch9]: ch09-00-error-handling.md
[ch10]: ch10-00-generics.md
[ch11]: ch11-00-testing.md
[ch13]: ch13-00-functional-features.md
[ch18]: ch18-00-oop.md
