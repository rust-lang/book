# Hata İşleme

Hatalar yazılımda hayatın bir gerçeğidir, bu nedenle Rust bir şeylerin yanlış gittiği durumları ele almak için
bir dizi özelliğe sahiptir. Çoğu durumda, Rust
bir hata olasılığını kabul etmenizi ve
kodunuz derlenmeden önce bazı eylemlerde bulunmanızı gerektirir. Bu gereklilik,
kodunuzu üretime dağıtmadan önce
hataları keşfetmenizi ve uygun şekilde ele almanızı sağlayarak programınızı daha sağlam hale getirir!


Rust hataları iki ana kategoride toplar: Kurtarılabilir_ ve _kurtarılamaz_
hatalar. Kurtarılabilir bir hata için, örneğin _dosya bulunamadı_ hatası, büyük olasılıkla sadece
sorunu kullanıcıya bildirmek ve işlemi yeniden denemek isteriz.
Kurtarılamayan hatalar her zaman bir dizinin sonunun ötesindeki bir
konumuna erişmeye çalışmak gibi hataların belirtileridir ve bu nedenle
programını derhal durdurmak isteriz.

Çoğu dil bu iki hata türü arasında ayrım yapmaz ve istisnalar gibi mekanizmalar kullanarak
her ikisini de aynı şekilde ele alır. Rust'ta
istisnaları yoktur. Bunun yerine, kurtarılabilir hatalar için `Result<T, E>` tipine ve program
kurtarılamaz bir hatayla karşılaştığında yürütmeyi durduran
`panic!` makrosuna sahiptir. Bu bölümde önce `panic!` çağrısı ele alınacak, ardından
adresinde `Result<T, E>` değerlerinin döndürülmesinden bahsedilecektir. Ek olarak, bir hatadan kurtulmaya mı yoksa
yürütmesini durdurmaya mı karar verirken
hususlarını inceleyeceğiz.