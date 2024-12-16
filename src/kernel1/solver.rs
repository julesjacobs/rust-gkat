use crate::gkat::*;
use ahash::{HashMap, HashSet};
use disjoint_sets::UnionFindNode;
use lru::LruCache;
use rsdd::builder::BottomUpBuilder;
use rsdd::repr::DDNNFPtr;
use std::num::NonZero;

pub struct Solver<'a, Ptr: DDNNFPtr<'a>, Builder: BottomUpBuilder<'a, Ptr>> {
    // gkat
    pub(super) gkat: GkatManager<'a, Ptr, Builder>,
    // search states
    pub(super) dead_states: HashSet<Exp<Ptr>>,
    pub(super) explored: HashSet<Exp<Ptr>>,
    pub(super) uf_table: HashMap<Exp<Ptr>, UnionFindNode<()>>,
    // caching
    pub(super) deriv_cache: LruCache<Exp<Ptr>, Vec<(Ptr, Exp<Ptr>, Action)>>,
}

impl<'a, Ptr: DDNNFPtr<'a>, Builder: BottomUpBuilder<'a, Ptr>> Solver<'a, Ptr, Builder> {
    pub fn new(gkat: GkatManager<'a, Ptr, Builder>) -> Self {
        Solver {
            // gkat
            gkat: gkat,
            // search init
            dead_states: HashSet::default(),
            explored: HashSet::default(),
            uf_table: HashMap::default(),
            // caching
            deriv_cache: LruCache::new(NonZero::new(1024).unwrap()),
        }
    }

    pub fn get_uf(&mut self, exp: &Exp<Ptr>) -> UnionFindNode<()> {
        match self.uf_table.get(exp) {
            Some(node) => node.clone(),
            None => {
                let node = UnionFindNode::new(());
                self.uf_table.insert(exp.clone(), node.clone());
                node
            }
        }
    }
}
