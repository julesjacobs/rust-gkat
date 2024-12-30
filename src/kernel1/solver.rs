use super::*;
use ahash::{HashMap, HashSet};
use disjoint_sets::UnionFindNode;
use lru::LruCache;
use std::num::NonZero;

pub struct Solver {
    // search states
    pub(super) dead_states: HashSet<Exp>,
    pub(super) explored: HashSet<Exp>,
    pub(super) uf_table: HashMap<Exp, UnionFindNode<()>>,
    // caching
    pub(super) eps_cache: LruCache<Exp, BExp>,
    pub(super) deriv_cache: LruCache<Exp, Vec<(BExp, Exp, u64)>>,
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            // search init
            dead_states: HashSet::default(),
            explored: HashSet::default(),
            uf_table: HashMap::default(),
            // caching
            eps_cache: LruCache::new(NonZero::new(1024).unwrap()),
            deriv_cache: LruCache::new(NonZero::new(1024).unwrap()),
        }
    }

    pub fn get_uf(&mut self, exp: &Exp) -> UnionFindNode<()> {
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
