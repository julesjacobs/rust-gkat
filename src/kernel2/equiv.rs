use super::*;
use crate::syntax::*;
use recursive::recursive;
use rsdd::{builder::BottomUpBuilder, repr::DDNNFPtr};

impl<'a, BExp: DDNNFPtr<'a>, Builder: BottomUpBuilder<'a, BExp>> Solver<BExp, Builder> {
    #[recursive]
    pub fn equiv(
        &mut self,
        gkat: &mut Gkat<'a, BExp, Builder>,
        i: u64,
        j: u64,
        m: &Automaton<BExp>,
        n: &Automaton<BExp>,
    ) -> bool {
        let reject1 = self.reject(gkat, i, m);
        let reject2 = self.reject(gkat, j, n);

        let mut uf1 = self.get_uf(i);
        let mut uf2 = self.get_uf(j);

        if uf1.equiv(&uf2) {
            true
        } else if self.dead_states.contains(&i) {
            self.is_dead(gkat, j, n)
        } else if self.dead_states.contains(&j) {
            self.is_dead(gkat, i, m)
        } else {
            let eps1 = m.eps_hat.get(&i).unwrap();
            let eps2 = n.eps_hat.get(&j).unwrap();
            let delta1 = m.delta_hat.get(&i).unwrap();
            let delta2 = n.delta_hat.get(&j).unwrap();
            if !(eps1 == eps2) {
                return false;
            }
            let assert1 = delta2.iter().all(|(b0, st, _)| {
                let b1 = gkat.mk_and(reject1, *b0);
                b1.is_false() || self.is_dead(gkat, *st, n)
            });
            if !assert1 {
                return false;
            }
            let assert2 = delta1.iter().all(|(b0, st, _)| {
                let b1 = gkat.mk_and(reject2, *b0);
                b1.is_false() || self.is_dead(gkat, *st, m)
            });
            if !assert2 {
                return false;
            }
            let mut assert3;
            for (be1, st1, p) in delta1 {
                for (be2, st2, q) in delta2 {
                    let b1b2 = gkat.mk_and(*be1, *be2);
                    if b1b2.is_false() {
                        continue;
                    } else if p == q {
                        uf1.union(&mut uf2);
                        assert3 = self.equiv(gkat, *st1, *st2, m, n);
                    } else {
                        let result1 = self.is_dead(gkat, *st1, m);
                        let result2 = self.is_dead(gkat, *st2, n);
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
