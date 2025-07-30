# Ortak Koleksiyonlar

Rust'ın standart kütüphanesi, aşağıdaki gibi adlandırılan bir dizi çok kullanışlı veri yapısı içerir
_collections_. Diğer veri türlerinin çoğu belirli bir değeri temsil eder, ancak
koleksiyonları birden fazla değer içerebilir. Yerleşik dizi ve tuple'ın aksine
türlerinde, bu koleksiyonların işaret ettiği veriler heap üzerinde depolanır.
veri miktarının derleme zamanında bilinmesine gerek olmadığı ve büyüyebileceği anlamına gelir
veya program çalıştıkça küçülür. Her koleksiyon türü farklıdır
yetenekleri ve maliyetleri ve mevcut durumunuz için uygun olanı seçmek
durum zaman içinde geliştireceğiniz bir beceridir. Bu bölümde şunları tartışacağız
Rust programlarında çok sık kullanılan üç koleksiyon:

- Bir _vector_ değişken sayıda değeri yan yana saklamanızı sağlar.
- Bir _string_ karakter koleksiyonudur. String` türünden daha önce bahsetmiştik
  daha önce bahsetmiştik, ancak bu bölümde bu konudan derinlemesine bahsedeceğiz.
- Bir _hash map_, bir değeri belirli bir anahtarla ilişkilendirmenize olanak tanır. Bu bir
  _map_ adı verilen daha genel bir veri yapısının özel bir uygulamasıdır.

Standart kütüphane tarafından sağlanan diğer koleksiyon türleri hakkında bilgi edinmek için,
belgelere][koleksiyonlara] bakın.

Vektörlerin, dizelerin ve hash haritalarının nasıl oluşturulacağını ve güncelleneceğini tartışacağız.
her birini özel kılan şey olarak.


[koleksiyonlar]: ../std/collections/index.md