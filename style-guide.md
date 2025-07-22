# Stil Kılavuzu

## Prose

- Bölüm/bölüm başlıkları için başlık büyük harfini tercih edin, örneğin: `## Gizli Sayı Oluşturma`
  yerine `## Gizli sayı oluşturma`.
- Bir terimi belirtirken tek tırnak yerine italik yazı kullanın, örneğin: `is an
  *associated function* of` yerine `is an ‘associated function’ of`.
- Metinde bir yöntemden bahsederken parantez kullanmayın, örneğin:
`read_line` yerine `read_line()`.
- 80 karakterde sert satır sonu
- Bir kelimede kod ve kod olmayanları karıştırmamayı tercih edin, örneğin: ```use std::io` yazdığımızı hatırlıyor musun?`` yerine ```use`d `std::io` kullandığımızı hatırlıyor musun?``
  .

## Kod

- Uygun olduğunda, hangi dosyadan bahsettiğimizi netleştirmek için markdown bloklarının önüne dosya adını ekleyin.

- Kodda değişiklik yaparken, kodun hangi kısımlarının değiştiğini ve
- Kodda değişiklik yaparken, kodun hangi kısımlarının değiştiğini ve hangilerinin aynı kaldığını açıkça belirtin... bunu nasıl yapacağımı henüz bilmiyorum

- Uzun satırları mümkünse 80 karakterin altında kalacak şekilde uygun şekilde bölün

- Komut satırı çıktı kod blokları için `bash` sözdizimi vurgulamayı kullanın


## Bağlantılar

Tüm komut dosyaları tamamlandıktan sonra:

- Bir bağlantı yazdırılmamalıysa, yok sayılması için işaretleyin.
  - Bu, HTML sürümü için bağlantı olması gereken tüm “Bölüm XX” kitap içi bağlantıları da içerir.
    
- Kitap içi bağlantıları ve stdlib API doküman bağlantılarını göreceli hale getirin, böylece kitap çevrimdışı okunsa da docs.rust-lang.org'da okunsa da çalışsınlar.
- Markdown bağlantıları kullanın ve bunların baskı sırasında `text at
  *url*` şeklinde değiştirileceğini unutmayın, bu nedenle bu formatta okunaklı olacak şekilde yazın.