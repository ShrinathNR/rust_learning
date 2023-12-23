use reflective::Reflective;

struct Foo {
    x:i64,
    y:String,
    z:bool,
}

impl Reflective for Foo {
    fn name(&self)-> &'static str {
        "Foo"
    }
}

fn main(){
    let foo = Foo {
        x: 5,
        y: "hello".to_string(),
        z: true
    };
    println!("the name of the struct is {}", foo.name())
}