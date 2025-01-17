use super::*;
use ahash::{HashMap, HashSet};
use disjoint_sets::UnionFindNode;
use lru::LruCache;
use std::num::NonZero;

pub type Deriv = Vec<(BExp, Exp, u64)>;

pub struct Solver {
    // search states
    dead_states: HashSet<Exp>,
    explored: HashSet<Exp>,
    uf_table: HashMap<Exp, UnionFindNode<()>>,
    // caching
    eps_cache: LruCache<Exp, BExp>,
    deriv_cache: LruCache<Exp, Deriv>,
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

    #[inline]
    pub fn get_eps(&mut self, exp: &Exp) -> Option<&BExp> {
        self.eps_cache.get(exp)
    }

    #[inline]
    pub fn set_eps(&mut self, exp: Exp, eps: BExp) {
        self.eps_cache.push(exp, eps);
    }

    #[inline]
    pub fn get_deriv(&mut self, exp: &Exp) -> Option<&Deriv> {
        self.deriv_cache.get(exp)
    }

    #[inline]
    pub fn set_deriv(&mut self, exp: Exp, deriv: Deriv) {
        self.deriv_cache.push(exp, deriv);
    }

    pub fn reject(&mut self, gkat: &mut Gkat, exp: &Exp) -> BExp {
        let dexp = self.derivative(gkat, exp);
        let eps = self.epsilon(gkat, exp);
        dexp.iter().fold(eps.not(), |acc, (b, _, _)| {
            let nb = b.not();
            nb.and(&acc)
        })
    }

    #[inline]
    pub fn known_dead(&self, exp: &Exp) -> bool {
        self.dead_states.contains(&exp)
    }

    pub fn is_dead(&mut self, gkat: &mut Gkat, exp: &Exp) -> bool {
        let mut stack = Vec::new();
        stack.push(exp.clone());
        self.explored.clear();
        while let Some(exp) = stack.pop() {
            if self.known_dead(&exp) || self.explored.contains(&exp) {
                continue;
            }
            self.explored.insert(exp.clone());
            let eps = self.epsilon(gkat, &exp);
            if eps.is_false() {
                for (b, e, _) in self.derivative(gkat, &exp) {
                    // check is not strictly needed due to eager-pruning
                    if b.is_false() {
                        continue;
                    }
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
