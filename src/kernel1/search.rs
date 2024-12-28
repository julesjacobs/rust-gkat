use super::*;
use crate::syntax::*;
use rsdd::{builder::BottomUpBuilder, repr::DDNNFPtr};

#[derive(Debug, Clone, Copy)]
pub enum VisitResult {
    Dead,
    Live,
    Unknown,
}

impl<'a, BExp: DDNNFPtr<'a>, Builder: BottomUpBuilder<'a, BExp>> Solver<BExp, Builder> {
    pub fn reject(&mut self, gkat: &mut Gkat<'a, BExp, Builder>, exp: &Exp<BExp>) -> BExp {
        let dexp = self.derivative(gkat, exp);
        let eps = self.epsilon(gkat, exp);
        let zero = gkat.mk_zero();
        let transitions = dexp
            .into_iter()
            .fold(zero, |acc, (b, _, _)| gkat.mk_or(acc, b));
        let not_epsilon = gkat.mk_not(eps);
        let not_transitions = gkat.mk_not(transitions);
        gkat.mk_and(not_epsilon, not_transitions)
    }

    fn visit_descendants(
        &mut self,
        gkat: &mut Gkat<'a, BExp, Builder>,
        exps: Vec<Exp<BExp>>,
    ) -> VisitResult {
        use VisitResult::*;
        let mut result = Unknown;
        for e in exps {
            match self.visit(gkat, &e) {
                Live => {
                    result = Live;
                    break;
                }
                Dead => {
                    self.explored.insert(e);
                }
                Unknown => {}
            }
        }
        result
    }

    pub fn visit(&mut self, gkat: &mut Gkat<'a, BExp, Builder>, exp: &Exp<BExp>) -> VisitResult {
        use VisitResult::*;
        if self.dead_states.contains(exp) {
            Dead
        } else if self.explored.contains(exp) {
            Unknown
        } else {
            self.explored.insert(exp.clone());
            let eps = self.epsilon(gkat, exp);
            if eps.is_false() {
                let dexp = self.derivative(gkat, exp);
                let next_exps: Vec<Exp<BExp>> = dexp
                    .into_iter()
                    .filter_map(|(b, e, _)| if b.is_false() { None } else { Some(e) })
                    .collect();
                self.visit_descendants(gkat, next_exps)
            } else {
                Live
            }
        }
    }

    pub fn is_dead(&mut self, gkat: &mut Gkat<'a, BExp, Builder>, exp: &Exp<BExp>) -> bool {
        use VisitResult::*;
        self.explored.clear();
        match self.visit(gkat, exp) {
            Unknown => {
                let explored = &self.explored;
                self.dead_states.extend(explored.iter().cloned());
                true
            }
            Live => false,
            Dead => true,
        }
    }
}
