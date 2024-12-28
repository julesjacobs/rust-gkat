use ahash::{HashMap, HashSet};
use disjoint_sets::UnionFindNode;
use std::marker::PhantomData;

pub struct Solver<BExp, Builder> {
    // automaton states
    pub(super) state_stamp: u64,
    // search states
    pub(super) dead_states: HashSet<u64>,
    pub(super) explored: HashSet<u64>,
    pub(super) uf_table: HashMap<u64, UnionFindNode<()>>,
    // phantom
    phantom0: PhantomData<BExp>,
    phantom1: PhantomData<Builder>,
}

impl<BExp, Builder> Solver<BExp, Builder> {
    pub fn new() -> Self {
        Solver {
            // automaton states
            state_stamp: 0,
            // search init
            dead_states: HashSet::default(),
            explored: HashSet::default(),
            uf_table: HashMap::default(),
            // phantom
            phantom0: PhantomData,
            phantom1: PhantomData,
        }
    }

    pub fn mk_state(&mut self) -> u64 {
        let st = self.state_stamp;
        self.state_stamp = st + 1;
        st
    }

    pub fn get_uf(&mut self, st: u64) -> UnionFindNode<()> {
        match self.uf_table.get(&st) {
            Some(node) => node.clone(),
            None => {
                let node = UnionFindNode::new(());
                self.uf_table.insert(st, node.clone());
                node
            }
        }
    }
}
