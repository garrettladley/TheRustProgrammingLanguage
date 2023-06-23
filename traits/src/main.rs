use std::fmt::{Debug, Display};

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!(
            "(Default impl...Read more. from {}..)",
            self.summarize_author()
        )
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary, U: Summary>(item1: &T, item2: &U) {
    println!(
        "First Breaking news! {}\nSecond Breaking news! {}",
        item1.summarize(),
        item2.summarize()
    );
}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("@tylerschaefer4"),
        content: String::from("oh monair"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest pair member is x = {}", self.x);
        } else {
            println!("The largest pair member is y = {}", self.y);
        }
    }
}

fn main() {
    let article = NewsArticle {
        author: String::from("Garrett Ladley"),
        headline: String::from("Muneer Wins His Bet Against Sean!"),
        content: String::from("Muneer gets to 850 trophies on El Primo!"),
    };

    println!("Article summary: {}", article.summarize());

    notify(&article);

    let tweet = returns_summarizable();

    println!("Tweet summary: {}", tweet.summarize());

    notify(&tweet);

    notify2(&article, &tweet);

    let pair = Pair::new(5, 10);
    println!("pair.x = {}", pair.x);
    println!("pair.y = {}", pair.y);
    pair.cmp_display();
}
