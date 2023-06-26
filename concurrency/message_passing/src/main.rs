use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx1, rx) = mpsc::channel();

    let tx2 = tx1.clone();

    thread::spawn(move || {
        vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ]
        .iter()
        .for_each(|val| {
            tx1.send(val.to_string()).unwrap();
            thread::sleep(Duration::from_secs(1));
        });
    });

    thread::spawn(move || {
        vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ]
        .iter()
        .for_each(|val| {
            tx2.send(val.to_string()).unwrap();
            thread::sleep(Duration::from_secs(1));
        });
    });

    rx.iter().for_each(|val| {
        println!("Got: {}", val);
    });
}
