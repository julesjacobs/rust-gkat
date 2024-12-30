use super::*;
use ahash::{HashMap, HashMapExt};
use recursive::recursive;

struct RawAutomaton {
    // pseudo-state behavior
    eps_star: BExp,
    delta_star: Vec<(BExp, u64, u64)>,
    // state behaviors
    eps_hat: HashMap<u64, BExp>,
    delta_hat: HashMap<u64, Vec<(BExp, u64, u64)>>,
}

#[derive(Debug)]
pub struct Automaton {
    // state behaviors
    pub eps_hat: HashMap<u64, BExp>,
    pub delta_hat: HashMap<u64, Vec<(BExp, u64, u64)>>,
}

impl Solver {
    pub fn mk_automaton(&mut self, gkat: &mut Gkat, m: &Exp) -> (u64, Automaton) {
        let r = self.mk_raw(gkat, m);
        let st = self.new_state();
        let eps_star = r.eps_star;
        let delta_star = r.delta_star;
        let mut eps_hat = r.eps_hat;
        let mut delta_hat = r.delta_hat;
        eps_hat.insert(st, eps_star);
        delta_hat.insert(st, delta_star);
        let automaton = Automaton {
            eps_hat: eps_hat,
            delta_hat: delta_hat,
        };
        (st, automaton)
    }

    #[recursive]
    fn mk_raw(&mut self, gkat: &mut Gkat, m: &Exp) -> RawAutomaton {
        use Exp_::*;
        match m.get() {
            Act(a) => {
                let st = self.new_state();
                // eps_star
                let eps_star = gkat.zero();
                // delta_star
                let delta_star = vec![(gkat.one(), st, *a)];
                // eps_hat
                let mut eps_hat = HashMap::new();
                eps_hat.insert(st, gkat.one());
                // delta_hat
                let mut delta_hat = HashMap::new();
                delta_hat.insert(st, vec![]);
                // raw_automaton
                RawAutomaton {
                    eps_star: eps_star,
                    delta_star: delta_star,
                    eps_hat: eps_hat,
                    delta_hat: delta_hat,
                }
            }
            Seq(p1, p2) => {
                let r1 = self.mk_raw(gkat, p1);
                let r2 = self.mk_raw(gkat, p2);
                // eps_star
                let eps_star = r1.eps_star.and(&r2.eps_star);
                // delta_star
                let mut delta_star = r1.delta_star;
                let delta_ext = GuardIterator::new(&r1.eps_star, r2.delta_star.iter());
                delta_star.extend(delta_ext);
                // eps_hat
                let mut eps_hat = r1.eps_hat.clone();
                for (_, be) in eps_hat.iter_mut() {
                    *be = be.and(&r2.eps_star)
                }
                eps_hat.extend(r2.eps_hat);
                // delta_hat
                let mut delta_hat = r1.delta_hat;
                for (i, elems) in delta_hat.iter_mut() {
                    let guard = r1.eps_hat.get(i).unwrap();
                    let elems_ext = GuardIterator::new(guard, r2.delta_star.iter());
                    elems.extend(elems_ext);
                }
                delta_hat.extend(r2.delta_hat);
                // raw_automaton
                RawAutomaton {
                    eps_star: eps_star,
                    delta_star: delta_star,
                    eps_hat: eps_hat,
                    delta_hat: delta_hat,
                }
            }
            Ifte(b, p1, p2) => {
                let r1 = self.mk_raw(gkat, p1);
                let r2 = self.mk_raw(gkat, p2);
                // eps_star
                let nb = b.not();
                let r1_eps = b.and(&r1.eps_star);
                let r2_eps = nb.and(&r2.eps_star);
                let eps_star = r1_eps.or(&r2_eps);
                // delta_star
                let mut delta_star: Vec<_> = GuardIterator::new(b, r1.delta_star.iter()).collect();
                let delta_ext = GuardIterator::new(&nb, r2.delta_star.iter());
                delta_star.extend(delta_ext);
                // eps_hat
                let mut eps_hat = r1.eps_hat;
                eps_hat.extend(r2.eps_hat);
                // delta_hat
                let mut delta_hat = r1.delta_hat;
                delta_hat.extend(r2.delta_hat);
                // raw_automaton
                RawAutomaton {
                    eps_star: eps_star,
                    delta_star: delta_star,
                    eps_hat: eps_hat,
                    delta_hat: delta_hat,
                }
            }
            Test(b) => RawAutomaton {
                eps_star: b.clone(),
                delta_star: vec![],
                eps_hat: HashMap::new(),
                delta_hat: HashMap::new(),
            },
            While(b, p) => {
                let r = self.mk_raw(gkat, p);
                // eps_star
                let eps_star = b.not();
                // delta_star
                let delta_star = GuardIterator::new(b, r.delta_star.iter()).collect();
                // eps_hat
                let mut eps_hat = r.eps_hat.clone();
                let nb = b.not();
                for (_, be) in eps_hat.iter_mut() {
                    *be = nb.and(be);
                }
                // delta_hat
                let mut delta_hat = r.delta_hat;
                for (i, elems) in delta_hat.iter_mut() {
                    let bx = r.eps_hat.get(i).unwrap();
                    let guard = b.and(bx);
                    let elems_ext = GuardIterator::new(&guard, r.delta_star.iter());
                    elems.extend(elems_ext);
                }
                // raw_automaton
                RawAutomaton {
                    eps_star: eps_star,
                    delta_star: delta_star,
                    eps_hat: eps_hat,
                    delta_hat: delta_hat,
                }
            }
        }
    }
}
