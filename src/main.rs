mod kernel1;
mod kernel2;
mod parsing;
mod syntax;

use clap::{Parser, ValueEnum};
use mimalloc::MiMalloc;
use parsing::*;
use std::fs;
use syntax::*;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Kernel {
    K1,
    K2,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Solver {
    BDD,
    SAT,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_enum, default_value_t = Kernel::K1)]
    kernel: Kernel,
    #[arg(short, long, value_enum, default_value_t = Solver::BDD)]
    solver: Solver,
    input: String,
}

fn main() {
    let args = Args::parse();
    let file = fs::read_to_string(args.input).expect("cannot read file");
    let (exp1, exp2, b) = parse(file);
    let result = match args.kernel {
        Kernel::K1 => match args.solver {
            Solver::BDD => {
                let mut gkat = BDDGkat::new();
                let mut solver = kernel1::Solver::new();
                let exp1 = gkat.from_exp(exp1);
                let exp2 = gkat.from_exp(exp2);
                solver.equiv_iter(&mut gkat, &exp1, &exp2)
            }
            Solver::SAT => {
                let mut gkat = SATGkat::new();
                let mut solver = kernel1::Solver::new();
                let exp1 = gkat.from_exp(exp1);
                let exp2 = gkat.from_exp(exp2);
                solver.equiv_iter(&mut gkat, &exp1, &exp2)
            }
        },
        Kernel::K2 => match args.solver {
            Solver::BDD => {
                let mut gkat = BDDGkat::new();
                let mut solver = kernel2::Solver::new();
                let exp1 = gkat.from_exp(exp1);
                let exp2 = gkat.from_exp(exp2);
                let (i, m) = solver.mk_automaton(&mut gkat, &exp1);
                let (j, n) = solver.mk_automaton(&mut gkat, &exp2);
                solver.equiv_iter(&mut gkat, i, j, &m, &n)
            }
            Solver::SAT => {
                let mut gkat = SATGkat::new();
                let mut solver = kernel2::Solver::new();
                let exp1 = gkat.from_exp(exp1);
                let exp2 = gkat.from_exp(exp2);
                let (i, m) = solver.mk_automaton(&mut gkat, &exp1);
                let (j, n) = solver.mk_automaton(&mut gkat, &exp2);
                solver.equiv_iter(&mut gkat, i, j, &m, &n)
            }
        },
    };

    println!("equiv_expected = {}", b);
    println!("equiv_result   = {}", result);
    assert!(b == result);
}
