use std::{sync::Arc, sync::Mutex, thread};

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            })
        })
        .collect();

    handles.into_iter().for_each(|handle| {
        handle.join().unwrap();
    });

    println!("Result: {}", *counter.lock().unwrap());
}
