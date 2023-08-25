fn main() {
    //hello world
    // println!("Hello, world!");

    // variables and mutability
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

    //data types
    // let _guess: u32 ="42".parse().expect("Not a number!");

    //types - scalar & compound
    //scalar - integer, floating-point, boolean, character
    //u - unsigned; n-bits =[0,2^n-1) 
    //i - signed; n-bits = [-2^(n-1),2^(n-1)-1]

    //if there is overflow in types, while building the complier panics
    //but if build with --release flag, it complies but now it holds the minimum of the values the type can hold
    // x: u8 = 256 -> x= 1

    //char in rust-4bits and uses unicode scalar(represent values more than ASCII)

    //compound type - tuple,array
    //tuple - fixed length, variety of types
    // let tup: (i32, f64) = (20,2.5);
    // access elements by destucturing or index
    // let (x,y,z) = tup;
    // let tup: (i32, f64) = (20,2.5);
    // let twenty = tup.0;

    //()-unit-tuple with no values; expressions return () when they dont return any other values

    //arrays - elements must have same type, fixed length
    // let a = [1,2,3,4];
    // let first = a[0];
}
