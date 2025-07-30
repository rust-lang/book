## Dosya Okuma

Şimdi `file_path`
argümanında belirtilen dosyayı okumak için işlevsellik ekleyeceğiz. Öncelikle bunu test etmek için bir örnek dosyaya ihtiyacımız var: birkaç satırda az miktarda metin ve bazı tekrarlanan kelimeler içeren bir dosya kullanacağız. Listing 12-3,
 bu iş için uygun bir Emily Dickinson şiiri içeriyor!
Projenizin kök dizininde Projenizin kök dizininde
_poem.txt_ adlı bir dosya oluşturun ve "I'm Nobody!
Who are you?" şiirini girin.

<Listing number="12-3" file-name="poem.txt" caption="A poem by Emily Dickinson makes a good test case.">

```text
{{#include ../listings/ch12-an-io-project/listing-12-03/poem.txt}}
```

</Listing>

Metin hazır olduğunda, _src/main.rs_ dosyasını düzenleyin ve dosyayı okumak için kod ekleyin,
Listing 12-4'te gösterildiği gibi.

<Listing number="12-4" file-name="src/main.rs" caption="Reading the contents of the file specified by the second argument">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/src/main.rs:here}}
```

</Listing>

Öncelikle, `use`
ifadesi ile standart kütüphanenin ilgili bir bölümünü getiriyoruz: dosyaları işlemek için `std::fs`'ye ihtiyacımız var.

`main` içinde, yeni `fs::read_to_string` ifadesi `file_path`'i alır,
bu dosyayı açar ve dosyanın içeriğini içeren `std::io::Result<String>` türünde bir değer döndürür.

Bundan sonra, dosyayı okuduktan sonra `contents` değerini yazdırmak için geçici bir `println!` ifadesi ekliyoruz, böylece programın

Bundan sonra, dosya okunduktan sonra `contents` değerini yazdırmak için geçici bir `println!` deyimi ekliyoruz, böylece programın
şimdiye kadar çalıştığını kontrol edebiliyoruz.

Bu kodu, ilk komut satırı argümanı olarak herhangi bir dize (çünkü

arama kısmını henüz uygulamadık) ve ikinci argüman olarak _poem.txt_ dosyası ile çalıştıralım:

```console
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/output.txt}}
```

Harika! Kod dosyayı okudu ve içeriğini yazdı. Ancak kodda
birkaç kusur var. Şu anda, `main` işlevi birden fazla
sorumluluğa sahip: genellikle, her işlev tek bir fikirden sorumlu olduğunda işlevler daha net ve bakımı daha kolay olur.
Diğer sorun ise, hataları olabildiğince iyi yönetemememizdir. Program hala küçük olduğundan, bu Diğer bir sorun ise, hataları
olabildiğince iyi yönetemememizdir. Program hala küçük olduğundan, bu
kusurlar büyük bir sorun teşkil etmiyor, ancak program büyüdükçe, bunları temiz bir şekilde düzeltmek
daha zor hale gelecektir. Bir program geliştirirken erken aşamada yeniden düzenlemeye başlamak
iyi bir uygulamadır, çünkü daha az miktarda kodu yeniden düzenlemek
çok daha kolaydır. Şimdi bunu yapacağız.