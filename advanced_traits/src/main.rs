fn main() {
    pub trait Iterator<T> {
        fn next(&mut self) -> Option<T>;
    }

    struct Counter {}

    impl Iterator<u32> for Counter {
        fn next(&mut self) -> Option<u32> {
            Some(0)
        }
    }

    impl Iterator<u16> for Counter {
        fn next(&mut self) -> Option<u16> {
            Some(0)
        }
    }

    use std::ops::Add;

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    struct Millimeters(u32);

    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    trait Pilot {
        fn fly();
    }

    trait Wizard {
        fn fly();
    }

    struct Human;

    impl Human {
        fn fly() {
            println!("*waving arms furiously*");
        }
    }

    impl Pilot for Human {
        fn fly() {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly() {
            println!("Up!");
        }
    }

    Human::fly();
    <Human as Wizard>::fly();
    <Human as Pilot>::fly();

    use std::fmt;

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point2 {
        x: i32,
        y: i32,
    }

    impl OutlinePrint for Point2 {}

    impl fmt::Display for Point2 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let p = Point2 { x: 1, y: 2 };
    p.outline_print();

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
