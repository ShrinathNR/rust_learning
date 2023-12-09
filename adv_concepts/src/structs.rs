//conventional struct
// #[derive(Debug)]
// struct User {
//     name:String,
//     active:bool,
//     email:String,
//     signin_count:u16,
// }

// methods for struct
// impl User {
//     fn printname(&self){
//         println!("hello {}", self.name);
//     }
// }

// associated func - methods which doesnt have self as parameter
// i.e the func doesnt need the instance of the type to work with
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// impl Rectangle {
//     fn square(size: u32) -> Self {
//         Self {
//             width: size,
//             height: size,
//         }
//     }
// }

//tuple structs
//used when the name the struct delivers the meaning to the field
// and no need for field name,this helps keep the structs clean 
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

pub fn structs (){

    //create a struct user1
    // let user1 = User {
    //     active: true,
    //     name: String::from("someusername123"),
    //     email: String::from("someone@example.com"),
    //     signin_count: 1,
    // };

    // user1.printname();

    // //create user2 from user1
    // // if we are using fields which are scalar and have Copy trait
    // // the fields will just be copied
    // //but if the fields like string are copied, user 1 is moved to user 2
    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     ..user1
    // };

    // as user struct doesnt implement display we are using {:#?}
    // to display the whole sturct
    // but to do this we need to implement Debug trait to User struct
    // using derive attrib 

    // println!("user1 is {:#?}", user1);

    // // instead of println! macro we can use dbg! which takes
    // // ownership of the expression, prints and returns the ownership
    // // whereas println! takes in reference
    //  dbg!(&user1) 

    // // associated func are accessed with :: 
    // let square = Rectangle::square(5);


    

}