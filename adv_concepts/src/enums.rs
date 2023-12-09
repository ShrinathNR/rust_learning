// this is a good example coz both 4 & 6 are IPs but of diff format
// and we are aware of all the possibility and it can only be one at a time
// enum IpAddrKind {
//     V4,
//     V6,
// }

// //enums inside struct
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

//enums with data
// enum IpAddrKind {
//     // we vavn also have diff data type for each outcome
//     // for example v4 can be (u8, u8, u8, u8)
//     V4(String),
//     V6(String),
// }

//match using enums
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }
pub fn enums() {
    // enums  are a neat way to define a type by enummerating its 
    // possible outcomes


    // let four =  IpAddrKind::V4;
    // fn route (ipKind: IpAddrKind){}



    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // let home = IpAddrKind::V4(String::from("127.0.0.1"));

    // when u have to just compare to one case and do nothing
    // in all other possibility insteas of using match
    // use if let
    // let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }

    // let config_max = Some(3u8);
    // if let Some(max) = config_max {
    //     println!("The maximum is configured to be {}", max);
    // }




}

// match for Coin enum
// fn value_in_coin (coin : Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }