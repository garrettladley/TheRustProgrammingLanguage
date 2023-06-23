struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    println!(
        "The largest number is {}",
        get_largest(vec![5, 10, 15, 20, 25])
    );

    println!(
        "The largest characters is {}",
        get_largest(vec!["a", "e", "i", "o", "u"])
    );

    let p1 = Point { x: 5, y: 10 };
}

fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
