mod structs;
mod enums;
mod common_collection;
mod generics;
mod traits;

use structs::structs;
use enums::enums;
use common_collection::common_collection;
use generics::generics;
use traits::{Summary, Tweet};
fn main() {
    // structs();
    // enums();
    // common_collection();
    // generics();
    //using Summary trait
    // to use methods of traits we need to import both type and trait
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
