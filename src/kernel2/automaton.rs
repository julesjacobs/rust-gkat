use super::solver::Solver;
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

struct Automaton<BExp> {
    // all states
    states: HashSet<u64>,
    // state behaviors
    eps_hat: HashMap<u64, BExp>,
    delta_hat: HashMap<u64, Vec<(BExp, u64, u64)>>,
}

impl<'a, BExp: DDNNFPtr<'a>, Builder: BottomUpBuilder<'a, BExp>> Solver<BExp, Builder> {
    fn mk_automaton(
        &mut self,
        gkat: &mut Gkat<'a, BExp, Builder>,
        m: Exp<BExp>,
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

    fn guard_map(gkat: &mut Gkat<'a, BExp, Builder>, guard: BExp, xs: &mut Vec<(BExp, u64, u64)>) {
        xs.retain_mut(|x| {
            let b = gkat.mk_and(guard, x.0);
            if b.is_false() {
                false
            } else {
                x.0 = b;
                true
            }
        });
    }

    #[recursive]
    fn mk_raw(&mut self, gkat: &mut Gkat<'a, BExp, Builder>, m: Exp<BExp>) -> RawAutomaton<BExp> {
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
            Seq(p1, p2) => todo!(),
            Ifte(b, p1, p2) => {
                let r1 = self.mk_raw(gkat, p1.clone());
                let r2 = self.mk_raw(gkat, p2.clone());
                // states
                let mut states = r1.states;
                states.extend(r2.states);
                // eps_star
                let nb = gkat.mk_not(*b);
                let r1_eps = gkat.mk_and(*b, r1.eps_star);
                let r2_eps = gkat.mk_and(nb, r2.eps_star);
                let eps_star = gkat.mk_or(r1_eps, r2_eps);
                // delta_star
                let mut r1_delta = r1.delta_star;
                let mut r2_delta = r2.delta_star;
                Self::guard_map(gkat, *b, &mut r1_delta);
                Self::guard_map(gkat, nb, &mut r2_delta);
                r1_delta.extend(r2_delta);
                let delta_star = r1_delta;
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
                let r = self.mk_raw(gkat, p.clone());
                // states
                let states = r.states;
                // eps_star
                let eps_star = gkat.mk_not(*b);
                // delta_star
                let mut delta_star = r.delta_star.clone();
                Self::guard_map(gkat, *b, &mut delta_star);
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
                    let mut new_elems = r.delta_star.clone();
                    let guard = gkat.mk_and(*b, *bx);
                    Self::guard_map(gkat, guard, &mut new_elems);
                    elems.extend(new_elems);
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
