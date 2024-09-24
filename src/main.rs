mod bexp;
mod exp;
use bexp::*;
use exp::{derivative, mk_test, Exp_};
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
    let mut fb: HConsign<BExp_> = HConsign::empty();
    let mut fp: HConsign<Exp_> = HConsign::empty();
    let s1 = String::from("value");
    let s2 = String::from("value");
    let x = mk_pbool(&mut fb, s1);
    let x = mk_not(&mut fb, x);
    let y = mk_pbool(&mut fb, s2);
    let y = mk_not(&mut fb, y);
    let e = mk_test(&mut fp, y);
    let dexp = derivative(&mut fb, &mut fp, &e);
    let builder = RobddBuilder::<AllIteTable<BddPtr>>::new_with_linear_order(1024);
    println!("{:?}", is_false(&builder, &x))
}
