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

use crate::gkat_ast::bexp::*;
use crate::gkat_ast::exp::*;
use gkat_symb::equiv::equiv;
use hashconsing::HConsign;

fn main() {
    let mut fb: HConsign<BExp_> = HConsign::empty();
    let mut fp: HConsign<Exp_> = HConsign::empty();
    let s1 = String::from("x");
    let s2 = String::from("y");
    let x = mk_pbool(&mut fb, s1);
    let x = mk_not(&mut fb, x);
    let e1 = mk_test(&mut fp, x);

    let y = mk_pbool(&mut fb, s2);
    let y = mk_not(&mut fb, y);
    let e2 = mk_test(&mut fp, y);

    let resut = equiv(&mut fb, &mut fp, &e1, &e2);
    println!("{:?}", resut);
}
