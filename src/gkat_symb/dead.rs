use std::collections::HashMap;
use std::collections::HashSet;

use hashconsing::HConsign;
use rsdd::{builder::BottomUpBuilder, repr::BddPtr};

use crate::gkat_ast::bexp::*;
use crate::gkat_ast::exp::*;
use crate::gkat_symb::derivative::*;
use crate::gkat_symb::epsilon::*;

#[derive(Debug, Clone, Copy)]
pub enum VisitResult {
    Dead,
    Live,
    Unknown,
}

fn visit_descendants<'a, Builder>(
    fb: &mut HConsign<BExp_>,
    fp: &mut HConsign<Exp_>,
    bdd: &'a Builder,
    cache: &mut HashMap<BExp, bool>,
    dead_states: &HashSet<Exp>,
    explored: &mut HashSet<Exp>,
    exps: Vec<Exp>,
) -> VisitResult
where
    Builder: BottomUpBuilder<'a, BddPtr<'a>>,
{
    use VisitResult::*;
    let mut result = Unknown;
    for e in exps {
        match visit(fb, fp, bdd, cache, dead_states, explored, &e) {
            Live => {
                result = Live;
                break;
            }
            Dead => {
                explored.insert(e);
            }
            Unknown => {}
        }
    }
    result
}

pub fn visit<'a, Builder>(
    fb: &mut HConsign<BExp_>,
    fp: &mut HConsign<Exp_>,
    bdd: &'a Builder,
    cache: &mut HashMap<BExp, bool>,
    dead_states: &HashSet<Exp>,
    explored: &mut HashSet<Exp>,
    exp: &Exp,
) -> VisitResult
where
    Builder: BottomUpBuilder<'a, BddPtr<'a>>,
{
    use VisitResult::*;
    if dead_states.contains(exp) {
        Dead
    } else if explored.contains(exp) {
        Unknown
    } else {
        explored.insert(exp.clone());
        let eps = epsilon(fb, exp);
        if is_false(bdd, cache, &eps) {
            let dexp = derivative(fb, fp, exp);
            let next_exps: Vec<Exp> = dexp
                .into_iter()
                .filter_map(|(b, (e, _))| {
                    if is_false(bdd, cache, &b) {
                        None
                    } else {
                        Some(e)
                    }
                })
                .collect();
            visit_descendants(fb, fp, bdd, cache, dead_states, explored, next_exps)
        } else {
            Live
        }
    }
}

pub fn is_dead<'a, Builder>(
    fb: &mut HConsign<BExp_>,
    fp: &mut HConsign<Exp_>,
    bdd: &'a Builder,
    cache: &mut HashMap<BExp, bool>,
    dead_states: &mut HashSet<Exp>,
    exp: &Exp,
) -> bool
where
    Builder: BottomUpBuilder<'a, BddPtr<'a>>,
{
    use VisitResult::*;
    let mut explored: HashSet<Exp> = HashSet::new();
    match visit(fb, fp, bdd, cache, dead_states, &mut explored, exp) {
        Unknown => {
            dead_states.extend(explored);
            true
        }
        Live => false,
        Dead => true,
    }
}
