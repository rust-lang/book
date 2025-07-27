## Cargo'yu Özel Komutlarla Genişletmek

Cargo, yeni alt komutlarla genişletilebilecek şekilde tasarlanmıştır; bunun için Cargo'nun kendisini değiştirmenize gerek yoktur. `$PATH`'inizdeki bir ikili dosyanın adı `cargo-birşey` ise, onu `cargo birşey` komutunu çalıştırarak bir Cargo alt komutu gibi kullanabilirsiniz. Bu tür özel komutlar, `cargo --list` komutunu çalıştırdığınızda da listelenir. `cargo install` ile uzantıları yükleyip, ardından bunları yerleşik Cargo araçları gibi çalıştırabilmek, Cargo'nun tasarımının çok pratik bir avantajıdır!

## Özet

Kodu Cargo ve [crates.io](https://crates.io/)<!-- ignore --> ile paylaşmak, Rust ekosistemini birçok farklı görev için kullanışlı kılan unsurlardan biridir. Rust'ın standart kütüphanesi küçük ve stabildir, ancak crate'ler dili farklı bir zaman çizelgesinde kolayca paylaşılabilir, kullanılabilir ve geliştirilebilir. Sizin için faydalı olan kodu [crates.io](https://crates.io/)<!-- ignore -->'da paylaşmaktan çekinmeyin; muhtemelen başkaları için de faydalı olacaktır!
