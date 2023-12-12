// use std::collections::HashMap;
pub fn common_collection(){
    //vectors
    // create vectors
    // let v: Vec<i32> = Vec::new();
    //using vec! macro
    // let mut v1 = vec![1,2,3];
    // v1.push(4);

    //bellow block of code will produce a err
    // coz we cant have mut and immutable reference in the same scope
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {first}");

    //String 
    // let s = String::from("intitial contents");
    // string literal
    // let data = "initial contents";
    //conversion
    // let s = data.to_string();
    
    // string is essentially a wrapper around vectors with each bytes as char
    // let mut s = String::from("foo");
    // let s2 = "bar"
    // s.push_str(s2);
    // concatenation
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // hashmap
    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);
    
    //loops in hasmap
    // for (key, value) in &scores {
    //     println!("{key}: {value}");
    // }

    // if u want to check if a key is present in hashmap 
    // if not insert the key with the given value
    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);

    // println!("{:?}", scores);

    // challenge 1

    

}