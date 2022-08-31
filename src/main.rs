#![allow(non_snake_case)]

extern crate num;

//use num::Zero;
use num::Num;

trait Convertable: std::convert::From<f64> + std::convert::From<f32> + std::convert::From<i32> + std::convert::From<i16> {} 
trait Supported: Num + Default + std::ops::AddAssign + std::cmp::PartialOrd + Copy + std::fmt::Display + Convertable {}

// takes 2 functions which take and return any numeric type T (traits defined in Supported and Convertable)
// returns value with type T that solves f(), accurate to the threshold value
fn newtons_method<T: Supported>(f: &dyn Fn(T) -> T, fp: &dyn Fn(T) -> T, _threshold: f64, guess: T) -> T {

    //let mut i: T = T::default();
    let mut i: T = guess;
    let threshold: T = T::try_from(_threshold).unwrap();

    let neg_one: T = T::try_from(-1).unwrap();

    while f(i) > threshold || f(i) < (threshold * neg_one) {
        i = i - (f(i) / fp(i));
    }

    i
}

// above here is like if you had a library
// user defined from here down

fn f1(x: f64) -> f64 {
    // x² - 9.4
    (x * x) - 9.4
}

fn f2(x: f64) -> f64 {
    // 2x
    2.0 * x
}

fn main() {
    let res: f64;

    // have to confirm that the type you use works before using it
    impl Convertable for f64 {}
    impl Supported for f64 {}

    res = newtons_method(&f1, &f2, 0.001, 1.0);

    println!("{}", res);
}
