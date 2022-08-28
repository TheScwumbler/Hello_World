#![allow(non_snake_case)] 

macro_rules! TestMacro {
    ( $( $x:expr ),* ) => {
        println!("prints before other arguments\n");

        $(
            println!($x);
        )*

        println!("prints after other arguments\n");
    };
}

fn main() {
    
    TestMacro!("testing", "testing again", "");

}