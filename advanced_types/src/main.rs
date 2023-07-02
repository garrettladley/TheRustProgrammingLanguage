use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        // --snip--
        Box::new(|| ())
    }

    fn bar() -> ! {
        // --snip--
    }

    impl<T> Option<T> {
        pub fn unwrap(self) -> T {
            match self {
                Some(val) => val,
                None => panic!("called `Option::unwrap()` on a `None` value"),
            }
        }
    }

    let s: &str = "Hello world!";

    fn generic<T: ?Sized>(t: &T) {
        // --snip--
    }
}
