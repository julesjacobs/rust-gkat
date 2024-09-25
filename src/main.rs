mod gkat_ast {
    pub mod bexp;
    pub mod exp;
}
mod gkat_symb {
    pub mod dead;
    pub mod derivative;
    pub mod epsilon;
    pub mod equiv;
}
mod parsing {
    pub mod ast;
    pub mod parser;
}

use std::env;
use std::fs;

use crate::gkat_ast::bexp::*;
use crate::gkat_ast::exp::*;
use gkat_symb::equiv::equiv;
use hashconsing::HConsign;
use parsing::parser::{parse, test};

fn main() {
    let mut nb: NameBuilder = NameBuilder::new();
    let mut fb: HConsign<BExp_> = HConsign::empty();
    let mut fp: HConsign<Exp_> = HConsign::empty();
    let args: Vec<String> = env::args().collect();
    let file1 = fs::read_to_string("examples/exp00_0.txt").expect("cannot read file");
    let file2 = fs::read_to_string("examples/exp00_1.txt").expect("cannot read file");
    let exp1 = parse(file1).to_hashcons(&mut nb, &mut fb, &mut fp);
    let exp2 = parse(file2).to_hashcons(&mut nb, &mut fb, &mut fp);
    println!("{}", equiv(&mut fb, &mut fp, &exp1, &exp2))
}
