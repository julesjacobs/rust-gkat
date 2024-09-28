mod gkat_ast;
mod gkat_symb;
mod parsing;

use gkat_ast::*;
use gkat_symb::equiv;
use hashconsing::HConsign;
use parsing::parse;
use std::{env, fs};

fn main() {
    test_main();
    // let mut nb: NameBuilder = NameBuilder::new();
    // let mut fb: HConsign<BExp_> = HConsign::empty();
    // let mut fp: HConsign<Exp_> = HConsign::empty();
    // let args: Vec<String> = env::args().collect();
    // let file = fs::read_to_string(&args[1]).expect("cannot read file");
    // let (exp1, exp2, b) = parse(file);
    // let exp1 = exp1.to_hashcons(&mut nb, &mut fb, &mut fp);
    // let exp2 = exp2.to_hashcons(&mut nb, &mut fb, &mut fp);
    // let result = equiv(&mut fb, &mut fp, &exp1, &exp2);
    // println!("equiv_result = {}", result);
    // assert!(b == result);
}

// #[test]
fn test_main() {
    let paths = fs::read_dir("./dataset/1000").unwrap();
    for p in paths {
        let p = p.unwrap().path();
        let mut nb: NameBuilder = NameBuilder::new();
        let mut fb: HConsign<BExp_> = HConsign::empty();
        let mut fp: HConsign<Exp_> = HConsign::empty();
        println!("{}", &p.to_str().unwrap());
        let file = fs::read_to_string(p).expect("cannot read file");
        let (exp1, exp2, b) = parse(file);
        let exp1 = exp1.to_hashcons(&mut nb, &mut fb, &mut fp);
        let exp2 = exp2.to_hashcons(&mut nb, &mut fb, &mut fp);
        let result = equiv(&mut fb, &mut fp, &exp1, &exp2);
        println!("equiv_expected = {}", b);
        println!("equiv_result   = {}", result);
        assert!(b == result);
    }
}
