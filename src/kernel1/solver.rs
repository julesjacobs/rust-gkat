use crate::syntax::*;
use ahash::{HashMap, HashSet};
use disjoint_sets::UnionFindNode;
use lru::LruCache;
use std::{hash::Hash, marker::PhantomData, num::NonZero};

pub struct Solver<Ptr, Builder> {
    // search states
    pub(super) dead_states: HashSet<Exp<Ptr>>,
    pub(super) explored: HashSet<Exp<Ptr>>,
    pub(super) uf_table: HashMap<Exp<Ptr>, UnionFindNode<()>>,
    // caching
    pub(super) deriv_cache: LruCache<Exp<Ptr>, Vec<(Ptr, Exp<Ptr>, Action)>>,
    // builder
    builder: PhantomData<Builder>,
}

impl<Ptr: Hash, Builder> Solver<Ptr, Builder> {
    pub fn new() -> Self {
        Solver {
            // search init
            dead_states: HashSet::default(),
            explored: HashSet::default(),
            uf_table: HashMap::default(),
            // caching
            deriv_cache: LruCache::new(NonZero::new(1024).unwrap()),
            // builder
            builder: PhantomData,
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
