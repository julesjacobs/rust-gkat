use crate::gkat_ast::bexp::*;
use crate::gkat_ast::exp::*;
use crate::gkat_symb::derivative::*;
use crate::gkat_symb::epsilon::*;
use hashbrown::HashMap;
use hashbrown::HashSet;
use hashconsing::HConsign;
use rsdd::{builder::BottomUpBuilder, repr::BddPtr};

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
    cache: &mut HashMap<BExp, BddPtr<'a>>,
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
    cache: &mut HashMap<BExp, BddPtr<'a>>,
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
    cache: &mut HashMap<BExp, BddPtr<'a>>,
    dead_states: &mut HashSet<Exp>,
    explored: &mut HashSet<Exp>,
    exp: &Exp,
) -> bool
where
    Builder: BottomUpBuilder<'a, BddPtr<'a>>,
{
    use VisitResult::*;
    match visit(fb, fp, bdd, cache, dead_states, explored, exp) {
        Unknown => {
            for x in explored.iter() {
                dead_states.insert(x.clone());
            }
            true
        }
        Live => false,
        Dead => true,
    }
}
