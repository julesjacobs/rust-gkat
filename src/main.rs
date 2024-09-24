mod gkat_symb;
use gkat_symb::*;
use hashconsing::HConsign;
use std::{collections::hash_map::DefaultHasher, hash::Hasher};

fn main() {
    let mut factory: HConsign<BExp_> = HConsign::empty();
    let s1 = String::from("value");
    let s2 = String::from("value");
    let x = pbool(&mut factory, s1);
    let x = not(&mut factory, x);
    let y = pbool(&mut factory, s2);
    let y = not(&mut factory, y);
    println!("{:?}", x == y)
}
