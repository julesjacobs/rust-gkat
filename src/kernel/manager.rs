use super::*;
use ahash::{HashMap, HashSet};
use disjoint_sets::UnionFindNode;
use hashconsing::HConsign;
use rsdd::builder::BottomUpBuilder;
use rsdd::repr::{BddPtr, DDNNFPtr, SddPtr};

pub struct GkatManager<'a, Ptr: DDNNFPtr<'a>, Builder: BottomUpBuilder<'a, Ptr>> {
    // bexp states
    pub(super) name_stamp: u64,
    pub(super) name_map: HashMap<String, u64>,
    pub(super) bexp_hcons: HConsign<BExp_>,
    pub(super) bexp_builder: &'a Builder,
    pub(super) bexp_cache: HashMap<BExp, Ptr>,
    // exp states
    pub(super) exp_hcons: HConsign<Exp_>,
    // search states
    pub(super) dead_states: HashSet<Exp>,
    pub(super) explored: HashSet<Exp>,
    pub(super) uf_table: HashMap<Exp, UnionFindNode<()>>,
}

impl<'a, Ptr: DDNNFPtr<'a>, Builder: BottomUpBuilder<'a, Ptr>> GkatManager<'a, Ptr, Builder> {
    pub fn new(builder: &'a Builder) -> Self {
        GkatManager {
            // bexp init
            name_stamp: 0,
            name_map: HashMap::default(),
            bexp_hcons: HConsign::empty(),
            bexp_builder: builder,
            bexp_cache: HashMap::default(),
            // exp init
            exp_hcons: HConsign::empty(),
            // search init
            dead_states: HashSet::default(),
            explored: HashSet::default(),
            uf_table: HashMap::default(),
        }
    }
}
