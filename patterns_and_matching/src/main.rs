fn main() {
    #[derive(Debug)]
    enum Language {
        English,
        Spanish,
        Russian,
        Japanese,
        Chinese,
        French,
    }

    let language = Language::English;

    match language {
        Language::English => println!("Hello World!"),
        Language::Spanish => println!("Hola Mundo!"),
        Language::Russian => println!("Привет, мир!"),
        Language::Japanese => println!("こんにちは世界!"),
        Language::Chinese => println!("你好世界!"),
        lang => println!("Unsupported language! {:?}", lang),
    }

    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "14".parse();

    if let Some(status) = authorization_status {
        println!("Authorization status: {}", status);
    } else if is_admin {
        println!("Authorization status: admin");
    } else if let Ok(group_id) = group_id {
        if group_id > 10 {
            println!("Authorization status: privileged");
        } else {
            println!("Authorization status: basic");
        }
    } else {
        println!("Authorization status: guest");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let x = 5;

    let (x, y, z) = (1, 2, 3);

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);

    // Irrefutable
    let x = 5;

    // Refutable
    let x: Option<&str> = None;
    if let Some(x) = x {
        println!("{}", x);
    }

    // The folllowing can only accept irrefutable patterns:
    // function parameters
    // let statements
    // for loops

    // the below code will not compile because there is a refutable pattern in the if let statement
    // let x: Option<&str> = None;
    // let Some(x) = x;

    // the below code will produce a compiler warning because the pattern will always match
    // if let x = 5 {
    //     println!("{}", x);
    // }
}
