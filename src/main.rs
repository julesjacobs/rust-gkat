mod gkat_ast {
    pub mod bexp;
    pub mod exp;
}
mod gkat_symb {
    pub mod dead;
    pub mod derivative;
    pub mod epsilon;
    pub mod equiv;
    pub mod equiv_iter;
}
mod parsing {
    pub mod ast;
    pub mod parser;
}

use std::env;
use std::fs;

use crate::gkat_ast::bexp::*;
use crate::gkat_ast::exp::*;
use gkat_symb::equiv;
use gkat_symb::equiv_iter;
use hashconsing::HConsign;
use parsing::parser::parse;

fn main() {
    let mut nb: NameBuilder = NameBuilder::new();
    let mut fb: HConsign<BExp_> = HConsign::empty();
    let mut fp: HConsign<Exp_> = HConsign::empty();
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).expect("cannot read file");
    let (exp1, exp2) = parse(file);
    let exp1 = exp1.to_hashcons(&mut nb, &mut fb, &mut fp);
    let exp2 = exp2.to_hashcons(&mut nb, &mut fb, &mut fp);
    println!("{}", equiv_iter::equiv(&mut fb, &mut fp, &exp1, &exp2))
}
