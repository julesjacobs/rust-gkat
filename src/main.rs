mod kernel1;
mod kernel2;
mod parsing;
mod syntax;

use clap::{Parser, ValueEnum};
use kernel1::*;
use parsing::*;
use rsdd::{
    builder::{self, cache::AllIteTable},
    repr::{BddPtr, VTree, VarLabel},
};
use std::fs;
use syntax::*;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Mode {
    Bdd,
    Sdd,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_enum, default_value_t = Mode::Bdd)]
    mode: Mode,
    input: String,
}

fn main() {
    let args = Args::parse();
    let file = fs::read_to_string(args.input).expect("cannot read file");
    let (exp1, exp2, b) = parse(file);
    let result = match args.mode {
        Mode::Bdd => {
            let builder =
                builder::bdd::RobddBuilder::<AllIteTable<BddPtr>>::new_with_linear_order(1024);
            let mut gkat = Gkat::new(&builder);
            let exp1 = gkat.from_exp(exp1);
            let exp2 = gkat.from_exp(exp2);
            let mut solver = Solver::new();
            solver.equiv_iter(&mut gkat, &exp1, &exp2)
        }
        Mode::Sdd => {
            let order: Vec<VarLabel> = (0..1024).map(|x| VarLabel::new(x as u64)).collect();
            let vtree = VTree::right_linear(&order);
            let builder = builder::sdd::CompressionSddBuilder::new(vtree);
            let mut gkat = Gkat::new(&builder);
            let exp1 = gkat.from_exp(exp1);
            let exp2 = gkat.from_exp(exp2);
            let mut solver = Solver::new();
            solver.equiv_iter(&mut gkat, &exp1, &exp2)
        }
    };
    println!("equiv_expected = {}", b);
    println!("equiv_result   = {}", result);
    assert!(b == result);
}
