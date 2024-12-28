use super::guard::*;
use super::solver::*;
use crate::syntax::*;
use ahash::{HashMap, HashMapExt, HashSet};
use recursive::recursive;
use rsdd::{builder::BottomUpBuilder, repr::DDNNFPtr};

struct RawAutomaton<BExp> {
    // all states
    states: HashSet<u64>,
    // pseudo-state behavior
    eps_star: BExp,
    delta_star: Vec<(BExp, u64, u64)>,
    // state behaviors
    eps_hat: HashMap<u64, BExp>,
    delta_hat: HashMap<u64, Vec<(BExp, u64, u64)>>,
}

#[derive(Debug)]
pub struct Automaton<BExp> {
    // all states
    pub states: HashSet<u64>,
    // state behaviors
    pub eps_hat: HashMap<u64, BExp>,
    pub delta_hat: HashMap<u64, Vec<(BExp, u64, u64)>>,
}

impl<'a, BExp: DDNNFPtr<'a>, Builder: BottomUpBuilder<'a, BExp>> Solver<BExp, Builder> {
    pub fn mk_automaton(
        &mut self,
        gkat: &mut Gkat<'a, BExp, Builder>,
        m: &Exp<BExp>,
    ) -> (u64, Automaton<BExp>) {
        let r = self.mk_raw(gkat, m);
        let st = self.mk_state();
        let mut states = r.states;
        let eps_star = r.eps_star;
        let delta_star = r.delta_star;
        let mut eps_hat = r.eps_hat;
        let mut delta_hat = r.delta_hat;
        states.insert(st);
        eps_hat.insert(st, eps_star);
        delta_hat.insert(st, delta_star);
        let automaton = Automaton {
            states: states,
            eps_hat: eps_hat,
            delta_hat: delta_hat,
        };
        (st, automaton)
    }

    #[recursive]
    fn mk_raw(&mut self, gkat: &mut Gkat<'a, BExp, Builder>, m: &Exp<BExp>) -> RawAutomaton<BExp> {
        use Exp_::*;
        match m.get() {
            Act(a) => {
                let st = self.mk_state();
                // states
                let mut states = HashSet::default();
                states.insert(st);
                // eps_star
                let eps_star = gkat.mk_zero();
                // delta_star
                let delta_star = vec![(gkat.mk_one(), st, *a)];
                // eps_hat
                let mut eps_hat = HashMap::new();
                eps_hat.insert(st, gkat.mk_one());
                // delta_hat
                let mut delta_hat = HashMap::new();
                delta_hat.insert(st, vec![]);
                // raw_automaton
                RawAutomaton {
                    states: states,
                    eps_star: eps_star,
                    delta_star: delta_star,
                    eps_hat: eps_hat,
                    delta_hat: delta_hat,
                }
            }
            Seq(p1, p2) => {
                let r1 = self.mk_raw(gkat, p1);
                let r2 = self.mk_raw(gkat, p2);
                // states
                let mut states = r1.states;
                states.extend(r2.states);
                // eps_star
                let eps_star = gkat.mk_and(r1.eps_star, r2.eps_star);
                // delta_star
                let mut delta_star = r1.delta_star;
                let delta_ext = GuardIterator::new(gkat, r1.eps_star, r2.delta_star.iter());
                delta_star.extend(delta_ext);
                // eps_hat
                let mut eps_hat = r1.eps_hat.clone();
                for (_, be) in eps_hat.iter_mut() {
                    *be = gkat.mk_and(*be, r2.eps_star)
                }
                eps_hat.extend(r2.eps_hat);
                // delta_hat
                let mut delta_hat = r1.delta_hat;
                for (i, elems) in delta_hat.iter_mut() {
                    let guard = r1.eps_hat.get(i).unwrap();
                    let elems_ext = GuardIterator::new(gkat, *guard, r2.delta_star.iter());
                    elems.extend(elems_ext);
                }
                delta_hat.extend(r2.delta_hat);
                // raw_automaton
                RawAutomaton {
                    states: states,
                    eps_star: eps_star,
                    delta_star: delta_star,
                    eps_hat: eps_hat,
                    delta_hat: delta_hat,
                }
            }
            Ifte(b, p1, p2) => {
                let r1 = self.mk_raw(gkat, p1);
                let r2 = self.mk_raw(gkat, p2);
                // states
                let mut states = r1.states;
                states.extend(r2.states);
                // eps_star
                let nb = gkat.mk_not(*b);
                let r1_eps = gkat.mk_and(*b, r1.eps_star);
                let r2_eps = gkat.mk_and(nb, r2.eps_star);
                let eps_star = gkat.mk_or(r1_eps, r2_eps);
                // delta_star
                let mut delta_star: Vec<_> =
                    GuardIterator::new(gkat, *b, r1.delta_star.iter()).collect();
                let delta_ext = GuardIterator::new(gkat, nb, r2.delta_star.iter());
                delta_star.extend(delta_ext);
                // eps_hat
                let mut eps_hat = r1.eps_hat;
                eps_hat.extend(r2.eps_hat);
                // delta_hat
                let mut delta_hat = r1.delta_hat;
                delta_hat.extend(r2.delta_hat);
                // raw_automaton
                RawAutomaton {
                    states: states,
                    eps_star: eps_star,
                    delta_star: delta_star,
                    eps_hat: eps_hat,
                    delta_hat: delta_hat,
                }
            }
            Test(b) => RawAutomaton {
                states: HashSet::default(),
                eps_star: *b,
                delta_star: vec![],
                eps_hat: HashMap::new(),
                delta_hat: HashMap::new(),
            },
            While(b, p) => {
                let r = self.mk_raw(gkat, p);
                // states
                let states = r.states;
                // eps_star
                let eps_star = gkat.mk_not(*b);
                // delta_star
                let delta_star = GuardIterator::new(gkat, *b, r.delta_star.iter()).collect();
                // eps_hat
                let mut eps_hat = r.eps_hat.clone();
                let nb = gkat.mk_not(*b);
                for (_, be) in eps_hat.iter_mut() {
                    *be = gkat.mk_and(nb, *be);
                }
                // delta_hat
                let mut delta_hat = r.delta_hat;
                for (i, elems) in delta_hat.iter_mut() {
                    let bx = r.eps_hat.get(i).unwrap();
                    let guard = gkat.mk_and(*b, *bx);
                    let elems_ext = GuardIterator::new(gkat, guard, r.delta_star.iter());
                    elems.extend(elems_ext);
                }
                // raw_automaton
                RawAutomaton {
                    states: states,
                    eps_star: eps_star,
                    delta_star: delta_star,
                    eps_hat: eps_hat,
                    delta_hat: delta_hat,
                }
            }
        }
    }
}
