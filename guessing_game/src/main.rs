use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("guess a number: ");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("secret number {secret_number}");
    println!("enter your guess number from 1 to 100");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    let guess : u32 = guess.trim().parse().expect("type a number");

    println!("you guessed: {guess}");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => println!("you win"),
    }


}
