use super::*;

impl<B: BExp> Solver<B> {
    pub fn equiv_iter<G: Gkat<B>>(&mut self, gkat: &mut G, exp1: &Exp<B>, exp2: &Exp<B>) -> bool {
        let mut stack = vec![(exp1.clone(), exp2.clone())];
        while let Some((exp1, exp2)) = stack.pop() {
            let mut exp1_uf = self.get_uf(&exp1);
            let mut exp2_uf = self.get_uf(&exp2);

            if exp1_uf.equiv(&exp2_uf) {
                continue;
            } else if self.known_dead(&exp1) && self.is_dead(gkat, &exp2) {
                continue;
            } else if self.known_dead(&exp2) && self.is_dead(gkat, &exp1) {
                continue;
            } else {
                let eps1 = self.epsilon(gkat, &exp1);
                let eps2 = self.epsilon(gkat, &exp2);
                let dexp1 = self.derivative(gkat, &exp1);
                let dexp2 = self.derivative(gkat, &exp2);
                let reject1 = self.reject(gkat, &eps1, &dexp1);
                let reject2 = self.reject(gkat, &eps2, &dexp2);

                if !(gkat.is_equiv(&eps1, &eps2)) {
                    return false;
                }
                let assert1 = dexp2.iter().all(|(b0, exp, _)| {
                    let b1 = gkat.mk_and(&reject1, b0);
                    gkat.is_false(&b1) || self.is_dead(gkat, &exp)
                });
                if !assert1 {
                    return false;
                }
                let assert2 = dexp1.iter().all(|(b0, exp, _)| {
                    let b1 = gkat.mk_and(&reject2, b0);
                    gkat.is_false(&b1) || self.is_dead(gkat, &exp)
                });
                if !assert2 {
                    return false;
                }
                for (be1, next_exp1, p) in &dexp1 {
                    for (be2, next_exp2, q) in &dexp2 {
                        let b1b2 = gkat.mk_and(be1, be2);
                        if gkat.is_false(&b1b2) {
                            continue;
                        } else if *p == *q {
                            exp1_uf.union(&mut exp2_uf);
                            stack.push((next_exp1.clone(), next_exp2.clone()));
                        } else {
                            let result1 = self.is_dead(gkat, &next_exp1);
                            let result2 = self.is_dead(gkat, &next_exp2);
                            if !(result1 && result2) {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        return true;
    }
}
