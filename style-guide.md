# Style Guide

## Düzyazı

- Bölüm/bölüm başlıkları için başlık harfini tercih edin, örn: `## Generating a Secret
  Number` yerine `## Generating a secret number`.
- Bir terimi çağırırken tek tırnak yerine italik harfleri tercih edin, örn: `'nin ‘ilişkili bir işlevidir` yerine `'nin *ilişkili işlevidir*`.
- Düzyazıda bir yöntemden bahsederken, parantezleri dahil ETMEYİN, örn:
  `read_line()` yerine `read_line`.
- 80 karakterde sert sarma
- Kod ve kod olmayanları tek bir kelimede karıştırmamayı tercih edin, örneğin: ``Hatırlayın ne zaman yazmıştık
  “std::io” kullandığımız zamanı hatırlayın`` yerine ``std::io`` kullanın


## Kod

- Hangi dosyayı kullandığımızı netleştirmek için markdown bloklarından önce dosya adını ekleyin
  uygun olduğunda, hakkında konuşmak.
- Kodda değişiklik yaparken, kodun hangi bölümlerinin değiştiğini açıkça belirtin
  ve hangileri aynı kaldı... bunu nasıl yapacağımdan henüz emin değilim
- Uzun satırları mümkünse 80 karakterin altında tutmak için uygun şekilde bölün
- Komut satırı çıktı kod blokları için `bash` sözdizimi vurgulamasını kullanma

## Bağlantılar

Tüm komut dosyaları tamamlandıktan sonra:

- Bir bağlantının yazdırılmaması gerekiyorsa, yok sayılmak üzere işaretleyin
  - Bu, bağlantı olması gereken tüm "Bölüm XX" kitap içi bağlantıları da içerir
    HTML sürümü için
- Kitap içi bağlantıları ve stdlib API doküman bağlantılarını göreceli hale getirin, böylece çalışsınlar
  kitap çevrimdışı olarak veya docs.rust-lang.org adresinden okunabilir
- Markdown bağlantılarını kullanın ve bunların `text at` olarak değiştirileceğini unutmayın.
  `*url*` şeklinde yazın, bu nedenle bu formatta iyi okunacak şekilde ifade edin