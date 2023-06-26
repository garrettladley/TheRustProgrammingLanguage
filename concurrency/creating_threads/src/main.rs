fn creating_threads() {
    use std::thread;

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn main() {
    creating_threads();
}
