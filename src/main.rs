mod kernel;
mod parsing;

use kernel::*;
use parsing::*;
use rsdd::{
    builder::{self, cache::AllIteTable, sdd},
    repr::{BddPtr, VTree, VTreeManager, VarLabel},
    util::btree::BTree,
};
use std::{env, fs};

fn main() {
    let order: Vec<VarLabel> = (0..1024).map(|x| VarLabel::new(x as u64)).collect();
    let vtree = VTree::even_split(&order, 4);
    let builder = builder::sdd::CompressionSddBuilder::new(vtree);
    // let builder =
    //     rsdd::builder::bdd::RobddBuilder::<AllIteTable<BddPtr>>::new_with_linear_order(1024);
    let mut gkat = GkatManager::new(&builder);
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).expect("cannot read file");
    let (exp1, exp2, b) = parse(file);
    let exp1 = gkat.from_exp(exp1);
    let exp2 = gkat.from_exp(exp2);
    let result = gkat.equiv_iter(&exp1, &exp2);
    println!("equiv_expected = {}", b);
    println!("equiv_result   = {}", result);
    assert!(b == result);
}
