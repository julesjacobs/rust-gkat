use super::*;
use crate::syntax::*;
use rsdd::{builder::BottomUpBuilder, repr::DDNNFPtr};

impl<'a, BExp: DDNNFPtr<'a>, Builder: BottomUpBuilder<'a, BExp>> Solver<BExp, Builder> {
    pub fn reject(&mut self, gkat: &mut Gkat<'a, BExp, Builder>, exp: &Exp<BExp>) -> BExp {
        let dexp = self.derivative(gkat, exp);
        let eps = self.epsilon(gkat, exp);
        dexp.iter().fold(gkat.mk_not(eps), |acc, (b, _, _)| {
            let nb = gkat.mk_not(*b);
            gkat.mk_and(acc, nb)
        })
    }

    pub fn is_dead(&mut self, gkat: &mut Gkat<'a, BExp, Builder>, exp: &Exp<BExp>) -> bool {
        let mut stack = Vec::new();
        stack.push(exp.clone());
        self.explored.clear();
        while let Some(exp) = stack.pop() {
            if self.dead_states.contains(&exp) || self.explored.contains(&exp) {
                continue;
            }
            self.explored.insert(exp.clone());
            let eps = self.epsilon(gkat, &exp);
            if eps.is_false() {
                for (_, e, _) in self.derivative(gkat, &exp) {
                    stack.push(e);
                }
            } else {
                return false;
            }
        }
        self.dead_states.extend(self.explored.iter().cloned());
        true
    }
}
