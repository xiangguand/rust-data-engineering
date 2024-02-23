use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(0));
    
    let data_clone0 = Arc::clone(&data);
    let handle0 = thread::spawn(move || {
        for _i in 0..50000 {
            let mut num = data_clone0.lock().unwrap();
            *num += 1;
        }
    });

    let data_clone1 = Arc::clone(&data);
    let handle1 = thread::spawn(move || {
        for _i in 0..50000 {
            let mut num = data_clone1.lock().unwrap();
            *num += 1;
        }
    });

    handle0.join().unwrap();
    handle1.join().unwrap();
    println!("Data: {}", *data.lock().unwrap());
}
