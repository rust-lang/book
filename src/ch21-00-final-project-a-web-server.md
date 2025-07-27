# Final Proje: Çoklu İş Parçacıklı Bir Web Sunucusu İnşa Etmek

Uzun bir yolculuk oldu, ancak kitabın sonuna ulaştık. Bu bölümde, son bölümlerde ele aldığımız bazı kavramları göstermek ve önceki dersleri de özetlemek için birlikte bir proje daha inşa edeceğiz.

Son projemiz için, "hello" diyen ve bir web tarayıcısında Şekil 21-1 gibi görünen bir web sunucusu yapacağız.

![rust'tan merhaba](img/trpl21-01.png)

<span class="caption">Şekil 21-1: Son ortak projemiz</span>

Web sunucusunu inşa etmek için planımız şunlar:

1. TCP ve HTTP hakkında biraz bilgi edinin.
2. Bir soket üzerinde TCP bağlantılarını dinleyin.
3. Az sayıda HTTP isteğini ayrıştırın.
4. Doğru bir HTTP yanıtı oluşturun.
5. Sunucumuzun verimini bir iş parçacığı havuzu ile artırın.

Başlamadan önce iki detaya değinmeliyiz. Birincisi, burada kullanacağımız yöntem Rust ile web sunucusu inşa etmenin en iyi yolu olmayacak. Topluluk üyeleri, [crates.io](https://crates.io/) üzerinde, bizim inşa edeceğimizden çok daha kapsamlı web sunucusu ve iş parçacığı havuzu uygulamaları sunan, üretime hazır birçok crate yayımladı. Ancak, bu bölümdeki amacımız kolay yolu seçmek değil, öğrenmenize yardımcı olmak. Rust bir sistem programlama dili olduğu için, çalışmak istediğimiz soyutlama seviyesini seçebilir ve diğer dillerde mümkün veya pratik olmayan daha alt seviyelere inebiliriz.

İkincisi, burada async ve await kullanmayacağız. Bir iş parçacığı havuzu inşa etmek başlı başına yeterince büyük bir meydan okuma; buna bir de async çalışma zamanı inşa etmeyi eklemeyeceğiz! Ancak, bu bölümde göreceğimiz bazı problemlere async ve await'in nasıl uygulanabileceğine de değineceğiz. Sonuçta, 17. Bölümde de belirttiğimiz gibi, birçok async çalışma zamanı işlerini yönetmek için iş parçacığı havuzları kullanır.

Bu nedenle, temel HTTP sunucusunu ve iş parçacığı havuzunu elle yazarak, gelecekte kullanabileceğiniz crate'lerin arkasındaki genel fikirleri ve teknikleri öğrenmenizi sağlayacağız.
