mod bexp;
mod prog;
use bexp::*;
use hashconsing::HConsign;
use rsdd::{
    builder::{
        bdd::{BddBuilder, RobddBuilder},
        cache::AllIteTable,
    },
    repr::BddPtr,
};
use std::{collections::hash_map::DefaultHasher, hash::Hasher};

fn main() {
    let mut factory: HConsign<BExp_> = HConsign::empty();
    let s1 = String::from("value");
    let s2 = String::from("value");
    let x = mk_pbool(&mut factory, s1);
    let x = mk_not(&mut factory, x);
    let y = mk_pbool(&mut factory, s2);
    let y = mk_not(&mut factory, y);
    let builder = RobddBuilder::<AllIteTable<BddPtr>>::new_with_linear_order(1024);
    println!("{:?}", is_false(&builder, &x))
}
