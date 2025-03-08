use std::marker::PhantomData;
use std::collections::{HashMap, HashSet};

use super::*;
use disjoint_sets::UnionFindNode;

pub struct Solver<B> {
    // automaton states
    state_stamp: u64,
    // search states
    dead_states: HashSet<u64>,
    explored: HashSet<u64>,
    uf_table: HashMap<u64, UnionFindNode<()>>,
    // phantom
    phantom: PhantomData<B>,
}

impl<B: BExp> Solver<B> {
    pub fn new() -> Self {
        Self {
            // automaton states
            state_stamp: 0,
            // search init
            dead_states: HashSet::default(),
            explored: HashSet::default(),
            uf_table: HashMap::default(),
            // phantom
            phantom: PhantomData,
        }
    }

    pub fn new_state(&mut self) -> u64 {
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

    pub fn reject<G: Gkat<B>>(&mut self, gkat: &mut G, st: u64, m: &Automaton<B>) -> B {
        let eps = m.eps_hat.get(&st).unwrap();
        let dexp = m.delta_hat.get(&st).unwrap();
        dexp.iter().fold(gkat.mk_not(eps), |acc, (b, _, _)| {
            let nb = gkat.mk_not(b);
            gkat.mk_and(&nb, &acc)
        })
    }

    #[inline]
    pub fn known_dead(&self, st: &u64) -> bool {
        self.dead_states.contains(st)
    }

    pub fn is_dead<G: Gkat<B>>(&mut self, gkat: &mut G, st: u64, m: &Automaton<B>) -> bool {
        let mut stack = Vec::new();
        stack.push(st);
        self.explored.clear();
        while let Some(st) = stack.pop() {
            if self.known_dead(&st) || self.explored.contains(&st) {
                continue;
            }
            self.explored.insert(st);
            let eps = m.eps_hat.get(&st).unwrap();
            if gkat.is_false(&eps) {
                for (_, st, _) in m.delta_hat.get(&st).unwrap() {
                    stack.push(*st);
                }
            } else {
                return false;
            }
        }
        self.dead_states.extend(self.explored.iter());
        return true;
    }
}
