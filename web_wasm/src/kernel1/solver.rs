use super::*;
use disjoint_sets::UnionFindNode;
use std::collections::{HashMap, HashSet};

pub type Deriv<B> = Vec<(B, Exp<B>, u64)>;

pub struct Solver<B> {
    // search states
    dead_states: HashSet<Exp<B>>,
    explored: HashSet<Exp<B>>,
    uf_table: HashMap<Exp<B>, UnionFindNode<()>>,
    // caching
    eps_cache: HashMap<Exp<B>, B>,
    drv_cache: HashMap<Exp<B>, Deriv<B>>,
}

impl<B: BExp> Solver<B> {
    pub fn new() -> Self {
        Solver {
            // search init
            dead_states: HashSet::default(),
            explored: HashSet::default(),
            uf_table: HashMap::default(),
            // caching
            eps_cache: HashMap::default(),
            drv_cache: HashMap::default(),
        }
    }

    pub fn get_uf(&mut self, exp: &Exp<B>) -> UnionFindNode<()> {
        match self.uf_table.get(exp) {
            Some(node) => node.clone(),
            None => {
                let node = UnionFindNode::new(());
                self.uf_table.insert(exp.clone(), node.clone());
                node
            }
        }
    }

    #[inline]
    pub fn get_eps(&mut self, exp: &Exp<B>) -> Option<&B> {
        self.eps_cache.get(exp)
    }

    #[inline]
    pub fn set_eps(&mut self, exp: Exp<B>, eps: B) {
        self.eps_cache.insert(exp, eps);
    }

    #[inline]
    pub fn get_drv(&mut self, exp: &Exp<B>) -> Option<&Deriv<B>> {
        self.drv_cache.get(exp)
    }

    #[inline]
    pub fn set_drv(&mut self, exp: Exp<B>, deriv: Deriv<B>) {
        self.drv_cache.insert(exp, deriv);
    }

    pub fn reject<G: Gkat<B>>(&mut self, gkat: &mut G, eps: &B, dexp: &Deriv<B>) -> B {
        dexp.iter().fold(gkat.mk_not(eps), |acc, (b, _, _)| {
            let nb = gkat.mk_not(b);
            gkat.mk_and(&nb, &acc)
        })
    }

    #[inline]
    pub fn known_dead(&self, exp: &Exp<B>) -> bool {
        self.dead_states.contains(&exp)
    }

    pub fn is_dead<G: Gkat<B>>(&mut self, gkat: &mut G, exp: &Exp<B>) -> bool {
        let mut stack = Vec::new();
        stack.push(exp.clone());
        self.explored.clear();
        while let Some(exp) = stack.pop() {
            if self.known_dead(&exp) || self.explored.contains(&exp) {
                continue;
            }
            self.explored.insert(exp.clone());
            let eps = self.epsilon(gkat, &exp);
            if gkat.is_false(&eps) {
                for (_, e, _) in self.derivative(gkat, &exp) {
                    stack.push(e);
                }
            } else {
                return false;
            }
        }
        self.dead_states.extend(self.explored.iter().cloned());
        return true;
    }
}
