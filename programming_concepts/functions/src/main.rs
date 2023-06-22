fn main() {
    let sum = my_func(11, 22);
    println!("The sum is: {}", sum);
}

fn my_func(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    x + y
}
