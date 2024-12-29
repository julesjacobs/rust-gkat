mod kernel1;
mod kernel2;
mod parsing;
mod syntax;

use clap::{Parser, ValueEnum};
use parsing::*;
use std::fs;
use syntax::*;
use xdd::*;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Kernel {
    K1,
    K2,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_enum, default_value_t = Kernel::K1)]
    kernel: Kernel,
    input: String,
}

fn main() {
    let args = Args::parse();
    let file = fs::read_to_string(args.input).expect("cannot read file");
    let (exp1, exp2, b) = parse(file);

    let builder = BDDFactory::<u32, NoMultiplicity>::new(1024);
    let mut gkat = Gkat::new(builder);
    let exp1 = gkat.from_exp(exp1);
    let exp2 = gkat.from_exp(exp2);

    let result = match args.kernel {
        Kernel::K1 => {
            let mut solver = kernel1::Solver::new();
            solver.equiv_iter(&mut gkat, &exp1, &exp2)
        }
        Kernel::K2 => {
            let mut solver = kernel2::Solver::new();
            let (i, m) = solver.mk_automaton(&mut gkat, &exp1);
            let (j, n) = solver.mk_automaton(&mut gkat, &exp2);
            solver.equiv_iter(&mut gkat, i, j, &m, &n)
        }
    };

    println!("equiv_expected = {}", b);
    println!("equiv_result   = {}", result);
    assert!(b == result);
}
