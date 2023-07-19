use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // The chapters accompanying text states that the print should be :
    // Result: 10
    // But the current code yields :
    // Result: 9
    // An update to this should resolve the drift between text and code :
    // 1..=10 - is a range that is inclusive of 1 and inclusive of 10
    for _ in 0..=10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
