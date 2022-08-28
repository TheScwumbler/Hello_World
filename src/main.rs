#![allow(non_snake_case)] 

fn test(s: &mut String) {

    *s = String::from("Hello World!");

}

fn main() {
    // create String named s
    // have to initialize to allow referencing
    let mut s: String = String::new();

    test(&mut s); // set s to "Hello World!"

    println!("{0}", s); // print s
}
