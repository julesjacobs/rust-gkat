use crate::gkat_ast::bexp::*;
use crate::gkat_ast::exp;
use crate::gkat_ast::exp::*;
use crate::gkat_symb::dead::*;
use crate::gkat_symb::derivative::*;
use crate::gkat_symb::epsilon::*;
use hashconsing::HConsign;
use recursive::recursive;
use rsdd::{
    builder::{bdd::RobddBuilder, cache::AllIteTable, BottomUpBuilder},
    repr::BddPtr,
};
use std::collections::{HashMap, HashSet};

fn reject(fb: &mut HConsign<BExp_>, fp: &mut HConsign<Exp_>, exp: &Exp) -> BExp {
    let dexp = derivative(fb, fp, exp);
    let eps = epsilon(fb, exp);
    let zero = mk_zero(fb);
    let transitions = dexp.into_iter().fold(zero, |acc, (b, _)| mk_or(fb, acc, b));
    let not_epsilon = mk_not(fb, eps);
    let not_transitions = mk_not(fb, transitions);
    mk_and(fb, not_epsilon, not_transitions)
}

#[derive(PartialEq, Eq, Hash)]
struct ExpKey(Exp, Exp);

#[recursive]
fn equiv_helper<'a, Builder>(
    fb: &mut HConsign<BExp_>,
    fp: &mut HConsign<Exp_>,
    bdd: &'a Builder,
    cache: &mut HashMap<BExp, BddPtr<'a>>,
    dead_states: &mut HashSet<Exp>,
    explored: &mut HashSet<Exp>,
    tbl: &mut HashMap<ExpKey, bool>,
    exp1: &Exp,
    exp2: &Exp,
) -> bool
where
    Builder: BottomUpBuilder<'a, BddPtr<'a>>,
{
    let reject1 = reject(fb, fp, exp1);
    let reject2 = reject(fb, fp, exp2);
    // println!("exp1 = {:?}", exp1);
    // println!("exp2 = {:?}", exp2);
    // println!("reject1 = {:?}", reject1);
    // println!("reject2 = {:?}", reject2);

    if tbl
        .get(&ExpKey(exp1.clone(), exp2.clone()))
        .is_some_and(|b| *b)
    {
        true
    } else if dead_states.contains(exp1) {
        is_dead(fb, fp, bdd, cache, dead_states, explored, exp2)
    } else if dead_states.contains(exp2) {
        is_dead(fb, fp, bdd, cache, dead_states, explored, exp1)
    } else {
        let eps1 = epsilon(fb, exp1);
        let eps2 = epsilon(fb, exp2);
        let dexp1 = derivative(fb, fp, exp1);
        let dexp2 = derivative(fb, fp, exp2);
        let assert0 = is_equiv(bdd, cache, &eps1, &eps2);
        if !assert0 {
            return false;
        }
        let assert1 = dexp2.clone().into_iter().all(|(b0, (exp, _))| {
            let b1 = mk_and(fb, reject1.clone(), b0);
            is_false(bdd, cache, &b1) || is_dead(fb, fp, bdd, cache, dead_states, explored, &exp)
        });
        if !assert1 {
            return false;
        }
        let assert2 = dexp1.clone().into_iter().all(|(b0, (exp, _))| {
            let b1 = mk_and(fb, reject2.clone(), b0);
            is_false(bdd, cache, &b1) || is_dead(fb, fp, bdd, cache, dead_states, explored, &exp)
        });
        if !assert2 {
            return false;
        }
        let mut assert3 = true;
        for (be1, (next_exp1, p)) in dexp1 {
            for (be2, (next_exp2, q)) in dexp2.clone() {
                if is_false(bdd, cache, &mk_and(fb, be1.clone(), be2)) {
                    continue;
                } else if p == q {
                    tbl.insert(ExpKey(exp1.clone(), exp2.clone()), true);
                    tbl.insert(ExpKey(exp2.clone(), exp1.clone()), true);
                    assert3 = assert3
                        && equiv_helper(
                            fb,
                            fp,
                            bdd,
                            cache,
                            dead_states,
                            explored,
                            tbl,
                            &next_exp1,
                            &next_exp2,
                        )
                } else {
                    let result1 = is_dead(fb, fp, bdd, cache, dead_states, explored, &next_exp1);
                    let result2 = is_dead(fb, fp, bdd, cache, dead_states, explored, &next_exp2);
                    assert3 = assert3 && (result1 && result2)
                }
                if !assert3 {
                    break;
                }
            }
            if !assert3 {
                break;
            }
        }
        assert3
    }
}

pub fn equiv(fb: &mut HConsign<BExp_>, fp: &mut HConsign<Exp_>, exp1: &Exp, exp2: &Exp) -> bool {
    let mut dead_states: HashSet<Exp> = HashSet::new();
    let mut explored: HashSet<Exp> = HashSet::new();
    let mut tbl: HashMap<ExpKey, bool> = HashMap::new();
    let mut cache: HashMap<BExp, BddPtr> = HashMap::new();
    let bdd = RobddBuilder::<AllIteTable<BddPtr>>::new_with_linear_order(1024);
    equiv_helper(
        fb,
        fp,
        &bdd,
        &mut cache,
        &mut dead_states,
        &mut explored,
        &mut tbl,
        exp1,
        exp2,
    )
}
