mod gkat_symb {
    pub mod bexp;
    pub mod dead_exp;
    pub mod exp;
}

use crate::gkat_symb::bexp::*;
use crate::gkat_symb::exp::*;
use gkat_symb::dead_exp::visit;
use hashconsing::HConsign;
use rsdd::{
    builder::{
        bdd::{BddBuilder, RobddBuilder},
        cache::AllIteTable,
    },
    repr::BddPtr,
};
use std::collections::HashSet;
use std::{collections::hash_map::DefaultHasher, hash::Hasher};

fn main() {
    let mut fb: HConsign<BExp_> = HConsign::empty();
    let mut fp: HConsign<Exp_> = HConsign::empty();
    let mut dead_states: HashSet<Exp> = HashSet::new();
    let mut explored: HashSet<Exp> = HashSet::new();
    let s1 = String::from("value");
    let s2 = String::from("value");
    let x = mk_pbool(&mut fb, s1);
    let x = mk_not(&mut fb, x);
    let y = mk_pbool(&mut fb, s2);
    let y = mk_not(&mut fb, y);
    let e = mk_test(&mut fp, y);
    let dexp = derivative(&mut fb, &mut fp, &e);
    let bdd = RobddBuilder::<AllIteTable<BddPtr>>::new_with_linear_order(1024);
    let resut = visit(&mut fb, &mut fp, &bdd, &dead_states, &mut explored, &e);
    println!("{:?}", is_false(&bdd, &x));
    println!("{:?}", resut);
}
