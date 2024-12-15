use std::num::NonZero;

use super::*;
use ahash::{HashMap, HashSet};
use disjoint_sets::UnionFindNode;
use hashconsing::HConsign;
use lru::LruCache;
use rsdd::builder::BottomUpBuilder;
use rsdd::repr::{DDNNFPtr, VarLabel};

pub struct GkatManager<'a, Ptr: DDNNFPtr<'a>, Builder: BottomUpBuilder<'a, Ptr>> {
    // bexp states
    pub(super) name_stamp: u64,
    pub(super) name_map: HashMap<String, VarLabel>,
    pub(super) bexp_builder: &'a Builder,
    // exp states
    pub(super) exp_hcons: HConsign<Exp_<Ptr>>,
    // search states
    pub(super) dead_states: HashSet<Exp<Ptr>>,
    pub(super) explored: HashSet<Exp<Ptr>>,
    pub(super) uf_table: HashMap<Exp<Ptr>, UnionFindNode<()>>,
    // caching
    pub(super) deriv_cache: LruCache<Exp<Ptr>, Vec<(Ptr, (Exp<Ptr>, Action))>>,
}

impl<'a, Ptr: DDNNFPtr<'a>, Builder: BottomUpBuilder<'a, Ptr>> GkatManager<'a, Ptr, Builder> {
    pub fn new(builder: &'a Builder) -> Self {
        GkatManager {
            // bexp init
            name_stamp: 0,
            name_map: HashMap::default(),
            bexp_builder: builder,
            // exp init
            exp_hcons: HConsign::empty(),
            // search init
            dead_states: HashSet::default(),
            explored: HashSet::default(),
            uf_table: HashMap::default(),
            // caching
            deriv_cache: LruCache::new(NonZero::new(1024).unwrap()),
        }
    }
}
