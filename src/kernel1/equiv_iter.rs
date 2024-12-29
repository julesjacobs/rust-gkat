use super::*;
use crate::syntax::*;
use rsdd::{builder::BottomUpBuilder, repr::DDNNFPtr};

impl<'a, BExp: DDNNFPtr<'a>, Builder: BottomUpBuilder<'a, BExp>> Solver<BExp, Builder> {
    pub fn equiv_iter(
        &mut self,
        gkat: &mut Gkat<'a, BExp, Builder>,
        exp1: &Exp<BExp>,
        exp2: &Exp<BExp>,
    ) -> bool {
        let mut queue = Vec::new();
        queue.push((exp1.clone(), exp2.clone()));
        while let Some((exp1, exp2)) = queue.pop() {
            let reject1 = self.reject(gkat, &exp1);
            let reject2 = self.reject(gkat, &exp2);

            let mut exp1_uf = self.get_uf(&exp1);
            let mut exp2_uf = self.get_uf(&exp2);

            if exp1_uf.equiv(&exp2_uf) {
                continue;
            } else if self.dead_states.contains(&exp1) && self.is_dead(gkat, &exp2) {
                continue;
            } else if self.dead_states.contains(&exp2) && self.is_dead(gkat, &exp1) {
                continue;
            } else {
                let eps1 = self.epsilon(gkat, &exp1);
                let eps2 = self.epsilon(gkat, &exp2);
                let dexp1 = self.derivative(gkat, &exp1);
                let dexp2 = self.derivative(gkat, &exp2);
                if !(eps1 == eps2) {
                    return false;
                }
                let assert1 = dexp2.iter().all(|(b0, exp, _)| {
                    let b1 = gkat.mk_and(reject1, *b0);
                    b1.is_false() || self.is_dead(gkat, &exp)
                });
                if !assert1 {
                    return false;
                }
                let assert2 = dexp1.iter().all(|(b0, exp, _)| {
                    let b1 = gkat.mk_and(reject2, *b0);
                    b1.is_false() || self.is_dead(gkat, &exp)
                });
                if !assert2 {
                    return false;
                }
                let mut assert3;
                for (be1, next_exp1, p) in dexp1 {
                    for (be2, next_exp2, q) in &dexp2 {
                        let b1b2 = gkat.mk_and(be1, *be2);
                        if b1b2.is_false() {
                            continue;
                        } else if p == *q {
                            exp1_uf.union(&mut exp2_uf);
                            queue.push((next_exp1.clone(), next_exp2.clone()));
                        } else {
                            let result1 = self.is_dead(gkat, &next_exp1);
                            let result2 = self.is_dead(gkat, &next_exp2);
                            assert3 = result1 && result2;
                            if !assert3 {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        true
    }
}
