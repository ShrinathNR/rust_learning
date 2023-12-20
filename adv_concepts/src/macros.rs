pub fn macros () {
}

// macros take in a token stream and spits out a token stream
// declarative macros are similar to match
// expcept we match for rust code and return a rust code

// macro_rules! four {
//     () => { 3+1 };
// }

//simple vec macros
#[macro_export]
macro_rules! vec {
    ($($x:expr),*) => {
        let temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    };
}

