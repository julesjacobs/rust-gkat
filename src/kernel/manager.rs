use super::*;
use ahash::AHasher;
use disjoint_sets::UnionFindNode;
use hashconsing::HConsign;
use rsdd::builder::BottomUpBuilder;
use rsdd::repr::BddPtr;
use std::collections::{HashMap, HashSet};
use std::hash::BuildHasherDefault;

#[non_exhaustive]
pub struct GkatManager<'a, Builder: BottomUpBuilder<'a, BddPtr<'a>>> {
    // bexp states
    pub name_stamp: u64,
    pub name_map: HashMap<String, u64, BuildHasherDefault<AHasher>>,
    pub bexp_hcons: HConsign<BExp_>,
    pub bexp_builder: &'a Builder,
    pub bexp_cache: HashMap<BExp, BddPtr<'a>, BuildHasherDefault<AHasher>>,
    // exp states
    pub action_stamp: u64,
    pub exp_hcons: HConsign<Exp_>,
    // search states
    pub dead_states: HashSet<Exp, BuildHasherDefault<AHasher>>,
    pub explored: HashSet<Exp, BuildHasherDefault<AHasher>>,
    pub uf_table: HashMap<Exp, UnionFindNode<()>, BuildHasherDefault<AHasher>>,
}

impl<'a, Builder: BottomUpBuilder<'a, BddPtr<'a>>> GkatManager<'a, Builder> {
    pub fn new(builder: &'a Builder) -> Self {
        GkatManager {
            // bexp init
            name_stamp: 0,
            name_map: HashMap::default(),
            bexp_hcons: HConsign::empty(),
            bexp_builder: builder,
            bexp_cache: HashMap::default(),
            // exp init
            action_stamp: 0,
            exp_hcons: HConsign::empty(),
            // search init
            dead_states: HashSet::default(),
            explored: HashSet::default(),
            uf_table: HashMap::default(),
        }
    }
}
