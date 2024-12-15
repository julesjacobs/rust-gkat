use super::*;
use rsdd::{builder::BottomUpBuilder, repr::DDNNFPtr};

#[derive(Debug, Clone, Copy)]
pub enum VisitResult {
    Dead,
    Live,
    Unknown,
}

impl<'a, Ptr: DDNNFPtr<'a>, Builder: BottomUpBuilder<'a, Ptr>> GkatManager<'a, Ptr, Builder> {
    pub fn reject(&mut self, exp: &Exp<Ptr>) -> Ptr {
        let dexp = self.derivative(exp);
        let eps = self.epsilon(exp);
        let zero = self.mk_zero();
        let transitions = dexp
            .into_iter()
            .fold(zero, |acc, (b, _)| self.mk_or(acc, b));
        let not_epsilon = self.mk_not(eps);
        let not_transitions = self.mk_not(transitions);
        self.mk_and(not_epsilon, not_transitions)
    }

    fn visit_descendants(&mut self, exps: Vec<Exp<Ptr>>) -> VisitResult {
        use VisitResult::*;
        let mut result = Unknown;
        for e in exps {
            match self.visit(&e) {
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

    pub fn visit(&mut self, exp: &Exp<Ptr>) -> VisitResult {
        use VisitResult::*;
        if self.dead_states.contains(exp) {
            Dead
        } else if self.explored.contains(exp) {
            Unknown
        } else {
            self.explored.insert(exp.clone());
            let eps = self.epsilon(exp);
            if eps.is_false() {
                let dexp = self.derivative(exp);
                let next_exps: Vec<Exp<Ptr>> = dexp
                    .into_iter()
                    .filter_map(|(b, (e, _))| if b.is_false() { None } else { Some(e) })
                    .collect();
                self.visit_descendants(next_exps)
            } else {
                Live
            }
        }
    }

    pub fn is_dead(&mut self, exp: &Exp<Ptr>) -> bool {
        use VisitResult::*;
        match self.visit(exp) {
            Unknown => {
                for x in self.explored.iter() {
                    self.dead_states.insert(x.clone());
                }
                true
            }
            Live => false,
            Dead => true,
        }
    }
}
