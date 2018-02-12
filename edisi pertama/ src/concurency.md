# Concurrency

Concurrency dan paralelisme adalah topik yang sangat penting dalam ilmu komputer
dan juga merupakan topik hangat di industri saat ini. Komputer semakin banyak
mendapatkan core, namun banyak programmer tidak siap untuk memanfaatkannya sepenuhnya.

Fitur keamanan memori Rust juga berlaku untuk bagian concurrency-nya. Bahkan
 semua program Rust  harus aman mengingat tidak ada persaingan data , Sesuai dengan tipe tugas sistem rust.
Tipe Sistem rust sampai pada tugasnya, dan memberi Anda cara yang ampuh untuk
memikirkan kode bersamaan pada waktu kompilasi.
 

Sebelum kita berbicara fitur konkurensi yang disertai Rust, penting untuk
memahami sesuatu:  rust cukup rendah sehingga sebagian besar ini disediakan 
oleh perpustakaan standar,  bukan oleh bahasa. Ini berarti bahwa jika 
Anda tidak menyukai beberapa aspek dari cara Rust menangani konkurensi
[mio](https://github.com/carllerche/mio) , ini adalah contoh nyata 
dari prinsip dalam sebuah tindakan

## Latar Belakang: `Send` dan `Sync`

Concurrency sulit dipikirkan. Di Rust, kita memiliki yang kuat, statis
jenis sistem untuk membantu kita alasan tentang kode kita. Dengan demikian, Rust memberi kita dua sifat
untuk membantu kita memahami kode yang mungkin bisa bersamaan.
### `Send`

Sifat yang pertama akan kita bicarakan adalah
[`Send`](../../std/marker/trait.Send.html). kapan sebuah tipe `T` menerapkan `Send`, ini 
menunjukkan bahwa sesuatu dari jenis ini dapat memiliki kepemilikan yang ditransfer 
dengan aman di antara thread.

Hal ini penting untuk memberlakukan pembatasan tertentu. Misalnya, jika kita memiliki
saluran yang menghubungkan dua thread, kita ingin bisa mengirim beberapa data
ke saluran dan ke thread lainnya. karena itu, kami memastikan bahwa `Send` diimplementasikan 
untuk tipe itu.

Sebaliknya, jika kita membungkus perpustakaan dengan [FFI][ffi]  yang tidak
aman dari thread, kita tidak ingin mengerjakan `Send`, dan kompilator akan membantu 
kita menegakkannya sehingga tidak dapat meninggalkan thread saat ini

[ffi]: ffi.html

### `Sync`

Sifat kedua dari sifat ini disebut [`Sync`](../../std/marker/trait.Sync.html).
Ketika sebuah tipe `T` diterapkan `Sync`, Ini menunjukkan sesuatu
dari jenis ini tidak mempunyai kemungkinan untuk mengenalkan memori yang tidak aman saat digunakan
beberapa thread secara bersamaan melalui referensi bersama. Ini menyiratkan itu
jenis yang tidak memiliki [interior mutability](mutability.html) secara inherently
`Sync`, yang mencakup tipe primitif sederhana s (like `u8`) dan tipe agregat 
mereka sama.

Untuk berbagi thread referensi di, Rust menyediakan jenis yang disebut
`Arc<T>`. `Arc<T>` penerapan `Send` dan `Sync` jika dan hanya jika... `T` penerapan
kedua `Send` dan `Sync`. untuk contoh, sebuah objek tipe `Arc<RefCell<U>>` tidak bisa
ditransfer melintasi threadn karena
[`RefCell`](choosing-your-guarantees.html#refcellt) tidak diterapkan
`Sync`, karena itu `Arc<RefCell<U>>` tidak diterapkan `Send`.

Kedua sifat ini memungkinkan Anda menggunakan jenis sistem untuk membuat jaminan yang kuat
Tentang rincian kode Anda di bawah concurrency. Sebelum kita berdemonstrasi
mengapa, kita perlu belajar bagaimana membuat program  bersamaan di sebuah 
tempat!

## Threads

Perpustakaan standar Rust menyediakan perpustakaan untuk urutan, yang memungkinkan Anda melakukannya
Jalankan kode Rust secara paralel. Berikut adalah contoh dasar penggunaan `std::thread`:

```rust
use std::thread;

fn main() {
    thread::spawn(|| {
        println!("Hello from a thread!");
    });
}
```

The `thread::spawn()` Metode menerima [closure](closures.html),yang dilaksanakan 
di thread baru.  Ia mengembalikan pegangan ke thread, yang bisa digunakan untuk menunggu 
benang anak selesai dan mengekstrak hasilnya:

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        "Hello from a thread!"
    });

    println!("{}", handle.join().unwrap());
}
```

Karena penutupan bisa mendapatkan variabel dari daerahnya, kita juga bisa mencoba
membawa beberapa data ke thread lain:

```rust,ignore
use std::thread;

fn main() {
    let x = 1;
    thread::spawn(|| {
        println!("x is {}", x);
    });
}
```

Namun, ini memberi kita kesalahan:

```text
5:19: 7:6 error: closure may outlive the current function, but it
                 borrows `x`, which is owned by the current function
...
5:19: 7:6 help: to force the closure to take ownership of `x` (and any other referenced variables),
          use the `move` keyword, as shown:
      thread::spawn(move || {
          println!("x is {}", x);
      });
```

Hal ini karena secara default menutup variabel dengan referensi, dan dengan
demikian penutupan hanya mendapat _referensi untuk `x`_. Ini adalah sebuah masalah, karena
thread bisa hidup lebih lama dari lingkup `x`, mengarah ke pointer.

Untuk memperbaikinya, kami menggunakan `move` penutupan seperti yang disebutkan dalam pesan kesalahan. `move`
penutupan dijelaskan secara depth [here](closures.html#move-closures); pada dasarnya
mereka memindahkan variabel dari lingkungan mereka ke dalam dirinya sendiri.

```rust
use std::thread;

fn main() {
    let x = 1;
    thread::spawn(move || {
        println!("x is {}", x);
    });
}
```

Banyak bahasa memiliki kemampuan untuk mengeksekusi thread, tapi sangat tidak aman.
Ada keseluruhan buku tentang bagaimana mencegah kesalahan yang terjadi dari shared
keadaan bisa berubah. rust membantu keluar dengan sistem jenis di ini juga, dengan mencegah
data ras pada waktu kompilasi Mari kita bicara tentang bagaimana Anda benar-benar berbagi sesuatu
antara thread.

## Safe Shared Mutable State

Karena jenis sistem Rust, kita memiliki konsep yang kedengarannya seperti kebohongan "safe
shared mutable state."Banyak programmer setuju bahwa keadaan mutable bersama adalah
sangat sangat buruk.

Seseorang pernah mengatakan ini:

> Keadaan bersama mutable adalah akar dari semua kejahatan. Kebanyakan bahasa berusaha untuk menangani
> Dengan masalah ini melalui bagian 'murtabel', tapi Rust menangani hal itu
> memecahkan 'shared' bagian.

sama [ownership system](ownership.html) yang membantu mencegah salah petunjuk 
penggunaan  juga membantu menyingkirkan data ras, salah satu jenis
bug konkurensi terburuk 

Sebagai contoh, di sini adalah program Rust yang memiliki banyak data dalam race
bahasa. Ini tidak akan dikompilasi:

```rust,ignore
use std::thread;
use std::time::Duration;

fn main() {
    let mut data = vec![1, 2, 3];

    for i in 0..3 {
        thread::spawn(move || {
            data[0] += i;
        });
    }

    thread::sleep(Duration::from_millis(50));
}
```

Ini memberi kita sebuah kesalahan

```text
8:17 error: capture of moved value: `data`
        data[0] += i;
        ^~~~
```

rust tahu ini tidak akan aman! Jika kita punya referensi ke `data` di setiap
thread, dan thread mengambil kepemilikan referensi, kita akan memiliki tiga pemilik!
`data` akan pindah dari `main` dalam panggilan pertama untuk `spawn()`, jadi selanjutnya
Panggilan dalam lingkaran tidak dapat menggunakan variabel ini.

jadi, kita memerlukan beberapa jenis yang memungkinkan kita memiliki lebih dari satu referensi 
ke nilai. Biasanya, akan  kita gunakan `Rc<T>` Untuk ini, yang merupakan tipe referensi dihitung
yang menyediakan kepemilikan bersama. Ini memiliki beberapa pembukuan runtime yang terus melacak
dari jumlah rujukannya, maka bagian "referensi dihitung" dari namanya.

panggilan `clone()` pada `Rc<T>`wasiat akan mengembalikan referensi yang dimiliki 
baru dan menemukan referensi internal. Kami membuat satu untuk masing-masing thread:


```rust,ignore
use std::thread;
use std::time::Duration;
use std::rc::Rc;

fn main() {
    let mut data = Rc::new(vec![1, 2, 3]);

    for i in 0..3 {
        // Create a new owned reference:
        let data_ref = data.clone();

        // Use it in a thread:
        thread::spawn(move || {
            data_ref[0] += i;
        });
    }

    thread::sleep(Duration::from_millis(50));
}
```

Ini tidak akan berhasil, dan akan memberi kita kesalahan::

```text
13:9: 13:22 error: the trait bound `alloc::rc::Rc<collections::vec::Vec<i32>> : core::marker::Send`
            is not satisfied
...
13:9: 13:22 note: `alloc::rc::Rc<collections::vec::Vec<i32>>`
            cannot be sent between threads safely
```

Seperti yang disebutkan pesan kesalahan, `Rc` tidak dapat dikirim antara benang dengan aman.  ini
adalah karena jumlah referensi internal tidak dipertahankan dalam thread-safe
cara dan bisa memiliki data race.

Untuk mengatasi ini, kita akan menggunakan `Arc<T>`, Tipe referensi atom standar rust.

Bagian atom berarti `Arc<T>` dapat diakses dengan aman dari banyak thread.
Untuk melakukan ini, kompiler menjamin bahwa mutasi dari hitungan internal 
menggunakan operasi yang tidak dapat dibagi yang tidak dapat memiliki data ras.

Intinya, `Arc<T>` adalah jenis yang memungkinkan kita berbagi kepemilikan data _across
threads_.


```rust,ignore
use std::thread;
use std::sync::Arc;
use std::time::Duration;

fn main() {
    let mut data = Arc::new(vec![1, 2, 3]);

    for i in 0..3 {
        let data = data.clone();
        thread::spawn(move || {
            data[0] += i;
        });
    }

    thread::sleep(Duration::from_millis(50));
}
```

Begitu pula untuk terakhir kalinya, kita gunakan  `clone()` untuk membuat pegangan baru yang dimiliki.
Pegangan ini kemudian dipindahkan ke thread baru.

Dan ... masih memberi kita sebuah kesalahan.

```text
<anon>:11:24 error: cannot borrow immutable borrowed content as mutable
<anon>:11                    data[0] += i;
                             ^~~~
```

`Arc<T>`secara default memiliki isi yang tidak berubah. Hal ini memungkinkan _sharing_ odata antar
thread, namun data yang dapat dibagi bersama tidak aman-dan bila 
thread terlibat dapat menyebabkan data berantakan!

Biasanya ketika kita ingin membuat sesuatu dalam posisi yang tidak berubah, kita menggunakan
`Cell<T>` atau `RefCell<T>` membiarkan mutasi aman melalui pemeriksaan runtime atau sebaliknya
sebaliknya (lihat juga: [Choosing Your Guarantees](choosing-your-guarantees.html)).
namun, mirip dengan `Rc`, ini tidak thread-aman. Jika kita mencoba menggunakan ini, 
akan mendapatkan kesalahan tentang jenis ini tidak `Sync`, dan kodenya akan gagal
tersusun.

Sepertinya kita memerlukan beberapa jenis yang memungkinkan kita untuk secara aman mengubah nilai bersama
melintasi benang, misalnya tipe yang hanya bisa memastikan satu benang pada satu waktu
mampu mengubah nilai di dalamnya dalam satu waktu.

Untuk itu, kita bisa menggunakan `Mutex<T>` jenis!

Berikut versi bekerja:

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    for i in 0..3 {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[0] += i;
        });
    }

    thread::sleep(Duration::from_millis(50));
}
```

Perhatikan bahwa nilai `i` terikat (disalin) sampai penutupan dan tidak dibagi
di antara benang.

kami "mengunci" mutex di sini. Sebuah mutex (singkatan dari "mutual exclusion"), seperti
yang disebutkan, hanya memungkinkan satu thread pada satu waktu untuk mengakses sebuah nilai. Bila kita 
ingin mengakses nilai, kita gunakan `lock()` ini di atasnya. Ini akan "kunci" si mutex, dan tidak
ada thread lain yang bisa menguncinya (dan karenanya, melakukan apapun dengan nilainya)
ampai kita selesai melakukannya. Jika sebuah thread mencoba mengunci mutex yang sudah terkunci,
maka akan menunggu sampai thread lain melepaskan kunci.

kunci "rilis" di sini adalah implisit; ketika hasil kunci  (dalam kasus ini,
`data`) keluar dari ruang lingkup, kunci dilepaskan secara otomatis..

perhatikan bahwa [`lock`](../../std/sync/struct.Mutex.html#method.lock) metode
[`Mutex`](../../std/sync/struct.Mutex.html) memiliki tanda tangan ini:

```rust,ignore
fn lock(&self) -> LockResult<MutexGuard<T>>
```

dan karena `Send` tidak diimplementasikan untuk `MutexGuard<T>`, penjaga tidak dapat melewati batas benang,
memastikan area penguncian dan pelepasan benang.

Mari memeriksa tubuh benang lebih dekat:

```rust
# use std::sync::{Arc, Mutex};
# use std::thread;
# use std::time::Duration;
# fn main() {
#     let data = Arc::new(Mutex::new(vec![1, 2, 3]));
#     for i in 0..3 {
#         let data = data.clone();
thread::spawn(move || {
    let mut data = data.lock().unwrap();
    data[0] += i;
});
#     }
#     thread::sleep(Duration::from_millis(50));
# }
```

Pertama, kita panggil  `lock()`, yang mendapatkan kunci mutex. Karena ini mungkin gagal,
ia mengembalikan sebuah  `Result<T, E>`,  dan karena ini hanyalah sebuah contoh, kita `unwrap()`
bisa mendapatkan referensi ke data. Kode nyata akan memiliki penanganan kesalahan yang 
lebih kuat di sini. Kita kemudian bebas untuk bermutasi, karena kita memiliki kunci.

Terakhir, saat benangnya sedang berjalan, kita tunggu sebentar. Tapi ini tidak ideal: 
kita mungkin telah memilih waktu yang tepat untuk menunggu tapi kemungkinan kita akan 
menunggu lebih lama dari yang diperlukan atau tidak cukup lama, tergantung pada berapa
banyak waktu yang benar-benar dibutuhkan untuk menyelesaikan 
komputasi saat program berjalan

Alternatif yang lebih tepat untuk timer adalah menggunakan salah 
satu mekanisme yang disediakan oleh perpustakaan standar Rust untuk 
menyinkronkan benang satu sama lain. Mari kita bicara tentang salah satunya: saluran.

## saluran

Inilah versi kode kami yang menggunakan saluran untuk sinkronisasi
daripada menunggu waktu tertentu:

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

fn main() {
    let data = Arc::new(Mutex::new(0));

    // `tx` is the "transmitter" or "sender".
    // `rx` is the "receiver".
    let (tx, rx) = mpsc::channel();

    for _ in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());

        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;

            tx.send(()).unwrap();
        });
    }

    for _ in 0..10 {
        rx.recv().unwrap();
    }
}
```

Kami menggunakan `mpsc::channel()` metode untuk membuat saluran baru. kita `send`
sederhana `()` menyusuri saluran, dan kemudian menunggu sepuluh dari mereka untuk kembali.

Saat saluran ini mengirimkan sinyal generik, kami dapat mengirim data apa pun yang 
`Send` melebihi saluran!

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let tx = tx.clone();

        thread::spawn(move || {
            let answer = i * i;

            tx.send(answer).unwrap();
        });
    }

    for _ in 0..10 {
        println!("{}", rx.recv().unwrap());
    }
}
```

Di sini kita membuat 10 benang, meminta masing-masing untuk menghitung kuadrat sebuah angka (`i`
pada saat ini `spawn()`), dan kemudian `send()` kembali jawaban atas saluran.


## Panics

A `panic!` akan merusak thread yang sedang dijalankan. Anda bisa menggunakan 
thread Rust sebagai mekanisme isolasi sederhana::

```rust
use std::thread;

let handle = thread::spawn(move || {
    panic!("oops!");
});

let result = handle.join();

assert!(result.is_err());
```

`Thread.join()` memberi kita `Result` punggung, yang memungkinkan kita untuk 
memeriksa apakah benang telah panik atau tidak.
