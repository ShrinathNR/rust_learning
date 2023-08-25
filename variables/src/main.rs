fn main() {
    // mutability
    // let mut x = 5;
    // println!("the value of x is {x}");
    // x=6;
    // println!("the value of x is {x}");

    //shadowing
    // let x = 5;
    // let mut x = 5;
    //creates a new variable x and assigns x+1
    //so need of mut(it will throw warning, that it need not be mutable)
    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("value of x in the inner scope is: {x}");
    // }
    // println!("value of x is: {x}");

    // when shadowing var using let we can change types
    // if used mut it will throw err saying you can't mutate type
    // let spaces = "    ";
    // println!("str spaces : {spaces}");
    // let spaces = spaces.len();
    // println!("num spaces : {spaces}");
}
