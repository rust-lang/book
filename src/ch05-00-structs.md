# İlgili Verileri Yapılandırmak için Yapıları Kullanma

Bir _struct_ veya _structure_,
adresini birlikte paketlemenize ve anlamlı bir grup oluşturan birden fazla ilgili değeri adlandırmanıza olanak tanıyan özel bir veri türüdür. Eğer
nesne yönelimli bir dile aşinaysanız,_struct_ bir
nesnesinin veri niteliklerine benzer. Bu bölümde
tuple'ları struct'larla karşılaştırarak bildiklerinizi geliştirecek ve struct'ların
verileri gruplamak için ne zaman daha iyi bir yol olduğunu göstereceğiz.

Yapıların nasıl tanımlanacağını ve örneklendirileceğini göstereceğiz. Bir struct türüyle ilişkili davranışları belirtmek için
ilişkili fonksiyonları, özellikle de
_methods_ adı verilen ilişkili fonksiyonları nasıl tanımlayacağımızı tartışacağız. Yapılar ve enumlar
(Bölüm 6'da ele alınmıştır), Rust'ın derleme zamanı tip denetiminden tam olarak yararlanmak için
programınızın etki alanında yeni tipler oluşturmak için yapı taşlarıdır.