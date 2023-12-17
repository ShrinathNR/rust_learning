pub fn macros () {
}

// macros take in a token stream and spits out a token stream
// declarative macros are similar to match
// expcept we match for rust code and return a rust code

macro_rules! four {
    () => { 3+1 };
}

