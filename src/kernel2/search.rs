use super::automaton::*;
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
    pub fn reject(
        &mut self,
        gkat: &mut Gkat<'a, BExp, Builder>,
        st: u64,
        m: &Automaton<BExp>,
    ) -> BExp {
        let eps = m.eps_hat.get(&st).unwrap();
        m.delta_hat
            .get(&st)
            .unwrap()
            .iter()
            .fold(gkat.mk_not(*eps), |acc, (b, _, _)| {
                let nb = gkat.mk_not(*b);
                gkat.mk_or(acc, nb)
            })
    }

    fn visit_descendants(
        &mut self,
        gkat: &mut Gkat<'a, BExp, Builder>,
        sts: &Vec<u64>,
        m: &Automaton<BExp>,
    ) -> VisitResult {
        use VisitResult::*;
        let mut result = Unknown;
        for st in sts {
            match self.visit(gkat, *st, m) {
                Live => {
                    result = Live;
                    break;
                }
                Dead => {
                    self.explored.insert(*st);
                }
                Unknown => {}
            }
        }
        result
    }

    pub fn visit(
        &mut self,
        gkat: &mut Gkat<'a, BExp, Builder>,
        st: u64,
        m: &Automaton<BExp>,
    ) -> VisitResult {
        use VisitResult::*;
        if self.dead_states.contains(&st) {
            Dead
        } else if self.explored.contains(&st) {
            Unknown
        } else {
            self.explored.insert(st);
            let eps = m.eps_hat.get(&st).unwrap();
            if eps.is_false() {
                let sts: Vec<_> = m
                    .delta_hat
                    .get(&st)
                    .unwrap()
                    .iter()
                    .filter_map(|(b, st, _)| if b.is_false() { None } else { Some(*st) })
                    .collect();
                self.visit_descendants(gkat, &sts, m)
            } else {
                Live
            }
        }
    }

    pub fn is_dead(
        &mut self,
        gkat: &mut Gkat<'a, BExp, Builder>,
        st: u64,
        m: &Automaton<BExp>,
    ) -> bool {
        use VisitResult::*;
        self.explored.clear();
        match self.visit(gkat, st, m) {
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
