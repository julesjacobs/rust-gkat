mod kernel1;
mod kernel2;
mod parsing;
mod syntax;

use clap::{Parser, ValueEnum};
use parsing::*;
use std::fs;
use syntax::*;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Kernel {
    k1, // Symbolic derivative method
    k2, // Symbolic Thompson's construction
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_enum, default_value_t = Kernel::k1)]
    kernel: Kernel,
    input: String,
}

fn main() {
    let args = Args::parse();
    let file = fs::read_to_string(args.input).expect("cannot read file");
    let (exp1, exp2, b) = parse(file);

    let result = match args.kernel {
        Kernel::k1 => {
            let mut gkat = PureBDDGkat::new();
            let mut solver = kernel1::Solver::new();
            let exp1 = gkat.from_exp(exp1);
            let exp2 = gkat.from_exp(exp2);
            solver.equiv_iter(&mut gkat, &exp1, &exp2)
        },
        Kernel::k2 => {
            let mut gkat = PureBDDGkat::new();
            let mut solver = kernel2::Solver::new();
            let exp1 = gkat.from_exp(exp1);
            let exp2 = gkat.from_exp(exp2);
            let (i, m) = solver.mk_automaton(&mut gkat, &exp1);
            let (j, n) = solver.mk_automaton(&mut gkat, &exp2);
            solver.equiv_iter(&mut gkat, i, j, &m, &n)
        },
    };

    println!("equiv_expected = {}", b);
    println!("equiv_result   = {}", result);
    assert!(b == result);
}