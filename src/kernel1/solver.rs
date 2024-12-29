use super::*;
use ahash::{HashMap, HashSet};
use disjoint_sets::UnionFindNode;
use lru::LruCache;
use std::{marker::PhantomData, num::NonZero};

pub struct Solver<A, M, Builder>
where
    A: NodeAddress,
    M: Multiplicity,
    Builder: DecisionDiagramFactory<A, M>,
{
    // search states
    pub(super) dead_states: HashSet<Exp<BExp<A, M>>>,
    pub(super) explored: HashSet<Exp<BExp<A, M>>>,
    pub(super) uf_table: HashMap<Exp<BExp<A, M>>, UnionFindNode<()>>,
    // caching
    pub(super) eps_cache: LruCache<Exp<BExp<A, M>>, BExp<A, M>>,
    pub(super) deriv_cache: LruCache<Exp<BExp<A, M>>, Vec<(NodeIndex<A, M>, Exp<BExp<A, M>>, u64)>>,
    // phantom
    phantom: PhantomData<Builder>,
}

impl<A, M, Builder> Solver<A, M, Builder>
where
    A: NodeAddress,
    M: Multiplicity,
    Builder: DecisionDiagramFactory<A, M>,
{
    pub fn new() -> Self {
        Solver {
            // search init
            dead_states: HashSet::default(),
            explored: HashSet::default(),
            uf_table: HashMap::default(),
            // caching
            eps_cache: LruCache::new(NonZero::new(1024).unwrap()),
            deriv_cache: LruCache::new(NonZero::new(1024).unwrap()),
            // builder
            phantom: PhantomData,
        }
    }

    pub fn get_uf(&mut self, exp: &Exp<BExp<A, M>>) -> UnionFindNode<()> {
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
