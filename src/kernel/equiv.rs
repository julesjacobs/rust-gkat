use super::*;
use recursive::recursive;
use rsdd::{builder::BottomUpBuilder, repr::DDNNFPtr};

impl<'a, Ptr: DDNNFPtr<'a>, Builder: BottomUpBuilder<'a, Ptr>> GkatManager<'a, Ptr, Builder> {
    #[recursive]
    pub fn equiv(&mut self, exp1: &Exp<Ptr>, exp2: &Exp<Ptr>) -> bool {
        let reject1 = self.reject(exp1);
        let reject2 = self.reject(exp2);

        let mut uf1 = self.get_uf(exp1);
        let mut uf2 = self.get_uf(exp2);

        if uf1.equiv(&uf2) {
            true
        } else if self.dead_states.contains(exp1) {
            self.is_dead(exp2)
        } else if self.dead_states.contains(exp2) {
            self.is_dead(exp1)
        } else {
            let eps1 = self.epsilon(exp1);
            let eps2 = self.epsilon(exp2);
            let dexp1 = self.derivative(exp1);
            let dexp2 = self.derivative(exp2);
            if !(eps1 == eps2) {
                return false;
            }
            let assert1 = dexp2.iter().all(|(b0, exp, _)| {
                let b1 = self.mk_and(reject1.clone(), b0.clone());
                b1.is_false() || self.is_dead(&exp)
            });
            if !assert1 {
                return false;
            }
            let assert2 = dexp1.iter().all(|(b0, exp, _)| {
                let b1 = self.mk_and(reject2.clone(), b0.clone());
                b1.is_false() || self.is_dead(&exp)
            });
            if !assert2 {
                return false;
            }
            let mut assert3;
            for (be1, next_exp1, p) in dexp1 {
                for (be2, next_exp2, q) in &dexp2 {
                    let b1b2 = self.mk_and(be1.clone(), be2.clone());
                    if b1b2.is_false() {
                        continue;
                    } else if p == *q {
                        uf1.union(&mut uf2);
                        assert3 = self.equiv(&next_exp1, &next_exp2);
                    } else {
                        let result1 = self.is_dead(&next_exp1);
                        let result2 = self.is_dead(&next_exp2);
                        assert3 = result1 && result2;
                    }
                    if !assert3 {
                        return false;
                    }
                }
            }
            true
        }
    }
}
