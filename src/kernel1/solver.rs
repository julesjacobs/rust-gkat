use super::*;
use ahash::{HashMap, HashSet};
use disjoint_sets::UnionFindNode;

pub type Deriv = Vec<(BExp, Exp, u64)>;

pub struct Solver {
    // search states
    dead_states: HashSet<Exp>,
    explored: HashSet<Exp>,
    uf_table: HashMap<Exp, UnionFindNode<()>>,
    // caching
    eps_cache: HashMap<Exp, BExp>,
    drv_cache: HashMap<Exp, Deriv>,
}

impl Solver {
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
        self.eps_cache.insert(exp, eps);
    }

    #[inline]
    pub fn get_drv(&mut self, exp: &Exp) -> Option<&Deriv> {
        self.drv_cache.get(exp)
    }

    #[inline]
    pub fn set_drv(&mut self, exp: Exp, deriv: Deriv) {
        self.drv_cache.insert(exp, deriv);
    }

    pub fn reject(&mut self, eps: &BExp, dexp: &Deriv) -> BExp {
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
