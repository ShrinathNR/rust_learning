// traits in other lang called as interface
// describes what behaviour a type can have. in trait we can have all
// the method signature describing how the interfacing that type would look like
// in impl we can define the body of each type which implements this trait
pub trait Summary {
    fn summarize(&self) -> String;
}
// above we have Summary trait which can be implemented by NewsArticle type 
// and Tweet type and they have differnt body accordingly
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// fn with traits as parameters 
// this lets any type that uses the Summary trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// this is the actual syntax above is the syntax sugar
// its called the trait bound syntax
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }


pub fn traits () {

}